use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/ZED-COMPATIBILITY-MATRIX.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Surface {
    name: &'static str,
    state: &'static str,
}

const SURFACES: &[Surface] = &[
    Surface {
        name: "Zed minimum/tested version",
        state: "Not established; no support claim",
    },
    Surface {
        name: "Ling compiler",
        state: "Seed compiler evidence only; no Zed integration",
    },
    Surface {
        name: "LSP executable/version",
        state: "Preview lifecycle/overlay only; no Zed compatibility range",
    },
    Surface {
        name: "Tree-sitter grammar",
        state: "Editor-only implementation; no Stable node compatibility",
    },
    Surface {
        name: "Grammar revision",
        state: "Pinned for this evidence snapshot, not a public Zed release tag",
    },
    Surface {
        name: "Tree-sitter CLI / Node",
        state: "Locked development toolchain; no consumer guarantee",
    },
    Surface {
        name: "Protocol/schema",
        state: "No Stable editor schema",
    },
    Surface {
        name: "Operating systems",
        state: "Windows grammar suite verified locally; no Zed OS support matrix",
    },
    Surface {
        name: "Binary acquisition",
        state: "Source-built ling CLI only; no Zed acquisition contract",
    },
    Surface {
        name: "Known limitations",
        state: "Explicitly unavailable; grammar-only development surface",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "not a Zed release or a Stable editor-support claim",
    "Tree-sitter package is an editor-oriented, tolerant parser",
    "Unknown values are recorded as `Not established`",
    "No Zed protocol",
    "`ling.semantic/0.1` is Experimental and `ling.audit/0.1` is Preview",
    "`ling lsp --stdio`",
    "Windows grammar suite passed",
    "Unicode 17.0.0",
    "original UTF-8 byte spans",
    "No placeholder command, download, protocol, backend, schema, or editor promise",
];

const REQUIRED_PACKAGE_MARKERS: &[(&str, &[&str])] = &[
    (
        "editors/tree-sitter-ling/package.json",
        &[
            "\"name\": \"tree-sitter-ling\"",
            "\"version\": \"0.0.1-dev\"",
            "\"tree-sitter-cli\": \"0.26.12\"",
            "\"node\": \">=20\"",
        ],
    ),
    (
        "editors/tree-sitter-ling/package-lock.json",
        &[
            "\"tree-sitter-cli\": \"0.26.12\"",
            "\"version\": \"0.26.12\"",
            "\"node\": \">=20\"",
        ],
    ),
    (
        "editors/tree-sitter-ling/tree-sitter.json",
        &[
            "\"scope\": \"source.ling\"",
            "\"file-types\":",
            "\"highlights\": \"queries/highlights.scm\"",
        ],
    ),
    (
        "editors/tree-sitter-ling/README.md",
        &[
            "Unicode 17.0.0",
            "npm test",
            "The grammar is not an authority",
        ],
    ),
    (
        "editors/tree-sitter-ling/KNOWN-DIFFERENCES.md",
        &[
            "None of these differences changes Ling syntax or semantics",
            "Unicode 17.0.0",
            "Tree-sitter node names",
        ],
    ),
];

const REQUIRED_LSP_MARKERS: &[(&str, &[&str])] = &[
    ("Cargo.toml", &["\"crates/ling-lsp\""]),
    ("crates/ling-lsp/Cargo.toml", &["name = \"ling-lsp\""]),
    (
        "docs/governance/protocol-inventory.toml",
        &[
            "id = \"PROTO-LSP-LIFECYCLE\"",
            "current_version = \"ling.lsp.lifecycle/0.1\"",
            "id = \"PROTO-LSP-OVERLAY\"",
            "current_version = \"ling.lsp.overlay/0.2\"",
            "ling lsp --stdio",
        ],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub surface_count: usize,
    pub package_file_count: usize,
    pub structured_json_count: usize,
    pub lsp_evidence_file_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-ZED-MATRIX-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate_matrix(&matrix);
    errors.extend(validate_package(root));
    errors.extend(validate_lsp_evidence(root));
    finish(errors).map(|()| CheckSummary {
        surface_count: SURFACES.len(),
        package_file_count: REQUIRED_PACKAGE_MARKERS.len(),
        structured_json_count: 3,
        lsp_evidence_file_count: REQUIRED_LSP_MARKERS.len(),
    })
}

fn validate_matrix(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(section) = matrix
        .split_once("## Matrix")
        .and_then(|(_, remainder)| remainder.split_once("## Existing grammar evidence"))
        .map(|(section, _)| section)
    else {
        return vec![format!(
            "GOV-ZED-MATRIX-0002: {MATRIX_PATH} is missing the Matrix section"
        )];
    };

    let rows = section.lines().filter_map(parse_row).collect::<Vec<_>>();
    let mut actual = BTreeMap::new();
    for (name, cells) in rows {
        if actual.insert(name.to_owned(), cells.to_owned()).is_some() {
            errors.push(format!(
                "GOV-ZED-MATRIX-0003: duplicate compatibility surface {name:?}"
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
            "GOV-ZED-MATRIX-0004: compatibility surface set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for surface in SURFACES {
        let Some(cells) = actual.get(surface.name) else {
            continue;
        };
        if cells.len() < 2 || cells.iter().any(|cell| cell.is_empty()) {
            errors.push(format!(
                "GOV-ZED-MATRIX-0005: compatibility surface {:?} has an empty evidence/state cell",
                surface.name
            ));
            continue;
        }
        if cells[1] != surface.state {
            errors.push(format!(
                "GOV-ZED-MATRIX-0006: compatibility surface {:?} must have state {:?}, found {:?}",
                surface.name, surface.state, cells[1]
            ));
        }
    }

    let normalized = normalize(matrix);
    for required in REQUIRED_POLICY_PHRASES {
        if !normalized.contains(&normalize(required)) {
            errors.push(format!(
                "GOV-ZED-MATRIX-0007: {MATRIX_PATH} is missing policy phrase {required:?}"
            ));
        }
    }
    errors
}

fn validate_package(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    for (path, markers) in REQUIRED_PACKAGE_MARKERS {
        let text = match fs::read_to_string(root.join(path)) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!("GOV-ZED-MATRIX-0008: cannot read {path}: {error}"));
                continue;
            }
        };
        errors.extend(validate_package_text(path, &text, markers));
    }
    errors.extend(validate_package_json(root));
    errors
}

fn validate_lsp_evidence(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    for (path, markers) in REQUIRED_LSP_MARKERS {
        let text = match fs::read_to_string(root.join(path)) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!(
                    "GOV-ZED-MATRIX-0012: cannot read LSP evidence {path}: {error}"
                ));
                continue;
            }
        };
        for marker in *markers {
            if !text.contains(marker) {
                errors.push(format!(
                    "GOV-ZED-MATRIX-0013: {path} is missing LSP evidence marker {marker:?}"
                ));
            }
        }
    }
    errors
}

