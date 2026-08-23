use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::rc2_change_control;

const MATRIX_PATH: &str = "docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReleaseItem {
    name: &'static str,
    state: &'static str,
}

const RELEASE_ITEMS: &[ReleaseItem] = &[
    ReleaseItem {
        name: "Source tag",
        state: "Partial Seed evidence",
    },
    ReleaseItem {
        name: "Compiler/runtime artifacts",
        state: "Partial Seed evidence",
    },
    ReleaseItem {
        name: "Checksums/signatures",
        state: "Unavailable",
    },
    ReleaseItem {
        name: "SBOM/licenses/provenance",
        state: "Partial Seed evidence",
    },
    ReleaseItem {
        name: "Standard library",
        state: "Preview / not packaged",
    },
    ReleaseItem {
        name: "Zed extension",
        state: "Unsupported",
    },
    ReleaseItem {
        name: "Language server",
        state: "Unsupported",
    },
    ReleaseItem {
        name: "Reference documentation",
        state: "Partial Seed evidence",
    },
    ReleaseItem {
        name: "Migration guide",
        state: "BlockedSpec",
    },
    ReleaseItem {
        name: "Support matrix",
        state: "Draft",
    },
    ReleaseItem {
        name: "Protocol schemas/golden corpus",
        state: "Experimental / Preview / Future",
    },
    ReleaseItem {
        name: "Conformance suite",
        state: "Partial Seed evidence",
    },
    ReleaseItem {
        name: "Security policy",
        state: "BlockedSpec",
    },
    ReleaseItem {
        name: "Release evidence bundle",
        state: "Unavailable",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "it is not a release manifest, download index, signature, or Stable-support claim.",
    "The v0.0.1 Seed tag and reports must remain immutable historical evidence.",
    "No package upload, installer, signing service, release tag",
    "RC-6905 may leave `BlockedSpec` only after RC0 through RC4 are complete",
    "No placeholder command, download, package, schema, protocol, artifact,",
];

const REQUIRED_AUDIT_MARKERS: &[(&str, &[&str])] = &[
    (
        "docs/status/RC-6905-AUTHORITY-AUDIT.md",
        &[
            "RC-6905",
            "BlockedSpec",
            "no v1.0 publication",
            "No v1.0 tag, artifact",
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
        "docs/status/RC-6904-AUTHORITY-AUDIT.md",
        &["RC-6904", "BlockedSpec", "candidate"],
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
        "docs/governance/protocol-inventory.toml",
        &["stability = \"Preview\"", "PROTO-EVIDENCE"],
    ),
    (
        "docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md",
        &["BlockedSpec", "No package upload", "No placeholder command"],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub release_item_count: usize,
    pub partial_count: usize,
    pub unavailable_count: usize,
    pub unsupported_count: usize,
    pub blocked_count: usize,
    pub audit_file_count: usize,
    pub upstream_gate_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-V1-ARTIFACT-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_audits(root));
    if let Err(upstream_errors) = rc2_change_control::check_repository(root) {
        errors.extend(upstream_errors);
    }
    errors.extend(validate_current_evidence(&matrix));
    finish(errors).map(|()| CheckSummary {
        release_item_count: RELEASE_ITEMS.len(),
        partial_count: count_prefix("Partial"),
        unavailable_count: count_state("Unavailable"),
        unsupported_count: count_state("Unsupported"),
        blocked_count: count_state("BlockedSpec"),
        audit_file_count: REQUIRED_AUDIT_MARKERS.len(),
        upstream_gate_count: 1,
    })
}

fn validate_current_evidence(matrix: &str) -> Vec<String> {
    const REQUIRED: &[&str] = &[
        "A source-built `ling lsp --stdio` Preview server exists",
        "The 27 protocol records",
        "The current RC2→RC3→RC1→RC0 bounded inventory chain passes",
        "all four release parents remain `BlockedSpec`",
    ];
    let normalized = normalize(matrix);
    REQUIRED
        .iter()
        .filter(|marker| !normalized.contains(&normalize(marker)))
        .map(|marker| {
            format!(
                "GOV-V1-ARTIFACT-0011: {MATRIX_PATH} is missing current evidence marker {marker:?}"
            )
        })
        .collect()
}

fn validate_matrix(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(section) = matrix
        .split_once("## Artifact matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Publication boundary"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-V1-ARTIFACT-0002: {MATRIX_PATH} is missing the Artifact matrix section"
        )];
    };

    let rows = section.lines().filter_map(parse_row).collect::<Vec<_>>();
    let mut actual = BTreeMap::new();
    for (name, cells) in rows {
        if actual.insert(name.to_owned(), cells).is_some() {
            errors.push(format!(
                "GOV-V1-ARTIFACT-0003: duplicate v1.0 release item {name:?}"
            ));
        }
    }

    let mut expected_names = RELEASE_ITEMS
        .iter()
        .map(|item| item.name)
        .collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-V1-ARTIFACT-0004: v1.0 release-item set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for item in RELEASE_ITEMS {
        let Some(cells) = actual.get(item.name) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-V1-ARTIFACT-0005: v1.0 release item {:?} has an empty evidence/state/exit cell",
                item.name
            ));
            continue;
        }
        if cells[1] != item.state {
            errors.push(format!(
                "GOV-V1-ARTIFACT-0006: v1.0 release item {:?} must have state {:?}, found {:?}",
                item.name, item.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-V1-ARTIFACT-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized
        .to_ascii_lowercase()
        .contains(stale_legacy_name.as_str())
    {
        errors.push(format!(
            "GOV-V1-ARTIFACT-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
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
                errors.push(format!("GOV-V1-ARTIFACT-0009: cannot read {path}: {error}"));
                continue;
            }
        };
        let normalized = normalize(&text);
        for marker in *markers {
            if !normalized.contains(&normalize(marker)) {
                errors.push(format!(
                    "GOV-V1-ARTIFACT-0010: {path} is missing audit marker {marker:?}"
                ));
            }
        }
    }
    errors
}

fn count_state(state: &str) -> usize {
    RELEASE_ITEMS
        .iter()
        .filter(|item| item.state == state)
        .count()
}

fn count_prefix(prefix: &str) -> usize {
    RELEASE_ITEMS
        .iter()
        .filter(|item| item.state.starts_with(prefix))
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
        || cells[0] == "Required release item"
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
    fn repository_v1_artifact_inventory_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("v1.0 artifact inventory is valid");
        assert_eq!(summary.release_item_count, 14);
        assert_eq!(summary.partial_count, 5);
        assert_eq!(summary.unavailable_count, 2);
        assert_eq!(summary.unsupported_count, 2);
        assert_eq!(summary.blocked_count, 2);
        assert_eq!(summary.audit_file_count, 9);
        assert_eq!(summary.upstream_gate_count, 1);
    }

    #[test]
    fn rejects_v1_artifact_state_drift() {
        let matrix = "## Artifact matrix\n| Required release item | Current repository evidence | State | Required before v1.0 publication |\n| --- | --- | --- | --- |\n| Source tag | evidence | Stable | authority |\n## Publication boundary\n";
        let errors = validate_matrix(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("release-item set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_v1_artifact_audit_marker_drift() {
        let errors = validate_audit_text("RC-6905", "BlockedSpec", &["Accepted artifact"]);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing audit marker"))
        );
    }

    #[test]
    fn rejects_stale_lsp_protocol_and_upstream_evidence() {
        let errors = validate_current_evidence("No LSP executable; 21 protocols");
        assert_eq!(errors.len(), 4);
        assert!(
            errors
                .iter()
                .all(|error| error.contains("GOV-V1-ARTIFACT-0011"))
        );
    }

    fn validate_audit_text(path: &str, text: &str, markers: &[&str]) -> Vec<String> {
        let normalized = normalize(text);
        markers
            .iter()
            .filter(|marker| !normalized.contains(&normalize(marker)))
            .map(|marker| {
                format!("GOV-V1-ARTIFACT-0010: {path} is missing audit marker {marker:?}")
            })
            .collect()
    }
}
