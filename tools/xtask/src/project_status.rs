use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/PROJECT-CLI-STATUS.md";
const STATUS_PATH: &str = "docs/status/implementation-status.toml";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProjectSurface {
    surface: &'static str,
    state: &'static str,
}

const PROJECT_SURFACES: &[ProjectSurface] = &[
    ProjectSurface {
        surface: "Explicit locked graph check CLI",
        state: "Experimental",
    },
    ProjectSurface {
        surface: "Read-only locked project snapshot",
        state: "Internal",
    },
    ProjectSurface {
        surface: "Locked project semantic snapshot",
        state: "Internal",
    },
    ProjectSurface {
        surface: "Public semantic project check",
        state: "Experimental",
    },
    ProjectSurface {
        surface: "Project run",
        state: "Experimental",
    },
    ProjectSurface {
        surface: "Project test",
        state: "Experimental",
    },
    ProjectSurface {
        surface: "Project build and artifacts",
        state: "Experimental",
    },
    ProjectSurface {
        surface: "Workspace and member selection",
        state: "Experimental",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "The full `PRJ-1107` task is `Done` for its accepted v0.1 scope.",
    "The semantic commands reuse `load_locked_project` and `CompilerDb::project_semantic_snapshot`",
    "The checked semantic artifact is not executable bytecode, native/Wasm output, a publication package, or a Stable 1.0 format.",
    "The complete v0.1 workspace rule is explicit single-root selection",
    "The verifier is deterministic, read-only, and offline.",
    "`L-IO-0005` is the only new diagnostic allocation.",
    "Only `ling` and `.ling` are valid public names.",
];

const REQUIRED_EVIDENCE: &[(&str, &[&str])] = &[
    (
        "crates/ling-cli/src/command_catalog.rs",
        &["ProjectCheck", "Self::ProjectCheck => \"project check\""],
    ),
    (
        "crates/ling-cli/src/main.rs",
        &[
            "Some(\"check\") => (Command::ProjectCheck",
            "fn execute_project_check",
            "resolve_package_graph_with_lock(project_root, &manifest, LockMode::Locked)",
            "\"protocol\": \"ling.project.check/0.1\"",
        ],
    ),
    (
        "crates/ling-cli/tests/project_check.rs",
        &[
            "locked_project_check_emits_path_free_deterministic_json",
            "missing_lock_is_a_validation_error_without_mutating_project_files",
            "project_check_rejects_missing_locked_or_positional_inputs",
        ],
    ),
    (
        "tests/protocols/project-check/README.md",
        &[
            "ling.project.check/0.1",
            "ling project check --manifest-path <path> --locked",
        ],
    ),
    (
        "crates/ling-project/src/workspace.rs",
        &[
            "pub struct LockedProject",
            "pub fn load_locked_project",
            "LockMode::Locked",
            "does not create or",
            "rewrite a lock file",
        ],
    ),
    (
        "crates/ling-project/tests/locked_project.rs",
        &[
            "locked_project_snapshot_is_repeatable_and_path_free",
            "locked_project_load_does_not_rewrite_the_lock",
            "to_canonical_bytes()",
        ],
    ),
    (
        "crates/ling-db/src/project_snapshot.rs",
        &[
            "accepts only the immutable `LockedProject` boundary",
            "pub fn build(project: &LockedProject)",
            "ling_semantic::build_project(checked)",
            "format!(\"package:{}/{}\"",
        ],
    ),
    (
        "crates/ling-db/src/lib.rs",
        &[
            "pub fn project_semantic_snapshot",
            "project_semantic_snapshots",
            "project_semantic_snapshot_is_locked_path_free_and_cached",
            "Arc::ptr_eq(&first, &repeated)",
        ],
    ),
    (
        "docs/status/PRJ-1107-CHECK-IMPLEMENTATION-REPORT.md",
        &[
            "RFC-0024 Preview slice",
            "parent PRJ-1107 task stays",
            "BlockedSpec",
        ],
    ),
    (
        "docs/status/PRJ-1107-LOAD-IMPLEMENTATION-REPORT.md",
        &[
            "read-only `LockedProject` snapshot boundary",
            "parent `PRJ-1107` remains `BlockedSpec`",
        ],
    ),
    (
        "docs/status/PRJ-1107-SEMANTIC-SNAPSHOT-IMPLEMENTATION-REPORT.md",
        &[
            "Accepted DEC-0083",
            "CompilerDb::project_semantic_snapshot",
            "parent `PRJ-1107` remains `BlockedSpec`",
        ],
    ),
    (
        "docs/status/PRJ-1107-AUTHORITY-AUDIT.md",
        &[
            "PRJ-1107 is `Done`",
            "Accepted RFC-0025",
            "explicit single-root workspace selection",
            "checked semantic artifact",
        ],
    ),
    (
        "docs/RFC-0025.md",
        &[
            "ling.project.command/0.1",
            "ling.project.artifact/0.1",
            "CompilerDb::project_semantic_snapshot",
        ],
    ),
    (
        "crates/ling-cli/src/project.rs",
        &[
            "pub fn compile(manifest_path: &Path)",
            "load_locked_project",
            "project_semantic_snapshot",
            "pub fn build",
            "OpenOptions::new()",
        ],
    ),
    (
        "crates/ling-cli/tests/project_commands.rs",
        &[
            "semantic_check_is_path_free_repeatable_and_does_not_mutate_inputs",
            "run_and_test_consume_the_checked_root_entry",
            "build_publishes_exact_canonical_artifact_once",
            "semantic_source_failure_uses_bilingual_registered_diagnostics",
        ],
    ),
    (
        "docs/status/PRJ-1107-IMPLEMENTATION-REPORT.md",
        &[
            "Status: **Done**",
            "ling.project.command/0.1",
            "ling.project.artifact/0.1",
            "Intentionally deferred",
        ],
    ),
];

const REQUIRED_TASK_STATES: &[(&str, &str)] = &[
    ("PRJ-1107", "Done"),
    ("PRJ-1107-CHECK", "Done"),
    ("PRJ-1107-LOAD", "Done"),
    ("PRJ-1107-SEMANTIC-SNAPSHOT", "Done"),
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
            "GOV-PROJECT-STATUS-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let status = fs::read_to_string(root.join(STATUS_PATH)).map_err(|error| {
        vec![format!(
            "GOV-PROJECT-STATUS-0001: cannot read {STATUS_PATH}: {error}"
        )]
    })?;

    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_evidence(root));
    errors.extend(validate_task_states(&status));

    finish(errors).map(|()| CheckSummary {
        surface_count: PROJECT_SURFACES.len(),
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
            "GOV-PROJECT-STATUS-0002: {MATRIX_PATH} is missing the Current surface matrix section"
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
                "GOV-PROJECT-STATUS-0003: duplicate project surface {surface:?}"
            ));
        }
    }

    let mut expected_names = PROJECT_SURFACES
        .iter()
        .map(|surface| surface.surface)
        .collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-PROJECT-STATUS-0004: project surface set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }

    for surface in PROJECT_SURFACES {
        let Some(cells) = actual.get(surface.surface) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-PROJECT-STATUS-0005: project surface {:?} has an empty evidence/state/authority cell",
                surface.surface
            ));
            continue;
        }
        if cells[1] != surface.state {
            errors.push(format!(
                "GOV-PROJECT-STATUS-0006: project surface {:?} must have state {:?}, found {:?}",
                surface.surface, surface.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-PROJECT-STATUS-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized
        .to_ascii_lowercase()
        .contains(stale_legacy_name.as_str())
    {
        errors.push(format!(
            "GOV-PROJECT-STATUS-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
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
                    "GOV-PROJECT-STATUS-0009: cannot read {path}: {error}"
                ));
                continue;
            }
        };
        let normalized = normalize(&text);
        for marker in *markers {
            if !normalized.contains(&normalize(marker)) {
                errors.push(format!(
                    "GOV-PROJECT-STATUS-0010: {path} is missing evidence marker {marker:?}"
                ));
            }
        }
    }
    errors
}

