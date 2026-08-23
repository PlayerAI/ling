use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use ling_lsp::{
    DIAGNOSTIC_CONTROL_PROTOCOL_VERSION, DIAGNOSTIC_PROTOCOL_VERSION,
    DOCUMENT_SYMBOL_PROTOCOL_VERSION, HOVER_PROTOCOL_VERSION, LifecycleState,
    NAVIGATION_PROTOCOL_VERSION, OVERLAY_PROTOCOL_VERSION, PREPARE_RENAME_PROTOCOL_VERSION,
    PROTOCOL_VERSION, PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION, PULL_DIAGNOSTICS_PROTOCOL_VERSION,
    REFERENCES_PROTOCOL_VERSION, RENAME_PROTOCOL_VERSION, run_stdio,
};
use serde_json::Value;

const FIXTURE_SCHEMA: &str = "ling.test.lsp-diagnostic-transcripts/1";
const REQUIRED_CASES: [&str; 4] = [
    "invalid-control-initialize",
    "pull-parity-recovery",
    "push-unicode-recovery",
    "storm-control-recovery",
];

#[derive(Debug)]
struct FixtureCase {
    id: String,
    input: String,
    output: String,
    exit_code: u8,
    protocols: Vec<String>,
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/lsp-diagnostics-v1")
}

fn exact_keys(value: &Value, expected: &[&str], context: &str) {
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("{context} must be an object"));
    let actual = object.keys().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(actual, expected, "{context} has the wrong members");
}

fn required_string(value: &Value, key: &str, context: &str) -> String {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("{context}.{key} must be a string"))
        .to_owned()
}

fn safe_file_name(name: &str, suffix: &str) {
    let path = Path::new(name);
    assert_eq!(path.components().count(), 1, "fixture path must be local");
    assert!(
        matches!(path.components().next(), Some(Component::Normal(_))),
        "fixture path must be a normal component"
    );
    assert!(name.ends_with(suffix), "fixture file has wrong suffix");
}

fn load_manifest() -> Vec<FixtureCase> {
    let bytes = fs::read(fixture_root().join("manifest.json")).expect("read fixture manifest");
    assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]), "manifest has BOM");
    let manifest: Value = serde_json::from_slice(&bytes).expect("parse fixture manifest");
    exact_keys(&manifest, &["cases", "schema"], "manifest");
    assert_eq!(manifest["schema"], FIXTURE_SCHEMA);
    let entries = manifest["cases"].as_array().expect("manifest.cases array");
    let active_protocols = BTreeSet::from([
        PROTOCOL_VERSION,
        OVERLAY_PROTOCOL_VERSION,
        DIAGNOSTIC_PROTOCOL_VERSION,
        PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION,
        PULL_DIAGNOSTICS_PROTOCOL_VERSION,
        DIAGNOSTIC_CONTROL_PROTOCOL_VERSION,
        DOCUMENT_SYMBOL_PROTOCOL_VERSION,
        HOVER_PROTOCOL_VERSION,
        NAVIGATION_PROTOCOL_VERSION,
        REFERENCES_PROTOCOL_VERSION,
        PREPARE_RENAME_PROTOCOL_VERSION,
        RENAME_PROTOCOL_VERSION,
    ]);
    let mut cases = Vec::with_capacity(entries.len());
    for (index, entry) in entries.iter().enumerate() {
        let context = format!("manifest.cases[{index}]");
        exact_keys(
            entry,
            &["exitCode", "id", "input", "output", "protocols"],
            &context,
        );
        let id = required_string(entry, "id", &context);
        let input = required_string(entry, "input", &context);
        let output = required_string(entry, "output", &context);
        safe_file_name(&input, ".input.jsonl");
        safe_file_name(&output, ".output.jsonl");
        let exit_code = entry["exitCode"]
            .as_u64()
            .and_then(|value| u8::try_from(value).ok())
            .unwrap_or_else(|| panic!("{context}.exitCode must fit u8"));
        let protocols = entry["protocols"]
            .as_array()
            .unwrap_or_else(|| panic!("{context}.protocols must be an array"))
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .unwrap_or_else(|| panic!("{context}.protocols entries must be strings"))
                    .to_owned()
            })
            .collect::<Vec<_>>();
        assert!(
            !protocols.is_empty(),
            "{context}.protocols must not be empty"
        );
        assert!(
            protocols
                .iter()
                .all(|protocol| active_protocols.contains(protocol.as_str())),
            "{context} names an inactive protocol"
        );
        cases.push(FixtureCase {
            id,
            input,
            output,
            exit_code,
            protocols,
        });
    }
    let ids = cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, REQUIRED_CASES, "required sorted fixture cases");
    let inputs = cases
        .iter()
        .map(|case| case.input.as_str())
        .collect::<BTreeSet<_>>();
    let outputs = cases
        .iter()
        .map(|case| case.output.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(inputs.len(), cases.len(), "duplicate fixture input");
    assert_eq!(outputs.len(), cases.len(), "duplicate fixture output");
    cases
}

