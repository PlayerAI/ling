//! Shared compiler orchestration for file and session frontends.

pub mod incident;
pub mod project;
pub mod session;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_hir::{LowerErrorKind, Program as HirProgram};
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceError, SourceFile, SourceId};

pub mod semantic_commands;

#[derive(Debug)]
pub struct Compiled {
    pub snapshot: ProgramSnapshot,
}

#[derive(Debug)]
pub enum CompileFailure {
    Diagnostics(Vec<Diagnostic>),
    Internal(String),
    SnapshotMismatch(String),
}

/// Compiles an entry file and its deterministic import closure.
pub fn compile_path(entry_path: &Path) -> Result<Compiled, CompileFailure> {
    let root = entry_path.parent().unwrap_or_else(|| Path::new("."));
    let mut loader = ModuleLoader::new(root.to_path_buf());
    let entry = loader.load_entry(entry_path)?;
    compile_programs(loader.programs.into_values().collect(), &entry)
}

/// Compiles one in-memory module through the same checked pipeline as files.
pub fn compile_source(
    source_name: impl Into<String>,
    bytes: Vec<u8>,
) -> Result<Compiled, CompileFailure> {
    let source_name = source_name.into();
    let program = lower_source(SourceId::new(0), source_name, bytes)?;
    let entry = program.module.name.normalized();
    compile_programs(vec![program], &entry)
}

pub(crate) fn compile_programs(
    programs: Vec<HirProgram>,
    entry: &str,
) -> Result<Compiled, CompileFailure> {
    let resolved = ling_resolve::resolve(programs, entry).map_err(|errors| {
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
    verify_snapshot(&snapshot).map_err(CompileFailure::SnapshotMismatch)?;
    Ok(Compiled { snapshot })
}

fn verify_snapshot(snapshot: &ProgramSnapshot) -> Result<(), String> {
    verify_snapshot_json(snapshot.graph(), snapshot.json())
}

fn verify_snapshot_json(graph: &ling_semantic::SemanticGraph, json: &str) -> Result<(), String> {
    let decoded = ling_semantic::read_json(json).map_err(|error| error.to_string())?;
    if &decoded == graph {
        Ok(())
    } else {
        Err("semantic JSON round-trip changed the checked graph".to_owned())
    }
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
                    return Err(CompileFailure::Diagnostics(vec![
                        Diagnostic::new(
                            codes::INVALID_MODULE,
                            Severity::Error,
                            format!(
                                "模块声明“{actual}”与 import 名称“{imported_name}”不一致"
                            ),
                            format!(
                                "module declaration `{actual}` does not match import `{imported_name}`"
                            ),
                        )
                        .with_primary_span(DiagnosticSpan::new(
                            &program.source_name,
                            program.module.span,
                        ))
                        .with_fact("expected_module", imported_name.clone())
                        .with_fact("actual_module", actual),
                    ]));
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
        lower_source(source_id, display_path, bytes)
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

fn lower_source(
    source_id: SourceId,
    source_name: String,
    bytes: Vec<u8>,
) -> Result<HirProgram, CompileFailure> {
    lower_source_with_counters(
        source_id,
        source_name,
        bytes,
        ling_hir::IdCounters::default(),
    )
    .map(|(program, _)| program)
}

pub(crate) fn lower_source_with_counters(
    source_id: SourceId,
    source_name: String,
    bytes: Vec<u8>,
    counters: ling_hir::IdCounters,
) -> Result<(HirProgram, ling_hir::IdCounters), CompileFailure> {
    let source =
        SourceFile::from_bytes(source_id, source_name.clone(), bytes).map_err(|error| {
            CompileFailure::Diagnostics(vec![source_error_diagnostic(&source_name, error)])
        })?;
    let parsed = ling_syntax::parse(&source);
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
    ling_hir::lower_with_counters(source.name(), &ast, counters).map_err(|error| {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/cli")
            .join(name)
            .join("Main.ling")
    }

    fn compile_error_codes(path: &Path) -> Vec<String> {
        match compile_path(path) {
            Err(CompileFailure::Diagnostics(diagnostics)) => diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code().to_string())
                .collect(),
            Err(other) => panic!("expected public diagnostics, found {other:?}"),
            Ok(_) => panic!("fixture unexpectedly compiled: {}", path.display()),
        }
    }

    #[test]
    fn in_memory_and_file_orchestration_share_the_checked_pipeline() {
        let compiled = compile_source("session.ling", b"module Main\n\nlet answer = 42\n".to_vec())
            .expect("in-memory module compiles");
        assert_eq!(compiled.snapshot.graph().entry_module, "Main");
        assert!(
            compiled
                .snapshot
                .graph()
                .definitions
                .iter()
                .any(|definition| definition.name == "answer")
        );
    }

    #[test]
    fn snapshot_verifier_rejects_invalid_or_changed_json() {
        let compiled = compile_source(
            "snapshot.ling",
            b"module Main\n\nlet answer = 42\n".to_vec(),
        )
        .expect("source compiles");
        let graph = compiled.snapshot.graph();
        assert!(verify_snapshot_json(graph, compiled.snapshot.json()).is_ok());

        let wrong_schema = compiled.snapshot.json().replacen(
            ling_semantic::SEMANTIC_SCHEMA,
            "ling.semantic/9.9",
            1,
        );
        assert!(verify_snapshot_json(graph, &wrong_schema).is_err());

        let mut changed = graph.clone();
        changed.entry_module = "Changed".to_owned();
        assert_eq!(
            verify_snapshot_json(&changed, compiled.snapshot.json()),
            Err("semantic JSON round-trip changed the checked graph".to_owned())
        );
    }

    #[test]
    fn module_loader_enforces_seed_paths_declarations_aliases_and_cycles() {
        for (name, expected) in [
            ("module-missing", "L-NAME-0008"),
            ("module-mismatch", "L-NAME-0003"),
            ("module-case-mismatch", "L-NAME-0008"),
            ("module-duplicate-alias", "L-NAME-0004"),
            ("module-cycle-two", "L-NAME-0005"),
            ("module-cycle-three", "L-NAME-0005"),
        ] {
            assert_eq!(compile_error_codes(&fixture(name)), [expected], "{name}");
        }
    }
}
