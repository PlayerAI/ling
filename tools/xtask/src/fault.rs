use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/FAULT-INJECTION.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Scenario {
    name: &'static str,
    state: &'static str,
}

const SCENARIOS: &[Scenario] = &[
    Scenario {
        name: "cache corruption",
        state: "Covered",
    },
    Scenario {
        name: "disk full",
        state: "Covered",
    },
    Scenario {
        name: "interrupted write",
        state: "Covered",
    },
    Scenario {
        name: "process crash",
        state: "Deferred",
    },
    Scenario {
        name: "network partition",
        state: "Deferred",
    },
    Scenario {
        name: "remote duplicate / reorder",
        state: "Deferred",
    },
    Scenario {
        name: "device lost / OOM",
        state: "Deferred",
    },
    Scenario {
        name: "actor restart storm",
        state: "Deferred",
    },
    Scenario {
        name: "replay truncation",
        state: "Deferred",
    },
    Scenario {
        name: "invalid proof / evidence",
        state: "Deferred",
    },
    Scenario {
        name: "language-server crash / restart",
        state: "Deferred",
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub scenario_count: usize,
    pub covered_count: usize,
    pub partial_count: usize,
    pub deferred_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-FAULT-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let errors = validate(&matrix);
    finish(errors).map(|()| CheckSummary {
        scenario_count: SCENARIOS.len(),
        covered_count: SCENARIOS
            .iter()
            .filter(|scenario| scenario.state == "Covered")
            .count(),
        partial_count: SCENARIOS
            .iter()
            .filter(|scenario| scenario.state == "Partial")
            .count(),
        deferred_count: SCENARIOS
            .iter()
            .filter(|scenario| scenario.state == "Deferred")
            .count(),
    })
}

fn validate(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let rows = matrix.lines().filter_map(parse_row).collect::<Vec<_>>();
    let expected = SCENARIOS
        .iter()
        .map(|scenario| (scenario.name, scenario.state))
        .collect::<BTreeMap<_, _>>();
    let mut actual = BTreeMap::new();
    for (name, state) in rows {
        if actual.insert(name.to_owned(), state.to_owned()).is_some() {
            errors.push(format!("GOV-FAULT-0002: duplicate fault scenario {name:?}"));
        }
    }

    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    let expected_names = expected.keys().copied().collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-FAULT-0003: scenario set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for (name, state) in expected {
        if actual.get(name).map(String::as_str) != Some(state) {
            errors.push(format!(
                "GOV-FAULT-0004: scenario {name:?} must be {state}, found {:?}",
                actual.get(name)
            ));
        }
    }

    for required in [
        "fault point and precondition",
        "retried, rolled back, committed",
        "cleanup and partial-output rules",
        "deterministic replay input",
        "named triage owner",
        "No public fault-injection command",
    ] {
        if !matrix.contains(required) {
            errors.push(format!(
                "GOV-FAULT-0005: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    errors
}

fn parse_row(line: &str) -> Option<(&str, &str)> {
    let cells = line
        .trim()
        .strip_prefix('|')?
        .strip_suffix('|')?
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    if cells.len() < 4
        || cells[0] == "Scenario"
        || cells[0].chars().all(|character| character == '-')
    {
        return None;
    }
    Some((cells[0], cells[1]))
}

fn finish(errors: Vec<String>) -> Result<(), Vec<String>> {
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
    fn repository_fault_matrix_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("fault matrix is valid");
        assert_eq!(summary.scenario_count, 11);
        assert_eq!(summary.covered_count, 3);
        assert_eq!(summary.partial_count, 0);
        assert_eq!(summary.deferred_count, 8);
    }

    #[test]
    fn rejects_fault_state_drift() {
        let matrix = "| Scenario | State | Current evidence | Missing evidence / owner |\n| --- | --- | --- | --- |\n| cache corruption | Deferred | x | y |\n";
        let errors = validate(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("scenario set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("must be Covered")));
    }
}
