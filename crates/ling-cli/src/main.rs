use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{BufRead, IsTerminal as _, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod command_catalog;
mod completion;
mod exit_catalog;
mod init;
mod output_policy;
mod test_runner;

use command_catalog::Command;
use exit_catalog::{
    EXIT_COMPILE_ERROR, EXIT_INTERNAL_ERROR, EXIT_INVALID_USAGE, EXIT_RUNTIME_FAULT,
    EXIT_SNAPSHOT_MISMATCH, EXIT_SUCCESS,
};
use ling_cli::incident::{InternalIncident, Reproduction};
use ling_cli::project::{self, BUILD_PROFILE, BUILD_TARGET, CheckedProject, ProjectFailure};
use ling_cli::semantic_commands::{
    QueryError, QueryReport, TransactionError, TransactionReport, TransactionRequest,
};
use ling_cli::session::{Session, SubmissionFailure, SubmissionKind, SubmissionSuccess};
use ling_cli::{CompileFailure, compile_path, compile_source};
use ling_diagnostics::Diagnostic;
use ling_effects::locate_main;
use ling_eval::{Console, HostError, HostErrorCategory, MemoryConsole};
use ling_format::{FormatDisposition, build_format_ir, format_core_with_disposition};
use ling_project::{
    LockMode, LockedGraphFailure, ManifestError, PackageGraph, discover_modules, parse_manifest,
    resolve_package_graph_with_lock,
};
use ling_unicode::UNICODE_VERSION;
use output_policy::{ColorChoice, HumanLanguage, OutputFormat, OutputPolicy, Verbosity};

const CLI_NAME: &str = "ling";
fn main() -> ExitCode {
    ExitCode::from(run(env::args_os().skip(1).collect()))
}

fn run(arguments: Vec<OsString>) -> u8 {
    if arguments.is_empty() {
        eprintln!("{}", usage());
        return EXIT_INVALID_USAGE;
    }

    if arguments[0] == "--version" || arguments[0] == "-V" {
        println!("{CLI_NAME} {}", env!("CARGO_PKG_VERSION"));
        println!("Unicode {UNICODE_VERSION}");
        return EXIT_SUCCESS;
    }
    if arguments[0] == "--help" || arguments[0] == "-h" {
        println!("{}", usage());
        return EXIT_SUCCESS;
    }

    let Some(command_name) = arguments[0].to_str() else {
        return invalid_usage("the command name must be valid Unicode");
    };
    let (command, command_arguments) = if command_name == "project" {
        match arguments.get(1).and_then(|value| value.to_str()) {
            Some("check") => (Command::ProjectCheck, &arguments[2..]),
            Some(subcommand) => {
                return invalid_usage(&format!("unknown project subcommand `{subcommand}`"));
            }
            None => return invalid_usage("`project` requires a subcommand"),
        }
    } else {
        let Some(command) = Command::parse(command_name) else {
            return invalid_usage(&format!("unknown command `{command_name}`"));
        };
        (command, &arguments[1..])
    };

    if command == Command::Completion {
        return execute_completion(command_arguments);
    }

    let options = match Options::parse(command, command_arguments) {
        Ok(options) => options,
        Err(message) => return invalid_usage(&message),
    };

    execute(options)
}

fn execute_completion(arguments: &[OsString]) -> u8 {
    let [shell] = arguments else {
        return invalid_usage(
            "`completion` requires exactly one shell: bash, zsh, fish, or powershell",
        );
    };
    let Some(shell) = shell.to_str() else {
        return invalid_usage("the completion shell must be valid Unicode");
    };
    let Some(shell) = completion::Shell::parse(shell) else {
        return invalid_usage(&format!("unsupported completion shell `{shell}`"));
    };
    let rendered = completion::render(shell);
    match write_stdout(rendered.as_bytes()) {
        Ok(()) => EXIT_SUCCESS,
        Err(error) => emit_host_io_failure("completion.stdout", &error, OutputPolicy::human()),
    }
}

fn execute(options: Options) -> u8 {
    if options.policy.is_verbose() {
        eprintln!("{}", options.policy.verbose_event(options.command.name()));
    }
    if options.command == Command::Init {
        return execute_init(
            options.policy,
            options.path.expect("init requires a destination"),
            options.init_name,
            options.init_display_name,
        );
    }
    if options.command == Command::Query {
        return execute_query(
            options.policy,
            options.path.expect("query requires a source path"),
            options.symbol.expect("query requires a symbol"),
        );
    }
    if options.command == Command::Patch {
        return execute_patch(
            options.policy,
            options.path.expect("patch requires a source path"),
            options
                .transaction_path
                .expect("patch requires a transaction path"),
        );
    }
    if options.command == Command::Build
        || (matches!(
            options.command,
            Command::Check | Command::Run | Command::Test
        ) && options.manifest_path.is_some())
    {
        return execute_project_command(options);
    }
    if options.command == Command::Test {
        return execute_test(
            options.policy,
            options.path.expect("test requires an input path"),
        );
    }
    if options.command == Command::ProjectCheck {
        return execute_project_check(
            options.policy,
            options
                .manifest_path
                .expect("project check requires a manifest path"),
        );
    }

    if options.command == Command::Lsp {
        debug_assert!(options.stdio);
        return execute_lsp();
    }

    if options.command == Command::Repl {
        return execute_repl(options.policy, options.capabilities);
    }

    if options.command == Command::Format {
        return execute_format(
            options.policy,
            options.check,
            options.path.expect("format requires an input"),
            options.stdin_name,
        );
    }

    let path = options
        .path
        .as_deref()
        .expect("commands other than repl require a path");
    let reproduction =
        Reproduction::new(format!("ling {}", options.command)).with_input(path.to_string_lossy());
    let compiled = match compile_path(path) {
        Ok(compiled) => compiled,
        Err(CompileFailure::Diagnostics(diagnostics)) => {
            return emit_compile_errors(&diagnostics, options.policy);
        }
        Err(CompileFailure::Internal(message)) => {
            return emit_internal_incident(
                "compile.pipeline",
                message,
                reproduction,
                options.policy,
            );
        }
        Err(CompileFailure::SnapshotMismatch(message)) => {
            return emit_snapshot_mismatch(&message, options.policy);
        }
    };

    if !compiled.snapshot.checked().warnings().is_empty() {
        let status = emit_diagnostics(
            compiled.snapshot.checked().warnings(),
            options.policy,
            EXIT_SUCCESS,
        );
        if status != EXIT_SUCCESS {
            return status;
        }
    }

    match options.command {
        Command::Check => EXIT_SUCCESS,
        Command::Semantic => {
            let mut stdout = std::io::stdout().lock();
            match stdout
                .write_all(compiled.snapshot.json().as_bytes())
                .and_then(|()| stdout.write_all(b"\n"))
            {
                Ok(()) => EXIT_SUCCESS,
                Err(error) => emit_host_io_failure("semantic.stdout", &error, options.policy),
            }
        }
        Command::Audit => {
            let audit = compiled.snapshot.audit_model();
            let rendered = match ling_format::render_audit(&audit) {
                Ok(rendered) => rendered,
                Err(error) => {
                    return emit_internal_incident(
                        "audit.render",
                        error.to_string(),
                        reproduction,
                        options.policy,
                    );
                }
            };
            let mut stdout = std::io::stdout().lock();
            match stdout.write_all(rendered.as_bytes()) {
                Ok(()) => EXIT_SUCCESS,
                Err(error) => emit_host_io_failure("audit.stdout", &error, options.policy),
            }
        }
        Command::Query | Command::Patch => unreachable!("handled before compilation"),
        Command::Run => {
            let main = match locate_main(compiled.snapshot.checked()) {
                Ok(main) => main,
                Err(error) => {
                    return emit_compile_error(error.to_diagnostic(), options.policy);
                }
            };
            let mut console = StdoutConsole;
            match ling_eval::execute_main(&compiled.snapshot, &main, &mut console) {
                Ok(()) => EXIT_SUCCESS,
                Err(fault) => {
                    emit_diagnostics(&[fault.to_diagnostic()], options.policy, EXIT_RUNTIME_FAULT)
                }
            }
        }
        Command::Repl => unreachable!("handled before compilation"),
        Command::Format => unreachable!("handled before compilation"),
        Command::ProjectCheck => unreachable!("handled before project checking"),
        Command::Lsp => unreachable!("handled before compilation"),
        Command::Init => unreachable!("handled before compilation"),
        Command::Test => unreachable!("handled before test execution"),
        Command::Build => unreachable!("handled before file compilation"),
        Command::Completion => unreachable!("handled before option parsing"),
    }
}

fn execute_query(policy: OutputPolicy, source_path: PathBuf, symbol: String) -> u8 {
    let reproduction = Reproduction::new("ling query").with_input(source_path.to_string_lossy());
    let compiled = match compile_path(&source_path) {
        Ok(compiled) => compiled,
        Err(failure) => return emit_compile_failure(failure, reproduction, policy),
    };
    if !compiled.snapshot.checked().warnings().is_empty() {
        let status = emit_diagnostics(compiled.snapshot.checked().warnings(), policy, EXIT_SUCCESS);
        if status != EXIT_SUCCESS {
            return status;
        }
    }
    let report = match QueryReport::build(&compiled.snapshot, &symbol) {
        Ok(report) => report,
        Err(QueryError::InvalidSymbol(detail) | QueryError::Scope(detail)) => {
            let diagnostic = Diagnostic::new(
                ling_diagnostics::codes::INVALID_SEMANTIC_QUERY,
                ling_diagnostics::Severity::Error,
                "Semantic Query 输入无效",
                "invalid Semantic Query input",
            )
            .with_fact("detail", detail);
            return emit_compile_error(diagnostic, policy);
        }
    };
    let rendered = match policy.format() {
        OutputFormat::Json => report.to_json().map(|json| format!("{json}\n")),
        OutputFormat::Human => Ok(render_query_human(&report, policy)),
    };
    emit_protocol_stdout("query.stdout", rendered, reproduction, policy)
}

fn render_query_human(report: &QueryReport, policy: OutputPolicy) -> String {
    let mut output = policy.human_summary(
        "Semantic Query 完成",
        "Semantic Query completed",
        &format!(
            "symbol={} matches={}",
            report.symbol(),
            report.matches().len()
        ),
    );
    output.push('\n');
    for result in report.matches() {
        output.push_str(&result.summary());
        output.push('\n');
    }
    output
}

fn execute_patch(policy: OutputPolicy, source_path: PathBuf, transaction_path: PathBuf) -> u8 {
    let reproduction = Reproduction::new("ling patch")
        .with_input(source_path.to_string_lossy())
        .with_input(transaction_path.to_string_lossy());
    let current = match compile_path(&source_path) {
        Ok(compiled) => compiled,
        Err(failure) => return emit_compile_failure(failure, reproduction.clone(), policy),
    };
    let transaction_bytes = match fs::read(&transaction_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return emit_transaction_error(
                TransactionError::InvalidInput(format!(
                    "cannot read transaction document: {}",
                    host_error_category(error.kind()).name()
                )),
                policy,
            );
        }
    };
    let request = match TransactionRequest::parse(&transaction_bytes) {
        Ok(request) => request,
        Err(error) => return emit_transaction_error(error, policy),
    };
    if let Err(error) = request.validate_current(&current.snapshot) {
        return emit_transaction_error(error, policy);
    }
    let logical_name = source_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("proposal.ling")
        .to_owned();
    let candidate = match compile_source(logical_name, request.replacement().as_bytes().to_vec()) {
        Ok(compiled) => compiled,
        Err(failure) => return emit_compile_failure(failure, reproduction.clone(), policy),
    };
    for warnings in [
        current.snapshot.checked().warnings(),
        candidate.snapshot.checked().warnings(),
    ] {
        if !warnings.is_empty() {
            let status = emit_diagnostics(warnings, policy, EXIT_SUCCESS);
            if status != EXIT_SUCCESS {
                return status;
            }
        }
    }
    let report = match TransactionReport::validate(&current.snapshot, &candidate.snapshot, &request)
    {
        Ok(report) => report,
        Err(error) => return emit_transaction_error(error, policy),
    };
    let rendered = match policy.format() {
        OutputFormat::Json => report.to_json().map(|json| format!("{json}\n")),
        OutputFormat::Human => Ok(format!(
            "{}\n",
            policy.human_summary(
                "Semantic Transaction 提案已验证；未提交",
                "Semantic Transaction proposal validated; not committed",
                &format!("changed_body_ids={}", report.changed_body_ids().join(",")),
            )
        )),
    };
    emit_protocol_stdout("patch.stdout", rendered, reproduction, policy)
}

