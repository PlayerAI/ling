use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const COVERAGE_PATH: &str = "docs/testing/TUTORIAL-COVERAGE.md";
const TUTORIAL_PATH: &str = "docs/TUTORIAL.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SourceSpec {
    path: &'static str,
    language: &'static str,
    output: &'static str,
    required_markers: &'static [&'static str],
}

const SOURCES: &[SourceSpec] = &[
    SourceSpec {
        path: "examples/tutorial-en.ling",
        language: "English",
        output: "alive",
        required_markers: &[
            "module Main",
            "requires Console.Write",
            "type Person",
            "mutable health",
            "let takeDamage",
            "let statusText",
            "Console.write (statusText guanYu)",
            "\"alive\"",
        ],
    },
    SourceSpec {
        path: "examples/tutorial-zh.ling",
        language: "Chinese-first",
        output: "存活",
        required_markers: &[
            "module Main",
            "requires Console.Write",
            "type 人物",
            "mutable 血量",
            "let 受到伤害",
            "let 状态文字",
            "Console.write (状态文字 关羽)",
            "\"存活\"",
        ],
    },
];

const REQUIREMENTS: &[&str] = &[
    "Chinese-first runnable source",
    "Idiomatic English equivalent",
    "Checked offline commands",
    "Semantic and Audit output",
    "Correct missing-Capability error",
    "Bilingual terminology",
    "Unicode 17 and original UTF-8 spans",
    "Unsupported 1.0 boundaries",
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "The coverage gate is inventory-only.",
    "It does not run examples or promote Seed evidence to Stable.",
    "No placeholder syntax, stale legacy name, or future API is introduced.",
    "Unicode 17.0.0 and original UTF-8 byte spans remain required.",
    "The parent DOC-6703 remains BlockedSpec until G1-G5 and Stable support evidence are accepted.",
    "cargo xtask tutorial verify",
];

const REQUIRED_TUTORIAL_MARKERS: &[&str] = &[
    "## 2. Chinese-first tutorial / 中文优先教程",
    "## 3. Equivalent English tutorial / 等价英文教程",
    "## 4. Correct errors / 正确错误",
    "## 5. Boundaries / 边界",
    "cargo run --locked --offline -- audit examples/tutorial-zh.ling",
    "p7-missing-capability",
    "Unicode 17.0.0",
    "original UTF-8 byte spans",
    "Experimental/Preview",
    "Profile",
    "ownership/borrow checker",
    "Native/FFI",
    "Task/Actor runtime",
    "LSP",
    "Zed language server",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub source_count: usize,
    pub requirement_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let coverage = fs::read_to_string(root.join(COVERAGE_PATH)).map_err(|error| {
        vec![format!(
            "GOV-TUTORIAL-MATRIX-0001: cannot read {COVERAGE_PATH}: {error}"
        )]
    })?;
    let tutorial = fs::read_to_string(root.join(TUTORIAL_PATH)).map_err(|error| {
        vec![format!(
            "GOV-TUTORIAL-MATRIX-0002: cannot read {TUTORIAL_PATH}: {error}"
        )]
    })?;

    let mut errors = validate_coverage(&coverage);
    errors.extend(validate_tutorial(&tutorial));
    for source in SOURCES {
        let text = match fs::read_to_string(root.join(source.path)) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!(
                    "GOV-TUTORIAL-MATRIX-0003: cannot read {}: {error}",
                    source.path
                ));
                continue;
            }
        };
        errors.extend(validate_source(source, &text));
    }

    finish(errors).map(|()| CheckSummary {
        source_count: SOURCES.len(),
        requirement_count: REQUIREMENTS.len(),
    })
}

