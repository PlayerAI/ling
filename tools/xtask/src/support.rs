use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::gaps;
use crate::governance::{self, DocumentRecords};
use crate::protocols::{self, ProtocolRecords};
use crate::traceability::{self, FeatureRecords};

const MANIFEST_PATH: &str = "docs/governance/support-matrix.toml";
const REQUIRED_PROFILES: &[&str] = &["Explore", "Native", "Critical"];
const REQUIRED_HOSTS: &[&str] = &[
    "HOST-WINDOWS-LATEST",
    "HOST-LINUX-LATEST",
    "HOST-MACOS-LATEST",
];
const REQUIRED_BACKENDS: &[&str] = &[
    "BACKEND-INTERPRETER",
    "BACKEND-VM",
    "BACKEND-NATIVE",
    "BACKEND-KERNEL-CPU",
    "BACKEND-GPU",
    "BACKEND-ACCELERATOR",
];
const REQUIRED_UNSUPPORTED: &[&str] = &[
    "UNSUP-PROFILE-SELECTION",
    "UNSUP-SUPPORT-CLI-JSON",
    "UNSUP-PACKAGES",
    "UNSUP-VM",
    "UNSUP-NATIVE-FFI",
    "UNSUP-CONCURRENCY-REPLAY",
    "UNSUP-DEVICE",
    "UNSUP-CRITICAL",
    "UNSUP-LSP-EDITOR",
];
const STABILITIES: &[&str] = &["Experimental", "Preview", "Stable", "Internal", "Future"];
const TIERS: &[&str] = &["Tier1", "Tier2", "Tier3", "Unsupported"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub feature_count: usize,
    pub profile_count: usize,
    pub host_count: usize,
    pub native_target_count: usize,
    pub backend_count: usize,
    pub standard_package_count: usize,
    pub protocol_count: usize,
    pub unsupported_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StatusFeatureRecord {
    pub current_state: String,
    pub stability: String,
    pub current_profiles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StatusInputs {
    pub features: BTreeMap<String, StatusFeatureRecord>,
    pub supported_targets: BTreeSet<String>,
    pub future_cli_command: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SupportMatrix {
    schema_version: u32,
    updated: String,
    matrix_target: String,
    status: String,
    product: String,
    compiler_version: String,
    language_version: String,
    unicode_version: String,
    traceability_release: String,
    protocol_inventory: String,
    report: String,
    version_fixture: String,
    support_fixture: String,
    version_fixture_schema: String,
    support_fixture_schema: String,
    #[serde(default)]
    future_cli_commands: Vec<String>,
    tier_policy: TierPolicy,
    #[serde(default)]
    feature: Vec<Feature>,
    #[serde(default)]
    profile: Vec<Profile>,
    #[serde(default)]
    host_platform: Vec<HostPlatform>,
    #[serde(default)]
    native_target: Vec<NativeTarget>,
    #[serde(default)]
    backend: Vec<Backend>,
    #[serde(default)]
    standard_package: Vec<StandardPackage>,
    #[serde(default)]
    protocol: Vec<Protocol>,
    #[serde(default)]
    unsupported: Vec<Unsupported>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TierPolicy {
    tier1: String,
    tier2: String,
    tier3: String,
    unsupported: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Feature {
    id: String,
    current_state: String,
    stability: String,
    #[serde(default)]
    current_profiles: Vec<String>,
    #[serde(default)]
    candidate_1_0_profiles: Vec<String>,
    profile_note: String,
    #[serde(default)]
    evidence: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Profile {
    id: String,
    current_state: String,
    stability: String,
    selectable: bool,
    candidate_for_1_0: bool,
    #[serde(default)]
    allowed_effects: Vec<String>,
    #[serde(default)]
    memory_models: Vec<String>,
    #[serde(default)]
    runtime_models: Vec<String>,
    #[serde(default)]
    explicitly_unsupported: Vec<String>,
    #[serde(default)]
    sources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HostPlatform {
    id: String,
    platform: String,
    runner: String,
    architecture: String,
    tier: String,
    stability: String,
    compiler_build: bool,
    workspace_tests: bool,
    release_artifacts: bool,
    last_verified_commit: String,
    #[serde(default)]
    evidence: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeTarget {
    id: String,
    target: String,
    tier: String,
    stability: String,
    implemented: bool,
    backend: String,
    #[serde(default)]
    blockers: Vec<String>,
    #[serde(default)]
    sources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Backend {
    id: String,
    kind: String,
    device: String,
    tier: String,
    stability: String,
    implemented: bool,
    #[serde(default)]
    profiles: Vec<String>,
    #[serde(default)]
    blockers: Vec<String>,
    #[serde(default)]
    sources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StandardPackage {
    id: String,
    package: String,
    version: String,
    state: String,
    stability: String,
    implemented: bool,
    packaged: bool,
    #[serde(default)]
    profiles: Vec<String>,
    #[serde(default)]
    authorities: Vec<String>,
    #[serde(default)]
    evidence: Vec<String>,
    #[serde(default)]
    explicitly_unsupported: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Protocol {
    id: String,
    visibility: String,
    version: String,
    stability: String,
    implemented: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Unsupported {
    id: String,
    area: String,
    capability: String,
    reason: String,
    #[serde(default)]
    blockers: Vec<String>,
    #[serde(default)]
    sources: Vec<String>,
}

#[derive(Serialize)]
struct VersionFixture<'a> {
    schema: &'a str,
    proposed_command: &'a str,
    implemented: bool,
    product: &'a str,
    compiler_version: &'a str,
    language_version: &'a str,
    unicode_version: &'a str,
    support_fixture_schema: &'a str,
    matrix_status: &'a str,
}

#[derive(Serialize)]
struct SupportFixture<'a> {
    schema: &'a str,
    proposed_command: &'a str,
    implemented: bool,
    generated_from: &'static str,
    matrix_target: &'a str,
    matrix_status: &'a str,
    product: &'a str,
    compiler_version: &'a str,
    language_version: &'a str,
    unicode_version: &'a str,
    tier_policy: &'a TierPolicy,
    features: Vec<&'a Feature>,
    profiles: Vec<&'a Profile>,
    host_platforms: Vec<&'a HostPlatform>,
    native_targets: Vec<&'a NativeTarget>,
    backends: Vec<&'a Backend>,
    standard_packages: Vec<&'a StandardPackage>,
    protocols: Vec<&'a Protocol>,
    explicitly_unsupported: Vec<&'a Unsupported>,
}

#[derive(Deserialize)]
struct CargoManifest {
    workspace: CargoWorkspace,
}

#[derive(Deserialize)]
struct CargoWorkspace {
    package: CargoWorkspacePackage,
}

#[derive(Deserialize)]
struct CargoWorkspacePackage {
    version: String,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = load_matrix(root)?;
    let feature_records = traceability::feature_records(root, &matrix.traceability_release)?;
    let protocol_records = protocols::protocol_records(root)?;
    let gap_ids = gaps::registered_gap_ids(root)?;
    let authorities = governance::document_records(root)?;
    let mut errors = validate(
        root,
        &matrix,
        &feature_records,
        &protocol_records,
        &gap_ids,
        &authorities,
    );
    compare_generated(root, &matrix.report, &render(&matrix), &mut errors);
    compare_generated(
        root,
        &matrix.version_fixture,
        &render_version_fixture(&matrix),
        &mut errors,
    );
    compare_generated(
        root,
        &matrix.support_fixture,
        &render_support_fixture(&matrix),
        &mut errors,
    );
    finish(errors).map(|()| summary(&matrix))
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    load_and_validate(root).map(|matrix| render(&matrix))
}

pub fn render_version_fixture_repository(root: &Path) -> Result<String, Vec<String>> {
    load_and_validate(root).map(|matrix| render_version_fixture(&matrix))
}

pub fn render_support_fixture_repository(root: &Path) -> Result<String, Vec<String>> {
    load_and_validate(root).map(|matrix| render_support_fixture(&matrix))
}

pub(crate) fn status_inputs(root: &Path) -> Result<StatusInputs, Vec<String>> {
    let matrix = load_and_validate(root)?;
    let features = matrix
        .feature
        .iter()
        .map(|feature| {
            (
                feature.id.clone(),
                StatusFeatureRecord {
                    current_state: feature.current_state.clone(),
                    stability: feature.stability.clone(),
                    current_profiles: sorted_strings(&feature.current_profiles),
                },
            )
        })
        .collect();
    let supported_targets = matrix
        .native_target
        .iter()
        .filter(|target| target.implemented && target.tier != "Unsupported")
        .map(|target| target.id.clone())
        .collect();
    let future_cli_command = matrix
        .future_cli_commands
        .get(1)
        .cloned()
        .unwrap_or_default();
    Ok(StatusInputs {
        features,
        supported_targets,
        future_cli_command,
    })
}

fn load_and_validate(root: &Path) -> Result<SupportMatrix, Vec<String>> {
    let matrix = load_matrix(root)?;
    let feature_records = traceability::feature_records(root, &matrix.traceability_release)?;
    let protocol_records = protocols::protocol_records(root)?;
    let gap_ids = gaps::registered_gap_ids(root)?;
    let authorities = governance::document_records(root)?;
    finish(validate(
        root,
        &matrix,
        &feature_records,
        &protocol_records,
        &gap_ids,
        &authorities,
    ))?;
    Ok(matrix)
}

fn load_matrix(root: &Path) -> Result<SupportMatrix, Vec<String>> {
    let text = fs::read_to_string(root.join(MANIFEST_PATH)).map_err(|error| {
        vec![format!(
            "GOV-SUPPORT-0002: cannot read {MANIFEST_PATH}: {error}"
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-SUPPORT-0009: invalid support matrix {MANIFEST_PATH}: {error}"
        )]
    })
}

fn validate(
    root: &Path,
    matrix: &SupportMatrix,
    feature_records: &FeatureRecords,
    protocol_records: &ProtocolRecords,
    gap_ids: &BTreeSet<String>,
    authorities: &DocumentRecords,
) -> Vec<String> {
    let mut errors = Vec::new();
    if matrix.schema_version != 1 {
        errors.push(format!(
            "GOV-SUPPORT-0010: unsupported schema_version {}; expected 1",
            matrix.schema_version
        ));
    }
    if !is_date(&matrix.updated) {
        errors.push("GOV-SUPPORT-0010: updated must be a YYYY-MM-DD date".to_owned());
    }
    if matrix.matrix_target != "1.0-draft" || matrix.status != "Draft" {
        errors.push(
            "GOV-SUPPORT-0010: GOV-0108 must remain a Draft 1.0 matrix until later review"
                .to_owned(),
        );
    }
    if matrix.product != "ling" {
        errors.push("GOV-SUPPORT-0010: product must be ling".to_owned());
    }
    validate_versions(root, matrix, &mut errors);
    validate_internal_fixtures(matrix, &mut errors);
    validate_artifact_path(root, &matrix.protocol_inventory, true, &mut errors);
    validate_artifact_path(root, &matrix.report, false, &mut errors);
    validate_artifact_path(root, &matrix.version_fixture, false, &mut errors);
    validate_artifact_path(root, &matrix.support_fixture, false, &mut errors);
    for value in [
        &matrix.tier_policy.tier1,
        &matrix.tier_policy.tier2,
        &matrix.tier_policy.tier3,
        &matrix.tier_policy.unsupported,
    ] {
        if value.trim().is_empty() {
            errors.push("GOV-SUPPORT-0010: tier policy entries must not be empty".to_owned());
        }
    }

    let profiles = validate_profiles(root, &matrix.profile, &mut errors);
    validate_features(
        root,
        &matrix.feature,
        feature_records,
        &profiles,
        &mut errors,
    );
    validate_hosts(root, &matrix.host_platform, &mut errors);
    validate_native_targets(
        root,
        &matrix.native_target,
        gap_ids,
        &matrix.backend,
        &mut errors,
    );
    validate_backends(root, &matrix.backend, gap_ids, &profiles, &mut errors);
    validate_standard_packages(
        root,
        &matrix.standard_package,
        authorities,
        &profiles,
        &mut errors,
    );
    validate_protocols(&matrix.protocol, protocol_records, &mut errors);
    validate_unsupported(root, &matrix.unsupported, gap_ids, &mut errors);
    errors
}

fn validate_versions(root: &Path, matrix: &SupportMatrix, errors: &mut Vec<String>) {
    let manifest = match fs::read_to_string(root.join("Cargo.toml")) {
        Ok(text) => toml::from_str::<CargoManifest>(&text).map_err(|error| error.to_string()),
        Err(error) => Err(error.to_string()),
    };
    match manifest {
        Ok(manifest) if manifest.workspace.package.version == matrix.compiler_version => {}
        Ok(manifest) => errors.push(format!(
            "GOV-SUPPORT-0007: compiler_version {} differs from workspace version {}",
            matrix.compiler_version, manifest.workspace.package.version
        )),
        Err(error) => errors.push(format!(
            "GOV-SUPPORT-0002: cannot read workspace version: {error}"
        )),
    }
    let semantic = fs::read_to_string(root.join("crates/ling-semantic/src/lib.rs"));
    match semantic {
        Ok(text)
            if text.contains(&format!(
                "pub const LANGUAGE_VERSION: &str = {:?}",
                matrix.language_version
            )) => {}
        Ok(_) => errors.push(format!(
            "GOV-SUPPORT-0007: language_version {} is absent from ling-semantic",
            matrix.language_version
        )),
        Err(error) => errors.push(format!(
            "GOV-SUPPORT-0002: cannot read ling-semantic version: {error}"
        )),
    }
    let unicode = fs::read_to_string(root.join("crates/ling-unicode/src/lib.rs"));
    let expected = matrix.unicode_version.split('.').collect::<Vec<_>>();
    match (unicode, expected.as_slice()) {
        (Ok(text), [major, minor, patch])
            if text
                .replace(' ', "")
                .contains(&format!("UnicodeVersion::new({major},{minor},{patch})")) => {}
        (Ok(_), _) => errors.push(format!(
            "GOV-SUPPORT-0007: unicode_version {} is absent from ling-unicode",
            matrix.unicode_version
        )),
        (Err(error), _) => errors.push(format!(
            "GOV-SUPPORT-0002: cannot read ling-unicode version: {error}"
        )),
    }
}

fn validate_internal_fixtures(matrix: &SupportMatrix, errors: &mut Vec<String>) {
    for (field, value) in [
        ("version_fixture_schema", &matrix.version_fixture_schema),
        ("support_fixture_schema", &matrix.support_fixture_schema),
    ] {
        if !value.starts_with("ling.governance.") || !value.ends_with("/1") {
            errors.push(format!(
                "GOV-SUPPORT-0010: {field} must be an internal ling.governance.* schema"
            ));
        }
    }
    let expected = BTreeSet::from(["ling version --format json", "ling support --format json"]);
    let actual = matrix
        .future_cli_commands
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if actual != expected {
        errors.push(
            "GOV-SUPPORT-0010: future_cli_commands must list the two unimplemented ling JSON commands"
                .to_owned(),
        );
    }
}

fn validate_features(
    root: &Path,
    features: &[Feature],
    expected: &FeatureRecords,
    profiles: &BTreeMap<&str, &Profile>,
    errors: &mut Vec<String>,
) {
    let mut actual = BTreeMap::new();
    for feature in features {
        if !valid_id(&feature.id, "FTR-") {
            errors.push(format!(
                "GOV-SUPPORT-0010: invalid feature id {:?}",
                feature.id
            ));
        }
        if actual.insert(feature.id.as_str(), feature).is_some() {
            errors.push(format!(
                "GOV-SUPPORT-0001: duplicate feature id {}",
                display_id(&feature.id)
            ));
        }
        if !matches!(
            feature.current_state.as_str(),
            "Implemented" | "Partial" | "Unavailable"
        ) {
            errors.push(format!(
                "GOV-SUPPORT-0010: feature {} has invalid current_state {}",
                display_id(&feature.id),
                feature.current_state
            ));
        }
        if !STABILITIES.contains(&feature.stability.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0010: feature {} has invalid stability {}",
                display_id(&feature.id),
                feature.stability
            ));
        }
        if feature.profile_note.trim().is_empty() || feature.evidence.is_empty() {
            errors.push(format!(
                "GOV-SUPPORT-0006: feature {} needs a profile_note and evidence",
                display_id(&feature.id)
            ));
        }
        validate_profile_references(
            &feature.id,
            "current_profiles",
            &feature.current_profiles,
            profiles,
            true,
            errors,
        );
        validate_profile_references(
            &feature.id,
            "candidate_1_0_profiles",
            &feature.candidate_1_0_profiles,
            profiles,
            false,
            errors,
        );
        for path in &feature.evidence {
            validate_artifact_path(root, path, true, errors);
        }
    }
    for (id, record) in expected {
        match actual.get(id.as_str()) {
            Some(feature)
                if feature.stability == record.stability && record.scope == "Public" => {}
            Some(feature) => errors.push(format!(
                "GOV-SUPPORT-0007: feature {id} support metadata ({}, {}) differs from traceability ({}, {})",
                feature.stability, "Public", record.stability, record.scope
            )),
            None => errors.push(format!(
                "GOV-SUPPORT-0003: traceability feature {id} is absent from the support matrix"
            )),
        }
    }
    for id in actual.keys() {
        if !expected.contains_key(*id) {
            errors.push(format!(
                "GOV-SUPPORT-0003: support feature {id} is absent from traceability"
            ));
        }
    }
}

fn validate_profiles<'a>(
    root: &Path,
    profiles: &'a [Profile],
    errors: &mut Vec<String>,
) -> BTreeMap<&'a str, &'a Profile> {
    let mut records = BTreeMap::new();
    for profile in profiles {
        if records.insert(profile.id.as_str(), profile).is_some() {
            errors.push(format!(
                "GOV-SUPPORT-0001: duplicate profile id {}",
                display_id(&profile.id)
            ));
        }
        if !REQUIRED_PROFILES.contains(&profile.id.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0010: unknown profile {}",
                display_id(&profile.id)
            ));
        }
        if !STABILITIES.contains(&profile.stability.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0010: profile {} has invalid stability {}",
                display_id(&profile.id),
                profile.stability
            ));
        }
        if profile.current_state == "Unavailable"
            && (profile.selectable
                || !profile.allowed_effects.is_empty()
                || !profile.memory_models.is_empty()
                || !profile.runtime_models.is_empty())
        {
            errors.push(format!(
                "GOV-SUPPORT-0005: unavailable profile {} claims selectable/runtime support",
                display_id(&profile.id)
            ));
        }
        if profile.explicitly_unsupported.is_empty() || profile.sources.is_empty() {
            errors.push(format!(
                "GOV-SUPPORT-0006: profile {} needs explicit unsupported entries and sources",
                display_id(&profile.id)
            ));
        }
        for path in &profile.sources {
            validate_artifact_path(root, path, true, errors);
        }
    }
    for id in REQUIRED_PROFILES {
        if !records.contains_key(id) {
            errors.push(format!("GOV-SUPPORT-0003: required profile {id} is absent"));
        }
    }
    records
}

fn validate_profile_references(
    owner: &str,
    field: &str,
    values: &[String],
    profiles: &BTreeMap<&str, &Profile>,
    require_selectable: bool,
    errors: &mut Vec<String>,
) {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            errors.push(format!(
                "GOV-SUPPORT-0001: {owner} repeats {field} value {value}"
            ));
        }
        match profiles.get(value.as_str()) {
            Some(profile) if require_selectable && !profile.selectable => errors.push(format!(
                "GOV-SUPPORT-0005: {owner} claims current support in unselectable profile {value}"
            )),
            Some(_) => {}
            None => errors.push(format!(
                "GOV-SUPPORT-0003: {owner} references unknown profile {value}"
            )),
        }
    }
}

fn validate_hosts(root: &Path, hosts: &[HostPlatform], errors: &mut Vec<String>) {
    let ci = fs::read_to_string(root.join(".github/workflows/ci.yml"));
    let mut records = BTreeMap::new();
    for host in hosts {
        if records.insert(host.id.as_str(), host).is_some() {
            errors.push(format!(
                "GOV-SUPPORT-0001: duplicate host id {}",
                display_id(&host.id)
            ));
        }
        validate_tier(&host.id, &host.tier, errors);
        if !STABILITIES.contains(&host.stability.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0010: host {} has invalid stability {}",
                display_id(&host.id),
                host.stability
            ));
        }
        if host.tier == "Tier1"
            && (!host.compiler_build || !host.workspace_tests || !host.release_artifacts)
        {
            errors.push(format!(
                "GOV-SUPPORT-0005: Tier1 host {} lacks build/test/release artifacts",
                display_id(&host.id)
            ));
        }
        if host.tier == "Tier2" && (!host.compiler_build || !host.workspace_tests) {
            errors.push(format!(
                "GOV-SUPPORT-0005: Tier2 host {} lacks build or workspace tests",
                display_id(&host.id)
            ));
        }
        if !valid_git_id(&host.last_verified_commit) {
            errors.push(format!(
                "GOV-SUPPORT-0010: host {} needs a full lowercase verified commit",
                display_id(&host.id)
            ));
        }
        match &ci {
            Ok(text) if text.contains(&host.runner) => {}
            Ok(_) => errors.push(format!(
                "GOV-SUPPORT-0004: host {} runner {} is absent from CI",
                display_id(&host.id),
                host.runner
            )),
            Err(error) => errors.push(format!(
                "GOV-SUPPORT-0002: cannot read CI workflow: {error}"
            )),
        }
        for value in [&host.platform, &host.runner, &host.architecture] {
            if value.trim().is_empty() {
                errors.push(format!(
                    "GOV-SUPPORT-0010: host {} has an empty identity field",
                    display_id(&host.id)
                ));
            }
        }
        for path in &host.evidence {
            validate_artifact_path(root, path, true, errors);
        }
    }
    for id in REQUIRED_HOSTS {
        if !records.contains_key(id) {
            errors.push(format!("GOV-SUPPORT-0003: required host {id} is absent"));
        }
    }
}