fn emit_compile_failure(
    failure: CompileFailure,
    reproduction: Reproduction,
    policy: OutputPolicy,
) -> u8 {
    match failure {
        CompileFailure::Diagnostics(diagnostics) => emit_compile_errors(&diagnostics, policy),
        CompileFailure::Internal(message) => {
            emit_internal_incident("compile.pipeline", message, reproduction, policy)
        }
        CompileFailure::SnapshotMismatch(message) => emit_snapshot_mismatch(&message, policy),
    }
}

fn emit_transaction_error(error: TransactionError, policy: OutputPolicy) -> u8 {
    let diagnostic = match error {
        TransactionError::InvalidInput(detail) => Diagnostic::new(
            ling_diagnostics::codes::INVALID_SEMANTIC_TRANSACTION,
            ling_diagnostics::Severity::Error,
            "Semantic Transaction 输入无效",
            "invalid Semantic Transaction input",
        )
        .with_fact("detail", detail),
        TransactionError::StaleBase { expected, found } => Diagnostic::new(
            ling_diagnostics::codes::STALE_SEMANTIC_TRANSACTION,
            ling_diagnostics::Severity::Error,
            "Semantic Transaction 基础快照已过期",
            "Semantic Transaction base snapshot is stale",
        )
        .with_fact("expected", expected)
        .with_fact("found", found),
        TransactionError::PreserveViolation(constraint) => Diagnostic::new(
            ling_diagnostics::codes::SEMANTIC_PRESERVE_VIOLATION,
            ling_diagnostics::Severity::Error,
            "Semantic Transaction 未保持要求的语义",
            "Semantic Transaction did not preserve required semantics",
        )
        .with_fact("constraint", constraint),
    };
    emit_compile_error(diagnostic, policy)
}

fn emit_protocol_stdout(
    stage: &str,
    rendered: Result<String, serde_json::Error>,
    reproduction: Reproduction,
    policy: OutputPolicy,
) -> u8 {
    let rendered = match rendered {
        Ok(rendered) => rendered,
        Err(error) => {
            return emit_internal_incident(stage, error.to_string(), reproduction, policy);
        }
    };
    match std::io::stdout().lock().write_all(rendered.as_bytes()) {
        Ok(()) => EXIT_SUCCESS,
        Err(error) => emit_host_io_failure(stage, &error, policy),
    }
}

fn execute_project_command(options: Options) -> u8 {
    let operation = options.command.name();
    let manifest_path = options
        .manifest_path
        .as_deref()
        .expect("project commands require a manifest path");
    let project = match project::compile(manifest_path) {
        Ok(project) => project,
        Err(failure) => return emit_project_command_failure(operation, failure, options.policy),
    };
    if !project.snapshot().checked().warnings().is_empty() {
        let status = emit_diagnostics(
            project.snapshot().checked().warnings(),
            options.policy,
            EXIT_SUCCESS,
        );
        if status != EXIT_SUCCESS {
            return status;
        }
    }

    match options.command {
        Command::Check => emit_project_check_success(&project, options.policy),
        Command::Run => execute_project_run(&project, options.policy),
        Command::Test => execute_project_test(&project, options.policy),
        Command::Build => execute_project_build(
            &project,
            options.output.expect("project build requires output"),
            options.policy,
        ),
        _ => unreachable!("only semantic project commands reach this function"),
    }
}

fn execute_project_run(project: &CheckedProject, policy: OutputPolicy) -> u8 {
    let main = match locate_main(project.snapshot().checked()) {
        Ok(main) => main,
        Err(error) => {
            return emit_project_command_diagnostics(
                "run",
                &[error.to_diagnostic()],
                None,
                policy,
                EXIT_COMPILE_ERROR,
            );
        }
    };
    let mut console = MemoryConsole::default();
    match ling_eval::execute_project_main(project.snapshot(), &main, &mut console) {
        Ok(()) => match policy.format() {
            OutputFormat::Human => match write_stdout(console.output().as_bytes()) {
                Ok(()) => EXIT_SUCCESS,
                Err(error) => emit_host_io_failure("project.run.stdout", &error, policy),
            },
            OutputFormat::Json => emit_project_command_success(
                project,
                "run",
                serde_json::json!({"stdout": console.output()}),
                policy,
            ),
        },
        Err(fault) => emit_project_command_diagnostics(
            "run",
            &[fault.to_diagnostic()],
            Some(console.output()),
            policy,
            EXIT_RUNTIME_FAULT,
        ),
    }
}

fn execute_project_test(project: &CheckedProject, policy: OutputPolicy) -> u8 {
    let name = format!(
        "{}::{}",
        project.manifest().package().name(),
        project.manifest().source().entry()
    );
    let main = match locate_main(project.snapshot().checked()) {
        Ok(main) => main,
        Err(error) => {
            return emit_project_command_diagnostics(
                "test",
                &[error.to_diagnostic()],
                None,
                policy,
                EXIT_COMPILE_ERROR,
            );
        }
    };
    let mut console = MemoryConsole::default();
    match ling_eval::execute_project_main(project.snapshot(), &main, &mut console) {
        Ok(()) => match policy.format() {
            OutputFormat::Human => {
                if !policy.is_quiet() {
                    let facts = format!("name={name} total=1 passed=1 failed=0");
                    println!(
                        "{}",
                        policy.human_summary("工程测试通过", "project test passed", &facts)
                    );
                }
                EXIT_SUCCESS
            }
            OutputFormat::Json => emit_project_command_success(
                project,
                "test",
                serde_json::json!({
                    "tests": [{"name": name, "status": "passed", "stdout": console.output()}],
                    "counts": {"total": 1, "passed": 1, "failed": 0},
                }),
                policy,
            ),
        },
        Err(fault) => emit_project_command_diagnostics(
            "test",
            &[fault.to_diagnostic()],
            Some(console.output()),
            policy,
            EXIT_RUNTIME_FAULT,
        ),
    }
}

fn execute_project_build(checked: &CheckedProject, output: PathBuf, policy: OutputPolicy) -> u8 {
    let artifact = match project::build(checked, output) {
        Ok(artifact) => artifact,
        Err(failure) => return emit_project_command_failure("build", failure, policy),
    };
    match policy.format() {
        OutputFormat::Human => {
            if !policy.is_quiet() {
                let facts = format!(
                    "artifact={} identity={} bytes={} profile={} target={}",
                    project::ARTIFACT_PROTOCOL,
                    artifact.identity(),
                    artifact.bytes().len(),
                    BUILD_PROFILE,
                    BUILD_TARGET,
                );
                println!(
                    "{}",
                    policy.human_summary("工程构建完成", "project build completed", &facts)
                );
            }
            EXIT_SUCCESS
        }
        OutputFormat::Json => emit_project_command_success(
            checked,
            "build",
            serde_json::json!({
                "artifact": {
                    "protocol": project::ARTIFACT_PROTOCOL,
                    "identity": artifact.identity(),
                    "bytes": artifact.bytes().len(),
                    "profile": BUILD_PROFILE,
                    "target": BUILD_TARGET,
                }
            }),
            policy,
        ),
    }
}

fn emit_project_check_success(project: &CheckedProject, policy: OutputPolicy) -> u8 {
    match policy.format() {
        OutputFormat::Human => {
            if !policy.is_quiet() {
                let facts = format!(
                    "package={} version={} entry={} packages={} modules={} graph={} program={}",
                    project.manifest().package().name(),
                    project.manifest().package().version(),
                    project.manifest().source().entry(),
                    project.locked().graph().packages().len(),
                    project.snapshot().graph().modules.len(),
                    project.locked().graph().id(),
                    project.snapshot().program_id(),
                );
                println!(
                    "{}",
                    policy.human_summary(
                        "工程语义检查通过",
                        "project semantic check passed",
                        &facts
                    )
                );
            }
            EXIT_SUCCESS
        }
        OutputFormat::Json => {
            emit_project_command_success(project, "check", serde_json::json!({}), policy)
        }
    }
}

fn emit_project_command_success(
    project: &CheckedProject,
    operation: &str,
    extra: serde_json::Value,
    policy: OutputPolicy,
) -> u8 {
    debug_assert_eq!(policy.format(), OutputFormat::Json);
    let mut report = serde_json::json!({
        "protocol": project::COMMAND_PROTOCOL,
        "operation": operation,
        "status": "ok",
        "package": {
            "name": project.manifest().package().name().as_str(),
            "version": project.manifest().package().version().to_string(),
        },
        "entry": project.manifest().source().entry().as_str(),
        "graph": project.locked().graph().id().as_str(),
        "program": project.snapshot().program_id().to_string(),
    });
    if let (Some(report), Some(extra)) = (report.as_object_mut(), extra.as_object()) {
        report.extend(extra.clone());
    }
    match serde_json::to_vec(&report) {
        Ok(mut rendered) => {
            rendered.push(b'\n');
            match write_stdout(&rendered) {
                Ok(()) => EXIT_SUCCESS,
                Err(error) => emit_host_io_failure("project.success.stdout", &error, policy),
            }
        }
        Err(error) => emit_internal_incident(
            "project.success-json",
            error.to_string(),
            Reproduction::new(format!("ling {operation} --manifest-path ling.toml")),
            policy,
        ),
    }
}

