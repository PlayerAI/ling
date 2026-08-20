use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

use serde::Deserialize;

use crate::governance::{self, DocumentRecords};

const MANIFEST_PATH: &str = "docs/traceability/registry.toml";
const FIXTURE_ROOT: &str = "tests/conformance";
const SCOPES: &[&str] = &["Public", "Internal"];
const STABILITIES: &[&str] = &["Experimental", "Preview", "Stable"];
const POLARITIES: &[&str] = &["Positive", "Negative"];
const DIFFERENTIAL_STATES: &[&str] = &["Covered", "Deferred", "NotApplicable"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub feature_count: usize,
    pub fixture_count: usize,
    pub evidence_count: usize,
    pub deferred_differential_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FeatureRecord {
    pub title_zh: String,
    pub title_en: String,
    pub scope: String,
    pub stability: String,
    pub last_verified_commit: String,
}

pub(crate) type FeatureRecords = BTreeMap<String, FeatureRecord>;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Registry {
    schema_version: u32,
    updated: String,
    #[serde(default)]
    release: Vec<Release>,
    #[serde(default)]
    feature: Vec<Feature>,
    #[serde(default)]
    evidence: Vec<Evidence>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Release {
    id: String,
    title_zh: String,
    title_en: String,
    report: String,
    candidate_commit: String,
    tag: String,
    #[serde(default)]
    legacy_evidence: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Feature {
    id: String,
    title_zh: String,
    title_en: String,
    release: String,
    scope: String,
    stability: String,
    #[serde(default)]
    requirements: Vec<Requirement>,
    #[serde(default)]
    authorities: Vec<String>,
    #[serde(default)]
    core: Vec<SourceLink>,
    #[serde(default)]
    implementation: Vec<SourceLink>,
    #[serde(default)]
    release_artifacts: Vec<String>,
    differential_state: String,
    differential_reason: String,
    #[serde(default)]
    differential_tracking: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Requirement {
    path: String,
    clause: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceLink {
    path: String,
    symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Evidence {
    id: String,
    kind: String,
    path: String,
    symbol: String,
    #[serde(default)]
    feature_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureExpectation {
    test_id: String,
    polarity: String,
    feature_ids: Vec<String>,
    arguments: Vec<String>,
    exit_code: i32,
    normative_clauses: Vec<String>,
    #[serde(default)]
    stdout: String,
    #[serde(default)]
    diagnostic_codes: Vec<String>,
}

#[derive(Debug)]
struct Fixture {
    id: String,
    polarity: String,
    feature_ids: Vec<String>,
    normative_clauses: Vec<String>,
    path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EvidenceKind {
    Positive,
    Negative,
    Differential,
}

pub fn check_repository(root: &Path, release_id: &str) -> Result<CheckSummary, Vec<String>> {
    let authorities = governance::document_records(root)?;
    let registry = load_registry(root)?;
    let (fixtures, mut errors) = discover_fixtures(root);
    errors.extend(validate(
        root,
        &registry,
        &authorities,
        &fixtures,
        release_id,
    ));

    if let Some(release) = registry
        .release
        .iter()
        .find(|release| release.id == release_id)
    {
        let rendered = render(&registry, &authorities, &fixtures, release);
        match fs::read_to_string(root.join(&release.report)) {
            Ok(actual) if normalize_newlines(&actual) == rendered => {}
            Ok(_) => errors.push(format!(
                "GOV-TRACE-0008: {} is not the deterministic rendering of {MANIFEST_PATH}",
                release.report
            )),
            Err(error) => errors.push(format!(
                "GOV-TRACE-0002: cannot read release report {}: {error}",
                release.report
            )),
        }
    }

    finish(errors).map(|()| summary(&registry, &fixtures, release_id))
}

pub fn render_repository(root: &Path, release_id: &str) -> Result<String, Vec<String>> {
    let authorities = governance::document_records(root)?;
    let registry = load_registry(root)?;
    let (fixtures, mut errors) = discover_fixtures(root);
    errors.extend(validate(
        root,
        &registry,
        &authorities,
        &fixtures,
        release_id,
    ));
    let release = registry
        .release
        .iter()
        .find(|release| release.id == release_id)
        .ok_or_else(|| {
            vec![format!(
                "GOV-TRACE-0003: release {release_id} is not registered in {MANIFEST_PATH}"
            )]
        })?;
    finish(errors).map(|()| render(&registry, &authorities, &fixtures, release))
}

pub(crate) fn feature_records(
    root: &Path,
    release_id: &str,
) -> Result<FeatureRecords, Vec<String>> {
    let authorities = governance::document_records(root)?;
    let registry = load_registry(root)?;
    let (fixtures, mut errors) = discover_fixtures(root);
    errors.extend(validate(
        root,
        &registry,
        &authorities,
        &fixtures,
        release_id,
    ));
    finish(errors)?;
    let last_verified_commit = registry
        .release
        .iter()
        .find(|release| release.id == release_id)
        .map(|release| release.candidate_commit.clone())
        .ok_or_else(|| {
            vec![format!(
                "GOV-TRACE-0003: release {release_id} is not registered in {MANIFEST_PATH}"
            )]
        })?;
    Ok(registry
        .feature
        .into_iter()
        .filter(|feature| feature.release == release_id)
        .map(|feature| {
            (
                feature.id,
                FeatureRecord {
                    title_zh: feature.title_zh,
                    title_en: feature.title_en,
                    scope: feature.scope,
                    stability: feature.stability,
                    last_verified_commit: last_verified_commit.clone(),
                },
            )
        })
        .collect())
}

fn load_registry(root: &Path) -> Result<Registry, Vec<String>> {
    let text = fs::read_to_string(root.join(MANIFEST_PATH)).map_err(|error| {
        vec![format!(
            "GOV-TRACE-0002: cannot read {MANIFEST_PATH}: {error}"
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-TRACE-0009: invalid traceability registry {MANIFEST_PATH}: {error}"
        )]
    })
}

fn discover_fixtures(root: &Path) -> (Vec<Fixture>, Vec<String>) {
    let mut fixtures = Vec::new();
    let mut errors = Vec::new();
    let fixture_root = root.join(FIXTURE_ROOT);
    let entries = match fs::read_dir(&fixture_root) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!(
                "GOV-TRACE-0002: cannot read {FIXTURE_ROOT}: {error}"
            ));
            return (fixtures, errors);
        }
    };
    let mut directories = entries
        .filter_map(|entry| match entry {
            Ok(entry) if entry.path().is_dir() => Some(entry.path()),
            Ok(_) => None,
            Err(error) => {
                errors.push(format!(
                    "GOV-TRACE-0002: cannot read a {FIXTURE_ROOT} entry: {error}"
                ));
                None
            }
        })
        .collect::<Vec<_>>();
    directories.sort();

    for directory in directories {
        let relative_directory = repository_path(root, &directory);
        let case_path = directory.join("case.ling");
        if !case_path.is_file() {
            errors.push(format!(
                "GOV-TRACE-0004: fixture {relative_directory} has no case.ling"
            ));
        }
        let expectation_path = directory.join("expect.toml");
        let text = match fs::read_to_string(&expectation_path) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!(
                    "GOV-TRACE-0002: cannot read {relative_directory}/expect.toml: {error}"
                ));
                continue;
            }
        };
        let expectation = match toml::from_str::<FixtureExpectation>(&text) {
            Ok(expectation) => expectation,
            Err(error) => {
                errors.push(format!(
                    "GOV-TRACE-0009: invalid fixture metadata {relative_directory}/expect.toml: {error}"
                ));
                continue;
            }
        };
        validate_fixture_expectation(&relative_directory, &expectation, &mut errors);
        fixtures.push(Fixture {
            id: expectation.test_id,
            polarity: expectation.polarity,
            feature_ids: expectation.feature_ids,
            normative_clauses: expectation.normative_clauses,
            path: format!("{relative_directory}/expect.toml"),
        });
    }
    (fixtures, errors)
}

fn validate_fixture_expectation(
    directory: &str,
    expectation: &FixtureExpectation,
    errors: &mut Vec<String>,
) {
    if !valid_prefixed_id(&expectation.test_id, "TEST-CONF-") {
        errors.push(format!(
            "GOV-TRACE-0010: {directory} has invalid test_id {:?}; expected TEST-CONF-*",
            expectation.test_id
        ));
    }
    if !POLARITIES.contains(&expectation.polarity.as_str()) {
        errors.push(format!(
            "GOV-TRACE-0010: {} has invalid polarity {:?}",
            display_id(&expectation.test_id),
            expectation.polarity
        ));
    }
    if expectation.feature_ids.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: {} has no feature_ids",
            display_id(&expectation.test_id)
        ));
    }
    if expectation.normative_clauses.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: {} has no normative_clauses",
            display_id(&expectation.test_id)
        ));
    }
    if expectation.arguments.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0010: {} has no CLI arguments",
            display_id(&expectation.test_id)
        ));
    }
    if expectation.polarity == "Positive"
        && (expectation.exit_code != 0 || !expectation.diagnostic_codes.is_empty())
    {
        errors.push(format!(
            "GOV-TRACE-0010: {} is Positive but expects a failure or diagnostic",
            display_id(&expectation.test_id)
        ));
    }
    if expectation.polarity == "Negative"
        && expectation.exit_code == 0
        && expectation.diagnostic_codes.is_empty()
    {
        errors.push(format!(
            "GOV-TRACE-0010: {} is Negative but expects neither failure nor diagnostic",
            display_id(&expectation.test_id)
        ));
    }
    let _ = &expectation.stdout;
}

