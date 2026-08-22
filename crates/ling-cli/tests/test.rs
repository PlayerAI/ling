use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct TempRoot(PathBuf);

impl TempRoot {
    fn new() -> Self {
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("ling-cli-test-{}-{sequence}", std::process::id()));
        fs::create_dir_all(&path).expect("test root is creatable");
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

fn run_test(root: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["test", "--format", "json"])
        .arg(root)
        .output()
        .expect("ling test process runs")
}

#[test]
fn test_json_runs_sorted_files_and_reports_runtime_precedence() {
    let root = TempRoot::new();
    fs::write(
        root.path().join("z-pass.ling"),
        "module Main\n\nlet main () = ()\n",
    )
    .unwrap();
    fs::write(
        root.path().join("a-compile.ling"),
        "module Main\n    requires Console.Write\n\nlet main () = Console.write 1\n",
    )
    .unwrap();
    fs::write(
        root.path().join("m-runtime.ling"),
        "module Main\n\nlet main () =\n    Text.format \"no placeholder\" 1\n    ()\n",
    )
    .unwrap();

    let output = run_test(root.path());
    assert_eq!(output.status.code(), Some(4));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["schema"], "ling.test/0.1");
    assert_eq!(report["status"], "failed");
    assert_eq!(
        report["counts"],
        serde_json::json!({"total": 3, "passed": 1, "failed": 2})
    );
    assert_eq!(report["tests"][0]["name"], "a-compile.ling");
    assert_eq!(report["tests"][0]["status"], "compile_failed");
    assert_eq!(report["tests"][1]["name"], "m-runtime.ling");
    assert_eq!(report["tests"][1]["status"], "runtime_failed");
    assert_eq!(report["tests"][2]["name"], "z-pass.ling");
    assert_eq!(report["tests"][2]["status"], "passed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("L-TYPE-0001"));
    assert!(stderr.contains("L-RUNTIME-0001"));
}

#[test]
fn test_rejects_empty_selection_with_stable_diagnostic() {
    let root = TempRoot::new();
    let output = run_test(root.path());
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let diagnostic: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(diagnostic["code"], "L-TEST-0001");
}
