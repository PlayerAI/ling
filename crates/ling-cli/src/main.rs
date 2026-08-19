use std::env;
use std::ffi::OsString;
use std::io::{BufRead, IsTerminal as _, Write as _};
use std::path::PathBuf;
use std::process::ExitCode;

use ling_cli::incident::{InternalIncident, Reproduction};
use ling_cli::session::{Session, SubmissionFailure, SubmissionKind, SubmissionSuccess};
use ling_cli::{CompileFailure, compile_path};
use ling_diagnostics::{Diagnostic, MessageLanguage};
use ling_effects::locate_main;
use ling_eval::{Console, HostError, HostErrorCategory};
use ling_unicode::UNICODE_VERSION;

const CLI_NAME: &str = "ling";
const EXIT_SUCCESS: u8 = 0;
const EXIT_COMPILE_ERROR: u8 = 1;
const EXIT_INVALID_USAGE: u8 = 2;
const EXIT_RUNTIME_FAULT: u8 = 4;
const EXIT_INTERNAL_ERROR: u8 = 5;
const EXIT_SNAPSHOT_MISMATCH: u8 = 6;

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
    let Some(command) = Command::parse(command_name) else {
        return invalid_usage(&format!("unknown command `{command_name}`"));
    };

    let options = match Options::parse(command, &arguments[1..]) {
        Ok(options) => options,
        Err(message) => return invalid_usage(&message),
    };

    execute(options)
}

fn execute(options: Options) -> u8 {
    if options.command == Command::Repl {
        return execute_repl(options.format, options.capabilities);
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
        "Usage:\n  {CLI_NAME} --version\n  {CLI_NAME} <run|check|semantic|audit> [--format human|json] <file>\n  {CLI_NAME} repl [--format human|json] [--capability Console.Write]"
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Command {
    Run,
    Check,
    Repl,
    Semantic,
    Audit,
}

impl Command {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "run" => Some(Self::Run),
            "check" => Some(Self::Check),
            "repl" => Some(Self::Repl),
            "semantic" => Some(Self::Semantic),
            "audit" => Some(Self::Audit),
            _ => None,
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Run => "run",
            Self::Check => "check",
            Self::Repl => "repl",
            Self::Semantic => "semantic",
            Self::Audit => "audit",
        };
        formatter.write_str(name)
    }
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
    capabilities: Vec<String>,
}

impl Options {
    fn parse(command: Command, arguments: &[OsString]) -> Result<Self, String> {
        let mut format = OutputFormat::Human;
        let mut path = None;
        let mut capabilities = Vec::new();
        let mut index = 0;

        while index < arguments.len() {
            let argument = &arguments[index];
            if argument == "--format" {
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

            if argument.to_string_lossy().starts_with('-') {
                return Err(format!("unknown option `{}`", argument.to_string_lossy()));
            }
            if path.replace(PathBuf::from(argument)).is_some() {
                return Err("only one source file may be provided".to_owned());
            }
            index += 1;
        }

        if command == Command::Repl && path.is_some() {
            return Err("`repl` does not accept a source file".to_owned());
        }
        if command != Command::Repl && path.is_none() {
            return Err(format!("`{command}` requires a source file"));
        }

        Ok(Self {
            command,
            format,
            path,
            capabilities,
        })
    }
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