fn validate(
    root: &Path,
    registry: &Registry,
    authorities: &DocumentRecords,
    fixtures: &[Fixture],
    requested_release: &str,
) -> Vec<String> {
    let mut errors = Vec::new();
    if registry.schema_version != 1 {
        errors.push(format!(
            "GOV-TRACE-0010: unsupported schema_version {}; expected 1",
            registry.schema_version
        ));
    }
    if !is_date(&registry.updated) {
        errors.push("GOV-TRACE-0010: updated must be a YYYY-MM-DD date".to_owned());
    }
    if registry.release.is_empty() {
        errors.push("GOV-TRACE-0010: traceability registry has no releases".to_owned());
    }
    if registry.feature.is_empty() {
        errors.push("GOV-TRACE-0010: traceability registry has no features".to_owned());
    }

    let mut releases = BTreeMap::new();
    for release in &registry.release {
        validate_release(root, release, &mut errors);
        if releases.insert(release.id.as_str(), release).is_some() {
            errors.push(format!(
                "GOV-TRACE-0001: duplicate release id {}",
                display_id(&release.id)
            ));
        }
    }
    if !releases.contains_key(requested_release) {
        errors.push(format!(
            "GOV-TRACE-0003: release {requested_release} is not registered in {MANIFEST_PATH}"
        ));
    }

    let mut features = BTreeMap::new();
    for feature in &registry.feature {
        validate_feature(root, feature, authorities, &releases, &mut errors);
        if features.insert(feature.id.as_str(), feature).is_some() {
            errors.push(format!(
                "GOV-TRACE-0001: duplicate feature_id {}",
                display_id(&feature.id)
            ));
        }
    }

    let mut evidence_ids = BTreeSet::new();
    let mut coverage = BTreeMap::<&str, BTreeSet<EvidenceKind>>::new();
    for fixture in fixtures {
        if !evidence_ids.insert(fixture.id.as_str()) {
            errors.push(format!(
                "GOV-TRACE-0001: duplicate evidence/test id {}",
                display_id(&fixture.id)
            ));
        }
        let kind = match fixture.polarity.as_str() {
            "Positive" => Some(EvidenceKind::Positive),
            "Negative" => Some(EvidenceKind::Negative),
            _ => None,
        };
        validate_feature_references(
            &fixture.id,
            &fixture.feature_ids,
            &features,
            kind,
            &mut coverage,
            &mut errors,
        );
    }
    for evidence in &registry.evidence {
        let kind = validate_evidence(root, evidence, &mut errors);
        if !evidence_ids.insert(evidence.id.as_str()) {
            errors.push(format!(
                "GOV-TRACE-0001: duplicate evidence/test id {}",
                display_id(&evidence.id)
            ));
        }
        validate_feature_references(
            &evidence.id,
            &evidence.feature_ids,
            &features,
            kind,
            &mut coverage,
            &mut errors,
        );
    }

    for feature in registry
        .feature
        .iter()
        .filter(|feature| feature.release == requested_release)
    {
        let kinds = coverage.get(feature.id.as_str());
        if feature.scope == "Public"
            && !kinds.is_some_and(|kinds| kinds.contains(&EvidenceKind::Positive))
        {
            errors.push(format!(
                "GOV-TRACE-0006: public feature {} has no positive evidence",
                feature.id
            ));
        }
        if feature.scope == "Public"
            && !kinds.is_some_and(|kinds| kinds.contains(&EvidenceKind::Negative))
        {
            errors.push(format!(
                "GOV-TRACE-0006: public feature {} has no negative evidence",
                feature.id
            ));
        }
        let has_differential =
            kinds.is_some_and(|kinds| kinds.contains(&EvidenceKind::Differential));
        match feature.differential_state.as_str() {
            "Covered" if !has_differential => errors.push(format!(
                "GOV-TRACE-0006: {} claims Covered differential status without Differential evidence",
                feature.id
            )),
            "Deferred" | "NotApplicable" if has_differential => errors.push(format!(
                "GOV-TRACE-0010: {} has Differential evidence but state {}",
                feature.id, feature.differential_state
            )),
            _ => {}
        }
    }
    errors
}