fn validate_package_json(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let package = read_json(root, "editors/tree-sitter-ling/package.json", &mut errors);
    if let Some(package) = package {
        expect_json_string(&package, &["name"], "tree-sitter-ling", &mut errors);
        expect_json_string(&package, &["version"], "0.0.1-dev", &mut errors);
        expect_json_bool(&package, &["private"], true, &mut errors);
        expect_json_string(
            &package,
            &["devDependencies", "tree-sitter-cli"],
            "0.26.12",
            &mut errors,
        );
        expect_json_string(&package, &["engines", "node"], ">=20", &mut errors);
        expect_json_string(
            &package,
            &["scripts", "verify"],
            "npm run generate && npm test && npm run parse:examples",
            &mut errors,
        );
    }

    let lock = read_json(
        root,
        "editors/tree-sitter-ling/package-lock.json",
        &mut errors,
    );
    if let Some(lock) = lock {
        expect_json_u64(&lock, &["lockfileVersion"], 3, &mut errors);
        expect_json_string(
            &lock,
            &["packages", "", "devDependencies", "tree-sitter-cli"],
            "0.26.12",
            &mut errors,
        );
        expect_json_string(
            &lock,
            &["packages", "", "engines", "node"],
            ">=20",
            &mut errors,
        );
        expect_json_string(
            &lock,
            &["packages", "node_modules/tree-sitter-cli", "version"],
            "0.26.12",
            &mut errors,
        );
    }

    let grammar = read_json(
        root,
        "editors/tree-sitter-ling/tree-sitter.json",
        &mut errors,
    );
    if let Some(grammar) = grammar {
        expect_json_string(&grammar, &["grammars", "0", "name"], "ling", &mut errors);
        expect_json_string(
            &grammar,
            &["grammars", "0", "scope"],
            "source.ling",
            &mut errors,
        );
        expect_json_string(
            &grammar,
            &["grammars", "0", "file-types", "0"],
            "ling",
            &mut errors,
        );
        expect_json_string(
            &grammar,
            &["grammars", "0", "highlights"],
            "queries/highlights.scm",
            &mut errors,
        );
        expect_json_string(&grammar, &["metadata", "version"], "0.0.1-dev", &mut errors);
    }
    errors
}

