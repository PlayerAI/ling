use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const FIXTURES: &[(&str, &[u8])] = &[
    (
        "bash",
        include_bytes!("../../../tests/protocols/cli-completion/ling.bash"),
    ),
    (
        "zsh",
        include_bytes!("../../../tests/protocols/cli-completion/_ling"),
    ),
    (
        "fish",
        include_bytes!("../../../tests/protocols/cli-completion/ling.fish"),
    ),
    (
        "powershell",
        include_bytes!("../../../tests/protocols/cli-completion/ling.ps1"),
    ),
];

fn run(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(arguments)
        .output()
        .expect("ling process starts")
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/protocols/cli-completion")
        .join(name)
}

#[test]
fn every_shell_matches_its_exact_utf8_lf_fixture() {
    for (shell, expected) in FIXTURES {
        let first = run(&["completion", shell]);
        let second = run(&["completion", shell]);
        assert!(first.status.success(), "{shell}");
        assert!(first.stderr.is_empty(), "{shell}");
        assert_eq!(&first.stdout, expected, "{shell}");
        assert_eq!(second.stdout, first.stdout, "{shell}");
        assert_eq!(second.stderr, first.stderr, "{shell}");
        assert_eq!(first.stdout.last(), Some(&b'\n'), "{shell}");
        assert!(!first.stdout.contains(&b'\r'), "{shell}");
    }
}

#[test]
fn invalid_completion_forms_are_usage_errors_without_stdout() {
    for arguments in [
        &["completion"][..],
        &["completion", "nu"][..],
        &["completion", "bash", "extra"][..],
        &["completion", "bash", "--format", "json"][..],
    ] {
        let output = run(arguments);
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
        let stderr = String::from_utf8(output.stderr).expect("usage is UTF-8");
        assert!(stderr.contains("error:"), "{arguments:?}");
        assert!(stderr.contains("ling completion"), "{arguments:?}");
    }
}

#[test]
fn installed_shells_accept_the_committed_fixtures() {
    check_if_available("bash", &["--version"], &["-n"], "ling.bash");
    check_if_available("zsh", &["--version"], &["-n"], "_ling");
    check_if_available("fish", &["--version"], &["-n"], "ling.fish");

    let powershell = if command_succeeds("pwsh", &["--version"]) {
        Some("pwsh")
    } else if command_succeeds(
        "powershell",
        &["-NoProfile", "-Command", "$PSVersionTable.PSVersion"],
    ) {
        Some("powershell")
    } else {
        None
    };
    if let Some(powershell) = powershell {
        let fixture = fixture_path("ling.ps1");
        let output = Command::new(powershell)
            .args(["-NoLogo", "-NoProfile", "-NonInteractive", "-File"])
            .arg(fixture)
            .stdin(Stdio::null())
            .output()
            .expect("available PowerShell starts");
        assert!(
            output.status.success(),
            "PowerShell rejected fixture: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn check_if_available(program: &str, probe: &[&str], syntax: &[&str], fixture: &str) {
    if !command_succeeds(program, probe) {
        return;
    }
    let output = Command::new(program)
        .args(syntax)
        .arg(fixture_path(fixture))
        .stdin(Stdio::null())
        .output()
        .expect("available shell starts");
    assert!(
        output.status.success(),
        "{program} rejected fixture: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn command_succeeds(program: &str, arguments: &[&str]) -> bool {
    Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}
