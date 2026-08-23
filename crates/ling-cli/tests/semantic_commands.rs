use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct TempRoot(PathBuf);

impl TempRoot {
    fn new() -> Self {
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ling-cli-semantic-commands-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("semantic-command test root is creatable");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn ling(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(arguments)
        .output()
        .expect("ling process runs")
}

fn semantic(source: &Path) -> serde_json::Value {
    let output = Command::new(env!("CARGO_BIN_EXE_ling"))
        .arg("semantic")
        .arg(source)
        .output()
        .expect("ling semantic runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("semantic output is JSON")
}

fn transaction(base: &serde_json::Value, target: &str, content: &str) -> serde_json::Value {
    serde_json::json!({
        "schema": "ling.semantic-transaction/0.1",
        "base_program_id": base["program_id"],
        "target_ids": [target],
        "operation": {"kind": "replace_source", "content": content},
        "preserve": ["definition_set", "types", "effects", "capabilities"],
        "provenance": {"actor": "conformance", "reason": "authorized body update"}
    })
}

#[test]
fn query_is_exact_nfc_deterministic_and_path_free() {
    let root = TempRoot::new();
    let source = root.path().join("Main.ling");
    fs::write(&source, "module Main\n\nlet é = 1\nlet main () = é\n").unwrap();
    let source_arg = source.to_str().unwrap();

    let first = ling(&[
        "query", "--symbol", "e\u{301}", "--format", "json", source_arg,
    ]);
    let second = ling(&[
        "query", "--symbol", "e\u{301}", "--format", "json", source_arg,
    ]);
    assert!(first.status.success());
    assert_eq!(first.stdout, second.stdout);
    assert!(first.stderr.is_empty());
    let report: serde_json::Value = serde_json::from_slice(&first.stdout).unwrap();
    assert_eq!(report["schema"], "ling.semantic-query/0.1");
    assert_eq!(report["symbol"], "é");
    assert_eq!(report["matches"].as_array().unwrap().len(), 1);
    assert_eq!(report["matches"][0]["name"], "é");
    assert!(!String::from_utf8_lossy(&first.stdout).contains(root.path().to_str().unwrap()));

    let missing = ling(&[
        "query", "--symbol", "missing", "--format", "json", source_arg,
    ]);
    assert!(missing.status.success());
    let missing: serde_json::Value = serde_json::from_slice(&missing.stdout).unwrap();
    assert_eq!(missing["matches"], serde_json::json!([]));
}

#[test]
fn patch_validates_body_change_without_mutating_source() {
    let root = TempRoot::new();
    let source = root.path().join("Main.ling");
    let original = "module Main\n\nlet value = 1\nlet main () = value\n";
    let replacement = "module Main\n\nlet value = 2\nlet main () = value\n";
    fs::write(&source, original).unwrap();
    let graph = semantic(&source);
    let target = graph["definitions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|definition| definition["name"] == "value")
        .unwrap()["definition_id"]
        .as_str()
        .unwrap()
        .to_owned();
    let request = transaction(&graph, &target, replacement);
    let request_path = root.path().join("transaction.json");
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();

    let first = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["patch", "--format", "json"])
        .arg(&request_path)
        .arg(&source)
        .output()
        .unwrap();
    let second = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["patch", "--format", "json"])
        .arg(&request_path)
        .arg(&source)
        .output()
        .unwrap();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert_eq!(first.stdout, second.stdout);
    assert!(first.stderr.is_empty());
    assert_eq!(fs::read_to_string(&source).unwrap(), original);
    assert_eq!(
        fs::read(&request_path).unwrap(),
        serde_json::to_vec(&request).unwrap()
    );

    let report: serde_json::Value = serde_json::from_slice(&first.stdout).unwrap();
    assert_eq!(report["schema"], "ling.semantic-transaction-result/0.1");
    assert_eq!(report["status"], "validated");
    assert_eq!(report["committed"], false);
    assert_eq!(report["changed_body_ids"], serde_json::json!([target]));
}

#[test]
fn patch_rejects_stale_base_and_preserve_drift() {
    let root = TempRoot::new();
    let source = root.path().join("Main.ling");
    fs::write(&source, "module Main\n\nlet value = 1\n").unwrap();
    let graph = semantic(&source);
    let target = graph["definitions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|definition| definition["name"] == "value")
        .unwrap()["definition_id"]
        .as_str()
        .unwrap();
    let request_path = root.path().join("transaction.json");

    let mut stale = transaction(&graph, target, "this candidate must not be compiled");
    stale["base_program_id"] = serde_json::json!("experimental:blake3:stale");
    fs::write(&request_path, serde_json::to_vec(&stale).unwrap()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["patch", "--format", "json"])
        .arg(&request_path)
        .arg(&source)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(diagnostic["code"], "L-TRANSACTION-0002");

    let drift = transaction(&graph, target, "module Main\n\nlet value = \"text\"\n");
    fs::write(&request_path, serde_json::to_vec(&drift).unwrap()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["patch", "--format", "json"])
        .arg(&request_path)
        .arg(&source)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(diagnostic["code"], "L-TRANSACTION-0003");
    assert_eq!(diagnostic["facts"]["constraint"], "types");
    assert_eq!(
        fs::read_to_string(&source).unwrap(),
        "module Main\n\nlet value = 1\n"
    );
}

#[test]
fn query_rejects_import_scope_and_patch_surfaces_candidate_diagnostics() {
    let root = TempRoot::new();
    let source = root.path().join("Main.ling");
    let helper = root.path().join("Helper.ling");
    fs::write(
        &source,
        "module Main\n\nimport Helper\n\nlet main () = Helper.value\n",
    )
    .unwrap();
    fs::write(&helper, "module Helper\n\nlet value = 1\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["query", "--symbol", "main", "--format", "json"])
        .arg(&source)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(diagnostic["code"], "L-QUERY-0001");

    let standalone = root.path().join("Standalone.ling");
    let original = "module Main\n\nlet value = 1\n";
    fs::write(&standalone, original).unwrap();
    let graph = semantic(&standalone);
    let target = graph["definitions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|definition| definition["name"] == "value")
        .unwrap()["definition_id"]
        .as_str()
        .unwrap();
    let request = transaction(&graph, target, "module Main\n\nlet value = (\n");
    let request_path = root.path().join("invalid-candidate.json");
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["patch", "--format", "json"])
        .arg(&request_path)
        .arg(&standalone)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert!(
        diagnostic["code"]
            .as_str()
            .is_some_and(|code| code.starts_with("L-SYNTAX-"))
    );
    assert_eq!(fs::read_to_string(&standalone).unwrap(), original);
}
