use ling_lsp::{
    HOVER_PROTOCOL_VERSION, HandleOutcome, JSON_RPC_VERSION, LspServer, MAX_HOVER_CONTENT_BYTES,
    MAX_HOVER_ENTRIES,
};
use serde_json::{Value, json};

fn request(id: u64, method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "id": id,
        "jsonrpc": JSON_RPC_VERSION,
        "method": method,
        "params": params,
    }))
    .expect("request JSON")
}

fn notification(method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": JSON_RPC_VERSION,
        "method": method,
        "params": params,
    }))
    .expect("notification JSON")
}

fn response(server: &mut LspServer, id: u64, method: &str, params: Value) -> Value {
    let HandleOutcome::Response(bytes) = server.handle_json(&request(id, method, params)) else {
        panic!("request must respond")
    };
    serde_json::from_slice(&bytes).expect("response JSON")
}

fn ready(formats: Option<&[&str]>, encoding: &str) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let hover = formats.map_or_else(|| json!({}), |formats| json!({"contentFormat": formats}));
    let initialized = response(
        &mut server,
        1,
        "initialize",
        json!({
            "capabilities": {
                "general": {"positionEncodings": [encoding]},
                "textDocument": {"hover": hover},
            },
        }),
    );
    assert_eq!(
        server.handle_json(&notification("initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    (server, initialized)
}

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&notification(
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

fn hover(server: &mut LspServer, id: u64, uri: &str, line: u32, character: u32) -> Value {
    response(
        server,
        id,
        "textDocument/hover",
        json!({
            "position": {"character": character, "line": line},
            "textDocument": {"uri": uri},
        }),
    )
}

fn position(text: &str, needle: &str, occurrence: usize, encoding: &str) -> (u32, u32) {
    let offset = text
        .match_indices(needle)
        .nth(occurrence)
        .unwrap_or_else(|| panic!("missing occurrence {occurrence} of {needle}"))
        .0;
    let prefix = &text[..offset];
    let line = u32::try_from(prefix.matches('\n').count()).expect("line fits u32");
    let line_text = prefix.rsplit_once('\n').map_or(prefix, |(_, line)| line);
    let character = match encoding {
        "utf-8" => line_text.len(),
        "utf-16" => line_text.encode_utf16().count(),
        "utf-32" => line_text.chars().count(),
        _ => panic!("unsupported test encoding"),
    };
    (line, u32::try_from(character).expect("character fits u32"))
}

#[test]
fn plaintext_hover_returns_canonical_signature_kind_and_exact_range() {
    let uri = "ling://workspace/src/Main.ling";
    let source = "module Main\n\nlet identity value = value\n";
    let (mut server, initialized) = ready(None, "utf-16");
    assert_eq!(initialized["result"]["capabilities"]["hoverProvider"], true);
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingHover"],
        json!({
            "maxContentBytes": MAX_HOVER_CONTENT_BYTES,
            "maxEntries": MAX_HOVER_ENTRIES,
            "markup": "plaintext",
            "version": HOVER_PROTOCOL_VERSION,
        })
    );
    open(&mut server, uri, 1, source);

    let definition = hover(&mut server, 2, uri, 2, 5);
    assert_eq!(
        definition["result"],
        json!({
            "contents": {
                "kind": "plaintext",
                "value": "identity: 'a -> 'a\n种类 / kind: value",
            },
            "range": {
                "end": {"character": 12, "line": 2},
                "start": {"character": 4, "line": 2},
            },
        })
    );
    let reference = hover(&mut server, 3, uri, 2, 22);
    assert_eq!(
        reference["result"]["contents"]["value"],
        "value: 'a\n种类 / kind: parameter"
    );
    assert_eq!(
        reference["result"]["range"],
        json!({
            "end": {"character": 26, "line": 2},
            "start": {"character": 21, "line": 2},
        })
    );
    assert_eq!(hover(&mut server, 4, uri, 1, 0)["result"], Value::Null);
    assert_eq!(
        hover(&mut server, 5, uri, 2, 5)["result"],
        definition["result"]
    );
}

#[test]
fn markdown_hover_includes_effect_capability_and_trait_selection() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "trait Renderable<'a> =\n",
        "    render: 'a -> Text\n\n",
        "type Item = { name: Text }\n\n",
        "impl Renderable Item =\n",
        "    let render item = item.name\n\n",
        "let show () = Console.write (Renderable.render { name = \"Ling\" })\n",
    );
    let (mut server, initialized) = ready(Some(&["markdown", "plaintext"]), "utf-16");
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingHover"]["markup"],
        "markdown"
    );
    open(&mut server, uri, 1, source);

    let (line, character) = position(source, "show", 0, "utf-16");
    let show = hover(&mut server, 2, uri, line, character);
    assert_eq!(show["result"]["contents"]["kind"], "markdown");
    assert_eq!(
        show["result"]["contents"]["value"],
        "```ling\nshow: Unit -> Unit\n```\n- 种类 / kind: `value`\n- 效果 / effects: `Console.Write`\n- 能力 / capabilities: `Console.Write`"
    );

    let (line, character) = position(source, "render", 2, "utf-16");
    let selected = hover(&mut server, 3, uri, line, character);
    assert!(
        selected["result"]["contents"]["value"]
            .as_str()
            .is_some_and(
                |value| value.contains("Trait 选择 / Trait selection: `Renderable<Item>.render`")
            ),
        "{selected}"
    );
    assert!(selected["result"].get("data").is_none());
}