fn validate_native_targets(
    root: &Path,
    targets: &[NativeTarget],
    gap_ids: &BTreeSet<String>,
    backends: &[Backend],
    errors: &mut Vec<String>,
) {
    if targets.is_empty() {
        errors.push("GOV-SUPPORT-0003: no native target tier is recorded".to_owned());
    }
    let backend_ids = backends
        .iter()
        .map(|backend| backend.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut ids = BTreeSet::new();
    for target in targets {
        if !valid_id(&target.id, "TARGET-") || !ids.insert(target.id.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0001: invalid or duplicate native target id {}",
                display_id(&target.id)
            ));
        }
        validate_tier(&target.id, &target.tier, errors);
        if !STABILITIES.contains(&target.stability.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0010: target {} has invalid stability {}",
                display_id(&target.id),
                target.stability
            ));
        }
        if !target.implemented && target.tier != "Unsupported" {
            errors.push(format!(
                "GOV-SUPPORT-0005: unimplemented target {} must be Unsupported",
                display_id(&target.id)
            ));
        }
        if target.implemented && !backend_ids.contains(target.backend.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0003: implemented target {} references unknown backend {}",
                display_id(&target.id),
                target.backend
            ));
        }
        validate_blockers(&target.id, &target.blockers, gap_ids, errors);
        for path in &target.sources {
            validate_artifact_path(root, path, true, errors);
        }
    }
}

