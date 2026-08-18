use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, MessageLanguage, Severity, codes};
use ling_effects::locate_main;
use ling_eval::{Console, HostError, HostErrorCategory};
use ling_hir::{LowerErrorKind, Program as HirProgram};
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceError, SourceFile, SourceId};
use ling_syntax::parse;
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
    if matches!(options.command, Command::Repl | Command::Audit) {
        return emit_compile_error(
            not_implemented_diagnostic(options.command, None, None, "cli"),
            options.format,
        );
    }

    let path = options
        .path
        .as_deref()
        .expect("commands other than repl require a path");
    let compiled = match compile(path) {
        Ok(compiled) => compiled,
        Err(CompileFailure::Diagnostics(diagnostics)) => {
            return emit_compile_errors(&diagnostics, options.format);
        }
        Err(CompileFailure::Internal(message)) => {
            eprintln!("internal compiler error: {message}");
            return EXIT_INTERNAL_ERROR;
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
            if stdout
                .write_all(compiled.snapshot.json().as_bytes())
                .and_then(|()| stdout.write_all(b"\n"))
                .is_err()
            {
                eprintln!("internal compiler error: failed to write semantic JSON");
                EXIT_SNAPSHOT_MISMATCH
            } else {
                EXIT_SUCCESS
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
        Command::Repl | Command::Audit => unreachable!("handled before compilation"),
    }
}

struct Compiled {
    snapshot: ProgramSnapshot,
}

enum CompileFailure {
    Diagnostics(Vec<Diagnostic>),
    Internal(String),
}

fn compile(entry_path: &Path) -> Result<Compiled, CompileFailure> {
    let root = entry_path.parent().unwrap_or_else(|| Path::new("."));
    let mut loader = ModuleLoader::new(root.to_path_buf());
    let entry = loader.load_entry(entry_path)?;
    let programs = loader.programs.into_values().collect::<Vec<_>>();
    let resolved = ling_resolve::resolve(programs, &entry).map_err(|errors| {
        CompileFailure::Diagnostics(
            errors
                .iter()
                .map(ling_resolve::ResolveError::to_diagnostic)
                .collect(),
        )
    })?;
    let typed = ling_types::check(resolved).map_err(|errors| {
        CompileFailure::Diagnostics(
            errors
                .iter()
                .map(ling_types::TypeError::to_diagnostic)
                .collect(),
        )
    })?;
    let checked = ling_effects::check(typed).map_err(|errors| {
        CompileFailure::Diagnostics(
            errors
                .iter()
                .map(ling_effects::EffectError::to_diagnostic)
                .collect(),
        )
    })?;
    let snapshot = ling_semantic::build(checked)
        .map_err(|error| CompileFailure::Internal(error.to_string()))?;
    Ok(Compiled { snapshot })
}

struct ModuleLoader {
    root: PathBuf,
    programs: BTreeMap<String, HirProgram>,
    paths: BTreeMap<String, PathBuf>,
    expanded: BTreeSet<String>,
    next_source_id: u32,
}

impl ModuleLoader {
    fn new(root: PathBuf) -> Self {
        Self {
            root,
            programs: BTreeMap::new(),
            paths: BTreeMap::new(),
            expanded: BTreeSet::new(),
            next_source_id: 0,
        }
    }

    fn load_entry(&mut self, path: &Path) -> Result<String, CompileFailure> {
        let program = self.load_source(path, None)?;
        let entry = program.module.name.normalized();
        self.insert_program(path, program)?;
        self.load_imports(&entry)?;
        Ok(entry)
    }

    fn load_imports(&mut self, module_name: &str) -> Result<(), CompileFailure> {
        if !self.expanded.insert(module_name.to_owned()) {
            return Ok(());
        }
        let imports = self
            .programs
            .get(module_name)
            .expect("loaded module exists")
            .imports
            .clone();
        for import in imports {
            let imported_name = import.module.normalized();
            if !self.programs.contains_key(&imported_name) {
                let path = self.import_path(&import.module);
                if let Err(diagnostic) = self.verify_exact_import_path(&path, &import.module) {
                    return Err(CompileFailure::Diagnostics(vec![
                        (*diagnostic).with_primary_span(DiagnosticSpan::new(
                            &self.programs[module_name].source_name,
                            import.span,
                        )),
                    ]));
                }
                let program = self.load_source(&path, Some((&imported_name, import.span)))?;
                let actual = program.module.name.normalized();
                if actual != imported_name {
                    return Err(CompileFailure::Diagnostics(vec![Diagnostic::new(
                        codes::INVALID_MODULE,
                        Severity::Error,
                        format!(
                            "模块声明“{actual}”与 import 名称“{imported_name}”不一致"
                        ),
                        format!(
                            "module declaration `{actual}` does not match import `{imported_name}`"
                        ),
                    )
                    .with_primary_span(DiagnosticSpan::new(&program.source_name, program.module.span))
                    .with_fact("expected_module", imported_name.clone())
                    .with_fact("actual_module", actual)]));
                }
                self.insert_program(&path, program)?;
            }
            self.load_imports(&imported_name)?;
        }
        Ok(())
    }

    fn insert_program(&mut self, path: &Path, program: HirProgram) -> Result<(), CompileFailure> {
        let module_name = program.module.name.normalized();
        if let Some(previous) = self.paths.get(&module_name) {
            if previous != path {
                return Err(CompileFailure::Diagnostics(vec![
                    Diagnostic::new(
                        codes::INVALID_MODULE,
                        Severity::Error,
                        format!("模块“{module_name}”由多个文件声明"),
                        format!("module `{module_name}` is declared by multiple files"),
                    )
                    .with_primary_span(DiagnosticSpan::new(
                        &program.source_name,
                        program.module.span,
                    )),
                ]));
            }
            return Ok(());
        }
        self.paths.insert(module_name.clone(), path.to_path_buf());
        self.programs.insert(module_name, program);
        Ok(())
    }

    fn load_source(
        &mut self,
        path: &Path,
        imported: Option<(&str, ling_source::Span)>,
    ) -> Result<HirProgram, CompileFailure> {
        let display_path = path.to_string_lossy().into_owned();
        let bytes = std::fs::read(path).map_err(|error| {
            let diagnostic = if let Some((module, _)) = imported {
                Diagnostic::new(
                    codes::MODULE_NOT_FOUND,
                    Severity::Error,
                    format!("找不到 import 模块“{module}”"),
                    format!("imported module `{module}` was not found"),
                )
                .with_fact("module", module)
            } else {
                Diagnostic::new(
                    codes::SOURCE_READ_FAILED,
                    Severity::Error,
                    format!("无法读取源码文件“{display_path}”"),
                    format!("failed to read source file `{display_path}`"),
                )
                .with_fact("io_kind", stable_io_kind(error.kind()))
            };
            CompileFailure::Diagnostics(vec![diagnostic])
        })?;
        let source_id = SourceId::new(self.next_source_id);
        self.next_source_id = self.next_source_id.saturating_add(1);
        let source =
            SourceFile::from_bytes(source_id, display_path.clone(), bytes).map_err(|error| {
                CompileFailure::Diagnostics(vec![source_error_diagnostic(&display_path, error)])
            })?;
        let parsed = parse(&source);
        if !parsed.lexical_errors().is_empty() {
            return Err(CompileFailure::Diagnostics(
                parsed
                    .lexical_errors()
                    .iter()
                    .map(|error| error.to_diagnostic(source.name()))
                    .collect(),
            ));
        }
        if !parsed.parse_errors().is_empty() {
            return Err(CompileFailure::Diagnostics(
                parsed
                    .parse_errors()
                    .iter()
                    .map(|error| error.to_diagnostic(source.name()))
                    .collect(),
            ));
        }
        let ast = ling_ast::lower(&source, &parsed)
            .map_err(|error| CompileFailure::Internal(format!("AST lowering failed: {error}")))?;
        ling_hir::lower(source.name(), &ast).map_err(|error| {
            let code = match error.kind {
                LowerErrorKind::InvalidAssignmentPlace => codes::INVALID_ASSIGNMENT,
                _ => codes::INVALID_MODULE,
            };
            CompileFailure::Diagnostics(vec![
                Diagnostic::new(
                    code,
                    Severity::Error,
                    format!("无法建立 Seed HIR：{error}"),
                    format!("cannot construct Seed HIR: {error}"),
                )
                .with_primary_span(DiagnosticSpan::new(source.name(), error.span)),
            ])
        })
    }

    fn import_path(&self, module: &ling_hir::QualifiedName) -> PathBuf {
        let mut path = self.root.clone();
        for segment in &module.segments[..module.segments.len().saturating_sub(1)] {
            path.push(&segment.normalized);
        }
        let last = &module
            .segments
            .last()
            .expect("qualified import names are non-empty")
            .normalized;
        path.push(format!("{last}.ling"));
        path
    }

    fn verify_exact_import_path(
        &self,
        path: &Path,
        module: &ling_hir::QualifiedName,
    ) -> Result<(), Box<Diagnostic>> {
        let desired = module
            .segments
            .iter()
            .enumerate()
            .map(|(index, segment)| {
                if index + 1 == module.segments.len() {
                    format!("{}.ling", segment.normalized)
                } else {
                    segment.normalized.clone()
                }
            })
            .collect::<Vec<_>>();
        let mut current = self.root.clone();
        for component in desired {
            let exact = std::fs::read_dir(&current).ok().is_some_and(|entries| {
                entries.filter_map(Result::ok).any(|entry| {
                    entry
                        .file_name()
                        .to_str()
                        .is_some_and(|name| name == component)
                })
            });
            if !exact {
                return Err(Box::new(
                    Diagnostic::new(
                        codes::MODULE_NOT_FOUND,
                        Severity::Error,
                        format!("找不到大小写完全匹配的 import 路径“{}”", path.display()),
                        format!("no exact-case import path exists at `{}`", path.display()),
                    )
                    .with_fact("module", module.normalized()),
                ));
            }
            current.push(component);
        }
        Ok(())
    }
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

const fn stable_io_kind(kind: std::io::ErrorKind) -> &'static str {
    match kind {
        std::io::ErrorKind::NotFound => "not_found",
        std::io::ErrorKind::PermissionDenied => "permission_denied",
        std::io::ErrorKind::InvalidData => "invalid_data",
        std::io::ErrorKind::Interrupted => "interrupted",
        _ => "other",
    }
}

fn source_error_diagnostic(path: &str, error: SourceError) -> Diagnostic {
    match error {
        SourceError::InvalidUtf8 {
            valid_up_to,
            error_len,
        } => {
            let end = valid_up_to.saturating_add(error_len.unwrap_or(1));
            Diagnostic::new(
                codes::INVALID_UTF8,
                Severity::Error,
                "源码不是有效的 UTF-8",
                "source is not valid UTF-8",
            )
            .with_primary_span(DiagnosticSpan::at(
                path,
                u32::try_from(valid_up_to).unwrap_or(u32::MAX),
                u32::try_from(end).unwrap_or(u32::MAX),
            ))
            .with_fact(
                "valid_up_to",
                u64::try_from(valid_up_to).unwrap_or(u64::MAX),
            )
        }
        SourceError::MisplacedByteOrderMark { byte_offset } => Diagnostic::new(
            codes::MISPLACED_BOM,
            Severity::Error,
            "UTF-8 BOM 只能出现在文件开头",
            "the UTF-8 byte-order mark is only allowed at the start of a file",
        )
        .with_primary_span(DiagnosticSpan::at(
            path,
            u32::try_from(byte_offset).unwrap_or(u32::MAX),
            u32::try_from(byte_offset.saturating_add(3)).unwrap_or(u32::MAX),
        )),
        SourceError::TooLarge { byte_len } => Diagnostic::new(
            codes::SOURCE_TOO_LARGE,
            Severity::Error,
            "源码文件超过当前实现支持的大小",
            "source file exceeds the size supported by this implementation",
        )
        .with_fact("byte_len", u64::try_from(byte_len).unwrap_or(u64::MAX))
        .with_fact("maximum_byte_len", u64::from(u32::MAX)),
    }
}

fn not_implemented_diagnostic(
    command: Command,
    source: Option<&SourceFile>,
    token_count: Option<usize>,
    completed_stage: &'static str,
) -> Diagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::FEATURE_NOT_IMPLEMENTED,
        Severity::Error,
        format!("`{command}` 命令所需的编译阶段尚未实现"),
        format!("the compiler stage required by `{command}` is not implemented yet"),
    )
    .with_fact("command", command.to_string())
    .with_fact("completed_stage", completed_stage);

    if let Some(source) = source {
        diagnostic = diagnostic
            .with_fact("source_name", source.name())
            .with_fact("had_bom", source.had_bom())
            .with_fact("unicode_version", UNICODE_VERSION.to_string());
    }
    if let Some(token_count) = token_count {
        diagnostic = diagnostic.with_fact(
            "token_count",
            u64::try_from(token_count).unwrap_or(u64::MAX),
        );
    }

    diagnostic
}

fn emit_compile_error(diagnostic: Diagnostic, format: OutputFormat) -> u8 {
    emit_compile_errors(&[diagnostic], format)
}

fn emit_compile_errors(diagnostics: &[Diagnostic], format: OutputFormat) -> u8 {
    emit_diagnostics(diagnostics, format, EXIT_COMPILE_ERROR)
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
                eprintln!("internal compiler error: {error}");
                return EXIT_INTERNAL_ERROR;
            }
        }
    }
    exit_code
}

fn invalid_usage(message: &str) -> u8 {
    eprintln!("error: {message}\n\n{}", usage());
    EXIT_INVALID_USAGE
}

fn usage() -> String {
    format!(
        "Usage:\n  {CLI_NAME} --version\n  {CLI_NAME} <run|check|semantic|audit> [--format human|json] <file>\n  {CLI_NAME} repl [--format human|json]"
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
}

impl Options {
    fn parse(command: Command, arguments: &[OsString]) -> Result<Self, String> {
        let mut format = OutputFormat::Human;
        let mut path = None;
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
}
