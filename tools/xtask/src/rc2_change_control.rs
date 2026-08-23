use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::rc3_verification;

const MATRIX_PATH: &str = "docs/testing/RC2-FINAL-CHANGE-CONTROL.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EvidenceClass {
    name: &'static str,
    state: &'static str,
}

const EVIDENCE_CLASSES: &[EvidenceClass] = &[
    EvidenceClass {
        name: "Blocker-only scope",
        state: "BlockedSpec",
    },
    EvidenceClass {
        name: "Regression test",
        state: "Partial Seed evidence",
    },
    EvidenceClass {
        name: "Risk analysis",
        state: "BlockedSpec",
    },
    EvidenceClass {
        name: "Affected protocol/artifact analysis",
        state: "BlockedSpec",
    },
    EvidenceClass {
        name: "Full relevant matrix",
        state: "BlockedSpec",
    },
    EvidenceClass {
        name: "New candidate identity",
        state: "BlockedSpec",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "This document records the proposed blocker-only change boundary; it does not declare an RC2, Final candidate, Go decision, or source freeze.",
    "no change may be classified as an RC2 fix from this document alone.",
    "No tag, release manifest, artifact, issue disposition",
    "RC-6904 may leave `BlockedSpec` only after RC0, RC1, and independent verification are complete",
    "No placeholder command, tag, artifact, schema, protocol, blocker status,",
];

const REQUIRED_AUDIT_MARKERS: &[(&str, &[&str])] = &[
    (
        "docs/status/RC-6904-AUTHORITY-AUDIT.md",
        &[
            "RC-6904",
            "BlockedSpec",
            "no RC2 or Final candidate",
            "No source fix, tag, release artifact",
        ],
    ),
    (
        "docs/status/RC-6901-AUTHORITY-AUDIT.md",
        &["RC0", "BlockedSpec", "release authorities"],
    ),
    (
        "docs/status/RC-6902-AUTHORITY-AUDIT.md",
        &["RC1", "BlockedSpec", "public RC1"],
    ),
    (
        "docs/status/RC-6903-AUTHORITY-AUDIT.md",
        &["RC-6903", "BlockedSpec", "independent"],
    ),
    (
        "docs/governance/support-matrix.md",
        &["1.0-draft", "Tier2", "not a target commitment"],
    ),
    (
        "docs/governance/protocol-inventory.toml",
        &["stability = \"Preview\"", "migration_tool = \"None"],
    ),
    (
        "docs/testing/RC2-FINAL-CHANGE-CONTROL.md",
        &[
            "BlockedSpec",
            "No tag, release manifest",
            "No placeholder command",
        ],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub evidence_class_count: usize,
    pub blocked_count: usize,
    pub partial_count: usize,
    pub audit_file_count: usize,
    pub upstream_gate_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-RC2-CHANGE-CONTROL-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_audits(root));
    if let Err(upstream_errors) = rc3_verification::check_repository(root) {
        errors.extend(upstream_errors);
    }
    errors.extend(validate_current_evidence(&matrix));
    finish(errors).map(|()| CheckSummary {
        evidence_class_count: EVIDENCE_CLASSES.len(),
        blocked_count: count_state("BlockedSpec"),
        partial_count: EVIDENCE_CLASSES
            .iter()
            .filter(|class| class.state.starts_with("Partial"))
            .count(),
        audit_file_count: REQUIRED_AUDIT_MARKERS.len(),
        upstream_gate_count: 1,
    })
}

fn validate_current_evidence(matrix: &str) -> Vec<String> {
    const REQUIRED: &[&str] = &[
        "The current RC3→RC1→RC0 bounded inventory chain passes",
        "all three predecessor release gates remain `BlockedSpec`",
        "The 27-record protocol inventory",
    ];
    let normalized = normalize(matrix);
    REQUIRED
        .iter()
        .filter(|marker| !normalized.contains(&normalize(marker)))
        .map(|marker| {
            format!(
                "GOV-RC2-CHANGE-CONTROL-0011: {MATRIX_PATH} is missing current evidence marker {marker:?}"
            )
        })
        .collect()
}

fn validate_matrix(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(section) = matrix
        .split_once("## Change-control matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Allowed and forbidden changes"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-RC2-CHANGE-CONTROL-0002: {MATRIX_PATH} is missing the Change-control matrix section"
        )];
    };

    let rows = section.lines().filter_map(parse_row).collect::<Vec<_>>();
    let mut actual = BTreeMap::new();
    for (name, cells) in rows {
        if actual.insert(name.to_owned(), cells).is_some() {
            errors.push(format!(
                "GOV-RC2-CHANGE-CONTROL-0003: duplicate RC2 evidence class {name:?}"
            ));
        }
    }

    let mut expected_names = EVIDENCE_CLASSES
        .iter()
        .map(|class| class.name)
        .collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-RC2-CHANGE-CONTROL-0004: RC2 evidence-class set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for class in EVIDENCE_CLASSES {
        let Some(cells) = actual.get(class.name) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-RC2-CHANGE-CONTROL-0005: RC2 evidence class {:?} has an empty state/evidence/exit cell",
                class.name
            ));
            continue;
        }
        if cells[1] != class.state {
            errors.push(format!(
                "GOV-RC2-CHANGE-CONTROL-0006: RC2 evidence class {:?} must have state {:?}, found {:?}",
                class.name, class.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-RC2-CHANGE-CONTROL-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized
        .to_ascii_lowercase()
        .contains(stale_legacy_name.as_str())
    {
        errors.push(format!(
            "GOV-RC2-CHANGE-CONTROL-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
        ));
    }
    errors
}

fn validate_audits(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    for (path, markers) in REQUIRED_AUDIT_MARKERS {
        let text = match fs::read_to_string(root.join(path)) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!(
                    "GOV-RC2-CHANGE-CONTROL-0009: cannot read {path}: {error}"
                ));
                continue;
            }
        };
        let normalized = normalize(&text);
        for marker in *markers {
            if !normalized.contains(&normalize(marker)) {
                errors.push(format!(
                    "GOV-RC2-CHANGE-CONTROL-0010: {path} is missing audit marker {marker:?}"
                ));
            }
        }
    }
    errors
}

