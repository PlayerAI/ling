use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/PERFORMANCE-BASELINE.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Measurement {
    name: &'static str,
    state: &'static str,
}

const MEASUREMENTS: &[Measurement] = &[
    Measurement {
        name: "cold check/build",
        state: "Partial",
    },
    Measurement {
        name: "warm/no-op build",
        state: "Partial",
    },
    Measurement {
        name: "single-file edit latency",
        state: "Covered for Seed query boundary",
    },
    Measurement {
        name: "large workspace edit latency",
        state: "Covered for synthetic parse boundary",
    },
    Measurement {
        name: "LSP diagnostics/hover/completion",
        state: "Deferred",
    },
    Measurement {
        name: "VM startup/throughput",
        state: "Deferred",
    },
    Measurement {
        name: "Native compile/runtime",
        state: "Deferred",
    },
    Measurement {
        name: "Actor/task overhead",
        state: "Deferred",
    },
    Measurement {
        name: "Replay overhead",
        state: "Deferred",
    },
    Measurement {
        name: "Kernel CPU/GPU",
        state: "Deferred",
    },
    Measurement {
        name: "memory peak",
        state: "Deferred",
    },
    Measurement {
        name: "Zed startup/highlight",
        state: "Deferred",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "trend baseline, not a release threshold",
    "Fixture construction is excluded from timed regions",
    "no absolute performance claim",
    "Do not convert this baseline into a hard gate",
    "Accepted performance policy",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub measurement_count: usize,
    pub covered_count: usize,
    pub partial_count: usize,
    pub deferred_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-PERF-MATRIX-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let errors = validate(&matrix);
    finish(errors).map(|()| CheckSummary {
        measurement_count: MEASUREMENTS.len(),
        covered_count: MEASUREMENTS
            .iter()
            .filter(|measurement| measurement.state.starts_with("Covered"))
            .count(),
        partial_count: MEASUREMENTS
            .iter()
            .filter(|measurement| measurement.state == "Partial")
            .count(),
        deferred_count: MEASUREMENTS
            .iter()
            .filter(|measurement| measurement.state == "Deferred")
            .count(),
    })
}

fn validate(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(plan) = matrix
        .split_once("## Plan coverage")
        .and_then(|(_, remainder)| remainder.split_once("## Reproduction"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-PERF-MATRIX-0002: {MATRIX_PATH} is missing the Plan coverage section"
        )];
    };

    let rows = plan.lines().filter_map(parse_row).collect::<Vec<_>>();
    let expected = MEASUREMENTS
        .iter()
        .map(|measurement| (measurement.name, measurement.state))
        .collect::<BTreeMap<_, _>>();
    let mut actual = BTreeMap::new();
    for (name, state) in rows {
        if actual.insert(name.to_owned(), state.to_owned()).is_some() {
            errors.push(format!(
                "GOV-PERF-MATRIX-0003: duplicate measurement {name:?}"
            ));
        }
    }

    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    let expected_names = expected.keys().copied().collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-PERF-MATRIX-0004: measurement set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for (name, state) in expected {
        if actual.get(name).map(String::as_str) != Some(state) {
            errors.push(format!(
                "GOV-PERF-MATRIX-0005: measurement {name:?} must be {state:?}, found {:?}",
                actual.get(name)
            ));
        }
    }

    for required in REQUIRED_POLICY_PHRASES {
        if !matrix.contains(required) {
            errors.push(format!(
                "GOV-PERF-MATRIX-0006: {MATRIX_PATH} is missing policy phrase {required:?}"
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
    if cells.len() < 3
        || cells[0] == "Planned measurement"
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
    fn repository_performance_matrix_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("performance matrix is valid");
        assert_eq!(summary.measurement_count, 12);
        assert_eq!(summary.covered_count, 2);
        assert_eq!(summary.partial_count, 2);
        assert_eq!(summary.deferred_count, 8);
    }

    #[test]
    fn rejects_performance_state_drift() {
        let matrix = "## Plan coverage\n| Planned measurement | State | Evidence / boundary |\n| --- | --- | --- |\n| cold check/build | Deferred | x |\n## Reproduction\n";
        let errors = validate(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("measurement set differs"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("must be \"Partial\""))
        );
    }
}
