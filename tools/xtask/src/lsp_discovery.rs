use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/LSP-DISCOVERY-ACQUISITION.md";

const CURRENT_EVIDENCE_PATHS: &[&str] = &[
    "Cargo.toml",
    "crates/ling-cli/Cargo.toml",
    "crates/ling-cli/src/main.rs",
    "crates/ling-cli/tests/lsp.rs",
    "crates/ling-lsp/Cargo.toml",
    "docs/governance/protocol-inventory.toml",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PrioritySpec {
    source: &'static str,
    state: &'static str,
}

const PRIORITIES: &[PrioritySpec] = &[
    PrioritySpec {
        source: "User-configured executable",
        state: "Not established",
    },
    PrioritySpec {
        source: "PATH lookup",
        state: "Not established",
    },
    PrioritySpec {
        source: "Official release download",
        state: "Unavailable",
    },
    PrioritySpec {
        source: "Explicit failure/install guidance",
        state: "Not established",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "It is not an installer design",
    "No setting key, PATH name, URL, protocol field, diagnostic code, installer, or fallback executable is invented here.",
    "The priority order is therefore documentation only.",
    "Source-built CLI availability does not establish discovery or acquisition.",
    "HTTPS-only transport",
    "explicit compiler/LSP version selection",
    "checksum and/or signature verification",
    "atomic installation",
    "no arbitrary execution before verification",
    "user override precedence",
    "offline behavior",
    "redacted diagnostics",
    "No language-server discovery test is claimed",
    "No stale legacy CLI/source name",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub priority_count: usize,
    pub unavailable_count: usize,
    pub not_established_count: usize,
    pub current_evidence_file_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-LSP-DISCOVERY-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate(&matrix);
    errors.extend(validate_current_evidence(root));
    finish(errors).map(|()| CheckSummary {
        priority_count: PRIORITIES.len(),
        unavailable_count: PRIORITIES
            .iter()
            .filter(|priority| priority.state == "Unavailable")
            .count(),
        not_established_count: PRIORITIES
            .iter()
            .filter(|priority| priority.state == "Not established")
            .count(),
        current_evidence_file_count: CURRENT_EVIDENCE_PATHS.len(),
    })
}

fn validate_current_evidence(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let mut sources = BTreeMap::new();
    for path in CURRENT_EVIDENCE_PATHS {
        match fs::read_to_string(root.join(path)) {
            Ok(source) => {
                sources.insert(*path, source);
            }
            Err(error) => errors.push(format!(
                "GOV-LSP-DISCOVERY-0009: cannot read current LSP evidence {path}: {error}"
            )),
        }
    }
    if errors.is_empty() {
        errors.extend(validate_current_evidence_sources(&sources));
    }
    errors
}

fn validate_current_evidence_sources(sources: &BTreeMap<&str, String>) -> Vec<String> {
    let mut errors = Vec::new();
    let workspace = parse_toml_source(sources, "Cargo.toml", &mut errors);
    if let Some(workspace) = workspace {
        expect_array_member(
            &workspace,
            &["workspace", "members"],
            "crates/ling-cli",
            "Cargo.toml",
            &mut errors,
        );
        expect_array_member(
            &workspace,
            &["workspace", "members"],
            "crates/ling-lsp",
            "Cargo.toml",
            &mut errors,
        );
    }

    let cli_manifest = parse_toml_source(sources, "crates/ling-cli/Cargo.toml", &mut errors);
    if let Some(cli_manifest) = cli_manifest {
        expect_toml_string(
            &cli_manifest,
            &["dependencies", "ling-lsp", "path"],
            "../ling-lsp",
            "crates/ling-cli/Cargo.toml",
            &mut errors,
        );
    }

    let lsp_manifest = parse_toml_source(sources, "crates/ling-lsp/Cargo.toml", &mut errors);
    if let Some(lsp_manifest) = lsp_manifest {
        expect_toml_string(
            &lsp_manifest,
            &["package", "name"],
            "ling-lsp",
            "crates/ling-lsp/Cargo.toml",
            &mut errors,
        );
    }

    expect_source_markers(
        sources,
        "crates/ling-cli/src/main.rs",
        &[
            "if options.command == Command::Lsp",
            "return execute_lsp();",
            "ling_lsp::run_stdio",
            "{CLI_NAME} lsp --stdio",
        ],
        &mut errors,
    );
    expect_source_markers(
        sources,
        "crates/ling-cli/tests/lsp.rs",
        &[
            "CARGO_BIN_EXE_ling",
            ".args([\"lsp\", \"--stdio\"])",
            "shutdown",
            "exit",
        ],
        &mut errors,
    );

    let inventory = parse_toml_source(
        sources,
        "docs/governance/protocol-inventory.toml",
        &mut errors,
    );
    if let Some(inventory) = inventory {
        let lifecycle = inventory
            .get("protocol")
            .and_then(toml::Value::as_array)
            .and_then(|protocols| {
                protocols.iter().find(|protocol| {
                    protocol.get("id").and_then(toml::Value::as_str) == Some("PROTO-LSP-LIFECYCLE")
                })
            });
        let Some(lifecycle) = lifecycle else {
            errors.push(
                "GOV-LSP-DISCOVERY-0013: protocol inventory is missing PROTO-LSP-LIFECYCLE"
                    .to_owned(),
            );
            return errors;
        };
        expect_toml_string(
            lifecycle,
            &["current_version"],
            "ling.lsp.lifecycle/0.1",
            "docs/governance/protocol-inventory.toml",
            &mut errors,
        );
        expect_toml_string(
            lifecycle,
            &["stability"],
            "Preview",
            "docs/governance/protocol-inventory.toml",
            &mut errors,
        );
        if lifecycle.get("implemented").and_then(toml::Value::as_bool) != Some(true) {
            errors.push(
                "GOV-LSP-DISCOVERY-0014: PROTO-LSP-LIFECYCLE must remain implemented".to_owned(),
            );
        }
        expect_array_member(
            lifecycle,
            &["producer"],
            "ling lsp --stdio",
            "docs/governance/protocol-inventory.toml",
            &mut errors,
        );
    }
    errors
}

fn parse_toml_source(
    sources: &BTreeMap<&str, String>,
    path: &str,
    errors: &mut Vec<String>,
) -> Option<toml::Value> {
    let source = sources.get(path)?;
    match toml::from_str(source) {
        Ok(value) => Some(value),
        Err(error) => {
            errors.push(format!(
                "GOV-LSP-DISCOVERY-0010: cannot parse current evidence {path}: {error}"
            ));
            None
        }
    }
}

fn toml_value<'a>(value: &'a toml::Value, path: &[&str]) -> Option<&'a toml::Value> {
    path.iter()
        .try_fold(value, |current, key| current.get(*key))
}