fn validate_task_states(status: &str) -> Vec<String> {
    let parsed = match toml::from_str::<toml::Value>(status) {
        Ok(parsed) => parsed,
        Err(error) => {
            return vec![format!(
                "GOV-PROJECT-STATUS-0011: cannot parse {STATUS_PATH}: {error}"
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
                format!("GOV-PROJECT-STATUS-0012: task {id} must be {expected}, found {actual:?}")
            })
        })
        .collect()
}

fn count_state(state: &str) -> usize {
    PROJECT_SURFACES
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
        || cells[0] == "Project surface"
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
    fn repository_project_status_is_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("project status inventory is valid");
        assert_eq!(summary.surface_count, 8);
        assert_eq!(summary.experimental_count, 6);
        assert_eq!(summary.internal_count, 2);
        assert_eq!(summary.blocked_count, 0);
        assert_eq!(summary.evidence_file_count, 16);
        assert_eq!(summary.task_count, 4);
    }

    #[test]
    fn rejects_surface_state_drift() {
        let matrix = valid_matrix().replace(
            "| Project run | No command | Experimental | Missing authority |",
            "| Project run | No command | Stable | Missing authority |",
        );
        let errors = validate_matrix(&matrix);
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_missing_or_promoted_task_states() {
        let status = r#"
[[task]]
id = "PRJ-1107"
state = "BlockedSpec"

[[task]]
id = "PRJ-1107-CHECK"
state = "Done"
"#;
        let errors = validate_task_states(status);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("PRJ-1107 must be Done"))
        );
        assert!(errors.iter().any(|error| error.contains("PRJ-1107-LOAD")));
    }

    #[test]
    fn rejects_missing_evidence_marker() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let source = fs::read_to_string(root.join("crates/ling-project/src/workspace.rs"))
            .expect("workspace source is readable");
        let markers = ["pub struct LockedProject", "not-present-evidence-marker"];
        let normalized = normalize(&source);
        let missing = markers
            .iter()
            .filter(|marker| !normalized.contains(&normalize(marker)))
            .collect::<Vec<_>>();
        assert_eq!(missing, vec![&"not-present-evidence-marker"]);
    }

    fn valid_matrix() -> String {
        let rows = PROJECT_SURFACES
            .iter()
            .map(|surface| {
                format!(
                    "| {} | No command | {} | Missing authority |",
                    surface.surface, surface.state
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "## Current surface matrix\n| Project surface | Current evidence | State | Authority / blocker |\n| --- | --- | --- | --- |\n{rows}\n## Evidence contract\n{}",
            REQUIRED_POLICY_PHRASES.join("\n")
        )
    }
}
