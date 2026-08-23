use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/ZED-EXTENSION-ACCEPTANCE.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AcceptanceSpec {
    area: &'static str,
    state: &'static str,
}

const ACCEPTANCE_AREAS: &[AcceptanceSpec] = &[
    AcceptanceSpec {
        area: ".ling recognition",
        state: "Covered for grammar-only parsing",
    },
    AcceptanceSpec {
        area: "Tree-sitter highlights",
        state: "Covered for query fixtures",
    },
    AcceptanceSpec {
        area: "Brackets",
        state: "Covered for query fixtures",
    },
    AcceptanceSpec {
        area: "Indentation",
        state: "Covered for query fixtures",
    },
    AcceptanceSpec {
        area: "Outline / textobjects / runnables",
        state: "Future / Unsupported",
    },
    AcceptanceSpec {
        area: "LSP diagnostics, hover, definition, references, rename, completion, code actions, format, semantic tokens",
        state: "Partial prerequisites; listed features Unsupported",
    },
    AcceptanceSpec {
        area: "Task / run / test / Audit",
        state: "Partial",
    },
    AcceptanceSpec {
        area: "Replay / evidence",
        state: "Unsupported",
    },
    AcceptanceSpec {
        area: "Chinese / emoji / CRLF / UTF-16 positions",
        state: "Partial",
    },
    AcceptanceSpec {
        area: "Language-server crash / restart",
        state: "Unsupported",
    },
    AcceptanceSpec {
        area: "Large file / workspace",
        state: "Partial / Future",
    },
    AcceptanceSpec {
        area: "Extension license / metadata / repository",
        state: "Partial",
    },
    AcceptanceSpec {
        area: "Development install / marketplace package",
        state: "Unsupported",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "It is not a claim that a Zed extension is packaged or ready for a marketplace.",
    "The Tree-sitter parser and queries are editor aids only.",
    "The support matrix records LSP document features, Zed extension integration, formatter editor transactions, and semantic mutation as unsupported.",
    "source-built Preview LSP lifecycle/full-text overlay",
    "Unicode 17.0.0",
    "original UTF-8 spans",
    "Windows `npm run verify --offline` suite passed",
    "No row may be promoted",
    "No row may be promoted by copying a planning checklist",
];

const REQUIRED_EVIDENCE_MARKERS: &[(&str, &[&str])] = &[
    (
        "editors/tree-sitter-ling/package.json",
        &[
            "\"name\": \"tree-sitter-ling\"",
            "\"version\": \"0.0.1-dev\"",
            "\"license\": \"Apache-2.0\"",
            "\"verify\": \"npm run generate",
        ],
    ),
    (
        "editors/tree-sitter-ling/package-lock.json",
        &["tree-sitter-cli", "0.26.12", "node", ">=20"],
    ),
    (
        "editors/tree-sitter-ling/tree-sitter.json",
        &[
            "\"scope\": \"source.ling\"",
            "\"file-types\"",
            "queries/highlights.scm",
        ],
    ),
    (
        "editors/tree-sitter-ling/README.md",
        &[
            "The grammar is not an authority",
            "Unicode 17.0.0",
            "npm test",
            "Generated parser sources under `src/` are committed",
        ],
    ),
    (
        "editors/tree-sitter-ling/KNOWN-DIFFERENCES.md",
        &[
            "None of these differences changes Ling syntax or semantics",
            "Unicode 17.0.0",
            "Tree-sitter node names",
        ],
    ),
    (
        "docs/status/TS-3108-IMPLEMENTATION-REPORT.md",
        &[
            "42",
            "npm test",
            "Tree-sitter editor parser",
            "original UTF-8",
        ],
    ),
    (
        "docs/status/ZQ-3201-IMPLEMENTATION-REPORT.md",
        &[
            "highlights.scm",
            "18 reviewed capture names",
            "Chinese",
            "query remains deliberately syntactic",
        ],
    ),
    (
        "docs/status/ZQ-3202-IMPLEMENTATION-REPORT.md",
        &[
            "brackets.scm",
            "20 positive/negative assertions",
            "nested block comments",
        ],
    ),
    (
        "docs/status/ZQ-3203-IMPLEMENTATION-REPORT.md",
        &[
            "indents.scm",
            "38 `@indent`",
            "pipeline continuation",
            "editor aid",
        ],
    ),
    (
        "docs/status/ZED-6801-CURRENT-EVIDENCE-IMPLEMENTATION-REPORT.md",
        &[
            "npm run verify --offline",
            "passed on Windows",
            "41 grammar cases",
            "42 conformance programs",
            "18 highlight captures",
            "4 bracket pairs",
            "15 indentation CST nodes",
            "Linux/macOS and Zed were not executed",
        ],
    ),
];

const POSITION_EVIDENCE_MARKERS: &[(&str, &[&str])] = &[
    (
        "crates/ling-source/src/position.rs",
        &[
            "SUPPORTED_POSITION_ENCODINGS",
            "PositionEncoding::Utf16",
            "rejects_inside_utf8_scalars_and_utf16_surrogate_pairs",
            "position_edit_preserves_leading_bom_and_crlf_bytes",
            "人物 😀",
        ],
    ),
    (
        "crates/ling-lsp/src/lib.rs",
        &[
            "positionEncodings",
            "positionEncoding",
            "PositionEncoding::Utf16",
            "textDocument/didOpen",
            "textDocument/didChange",
            "textDocument/didClose",
        ],
    ),
    (
        "crates/ling-lsp/tests/position_encoding.rs",
        &[
            "negotiation_selects_the_first_supported_client_label",
            "negotiation_uses_utf16_for_absent_or_empty_client_lists",
            "malformed_position_encoding_metadata_is_rejected_before_lifecycle_transition",
            "positionEncoding",
        ],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub acceptance_count: usize,
    pub covered_count: usize,
    pub partial_count: usize,
    pub unsupported_count: usize,
    pub future_count: usize,
    pub evidence_file_count: usize,
    pub upstream_gate_count: usize,
    pub position_evidence_file_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-ZED-ACCEPTANCE-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_evidence(root));
    if let Err(upstream_errors) = crate::zed_matrix::check_repository(root) {
        errors.extend(upstream_errors);
    }
    if let Err(upstream_errors) = crate::lsp_discovery::check_repository(root) {
        errors.extend(upstream_errors);
    }
    errors.extend(validate_position_evidence(root));
    finish(errors).map(|()| CheckSummary {
        acceptance_count: ACCEPTANCE_AREAS.len(),
        covered_count: count_states_containing("Covered"),
        partial_count: count_states_containing("Partial"),
        unsupported_count: count_states_containing("Unsupported"),
        future_count: count_states_containing("Future"),
        evidence_file_count: REQUIRED_EVIDENCE_MARKERS.len(),
        upstream_gate_count: 2,
        position_evidence_file_count: POSITION_EVIDENCE_MARKERS.len(),
    })
}

fn validate_matrix(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(acceptance_section) = matrix
        .split_once("## Acceptance matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Existing evidence commands"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-ZED-ACCEPTANCE-0002: {MATRIX_PATH} is missing the Acceptance matrix section"
        )];
    };

    let rows = acceptance_section
        .lines()
        .filter_map(parse_row)
        .collect::<Vec<_>>();
    let mut actual = BTreeMap::new();
    for (area, cells) in rows {
        if actual.insert(area.to_owned(), cells.to_owned()).is_some() {
            errors.push(format!(
                "GOV-ZED-ACCEPTANCE-0003: duplicate acceptance area {area:?}"
            ));
        }
    }

    let mut expected_names = ACCEPTANCE_AREAS
        .iter()
        .map(|area| area.area)
        .collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-ZED-ACCEPTANCE-0004: acceptance area set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }

    for area in ACCEPTANCE_AREAS {
        let Some(cells) = actual.get(area.area) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-ZED-ACCEPTANCE-0005: acceptance area {:?} has an empty evidence/state/boundary cell",
                area.area
            ));
            continue;
        }
        if cells[1] != area.state {
            errors.push(format!(
                "GOV-ZED-ACCEPTANCE-0006: acceptance area {:?} must have state {:?}, found {:?}",
                area.area, area.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-ZED-ACCEPTANCE-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized
        .to_ascii_lowercase()
        .contains(stale_legacy_name.as_str())
    {
        errors.push(format!(
            "GOV-ZED-ACCEPTANCE-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
        ));
    }
    errors
}

fn validate_evidence(root: &Path) -> Vec<String> {
    validate_evidence_files(root, REQUIRED_EVIDENCE_MARKERS)
}

fn validate_position_evidence(root: &Path) -> Vec<String> {
    validate_evidence_files(root, POSITION_EVIDENCE_MARKERS)
}

fn validate_evidence_files(root: &Path, evidence: &[(&str, &[&str])]) -> Vec<String> {
    let mut errors = Vec::new();
    for (path, markers) in evidence {
        let text = match fs::read_to_string(root.join(path)) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!(
                    "GOV-ZED-ACCEPTANCE-0009: cannot read {path}: {error}"
                ));
                continue;
            }
        };
        errors.extend(validate_evidence_text(path, &text, markers));
    }
    errors
}