fn payload_lines(path: &Path) -> Vec<Vec<u8>> {
    let bytes = fs::read(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        !bytes.starts_with(&[0xef, 0xbb, 0xbf]),
        "{} has BOM",
        path.display()
    );
    assert!(bytes.ends_with(b"\n"), "{} needs final LF", path.display());
    let lines = bytes[..bytes.len() - 1]
        .split(|byte| *byte == b'\n')
        .map(|line| {
            assert!(!line.is_empty(), "{} has a blank line", path.display());
            assert!(!line.ends_with(b"\r"), "{} uses CRLF", path.display());
            let value: Value = serde_json::from_slice(line)
                .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));
            assert!(
                value.is_object(),
                "{} line root must be object",
                path.display()
            );
            assert_eq!(
                serde_json::to_vec(&value).expect("serialize fixture payload"),
                line,
                "{} line must be compact canonical JSON without duplicate keys",
                path.display()
            );
            line.to_vec()
        })
        .collect::<Vec<_>>();
    assert!(!lines.is_empty(), "{} must not be empty", path.display());
    lines
}

fn framed_input(lines: &[Vec<u8>]) -> Vec<u8> {
    lines
        .iter()
        .flat_map(|line| {
            let mut frame = format!("Content-Length: {}\r\n\r\n", line.len()).into_bytes();
            frame.extend_from_slice(line);
            frame
        })
        .collect()
}

fn decode_frames(mut bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut bodies = Vec::new();
    while !bytes.is_empty() {
        let header_end = bytes
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .expect("complete output header");
        let header = std::str::from_utf8(&bytes[..header_end]).expect("ASCII output header");
        let length = header
            .strip_prefix("Content-Length: ")
            .and_then(|value| value.parse::<usize>().ok())
            .expect("single numeric Content-Length header");
        let body_start = header_end + 4;
        let body_end = body_start
            .checked_add(length)
            .expect("frame length overflow");
        assert!(body_end <= bytes.len(), "truncated output body");
        bodies.push(bytes[body_start..body_end].to_vec());
        bytes = &bytes[body_end..];
    }
    bodies
}

fn execute(case: &FixtureCase) -> (Vec<Vec<u8>>, Vec<Value>) {
    let root = fixture_root();
    let input = framed_input(&payload_lines(&root.join(&case.input)));
    let expected = payload_lines(&root.join(&case.output));
    let mut first_output = Vec::new();
    let first = run_stdio(input.as_slice(), &mut first_output).expect("first fixture execution");
    let mut second_output = Vec::new();
    let second = run_stdio(input.as_slice(), &mut second_output).expect("second fixture execution");
    assert_eq!(first.exit_code(), case.exit_code, "{} exit code", case.id);
    assert_eq!(first.state(), LifecycleState::Exited, "{} state", case.id);
    assert_eq!(second, first, "{} repeated result", case.id);
    assert_eq!(second_output, first_output, "{} repeated bytes", case.id);
    let actual = decode_frames(&first_output);
    assert_eq!(actual, expected, "{} exact output bodies", case.id);
    let values = actual
        .iter()
        .map(|body| serde_json::from_slice(body).expect("output body JSON"))
        .collect();
    (actual, values)
}

fn publications(values: &[Value]) -> Vec<&Value> {
    values
        .iter()
        .filter(|value| value["method"] == "textDocument/publishDiagnostics")
        .collect()
}

#[test]
fn diagnostic_transcript_corpus_is_exact_and_deterministic() {
    let cases = load_manifest();
    assert!(
        cases.iter().all(|case| !case.protocols.is_empty()),
        "protocol metadata is present"
    );
    for case in &cases {
        let (_, values) = execute(case);
        match case.id.as_str() {
            "invalid-control-initialize" => {
                assert_eq!(values.len(), 1);
                assert_eq!(values[0]["error"]["code"], -32602);
            }
            "pull-parity-recovery" => {
                let push = publications(&values);
                assert_eq!(push.len(), 2);
                assert_eq!(push[0]["params"]["version"], 1);
                assert_eq!(push[1]["params"]["diagnostics"], Value::Array(vec![]));
                let pull = values
                    .iter()
                    .find(|value| value["id"] == 2)
                    .expect("full pull");
                assert_eq!(
                    pull["result"]["items"], push[0]["params"]["diagnostics"],
                    "push and pull arrays are byte-equivalent values"
                );
                let recovered = values
                    .iter()
                    .find(|value| value["id"] == 3)
                    .expect("recovery pull");
                assert_eq!(recovered["result"]["items"], Value::Array(vec![]));
            }
            "push-unicode-recovery" => {
                let push = publications(&values);
                assert_eq!(push.len(), 3);
                let diagnostic = &push[1]["params"]["diagnostics"][0];
                assert_eq!(diagnostic["code"], "L-TYPE-0001");
                assert_eq!(diagnostic["range"]["start"]["line"], 2);
                assert!(
                    diagnostic["message"]
                        .as_str()
                        .is_some_and(|message| message.contains(" / "))
                );
                assert_eq!(push[2]["params"]["version"], 3);
                assert_eq!(push[2]["params"]["diagnostics"], Value::Array(vec![]));
            }
            "storm-control-recovery" => {
                let push = publications(&values);
                assert_eq!(push.len(), 2);
                let diagnostics = push[0]["params"]["diagnostics"]
                    .as_array()
                    .expect("diagnostics");
                assert_eq!(diagnostics.len(), 2);
                assert_eq!(diagnostics[1]["code"], "L-LSP-0001");
                assert_eq!(diagnostics[1]["data"]["facts"]["scope"], "document");
                assert_eq!(push[1]["params"]["diagnostics"], Value::Array(vec![]));
            }
            _ => unreachable!("manifest case set was validated"),
        }
    }
}
