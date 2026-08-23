use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::gaps;
use crate::support::{self, StatusInputs};
use crate::traceability::{self, FeatureRecords};

const MANIFEST_PATH: &str = "docs/status/implementation-status.toml";
const TASK_STATES: &[&str] = &[
    "Ready",
    "In Progress",
    "BlockedSpec",
    "BlockedDependency",
    "Review",
    "Done",
];
const TASK_SIZES: &[&str] = &["XS", "S", "M", "L", "XL"];
const FEATURE_STATES: &[&str] = &["Unavailable", "Partial", "Implemented"];
const FEATURE_STABILITIES: &[&str] =
    &["Experimental", "Preview", "Stable", "Deprecated", "Removed"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub task_count: usize,
    pub done_task_count: usize,
    pub feature_count: usize,
    pub blocked_feature_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Registry {
    schema_version: u32,
    feature_schema_version: u32,
    updated: String,
    authority: String,
    plan_root: String,
    recommended_plan_root: String,
    baseline_commit: String,
    baseline_release: String,
    feature_release: String,
    support_matrix: String,
    feature_report: String,
    release_notes: String,
    cli_fixture: String,
    cli_fixture_schema: String,
    future_cli_command: String,
    cli_implemented: bool,
    #[serde(default, rename = "task")]
    tasks: Vec<Task>,
    #[serde(default, rename = "feature")]
    features: Vec<FeatureStatus>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Task {
    id: String,
    title: String,
    state: String,
    release: String,
    size: String,
    owner: String,
    branch: String,
    #[serde(default)]
    depends_on: Vec<String>,
    #[serde(default)]
    spec: Vec<String>,
    #[serde(default)]
    artifacts: Vec<String>,
    #[serde(default)]
    acceptance: Vec<String>,
    verified_against_commit: String,
    completion_commit: String,
    integration_state: String,
    completed: String,
    #[serde(default)]
    notes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct FeatureStatus {
    id: String,
    current_state: String,
    stability: String,
    implemented: bool,
    tested: bool,
    documented: bool,
    #[serde(default)]
    blockers: Vec<String>,
    last_verified_commit: String,
    #[serde(default)]
    supported_profiles: Vec<String>,
    #[serde(default)]
    supported_targets: Vec<String>,
    #[serde(default)]
    evidence: Vec<String>,
}

#[derive(Serialize)]
struct CliFixture<'a> {
    schema: &'a str,
    proposed_command: &'a str,
    implemented: bool,
    public_contract: bool,
    generated_from: &'static str,
    release: &'a str,
    feature_schema_version: u32,
    features: Vec<CliFeature<'a>>,
}

#[derive(Serialize)]
struct CliFeature<'a> {
    id: &'a str,
    title: LocalizedTitle<'a>,
    current_state: &'a str,
    stability: &'a str,
    implemented: bool,
    tested: bool,
    documented: bool,
    blockers: &'a [String],
    last_verified_commit: &'a str,
    supported_profiles: &'a [String],
    supported_targets: &'a [String],
}

#[derive(Serialize)]
struct LocalizedTitle<'a> {
    zh: &'a str,
    en: &'a str,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let (registry, trace_features, support_inputs, gap_ids) = load_and_validate(root)?;
    let mut errors = Vec::new();
    compare_generated(
        root,
        &registry.feature_report,
        &render(&registry, &trace_features),
        &mut errors,
    );
    compare_generated(
        root,
        &registry.release_notes,
        &render_release_notes(&registry, &trace_features),
        &mut errors,
    );
    compare_generated(
        root,
        &registry.cli_fixture,
        &render_cli_fixture(&registry, &trace_features),
        &mut errors,
    );
    finish(errors).map(|()| summary(&registry, &gap_ids, &support_inputs))
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    let (registry, trace_features, _, _) = load_and_validate(root)?;
    Ok(render(&registry, &trace_features))
}

pub fn render_release_notes_repository(root: &Path) -> Result<String, Vec<String>> {
    let (registry, trace_features, _, _) = load_and_validate(root)?;
    Ok(render_release_notes(&registry, &trace_features))
}

