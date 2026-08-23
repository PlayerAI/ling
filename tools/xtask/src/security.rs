use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MATRIX_PATH: &str = "docs/testing/SECURITY-AUDIT.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Surface {
    name: &'static str,
    state: &'static str,
}

const SURFACES: &[Surface] = &[
    Surface {
        name: "Rust `unsafe`",
        state: "Covered for current crates",
    },
    Surface {
        name: "FFI / Target Primitive TCB",
        state: "Deferred",
    },
    Surface {
        name: "Deserializers",
        state: "Covered for implemented schemas",
    },
    Surface {
        name: "Package extraction / build sandbox",
        state: "Partial",
    },
    Surface {
        name: "Capability enforcement",
        state: "Covered for Seed effects",
    },
    Surface {
        name: "Remote protocol",
        state: "Deferred",
    },
    Surface {
        name: "Replay / evidence sensitive data",
        state: "Deferred",
    },
    Surface {
        name: "Zed extension binary download / verification",
        state: "Deferred",
    },
    Surface {
        name: "Dependency / license / SBOM",
        state: "Partial",
    },
];

const REQUIRED_POLICY_PHRASES: &[&str] = &[
    "threat model and trust-boundary inventory",
    "accepted security decisions",
    "deterministic hostile-input",
    "reproducible advisory, license, SBOM, checksum, and provenance reports",
    "incident/disclosure process",
    "No security feature is inferred",
    "No security API is inferred",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub surface_count: usize,
    pub covered_count: usize,
    pub partial_count: usize,
    pub deferred_count: usize,
    pub workspace_member_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let matrix = fs::read_to_string(root.join(MATRIX_PATH)).map_err(|error| {
        vec![format!(
            "GOV-SECURITY-0001: cannot read {MATRIX_PATH}: {error}"
        )]
    })?;
    let mut errors = validate(&matrix);
    let workspace_member_count = validate_workspace_unsafe_policy(root, &mut errors);
    finish(errors).map(|()| CheckSummary {
        surface_count: SURFACES.len(),
        covered_count: SURFACES
            .iter()
            .filter(|surface| surface.state.starts_with("Covered"))
            .count(),
        partial_count: SURFACES
            .iter()
            .filter(|surface| surface.state == "Partial")
            .count(),
        deferred_count: SURFACES
            .iter()
            .filter(|surface| surface.state == "Deferred")
            .count(),
        workspace_member_count,
    })
}

fn validate_workspace_unsafe_policy(root: &Path, errors: &mut Vec<String>) -> usize {
    let Some(workspace_manifest) = read_manifest(&root.join("Cargo.toml"), errors) else {
        return 0;
    };
    let unsafe_level = workspace_unsafe_level(&workspace_manifest);
    if unsafe_level != Some("deny") {
        errors.push(format!(
            "GOV-SECURITY-0007: workspace.lints.rust.unsafe_code must be \"deny\", found {unsafe_level:?}"
        ));
    }

    let Some(members) = workspace_manifest
        .get("workspace")
        .and_then(|value| value.get("members"))
        .and_then(toml::Value::as_array)
    else {
        errors.push("GOV-SECURITY-0007: workspace.members must be an array".to_owned());
        return 0;
    };

    let mut member_count = 0;
    let mut seen = std::collections::BTreeSet::new();
    for member in members {
        let Some(member) = member.as_str() else {
            errors.push("GOV-SECURITY-0008: every workspace member must be a string".to_owned());
            continue;
        };
        if !safe_member_path(member) {
            errors.push(format!(
                "GOV-SECURITY-0008: workspace member path is not a safe explicit relative path: {member:?}"
            ));
            continue;
        }
        if !seen.insert(member) {
            errors.push(format!(
                "GOV-SECURITY-0008: duplicate workspace member {member:?}"
            ));
            continue;
        }
        member_count += 1;

        let manifest_path = root.join(member).join("Cargo.toml");
        let Some(manifest) = read_manifest(&manifest_path, errors) else {
            continue;
        };
        let inherits = member_inherits_workspace_lints(&manifest);
        if inherits != Some(true) {
            errors.push(format!(
                "GOV-SECURITY-0008: {member}/Cargo.toml must set lints.workspace = true, found {inherits:?}"
            ));
        }
    }
    member_count
}

