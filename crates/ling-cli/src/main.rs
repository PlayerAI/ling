use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{BufRead, IsTerminal as _, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod command_catalog;
mod exit_catalog;
mod init;
mod test_runner;

use command_catalog::Command;
use exit_catalog::{
    EXIT_COMPILE_ERROR, EXIT_INTERNAL_ERROR, EXIT_INVALID_USAGE, EXIT_RUNTIME_FAULT,
    EXIT_SNAPSHOT_MISMATCH, EXIT_SUCCESS,
};
use ling_cli::incident::{InternalIncident, Reproduction};
use ling_cli::session::{Session, SubmissionFailure, SubmissionKind, SubmissionSuccess};
use ling_cli::{CompileFailure, compile_path};
use ling_diagnostics::{Diagnostic, MessageLanguage};
use ling_effects::locate_main;
use ling_eval::{Console, HostError, HostErrorCategory};
use ling_format::{FormatDisposition, build_format_ir, format_core_with_disposition};
use ling_project::{
    LockMode, LockedGraphFailure, ManifestError, PackageGraph, discover_modules, parse_manifest,
    resolve_package_graph_with_lock,
};
use ling_unicode::UNICODE_VERSION;

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

    let options = match Options::parse(command, command_arguments) {
        Ok(options) => options,
        Err(message) => return invalid_usage(&message),
    };

    execute(options)
}

fn execute(options: Options) -> u8 {
    if options.command == Command::Init {
        return execute_init(
            options.format,
            options.path.expect("init requires a destination"),
            options.init_name,
            options.init_display_name,
        );
    }
    if options.command == Command::Test {
        return execute_test(
            options.format,
            options.path.expect("test requires an input path"),
        );
    }
    if options.command == Command::ProjectCheck {
        return execute_project_check(
            options.format,
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
        return execute_repl(options.format, options.capabilities);
    }

    if options.command == Command::Format {
        return execute_format(
            options.format,
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
            return emit_compile_errors(&diagnostics, options.format);
        }
        Err(CompileFailure::Internal(message)) => {
            return emit_internal_incident(
                "compile.pipeline",
                message,
                reproduction,
                options.format,
            );
        }
        Err(CompileFailure::SnapshotMismatch(message)) => {
            return emit_snapshot_mismatch(&message, options.format);
        }
    };

    if !compiled.snapshot.checked().warnings().is_empty() {
        let status = emit_diagnostics(
            compiled.snapshot.checked().warnings(),
            options.format,
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
                Err(error) => emit_host_io_failure("semantic.stdout", &error, options.format),
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
                        options.format,
                    );
                }
            };
            let mut stdout = std::io::stdout().lock();
            match stdout.write_all(rendered.as_bytes()) {
                Ok(()) => EXIT_SUCCESS,
                Err(error) => emit_host_io_failure("audit.stdout", &error, options.format),
            }
        }
        Command::Run => {
            let main = match locate_main(compiled.snapshot.checked()) {
                Ok(main) => main,
                Err(error) => {
                    return emit_compile_error(error.to_diagnostic(), options.format);
                }
            };
            let mut console = StdoutConsole;
            match ling_eval::execute_main(&compiled.snapshot, &main, &mut console) {
                Ok(()) => EXIT_SUCCESS,
                Err(fault) => {
                    emit_diagnostics(&[fault.to_diagnostic()], options.format, EXIT_RUNTIME_FAULT)
                }
            }
        }
        Command::Repl => unreachable!("handled before compilation"),
        Command::Format => unreachable!("handled before compilation"),
        Command::ProjectCheck => unreachable!("handled before project checking"),
        Command::Lsp => unreachable!("handled before compilation"),
        Command::Init => unreachable!("handled before compilation"),
        Command::Test => unreachable!("handled before test execution"),
    }
}

