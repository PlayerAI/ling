use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_project::{
    DiscoveryFailure, LockMode, LockedGraphFailure, MANIFEST_FILE_NAME, discover_modules,
    parse_manifest, resolve_package_graph_with_lock,
};

pub(crate) const INIT_PROTOCOL: &str = "ling.init/0.1";
pub(crate) const INIT_TEMPLATE_VERSION: &str = "1";
const MAIN_SOURCE: &[u8] = b"let main () = ()\n";
const GITIGNORE: &[u8] = b"target/\n.ling-cache/\n";
const MAX_STAGING_ATTEMPTS: usize = 1_024;
static STAGING_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub(crate) enum Failure {
    Usage(String),
    Diagnostics(Vec<Diagnostic>),
    Internal(String),
    Io {
        operation: &'static str,
        kind: io::ErrorKind,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ResultSummary {
    pub(crate) directory: String,
    pub(crate) package_name: String,
    pub(crate) files: Vec<String>,
}

pub(crate) fn create(
    destination: PathBuf,
    package_name: Option<String>,
    display_name: Option<String>,
) -> Result<ResultSummary, Failure> {
    validate_destination(&destination)?;
    let package_name = package_name
        .or_else(|| {
            destination
                .file_name()
                .and_then(|name| name.to_str())
                .map(str::to_owned)
        })
        .ok_or_else(|| {
            Failure::Usage(
                "`init` destination must have a UTF-8 final component or use `--name`".to_owned(),
            )
        })?;
    if !valid_package_name(&package_name) {
        return Err(Failure::Usage(format!(
            "invalid package name `{package_name}`; use lowercase ASCII letters, digits, and hyphens"
        )));
    }

    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let staging = StagingDir::create(parent, destination.file_name())?;
    let manifest = manifest_bytes(&package_name, display_name.as_deref());
    write_new_file(
        &staging.path.join(MANIFEST_FILE_NAME),
        &manifest,
        "write manifest",
    )?;
    let source_root = staging.path.join("src");
    fs::create_dir(&source_root).map_err(|error| Failure::Io {
        operation: "create source root",
        kind: error.kind(),
    })?;
    write_new_file(
        &source_root.join("Main.ling"),
        MAIN_SOURCE,
        "write entry source",
    )?;
    write_new_file(
        &staging.path.join(".gitignore"),
        GITIGNORE,
        "write gitignore",
    )?;

    let parsed = parse_manifest(MANIFEST_FILE_NAME, &manifest)
        .map_err(|error| Failure::Diagnostics(vec![error.diagnostic()]))?;
    match discover_modules(&staging.path, &parsed) {
        Ok(_) => {}
        Err(DiscoveryFailure::Diagnostics(diagnostics)) => {
            return Err(Failure::Diagnostics(diagnostics.into_vec()));
        }
        Err(DiscoveryFailure::Internal(message)) => return Err(Failure::Internal(message)),
    }
    match resolve_package_graph_with_lock(&staging.path, &parsed, LockMode::Update) {
        Ok(_) => {}
        Err(failure) => return Err(lock_failure(failure)),
    }

    staging.commit(&destination)?;
    Ok(ResultSummary {
        directory: destination.to_string_lossy().into_owned(),
        package_name,
        files: vec![
            ".gitignore".to_owned(),
            "ling.lock".to_owned(),
            "ling.toml".to_owned(),
            "src/Main.ling".to_owned(),
        ],
    })
}

fn validate_destination(destination: &Path) -> Result<(), Failure> {
    if destination.as_os_str().is_empty() || destination == Path::new("-") {
        return Err(Failure::Usage(
            "`init` requires a non-empty destination directory, not `-`".to_owned(),
        ));
    }
    if destination.exists() {
        return Err(Failure::Usage(format!(
            "init destination already exists: {}",
            destination.display()
        )));
    }
    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !parent.is_dir() {
        return Err(Failure::Usage(format!(
            "init parent is not an existing directory: {}",
            parent.display()
        )));
    }
    Ok(())
}

fn valid_package_name(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes.len() > 63 || !bytes[0].is_ascii_lowercase() {
        return false;
    }
    let mut previous_hyphen = false;
    for (index, byte) in bytes.iter().copied().enumerate() {
        if byte == b'-' {
            if index == 0 || previous_hyphen || index + 1 == bytes.len() {
                return false;
            }
            previous_hyphen = true;
        } else if !byte.is_ascii_lowercase() && !byte.is_ascii_digit() {
            return false;
        } else {
            previous_hyphen = false;
        }
    }
    true
}

fn manifest_bytes(package_name: &str, display_name: Option<&str>) -> Vec<u8> {
    let mut output = format!("manifest-version = 1\n\n[package]\nname = \"{package_name}\"\n");
    if let Some(display_name) = display_name {
        output.push_str("display-name = \"");
        output.push_str(&toml_basic_string(display_name));
        output.push_str("\"\n");
    }
    output.push_str(
        "version = \"0.1.0\"\nlanguage = \"0.1\"\n\n[source]\nroots = [\"src\"]\nentry = \"Main\"\n",
    );
    output.into_bytes()
}

fn toml_basic_string(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '\\' => ['\\', '\\'].into_iter().collect::<Vec<_>>(),
            '"' => ['\\', '"'].into_iter().collect::<Vec<_>>(),
            character => [character].into_iter().collect::<Vec<_>>(),
        })
        .collect()
}

fn write_new_file(path: &Path, bytes: &[u8], operation: &'static str) -> Result<(), Failure> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Failure::Io {
            operation,
            kind: error.kind(),
        })?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| Failure::Io {
            operation,
            kind: error.kind(),
        })
}