fn validate_release(root: &Path, release: &Release, errors: &mut Vec<String>) {
    if !valid_release_id(&release.id) {
        errors.push(format!(
            "GOV-TRACE-0010: invalid release id {:?}; expected v<semver>",
            release.id
        ));
    }
    for (field, value) in [
        ("title_zh", release.title_zh.as_str()),
        ("title_en", release.title_en.as_str()),
        ("tag", release.tag.as_str()),
    ] {
        if value.trim().is_empty() {
            errors.push(format!(
                "GOV-TRACE-0010: release {} has empty {field}",
                display_id(&release.id)
            ));
        }
    }
    if release.candidate_commit.len() != 40
        || !release
            .candidate_commit
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        errors.push(format!(
            "GOV-TRACE-0010: release {} candidate_commit must be a full lowercase Git object id",
            display_id(&release.id)
        ));
    }
    validate_path(root, &release.report, "release report", false, errors);
    if !release.report.starts_with("docs/traceability/") || !release.report.ends_with(".md") {
        errors.push(format!(
            "GOV-TRACE-0010: release {} report must be docs/traceability/<release>.md",
            display_id(&release.id)
        ));
    }
    for path in &release.legacy_evidence {
        validate_path(root, path, "legacy release evidence", true, errors);
    }
}

