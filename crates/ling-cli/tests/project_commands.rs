use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/projects/offline-lock")
}

fn run(project: &Path, operation: &str, format: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ling"))
        .arg(operation)
        .args(["--manifest-path"])
        .arg(project.join("ling.toml"))
        .args(["--locked", "--offline", "--format", format])
        .output()
        .expect("project command process runs")
}

fn run_build(project: &Path, output: &Path, format: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["build", "--manifest-path"])
        .arg(project.join("ling.toml"))
        .args([
            "--locked",
            "--offline",
            "--profile",
            "explore",
            "--target",
            "semantic",
            "--output",
        ])
        .arg(output)
        .args(["--format", format])
        .output()
        .expect("project build process runs")
}

#[test]
fn semantic_check_is_path_free_repeatable_and_does_not_mutate_inputs() {
    let project = fixture();
    let lock_before = fs::read(project.join("ling.lock")).expect("fixture lock is readable");
    let source_before =
        fs::read(project.join("src/Main.ling")).expect("fixture source is readable");
    let first = run(&project, "check", "json");
    let second = run(&project, "check", "json");

    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(first.stderr.is_empty());
    assert_eq!(first.stdout, second.stdout);
    let report: serde_json::Value =
        serde_json::from_slice(&first.stdout).expect("check report is JSON");
    assert_eq!(report["protocol"], "ling.project.command/0.1");
    assert_eq!(report["operation"], "check");
    assert_eq!(report["status"], "ok");
    assert_eq!(report["package"]["name"], "offline-app");
    assert_eq!(report["entry"], "Main");
    assert!(
        report["program"]
            .as_str()
            .is_some_and(|value| !value.is_empty())
    );
    assert!(!String::from_utf8_lossy(&first.stdout).contains(project.to_string_lossy().as_ref()));
    assert_eq!(fs::read(project.join("ling.lock")).unwrap(), lock_before);
    assert_eq!(
        fs::read(project.join("src/Main.ling")).unwrap(),
        source_before
    );
}

#[test]
fn run_and_test_consume_the_checked_root_entry() {
    let project = temp_root("run-test");
    copy_tree(&fixture(), &project);
    fs::write(
        project.join("src/Main.ling"),
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "import cached.Api as Cached\n\n",
            "let main () = Console.write (Text.format \"{}\" Cached.cached)\n",
        ),
    )
    .expect("temporary entry is writable");
    update_lock(&project);
    let run_output = run(&project, "run", "json");
    assert!(run_output.status.success());
    assert!(run_output.stderr.is_empty());
    let run_report: serde_json::Value =
        serde_json::from_slice(&run_output.stdout).expect("run report is JSON");
    assert_eq!(run_report["operation"], "run");
    assert_eq!(run_report["stdout"], "42\n");

    let test_output = run(&project, "test", "json");
    assert!(test_output.status.success());
    assert!(test_output.stderr.is_empty());
    let test_report: serde_json::Value =
        serde_json::from_slice(&test_output.stdout).expect("test report is JSON");
    assert_eq!(test_report["operation"], "test");
    assert_eq!(test_report["counts"]["total"], 1);
    assert_eq!(test_report["counts"]["passed"], 1);
    assert_eq!(test_report["tests"][0]["name"], "offline-app::Main");
    assert_eq!(test_report["tests"][0]["stdout"], "42\n");
    cleanup(&project);
}

#[test]
fn build_publishes_exact_canonical_artifact_once() {
    let root = temp_root("build");
    let artifact_path = root.join("offline-app.ling-project.json");
    let first = run_build(&fixture(), &artifact_path, "json");
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(first.stderr.is_empty());
    let report: serde_json::Value =
        serde_json::from_slice(&first.stdout).expect("build report is JSON");
    let bytes = fs::read(&artifact_path).expect("artifact was published");
    let artifact: serde_json::Value =
        serde_json::from_slice(&bytes).expect("artifact is canonical JSON");
    assert_eq!(report["operation"], "build");
    assert_eq!(report["artifact"]["protocol"], "ling.project.artifact/0.1");
    assert_eq!(report["artifact"]["bytes"], bytes.len());
    assert_eq!(artifact["protocol"], "ling.project.artifact/0.1");
    assert_eq!(artifact["profile"], "explore");
    assert_eq!(artifact["target"], "semantic");
    assert_eq!(artifact["semantic"]["schema"], "ling.semantic/0.2");
    assert_eq!(
        report["artifact"]["identity"],
        format!(
            "sha256:{}",
            Sha256::digest(&bytes)
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        )
    );
    assert!(bytes.ends_with(b"\n"));
    assert!(!String::from_utf8_lossy(&bytes).contains(fixture().to_string_lossy().as_ref()));

    let second = run_build(&fixture(), &artifact_path, "json");
    assert_eq!(second.status.code(), Some(4));
    assert!(second.stdout.is_empty());
    let failure: serde_json::Value =
        serde_json::from_slice(&second.stderr).expect("build failure is JSON");
    assert_eq!(failure["operation"], "build");
    assert_eq!(failure["diagnostics"][0]["code"], "L-IO-0005");
    assert_eq!(fs::read(&artifact_path).unwrap(), bytes);
    cleanup(&root);
}