fn emit_project_command_failure(
    operation: &str,
    failure: ProjectFailure,
    policy: OutputPolicy,
) -> u8 {
    match failure {
        ProjectFailure::Diagnostics(diagnostics) => emit_project_command_diagnostics(
            operation,
            &diagnostics,
            None,
            policy,
            EXIT_COMPILE_ERROR,
        ),
        ProjectFailure::SnapshotMismatch(message) => emit_project_command_diagnostics(
            operation,
            &[snapshot_mismatch_diagnostic(&message)],
            None,
            policy,
            EXIT_SNAPSHOT_MISMATCH,
        ),
        ProjectFailure::ArtifactIo {
            operation: stage,
            kind,
        } => {
            let diagnostic = Diagnostic::new(
                ling_diagnostics::codes::PROJECT_ARTIFACT_IO_FAILED,
                ling_diagnostics::Severity::Error,
                "工程构建产物操作失败",
                "project build artifact operation failed",
            )
            .with_fact("io_kind", project::stable_io_kind(kind))
            .with_fact("operation", stage);
            emit_project_command_diagnostics(
                operation,
                &[diagnostic],
                None,
                policy,
                EXIT_RUNTIME_FAULT,
            )
        }
        ProjectFailure::Internal(message) => {
            let incident = InternalIncident::capture(
                "project.compile",
                message,
                Reproduction::new(format!("ling {operation} --manifest-path ling.toml")),
            );
            emit_project_command_diagnostics(
                operation,
                &[incident.diagnostic()],
                None,
                policy,
                EXIT_INTERNAL_ERROR,
            )
        }
    }
}

fn emit_project_command_diagnostics(
    operation: &str,
    diagnostics: &[Diagnostic],
    stdout: Option<&str>,
    policy: OutputPolicy,
    exit_code: u8,
) -> u8 {
    match policy.format() {
        OutputFormat::Human => emit_diagnostics(diagnostics, policy, exit_code),
        OutputFormat::Json => {
            let values = match diagnostic_values(diagnostics) {
                Ok(values) => values,
                Err(status) => return status,
            };
            let mut report = serde_json::json!({
                "protocol": project::COMMAND_PROTOCOL,
                "operation": operation,
                "status": "error",
                "diagnostics": values,
            });
            if let Some(stdout) = stdout {
                report["stdout"] = serde_json::Value::String(stdout.to_owned());
            }
            match serde_json::to_vec(&report) {
                Ok(mut rendered) => {
                    rendered.push(b'\n');
                    match std::io::stderr().lock().write_all(&rendered) {
                        Ok(()) => exit_code,
                        Err(error) => emit_host_io_failure(
                            "project.failure.stderr",
                            &error,
                            OutputPolicy::human(),
                        ),
                    }
                }
                Err(error) => emit_internal_incident(
                    "project.failure-json",
                    error.to_string(),
                    Reproduction::new(format!("ling {operation} --manifest-path ling.toml")),
                    policy,
                ),
            }
        }
    }
}

fn execute_test(policy: OutputPolicy, root: PathBuf) -> u8 {
    let summary = match test_runner::run(root) {
        Ok(summary) => summary,
        Err(test_runner::Failure::Usage(message)) => return invalid_usage(&message),
        Err(failure @ test_runner::Failure::Io { .. })
        | Err(failure @ test_runner::Failure::NoCases { .. }) => {
            let diagnostic = failure
                .diagnostic()
                .expect("test discovery failures always have a diagnostic");
            return emit_diagnostics(
                &[diagnostic],
                policy,
                if matches!(failure, test_runner::Failure::NoCases { .. }) {
                    EXIT_COMPILE_ERROR
                } else {
                    EXIT_RUNTIME_FAULT
                },
            );
        }
        Err(test_runner::Failure::Internal(message)) => {
            return emit_internal_incident(
                "test.runner",
                message,
                Reproduction::new("ling test"),
                policy,
            );
        }
        Err(test_runner::Failure::Snapshot(message)) => {
            return emit_snapshot_mismatch(&message, policy);
        }
    };

    let exit_code = summary.exit_code();
    let diagnostics = summary.diagnostics();
    if !diagnostics.is_empty() {
        let rendered_status = emit_diagnostics(&diagnostics, policy, exit_code);
        if rendered_status != exit_code {
            return rendered_status;
        }
    }

    match policy.format() {
        OutputFormat::Human => {
            if policy.is_quiet() && exit_code == EXIT_SUCCESS {
                return exit_code;
            }
            let facts = format!(
                "root={} total={} passed={} failed={}",
                summary.root(),
                summary.total(),
                summary.passed(),
                summary.failed()
            );
            println!(
                "{}",
                policy.human_summary("测试完成", "tests completed", &facts)
            );
            exit_code
        }
        OutputFormat::Json => {
            let tests = summary
                .cases()
                .iter()
                .map(|case| {
                    serde_json::json!({
                        "name": case.name(),
                        "status": case.status().as_str(),
                        "stdout": case.stdout(),
                    })
                })
                .collect::<Vec<_>>();
            let report = serde_json::json!({
                "schema": test_runner::TEST_PROTOCOL,
                "status": summary.status(),
                "root": summary.root(),
                "tests": tests,
                "counts": {
                    "total": summary.total(),
                    "passed": summary.passed(),
                    "failed": summary.failed(),
                },
            });
            match serde_json::to_string(&report) {
                Ok(rendered) => match write_stdout(format!("{rendered}\n").as_bytes()) {
                    Ok(()) => exit_code,
                    Err(error) => emit_host_io_failure("test.stdout", &error, policy),
                },
                Err(error) => emit_internal_incident(
                    "test.success-json",
                    error.to_string(),
                    Reproduction::new("ling test --format json"),
                    policy,
                ),
            }
        }
    }
}

fn execute_init(
    policy: OutputPolicy,
    destination: PathBuf,
    package_name: Option<String>,
    display_name: Option<String>,
) -> u8 {
    match init::create(destination, package_name, display_name) {
        Ok(summary) => match policy.format() {
            OutputFormat::Human => {
                if policy.is_quiet() {
                    return EXIT_SUCCESS;
                }
                let facts = format!(
                    "directory={} package={} files={}",
                    summary.directory,
                    summary.package_name,
                    summary.files.join(",")
                );
                println!(
                    "{}",
                    policy.human_summary("已创建 Ling 工程", "created Ling project", &facts)
                );
                EXIT_SUCCESS
            }
            OutputFormat::Json => {
                let report = serde_json::json!({
                    "schema": init::INIT_PROTOCOL,
                    "status": "ok",
                    "directory": summary.directory,
                    "template_version": init::INIT_TEMPLATE_VERSION,
                    "package": {"name": summary.package_name, "version": "0.1.0"},
                    "files": summary.files,
                });
                match serde_json::to_string(&report) {
                    Ok(rendered) => match write_stdout(format!("{rendered}\n").as_bytes()) {
                        Ok(()) => EXIT_SUCCESS,
                        Err(error) => emit_host_io_failure("init.stdout", &error, policy),
                    },
                    Err(error) => emit_internal_incident(
                        "init.success-json",
                        error.to_string(),
                        Reproduction::new("ling init --format json"),
                        policy,
                    ),
                }
            }
        },
        Err(init::Failure::Usage(message)) => invalid_usage(&message),
        Err(init::Failure::Diagnostics(diagnostics)) => {
            emit_diagnostics(&diagnostics, policy, EXIT_COMPILE_ERROR)
        }
        Err(init::Failure::Internal(message)) => emit_internal_incident(
            "init.template",
            message,
            Reproduction::new("ling init"),
            policy,
        ),
        Err(failure @ init::Failure::Io { .. }) => {
            let diagnostic = init::diagnostic_for_failure(&failure)
                .expect("I/O init failures always have a diagnostic");
            emit_diagnostics(&[diagnostic], policy, EXIT_RUNTIME_FAULT)
        }
    }
}

fn execute_lsp() -> u8 {
    match ling_lsp::run_stdio(std::io::stdin().lock(), std::io::stdout()) {
        Ok(result) => result.exit_code(),
        Err(error) => {
            eprintln!("LSP 传输失败：{error}\nLSP transport failed: {error}");
            EXIT_RUNTIME_FAULT
        }
    }
}

fn execute_project_check(policy: OutputPolicy, manifest_path: PathBuf) -> u8 {
    let project_root = manifest_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let bytes = match fs::read(&manifest_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return emit_internal_incident(
                "project.manifest-read",
                error.to_string(),
                Reproduction::new("ling project check"),
                policy,
            );
        }
    };
    let manifest = match parse_manifest("ling.toml", &bytes) {
        Ok(manifest) => manifest,
        Err(error) => return emit_project_manifest_failure(policy, error),
    };

    if let Err(error) = discover_modules(project_root, &manifest) {
        return emit_project_discovery_failure(policy, error);
    }
    let graph = match resolve_package_graph_with_lock(project_root, &manifest, LockMode::Locked) {
        Ok(graph) => graph,
        Err(error) => return emit_project_locked_failure(policy, error),
    };
    emit_project_success(policy, &graph)
}

fn emit_project_manifest_failure(policy: OutputPolicy, error: ManifestError) -> u8 {
    emit_project_diagnostics(policy, &[error.diagnostic()])
}

fn emit_project_discovery_failure(
    policy: OutputPolicy,
    error: ling_project::DiscoveryFailure,
) -> u8 {
    match error {
        ling_project::DiscoveryFailure::Diagnostics(diagnostics) => {
            emit_project_diagnostics(policy, &diagnostics)
        }
        ling_project::DiscoveryFailure::Internal(message) => emit_internal_incident(
            "project.module-discovery",
            message,
            Reproduction::new("ling project check"),
            policy,
        ),
    }
}

fn emit_project_locked_failure(policy: OutputPolicy, error: LockedGraphFailure) -> u8 {
    match error.diagnostics() {
        Some(diagnostics) => emit_project_diagnostics(policy, diagnostics),
        None => emit_internal_incident(
            "project.locked-resolution",
            error.to_string(),
            Reproduction::new("ling project check"),
            policy,
        ),
    }
}

