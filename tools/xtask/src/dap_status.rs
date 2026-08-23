use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/DAP-STATUS.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DapSpec {
    surface: &'static str,
    state: &'static str,
}

const DAP_SURFACES: &[DapSpec] = &[
    DapSpec {
        surface: "Adapter process and stdio framing",
        state: "Unavailable",
    },
    DapSpec {
        surface: "Zed debugger registration",
        state: "Unavailable",
    },
    DapSpec {
        surface: "Launch/attach/session",
        state: "Future",
    },
    DapSpec {
        surface: "Breakpoints/continue/step",
        state: "Future",
    },
    DapSpec {
        surface: "Stack/scopes/variables",
        state: "Future",
    },
    DapSpec {
        surface: "Fault/stop/exit",
        state: "Partial foundation only",
    },
    DapSpec {
        surface: "Task/Actor views",
        state: "Unsupported",
    },
    DapSpec {
        surface: "Protocol inventory and compatibility",
        state: "Unavailable",
    },
    DapSpec {
        surface: "Security and platform support",
        state: "Unavailable",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "does not implement or register a Debug Adapter Protocol (DAP) server.",
    "DAP is not a 1.0 blocker",
    "Unavailable / Future",
    "not a usable debugger",
    "Runtime Fault and VM control evidence is not a DAP protocol",
    "No DAP process, network request, extension registration, debugger button, or system configuration was exercised by this audit.",
    "No placeholder command, protocol, backend, schema, migration promise, or stale legacy name is added here.",
    "must not block language checking, running, grammar-only editing",
];

const REQUIRED_AUDIT_MARKERS: &[(&str, &[&str])] = &[
    (
        "docs/status/DAP-3601-AUTHORITY-AUDIT.md",
        &["BlockedSpec", "No DAP adapter", "Accepted debugger RFC"],
    ),
    (
        "docs/status/DAP-3602-AUTHORITY-AUDIT.md",
        &["BlockedSpec", "No Zed extension directory", "DAP adapter"],
    ),
    (
        "docs/status/DAP-3603-AUTHORITY-AUDIT.md",
        &[
            "BlockedSpec",
            "The workspace has no DAP adapter",
            "debugger capability",
            "DAP-3602",
        ],
    ),
];

const REQUIRED_OBSERVATION_MARKERS: &[(&str, &[&str])] = &[
    (
        "crates/ling-types/tests/dap_debugger_boundary_evidence.rs",
        &[
            "const ALL: [Self; 60]",
            "ling.dap-debugger-observation/0",
            "proposed_dap_boundaries_are_complete_and_ordered",
            "dap_evidence_is_order_independent_and_duplicate_checked",
            "dap_evidence_has_no_debugger_protocol_authority",
        ],
    ),
    (
        "crates/ling-types/tests/zed_debugger_registration_evidence.rs",
        &[
            "const ALL: [Self; 60]",
            "ling.zed-debugger-observation/0",
            "proposed_zed_debugger_boundaries_are_complete_and_ordered",
            "zed_debugger_evidence_is_order_independent_and_duplicate_checked",
            "zed_debugger_evidence_has_no_extension_or_protocol_authority",
        ],
    ),
    (
        "crates/ling-types/tests/staged_debugger_capability_evidence.rs",
        &[
            "const ALL: [Self; 60]",
            "ling.staged-debugger-observation/0",
            "proposed_debugger_capabilities_are_complete_and_ordered",
            "staged_debugger_evidence_is_order_independent_and_duplicate_checked",
            "staged_debugger_evidence_has_no_capability_authority",
        ],
    ),
    (
        "docs/status/DAP-3601-OBSERVATION-IMPLEMENTATION-REPORT.md",
        &[
            "Accepted `DEC-0144`",
            "sixty proposed DAP/debugger boundaries",
            "no debugger protocol or public API authority",
            "Public `DAP-3601` remains `BlockedSpec`",
        ],
    ),
    (
        "docs/status/DAP-3602-OBSERVATION-IMPLEMENTATION-REPORT.md",
        &[
            "Accepted `DEC-0145`",
            "sixty proposed Zed debugger-registration boundaries",
            "no extension, debugger, or protocol authority",
            "Public `DAP-3602` remains `BlockedSpec`",
        ],
    ),
    (
        "docs/status/DAP-3603-OBSERVATION-IMPLEMENTATION-REPORT.md",
        &[
            "Accepted `DEC-0146`",
            "sixty proposed staged-debugger boundaries",
            "no debugger capability or protocol authority",
            "Public `DAP-3603` remains `BlockedSpec`",
        ],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub surface_count: usize,
    pub unavailable_count: usize,
    pub future_count: usize,
    pub partial_count: usize,
    pub unsupported_count: usize,
    pub audit_file_count: usize,
    pub observation_file_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-DAP-STATUS-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_audits(root));
    errors.extend(validate_observations(root));
    finish(errors).map(|()| CheckSummary {
        surface_count: DAP_SURFACES.len(),
        unavailable_count: count_state("Unavailable"),
        future_count: count_state("Future"),
        partial_count: count_state("Partial foundation only"),
        unsupported_count: count_state("Unsupported"),
        audit_file_count: REQUIRED_AUDIT_MARKERS.len(),
        observation_file_count: REQUIRED_OBSERVATION_MARKERS.len(),
    })
}

fn validate_matrix(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(matrix_section) = matrix
        .split_once("## Current matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Evidence and verification"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-DAP-STATUS-0002: {MATRIX_PATH} is missing the Current matrix section"
        )];
    };

    let rows = matrix_section
        .lines()
        .filter_map(parse_row)
        .collect::<Vec<_>>();
    let mut actual = BTreeMap::new();
    for (surface, cells) in rows {
        if actual
            .insert(surface.to_owned(), cells.to_owned())
            .is_some()
        {
            errors.push(format!(
                "GOV-DAP-STATUS-0003: duplicate DAP surface {surface:?}"
            ));
        }
    }

    let mut expected_names = DAP_SURFACES
        .iter()
        .map(|surface| surface.surface)
        .collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-DAP-STATUS-0004: DAP surface set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for surface in DAP_SURFACES {
        let Some(cells) = actual.get(surface.surface) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-DAP-STATUS-0005: DAP surface {:?} has an empty evidence/state/authority cell",
                surface.surface
            ));
            continue;
        }
        if cells[1] != surface.state {
            errors.push(format!(
                "GOV-DAP-STATUS-0006: DAP surface {:?} must have state {:?}, found {:?}",
                surface.surface, surface.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-DAP-STATUS-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized
        .to_ascii_lowercase()
        .contains(stale_legacy_name.as_str())
    {
        errors.push(format!(
            "GOV-DAP-STATUS-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
        ));
    }
    errors
}

fn validate_audits(root: &Path) -> Vec<String> {
    validate_marker_files(root, REQUIRED_AUDIT_MARKERS)
}

fn validate_observations(root: &Path) -> Vec<String> {
    validate_marker_files(root, REQUIRED_OBSERVATION_MARKERS)
}

fn validate_marker_files(root: &Path, evidence: &[(&str, &[&str])]) -> Vec<String> {
    let mut errors = Vec::new();
    for (path, markers) in evidence {
        let text = match fs::read_to_string(root.join(path)) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!("GOV-DAP-STATUS-0009: cannot read {path}: {error}"));
                continue;
            }
        };
        errors.extend(validate_marker_text(path, &text, markers));
    }
    errors
}

