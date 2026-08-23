use ling_lsp::{HandleOutcome, LspServer, MAX_NAVIGATION_TARGETS, NAVIGATION_PROTOCOL_VERSION};
use serde_json::{Value, json};

fn request(id: u64, method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "id": id,
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    }))
    .expect("request JSON")
}

fn notification(method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    }))
    .expect("notification JSON")
}

fn response(server: &mut LspServer, id: u64, method: &str, params: Value) -> Value {
    match server.handle_json(&request(id, method, params)) {
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
                    "declaration": {"dynamicRegistration": false},
                    "definition": {"dynamicRegistration": true},
                    "typeDefinition": {},
                },
            },
            "workspaceFolders": [],
        }),
    );
    assert!(matches!(
        server.handle_json(&notification("initialized", json!({}))),
        HandleOutcome::NoResponse
    ));
    (server, initialized)
}

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert!(matches!(
        server.handle_json(&notification(
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

fn navigate(
    server: &mut LspServer,
    id: u64,
    method: &str,
    uri: &str,
    line: u32,
    character: u32,
) -> Value {
    response(
        server,
        id,
        method,
        json!({
            "position": {"character": character, "line": line},
            "textDocument": {"uri": uri},
        }),
    )
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
        _ => panic!("unsupported test encoding"),
    };
    (line, u32::try_from(character).expect("character fits u32"))
}

#[test]
fn initialize_advertises_exact_navigation_providers_and_rejects_malformed_capabilities() {
    let (_, initialized) = ready("utf-16");
    let capabilities = &initialized["result"]["capabilities"];
    assert_eq!(capabilities["declarationProvider"], true);
    assert_eq!(capabilities["definitionProvider"], true);
    assert_eq!(capabilities["typeDefinitionProvider"], true);
    assert_eq!(
        capabilities["experimental"]["lingNavigation"],
        json!({
            "maxTargets": MAX_NAVIGATION_TARGETS,
            "version": NAVIGATION_PROTOCOL_VERSION,
        })
    );

    for text_document in [
        json!({"definition": true}),
        json!({"declaration": {"dynamicRegistration": "false"}}),
        json!({"typeDefinition": []}),
    ] {
        let mut server = LspServer::new();
        let failure = response(
            &mut server,
            1,
            "initialize",
            json!({"capabilities": {"textDocument": text_document}}),
        );
        assert_eq!(failure["error"]["code"], -32602);
    }
}

#[test]
fn definition_declaration_and_type_definition_return_exact_unique_locations() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n\n",
        "type Item = { name: Text }\n",
        "type State =\n",
        "    | Idle\n",
        "    | Ready of Int\n\n",
        "let make name = { name = name }\n",
        "let main () =\n",
        "    let local = make \"Ling\"\n",
        "    let state = Ready 1\n",
        "    local\n",
    );
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, 1, source);

    let (line, character) = position(source, "make", 1, "utf-16");
    let definition = navigate(
        &mut server,
        2,
        "textDocument/definition",
        uri,
        line,
        character,
    );
    let declaration = navigate(
        &mut server,
        3,
        "textDocument/declaration",
        uri,
        line,
        character,
    );
    assert_eq!(definition["result"], declaration["result"]);
    assert_eq!(definition["result"]["uri"], uri);
    assert_eq!(
        definition["result"]["range"],
        json!({
            "end": {"character": 8, "line": 7},
            "start": {"character": 4, "line": 7},
        })
    );

    let type_definition = navigate(
        &mut server,
        4,
        "textDocument/typeDefinition",
        uri,
        line,
        character,
    );
    assert_eq!(
        type_definition["result"]["range"],
        json!({
            "end": {"character": 9, "line": 2},
            "start": {"character": 5, "line": 2},
        })
    );

    let (line, character) = position(source, "Ready", 1, "utf-16");
    let constructor_type = navigate(
        &mut server,
        5,
        "textDocument/typeDefinition",
        uri,
        line,
        character,
    );
    assert_eq!(constructor_type["result"]["range"]["start"]["line"], 3);

    let (line, character) = position(source, "local", 1, "utf-16");
    let local = navigate(
        &mut server,
        6,
        "textDocument/definition",
        uri,
        line,
        character,
    );
    assert_eq!(local["result"]["range"]["start"]["line"], 9);

    let (line, character) = position(source, "name", 3, "utf-16");
    let parameter = navigate(
        &mut server,
        7,
        "textDocument/declaration",
        uri,
        line,
        character,
    );
    assert_eq!(
        parameter["result"]["range"],
        json!({
            "end": {"character": 13, "line": 7},
            "start": {"character": 9, "line": 7},
        })
    );

    let (line, character) = position(source, "make", 0, "utf-16");
    assert_eq!(
        navigate(
            &mut server,
            8,
            "textDocument/definition",
            uri,
            line,
            character,
        )["result"],
        Value::Null
    );
}

#[test]
fn trait_member_calls_navigate_to_the_exact_member_identifier() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n\n",
        "trait Renderable<'a> =\n",
        "    render: 'a -> Text\n\n",
        "type Item = { name: Text }\n\n",
        "impl Renderable Item =\n",
        "    let render item = item.name\n\n",
        "let main () = Renderable.render { name = \"Ling\" }\n",
    );
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, 1, source);
    let (line, character) = position(source, "render", 2, "utf-16");
    let result = navigate(
        &mut server,
        2,
        "textDocument/definition",
        uri,
        line,
        character,
    );
    assert_eq!(
        result["result"],
        json!({
            "range": {
                "end": {"character": 10, "line": 3},
                "start": {"character": 4, "line": 3},
            },
            "uri": uri,
        })
    );
}