pub fn render_cli_fixture_repository(root: &Path) -> Result<String, Vec<String>> {
    let (registry, trace_features, _, _) = load_and_validate(root)?;
    Ok(render_cli_fixture(&registry, &trace_features))
}

fn load_and_validate(
    root: &Path,
) -> Result<(Registry, FeatureRecords, StatusInputs, BTreeSet<String>), Vec<String>> {
    let registry = load_registry(root)?;
    let trace_features = traceability::feature_records(root, &registry.feature_release)?;
    let support_inputs = support::status_inputs(root)?;
    let gap_ids = gaps::registered_gap_ids(root)?;
    finish(validate(
        root,
        &registry,
        &trace_features,
        &support_inputs,
        &gap_ids,
    ))?;
    Ok((registry, trace_features, support_inputs, gap_ids))
}

fn load_registry(root: &Path) -> Result<Registry, Vec<String>> {
    let text = fs::read_to_string(root.join(MANIFEST_PATH)).map_err(|error| {
        vec![format!(
            "GOV-STATUS-0002: cannot read {MANIFEST_PATH}: {error}"
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-STATUS-0003: invalid status registry {MANIFEST_PATH}: {error}"
        )]
    })
}

fn validate(
    root: &Path,
    registry: &Registry,
    trace_features: &FeatureRecords,
    support_inputs: &StatusInputs,
    gap_ids: &BTreeSet<String>,
) -> Vec<String> {
    let mut errors = Vec::new();
    validate_metadata(root, registry, support_inputs, &mut errors);
    validate_tasks(root, &registry.tasks, &mut errors);
    validate_features(
        root,
        &registry.features,
        trace_features,
        support_inputs,
        gap_ids,
        &mut errors,
    );
    errors
}

fn validate_metadata(
    root: &Path,
    registry: &Registry,
    support_inputs: &StatusInputs,
    errors: &mut Vec<String>,
) {
    if registry.schema_version != 2 {
        errors.push(format!(
            "GOV-STATUS-0004: schema_version must be 2, found {}",
            registry.schema_version
        ));
    }
    if registry.feature_schema_version != 1 {
        errors.push(format!(
            "GOV-STATUS-0004: feature_schema_version must be 1, found {}",
            registry.feature_schema_version
        ));
    }
    if !valid_date(&registry.updated) {
        errors.push(format!(
            "GOV-STATUS-0004: updated must be YYYY-MM-DD, found {:?}",
            registry.updated
        ));
    }
    if !valid_commit(&registry.baseline_commit) {
        errors
            .push("GOV-STATUS-0004: baseline_commit must be a lowercase 40-hex commit".to_owned());
    }
    if registry.baseline_release.is_empty() || registry.feature_release.is_empty() {
        errors.push(
            "GOV-STATUS-0004: baseline_release and feature_release must be non-empty".to_owned(),
        );
    }
    for (label, path, must_exist) in [
        ("authority", registry.authority.as_str(), true),
        ("plan_root", registry.plan_root.as_str(), true),
        ("support_matrix", registry.support_matrix.as_str(), true),
        ("feature_report", registry.feature_report.as_str(), false),
        ("release_notes", registry.release_notes.as_str(), false),
        ("cli_fixture", registry.cli_fixture.as_str(), false),
    ] {
        validate_path(root, label, path, must_exist, errors);
    }
    if registry.recommended_plan_root.trim().is_empty() {
        errors.push("GOV-STATUS-0004: recommended_plan_root must be non-empty".to_owned());
    }
    if registry.feature_report == registry.release_notes
        || registry.feature_report == registry.cli_fixture
        || registry.release_notes == registry.cli_fixture
    {
        errors.push("GOV-STATUS-0004: generated artifact paths must be distinct".to_owned());
    }
    if !registry.feature_report.ends_with(".md") || !registry.release_notes.ends_with(".md") {
        errors
            .push("GOV-STATUS-0004: feature_report and release_notes must be Markdown".to_owned());
    }
    if !registry.cli_fixture.ends_with(".json") {
        errors.push("GOV-STATUS-0004: cli_fixture must be JSON".to_owned());
    }
    if !registry.cli_fixture_schema.starts_with("ling.governance.")
        || !registry.cli_fixture_schema.ends_with("/1")
    {
        errors.push(
            "GOV-STATUS-0005: CLI fixture schema must be an internal ling.governance.* /1 schema"
                .to_owned(),
        );
    }
    if registry.cli_implemented {
        errors.push(
            "GOV-STATUS-0005: cli_implemented must remain false until an accepted public CLI task exists"
                .to_owned(),
        );
    }
    if registry.future_cli_command != support_inputs.future_cli_command {
        errors.push(format!(
            "GOV-STATUS-0005: future_cli_command {:?} disagrees with support matrix {:?}",
            registry.future_cli_command, support_inputs.future_cli_command
        ));
    }
}