fn emit_project_diagnostics(policy: OutputPolicy, diagnostics: &[Diagnostic]) -> u8 {
    match policy.format() {
        OutputFormat::Human => emit_diagnostics(diagnostics, policy, EXIT_COMPILE_ERROR),
        OutputFormat::Json => {
            let values = match diagnostic_values(diagnostics) {
                Ok(values) => values,
                Err(status) => return status,
            };
            let report = serde_json::json!({
                "protocol": "ling.project.check/0.1",
                "status": "error",
                "diagnostics": values,
            });
            match serde_json::to_string(&report) {
                Ok(rendered) => {
                    eprintln!("{rendered}");
                    EXIT_COMPILE_ERROR
                }
                Err(error) => emit_internal_incident(
                    "project.error-json",
                    error.to_string(),
                    Reproduction::new("ling project check --format json"),
                    policy,
                ),
            }
        }
    }
}

fn emit_project_success(policy: OutputPolicy, graph: &PackageGraph) -> u8 {
    let Some(root_package) = graph.package(graph.root()) else {
        return emit_internal_incident(
            "project.root-package",
            "locked graph omitted its root package",
            Reproduction::new("ling project check"),
            policy,
        );
    };
    let module_count = graph
        .packages()
        .iter()
        .map(|package| package.modules().nodes().len())
        .sum::<usize>();
    let package_count = graph.packages().len();
    let version = root_package.identity().version().to_string();
    let package = root_package.identity().name().as_str();
    let entry = root_package.entry().as_str();
    match policy.format() {
        OutputFormat::Human => {
            if !policy.is_quiet() {
                let facts = format!(
                    "package={package}@{version} entry={entry} modules={module_count} packages={package_count}"
                );
                println!(
                    "{}",
                    policy.human_summary("项目检查通过", "project check passed", &facts)
                );
            }
            EXIT_SUCCESS
        }
        OutputFormat::Json => {
            let report = serde_json::json!({
                "protocol": "ling.project.check/0.1",
                "status": "ok",
                "package": {"name": package, "version": version},
                "entry": entry,
                "modules": module_count,
                "packages": package_count,
                "graph": graph.id().as_str(),
            });
            match serde_json::to_string(&report) {
                Ok(rendered) => match write_stdout(format!("{rendered}\n").as_bytes()) {
                    Ok(()) => EXIT_SUCCESS,
                    Err(error) => emit_host_io_failure("project.stdout", &error, policy),
                },
                Err(error) => emit_internal_incident(
                    "project.success-json",
                    error.to_string(),
                    Reproduction::new("ling project check --format json"),
                    policy,
                ),
            }
        }
    }
}

fn execute_format(
    policy: OutputPolicy,
    check: bool,
    path: PathBuf,
    stdin_name: Option<String>,
) -> u8 {
    let (source_name, bytes) = match read_format_input(&path, stdin_name.as_deref()) {
        Ok(input) => input,
        Err((source_name, diagnostics)) => {
            return emit_format_failure(policy, source_name, check, diagnostics);
        }
    };

    let source = match ling_source::SourceFile::from_bytes(
        ling_source::SourceId::new(0),
        source_name.clone(),
        bytes,
    ) {
        Ok(source) => source,
        Err(error) => {
            return emit_format_failure(
                policy,
                source_name.clone(),
                check,
                vec![format_source_error_diagnostic(&source_name, error)],
            );
        }
    };
    let parsed = ling_syntax::parse(&source);
    let mut diagnostics = parsed
        .lexical_errors()
        .iter()
        .map(|error| error.to_diagnostic(source.name()))
        .collect::<Vec<_>>();
    diagnostics.extend(
        parsed
            .parse_errors()
            .iter()
            .map(|error| error.to_diagnostic(source.name())),
    );
    if !diagnostics.is_empty() {
        return emit_format_failure(policy, source_name, check, diagnostics);
    }

    let document = match build_format_ir(&source, &parsed) {
        Ok(document) => document,
        Err(error) => {
            return emit_internal_incident(
                "format.ir",
                error.to_string(),
                Reproduction::new("ling fmt").with_input(source_name),
                policy,
            );
        }
    };
    let result = format_core_with_disposition(&document);
    if matches!(
        result.disposition(),
        FormatDisposition::OriginalInvalidSource | FormatDisposition::OriginalRejectedCandidate
    ) {
        return emit_format_failure(
            policy,
            source_name,
            check,
            vec![format_rejected_diagnostic()],
        );
    }

    let changed = result.text() != source.original_text();
    let disposition = if changed { "formatted" } else { "unchanged" };
    if policy.format() == OutputFormat::Json {
        return emit_format_report(
            source_name,
            check,
            changed,
            disposition,
            (!check).then(|| result.text().to_owned()),
            Vec::new(),
        );
    }
    if check {
        if changed {
            eprintln!(
                "{}",
                policy.human_text(
                    &format!("需要格式化：{source_name}"),
                    &format!("would reformat: {source_name}")
                )
            );
            EXIT_COMPILE_ERROR
        } else {
            EXIT_SUCCESS
        }
    } else {
        match write_stdout(result.text().as_bytes()) {
            Ok(()) => EXIT_SUCCESS,
            Err(error) => emit_host_io_failure("format.stdout", &error, policy),
        }
    }
}

fn read_format_input(
    path: &PathBuf,
    stdin_name: Option<&str>,
) -> Result<(String, Vec<u8>), (String, Vec<Diagnostic>)> {
    if path.as_os_str() == "-" {
        let Some(name) = stdin_name else {
            return Err((
                "<stdin>".to_owned(),
                vec![Diagnostic::new(
                    ling_diagnostics::codes::SOURCE_READ_FAILED,
                    ling_diagnostics::Severity::Error,
                    "格式化标准输入缺少逻辑文件名",
                    "formatter stdin requires a logical source name",
                )],
            ));
        };
        let mut bytes = Vec::new();
        if let Err(error) = std::io::stdin().read_to_end(&mut bytes) {
            return Err((
                name.to_owned(),
                vec![
                    Diagnostic::new(
                        ling_diagnostics::codes::SOURCE_READ_FAILED,
                        ling_diagnostics::Severity::Error,
                        "无法读取标准输入",
                        "failed to read standard input",
                    )
                    .with_fact("io_kind", format_stable_io_kind(error.kind())),
                ],
            ));
        }
        return Ok((name.to_owned(), bytes));
    }

    if stdin_name.is_some() {
        return Err((
            path.to_string_lossy().into_owned(),
            vec![Diagnostic::new(
                ling_diagnostics::codes::SOURCE_READ_FAILED,
                ling_diagnostics::Severity::Error,
                "格式化输入逻辑文件名使用错误",
                "formatter logical source name is only valid for stdin",
            )],
        ));
    }
    let source_name = path.to_string_lossy().into_owned();
    match std::fs::read(path) {
        Ok(bytes) => Ok((source_name, bytes)),
        Err(error) => Err((
            source_name.clone(),
            vec![
                Diagnostic::new(
                    ling_diagnostics::codes::SOURCE_READ_FAILED,
                    ling_diagnostics::Severity::Error,
                    format!("无法读取源码文件“{source_name}”"),
                    format!("failed to read source file `{source_name}`"),
                )
                .with_fact("io_kind", format_stable_io_kind(error.kind())),
            ],
        )),
    }
}

fn emit_format_failure(
    policy: OutputPolicy,
    source_name: String,
    check: bool,
    diagnostics: Vec<Diagnostic>,
) -> u8 {
    if policy.format() == OutputFormat::Json {
        return match diagnostic_values(&diagnostics) {
            Ok(values) => emit_format_report(source_name, check, false, "invalid", None, values),
            Err(status) => status,
        };
    }
    emit_compile_errors(&diagnostics, policy)
}

fn emit_format_report(
    source: String,
    check: bool,
    changed: bool,
    disposition: &str,
    text: Option<String>,
    diagnostics: Vec<serde_json::Value>,
) -> u8 {
    let mut report = serde_json::json!({
        "schema": "ling.format/0.1",
        "source": source,
        "check": check,
        "changed": changed,
        "disposition": disposition,
    });
    let object = report.as_object_mut().expect("format report is an object");
    if let Some(text) = text {
        object.insert("text".to_owned(), serde_json::Value::String(text));
    }
    if !diagnostics.is_empty() {
        object.insert(
            "diagnostics".to_owned(),
            serde_json::Value::Array(diagnostics),
        );
    }
    let mut bytes = match serde_json::to_vec(&report) {
        Ok(bytes) => bytes,
        Err(error) => {
            return emit_internal_incident(
                "format.report",
                error.to_string(),
                Reproduction::new("ling fmt --format json"),
                OutputPolicy::json(),
            );
        }
    };
    bytes.push(b'\n');
    match write_stdout(&bytes) {
        Ok(()) => {
            if disposition == "invalid" || (check && changed) {
                EXIT_COMPILE_ERROR
            } else {
                EXIT_SUCCESS
            }
        }
        Err(error) => emit_host_io_failure("format.stdout", &error, OutputPolicy::json()),
    }
}

fn format_rejected_diagnostic() -> Diagnostic {
    Diagnostic::new(
        ling_diagnostics::codes::INTERNAL_COMPILER_ERROR,
        ling_diagnostics::Severity::Error,
        "格式化候选文本未通过编译器验证",
        "formatter candidate did not pass compiler validation",
    )
}

const fn format_stable_io_kind(kind: std::io::ErrorKind) -> &'static str {
    match kind {
        std::io::ErrorKind::NotFound => "not_found",
        std::io::ErrorKind::PermissionDenied => "permission_denied",
        std::io::ErrorKind::InvalidData => "invalid_data",
        std::io::ErrorKind::Interrupted => "interrupted",
        _ => "other",
    }
}

