use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::Deserialize;

const MATRIX_PATH: &str = "docs/testing/EXAMPLE-COVERAGE.md";
const CASES_PATH: &str = "tests/examples/seed-cases.toml";
const CASES_SCHEMA: &str = "ling.seed-example-cases/0";

const EXPECTED_CASES: &[(&str, &str, &str, &str, &str, &str)] = &[
    (
        "adt-match",
        "examples/adt-match.ling",
        "core-realistic",
        "chinese",
        "受伤 30\n",
        "生存状态",
    ),
    (
        "hello",
        "examples/hello.ling",
        "core-minimal",
        "ascii",
        "你好，零\n",
        "main",
    ),
    (
        "person",
        "examples/人物.ling",
        "core-realistic",
        "chinese",
        "存活\n",
        "受到伤害",
    ),
    (
        "pipeline",
        "examples/pipeline.ling",
        "core-realistic",
        "chinese",
        "9\n",
        "加一",
    ),
    (
        "tutorial-en",
        "examples/tutorial-en.ling",
        "tutorial",
        "ascii",
        "alive\n",
        "takeDamage",
    ),
    (
        "tutorial-zh",
        "examples/tutorial-zh.ling",
        "tutorial",
        "chinese",
        "存活\n",
        "受到伤害",
    ),
];

const REQUIREMENTS: &[&str] = &[
    "Checked execution and entry",
    "Diagnostics and exits",
    "Unicode 17 and Chinese names",
    "Types, patterns, and Place",
    "Effect and Capability",
    "Semantic Graph and Audit Source",
    "Deterministic tooling",
];

const FEATURES: &[&str] = &[
    "FTR-SEED-0001",
    "FTR-SEED-0002",
    "FTR-SEED-0003",
    "FTR-SEED-0004",
    "FTR-SEED-0005",
    "FTR-SEED-0006",
    "FTR-SEED-0007",
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "not a claim that the G6 documentation gate",
    "current support matrix records all seven Seed features as",
    "Exact semantic IDs are experimental",
    "No example in this inventory introduces placeholder syntax",
    "cargo xtask examples verify",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub requirement_count: usize,
    pub feature_count: usize,
    pub case_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExampleCases {
    schema: String,
    case: Vec<ExampleCase>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExampleCase {
    id: String,
    path: String,
    role: String,
    expected_stdout: String,
    semantic_name: String,
    identifier_style: String,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let inventory = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-EXAMPLES-MATRIX-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let cases_source = fs::read_to_string(root.join(CASES_PATH)).map_err(|error| {
        vec![format!(
            "GOV-EXAMPLES-MATRIX-0010: cannot read {CASES_PATH}: {error}"
        )]
    })?;
    let cases = toml::from_str::<ExampleCases>(&cases_source).map_err(|error| {
        vec![format!(
            "GOV-EXAMPLES-MATRIX-0010: cannot parse {CASES_PATH}: {error}"
        )]
    })?;
    let mut errors = validate(&inventory);
    errors.extend(validate_cases(root, &cases));
    finish(errors).map(|()| CheckSummary {
        requirement_count: REQUIREMENTS.len(),
        feature_count: FEATURES.len(),
        case_count: cases.case.len(),
    })
}

fn validate_cases(root: &Path, manifest: &ExampleCases) -> Vec<String> {
    let mut errors = Vec::new();
    if manifest.schema != CASES_SCHEMA {
        errors.push(format!(
            "GOV-EXAMPLES-MATRIX-0011: expected example schema {CASES_SCHEMA:?}, found {:?}",
            manifest.schema
        ));
    }

    let mut actual = BTreeMap::new();
    let mut paths = BTreeSet::new();
    for case in &manifest.case {
        if actual.insert(case.id.as_str(), case).is_some() {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0012: duplicate example id {:?}",
                case.id
            ));
        }
        if !paths.insert(case.path.as_str()) {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0012: duplicate example path {:?}",
                case.path
            ));
        }
    }

    let actual_ids = actual.keys().copied().collect::<Vec<_>>();
    let expected_ids = EXPECTED_CASES
        .iter()
        .map(|(id, _, _, _, _, _)| *id)
        .collect::<Vec<_>>();
    if actual_ids != expected_ids {
        errors.push(format!(
            "GOV-EXAMPLES-MATRIX-0013: example case set differs; expected {expected_ids:?}, found {actual_ids:?}"
        ));
    }

    for (id, path, role, identifier_style, expected_stdout, semantic_name) in EXPECTED_CASES {
        let Some(case) = actual.get(id) else {
            continue;
        };
        if case.path != *path
            || case.role != *role
            || case.identifier_style != *identifier_style
            || case.expected_stdout != *expected_stdout
            || case.semantic_name != *semantic_name
        {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0014: example {id:?} record differs from the accepted path, role, identifier style, stdout, or Semantic witness"
            ));
        }
        if !safe_example_path(&case.path) || !root.join(&case.path).is_file() {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0015: example {id:?} has an unsafe or missing source path {:?}",
                case.path
            ));
        }
        if case.expected_stdout.is_empty()
            || !case.expected_stdout.ends_with('\n')
            || case.expected_stdout.contains('\r')
            || case.semantic_name.is_empty()
        {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0016: example {id:?} must have non-empty LF stdout and a semantic witness"
            ));
        }
    }
    errors
}