fn execute_test(format: OutputFormat, root: PathBuf) -> u8 {
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
                format,
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
                format,
            );
        }
        Err(test_runner::Failure::Snapshot(message)) => {
            return emit_snapshot_mismatch(&message, format);
        }
    };

    let exit_code = summary.exit_code();
    let diagnostics = summary.diagnostics();
    if !diagnostics.is_empty() {
        let rendered_status = emit_diagnostics(&diagnostics, format, exit_code);
        if rendered_status != exit_code {
            return rendered_status;
        }
    }

    match format {
        OutputFormat::Human => {
            println!(
                "测试完成 / tests completed: root={} total={} passed={} failed={}",
                summary.root(),
                summary.total(),
                summary.passed(),
                summary.failed()
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
                    Err(error) => emit_host_io_failure("test.stdout", &error, format),
                },
                Err(error) => emit_internal_incident(
                    "test.success-json",
                    error.to_string(),
                    Reproduction::new("ling test --format json"),
                    format,
                ),
            }
        }
    }
}

fn execute_init(
    format: OutputFormat,
    destination: PathBuf,
    package_name: Option<String>,
    display_name: Option<String>,
) -> u8 {
    match init::create(destination, package_name, display_name) {
        Ok(summary) => match format {
            OutputFormat::Human => {
                println!(
                    "已创建 Ling 工程 / created Ling project: directory={} package={} files={}",
                    summary.directory,
                    summary.package_name,
                    summary.files.join(",")
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
                        Err(error) => emit_host_io_failure("init.stdout", &error, format),
                    },
                    Err(error) => emit_internal_incident(
                        "init.success-json",
                        error.to_string(),
                        Reproduction::new("ling init --format json"),
                        format,
                    ),
                }
            }
        },
        Err(init::Failure::Usage(message)) => invalid_usage(&message),
        Err(init::Failure::Diagnostics(diagnostics)) => {
            emit_diagnostics(&diagnostics, format, EXIT_COMPILE_ERROR)
        }
        Err(init::Failure::Internal(message)) => emit_internal_incident(
            "init.template",
            message,
            Reproduction::new("ling init"),
            format,
        ),
        Err(failure @ init::Failure::Io { .. }) => {
            let diagnostic = init::diagnostic_for_failure(&failure)
                .expect("I/O init failures always have a diagnostic");
            emit_diagnostics(&[diagnostic], format, EXIT_RUNTIME_FAULT)
        }
    }
}

fn execute_lsp() -> u8 {
    match ling_lsp::run_stdio(std::io::stdin().lock(), std::io::stdout().lock()) {
        Ok(result) => result.exit_code(),
        Err(error) => {
            eprintln!("LSP 传输失败：{error}\nLSP transport failed: {error}");
            EXIT_RUNTIME_FAULT
        }
    }
}

fn execute_project_check(format: OutputFormat, manifest_path: PathBuf) -> u8 {
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
                format,
            );
        }
    };
    let manifest = match parse_manifest("ling.toml", &bytes) {
        Ok(manifest) => manifest,
        Err(error) => return emit_project_manifest_failure(format, error),
    };

    if let Err(error) = discover_modules(project_root, &manifest) {
        return emit_project_discovery_failure(format, error);
    }
    let graph = match resolve_package_graph_with_lock(project_root, &manifest, LockMode::Locked) {
        Ok(graph) => graph,
        Err(error) => return emit_project_locked_failure(format, error),
    };
    emit_project_success(format, &graph)
}

fn emit_project_manifest_failure(format: OutputFormat, error: ManifestError) -> u8 {
    emit_project_diagnostics(format, &[error.diagnostic()])
}

fn emit_project_discovery_failure(
    format: OutputFormat,
    error: ling_project::DiscoveryFailure,
) -> u8 {
    match error {
        ling_project::DiscoveryFailure::Diagnostics(diagnostics) => {
            emit_project_diagnostics(format, &diagnostics)
        }
        ling_project::DiscoveryFailure::Internal(message) => emit_internal_incident(
            "project.module-discovery",
            message,
            Reproduction::new("ling project check"),
            format,
        ),
    }
}

