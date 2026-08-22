use std::process::Command;

fn run(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(arguments)
        .output()
        .expect("ling process starts")
}

#[test]
fn help_aliases_have_the_same_truthful_surface() {
    let long = run(&["--help"]);
    let short = run(&["-h"]);

    assert!(long.status.success());
    assert!(short.status.success());
    assert!(long.stderr.is_empty());
    assert!(short.stderr.is_empty());
    assert_eq!(long.stdout, short.stdout);

    let help = String::from_utf8(long.stdout).expect("help is UTF-8");
    for command in [
        "run",
        "check",
        "semantic",
        "audit",
        "repl",
        "fmt",
        "init",
        "test",
        "project check",
        "lsp",
    ] {
        assert!(help.contains(command), "help is missing `{command}`");
    }
    for stale in ["build", "query", "patch", "zero", ".zero"] {
        assert!(
            !help.contains(stale),
            "help advertises stale command `{stale}`"
        );
    }
}

#[test]
fn unknown_future_command_is_rejected_with_usage_on_stderr() {
    let output = run(&["query"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("diagnostic is UTF-8");
    assert!(stderr.contains("unknown command `query`"));
    assert!(stderr.contains("Usage:"));
    assert!(stderr.contains("ling test"));
    assert!(!stderr.contains("ling query"));
}