fn validate_tasks(root: &Path, tasks: &[Task], errors: &mut Vec<String>) {
    let mut by_id = BTreeMap::new();
    for task in tasks {
        if !valid_id(&task.id) {
            errors.push(format!(
                "GOV-STATUS-0006: task ID {:?} must use uppercase ASCII, digits, and hyphens",
                task.id
            ));
        }
        if by_id.insert(task.id.as_str(), task).is_some() {
            errors.push(format!("GOV-STATUS-0006: duplicate task ID {}", task.id));
        }
        if task.title.trim().is_empty() || task.release.trim().is_empty() {
            errors.push(format!(
                "GOV-STATUS-0006: task {} requires a title and release",
                task.id
            ));
        }
        if !TASK_STATES.contains(&task.state.as_str()) {
            errors.push(format!(
                "GOV-STATUS-0006: task {} has unknown state {:?}",
                task.id, task.state
            ));
        }
        if !TASK_SIZES.contains(&task.size.as_str()) {
            errors.push(format!(
                "GOV-STATUS-0006: task {} has unknown size {:?}",
                task.id, task.size
            ));
        }
        validate_unique_sorted(
            &task.depends_on,
            &format!("task {} depends_on", task.id),
            errors,
        );
        if task
            .depends_on
            .iter()
            .any(|dependency| dependency == &task.id)
        {
            errors.push(format!(
                "GOV-STATUS-0006: task {} cannot depend on itself",
                task.id
            ));
        }
        for path in &task.spec {
            validate_declared_evidence(root, &task.id, "spec", path, errors);
        }
        for path in &task.artifacts {
            validate_declared_evidence(root, &task.id, "artifact", path, errors);
        }
        if task.state == "Done" {
            if task.owner.trim().is_empty() || task.branch.trim().is_empty() {
                errors.push(format!(
                    "GOV-STATUS-0007: Done task {} requires owner and branch",
                    task.id
                ));
            }
            if !valid_commit(&task.verified_against_commit)
                || !valid_commit(&task.completion_commit)
            {
                errors.push(format!(
                    "GOV-STATUS-0007: Done task {} requires lowercase 40-hex verification and completion commits",
                    task.id
                ));
            }
            if task.integration_state != "committed" {
                errors.push(format!(
                    "GOV-STATUS-0007: Done task {} integration_state must be committed",
                    task.id
                ));
            }
            if !valid_date(&task.completed) {
                errors.push(format!(
                    "GOV-STATUS-0007: Done task {} requires a YYYY-MM-DD completed date",
                    task.id
                ));
            }
            if task.acceptance.is_empty() || task.artifacts.is_empty() || task.notes.is_empty() {
                errors.push(format!(
                    "GOV-STATUS-0007: Done task {} requires acceptance, artifacts, and notes",
                    task.id
                ));
            }
        }
    }
    for task in tasks {
        for dependency in &task.depends_on {
            let Some(dependency_task) = by_id.get(dependency.as_str()) else {
                errors.push(format!(
                    "GOV-STATUS-0006: task {} has unknown dependency {}",
                    task.id, dependency
                ));
                continue;
            };
            if task.state == "Done" && dependency_task.state != "Done" {
                errors.push(format!(
                    "GOV-STATUS-0007: Done task {} depends on non-Done task {}",
                    task.id, dependency
                ));
            }
        }
    }
    detect_task_cycles(&by_id, errors);
}