fn emit_project_locked_failure(format: OutputFormat, error: LockedGraphFailure) -> u8 {
    match error.diagnostics() {
        Some(diagnostics) => emit_project_diagnostics(format, diagnostics),
        None => emit_internal_incident(
            "project.locked-resolution",
            error.to_string(),
            Reproduction::new("ling project check"),
            format,
        ),
    }
}

fn emit_project_diagnostics(format: OutputFormat, diagnostics: &[Diagnostic]) -> u8 {
    match format {
        OutputFormat::Human => emit_diagnostics(diagnostics, format, EXIT_COMPILE_ERROR),
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
                    format,
                ),
            }
        }
    }
}

fn emit_project_success(format: OutputFormat, graph: &PackageGraph) -> u8 {
    let Some(root_package) = graph.package(graph.root()) else {
        return emit_internal_incident(
            "project.root-package",
            "locked graph omitted its root package",
            Reproduction::new("ling project check"),
            format,
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
    match format {
        OutputFormat::Human => {
            println!(
                "项目检查通过 / project check passed: package={package}@{version} entry={entry} modules={module_count} packages={package_count}"
            );
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
                    Err(error) => emit_host_io_failure("project.stdout", &error, format),
                },
                Err(error) => emit_internal_incident(
                    "project.success-json",
                    error.to_string(),
                    Reproduction::new("ling project check --format json"),
                    format,
                ),
            }
        }
    }
}

fn execute_format(
    format: OutputFormat,
    check: bool,
    path: PathBuf,
    stdin_name: Option<String>,
) -> u8 {
    let (source_name, bytes) = match read_format_input(&path, stdin_name.as_deref()) {
        Ok(input) => input,
        Err((source_name, diagnostics)) => {
            return emit_format_failure(format, source_name, check, diagnostics);
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
                format,
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
        return emit_format_failure(format, source_name, check, diagnostics);
    }

    let document = match build_format_ir(&source, &parsed) {
        Ok(document) => document,
        Err(error) => {
            return emit_internal_incident(
                "format.ir",
                error.to_string(),
                Reproduction::new("ling fmt").with_input(source_name),
                format,
            );
        }
    };
    let result = format_core_with_disposition(&document);
    if matches!(
        result.disposition(),
        FormatDisposition::OriginalInvalidSource | FormatDisposition::OriginalRejectedCandidate
    ) {
        return emit_format_failure(
            format,
            source_name,
            check,
            vec![format_rejected_diagnostic()],
        );
    }

    let changed = result.text() != source.original_text();
    let disposition = if changed { "formatted" } else { "unchanged" };
    if format == OutputFormat::Json {
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
            eprintln!("需要格式化：{source_name}\nwould reformat: {source_name}");
            EXIT_COMPILE_ERROR
        } else {
            EXIT_SUCCESS
        }
    } else {
        match write_stdout(result.text().as_bytes()) {
            Ok(()) => EXIT_SUCCESS,
            Err(error) => emit_host_io_failure("format.stdout", &error, format),
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
    format: OutputFormat,
    source_name: String,
    check: bool,
    diagnostics: Vec<Diagnostic>,
) -> u8 {
    if format == OutputFormat::Json {
        return match diagnostic_values(&diagnostics) {
            Ok(values) => emit_format_report(source_name, check, false, "invalid", None, values),
            Err(status) => status,
        };
    }
    emit_compile_errors(&diagnostics, OutputFormat::Human)
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
                OutputFormat::Json,
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
        Err(error) => emit_host_io_failure("format.stdout", &error, OutputFormat::Json),
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

fn execute_repl(format: OutputFormat, capabilities: Vec<String>) -> u8 {
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
            format,
            &mut had_compile_failure,
            &mut had_runtime_failure,
        )
    } else {
        execute_script_repl(
            &mut stdin.lock(),
            &mut session,
            &mut buffer,
            format,
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
    format: OutputFormat,
    had_compile_failure: &mut bool,
    had_runtime_failure: &mut bool,
) -> Result<(), u8> {
    let mut editor = rustyline::DefaultEditor::new()
        .map_err(|_| emit_host_failure("repl.terminal-init", "other", format))?;
    loop {
        let prompt = if format == OutputFormat::Json {
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
                format,
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
                    format,
                    had_compile_failure,
                    had_runtime_failure,
                );
            }
            Err(rustyline::error::ReadlineError::Io(error)) => {
                return Err(emit_host_io_failure("repl.terminal-input", &error, format));
            }
            Err(_) => return Err(emit_host_failure("repl.terminal-input", "other", format)),
        }
    }
}

fn execute_script_repl(
    input: &mut impl BufRead,
    session: &mut Session,
    buffer: &mut String,
    format: OutputFormat,
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
                    format,
                    had_compile_failure,
                    had_runtime_failure,
                );
            }
            Ok(_) => handle_repl_line(
                session,
                buffer,
                &line,
                format,
                had_compile_failure,
                had_runtime_failure,
            )?,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {
                cancel_repl_submission(buffer);
            }
            Err(error) => {
                return Err(emit_host_io_failure("repl.script-input", &error, format));
            }
        }
    }
}

