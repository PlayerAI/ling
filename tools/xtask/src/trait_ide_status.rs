use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/TRAIT-IDE-STATUS.md";
const STATUS_PATH: &str = "docs/status/implementation-status.toml";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TraitIdeSurface {
    surface: &'static str,
    state: &'static str,
}

const TRAIT_IDE_SURFACES: &[TraitIdeSurface] = &[
    TraitIdeSurface {
        surface: "Trait Semantic Graph projection",
        state: "Experimental",
    },
    TraitIdeSurface {
        surface: "Projection identity lookups",
        state: "Internal",
    },
    TraitIdeSurface {
        surface: "Trait hover",
        state: "BlockedSpec",
    },
    TraitIdeSurface {
        surface: "Go to Trait or implementation",
        state: "BlockedSpec",
    },
    TraitIdeSurface {
        surface: "Trait completion",
        state: "BlockedSpec",
    },
    TraitIdeSurface {
        surface: "Identity-preserving Trait rename",
        state: "BlockedSpec",
    },
    TraitIdeSurface {
        surface: "Trait diagnostics and repairs",
        state: "BlockedSpec",
    },
    TraitIdeSurface {
        surface: "Trait LSP transactions and versions",
        state: "BlockedSpec",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "The full `TRAIT-1308` task remains `BlockedSpec`.",
    "must consume the checked immutable witness and must not re-run Trait selection.",
    "No hover, navigation request, completion, rename, diagnostic, repair, JSON-RPC method, Workspace Edit, or Semantic Transaction is implemented by this inventory.",
    "The verifier is deterministic, read-only, and offline.",
    "No language semantics, public diagnostic allocation, core schema, Semantic ID, runtime, bytecode, VM, ABI, or Unicode 17.0.0 behavior changes.",
    "Only `ling` and `.ling` are valid public names.",
];

const REQUIRED_EVIDENCE: &[(&str, &[&str])] = &[
    (
        "crates/ling-semantic/src/lib.rs",
        &[
            "pub struct SemanticTraitIdeProjection",
            "pub fn witnesses_by_trait_id",
            "pub fn witness_by_implementation_id",
            "pub fn members_by_trait_definition_id",
            "pub fn member_by_implementation_definition_id",
            "trait_ide_projection_preserves_selected_ids_and_original_spans",
            "trait_ide_projection_lookups_are_read_only_and_projection_ordered",
            "trait_ide_projection_rejects_bad_extension_version_and_spans",
        ],
    ),
    (
        "docs/RFC-0022.md",
        &[
            "x-ling-trait-ide",
            "The extension is observational only.",
            "MUST never re-run coherence or solver selection",
        ],
    ),
    (
        "docs/decisions/0059-trait-ide-projection-lookups.md",
        &[
            "read-only lookup helpers",
            "never mutate, revalidate, normalize, reselect, or synthesize",
            "in-process Rust APIs only",
        ],
    ),
    (
        "docs/status/TRAIT-1308-PROJECTION-IMPLEMENTATION-REPORT.md",
        &[
            "Status: Done (bounded projection slice).",
            "parent remains `BlockedSpec`",
            "never re-runs Trait selection",
        ],
    ),
    (
        "docs/status/TRAIT-1308-QUERY-IMPLEMENTATION-REPORT.md",
        &[
            "Status: Done (bounded read-only lookup slice).",
            "parent `TRAIT-1308` task remains `BlockedSpec`",
            "does not claim hover, completion, rename, repairs, diagnostics",
        ],
    ),
    (
        "docs/status/TRAIT-1308-AUTHORITY-AUDIT.md",
        &[
            "`TRAIT-1308` remains `BlockedSpec`",
            "GAP-LSP-TRANSACTION-PROTOCOL-001",
            "GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001",
            "must not re-run Trait selection",
        ],
    ),
    (
        "docs/governance/protocol-inventory.toml",
        &[
            "id = \"PROTO-SEMANTIC-GRAPH-JSON\"",
            "stability = \"Experimental\"",
            "RFC-0022 defines the optional Experimental x-ling-trait-ide",
        ],
    ),
];

const REQUIRED_TASK_STATES: &[(&str, &str)] = &[
    ("TRAIT-1308", "BlockedSpec"),
    ("TRAIT-1308-PROJECTION", "Done"),
    ("TRAIT-1308-QUERY", "Done"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub surface_count: usize,
    pub experimental_count: usize,
    pub internal_count: usize,
    pub blocked_count: usize,
    pub evidence_file_count: usize,
    pub task_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-TRAIT-IDE-STATUS-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let status = fs::read_to_string(root.join(STATUS_PATH)).map_err(|error| {
        vec![format!(
            "GOV-TRAIT-IDE-STATUS-0001: cannot read {STATUS_PATH}: {error}"
        )]
    })?;

    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_evidence(root));
    errors.extend(validate_task_states(&status));
    finish(errors).map(|()| CheckSummary {
        surface_count: TRAIT_IDE_SURFACES.len(),
        experimental_count: count_state("Experimental"),
        internal_count: count_state("Internal"),
        blocked_count: count_state("BlockedSpec"),
        evidence_file_count: REQUIRED_EVIDENCE.len(),
        task_count: REQUIRED_TASK_STATES.len(),
    })
}

fn validate_matrix(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(matrix_section) = matrix
        .split_once("## Current surface matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Evidence contract"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-TRAIT-IDE-STATUS-0002: {MATRIX_PATH} is missing the Current surface matrix section"
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
                "GOV-TRAIT-IDE-STATUS-0003: duplicate Trait IDE surface {surface:?}"
            ));
        }
    }
    let mut expected_names = TRAIT_IDE_SURFACES
        .iter()
        .map(|surface| surface.surface)
        .collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-TRAIT-IDE-STATUS-0004: Trait IDE surface set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for surface in TRAIT_IDE_SURFACES {
        let Some(cells) = actual.get(surface.surface) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-TRAIT-IDE-STATUS-0005: Trait IDE surface {:?} has an empty evidence/state/authority cell",
                surface.surface
            ));
            continue;
        }
        if cells[1] != surface.state {
            errors.push(format!(
                "GOV-TRAIT-IDE-STATUS-0006: Trait IDE surface {:?} must have state {:?}, found {:?}",
                surface.surface, surface.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-TRAIT-IDE-STATUS-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized
        .to_ascii_lowercase()
        .contains(stale_legacy_name.as_str())
    {
        errors.push(format!(
            "GOV-TRAIT-IDE-STATUS-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
        ));
    }
    errors
}

fn validate_evidence(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    for (path, markers) in REQUIRED_EVIDENCE {
        let text = match fs::read_to_string(root.join(path)) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!(
                    "GOV-TRAIT-IDE-STATUS-0009: cannot read {path}: {error}"
                ));
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
        .map(|marker| {
            format!("GOV-TRAIT-IDE-STATUS-0010: {path} is missing evidence marker {marker:?}")
        })
        .collect()
}

fn validate_task_states(status: &str) -> Vec<String> {
    let parsed = match toml::from_str::<toml::Value>(status) {
        Ok(parsed) => parsed,
        Err(error) => {
            return vec![format!(
                "GOV-TRAIT-IDE-STATUS-0011: cannot parse {STATUS_PATH}: {error}"
            )];
        }
    };
    let mut states = BTreeMap::new();
    if let Some(tasks) = parsed.get("task").and_then(toml::Value::as_array) {
        for task in tasks {
            let Some(id) = task.get("id").and_then(toml::Value::as_str) else {
                continue;
            };
            let Some(state) = task.get("state").and_then(toml::Value::as_str) else {
                continue;
            };
            states.insert(id, state);
        }
    }
    REQUIRED_TASK_STATES
        .iter()
        .filter_map(|(id, expected)| {
            let actual = states.get(id).copied();
            (actual != Some(*expected)).then(|| {
                format!("GOV-TRAIT-IDE-STATUS-0012: task {id} must be {expected}, found {actual:?}")
            })
        })
        .collect()
}

fn count_state(state: &str) -> usize {
    TRAIT_IDE_SURFACES
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
        || cells[0] == "Trait IDE surface"
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
    fn repository_trait_ide_status_is_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("Trait IDE status inventory is valid");
        assert_eq!(summary.surface_count, 8);
        assert_eq!(summary.experimental_count, 1);
        assert_eq!(summary.internal_count, 1);
        assert_eq!(summary.blocked_count, 6);
        assert_eq!(summary.evidence_file_count, 7);
        assert_eq!(summary.task_count, 3);
    }

    #[test]
    fn rejects_surface_state_drift() {
        let matrix = valid_matrix().replace(
            "| Trait hover | Current evidence | BlockedSpec | Authority |",
            "| Trait hover | Current evidence | Stable | Authority |",
        );
        let errors = validate_matrix(&matrix);
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_parent_promotion_and_missing_child() {
        let status = r#"
[[task]]
id = "TRAIT-1308"
state = "Done"

[[task]]
id = "TRAIT-1308-PROJECTION"
state = "Done"
"#;
        let errors = validate_task_states(status);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("TRAIT-1308 must be BlockedSpec"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("TRAIT-1308-QUERY"))
        );
    }

    #[test]
    fn rejects_missing_evidence_marker() {
        let errors = validate_marker_text(
            "fixture.rs",
            "pub struct SemanticTraitIdeProjection;",
            &["SemanticTraitIdeProjection", "witnesses_by_trait_id"],
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("witnesses_by_trait_id"));
    }

    fn valid_matrix() -> String {
        let rows = TRAIT_IDE_SURFACES
            .iter()
            .map(|surface| {
                format!(
                    "| {} | Current evidence | {} | Authority |",
                    surface.surface, surface.state
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "## Current surface matrix\n| Trait IDE surface | Current evidence | State | Authority / blocker |\n| --- | --- | --- | --- |\n{rows}\n## Evidence contract\n{}",
            REQUIRED_POLICY_PHRASES.join("\n")
        )
    }
}
