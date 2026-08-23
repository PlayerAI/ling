use ling_lsp::{
    COMPLETION_RESOLVE_COMPLETION_VERSION, COMPLETION_RESOLVE_PROTOCOL_VERSION, HandleOutcome,
    JSON_RPC_VERSION, LspServer, MAX_COMPLETION_DOCUMENTATION_BYTES, MAX_COMPLETION_ITEMS,
    MAX_COMPLETION_RESOLVE_HANDLES,
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

fn ready(format: &[&str]) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let initialized = response(
        &mut server,
        1,
        "initialize",
        json!({
            "capabilities": {
                "general": {"positionEncodings": ["utf-16"]},
                "textDocument": {
                    "completion": {
                        "completionItem": {
                            "documentationFormat": format,
                            "resolveSupport": {
                                "properties": ["documentation", "detail"],
                            },
                        },
                        "dynamicRegistration": false,
                    },
                },
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

fn change(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&notification(
            "textDocument/didChange",
            json!({
                "contentChanges": [{"text": text}],
                "textDocument": {"uri": uri, "version": version},
            }),
        )),
        HandleOutcome::NoResponse
    );
}

fn completion(server: &mut LspServer, id: u64, uri: &str, line: u32, character: u32) -> Value {
    response(
        server,
        id,
        "textDocument/completion",
        json!({
            "context": {"triggerKind": 1},
            "position": {"character": character, "line": line},
            "textDocument": {"uri": uri},
        }),
    )
}

fn first_item(completion: &Value) -> Value {
    completion["result"]["items"][0].clone()
}

#[test]
fn initialize_negotiates_exact_resolve_contract() {
    let (_, initialized) = ready(&["markdown", "plaintext"]);
    assert_eq!(
        initialized["result"]["capabilities"]["completionProvider"],
        json!({"resolveProvider": true, "triggerCharacters": ["."]})
    );
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingCompletion"],
        json!({
            "contexts": ["expression", "member", "type", "pattern", "module", "keyword"],
            "maxItems": MAX_COMPLETION_ITEMS,
            "resolve": {
                "documentationFormat": "markdown",
                "maxHandles": MAX_COMPLETION_RESOLVE_HANDLES,
                "properties": ["detail", "documentation"],
                "version": COMPLETION_RESOLVE_PROTOCOL_VERSION,
            },
            "source": "checked",
            "version": COMPLETION_RESOLVE_COMPLETION_VERSION,
        })
    );
}

#[test]
fn checked_definition_resolves_signature_effects_and_capabilities() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "/// Writes a greeting.\n",
        "/// 写入问候。\n",
        "let show () = Console.write \"hello\"\n\n",
        "let main () = show\n",
    );
    let (mut server, _) = ready(&["plaintext"]);
    open(&mut server, uri, 1, source);

    let completed = completion(&mut server, 2, uri, 7, 18);
    let item = first_item(&completed);
    assert_eq!(item["label"], "show");
    assert_eq!(item["data"]["version"], COMPLETION_RESOLVE_PROTOCOL_VERSION);
    let handle = item["data"]["handle"].as_str().expect("opaque handle");
    assert!(handle.starts_with("ling.lsp.completion-resolve/0.1:blake3:"));
    assert!(!handle.contains(uri));
    assert!(!handle.contains("show"));

    let resolved = response(&mut server, 3, "completionItem/resolve", item.clone());
    assert_eq!(
        resolved["result"]["detail"],
        "show: Unit -> Unit ! {Console.Write} requires {Console.Write}"
    );
    assert_eq!(
        resolved["result"]["documentation"],
        json!({
            "kind": "plaintext",
            "value": concat!(
                "已检查的 Ling 符号 / checked Ling symbol\n",
                "show: Unit -> Unit ! {Console.Write} requires {Console.Write}\n",
                "文档 / documentation:\n",
                "Writes a greeting.\n",
                "写入问候。\n",
                "种类 / kind: definition\n",
                "效果 / effects: Console.Write\n",
                "能力 / capabilities: Console.Write",
            ),
        })
    );
    for field in [
        "label",
        "kind",
        "sortText",
        "filterText",
        "insertTextFormat",
        "textEdit",
        "data",
    ] {
        assert_eq!(resolved["result"][field], item[field], "field {field}");
    }
}

