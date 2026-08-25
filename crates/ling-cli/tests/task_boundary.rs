use ling_cli::compile_source;
use ling_diagnostics::codes;
use ling_effects::locate_main;
use ling_eval::{MemoryConsole, execute_main};

const CHECKED_TASK: &str = concat!(
    "module Main\n\n",
    "task worker value =\n",
    "    scope\n",
    "        return value\n\n",
    "let main () = ()\n",
);

#[test]
fn task_declaration_reaches_checked_snapshot_but_not_execution() {
    let compiled = compile_source("task.ling", CHECKED_TASK.as_bytes().to_vec())
        .expect("Accepted Task syntax must reach a checked snapshot");
    assert_eq!(compiled.snapshot.checked().task_cores().len(), 1);
    assert_eq!(
        compiled
            .snapshot
            .graph()
            .task
            .as_ref()
            .map(|projection| projection.tasks.len()),
        Some(1)
    );
    let audit = compiled.snapshot.audit_model();
    let rendered = ling_format::render_audit(&audit).expect("Task Audit renders");
    assert!(rendered.starts_with("audit ling.audit/0.3 {\n"));
    assert_eq!(
        ling_format::parse_audit(&rendered).expect("Task Audit parses"),
        audit
    );

    let main = locate_main(compiled.snapshot.checked()).expect("fixture has Main.main");
    let mut console = MemoryConsole::default();
    let fault = execute_main(&compiled.snapshot, &main, &mut console)
        .expect_err("checked Task programs must stop at the execution boundary");
    let diagnostic = fault.to_diagnostic();
    assert_eq!(diagnostic.code(), codes::TASK_IMPLEMENTATION_BOUNDARY);
    assert_eq!(
        diagnostic
            .primary_span()
            .expect("original Task span")
            .file(),
        "task.ling"
    );
    assert!(console.output().is_empty());
    let json = diagnostic.render_json().expect("diagnostic JSON");
    assert!(json.contains("cannot enter"));
    assert!(json.contains("尚不能进入"));
}

#[test]
fn task_contextual_words_remain_ordinary_identifiers() {
    compile_source(
        "contextual.ling",
        concat!(
            "module Main\n\n",
            "let task = 1\n",
            "let scope = 2\n",
            "let spawn = 3\n",
            "let await = 4\n",
            "let return = 5\n",
            "let main () = task + scope + spawn + await + return\n",
        )
        .as_bytes()
        .to_vec(),
    )
    .expect("Task contextual words remain usable as ordinary identifiers");
}

#[test]
fn file_check_succeeds_while_file_run_stops_before_execution() {
    let path = std::env::temp_dir().join(format!("ling-task-boundary-{}.ling", std::process::id()));
    let temporary = TemporaryFile(path);
    fs::write(&temporary.0, CHECKED_TASK).expect("temporary Task source is writable");

    let check = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["check", "--format", "json"])
        .arg(&temporary.0)
        .output()
        .expect("ling check runs");
    assert!(check.status.success());
    assert!(check.stdout.is_empty());
    assert!(check.stderr.is_empty());

    let run = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["run", "--format", "json"])
        .arg(&temporary.0)
        .output()
        .expect("ling run executes boundary check");
    assert_eq!(run.status.code(), Some(1));
    assert!(run.stdout.is_empty());
    assert!(String::from_utf8_lossy(&run.stderr).contains("L-TASK-0004"));
}

struct TemporaryFile(PathBuf);

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
use std::fs;
use std::path::PathBuf;
use std::process::Command;
