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
        "query",
        "patch",
        "repl",
        "fmt",
        "init",
        "test",
        "build",
        "project check",
        "lsp",
        "completion",
    ] {
        assert!(help.contains(command), "help is missing `{command}`");
    }
    for stale in [
        "replay", "explain", "evidence", "version", "support", "features", "zero", ".zero",
    ] {
        assert!(
            !help.contains(&format!("  ling {stale}")),
            "help advertises stale command `{stale}`"
        );
    }
    assert!(help.contains("exact checked `task main ()`"));
    assert!(help.contains("only by file/project interpreter run"));
    assert!(help.contains("Task test, build, REPL, artifacts, bytecode, VM, Native, and Wasm"));
}

#[test]
fn unknown_future_command_is_rejected_with_usage_on_stderr() {
    for command in [
        "replay", "explain", "evidence", "version", "support", "features",
    ] {
        let output = run(&[command]);

        assert_eq!(output.status.code(), Some(2), "{command}");
        assert!(output.stdout.is_empty(), "{command}");
        let stderr = String::from_utf8(output.stderr).expect("diagnostic is UTF-8");
        assert!(
            stderr.contains(&format!("unknown command `{command}`")),
            "{command}"
        );
        assert!(stderr.contains("Usage:"), "{command}");
        assert!(stderr.contains("ling test"), "{command}");
        assert!(!stderr.contains(&format!("ling {command}\n")), "{command}");
    }
}
