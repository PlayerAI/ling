use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::governance;

const MANIFEST_PATH: &str = "docs/governance/gap-register.toml";
const REPORT_PATH: &str = "docs/governance/gap-register.md";
const SOURCE_ROOTS: &[&str] = &["crates", "tests", "examples"];
const STATUSES: &[&str] = &["Open", "Proposed", "Accepted", "Rejected", "Superseded"];
const PRIORITIES: &[&str] = &["P0", "P1", "P2", "P3"];
const RELEASES: &[&str] = &["v0.1", "v0.2", "v0.3", "v0.4", "v0.5", "v1.0"];
const REQUIRED_EVIDENCE: &[&str] = &["positive", "negative", "migration"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub gap_count: usize,
    pub open_count: usize,
    pub gate_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GapRegister {
    schema_version: u32,
    updated: String,
    #[serde(default)]
    gate: Vec<Gate>,
    #[serde(default)]
    gap: Vec<Gap>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Gate {
    id: String,
    title: String,
    release: String,
    #[serde(default)]
    authority: Vec<String>,
    #[serde(default)]
    gaps: Vec<String>,
    #[serde(default)]
    decisions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Gap {
    id: String,
    title: String,
    status: String,
    priority: String,
    #[serde(default)]
    blocked_releases: Vec<String>,
    #[serde(default)]
    blocked_tasks: Vec<String>,
    observable_behavior: String,
    #[serde(default)]
    authority: Vec<String>,
    #[serde(default)]
    candidate_rfcs: Vec<String>,
    #[serde(default)]
    source_items: Vec<String>,
    #[serde(default)]
    options: Vec<String>,
    #[serde(default)]
    irreversible_consequences: Vec<String>,
    #[serde(default)]
    required_evidence: Vec<String>,
    owner_role: String,
    next_action: String,
    #[serde(default)]
    resolution: Vec<String>,
    #[serde(default)]
    superseded_by: String,
    #[serde(default)]
    observed_markers: Vec<String>,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let authority = governance::document_statuses(root)?;
    let register = load_register(root)?;
    let mut errors = validate(root, &register, &authority);
    let rendered = render(&register);
    match fs::read_to_string(root.join(REPORT_PATH)) {
        Ok(actual) if normalize_newlines(&actual) == rendered => {}
        Ok(_) => errors.push(format!(
            "GAP-REG-0008: {} is not the deterministic rendering of {}",
            REPORT_PATH, MANIFEST_PATH
        )),
        Err(error) => errors.push(format!(
            "GAP-REG-0002: cannot read {}: {error}",
            REPORT_PATH
        )),
    }

    finish(errors).map(|()| CheckSummary {
        gap_count: register.gap.len(),
        open_count: register
            .gap
            .iter()
            .filter(|gap| gap.status == "Open")
            .count(),
        gate_count: register.gate.len(),
    })
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    let authority = governance::document_statuses(root)?;
    let register = load_register(root)?;
    finish(validate(root, &register, &authority)).map(|()| render(&register))
}

pub(crate) fn registered_gap_ids(root: &Path) -> Result<BTreeSet<String>, Vec<String>> {
    let authority_statuses = governance::document_statuses(root)?;
    let register = load_register(root)?;
    finish(validate(root, &register, &authority_statuses))?;
    Ok(register.gap.into_iter().map(|gap| gap.id).collect())
}

fn load_register(root: &Path) -> Result<GapRegister, Vec<String>> {
    let text = fs::read_to_string(root.join(MANIFEST_PATH)).map_err(|error| {
        vec![format!(
            "GAP-REG-0002: cannot read {}: {error}",
            MANIFEST_PATH
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GAP-REG-0014: invalid gap register {}: {error}",
            MANIFEST_PATH
        )]
    })
}

fn validate(
    root: &Path,
    register: &GapRegister,
    authority: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut errors = Vec::new();
    if register.schema_version != 1 {
        errors.push(format!(
            "GAP-REG-0007: unsupported schema_version {}; expected 1",
            register.schema_version
        ));
    }
    if register.updated.trim().is_empty() {
        errors.push("GAP-REG-0007: updated must not be empty".to_owned());
    }
    if register.gap.is_empty() {
        errors.push("GAP-REG-0007: gap register contains no gaps".to_owned());
    }

    let mut by_id = BTreeMap::new();
    let mut declared_markers = BTreeMap::new();
    for gap in &register.gap {
        validate_gap(root, gap, authority, &mut declared_markers, &mut errors);
        if by_id.insert(gap.id.clone(), gap).is_some() {
            errors.push(format!("GAP-REG-0001: duplicate gap id {}", gap.id));
        }
    }

    for gap in &register.gap {
        validate_lifecycle(gap, &by_id, authority, &mut errors);
    }
    validate_supersession_cycles(&by_id, &mut errors);
    validate_gates(&register.gate, &by_id, authority, &mut errors);
    validate_source_markers(root, &by_id, &declared_markers, &mut errors);
    errors
}

fn validate_gap(
    root: &Path,
    gap: &Gap,
    authority: &BTreeMap<String, String>,
    declared_markers: &mut BTreeMap<(String, String), String>,
    errors: &mut Vec<String>,
) {
    if !is_gap_id(&gap.id) {
        errors.push(format!(
            "GAP-REG-0007: {} is not a valid GAP identifier",
            display_id(gap)
        ));
    }
    if gap.title.trim().is_empty()
        || gap.observable_behavior.trim().is_empty()
        || gap.owner_role.trim().is_empty()
        || gap.next_action.trim().is_empty()
    {
        errors.push(format!(
            "GAP-REG-0007: {} has an empty required field",
            display_id(gap)
        ));
    }
    if !STATUSES.contains(&gap.status.as_str()) {
        errors.push(format!(
            "GAP-REG-0003: {} has unknown status {}",
            display_id(gap),
            gap.status
        ));
    }
    if !PRIORITIES.contains(&gap.priority.as_str()) {
        errors.push(format!(
            "GAP-REG-0007: {} has unknown priority {}",
            display_id(gap),
            gap.priority
        ));
    }
    if gap.blocked_releases.is_empty() || gap.blocked_tasks.is_empty() {
        errors.push(format!(
            "GAP-REG-0007: {} must block at least one release and task",
            display_id(gap)
        ));
    }
    for release in &gap.blocked_releases {
        if !RELEASES.contains(&release.as_str()) {
            errors.push(format!(
                "GAP-REG-0007: {} has unknown blocked release {}",
                display_id(gap),
                release
            ));
        }
    }
    validate_unique_values(
        display_id(gap),
        "blocked release",
        &gap.blocked_releases,
        errors,
    );
    validate_unique_values(display_id(gap), "blocked task", &gap.blocked_tasks, errors);

    if gap.authority.is_empty()
        || gap.source_items.is_empty()
        || gap.options.len() < 2
        || gap.irreversible_consequences.is_empty()
    {
        errors.push(format!(
            "GAP-REG-0007: {} lacks authority, source items, two options, or irreversible consequences",
            display_id(gap)
        ));
    }
    for authority_id in &gap.authority {
        if !authority.contains_key(authority_id) {
            errors.push(format!(
                "GAP-REG-0004: {} references unknown authority {}",
                display_id(gap),
                authority_id
            ));
        }
    }
    for rfc in &gap.candidate_rfcs {
        if !is_rfc_id(rfc) {
            errors.push(format!(
                "GAP-REG-0007: {} has invalid candidate RFC {}",
                display_id(gap),
                rfc
            ));
        }
    }
    validate_unique_values(display_id(gap), "authority", &gap.authority, errors);
    validate_unique_values(
        display_id(gap),
        "candidate RFC",
        &gap.candidate_rfcs,
        errors,
    );
    validate_unique_values(display_id(gap), "source item", &gap.source_items, errors);
    validate_unique_values(display_id(gap), "evidence", &gap.required_evidence, errors);
    for required in REQUIRED_EVIDENCE {
        if !gap.required_evidence.iter().any(|item| item == required) {
            errors.push(format!(
                "GAP-REG-0007: {} lacks required {required} evidence",
                display_id(gap)
            ));
        }
    }

    for marker in &gap.observed_markers {
        let Some((kind, path)) = marker.split_once(':') else {
            errors.push(format!(
                "GAP-REG-0007: {} has malformed observed marker {}",
                display_id(gap),
                marker
            ));
            continue;
        };
        if !matches!(kind, "experimental" | "UNSPECIFIED") || !is_relative_path(path) {
            errors.push(format!(
                "GAP-REG-0007: {} has invalid observed marker {}",
                display_id(gap),
                marker
            ));
            continue;
        }
        if !root.join(path).is_file() {
            errors.push(format!(
                "GAP-REG-0002: {} observed marker path is missing: {}",
                display_id(gap),
                path
            ));
        }
        let key = (kind.to_owned(), path.to_owned());
        if let Some(previous) = declared_markers.insert(key, gap.id.clone()) {
            errors.push(format!(
                "GAP-REG-0007: implementation marker {} is assigned to both {} and {}",
                marker, previous, gap.id
            ));
        }
    }
}

fn validate_lifecycle(
    gap: &Gap,
    gaps: &BTreeMap<String, &Gap>,
    authority: &BTreeMap<String, String>,
    errors: &mut Vec<String>,
) {
    match gap.status.as_str() {
        "Accepted" | "Rejected" => {
            if gap.resolution.is_empty() {
                errors.push(format!(
                    "GAP-REG-0005: {} status {} requires an Accepted resolution document",
                    display_id(gap),
                    gap.status
                ));
            }
            if !gap.superseded_by.is_empty() {
                errors.push(format!(
                    "GAP-REG-0005: {} status {} cannot set superseded_by",
                    display_id(gap),
                    gap.status
                ));
            }
        }
        "Superseded" => {
            if gap.superseded_by.is_empty() {
                errors.push(format!(
                    "GAP-REG-0005: {} status Superseded requires superseded_by",
                    display_id(gap)
                ));
            } else if gap.superseded_by == gap.id {
                errors.push(format!(
                    "GAP-REG-0005: {} cannot supersede itself",
                    display_id(gap)
                ));
            } else if !gaps.contains_key(&gap.superseded_by) {
                errors.push(format!(
                    "GAP-REG-0004: {} is superseded by unknown gap {}",
                    display_id(gap),
                    gap.superseded_by
                ));
            }
            if !gap.resolution.is_empty() {
                errors.push(format!(
                    "GAP-REG-0005: {} status Superseded cannot set resolution",
                    display_id(gap)
                ));
            }
        }
        "Open" | "Proposed" if !gap.resolution.is_empty() || !gap.superseded_by.is_empty() => {
            errors.push(format!(
                "GAP-REG-0005: {} status {} cannot claim a resolution or successor",
                display_id(gap),
                gap.status
            ));
        }
        "Open" | "Proposed" => {}
        _ => {}
    }

    for resolution in &gap.resolution {
        match authority.get(resolution) {
            Some(status) if status == "Accepted" => {}
            Some(status) => errors.push(format!(
                "GAP-REG-0005: {} resolution {} is {}, not Accepted",
                display_id(gap),
                resolution,
                status
            )),
            None => errors.push(format!(
                "GAP-REG-0004: {} references unknown resolution {}",
                display_id(gap),
                resolution
            )),
        }
    }
}

fn validate_gates(
    gates: &[Gate],
    gaps: &BTreeMap<String, &Gap>,
    authority: &BTreeMap<String, String>,
    errors: &mut Vec<String>,
) {
    let mut ids = BTreeSet::new();
    for gate in gates {
        if gate.id.trim().is_empty()
            || gate.title.trim().is_empty()
            || !RELEASES.contains(&gate.release.as_str())
            || gate.authority.is_empty()
            || (gate.gaps.is_empty() && gate.decisions.is_empty())
        {
            errors.push(format!(
                "GAP-REG-0013: gate {} has invalid or missing required fields",
                display_gate_id(gate)
            ));
        }
        if !ids.insert(&gate.id) {
            errors.push(format!("GAP-REG-0001: duplicate gate id {}", gate.id));
        }
        for authority_id in &gate.authority {
            if !authority.contains_key(authority_id) {
                errors.push(format!(
                    "GAP-REG-0004: gate {} references unknown authority {}",
                    display_gate_id(gate),
                    authority_id
                ));
            }
        }
        for gap_id in &gate.gaps {
            match gaps.get(gap_id) {
                Some(gap) if !gap.blocked_releases.contains(&gate.release) => {
                    errors.push(format!(
                        "GAP-REG-0013: gate {} release {} is not blocked by {}",
                        display_gate_id(gate),
                        gate.release,
                        gap_id
                    ));
                }
                Some(_) => {}
                None => errors.push(format!(
                    "GAP-REG-0004: gate {} references unknown gap {}",
                    display_gate_id(gate),
                    gap_id
                )),
            }
        }
        for decision in &gate.decisions {
            match authority.get(decision) {
                Some(status) if status == "Accepted" => {}
                Some(status) => errors.push(format!(
                    "GAP-REG-0013: gate {} decision {} is {}, not Accepted",
                    display_gate_id(gate),
                    decision,
                    status
                )),
                None => errors.push(format!(
                    "GAP-REG-0004: gate {} references unknown decision {}",
                    display_gate_id(gate),
                    decision
                )),
            }
        }
        validate_unique_values(display_gate_id(gate), "gap", &gate.gaps, errors);
        validate_unique_values(display_gate_id(gate), "decision", &gate.decisions, errors);
    }
}

fn validate_supersession_cycles(gaps: &BTreeMap<String, &Gap>, errors: &mut Vec<String>) {
    let mut visited = BTreeSet::new();
    let mut visiting = Vec::new();
    for id in gaps.keys() {
        visit_successor(id, gaps, &mut visited, &mut visiting, errors);
    }
}

fn visit_successor(
    id: &str,
    gaps: &BTreeMap<String, &Gap>,
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
            "GAP-REG-0006: gap supersession cycle: {}",
            cycle.join(" -> ")
        ));
        return;
    }
    let Some(gap) = gaps.get(id) else {
        return;
    };
    visiting.push(id.to_owned());
    if !gap.superseded_by.is_empty() && gaps.contains_key(&gap.superseded_by) {
        visit_successor(&gap.superseded_by, gaps, visited, visiting, errors);
    }
    visiting.pop();
    visited.insert(id.to_owned());
}

