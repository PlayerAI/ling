use ling_lsp::{
    CancellationToken, HandleOutcome, LspServer, MAX_SEMANTIC_TOKEN_DATA_ELEMENTS,
    MAX_SEMANTIC_TOKEN_RESULTS, SEMANTIC_TOKEN_PROTOCOL_VERSION, SEMANTIC_TOKEN_TAXONOMY_VERSION,
};
use serde_json::{Value, json};

const TYPES: [&str; 17] = [
    "namespace",
    "type",
    "enum",
    "interface",
    "struct",
    "typeParameter",
    "parameter",
    "variable",
    "property",
    "enumMember",
    "function",
    "method",
    "keyword",
    "comment",
    "string",
    "number",
    "operator",
];

const MODIFIERS: [&str; 7] = [
    "declaration",
    "definition",
    "readonly",
    "modification",
    "documentation",
    "defaultLibrary",
    "mutable",
];

fn message(id: Option<u64>, method: &str, params: Value) -> Vec<u8> {
    let mut value = json!({"jsonrpc": "2.0", "method": method, "params": params});
    if let Some(id) = id {
        value["id"] = json!(id);
    }
    serde_json::to_vec(&value).expect("message JSON")
}

fn response(server: &mut LspServer, id: u64, method: &str, params: Value) -> Value {
    let HandleOutcome::Response(bytes) = server.handle_json(&message(Some(id), method, params))
    else {
        panic!("request must respond")
    };
    serde_json::from_slice(&bytes).expect("response JSON")
}

fn response_with_cancellation(
    server: &mut LspServer,
    id: u64,
    method: &str,
    params: Value,
    cancellation: &CancellationToken,
) -> Value {
    let HandleOutcome::Response(bytes) =
        server.handle_json_with_cancellation(&message(Some(id), method, params), cancellation)
    else {
        panic!("request must respond")
    };
    serde_json::from_slice(&bytes).expect("response JSON")
}

fn capability(full: Value, token_types: &[&str], modifiers: &[&str]) -> Value {
    json!({
        "requests": {"full": full, "range": false},
        "tokenModifiers": modifiers,
        "tokenTypes": token_types,
        "formats": ["relative"],
        "unknownFutureMember": {"ignored": true},
    })
}

fn ready(
    encoding: &str,
    full: Value,
    token_types: &[&str],
    modifiers: &[&str],
) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let initialized = response(
        &mut server,
        1,
        "initialize",
        json!({
            "capabilities": {
                "general": {"positionEncodings": [encoding]},
                "textDocument": {
                    "semanticTokens": capability(full, token_types, modifiers),
                },
            },
        }),
    );
    assert_eq!(
        server.handle_json(&message(None, "initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    (server, initialized)
}

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&message(
            None,
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "languageId": "ling",
                    "text": text,
                    "uri": uri,
                    "version": version,
                },
            }),
        )),
        HandleOutcome::NoResponse
    );
}

fn change(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&message(
            None,
            "textDocument/didChange",
            json!({
                "contentChanges": [{"text": text}],
                "textDocument": {"uri": uri, "version": version},
            }),
        )),
        HandleOutcome::NoResponse
    );
}

fn full(server: &mut LspServer, id: u64, uri: &str) -> Value {
    response(
        server,
        id,
        "textDocument/semanticTokens/full",
        json!({"textDocument": {"uri": uri}}),
    )["result"]
        .clone()
}

