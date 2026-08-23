use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/TRAIT-PERFORMANCE-STATUS.md";
const STATUS_PATH: &str = "docs/status/implementation-status.toml";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Surface {
    name: &'static str,
    state: &'static str,
}

const SURFACES: &[Surface] = &[
    Surface {
        name: "Active-obligation cycle rejection",
        state: "Internal",
    },
    Surface {
        name: "Nested-obligation depth limit",
        state: "Internal",
    },
    Surface {
        name: "Source-evidence-independent selection",
        state: "Internal",
    },
    Surface {
        name: "Production HIR and Typed Core integration",
        state: "BlockedSpec",
    },
    Surface {
        name: "Deterministic solver work budget",
        state: "BlockedSpec",
    },
    Surface {
        name: "Trait benchmark corpus and thresholds",
        state: "BlockedSpec",
    },
    Surface {
        name: "LSP cancellation and stale results",
        state: "BlockedSpec",
    },
    Surface {
        name: "Public benchmark evidence protocol",
        state: "BlockedSpec",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "The full `TRAIT-1309` task remains `BlockedSpec`.",
    "The accepted semantic nesting limit remains exactly 64.",
    "Host wall-clock time is not Ling language semantics.",
    "No benchmark command, timing threshold, allocation or candidate budget, cancellation API, diagnostic, schema, or public Trait service is implemented by this inventory.",
    "The verifier is deterministic, read-only, and offline.",
    "No language semantics, public diagnostic allocation, schema, Semantic ID, runtime, bytecode, VM, ABI, or Unicode 17.0.0 behavior changes.",
    "Only `ling` and `.ling` are valid public names.",
];

const REQUIRED_EVIDENCE: &[(&str, &[&str])] = &[
    (
        "crates/ling-types/src/solver.rs",
        &[
            "const MAX_NESTED_OBLIGATIONS: usize = 64;",
            "active: BTreeSet::new()",
            "SolverErrorKind::Cycle",
            "SolverErrorKind::DepthLimit",
            "rejects_active_cycles_and_the_bounded_depth_limit",
            "bounded_termination_projection_ignores_source_evidence",
        ],
    ),
    (
        "docs/RFC-0005.md",
        &[
            "A repeated tuple is a cycle and MUST be rejected",
            "64 nested obligations",
            "bounded static failure",
        ],
    ),
    (
        "docs/decisions/0026-trait-solver-v0-boundary.md",
        &[
            "Repeated active keys are rejected as `Cycle`",
            "rejected as `DepthLimit`",
            "module remains crate-private",
        ],
    ),
    (
        "docs/decisions/0068-trait-termination-corpus.md",
        &[
            "source-evidence independence",
            "MUST preserve the RFC-0005 64-level nesting limit",
            "No benchmark output schema, timing threshold, LSP request",
        ],
    ),
    (
        "docs/status/TRAIT-1309-TERMINATION-AUTHORITY-AUDIT.md",
        &[
            "bounded child",
            "parent remains",
            "`BlockedSpec`",
            "No wall-clock measurement",
        ],
    ),
    (
        "docs/status/TRAIT-1309-TERMINATION-IMPLEMENTATION-REPORT.md",
        &[
            "Done (bounded termination-evidence child)",
            "source evidence does not affect selection",
            "Full TRAIT-1309 performance and LSP work remains deferred",
        ],
    ),
    (
        "docs/status/TRAIT-1309-AUTHORITY-AUDIT.md",
        &[
            "`TRAIT-1309` remains correctly recorded as `BlockedSpec`",
            "No benchmark command, public timing schema, wall-clock guarantee",
            "GAP-LSP-TRANSACTION-PROTOCOL-001",
        ],
    ),
];

const REQUIRED_TASK_STATES: &[(&str, &str)] = &[
    ("TRAIT-1309", "BlockedSpec"),
    ("TRAIT-1309-TERMINATION", "Done"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub surface_count: usize,
    pub internal_count: usize,
    pub blocked_count: usize,
    pub evidence_file_count: usize,
    pub task_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = read(root, MATRIX_PATH)?;
    let status = read(root, STATUS_PATH)?;
    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_evidence(root));
    errors.extend(validate_task_states(&status));
    finish(errors).map(|()| CheckSummary {
        surface_count: SURFACES.len(),
        internal_count: count_state("Internal"),
        blocked_count: count_state("BlockedSpec"),
        evidence_file_count: REQUIRED_EVIDENCE.len(),
        task_count: REQUIRED_TASK_STATES.len(),
    })
}

fn read(root: &Path, path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(root.join(path)).map_err(|error| {
        vec![format!(
            "GOV-TRAIT-PERFORMANCE-0001: cannot read {path}: {error}"
        )]
    })
}

fn validate_matrix(matrix: &str) -> Vec<String> {
    let Some(section) = matrix
        .split_once("## Current surface matrix")
        .and_then(|(_, tail)| tail.split_once("## Evidence contract"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-TRAIT-PERFORMANCE-0002: {MATRIX_PATH} is missing the Current surface matrix section"
        )];
    };
    let mut errors = Vec::new();
    let mut actual = BTreeMap::new();
    for (name, cells) in section.lines().filter_map(parse_row) {
        if actual.insert(name.to_owned(), cells.to_owned()).is_some() {
            errors.push(format!(
                "GOV-TRAIT-PERFORMANCE-0003: duplicate Trait performance surface {name:?}"
            ));
        }
    }
    let mut expected_names = SURFACES
        .iter()
        .map(|surface| surface.name)
        .collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-TRAIT-PERFORMANCE-0004: surface set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for surface in SURFACES {
        let Some(cells) = actual.get(surface.name) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-TRAIT-PERFORMANCE-0005: surface {:?} has an empty evidence/state/authority cell",
                surface.name
            ));
        } else if cells[1] != surface.state {
            errors.push(format!(
                "GOV-TRAIT-PERFORMANCE-0006: surface {:?} must have state {:?}, found {:?}",
                surface.name, surface.state, cells[1]
            ));
        }
    }
    let normalized = normalize(matrix);
    for phrase in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(phrase)) {
            errors.push(format!(
                "GOV-TRAIT-PERFORMANCE-0007: {MATRIX_PATH} is missing policy phrase {phrase:?}"
            ));
        }
    }
    let stale_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized.to_ascii_lowercase().contains(&stale_name) {
        errors.push(format!(
            "GOV-TRAIT-PERFORMANCE-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
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
                    "GOV-TRAIT-PERFORMANCE-0009: cannot read {path}: {error}"
                ));
                continue;
            }
        };
        errors.extend(validate_markers(path, &text, markers));
    }
    errors
}

