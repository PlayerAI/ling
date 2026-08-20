use std::fs;
use std::path::{Path, PathBuf};

use ling_project::parse_manifest;
use serde::Deserialize;
use serde_json::Value;

const FIXTURE_ROOT: &str = "../../tests/projects/manifest-v1";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Expectation {
    valid: bool,
    #[serde(default)]
    name: String,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    version: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    roots: Vec<String>,
    #[serde(default)]
    entry: String,
    #[serde(default)]
    exports: Vec<String>,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    code: String,
    #[serde(default)]
    highlight: String,
}

#[test]
fn manifest_v1_fixtures_match_the_accepted_contract() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_ROOT);
    let cases = fixture_directories(&root);
    assert_eq!(
        cases.iter().map(|path| case_name(path)).collect::<Vec<_>>(),
        [
            "duplicate-field",
            "invalid-dependency-path",
            "invalid-entry",
            "invalid-package-name",
            "non-nfc-display-name",
            "overlapping-roots",
            "path-traversal",
            "unknown-field",
            "unsupported-language",
            "valid-minimal",
            "valid-unicode",
        ]
    );

    for case in cases {
        let name = case_name(&case);
        let manifest_path = case.join("ling.toml");
        let bytes = fs::read(&manifest_path).expect("fixture manifest is readable");
        let expectation_text =
            fs::read_to_string(case.join("expect.toml")).expect("fixture expectation is readable");
        let expectation: Expectation =
            toml::from_str(&expectation_text).expect("fixture expectation is valid TOML");
        let source_name = format!("tests/projects/manifest-v1/{name}/ling.toml");
        let result = parse_manifest(&source_name, &bytes);

        if expectation.valid {
            let manifest = result.unwrap_or_else(|error| {
                panic!(
                    "{name} should be valid, got {}",
                    error
                        .diagnostic()
                        .render_human(ling_diagnostics::MessageLanguage::English)
                )
            });
            assert_eq!(manifest.package().name().as_str(), expectation.name);
            assert_eq!(
                manifest.package().display_name().map(|name| name.as_str()),
                expectation.display_name.as_deref()
            );
            assert_eq!(
                manifest.package().version().to_string(),
                expectation.version
            );
            assert_eq!(manifest.package().language().as_str(), expectation.language);
            assert_eq!(
                manifest
                    .source()
                    .roots()
                    .iter()
                    .map(|path| path.as_str())
                    .collect::<Vec<_>>(),
                expectation.roots
            );
            assert_eq!(manifest.source().entry().as_str(), expectation.entry);
            assert_eq!(
                manifest
                    .exports()
                    .iter()
                    .map(|module| module.as_str())
                    .collect::<Vec<_>>(),
                expectation.exports
            );
            assert_eq!(
                manifest
                    .dependencies()
                    .iter()
                    .map(|(dependency, details)| {
                        format!("{}={}", dependency.as_str(), details.path().as_str())
                    })
                    .collect::<Vec<_>>(),
                expectation.dependencies
            );
        } else {
            let error = result.expect_err(&format!("{name} should be invalid"));
            let rendered: Value = serde_json::from_str(
                &error
                    .diagnostic()
                    .render_json()
                    .expect("diagnostic is JSON"),
            )
            .expect("diagnostic schema value parses");
            assert_eq!(rendered["code"], expectation.code, "case {name}");
            assert_eq!(rendered["primary_span"]["file"], source_name, "case {name}");

            let start = find_once(&bytes, expectation.highlight.as_bytes());
            let end = start + expectation.highlight.len();
            assert_eq!(
                rendered["primary_span"]["start_byte"],
                u64::try_from(start).unwrap(),
                "case {name}"
            );
            assert_eq!(
                rendered["primary_span"]["end_byte"],
                u64::try_from(end).unwrap(),
                "case {name}"
            );
            assert!(
                rendered["message_zh"]
                    .as_str()
                    .is_some_and(|message| !message.is_empty()),
                "case {name} lacks Chinese diagnostics"
            );
            assert!(
                rendered["message_en"]
                    .as_str()
                    .is_some_and(|message| !message.is_empty()),
                "case {name} lacks English diagnostics"
            );
        }
    }
}

