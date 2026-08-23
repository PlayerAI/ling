use ling_lsp::{HandleOutcome, LspServer, MAX_REFERENCE_LOCATIONS, REFERENCES_PROTOCOL_VERSION};
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
                "textDocument": {"references": {"dynamicRegistration": false}},
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

fn references(
    server: &mut LspServer,
    id: u64,
    uri: &str,
    line: u32,
    character: u32,
    include_declaration: bool,
) -> Value {
    response(
        server,
        id,
        "textDocument/references",
        json!({
            "context": {"includeDeclaration": include_declaration},
            "position": {"character": character, "line": line},
            "textDocument": {"uri": uri},
        }),
    )
}

#[test]
fn initialize_advertises_exact_references_provider_and_validates_capability() {
    let (_, initialized) = ready("utf-16");
    let capabilities = &initialized["result"]["capabilities"];
    assert_eq!(capabilities["referencesProvider"], true);
    assert_eq!(
        capabilities["experimental"]["lingReferences"],
        json!({
            "emittedRelationKinds": ["read", "write", "call"],
            "maxLocations": MAX_REFERENCE_LOCATIONS,
            "relationKinds": ["read", "write", "call", "type", "implementation"],
            "version": REFERENCES_PROTOCOL_VERSION,
        })
    );

    for capability in [json!(true), json!([]), json!({"dynamicRegistration": 1})] {
        let mut server = LspServer::new();
        let failure = response(
            &mut server,
            1,
            "initialize",
            json!({"capabilities": {"textDocument": {"references": capability}}}),
        );
        assert_eq!(failure["error"]["code"], -32602);
    }
}

#[test]
fn declaration_and_reference_positions_return_exact_canonical_locations() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n\n",
        "let helper value = value\n",
        "let main () =\n",
        "    let first = helper 1\n",
        "    helper first\n",
    );
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, source);

    for occurrence in [0, 1, 2] {
        let (line, character) = position(source, "helper", occurrence, "utf-16");
        let result = references(
            &mut server,
            occurrence as u64 + 2,
            uri,
            line,
            character,
            true,
        );
        let locations = result["result"].as_array().expect("location array");
        assert_eq!(locations.len(), 3);
        assert!(locations.iter().all(|location| location["uri"] == uri));
        assert_eq!(
            locations[0]["range"]["start"],
            json!({"line": 2, "character": 4})
        );
        assert_eq!(
            locations[1]["range"]["start"],
            json!({"line": 4, "character": 16})
        );
        assert_eq!(
            locations[2]["range"]["start"],
            json!({"line": 5, "character": 4})
        );
    }

    let (line, character) = position(source, "helper", 1, "utf-16");
    assert_eq!(
        references(&mut server, 8, uri, line, character, false)["result"]
            .as_array()
            .expect("reference array")
            .len(),
        2
    );
}

#[test]
fn workspace_and_read_only_dependency_locations_reuse_exact_registered_uris() {
    let main_uri = "ling://workspace/src/Main.ling";
    let support_uri = "ling://dependency/Support/src/Support.ling";
    let main = "module Main\n\nimport Support as S\n\nlet main () = S.answer + S.answer\n";
    let support = "module Support\n\nlet answer = 42\n";
    let mut results = Vec::new();
    for reverse in [false, true] {
        let (mut server, _) = ready("utf-16");
        if reverse {
            server
                .publish_disk_snapshot(main_uri, main)
                .expect("main snapshot");
            server
                .publish_disk_snapshot(support_uri, support)
                .expect("dependency snapshot");
        } else {
            server
                .publish_disk_snapshot(support_uri, support)
                .expect("dependency snapshot");
            server
                .publish_disk_snapshot(main_uri, main)
                .expect("main snapshot");
        }
        let (line, character) = position(main, "answer", 0, "utf-16");
        results.push(references(&mut server, 2, main_uri, line, character, true)["result"].clone());
    }
    assert_eq!(results[0], results[1]);
    let locations = results[0].as_array().expect("location array");
    assert_eq!(locations.len(), 3);
    assert_eq!(locations[0]["uri"], support_uri);
    assert_eq!(locations[1]["uri"], main_uri);
    assert_eq!(locations[2]["uri"], main_uri);
}

