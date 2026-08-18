use std::fs;
use std::path::Path;
use std::process::Command;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Expectation {
    arguments: Vec<String>,
    exit_code: i32,
    normative_clauses: Vec<String>,
    #[serde(default)]
    stdout: String,
    #[serde(default)]
    diagnostic_codes: Vec<String>,
}

#[test]
fn conformance_fixtures() {
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/conformance");
    let mut fixture_directories = fs::read_dir(&fixture_root)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", fixture_root.display()))
        .map(|entry| {
            entry
                .expect("failed to read fixture directory entry")
                .path()
        })
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    fixture_directories.sort();

    assert!(
        !fixture_directories.is_empty(),
        "no conformance fixtures were discovered"
    );
    for fixture_directory in fixture_directories {
        run_fixture(&fixture_directory);
    }
}

#[test]
fn semantic_output_is_deterministic_and_versioned() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/conformance/p7-hello-run/case.ling");
    let first = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["semantic", "--format", "json"])
        .arg(&fixture)
        .output()
        .expect("first semantic process runs");
    let second = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["semantic", "--format", "json"])
        .arg(&fixture)
        .output()
        .expect("second semantic process runs");

    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(
        second.status.success(),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert_eq!(first.stdout, second.stdout);
    let graph: serde_json::Value =
        serde_json::from_slice(&first.stdout).expect("semantic output is JSON");
    assert_eq!(graph["schema"], "ling.semantic/0.1");
    assert_eq!(graph["entry_module"], "Main");
}

#[test]
fn loads_an_explicit_import_from_the_entry_module_root() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/cli/import/Main.ling");
    let output = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["check", "--format", "json"])
        .arg(&fixture)
        .output()
        .expect("import fixture runs");

    assert!(
        output.status.success(),
        "import fixture failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

fn run_fixture(directory: &Path) {
    let expectation_path = directory.join("expect.toml");
    let expectation_text = fs::read_to_string(&expectation_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", expectation_path.display()));
    let expectation: Expectation = toml::from_str(&expectation_text)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", expectation_path.display()));
    assert!(
        !expectation.normative_clauses.is_empty(),
        "{} must cite at least one normative clause",
        expectation_path.display()
    );

    let output = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(&expectation.arguments)
        .current_dir(directory)
        .output()
        .unwrap_or_else(|error| panic!("failed to run fixture {}: {error}", directory.display()));

    assert_eq!(
        output.status.code(),
        Some(expectation.exit_code),
        "wrong exit code for {}\nstderr:\n{}",
        directory.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout must be UTF-8"),
        expectation.stdout,
        "wrong stdout for {}",
        directory.display()
    );

    let actual_codes = diagnostic_codes(&output.stderr, directory);
    assert_eq!(
        actual_codes,
        expectation.diagnostic_codes,
        "wrong diagnostics for {}",
        directory.display()
    );
}

fn diagnostic_codes(stderr: &[u8], directory: &Path) -> Vec<String> {
    let stderr = std::str::from_utf8(stderr)
        .unwrap_or_else(|error| panic!("non-UTF-8 stderr for {}: {error}", directory.display()));
    stderr
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let value: serde_json::Value = serde_json::from_str(line).unwrap_or_else(|error| {
                panic!(
                    "invalid diagnostic JSON for {}: {error}\n{line}",
                    directory.display()
                )
            });
            value["code"]
                .as_str()
                .unwrap_or_else(|| panic!("diagnostic has no code for {}", directory.display()))
                .to_owned()
        })
        .collect()
}
