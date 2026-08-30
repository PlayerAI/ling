use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::governance;

const MANIFEST_PATH: &str = "docs/governance/lifecycle.toml";
const REPORT_PATH: &str = "docs/governance/lifecycle.md";
const RFC_TEMPLATE_PATH: &str = "docs/governance/templates/RFC.md";
const DECISION_TEMPLATE_PATH: &str = "docs/governance/templates/DECISION.md";
const PR_TEMPLATE_PATH: &str = ".github/pull_request_template.md";
const STATES: &[&str] = &[
    "Open",
    "Draft",
    "Proposed",
    "Accepted",
    "Rejected",
    "Superseded",
];
const LEGACY_IDS: &[&str] = &[
    "RFC-0001", "DEC-0001", "DEC-0002", "DEC-0003", "DEC-0004", "DEC-0005", "DEC-0006", "DEC-0007",
    "DEC-0008", "DEC-0009", "DEC-0010", "DEC-0011", "DEC-0012", "DEC-0013", "DEC-0014", "DEC-0015",
    "DEC-0016",
];
const RFC_HEADINGS: &[&str] = &[
    "## Summary",
    "## Status and scope",
    "## Normative changes",
    "## Conformance plan",
    "## Compatibility impact",
    "## Unresolved alternatives",
    "## Supersession",
];
const DECISION_HEADINGS: &[&str] = &[
    "## Question",
    "## Decision",
    "## Conformance plan",
    "## Compatibility impact",
    "## Unresolved alternatives",
    "## Supersession",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub record_count: usize,
    pub accepted_count: usize,
    pub legacy_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LifecycleRegistry {
    schema_version: u32,
    updated: String,
    #[serde(default)]
    record: Vec<Record>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    id: String,
    kind: String,
    status: String,
    path: String,
    #[serde(default)]
    history: Vec<String>,
    opened: String,
    proposed: String,
    decided: String,
    #[serde(default)]
    stable_basis: bool,
    #[serde(default)]
    legacy_format: bool,
    #[serde(default)]
    conformance_plan: Vec<String>,
    #[serde(default)]
    compatibility_impact: Vec<String>,
    #[serde(default)]
    unresolved_alternatives: Vec<String>,
    #[serde(default)]
    superseded_by: String,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let specifications = governance::specification_records(root)?;
    let registry = load_registry(root)?;
    let mut errors = validate(root, &registry, &specifications);
    let rendered = render(&registry);
    match fs::read_to_string(root.join(REPORT_PATH)) {
        Ok(actual) if normalize_newlines(&actual) == rendered => {}
        Ok(_) => errors.push(format!(
            "GOV-LIFE-0008: {} is not the deterministic rendering of {}",
            REPORT_PATH, MANIFEST_PATH
        )),
        Err(error) => errors.push(format!(
            "GOV-LIFE-0002: cannot read {}: {error}",
            REPORT_PATH
        )),
    }

    finish(errors).map(|()| CheckSummary {
        record_count: registry.record.len(),
        accepted_count: registry
            .record
            .iter()
            .filter(|record| record.status == "Accepted")
            .count(),
        legacy_count: registry
            .record
            .iter()
            .filter(|record| record.legacy_format)
            .count(),
    })
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    let specifications = governance::specification_records(root)?;
    let registry = load_registry(root)?;
    finish(validate(root, &registry, &specifications)).map(|()| render(&registry))
}

fn load_registry(root: &Path) -> Result<LifecycleRegistry, Vec<String>> {
    let text = fs::read_to_string(root.join(MANIFEST_PATH)).map_err(|error| {
        vec![format!(
            "GOV-LIFE-0002: cannot read {}: {error}",
            MANIFEST_PATH
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-LIFE-0009: invalid lifecycle registry {}: {error}",
            MANIFEST_PATH
        )]
    })
}

fn validate(
    root: &Path,
    registry: &LifecycleRegistry,
    specifications: &governance::SpecificationRecords,
) -> Vec<String> {
    let mut errors = Vec::new();
    if registry.schema_version != 1 {
        errors.push(format!(
            "GOV-LIFE-0010: unsupported schema_version {}; expected 1",
            registry.schema_version
        ));
    }
    if registry.updated.trim().is_empty() {
        errors.push("GOV-LIFE-0010: updated must not be empty".to_owned());
    }
    if registry.record.is_empty() {
        errors.push("GOV-LIFE-0010: lifecycle registry contains no records".to_owned());
    }

    let mut records = BTreeMap::new();
    for record in &registry.record {
        validate_record(root, record, specifications, &mut errors);
        if records.insert(record.id.clone(), record).is_some() {
            errors.push(format!(
                "GOV-LIFE-0001: duplicate lifecycle id {}",
                record.id
            ));
        }
    }

    for (id, (_, _, path)) in specifications {
        if !records.contains_key(id) {
            errors.push(format!(
                "GOV-LIFE-0004: specification {id} at {path} is absent from {}",
                MANIFEST_PATH
            ));
        }
    }
    for id in records.keys() {
        if !specifications.contains_key(id) {
            errors.push(format!(
                "GOV-LIFE-0004: lifecycle record {id} has no authority-index specification"
            ));
        }
    }

    validate_supersession(&records, &mut errors);
    validate_supporting_files(root, &mut errors);
    errors
}

fn validate_record(
    root: &Path,
    record: &Record,
    specifications: &governance::SpecificationRecords,
    errors: &mut Vec<String>,
) {
    if record.id.trim().is_empty()
        || record.path.trim().is_empty()
        || record.opened.trim().is_empty()
    {
        errors.push(format!(
            "GOV-LIFE-0010: {} has an empty required field",
            display_id(record)
        ));
    }
    if !matches!(record.kind.as_str(), "RFC" | "Decision") {
        errors.push(format!(
            "GOV-LIFE-0010: {} has unknown kind {}",
            display_id(record),
            record.kind
        ));
    }
    if !STATES.contains(&record.status.as_str()) {
        errors.push(format!(
            "GOV-LIFE-0003: {} has unknown state {}",
            display_id(record),
            record.status
        ));
    }
    if !is_date(&record.opened)
        || (!record.proposed.is_empty() && !is_date(&record.proposed))
        || (!record.decided.is_empty() && !is_date(&record.decided))
    {
        errors.push(format!(
            "GOV-LIFE-0010: {} has a non-YYYY-MM-DD lifecycle date",
            display_id(record)
        ));
    }
    if !is_relative_path(&record.path) || !root.join(&record.path).is_file() {
        errors.push(format!(
            "GOV-LIFE-0002: {} references invalid or missing path {}",
            display_id(record),
            record.path
        ));
    }

    if let Some((kind, status, path)) = specifications.get(&record.id) {
        if kind != &record.kind || status != &record.status || path != &record.path {
            errors.push(format!(
                "GOV-LIFE-0004: {} lifecycle tuple ({}, {}, {}) differs from authority tuple ({kind}, {status}, {path})",
                display_id(record),
                record.kind,
                record.status,
                record.path
            ));
        }
    }

    validate_history(record, errors);
    validate_state_metadata(record, errors);

    if record.legacy_format {
        if !LEGACY_IDS.contains(&record.id.as_str()) {
            errors.push(format!(
                "GOV-LIFE-0006: {} is not in the closed legacy-format allowlist",
                display_id(record)
            ));
        }
    } else if let Ok(text) = fs::read_to_string(root.join(&record.path)) {
        validate_headings(&record.path, &text, headings_for(&record.kind), errors);
    }
}

fn validate_history(record: &Record, errors: &mut Vec<String>) {
    if record.history.is_empty() {
        errors.push(format!(
            "GOV-LIFE-0005: {} has no lifecycle history",
            display_id(record)
        ));
        return;
    }
    for state in &record.history {
        if !STATES.contains(&state.as_str()) {
            errors.push(format!(
                "GOV-LIFE-0003: {} history contains unknown state {}",
                display_id(record),
                state
            ));
        }
    }
    if record.history.last() != Some(&record.status) {
        errors.push(format!(
            "GOV-LIFE-0005: {} history does not end in current state {}",
            display_id(record),
            record.status
        ));
    }
    if record.legacy_format {
        return;
    }
    if record.history.first().map(String::as_str) != Some("Open") {
        errors.push(format!(
            "GOV-LIFE-0005: {} non-legacy history must start at Open",
            display_id(record)
        ));
    }
    for states in record.history.windows(2) {
        if !transition_is_allowed(&states[0], &states[1]) {
            errors.push(format!(
                "GOV-LIFE-0005: {} has invalid transition {} -> {}",
                display_id(record),
                states[0],
                states[1]
            ));
        }
    }
}

fn validate_state_metadata(record: &Record, errors: &mut Vec<String>) {
    match record.status.as_str() {
        "Accepted" => {
            if record.decided.is_empty()
                || !record.stable_basis
                || record.conformance_plan.is_empty()
                || record.compatibility_impact.is_empty()
                || record.unresolved_alternatives.is_empty()
                || !record.superseded_by.is_empty()
            {
                errors.push(format!(
                    "GOV-LIFE-0007: {} Accepted metadata requires a decision date, Stable basis, conformance plan, compatibility impact, unresolved alternatives, and no successor",
                    display_id(record)
                ));
            }
        }
        "Rejected" => {
            if record.decided.is_empty()
                || record.stable_basis
                || record.compatibility_impact.is_empty()
                || record.unresolved_alternatives.is_empty()
                || !record.superseded_by.is_empty()
            {
                errors.push(format!(
                    "GOV-LIFE-0007: {} Rejected metadata is incomplete or claims a Stable basis/successor",
                    display_id(record)
                ));
            }
        }
        "Superseded" => {
            if record.superseded_by.is_empty() || record.stable_basis {
                errors.push(format!(
                    "GOV-LIFE-0007: {} Superseded metadata requires a successor and cannot be a Stable basis",
                    display_id(record)
                ));
            }
        }
        "Open" | "Draft" | "Proposed"
            if record.stable_basis
                || !record.decided.is_empty()
                || !record.superseded_by.is_empty() =>
        {
            errors.push(format!(
                "GOV-LIFE-0007: {} state {} cannot be Stable, decided, or superseded",
                display_id(record),
                record.status
            ));
        }
        _ => {}
    }
    validate_unique_values(record, "conformance plan", &record.conformance_plan, errors);
    validate_unique_values(
        record,
        "compatibility impact",
        &record.compatibility_impact,
        errors,
    );
    validate_unique_values(
        record,
        "unresolved alternative",
        &record.unresolved_alternatives,
        errors,
    );
}

fn validate_supersession(records: &BTreeMap<String, &Record>, errors: &mut Vec<String>) {
    for record in records.values() {
        if record.superseded_by.is_empty() {
            continue;
        }
        match records.get(&record.superseded_by) {
            Some(successor) if successor.status == "Accepted" => {}
            Some(successor) => errors.push(format!(
                "GOV-LIFE-0004: {} successor {} is {}, not Accepted",
                record.id, successor.id, successor.status
            )),
            None => errors.push(format!(
                "GOV-LIFE-0004: {} references unknown successor {}",
                record.id, record.superseded_by
            )),
        }
    }

    let mut visited = BTreeSet::new();
    let mut visiting = Vec::new();
    for id in records.keys() {
        visit_successor(id, records, &mut visited, &mut visiting, errors);
    }
}

fn visit_successor(
    id: &str,
    records: &BTreeMap<String, &Record>,
    visited: &mut BTreeSet<String>,
    visiting: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    if visited.contains(id) {
        return;
    }
    if let Some(position) = visiting.iter().position(|candidate| candidate == id) {
        let mut cycle = visiting[position..].to_vec();
        cycle.push(id.to_owned());
        errors.push(format!(
            "GOV-LIFE-0011: lifecycle supersession cycle: {}",
            cycle.join(" -> ")
        ));
        return;
    }
    let Some(record) = records.get(id) else {
        return;
    };
    visiting.push(id.to_owned());
    if !record.superseded_by.is_empty() && records.contains_key(&record.superseded_by) {
        visit_successor(&record.superseded_by, records, visited, visiting, errors);
    }
    visiting.pop();
    visited.insert(id.to_owned());
}

fn validate_supporting_files(root: &Path, errors: &mut Vec<String>) {
    validate_template(
        root,
        RFC_TEMPLATE_PATH,
        "RFC-NNNN",
        "> 状态：Draft",
        RFC_HEADINGS,
        errors,
    );
    validate_template(
        root,
        DECISION_TEMPLATE_PATH,
        "DEC-NNNN",
        "> 状态：Proposed",
        DECISION_HEADINGS,
        errors,
    );
    let required_pr_fields = [
        "Accepted specification IDs",
        "Specification gaps",
        "Normative clauses",
        "Compatibility impact",
        "Determinism and Unicode impact",
        "Intentionally deferred work",
    ];
    match fs::read_to_string(root.join(PR_TEMPLATE_PATH)) {
        Ok(text) => {
            for field in required_pr_fields {
                if !text.contains(field) {
                    errors.push(format!(
                        "GOV-LIFE-0012: {} lacks required field {}",
                        PR_TEMPLATE_PATH, field
                    ));
                }
            }
        }
        Err(error) => errors.push(format!(
            "GOV-LIFE-0002: cannot read {}: {error}",
            PR_TEMPLATE_PATH
        )),
    }
}

fn validate_template(
    root: &Path,
    path: &str,
    id_marker: &str,
    state_marker: &str,
    headings: &[&str],
    errors: &mut Vec<String>,
) {
    match fs::read_to_string(root.join(path)) {
        Ok(text) => {
            if !text.contains(id_marker) || !text.contains(state_marker) {
                errors.push(format!(
                    "GOV-LIFE-0012: {path} lacks ID or initial-state metadata"
                ));
            }
            validate_headings(path, &text, headings, errors);
        }
        Err(error) => errors.push(format!("GOV-LIFE-0002: cannot read {path}: {error}")),
    }
}

fn validate_headings(path: &str, text: &str, headings: &[&str], errors: &mut Vec<String>) {
    let actual = text.lines().map(str::trim).collect::<BTreeSet<_>>();
    for heading in headings {
        if !actual.contains(heading) {
            errors.push(format!(
                "GOV-LIFE-0012: {path} lacks required heading {heading}"
            ));
        }
    }
}

fn transition_is_allowed(from: &str, to: &str) -> bool {
    from == to
        || matches!(
            (from, to),
            ("Open", "Draft")
                | ("Draft", "Proposed")
                | ("Proposed", "Accepted" | "Rejected")
                | ("Accepted" | "Rejected", "Superseded")
        )
}

fn render(registry: &LifecycleRegistry) -> String {
    let mut records = registry.record.iter().collect::<Vec<_>>();
    records.sort_by(|left, right| {
        kind_rank(&left.kind)
            .cmp(&kind_rank(&right.kind))
            .then_with(|| left.id.cmp(&right.id))
    });

    let mut output = String::new();
    output.push_str("# RFC 与 Decision 生命周期 / Lifecycle Registry\n\n");
    output.push_str("> 状态：由 `lifecycle.toml` 确定性生成\n");
    output.push_str(&format!("> 更新日期：{}\n", registry.updated));
    output.push_str("> 本文件定义治理状态和证据要求，不新增语言语义。\n\n");
    output.push_str("## State machine\n\n");
    output.push_str("```text\n");
    output.push_str("Open → Draft → Proposed → Accepted / Rejected → Superseded\n");
    output.push_str("```\n\n");
    output.push_str("Draft and Proposed documents cannot authorize Stable implementation. Accepted records require conformance, compatibility, and unresolved-alternative metadata. Superseded records name an Accepted successor.\n\n");
    output.push_str("## Records\n\n");
    output.push_str(
        "| ID | Kind | Status | History | Stable basis | Legacy format | Decided | Path |\n",
    );
    output.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for record in records {
        output.push_str(&format!(
            "| `{}` | {} | `{}` | {} | {} | {} | {} | [{}]({}) |\n",
            escape_cell(&record.id),
            escape_cell(&record.kind),
            escape_cell(&record.status),
            list_cell(&record.history),
            yes_no(record.stable_basis),
            yes_no(record.legacy_format),
            if record.decided.is_empty() {
                "—".to_owned()
            } else {
                format!("`{}`", escape_cell(&record.decided))
            },
            escape_cell(&record.id),
            report_link(&record.path)
        ));
    }
    output.push_str("\n## Migration boundary\n\n");
    output.push_str("RFC-0001 and DEC-0001 through DEC-0016 predate the section template and are listed in a closed legacy-format allowlist. Their required Accepted metadata is carried by `lifecycle.toml`. Every later RFC/decision must use the checked template headings; new legacy exemptions are rejected.\n\n");
    output.push_str("## Merge policy\n\n");
    output.push_str(
        "- Experimental implementation must map to a Draft RFC or registered specification gap.\n",
    );
    output.push_str("- A language-semantic pull request must cite the Accepted specification IDs and normative clauses that authorize it. A Draft, Proposed record, roadmap item, snapshot, or gap is not authorization.\n");
    output.push_str("- Supersession preserves the historical record and points to an Accepted replacement; IDs and published meanings are not silently reused.\n\n");
    output.push_str("## Machine source and templates\n\n");
    output.push_str("- [`lifecycle.toml`](lifecycle.toml)\n");
    output.push_str("- [`templates/RFC.md`](templates/RFC.md)\n");
    output.push_str("- [`templates/DECISION.md`](templates/DECISION.md)\n");
    output.push_str("- [Pull request template](../../.github/pull_request_template.md)\n\n");
    output.push_str("Run `cargo xtask governance check-lifecycle` to reject invalid states/transitions, Draft Stable bases, incomplete Accepted metadata, dangling/cyclic supersession, unindexed specifications, template drift, and report drift.\n");
    output
}

fn headings_for(kind: &str) -> &'static [&'static str] {
    if kind == "RFC" {
        RFC_HEADINGS
    } else {
        DECISION_HEADINGS
    }
}

