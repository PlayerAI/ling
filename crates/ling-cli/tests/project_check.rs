use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/projects")
        .join(name)
}

fn run_project_check(project: &Path, format: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["project", "check", "--manifest-path"])
        .arg(project.join("ling.toml"))
        .args(["--locked", "--format", format])
        .output()
        .expect("project check process runs")
}

#[test]
fn locked_project_check_emits_path_free_deterministic_json() {
    let project = fixture("offline-lock");
    let lock_before = fs::read(project.join("ling.lock")).expect("fixture lock is readable");
    let first = run_project_check(&project, "json");
    let second = run_project_check(&project, "json");

    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(first.stderr.is_empty());
    assert_eq!(first.stdout, second.stdout);
    let report: serde_json::Value =
        serde_json::from_slice(&first.stdout).expect("project report is JSON");
    assert_eq!(report["protocol"], "ling.project.check/0.1");
    assert_eq!(report["status"], "ok");
    assert_eq!(report["entry"], "Main");
    assert!(
        report["graph"]
            .as_str()
            .is_some_and(|value| !value.is_empty())
    );
    let output = String::from_utf8_lossy(&first.stdout);
    assert!(!output.contains(project.to_string_lossy().as_ref()));
    assert_eq!(
        fs::read(project.join("ling.lock")).expect("fixture lock remains readable"),
        lock_before
    );
}

#[test]
fn human_project_check_is_one_path_free_success_line() {
    let project = fixture("offline-lock");
    let output = run_project_check(&project, "human");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("项目检查通过 / project check passed: "));
    assert!(stdout.ends_with('\n'));
    assert!(!stdout.contains(project.to_string_lossy().as_ref()));
}

#[test]
fn missing_lock_is_a_validation_error_without_mutating_project_files() {
    let source = fixture("offline-lock");
    let root = std::env::temp_dir().join(format!("ling-project-check-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).expect("stale test root is removable");
    }
    copy_tree(&source, &root);
    let lock = root.join("ling.lock");
    fs::remove_file(&lock).expect("fixture lock exists");
    let output = run_project_check(&root, "json");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let report: serde_json::Value =
        serde_json::from_slice(&output.stderr).expect("project error report is JSON");
    assert_eq!(report["protocol"], "ling.project.check/0.1");
    assert_eq!(report["status"], "error");
    assert!(
        report["diagnostics"]
            .as_array()
            .is_some_and(|items| !items.is_empty())
    );
    assert!(!lock.exists());
    fs::remove_dir_all(&root).expect("temporary project root is removable");
}

#[test]
fn project_check_rejects_missing_locked_or_positional_inputs() {
    let binary = env!("CARGO_BIN_EXE_ling");
    let missing_manifest = Command::new(binary)
        .args(["project", "check", "--locked"])
        .output()
        .expect("missing manifest invocation runs");
    assert_eq!(missing_manifest.status.code(), Some(2));

    let positional = Command::new(binary)
        .args(["project", "check", "--locked", "ling.toml"])
        .output()
        .expect("positional invocation runs");
    assert_eq!(positional.status.code(), Some(2));
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("temporary project root is creatable");
    for entry in fs::read_dir(source).expect("fixture project is readable") {
        let entry = entry.expect("fixture entry is readable");
        let target = destination.join(entry.file_name());
        if entry
            .file_type()
            .expect("fixture entry type is readable")
            .is_dir()
        {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("fixture file is copyable");
        }
    }
}
