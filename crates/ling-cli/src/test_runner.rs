use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_effects::locate_main;
use ling_eval::{MemoryConsole, execute_main};

use crate::{CompileFailure, checked_execution_implementation_boundary, compile_path};

pub(crate) const TEST_PROTOCOL: &str = "ling.test/0.1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum CaseStatus {
    Passed,
    CompileFailed,
    RuntimeFailed,
}

impl CaseStatus {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::CompileFailed => "compile_failed",
            Self::RuntimeFailed => "runtime_failed",
        }
    }
}

#[derive(Debug)]
pub(crate) struct CaseReport {
    name: String,
    status: CaseStatus,
    stdout: String,
    diagnostics: Vec<Diagnostic>,
}

impl CaseReport {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn status(&self) -> CaseStatus {
        self.status
    }

    pub(crate) fn stdout(&self) -> &str {
        &self.stdout
    }

    fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

#[derive(Debug)]
pub(crate) struct Summary {
    root: String,
    cases: Vec<CaseReport>,
    failure: FailureClass,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum FailureClass {
    None,
    Compile,
    Runtime,
}

impl Summary {
    pub(crate) fn root(&self) -> &str {
        &self.root
    }

    pub(crate) fn cases(&self) -> &[CaseReport] {
        &self.cases
    }

    pub(crate) fn total(&self) -> usize {
        self.cases.len()
    }

    pub(crate) fn passed(&self) -> usize {
        self.cases
            .iter()
            .filter(|case| case.status == CaseStatus::Passed)
            .count()
    }

    pub(crate) fn failed(&self) -> usize {
        self.total().saturating_sub(self.passed())
    }

    pub(crate) fn exit_code(&self) -> u8 {
        match self.failure {
            FailureClass::None => crate::exit_catalog::EXIT_SUCCESS,
            FailureClass::Compile => crate::exit_catalog::EXIT_COMPILE_ERROR,
            FailureClass::Runtime => crate::exit_catalog::EXIT_RUNTIME_FAULT,
        }
    }

    pub(crate) fn status(&self) -> &'static str {
        if self.failure == FailureClass::None {
            "ok"
        } else {
            "failed"
        }
    }

