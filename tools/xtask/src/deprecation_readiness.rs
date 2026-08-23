use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path};

use serde::Deserialize;

const MANIFEST_PATH: &str = "docs/governance/deprecation-readiness.toml";
const REPORT_PATH: &str = "docs/governance/deprecation-readiness.md";
const ERROR_REGISTRY_PATH: &str = "docs/ERROR-CODES.md";
const ERROR_LOCK_PATH: &str = "docs/governance/error-code-lock.toml";
const AUTHORITY_PATH: &str = "docs/governance/authority.toml";
const MIGRATION_PATH: &str = "docs/governance/migration-readiness.toml";
const EXPECTED_REQUIREMENTS: &[(&str, &str)] = &[
    ("one-x-compatibility-promise", "Unavailable"),
    ("minimum-deprecation-period", "Unavailable"),
    ("diagnostic-lifecycle", "GuardedSubset"),
    ("schema-n-minus-one-policy", "Unavailable"),
    ("target-profile-support-lifecycle", "Unavailable"),
    ("security-exception", "Unavailable"),
    ("migration-tooling-commitment", "Unavailable"),
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadinessManifest {
    schema_version: u32,
    authority: String,
    released_major_versions: usize,
    public_policy: String,
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
    pub guarded_subset_count: usize,
    pub released_major_versions: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let manifest = load_manifest(root)?;
    let sources = load_sources(root)?;
    let mut errors = validate(root, &manifest, &sources);
    let report = render(&manifest);
    match fs::read_to_string(root.join(REPORT_PATH)) {
        Ok(actual) if normalize_newlines(&actual) == report => {}
        Ok(_) => errors.push(format!(
            "GOV-DEPRECATION-0008: {REPORT_PATH} is stale; run cargo xtask deprecation render"
        )),
        Err(error) => errors.push(format!(
            "GOV-DEPRECATION-0001: cannot read {REPORT_PATH}: {error}"
        )),
    }
    finish(errors).map(|()| summary(&manifest))
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    let manifest = load_manifest(root)?;
    let sources = load_sources(root)?;
    finish(validate(root, &manifest, &sources)).map(|()| render(&manifest))
}

struct Sources {
    error_registry: String,
    error_lock: String,
    authority: String,
    migration: String,
}

fn load_sources(root: &Path) -> Result<Sources, Vec<String>> {
    Ok(Sources {
        error_registry: read(root, ERROR_REGISTRY_PATH)?,
        error_lock: read(root, ERROR_LOCK_PATH)?,
        authority: read(root, AUTHORITY_PATH)?,
        migration: read(root, MIGRATION_PATH)?,
    })
}

fn read(root: &Path, path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(root.join(path))
        .map_err(|error| vec![format!("GOV-DEPRECATION-0001: cannot read {path}: {error}")])
}

fn load_manifest(root: &Path) -> Result<ReadinessManifest, Vec<String>> {
    let text = read(root, MANIFEST_PATH)?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-DEPRECATION-0002: invalid deprecation readiness manifest: {error}"
        )]
    })
}

fn validate(root: &Path, manifest: &ReadinessManifest, sources: &Sources) -> Vec<String> {
    let mut errors = Vec::new();
    if manifest.schema_version != 1
        || manifest.authority != "DEC-0233"
        || manifest.released_major_versions != 0
        || manifest.public_policy != "Absent"
        || manifest.report != REPORT_PATH
    {
        errors.push(
            "GOV-DEPRECATION-0003: readiness markers must remain schema 1, DEC-0233, zero released major versions, Absent public policy, and the registered report"
                .to_owned(),
        );
    }

    let actual = manifest
        .requirement
        .iter()
        .map(|requirement| (requirement.id.as_str(), requirement.state.as_str()))
        .collect::<Vec<_>>();
    if actual != EXPECTED_REQUIREMENTS {
        errors.push(format!(
            "GOV-DEPRECATION-0004: requirement order/state differs; expected {EXPECTED_REQUIREMENTS:?}, found {actual:?}"
        ));
    }

    let mut ids = BTreeSet::new();
    for requirement in &manifest.requirement {
        if !ids.insert(requirement.id.as_str())
            || requirement.blocker.trim().is_empty()
            || requirement.evidence.is_empty()
        {
            errors.push(format!(
                "GOV-DEPRECATION-0005: requirement {:?} must be unique and carry blocker/evidence",
                requirement.id
            ));
        }
        for evidence in &requirement.evidence {
            let path = Path::new(evidence);
            if !is_relative_path(path) || !root.join(path).exists() {
                errors.push(format!(
                    "GOV-DEPRECATION-0006: requirement {:?} has invalid or missing evidence {evidence:?}",
                    requirement.id
                ));
            }
        }
    }

    if !sources
        .error_registry
        .contains("## Retired allocations / 退役分配")
        || !sources.error_registry.contains("`L-IMPL-0001`")
        || !sources.error_lock.contains("id = \"L-IMPL-0001\"")
        || !sources.error_lock.contains("retired = true")
    {
        errors.push(
            "GOV-DEPRECATION-0007: the bounded diagnostic retirement guard is missing".to_owned(),
        );
    }
    for marker in [
        "id = \"SCHEMA-LIFECYCLE-POLICY\"",
        "id = \"SUPPORT-MATRIX\"",
    ] {
        let Some(start) = sources.authority.find(marker) else {
            errors.push(format!(
                "GOV-DEPRECATION-0009: authority marker {marker:?} is missing"
            ));
            continue;
        };
        let tail = &sources.authority[start..];
        let section = tail.split("[[document]]").next().unwrap_or(tail);
        if !section.contains("status = \"Draft\"") || !section.contains("stable_basis = false") {
            errors.push(format!(
                "GOV-DEPRECATION-0009: {marker:?} must remain Draft with stable_basis false"
            ));
        }
    }
    if !sources.migration.contains("accepted_version_pair = false")
        || !sources.migration.contains("public_command = \"Absent\"")
    {
        errors.push(
            "GOV-DEPRECATION-0010: migration tooling must remain unavailable without an accepted version pair"
                .to_owned(),
        );
    }
    errors
}

fn render(manifest: &ReadinessManifest) -> String {
    let mut output = String::new();
    output.push_str("# Deprecation Policy Readiness\n\n");
    output.push_str("This generated report records the current bounded guards and blockers. It is not a public deprecation policy or compatibility promise.\n\n");
    output.push_str(&format!("- Authority: `{}`\n", manifest.authority));
    output.push_str(&format!(
        "- Released major versions: `{}`\n",
        manifest.released_major_versions
    ));
    output.push_str(&format!(
        "- Public deprecation policy: `{}`\n\n",
        manifest.public_policy
    ));
    output.push_str(
        "| Required policy area | State | Boundary or blocker | Evidence |\n| --- | --- | --- | --- |\n",
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
    output.push_str("\n`GuardedSubset` applies only to diagnostic-code non-reuse and retired-code exclusion. It is not a general deprecation lifecycle. All other rows remain `Unavailable`.\n");
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
        guarded_subset_count: manifest
            .requirement
            .iter()
            .filter(|requirement| requirement.state == "GuardedSubset")
            .count(),
        released_major_versions: manifest.released_major_versions,
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
    fn repository_deprecation_readiness_is_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("deprecation readiness is valid");
        assert_eq!(summary.requirement_count, 7);
        assert_eq!(summary.unavailable_count, 6);
        assert_eq!(summary.guarded_subset_count, 1);
        assert_eq!(summary.released_major_versions, 0);
    }
}
