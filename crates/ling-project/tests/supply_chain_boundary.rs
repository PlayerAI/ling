use ling_project::parse_manifest;

const MINIMAL_MANIFEST: &str = r#"manifest-version = 1

[package]
name = "ling"
display-name = "零语言"
version = "0.1.0"
language = "0.1"

[source]
roots = ["src"]
entry = "Main"
"#;

#[test]
fn technical_package_names_reject_namespace_and_unicode_spoofing() {
    for spoofed_name in [
        "playerai/ling",
        "playerai:ling",
        "playerai.ling",
        "lіng", // U+0456 CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
        "ⅼing", // U+217C SMALL ROMAN NUMERAL FIFTY
        "ling\u{200d}",
    ] {
        let manifest =
            MINIMAL_MANIFEST.replacen("name = \"ling\"", &format!("name = {spoofed_name:?}"), 1);
        assert!(
            parse_manifest("spoofed/ling.toml", manifest.as_bytes()).is_err(),
            "technical package name must reject {spoofed_name:?}"
        );
    }

    let manifest = parse_manifest("local/ling.toml", MINIMAL_MANIFEST.as_bytes())
        .expect("a separate Unicode display name remains valid");
    assert_eq!(manifest.package().name().as_str(), "ling");
    assert_eq!(
        manifest.package().display_name().map(|name| name.as_str()),
        Some("零语言")
    );
}

#[test]
fn malicious_manifest_fields_and_external_locators_are_rejected() {
    for field in [
        "registry",
        "archive",
        "signature",
        "build-script",
        "capabilities",
        "publisher",
        "yanked",
    ] {
        let manifest = MINIMAL_MANIFEST.replacen(
            "manifest-version = 1",
            &format!("manifest-version = 1\n{field} = \"hostile\""),
            1,
        );
        assert!(
            parse_manifest("hostile/field/ling.toml", manifest.as_bytes()).is_err(),
            "manifest v1 must reject unsupported field {field}"
        );
    }

    for locator in [
        "path = \"../escape\"",
        "path = \"/absolute\"",
        "path = \"C:/absolute\"",
        "version = \"9.9.9\"",
        "registry = \"https://packages.invalid\"",
        "git = \"https://git.invalid/hostile.git\"",
    ] {
        let manifest = format!("{MINIMAL_MANIFEST}\n[dependencies]\ndep = {{ {locator} }}\n");
        assert!(
            parse_manifest("hostile/dependency/ling.toml", manifest.as_bytes()).is_err(),
            "manifest v1 must reject hostile dependency locator {locator}"
        );
    }
}

#[test]
fn unavailable_supply_chain_protocols_have_no_project_execution_route() {
    const PROJECT_SOURCE: &str = concat!(
        include_str!("../src/lib.rs"),
        include_str!("../src/discovery.rs"),
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
        "ArchiveReader",
        "decompress_package",
        "verify_signature",
        "resolve_yanked_package",
        "execute_build_script",
    ] {
        assert!(
            !PROJECT_SOURCE.contains(forbidden),
            "unsupported supply-chain authority must not enter ling-project through {forbidden}"
        );
    }
}