fn read_json(root: &Path, path: &str, errors: &mut Vec<String>) -> Option<serde_json::Value> {
    let source = match fs::read_to_string(root.join(path)) {
        Ok(source) => source,
        Err(error) => {
            errors.push(format!("GOV-ZED-MATRIX-0010: cannot read {path}: {error}"));
            return None;
        }
    };
    match serde_json::from_str(&source) {
        Ok(value) => Some(value),
        Err(error) => {
            errors.push(format!("GOV-ZED-MATRIX-0010: cannot parse {path}: {error}"));
            None
        }
    }
}

fn json_value<'a>(value: &'a serde_json::Value, path: &[&str]) -> Option<&'a serde_json::Value> {
    path.iter().try_fold(value, |current, component| {
        component
            .parse::<usize>()
            .ok()
            .and_then(|index| current.as_array()?.get(index))
            .or_else(|| current.as_object()?.get(*component))
    })
}

fn expect_json_string(
    value: &serde_json::Value,
    path: &[&str],
    expected: &str,
    errors: &mut Vec<String>,
) {
    if json_value(value, path).and_then(serde_json::Value::as_str) != Some(expected) {
        errors.push(format!(
            "GOV-ZED-MATRIX-0011: JSON field {} must equal {expected:?}",
            path.join(".")
        ));
    }
}

fn expect_json_bool(
    value: &serde_json::Value,
    path: &[&str],
    expected: bool,
    errors: &mut Vec<String>,
) {
    if json_value(value, path).and_then(serde_json::Value::as_bool) != Some(expected) {
        errors.push(format!(
            "GOV-ZED-MATRIX-0011: JSON field {} must equal {expected}",
            path.join(".")
        ));
    }
}

fn expect_json_u64(
    value: &serde_json::Value,
    path: &[&str],
    expected: u64,
    errors: &mut Vec<String>,
) {
    if json_value(value, path).and_then(serde_json::Value::as_u64) != Some(expected) {
        errors.push(format!(
            "GOV-ZED-MATRIX-0011: JSON field {} must equal {expected}",
            path.join(".")
        ));
    }
}

fn validate_package_text(path: &str, text: &str, markers: &[&str]) -> Vec<String> {
    let normalized = normalize(text);
    markers
        .iter()
        .filter(|marker| !normalized.contains(&normalize(marker)))
        .map(|marker| format!("GOV-ZED-MATRIX-0009: {path} is missing package marker {marker:?}"))
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
    if cells.len() < 3
        || cells[0] == "Surface"
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
    fn repository_zed_matrix_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("Zed matrix is valid");
        assert_eq!(summary.surface_count, 10);
        assert_eq!(summary.package_file_count, 5);
        assert_eq!(summary.structured_json_count, 3);
        assert_eq!(summary.lsp_evidence_file_count, 3);
    }

    #[test]
    fn rejects_compatibility_state_drift() {
        let matrix = "## Matrix\n| Surface | Current evidence | Compatibility state |\n| --- | --- | --- |\n| Ling compiler | x | Unsupported |\n## Existing grammar evidence\n";
        let errors = validate_matrix(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("surface set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("must have state")));
    }

    #[test]
    fn rejects_package_marker_drift() {
        let errors = validate_package_text(
            "package.json",
            "{ \"name\": \"tree-sitter-ling\" }",
            &["\"name\": \"tree-sitter-ling\"", "\"node\": \">=20\""],
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing package marker"))
        );
    }

    #[test]
    fn rejects_structured_package_metadata_drift() {
        let value = serde_json::json!({
            "name": "tree-sitter-ling",
            "version": "0.0.1-dev",
            "private": false
        });
        let mut errors = Vec::new();
        expect_json_bool(&value, &["private"], true, &mut errors);
        expect_json_string(
            &value,
            &["devDependencies", "tree-sitter-cli"],
            "0.26.12",
            &mut errors,
        );
        assert_eq!(errors.len(), 2);
        assert!(errors.iter().all(|error| error.contains("JSON field")));
    }
}