fn validate_source_markers(
    root: &Path,
    gaps: &BTreeMap<String, &Gap>,
    declared: &BTreeMap<(String, String), String>,
    errors: &mut Vec<String>,
) {
    let mut found = BTreeSet::new();
    for source_root in SOURCE_ROOTS {
        let path = root.join(source_root);
        if path.is_dir() {
            scan_directory(root, &path, gaps, &mut found, errors);
        }
    }

    for marker in &found {
        if !declared.contains_key(marker) {
            errors.push(format!(
                "GAP-REG-0011: unmapped implementation marker {}:{}",
                marker.0, marker.1
            ));
        }
    }
    for ((kind, path), gap_id) in declared {
        if !found.contains(&(kind.clone(), path.clone())) {
            errors.push(format!(
                "GAP-REG-0012: {} declares stale implementation marker {}:{}",
                gap_id, kind, path
            ));
        }
    }
}

fn scan_directory(
    root: &Path,
    directory: &Path,
    gaps: &BTreeMap<String, &Gap>,
    found: &mut BTreeSet<(String, String)>,
    errors: &mut Vec<String>,
) {
    let Ok(entries) = fs::read_dir(directory) else {
        errors.push(format!(
            "GAP-REG-0015: cannot read source directory {}",
            relative_path(root, directory)
        ));
        return;
    };
    let mut paths = entries
        .flatten()
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    paths.sort();
    for path in paths {
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            errors.push(format!(
                "GAP-REG-0015: cannot inspect source path {}",
                relative_path(root, &path)
            ));
            continue;
        };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            scan_directory(root, &path, gaps, found, errors);
            continue;
        }
        if !matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("rs" | "ling")
        ) {
            continue;
        }
        let relative = relative_path(root, &path);
        let Ok(text) = fs::read_to_string(&path) else {
            errors.push(format!("GAP-REG-0015: cannot read source file {relative}"));
            continue;
        };
        if text.contains("experimental:") {
            found.insert(("experimental".to_owned(), relative.clone()));
        }
        if text.contains("UNSPECIFIED") {
            found.insert(("UNSPECIFIED".to_owned(), relative.clone()));
        }
        for (line_index, line) in text.lines().enumerate() {
            validate_todo_markers(&relative, line_index + 1, line, gaps, errors);
        }
    }
}

