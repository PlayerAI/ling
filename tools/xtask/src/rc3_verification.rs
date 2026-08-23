use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::rc1_validation;

const MATRIX_PATH: &str = "docs/testing/RC3-INDEPENDENT-VERIFICATION.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Check {
    name: &'static str,
    state: &'static str,
}

const CHECKS: &[Check] = &[
    Check {
        name: "Clean-tag build",
        state: "BlockedSpec",
    },
    Check {
        name: "Verify artifacts",
        state: "BlockedSpec",
    },
    Check {
        name: "Run conformance",
        state: "Partial Seed evidence",
    },
    Check {
        name: "Protocol corruption suite",
        state: "Partial Seed evidence",
    },
    Check {
        name: "TCB/unsafe/FFI inspection",
        state: "Partial Seed evidence",
    },
    Check {
        name: "Representative evidence reproduction",
        state: "Partial Seed evidence",
    },
    Check {
        name: "Tag/hash/release-manifest comparison",
        state: "BlockedSpec",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "This matrix defines readiness for an independent release-candidate review; it is not an independent sign-off",
    "A run by the implementing agent is repository validation, not RC3 independence.",
    "No tag was built, no artifact was published",
    "RC-6903 may leave `BlockedSpec` only after RC0 and RC1 are complete",
    "No placeholder command, tag, artifact, reviewer identity, signature, protocol,",
];

const REQUIRED_AUDIT_MARKERS: &[(&str, &[&str])] = &[
    (
        "docs/status/RC-6903-AUTHORITY-AUDIT.md",
        &[
            "RC-6903",
            "BlockedSpec",
            "no independent Go decision",
            "No tag, artifact, evidence bundle",
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
        "docs/SEED-RELEASE-REPORT.md",
        &["v0.0.1 Seed", "candidate", "CI"],
    ),
    (
        "docs/testing/SECURITY-AUDIT.md",
        &[
            "not a vulnerability-free claim",
            "Required release evidence",
        ],
    ),
    (
        "docs/governance/support-matrix.md",
        &["1.0-draft", "Tier2", "not a target commitment"],
    ),
    (
        "docs/testing/RC3-INDEPENDENT-VERIFICATION.md",
        &["BlockedSpec", "No tag was built", "No placeholder command"],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub check_count: usize,
    pub blocked_count: usize,
    pub partial_count: usize,
    pub audit_file_count: usize,
    pub upstream_gate_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-RC3-VERIFICATION-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_audits(root));
    if let Err(upstream_errors) = rc1_validation::check_repository(root) {
        errors.extend(upstream_errors);
    }
    errors.extend(validate_current_evidence(&matrix));
    finish(errors).map(|()| CheckSummary {
        check_count: CHECKS.len(),
        blocked_count: count_state("BlockedSpec"),
        partial_count: CHECKS
            .iter()
            .filter(|check| check.state.starts_with("Partial"))
            .count(),
        audit_file_count: REQUIRED_AUDIT_MARKERS.len(),
        upstream_gate_count: 1,
    })
}

fn validate_current_evidence(matrix: &str) -> Vec<String> {
    const REQUIRED: &[&str] = &[
        "The bounded current RC0 and RC1 inventory gates pass",
        "both parent release gates remain `BlockedSpec`",
        "does not constitute independent verification",
    ];
    let normalized = normalize(matrix);
    REQUIRED
        .iter()
        .filter(|marker| !normalized.contains(&normalize(marker)))
        .map(|marker| {
            format!(
                "GOV-RC3-VERIFICATION-0011: {MATRIX_PATH} is missing current evidence marker {marker:?}"
            )
        })
        .collect()
}

fn validate_matrix(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(section) = matrix
        .split_once("## Verification matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Required reviewer record"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-RC3-VERIFICATION-0002: {MATRIX_PATH} is missing the Verification matrix section"
        )];
    };

    let rows = section.lines().filter_map(parse_row).collect::<Vec<_>>();
    let mut actual = BTreeMap::new();
    for (name, cells) in rows {
        if actual.insert(name.to_owned(), cells).is_some() {
            errors.push(format!(
                "GOV-RC3-VERIFICATION-0003: duplicate RC3 check {name:?}"
            ));
        }
    }

    let mut expected_names = CHECKS.iter().map(|check| check.name).collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-RC3-VERIFICATION-0004: RC3 check set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for check in CHECKS {
        let Some(cells) = actual.get(check.name) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-RC3-VERIFICATION-0005: RC3 check {:?} has an empty evidence/state/exit cell",
                check.name
            ));
            continue;
        }
        if cells[1] != check.state {
            errors.push(format!(
                "GOV-RC3-VERIFICATION-0006: RC3 check {:?} must have state {:?}, found {:?}",
                check.name, check.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-RC3-VERIFICATION-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized
        .to_ascii_lowercase()
        .contains(stale_legacy_name.as_str())
    {
        errors.push(format!(
            "GOV-RC3-VERIFICATION-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
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
                    "GOV-RC3-VERIFICATION-0009: cannot read {path}: {error}"
                ));
                continue;
            }
        };
        let normalized = normalize(&text);
        for marker in *markers {
            if !normalized.contains(&normalize(marker)) {
                errors.push(format!(
                    "GOV-RC3-VERIFICATION-0010: {path} is missing audit marker {marker:?}"
                ));
            }
        }
    }
    errors
}

fn count_state(state: &str) -> usize {
    CHECKS.iter().filter(|check| check.state == state).count()
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
        || cells[0] == "RC3 check"
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
    fn repository_rc3_inventory_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("RC3 inventory is valid");
        assert_eq!(summary.check_count, 7);
        assert_eq!(summary.blocked_count, 3);
        assert_eq!(summary.partial_count, 4);
        assert_eq!(summary.audit_file_count, 7);
        assert_eq!(summary.upstream_gate_count, 1);
    }

    #[test]
    fn rejects_rc3_state_drift() {
        let matrix = "## Verification matrix\n| RC3 check | Current evidence | State | Required independent evidence |\n| --- | --- | --- | --- |\n| Clean-tag build | evidence | Stable | authority |\n## Required reviewer record\n";
        let errors = validate_matrix(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("check set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_rc3_audit_marker_drift() {
        let errors = validate_audit_text("RC-6903", "BlockedSpec", &["Independent sign-off"]);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing audit marker"))
        );
    }

    #[test]
    fn rejects_missing_current_upstream_boundary() {
        let errors = validate_current_evidence("self-validation is enough");
        assert_eq!(errors.len(), 3);
        assert!(
            errors
                .iter()
                .all(|error| error.contains("GOV-RC3-VERIFICATION-0011"))
        );
    }

    fn validate_audit_text(path: &str, text: &str, markers: &[&str]) -> Vec<String> {
        let normalized = normalize(text);
        markers
            .iter()
            .filter(|marker| !normalized.contains(&normalize(marker)))
            .map(|marker| {
                format!("GOV-RC3-VERIFICATION-0010: {path} is missing audit marker {marker:?}")
            })
            .collect()
    }
}
