use ling_lsp::{HandleOutcome, LspServer, PREPARE_RENAME_PROTOCOL_VERSION};
use serde_json::{Value, json};

fn message(id: Option<u64>, method: &str, params: Value) -> Vec<u8> {
    let mut value = json!({"jsonrpc": "2.0", "method": method, "params": params});
    if let Some(id) = id {
        value["id"] = json!(id);
    }
    serde_json::to_vec(&value).expect("message JSON")
}

fn response(server: &mut LspServer, id: u64, method: &str, params: Value) -> Value {
    match server.handle_json(&message(Some(id), method, params)) {
        HandleOutcome::Response(bytes) => serde_json::from_slice(&bytes).expect("response JSON"),
        outcome => panic!("expected response, got {outcome:?}"),
    }
}

fn ready(encoding: &str) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let initialized = response(
        &mut server,
        1,
        "initialize",
        json!({
            "capabilities": {
                "general": {"positionEncodings": [encoding]},
                "textDocument": {
                    "rename": {
                        "dynamicRegistration": false,
                        "prepareSupport": true,
                        "prepareSupportDefaultBehavior": 1,
                    }
                },
            },
            "workspaceFolders": [],
        }),
    );
    assert!(matches!(
        server.handle_json(&message(None, "initialized", json!({}))),
        HandleOutcome::NoResponse
    ));
    (server, initialized)
}

fn open(server: &mut LspServer, uri: &str, text: &str) {
    assert!(matches!(
        server.handle_json(&message(
            None,
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "languageId": "ling",
                    "text": text,
                    "uri": uri,
                    "version": 1,
                }
            }),
        )),
        HandleOutcome::NoResponse
    ));
}

fn position(text: &str, needle: &str, occurrence: usize, encoding: &str) -> (u32, u32) {
    let byte = text
        .match_indices(needle)
        .nth(occurrence)
        .unwrap_or_else(|| panic!("missing occurrence {occurrence} of {needle}"))
        .0;
    let prefix = &text[..byte];
    let line = u32::try_from(prefix.matches('\n').count()).expect("line fits u32");
    let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
    let line_prefix = &text[line_start..byte];
    let character = match encoding {
        "utf-8" => line_prefix.len(),
        "utf-16" => line_prefix.encode_utf16().count(),
        "utf-32" => line_prefix.chars().count(),
        _ => panic!("unsupported encoding"),
    };
    (line, u32::try_from(character).expect("character fits u32"))
}

fn prepare(server: &mut LspServer, id: u64, uri: &str, line: u32, character: u32) -> Value {
    response(
        server,
        id,
        "textDocument/prepareRename",
        json!({
            "position": {"character": character, "line": line},
            "textDocument": {"uri": uri},
        }),
    )
}

#[test]
fn initialize_advertises_exact_provider_and_validates_capability() {
    let (_, initialized) = ready("utf-16");
    assert_eq!(
        initialized["result"]["capabilities"]["renameProvider"],
        json!({"prepareProvider": true, "workDoneProgress": false})
    );
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingPrepareRename"],
        json!({
            "result": "rangeWithPlaceholder",
            "version": PREPARE_RENAME_PROTOCOL_VERSION,
        })
    );

    for capability in [
        json!(true),
        json!({"prepareSupport": 1}),
        json!({"prepareSupportDefaultBehavior": 2}),
    ] {
        let mut server = LspServer::new();
        let failure = response(
            &mut server,
            1,
            "initialize",
            json!({"capabilities": {"textDocument": {"rename": capability}}}),
        );
        assert_eq!(failure["error"]["code"], -32602);
    }
}

#[test]
fn declaration_reference_and_unreferenced_local_return_exact_placeholder() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n\n",
        "let helper value = value\n",
        "let unused = 1\n",
        "let main () = helper 1\n",
    );
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, source);

    for (id, needle, occurrence, expected_line, expected_character) in [
        (2, "helper", 0, 2, 4),
        (3, "helper", 1, 4, 14),
        (4, "unused", 0, 3, 4),
        (5, "value", 1, 2, 19),
    ] {
        let (line, character) = position(source, needle, occurrence, "utf-16");
        let result = prepare(&mut server, id, uri, line, character);
        assert_eq!(result["result"]["placeholder"], needle);
        assert_eq!(
            result["result"]["range"]["start"],
            json!({"line": expected_line, "character": expected_character})
        );
    }
}

