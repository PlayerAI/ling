use ling_project::{LOCK_FILE_FORMAT, LOCK_FILE_NAME, MANIFEST_FILE_NAME, MANIFEST_VERSION};

const MINIMAL_MANIFEST: &str = r#"manifest-version = 1

[package]
name = "hello"
display-name = "你好"
version = "0.1.0"
language = "0.1"

[source]
roots = ["src"]
entry = "Main"
"#;

#[test]
fn accepted_local_project_protocol_markers_are_exact() {
    assert_eq!(MANIFEST_FILE_NAME, "ling.toml");
    assert_eq!(MANIFEST_VERSION, 1);
    assert_eq!(LOCK_FILE_NAME, "ling.lock");
    assert_eq!(LOCK_FILE_FORMAT, "ling.lock/1");

    let manifest = ling_project::parse_manifest("local/ling.toml", MINIMAL_MANIFEST.as_bytes())
        .expect("the Accepted local manifest remains valid");
    assert_eq!(manifest.package().name().as_str(), "hello");
    assert_eq!(
        manifest.package().display_name().map(|name| name.as_str()),
        Some("你好")
    );
    assert_eq!(manifest.package().version().to_string(), "0.1.0");
    assert_eq!(manifest.package().language().as_str(), "0.1");
}

#[test]
fn manifest_v1_rejects_publication_only_top_level_and_package_fields() {
    for field in [
        "registry",
        "publisher",
        "namespace",
        "artifact",
        "checksum",
        "signature",
        "provenance",
        "yanked",
        "deprecated",
        "mirror",
        "cache",
    ] {
        let top_level = MINIMAL_MANIFEST.replacen(
            "manifest-version = 1",
            &format!("manifest-version = 1\n{field} = \"unsupported\""),
            1,
        );
        assert!(
            ling_project::parse_manifest("publication/top-level/ling.toml", top_level.as_bytes())
                .is_err(),
            "manifest v1 must reject top-level publication field {field}"
        );

        let package = MINIMAL_MANIFEST.replacen(
            "[package]",
            &format!("[package]\n{field} = \"unsupported\""),
            1,
        );
        assert!(
            ling_project::parse_manifest("publication/package/ling.toml", package.as_bytes())
                .is_err(),
            "manifest v1 must reject package publication field {field}"
        );
    }
}

#[test]
fn manifest_v1_accepts_only_content_locked_local_dependency_paths() {
    for external_locator in [
        "version = \"1.0.0\"",
        "registry = \"https://packages.invalid\"",
        "git = \"https://git.invalid/package.git\"",
        "checksum = \"sha256:00\"",
    ] {
        let manifest =
            format!("{MINIMAL_MANIFEST}\n[dependencies]\ndep = {{ {external_locator} }}\n");
        assert!(
            ling_project::parse_manifest("publication/dependency/ling.toml", manifest.as_bytes())
                .is_err(),
            "manifest v1 must reject external dependency locator {external_locator}"
        );
    }

    let local = format!("{MINIMAL_MANIFEST}\n[dependencies]\ndep = {{ path = \"deps/dep\" }}\n");
    let manifest = ling_project::parse_manifest("local/dependency/ling.toml", local.as_bytes())
        .expect("a local vendored dependency remains valid");
    assert_eq!(
        manifest
            .dependencies()
            .iter()
            .find(|(name, _)| name.as_str() == "dep")
            .map(|(_, dependency)| dependency)
            .expect("dependency exists")
            .path()
            .as_str(),
        "deps/dep"
    );
}

#[test]
fn project_implementation_exposes_no_registry_network_process_or_signing_route() {
    const PROJECT_SOURCE: &str = concat!(
        include_str!("../src/lib.rs"),
        include_str!("../src/package_graph.rs"),
        include_str!("../src/lockfile.rs"),
        include_str!("../src/workspace.rs"),
    );

    for forbidden in [
        "std::net::",
        "std::process::Command",
        "Command::new",
        "reqwest::",
        "git2::",
        "RegistryClient",
        "publish_package",
        "install_package",
        "verify_signature",
    ] {
        assert!(
            !PROJECT_SOURCE.contains(forbidden),
            "publication authority must not enter ling-project through {forbidden}"
        );
    }
}