#[test]
fn crlf_and_field_order_decode_to_the_same_manifest_model() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_ROOT);
    let lf = fs::read_to_string(root.join("valid-minimal/ling.toml")).unwrap();
    let crlf = lf.replace('\n', "\r\n");
    let reordered = r#"manifest-version = 1

[source]
entry = "Main"
roots = ["src"]

[package]
language = "0.1"
version = "0.1.0"
name = "hello"
"#;

    let lf_manifest = parse_manifest("lf/ling.toml", lf.as_bytes()).unwrap();
    let crlf_manifest = parse_manifest("crlf/ling.toml", crlf.as_bytes()).unwrap();
    let reordered_manifest = parse_manifest("reordered/ling.toml", reordered.as_bytes()).unwrap();
    assert_eq!(lf_manifest, crlf_manifest);
    assert_eq!(lf_manifest, reordered_manifest);

    let commented = format!("# 注释与 😀 不影响模型\n{lf}# trailing comment\n");
    let commented_manifest = parse_manifest("commented/ling.toml", commented.as_bytes()).unwrap();
    assert_eq!(lf_manifest, commented_manifest);
}

#[test]
fn byte_boundary_failures_are_bounded_and_byte_accurate() {
    let valid = include_bytes!("../../../tests/projects/manifest-v1/valid-minimal/ling.toml");

    let mut bom = b"\xef\xbb\xbf".to_vec();
    bom.extend_from_slice(valid);
    assert_error(&bom, "L-PROJECT-0001", 0, 3);

    let mut nul = valid.to_vec();
    nul.push(0);
    let nul_start = nul.len() - 1;
    assert_error(&nul, "L-PROJECT-0001", nul_start, nul.len());

    let invalid_utf8 = [0xf0, 0x28, 0x8c, 0x28];
    assert_error(&invalid_utf8, "L-PROJECT-0001", 0, 1);

    let oversized = vec![b' '; ling_project::MAX_MANIFEST_BYTES + 1];
    assert_error(&oversized, "L-PROJECT-0001", 0, 0);
}

#[test]
fn validation_domains_and_original_unicode_spans_are_stable() {
    let cases = [
        (
            minimal_manifest().replace("manifest-version = 1", "manifest-version = 2"),
            "L-PROJECT-0003",
            "2",
            "manifest-version",
        ),
        (
            minimal_manifest().replace("version = \"0.1.0\"", "version = \"01.0.0\""),
            "L-PROJECT-0004",
            "\"01.0.0\"",
            "package.version",
        ),
        (
            minimal_manifest().replace("roots = [\"src\"]", "roots = []"),
            "L-PROJECT-0005",
            "[]",
            "source.roots",
        ),
        (
            format!(
                "{}\n[exports]\nmodules = [\"公共..人物\"]\n",
                minimal_manifest()
            ),
            "L-PROJECT-0006",
            "\"公共..人物\"",
            "",
        ),
        (
            format!(
                "{}\n[dependencies]\n\"Math\" = {{ path = \"deps/math\" }}\n",
                minimal_manifest()
            ),
            "L-PROJECT-0007",
            "\"Math\"",
            "",
        ),
        (
            format!(
                "{}\n[dependencies]\nmath = {{ path = \"deps/\\u0000math\" }}\n",
                minimal_manifest()
            ),
            "L-PROJECT-0007",
            "\"deps/\\u0000math\"",
            "",
        ),
    ];

    for (input, code, highlight, field) in cases {
        let value = diagnostic_value("matrix/ling.toml", input.as_bytes());
        assert_eq!(value["code"], code);
        assert_exact_span(&value, input.as_bytes(), highlight);
        if !field.is_empty() {
            assert_eq!(value["facts"]["field"], field);
        }
    }

    let crlf = concat!(
        "# 😀 prefix\r\n",
        "manifest-version = 1\r\n",
        "\r\n",
        "[package]\r\n",
        "name = \"hello\"\r\n",
        "display-name = \"零\u{202e}\"\r\n",
        "version = \"0.1.0\"\r\n",
        "language = \"0.1\"\r\n",
        "\r\n",
        "[source]\r\n",
        "roots = [\"src\"]\r\n",
        "entry = \"Main\"\r\n",
    );
    let value = diagnostic_value("unicode-crlf/ling.toml", crlf.as_bytes());
    assert_eq!(value["code"], "L-PROJECT-0004");
    assert_exact_span(&value, crlf.as_bytes(), "\"零\u{202e}\"");
}