fn format_source_error_diagnostic(path: &str, error: ling_source::SourceError) -> Diagnostic {
    match error {
        ling_source::SourceError::InvalidUtf8 {
            valid_up_to,
            error_len,
        } => {
            let end = valid_up_to.saturating_add(error_len.unwrap_or(1));
            Diagnostic::new(
                ling_diagnostics::codes::INVALID_UTF8,
                ling_diagnostics::Severity::Error,
                "源码不是有效的 UTF-8",
                "source is not valid UTF-8",
            )
            .with_primary_span(ling_diagnostics::DiagnosticSpan::at(
                path,
                u32::try_from(valid_up_to).unwrap_or(u32::MAX),
                u32::try_from(end).unwrap_or(u32::MAX),
            ))
            .with_fact(
                "valid_up_to",
                u64::try_from(valid_up_to).unwrap_or(u64::MAX),
            )
        }
        ling_source::SourceError::MisplacedByteOrderMark { byte_offset } => Diagnostic::new(
            ling_diagnostics::codes::MISPLACED_BOM,
            ling_diagnostics::Severity::Error,
            "UTF-8 BOM 只能出现在文件开头",
            "the UTF-8 byte-order mark is only allowed at the start of a file",
        )
        .with_primary_span(ling_diagnostics::DiagnosticSpan::at(
            path,
            u32::try_from(byte_offset).unwrap_or(u32::MAX),
            u32::try_from(byte_offset.saturating_add(3)).unwrap_or(u32::MAX),
        )),
        ling_source::SourceError::TooLarge { byte_len } => Diagnostic::new(
            ling_diagnostics::codes::SOURCE_TOO_LARGE,
            ling_diagnostics::Severity::Error,
            "源码文件超过当前实现支持的大小",
            "source file exceeds the size supported by this implementation",
        )
        .with_fact("byte_len", u64::try_from(byte_len).unwrap_or(u64::MAX))
        .with_fact("maximum_byte_len", u64::from(u32::MAX)),
    }
}

fn execute_repl(policy: OutputPolicy, capabilities: Vec<String>) -> u8 {
    let stdin = std::io::stdin();
    let interactive = stdin.is_terminal() && std::io::stdout().is_terminal();
    let mut session = Session::new(capabilities);
    let mut buffer = String::new();
    let mut had_compile_failure = false;
    let mut had_runtime_failure = false;

    let result = if interactive {
        execute_interactive_repl(
            &mut session,
            &mut buffer,
            policy,
            &mut had_compile_failure,
            &mut had_runtime_failure,
        )
    } else {
        execute_script_repl(
            &mut stdin.lock(),
            &mut session,
            &mut buffer,
            policy,
            &mut had_compile_failure,
            &mut had_runtime_failure,
        )
    };
    if let Err(status) = result {
        return status;
    }

    if had_runtime_failure {
        EXIT_RUNTIME_FAULT
    } else if had_compile_failure {
        EXIT_COMPILE_ERROR
    } else {
        EXIT_SUCCESS
    }
}

fn execute_interactive_repl(
    session: &mut Session,
    buffer: &mut String,
    policy: OutputPolicy,
    had_compile_failure: &mut bool,
    had_runtime_failure: &mut bool,
) -> Result<(), u8> {
    let mut editor = rustyline::DefaultEditor::new()
        .map_err(|_| emit_host_failure("repl.terminal-init", "other", policy))?;
    loop {
        let prompt = if policy.format() == OutputFormat::Json {
            ""
        } else if buffer.is_empty() {
            "ling> "
        } else {
            "....> "
        };
        match editor.readline(prompt) {
            Ok(line) => handle_repl_line(
                session,
                buffer,
                &line,
                policy,
                had_compile_failure,
                had_runtime_failure,
            )?,
            Err(rustyline::error::ReadlineError::Interrupted) => {
                cancel_repl_submission(buffer);
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                return process_pending_submission(
                    session,
                    buffer,
                    policy,
                    had_compile_failure,
                    had_runtime_failure,
                );
            }
            Err(rustyline::error::ReadlineError::Io(error)) => {
                return Err(emit_host_io_failure("repl.terminal-input", &error, policy));
            }
            Err(_) => return Err(emit_host_failure("repl.terminal-input", "other", policy)),
        }
    }
}

fn execute_script_repl(
    input: &mut impl BufRead,
    session: &mut Session,
    buffer: &mut String,
    policy: OutputPolicy,
    had_compile_failure: &mut bool,
    had_runtime_failure: &mut bool,
) -> Result<(), u8> {
    let mut line = String::new();

    loop {
        line.clear();
        match input.read_line(&mut line) {
            Ok(0) => {
                return process_pending_submission(
                    session,
                    buffer,
                    policy,
                    had_compile_failure,
                    had_runtime_failure,
                );
            }
            Ok(_) => handle_repl_line(
                session,
                buffer,
                &line,
                policy,
                had_compile_failure,
                had_runtime_failure,
            )?,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
                cancel_repl_submission(buffer);
            }
            Err(error) => {
                return Err(emit_host_io_failure("repl.script-input", &error, policy));
            }
        }
    }
}

fn handle_repl_line(
    session: &mut Session,
    buffer: &mut String,
    line: &str,
    policy: OutputPolicy,
    had_compile_failure: &mut bool,
    had_runtime_failure: &mut bool,
) -> Result<(), u8> {
    if line.trim().is_empty() && !buffer.trim().is_empty() && delimiters_closed(buffer) {
        process_submission(
            session,
            buffer,
            policy,
            had_compile_failure,
            had_runtime_failure,
        )?;
        buffer.clear();
    } else {
        buffer.push_str(line);
        if !line.ends_with('\n') {
            buffer.push('\n');
        }
    }
    Ok(())
}

fn process_pending_submission(
    session: &mut Session,
    buffer: &str,
    policy: OutputPolicy,
    had_compile_failure: &mut bool,
    had_runtime_failure: &mut bool,
) -> Result<(), u8> {
    if buffer.trim().is_empty() {
        Ok(())
    } else {
        process_submission(
            session,
            buffer,
            policy,
            had_compile_failure,
            had_runtime_failure,
        )
    }
}

fn cancel_repl_submission(buffer: &mut String) {
    buffer.clear();
}

fn process_submission(
    session: &mut Session,
    source: &str,
    policy: OutputPolicy,
    had_compile_failure: &mut bool,
    had_runtime_failure: &mut bool,
) -> Result<(), u8> {
    let mut console = ling_eval::MemoryConsole::default();
    let outcome = session.submit(source.trim_end(), &mut console);
    if !console.output().is_empty() {
        match policy.format() {
            OutputFormat::Human => {
                write_stdout(console.output().as_bytes())
                    .map_err(|error| emit_host_io_failure("repl.console", &error, policy))?;
            }
            OutputFormat::Json => {
                let submission = match &outcome {
                    Ok(success) => success.submission,
                    Err(failure) => failure.submission(),
                };
                emit_repl_json(&serde_json::json!({
                    "schema": "ling.repl/0.1",
                    "status": "console",
                    "committed": false,
                    "submission": submission,
                    "console": console.output(),
                }))
                .map_err(|error| emit_host_io_failure("repl.console-json", &error, policy))?;
            }
        }
    }

    match outcome {
        Ok(success) => emit_repl_success(&success, policy)?,
        Err(SubmissionFailure::Compile {
            submission,
            diagnostics,
        }) => {
            *had_compile_failure = true;
            emit_repl_failure(submission, &diagnostics, policy)?;
        }
        Err(SubmissionFailure::Runtime { submission, fault }) => {
            *had_runtime_failure = true;
            emit_repl_runtime_failure(submission, &fault, policy)?;
        }
        Err(SubmissionFailure::Internal {
            submission,
            message,
        }) => {
            return Err(emit_internal_incident(
                "repl.submission",
                message,
                Reproduction::new("ling repl")
                    .with_submission(submission)
                    .with_source(source),
                policy,
            ));
        }
        Err(SubmissionFailure::SnapshotMismatch {
            submission,
            message,
        }) => {
            return Err(emit_repl_snapshot_mismatch(submission, &message, policy));
        }
    }
    Ok(())
}

fn emit_repl_success(success: &SubmissionSuccess, policy: OutputPolicy) -> Result<(), u8> {
    match policy.format() {
        OutputFormat::Human => {
            if !success.warnings.is_empty() {
                let _ = emit_diagnostics(&success.warnings, policy, EXIT_SUCCESS);
            }
            match success.kind {
                SubmissionKind::Expression => {
                    if let (Some(value), Some(type_name)) = (&success.value, &success.type_name) {
                        write_stdout(format!("{value} : {type_name}\n").as_bytes())
                            .map_err(|error| emit_host_io_failure("repl.result", &error, policy))?;
                    }
                }
                SubmissionKind::ValueDeclaration => {
                    if let (Some(name), Some(type_name)) = (&success.name, &success.type_name) {
                        write_stdout(format!("{name} : {type_name}\n").as_bytes())
                            .map_err(|error| emit_host_io_failure("repl.result", &error, policy))?;
                    }
                }
                SubmissionKind::TypeDeclaration => {
                    if let Some(name) = &success.name {
                        write_stdout(format!("type {name}\n").as_bytes())
                            .map_err(|error| emit_host_io_failure("repl.result", &error, policy))?;
                    }
                }
            }
        }
        OutputFormat::Json => {
            let mut event = serde_json::json!({
                "schema": "ling.repl/0.1",
                "status": "ok",
                "committed": success.committed,
                "submission": success.submission,
                "effects": success.effects,
                "capabilities": success.capabilities,
            });
            let object = event.as_object_mut().expect("JSON object literal");
            if let Some(name) = &success.name {
                object.insert("name".to_owned(), serde_json::json!(name));
            }
            if let Some(type_name) = &success.type_name {
                object.insert("type".to_owned(), serde_json::json!(type_name));
            }
            if let Some(value) = &success.value {
                object.insert("value".to_owned(), serde_json::json!(value));
            }
            if let Some(definition_id) = &success.definition_id {
                object.insert("definition_id".to_owned(), serde_json::json!(definition_id));
            }
            if !success.warnings.is_empty() {
                object.insert(
                    "diagnostics".to_owned(),
                    serde_json::Value::Array(diagnostic_values(&success.warnings)?),
                );
            }
            emit_repl_json(&event)
                .map_err(|error| emit_host_io_failure("repl.result-json", &error, policy))?;
        }
    }
    Ok(())
}

fn emit_repl_failure(
    submission: u64,
    diagnostics: &[Diagnostic],
    policy: OutputPolicy,
) -> Result<(), u8> {
    match policy.format() {
        OutputFormat::Human => {
            let _ = emit_compile_errors(diagnostics, policy);
        }
        OutputFormat::Json => {
            emit_repl_json(&serde_json::json!({
                "schema": "ling.repl/0.1",
                "status": "compile_error",
                "committed": false,
                "submission": submission,
                "diagnostics": diagnostic_values(diagnostics)?,
            }))
            .map_err(|error| emit_host_io_failure("repl.compile-error-json", &error, policy))?;
        }
    }
    Ok(())
}