fn validate_feature(
    root: &Path,
    feature: &Feature,
    authorities: &DocumentRecords,
    releases: &BTreeMap<&str, &Release>,
    errors: &mut Vec<String>,
) {
    if !valid_prefixed_id(&feature.id, "FTR-") {
        errors.push(format!(
            "GOV-TRACE-0010: invalid feature_id {:?}; expected FTR-*",
            feature.id
        ));
    }
    for (field, value) in [
        ("title_zh", feature.title_zh.as_str()),
        ("title_en", feature.title_en.as_str()),
    ] {
        if value.trim().is_empty() {
            errors.push(format!(
                "GOV-TRACE-0010: feature {} has empty {field}",
                display_id(&feature.id)
            ));
        }
    }
    if !releases.contains_key(feature.release.as_str()) {
        errors.push(format!(
            "GOV-TRACE-0003: feature {} references unknown release {}",
            display_id(&feature.id),
            feature.release
        ));
    }
    if !SCOPES.contains(&feature.scope.as_str()) {
        errors.push(format!(
            "GOV-TRACE-0010: feature {} has invalid scope {}",
            display_id(&feature.id),
            feature.scope
        ));
    }
    if !STABILITIES.contains(&feature.stability.as_str()) {
        errors.push(format!(
            "GOV-TRACE-0010: feature {} has invalid stability {}",
            display_id(&feature.id),
            feature.stability
        ));
    }
    if feature.requirements.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: feature {} has no Requirement/Spec links",
            display_id(&feature.id)
        ));
    }
    for requirement in &feature.requirements {
        validate_requirement(root, &feature.id, requirement, errors);
    }
    if feature.authorities.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: feature {} has no RFC/Decision authority links",
            display_id(&feature.id)
        ));
    }
    let mut has_accepted_basis = false;
    let mut seen_authorities = BTreeSet::new();
    for id in &feature.authorities {
        if !seen_authorities.insert(id) {
            errors.push(format!(
                "GOV-TRACE-0001: feature {} repeats authority {id}",
                display_id(&feature.id)
            ));
        }
        match authorities.get(id) {
            Some(record) => has_accepted_basis |= record.status == "Accepted",
            None => errors.push(format!(
                "GOV-TRACE-0003: feature {} references unknown authority {id}",
                display_id(&feature.id)
            )),
        }
    }
    if !has_accepted_basis {
        errors.push(format!(
            "GOV-TRACE-0006: feature {} has no Accepted authority basis",
            display_id(&feature.id)
        ));
    }
    if feature.core.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: feature {} has no Core node/Schema link",
            display_id(&feature.id)
        ));
    }
    for link in &feature.core {
        validate_source_link(root, &feature.id, "Core node/Schema", link, errors);
    }
    if feature.implementation.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: feature {} has no Implementation link",
            display_id(&feature.id)
        ));
    }
    for link in &feature.implementation {
        validate_source_link(root, &feature.id, "Implementation", link, errors);
    }
    if feature.release_artifacts.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: feature {} has no Release artifact",
            display_id(&feature.id)
        ));
    }
    for path in &feature.release_artifacts {
        validate_path(root, path, "release artifact", true, errors);
    }
    if !DIFFERENTIAL_STATES.contains(&feature.differential_state.as_str()) {
        errors.push(format!(
            "GOV-TRACE-0010: feature {} has invalid differential_state {}",
            display_id(&feature.id),
            feature.differential_state
        ));
    }
    if feature.differential_reason.trim().is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: feature {} has no differential_reason",
            display_id(&feature.id)
        ));
    }
    if feature.differential_state == "Deferred" && feature.differential_tracking.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: deferred feature {} has no differential_tracking link",
            display_id(&feature.id)
        ));
    }
    for path in &feature.differential_tracking {
        validate_path(root, path, "differential tracking", true, errors);
    }
}

fn validate_requirement(
    root: &Path,
    feature_id: &str,
    requirement: &Requirement,
    errors: &mut Vec<String>,
) {
    validate_path(root, &requirement.path, "requirement/spec", true, errors);
    if requirement.clause.trim().is_empty() {
        errors.push(format!(
            "GOV-TRACE-0010: feature {} has an empty requirement clause",
            display_id(feature_id)
        ));
        return;
    }
    let Ok(text) = fs::read_to_string(root.join(&requirement.path)) else {
        return;
    };
    if !text
        .lines()
        .filter_map(markdown_heading)
        .any(|heading| clause_matches(heading, &requirement.clause))
    {
        errors.push(format!(
            "GOV-TRACE-0004: feature {} clause {} is not a Markdown heading in {}",
            display_id(feature_id),
            requirement.clause,
            requirement.path
        ));
    }
}

fn validate_source_link(
    root: &Path,
    feature_id: &str,
    field: &str,
    link: &SourceLink,
    errors: &mut Vec<String>,
) {
    validate_path(root, &link.path, field, true, errors);
    if link.symbol.trim().is_empty() {
        errors.push(format!(
            "GOV-TRACE-0010: feature {} has an empty {field} symbol",
            display_id(feature_id)
        ));
        return;
    }
    let Ok(text) = fs::read_to_string(root.join(&link.path)) else {
        return;
    };
    if !text.contains(&link.symbol) {
        errors.push(format!(
            "GOV-TRACE-0004: feature {} {field} symbol {:?} is absent from {}",
            display_id(feature_id),
            link.symbol,
            link.path
        ));
    }
}