fn validate_backends(
    root: &Path,
    backends: &[Backend],
    gap_ids: &BTreeSet<String>,
    profiles: &BTreeMap<&str, &Profile>,
    errors: &mut Vec<String>,
) {
    let mut records = BTreeMap::new();
    for backend in backends {
        if !valid_id(&backend.id, "BACKEND-")
            || records.insert(backend.id.as_str(), backend).is_some()
        {
            errors.push(format!(
                "GOV-SUPPORT-0001: invalid or duplicate backend id {}",
                display_id(&backend.id)
            ));
        }
        validate_backend_claim(backend, errors);
        validate_profile_references(
            &backend.id,
            "profiles",
            &backend.profiles,
            profiles,
            true,
            errors,
        );
        validate_blockers(&backend.id, &backend.blockers, gap_ids, errors);
        for path in &backend.sources {
            validate_artifact_path(root, path, true, errors);
        }
    }
    for id in REQUIRED_BACKENDS {
        if !records.contains_key(id) {
            errors.push(format!("GOV-SUPPORT-0003: required backend {id} is absent"));
        }
    }
}

fn validate_backend_claim(backend: &Backend, errors: &mut Vec<String>) {
    validate_tier(&backend.id, &backend.tier, errors);
    if !STABILITIES.contains(&backend.stability.as_str()) {
        errors.push(format!(
            "GOV-SUPPORT-0010: backend {} has invalid stability {}",
            display_id(&backend.id),
            backend.stability
        ));
    }
    if !backend.implemented && backend.tier != "Unsupported" {
        errors.push(format!(
            "GOV-SUPPORT-0005: unimplemented backend {} must be Unsupported",
            display_id(&backend.id)
        ));
    }
    if backend.implemented && backend.tier == "Unsupported" {
        errors.push(format!(
            "GOV-SUPPORT-0005: implemented backend {} cannot be Unsupported",
            display_id(&backend.id)
        ));
    }
    if backend.kind.trim().is_empty()
        || backend.device.trim().is_empty()
        || backend.sources.is_empty()
    {
        errors.push(format!(
            "GOV-SUPPORT-0006: backend {} needs kind, device, and sources",
            display_id(&backend.id)
        ));
    }
}