#[test]
fn temporary_sources_are_self_contained_and_cannot_observe_workspace_documents() {
    let workspace_uri = "ling://workspace/src/Support.ling";
    let temporary_uri = "untitled://ling/Scratch.ling";
    let (mut server, _) = ready("utf-16");
    server
        .publish_disk_snapshot(workspace_uri, "module Support\n\nlet answer = 42\n")
        .expect("workspace snapshot");
    let temporary = concat!(
        "module Scratch\n\n",
        "let local = 1\n",
        "let main () = local + local\n",
    );
    open(&mut server, temporary_uri, temporary);
    let (line, character) = position(temporary, "local", 1, "utf-16");
    let result = references(&mut server, 2, temporary_uri, line, character, true);
    assert_eq!(
        result["result"]
            .as_array()
            .expect("temporary locations")
            .len(),
        3
    );
    assert!(
        result["result"]
            .as_array()
            .expect("temporary locations")
            .iter()
            .all(|location| location["uri"] == temporary_uri)
    );

    let importing_uri = "untitled://ling/Importing.ling";
    let importing = "module Importing\n\nimport Support as S\n\nlet main = S.answer\n";
    open(&mut server, importing_uri, importing);
    let (line, character) = position(importing, "answer", 0, "utf-16");
    assert_eq!(
        references(&mut server, 3, importing_uri, line, character, true)["error"]["code"],
        -32803
    );
}

#[test]
fn negotiated_ranges_preserve_bom_crlf_and_unicode_prefixes() {
    for encoding in ["utf-8", "utf-16", "utf-32"] {
        let uri = format!("ling://workspace/{encoding}/Main.ling");
        let source = concat!(
            "\u{feff}module Main\r\n\r\n",
            "let 名称 = 1\r\n",
            "let main () =\r\n",
            "    let prefix = \"😀e\u{301}\"\r\n",
            "    名称 + 名称\r\n",
        );
        let (mut server, _) = ready(encoding);
        open(&mut server, &uri, source);
        let (line, character) = position(source, "名称", 1, encoding);
        let result = references(&mut server, 2, &uri, line, character, true);
        let locations = result["result"].as_array().expect("location array");
        assert_eq!(locations.len(), 3);
        assert!(locations.iter().all(|location| location["uri"] == uri));
        assert_eq!(locations[0]["range"]["start"]["line"], 2);
        assert_eq!(locations[1]["range"]["start"]["line"], 5);
        assert_eq!(locations[2]["range"]["start"]["line"], 5);
    }
}

#[test]
fn checked_failure_invalid_params_null_selection_and_notifications_are_atomic() {
    let uri = "untitled://ling/References.ling";
    let source = "module References\n\nlet value = 1\nlet main () = value\n";
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, source);

    let (line, character) = position(source, "value", 1, "utf-16");
    let invalid = response(
        &mut server,
        2,
        "textDocument/references",
        json!({
            "context": {"includeDeclaration": "yes"},
            "position": {"character": character, "line": line},
            "textDocument": {"uri": uri},
        }),
    );
    assert_eq!(invalid["error"]["code"], -32602);

    assert_eq!(
        references(&mut server, 3, uri, 0, 0, true)["result"],
        json!([])
    );
    assert!(matches!(
        server.handle_json(&message(
            None,
            "textDocument/references",
            json!({
                "context": {"includeDeclaration": true},
                "position": {"character": character, "line": line},
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
    let failure = references(&mut broken_server, 2, broken_uri, line, character, true);
    assert_eq!(failure["error"]["code"], -32803);
    assert_eq!(
        failure["error"]["message"],
        "引用查询不可用 / references unavailable"
    );
}