fn emit_repl_runtime_failure(
    submission: u64,
    fault: &ling_eval::RuntimeFault,
    policy: OutputPolicy,
) -> Result<(), u8> {
    let diagnostic = fault.to_diagnostic().with_fact("committed", false);
    match policy.format() {
        OutputFormat::Human => {
            let _ = emit_diagnostics(&[diagnostic], policy, EXIT_RUNTIME_FAULT);
        }
        OutputFormat::Json => {
            emit_repl_json(&serde_json::json!({
                "schema": "ling.repl/0.1",
                "status": "runtime_error",
                "committed": false,
                "submission": submission,
                "diagnostics": diagnostic_values(&[diagnostic])?,
            }))
            .map_err(|error| emit_host_io_failure("repl.runtime-error-json", &error, policy))?;
        }
    }
    Ok(())
}

fn emit_repl_snapshot_mismatch(submission: u64, message: &str, policy: OutputPolicy) -> u8 {
    let diagnostic = snapshot_mismatch_diagnostic(message).with_fact("committed", false);
    match policy.format() {
        OutputFormat::Human => emit_diagnostics(&[diagnostic], policy, EXIT_SNAPSHOT_MISMATCH),
        OutputFormat::Json => {
            let event = serde_json::json!({
                "schema": "ling.repl/0.1",
                "status": "snapshot_mismatch",
                "committed": false,
                "submission": submission,
                "diagnostics": match diagnostic_values(&[diagnostic]) {
                    Ok(values) => values,
                    Err(status) => return status,
                },
            });
            match emit_repl_json(&event) {
                Ok(()) => EXIT_SNAPSHOT_MISMATCH,
                Err(error) => emit_host_io_failure("repl.snapshot-mismatch-json", &error, policy),
            }
        }
    }
}

fn diagnostic_values(diagnostics: &[Diagnostic]) -> Result<Vec<serde_json::Value>, u8> {
    diagnostics
        .iter()
        .map(|diagnostic| {
            diagnostic
                .render_json()
                .map_err(|error| {
                    emit_internal_incident(
                        "diagnostic.render",
                        error.to_string(),
                        Reproduction::new("ling repl --format json"),
                        OutputPolicy::json(),
                    )
                })
                .and_then(|rendered| {
                    serde_json::from_str(&rendered).map_err(|error| {
                        emit_internal_incident(
                            "diagnostic.parse-rendered-json",
                            error.to_string(),
                            Reproduction::new("ling repl --format json"),
                            OutputPolicy::json(),
                        )
                    })
                })
        })
        .collect()
}

fn emit_repl_json(value: &serde_json::Value) -> std::io::Result<()> {
    let mut rendered = serde_json::to_vec(value).map_err(std::io::Error::other)?;
    rendered.push(b'\n');
    write_stdout(&rendered)
}

fn write_stdout(bytes: &[u8]) -> std::io::Result<()> {
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(bytes)
}

fn delimiters_closed(source: &str) -> bool {
    let mut delimiters = Vec::new();
    let mut characters = source.chars().peekable();
    let mut in_text = false;
    let mut escaped = false;
    let mut line_comment = false;
    let mut block_depth = 0_u32;
    while let Some(character) = characters.next() {
        if line_comment {
            if character == '\n' {
                line_comment = false;
            }
            continue;
        }
        if block_depth > 0 {
            if character == '/' && characters.peek() == Some(&'*') {
                characters.next();
                block_depth = block_depth.saturating_add(1);
            } else if character == '*' && characters.peek() == Some(&'/') {
                characters.next();
                block_depth = block_depth.saturating_sub(1);
            }
            continue;
        }
        if in_text {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_text = false;
            }
            continue;
        }
        match character {
            '"' => in_text = true,
            '/' if characters.peek() == Some(&'/') => {
                characters.next();
                line_comment = true;
            }
            '/' if characters.peek() == Some(&'*') => {
                characters.next();
                block_depth = 1;
            }
            '(' | '[' | '{' => delimiters.push(character),
            ')' | ']' | '}' => {
                let expected = match character {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    _ => unreachable!("closing delimiter matched above"),
                };
                if delimiters.pop() != Some(expected) {
                    return true;
                }
            }
            _ => {}
        }
    }
    delimiters.is_empty() && !in_text && block_depth == 0
}

struct StdoutConsole;

impl Console for StdoutConsole {
    fn write(&mut self, text: &str) -> Result<(), HostError> {
        std::io::stdout()
            .lock()
            .write_all(text.as_bytes())
            .map_err(|error| HostError::new(host_error_category(error.kind())))
    }
}

const fn host_error_category(kind: std::io::ErrorKind) -> HostErrorCategory {
    match kind {
        std::io::ErrorKind::BrokenPipe => HostErrorCategory::BrokenPipe,
        std::io::ErrorKind::PermissionDenied => HostErrorCategory::PermissionDenied,
        std::io::ErrorKind::Interrupted => HostErrorCategory::Interrupted,
        _ => HostErrorCategory::Other,
    }
}

fn emit_compile_error(diagnostic: Diagnostic, policy: OutputPolicy) -> u8 {
    emit_compile_errors(&[diagnostic], policy)
}

fn emit_compile_errors(diagnostics: &[Diagnostic], policy: OutputPolicy) -> u8 {
    emit_diagnostics(diagnostics, policy, EXIT_COMPILE_ERROR)
}

fn emit_snapshot_mismatch(message: &str, policy: OutputPolicy) -> u8 {
    emit_diagnostics(
        &[snapshot_mismatch_diagnostic(message)],
        policy,
        EXIT_SNAPSHOT_MISMATCH,
    )
}

fn snapshot_mismatch_diagnostic(message: &str) -> Diagnostic {
    Diagnostic::new(
        ling_diagnostics::codes::SEMANTIC_SNAPSHOT_MISMATCH,
        ling_diagnostics::Severity::Error,
        "Semantic Graph 快照验证失败",
        "Semantic Graph snapshot validation failed",
    )
    .with_fact("detail", message)
}

fn emit_host_io_failure(operation: &str, error: &std::io::Error, policy: OutputPolicy) -> u8 {
    let category = host_error_category(error.kind()).name();
    emit_host_failure(operation, category, policy)
}

fn emit_host_failure(operation: &str, category: &str, policy: OutputPolicy) -> u8 {
    let diagnostic = Diagnostic::new(
        ling_diagnostics::codes::RUNTIME_FAULT,
        ling_diagnostics::Severity::Error,
        format!("宿主输出操作“{operation}”失败"),
        format!("host output operation `{operation}` failed"),
    )
    .with_fact("category", category)
    .with_fact("operation", operation);
    emit_diagnostics(&[diagnostic], policy, EXIT_RUNTIME_FAULT)
}

fn emit_diagnostics(diagnostics: &[Diagnostic], policy: OutputPolicy, exit_code: u8) -> u8 {
    for diagnostic in diagnostics {
        let rendered = match policy.format() {
            OutputFormat::Human => Ok(policy.render_diagnostic(diagnostic)),
            OutputFormat::Json => diagnostic.render_json().map_err(|error| error.to_string()),
        };
        match rendered {
            Ok(rendered) => eprintln!("{rendered}"),
            Err(error) => {
                return emit_internal_incident(
                    "diagnostic.render",
                    error,
                    Reproduction::new("ling diagnostics"),
                    policy,
                );
            }
        }
    }
    exit_code
}

fn emit_internal_incident(
    stage: &str,
    detail: impl Into<String>,
    reproduction: Reproduction,
    policy: OutputPolicy,
) -> u8 {
    let incident = InternalIncident::capture(stage, detail, reproduction);
    let diagnostic = incident.diagnostic();
    let rendered = match policy.format() {
        OutputFormat::Human => Ok(policy.render_diagnostic(&diagnostic)),
        OutputFormat::Json => diagnostic.render_json().map_err(|error| error.to_string()),
    };
    match rendered {
        Ok(rendered) => eprintln!("{rendered}"),
        Err(_) => eprintln!(
            "error[L-INTERNAL-0001]: internal compiler error; incident ID: {}",
            incident.id()
        ),
    }
    EXIT_INTERNAL_ERROR
}

fn invalid_usage(message: &str) -> u8 {
    eprintln!("error: {message}\n\n{}", usage());
    EXIT_INVALID_USAGE
}