#[test]
fn public_hover_covers_every_checked_seed_target_kind_without_internal_ordinals() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "trait Renderable<'a> =\n",
        "    render: 'a -> Text\n\n",
        "type Item = { name: Text }\n",
        "type State =\n",
        "    | Idle\n",
        "    | Ready of Int\n\n",
        "impl Renderable Item =\n",
        "    let render item = item.name\n\n",
        "let main value =\n",
        "    let local = Renderable.render { name = value }\n",
        "    Console.write local\n",
    );
    let (mut server, _) = ready(None, "utf-16");
    open(&mut server, uri, 1, source);

    let targets = [
        ("main", 0, "value", "main"),
        ("State", 0, "type", "State"),
        ("Idle", 0, "constructor", "Idle"),
        ("write", 0, "builtin", "Console.write"),
        ("render", 0, "trait-member", "Renderable.render"),
        ("render", 1, "implementation-member", "render"),
        ("local", 0, "binding", "local"),
        ("value", 0, "parameter", "value"),
    ];
    for (index, (needle, occurrence, kind, name)) in targets.into_iter().enumerate() {
        let (line, character) = position(source, needle, occurrence, "utf-16");
        let result = hover(
            &mut server,
            u64::try_from(index + 2).expect("request id fits u64"),
            uri,
            line,
            character,
        );
        let content = result["result"]["contents"]["value"]
            .as_str()
            .expect("plaintext hover content");
        assert!(content.starts_with(name), "{needle}: {result}");
        assert!(
            content.contains(&format!("种类 / kind: {kind}")),
            "{needle}: {result}"
        );
        assert!(!content.contains("#0"), "{needle}: {result}");
        assert_eq!(
            result["result"]["range"],
            json!({
                "end": {
                    "character": character + u32::try_from(needle.len()).unwrap(),
                    "line": line,
                },
                "start": {"character": character, "line": line},
            }),
            "{needle}: {result}"
        );
    }
}

#[test]
fn negotiated_encodings_project_unicode_identifier_ranges() {
    let uri = "ling://workspace/src/Main.ling";
    let source = "\u{feff}module Main\r\n\r\n/*🙂e\u{301}*/ let 标识 value = value\r\n";
    for encoding in ["utf-8", "utf-16", "utf-32"] {
        let (mut server, _) = ready(Some(&["plaintext"]), encoding);
        open(&mut server, uri, 1, source);
        let (line, character) = position(source, "标识", 0, encoding);
        let result = hover(&mut server, 2, uri, line, character);
        assert_eq!(
            result["result"]["range"]["start"],
            json!({"character": character, "line": line}),
            "{encoding}: {result}"
        );
        let expected_end = character
            + match encoding {
                "utf-8" => 6,
                "utf-16" | "utf-32" => 2,
                _ => unreachable!(),
            };
        assert_eq!(result["result"]["range"]["end"]["character"], expected_end);
    }
}

#[test]
fn invalid_capabilities_params_positions_and_checked_failures_are_atomic() {
    for hover_capability in [
        json!(true),
        json!({"contentFormat": []}),
        json!({"contentFormat": ["html"]}),
        json!({"contentFormat": [7]}),
    ] {
        let mut server = LspServer::new();
        let result = response(
            &mut server,
            1,
            "initialize",
            json!({"capabilities": {"textDocument": {"hover": hover_capability}}}),
        );
        assert_eq!(result["error"]["code"], -32602);
    }

    let uri = "ling://workspace/src/Main.ling";
    let (mut server, _) = ready(None, "utf-16");
    open(&mut server, uri, 1, "module Main\n\nlet value = missing\n");
    assert_eq!(hover(&mut server, 2, uri, 99, 0)["error"]["code"], -32602);
    assert_eq!(
        hover(&mut server, 3, "ling://workspace/src/Missing.ling", 0, 0)["error"]["code"],
        -32602
    );
    assert_eq!(hover(&mut server, 4, uri, 2, 5)["error"]["code"], -32803);
    assert_eq!(
        server.handle_json(&notification(
            "textDocument/hover",
            json!({
                "position": {"character": 5, "line": 2},
                "textDocument": {"uri": uri},
            }),
        )),
        HandleOutcome::NoResponse
    );
    assert_eq!(
        server.handle_json(&notification(
            "textDocument/didChange",
            json!({
                "contentChanges": [{"text": "module Main\n\nlet value = 1\n"}],
                "textDocument": {"uri": uri, "version": 2},
            }),
        )),
        HandleOutcome::NoResponse
    );
    assert_eq!(
        hover(&mut server, 5, uri, 2, 5)["result"]["contents"]["value"],
        "value: Int\n种类 / kind: value"
    );
}

#[test]
fn temporary_hover_is_isolated_and_content_overflow_fails_without_truncation() {
    let (mut server, _) = ready(None, "utf-16");
    open(
        &mut server,
        "ling://workspace/src/Broken.ling",
        1,
        "module Scratch\n\nlet broken = missing\n",
    );
    let temporary_uri = "untitled://ling/scratch/Main.ling";
    open(
        &mut server,
        temporary_uri,
        1,
        "module Scratch\n\nlet temporary value = value\n",
    );
    assert_eq!(
        hover(&mut server, 2, temporary_uri, 2, 5)["result"]["contents"]["value"],
        "temporary: 'a -> 'a\n种类 / kind: value"
    );

    let long_name = format!("a{}", "b".repeat(MAX_HOVER_CONTENT_BYTES));
    let oversized_uri = "untitled://ling/scratch/Oversized.ling";
    let source = format!("module Oversized\n\nlet {long_name} = 1\n");
    open(&mut server, oversized_uri, 1, &source);
    let result = hover(&mut server, 3, oversized_uri, 2, 5);
    assert_eq!(result["error"]["code"], -32803);
    assert!(result.get("result").is_none());
}