fn validate_markers(path: &str, text: &str, markers: &[&str]) -> Vec<String> {
    let text = normalize(text);
    markers
        .iter()
        .filter(|marker| !text.contains(&normalize(marker)))
        .map(|marker| {
            format!("GOV-TRAIT-PERFORMANCE-0010: {path} is missing evidence marker {marker:?}")
        })
        .collect()
}

fn validate_task_states(status: &str) -> Vec<String> {
    let parsed = match toml::from_str::<toml::Value>(status) {
        Ok(parsed) => parsed,
        Err(error) => {
            return vec![format!(
                "GOV-TRAIT-PERFORMANCE-0011: cannot parse {STATUS_PATH}: {error}"
            )];
        }
    };
    let mut states = BTreeMap::new();
    if let Some(tasks) = parsed.get("task").and_then(toml::Value::as_array) {
        for task in tasks {
            if let (Some(id), Some(state)) = (
                task.get("id").and_then(toml::Value::as_str),
                task.get("state").and_then(toml::Value::as_str),
            ) {
                states.insert(id, state);
            }
        }
    }
    REQUIRED_TASK_STATES
        .iter()
        .filter_map(|(id, expected)| {
            let actual = states.get(id).copied();
            (actual != Some(*expected)).then(|| {
                format!(
                    "GOV-TRAIT-PERFORMANCE-0012: task {id} must be {expected}, found {actual:?}"
                )
            })
        })
        .collect()
}

fn count_state(state: &str) -> usize {
    SURFACES
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
        || cells[0] == "Trait performance surface"
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
    fn repository_trait_performance_status_is_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("Trait performance inventory is valid");
        assert_eq!(summary.surface_count, 8);
        assert_eq!(summary.internal_count, 3);
        assert_eq!(summary.blocked_count, 5);
        assert_eq!(summary.evidence_file_count, 7);
        assert_eq!(summary.task_count, 2);
    }

    #[test]
    fn rejects_surface_promotion() {
        let matrix = valid_matrix().replace(
            "| Deterministic solver work budget | Evidence | BlockedSpec | Authority |",
            "| Deterministic solver work budget | Evidence | Stable | Authority |",
        );
        assert!(
            validate_matrix(&matrix)
                .iter()
                .any(|error| error.contains("must have state"))
        );
    }

    #[test]
    fn rejects_parent_promotion_or_missing_child() {
        let status = "[[task]]\nid = \"TRAIT-1309\"\nstate = \"Done\"\n";
        let errors = validate_task_states(status);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("must be BlockedSpec"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("TRAIT-1309-TERMINATION"))
        );
    }

    #[test]
    fn rejects_missing_solver_marker() {
        let errors = validate_markers(
            "solver.rs",
            "const MAX_NESTED_OBLIGATIONS: usize = 64;",
            &["MAX_NESTED_OBLIGATIONS", "bounded_termination_projection"],
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("bounded_termination_projection"));
    }

    fn valid_matrix() -> String {
        let rows = SURFACES
            .iter()
            .map(|surface| {
                format!(
                    "| {} | Evidence | {} | Authority |",
                    surface.name, surface.state
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "## Current surface matrix\n| Trait performance surface | Current evidence | State | Authority / blocker |\n| --- | --- | --- | --- |\n{rows}\n## Evidence contract\n{}",
            REQUIRED_POLICY_PHRASES.join("\n")
        )
    }
}
