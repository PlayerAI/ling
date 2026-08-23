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
            "ling-cli-output-policy-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("output-policy test root is creatable");
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

fn hello() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/hello.ling")
}

#[test]
fn human_diagnostics_retain_both_languages_and_apply_explicit_color() {
    let root = TempRoot::new();
    let source = root.path().join("invalid.ling");
    fs::write(&source, b"let main () = missing\n").expect("invalid source is writable");

    let output = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["check", "--language", "en", "--color", "always"])
        .arg(&source)
        .output()
        .expect("ling check process runs");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("diagnostic stderr is UTF-8");
    assert!(stderr.starts_with("\u{1b}[31merror[L-"));
    assert!(stderr.contains("\n = 中文: "));
    assert!(stderr.ends_with("\u{1b}[0m\n"));

    let normal_json = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["check", "--format", "json"])
        .arg(&source)
        .output()
        .expect("default-language JSON check runs");
    let english_json = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["check", "--format", "json", "--language", "en"])
        .arg(&source)
        .output()
        .expect("English-language JSON check runs");
    assert_eq!(normal_json.status.code(), Some(1));
    assert_eq!(normal_json.stderr, english_json.stderr);
    assert!(!normal_json.stderr.contains(&0x1b));

    let quiet = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["check", "--quiet", "--color", "never"])
        .arg(&source)
        .output()
        .expect("quiet failing check runs");
    assert_eq!(quiet.status.code(), Some(1));
    assert!(!quiet.stderr.is_empty());
}

#[test]
fn json_rejects_color_and_verbosity_that_could_contaminate_the_protocol() {
    for arguments in [
        vec![
            "check",
            "--format",
            "json",
            "--color",
            "always",
            "examples/hello.ling",
        ],
        vec![
            "check",
            "--format",
            "json",
            "--verbose",
            "examples/hello.ling",
        ],
    ] {
        let output = ling(&arguments);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.contains(&0x1b));
    }
}

#[test]
fn quiet_suppresses_only_auxiliary_success_output() {
    let root = TempRoot::new();
    let destination = root.path().join("starter");
    let init = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["init", "--quiet"])
        .arg(&destination)
        .output()
        .expect("ling init process runs");
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );
    assert!(init.stdout.is_empty());
    assert!(destination.join("src/Main.ling").is_file());

    let run = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["run", "--quiet"])
        .arg(hello())
        .output()
        .expect("quiet run process runs");
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout), "你好，零\n");
}

#[test]
fn verbose_event_is_deterministic_and_does_not_expose_paths() {
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_ling"))
            .args(["check", "--verbose", "--language", "en", "--color", "never"])
            .arg(hello())
            .output()
            .expect("verbose check process runs")
    };
    let first = invoke();
    let second = invoke();
    assert!(first.status.success());
    assert!(first.stdout.is_empty());
    assert_eq!(first.stderr, second.stderr);
    let stderr = String::from_utf8(first.stderr).expect("verbose stderr is UTF-8");
    assert_eq!(
        stderr,
        "verbose: command=check format=human language=en color=never verbosity=verbose\n\
中文: 详细：command=check format=human language=en color=never verbosity=verbose\n"
    );
    assert!(!stderr.contains("hello.ling"));
}