fn expect_toml_string(
    value: &toml::Value,
    key_path: &[&str],
    expected: &str,
    source_path: &str,
    errors: &mut Vec<String>,
) {
    let actual = toml_value(value, key_path).and_then(toml::Value::as_str);
    if actual != Some(expected) {
        errors.push(format!(
            "GOV-LSP-DISCOVERY-0011: {source_path} field {} must be {expected:?}, found {actual:?}",
            key_path.join(".")
        ));
    }
}

fn expect_array_member(
    value: &toml::Value,
    key_path: &[&str],
    expected: &str,
    source_path: &str,
    errors: &mut Vec<String>,
) {
    let present = toml_value(value, key_path)
        .and_then(toml::Value::as_array)
        .is_some_and(|values| values.iter().any(|value| value.as_str() == Some(expected)));
    if !present {
        errors.push(format!(
            "GOV-LSP-DISCOVERY-0012: {source_path} field {} must contain {expected:?}",
            key_path.join(".")
        ));
    }
}

fn expect_source_markers(
    sources: &BTreeMap<&str, String>,
    path: &str,
    markers: &[&str],
    errors: &mut Vec<String>,
) {
    let Some(source) = sources.get(path) else {
        return;
    };
    for marker in markers {
        if !source.contains(marker) {
            errors.push(format!(
                "GOV-LSP-DISCOVERY-0015: {path} is missing current LSP evidence marker {marker:?}"
            ));
        }
    }
}

