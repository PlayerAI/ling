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
            std::env::temp_dir().join(format!("ling-cli-init-{}-{sequence}", std::process::id()));
        fs::create_dir_all(&path).expect("init test parent is creatable");
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

fn run_init(parent: &Path, arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["init"])
        .args(arguments)
        .current_dir(parent)
        .output()
        .expect("ling init process runs")
}

#[test]
fn init_json_creates_a_valid_deterministic_scaffold() {
    let root = TempRoot::new();
    let first = run_init(root.path(), &["--format", "json", "hello"]);
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(first.stderr.is_empty());
    let report: serde_json::Value =
        serde_json::from_slice(&first.stdout).expect("init report is JSON");
    assert_eq!(report["schema"], "ling.init/0.1");
    assert_eq!(report["status"], "ok");
    assert_eq!(report["template_version"], "1");
    assert_eq!(report["package"]["name"], "hello");
    assert_eq!(
        report["files"],
        serde_json::json!([".gitignore", "ling.lock", "ling.toml", "src/Main.ling"])
    );
    let project = root.path().join("hello");
    assert!(project.join("ling.toml").is_file());
    assert!(project.join("ling.lock").is_file());
    assert_eq!(
        fs::read(project.join("src/Main.ling")).unwrap(),
        b"let main () = ()\n"
    );

    let second = run_init(root.path(), &["--format", "json", "hello"]);
    assert_eq!(second.status.code(), Some(2));
    assert!(second.stdout.is_empty());
    assert!(project.join("src/Main.ling").is_file());
}

#[test]
fn init_accepts_explicit_package_and_display_names() {
    let root = TempRoot::new();
    let output = run_init(
        root.path(),
        &["--name", "hello-world", "--display-name", "你好", "starter"],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    let manifest = fs::read_to_string(root.path().join("starter/ling.toml")).unwrap();
    assert!(manifest.contains("name = \"hello-world\""));
    assert!(manifest.contains("display-name = \"你好\""));
}

#[test]
fn init_rejects_invalid_name_without_creating_target() {
    let root = TempRoot::new();
    let output = run_init(root.path(), &["--name", "Not_Valid", "starter"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(!root.path().join("starter").exists());
}