fn validate_features(
    root: &Path,
    features: &[FeatureStatus],
    trace_features: &FeatureRecords,
    support_inputs: &StatusInputs,
    gap_ids: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    let mut seen = BTreeSet::new();
    for feature in features {
        if !seen.insert(feature.id.as_str()) {
            errors.push(format!(
                "GOV-STATUS-0008: duplicate feature ID {}",
                feature.id
            ));
        }
        if !FEATURE_STATES.contains(&feature.current_state.as_str()) {
            errors.push(format!(
                "GOV-STATUS-0008: feature {} has unknown current_state {:?}",
                feature.id, feature.current_state
            ));
        }
        if !FEATURE_STABILITIES.contains(&feature.stability.as_str()) {
            errors.push(format!(
                "GOV-STATUS-0008: feature {} has unknown stability {:?}",
                feature.id, feature.stability
            ));
        }
        let Some(trace) = trace_features.get(&feature.id) else {
            errors.push(format!(
                "GOV-STATUS-0008: feature {} is absent from traceability",
                feature.id
            ));
            continue;
        };
        let Some(matrix) = support_inputs.features.get(&feature.id) else {
            errors.push(format!(
                "GOV-STATUS-0008: feature {} is absent from the support matrix",
                feature.id
            ));
            continue;
        };
        if trace.scope != "Public" {
            errors.push(format!(
                "GOV-STATUS-0008: feature {} is not a Public traceability feature",
                feature.id
            ));
        }
        if feature.stability != trace.stability || feature.stability != matrix.stability {
            errors.push(format!(
                "GOV-STATUS-0008: feature {} stability {:?} disagrees with traceability/support {:?}/{:?}",
                feature.id, feature.stability, trace.stability, matrix.stability
            ));
        }
        if feature.current_state != matrix.current_state {
            errors.push(format!(
                "GOV-STATUS-0008: feature {} current_state {:?} disagrees with support matrix {:?}",
                feature.id, feature.current_state, matrix.current_state
            ));
        }
        if feature.implemented != (feature.current_state == "Implemented") {
            errors.push(format!(
                "GOV-STATUS-0009: feature {} implemented flag disagrees with current_state",
                feature.id
            ));
        }
        if feature.implemented && (!feature.tested || !feature.documented) {
            errors.push(format!(
                "GOV-STATUS-0009: implemented feature {} must retain tested and documented traceability evidence",
                feature.id
            ));
        }
        if feature.last_verified_commit != trace.last_verified_commit {
            errors.push(format!(
                "GOV-STATUS-0009: feature {} last_verified_commit {:?} disagrees with release evidence {:?}",
                feature.id, feature.last_verified_commit, trace.last_verified_commit
            ));
        }
        validate_unique_sorted(
            &feature.blockers,
            &format!("feature {} blockers", feature.id),
            errors,
        );
        if feature.stability != "Stable" && feature.blockers.is_empty() {
            errors.push(format!(
                "GOV-STATUS-0009: non-Stable feature {} must name its stabilization blockers",
                feature.id
            ));
        }
        for blocker in &feature.blockers {
            if !gap_ids.contains(blocker) {
                errors.push(format!(
                    "GOV-STATUS-0009: feature {} has unknown blocker {}",
                    feature.id, blocker
                ));
            }
        }
        validate_unique_sorted(
            &feature.supported_profiles,
            &format!("feature {} supported_profiles", feature.id),
            errors,
        );
        if feature.supported_profiles != matrix.current_profiles {
            errors.push(format!(
                "GOV-STATUS-0010: feature {} supported_profiles {:?} disagree with support matrix {:?}",
                feature.id, feature.supported_profiles, matrix.current_profiles
            ));
        }
        validate_unique_sorted(
            &feature.supported_targets,
            &format!("feature {} supported_targets", feature.id),
            errors,
        );
        for target in &feature.supported_targets {
            if !support_inputs.supported_targets.contains(target) {
                errors.push(format!(
                    "GOV-STATUS-0010: feature {} claims unsupported target {}",
                    feature.id, target
                ));
            }
        }
        if feature.evidence.is_empty() {
            errors.push(format!(
                "GOV-STATUS-0009: feature {} requires evidence paths",
                feature.id
            ));
        }
        for path in &feature.evidence {
            validate_declared_evidence(root, &feature.id, "evidence", path, errors);
        }
    }

    for id in trace_features.keys() {
        if !seen.contains(id.as_str()) {
            errors.push(format!(
                "GOV-STATUS-0008: traceability feature {id} is missing from {MANIFEST_PATH}"
            ));
        }
    }
    for id in support_inputs.features.keys() {
        if !seen.contains(id.as_str()) {
            errors.push(format!(
                "GOV-STATUS-0008: support-matrix feature {id} is missing from {MANIFEST_PATH}"
            ));
        }
    }
}