fn delta(server: &mut LspServer, id: u64, uri: &str, previous: &str) -> Value {
    response(
        server,
        id,
        "textDocument/semanticTokens/full/delta",
        json!({
            "previousResultId": previous,
            "textDocument": {"uri": uri},
        }),
    )["result"]
        .clone()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DecodedToken {
    line: u32,
    start: u32,
    length: u32,
    kind: u32,
    modifiers: u32,
}

fn decode(data: &[Value]) -> Vec<DecodedToken> {
    assert_eq!(data.len() % 5, 0);
    let mut previous_line = 0_u32;
    let mut previous_start = 0_u32;
    data.chunks_exact(5)
        .map(|token| {
            let delta_line = token[0].as_u64().unwrap() as u32;
            let delta_start = token[1].as_u64().unwrap() as u32;
            let line = previous_line + delta_line;
            let start = if delta_line == 0 {
                previous_start + delta_start
            } else {
                delta_start
            };
            previous_line = line;
            previous_start = start;
            DecodedToken {
                line,
                start,
                length: token[2].as_u64().unwrap() as u32,
                kind: token[3].as_u64().unwrap() as u32,
                modifiers: token[4].as_u64().unwrap() as u32,
            }
        })
        .collect()
}

fn apply_delta(mut data: Vec<u32>, result: &Value) -> Vec<u32> {
    let mut edits = result["edits"].as_array().unwrap().clone();
    edits.sort_by_key(|edit| std::cmp::Reverse(edit["start"].as_u64().unwrap()));
    for edit in edits {
        let start = edit["start"].as_u64().unwrap() as usize;
        let delete_count = edit["deleteCount"].as_u64().unwrap() as usize;
        let inserted = edit
            .get("data")
            .and_then(Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .map(|value| value.as_u64().unwrap() as u32)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        data.splice(start..start + delete_count, inserted);
    }
    data
}

fn integer_data(result: &Value) -> Vec<u32> {
    result["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_u64().unwrap() as u32)
        .collect()
}

#[test]
fn initialize_negotiates_exact_full_delta_and_partial_legends() {
    let (server, initialized) = ready(
        "utf-16",
        json!({"delta": true}),
        &["function", "variable", "type", "unknown"],
        &["mutable", "definition", "unknown"],
    );
    assert_eq!(
        initialized["result"]["capabilities"]["semanticTokensProvider"],
        json!({
            "full": {"delta": true},
            "legend": {
                "tokenModifiers": ["definition", "mutable"],
                "tokenTypes": ["type", "variable", "function"],
            },
            "range": false,
            "workDoneProgress": false,
        })
    );
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingSemanticTokens"],
        json!({
            "delta": true,
            "generation": "ling.semantic-token-generation/0.1",
            "maxDataElements": MAX_SEMANTIC_TOKEN_DATA_ELEMENTS,
            "maxRetainedResults": MAX_SEMANTIC_TOKEN_RESULTS,
            "positionEncoding": "utf-16",
            "taxonomy": SEMANTIC_TOKEN_TAXONOMY_VERSION,
            "version": SEMANTIC_TOKEN_PROTOCOL_VERSION,
        })
    );
    assert_eq!(server.position_encoding().wire_name(), "utf-16");

    let (_, full_only) = ready("utf-8", json!(true), &TYPES, &MODIFIERS);
    assert_eq!(
        full_only["result"]["capabilities"]["semanticTokensProvider"]["full"],
        json!({"delta": false})
    );

    let mut disabled = LspServer::new();
    let result = response(
        &mut disabled,
        1,
        "initialize",
        json!({"capabilities": {"textDocument": {}}}),
    );
    assert!(
        result["result"]["capabilities"]
            .get("semanticTokensProvider")
            .is_none()
    );
    assert!(
        result["result"]["capabilities"]["experimental"]
            .get("lingSemanticTokens")
            .is_none()
    );
    for disabled_capability in [
        capability(json!(false), &TYPES, &MODIFIERS),
        capability(json!(true), &[], &MODIFIERS),
        json!({
            "formats": ["absolute"],
            "requests": {"full": true},
            "tokenModifiers": MODIFIERS,
            "tokenTypes": TYPES,
        }),
    ] {
        let mut server = LspServer::new();
        let initialized = response(
            &mut server,
            1,
            "initialize",
            json!({
                "capabilities": {
                    "textDocument": {"semanticTokens": disabled_capability},
                },
            }),
        );
        assert!(
            initialized["result"]["capabilities"]
                .get("semanticTokensProvider")
                .is_none()
        );
    }
}

#[test]
fn malformed_known_capabilities_fail_atomically() {
    for malformed in [
        json!(7),
        json!({}),
        json!({"formats": ["relative"], "requests": 1, "tokenModifiers": [], "tokenTypes": ["variable"]}),
        json!({"formats": ["relative"], "requests": {"full": {"delta": 1}}, "tokenModifiers": [], "tokenTypes": ["variable"]}),
        json!({"formats": ["relative"], "requests": {"full": true}, "tokenModifiers": [1], "tokenTypes": ["variable"]}),
        json!({"formats": ["relative"], "requests": {"full": true, "range": 1}, "tokenModifiers": [], "tokenTypes": ["variable"]}),
        json!({"formats": ["relative"], "requests": {"full": true}, "tokenModifiers": [], "tokenTypes": ["variable"], "multilineTokenSupport": "yes"}),
    ] {
        let mut server = LspServer::new();
        let initialized = response(
            &mut server,
            1,
            "initialize",
            json!({
                "capabilities": {"textDocument": {"semanticTokens": malformed}},
            }),
        );
        assert_eq!(initialized["error"]["code"], -32602);
        assert_eq!(server.state(), ling_lsp::LifecycleState::Uninitialized);
    }
}

#[test]
fn full_projection_preserves_unicode_bom_crlf_and_negotiated_units() {
    let uri = "ling://workspace/Unicode.ling";
    let source = concat!(
        "\u{feff}module Main\r\n\r\n",
        "/*🙂\r\n人物 */\r\n",
        "let 人物 = \"😀\"\r\n",
    );
    for (
        encoding,
        first_comment_length,
        second_comment_length,
        person_length,
        string_start,
        string_length,
    ) in [
        ("utf-8", 6, 9, 6, 13, 6),
        ("utf-16", 4, 5, 2, 9, 4),
        ("utf-32", 3, 5, 2, 9, 3),
    ] {
        let (mut server, _) = ready(encoding, json!({"delta": true}), &TYPES, &MODIFIERS);
        open(&mut server, uri, 7, source);
        let first = full(&mut server, 2, uri);
        assert_eq!(full(&mut server, 3, uri), first);
        let data = first["data"].as_array().unwrap();
        assert_eq!(data.len() % 5, 0);
        let tokens = decode(data);
        assert!(tokens.windows(2).all(|pair| {
            (pair[0].line, pair[0].start + pair[0].length) <= (pair[1].line, pair[1].start)
        }));
        assert!(tokens.contains(&DecodedToken {
            line: 2,
            start: 0,
            length: first_comment_length,
            kind: 13,
            modifiers: 0,
        }));
        assert!(tokens.contains(&DecodedToken {
            line: 3,
            start: 0,
            length: second_comment_length,
            kind: 13,
            modifiers: 0,
        }));
        assert!(tokens.contains(&DecodedToken {
            line: 4,
            start: 4,
            length: person_length,
            kind: 7,
            modifiers: 6,
        }));
        assert!(tokens.contains(&DecodedToken {
            line: 4,
            start: string_start,
            length: string_length,
            kind: 14,
            modifiers: 0,
        }));
        assert!(first["resultId"].as_str().unwrap().starts_with("st1-"));
        assert_eq!(first["resultId"].as_str().unwrap().len(), 68);
    }
}

#[test]
fn delta_is_canonical_equivalent_and_invalid_or_foreign_bases_return_full() {
    let first_uri = "ling://workspace/Main.ling";
    let second_uri = "ling://workspace/Other.ling";
    let (mut server, _) = ready("utf-16", json!({"delta": true}), &TYPES, &MODIFIERS);
    open(&mut server, first_uri, 1, "module Main\n\nlet alpha = 1\n");
    open(
        &mut server,
        second_uri,
        1,
        "module Other\n\nlet other = 0\n",
    );
    let base = full(&mut server, 2, first_uri);
    let base_data = integer_data(&base);
    let base_id = base["resultId"].as_str().unwrap().to_owned();

    change(
        &mut server,
        first_uri,
        2,
        "module Main\n\nlet inserted = 0\nlet alpha = 2\n",
    );
    let changed = delta(&mut server, 3, first_uri, &base_id);
    assert!(changed.get("edits").is_some());
    assert_eq!(changed["edits"].as_array().unwrap().len(), 1);
    let current = full(&mut server, 4, first_uri);
    assert_eq!(changed["resultId"], current["resultId"]);
    assert_eq!(apply_delta(base_data, &changed), integer_data(&current));

    change(
        &mut server,
        first_uri,
        3,
        "module Main\n\nlet alpha = 3\nlet beta = 4\n",
    );
    let inserted_id = current["resultId"].as_str().unwrap();
    let deletion = delta(&mut server, 5, first_uri, inserted_id);
    let after_deletion = full(&mut server, 6, first_uri);
    assert_eq!(
        apply_delta(integer_data(&current), &deletion),
        integer_data(&after_deletion)
    );

    change(
        &mut server,
        first_uri,
        4,
        "module Main\n\nlet beta = 4\nlet alpha = 3\n",
    );
    let reordered = delta(
        &mut server,
        7,
        first_uri,
        after_deletion["resultId"].as_str().unwrap(),
    );
    let after_reorder = full(&mut server, 8, first_uri);
    assert_eq!(
        apply_delta(integer_data(&after_deletion), &reordered),
        integer_data(&after_reorder)
    );

    let equal = delta(
        &mut server,
        9,
        first_uri,
        after_reorder["resultId"].as_str().unwrap(),
    );
    assert_eq!(equal["edits"], json!([]));

    let invalid = delta(&mut server, 10, first_uri, "st1-unknown");
    assert!(invalid.get("data").is_some());
    assert!(invalid.get("edits").is_none());

    let foreign = delta(&mut server, 11, second_uri, &base_id);
    assert!(foreign.get("data").is_some());
    assert!(foreign.get("edits").is_none());
}

#[test]
fn result_history_is_fifo_and_expired_bases_fall_back_to_full() {
    let uri = "ling://workspace/Main.ling";
    let text = "module Main\n\nlet value = 1\n";
    let (mut server, _) = ready("utf-16", json!({"delta": true}), &TYPES, &MODIFIERS);
    open(&mut server, uri, 1, text);
    let first = full(&mut server, 2, uri);
    let first_id = first["resultId"].as_str().unwrap().to_owned();
    for version in 2..=MAX_SEMANTIC_TOKEN_RESULTS as i64 + 1 {
        change(&mut server, uri, version, text);
        let current = full(&mut server, version as u64 + 2, uri);
        assert_ne!(current["resultId"], first["resultId"]);
    }
    let expired = delta(&mut server, 100, uri, &first_id);
    assert!(expired.get("data").is_some());
    assert!(expired.get("edits").is_none());
}

#[test]
fn temporary_closed_fallback_and_partial_projection_are_supported() {
    let temporary_uri = "untitled://ling/Scratch.ling";
    let closed_uri = "ling://workspace/Closed.ling";
    let (mut server, _) = ready(
        "utf-16",
        json!(true),
        &["variable"],
        &["definition", "mutable"],
    );
    open(
        &mut server,
        temporary_uri,
        1,
        concat!(
            "module Scratch\n\n",
            "let main () =\n",
            "    let mutable value = 1\n",
            "    value <- 2\n",
            "    value\n",
        ),
    );
    let temporary = full(&mut server, 2, temporary_uri);
    let temporary_data = temporary["data"].as_array().unwrap();
    assert!(!temporary_data.is_empty());
    assert!(temporary_data.chunks_exact(5).all(|token| token[3] == 0));
    assert!(
        temporary_data
            .chunks_exact(5)
            .any(|token| token[4].as_u64().unwrap() & 2 != 0)
    );

    server
        .publish_disk_snapshot(closed_uri, "module Closed\n\nlet value = 1\n")
        .unwrap();
    let closed = full(&mut server, 3, closed_uri);
    assert!(!closed["data"].as_array().unwrap().is_empty());

    change(
        &mut server,
        temporary_uri,
        2,
        "module Scratch\n\nlet broken =\n\"text\" + 1\n",
    );
    let fallback = full(&mut server, 4, temporary_uri);
    assert!(fallback["data"].as_array().unwrap().is_empty());
}

#[test]
fn lifecycle_params_cancellation_methods_and_limits_are_atomic() {
    let uri = "ling://workspace/Main.ling";
    let mut uninitialized = LspServer::new();
    assert_eq!(
        response(
            &mut uninitialized,
            1,
            "textDocument/semanticTokens/full",
            json!({"textDocument": {"uri": uri}}),
        )["error"]["code"],
        -32002
    );

    let (mut full_only, _) = ready("utf-16", json!(true), &TYPES, &MODIFIERS);
    open(&mut full_only, uri, 1, "module Main\n\nlet value = 1\n");
    assert_eq!(
        response(
            &mut full_only,
            2,
            "textDocument/semanticTokens/full/delta",
            json!({"previousResultId": "st1-x", "textDocument": {"uri": uri}}),
        )["error"]["code"],
        -32601
    );
    assert_eq!(
        full_only.handle_json(&message(
            None,
            "textDocument/semanticTokens/full",
            json!({"textDocument": {"uri": uri}}),
        )),
        HandleOutcome::NoResponse
    );
    for params in [
        json!({}),
        json!({"textDocument": {"uri": 1}}),
        json!({"textDocument": {"uri": ""}}),
        json!({"textDocument": {"uri": "bad\u{0}uri"}}),
    ] {
        assert_eq!(
            response(
                &mut full_only,
                3,
                "textDocument/semanticTokens/full",
                params,
            )["error"]["code"],
            -32602
        );
    }
    assert_eq!(
        response(
            &mut full_only,
            4,
            "textDocument/semanticTokens/full",
            json!({"textDocument": {"uri": "ling://workspace/Missing.ling"}}),
        )["error"]["code"],
        -32803
    );

    let cancellation = CancellationToken::new();
    cancellation.cancel();
    assert_eq!(
        response_with_cancellation(
            &mut full_only,
            5,
            "textDocument/semanticTokens/full",
            json!({"textDocument": {"uri": uri}}),
            &cancellation,
        )["error"]["code"],
        -32800
    );
    assert!(full(&mut full_only, 6, uri).get("data").is_some());

    let mut oversized = String::from("module Main\n\nlet broken =\n");
    oversized.push_str(&"1+".repeat(MAX_SEMANTIC_TOKEN_DATA_ELEMENTS / 5 + 1));
    change(&mut full_only, uri, 2, &oversized);
    assert_eq!(
        response(
            &mut full_only,
            7,
            "textDocument/semanticTokens/full",
            json!({"textDocument": {"uri": uri}}),
        )["error"]["code"],
        -32803
    );
}