fn usage() -> String {
    format!(
        "Usage:\n  {CLI_NAME} --version\n  {CLI_NAME} <run|check|semantic|audit> [OUTPUT] <file>\n  {CLI_NAME} query --symbol name [OUTPUT] <file>\n  {CLI_NAME} patch [OUTPUT] <transaction.json> <file>\n  {CLI_NAME} test [OUTPUT] <file-or-directory>\n  {CLI_NAME} <run|check|test> --manifest-path path --locked --offline [OUTPUT]\n  {CLI_NAME} build --manifest-path path --locked --offline --profile explore --target semantic --output path [OUTPUT]\n  {CLI_NAME} fmt [--check] [--stdin-name name] [OUTPUT] <file|->\n  {CLI_NAME} init [--name package] [--display-name text] [OUTPUT] <directory>\n  {CLI_NAME} project check --manifest-path path --locked [OUTPUT]\n  {CLI_NAME} repl [--capability Console.Write] [OUTPUT]\n  {CLI_NAME} lsp --stdio\n  {CLI_NAME} completion <bash|zsh|fish|powershell>\n\nOUTPUT:\n  [--format human|json] [--language bilingual|zh-CN|en]\n  [--color auto|always|never] [--quiet|--verbose]"
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Options {
    command: Command,
    policy: OutputPolicy,
    path: Option<PathBuf>,
    transaction_path: Option<PathBuf>,
    symbol: Option<String>,
    manifest_path: Option<PathBuf>,
    capabilities: Vec<String>,
    check: bool,
    locked: bool,
    offline: bool,
    profile: Option<String>,
    target: Option<String>,
    output: Option<PathBuf>,
    stdin_name: Option<String>,
    stdio: bool,
    init_name: Option<String>,
    init_display_name: Option<String>,
}

impl Options {
    fn parse(command: Command, arguments: &[OsString]) -> Result<Self, String> {
        let mut format = OutputFormat::Human;
        let mut format_seen = false;
        let mut language = HumanLanguage::Bilingual;
        let mut language_seen = false;
        let mut color = ColorChoice::Auto;
        let mut color_seen = false;
        let mut verbosity = Verbosity::Normal;
        let mut path = None;
        let mut transaction_path = None;
        let mut symbol = None;
        let mut manifest_path = None;
        let mut capabilities = Vec::new();
        let mut check = false;
        let mut locked = false;
        let mut offline = false;
        let mut profile = None;
        let mut target = None;
        let mut output = None;
        let mut stdin_name = None;
        let mut stdio = false;
        let mut init_name = None;
        let mut init_display_name = None;
        let mut index = 0;

        while index < arguments.len() {
            let argument = &arguments[index];
            if argument == "--format" {
                if command == Command::Lsp {
                    return Err(
                        "`lsp --stdio` does not accept `--format`; stdout is protocol-only"
                            .to_owned(),
                    );
                }
                if format_seen {
                    return Err("only one `--format` may be provided".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--format` requires `human` or `json`".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the output format must be valid Unicode".to_owned())?;
                format = OutputFormat::parse(value)
                    .ok_or_else(|| format!("unsupported output format `{value}`"))?;
                format_seen = true;
                index += 2;
                continue;
            }

            if argument == "--language" {
                if command == Command::Lsp {
                    return Err(
                        "`lsp --stdio` does not accept `--language`; stdout is protocol-only"
                            .to_owned(),
                    );
                }
                if language_seen {
                    return Err("only one `--language` may be provided".to_owned());
                }
                let value = arguments.get(index + 1).ok_or_else(|| {
                    "`--language` requires `bilingual`, `zh-CN`, or `en`".to_owned()
                })?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the output language must be valid Unicode".to_owned())?;
                language = HumanLanguage::parse(value)
                    .ok_or_else(|| format!("unsupported output language `{value}`"))?;
                language_seen = true;
                index += 2;
                continue;
            }

            if argument == "--color" {
                if command == Command::Lsp {
                    return Err(
                        "`lsp --stdio` does not accept `--color`; stdout is protocol-only"
                            .to_owned(),
                    );
                }
                if color_seen {
                    return Err("only one `--color` may be provided".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--color` requires `auto`, `always`, or `never`".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the color choice must be valid Unicode".to_owned())?;
                color = ColorChoice::parse(value)
                    .ok_or_else(|| format!("unsupported color choice `{value}`"))?;
                color_seen = true;
                index += 2;
                continue;
            }

            if argument == "--quiet" || argument == "--verbose" {
                if command == Command::Lsp {
                    return Err(format!(
                        "`lsp --stdio` does not accept `{}`; stdout is protocol-only",
                        argument.to_string_lossy()
                    ));
                }
                let requested = if argument == "--quiet" {
                    Verbosity::Quiet
                } else {
                    Verbosity::Verbose
                };
                if verbosity != Verbosity::Normal {
                    return Err("only one of `--quiet` or `--verbose` may be provided".to_owned());
                }
                verbosity = requested;
                index += 1;
                continue;
            }

            if argument == "--capability" {
                if command != Command::Repl {
                    return Err("`--capability` is only valid with `repl`".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--capability` requires a capability name".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the capability name must be valid Unicode".to_owned())?;
                if value != "Console.Write" {
                    return Err(format!("unsupported REPL capability `{value}`"));
                }
                capabilities.push(value.to_owned());
                index += 2;
                continue;
            }

            if argument == "--symbol" {
                if command != Command::Query {
                    return Err("`--symbol` is only valid with `query`".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--symbol` requires an identifier".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the query symbol must be valid Unicode".to_owned())?;
                if symbol.replace(value.to_owned()).is_some() {
                    return Err("only one `--symbol` may be provided".to_owned());
                }
                index += 2;
                continue;
            }

            if argument == "--check" {
                if command != Command::Format {
                    return Err("`--check` is only valid with `fmt`".to_owned());
                }
                check = true;
                index += 1;
                continue;
            }

            if argument == "--stdin-name" {
                if command != Command::Format {
                    return Err("`--stdin-name` is only valid with `fmt`".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--stdin-name` requires a logical `.ling` name".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the stdin logical name must be valid Unicode".to_owned())?;
                if !valid_stdin_name(value) {
                    return Err(
                        "`--stdin-name` must be a relative UTF-8 path ending in `.ling`".to_owned(),
                    );
                }
                if stdin_name.replace(value.to_owned()).is_some() {
                    return Err("only one `--stdin-name` may be provided".to_owned());
                }
                index += 2;
                continue;
            }

            if argument == "--stdio" {
                if command != Command::Lsp {
                    return Err("`--stdio` is only valid with `lsp`".to_owned());
                }
                if stdio {
                    return Err("only one `--stdio` may be provided".to_owned());
                }
                stdio = true;
                index += 1;
                continue;
            }

            if argument == "--name" {
                if command != Command::Init {
                    return Err("`--name` is only valid with `init`".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--name` requires a package name".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the package name must be valid Unicode".to_owned())?;
                if init_name.replace(value.to_owned()).is_some() {
                    return Err("only one `--name` may be provided".to_owned());
                }
                index += 2;
                continue;
            }

            if argument == "--display-name" {
                if command != Command::Init {
                    return Err("`--display-name` is only valid with `init`".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--display-name` requires text".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the display name must be valid Unicode".to_owned())?;
                if init_display_name.replace(value.to_owned()).is_some() {
                    return Err("only one `--display-name` may be provided".to_owned());
                }
                index += 2;
                continue;
            }

            if argument == "--manifest-path" {
                if !matches!(
                    command,
                    Command::Check
                        | Command::Run
                        | Command::Test
                        | Command::Build
                        | Command::ProjectCheck
                ) {
                    return Err(
                        "`--manifest-path` is only valid with project-capable commands".to_owned(),
                    );
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--manifest-path` requires a path".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the manifest path must be valid Unicode".to_owned())?;
                let candidate = PathBuf::from(value);
                if candidate == Path::new("-")
                    || candidate.file_name().and_then(|name| name.to_str()) != Some("ling.toml")
                {
                    return Err("`--manifest-path` must name a `ling.toml` file".to_owned());
                }
                if manifest_path.replace(candidate).is_some() {
                    return Err("only one `--manifest-path` may be provided".to_owned());
                }
                index += 2;
                continue;
            }

            if argument == "--locked" {
                if !matches!(
                    command,
                    Command::Check
                        | Command::Run
                        | Command::Test
                        | Command::Build
                        | Command::ProjectCheck
                ) {
                    return Err("`--locked` is only valid with project-capable commands".to_owned());
                }
                if locked {
                    return Err("only one `--locked` may be provided".to_owned());
                }
                locked = true;
                index += 1;
                continue;
            }

            if argument == "--offline" {
                if !matches!(
                    command,
                    Command::Check | Command::Run | Command::Test | Command::Build
                ) {
                    return Err(
                        "`--offline` is only valid with semantic project commands".to_owned()
                    );
                }
                if offline {
                    return Err("only one `--offline` may be provided".to_owned());
                }
                offline = true;
                index += 1;
                continue;
            }

            if argument == "--profile" {
                if command != Command::Build {
                    return Err("`--profile` is only valid with `build`".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--profile` requires `explore`".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the build profile must be valid Unicode".to_owned())?;
                if value != BUILD_PROFILE {
                    return Err(format!("unsupported project build profile `{value}`"));
                }
                if profile.replace(value.to_owned()).is_some() {
                    return Err("only one `--profile` may be provided".to_owned());
                }
                index += 2;
                continue;
            }

            if argument == "--target" {
                if command != Command::Build {
                    return Err("`--target` is only valid with `build`".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--target` requires `semantic`".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the build target must be valid Unicode".to_owned())?;
                if value != BUILD_TARGET {
                    return Err(format!("unsupported project build target `{value}`"));
                }
                if target.replace(value.to_owned()).is_some() {
                    return Err("only one `--target` may be provided".to_owned());
                }
                index += 2;
                continue;
            }

            if argument == "--output" {
                if command != Command::Build {
                    return Err("`--output` is only valid with `build`".to_owned());
                }
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--output` requires a path".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the build output path must be valid Unicode".to_owned())?;
                if value.is_empty() || value == "-" {
                    return Err("`--output` must name a filesystem path".to_owned());
                }
                if output.replace(PathBuf::from(value)).is_some() {
                    return Err("only one `--output` may be provided".to_owned());
                }
                index += 2;
                continue;
            }

            if argument.to_string_lossy().starts_with('-') && argument != "-" {
                return Err(format!("unknown option `{}`", argument.to_string_lossy()));
            }
            if matches!(command, Command::ProjectCheck | Command::Build) {
                return Err(format!("`{command}` does not accept a positional path"));
            }
            if command == Command::Patch && transaction_path.is_none() {
                transaction_path = Some(PathBuf::from(argument));
                index += 1;
                continue;
            }
            if path.replace(PathBuf::from(argument)).is_some() {
                return Err(if command == Command::Init {
                    "only one init destination may be provided".to_owned()
                } else {
                    "only one source file may be provided".to_owned()
                });
            }
            index += 1;
        }

        let semantic_project = matches!(
            command,
            Command::Check | Command::Run | Command::Test | Command::Build
        ) && manifest_path.is_some();
        if semantic_project && path.is_some() {
            return Err("project mode does not accept a positional source path".to_owned());
        }
        if matches!(
            command,
            Command::Repl | Command::Lsp | Command::ProjectCheck | Command::Build
        ) && path.is_some()
        {
            return Err(format!("`{command}` does not accept a source file"));
        }
        if command == Command::Query && symbol.is_none() {
            return Err("`query` requires `--symbol name`".to_owned());
        }
        if command != Command::Query && symbol.is_some() {
            return Err("`--symbol` is only valid with `query`".to_owned());
        }
        if command == Command::Patch {
            if transaction_path.is_none() || path.is_none() {
                return Err("`patch` requires a transaction JSON file and a source file".to_owned());
            }
            if transaction_path
                .as_deref()
                .and_then(Path::extension)
                .and_then(|extension| extension.to_str())
                != Some("json")
            {
                return Err("`patch` transaction input must end in `.json`".to_owned());
            }
        } else if transaction_path.is_some() {
            return Err("a transaction path is only valid with `patch`".to_owned());
        }
        if matches!(command, Command::Query | Command::Patch)
            && path
                .as_deref()
                .and_then(Path::extension)
                .and_then(|extension| extension.to_str())
                != Some("ling")
        {
            return Err(format!("`{command}` source input must end in `.ling`"));
        }
        if command != Command::Repl
            && command != Command::Lsp
            && command != Command::ProjectCheck
            && command != Command::Build
            && !semantic_project
            && path.is_none()
        {
            return Err(if command == Command::Init {
                "`init` requires a destination directory".to_owned()
            } else {
                format!("`{command}` requires a source file")
            });
        }
        if command == Command::Lsp && !stdio {
            return Err("`lsp` requires `--stdio`".to_owned());
        }
        if command != Command::Lsp && stdio {
            return Err("`--stdio` is only valid with `lsp`".to_owned());
        }
        if command == Command::Format {
            let is_stdin = path.as_deref().is_some_and(|value| value == Path::new("-"));
            if is_stdin && stdin_name.is_none() {
                return Err("`fmt -` requires `--stdin-name name`".to_owned());
            }
            if !is_stdin && stdin_name.is_some() {
                return Err("`--stdin-name` is valid only with `fmt -`".to_owned());
            }
            if !is_stdin
                && path
                    .as_deref()
                    .and_then(Path::extension)
                    .and_then(|extension| extension.to_str())
                    != Some("ling")
            {
                return Err("`fmt` input must be a `.ling` file or `-`".to_owned());
            }
        } else if check || stdin_name.is_some() {
            return Err("formatter-only options require `fmt`".to_owned());
        }
        if command == Command::ProjectCheck {
            if manifest_path.is_none() {
                return Err("`project check` requires `--manifest-path path`".to_owned());
            }
            if !locked {
                return Err("`project check` requires `--locked`".to_owned());
            }
        } else if semantic_project {
            if !locked {
                return Err(format!("project `{command}` requires `--locked`"));
            }
            if !offline {
                return Err(format!("project `{command}` requires `--offline`"));
            }
            if command == Command::Build {
                if profile.as_deref() != Some(BUILD_PROFILE) {
                    return Err("`build` requires `--profile explore`".to_owned());
                }
                if target.as_deref() != Some(BUILD_TARGET) {
                    return Err("`build` requires `--target semantic`".to_owned());
                }
                if output.is_none() {
                    return Err("`build` requires `--output path`".to_owned());
                }
            }
        } else if manifest_path.is_some() || locked || offline {
            return Err("project options require an explicit `--manifest-path`".to_owned());
        }
        if command == Command::Build && !semantic_project {
            return Err("`build` requires `--manifest-path path`".to_owned());
        }
        if command != Command::Build && (profile.is_some() || target.is_some() || output.is_some())
        {
            return Err("build-only options require `build`".to_owned());
        }

        if command != Command::Init && (init_name.is_some() || init_display_name.is_some()) {
            return Err("init metadata options require `init`".to_owned());
        }
        if command == Command::Init && path.as_deref() == Some(Path::new("-")) {
            return Err("`init` requires a destination directory, not `-`".to_owned());
        }
        if format == OutputFormat::Json && verbosity != Verbosity::Normal {
            return Err("`--quiet` and `--verbose` are not valid with `--format json`".to_owned());
        }
        if format == OutputFormat::Json && color_seen && color != ColorChoice::Never {
            return Err("`--format json` accepts only an explicit `--color never`".to_owned());
        }

        Ok(Self {
            command,
            policy: OutputPolicy::new(format, language, color, verbosity),
            path,
            transaction_path,
            symbol,
            manifest_path,
            capabilities,
            check,
            locked,
            offline,
            profile,
            target,
            output,
            stdin_name,
            stdio,
            init_name,
            init_display_name,
        })
    }
}

fn valid_stdin_name(value: &str) -> bool {
    if value.is_empty() || !value.ends_with(".ling") || value.contains('\\') || value.contains('\0')
    {
        return false;
    }
    let path = Path::new(value);
    !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, std::path::Component::Normal(_)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_format_before_or_after_the_path() {
        let before = Options::parse(
            Command::Check,
            &["--format".into(), "json".into(), "main.ling".into()],
        )
        .unwrap();
        let after = Options::parse(
            Command::Check,
            &["main.ling".into(), "--format".into(), "json".into()],
        )
        .unwrap();

        assert_eq!(before, after);
        assert_eq!(before.policy.format(), OutputFormat::Json);
        assert_eq!(
            before.path.as_deref(),
            Some(std::path::Path::new("main.ling"))
        );
    }

    #[test]
    fn rejects_missing_paths() {
        assert_eq!(
            Options::parse(Command::Check, &[]).unwrap_err(),
            "`check` requires a source file"
        );
    }

    #[test]
    fn parses_formatter_file_and_stdin_inputs() {
        let file = Options::parse(
            Command::Format,
            &[
                "--check".into(),
                "--format".into(),
                "json".into(),
                "main.ling".into(),
            ],
        )
        .unwrap();
        assert!(file.check);
        assert_eq!(file.policy.format(), OutputFormat::Json);
        assert_eq!(file.path.as_deref(), Some(Path::new("main.ling")));
        assert!(file.stdin_name.is_none());

        let stdin = Options::parse(
            Command::Format,
            &["-".into(), "--stdin-name".into(), "stdin/main.ling".into()],
        )
        .unwrap();
        assert_eq!(stdin.path.as_deref(), Some(Path::new("-")));
        assert_eq!(stdin.stdin_name.as_deref(), Some("stdin/main.ling"));
    }

    #[test]
    fn rejects_invalid_formatter_inputs() {
        assert_eq!(
            Options::parse(Command::Format, &["-".into()]).unwrap_err(),
            "`fmt -` requires `--stdin-name name`"
        );
        assert_eq!(
            Options::parse(Command::Format, &["main.txt".into()]).unwrap_err(),
            "`fmt` input must be a `.ling` file or `-`"
        );
        assert!(!valid_stdin_name("../main.ling"));
        assert!(!valid_stdin_name("main.txt"));
    }

    #[test]
    fn parses_only_the_stdio_lsp_launcher() {
        let options = Options::parse(Command::Lsp, &["--stdio".into()]).unwrap();
        assert!(options.stdio);
        assert_eq!(options.path, None);
        assert_eq!(
            Options::parse(Command::Lsp, &[]).unwrap_err(),
            "`lsp` requires `--stdio`"
        );
        assert_eq!(
            Options::parse(Command::Lsp, &["--format".into(), "human".into()]).unwrap_err(),
            "`lsp --stdio` does not accept `--format`; stdout is protocol-only"
        );
        for (argument, error) in [
            (
                vec!["--language".into(), "en".into()],
                "`lsp --stdio` does not accept `--language`; stdout is protocol-only",
            ),
            (
                vec!["--color".into(), "never".into()],
                "`lsp --stdio` does not accept `--color`; stdout is protocol-only",
            ),
            (
                vec!["--quiet".into()],
                "`lsp --stdio` does not accept `--quiet`; stdout is protocol-only",
            ),
            (
                vec!["--verbose".into()],
                "`lsp --stdio` does not accept `--verbose`; stdout is protocol-only",
            ),
        ] {
            assert_eq!(Options::parse(Command::Lsp, &argument).unwrap_err(), error);
        }
    }

    #[test]
    fn parses_output_policy_defaults_and_explicit_values() {
        let defaults = Options::parse(Command::Check, &["main.ling".into()]).unwrap();
        assert_eq!(defaults.policy.format(), OutputFormat::Human);
        assert_eq!(defaults.policy.language(), HumanLanguage::Bilingual);
        assert_eq!(defaults.policy.color(), ColorChoice::Auto);
        assert_eq!(defaults.policy.verbosity(), Verbosity::Normal);

        let explicit = Options::parse(
            Command::Check,
            &[
                "--verbose".into(),
                "--color".into(),
                "never".into(),
                "main.ling".into(),
                "--language".into(),
                "en".into(),
            ],
        )
        .unwrap();
        assert_eq!(explicit.policy.language(), HumanLanguage::English);
        assert_eq!(explicit.policy.color(), ColorChoice::Never);
        assert_eq!(explicit.policy.verbosity(), Verbosity::Verbose);
    }

    #[test]
    fn rejects_incompatible_or_repeated_output_options() {
        for arguments in [
            vec!["--quiet".into(), "--verbose".into(), "main.ling".into()],
            vec!["--quiet".into(), "--quiet".into(), "main.ling".into()],
        ] {
            assert_eq!(
                Options::parse(Command::Check, &arguments).unwrap_err(),
                "only one of `--quiet` or `--verbose` may be provided"
            );
        }
        assert_eq!(
            Options::parse(
                Command::Check,
                &[
                    "--format".into(),
                    "json".into(),
                    "--quiet".into(),
                    "main.ling".into(),
                ],
            )
            .unwrap_err(),
            "`--quiet` and `--verbose` are not valid with `--format json`"
        );
        assert_eq!(
            Options::parse(
                Command::Check,
                &[
                    "--format".into(),
                    "json".into(),
                    "--color".into(),
                    "always".into(),
                    "main.ling".into(),
                ],
            )
            .unwrap_err(),
            "`--format json` accepts only an explicit `--color never`"
        );
        let json = Options::parse(
            Command::Check,
            &[
                "--color".into(),
                "never".into(),
                "--format".into(),
                "json".into(),
                "main.ling".into(),
            ],
        )
        .unwrap();
        assert_eq!(json.policy.format(), OutputFormat::Json);

        for (arguments, expected) in [
            (
                vec!["--language".into(), "fr".into(), "main.ling".into()],
                "unsupported output language `fr`",
            ),
            (
                vec!["--color".into(), "sometimes".into(), "main.ling".into()],
                "unsupported color choice `sometimes`",
            ),
            (
                vec![
                    "--language".into(),
                    "en".into(),
                    "--language".into(),
                    "zh-CN".into(),
                    "main.ling".into(),
                ],
                "only one `--language` may be provided",
            ),
            (
                vec![
                    "--color".into(),
                    "never".into(),
                    "--color".into(),
                    "auto".into(),
                    "main.ling".into(),
                ],
                "only one `--color` may be provided",
            ),
        ] {
            assert_eq!(
                Options::parse(Command::Check, &arguments).unwrap_err(),
                expected
            );
        }
    }

    #[test]
    fn help_mentions_only_the_implemented_command_catalog() {
        let help = usage();
        for command in Command::all() {
            assert!(
                help.contains(command.name()),
                "help is missing implemented command `{}`",
                command.name()
            );
        }
        for stale in ["zero", ".zero"] {
            assert!(
                !help.contains(stale),
                "help advertises stale command `{stale}`"
            );
        }
    }

    #[test]
    fn repl_completion_ignores_delimiters_in_text_and_comments() {
        assert!(delimiters_closed("\"(\" // [\n/* { } */\n"));
        assert!(!delimiters_closed("sum [1; 2"));
        assert!(!delimiters_closed("/* open"));
        assert!(delimiters_closed("sum [1; 2]"));
    }

    #[test]
    fn repl_interrupt_clears_only_the_pending_submission() {
        let mut session = Session::new(Vec::new());
        let mut console = ling_eval::MemoryConsole::default();
        session
            .submit("let answer = 42", &mut console)
            .expect("definition commits");
        let mut pending = "let unfinished = (\n".to_owned();

        cancel_repl_submission(&mut pending);

        assert!(pending.is_empty());
        let result = session
            .submit("answer", &mut console)
            .expect("committed state survives interrupt");
        assert_eq!(result.value.as_deref(), Some("42"));
    }
}