#[test]
fn handles_and_markdown_are_deterministic_and_preserve_unicode_edits() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "\u{feff}module Main\r\n\r\n",
        "/// 返回 *原值* <安全>。\r\n",
        "let 帮助 value = value\r\n\r\n",
        "let main () = 帮助 1\r\n",
    );
    let (mut server, _) = ready(&["markdown", "plaintext"]);
    open(&mut server, uri, 1, source);

    let first = completion(&mut server, 2, uri, 5, 15);
    let second = completion(&mut server, 3, uri, 5, 15);
    assert_eq!(first["result"], second["result"]);
    let item = first_item(&first);
    assert_eq!(item["label"], "帮助");
    let original_edit = item["textEdit"].clone();

    let resolved = response(&mut server, 4, "completionItem/resolve", item);
    assert_eq!(resolved["result"]["textEdit"], original_edit);
    assert_eq!(resolved["result"]["documentation"]["kind"], "markdown");
    assert!(
        resolved["result"]["documentation"]["value"]
            .as_str()
            .expect("documentation")
            .starts_with("已检查的 Ling 符号 / checked Ling symbol\n\n```ling\n帮助:")
    );
    assert!(
        resolved["result"]["documentation"]["value"]
            .as_str()
            .expect("documentation")
            .contains("> 返回 \\*原值\\* \\<安全\\>。")
    );
}

#[test]
fn unsupported_candidate_resolves_without_fabricated_metadata() {
    let uri = "ling://workspace/src/Main.ling";
    let source = "module Main\n\nlet main () = 1\n";
    let (mut server, _) = ready(&["plaintext"]);
    open(&mut server, uri, 1, source);

    let completed = completion(&mut server, 2, uri, 2, 2);
    let item = first_item(&completed);
    assert_eq!(item["label"], "let");
    let resolved = response(&mut server, 3, "completionItem/resolve", item.clone());
    assert_eq!(resolved["result"], item);
    assert!(resolved["result"].get("detail").is_none());
    assert!(resolved["result"].get("documentation").is_none());
}

#[test]
fn local_binding_uses_binding_metadata_and_unknown_fields_are_not_reflected() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n\n",
        "let main value =\n",
        "    let local = value\n",
        "    local\n",
    );
    let (mut server, _) = ready(&["plaintext"]);
    open(&mut server, uri, 1, source);

    let completed = completion(&mut server, 2, uri, 4, 9);
    let mut item = first_item(&completed);
    assert_eq!(item["label"], "local");
    item["clientExtension"] = json!({"ignored": true});
    let resolved = response(&mut server, 3, "completionItem/resolve", item);
    assert!(
        resolved["result"]["detail"]
            .as_str()
            .expect("binding detail")
            .starts_with("local: ")
    );
    assert!(
        resolved["result"]["documentation"]["value"]
            .as_str()
            .expect("binding documentation")
            .contains("种类 / kind: binding")
    );
    assert!(resolved["result"].get("clientExtension").is_none());
}

#[test]
fn detached_documentation_is_not_associated_or_inferred() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n",
        "/// detached text\n",
        "\n",
        "let helper = 1\n\n",
        "let main () = helper\n",
    );
    let (mut server, _) = ready(&["plaintext"]);
    open(&mut server, uri, 1, source);

    let item = first_item(&completion(&mut server, 2, uri, 5, 18));
    let resolved = response(&mut server, 3, "completionItem/resolve", item);
    let documentation = resolved["result"]["documentation"]["value"]
        .as_str()
        .expect("checked documentation");
    assert!(!documentation.contains("detached text"));
    assert!(!documentation.contains("文档 / documentation:"));
}