fn validate_todo_markers(
    path: &str,
    line_number: usize,
    line: &str,
    gaps: &BTreeMap<String, &Gap>,
    errors: &mut Vec<String>,
) {
    let mut remaining = line;
    while let Some(position) = remaining.find("TODO(spec") {
        let marker = &remaining[position..];
        let Some(rest) = marker.strip_prefix("TODO(spec:") else {
            errors.push(format!(
                "GAP-REG-0009: {path}:{line_number} TODO(spec) lacks a gap ID"
            ));
            remaining = &marker["TODO(spec".len()..];
            continue;
        };
        let Some(end) = rest.find(')') else {
            errors.push(format!(
                "GAP-REG-0009: {path}:{line_number} has an unterminated TODO(spec:...) marker"
            ));
            break;
        };
        let id = &rest[..end];
        if !is_gap_id(id) {
            errors.push(format!(
                "GAP-REG-0009: {path}:{line_number} has invalid TODO(spec) gap ID {id}"
            ));
        } else if !gaps.contains_key(id) {
            errors.push(format!(
                "GAP-REG-0010: {path}:{line_number} references unknown gap {id}"
            ));
        }
        remaining = &rest[end + 1..];
    }
}

fn render(register: &GapRegister) -> String {
    let mut gates = register.gate.iter().collect::<Vec<_>>();
    gates.sort_by(|left, right| {
        release_rank(&left.release)
            .cmp(&release_rank(&right.release))
            .then_with(|| left.id.cmp(&right.id))
    });
    let mut gaps = register.gap.iter().collect::<Vec<_>>();
    gaps.sort_by(|left, right| gap_sort_key(left).cmp(&gap_sort_key(right)));

    let mut output = String::new();
    output.push_str("# Ling 规范缺口台账 / Specification Gap Register\n\n");
    output.push_str("> 状态：由 `gap-register.toml` 确定性生成\n");
    output.push_str(&format!("> 更新日期：{}\n", register.updated));
    output.push_str("> 本台账记录未决问题及阻断关系，不替任何候选方案作出语义决议。\n\n");

    output.push_str("## Summary\n\n");
    output.push_str(&format!("- Total gaps: {}\n", gaps.len()));
    for status in STATUSES {
        let count = gaps.iter().filter(|gap| gap.status == *status).count();
        output.push_str(&format!("- {status}: {count}\n"));
    }

    output.push_str("\n## Specification-gate coverage\n\n");
    output.push_str("| Gate | Release | Authority | Open gaps | Accepted decisions |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for gate in gates {
        output.push_str(&format!(
            "| `{}` — {} | `{}` | {} | {} | {} |\n",
            escape_cell(&gate.id),
            escape_cell(&gate.title),
            escape_cell(&gate.release),
            list_cell(&gate.authority),
            list_cell(&gate.gaps),
            list_cell(&gate.decisions)
        ));
    }

    output.push_str("\n## Gaps by earliest blocked release\n");
    for release in RELEASES {
        let release_gaps = gaps
            .iter()
            .copied()
            .filter(|gap| earliest_release(gap) == *release)
            .collect::<Vec<_>>();
        if release_gaps.is_empty() {
            continue;
        }
        output.push_str(&format!("\n### {release}\n\n"));
        output.push_str("| ID | Priority | Status | Title | Blocked tasks | Candidate RFCs |\n");
        output.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for gap in release_gaps {
            output.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} | {} |\n",
                escape_cell(&gap.id),
                escape_cell(&gap.priority),
                escape_cell(&gap.status),
                escape_cell(&gap.title),
                list_cell(&gap.blocked_tasks),
                list_cell(&gap.candidate_rfcs)
            ));
        }
    }

    output.push_str("\n## Workflow\n\n");
    output.push_str("1. Add an Open gap before implementation would otherwise choose unspecified observable behavior.\n");
    output.push_str("2. Keep candidate options neutral; prototypes remain isolated and cannot create Stable behavior.\n");
    output.push_str(
        "3. Move a gap to Accepted or Rejected only with an Accepted resolution document.\n",
    );
    output.push_str("4. Attach positive, negative, and migration evidence before unblocking the affected release tasks.\n");
    output.push_str(
        "5. Use `TODO(spec:GAP-...)` in source; an unregistered `TODO(spec)` fails the checker.\n",
    );
    output.push_str(
        "6. Run `cargo xtask governance check-gaps` and the relevant conformance suites.\n\n",
    );
    output.push_str("## Machine source\n\n");
    output.push_str("The machine-readable source is [`gap-register.toml`](gap-register.toml). The checker rejects duplicate IDs, invalid lifecycle transitions, dangling authority/gate/supersession relations, supersession cycles, incomplete evidence categories, unmapped implementation markers, unregistered `TODO(spec)`, and report drift.\n");
    output
}