fn count_state(state: &str) -> usize {
    EVIDENCE_CLASSES
        .iter()
        .filter(|class| class.state == state)
        .count()
}

fn parse_row(line: &str) -> Option<(&str, Vec<&str>)> {
    let cells = line
        .trim()
        .strip_prefix('|')?
        .strip_suffix('|')?
        .split('|')
        .map(|cell| cell.trim().trim_matches('`'))
        .collect::<Vec<_>>();
    if cells.len() < 4
        || cells[0] == "Required RC2 evidence"
        || cells[0].chars().all(|character| character == '-')
    {
        return None;
    }
    Some((cells[0], cells[1..].to_vec()))
}

fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
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
    fn repository_rc2_inventory_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("RC2 inventory is valid");
        assert_eq!(summary.evidence_class_count, 6);
        assert_eq!(summary.blocked_count, 5);
        assert_eq!(summary.partial_count, 1);
        assert_eq!(summary.audit_file_count, 7);
        assert_eq!(summary.upstream_gate_count, 1);
    }

    #[test]
    fn rejects_rc2_state_drift() {
        let matrix = "## Change-control matrix\n| Required RC2 evidence | Current repository state | State | Required before an RC2 change |\n| --- | --- | --- | --- |\n| Blocker-only scope | evidence | Stable | authority |\n## Allowed and forbidden changes\n";
        let errors = validate_matrix(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("evidence-class set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_rc2_audit_marker_drift() {
        let errors = validate_audit_text("RC-6904", "BlockedSpec", &["Accepted blocker"]);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing audit marker"))
        );
    }

    #[test]
    fn rejects_stale_upstream_and_protocol_evidence() {
        let errors = validate_current_evidence("The 21-record protocol inventory");
        assert_eq!(errors.len(), 3);
        assert!(
            errors
                .iter()
                .all(|error| error.contains("GOV-RC2-CHANGE-CONTROL-0011"))
        );
    }

    fn validate_audit_text(path: &str, text: &str, markers: &[&str]) -> Vec<String> {
        let normalized = normalize(text);
        markers
            .iter()
            .filter(|marker| !normalized.contains(&normalize(marker)))
            .map(|marker| {
                format!("GOV-RC2-CHANGE-CONTROL-0010: {path} is missing audit marker {marker:?}")
            })
            .collect()
    }
}