#[test]
fn declared_collection_and_byte_limits_are_enforced_at_the_boundary() {
    let roots = (0..ling_project::MAX_SOURCE_ROOTS)
        .map(|index| format!("root{index}"))
        .collect::<Vec<_>>();
    let exports = (0..ling_project::MAX_EXPORTS)
        .map(|index| format!("Module{index}"))
        .collect::<Vec<_>>();
    let dependencies = (0..ling_project::MAX_DEPENDENCIES)
        .map(|index| (format!("dep{index}"), format!("deps/dep{index}")))
        .collect::<Vec<_>>();
    let at_limits = manifest_with_collections(&roots, &exports, &dependencies);
    let parsed = parse_manifest("limits/ling.toml", at_limits.as_bytes()).unwrap();
    assert_eq!(
        parsed.source().roots().len(),
        ling_project::MAX_SOURCE_ROOTS
    );
    assert_eq!(parsed.exports().len(), ling_project::MAX_EXPORTS);
    assert_eq!(parsed.dependencies().len(), ling_project::MAX_DEPENDENCIES);

    let too_many_roots = (0..=ling_project::MAX_SOURCE_ROOTS)
        .map(|index| format!("root{index}"))
        .collect::<Vec<_>>();
    assert_eq!(
        diagnostic_value(
            "roots-limit/ling.toml",
            manifest_with_collections(&too_many_roots, &[], &[]).as_bytes()
        )["code"],
        "L-PROJECT-0005"
    );

    let too_many_exports = (0..=ling_project::MAX_EXPORTS)
        .map(|index| format!("Module{index}"))
        .collect::<Vec<_>>();
    assert_eq!(
        diagnostic_value(
            "exports-limit/ling.toml",
            manifest_with_collections(&["src".to_owned()], &too_many_exports, &[]).as_bytes()
        )["code"],
        "L-PROJECT-0006"
    );

    let too_many_dependencies = (0..=ling_project::MAX_DEPENDENCIES)
        .map(|index| (format!("dep{index}"), format!("deps/dep{index}")))
        .collect::<Vec<_>>();
    assert_eq!(
        diagnostic_value(
            "dependencies-limit/ling.toml",
            manifest_with_collections(&["src".to_owned()], &[], &too_many_dependencies).as_bytes()
        )["code"],
        "L-PROJECT-0007"
    );

    let maximum_path = "a".repeat(ling_project::MAX_LOGICAL_PATH_BYTES);
    assert!(
        parse_manifest(
            "path-limit/ling.toml",
            manifest_with_collections(std::slice::from_ref(&maximum_path), &[], &[]).as_bytes()
        )
        .is_ok()
    );
    let oversized_path = "a".repeat(ling_project::MAX_LOGICAL_PATH_BYTES + 1);
    assert_eq!(
        diagnostic_value(
            "path-over-limit/ling.toml",
            manifest_with_collections(std::slice::from_ref(&oversized_path), &[], &[]).as_bytes()
        )["code"],
        "L-PROJECT-0005"
    );

    let mut exact_size = minimal_manifest().into_bytes();
    exact_size.push(b'#');
    exact_size.resize(ling_project::MAX_MANIFEST_BYTES, b' ');
    assert_eq!(exact_size.len(), ling_project::MAX_MANIFEST_BYTES);
    assert!(parse_manifest("byte-limit/ling.toml", &exact_size).is_ok());
}

