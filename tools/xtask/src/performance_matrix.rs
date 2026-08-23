use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::performance::{SAMPLE_COUNT, SCENARIO_NAMES, SYNTHETIC_FILE_COUNT};

const MATRIX_PATH: &str = "docs/testing/PERFORMANCE-BASELINE.md";
const ARTIFACT_PATH: &str = "docs/status/INC-1410-PERFORMANCE-BASELINE.json";
const ARTIFACT_SCHEMA: &str = "ling.performance-baseline/1";

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

#[derive(Debug, Clone, Copy)]
struct ScenarioContract {
    trace_events: usize,
    misses: usize,
    hits: usize,
    synthetic: bool,
}

const SCENARIO_CONTRACTS: [ScenarioContract; 8] = [
    ScenarioContract {
        trace_events: 24,
        misses: 9,
        hits: 15,
        synthetic: false,
    },
    ScenarioContract {
        trace_events: 8,
        misses: 0,
        hits: 8,
        synthetic: false,
    },
    ScenarioContract {
        trace_events: 20,
        misses: 5,
        hits: 15,
        synthetic: false,
    },
    ScenarioContract {
        trace_events: 20,
        misses: 5,
        hits: 15,
        synthetic: false,
    },
    ScenarioContract {
        trace_events: 24,
        misses: 9,
        hits: 15,
        synthetic: false,
    },
    ScenarioContract {
        trace_events: 20_000,
        misses: 20_000,
        hits: 0,
        synthetic: true,
    },
    ScenarioContract {
        trace_events: 20_000,
        misses: 0,
        hits: 20_000,
        synthetic: true,
    },
    ScenarioContract {
        trace_events: 20_000,
        misses: 2,
        hits: 19_998,
        synthetic: true,
    },
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BaselineArtifact {
    schema: String,
    sample_count: usize,
    synthetic_file_count: usize,
    timed_region_excludes_fixture_setup: bool,
    scenarios: Vec<ScenarioArtifact>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ScenarioArtifact {
    name: String,
    samples_ns: Vec<u128>,
    trace_events: Vec<usize>,
    misses: Vec<usize>,
    hits: Vec<usize>,
    completed_items: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub measurement_count: usize,
    pub covered_count: usize,
    pub partial_count: usize,
    pub deferred_count: usize,
    pub artifact_scenario_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-PERF-MATRIX-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate(&matrix);
    let artifact_scenario_count = validate_artifact(root, &mut errors);
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
        artifact_scenario_count,
    })
}

fn validate_artifact(root: &Path, errors: &mut Vec<String>) -> usize {
    let text = match fs::read_to_string(root.join(ARTIFACT_PATH)) {
        Ok(text) => text,
        Err(error) => {
            errors.push(format!(
                "GOV-PERF-MATRIX-0007: cannot read {ARTIFACT_PATH}: {error}"
            ));
            return 0;
        }
    };
    validate_artifact_text(&text, errors)
}