fn validate_standard_packages(
    root: &Path,
    packages: &[StandardPackage],
    authorities: &DocumentRecords,
    profiles: &BTreeMap<&str, &Profile>,
    errors: &mut Vec<String>,
) {
    if packages.is_empty() {
        errors.push("GOV-SUPPORT-0003: no standard package stability is recorded".to_owned());
    }
    let mut ids = BTreeSet::new();
    for package in packages {
        if !valid_id(&package.id, "STD-") || !ids.insert(package.id.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0001: invalid or duplicate standard package id {}",
                display_id(&package.id)
            ));
        }
        if !STABILITIES.contains(&package.stability.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0010: standard package {} has invalid stability {}",
                display_id(&package.id),
                package.stability
            ));
        }
        if package.packaged && !package.implemented {
            errors.push(format!(
                "GOV-SUPPORT-0005: unimplemented package {} cannot be packaged",
                display_id(&package.id)
            ));
        }
        validate_profile_references(
            &package.id,
            "profiles",
            &package.profiles,
            profiles,
            true,
            errors,
        );
        let mut has_accepted = false;
        for id in &package.authorities {
            match authorities.get(id) {
                Some(record) => has_accepted |= record.status == "Accepted",
                None => errors.push(format!(
                    "GOV-SUPPORT-0003: package {} references unknown authority {id}",
                    display_id(&package.id)
                )),
            }
        }
        if matches!(package.stability.as_str(), "Preview" | "Stable") && !has_accepted {
            errors.push(format!(
                "GOV-SUPPORT-0006: {} package {} has no Accepted authority",
                package.stability,
                display_id(&package.id)
            ));
        }
        if package.evidence.is_empty() || package.explicitly_unsupported.is_empty() {
            errors.push(format!(
                "GOV-SUPPORT-0006: package {} needs evidence and explicit unsupported boundaries",
                display_id(&package.id)
            ));
        }
        for path in &package.evidence {
            validate_artifact_path(root, path, true, errors);
        }
    }
}

