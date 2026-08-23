const MINIMAL_MANIFEST: &str = r#"manifest-version = 1

[package]
name = "hello"
version = "0.1.0"
language = "0.1"

[source]
roots = ["src"]
entry = "Main"
"#;

#[test]
fn manifest_v1_rejects_build_system_top_level_and_package_fields() {
    for field in [
        "build",
        "build-script",
        "build-graph",
        "generator",
        "generated-source",
        "inputs",
        "outputs",
        "capabilities",
        "sandbox",
        "environment",
        "network",
        "artifact",
        "build-cache",
    ] {
        let top_level = MINIMAL_MANIFEST.replacen(
            "manifest-version = 1",
            &format!("manifest-version = 1\n{field} = \"unsupported\""),
            1,
        );
        assert!(
            ling_project::parse_manifest("hermetic/top-level/ling.toml", top_level.as_bytes())
                .is_err(),
            "manifest v1 must reject top-level build field {field}"
        );

        let package = MINIMAL_MANIFEST.replacen(
            "[package]",
            &format!("[package]\n{field} = \"unsupported\""),
            1,
        );
        assert!(
            ling_project::parse_manifest("hermetic/package/ling.toml", package.as_bytes()).is_err(),
            "manifest v1 must reject package build field {field}"
        );
    }
}

#[test]
fn local_dependencies_cannot_declare_build_execution_or_host_authority() {
    for field in [
        "build-script",
        "generator",
        "capability",
        "environment",
        "network",
        "shell",
        "command",
        "artifact",
        "cache",
    ] {
        let manifest = format!(
            "{MINIMAL_MANIFEST}\n[dependencies]\ndep = {{ path = \"deps/dep\", {field} = \"unsupported\" }}\n"
        );
        assert!(
            ling_project::parse_manifest("hermetic/dependency/ling.toml", manifest.as_bytes())
                .is_err(),
            "local dependency must not gain build authority through {field}"
        );
    }
}

#[test]
fn project_protocol_has_no_build_executor_or_shell_adapter_surface() {
    const PROJECT_SOURCE: &str = concat!(
        include_str!("../src/lib.rs"),
        include_str!("../src/package_graph.rs"),
        include_str!("../src/lockfile.rs"),
        include_str!("../src/workspace.rs"),
    );

    for forbidden in [
        "BuildExecutor",
        "BuildScript",
        "ShellAdapter",
        "execute_build",
        "run_build_script",
        "std::process::Command",
        "Command::new",
        "std::net::",
    ] {
        assert!(
            !PROJECT_SOURCE.contains(forbidden),
            "hermetic-build authority must not enter ling-project through {forbidden}"
        );
    }
}