fn validate_artifact_text(text: &str, errors: &mut Vec<String>) -> usize {
    let artifact: BaselineArtifact = match serde_json::from_str(text) {
        Ok(artifact) => artifact,
        Err(error) => {
            errors.push(format!(
                "GOV-PERF-MATRIX-0007: cannot parse {ARTIFACT_PATH}: {error}"
            ));
            return 0;
        }
    };

    if artifact.schema != ARTIFACT_SCHEMA {
        errors.push(format!(
            "GOV-PERF-MATRIX-0008: artifact schema must be {ARTIFACT_SCHEMA:?}, found {:?}",
            artifact.schema
        ));
    }
    if artifact.sample_count != SAMPLE_COUNT {
        errors.push(format!(
            "GOV-PERF-MATRIX-0008: artifact sample_count must be {SAMPLE_COUNT}, found {}",
            artifact.sample_count
        ));
    }
    if artifact.synthetic_file_count != SYNTHETIC_FILE_COUNT {
        errors.push(format!(
            "GOV-PERF-MATRIX-0008: artifact synthetic_file_count must be {SYNTHETIC_FILE_COUNT}, found {}",
            artifact.synthetic_file_count
        ));
    }
    if !artifact.timed_region_excludes_fixture_setup {
        errors.push(
            "GOV-PERF-MATRIX-0008: artifact must exclude fixture setup from timed regions"
                .to_owned(),
        );
    }

    let actual_names = artifact
        .scenarios
        .iter()
        .map(|scenario| scenario.name.as_str())
        .collect::<Vec<_>>();
    if actual_names != SCENARIO_NAMES {
        errors.push(format!(
            "GOV-PERF-MATRIX-0009: artifact scenarios must be {SCENARIO_NAMES:?}, found {actual_names:?}"
        ));
    }

    for (index, scenario) in artifact.scenarios.iter().enumerate() {
        let Some(contract) = SCENARIO_CONTRACTS.get(index) else {
            continue;
        };
        validate_samples(scenario, *contract, errors);
    }
    artifact.scenarios.len()
}

fn validate_samples(
    scenario: &ScenarioArtifact,
    contract: ScenarioContract,
    errors: &mut Vec<String>,
) {
    for (field, length) in [
        ("samples_ns", scenario.samples_ns.len()),
        ("trace_events", scenario.trace_events.len()),
        ("misses", scenario.misses.len()),
        ("hits", scenario.hits.len()),
        ("completed_items", scenario.completed_items.len()),
    ] {
        if length != SAMPLE_COUNT {
            errors.push(format!(
                "GOV-PERF-MATRIX-0010: scenario {:?} field {field} must contain {SAMPLE_COUNT} samples, found {length}",
                scenario.name
            ));
        }
    }
    if scenario.samples_ns.contains(&0) {
        errors.push(format!(
            "GOV-PERF-MATRIX-0010: scenario {:?} contains a zero-duration sample",
            scenario.name
        ));
    }
    for (field, values, expected) in [
        (
            "trace_events",
            &scenario.trace_events,
            contract.trace_events,
        ),
        ("misses", &scenario.misses, contract.misses),
        ("hits", &scenario.hits, contract.hits),
    ] {
        if values.iter().any(|value| *value != expected) {
            errors.push(format!(
                "GOV-PERF-MATRIX-0011: scenario {:?} field {field} must contain only {expected}, found {values:?}",
                scenario.name
            ));
        }
    }
    if contract.synthetic {
        if scenario
            .completed_items
            .iter()
            .any(|value| *value != SYNTHETIC_FILE_COUNT)
        {
            errors.push(format!(
                "GOV-PERF-MATRIX-0011: synthetic scenario {:?} must complete {SYNTHETIC_FILE_COUNT} items, found {:?}",
                scenario.name, scenario.completed_items
            ));
        }
    } else if scenario.completed_items.first().is_none_or(|first| {
        *first == 0 || scenario.completed_items.iter().any(|value| value != first)
    }) {
        errors.push(format!(
            "GOV-PERF-MATRIX-0011: checked scenario {:?} must report one stable non-zero completed-item count, found {:?}",
            scenario.name, scenario.completed_items
        ));
    }
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
        assert_eq!(summary.artifact_scenario_count, 8);
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

    #[test]
    fn rejects_baseline_schema_and_work_drift() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let artifact =
            fs::read_to_string(root.join(ARTIFACT_PATH)).expect("performance artifact is readable");
        let mutated = artifact
            .replacen(ARTIFACT_SCHEMA, "ling.performance-baseline/2", 1)
            .replacen("\"misses\": [9, 9, 9]", "\"misses\": [8, 8, 8]", 1);
        let mut errors = Vec::new();
        assert_eq!(validate_artifact_text(&mutated, &mut errors), 8);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("artifact schema must be"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("field misses must contain only 9"))
        );
    }
}