fn validate_protocols(
    protocols: &[Protocol],
    expected: &ProtocolRecords,
    errors: &mut Vec<String>,
) {
    let mut actual = BTreeMap::new();
    for protocol in protocols {
        if !valid_id(&protocol.id, "PROTO-")
            || actual.insert(protocol.id.as_str(), protocol).is_some()
        {
            errors.push(format!(
                "GOV-SUPPORT-0001: invalid or duplicate protocol id {}",
                display_id(&protocol.id)
            ));
        }
    }
    for (id, record) in expected {
        match actual.get(id.as_str()) {
            Some(protocol)
                if protocol.visibility == record.visibility
                    && protocol.version == record.current_version
                    && protocol.stability == record.stability
                    && protocol.implemented == record.implemented => {}
            Some(_) => errors.push(format!(
                "GOV-SUPPORT-0007: protocol {id} differs from protocol-inventory.toml"
            )),
            None => errors.push(format!(
                "GOV-SUPPORT-0003: protocol {id} is absent from the support matrix"
            )),
        }
    }
    for id in actual.keys() {
        if !expected.contains_key(*id) {
            errors.push(format!(
                "GOV-SUPPORT-0003: support protocol {id} is absent from protocol inventory"
            ));
        }
    }
}

fn validate_unsupported(
    root: &Path,
    unsupported: &[Unsupported],
    gap_ids: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let mut ids = BTreeSet::new();
    for item in unsupported {
        if !valid_id(&item.id, "UNSUP-") || !ids.insert(item.id.as_str()) {
            errors.push(format!(
                "GOV-SUPPORT-0001: invalid or duplicate unsupported id {}",
                display_id(&item.id)
            ));
        }
        if item.area.trim().is_empty()
            || item.capability.trim().is_empty()
            || item.reason.trim().is_empty()
            || item.sources.is_empty()
        {
            errors.push(format!(
                "GOV-SUPPORT-0006: unsupported record {} needs area, capability, reason, and sources",
                display_id(&item.id)
            ));
        }
        validate_blockers(&item.id, &item.blockers, gap_ids, errors);
        for path in &item.sources {
            validate_artifact_path(root, path, true, errors);
        }
    }
    for id in REQUIRED_UNSUPPORTED {
        if !ids.contains(id) {
            errors.push(format!(
                "GOV-SUPPORT-0003: required unsupported record {id} is absent"
            ));
        }
    }
}

fn validate_blockers(
    owner: &str,
    blockers: &[String],
    gap_ids: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let mut seen = BTreeSet::new();
    for blocker in blockers {
        if !seen.insert(blocker) {
            errors.push(format!(
                "GOV-SUPPORT-0001: {owner} repeats blocker {blocker}"
            ));
        }
        if !gap_ids.contains(blocker) {
            errors.push(format!(
                "GOV-SUPPORT-0003: {owner} references unknown gap {blocker}"
            ));
        }
    }
}