#[test]
fn run_reports_runtime_fault_without_leaking_console_output() {
    let root = temp_root("runtime-error");
    copy_tree(&fixture(), &root);
    fs::write(
        root.join("src/Main.ling"),
        concat!(
            "module Main\n\n",
            "let main () =\n",
            "    Text.format \"no placeholder\" 1\n",
            "    ()\n",
        ),
    )
    .expect("temporary entry is writable");
    update_lock(&root);

    let output = run(&root, "run", "json");
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    let report: serde_json::Value =
        serde_json::from_slice(&output.stderr).expect("runtime failure is JSON");
    assert_eq!(report["protocol"], "ling.project.command/0.1");
    assert_eq!(report["operation"], "run");
    assert_eq!(report["status"], "error");
    assert_eq!(report["stdout"], "");
    assert_eq!(report["diagnostics"][0]["code"], "L-RUNTIME-0001");
    cleanup(&root);
}

#[test]
fn semantic_source_failure_uses_bilingual_registered_diagnostics() {
    let root = temp_root("semantic-error");
    copy_tree(&fixture(), &root);
    fs::write(
        root.join("src/Main.ling"),
        b"module Main\n\nlet main () = missing\n",
    )
    .expect("temporary source is writable");
    update_lock(&root);

    let output = run(&root, "check", "json");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let report: serde_json::Value =
        serde_json::from_slice(&output.stderr).expect("semantic failure is JSON");
    assert_eq!(report["protocol"], "ling.project.command/0.1");
    assert_eq!(report["operation"], "check");
    assert_eq!(report["status"], "error");
    assert!(
        report["diagnostics"][0]["code"]
            .as_str()
            .is_some_and(|code| code.starts_with("L-NAME-") || code.starts_with("L-TYPE-"))
    );
    assert!(
        report["diagnostics"][0]["primary_span"]["file"]
            .as_str()
            .is_some_and(|file| file.starts_with("package:offline-app/"))
    );
    cleanup(&root);
}

#[test]
fn project_mode_is_explicit_and_rejects_incomplete_or_mixed_selection() {
    let binary = env!("CARGO_BIN_EXE_ling");
    for arguments in [
        vec!["check", "--manifest-path", "ling.toml", "--locked"],
        vec![
            "run",
            "Main.ling",
            "--manifest-path",
            "ling.toml",
            "--locked",
            "--offline",
        ],
        vec![
            "build",
            "--manifest-path",
            "ling.toml",
            "--locked",
            "--offline",
            "--profile",
            "native",
            "--target",
            "semantic",
            "--output",
            "artifact.json",
        ],
        vec![
            "check",
            "--manifest-path",
            "ling.toml",
            "--locked",
            "--offline",
            "--format",
            "human",
            "--format",
            "json",
        ],
    ] {
        let output = Command::new(binary)
            .args(arguments)
            .output()
            .expect("invalid invocation runs");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn checked_task_project_allows_check_but_rejects_run_test_and_build() {
    let root = temp_root("task-boundary");
    copy_tree(&fixture(), &root);
    fs::write(
        root.join("src/Main.ling"),
        concat!(
            "module Main\n\n",
            "task worker value =\n",
            "    scope\n",
            "        return value\n\n",
            "let main () = ()\n",
        ),
    )
    .expect("temporary Task source is writable");
    update_lock(&root);

    let checked = run(&root, "check", "json");
    assert!(
        checked.status.success(),
        "{}",
        String::from_utf8_lossy(&checked.stderr)
    );
    for operation in ["run", "test"] {
        let output = run(&root, operation, "json");
        assert_eq!(output.status.code(), Some(1), "{operation}");
        assert!(output.stdout.is_empty(), "{operation}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("L-TASK-0004"),
            "{operation}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let artifact = root.join("task-artifact.json");
    let built = run_build(&root, &artifact, "json");
    assert_eq!(built.status.code(), Some(1));
    assert!(built.stdout.is_empty());
    assert!(String::from_utf8_lossy(&built.stderr).contains("L-TASK-0004"));
    assert!(!artifact.exists());
    cleanup(&root);
}

fn update_lock(root: &Path) {
    let manifest_path = root.join("ling.toml");
    let bytes = fs::read(&manifest_path).expect("temporary manifest is readable");
    let manifest =
        ling_project::parse_manifest("ling.toml", &bytes).expect("temporary manifest is valid");
    ling_project::resolve_package_graph_with_lock(root, &manifest, ling_project::LockMode::Update)
        .expect("temporary lock updates");
}

fn temp_root(label: &str) -> PathBuf {
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "ling-project-command-{label}-{}-{sequence}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("stale test root is removable");
    }
    fs::create_dir_all(&root).expect("test root is creatable");
    root
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("destination is creatable");
    for entry in fs::read_dir(source).expect("source directory is readable") {
        let entry = entry.expect("source entry is readable");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("entry type is readable").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("fixture file is copyable");
        }
    }
}

fn cleanup(path: &Path) {
    fs::remove_dir_all(path).expect("temporary root is removable");
}