fn report_link(path: &str) -> String {
    match path.strip_prefix("docs/") {
        Some(relative) => format!("../{relative}"),
        None => format!("../../{path}"),
    }
}

fn kind_rank(kind: &str) -> u8 {
    match kind {
        "RFC" => 1,
        "Decision" => 2,
        _ => u8::MAX,
    }
}

fn validate_unique_values(
    record: &Record,
    field: &str,
    values: &[String],
    errors: &mut Vec<String>,
) {
    let mut seen = BTreeSet::new();
    for value in values {
        if value.trim().is_empty() {
            errors.push(format!(
                "GOV-LIFE-0010: {} has an empty {field}",
                display_id(record)
            ));
        } else if !seen.insert(value) {
            errors.push(format!(
                "GOV-LIFE-0010: {} repeats {field} value {value}",
                display_id(record)
            ));
        }
    }
}

fn display_id(record: &Record) -> &str {
    if record.id.is_empty() {
        "<missing-id>"
    } else {
        &record.id
    }
}

fn is_date(value: &str) -> bool {
    value.len() == 10
        && value.as_bytes()[4] == b'-'
        && value.as_bytes()[7] == b'-'
        && value
            .chars()
            .enumerate()
            .all(|(index, character)| matches!(index, 4 | 7) || character.is_ascii_digit())
}