fn gap_sort_key(gap: &Gap) -> (u8, u8, u8, &str) {
    (
        release_rank(earliest_release(gap)),
        priority_rank(&gap.priority),
        status_rank(&gap.status),
        &gap.id,
    )
}

fn earliest_release(gap: &Gap) -> &str {
    gap.blocked_releases
        .iter()
        .min_by_key(|release| release_rank(release))
        .map(String::as_str)
        .unwrap_or("v1.0")
}

fn release_rank(release: &str) -> u8 {
    RELEASES
        .iter()
        .position(|candidate| *candidate == release)
        .map_or(u8::MAX, |index| index as u8)
}

fn priority_rank(priority: &str) -> u8 {
    PRIORITIES
        .iter()
        .position(|candidate| *candidate == priority)
        .map_or(u8::MAX, |index| index as u8)
}

fn status_rank(status: &str) -> u8 {
    STATUSES
        .iter()
        .position(|candidate| *candidate == status)
        .map_or(u8::MAX, |index| index as u8)
}

fn validate_unique_values(id: &str, field: &str, values: &[String], errors: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    for value in values {
        if value.trim().is_empty() {
            errors.push(format!("GAP-REG-0007: {id} has an empty {field}"));
        } else if !seen.insert(value) {
            errors.push(format!("GAP-REG-0007: {id} repeats {field} value {value}"));
        }
    }
}