fn validate(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(priority_section) = matrix
        .split_once("## Required priority matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Security and operational contract"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-LSP-DISCOVERY-0002: {MATRIX_PATH} is missing the Required priority matrix section"
        )];
    };

    let rows = priority_section
        .lines()
        .filter_map(parse_row)
        .collect::<Vec<_>>();
    let mut actual = BTreeMap::new();
    for (source, cells) in rows {
        if actual.insert(source.to_owned(), cells.to_owned()).is_some() {
            errors.push(format!(
                "GOV-LSP-DISCOVERY-0003: duplicate discovery source {source:?}"
            ));
        }
    }

    let mut expected_names = PRIORITIES
        .iter()
        .map(|priority| priority.source)
        .collect::<Vec<_>>();
    expected_names.sort_unstable();
    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-LSP-DISCOVERY-0004: discovery source set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }

    for priority in PRIORITIES {
        let Some(cells) = actual.get(priority.source) else {
            continue;
        };
        if cells.len() < 3 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-LSP-DISCOVERY-0005: discovery source {:?} has an empty evidence/state/authority cell",
                priority.source
            ));
            continue;
        }
        if cells[1] != priority.state {
            errors.push(format!(
                "GOV-LSP-DISCOVERY-0006: discovery source {:?} must have state {:?}, found {:?}",
                priority.source, priority.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-LSP-DISCOVERY-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
    if normalized
        .to_ascii_lowercase()
        .contains(stale_legacy_name.as_str())
    {
        errors.push(format!(
            "GOV-LSP-DISCOVERY-0008: {MATRIX_PATH} contains a stale legacy CLI/source name"
        ));
    }
    errors
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
        || cells[0] == "Planned source"
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
    fn repository_lsp_discovery_inventory_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("LSP discovery inventory is valid");
        assert_eq!(summary.priority_count, 4);
        assert_eq!(summary.unavailable_count, 1);
        assert_eq!(summary.not_established_count, 3);
        assert_eq!(summary.current_evidence_file_count, 6);
    }

    #[test]
    fn rejects_priority_state_drift() {
        let matrix = "## Required priority matrix\n| Planned source | Current repository evidence | State | Required authority before implementation |\n| --- | --- | --- | --- |\n| PATH lookup | evidence | Unavailable | authority |\n## Security and operational contract\n";
        let errors = validate(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("discovery source set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_stale_legacy_name() {
        let stale_legacy_name = ['z', 'e', 'r', 'o'].iter().collect::<String>();
        let matrix = format!(
            "## Required priority matrix\n| Planned source | Current repository evidence | State | Required authority before implementation |\n| --- | --- | --- | --- |\n| User-configured executable | evidence | Not established | authority |\n| PATH lookup | evidence | Unavailable | authority |\n| Official release download | evidence | Unavailable | authority |\n| Explicit failure/install guidance | evidence | Not established | authority |\n## Security and operational contract\n{stale_legacy_name}\n"
        );
        let errors = validate(&matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("stale legacy CLI/source name"))
        );
    }

    #[test]
    fn rejects_missing_security_policy_phrase() {
        let matrix = "## Required priority matrix\n| Planned source | Current repository evidence | State | Required authority before implementation |\n| --- | --- | --- | --- |\n| User-configured executable | evidence | Not established | authority |\n| PATH lookup | evidence | Unavailable | authority |\n| Official release download | evidence | Unavailable | authority |\n| Explicit failure/install guidance | evidence | Not established | authority |\n## Security and operational contract\n";
        let errors = validate(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing policy phrase"))
        );
    }

    #[test]
    fn rejects_false_current_lsp_evidence() {
        let sources = BTreeMap::from([
            (
                "Cargo.toml",
                "[workspace]\nmembers = [\"crates/ling-cli\"]\n".to_owned(),
            ),
            (
                "crates/ling-cli/Cargo.toml",
                "[dependencies]\nling-lsp = { path = \"../wrong\" }\n".to_owned(),
            ),
            ("crates/ling-cli/src/main.rs", String::new()),
            ("crates/ling-cli/tests/lsp.rs", String::new()),
            (
                "crates/ling-lsp/Cargo.toml",
                "[package]\nname = \"not-ling-lsp\"\n".to_owned(),
            ),
            (
                "docs/governance/protocol-inventory.toml",
                "[[protocol]]\nid = \"OTHER\"\n".to_owned(),
            ),
        ]);
        let errors = validate_current_evidence_sources(&sources);
        assert!(errors.iter().any(|error| error.contains("crates/ling-lsp")));
        assert!(errors.iter().any(|error| error.contains("../ling-lsp")));
        assert!(
            errors
                .iter()
                .any(|error| error.contains("PROTO-LSP-LIFECYCLE"))
        );
        assert!(errors.iter().any(|error| error.contains("run_stdio")));
    }
}