fn validate_evidence_text(path: &str, text: &str, markers: &[&str]) -> Vec<String> {
    let normalized = normalize(text);
    markers
        .iter()
        .filter(|marker| !normalized.contains(&normalize(marker)))
        .map(|marker| {
            format!("GOV-ZED-ACCEPTANCE-0010: {path} is missing evidence marker {marker:?}")
        })
        .collect()
}

fn count_states_containing(fragment: &str) -> usize {
    ACCEPTANCE_AREAS
        .iter()
        .filter(|area| area.state.contains(fragment))
        .count()
}

fn parse_row(line: &str) -> Option<(String, Vec<String>)> {
    let cells = line
        .trim()
        .strip_prefix('|')?
        .strip_suffix('|')?
        .split('|')
        .map(|cell| cell.trim().trim_matches('`').replace('`', ""))
        .collect::<Vec<_>>();
    if cells.len() < 4
        || cells[0] == "Acceptance area"
        || cells[0].chars().all(|character| character == '-')
    {
        return None;
    }
    Some((cells[0].clone(), cells[1..].to_vec()))
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
    fn repository_zed_acceptance_inventory_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("Zed acceptance inventory is valid");
        assert_eq!(summary.acceptance_count, 13);
        assert_eq!(summary.covered_count, 4);
        assert_eq!(summary.partial_count, 5);
        assert_eq!(summary.unsupported_count, 5);
        assert_eq!(summary.future_count, 2);
        assert_eq!(summary.evidence_file_count, 10);
        assert_eq!(summary.upstream_gate_count, 2);
        assert_eq!(summary.position_evidence_file_count, 3);
    }

    #[test]
    fn rejects_acceptance_state_drift() {
        let matrix = "## Acceptance matrix\n| Acceptance area | Current evidence | State | Boundary / required next evidence |\n| --- | --- | --- | --- |\n| .ling recognition | evidence | Unsupported | boundary |\n## Existing evidence commands\n";
        let errors = validate_matrix(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("acceptance area set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_evidence_marker_drift() {
        let errors = validate_evidence_text(
            "package.json",
            "{ \"name\": \"tree-sitter-ling\" }",
            &["npm run verify"],
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing evidence marker"))
        );
    }

    #[test]
    fn rejects_position_evidence_marker_drift() {
        let errors = validate_evidence_text(
            "position.rs",
            "PositionEncoding::Utf16",
            &["utf16_surrogate_pairs", "crlf_bytes"],
        );
        assert_eq!(errors.len(), 2);
    }
}