fn validate_marker_text(path: &str, text: &str, markers: &[&str]) -> Vec<String> {
    let normalized = normalize(text);
    markers
        .iter()
        .filter(|marker| !normalized.contains(&normalize(marker)))
        .map(|marker| format!("GOV-DAP-STATUS-0010: {path} is missing evidence marker {marker:?}"))
        .collect()
}

fn count_state(state: &str) -> usize {
    DAP_SURFACES
        .iter()
        .filter(|surface| surface.state == state)
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
        || cells[0] == "DAP surface"
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
    fn repository_dap_status_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("DAP status inventory is valid");
        assert_eq!(summary.surface_count, 9);
        assert_eq!(summary.unavailable_count, 4);
        assert_eq!(summary.future_count, 3);
        assert_eq!(summary.partial_count, 1);
        assert_eq!(summary.unsupported_count, 1);
        assert_eq!(summary.audit_file_count, 3);
        assert_eq!(summary.observation_file_count, 6);
    }

    #[test]
    fn rejects_dap_state_drift() {
        let matrix = "## Current matrix\n| DAP surface | Repository evidence | Current state | Required before Preview |\n| --- | --- | --- | --- |\n| Adapter process and stdio framing | evidence | Future | authority |\n## Evidence and verification\n";
        let errors = validate_matrix(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("DAP surface set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_dap_audit_marker_drift() {
        let errors = validate_marker_text("DAP-3601", "BlockedSpec", &["No DAP adapter"]);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing evidence marker"))
        );
    }

    #[test]
    fn rejects_observation_marker_drift() {
        let errors = validate_marker_text(
            "dap_debugger_boundary_evidence.rs",
            "ling.dap-debugger-observation/0",
            &["const ALL: [Self; 60]", "no_debugger_protocol_authority"],
        );
        assert_eq!(errors.len(), 2);
    }
}
