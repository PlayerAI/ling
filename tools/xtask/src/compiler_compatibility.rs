use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path};

use serde::Deserialize;

const MANIFEST_PATH: &str = "docs/governance/compiler-compatibility-boundary.toml";
const REPORT_PATH: &str = "docs/governance/compiler-compatibility-boundary.md";
const SEED_FREEZE_PATH: &str = "docs/governance/seed-corpus-freeze.toml";
const EXPECTED_RELEASES: &[(&str, &str)] = &[
    ("v0.0.1", "AcceptUnchanged"),
    ("v0.1", "NoReleasedVersion"),
    ("v0.2", "NoReleasedVersion"),
    ("v0.3", "NoReleasedVersion"),
    ("v0.4", "NoReleasedVersion"),
    ("v0.5", "NoReleasedVersion"),
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CompatibilityManifest {
    schema_version: u32,
    authority: String,
    compiler_version: String,
    compiler_state: String,
    unicode_version: String,
    report: String,
    seed_corpus_sha256: String,
    verified_n_minus_one_edges: usize,
    release: Vec<Release>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Release {
    id: String,
    outcome: String,
    authority: String,
    reason: String,
    evidence: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SeedFreeze {
    sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub release_count: usize,
    pub accepted_unchanged_count: usize,
    pub unreleased_count: usize,
    pub verified_n_minus_one_edges: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let manifest = load_manifest(root)?;
    let seed = load_seed_freeze(root)?;
    let mut errors = validate(root, &manifest, &seed);
    let report = render(&manifest);
    match fs::read_to_string(root.join(REPORT_PATH)) {
        Ok(actual) if normalize_newlines(&actual) == report => {}
        Ok(_) => errors.push(format!(
            "GOV-COMPAT-0008: {REPORT_PATH} is stale; run cargo xtask compatibility render"
        )),
        Err(error) => errors.push(format!(
            "GOV-COMPAT-0001: cannot read {REPORT_PATH}: {error}"
        )),
    }
    finish(errors).map(|()| summary(&manifest))
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    let manifest = load_manifest(root)?;
    let seed = load_seed_freeze(root)?;
    finish(validate(root, &manifest, &seed)).map(|()| render(&manifest))
}

fn load_manifest(root: &Path) -> Result<CompatibilityManifest, Vec<String>> {
    load_toml(root, MANIFEST_PATH)
}

fn load_seed_freeze(root: &Path) -> Result<SeedFreeze, Vec<String>> {
    load_toml(root, SEED_FREEZE_PATH)
}

fn load_toml<T>(root: &Path, path: &str) -> Result<T, Vec<String>>
where
    T: for<'de> Deserialize<'de>,
{
    let text = fs::read_to_string(root.join(path))
        .map_err(|error| vec![format!("GOV-COMPAT-0001: cannot read {path}: {error}")])?;
    toml::from_str(&text)
        .map_err(|error| vec![format!("GOV-COMPAT-0002: invalid TOML {path}: {error}")])
}

fn validate(root: &Path, manifest: &CompatibilityManifest, seed: &SeedFreeze) -> Vec<String> {
    let mut errors = Vec::new();
    if manifest.schema_version != 1
        || manifest.authority != "DEC-0231"
        || manifest.compiler_version != "0.0.1-dev"
        || manifest.compiler_state != "Development"
        || manifest.unicode_version != "17.0.0"
        || manifest.report != REPORT_PATH
    {
        errors.push(
            "GOV-COMPAT-0003: compatibility markers must remain schema 1, DEC-0231, compiler 0.0.1-dev/Development, Unicode 17.0.0, and the registered report"
                .to_owned(),
        );
    }
    if manifest.seed_corpus_sha256 != seed.sha256 {
        errors.push(format!(
            "GOV-COMPAT-0004: Seed corpus identity {} differs from frozen {}",
            manifest.seed_corpus_sha256, seed.sha256
        ));
    }
    if manifest.verified_n_minus_one_edges != 0 {
        errors.push(format!(
            "GOV-COMPAT-0005: no general N-1 compiler edge is verified; found {}",
            manifest.verified_n_minus_one_edges
        ));
    }

    let actual = manifest
        .release
        .iter()
        .map(|release| (release.id.as_str(), release.outcome.as_str()))
        .collect::<Vec<_>>();
    if actual != EXPECTED_RELEASES {
        errors.push(format!(
            "GOV-COMPAT-0006: release outcomes differ; expected {EXPECTED_RELEASES:?}, found {actual:?}"
        ));
    }

    let mut ids = BTreeSet::new();
    for release in &manifest.release {
        if !ids.insert(release.id.as_str())
            || release.authority.trim().is_empty()
            || release.reason.trim().is_empty()
            || release.evidence.is_empty()
        {
            errors.push(format!(
                "GOV-COMPAT-0007: release {:?} is duplicate or lacks authority, reason, or evidence",
                release.id
            ));
        }
        for evidence in &release.evidence {
            let path = Path::new(evidence);
            if !is_relative_path(path) || !root.join(path).exists() {
                errors.push(format!(
                    "GOV-COMPAT-0009: release {:?} has invalid or missing evidence {evidence:?}",
                    release.id
                ));
            }
        }
        match release.outcome.as_str() {
            "AcceptUnchanged" if release.id == "v0.0.1" && release.authority == "CONFORMANCE" => {}
            "NoReleasedVersion" if release.id != "v0.0.1" && release.authority == "DEC-0231" => {}
            _ => errors.push(format!(
                "GOV-COMPAT-0010: release {:?} has an unauthorized outcome/authority pair {:?}/{:?}",
                release.id, release.outcome, release.authority
            )),
        }
    }
    errors
}

fn render(manifest: &CompatibilityManifest) -> String {
    let mut output = String::new();
    output.push_str("# Current Compiler Compatibility Boundary\n\n");
    output.push_str("This generated matrix describes the development compiler's verified input boundary. It is not a Ling 1.0 compatibility promise.\n\n");
    output.push_str(&format!("- Authority: `{}`\n", manifest.authority));
    output.push_str(&format!(
        "- Compiler: `{}` (`{}`)\n",
        manifest.compiler_version, manifest.compiler_state
    ));
    output.push_str(&format!("- Unicode: `{}`\n", manifest.unicode_version));
    output.push_str(&format!(
        "- Seed corpus SHA-256: `{}`\n",
        manifest.seed_corpus_sha256
    ));
    output.push_str(&format!(
        "- Verified general N-1 edges: `{}`\n\n",
        manifest.verified_n_minus_one_edges
    ));
    output.push_str(
        "| Release | Outcome | Authority | Reason | Evidence |\n| --- | --- | --- | --- | --- |\n",
    );
    for release in &manifest.release {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | {} |\n",
            release.id,
            release.outcome,
            release.authority,
            escape_cell(&release.reason),
            release
                .evidence
                .iter()
                .map(|path| format!("`{path}`"))
                .collect::<Vec<_>>()
                .join("<br>")
        ));
    }
    output.push_str("\n`NoReleasedVersion` is not rejection, warning, or migration. Those outcomes require an actual historical input and separate Accepted authority.\n");
    output
}

fn summary(manifest: &CompatibilityManifest) -> CheckSummary {
    CheckSummary {
        release_count: manifest.release.len(),
        accepted_unchanged_count: manifest
            .release
            .iter()
            .filter(|release| release.outcome == "AcceptUnchanged")
            .count(),
        unreleased_count: manifest
            .release
            .iter()
            .filter(|release| release.outcome == "NoReleasedVersion")
            .count(),
        verified_n_minus_one_edges: manifest.verified_n_minus_one_edges,
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
    fn repository_compiler_compatibility_boundary_is_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("compatibility boundary is valid");
        assert_eq!(summary.release_count, 6);
        assert_eq!(summary.accepted_unchanged_count, 1);
        assert_eq!(summary.unreleased_count, 5);
        assert_eq!(summary.verified_n_minus_one_edges, 0);
    }
}
