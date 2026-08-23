use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path};

use serde::Deserialize;

const MANIFEST_PATH: &str = "docs/governance/migration-readiness.toml";
const REPORT_PATH: &str = "docs/governance/migration-readiness.md";
const COMMAND_CATALOG_PATH: &str = "crates/ling-cli/src/command_catalog.rs";
const EXPECTED_REQUIREMENTS: &[&str] = &[
    "parser-semantic-transaction",
    "dry-run",
    "semantic-diff",
    "stale-edit-check",
    "backup-transaction",
    "formatter",
    "post-check-test",
    "machine-readable-report",
    "human-choice-stop",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadinessManifest {
    schema_version: u32,
    authority: String,
    released_source_versions: usize,
    accepted_version_pair: bool,
    public_command: String,
    report: String,
    requirement: Vec<Requirement>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Requirement {
    id: String,
    state: String,
    blocker: String,
    evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub requirement_count: usize,
    pub unavailable_count: usize,
    pub released_source_versions: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let manifest = load_manifest(root)?;
    let catalog = fs::read_to_string(root.join(COMMAND_CATALOG_PATH)).map_err(|error| {
        vec![format!(
            "GOV-MIGRATE-0001: cannot read {COMMAND_CATALOG_PATH}: {error}"
        )]
    })?;
    let mut errors = validate(root, &manifest, &catalog);
    let report = render(&manifest);
    match fs::read_to_string(root.join(REPORT_PATH)) {
        Ok(actual) if normalize_newlines(&actual) == report => {}
        Ok(_) => errors.push(format!(
            "GOV-MIGRATE-0008: {REPORT_PATH} is stale; run cargo xtask migration render"
        )),
        Err(error) => errors.push(format!(
            "GOV-MIGRATE-0001: cannot read {REPORT_PATH}: {error}"
        )),
    }
    finish(errors).map(|()| summary(&manifest))
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    let manifest = load_manifest(root)?;
    let catalog = fs::read_to_string(root.join(COMMAND_CATALOG_PATH)).map_err(|error| {
        vec![format!(
            "GOV-MIGRATE-0001: cannot read {COMMAND_CATALOG_PATH}: {error}"
        )]
    })?;
    finish(validate(root, &manifest, &catalog)).map(|()| render(&manifest))
}

fn load_manifest(root: &Path) -> Result<ReadinessManifest, Vec<String>> {
    let text = fs::read_to_string(root.join(MANIFEST_PATH)).map_err(|error| {
        vec![format!(
            "GOV-MIGRATE-0001: cannot read {MANIFEST_PATH}: {error}"
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-MIGRATE-0002: invalid migration readiness manifest: {error}"
        )]
    })
}

fn validate(root: &Path, manifest: &ReadinessManifest, catalog: &str) -> Vec<String> {
    let mut errors = Vec::new();
    if manifest.schema_version != 1
        || manifest.authority != "DEC-0232"
        || manifest.released_source_versions != 1
        || manifest.accepted_version_pair
        || manifest.public_command != "Absent"
        || manifest.report != REPORT_PATH
    {
        errors.push(
            "GOV-MIGRATE-0003: readiness markers must remain schema 1, DEC-0232, one released source version, no accepted pair, Absent command, and the registered report"
                .to_owned(),
        );
    }

    let actual = manifest
        .requirement
        .iter()
        .map(|requirement| requirement.id.as_str())
        .collect::<Vec<_>>();
    if actual != EXPECTED_REQUIREMENTS {
        errors.push(format!(
            "GOV-MIGRATE-0004: requirement order/set differs; expected {EXPECTED_REQUIREMENTS:?}, found {actual:?}"
        ));
    }
    let mut ids = BTreeSet::new();
    for requirement in &manifest.requirement {
        if !ids.insert(requirement.id.as_str())
            || requirement.state != "Unavailable"
            || requirement.blocker.trim().is_empty()
            || requirement.evidence.is_empty()
        {
            errors.push(format!(
                "GOV-MIGRATE-0005: requirement {:?} must be unique, Unavailable, and carry blocker/evidence",
                requirement.id
            ));
        }
        for evidence in &requirement.evidence {
            let path = Path::new(evidence);
            if !is_relative_path(path) || !root.join(path).exists() {
                errors.push(format!(
                    "GOV-MIGRATE-0006: requirement {:?} has invalid or missing evidence {evidence:?}",
                    requirement.id
                ));
            }
        }
    }
    for forbidden in ["Self::Migrate", "Migrate,", "\"migrate\" => Some"] {
        if catalog.contains(forbidden) {
            errors.push(format!(
                "GOV-MIGRATE-0007: public migration command authority appeared through {forbidden:?}"
            ));
        }
    }
    if !catalog.contains("\"migrate\"") {
        errors.push(
            "GOV-MIGRATE-0009: command catalog must explicitly test migrate as plan-only"
                .to_owned(),
        );
    }
    errors
}

fn render(manifest: &ReadinessManifest) -> String {
    let mut output = String::new();
    output.push_str("# Language Migration Readiness\n\n");
    output.push_str("This generated report records why no public migration tool exists. It is not a migration protocol or implementation.\n\n");
    output.push_str(&format!("- Authority: `{}`\n", manifest.authority));
    output.push_str(&format!(
        "- Released source versions: `{}`\n",
        manifest.released_source_versions
    ));
    output.push_str(&format!(
        "- Accepted version pair: `{}`\n",
        manifest.accepted_version_pair
    ));
    output.push_str(&format!(
        "- Public command: `{}`\n\n",
        manifest.public_command
    ));
    output.push_str(
        "| Required capability | State | Blocker | Evidence |\n| --- | --- | --- | --- |\n",
    );
    for requirement in &manifest.requirement {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} |\n",
            requirement.id,
            requirement.state,
            escape_cell(&requirement.blocker),
            requirement
                .evidence
                .iter()
                .map(|path| format!("`{path}`"))
                .collect::<Vec<_>>()
                .join("<br>")
        ));
    }
    output.push_str("\nAll rows remain `Unavailable` until an Accepted source-version pair and transformation contract exist. No command is reserved.\n");
    output
}

fn summary(manifest: &ReadinessManifest) -> CheckSummary {
    CheckSummary {
        requirement_count: manifest.requirement.len(),
        unavailable_count: manifest
            .requirement
            .iter()
            .filter(|requirement| requirement.state == "Unavailable")
            .count(),
        released_source_versions: manifest.released_source_versions,
    }
}

fn is_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn escape_cell(value: &str) -> String {
    value.replace('|', "\\|").replace(['\r', '\n'], " ")
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn finish(mut errors: Vec<String>) -> Result<(), Vec<String>> {
    errors.sort();
    errors.dedup();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_migration_readiness_is_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("migration readiness is valid");
        assert_eq!(summary.requirement_count, 9);
        assert_eq!(summary.unavailable_count, 9);
        assert_eq!(summary.released_source_versions, 1);
    }
}