fn validate_evidence(
    root: &Path,
    evidence: &Evidence,
    errors: &mut Vec<String>,
) -> Option<EvidenceKind> {
    let kind = match evidence.kind.as_str() {
        "Positive" => Some(EvidenceKind::Positive),
        "Negative" => Some(EvidenceKind::Negative),
        "Differential" => Some(EvidenceKind::Differential),
        _ => {
            errors.push(format!(
                "GOV-TRACE-0010: evidence {} has invalid kind {}",
                display_id(&evidence.id),
                evidence.kind
            ));
            None
        }
    };
    let expected_prefix = match kind {
        Some(EvidenceKind::Positive) => "EVD-POS-",
        Some(EvidenceKind::Negative) => "EVD-NEG-",
        Some(EvidenceKind::Differential) => "EVD-DIFF-",
        None => "EVD-",
    };
    if !valid_prefixed_id(&evidence.id, expected_prefix) {
        errors.push(format!(
            "GOV-TRACE-0010: invalid evidence id {:?}; expected {expected_prefix}*",
            evidence.id
        ));
    }
    if evidence.feature_ids.is_empty() {
        errors.push(format!(
            "GOV-TRACE-0006: evidence {} has no feature_ids",
            display_id(&evidence.id)
        ));
    }
    validate_path(root, &evidence.path, "evidence", true, errors);
    if evidence.symbol.trim().is_empty() {
        errors.push(format!(
            "GOV-TRACE-0010: evidence {} has an empty symbol",
            display_id(&evidence.id)
        ));
    } else if let Ok(text) = fs::read_to_string(root.join(&evidence.path)) {
        if !text.contains(&evidence.symbol) {
            errors.push(format!(
                "GOV-TRACE-0004: evidence {} symbol {:?} is absent from {}",
                display_id(&evidence.id),
                evidence.symbol,
                evidence.path
            ));
        }
    }
    kind
}

fn validate_feature_references<'a>(
    evidence_id: &str,
    feature_ids: &'a [String],
    features: &BTreeMap<&'a str, &'a Feature>,
    kind: Option<EvidenceKind>,
    coverage: &mut BTreeMap<&'a str, BTreeSet<EvidenceKind>>,
    errors: &mut Vec<String>,
) {
    let mut seen = BTreeSet::new();
    for feature_id in feature_ids {
        if !seen.insert(feature_id) {
            errors.push(format!(
                "GOV-TRACE-0001: evidence {} repeats feature_id {feature_id}",
                display_id(evidence_id)
            ));
        }
        match features.get(feature_id.as_str()) {
            Some(_) => {
                if let Some(kind) = kind {
                    coverage.entry(feature_id).or_default().insert(kind);
                }
            }
            None => errors.push(format!(
                "GOV-TRACE-0003: evidence {} references unknown feature {feature_id}",
                display_id(evidence_id)
            )),
        }
    }
}

fn validate_path(
    root: &Path,
    value: &str,
    field: &str,
    must_exist: bool,
    errors: &mut Vec<String>,
) {
    if !is_relative_path(value) {
        errors.push(format!(
            "GOV-TRACE-0010: {field} path {value:?} must be a normalized repository-relative path"
        ));
        return;
    }
    if must_exist && !root.join(value).exists() {
        errors.push(format!(
            "GOV-TRACE-0004: {field} path does not exist: {value}"
        ));
    }
}

