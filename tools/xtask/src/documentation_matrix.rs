use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const INVENTORY_PATH: &str = "docs/testing/DOCUMENTATION-INVENTORY.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Manual {
    name: &'static str,
    state: &'static str,
}

const MANUALS: &[Manual] = &[
    Manual {
        name: "Language Reference",
        state: "Seed",
    },
    Manual {
        name: "Semantics Reference",
        state: "Seed",
    },
    Manual {
        name: "CLI / Tooling",
        state: "Seed / Preview",
    },
    Manual {
        name: "Project / Package",
        state: "Seed library slice",
    },
    Manual {
        name: "Effect / Capability",
        state: "Seed",
    },
    Manual {
        name: "Task / Actor / Replay",
        state: "Future / Unsupported",
    },
    Manual {
        name: "Native / Ownership / FFI",
        state: "Future / Unsupported",
    },
    Manual {
        name: "Kernel / Device",
        state: "Future / Unsupported",
    },
    Manual {
        name: "Critical / Node / Contract / Evidence",
        state: "Future / Unsupported",
    },
    Manual {
        name: "LSP / Zed",
        state: "Zed grammar only; LSP future",
    },
    Manual {
        name: "Migration / Compatibility",
        state: "Partial Seed",
    },
    Manual {
        name: "Security / Disclosure",
        state: "Seed audit only",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "A plan mention is never treated as a feature",
    "must not copy stale legacy CLI or source names",
    "Every new stable manual must link its Accepted RFC/decision",
    "They do not turn future manuals into implemented features.",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub manual_count: usize,
    pub future_unsupported_count: usize,
    pub evidence_path_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let inventory = fs::read_to_string(root.join(INVENTORY_PATH)).map_err(|error| {
        vec![format!(
            "GOV-DOCS-MATRIX-0001: cannot read {INVENTORY_PATH}: {error}"
        )]
    })?;
    let (errors, evidence_path_count) = validate(root, &inventory);
    finish(errors).map(|()| CheckSummary {
        manual_count: MANUALS.len(),
        future_unsupported_count: MANUALS
            .iter()
            .filter(|manual| manual.state == "Future / Unsupported")
            .count(),
        evidence_path_count,
    })
}

fn validate(root: &Path, inventory: &str) -> (Vec<String>, usize) {
    let mut errors = Vec::new();
    let Some(formal_set) = inventory
        .split_once("## Formal set")
        .and_then(|(_, remainder)| remainder.split_once("## Required form"))
        .map(|(section, _)| section)
    else {
        return (
            vec![format!(
                "GOV-DOCS-MATRIX-0002: {INVENTORY_PATH} is missing the Formal set section"
            )],
            0,
        );
    };

    let rows = formal_set.lines().filter_map(parse_row).collect::<Vec<_>>();
    let expected = MANUALS
        .iter()
        .map(|manual| (manual.name, manual.state))
        .collect::<BTreeMap<_, _>>();
    let mut actual = BTreeMap::new();
    let mut evidence_path_count = 0;
    for (name, evidence, state) in rows {
        if actual.insert(name.to_owned(), state.to_owned()).is_some() {
            errors.push(format!("GOV-DOCS-MATRIX-0003: duplicate manual {name:?}"));
        }
        evidence_path_count += validate_evidence_paths(root, name, evidence, &mut errors);
    }

    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    let expected_names = expected.keys().copied().collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-DOCS-MATRIX-0004: manual set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for (name, state) in expected {
        if actual.get(name).map(String::as_str) != Some(state) {
            errors.push(format!(
                "GOV-DOCS-MATRIX-0005: manual {name:?} must be {state:?}, found {:?}",
                actual.get(name)
            ));
        }
    }

    let normalized = inventory.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(required) {
            errors.push(format!(
                "GOV-DOCS-MATRIX-0006: {INVENTORY_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    (errors, evidence_path_count)
}

fn validate_evidence_paths(
    root: &Path,
    manual: &str,
    evidence: &str,
    errors: &mut Vec<String>,
) -> usize {
    let paths = evidence
        .split('`')
        .enumerate()
        .filter_map(|(index, token)| (index % 2 == 1 && evidence_path(token)).then_some(token))
        .collect::<Vec<_>>();
    if paths.is_empty() {
        errors.push(format!(
            "GOV-DOCS-MATRIX-0007: manual {manual:?} must cite at least one exact repository path"
        ));
    }
    for path in &paths {
        if !safe_evidence_path(path) {
            errors.push(format!(
                "GOV-DOCS-MATRIX-0007: manual {manual:?} has unsafe or non-exact evidence path {path:?}"
            ));
        } else if !root.join(path.trim_end_matches('/')).exists() {
            errors.push(format!(
                "GOV-DOCS-MATRIX-0008: manual {manual:?} evidence path does not exist: {path}"
            ));
        }
    }
    paths.len()
}

fn evidence_path(token: &str) -> bool {
    token == "README.md"
        || ["docs/", "tests/", "crates/", "tools/", "editors/"]
            .iter()
            .any(|prefix| token.starts_with(prefix))
}

fn safe_evidence_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['*', '?', '[', ']', '\\', ':'])
        && path
            .trim_end_matches('/')
            .split('/')
            .all(|component| !component.is_empty() && component != "." && component != "..")
}

fn parse_row(line: &str) -> Option<(&str, &str, &str)> {
    let cells = line
        .trim()
        .strip_prefix('|')?
        .strip_suffix('|')?
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    if cells.len() < 4
        || cells[0] == "Planned manual"
        || cells[0].chars().all(|character| character == '-')
    {
        return None;
    }
    Some((cells[0], cells[1], cells[2]))
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
    fn repository_documentation_inventory_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("documentation inventory is valid");
        assert_eq!(summary.manual_count, 12);
        assert_eq!(summary.future_unsupported_count, 4);
        assert_eq!(summary.evidence_path_count, 46);
    }

    #[test]
    fn rejects_documentation_state_drift() {
        let inventory = "## Formal set\n| Planned manual | Current source and evidence | State | Boundary / missing work |\n| --- | --- | --- | --- |\n| Language Reference | x | Future / Unsupported | y |\n## Required form\n";
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let (errors, _) = validate(root, inventory);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("manual set differs"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("must be \"Seed\""))
        );
    }

    #[test]
    fn rejects_non_exact_and_missing_evidence_paths() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let mut errors = Vec::new();
        assert_eq!(
            validate_evidence_paths(
                root,
                "fixture",
                "`docs/*.md`, `docs/not-present.md`, `README.md`",
                &mut errors,
            ),
            3
        );
        assert!(errors.iter().any(|error| error.contains("non-exact")));
        assert!(errors.iter().any(|error| error.contains("does not exist")));
    }
}