fn handle_repl_line(
    session: &mut Session,
    buffer: &mut String,
    line: &str,
    format: OutputFormat,
    had_compile_failure: &mut bool,
    had_runtime_failure: &mut bool,
) -> Result<(), u8> {
    if line.trim().is_empty() && !buffer.trim().is_empty() && delimiters_closed(buffer) {
        process_submission(
            session,
            buffer,
            format,
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
    format: OutputFormat,
    had_compile_failure: &mut bool,
    had_runtime_failure: &mut bool,
) -> Result<(), u8> {
    if buffer.trim().is_empty() {
        Ok(())
    } else {
        process_submission(
            session,
            buffer,
            format,
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
    format: OutputFormat,
    had_compile_failure: &mut bool,
    had_runtime_failure: &mut bool,
) -> Result<(), u8> {
    let mut console = ling_eval::MemoryConsole::default();
    let outcome = session.submit(source.trim_end(), &mut console);
    if !console.output().is_empty() {
        match format {
            OutputFormat::Human => {
                write_stdout(console.output().as_bytes())
                    .map_err(|error| emit_host_io_failure("repl.console", &error, format))?;
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
                .map_err(|error| emit_host_io_failure("repl.console-json", &error, format))?;
            }
        }
    }

    match outcome {
        Ok(success) => emit_repl_success(&success, format)?,
        Err(SubmissionFailure::Compile {
            submission,
            diagnostics,
        }) => {
            *had_compile_failure = true;
            emit_repl_failure(submission, &diagnostics, format)?;
        }
        Err(SubmissionFailure::Runtime { submission, fault }) => {
            *had_runtime_failure = true;
            emit_repl_runtime_failure(submission, &fault, format)?;
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
                format,
            ));
        }
        Err(SubmissionFailure::SnapshotMismatch {
            submission,
            message,
        }) => {
            return Err(emit_repl_snapshot_mismatch(submission, &message, format));
        }
    }
    Ok(())
}

fn emit_repl_success(success: &SubmissionSuccess, format: OutputFormat) -> Result<(), u8> {
    match format {
        OutputFormat::Human => {
            if !success.warnings.is_empty() {
                let _ = emit_diagnostics(&success.warnings, OutputFormat::Human, EXIT_SUCCESS);
            }
            match success.kind {
                SubmissionKind::Expression => {
                    if let (Some(value), Some(type_name)) = (&success.value, &success.type_name) {
                        write_stdout(format!("{value} : {type_name}\n").as_bytes())
                            .map_err(|error| emit_host_io_failure("repl.result", &error, format))?;
                    }
                }
                SubmissionKind::ValueDeclaration => {
                    if let (Some(name), Some(type_name)) = (&success.name, &success.type_name) {
                        write_stdout(format!("{name} : {type_name}\n").as_bytes())
                            .map_err(|error| emit_host_io_failure("repl.result", &error, format))?;
                    }
                }
                SubmissionKind::TypeDeclaration => {
                    if let Some(name) = &success.name {
                        write_stdout(format!("type {name}\n").as_bytes())
                            .map_err(|error| emit_host_io_failure("repl.result", &error, format))?;
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
                .map_err(|error| emit_host_io_failure("repl.result-json", &error, format))?;
        }
    }
    Ok(())
}

fn emit_repl_failure(
    submission: u64,
    diagnostics: &[Diagnostic],
    format: OutputFormat,
) -> Result<(), u8> {
    match format {
        OutputFormat::Human => {
            let _ = emit_compile_errors(diagnostics, OutputFormat::Human);
        }
        OutputFormat::Json => {
            emit_repl_json(&serde_json::json!({
                "schema": "ling.repl/0.1",
                "status": "compile_error",
                "committed": false,
                "submission": submission,
                "diagnostics": diagnostic_values(diagnostics)?,
            }))
            .map_err(|error| emit_host_io_failure("repl.compile-error-json", &error, format))?;
        }
    }
    Ok(())
}

fn emit_repl_runtime_failure(
    submission: u64,
    fault: &ling_eval::RuntimeFault,
    format: OutputFormat,
) -> Result<(), u8> {
    let diagnostic = fault.to_diagnostic().with_fact("committed", false);
    match format {
        OutputFormat::Human => {
            let _ = emit_diagnostics(&[diagnostic], OutputFormat::Human, EXIT_RUNTIME_FAULT);
        }
        OutputFormat::Json => {
            emit_repl_json(&serde_json::json!({
                "schema": "ling.repl/0.1",
                "status": "runtime_error",
                "committed": false,
                "submission": submission,
                "diagnostics": diagnostic_values(&[diagnostic])?,
            }))
            .map_err(|error| emit_host_io_failure("repl.runtime-error-json", &error, format))?;
        }
    }
    Ok(())
}

fn emit_repl_snapshot_mismatch(submission: u64, message: &str, format: OutputFormat) -> u8 {
    let diagnostic = snapshot_mismatch_diagnostic(message).with_fact("committed", false);
    match format {
        OutputFormat::Human => {
            emit_diagnostics(&[diagnostic], OutputFormat::Human, EXIT_SNAPSHOT_MISMATCH)
        }
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
                Err(error) => emit_host_io_failure("repl.snapshot-mismatch-json", &error, format),
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
                        OutputFormat::Json,
                    )
                })
                .and_then(|rendered| {
                    serde_json::from_str(&rendered).map_err(|error| {
                        emit_internal_incident(
                            "diagnostic.parse-rendered-json",
                            error.to_string(),
                            Reproduction::new("ling repl --format json"),
                            OutputFormat::Json,
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

fn emit_compile_error(diagnostic: Diagnostic, format: OutputFormat) -> u8 {
    emit_compile_errors(&[diagnostic], format)
}

fn emit_compile_errors(diagnostics: &[Diagnostic], format: OutputFormat) -> u8 {
    emit_diagnostics(diagnostics, format, EXIT_COMPILE_ERROR)
}

fn emit_snapshot_mismatch(message: &str, format: OutputFormat) -> u8 {
    emit_diagnostics(
        &[snapshot_mismatch_diagnostic(message)],
        format,
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

fn emit_host_io_failure(operation: &str, error: &std::io::Error, format: OutputFormat) -> u8 {
    let category = host_error_category(error.kind()).name();
    emit_host_failure(operation, category, format)
}

fn emit_host_failure(operation: &str, category: &str, format: OutputFormat) -> u8 {
    let diagnostic = Diagnostic::new(
        ling_diagnostics::codes::RUNTIME_FAULT,
        ling_diagnostics::Severity::Error,
        format!("宿主输出操作“{operation}”失败"),
        format!("host output operation `{operation}` failed"),
    )
    .with_fact("category", category)
    .with_fact("operation", operation);
    emit_diagnostics(&[diagnostic], format, EXIT_RUNTIME_FAULT)
}

fn emit_diagnostics(diagnostics: &[Diagnostic], format: OutputFormat, exit_code: u8) -> u8 {
    for diagnostic in diagnostics {
        let rendered = match format {
            OutputFormat::Human => Ok(diagnostic.render_human(MessageLanguage::Chinese)),
            OutputFormat::Json => diagnostic.render_json().map_err(|error| error.to_string()),
        };
        match rendered {
            Ok(rendered) => eprintln!("{rendered}"),
            Err(error) => {
                return emit_internal_incident(
                    "diagnostic.render",
                    error,
                    Reproduction::new("ling diagnostics"),
                    format,
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
    format: OutputFormat,
) -> u8 {
    let incident = InternalIncident::capture(stage, detail, reproduction);
    let diagnostic = incident.diagnostic();
    let rendered = match format {
        OutputFormat::Human => Ok(diagnostic.render_human(MessageLanguage::Chinese)),
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
        "Usage:\n  {CLI_NAME} --version\n  {CLI_NAME} <run|check|semantic|audit> [--format human|json] <file>\n  {CLI_NAME} test [--format human|json] <file-or-directory>\n  {CLI_NAME} fmt [--check] [--format human|json] [--stdin-name name] <file|->\n  {CLI_NAME} init [--format human|json] [--name package] [--display-name text] <directory>\n  {CLI_NAME} project check --manifest-path path --locked [--format human|json]\n  {CLI_NAME} repl [--format human|json] [--capability Console.Write]\n  {CLI_NAME} lsp --stdio"
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OutputFormat {
    Human,
    Json,
}

impl OutputFormat {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "human" => Some(Self::Human),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Options {
    command: Command,
    format: OutputFormat,
    path: Option<PathBuf>,
    manifest_path: Option<PathBuf>,
    capabilities: Vec<String>,
    check: bool,
    locked: bool,
    stdin_name: Option<String>,
    stdio: bool,
    init_name: Option<String>,
    init_display_name: Option<String>,
}

impl Options {
    fn parse(command: Command, arguments: &[OsString]) -> Result<Self, String> {
        let mut format = OutputFormat::Human;
        let mut path = None;
        let mut manifest_path = None;
        let mut capabilities = Vec::new();
        let mut check = false;
        let mut locked = false;
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
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "`--format` requires `human` or `json`".to_owned())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "the output format must be valid Unicode".to_owned())?;
                format = OutputFormat::parse(value)
                    .ok_or_else(|| format!("unsupported output format `{value}`"))?;
                index += 2;
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
                if command != Command::ProjectCheck {
                    return Err("`--manifest-path` is only valid with `project check`".to_owned());
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
                if command != Command::ProjectCheck {
                    return Err("`--locked` is only valid with `project check`".to_owned());
                }
                if locked {
                    return Err("only one `--locked` may be provided".to_owned());
                }
                locked = true;
                index += 1;
                continue;
            }

            if argument.to_string_lossy().starts_with('-') && argument != "-" {
                return Err(format!("unknown option `{}`", argument.to_string_lossy()));
            }
            if command == Command::ProjectCheck {
                return Err("`project check` does not accept a positional path".to_owned());
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

        if matches!(
            command,
            Command::Repl | Command::Lsp | Command::ProjectCheck
        ) && path.is_some()
        {
            return Err(format!("`{command}` does not accept a source file"));
        }
        if command != Command::Repl
            && command != Command::Lsp
            && command != Command::ProjectCheck
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
        } else if manifest_path.is_some() || locked {
            return Err("project options require `project check`".to_owned());
        }

        if command != Command::Init && (init_name.is_some() || init_display_name.is_some()) {
            return Err("init metadata options require `init`".to_owned());
        }
        if command == Command::Init && path.as_deref() == Some(Path::new("-")) {
            return Err("`init` requires a destination directory, not `-`".to_owned());
        }

        Ok(Self {
            command,
            format,
            path,
            manifest_path,
            capabilities,
            check,
            locked,
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
        assert_eq!(before.format, OutputFormat::Json);
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
        assert_eq!(file.format, OutputFormat::Json);
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
