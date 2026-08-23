//! Shared orchestration for one explicitly selected locked RFC-0002 project.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use ling_db::{CompilerDb, ProjectSnapshotError, QueryError};
use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_project::{LockedGraphFailure, LockedProject, Manifest, parse_manifest};
use ling_semantic::ProjectProgramSnapshot;
use sha2::{Digest, Sha256};

pub const COMMAND_PROTOCOL: &str = "ling.project.command/0.1";
pub const ARTIFACT_PROTOCOL: &str = "ling.project.artifact/0.1";
pub const BUILD_PROFILE: &str = "explore";
pub const BUILD_TARGET: &str = "semantic";

#[derive(Debug)]
pub struct CheckedProject {
    manifest: Manifest,
    locked: LockedProject,
    snapshot: Arc<ProjectProgramSnapshot>,
}

impl CheckedProject {
    #[must_use]
    pub const fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    #[must_use]
    pub const fn locked(&self) -> &LockedProject {
        &self.locked
    }

    #[must_use]
    pub fn snapshot(&self) -> &ProjectProgramSnapshot {
        &self.snapshot
    }
}

#[derive(Debug)]
pub enum ProjectFailure {
    Diagnostics(Vec<Diagnostic>),
    Internal(String),
    SnapshotMismatch(String),
    ArtifactIo {
        operation: &'static str,
        kind: io::ErrorKind,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectArtifact {
    bytes: Vec<u8>,
    identity: String,
}

impl ProjectArtifact {
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

/// Loads and semantically checks exactly one explicit locked project.
pub fn compile(manifest_path: &Path) -> Result<CheckedProject, ProjectFailure> {
    let project_root = manifest_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let bytes = fs::read(manifest_path).map_err(|error| {
        ProjectFailure::Diagnostics(vec![manifest_read_diagnostic(error.kind())])
    })?;
    let manifest = parse_manifest("ling.toml", &bytes)
        .map_err(|error| ProjectFailure::Diagnostics(vec![error.diagnostic()]))?;
    let locked =
        ling_project::load_locked_project(project_root, &manifest).map_err(locked_failure)?;
    let mut compiler = CompilerDb::new();
    let snapshot = compiler
        .project_semantic_snapshot(&locked)
        .map_err(snapshot_failure)?;
    verify_snapshot(&snapshot)?;
    Ok(CheckedProject {
        manifest,
        locked,
        snapshot,
    })
}

/// Creates and exclusively publishes one canonical checked semantic artifact.
pub fn build(project: &CheckedProject, output: PathBuf) -> Result<ProjectArtifact, ProjectFailure> {
    let artifact = encode_artifact(project)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output)
        .map_err(|error| ProjectFailure::ArtifactIo {
            operation: "create_artifact",
            kind: error.kind(),
        })?;
    if let Err(error) = file
        .write_all(artifact.bytes())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = fs::remove_file(&output);
        return Err(ProjectFailure::ArtifactIo {
            operation: "write_artifact",
            kind: error.kind(),
        });
    }
    Ok(artifact)
}

fn encode_artifact(project: &CheckedProject) -> Result<ProjectArtifact, ProjectFailure> {
    let graph = project.locked.graph().id().as_str();
    let program = project.snapshot.program_id().to_string();
    let semantic = project.snapshot.json();
    let decoded = ling_semantic::read_project_json(semantic)
        .map_err(|error| ProjectFailure::SnapshotMismatch(error.to_string()))?;
    if &decoded != project.snapshot.graph() {
        return Err(ProjectFailure::SnapshotMismatch(
            "semantic JSON round-trip changed the checked project graph".to_owned(),
        ));
    }

    let graph = serde_json::to_string(graph)
        .map_err(|error| ProjectFailure::Internal(error.to_string()))?;
    let program = serde_json::to_string(&program)
        .map_err(|error| ProjectFailure::Internal(error.to_string()))?;
    let mut bytes = format!(
        concat!(
            "{{\"graph\":{graph},\"profile\":\"explore\",",
            "\"program\":{program},",
            "\"protocol\":\"ling.project.artifact/0.1\",",
            "\"semantic\":{semantic},\"target\":\"semantic\"}}\n"
        ),
        graph = graph,
        program = program,
        semantic = semantic,
    )
    .into_bytes();
    debug_assert_eq!(bytes.last(), Some(&b'\n'));
    let digest = Sha256::digest(&bytes);
    let mut identity = String::with_capacity(71);
    identity.push_str("sha256:");
    for byte in digest {
        use std::fmt::Write as _;
        write!(identity, "{byte:02x}").expect("writing a SHA-256 digest to String cannot fail");
    }
    bytes.shrink_to_fit();
    Ok(ProjectArtifact { bytes, identity })
}

fn verify_snapshot(snapshot: &ProjectProgramSnapshot) -> Result<(), ProjectFailure> {
    let decoded = ling_semantic::read_project_json(snapshot.json())
        .map_err(|error| ProjectFailure::SnapshotMismatch(error.to_string()))?;
    if &decoded == snapshot.graph() {
        Ok(())
    } else {
        Err(ProjectFailure::SnapshotMismatch(
            "semantic JSON round-trip changed the checked project graph".to_owned(),
        ))
    }
}

fn locked_failure(failure: LockedGraphFailure) -> ProjectFailure {
    failure.diagnostics().map_or_else(
        || ProjectFailure::Internal(failure.to_string()),
        |diagnostics| ProjectFailure::Diagnostics(diagnostics.to_vec()),
    )
}

fn snapshot_failure(failure: QueryError) -> ProjectFailure {
    match failure {
        QueryError::ProjectSnapshot { error } => project_snapshot_failure(error),
        other => ProjectFailure::Internal(other.to_string()),
    }
}

fn project_snapshot_failure(failure: ProjectSnapshotError) -> ProjectFailure {
    failure.diagnostics().map_or_else(
        || ProjectFailure::Internal(failure.to_string()),
        ProjectFailure::Diagnostics,
    )
}

fn manifest_read_diagnostic(kind: io::ErrorKind) -> Diagnostic {
    Diagnostic::new(
        codes::SOURCE_READ_FAILED,
        Severity::Error,
        "无法读取工程 manifest `ling.toml`",
        "failed to read project manifest `ling.toml`",
    )
    .with_primary_span(DiagnosticSpan::at("ling.toml", 0, 0))
    .with_fact("io_kind", stable_io_kind(kind))
}

#[must_use]
pub const fn stable_io_kind(kind: io::ErrorKind) -> &'static str {
    match kind {
        io::ErrorKind::NotFound => "not_found",
        io::ErrorKind::PermissionDenied => "permission_denied",
        io::ErrorKind::AlreadyExists => "already_exists",
        io::ErrorKind::InvalidInput => "invalid_input",
        io::ErrorKind::InvalidData => "invalid_data",
        io::ErrorKind::Interrupted => "interrupted",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/projects/offline-lock/ling.toml")
    }