#[test]
fn deterministic_mutations_never_panic_and_diagnostics_stay_bounded() {
    let seed = minimal_manifest().into_bytes();
    for index in 0..seed.len() {
        let mut deleted = seed.clone();
        deleted.remove(index);
        assert_bounded_result(&deleted);

        let mut invalid_utf8 = seed.clone();
        invalid_utf8[index] = 0xff;
        assert_bounded_result(&invalid_utf8);
    }

    let long_language = minimal_manifest().replace(
        "language = \"0.1\"",
        &format!("language = \"{}\"", "x".repeat(100_000)),
    );
    let value = diagnostic_value("bounded-facts/ling.toml", long_language.as_bytes());
    assert_eq!(value["code"], "L-PROJECT-0003");
    assert!(serde_json::to_vec(&value).unwrap().len() < 1_024);
}

fn assert_error(bytes: &[u8], code: &str, start: usize, end: usize) {
    let error = parse_manifest("boundary/ling.toml", bytes).expect_err("input must fail");
    let value: Value = serde_json::from_str(&error.diagnostic().render_json().unwrap()).unwrap();
    assert_eq!(value["code"], code);
    assert_eq!(value["primary_span"]["start_byte"], start);
    assert_eq!(value["primary_span"]["end_byte"], end);
}

fn diagnostic_value(source_name: &str, bytes: &[u8]) -> Value {
    let error = parse_manifest(source_name, bytes).expect_err("input must fail");
    serde_json::from_str(&error.diagnostic().render_json().unwrap()).unwrap()
}

fn assert_exact_span(value: &Value, bytes: &[u8], highlight: &str) {
    let start = find_once(bytes, highlight.as_bytes());
    assert_eq!(value["primary_span"]["start_byte"], start);
    assert_eq!(value["primary_span"]["end_byte"], start + highlight.len());
}

fn assert_bounded_result(bytes: &[u8]) {
    let result = std::panic::catch_unwind(|| parse_manifest("mutation/ling.toml", bytes));
    let result = result.expect("user-controlled bytes must not panic");
    if let Err(error) = result {
        assert!(error.diagnostic().render_json().unwrap().len() < 4_096);
    }
}

fn minimal_manifest() -> String {
    fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("{FIXTURE_ROOT}/valid-minimal/ling.toml")),
    )
    .unwrap()
}

fn manifest_with_collections(
    roots: &[String],
    exports: &[String],
    dependencies: &[(String, String)],
) -> String {
    let quoted_roots = roots
        .iter()
        .map(|root| format!("\"{root}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let mut manifest = format!(
        "manifest-version = 1\n\n[package]\nname = \"hello\"\nversion = \"0.1.0\"\nlanguage = \"0.1\"\n\n[source]\nroots = [{quoted_roots}]\nentry = \"Main\"\n"
    );
    if !exports.is_empty() {
        let quoted_exports = exports
            .iter()
            .map(|module| format!("\"{module}\""))
            .collect::<Vec<_>>()
            .join(", ");
        manifest.push_str(&format!("\n[exports]\nmodules = [{quoted_exports}]\n"));
    }
    if !dependencies.is_empty() {
        manifest.push_str("\n[dependencies]\n");
        for (name, path) in dependencies {
            manifest.push_str(&format!("{name} = {{ path = \"{path}\" }}\n"));
        }
    }
    manifest
}

fn fixture_directories(root: &Path) -> Vec<PathBuf> {
    let mut cases = fs::read_dir(root)
        .expect("manifest fixture root exists")
        .map(|entry| entry.expect("fixture entry is readable").path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    cases.sort_by_key(|path| case_name(path));
    cases
}

fn case_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .expect("fixture directory name is UTF-8")
        .to_owned()
}

fn find_once(haystack: &[u8], needle: &[u8]) -> usize {
    assert!(!needle.is_empty(), "fixture highlight must not be empty");
    let matches = haystack
        .windows(needle.len())
        .enumerate()
        .filter_map(|(index, candidate)| (candidate == needle).then_some(index))
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "fixture highlight must occur exactly once"
    );
    matches[0]
}
