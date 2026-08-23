use ling_lsp::{CancellationToken, HandleOutcome, LspServer, RENAME_PROTOCOL_VERSION};
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

fn initialize(transactional: bool, encoding: &str) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let workspace = if transactional {
        json!({
            "workspaceEdit": {
                "documentChanges": true,
                "failureHandling": "transactional",
            }
        })
    } else {
        json!({})
    };
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
                "workspace": workspace,
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

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert!(matches!(
        server.handle_json(&message(
            None,
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "languageId": "ling",
                    "text": text,
                    "uri": uri,
                    "version": version,
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

fn rename(
    server: &mut LspServer,
    id: u64,
    uri: &str,
    line: u32,
    character: u32,
    new_name: &str,
) -> Value {
    response(
        server,
        id,
        "textDocument/rename",
        json!({
            "newName": new_name,
            "position": {"character": character, "line": line},
            "textDocument": {"uri": uri},
        }),
    )
}

#[test]
fn cancelled_rename_publishes_no_partial_workspace_edit() {
    let uri = "ling://workspace/src/CancelledRename.ling";
    let source = "module CancelledRename\n\nlet helper = 1\n\nlet main () = helper\n";
    let (mut server, _) = initialize(true, "utf-16");
    open(&mut server, uri, 1, source);
    let (line, character) = position(source, "helper", 1, "utf-16");
    let body = message(
        Some(2),
        "textDocument/rename",
        json!({
            "newName": "utility",
            "position": {"character": character, "line": line},
            "textDocument": {"uri": uri},
        }),
    );
    let token = CancellationToken::new();
    token.cancel();
    let HandleOutcome::Response(bytes) = server.handle_json_with_cancellation(&body, &token) else {
        panic!("cancelled rename must respond")
    };
    let cancelled: Value = serde_json::from_slice(&bytes).expect("response JSON");
    assert_eq!(cancelled["error"]["code"], -32_800);
    assert!(cancelled.get("result").is_none());

    let active = rename(&mut server, 3, uri, line, character, "utility");
    assert!(active["result"]["documentChanges"].is_array());
}

#[test]
fn initialize_advertises_exact_preview_and_requires_transactional_capability() {
    let (_, initialized) = initialize(true, "utf-16");
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingRename"],
        json!({
            "newName": "unicode17-xid-nfc-allowed-non-suspicious",
            "result": "versionedDocumentChanges",
            "transactional": true,
            "version": RENAME_PROTOCOL_VERSION,
        })
    );

    for capability in [
        json!(true),
        json!({"documentChanges": "yes"}),
        json!({"failureHandling": "partial"}),
    ] {
        let mut server = LspServer::new();
        let failure = response(
            &mut server,
            1,
            "initialize",
            json!({
                "capabilities": {"workspace": {"workspaceEdit": capability}}
            }),
        );
        assert_eq!(failure["error"]["code"], -32602);
    }

    let uri = "untitled://ling/UnsupportedRename.ling";
    let source = "module UnsupportedRename\n\nlet value = 1\n";
    let (mut unsupported, _) = initialize(false, "utf-16");
    open(&mut unsupported, uri, 1, source);
    let (line, character) = position(source, "value", 0, "utf-16");
    assert_eq!(
        rename(&mut unsupported, 2, uri, line, character, "renamed")["error"]["code"],
        -32803
    );
}

#[test]
fn definition_and_binding_renames_return_exact_versioned_edits() {
    let uri = "untitled://ling/Rename.ling";
    let source = concat!(
        "module Rename\n\n",
        "let helper value = value\n",
        "let main () = helper 1\n",
    );
    let (mut server, _) = initialize(true, "utf-16");
    open(&mut server, uri, 7, source);

    let (line, character) = position(source, "helper", 1, "utf-16");
    let result = rename(&mut server, 2, uri, line, character, "utility");
    let changes = result["result"]["documentChanges"]
        .as_array()
        .expect("document changes");
    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes[0]["textDocument"],
        json!({"uri": uri, "version": 7})
    );
    let edits = changes[0]["edits"].as_array().expect("edits");
    assert_eq!(edits.len(), 2);
    assert!(edits.iter().all(|edit| edit["newText"] == "utility"));
    assert_eq!(
        edits[0]["range"]["start"],
        json!({"line": 2, "character": 4})
    );
    assert_eq!(
        edits[1]["range"]["start"],
        json!({"line": 3, "character": 14})
    );

    let (line, character) = position(source, "value", 1, "utf-16");
    let binding = rename(&mut server, 3, uri, line, character, "item");
    assert_eq!(
        binding["result"]["documentChanges"][0]["edits"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn cross_document_definition_and_import_alias_renames_are_identity_based() {
    let main_uri = "ling://workspace/src/Main.ling";
    let support_uri = "ling://workspace/src/Support.ling";
    let main = concat!(
        "module Main\n\n",
        "import Support as S\n\n",
        "let main () = S.answer + S.answer\n",
    );
    let support = "module Support\n\nlet answer = 21\n";
    let (mut server, _) = initialize(true, "utf-16");
    open(&mut server, main_uri, 3, main);
    open(&mut server, support_uri, 4, support);

    let (line, character) = position(main, "answer", 0, "utf-16");
    let definition = rename(&mut server, 2, main_uri, line, character, "total");
    let changes = definition["result"]["documentChanges"].as_array().unwrap();
    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0]["textDocument"]["uri"], main_uri);
    assert_eq!(changes[0]["textDocument"]["version"], 3);
    assert_eq!(changes[0]["edits"].as_array().unwrap().len(), 2);
    assert_eq!(changes[1]["textDocument"]["uri"], support_uri);
    assert_eq!(changes[1]["textDocument"]["version"], 4);
    assert_eq!(changes[1]["edits"].as_array().unwrap().len(), 1);

    let (line, character) = position(main, "S.answer", 1, "utf-16");
    let alias = rename(&mut server, 3, main_uri, line, character, "Lib");
    let edits = alias["result"]["documentChanges"][0]["edits"]
        .as_array()
        .unwrap();
    assert_eq!(edits.len(), 3);
    assert!(edits.iter().all(|edit| edit["newText"] == "Lib"));
}

#[test]
fn unicode_keyword_collision_and_coherence_failures_are_atomic() {
    let uri = "untitled://ling/RenameValidation.ling";
    let source = concat!(
        "module RenameValidation\n\n",
        "let helper = 1\n",
        "let m = 2\n",
        "let existing = helper\n",
        "let main = existing\n",
    );
    let (mut server, _) = initialize(true, "utf-16");
    open(&mut server, uri, 1, source);
    let (line, character) = position(source, "helper", 0, "utf-16");
    for (id, new_name) in [
        (2, "9invalid"),
        (3, "let"),
        (4, "e\u{301}"),
        (5, "pаypal"),
        (6, "existing"),
        (7, "rn"),
    ] {
        let failure = rename(&mut server, id, uri, line, character, new_name);
        assert_eq!(failure["error"]["code"], -32803, "{new_name}");
        assert_eq!(
            failure["error"]["message"],
            "重命名不可用 / rename unavailable"
        );
    }
    assert_eq!(
        rename(&mut server, 8, uri, line, character, "helper")["result"],
        Value::Null
    );

    let trait_uri = "untitled://ling/TraitRename.ling";
    let trait_source = concat!(
        "module TraitRename\n\n",
        "trait Renderable<'a> =\n",
        "    render: 'a -> Text\n\n",
        "type Item = { name: Text }\n\n",
        "impl Renderable Item =\n",
        "    let render item = item.name\n\n",
        "let main value = Renderable.render { name = value }\n",
    );
    let (mut trait_server, _) = initialize(true, "utf-16");
    open(&mut trait_server, trait_uri, 1, trait_source);
    let (line, character) = position(trait_source, "render", 0, "utf-16");
    assert_eq!(
        rename(&mut trait_server, 2, trait_uri, line, character, "draw")["error"]["code"],
        -32803
    );
}

#[test]
fn read_only_builtin_and_empty_selections_return_null() {
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
    let (mut server, _) = initialize(true, "utf-16");
    server
        .publish_disk_snapshot(dependency_uri, dependency)
        .expect("dependency snapshot");
    open(&mut server, main_uri, 1, main);

    for (id, needle) in [(2, "write"), (3, "answer")] {
        let (line, character) = position(main, needle, 0, "utf-16");
        assert_eq!(
            rename(&mut server, id, main_uri, line, character, "changed")["result"],
            Value::Null
        );
    }
    let (line, character) = position(dependency, "answer", 0, "utf-16");
    assert_eq!(
        rename(&mut server, 4, dependency_uri, line, character, "changed",)["result"],
        Value::Null
    );
    assert_eq!(
        rename(&mut server, 5, main_uri, 0, 0, "changed")["result"],
        Value::Null
    );
}

#[test]
fn ranges_preserve_bom_crlf_unicode_encodings_and_closed_versions() {
    for encoding in ["utf-8", "utf-16", "utf-32"] {
        let uri = format!("ling://workspace/src/{encoding}.ling");
        let source = concat!(
            "\u{feff}module Main\r\n\r\n",
            "let 名称 = 1\r\n",
            "let main () =\r\n",
            "    let prefix = \"😀e\u{301}\"\r\n",
            "    名称\r\n",
        );
        let (mut server, _) = initialize(true, encoding);
        server
            .publish_disk_snapshot(&uri, source)
            .expect("closed source snapshot");
        let (line, character) = position(source, "名称", 1, encoding);
        let result = rename(&mut server, 2, &uri, line, character, "结果");
        let change = &result["result"]["documentChanges"][0];
        assert_eq!(change["textDocument"]["version"], Value::Null);
        assert_eq!(change["edits"].as_array().unwrap().len(), 2);
        assert_eq!(change["edits"][1]["range"]["start"]["line"], 5);
    }
}

#[test]
fn invalid_params_checked_failure_and_notification_produce_no_partial_edit() {
    let uri = "untitled://ling/InvalidRename.ling";
    let source = "module InvalidRename\n\nlet value = 1\nlet main = value + true\n";
    let (mut server, _) = initialize(true, "utf-16");
    open(&mut server, uri, 1, source);

    let invalid = response(
        &mut server,
        2,
        "textDocument/rename",
        json!({
            "newName": 7,
            "position": {"character": 4, "line": 2},
            "textDocument": {"uri": uri},
        }),
    );
    assert_eq!(invalid["error"]["code"], -32602);
    assert!(matches!(
        server.handle_json(&message(
            None,
            "textDocument/rename",
            json!({
                "newName": "renamed",
                "position": {"character": 4, "line": 2},
                "textDocument": {"uri": uri},
            }),
        )),
        HandleOutcome::NoResponse
    ));

    let (line, character) = position(source, "value", 1, "utf-16");
    let failure = rename(&mut server, 3, uri, line, character, "renamed");
    assert_eq!(failure["error"]["code"], -32803);
}