fn validate_tier(owner: &str, tier: &str, errors: &mut Vec<String>) {
    if !TIERS.contains(&tier) {
        errors.push(format!("GOV-SUPPORT-0010: {owner} has invalid tier {tier}"));
    }
}

fn validate_artifact_path(root: &Path, value: &str, must_exist: bool, errors: &mut Vec<String>) {
    if !is_relative_path(value) {
        errors.push(format!(
            "GOV-SUPPORT-0010: path {value:?} must be normalized and repository-relative"
        ));
    } else if must_exist && !root.join(value).exists() {
        errors.push(format!("GOV-SUPPORT-0004: path does not exist: {value}"));
    }
}

fn compare_generated(root: &Path, path: &str, expected: &str, errors: &mut Vec<String>) {
    match fs::read_to_string(root.join(path)) {
        Ok(actual) if normalize_newlines(&actual) == expected => {}
        Ok(_) => errors.push(format!(
            "GOV-SUPPORT-0008: {path} is not the deterministic rendering of {MANIFEST_PATH}"
        )),
        Err(error) => errors.push(format!(
            "GOV-SUPPORT-0002: cannot read generated artifact {path}: {error}"
        )),
    }
}

fn render(matrix: &SupportMatrix) -> String {
    let mut output = String::new();
    output.push_str("# Ling 1.0 support matrix draft / Ling 1.0 支持矩阵草案\n\n");
    output.push_str("> Generated deterministically from `support-matrix.toml`; do not edit this report manually.\n> 本报告由 `support-matrix.toml` 确定性生成；不得手工编辑。\n\n");
    output.push_str(&format!(
        "- Matrix target: `{}`\n- Status: `{}`\n- Current compiler: `{}`\n- Current language: `{}`\n- Unicode: `{}`\n- Updated: `{}`\n\n",
        matrix.matrix_target,
        matrix.status,
        matrix.compiler_version,
        matrix.language_version,
        matrix.unicode_version,
        matrix.updated
    ));
    output.push_str("This draft separates current evidence from candidate 1.0 scope. Empty current-profile lists mean the Seed implementation is unprofiled; candidate profile entries are planning input, not support claims. No Native target, VM, device backend, or Critical guarantee is currently supported.\n\n本草案严格区分当前证据与 1.0 候选范围。当前 profile 为空表示 Seed 实现尚未 profile 化；候选 profile 仅是规划输入，不是支持声明。目前不支持 Native target、VM、设备 backend 或 Critical 保证。\n\n");

    output.push_str("## Feature/profile/stability\n\n| Feature | Current state | Stability | Current profiles | Candidate 1.0 profiles | Boundary | Evidence |\n| --- | --- | --- | --- | --- | --- | --- |\n");
    for item in sorted_by_id(&matrix.feature, |item| &item.id) {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | {} | {} | {} |\n",
            item.id,
            item.current_state,
            item.stability,
            code_list(&item.current_profiles),
            code_list(&item.candidate_1_0_profiles),
            escape_cell(&item.profile_note),
            path_list(&item.evidence)
        ));
    }

    output.push_str("\n## Profiles\n\n| Profile | Current state | Stability | Selectable | 1.0 candidate | Allowed Effects | Memory models | Runtime models | Explicitly unsupported |\n| --- | --- | --- | ---: | ---: | --- | --- | --- | --- |\n");
    for item in sorted_by_id(&matrix.profile, |item| &item.id) {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | {} | {} | {} | {} | {} |\n",
            item.id,
            item.current_state,
            item.stability,
            yes_no(item.selectable),
            yes_no(item.candidate_for_1_0),
            code_list(&item.allowed_effects),
            code_list(&item.memory_models),
            code_list(&item.runtime_models),
            text_list(&item.explicitly_unsupported)
        ));
    }

    output.push_str("\n## Host platform tiers\n\n| Host | Platform | Runner | Architecture | Tier | Stability | Build | Tests | Release artifacts | Last verified commit |\n| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: | --- |\n");
    for item in sorted_by_id(&matrix.host_platform, |item| &item.id) {
        output.push_str(&format!(
            "| `{}` | {} | `{}` | {} | `{}` | `{}` | {} | {} | {} | `{}` |\n",
            item.id,
            escape_cell(&item.platform),
            item.runner,
            escape_cell(&item.architecture),
            item.tier,
            item.stability,
            yes_no(item.compiler_build),
            yes_no(item.workspace_tests),
            yes_no(item.release_artifacts),
            item.last_verified_commit
        ));
    }

    output.push_str("\n## Native target tiers\n\n| Target ID | Target | Tier | Stability | Implemented | Backend | Blockers |\n| --- | --- | --- | --- | ---: | --- | --- |\n");
    for item in sorted_by_id(&matrix.native_target, |item| &item.id) {
        output.push_str(&format!(
            "| `{}` | {} | `{}` | `{}` | {} | {} | {} |\n",
            item.id,
            escape_cell(&item.target),
            item.tier,
            item.stability,
            yes_no(item.implemented),
            code_or_dash(&item.backend),
            code_list(&item.blockers)
        ));
    }

    output.push_str("\n## Backend/device tiers\n\n| Backend | Kind | Device | Tier | Stability | Implemented | Profiles | Blockers |\n| --- | --- | --- | --- | --- | ---: | --- | --- |\n");
    for item in sorted_by_id(&matrix.backend, |item| &item.id) {
        output.push_str(&format!(
            "| `{}` | {} | {} | `{}` | `{}` | {} | {} | {} |\n",
            item.id,
            escape_cell(&item.kind),
            escape_cell(&item.device),
            item.tier,
            item.stability,
            yes_no(item.implemented),
            code_list(&item.profiles),
            code_list(&item.blockers)
        ));
    }

    output.push_str("\n## Standard package stability\n\n| ID | Package | Version | State | Stability | Implemented | Packaged | Profiles | Explicitly unsupported |\n| --- | --- | --- | --- | --- | ---: | ---: | --- | --- |\n");
    for item in sorted_by_id(&matrix.standard_package, |item| &item.id) {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | {} | {} | {} | {} |\n",
            item.id,
            item.package,
            item.version,
            item.state,
            item.stability,
            yes_no(item.implemented),
            yes_no(item.packaged),
            code_list(&item.profiles),
            text_list(&item.explicitly_unsupported)
        ));
    }

    output.push_str("\n## Protocol versions\n\n| Protocol | Visibility | Version | Stability | Implemented |\n| --- | --- | --- | --- | ---: |\n");
    for item in sorted_by_id(&matrix.protocol, |item| &item.id) {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | `{}` | {} |\n",
            item.id,
            item.visibility,
            code_or_dash(&item.version),
            item.stability,
            yes_no(item.implemented)
        ));
    }

    output.push_str("\n## Explicitly unsupported\n\n| ID | Area | Capability | Reason | Blockers | Sources |\n| --- | --- | --- | --- | --- | --- |\n");
    for item in sorted_by_id(&matrix.unsupported, |item| &item.id) {
        output.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} |\n",
            item.id,
            escape_cell(&item.area),
            escape_cell(&item.capability),
            escape_cell(&item.reason),
            code_list(&item.blockers),
            path_list(&item.sources)
        ));
    }

    output.push_str("\n## Tier policy\n\n");
    output.push_str(&format!(
        "- `Tier1`: {}\n- `Tier2`: {}\n- `Tier3`: {}\n- `Unsupported`: {}\n",
        matrix.tier_policy.tier1,
        matrix.tier_policy.tier2,
        matrix.tier_policy.tier3,
        matrix.tier_policy.unsupported
    ));
    output.push_str("\n## Future CLI fixtures\n\nThe checked-in JSON files are internal `ling.governance.*` fixtures with `implemented: false`. They do not create `ling version` or `ling support`, and they are not public compatibility contracts. A later accepted CLI/protocol task must define and migrate any public schema.\n\n```text\ncargo xtask support verify\ncargo xtask support render\ncargo xtask support render-version-fixture\ncargo xtask support render-support-fixture\n```\n");
    output
}