#[test]
fn cross_document_and_read_only_dependency_targets_preserve_registered_uris() {
    let main_uri = "ling://workspace/src/Main.ling";
    let dependency_uri = "ling://dependency/pkg/src/Support.ling";
    let main = "module Main\n\nimport Support as S\n\nlet main = S.answer\n";
    let dependency = "module Support\n\nlet answer = 42\n";
    let mut results = Vec::new();
    for reverse in [false, true] {
        let (mut server, _) = ready("utf-16");
        if reverse {
            open(&mut server, main_uri, 1, main);
            server
                .publish_disk_snapshot(dependency_uri, dependency)
                .expect("dependency snapshot");
        } else {
            server
                .publish_disk_snapshot(dependency_uri, dependency)
                .expect("dependency snapshot");
            open(&mut server, main_uri, 1, main);
        }
        let (line, character) = position(main, "answer", 0, "utf-16");
        results.push(
            navigate(
                &mut server,
                2,
                "textDocument/definition",
                main_uri,
                line,
                character,
            )["result"]
                .clone(),
        );
    }
    assert_eq!(results[0], results[1]);
    assert_eq!(results[0]["uri"], dependency_uri);
    assert_eq!(
        results[0]["range"],
        json!({
            "end": {"character": 10, "line": 2},
            "start": {"character": 4, "line": 2},
        })
    );
}

#[test]
fn negotiated_encodings_project_bom_crlf_unicode_target_ranges() {
    let uri = "ling://workspace/src/Main.ling";
    let source = "\u{feff}module Main\r\n\r\n/*🙂e\u{301}*/ let 标识 = 1\r\nlet main = 标识\r\n";
    for encoding in ["utf-8", "utf-16", "utf-32"] {
        let (mut server, _) = ready(encoding);
        open(&mut server, uri, 1, source);
        let (line, character) = position(source, "标识", 1, encoding);
        let result = navigate(
            &mut server,
            2,
            "textDocument/definition",
            uri,
            line,
            character,
        );
        let expected_start = match encoding {
            "utf-8" => 16,
            "utf-16" => 13,
            "utf-32" => 12,
            _ => unreachable!(),
        };
        assert_eq!(
            result["result"]["range"],
            json!({
                "end": {"character": expected_start + if encoding == "utf-8" { 6 } else { 2 }, "line": 2},
                "start": {"character": expected_start, "line": 2},
            }),
            "{encoding}: {result}"
        );
    }
}

#[test]
fn invalid_inputs_checked_failure_null_and_notification_behavior_are_atomic() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let helper = 1\n",
        "let main () = Console.write (helper + \"bad\")\n",
    );
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, 1, source);
    let (line, character) = position(source, "helper", 1, "utf-16");
    let definition = navigate(
        &mut server,
        2,
        "textDocument/definition",
        uri,
        line,
        character,
    );
    assert_eq!(definition["result"]["range"]["start"]["line"], 3);
    let type_failure = navigate(
        &mut server,
        3,
        "textDocument/typeDefinition",
        uri,
        line,
        character,
    );
    assert_eq!(type_failure["error"]["code"], -32803);

    let (line, character) = position(source, "write", 0, "utf-16");
    assert_eq!(
        navigate(
            &mut server,
            4,
            "textDocument/definition",
            uri,
            line,
            character,
        )["result"],
        Value::Null
    );
    assert_eq!(
        response(
            &mut server,
            5,
            "textDocument/definition",
            json!({"position": {"character": -1, "line": 0}, "textDocument": {"uri": uri}}),
        )["error"]["code"],
        -32602
    );
    assert!(matches!(
        server.handle_json(&notification(
            "textDocument/typeDefinition",
            json!({
                "position": {"character": character, "line": line},
                "textDocument": {"uri": uri},
            }),
        )),
        HandleOutcome::NoResponse
    ));
}

#[test]
fn temporary_navigation_is_isolated_and_composite_type_targets_are_null() {
    let workspace_uri = "ling://workspace/src/Support.ling";
    let temporary_uri = "untitled://ling/Scratch.ling";
    let (mut server, _) = ready("utf-16");
    server
        .publish_disk_snapshot(workspace_uri, "module Support\n\nlet answer = 42\n")
        .expect("workspace snapshot");
    let temporary = concat!(
        "module Scratch\n\n",
        "let numbers = [1; 2]\n",
        "let local = numbers\n",
        "let identity value = value\n",
        "let generic = identity 1\n",
    );
    open(&mut server, temporary_uri, 1, temporary);
    let (line, character) = position(temporary, "numbers", 1, "utf-16");
    let definition = navigate(
        &mut server,
        2,
        "textDocument/definition",
        temporary_uri,
        line,
        character,
    );
    assert_eq!(definition["result"]["uri"], temporary_uri);
    assert_eq!(
        navigate(
            &mut server,
            3,
            "textDocument/typeDefinition",
            temporary_uri,
            line,
            character,
        )["result"],
        Value::Null
    );

    let (line, character) = position(temporary, "identity", 1, "utf-16");
    assert_eq!(
        navigate(
            &mut server,
            4,
            "textDocument/typeDefinition",
            temporary_uri,
            line,
            character,
        )["result"],
        Value::Null
    );

    let isolated_uri = "untitled://ling/Importing.ling";
    let importing = "module Importing\n\nimport Support as S\n\nlet main = S.answer\n";
    open(&mut server, isolated_uri, 1, importing);
    let (line, character) = position(importing, "answer", 0, "utf-16");
    assert_eq!(
        navigate(
            &mut server,
            5,
            "textDocument/definition",
            isolated_uri,
            line,
            character,
        )["error"]["code"],
        -32803
    );
}