fn safe_example_path(path: &str) -> bool {
    path.starts_with("examples/")
        && path.ends_with(".ling")
        && !path.contains('\\')
        && !path.contains(':')
        && path
            .split('/')
            .all(|component| !component.is_empty() && component != "." && component != "..")
}

fn validate(inventory: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(requirement_section) = inventory
        .split_once("## Seed two-layer matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Capability-to-example traceability"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-EXAMPLES-MATRIX-0002: {MATRIX_PATH} is missing the Seed two-layer matrix section"
        )];
    };
    let Some(feature_section) = inventory
        .split_once("## Capability-to-example traceability")
        .and_then(|(_, remainder)| remainder.split_once("## Reproduction commands"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-EXAMPLES-MATRIX-0002: {MATRIX_PATH} is missing the feature traceability section"
        )];
    };

    let requirement_rows = requirement_section
        .lines()
        .filter_map(parse_requirement_row)
        .collect::<Vec<_>>();
    let mut expected_requirements = REQUIREMENTS.to_vec();
    expected_requirements.sort_unstable();
    let mut actual_requirements = BTreeMap::new();
    for (name, cells) in requirement_rows {
        if actual_requirements
            .insert(name.to_owned(), cells.to_owned())
            .is_some()
        {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0003: duplicate requirement {name:?}"
            ));
        }
    }
    let actual_names = actual_requirements
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if actual_names != expected_requirements {
        errors.push(format!(
            "GOV-EXAMPLES-MATRIX-0004: requirement set differs; expected {expected_requirements:?}, found {actual_names:?}"
        ));
    }
    for requirement in REQUIREMENTS {
        let Some(cells) = actual_requirements.get(*requirement) else {
            continue;
        };
        if cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0005: requirement {requirement:?} has an empty evidence cell"
            ));
        }
    }

    let feature_rows = feature_section
        .lines()
        .filter_map(parse_feature_row)
        .collect::<Vec<_>>();
    let expected_features = FEATURES.to_vec();
    let mut actual_features = BTreeMap::new();
    for (id, cells) in feature_rows {
        if actual_features
            .insert(id.to_owned(), cells.to_owned())
            .is_some()
        {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0006: duplicate feature {id:?}"
            ));
        }
    }
    let actual_feature_ids = actual_features
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if actual_feature_ids != expected_features {
        errors.push(format!(
            "GOV-EXAMPLES-MATRIX-0007: feature set differs; expected {expected_features:?}, found {actual_feature_ids:?}"
        ));
    }
    for feature in FEATURES {
        let Some(cells) = actual_features.get(*feature) else {
            continue;
        };
        if cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0008: feature {feature:?} has an empty traceability cell"
            ));
        }
    }

    let normalized = inventory.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(required) {
            errors.push(format!(
                "GOV-EXAMPLES-MATRIX-0009: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    errors
}

fn parse_requirement_row(line: &str) -> Option<(&str, Vec<&str>)> {
    let cells = parse_cells(line)?;
    if cells.len() < 4 || cells[0] == "Requirement" || is_separator(cells[0]) {
        return None;
    }
    Some((cells[0], cells[1..].to_vec()))
}

fn parse_feature_row(line: &str) -> Option<(&str, Vec<&str>)> {
    let mut cells = parse_cells(line)?;
    if cells.len() < 5 || cells[0] == "Feature ID" || is_separator(cells[0]) {
        return None;
    }
    cells[0] = cells[0].trim_matches('`');
    Some((cells[0], cells[1..].to_vec()))
}

fn parse_cells(line: &str) -> Option<Vec<&str>> {
    Some(
        line.trim()
            .strip_prefix('|')?
            .strip_suffix('|')?
            .split('|')
            .map(str::trim)
            .collect(),
    )
}

fn is_separator(value: &str) -> bool {
    value.chars().all(|character| character == '-')
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
    fn repository_example_matrix_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("example matrix is valid");
        assert_eq!(summary.requirement_count, 7);
        assert_eq!(summary.feature_count, 7);
        assert_eq!(summary.case_count, 6);
    }

    #[test]
    fn rejects_example_requirement_drift() {
        let inventory = "## Seed two-layer matrix\n| Requirement | Layer 1: minimal/reproducible | Layer 2: realistic or negative evidence | Audit/Semantic and status |\n| --- | --- | --- | --- |\n| Checked execution and entry | x | y | z |\n## Capability-to-example traceability\n| Feature ID | Accepted/registered authority | Positive examples | Negative/error evidence | Deferred boundary |\n| --- | --- | --- | --- | --- |\n## Reproduction commands\n";
        let errors = validate(inventory);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("requirement set differs"))
        );
    }

    #[test]
    fn rejects_missing_or_malformed_example_cases() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let source = fs::read_to_string(root.join(CASES_PATH)).expect("case manifest exists");
        let mut manifest = toml::from_str::<ExampleCases>(&source).expect("case manifest parses");
        manifest.case[0].path = "examples/../missing.ling".to_owned();
        manifest.case[1].expected_stdout = "missing newline".to_owned();
        let errors = validate_cases(root, &manifest);
        assert!(errors.iter().any(|error| error.contains("record differs")));
        assert!(
            errors
                .iter()
                .any(|error| error.contains("unsafe or missing"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("non-empty LF stdout"))
        );
    }
}