fn render_version_fixture(matrix: &SupportMatrix) -> String {
    let value = VersionFixture {
        schema: &matrix.version_fixture_schema,
        proposed_command: matrix
            .future_cli_commands
            .first()
            .map(String::as_str)
            .unwrap_or_default(),
        implemented: false,
        product: &matrix.product,
        compiler_version: &matrix.compiler_version,
        language_version: &matrix.language_version,
        unicode_version: &matrix.unicode_version,
        support_fixture_schema: &matrix.support_fixture_schema,
        matrix_status: &matrix.status,
    };
    json(&value)
}

fn render_support_fixture(matrix: &SupportMatrix) -> String {
    let value = SupportFixture {
        schema: &matrix.support_fixture_schema,
        proposed_command: matrix
            .future_cli_commands
            .get(1)
            .map(String::as_str)
            .unwrap_or_default(),
        implemented: false,
        generated_from: MANIFEST_PATH,
        matrix_target: &matrix.matrix_target,
        matrix_status: &matrix.status,
        product: &matrix.product,
        compiler_version: &matrix.compiler_version,
        language_version: &matrix.language_version,
        unicode_version: &matrix.unicode_version,
        tier_policy: &matrix.tier_policy,
        features: sorted_by_id(&matrix.feature, |item| &item.id),
        profiles: sorted_by_id(&matrix.profile, |item| &item.id),
        host_platforms: sorted_by_id(&matrix.host_platform, |item| &item.id),
        native_targets: sorted_by_id(&matrix.native_target, |item| &item.id),
        backends: sorted_by_id(&matrix.backend, |item| &item.id),
        standard_packages: sorted_by_id(&matrix.standard_package, |item| &item.id),
        protocols: sorted_by_id(&matrix.protocol, |item| &item.id),
        explicitly_unsupported: sorted_by_id(&matrix.unsupported, |item| &item.id),
    };
    json(&value)
}

fn json(value: &impl Serialize) -> String {
    let mut output = serde_json::to_string_pretty(value)
        .expect("support registry strings are JSON-serializable");
    output.push('\n');
    output
}

fn sorted_by_id<T, F>(values: &[T], id: F) -> Vec<&T>
where
    F: Fn(&T) -> &str,
{
    let mut values = values.iter().collect::<Vec<_>>();
    values.sort_by(|left, right| id(left).cmp(id(right)));
    values
}

fn sorted_strings(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort();
    values
}

fn path_list(paths: &[String]) -> String {
    if paths.is_empty() {
        return "—".to_owned();
    }
    paths
        .iter()
        .map(|path| format!("[`{}`]({})", escape_cell(path), report_link(path)))
        .collect::<Vec<_>>()
        .join("<br>")
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

fn text_list(values: &[String]) -> String {
    if values.is_empty() {
        return "—".to_owned();
    }
    values
        .iter()
        .map(|value| escape_cell(value))
        .collect::<Vec<_>>()
        .join("<br>")
}

fn code_or_dash(value: &str) -> String {
    if value.is_empty() {
        "—".to_owned()
    } else {
        format!("`{}`", escape_cell(value))
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn escape_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', "<br>")
}

fn summary(matrix: &SupportMatrix) -> CheckSummary {
    CheckSummary {
        feature_count: matrix.feature.len(),
        profile_count: matrix.profile.len(),
        host_count: matrix.host_platform.len(),
        native_target_count: matrix.native_target.len(),
        backend_count: matrix.backend.len(),
        standard_package_count: matrix.standard_package.len(),
        protocol_count: matrix.protocol.len(),
        unsupported_count: matrix.unsupported.len(),
    }
}

fn valid_id(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        !suffix.is_empty()
            && !suffix.starts_with('-')
            && !suffix.ends_with('-')
            && suffix
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-')
    })
}

