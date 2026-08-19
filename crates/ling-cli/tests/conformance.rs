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
        .write_all(b"let answer = 42\r\r")
        .expect("committed submission is written");
    input.flush().expect("committed submission is flushed");
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
        .write_all(b"answer\r\r")
        .expect("post-interrupt expression is written");
    input.flush().expect("post-interrupt expression is flushed");
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
    let examples = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    for (file, expected_stdout, required_name) in [
        ("人物.ling", "存活\n", "受到伤害"),
        ("adt-match.ling", "受伤 30\n", "生存状态"),
        ("pipeline.ling", "9\n", "加一"),
    ] {
        let path = examples.join(file);
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
            expected_stdout
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
                .any(|definition| definition["name"] == required_name)
        }));
    }
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