fn workspace_unsafe_level(manifest: &toml::Value) -> Option<&str> {
    manifest
        .get("workspace")
        .and_then(|value| value.get("lints"))
        .and_then(|value| value.get("rust"))
        .and_then(|value| value.get("unsafe_code"))
        .and_then(toml::Value::as_str)
}

fn member_inherits_workspace_lints(manifest: &toml::Value) -> Option<bool> {
    manifest
        .get("lints")
        .and_then(|value| value.get("workspace"))
        .and_then(toml::Value::as_bool)
}

fn read_manifest(path: &Path, errors: &mut Vec<String>) -> Option<toml::Value> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            errors.push(format!(
                "GOV-SECURITY-0006: cannot read {}: {error}",
                path.display()
            ));
            return None;
        }
    };
    match toml::from_str(&text) {
        Ok(manifest) => Some(manifest),
        Err(error) => {
            errors.push(format!(
                "GOV-SECURITY-0006: cannot parse {}: {error}",
                path.display()
            ));
            None
        }
    }
}

fn safe_member_path(member: &str) -> bool {
    !member.is_empty()
        && !member.contains(['*', '?', '[', ']', '\\', ':'])
        && member
            .split('/')
            .all(|component| !component.is_empty() && component != "." && component != "..")
}

fn validate(matrix: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let rows = matrix.lines().filter_map(parse_row).collect::<Vec<_>>();
    let expected = SURFACES
        .iter()
        .map(|surface| (surface.name, surface.state))
        .collect::<BTreeMap<_, _>>();
    let mut actual = BTreeMap::new();
    for (name, state) in rows {
        if actual.insert(name.to_owned(), state.to_owned()).is_some() {
            errors.push(format!(
                "GOV-SECURITY-0002: duplicate security surface {name:?}"
            ));
        }
    }

    let actual_names = actual.keys().map(String::as_str).collect::<Vec<_>>();
    let expected_names = expected.keys().copied().collect::<Vec<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-SECURITY-0003: surface set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for (name, state) in expected {
        if actual.get(name).map(String::as_str) != Some(state) {
            errors.push(format!(
                "GOV-SECURITY-0004: surface {name:?} must be {state:?}, found {:?}",
                actual.get(name)
            ));
        }
    }

    for required in REQUIRED_POLICY_PHRASES {
        if !matrix.contains(required) {
            errors.push(format!(
                "GOV-SECURITY-0005: {MATRIX_PATH} is missing policy phrase {required:?}"
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
        || cells[0] == "Required audit surface"
        || cells[0].chars().all(|character| character == '-')
    {
        return None;
    }
    Some((cells[0], cells[2]))
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
    fn repository_security_matrix_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("security matrix is valid");
        assert_eq!(summary.surface_count, 9);
        assert_eq!(summary.covered_count, 3);
        assert_eq!(summary.partial_count, 2);
        assert_eq!(summary.deferred_count, 4);
        assert_eq!(summary.workspace_member_count, 22);
    }

    #[test]
    fn rejects_security_state_drift() {
        let matrix = "| Required audit surface | Current control / evidence | State | Deferred work |\n| --- | --- | --- | --- |\n| Rust `unsafe` | x | Deferred | y |\n";
        let errors = validate(matrix);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("surface set differs"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("must be \"Covered for current crates\""))
        );
    }

    #[test]
    fn rejects_unsafe_member_paths() {
        for path in [
            "",
            ".",
            "../outside",
            "/absolute",
            "C:/absolute",
            "crates/*",
            "crates/../other",
            "crates//other",
            "crates\\other",
        ] {
            assert!(!safe_member_path(path), "unexpected safe path: {path}");
        }
        assert!(safe_member_path("crates/ling-ast"));
        assert!(safe_member_path("tools/xtask"));
    }

    #[test]
    fn detects_policy_and_inheritance_drift() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let root_manifest =
            fs::read_to_string(root.join("Cargo.toml")).expect("workspace manifest is readable");
        let weakened =
            root_manifest.replacen("unsafe_code = \"deny\"", "unsafe_code = \"warn\"", 1);
        let weakened: toml::Value = toml::from_str(&weakened).expect("mutated manifest parses");
        assert_eq!(workspace_unsafe_level(&weakened), Some("warn"));

        let opted_out: toml::Value =
            toml::from_str("[lints]\nworkspace = false\n").expect("member manifest parses");
        assert_eq!(member_inherits_workspace_lints(&opted_out), Some(false));
    }
}
