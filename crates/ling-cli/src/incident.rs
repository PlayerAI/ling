//! Stable internal-error fingerprints and local reproduction reports.

use std::fs;
use std::path::{Path, PathBuf};

use ling_diagnostics::{Diagnostic, Severity, codes};

pub const INCIDENT_SCHEMA: &str = "ling.internal-incident/0.1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reproduction {
    command: String,
    input: Option<String>,
    submission: Option<u64>,
    source: Option<String>,
}

impl Reproduction {
    #[must_use]
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            input: None,
            submission: None,
            source: None,
        }
    }

    #[must_use]
    pub fn with_input(mut self, input: impl Into<String>) -> Self {
        self.input = Some(input.into());
        self
    }

    #[must_use]
    pub fn with_submission(mut self, submission: u64) -> Self {
        self.submission = Some(submission);
        self
    }

    #[must_use]
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InternalIncident {
    id: String,
    stage: String,
    report_path: Option<PathBuf>,
    report_error: Option<&'static str>,
}

impl InternalIncident {
    /// Records a minimal local reproduction report below the OS temporary directory.
    #[must_use]
    pub fn capture(
        stage: impl Into<String>,
        detail: impl Into<String>,
        reproduction: Reproduction,
    ) -> Self {
        Self::capture_in(
            std::env::temp_dir().join("ling-incidents"),
            stage.into(),
            detail.into(),
            reproduction,
        )
    }

