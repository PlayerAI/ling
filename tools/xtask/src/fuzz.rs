use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const MANIFEST_PATH: &str = "fuzz/Cargo.toml";
const INVENTORY_PATH: &str = "docs/testing/FUZZ-COVERAGE.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Harness {
    name: &'static str,
    target_path: &'static str,
    corpus_dir: &'static str,
    seed_count: usize,
}

const HARNESSES: &[Harness] = &[
    Harness {
        name: "source_bytes",
        target_path: "fuzz/fuzz_targets/source_bytes.rs",
        corpus_dir: "source_bytes",
        seed_count: 2,
    },
    Harness {
        name: "lexer_utf8",
        target_path: "fuzz/fuzz_targets/lexer_utf8.rs",
        corpus_dir: "lexer_utf8",
        seed_count: 2,
    },
    Harness {
        name: "parser_utf8",
        target_path: "fuzz/fuzz_targets/parser_utf8.rs",
        corpus_dir: "parser_utf8",
        seed_count: 5,
    },
    Harness {
        name: "formatter_utf8",
        target_path: "fuzz/fuzz_targets/formatter_utf8.rs",
        corpus_dir: "formatter_utf8",
        seed_count: 1,
    },
    Harness {
        name: "audit_schema_bytes",
        target_path: "fuzz/fuzz_targets/audit_schema_bytes.rs",
        corpus_dir: "audit_schema_bytes",
        seed_count: 1,
    },
    Harness {
        name: "semantic_schema_bytes",
        target_path: "fuzz/fuzz_targets/semantic_schema_bytes.rs",
        corpus_dir: "semantic_schema_bytes",
        seed_count: 2,
    },
    Harness {
        name: "manifest_bytes",
        target_path: "fuzz/fuzz_targets/manifest_bytes.rs",
        corpus_dir: "manifest_bytes",
        seed_count: 4,
    },
    Harness {
        name: "lock_bytes",
        target_path: "fuzz/fuzz_targets/lock_bytes.rs",
        corpus_dir: "lock_bytes",
        seed_count: 1,
    },
    Harness {
        name: "bytecode_bytes",
        target_path: "fuzz/fuzz_targets/bytecode_bytes.rs",
        corpus_dir: "bytecode_bytes",
        seed_count: 2,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub target_count: usize,
    pub corpus_file_count: usize,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let manifest_path = root.join(MANIFEST_PATH);
    let manifest = fs::read_to_string(&manifest_path).map_err(|error| {
        vec![format!(
            "GOV-FUZZ-0001: cannot read {MANIFEST_PATH}: {error}"
        )]
    })?;
    let inventory = fs::read_to_string(root.join(INVENTORY_PATH)).map_err(|error| {
        vec![format!(
            "GOV-FUZZ-0001: cannot read {INVENTORY_PATH}: {error}"
        )]
    })?;
    let errors = validate(root, &manifest, &inventory);
    finish(errors).map(|()| CheckSummary {
        target_count: HARNESSES.len(),
        corpus_file_count: HARNESSES.iter().map(|harness| harness.seed_count).sum(),
    })
}

fn validate(root: &Path, manifest_text: &str, inventory: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let manifest = match toml::from_str::<toml::Value>(manifest_text) {
        Ok(value) => value,
        Err(error) => {
            errors.push(format!(
                "GOV-FUZZ-0002: {MANIFEST_PATH} is invalid TOML: {error}"
            ));
            return errors;
        }
    };

    if manifest
        .get("package")
        .and_then(toml::Value::as_table)
        .and_then(|package| package.get("metadata"))
        .and_then(toml::Value::as_table)
        .and_then(|metadata| metadata.get("cargo-fuzz"))
        .and_then(toml::Value::as_bool)
        != Some(true)
    {
        errors.push(
            "GOV-FUZZ-0003: fuzz/Cargo.toml must declare package.metadata.cargo-fuzz = true"
                .to_owned(),
        );
    }

    let expected_names = HARNESSES
        .iter()
        .map(|harness| harness.name)
        .collect::<BTreeSet<_>>();
    let mut actual_bins = BTreeMap::new();
    match manifest.get("bin").and_then(toml::Value::as_array) {
        Some(bins) => {
            for (index, bin) in bins.iter().enumerate() {
                let Some(table) = bin.as_table() else {
                    errors.push(format!(
                        "GOV-FUZZ-0004: [[bin]] entry {index} must be a TOML table"
                    ));
                    continue;
                };
                let Some(name) = table.get("name").and_then(toml::Value::as_str) else {
                    errors.push(format!(
                        "GOV-FUZZ-0004: [[bin]] entry {index} is missing a string name"
                    ));
                    continue;
                };
                if actual_bins.insert(name.to_owned(), table.clone()).is_some() {
                    errors.push(format!("GOV-FUZZ-0004: duplicate fuzz target {name:?}"));
                }
            }
        }
        None => errors.push("GOV-FUZZ-0004: fuzz/Cargo.toml has no [[bin]] targets".to_owned()),
    }

    let actual_names = actual_bins
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-FUZZ-0005: fuzz target set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }

    for harness in HARNESSES {
        let Some(bin) = actual_bins.get(harness.name) else {
            continue;
        };
        let path = bin.get("path").and_then(toml::Value::as_str);
        if path
            != Some(
                harness
                    .target_path
                    .strip_prefix("fuzz/")
                    .unwrap_or(harness.target_path),
            )
        {
            errors.push(format!(
                "GOV-FUZZ-0006: target {} path must be {:?}, found {:?}",
                harness.name,
                harness
                    .target_path
                    .strip_prefix("fuzz/")
                    .unwrap_or(harness.target_path),
                path
            ));
        }
        for field in ["test", "doc", "bench"] {
            if bin.get(field).and_then(toml::Value::as_bool) != Some(false) {
                errors.push(format!(
                    "GOV-FUZZ-0006: target {} must set {field} = false",
                    harness.name
                ));
            }
        }

        let target_path = root.join(harness.target_path);
        match fs::read_to_string(&target_path) {
            Ok(source) if source.contains("fuzz_target!") => {}
            Ok(_) => errors.push(format!(
                "GOV-FUZZ-0007: target {} does not contain a libFuzzer entry",
                harness.name
            )),
            Err(error) => errors.push(format!(
                "GOV-FUZZ-0007: cannot read {}: {error}",
                harness.target_path
            )),
        }

        let corpus_path = root.join("fuzz/corpus").join(harness.corpus_dir);
        match corpus_files(&corpus_path) {
            Ok(count) if count == harness.seed_count => {}
            Ok(count) => errors.push(format!(
                "GOV-FUZZ-0008: corpus {} has {count} files; expected {}",
                harness.corpus_dir, harness.seed_count
            )),
            Err(error) => errors.push(format!(
                "GOV-FUZZ-0008: cannot inspect corpus {}: {error}",
                harness.corpus_dir
            )),
        }

        if !inventory.contains(harness.name) {
            errors.push(format!(
                "GOV-FUZZ-0009: {INVENTORY_PATH} does not name target {}",
                harness.name
            ));
        }
    }

    errors
}

fn corpus_files(path: &Path) -> Result<usize, String> {
    let entries = fs::read_dir(path).map_err(|error| error.to_string())?;
    let mut count = 0;
    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let file_type = entry.file_type().map_err(|error| error.to_string())?;
        if file_type.is_dir() {
            return Err(format!(
                "nested directory {} is not allowed",
                entry.path().display()
            ));
        }
        if !file_type.is_file() {
            return Err(format!(
                "non-regular entry {} is not allowed",
                entry.path().display()
            ));
        }
        count += 1;
    }
    Ok(count)
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
    fn repository_fuzz_inventory_is_deterministic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("fuzz inventory is valid");
        assert_eq!(summary.target_count, 9);
        assert_eq!(summary.corpus_file_count, 20);
    }

    #[test]
    fn rejects_target_set_drift() {
        let manifest = r#"
[package]
[package.metadata]
cargo-fuzz = true

[[bin]]
name = "unexpected"
path = "fuzz_targets/unexpected.rs"
test = false
doc = false
bench = false
"#;
        let errors = validate(Path::new("."), manifest, "");
        assert!(
            errors
                .iter()
                .any(|error| error.contains("target set differs"))
        );
    }
}