fn valid_git_id(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
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

fn display_id(value: &str) -> &str {
    if value.is_empty() {
        "<missing-id>"
    } else {
        value
    }
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
    fn rejects_unavailable_profile_that_claims_runtime_support() {
        let profile = Profile {
            id: "Explore".to_owned(),
            current_state: "Unavailable".to_owned(),
            stability: "Experimental".to_owned(),
            selectable: true,
            candidate_for_1_0: true,
            allowed_effects: vec!["Console".to_owned()],
            memory_models: Vec::new(),
            runtime_models: vec!["VM".to_owned()],
            explicitly_unsupported: vec!["none".to_owned()],
            sources: vec!["docs/SEMANTICS.md".to_owned()],
        };
        let mut errors = Vec::new();
        validate_profiles(Path::new("."), &[profile], &mut errors);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("claims selectable/runtime support"))
        );
    }

    #[test]
    fn rejects_unimplemented_backend_with_supported_tier() {
        let backend = Backend {
            id: "BACKEND-VM".to_owned(),
            kind: "VM".to_owned(),
            device: "CPU".to_owned(),
            tier: "Tier2".to_owned(),
            stability: "Experimental".to_owned(),
            implemented: false,
            profiles: Vec::new(),
            blockers: Vec::new(),
            sources: vec!["docs/ROADMAP-1.0.md".to_owned()],
        };
        let mut errors = Vec::new();
        validate_backend_claim(&backend, &mut errors);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("must be Unsupported"))
        );
    }

    #[test]
    fn rejects_protocol_version_drift_and_omission() {
        let expected = BTreeMap::from([
            (
                "PROTO-A".to_owned(),
                protocols::ProtocolRecord {
                    category: "JSON".to_owned(),
                    visibility: "Public".to_owned(),
                    current_version: "1".to_owned(),
                    stability: "Preview".to_owned(),
                    implemented: true,
                    public_schema: true,
                    canonical: false,
                },
            ),
            (
                "PROTO-B".to_owned(),
                protocols::ProtocolRecord {
                    category: "Bytecode".to_owned(),
                    visibility: "Planned public".to_owned(),
                    current_version: String::new(),
                    stability: "Future".to_owned(),
                    implemented: false,
                    public_schema: false,
                    canonical: false,
                },
            ),
        ]);
        let actual = vec![Protocol {
            id: "PROTO-A".to_owned(),
            visibility: "Public".to_owned(),
            version: "2".to_owned(),
            stability: "Preview".to_owned(),
            implemented: true,
        }];
        let mut errors = Vec::new();
        validate_protocols(&actual, &expected, &mut errors);
        assert!(errors.iter().any(|error| error.contains("differs")));
        assert!(errors.iter().any(|error| error.contains("PROTO-B")));
    }

    #[test]
    fn rejects_unknown_gap_blocker() {
        let mut errors = Vec::new();
        validate_blockers(
            "UNSUP-TEST",
            &["GAP-UNKNOWN".to_owned()],
            &BTreeSet::new(),
            &mut errors,
        );
        assert!(errors.iter().any(|error| error.contains("unknown gap")));
    }

    #[test]
    fn internal_fixture_schemas_cannot_masquerade_as_public_contracts() {
        let matrix = load_matrix(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .and_then(Path::parent)
                .expect("xtask is under tools/xtask"),
        )
        .expect("repository matrix parses");
        let mut errors = Vec::new();
        validate_internal_fixtures(&matrix, &mut errors);
        assert!(errors.is_empty());
        assert!(render_version_fixture(&matrix).contains("\"implemented\": false"));
        assert!(render_support_fixture(&matrix).contains("ling.governance.support-fixture/1"));
    }

    #[test]
    fn support_json_rendering_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let matrix = load_matrix(root).expect("repository matrix parses");
        assert_eq!(
            render_support_fixture(&matrix),
            render_support_fixture(&matrix)
        );
        assert_eq!(
            render_version_fixture(&matrix),
            render_version_fixture(&matrix)
        );
    }

    #[test]
    fn repository_support_matrix_is_valid_and_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("repository support matrix is valid");
        assert_eq!(summary.feature_count, 7);
        assert_eq!(summary.profile_count, 3);
        assert_eq!(summary.host_count, 3);
        assert_eq!(summary.protocol_count, 44);
        assert_eq!(summary.unsupported_count, 9);
    }

    #[test]
    fn repository_standard_package_scope_is_exact_and_not_stable() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let matrix = load_matrix(root).expect("repository matrix parses");
        assert_eq!(matrix.standard_package.len(), 1);
        let package = &matrix.standard_package[0];
        assert_eq!(package.id, "STD-LING-PRELUDE");
        assert_eq!(package.package, "Ling.Prelude");
        assert_eq!(package.version, "0.0.1-dev");
        assert_eq!(package.state, "BuiltinOnly");
        assert_eq!(package.stability, "Preview");
        assert!(package.implemented);
        assert!(!package.packaged);
        assert!(package.profiles.is_empty());
        assert_eq!(package.authorities, ["DEC-0014"]);
        assert_eq!(package.explicitly_unsupported.len(), 3);
    }

    #[test]
    fn repository_registry_deferment_is_explicit_and_local_protocols_stay_experimental() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let matrix = load_matrix(root).expect("repository matrix parses");
        let unsupported = matrix
            .unsupported
            .iter()
            .find(|item| item.id == "UNSUP-PACKAGES")
            .expect("package registry deferment is recorded");
        assert_eq!(
            unsupported.capability,
            "Package installation, publication, or registry distribution"
        );
        assert!(unsupported.reason.contains("DEC-0228"));
        assert!(unsupported.reason.contains("Ling 1.0"));
        assert!(
            unsupported
                .sources
                .iter()
                .any(|source| source == "docs/decisions/0228-registry-deferred-through-v1.md")
        );
        assert!(
            matrix
                .protocol
                .iter()
                .all(|protocol| protocol.id != "PROTO-PACKAGE-REGISTRY")
        );
        for id in [
            "PROTO-PACKAGE-MANIFEST",
            "PROTO-PACKAGE-IDENTITY",
            "PROTO-LOCKFILE",
        ] {
            let protocol = matrix
                .protocol
                .iter()
                .find(|protocol| protocol.id == id)
                .unwrap_or_else(|| panic!("missing local package protocol {id}"));
            assert!(protocol.implemented, "{id}");
            assert_eq!(protocol.stability, "Experimental", "{id}");
        }
    }
}
