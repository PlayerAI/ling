use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const WORKFLOW_PATH: &str = ".github/workflows/ci.yml";
const CARGO_CONFIG_PATH: &str = ".cargo/config.toml";
const REQUIRED_GATES: &[GateContract] = &[
    GateContract {
        name: "governance-authority",
        commands: &[
            "cargo xtask governance check-authority",
            "cargo xtask governance check-lifecycle",
            "cargo xtask governance check-all",
            "cargo xtask docs verify",
            "cargo xtask examples verify",
            "cargo xtask tutorial verify",
            "cargo xtask lsp verify",
            "cargo xtask zed verify",
            "cargo xtask zed-extension verify",
            "cargo xtask dap verify",
            "cargo xtask ci verify",
        ],
    },
    GateContract {
        name: "gap-register",
        commands: &["cargo xtask governance check-gaps"],
    },
    GateContract {
        name: "protocol-schema",
        commands: &[
            "cargo xtask governance check-protocols",
            "cargo xtask schema validate-all",
            "cargo xtask schema compatibility --from N-1 --to N",
            "cargo xtask schema corrupt-inputs",
        ],
    },
    GateContract {
        name: "error-code-registry",
        commands: &["cargo xtask governance check-error-codes"],
    },
    GateContract {
        name: "traceability-links",
        commands: &["cargo xtask traceability verify --release v0.0.1"],
    },
    GateContract {
        name: "support-matrix",
        commands: &["cargo xtask support verify", "cargo xtask status verify"],
    },
    GateContract {
        name: "canonical-determinism",
        commands: &[
            "cargo test --package ling-semantic hello_snapshot_is_byte_deterministic --locked --offline",
            "cargo test --package ling-cli --test conformance output_is_deterministic --locked --offline",
        ],
    },
    GateContract {
        name: "seed-reproducibility",
        commands: &[
            "cargo xtask fault verify",
            "cargo xtask fuzz verify",
            "cargo xtask security verify",
            "cargo xtask performance verify",
            "cargo run --package unicode-gen --locked --offline",
            "git diff --exit-code -- crates/ling-unicode/src/generated.rs editors/tree-sitter-ling/src/unicode-identifiers.generated.js",
            "cargo xtask seed reproduce",
            "cargo test --package ling-cli --test conformance seed_examples_check_run_and_emit_semantic_graphs --locked --offline",
        ],
    },
];

struct GateContract {
    name: &'static str,
    commands: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub gate_count: usize,
    pub command_count: usize,
    pub host_count: usize,
}

pub fn verify(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let workflow = fs::read_to_string(root.join(WORKFLOW_PATH))
        .map_err(|error| vec![format!("GOV-CI-0002: cannot read {WORKFLOW_PATH}: {error}")])?;
    let cargo_config = fs::read_to_string(root.join(CARGO_CONFIG_PATH)).map_err(|error| {
        vec![format!(
            "GOV-CI-0002: cannot read {CARGO_CONFIG_PATH}: {error}"
        )]
    })?;
    let errors = validate(&workflow, &cargo_config);
    finish(errors).map(|()| CheckSummary {
        gate_count: REQUIRED_GATES.len(),
        command_count: REQUIRED_GATES.iter().map(|gate| gate.commands.len()).sum(),
        host_count: 3,
    })
}