#[test]
fn oversized_attached_documentation_fails_without_partial_metadata() {
    let uri = "ling://workspace/src/Main.ling";
    let source = format!(
        "module Main\n\n/// {}\nlet helper = 1\n\nlet main () = helper\n",
        "文".repeat(MAX_COMPLETION_DOCUMENTATION_BYTES)
    );
    let (mut server, _) = ready(&["plaintext"]);
    open(&mut server, uri, 1, &source);

    let item = first_item(&completion(&mut server, 2, uri, 5, 18));
    let resolved = response(&mut server, 3, "completionItem/resolve", item);
    assert_eq!(resolved["error"]["code"], -32803);
    assert_eq!(
        resolved["error"]["message"],
        "补全项解析不可用 / completion-item resolve unavailable"
    );
    assert!(resolved.get("result").is_none());
}

#[test]
fn malformed_modified_missing_and_stale_handles_fail_atomically() {
    let uri = "ling://workspace/src/Main.ling";
    let source = "module Main\n\nlet helper = 1\n\nlet main () = helper\n";
    let (mut server, _) = ready(&["plaintext"]);
    open(&mut server, uri, 1, source);
    let item = first_item(&completion(&mut server, 2, uri, 4, 18));
    let earlier_cursor_item = first_item(&completion(&mut server, 20, uri, 4, 15));
    assert_ne!(
        earlier_cursor_item["data"]["handle"],
        item["data"]["handle"]
    );
    let earlier_resolved = response(
        &mut server,
        21,
        "completionItem/resolve",
        earlier_cursor_item,
    );
    assert_eq!(earlier_resolved["result"]["label"], "helper");

    let mut modified = item.clone();
    modified["textEdit"]["newText"] = json!("other");
    let modified_response = response(&mut server, 3, "completionItem/resolve", modified);
    assert_eq!(modified_response["error"]["code"], -32602);

    let mut missing = item.clone();
    missing["data"]["handle"] = json!(format!(
        "ling.lsp.completion-resolve/0.1:blake3:{}",
        "0".repeat(64)
    ));
    let missing_response = response(&mut server, 4, "completionItem/resolve", missing);
    assert_eq!(missing_response["error"]["code"], -32803);

    let mut malformed = item.clone();
    malformed["data"]["extra"] = json!(true);
    let malformed_response = response(&mut server, 5, "completionItem/resolve", malformed);
    assert_eq!(malformed_response["error"]["code"], -32602);

    change(&mut server, uri, 2, source);
    let stale_response = response(&mut server, 6, "completionItem/resolve", item.clone());
    assert_eq!(stale_response["error"]["code"], -32803);
    assert_eq!(
        stale_response["error"]["message"],
        "补全项解析不可用 / completion-item resolve unavailable"
    );

    let fresh_item = first_item(&completion(&mut server, 7, uri, 4, 18));
    assert_ne!(fresh_item["data"]["handle"], item["data"]["handle"]);
    server
        .publish_disk_snapshot(
            "ling://workspace/src/Support.ling",
            "module Support\n\nlet value = 1\n",
        )
        .expect("workspace mutation");
    let workspace_stale = response(&mut server, 8, "completionItem/resolve", fresh_item);
    assert_eq!(workspace_stale["error"]["code"], -32803);
    assert_eq!(
        server.handle_json(&notification("completionItem/resolve", item)),
        HandleOutcome::NoResponse
    );
}

#[test]
fn malformed_capability_and_unnegotiated_resolve_are_rejected() {
    let mut malformed_server = LspServer::new();
    let malformed = response(
        &mut malformed_server,
        1,
        "initialize",
        json!({
            "capabilities": {
                "textDocument": {
                    "completion": {
                        "completionItem": {
                            "resolveSupport": {"properties": []},
                        },
                    },
                },
            },
        }),
    );
    assert_eq!(malformed["error"]["code"], -32602);

    let mut fallback_server = LspServer::new();
    let initialized = response(
        &mut fallback_server,
        1,
        "initialize",
        json!({"capabilities": {"textDocument": {"completion": {}}}}),
    );
    assert_eq!(
        initialized["result"]["capabilities"]["completionProvider"]["resolveProvider"],
        false
    );
    assert_eq!(
        fallback_server.handle_json(&notification("initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    let unavailable = response(&mut fallback_server, 2, "completionItem/resolve", json!({}));
    assert_eq!(unavailable["error"]["code"], -32803);
}