fn validate_coverage(coverage: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(source_section) = coverage
        .split_once("## Source matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Requirement matrix"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-TUTORIAL-MATRIX-0004: {COVERAGE_PATH} is missing the Source matrix section"
        )];
    };
    let Some(requirement_section) = coverage
        .split_once("## Requirement matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Verification"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-TUTORIAL-MATRIX-0005: {COVERAGE_PATH} is missing the Requirement matrix section"
        )];
    };

    let source_rows = source_section
        .lines()
        .filter_map(parse_row)
        .collect::<Vec<_>>();
    let mut actual_sources = BTreeMap::new();
    for (path, cells) in source_rows {
        if actual_sources
            .insert(path.to_owned(), cells.to_owned())
            .is_some()
        {
            errors.push(format!(
                "GOV-TUTORIAL-MATRIX-0006: duplicate tutorial source {path:?}"
            ));
        }
    }
    let expected_sources = SOURCES.iter().map(|source| source.path).collect::<Vec<_>>();
    let actual_source_names = actual_sources
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if actual_source_names != expected_sources {
        errors.push(format!(
            "GOV-TUTORIAL-MATRIX-0007: source set differs; expected {expected_sources:?}, found {actual_source_names:?}"
        ));
    }
    for source in SOURCES {
        let Some(cells) = actual_sources.get(source.path) else {
            continue;
        };
        if cells.len() < 4 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-TUTORIAL-MATRIX-0008: tutorial source {:?} has an empty evidence cell",
                source.path
            ));
            continue;
        }
        if cells[0] != source.language {
            errors.push(format!(
                "GOV-TUTORIAL-MATRIX-0009: tutorial source {:?} must be labeled {:?}, found {:?}",
                source.path, source.language, cells[0]
            ));
        }
        if cells[1] != source.output {
            errors.push(format!(
                "GOV-TUTORIAL-MATRIX-0010: tutorial source {:?} must expect {:?}, found {:?}",
                source.path, source.output, cells[1]
            ));
        }
    }

    let requirement_rows = requirement_section
        .lines()
        .filter_map(parse_row)
        .collect::<Vec<_>>();
    let mut actual_requirements = BTreeMap::new();
    for (name, cells) in requirement_rows {
        if actual_requirements
            .insert(name.to_owned(), cells.to_owned())
            .is_some()
        {
            errors.push(format!(
                "GOV-TUTORIAL-MATRIX-0011: duplicate tutorial requirement {name:?}"
            ));
        }
    }
    let mut expected_requirements = REQUIREMENTS.to_vec();
    expected_requirements.sort_unstable();
    let actual_requirement_names = actual_requirements
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if actual_requirement_names != expected_requirements {
        errors.push(format!(
            "GOV-TUTORIAL-MATRIX-0012: requirement set differs; expected {expected_requirements:?}, found {actual_requirement_names:?}"
        ));
    }
    for requirement in REQUIREMENTS {
        let Some(cells) = actual_requirements.get(*requirement) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-TUTORIAL-MATRIX-0013: tutorial requirement {requirement:?} has an empty evidence cell"
            ));
        }
    }

    let normalized = normalize(coverage);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-TUTORIAL-MATRIX-0014: {COVERAGE_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    errors
}

fn validate_tutorial(tutorial: &str) -> Vec<String> {
    let normalized = normalize(tutorial);
    REQUIRED_TUTORIAL_MARKERS
        .iter()
        .filter(|required| !normalized.contains(&normalize(required)))
        .map(|required| {
            format!(
                "GOV-TUTORIAL-MATRIX-0015: {TUTORIAL_PATH} is missing required marker {required:?}"
            )
        })
        .collect()
}

fn validate_source(source: &SourceSpec, text: &str) -> Vec<String> {
    let normalized = normalize(text);
    source
        .required_markers
        .iter()
        .filter(|required| !normalized.contains(&normalize(required)))
        .map(|required| {
            format!(
                "GOV-TUTORIAL-MATRIX-0016: {} is missing required source marker {required:?}",
                source.path
            )
        })
        .collect()
}

fn parse_row(line: &str) -> Option<(&str, Vec<&str>)> {
    let cells = line
        .trim()
        .strip_prefix('|')?
        .strip_suffix('|')?
        .split('|')
        .map(|cell| cell.trim().trim_matches('`'))
        .collect::<Vec<_>>();
    if cells.len() < 4 || cells[0].starts_with("Source") || cells[0].starts_with("Requirement") {
        return None;
    }
    if cells[0].chars().all(|character| character == '-') {
        return None;
    }
    Some((cells[0], cells[1..].to_vec()))
}

fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
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
    fn repository_tutorial_matrix_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("tutorial matrix is valid");
        assert_eq!(summary.source_count, 2);
        assert_eq!(summary.requirement_count, 8);
    }

    #[test]
    fn rejects_tutorial_requirement_drift() {
        let coverage = "## Source matrix\n| Source | Language | Expected output | Process evidence | Boundary |\n| --- | --- | --- | --- | --- |\n| examples/tutorial-en.ling | English | alive | x | y |\n| examples/tutorial-zh.ling | Chinese-first | 存活 | x | y |\n## Requirement matrix\n| Requirement | Tutorial evidence | Authority/evidence | State |\n| --- | --- | --- | --- |\n| Chinese-first runnable source | x | y | z |\n## Verification\n";
        let errors = validate_coverage(coverage);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("requirement set differs"))
        );
    }

    #[test]
    fn rejects_tutorial_boundary_drift() {
        let errors = validate_tutorial("## 2. Chinese-first tutorial / 中文优先教程\n");
        assert!(
            errors
                .iter()
                .any(|error| error.contains("Boundaries / 边界"))
        );
    }

    #[test]
    fn rejects_source_marker_drift() {
        let source = SOURCES
            .iter()
            .find(|source| source.path == "examples/tutorial-zh.ling")
            .expect("Chinese tutorial spec");
        let errors = validate_source(source, "module Main\n");
        assert!(errors.iter().any(|error| error.contains("type 人物")));
    }
}