    pub(crate) fn diagnostics(&self) -> Vec<Diagnostic> {
        self.cases
            .iter()
            .flat_map(|case| case.diagnostics().iter().cloned())
            .collect()
    }
}

#[derive(Debug)]
pub(crate) enum Failure {
    Usage(String),
    Io {
        operation: &'static str,
        path: String,
        kind: io::ErrorKind,
    },
    NoCases {
        root: String,
    },
    Internal(String),
    Snapshot(String),
}

impl Failure {
    pub(crate) fn diagnostic(&self) -> Option<Diagnostic> {
        match self {
            Self::Io {
                operation,
                path,
                kind,
            } => Some(
                Diagnostic::new(
                    codes::PROJECT_TEST_IO_FAILED,
                    Severity::Error,
                    "测试输入文件操作失败",
                    "test input file operation failed",
                )
                .with_primary_span(DiagnosticSpan::at(path.clone(), 0, 0))
                .with_fact("io_kind", crate::init::stable_io_kind(*kind))
                .with_fact("operation", *operation)
                .with_fact("path", path.clone()),
            ),
            Self::NoCases { root } => Some(
                Diagnostic::new(
                    codes::TEST_NO_CASES,
                    Severity::Error,
                    "没有发现可运行的 `.ling` 测试文件",
                    "no runnable `.ling` test files were found",
                )
                .with_primary_span(DiagnosticSpan::at("ling test", 0, 0))
                .with_fact("root", root.clone()),
            ),
            Self::Usage(_) | Self::Internal(_) | Self::Snapshot(_) => None,
        }
    }
}

#[derive(Debug)]
struct TestFile {
    path: PathBuf,
    name: String,
}

pub(crate) fn run(root: PathBuf) -> Result<Summary, Failure> {
    let root_name = root
        .to_str()
        .ok_or_else(|| Failure::Usage("test paths must be valid UTF-8".to_owned()))?
        .to_owned();
    let files = discover(&root)?;
    if files.is_empty() {
        return Err(Failure::NoCases { root: root_name });
    }

    let mut cases = Vec::with_capacity(files.len());
    let mut failure = FailureClass::None;
    for file in files {
        let case = match compile_path(&file.path) {
            Ok(compiled) => {
                if let Some(diagnostic) =
                    checked_execution_implementation_boundary(compiled.snapshot.checked(), "test")
                {
                    CaseReport {
                        name: file.name,
                        status: CaseStatus::CompileFailed,
                        stdout: String::new(),
                        diagnostics: vec![diagnostic],
                    }
                } else {
                    match locate_main(compiled.snapshot.checked()) {
                        Ok(main) => {
                            let mut console = MemoryConsole::default();
                            match execute_main(&compiled.snapshot, &main, &mut console) {
                                Ok(()) => CaseReport {
                                    name: file.name,
                                    status: CaseStatus::Passed,
                                    stdout: console.output().to_owned(),
                                    diagnostics: Vec::new(),
                                },
                                Err(fault) => CaseReport {
                                    name: file.name,
                                    status: CaseStatus::RuntimeFailed,
                                    stdout: console.output().to_owned(),
                                    diagnostics: vec![fault.to_diagnostic()],
                                },
                            }
                        }
                        Err(error) => CaseReport {
                            name: file.name,
                            status: CaseStatus::CompileFailed,
                            stdout: String::new(),
                            diagnostics: vec![error.to_diagnostic()],
                        },
                    }
                }
            }
            Err(CompileFailure::Diagnostics(diagnostics)) => CaseReport {
                name: file.name,
                status: CaseStatus::CompileFailed,
                stdout: String::new(),
                diagnostics,
            },
            Err(CompileFailure::Internal(message)) => {
                return Err(Failure::Internal(format!("{}: {message}", file.name)));
            }
            Err(CompileFailure::SnapshotMismatch(message)) => {
                return Err(Failure::Snapshot(format!("{}: {message}", file.name)));
            }
        };
        failure = failure.max(match case.status {
            CaseStatus::Passed => FailureClass::None,
            CaseStatus::CompileFailed => FailureClass::Compile,
            CaseStatus::RuntimeFailed => FailureClass::Runtime,
        });
        cases.push(case);
    }

    Ok(Summary {
        root: root_name,
        cases,
        failure,
    })
}

fn discover(root: &Path) -> Result<Vec<TestFile>, Failure> {
    let metadata = fs::symlink_metadata(root).map_err(|error| Failure::Io {
        operation: "read_root",
        path: root.to_string_lossy().into_owned(),
        kind: error.kind(),
    })?;
    if metadata.file_type().is_symlink() {
        return Err(Failure::Io {
            operation: "reject_symlink_root",
            path: root.to_string_lossy().into_owned(),
            kind: io::ErrorKind::InvalidInput,
        });
    }
    if metadata.is_file() {
        if root.extension().and_then(|value| value.to_str()) != Some("ling") {
            return Err(Failure::Usage(
                "`test` file operands must end in `.ling`".to_owned(),
            ));
        }
        let name = root
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| Failure::Usage("test paths must be valid UTF-8".to_owned()))?;
        return Ok(vec![TestFile {
            path: root.to_owned(),
            name: name.to_owned(),
        }]);
    }
    if !metadata.is_dir() {
        return Err(Failure::Usage(
            "`test` requires a regular `.ling` file or directory".to_owned(),
        ));
    }

    let mut files = Vec::new();
    walk_directory(root, root, &mut files)?;
    files.sort_by(|left, right| left.name.as_bytes().cmp(right.name.as_bytes()));
    Ok(files)
}