    fn capture_in(
        report_root: PathBuf,
        stage: String,
        detail: String,
        reproduction: Reproduction,
    ) -> Self {
        let id = incident_id(&stage, &detail);
        let digest = id
            .rsplit(':')
            .next()
            .expect("incident ID contains a digest");
        let path = report_root.join(format!("incident-{digest}.json"));
        let report = report_json(&id, &stage, &detail, &reproduction);
        let result = fs::create_dir_all(&report_root).and_then(|()| fs::write(&path, report));
        let (report_path, report_error) = match result {
            Ok(()) => (Some(path), None),
            Err(error) => (None, Some(stable_io_kind(error.kind()))),
        };
        Self {
            id,
            stage,
            report_path,
            report_error,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn diagnostic(&self) -> Diagnostic {
        let report = self
            .report_path
            .as_deref()
            .map_or_else(|| "unavailable".to_owned(), report_label);
        let mut diagnostic = Diagnostic::new(
            codes::INTERNAL_COMPILER_ERROR,
            Severity::Error,
            format!("内部编译器错误；事件 ID：{}；重现信息：{report}", self.id),
            format!(
                "internal compiler error; incident ID: {}; reproduction: {report}",
                self.id
            ),
        )
        .with_fact("incident_id", self.id.clone())
        .with_fact("stage", self.stage.clone());
        if let Some(path) = &self.report_path {
            diagnostic = diagnostic.with_fact("reproduction", report_label(path));
        }
        if let Some(error) = self.report_error {
            diagnostic = diagnostic.with_fact("reproduction_error", error);
        }
        diagnostic
    }
}

fn incident_id(stage: &str, detail: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hash_part(&mut hasher, b"ling.internal-incident-id/v1");
    hash_part(&mut hasher, env!("CARGO_PKG_VERSION").as_bytes());
    hash_part(&mut hasher, stage.as_bytes());
    hash_part(&mut hasher, detail.as_bytes());
    format!("experimental:blake3:{}", hasher.finalize().to_hex())
}

fn hash_part(hasher: &mut blake3::Hasher, bytes: &[u8]) {
    hasher.update(&u64::try_from(bytes.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(bytes);
}

fn report_json(id: &str, stage: &str, detail: &str, reproduction: &Reproduction) -> Vec<u8> {
    let mut reproduction_json = serde_json::json!({
        "command": reproduction.command,
    });
    let object = reproduction_json
        .as_object_mut()
        .expect("JSON object literal");
    if let Some(input) = &reproduction.input {
        object.insert("input".to_owned(), serde_json::json!(input));
    }
    if let Some(submission) = reproduction.submission {
        object.insert("submission".to_owned(), serde_json::json!(submission));
    }
    if let Some(source) = &reproduction.source {
        object.insert("source".to_owned(), serde_json::json!(source));
    }
    let mut bytes = serde_json::to_vec_pretty(&serde_json::json!({
        "schema": INCIDENT_SCHEMA,
        "incident_id": id,
        "compiler_version": env!("CARGO_PKG_VERSION"),
        "stage": stage,
        "detail": detail,
        "reproduction": reproduction_json,
    }))
    .expect("incident report contains only JSON-compatible values");
    bytes.push(b'\n');
    bytes
}

fn report_label(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map_or_else(|| "incident.json".into(), |name| name.to_string_lossy());
    format!("os-temp/ling-incidents/{file_name}")
}

const fn stable_io_kind(kind: std::io::ErrorKind) -> &'static str {
    match kind {
        std::io::ErrorKind::NotFound => "not_found",
        std::io::ErrorKind::PermissionDenied => "permission_denied",
        std::io::ErrorKind::AlreadyExists => "already_exists",
        std::io::ErrorKind::InvalidInput => "invalid_input",
        std::io::ErrorKind::InvalidData => "invalid_data",
        std::io::ErrorKind::WriteZero => "write_zero",
        std::io::ErrorKind::Interrupted => "interrupted",
        std::io::ErrorKind::OutOfMemory => "out_of_memory",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incident_ids_are_stable_and_reports_are_structured() {
        let reproduction = Reproduction::new("ling check")
            .with_input("example.ling")
            .with_submission(7)
            .with_source("let value = 1");
        let first_id = incident_id("semantic.build", "serialization invariant failed");
        let second_id = incident_id("semantic.build", "serialization invariant failed");
        assert_eq!(first_id, second_id);
        assert!(first_id.starts_with("experimental:blake3:"));

        let report = report_json(
            &first_id,
            "semantic.build",
            "serialization invariant failed",
            &reproduction,
        );
        let report: serde_json::Value = serde_json::from_slice(&report).expect("valid report JSON");
        assert_eq!(report["schema"], INCIDENT_SCHEMA);
        assert_eq!(report["incident_id"], first_id);
        assert_eq!(report["reproduction"]["command"], "ling check");
        assert_eq!(report["reproduction"]["input"], "example.ling");
        assert_eq!(report["reproduction"]["submission"], 7);
        assert_eq!(report["reproduction"]["source"], "let value = 1");
    }

    #[test]
    fn capture_saves_a_windows_compatible_reproduction_file() {
        let incident = InternalIncident::capture_in(
            std::env::temp_dir().join("ling-cli-incident-tests"),
            "audit.render".to_owned(),
            "invariant failed".to_owned(),
            Reproduction::new("ling audit").with_input("example.ling"),
        );
        assert_eq!(incident.report_error, None);
        let diagnostic: serde_json::Value = serde_json::from_str(
            &incident
                .diagnostic()
                .render_json()
                .expect("internal diagnostic renders"),
        )
        .expect("internal diagnostic is JSON");
        assert_eq!(diagnostic["code"], "L-INTERNAL-0001");
        assert_eq!(diagnostic["facts"]["incident_id"], incident.id);
        assert!(
            diagnostic["facts"]["reproduction"]
                .as_str()
                .expect("logical reproduction location")
                .starts_with("os-temp/ling-incidents/incident-")
        );
        let path = incident.report_path.expect("report path");
        assert!(path.is_file());
        assert!(!path.to_string_lossy().contains("experimental:blake3:"));
        let report: serde_json::Value =
            serde_json::from_slice(&fs::read(path).expect("saved incident report is readable"))
                .expect("saved report is valid JSON");
        assert_eq!(report["incident_id"], incident.id);
    }
}
