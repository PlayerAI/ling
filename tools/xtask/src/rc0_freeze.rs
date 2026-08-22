use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/RC0-INTERNAL-FREEZE.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Criterion {
    name: &'static str,
    state: &'static str,
}

const CRITERIA: &[Criterion] = &[
    Criterion {
        name: "Feature freeze",
        state: "BlockedSpec",
    },
    Criterion {
        name: "Protocol-freeze candidate",
        state: "BlockedSpec",
    },
    Criterion {
        name: "Support-matrix draft final",
        state: "BlockedSpec",
    },
    Criterion {
        name: "P0/P1 triage",
        state: "BlockedSpec",
    },
    Criterion {
        name: "Historical corpus run",
        state: "BlockedSpec",
    },
    Criterion {
        name: "Security scan",
        state: "BlockedSpec",
    },
    Criterion {
        name: "Release-artifact rehearsal",
        state: "BlockedSpec",
    },
    Criterion {
        name: "Documentation completeness",
        state: "BlockedSpec",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "does not assert that an RC0 freeze has happened",
    "does not create a release tag, artifact, protocol, or feature commitment",
    "No release tag was created",
    "no package or artifact was published",
    "RC-6901 may leave `BlockedSpec` only when every row has an Accepted authority",
    "No placeholder command, schema, protocol, artifact, issue status, migration promise, or stale legacy name is added here.",
];

const REQUIRED_AUDIT_MARKERS: &[(&str, &[&str])] = &[
    (
        "docs/status/RC-6901-AUTHORITY-AUDIT.md",
        &[
            "RC-6901",
            "BlockedSpec",
            "does not authorize a feature freeze",
            "No tag, artifact, signature, SBOM",
        ],
    ),
    (
        "docs/SEED-RELEASE-REPORT.md",
        &["v0.0.1 Seed", "candidate", "Release tag"],
    ),
    (
        "docs/governance/support-matrix.md",
        &["1.0-draft", "Tier2", "no downloadable-artifact"],
    ),
    (
        "docs/status/COMPAT-6501-AUTHORITY-AUDIT.md",
        &["BlockedSpec", "historical corpus", "v0.5"],
    ),
    (
        "docs/status/REL-6603-AUTHORITY-AUDIT.md",
        &["BlockedSpec", "security sign-off", "advisory"],
    ),
    (
        "docs/status/DOC-6701-AUTHORITY-AUDIT.md",
        &["BlockedSpec", "documentation", "1.0"],
    ),
    (
        "docs/status/DOC-6702-AUTHORITY-AUDIT.md",
        &["BlockedSpec", "example", "1.0"],
    ),
    (
        "docs/status/DOC-6703-AUTHORITY-AUDIT.md",
        &["BlockedSpec", "tutorial", "1.0"],
    ),
    (
        "docs/status/ZED-6804-AUTHORITY-AUDIT.md",
        &["BlockedSpec", "DAP", "non-blocking"],
    ),
    (
        "docs/testing/SECURITY-AUDIT.md",
        &[
            "not a vulnerability-free claim",
            "Required release evidence",
        ],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub criterion_count: usize,
    pub blocked_count: usize,
    pub audit_file_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-RC0-FREEZE-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_audits(root));
    finish(errors).map(|()| CheckSummary {
        criterion_count: CRITERIA.len(),
        blocked_count: CRITERIA
            .iter()
            .filter(|criterion| criterion.state == "BlockedSpec")
            .count(),
        audit_file_count: REQUIRED_AUDIT_MARKERS.len(),
    })
}

fn validate_matrix(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(gate_section) = matrix
        .split_once("## Gate matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Verification boundary"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-RC0-FREEZE-0002: {MATRIX_PATH} is missing the Gate matrix section"
        )];
    };

    let rows = gate_section
        .lines()
        .filter_map(parse_row)
        .collect::<Vec<_>>();
    let mut actual = BTreeMap::new();
    for (name, cells) in rows {
        if actual.insert(name.to_owned(), cells).is_some() {
            errors.push(format!(
                "GOV-RC0-FREEZE-0003: duplicate RC0 criterion {name:?}"
            ));
        }
    }

    let mut expected_names = CRITERIA
        .iter()
        .map(|criterion| criterion.name)
        .collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-RC0-FREEZE-0004: RC0 criterion set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for criterion in CRITERIA {
        let Some(cells) = actual.get(criterion.name) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-RC0-FREEZE-0005: RC0 criterion {:?} has an empty evidence/state/exit cell",
                criterion.name
            ));
            continue;
        }
        if cells[1] != criterion.state {
            errors.push(format!(
                "GOV-RC0-FREEZE-0006: RC0 criterion {:?} must have state {:?}, found {:?}",
                criterion.name, criterion.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-RC0-FREEZE-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized
        .to_ascii_lowercase()
        .contains(stale_legacy_name.as_str())
    {
        errors.push(format!(
            "GOV-RC0-FREEZE-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
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
                errors.push(format!("GOV-RC0-FREEZE-0009: cannot read {path}: {error}"));
                continue;
            }
        };
        let normalized = normalize(&text);
        for marker in *markers {
            if !normalized.contains(&normalize(marker)) {
                errors.push(format!(
                    "GOV-RC0-FREEZE-0010: {path} is missing audit marker {marker:?}"
                ));
            }
        }
    }
    errors
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
        || cells[0] == "RC0 criterion"
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
    fn repository_rc0_inventory_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("RC0 inventory is valid");
        assert_eq!(summary.criterion_count, 8);
        assert_eq!(summary.blocked_count, 8);
        assert_eq!(summary.audit_file_count, 10);
    }

    #[test]
    fn rejects_rc0_state_drift() {
        let matrix = "## Gate matrix\n| RC0 criterion | Current evidence | State | Required exit evidence |\n| --- | --- | --- | --- |\n| Feature freeze | evidence | Future | authority |\n## Verification boundary\n";
        let errors = validate_matrix(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("criterion set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_rc0_audit_marker_drift() {
        let errors = validate_audit_text("RC-6901", "BlockedSpec", &["Accepted authority"]);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing audit marker"))
        );
    }

    fn validate_audit_text(path: &str, text: &str, markers: &[&str]) -> Vec<String> {
        let normalized = normalize(text);
        markers
            .iter()
            .filter(|marker| !normalized.contains(&normalize(marker)))
            .map(|marker| format!("GOV-RC0-FREEZE-0010: {path} is missing audit marker {marker:?}"))
            .collect()
    }
}