fn render(
    registry: &Registry,
    authorities: &DocumentRecords,
    fixtures: &[Fixture],
    release: &Release,
) -> String {
    let features = registry
        .feature
        .iter()
        .filter(|feature| feature.release == release.id)
        .map(|feature| (feature.id.as_str(), feature))
        .collect::<BTreeMap<_, _>>();
    let mut evidence_by_feature = BTreeMap::<&str, Vec<RenderedEvidence<'_>>>::new();
    for fixture in fixtures {
        for feature_id in &fixture.feature_ids {
            if features.contains_key(feature_id.as_str()) {
                evidence_by_feature
                    .entry(feature_id)
                    .or_default()
                    .push(RenderedEvidence {
                        id: &fixture.id,
                        kind: &fixture.polarity,
                        path: &fixture.path,
                        symbol: "",
                    });
            }
        }
    }
    for evidence in &registry.evidence {
        for feature_id in &evidence.feature_ids {
            if features.contains_key(feature_id.as_str()) {
                evidence_by_feature
                    .entry(feature_id)
                    .or_default()
                    .push(RenderedEvidence {
                        id: &evidence.id,
                        kind: &evidence.kind,
                        path: &evidence.path,
                        symbol: &evidence.symbol,
                    });
            }
        }
    }
    for evidence in evidence_by_feature.values_mut() {
        evidence.sort_by_key(|item| item.id);
    }

    let mut output = String::new();
    output.push_str(&format!(
        "# {} / {}\n\n",
        release.title_zh, release.title_en
    ));
    output.push_str(
        "> 本文件由 `docs/traceability/registry.toml` 确定性生成；请勿手工编辑。\n> This file is deterministically generated from `docs/traceability/registry.toml`; do not edit it manually.\n\n",
    );
    output.push_str(&format!(
        "- 发布 / Release: `{}`\n- 候选提交 / Candidate commit: `{}`\n- 标签 / Tag: `{}`\n- 注册表更新 / Registry updated: `{}`\n\n",
        release.id, release.candidate_commit, release.tag, registry.updated
    ));
    output.push_str("RFC-0001 仍为 Draft；本索引显示其验收条款，但只有标为 Accepted 的规范或 decision 才构成已接受依据。所有当前公开 Seed feature 均标为 Experimental，不因发布事实自动升级稳定级别。\n\nRFC-0001 remains Draft. This index displays its acceptance clauses, while only specifications or decisions marked Accepted constitute accepted authority. Every current public Seed feature remains Experimental; release history does not promote stability.\n\n");
    output.push_str("| Requirement/Spec | RFC/Decision | Core node/Schema | Implementation | Positive | Negative | Differential | Release artifact |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for feature in features.values() {
        let requirement = format!(
            "`{}` — {} / {}<br>{}<br>Scope: `{}`; Stability: `{}`",
            feature.id,
            escape_cell(&feature.title_zh),
            escape_cell(&feature.title_en),
            feature
                .requirements
                .iter()
                .map(render_requirement)
                .collect::<Vec<_>>()
                .join("<br>"),
            feature.scope,
            feature.stability
        );
        let authority = feature
            .authorities
            .iter()
            .map(|id| match authorities.get(id) {
                Some(record) => format!(
                    "[`{}`]({}) (`{}`, `{}`)",
                    escape_cell(id),
                    report_link(&record.path),
                    escape_cell(&record.kind),
                    escape_cell(&record.status)
                ),
                None => format!("`{}`", escape_cell(id)),
            })
            .collect::<Vec<_>>()
            .join("<br>");
        let core = feature
            .core
            .iter()
            .map(render_source_link)
            .collect::<Vec<_>>()
            .join("<br>");
        let implementation = feature
            .implementation
            .iter()
            .map(render_source_link)
            .collect::<Vec<_>>()
            .join("<br>");
        let evidence = evidence_by_feature
            .get(feature.id.as_str())
            .map(Vec::as_slice)
            .unwrap_or_default();
        let positive = render_evidence(evidence, "Positive");
        let negative = render_evidence(evidence, "Negative");
        let differential = if feature.differential_state == "Covered" {
            render_evidence(evidence, "Differential")
        } else {
            let tracking = feature
                .differential_tracking
                .iter()
                .map(|path| format!("[tracking]({})", report_link(path)))
                .collect::<Vec<_>>()
                .join(", ");
            if tracking.is_empty() {
                format!(
                    "`{}` — {}",
                    feature.differential_state,
                    escape_cell(&feature.differential_reason)
                )
            } else {
                format!(
                    "`{}` — {}<br>{tracking}",
                    feature.differential_state,
                    escape_cell(&feature.differential_reason)
                )
            }
        };
        let artifacts = feature
            .release_artifacts
            .iter()
            .map(|path| format!("[`{}`]({})", escape_cell(path), report_link(path)))
            .collect::<Vec<_>>()
            .join("<br>");
        output.push_str(&format!(
            "| {requirement} | {authority} | {core} | {implementation} | {positive} | {negative} | {differential} | {artifacts} |\n"
        ));
    }

    output.push_str("\n## Conformance fixture index / Conformance fixture 索引\n\n");
    output.push_str("Every directory under `tests/conformance/` carries one immutable `test_id`, an explicit polarity, feature links, and normative clauses in `expect.toml`.\n\n");
    output.push_str("| Test ID | Polarity | Features | Normative clauses | Fixture |\n| --- | --- | --- | --- | --- |\n");
    let mut sorted_fixtures = fixtures.iter().collect::<Vec<_>>();
    sorted_fixtures.sort_by_key(|fixture| fixture.id.as_str());
    for fixture in sorted_fixtures {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} | [`{}`]({}) |\n",
            escape_cell(&fixture.id),
            fixture.polarity,
            code_list(&fixture.feature_ids),
            code_list(&fixture.normative_clauses),
            escape_cell(&fixture.path),
            report_link(&fixture.path)
        ));
    }

    if !registry.evidence.is_empty() {
        output.push_str("\n## Named Rust evidence / 命名 Rust 证据\n\n");
        output.push_str(
            "| Evidence ID | Kind | Features | Test symbol |\n| --- | --- | --- | --- |\n",
        );
        let mut evidence = registry.evidence.iter().collect::<Vec<_>>();
        evidence.sort_by_key(|item| item.id.as_str());
        for item in evidence {
            output.push_str(&format!(
                "| `{}` | `{}` | {} | [`{}`]({}) |\n",
                escape_cell(&item.id),
                item.kind,
                code_list(&item.feature_ids),
                escape_cell(&item.symbol),
                report_link(&item.path)
            ));
        }
    }

    output.push_str("\n## Historical release evidence / 历史发布证据\n\n");
    for path in &release.legacy_evidence {
        output.push_str(&format!(
            "- [`{}`]({})\n",
            escape_cell(path),
            report_link(path)
        ));
    }
    output.push_str("\n## Verification / 验证\n\n```text\n");
    output.push_str(&format!(
        "cargo xtask traceability verify --release {}\n",
        release.id
    ));
    output.push_str("cargo test --workspace --all-features --locked --offline\n```\n");
    output
}

struct RenderedEvidence<'a> {
    id: &'a str,
    kind: &'a str,
    path: &'a str,
    symbol: &'a str,
}

fn render_requirement(requirement: &Requirement) -> String {
    format!(
        "[`{} {}`]({})",
        escape_cell(&requirement.path),
        escape_cell(&requirement.clause),
        report_link(&requirement.path)
    )
}