fn render(registry: &Registry, trace_features: &FeatureRecords) -> String {
    let mut output = String::new();
    output.push_str("# Ling implementation status / Ling 实现状态\n\n");
    output.push_str(
        "> Generated deterministically from `implementation-status.toml`; do not edit manually.\n",
    );
    output.push_str("> 本文由 `implementation-status.toml` 确定性生成；不得手工编辑。\n\n");
    output.push_str(&format!(
        "- Registry schema: `{}`\n- Feature schema: `{}`\n- Updated: `{}`\n- Feature release: `{}`\n- Baseline release: `{}`\n- Baseline commit: `{}`\n\n",
        registry.schema_version,
        registry.feature_schema_version,
        registry.updated,
        registry.feature_release,
        registry.baseline_release,
        registry.baseline_commit
    ));
    output.push_str("## Feature state / 功能状态\n\n");
    output.push_str("| Feature | Title / 标题 | Current | Stability | I/T/D | Stabilization blockers | Profiles | Targets | Last verified |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for feature in sorted_by_id(&registry.features, |feature| &feature.id) {
        let Some(trace) = trace_features.get(&feature.id) else {
            continue;
        };
        output.push_str(&format!(
            "| `{}` | {} / {} | `{}` | `{}` | `{}/{}/{}` | {} | {} | {} | `{}` |\n",
            feature.id,
            trace.title_zh,
            trace.title_en,
            feature.current_state,
            feature.stability,
            yes_no(feature.implemented),
            yes_no(feature.tested),
            yes_no(feature.documented),
            code_list(&feature.blockers),
            code_list(&feature.supported_profiles),
            code_list(&feature.supported_targets),
            feature.last_verified_commit
        ));
    }
    output.push_str("\n`I/T/D` means implemented/tested/documented. Empty Profile and target cells are intentional: the current Seed interpreter is unprofiled, and no Ling Native target is supported.\n");
    output.push_str("`I/T/D` 表示已实现/已测试/已文档化。Profile 与 target 为空是有意的：当前 Seed 解释器未启用 Profile，且没有受支持的 Ling Native target。\n\n");
    output.push_str("## Task state / 任务状态\n\n");
    output
        .push_str("| Task | Title | Release | Size | State | Dependencies | Completion commit |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
    for task in sorted_by_id(&registry.tasks, |task| &task.id) {
        output.push_str(&format!(
            "| `{}` | {} | `{}` | `{}` | `{}` | {} | `{}` |\n",
            task.id,
            task.title,
            task.release,
            task.size,
            task.state,
            code_list(&task.depends_on),
            task.completion_commit
        ));
    }
    output.push_str("\n## Generated consumers / 生成视图\n\n");
    output.push_str(&format!(
        "- Release-note fragment: [`{}`]({})\n- Internal CLI fixture: `{}` (`{}`, `implemented: false`)\n- Proposed command: `{}` (not implemented)\n\n",
        registry.release_notes,
        relative_status_link(&registry.release_notes),
        registry.cli_fixture,
        registry.cli_fixture_schema,
        registry.future_cli_command
    ));
    output.push_str("```text\ncargo xtask status verify\ncargo xtask status render\ncargo xtask status render-release-notes\ncargo xtask status render-cli-fixture\n```\n");
    output
}

fn render_release_notes(registry: &Registry, trace_features: &FeatureRecords) -> String {
    let mut output = String::new();
    output.push_str(&format!(
        "# {} generated feature-state notes / {} 功能状态生成说明\n\n",
        registry.feature_release, registry.feature_release
    ));
    output.push_str("> Generated from `implementation-status.toml`. This is an auditable input fragment, not a published release announcement or a stability promise.\n");
    output.push_str("> 本文由 `implementation-status.toml` 生成，是可审计的发布说明输入片段，不是已发布公告或稳定性承诺。\n\n");
    output.push_str("## Current feature evidence / 当前功能证据\n\n");
    for feature in sorted_by_id(&registry.features, |feature| &feature.id) {
        let Some(trace) = trace_features.get(&feature.id) else {
            continue;
        };
        output.push_str(&format!(
            "- `{}` — {} / {}: `{}` + `{}`; implemented/tested/documented = `{}/{}/{}`; verified at `{}`.\n",
            feature.id,
            trace.title_zh,
            trace.title_en,
            feature.current_state,
            feature.stability,
            yes_no(feature.implemented),
            yes_no(feature.tested),
            yes_no(feature.documented),
            feature.last_verified_commit
        ));
    }
    output.push_str("\n## Stabilization blockers / 稳定化阻断项\n\n");
    for feature in sorted_by_id(&registry.features, |feature| &feature.id) {
        output.push_str(&format!(
            "- `{}`: {}\n",
            feature.id,
            code_list(&feature.blockers)
        ));
    }
    output.push_str("\n## Profile and target scope / Profile 与 target 范围\n\nNo feature currently claims a selectable Profile or supported Ling Native target. The support matrix remains the authority for host/backend tiers and explicitly unsupported capability groups.\n\n当前没有功能宣称可选择的 Profile 或受支持的 Ling Native target。主机/backend 分级及明确不支持的能力仍以支持矩阵为准。\n");
    output
}

fn render_cli_fixture(registry: &Registry, trace_features: &FeatureRecords) -> String {
    let features = sorted_by_id(&registry.features, |feature| &feature.id)
        .into_iter()
        .filter_map(|feature| {
            let trace = trace_features.get(&feature.id)?;
            Some(CliFeature {
                id: &feature.id,
                title: LocalizedTitle {
                    zh: &trace.title_zh,
                    en: &trace.title_en,
                },
                current_state: &feature.current_state,
                stability: &feature.stability,
                implemented: feature.implemented,
                tested: feature.tested,
                documented: feature.documented,
                blockers: &feature.blockers,
                last_verified_commit: &feature.last_verified_commit,
                supported_profiles: &feature.supported_profiles,
                supported_targets: &feature.supported_targets,
            })
        })
        .collect();
    json(&CliFixture {
        schema: &registry.cli_fixture_schema,
        proposed_command: &registry.future_cli_command,
        implemented: registry.cli_implemented,
        public_contract: false,
        generated_from: MANIFEST_PATH,
        release: &registry.feature_release,
        feature_schema_version: registry.feature_schema_version,
        features,
    })
}

fn detect_task_cycles(tasks: &BTreeMap<&str, &Task>, errors: &mut Vec<String>) {
    fn visit<'a>(
        id: &'a str,
        tasks: &BTreeMap<&'a str, &'a Task>,
        visiting: &mut BTreeSet<&'a str>,
        visited: &mut BTreeSet<&'a str>,
    ) -> bool {
        if visited.contains(id) {
            return false;
        }
        if !visiting.insert(id) {
            return true;
        }
        if let Some(task) = tasks.get(id) {
            for dependency in &task.depends_on {
                if tasks.contains_key(dependency.as_str())
                    && visit(dependency, tasks, visiting, visited)
                {
                    return true;
                }
            }
        }
        visiting.remove(id);
        visited.insert(id);
        false
    }

    let mut visited = BTreeSet::new();
    for id in tasks.keys().copied() {
        if visit(id, tasks, &mut BTreeSet::new(), &mut visited) {
            errors.push(format!(
                "GOV-STATUS-0006: task dependency cycle reaches {id}"
            ));
        }
    }
}