fn is_relative_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains('\\')
        && value.as_bytes().get(1) != Some(&b':')
        && value
            .split('/')
            .all(|segment| !segment.is_empty() && !matches!(segment, "." | ".."))
}

fn list_cell(values: &[String]) -> String {
    if values.is_empty() {
        "—".to_owned()
    } else {
        values
            .iter()
            .map(|value| format!("`{}`", escape_cell(value)))
            .collect::<Vec<_>>()
            .join(" → ")
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn escape_cell(value: &str) -> String {
    value.replace(['\r', '\n'], " ").replace('|', "\\|")
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
    use super::{LifecycleRegistry, render, transition_is_allowed, validate};
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

    struct TempRepository(PathBuf);

    impl TempRepository {
        fn new() -> Self {
            let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("ling-lifecycle-{}-{sequence}", std::process::id()));
            fs::create_dir_all(path.join("docs/governance/templates"))
                .expect("create governance templates directory");
            fs::create_dir_all(path.join("docs/decisions")).expect("create decisions directory");
            fs::create_dir_all(path.join(".github")).expect("create GitHub directory");
            let repository = Self(path);
            repository.write(
                "docs/governance/templates/RFC.md",
                "# RFC-NNNN\n> 状态：Draft\n## Summary\n## Status and scope\n## Normative changes\n## Conformance plan\n## Compatibility impact\n## Unresolved alternatives\n## Supersession\n",
            );
            repository.write(
                "docs/governance/templates/DECISION.md",
                "# DEC-NNNN\n> 状态：Proposed\n## Question\n## Decision\n## Conformance plan\n## Compatibility impact\n## Unresolved alternatives\n## Supersession\n",
            );
            repository.write(
                ".github/pull_request_template.md",
                "Accepted specification IDs\nSpecification gaps\nNormative clauses\nCompatibility impact\nDeterminism and Unicode impact\nIntentionally deferred work\n",
            );
            repository
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn write(&self, relative: &str, text: &str) {
            let path = self.0.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("create fixture parent");
            }
            fs::write(path, text).expect("write fixture");
        }
    }

    impl Drop for TempRepository {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn accepted_record() -> String {
        r#"
[[record]]
id = "DEC-0001"
kind = "Decision"
status = "Accepted"
path = "docs/decisions/0001.md"
history = ["Accepted"]
opened = "2026-08-20"
proposed = ""
decided = "2026-08-20"
stable_basis = true
legacy_format = true
conformance_plan = ["positive and negative fixtures"]
compatibility_impact = ["published behavior is stable"]
unresolved_alternatives = ["future scope remains unresolved"]
superseded_by = ""
"#
        .to_owned()
    }

    fn superseded_record(successor: &str) -> String {
        accepted_record()
            .replace("status = \"Accepted\"", "status = \"Superseded\"")
            .replace("history = [\"Accepted\"]", "history = [\"Superseded\"]")
            .replace("stable_basis = true", "stable_basis = false")
            .replace(
                "superseded_by = \"\"",
                &format!("superseded_by = \"{successor}\""),
            )
    }

    fn parse(body: &str) -> LifecycleRegistry {
        toml::from_str(&format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{body}"
        ))
        .expect("valid fixture lifecycle registry")
    }

    fn specifications() -> BTreeMap<String, (String, String, String)> {
        [(
            "DEC-0001".to_owned(),
            (
                "Decision".to_owned(),
                "Accepted".to_owned(),
                "docs/decisions/0001.md".to_owned(),
            ),
        )]
        .into_iter()
        .collect()
    }

    #[test]
    fn transition_matrix_accepts_only_declared_edges_and_same_state() {
        assert!(transition_is_allowed("Open", "Draft"));
        assert!(transition_is_allowed("Draft", "Proposed"));
        assert!(transition_is_allowed("Proposed", "Accepted"));
        assert!(transition_is_allowed("Proposed", "Rejected"));
        assert!(transition_is_allowed("Accepted", "Superseded"));
        assert!(transition_is_allowed("Rejected", "Superseded"));
        assert!(transition_is_allowed("Draft", "Draft"));
        assert!(!transition_is_allowed("Draft", "Accepted"));
        assert!(!transition_is_allowed("Accepted", "Draft"));
    }

    #[test]
    fn accepts_imported_legacy_accepted_record_with_sidecar_evidence() {
        let repository = TempRepository::new();
        repository.write("docs/decisions/0001.md", "legacy accepted decision\n");
        assert!(
            validate(
                repository.path(),
                &parse(&accepted_record()),
                &specifications()
            )
            .is_empty()
        );
    }

    #[test]
    fn accepts_supersession_by_an_accepted_replacement() {
        let repository = TempRepository::new();
        repository.write("docs/decisions/0001.md", "legacy superseded decision\n");
        repository.write("docs/decisions/0002.md", "legacy accepted replacement\n");
        let replacement = accepted_record()
            .replace("DEC-0001", "DEC-0002")
            .replace("0001.md", "0002.md");
        let registry = parse(&format!("{}{}", superseded_record("DEC-0002"), replacement));
        let specs = [
            (
                "DEC-0001".to_owned(),
                (
                    "Decision".to_owned(),
                    "Superseded".to_owned(),
                    "docs/decisions/0001.md".to_owned(),
                ),
            ),
            (
                "DEC-0002".to_owned(),
                (
                    "Decision".to_owned(),
                    "Accepted".to_owned(),
                    "docs/decisions/0002.md".to_owned(),
                ),
            ),
        ]
        .into_iter()
        .collect();
        assert!(validate(repository.path(), &registry, &specs).is_empty());
    }

    #[test]
    fn rejects_unknown_supersession_target() {
        let repository = TempRepository::new();
        repository.write("docs/decisions/0001.md", "legacy superseded decision\n");
        let specs = [(
            "DEC-0001".to_owned(),
            (
                "Decision".to_owned(),
                "Superseded".to_owned(),
                "docs/decisions/0001.md".to_owned(),
            ),
        )]
        .into_iter()
        .collect();
        let errors = validate(
            repository.path(),
            &parse(&superseded_record("DEC-9999")),
            &specs,
        );
        assert!(errors.iter().any(|error| {
            error.contains("GOV-LIFE-0004") && error.contains("unknown successor DEC-9999")
        }));
    }

    #[test]
    fn rejects_draft_as_stable_basis() {
        let repository = TempRepository::new();
        repository.write("docs/decisions/0001.md", "legacy decision\n");
        let body = accepted_record()
            .replace("status = \"Accepted\"", "status = \"Draft\"")
            .replace("history = [\"Accepted\"]", "history = [\"Draft\"]")
            .replace("decided = \"2026-08-20\"", "decided = \"\"");
        let specs = [(
            "DEC-0001".to_owned(),
            (
                "Decision".to_owned(),
                "Draft".to_owned(),
                "docs/decisions/0001.md".to_owned(),
            ),
        )]
        .into_iter()
        .collect();
        let errors = validate(repository.path(), &parse(&body), &specs);
        assert!(errors.iter().any(|error| error.contains("GOV-LIFE-0007")));
    }

    #[test]
    fn rejects_accepted_record_without_required_evidence() {
        let repository = TempRepository::new();
        repository.write("docs/decisions/0001.md", "legacy accepted decision\n");
        let body = accepted_record().replace(
            "conformance_plan = [\"positive and negative fixtures\"]",
            "conformance_plan = []",
        );
        let errors = validate(repository.path(), &parse(&body), &specifications());
        assert!(errors.iter().any(|error| error.contains("GOV-LIFE-0007")));
    }

    #[test]
    fn rejects_new_legacy_exemption() {
        let repository = TempRepository::new();
        repository.write("docs/decisions/0017.md", "legacy accepted decision\n");
        let body = accepted_record()
            .replace("DEC-0001", "DEC-0017")
            .replace("0001.md", "0017.md");
        let specs = [(
            "DEC-0017".to_owned(),
            (
                "Decision".to_owned(),
                "Accepted".to_owned(),
                "docs/decisions/0017.md".to_owned(),
            ),
        )]
        .into_iter()
        .collect();
        let errors = validate(repository.path(), &parse(&body), &specs);
        assert!(errors.iter().any(|error| error.contains("GOV-LIFE-0006")));
    }

    #[test]
    fn rejects_nonlegacy_document_without_required_headings() {
        let repository = TempRepository::new();
        repository.write("docs/decisions/0017.md", "# DEC-0017\n## Decision\n");
        let body = accepted_record()
            .replace("DEC-0001", "DEC-0017")
            .replace("0001.md", "0017.md")
            .replace("legacy_format = true", "legacy_format = false")
            .replace(
                "history = [\"Accepted\"]",
                "history = [\"Open\", \"Draft\", \"Proposed\", \"Accepted\"]",
            );
        let specs = [(
            "DEC-0017".to_owned(),
            (
                "Decision".to_owned(),
                "Accepted".to_owned(),
                "docs/decisions/0017.md".to_owned(),
            ),
        )]
        .into_iter()
        .collect();
        let errors = validate(repository.path(), &parse(&body), &specs);
        assert!(errors.iter().any(|error| error.contains("GOV-LIFE-0012")));
    }

    #[test]
    fn rendering_is_deterministic() {
        let registry = parse(&accepted_record());
        assert_eq!(render(&registry), render(&registry));
    }

    #[test]
    fn repository_lifecycle_covers_all_current_specifications() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let text = fs::read_to_string(root.join("docs/governance/lifecycle.toml"))
            .expect("read repository lifecycle registry");
        let registry: LifecycleRegistry = toml::from_str(&text).expect("parse lifecycle registry");
        assert_eq!(registry.record.len(), 310);
        assert_eq!(
            registry
                .record
                .iter()
                .filter(|record| record.status == "Accepted")
                .count(),
            309
        );
        assert_eq!(
            registry
                .record
                .iter()
                .filter(|record| record.legacy_format)
                .count(),
            17
        );
    }
}