fn lock_failure(failure: LockedGraphFailure) -> Failure {
    if let Some(diagnostics) = failure.diagnostics() {
        Failure::Diagnostics(diagnostics.to_vec())
    } else {
        Failure::Internal(failure.to_string())
    }
}

fn io_diagnostic(operation: &'static str, kind: io::ErrorKind) -> Diagnostic {
    Diagnostic::new(
        codes::PROJECT_INIT_IO_FAILED,
        Severity::Error,
        "工程脚手架文件操作失败",
        "project scaffold file operation failed",
    )
    .with_fact("io_kind", stable_io_kind(kind))
    .with_fact("operation", operation)
    .with_primary_span(DiagnosticSpan::at("ling init", 0, 0))
}

fn stable_io_kind(kind: io::ErrorKind) -> &'static str {
    match kind {
        io::ErrorKind::AlreadyExists => "already_exists",
        io::ErrorKind::NotFound => "not_found",
        io::ErrorKind::PermissionDenied => "permission_denied",
        io::ErrorKind::ReadOnlyFilesystem => "read_only_filesystem",
        io::ErrorKind::InvalidInput => "invalid_input",
        io::ErrorKind::InvalidData => "invalid_data",
        io::ErrorKind::Interrupted => "interrupted",
        io::ErrorKind::WouldBlock => "would_block",
        _ => "other",
    }
}

struct StagingDir {
    path: PathBuf,
    committed: bool,
}

impl StagingDir {
    fn create(parent: &Path, destination: Option<&std::ffi::OsStr>) -> Result<Self, Failure> {
        let label = destination
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .unwrap_or("project");
        for _ in 0..MAX_STAGING_ATTEMPTS {
            let sequence = STAGING_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!(
                ".{label}.ling-init-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => {
                    return Ok(Self {
                        path,
                        committed: false,
                    });
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => {
                    return Err(Failure::Io {
                        operation: "create staging directory",
                        kind: error.kind(),
                    });
                }
            }
        }
        Err(Failure::Io {
            operation: "reserve staging directory",
            kind: io::ErrorKind::AlreadyExists,
        })
    }

    fn commit(mut self, destination: &Path) -> Result<(), Failure> {
        fs::rename(&self.path, destination).map_err(|error| Failure::Io {
            operation: "commit staging directory",
            kind: error.kind(),
        })?;
        self.committed = true;
        Ok(())
    }
}

impl Drop for StagingDir {
    fn drop(&mut self) {
        if !self.committed {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

pub(crate) fn diagnostic_for_failure(failure: &Failure) -> Option<Diagnostic> {
    match failure {
        Failure::Io { operation, kind } => Some(io_diagnostic(operation, *kind)),
        Failure::Diagnostics(_) | Failure::Usage(_) | Failure::Internal(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new() -> Self {
            let sequence = TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("ling-init-test-{}-{sequence}", std::process::id()));
            fs::create_dir_all(&path).expect("create init test root");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn creates_valid_minimal_project_and_lock() {
        let root = TempRoot::new();
        let destination = root.path().join("hello");
        let summary = create(destination.clone(), None, None).expect("init succeeds");
        assert_eq!(summary.package_name, "hello");
        assert_eq!(
            summary.files,
            [".gitignore", "ling.lock", "ling.toml", "src/Main.ling"]
        );
        let manifest = fs::read(destination.join("ling.toml")).expect("manifest exists");
        parse_manifest(MANIFEST_FILE_NAME, &manifest).expect("generated manifest is valid");
        assert_eq!(
            fs::read(destination.join("src/Main.ling")).unwrap(),
            MAIN_SOURCE
        );
        assert_eq!(fs::read(destination.join(".gitignore")).unwrap(), GITIGNORE);
        let lock = fs::read(destination.join("ling.lock")).expect("lock exists");
        assert!(serde_json::from_slice::<serde_json::Value>(&lock).is_ok());
        assert!(!destination.join(".ling-init").exists());
    }

    #[test]
    fn explicit_name_and_display_name_are_encoded_deterministically() {
        let root = TempRoot::new();
        let destination = root.path().join("starter");
        create(
            destination.clone(),
            Some("hello-world".to_owned()),
            Some("你好\\\"Ling".to_owned()),
        )
        .expect("init with metadata succeeds");
        let manifest = fs::read_to_string(destination.join("ling.toml")).unwrap();
        assert!(manifest.contains("name = \"hello-world\""));
        assert!(manifest.contains("display-name = \"你好\\\\\\\"Ling\""));
        parse_manifest(MANIFEST_FILE_NAME, manifest.as_bytes()).expect("metadata is valid");
    }

    #[test]
    fn rejects_existing_destination_without_mutation() {
        let root = TempRoot::new();
        let destination = root.path().join("existing");
        fs::create_dir(&destination).unwrap();
        fs::write(destination.join("keep.txt"), b"keep").unwrap();
        let error = create(destination.clone(), None, None).unwrap_err();
        assert!(matches!(error, Failure::Usage(message) if message.contains("already exists")));
        assert_eq!(fs::read(destination.join("keep.txt")).unwrap(), b"keep");
    }

    #[test]
    fn invalid_metadata_leaves_no_destination_or_staging() {
        let root = TempRoot::new();
        let destination = root.path().join("hello");
        let error = create(destination.clone(), None, Some("bad\nname".to_owned())).unwrap_err();
        assert!(matches!(error, Failure::Diagnostics(_)));
        assert!(!destination.exists());
        let entries = fs::read_dir(root.path())
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn package_name_validation_matches_rfc_0002_shape() {
        for valid in ["a", "hello", "hello-world", "a1-b2"] {
            assert!(valid_package_name(valid), "{valid}");
        }
        for invalid in ["", "A", "-hello", "hello-", "hello--world", "hello_world"] {
            assert!(!valid_package_name(invalid), "{invalid}");
        }
    }
}