fn validate_declared_evidence(
    root: &Path,
    id: &str,
    label: &str,
    path: &str,
    errors: &mut Vec<String>,
) {
    if !safe_relative_path(path) {
        errors.push(format!(
            "GOV-STATUS-0011: {id} {label} path is unsafe: {path:?}"
        ));
        return;
    }
    if path.contains('*') {
        if !wildcard_path_exists(root, path) {
            errors.push(format!(
                "GOV-STATUS-0011: {id} {label} wildcard has no match: {path}"
            ));
        }
    } else if !root.join(path).exists() {
        errors.push(format!(
            "GOV-STATUS-0011: {id} {label} path does not exist: {path}"
        ));
    }
}

fn wildcard_path_exists(root: &Path, path: &str) -> bool {
    let parts = path.split('/').collect::<Vec<_>>();
    let stars = parts
        .iter()
        .enumerate()
        .filter(|(_, part)| **part == "*")
        .collect::<Vec<_>>();
    if stars.len() != 1 || parts.iter().any(|part| part.contains('*') && *part != "*") {
        return false;
    }
    let star = stars[0].0;
    let prefix = root.join(parts[..star].join("/"));
    let suffix = parts[star + 1..].join("/");
    fs::read_dir(prefix).is_ok_and(|entries| {
        entries
            .filter_map(Result::ok)
            .any(|entry| entry.path().join(&suffix).exists())
    })
}