#[test]
fn trait_members_and_nfc_equivalent_spellings_keep_selected_source_text() {
    let trait_uri = "untitled://ling/TraitPrepare.ling";
    let trait_source = concat!(
        "module TraitPrepare\n    requires Console.Write\n\n",
        "trait Renderable<'a> =\n",
        "    render: 'a -> Text\n\n",
        "type Item = { name: Text }\n\n",
        "impl Renderable Item =\n",
        "    let render item = item.name\n\n",
        "let main value =\n",
        "    let local = Renderable.render { name = value }\n",
        "    Console.write local\n",
    );
    let (mut server, _) = ready("utf-16");
    open(&mut server, trait_uri, trait_source);
    for (id, occurrence) in [(2, 0), (3, 1), (4, 2)] {
        let (line, character) = position(trait_source, "render", occurrence, "utf-16");
        assert_eq!(
            prepare(&mut server, id, trait_uri, line, character)["result"]["placeholder"],
            "render",
            "render occurrence {occurrence}"
        );
    }

    let unicode_uri = "untitled://ling/UnicodePrepare.ling";
    let unicode_source = "module UnicodePrepare\n\nlet e\u{301} = 1\nlet main = é\n";
    let (mut unicode_server, _) = ready("utf-16");
    open(&mut unicode_server, unicode_uri, unicode_source);
    for (id, needle) in [(2, "e\u{301}"), (3, "é")] {
        let (line, character) = position(unicode_source, needle, 0, "utf-16");
        assert_eq!(
            prepare(&mut unicode_server, id, unicode_uri, line, character)["result"]["placeholder"],
            needle
        );
    }
}

#[test]
fn read_only_dependency_and_source_less_builtin_return_null() {
    let main_uri = "ling://workspace/src/Main.ling";
    let dependency_uri = "ling://dependency/Support/src/Support.ling";
    let main = concat!(
        "module Main\n    requires Console.Write\n\n",
        "import Support as S\n\n",
        "let main () =\n",
        "    Console.write \"x\"\n",
        "    S.answer\n",
    );
    let dependency = "module Support\n\nlet answer = 42\n";
    let (mut server, _) = ready("utf-16");
    server
        .publish_disk_snapshot(dependency_uri, dependency)
        .expect("dependency snapshot");
    open(&mut server, main_uri, main);

    for (id, needle) in [(2, "write"), (3, "answer")] {
        let (line, character) = position(main, needle, 0, "utf-16");
        assert_eq!(
            prepare(&mut server, id, main_uri, line, character)["result"],
            Value::Null
        );
    }
    let (line, character) = position(dependency, "answer", 0, "utf-16");
    assert_eq!(
        prepare(&mut server, 4, dependency_uri, line, character)["result"],
        Value::Null
    );
}

#[test]
fn ranges_preserve_bom_crlf_and_unicode_for_every_encoding() {
    for encoding in ["utf-8", "utf-16", "utf-32"] {
        let uri = format!("untitled://ling/{encoding}.ling");
        let source = concat!(
            "\u{feff}module Main\r\n\r\n",
            "let 名称 = 1\r\n",
            "let main () =\r\n",
            "    let prefix = \"😀e\u{301}\"\r\n",
            "    名称\r\n",
        );
        let (mut server, _) = ready(encoding);
        open(&mut server, &uri, source);
        let (line, character) = position(source, "名称", 1, encoding);
        let result = prepare(&mut server, 2, &uri, line, character);
        assert_eq!(result["result"]["placeholder"], "名称");
        assert_eq!(result["result"]["range"]["start"]["line"], 5);
        assert_eq!(result["result"]["range"]["end"]["line"], 5);
    }
}

#[test]
fn invalid_params_empty_selection_checked_failure_and_notification_are_atomic() {
    let uri = "untitled://ling/Prepare.ling";
    let source = "module Prepare\n\nlet value = 1\nlet main () = value\n";
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, source);

    let invalid = response(
        &mut server,
        2,
        "textDocument/prepareRename",
        json!({
            "position": {"character": "zero", "line": 0},
            "textDocument": {"uri": uri},
        }),
    );
    assert_eq!(invalid["error"]["code"], -32602);
    assert_eq!(prepare(&mut server, 3, uri, 0, 0)["result"], Value::Null);
    assert!(matches!(
        server.handle_json(&message(
            None,
            "textDocument/prepareRename",
            json!({
                "position": {"character": 4, "line": 2},
                "textDocument": {"uri": uri},
            }),
        )),
        HandleOutcome::NoResponse
    ));

    let broken_uri = "untitled://ling/Broken.ling";
    let broken = "module Broken\n\nlet value = 1\nlet main () = value + true\n";
    let (mut broken_server, _) = ready("utf-16");
    open(&mut broken_server, broken_uri, broken);
    let (line, character) = position(broken, "value", 1, "utf-16");
    let failure = prepare(&mut broken_server, 2, broken_uri, line, character);
    assert_eq!(failure["error"]["code"], -32803);
    assert_eq!(
        failure["error"]["message"],
        "重命名准备不可用 / prepare rename unavailable"
    );
}