fn walk_directory(root: &Path, current: &Path, files: &mut Vec<TestFile>) -> Result<(), Failure> {
    let mut entries = fs::read_dir(current)
        .map_err(|error| Failure::Io {
            operation: "read_directory",
            path: current.to_string_lossy().into_owned(),
            kind: error.kind(),
        })?
        .map(|entry| {
            entry
                .map(|entry| {
                    let name = entry.file_name().to_str().map(str::to_owned);
                    (name, entry.path())
                })
                .map_err(|error| Failure::Io {
                    operation: "read_directory_entry",
                    path: current.to_string_lossy().into_owned(),
                    kind: error.kind(),
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by(|left, right| left.0.cmp(&right.0));

    for (name, path) in entries {
        let _name =
            name.ok_or_else(|| Failure::Usage("test paths must be valid UTF-8".to_owned()))?;
        let metadata = fs::symlink_metadata(&path).map_err(|error| Failure::Io {
            operation: "inspect_test_input",
            path: path.to_string_lossy().into_owned(),
            kind: error.kind(),
        })?;
        if metadata.file_type().is_symlink() {
            return Err(Failure::Io {
                operation: "reject_symlink_input",
                path: path.to_string_lossy().into_owned(),
                kind: io::ErrorKind::InvalidInput,
            });
        }
        if metadata.is_dir() {
            walk_directory(root, &path, files)?;
        } else if metadata.is_file()
            && path.extension().and_then(|value| value.to_str()) == Some("ling")
        {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| Failure::Internal("test path escaped its root".to_owned()))?;
            let relative = relative
                .components()
                .map(|component| component.as_os_str().to_str())
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| Failure::Usage("test paths must be valid UTF-8".to_owned()))?
                .join("/");
            files.push(TestFile {
                path,
                name: relative,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQUENCE: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn runs_one_standalone_program_and_captures_console_output() {
        let root = temp_root("single");
        let path = root.join("case.ling");
        write(
            &path,
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"ok\"\n",
        );

        let summary = run(path).expect("test file runs");
        assert_eq!(summary.status(), "ok");
        assert_eq!(summary.total(), 1);
        assert_eq!(summary.cases()[0].name(), "case.ling");
        assert_eq!(summary.cases()[0].stdout(), "ok\n");
        cleanup(&root);
    }

    #[test]
    fn directory_order_is_logical_and_compile_failures_do_not_stop_later_files() {
        let root = temp_root("directory");
        write(&root.join("z.ling"), "module Main\n\nlet main () = ()\n");
        write(
            &root.join("a.ling"),
            "module Main\n\nlet main () = missing\n",
        );

        let summary = run(root.clone()).expect("directory discovery succeeds");
        assert_eq!(summary.status(), "failed");
        assert_eq!(summary.cases()[0].name(), "a.ling");
        assert_eq!(summary.cases()[0].status(), CaseStatus::CompileFailed);
        assert_eq!(summary.cases()[1].name(), "z.ling");
        assert_eq!(summary.cases()[1].status(), CaseStatus::Passed);
        assert_eq!(summary.exit_code(), crate::exit_catalog::EXIT_COMPILE_ERROR);
        cleanup(&root);
    }

    #[test]
    fn empty_directory_is_a_stable_test_failure() {
        let root = temp_root("empty");
        let failure = run(root.clone()).expect_err("empty selection fails");
        assert!(matches!(failure, Failure::NoCases { .. }));
        assert_eq!(failure.diagnostic().unwrap().code().as_str(), "L-TEST-0001");
        cleanup(&root);
    }

    fn temp_root(label: &str) -> PathBuf {
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ling-cli-1704-{label}-{}-{sequence}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale test root is removable");
        }
        fs::create_dir_all(&root).expect("test root is creatable");
        root
    }

    fn write(path: &Path, contents: &str) {
        fs::write(path, contents).expect("test file is writable");
    }

    fn cleanup(path: &Path) {
        fs::remove_dir_all(path).expect("test root is removable");
    }
}