fn validate_path(root: &Path, label: &str, path: &str, must_exist: bool, errors: &mut Vec<String>) {
    if !safe_relative_path(path) || path.contains('*') {
        errors.push(format!(
            "GOV-STATUS-0011: {label} must be a safe repository-relative path, found {path:?}"
        ));
    } else if must_exist && !root.join(path).exists() {
        errors.push(format!(
            "GOV-STATUS-0011: {label} path does not exist: {path}"
        ));
    }
}

fn safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains('\\')
        && !Path::new(path).is_absolute()
        && Path::new(path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn validate_unique_sorted(values: &[String], label: &str, errors: &mut Vec<String>) {
    let mut sorted = values.to_vec();
    sorted.sort();
    let mut unique = sorted.clone();
    unique.dedup();
    if unique.len() != values.len() {
        errors.push(format!("GOV-STATUS-0012: {label} contains duplicates"));
    }
    if sorted != values {
        errors.push(format!(
            "GOV-STATUS-0012: {label} must be sorted deterministically"
        ));
    }
}

fn compare_generated(root: &Path, path: &str, expected: &str, errors: &mut Vec<String>) {
    match fs::read_to_string(root.join(path)) {
        Ok(actual) if normalize_newlines(&actual) == expected => {}
        Ok(_) => errors.push(format!(
            "GOV-STATUS-0013: {path} is not the deterministic rendering of {MANIFEST_PATH}"
        )),
        Err(error) => errors.push(format!(
            "GOV-STATUS-0002: cannot read generated artifact {path}: {error}"
        )),
    }
}

fn summary(
    registry: &Registry,
    _gap_ids: &BTreeSet<String>,
    _support_inputs: &StatusInputs,
) -> CheckSummary {
    CheckSummary {
        task_count: registry.tasks.len(),
        done_task_count: registry
            .tasks
            .iter()
            .filter(|task| task.state == "Done")
            .count(),
        feature_count: registry.features.len(),
        blocked_feature_count: registry
            .features
            .iter()
            .filter(|feature| !feature.blockers.is_empty())
            .count(),
    }
}

fn sorted_by_id<T, F>(values: &[T], id: F) -> Vec<&T>
where
    F: Fn(&T) -> &str,
{
    let mut values = values.iter().collect::<Vec<_>>();
    values.sort_by(|left, right| id(left).cmp(id(right)));
    values
}

fn code_list(values: &[String]) -> String {
    if values.is_empty() {
        "—".to_owned()
    } else {
        values
            .iter()
            .map(|value| format!("`{value}`"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn relative_status_link(path: &str) -> String {
    path.strip_prefix("docs/status/").unwrap_or(path).to_owned()
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-')
}

fn valid_commit(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_date(value: &str) -> bool {
    value.len() == 10
        && value.as_bytes()[4] == b'-'
        && value.as_bytes()[7] == b'-'
        && value
            .bytes()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit())
}

fn json(value: &impl Serialize) -> String {
    let mut output =
        serde_json::to_string_pretty(value).expect("status registry strings are JSON-serializable");
    output.push('\n');
    output
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

    fn repository_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repository root")
    }

    fn inputs(root: &Path) -> (Registry, FeatureRecords, StatusInputs, BTreeSet<String>) {
        let registry = load_registry(root).expect("status registry parses");
        let trace = traceability::feature_records(root, &registry.feature_release)
            .expect("traceability is valid");
        let support = support::status_inputs(root).expect("support matrix is valid");
        let gaps = gaps::registered_gap_ids(root).expect("gap register is valid");
        (registry, trace, support, gaps)
    }

    #[test]
    fn rejects_done_task_without_commit_evidence() {
        let root = repository_root();
        let (mut registry, trace, support, gaps) = inputs(&root);
        registry.tasks[0].completion_commit = "short".to_owned();
        let errors = validate(&root, &registry, &trace, &support, &gaps);
        assert!(
            errors
                .iter()
                .any(|error| { error.contains("Done task BASE-0001 requires lowercase 40-hex") })
        );
    }

    #[test]
    fn rejects_unknown_blocker_and_target_overclaim() {
        let root = repository_root();
        let (mut registry, trace, support, gaps) = inputs(&root);
        registry.features[0].blockers = vec!["GAP-NOT-REGISTERED-001".to_owned()];
        registry.features[0].supported_targets = vec!["TARGET-NOT-REAL".to_owned()];
        let errors = validate(&root, &registry, &trace, &support, &gaps);
        assert!(errors.iter().any(|error| error.contains("unknown blocker")));
        assert!(
            errors
                .iter()
                .any(|error| error.contains("claims unsupported target"))
        );
    }

    #[test]
    fn rejects_feature_omission_and_state_drift() {
        let root = repository_root();
        let (mut registry, trace, support, gaps) = inputs(&root);
        registry.features.remove(0);
        registry.features[0].current_state = "Partial".to_owned();
        let errors = validate(&root, &registry, &trace, &support, &gaps);
        assert!(errors.iter().any(|error| error.contains("is missing from")));
        assert!(
            errors
                .iter()
                .any(|error| error.contains("disagrees with support matrix"))
        );
    }

    #[test]
    fn feature_state_and_stability_vocabularies_are_closed_and_distinct() {
        assert_eq!(FEATURE_STATES, ["Unavailable", "Partial", "Implemented"]);
        assert_eq!(
            FEATURE_STABILITIES,
            ["Experimental", "Preview", "Stable", "Deprecated", "Removed"]
        );

        let root = repository_root();
        let (mut registry, trace, support, gaps) = inputs(&root);
        registry.features[0].current_state = "Stable".to_owned();
        registry.features[1].stability = "Implemented".to_owned();
        let errors = validate(&root, &registry, &trace, &support, &gaps);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("unknown current_state \"Stable\""))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("unknown stability \"Implemented\""))
        );
    }

    #[test]
    fn cli_fixture_is_internal_and_unimplemented() {
        let root = repository_root();
        let (registry, trace, _, _) = inputs(&root);
        let fixture = render_cli_fixture(&registry, &trace);
        assert!(fixture.contains("\"schema\": \"ling.governance."));
        assert!(fixture.contains("\"implemented\": false"));
        assert!(fixture.contains("\"public_contract\": false"));
    }

    #[test]
    fn rendering_is_deterministic() {
        let root = repository_root();
        let (registry, trace, _, _) = inputs(&root);
        assert_eq!(render(&registry, &trace), render(&registry, &trace));
        assert_eq!(
            render_release_notes(&registry, &trace),
            render_release_notes(&registry, &trace)
        );
        assert_eq!(
            render_cli_fixture(&registry, &trace),
            render_cli_fixture(&registry, &trace)
        );
    }

    #[test]
    fn repository_status_is_valid_and_current() {
        let root = repository_root();
        let summary = check_repository(&root).expect("repository status is valid");
        assert_eq!(summary.feature_count, 7);
        assert_eq!(summary.task_count, 483);
        assert_eq!(summary.done_task_count, 279);
    }
}