fn render_source_link(link: &SourceLink) -> String {
    format!(
        "[`{}`]({}) :: `{}`",
        escape_cell(&link.path),
        report_link(&link.path),
        escape_cell(&link.symbol)
    )
}

fn render_evidence(evidence: &[RenderedEvidence<'_>], kind: &str) -> String {
    let items = evidence
        .iter()
        .filter(|item| item.kind == kind)
        .map(|item| {
            let suffix = if item.symbol.is_empty() {
                String::new()
            } else {
                format!(" :: `{}`", escape_cell(item.symbol))
            };
            format!(
                "[`{}`]({}){suffix}",
                escape_cell(item.id),
                report_link(item.path)
            )
        })
        .collect::<Vec<_>>();
    if items.is_empty() {
        "—".to_owned()
    } else {
        items.join("<br>")
    }
}

fn summary(registry: &Registry, fixtures: &[Fixture], release_id: &str) -> CheckSummary {
    let feature_ids = registry
        .feature
        .iter()
        .filter(|feature| feature.release == release_id)
        .map(|feature| feature.id.as_str())
        .collect::<BTreeSet<_>>();
    let fixture_count = fixtures
        .iter()
        .filter(|fixture| {
            fixture
                .feature_ids
                .iter()
                .any(|id| feature_ids.contains(id.as_str()))
        })
        .count();
    let named_evidence_count = registry
        .evidence
        .iter()
        .filter(|evidence| {
            evidence
                .feature_ids
                .iter()
                .any(|id| feature_ids.contains(id.as_str()))
        })
        .count();
    CheckSummary {
        feature_count: feature_ids.len(),
        fixture_count,
        evidence_count: fixture_count + named_evidence_count,
        deferred_differential_count: registry
            .feature
            .iter()
            .filter(|feature| {
                feature.release == release_id && feature.differential_state == "Deferred"
            })
            .count(),
    }
}

fn markdown_heading(line: &str) -> Option<&str> {
    let line = line.trim_start();
    let content = line.trim_start_matches('#');
    (content.len() < line.len())
        .then(|| content.trim_start())
        .filter(|content| !content.is_empty())
}

fn clause_matches(heading: &str, clause: &str) -> bool {
    let clause = clause.trim().trim_start_matches('§');
    heading.strip_prefix(clause).is_some_and(|rest| {
        rest.is_empty()
            || rest.starts_with(char::is_whitespace)
            || rest.starts_with('.')
            || rest.starts_with('：')
            || rest.starts_with(':')
    })
}

fn valid_prefixed_id(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        !suffix.is_empty()
            && !suffix.starts_with('-')
            && !suffix.ends_with('-')
            && suffix
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-')
    })
}

fn valid_release_id(value: &str) -> bool {
    let Some(version) = value.strip_prefix('v') else {
        return false;
    };
    let parts = version.split('.').collect::<Vec<_>>();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
}

fn is_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit())
}

