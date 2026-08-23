use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::{
    io::Read as _,
    process::{Child, ExitStatus},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Expectation {
    test_id: String,
    polarity: String,
    feature_ids: Vec<String>,
    arguments: Vec<String>,
    exit_code: i32,
    normative_clauses: Vec<String>,
    #[serde(default)]
    stdout: String,
    #[serde(default)]
    diagnostic_codes: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExampleCases {
    schema: String,
    case: Vec<ExampleCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExampleCase {
    id: String,
    path: String,
    role: String,
    expected_stdout: String,
    semantic_name: String,
    identifier_style: String,
}

#[derive(Debug, PartialEq, Eq)]
struct TutorialSemanticShape {
    schema: String,
    language_version: String,
    unicode_version: String,
    entry_module: String,
    module_requirements: Vec<Vec<String>>,
    definitions: Vec<(String, String, Vec<String>, Vec<String>)>,
    nodes: Vec<(String, String, Vec<String>, Vec<String>)>,
    references: Vec<(String, String)>,
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
    let mut test_ids = BTreeSet::new();
    for fixture_directory in fixture_directories {
        let test_id = run_fixture(&fixture_directory);
        assert!(
            test_ids.insert(test_id.clone()),
            "duplicate conformance test_id {test_id}"
        );
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
    assert_eq!(
        first.stdout,
        include_bytes!("../../../schemas/semantic/0.1/canonical/hello.bin")
    );
    let graph: serde_json::Value =
        serde_json::from_slice(&first.stdout).expect("semantic output is JSON");
    assert_eq!(graph["schema"], "ling.semantic/0.1");
    assert_eq!(graph["entry_module"], "Main");
}

#[test]
fn formatter_json_report_is_deterministic_and_non_mutating() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/conformance/p7-hello-run/case.ling");
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_ling"))
            .args(["fmt", "--format", "json"])
            .arg(&fixture)
            .output()
            .expect("formatter process runs")
    };
    let first = run();
    let second = run();

    assert!(first.status.success());
    assert!(first.stderr.is_empty());
    assert_eq!(first.stdout, second.stdout);
    let report: serde_json::Value =
        serde_json::from_slice(&first.stdout).expect("formatter report is JSON");
    assert_eq!(report["schema"], "ling.format/0.1");
    assert_eq!(report["check"], false);
    assert_eq!(report["changed"], false);
    assert_eq!(report["disposition"], "unchanged");
    assert!(report["text"].as_str().is_some());
}

#[test]
fn formatter_stdin_and_invalid_source_use_the_report_boundary() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["fmt", "--format", "json", "--stdin-name", "stdin.ling", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("formatter stdin process starts");
    child
        .stdin
        .take()
        .expect("formatter stdin is piped")
        .write_all(b"let answer = 42")
        .expect("formatter stdin is written");
    let output = child.wait_with_output().expect("formatter stdin exits");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdin formatter report is JSON");
    assert_eq!(report["source"], "stdin.ling");
    assert_eq!(report["changed"], true);
    assert_eq!(report["disposition"], "formatted");

    let mut child = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args([
            "fmt",
            "--format",
            "json",
            "--stdin-name",
            "invalid.ling",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("invalid formatter stdin process starts");
    child
        .stdin
        .take()
        .expect("invalid formatter stdin is piped")
        .write_all(b"let =")
        .expect("invalid formatter stdin is written");
    let output = child
        .wait_with_output()
        .expect("invalid formatter stdin exits");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid formatter report is JSON");
    assert_eq!(report["source"], "invalid.ling");
    assert_eq!(report["changed"], false);
    assert_eq!(report["disposition"], "invalid");
    assert!(
        report["diagnostics"]
            .as_array()
            .is_some_and(|items| !items.is_empty())
    );
}

#[test]
fn audit_output_is_deterministic_and_round_trips() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/conformance/p7-hello-run/case.ling");
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_ling"))
            .args(["audit", "--format", "json"])
            .arg(&fixture)
            .output()
            .expect("audit process runs")
    };
    let first = run();
    let second = run();

    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(first.stderr.is_empty());
    assert_eq!(first.stdout, second.stdout);
    assert!(!first.stdout.starts_with(&[0xEF, 0xBB, 0xBF]));
    assert!(first.stdout.ends_with(b"\n"));
    assert!(!first.stdout.ends_with(b"\n\n"));
    assert!(!first.stdout.contains(&b'\r'));

    let source = std::str::from_utf8(&first.stdout).expect("Audit output is UTF-8");
    let model = ling_format::parse_audit(source).expect("Audit output parses");
    assert_eq!(
        ling_format::render_audit(&model).expect("Audit model renders"),
        source
    );
}

#[test]
fn repl_json_is_transactional_and_preserves_generations() {
    let output = run_repl(
        &["--format", "json"],
        concat!(
            "let value = 1\n\n",
            "let old () = value\n\n",
            "let value = 2\n\n",
            "let broken = missing\n\n",
            "old ()\n\n",
            "value\n",
        ),
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let events = json_lines(&output.stdout);
    assert_eq!(events.len(), 6);
    assert_eq!(events[0]["status"], "ok");
    assert_eq!(events[0]["committed"], true);
    assert_eq!(events[3]["status"], "compile_error");
    assert_eq!(events[3]["committed"], false);
    assert_eq!(events[3]["diagnostics"][0]["code"], "L-NAME-0001");
    assert_eq!(events[4]["value"], "1");
    assert_eq!(events[5]["value"], "2");
    assert_ne!(events[0]["definition_id"], events[2]["definition_id"]);
}

#[test]
fn repl_handles_multiline_chinese_and_complete_eof_submission() {
    let output = run_repl(
        &["--format", "human"],
        "let 加一 value =\n    value + 1\n\n加一 2",
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    assert_eq!(
        String::from_utf8(output.stdout).expect("REPL stdout is UTF-8"),
        "加一 : Int -> Int\n3 : Int\n"
    );
}

#[test]
fn repl_routes_console_to_json_events_and_returns_runtime_exit_code() {
    let output = run_repl(
        &["--format", "json", "--capability", "Console.Write"],
        "Console.write \"hello\"\n\nlet failed = 1 / 0\n",
    );
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stderr.is_empty());
    let events = json_lines(&output.stdout);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0]["status"], "console");
    assert_eq!(events[0]["console"], "hello\n");
    assert_eq!(events[1]["status"], "ok");
    assert_eq!(events[1]["type"], "Unit");
    assert!(events[1].get("value").is_none());
    assert_eq!(events[2]["status"], "runtime_error");
    assert_eq!(events[2]["committed"], false);
}

#[test]
fn repl_json_writer_matches_schema_corpus() {
    let output = run_repl(&["--format", "json"], "let answer = 42\n\nanswer\n");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let actual = json_lines(&output.stdout);
    let expected: [serde_json::Value; 2] = [
        include_str!("../../../schemas/repl/0.1/valid/binding.json"),
        include_str!("../../../schemas/repl/0.1/valid/value.json"),
    ]
    .map(|fixture| serde_json::from_str(fixture).expect("valid REPL schema fixture"));

    assert_eq!(actual, expected);
}

#[test]
fn repl_rejects_confusables_and_reports_incomplete_eof() {
    let confusable = run_repl(&["--format", "json"], "let a = 1\n\nlet а = 2\n");
    assert_eq!(confusable.status.code(), Some(1));
    let events = json_lines(&confusable.stdout);
    assert_eq!(events[1]["status"], "compile_error");
    assert_eq!(events[1]["diagnostics"][0]["code"], "L-NAME-0006");

    let incomplete = run_repl(&["--format", "json"], "(");
    assert_eq!(incomplete.status.code(), Some(1));
    let events = json_lines(&incomplete.stdout);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["status"], "compile_error");
    assert_eq!(events[0]["diagnostics"][0]["code"], "L-SYNTAX-0006");
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[test]
fn repl_tty_interrupt_discards_pending_submission_but_preserves_state() {
    let mut command = tty_script_command();
    let mut child = ChildGuard(
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("PTY-backed REPL process starts"),
    );
    let mut input = child.0.stdin.take().expect("PTY stdin is piped");
    let mut output = child.0.stdout.take().expect("PTY stdout is piped");
    let (sender, receiver) = mpsc::channel();
    let reader = thread::spawn(move || {
        let mut complete = Vec::new();
        let mut buffer = [0_u8; 1024];
        loop {
            match output.read(&mut buffer) {
                Ok(0) => return complete,
                Ok(length) => {
                    let chunk = buffer[..length].to_vec();
                    complete.extend_from_slice(&chunk);
                    if sender.send(chunk).is_err() {
                        return complete;
                    }
                }
                Err(error) => panic!("failed to read PTY output: {error}"),
            }
        }
    });

    let mut observed = Vec::new();
    let mut cursor = expect_pty_output(&receiver, &mut observed, 0, b"ling> ");

    input
        .write_all(b"let answer = 42\r")
        .expect("submission line is written");
    input.flush().expect("submission line is flushed");
    cursor = expect_pty_output(&receiver, &mut observed, cursor, b"....> ");
    input.write_all(b"\r").expect("commit line is written");
    input.flush().expect("commit line is flushed");
    cursor = expect_pty_output(&receiver, &mut observed, cursor, b"answer : Int");
    cursor = expect_pty_output(&receiver, &mut observed, cursor, b"ling> ");

    input
        .write_all(b"let unfinished = (\r")
        .expect("incomplete submission is written");
    input.flush().expect("incomplete submission is flushed");
    cursor = expect_pty_output(&receiver, &mut observed, cursor, b"....> ");

    input.write_all(&[3]).expect("Ctrl-C byte is written");
    input.flush().expect("Ctrl-C byte is flushed");
    cursor = expect_pty_output(&receiver, &mut observed, cursor, b"ling> ");

    input
        .write_all(b"answer\r")
        .expect("post-interrupt expression line is written");
    input
        .flush()
        .expect("post-interrupt expression line is flushed");
    cursor = expect_pty_output(&receiver, &mut observed, cursor, b"....> ");
    input
        .write_all(b"\r")
        .expect("post-interrupt commit line is written");
    input
        .flush()
        .expect("post-interrupt commit line is flushed");
    cursor = expect_pty_output(&receiver, &mut observed, cursor, b"42 : Int");
    let _ = expect_pty_output(&receiver, &mut observed, cursor, b"ling> ");

    input.write_all(&[4]).expect("Ctrl-D byte is written");
    input.flush().expect("Ctrl-D byte is flushed");
    drop(input);

    let status = wait_for_child(&mut child.0, Duration::from_secs(10)).unwrap_or_else(|error| {
        panic!(
            "{error}\nPTY output:\n{}",
            String::from_utf8_lossy(&observed)
        )
    });
    let complete = reader.join().expect("PTY reader thread exits");
    let mut stderr = Vec::new();
    child
        .0
        .stderr
        .take()
        .expect("PTY stderr is piped")
        .read_to_end(&mut stderr)
        .expect("PTY stderr is read");

    assert!(
        status.success(),
        "PTY wrapper exited with {status}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&complete),
        String::from_utf8_lossy(&stderr)
    );
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

#[test]
fn seed_examples_check_run_and_emit_semantic_graphs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let manifest_path = root.join("tests/examples/seed-cases.toml");
    let manifest_source = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", manifest_path.display()));
    let manifest: ExampleCases = toml::from_str(&manifest_source)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", manifest_path.display()));
    assert_eq!(manifest.schema, "ling.seed-example-cases/0");
    assert_eq!(manifest.case.len(), 6);

    let mut ids = BTreeSet::new();
    let mut tutorial_shapes = BTreeMap::new();
    for case in manifest.case {
        assert!(
            ids.insert(case.id.clone()),
            "duplicate example id {}",
            case.id
        );
        assert!(matches!(
            case.role.as_str(),
            "core-minimal" | "core-realistic" | "tutorial"
        ));
        assert!(matches!(
            case.identifier_style.as_str(),
            "ascii" | "chinese"
        ));
        let is_tutorial = case.role == "tutorial";
        let identifier_style = case.identifier_style.clone();
        let path = root.join(&case.path);
        let checked = Command::new(env!("CARGO_BIN_EXE_ling"))
            .args(["check", "--format", "json"])
            .arg(&path)
            .output()
            .unwrap_or_else(|error| panic!("failed to check {}: {error}", path.display()));
        assert!(
            checked.status.success(),
            "{}\n{}",
            path.display(),
            String::from_utf8_lossy(&checked.stderr)
        );
        assert!(checked.stdout.is_empty());
        assert!(checked.stderr.is_empty());

        let executed = Command::new(env!("CARGO_BIN_EXE_ling"))
            .args(["run", "--format", "json"])
            .arg(&path)
            .output()
            .unwrap_or_else(|error| panic!("failed to run {}: {error}", path.display()));
        assert!(executed.status.success(), "{}", path.display());
        assert_eq!(
            String::from_utf8(executed.stdout).expect("example stdout is UTF-8"),
            case.expected_stdout
        );
        assert!(executed.stderr.is_empty());

        let semantic = Command::new(env!("CARGO_BIN_EXE_ling"))
            .args(["semantic", "--format", "json"])
            .arg(&path)
            .output()
            .unwrap_or_else(|error| {
                panic!(
                    "failed to emit semantic graph for {}: {error}",
                    path.display()
                )
            });
        assert!(semantic.status.success(), "{}", path.display());
        assert!(semantic.stderr.is_empty());
        let graph: serde_json::Value =
            serde_json::from_slice(&semantic.stdout).expect("semantic output is JSON");
        assert_eq!(graph["schema"], "ling.semantic/0.1");
        assert!(graph["definitions"].as_array().is_some_and(|definitions| {
            definitions
                .iter()
                .any(|definition| definition["name"] == case.semantic_name)
        }));
        if is_tutorial {
            assert!(
                tutorial_shapes
                    .insert(identifier_style, tutorial_semantic_shape(&graph))
                    .is_none(),
                "duplicate tutorial identifier style"
            );
        }
    }
    assert_eq!(tutorial_shapes.len(), 2);
    assert_eq!(tutorial_shapes["ascii"], tutorial_shapes["chinese"]);
}

fn tutorial_semantic_shape(graph: &serde_json::Value) -> TutorialSemanticShape {
    let definitions = graph["definitions"]
        .as_array()
        .expect("Semantic definitions are an array");
    let nominal_type = definitions
        .iter()
        .find(|definition| definition["origin"] == "user" && definition["kind"] == "type")
        .and_then(|definition| definition["name"].as_str())
        .expect("tutorial has one user nominal type");

    let mut module_requirements = graph["modules"]
        .as_array()
        .expect("Semantic modules are an array")
        .iter()
        .map(|module| string_array(&module["requires"]))
        .collect::<Vec<_>>();
    module_requirements.sort();

    let mut definition_shapes = definitions
        .iter()
        .filter(|definition| definition["origin"] == "user")
        .map(|definition| {
            (
                json_string(&definition["kind"]),
                normalize_nominal(&json_string(&definition["type"]), nominal_type),
                normalize_nominals(string_array(&definition["effects"]), nominal_type),
                string_array(&definition["capabilities"]),
            )
        })
        .collect::<Vec<_>>();
    definition_shapes.sort();

    let mut node_shapes = graph["nodes"]
        .as_array()
        .expect("Semantic nodes are an array")
        .iter()
        .map(|node| {
            (
                json_string(&node["kind"]),
                normalize_nominal(&json_optional_string(&node["type"]), nominal_type),
                normalize_nominals(string_array(&node["effects"]), nominal_type),
                string_array(&node["capabilities"]),
            )
        })
        .collect::<Vec<_>>();
    node_shapes.sort();

    let mut reference_shapes = graph["references"]
        .as_array()
        .expect("Semantic references are an array")
        .iter()
        .map(|reference| {
            (
                json_string(&reference["source_kind"]),
                json_string(&reference["target_kind"]),
            )
        })
        .collect::<Vec<_>>();
    reference_shapes.sort();

    TutorialSemanticShape {
        schema: json_string(&graph["schema"]),
        language_version: json_string(&graph["language_version"]),
        unicode_version: json_string(&graph["unicode_version"]),
        entry_module: json_string(&graph["entry_module"]),
        module_requirements,
        definitions: definition_shapes,
        nodes: node_shapes,
        references: reference_shapes,
    }
}

fn json_string(value: &serde_json::Value) -> String {
    value
        .as_str()
        .expect("Semantic field is a string")
        .to_owned()
}

fn json_optional_string(value: &serde_json::Value) -> String {
    value.as_str().unwrap_or("<none>").to_owned()
}

fn string_array(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .expect("Semantic field is an array")
        .iter()
        .map(json_string)
        .collect()
}

fn normalize_nominal(value: &str, nominal_type: &str) -> String {
    value.replace(nominal_type, "$DomainType")
}

fn normalize_nominals(values: Vec<String>, nominal_type: &str) -> Vec<String> {
    values
        .into_iter()
        .map(|value| normalize_nominal(&value, nominal_type))
        .collect()
}

fn run_fixture(directory: &Path) -> String {
    let expectation_path = directory.join("expect.toml");
    let expectation_text = fs::read_to_string(&expectation_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", expectation_path.display()));
    let expectation: Expectation = toml::from_str(&expectation_text)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", expectation_path.display()));
    assert!(
        valid_test_id(&expectation.test_id),
        "{} has invalid test_id {:?}; expected TEST-CONF-*",
        expectation_path.display(),
        expectation.test_id
    );
    assert!(
        matches!(expectation.polarity.as_str(), "Positive" | "Negative"),
        "{} has invalid polarity {:?}",
        expectation_path.display(),
        expectation.polarity
    );
    assert!(
        !expectation.feature_ids.is_empty(),
        "{} must map to at least one stable feature_id",
        expectation_path.display()
    );
    assert!(
        expectation
            .feature_ids
            .iter()
            .all(|feature_id| valid_feature_id(feature_id)),
        "{} contains an invalid feature_id",
        expectation_path.display()
    );
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
    match expectation.polarity.as_str() {
        "Positive" => assert!(
            expectation.exit_code == 0 && expectation.diagnostic_codes.is_empty(),
            "{} is Positive but expects a failure or diagnostic",
            expectation_path.display()
        ),
        "Negative" => assert!(
            expectation.exit_code != 0 || !expectation.diagnostic_codes.is_empty(),
            "{} is Negative but expects neither failure nor diagnostic",
            expectation_path.display()
        ),
        _ => unreachable!("polarity was validated above"),
    }
    expectation.test_id
}

fn valid_test_id(value: &str) -> bool {
    valid_upper_id(value, "TEST-CONF-")
}

fn valid_feature_id(value: &str) -> bool {
    valid_upper_id(value, "FTR-")
}

fn valid_upper_id(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        !suffix.is_empty()
            && !suffix.starts_with('-')
            && !suffix.ends_with('-')
            && suffix
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-')
    })
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

fn run_repl(arguments: &[&str], input: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ling"))
        .arg("repl")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("REPL process starts");
    child
        .stdin
        .take()
        .expect("REPL stdin is piped")
        .write_all(input.as_bytes())
        .expect("REPL input is written");
    child.wait_with_output().expect("REPL process exits")
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn tty_script_command() -> Command {
    #[cfg(target_os = "linux")]
    {
        let executable = shell_quote(Path::new(env!("CARGO_BIN_EXE_ling")));
        let mut command = Command::new("script");
        command
            .arg("-qefc")
            .arg(format!("{executable} repl --format human"))
            .arg("/dev/null")
            .env("TERM", "xterm");
        command
    }

    #[cfg(target_os = "macos")]
    {
        let mut command = Command::new("script");
        command
            .arg("-q")
            .arg("/dev/null")
            .arg(env!("CARGO_BIN_EXE_ling"))
            .args(["repl", "--format", "human"])
            .env("TERM", "xterm");
        command
    }
}

#[cfg(target_os = "linux")]
fn shell_quote(path: &Path) -> String {
    format!("'{}'", path.to_string_lossy().replace('\'', "'\\''"))
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn expect_pty_output(
    receiver: &Receiver<Vec<u8>>,
    observed: &mut Vec<u8>,
    cursor: usize,
    expected: &[u8],
) -> usize {
    wait_for_pty_output(
        receiver,
        observed,
        cursor,
        expected,
        Duration::from_secs(10),
    )
    .unwrap_or_else(|error| {
        panic!(
            "{error}\nPTY output:\n{}",
            String::from_utf8_lossy(observed)
        )
    })
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn wait_for_pty_output(
    receiver: &Receiver<Vec<u8>>,
    observed: &mut Vec<u8>,
    cursor: usize,
    expected: &[u8],
    timeout: Duration,
) -> Result<usize, String> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(offset) = observed[cursor..]
            .windows(expected.len())
            .position(|window| window == expected)
        {
            return Ok(cursor + offset + expected.len());
        }
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .ok_or_else(|| {
                format!(
                    "timed out waiting for {:?}",
                    String::from_utf8_lossy(expected)
                )
            })?;
        let chunk = receiver.recv_timeout(remaining).map_err(|error| {
            format!(
                "PTY output ended while waiting for {:?}: {error}",
                String::from_utf8_lossy(expected)
            )
        })?;
        observed.extend_from_slice(&chunk);
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn wait_for_child(child: &mut Child, timeout: Duration) -> Result<ExitStatus, String> {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
            Ok(None) => return Err("timed out waiting for the PTY wrapper to exit".to_owned()),
            Err(error) => return Err(format!("failed to wait for the PTY wrapper: {error}")),
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
struct ChildGuard(Child);

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn json_lines(bytes: &[u8]) -> Vec<serde_json::Value> {
    std::str::from_utf8(bytes)
        .expect("JSON lines are UTF-8")
        .lines()
        .map(|line| serde_json::from_str(line).expect("REPL event is JSON"))
        .collect()
}