fn validate(workflow: &str, cargo_config: &str) -> Vec<String> {
    let workflow = normalize_newlines(workflow);
    let cargo_config = normalize_newlines(cargo_config);
    let mut errors = Vec::new();

    if !workflow.contains("on:\n  pull_request:\n  push:\n    branches: [main]\n") {
        errors.push("GOV-CI-0003: CI must run on every pull request and main push".to_owned());
    }
    if workflow.lines().any(|line| {
        matches!(
            line.trim().split_once(':').map(|(key, _)| key),
            Some("paths" | "paths-ignore")
        )
    }) {
        errors.push(
            "GOV-CI-0003: path filters are forbidden; always-on G0 gates are the required conservative superset"
                .to_owned(),
        );
    }
    if !workflow.contains("permissions:\n  contents: read\n") {
        errors.push("GOV-CI-0003: CI permissions must remain contents: read".to_owned());
    }
    if !workflow.contains("  g0-gates:\n    name: ${{ matrix.name }}\n    runs-on: ubuntu-latest\n")
        || !workflow.contains("      fail-fast: false\n")
        || !workflow.contains("        run: ${{ matrix.command }}\n")
    {
        errors.push(
            "GOV-CI-0004: g0-gates must use the named fail-fast=false Ubuntu command matrix"
                .to_owned(),
        );
    }

    let gates = extract_gate_blocks(&workflow, &mut errors);
    let expected_names = REQUIRED_GATES
        .iter()
        .map(|gate| gate.name)
        .collect::<BTreeSet<_>>();
    let actual_names = gates.keys().map(String::as_str).collect::<BTreeSet<_>>();
    if actual_names != expected_names {
        errors.push(format!(
            "GOV-CI-0001: G0 gate set differs; expected {expected_names:?}, found {actual_names:?}"
        ));
    }
    for contract in REQUIRED_GATES {
        let Some(block) = gates.get(contract.name) else {
            continue;
        };
        let lines = block.lines().map(str::trim).collect::<BTreeSet<_>>();
        for command in contract.commands {
            let inline = format!("command: {command}");
            if !lines.contains(command) && !lines.contains(inline.as_str()) {
                errors.push(format!(
                    "GOV-CI-0005: gate {} is missing command {command:?}",
                    contract.name
                ));
            }
        }
    }

    if !workflow.contains("matrix:\n        os: [ubuntu-latest, macos-latest, windows-latest]") {
        errors.push(
            "GOV-CI-0006: workspace tests must retain Ubuntu, macOS, and Windows runners"
                .to_owned(),
        );
    }
    for required in [
        "cargo fmt --all -- --check",
        "cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings",
        "cargo test --workspace --all-features --locked --offline",
        "cargo doc --workspace --all-features --no-deps --locked --offline",
        "cargo build --workspace --all-features --release --locked --offline",
        "name: fuzz corpus smoke",
        "RUSTUP_TOOLCHAIN: nightly-2026-08-15",
        "cargo fuzz run manifest_bytes fuzz/corpus/manifest_bytes -- -runs=256",
        "name: Rust 1.85 MSRV",
        "toolchain: \"1.85\"",
        "cargo check --workspace --all-features --locked --offline",
    ] {
        if !workflow.contains(required) {
            errors.push(format!(
                "GOV-CI-0006: CI is missing required platform, quality, fuzz, or MSRV evidence {required:?}"
            ));
        }
    }
    if !cargo_config.contains("xtask = \"run --locked --offline --package xtask --\"") {
        errors.push("GOV-CI-0007: cargo xtask alias must remain locked and offline".to_owned());
    }
    errors
}

fn extract_gate_blocks(workflow: &str, errors: &mut Vec<String>) -> BTreeMap<String, String> {
    let Some((_, matrix_tail)) = workflow.split_once("      matrix:\n        include:\n") else {
        errors.push("GOV-CI-0004: cannot find G0 matrix include section".to_owned());
        return BTreeMap::new();
    };
    let Some((matrix, _)) = matrix_tail.split_once("\n    steps:\n") else {
        errors.push("GOV-CI-0004: cannot find end of G0 matrix include section".to_owned());
        return BTreeMap::new();
    };
    let marker = "          - name: ";
    let mut gates = BTreeMap::new();
    let mut current_name: Option<String> = None;
    let mut current_block = String::new();
    for line in matrix.lines() {
        if let Some(name) = line.strip_prefix(marker) {
            if let Some(previous) = current_name.take() {
                if gates
                    .insert(previous.clone(), current_block.clone())
                    .is_some()
                {
                    errors.push(format!("GOV-CI-0001: duplicate G0 gate {previous}"));
                }
                current_block.clear();
            }
            current_name = Some(name.trim().to_owned());
        }
        if current_name.is_some() {
            current_block.push_str(line);
            current_block.push('\n');
        }
    }
    if let Some(previous) = current_name {
        if gates.insert(previous.clone(), current_block).is_some() {
            errors.push(format!("GOV-CI-0001: duplicate G0 gate {previous}"));
        }
    }
    gates
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
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

    fn repository_inputs() -> (String, String) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        (
            fs::read_to_string(root.join(WORKFLOW_PATH)).expect("CI workflow is readable"),
            fs::read_to_string(root.join(CARGO_CONFIG_PATH)).expect("Cargo config is readable"),
        )
    }

    #[test]
    fn repository_ci_contract_is_complete() {
        let (workflow, cargo_config) = repository_inputs();
        assert!(validate(&workflow, &cargo_config).is_empty());
    }

    #[test]
    fn rejects_missing_gate_and_path_filtering() {
        let (workflow, cargo_config) = repository_inputs();
        let changed = normalize_newlines(&workflow)
            .replace(
                "          - name: gap-register",
                "          - name: omitted-gap",
            )
            .replace(
                "  pull_request:\n",
                "  pull_request:\n    paths: [docs/**]\n",
            );
        let errors = validate(&changed, &cargo_config);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("gate set differs"))
        );
        assert!(errors.iter().any(|error| error.contains("path filters")));
    }

    #[test]
    fn rejects_unlocked_xtask_alias_and_missing_gate_command() {
        let (workflow, cargo_config) = repository_inputs();
        let workflow = workflow.replace(
            "cargo xtask schema corrupt-inputs",
            "cargo xtask schema omitted-corrupt-inputs",
        );
        let cargo_config = cargo_config.replace("--locked --offline ", "");
        let errors = validate(&workflow, &cargo_config);
        assert!(errors.iter().any(|error| error.contains("corrupt-inputs")));
        assert!(
            errors
                .iter()
                .any(|error| error.contains("locked and offline"))
        );
    }
}