    #[test]
    fn compile_uses_the_checked_project_snapshot_and_is_repeatable() {
        let first = compile(&fixture()).expect("fixture project compiles");
        let second = compile(&fixture()).expect("fixture project recompiles");
        assert_eq!(first.snapshot().json(), second.snapshot().json());
        assert_eq!(first.locked().graph().id(), second.locked().graph().id());
    }

    #[test]
    fn artifact_bytes_are_canonical_path_free_and_identity_bound() {
        let project = compile(&fixture()).expect("fixture project compiles");
        let first = encode_artifact(&project).expect("artifact encodes");
        let second = encode_artifact(&project).expect("artifact re-encodes");
        assert_eq!(first, second);
        assert!(first.bytes().ends_with(b"\n"));
        assert!(first.identity().starts_with("sha256:"));
        assert!(
            !String::from_utf8_lossy(first.bytes()).contains(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .to_string_lossy()
                    .as_ref()
            )
        );
        let value: serde_json::Value =
            serde_json::from_slice(first.bytes()).expect("artifact is JSON");
        assert_eq!(value["protocol"], ARTIFACT_PROTOCOL);
        assert_eq!(value["profile"], BUILD_PROFILE);
        assert_eq!(value["target"], BUILD_TARGET);
        assert_eq!(value["semantic"]["schema"], "ling.semantic/0.2");
    }
}