fn display_id(gap: &Gap) -> &str {
    if gap.id.is_empty() {
        "<missing-id>"
    } else {
        &gap.id
    }
}

fn display_gate_id(gate: &Gate) -> &str {
    if gate.id.is_empty() {
        "<missing-gate-id>"
    } else {
        &gate.id
    }
}

fn is_gap_id(value: &str) -> bool {
    value.starts_with("GAP-")
        && !value.ends_with('-')
        && !value.contains("--")
        && value[4..].chars().all(|character| {
            character.is_ascii_uppercase() || character.is_ascii_digit() || character == '-'
        })
}

fn is_rfc_id(value: &str) -> bool {
    value.len() == 8
        && value.starts_with("RFC-")
        && value[4..]
            .chars()
            .all(|character| character.is_ascii_digit())
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

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn list_cell(values: &[String]) -> String {
    if values.is_empty() {
        "—".to_owned()
    } else {
        values
            .iter()
            .map(|value| format!("`{}`", escape_cell(value)))
            .collect::<Vec<_>>()
            .join(", ")
    }
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
    use super::{GapRegister, render, validate};
    use std::collections::{BTreeMap, BTreeSet};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

    struct TempRepository(PathBuf);

    impl TempRepository {
        fn new() -> Self {
            let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "ling-gap-register-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(path.join("crates/fixture/src")).expect("create fixture source");
            Self(path)
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

    fn authority() -> BTreeMap<String, String> {
        [
            ("SEMANTICS", "Draft"),
            ("ROADMAP-1.0", "Planning"),
            ("DEC-0001", "Accepted"),
            ("RFC-0001", "Draft"),
        ]
        .into_iter()
        .map(|(id, status)| (id.to_owned(), status.to_owned()))
        .collect()
    }

    fn gap(id: &str) -> String {
        format!(
            r#"
[[gap]]
id = "{id}"
title = "Fixture gap"
status = "Open"
priority = "P0"
blocked_releases = ["v0.1"]
blocked_tasks = ["FIX-0001"]
observable_behavior = "Fixture behavior is undecided."
authority = ["SEMANTICS", "ROADMAP-1.0"]
candidate_rfcs = ["RFC-0002"]
source_items = ["FIXTURE-1"]
options = ["Option A", "Option B"]
irreversible_consequences = ["Published behavior becomes compatible input."]
required_evidence = ["positive", "negative", "migration"]
owner_role = "language-design"
next_action = "Draft a decision."
resolution = []
superseded_by = ""
observed_markers = []
"#
        )
    }

    fn gate(gap_id: &str) -> String {
        format!(
            r#"
[[gate]]
id = "G1-FIXTURE"
title = "Fixture gate"
release = "v0.1"
authority = ["ROADMAP-1.0"]
gaps = ["{gap_id}"]
decisions = []
"#
        )
    }

    fn parse(body: &str) -> GapRegister {
        toml::from_str(&format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{body}"
        ))
        .expect("valid fixture register")
    }

    #[test]
    fn rejects_duplicate_gap_ids() {
        let repository = TempRepository::new();
        let register = parse(&format!(
            "{}{}",
            gap("GAP-FIXTURE-001"),
            gap("GAP-FIXTURE-001")
        ));
        let errors = validate(repository.path(), &register, &authority());
        assert!(errors.iter().any(|error| error.contains("GAP-REG-0001")));
    }

    #[test]
    fn rejects_accepted_gap_without_accepted_resolution() {
        let repository = TempRepository::new();
        let register =
            parse(&gap("GAP-FIXTURE-001").replace("status = \"Open\"", "status = \"Accepted\""));
        let errors = validate(repository.path(), &register, &authority());
        assert!(errors.iter().any(|error| error.contains("GAP-REG-0005")));
    }

    #[test]
    fn accepts_rejected_gap_with_accepted_resolution() {
        let repository = TempRepository::new();
        let body = gap("GAP-FIXTURE-001")
            .replace("status = \"Open\"", "status = \"Rejected\"")
            .replace("resolution = []", "resolution = [\"DEC-0001\"]");
        assert!(validate(repository.path(), &parse(&body), &authority()).is_empty());
    }

    #[test]
    fn accepts_accepted_gap_with_accepted_resolution() {
        let repository = TempRepository::new();
        let body = gap("GAP-FIXTURE-001")
            .replace("status = \"Open\"", "status = \"Accepted\"")
            .replace("resolution = []", "resolution = [\"DEC-0001\"]");
        assert!(validate(repository.path(), &parse(&body), &authority()).is_empty());
    }

    #[test]
    fn rejects_unknown_lifecycle_status() {
        let repository = TempRepository::new();
        let body = gap("GAP-FIXTURE-001").replace("status = \"Open\"", "status = \"Closed\"");
        let errors = validate(repository.path(), &parse(&body), &authority());
        assert!(errors.iter().any(|error| error.contains("GAP-REG-0003")));
    }

    #[test]
    fn rejects_gate_with_unknown_gap() {
        let repository = TempRepository::new();
        let register = parse(&format!(
            "{}{}",
            gate("GAP-MISSING-001"),
            gap("GAP-FIXTURE-001")
        ));
        let errors = validate(repository.path(), &register, &authority());
        assert!(errors.iter().any(|error| error.contains("GAP-REG-0004")));
    }

    #[test]
    fn rejects_todo_spec_without_gap_id() {
        let repository = TempRepository::new();
        repository.write("crates/fixture/src/lib.rs", "// TODO(spec): decide this\n");
        let register = parse(&gap("GAP-FIXTURE-001"));
        let errors = validate(repository.path(), &register, &authority());
        assert!(errors.iter().any(|error| error.contains("GAP-REG-0009")));
    }

    #[test]
    fn rejects_todo_spec_with_unknown_gap_id() {
        let repository = TempRepository::new();
        repository.write(
            "crates/fixture/src/lib.rs",
            "// TODO(spec:GAP-MISSING-001): decide this\n",
        );
        let register = parse(&gap("GAP-FIXTURE-001"));
        let errors = validate(repository.path(), &register, &authority());
        assert!(errors.iter().any(|error| error.contains("GAP-REG-0010")));
    }

    #[test]
    fn accepts_todo_spec_with_registered_gap_id() {
        let repository = TempRepository::new();
        repository.write(
            "crates/fixture/src/lib.rs",
            "// TODO(spec:GAP-FIXTURE-001): decide this\n",
        );
        assert!(
            validate(
                repository.path(),
                &parse(&gap("GAP-FIXTURE-001")),
                &authority()
            )
            .is_empty()
        );
    }

    #[test]
    fn rejects_unmapped_experimental_marker() {
        let repository = TempRepository::new();
        repository.write(
            "crates/fixture/src/lib.rs",
            "const ID: &str = \"experimental:x\";\n",
        );
        let register = parse(&gap("GAP-FIXTURE-001"));
        let errors = validate(repository.path(), &register, &authority());
        assert!(errors.iter().any(|error| error.contains("GAP-REG-0011")));
    }

    #[test]
    fn rejects_supersession_cycles() {
        let repository = TempRepository::new();
        let left = gap("GAP-FIXTURE-001")
            .replace("status = \"Open\"", "status = \"Superseded\"")
            .replace(
                "superseded_by = \"\"",
                "superseded_by = \"GAP-FIXTURE-002\"",
            );
        let right = gap("GAP-FIXTURE-002")
            .replace("status = \"Open\"", "status = \"Superseded\"")
            .replace(
                "superseded_by = \"\"",
                "superseded_by = \"GAP-FIXTURE-001\"",
            );
        let errors = validate(
            repository.path(),
            &parse(&format!("{left}{right}")),
            &authority(),
        );
        assert!(errors.iter().any(|error| error.contains("GAP-REG-0006")));
    }

    #[test]
    fn rendering_is_deterministic() {
        let register = parse(&format!(
            "{}{}",
            gate("GAP-FIXTURE-001"),
            gap("GAP-FIXTURE-001")
        ));
        assert_eq!(render(&register), render(&register));
    }

    #[test]
    fn rendering_sorts_by_release_priority_status_and_id() {
        let later = gap("GAP-Z-LATER-001")
            .replace(
                "blocked_releases = [\"v0.1\"]",
                "blocked_releases = [\"v0.2\"]",
            )
            .replace("priority = \"P0\"", "priority = \"P1\"");
        let first = gap("GAP-A-FIRST-001");
        let output = render(&parse(&format!("{later}{first}")));
        assert!(
            output.find("GAP-A-FIRST-001").expect("first gap rendered")
                < output.find("GAP-Z-LATER-001").expect("later gap rendered")
        );
    }

    #[test]
    fn repository_register_covers_required_sources_and_g1_gates() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let text = fs::read_to_string(root.join("docs/governance/gap-register.toml"))
            .expect("read repository gap register");
        let register: GapRegister = toml::from_str(&text).expect("parse repository gap register");
        let sources = register
            .gap
            .iter()
            .flat_map(|gap| gap.source_items.iter().map(String::as_str))
            .collect::<BTreeSet<_>>();
        for item in 1..=11 {
            assert!(sources.contains(format!("SEMANTICS-31.{item}").as_str()));
        }
        for rfc in 2..=13 {
            assert!(sources.contains(format!("RFC-0001-22:RFC-{rfc:04}").as_str()));
        }
        let expected_gates = [
            "G1-PACKAGE",
            "G1-BYTECODE",
            "G1-TRAIT",
            "G1-INCREMENTAL",
            "G1-FORMATTER",
            "G1-LSP-TRANSACTION",
        ];
        assert_eq!(register.gate.len(), expected_gates.len());
        for expected in expected_gates {
            let gate = register
                .gate
                .iter()
                .find(|gate| gate.id == expected)
                .expect("required G1 gate is registered");
            assert!(!gate.gaps.is_empty() || !gate.decisions.is_empty());
            for gap_id in &gate.gaps {
                let gap = register
                    .gap
                    .iter()
                    .find(|gap| &gap.id == gap_id)
                    .expect("gate gap exists");
                assert_eq!(gap.priority, "P0");
            }
        }

        let plan_directory = root.join("docs/ling_execution_plan");
        let mut plan_text = String::new();
        for entry in fs::read_dir(plan_directory).expect("read execution plan") {
            let path = entry.expect("read execution plan entry").path();
            if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
                plan_text.push_str(&fs::read_to_string(path).expect("read execution plan file"));
            }
        }
        for gap in &register.gap {
            for task in &gap.blocked_tasks {
                assert!(
                    plan_text.contains(task),
                    "{} blocks unknown execution task {task}",
                    gap.id
                );
            }
        }
    }
}