fn is_relative_path(value: &str) -> bool {
    if value.is_empty() || value.contains('\\') {
        return false;
    }
    let path = Path::new(value);
    !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn repository_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn report_link(path: &str) -> String {
    if let Some(path) = path.strip_prefix("docs/") {
        format!("../{path}")
    } else {
        format!("../../{path}")
    }
}

fn code_list(values: &[String]) -> String {
    if values.is_empty() {
        return "—".to_owned();
    }
    values
        .iter()
        .map(|value| format!("`{}`", escape_cell(value)))
        .collect::<Vec<_>>()
        .join("<br>")
}

fn escape_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', "<br>")
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn display_id(value: &str) -> &str {
    if value.is_empty() {
        "<missing-id>"
    } else {
        value
    }
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
    use crate::governance::DocumentRecord;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

    struct TempRepo {
        root: PathBuf,
    }

    impl TempRepo {
        fn new() -> Self {
            let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir()
                .join(format!("ling-traceability-{}-{serial}", std::process::id()));
            fs::create_dir_all(&root).expect("temporary repository is created");
            Self { root }
        }

        fn write(&self, relative: &str, text: &str) {
            let path = self.root.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("temporary parent is created");
            }
            fs::write(path, text).expect("temporary fixture is written");
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn authorities() -> DocumentRecords {
        BTreeMap::from([(
            "DEC-0001".to_owned(),
            DocumentRecord {
                kind: "Decision".to_owned(),
                status: "Accepted".to_owned(),
                path: "docs/decision.md".to_owned(),
            },
        )])
    }

    fn registry() -> Registry {
        toml::from_str(
            r#"
schema_version = 1
updated = "2026-08-20"

[[release]]
id = "v0.0.1"
title_zh = "证据"
title_en = "Evidence"
report = "docs/traceability/v0.0.1.md"
candidate_commit = "0123456789abcdef0123456789abcdef01234567"
tag = "v0.0.1"
legacy_evidence = ["docs/legacy.md"]

[[feature]]
id = "FTR-SEED-0001"
title_zh = "运行"
title_en = "Run"
release = "v0.0.1"
scope = "Public"
stability = "Experimental"
requirements = [{ path = "docs/spec.md", clause = "1.1" }]
authorities = ["DEC-0001"]
core = [{ path = "src/lib.rs", symbol = "Core" }]
implementation = [{ path = "src/lib.rs", symbol = "run" }]
release_artifacts = ["artifact.txt"]
differential_state = "Deferred"
differential_reason = "One evaluator"
differential_tracking = ["docs/tracking.md"]

[[evidence]]
id = "EVD-POS-RUN"
kind = "Positive"
path = "src/lib.rs"
symbol = "positive_test"
feature_ids = ["FTR-SEED-0001"]

[[evidence]]
id = "EVD-NEG-RUN"
kind = "Negative"
path = "src/lib.rs"
symbol = "negative_test"
feature_ids = ["FTR-SEED-0001"]
"#,
        )
        .expect("registry parses")
    }

    fn repo() -> TempRepo {
        let repo = TempRepo::new();
        repo.write("docs/spec.md", "# Spec\n\n## 1.1 Run\n");
        repo.write("docs/decision.md", "# Decision\n");
        repo.write("docs/legacy.md", "legacy\n");
        repo.write("docs/tracking.md", "tracking\n");
        repo.write(
            "src/lib.rs",
            "struct Core; fn run() {} fn positive_test() {} fn negative_test() {}\n",
        );
        repo.write("artifact.txt", "artifact\n");
        repo
    }

    fn validation_errors(repo: &TempRepo, registry: &Registry) -> Vec<String> {
        validate(&repo.root, registry, &authorities(), &[], "v0.0.1")
    }

    #[test]
    fn accepts_complete_public_feature_chain() {
        let repo = repo();
        assert!(validation_errors(&repo, &registry()).is_empty());
    }

    #[test]
    fn rejects_duplicate_feature_ids() {
        let repo = repo();
        let mut registry = registry();
        let duplicate: Feature = toml::from_str(
            r#"
id = "FTR-SEED-0001"
title_zh = "重复"
title_en = "Duplicate"
release = "v0.0.1"
scope = "Internal"
stability = "Experimental"
requirements = [{ path = "docs/spec.md", clause = "1.1" }]
authorities = ["DEC-0001"]
core = [{ path = "src/lib.rs", symbol = "Core" }]
implementation = [{ path = "src/lib.rs", symbol = "run" }]
release_artifacts = ["artifact.txt"]
differential_state = "NotApplicable"
differential_reason = "Internal metadata"
"#,
        )
        .expect("feature parses");
        registry.feature.push(duplicate);
        assert!(
            validation_errors(&repo, &registry)
                .iter()
                .any(|error| error.contains("duplicate feature_id"))
        );
    }

    #[test]
    fn rejects_missing_clause_and_symbol_links() {
        let repo = repo();
        let mut registry = registry();
        registry.feature[0].requirements[0].clause = "9.9".to_owned();
        registry.feature[0].core[0].symbol = "MissingCore".to_owned();
        let errors = validation_errors(&repo, &registry);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("not a Markdown heading"))
        );
        assert!(errors.iter().any(|error| error.contains("MissingCore")));
    }

    #[test]
    fn rejects_unknown_or_nonaccepted_authority() {
        let repo = repo();
        let mut registry = registry();
        registry.feature[0].authorities = vec!["RFC-404".to_owned()];
        let errors = validation_errors(&repo, &registry);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("unknown authority"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("no Accepted authority"))
        );
    }

    #[test]
    fn rejects_public_feature_without_both_polarities() {
        let repo = repo();
        let mut registry = registry();
        registry.evidence.retain(|item| item.kind == "Positive");
        assert!(
            validation_errors(&repo, &registry)
                .iter()
                .any(|error| error.contains("no negative evidence"))
        );
    }

    #[test]
    fn rejects_covered_differential_without_evidence() {
        let repo = repo();
        let mut registry = registry();
        registry.feature[0].differential_state = "Covered".to_owned();
        registry.feature[0].differential_tracking.clear();
        assert!(
            validation_errors(&repo, &registry)
                .iter()
                .any(|error| error.contains("without Differential evidence"))
        );
    }

    #[test]
    fn fixture_metadata_requires_stable_id_and_meaningful_polarity() {
        let expectation = FixtureExpectation {
            test_id: "temporary".to_owned(),
            polarity: "Negative".to_owned(),
            feature_ids: Vec::new(),
            arguments: vec!["check".to_owned()],
            exit_code: 0,
            normative_clauses: Vec::new(),
            stdout: String::new(),
            diagnostic_codes: Vec::new(),
        };
        let mut errors = Vec::new();
        validate_fixture_expectation("tests/conformance/example", &expectation, &mut errors);
        assert!(errors.iter().any(|error| error.contains("invalid test_id")));
        assert!(errors.iter().any(|error| error.contains("no feature_ids")));
        assert!(
            errors
                .iter()
                .any(|error| error.contains("neither failure nor diagnostic"))
        );
    }

    #[test]
    fn rendering_is_deterministic() {
        let registry = registry();
        let release = &registry.release[0];
        let first = render(&registry, &authorities(), &[], release);
        let second = render(&registry, &authorities(), &[], release);
        assert_eq!(first, second);
        assert!(first.contains("| Requirement/Spec | RFC/Decision |"));
    }

    #[test]
    fn repository_registry_is_valid_and_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root, "v0.0.1").expect("repository traceability is valid");
        assert_eq!(summary.feature_count, 7);
        assert_eq!(summary.fixture_count, 38);
        assert!(summary.evidence_count >= 60);
    }
}
