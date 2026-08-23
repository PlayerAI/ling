use ling_lsp::{
    COMPLETION_PROTOCOL_VERSION, HandleOutcome, JSON_RPC_VERSION, LspServer, MAX_COMPLETION_ITEMS,
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

fn ready(encoding: &str) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let initialized = response(
        &mut server,
        1,
        "initialize",
        json!({
            "capabilities": {
                "general": {"positionEncodings": [encoding]},
                "textDocument": {"completion": {"dynamicRegistration": false}},
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

fn labels(response: &Value) -> Vec<&str> {
    response["result"]["items"]
        .as_array()
        .unwrap_or_else(|| panic!("completion items in {response}"))
        .iter()
        .map(|item| item["label"].as_str().expect("label"))
        .collect()
}

#[test]
fn initialize_advertises_exact_checked_completion_contract() {
    let (_, initialized) = ready("utf-16");
    assert_eq!(
        initialized["result"]["capabilities"]["completionProvider"],
        json!({"resolveProvider": false, "triggerCharacters": ["."]})
    );
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingCompletion"],
        json!({
            "contexts": ["expression", "member", "type", "pattern", "module", "keyword"],
            "maxItems": MAX_COMPLETION_ITEMS,
            "source": "checked",
            "version": COMPLETION_PROTOCOL_VERSION,
        })
    );
}

#[test]
fn expression_completion_is_prefix_filtered_checked_and_exactly_edited() {
    let uri = "ling://workspace/src/Main.ling";
    let source = "module Main\n\nlet helper = 1\n\nlet main () = helper\n";
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, 1, source);

    let result = completion(&mut server, 2, uri, 4, 18);
    assert_eq!(labels(&result), vec!["helper"]);
    assert_eq!(
        result["result"]["items"][0],
        json!({
            "filterText": "helper",
            "insertTextFormat": 1,
            "kind": 12,
            "label": "helper",
            "sortText": "000000",
            "textEdit": {
                "newText": "helper",
                "range": {
                    "end": {"character": 20, "line": 4},
                    "start": {"character": 14, "line": 4},
                },
            },
        })
    );
}

#[test]
fn module_member_import_and_type_contexts_use_checked_workspace_facts() {
    let support_uri = "ling://workspace/src/Support.ling";
    let main_uri = "ling://workspace/src/Main.ling";
    let support = "module Support\n\ntype Point = { x: Int; y: Int }\n\nlet answer = 42\n";
    let main = concat!(
        "module Main\n\n",
        "import Support as S\n\n",
        "let origin: S.Point = { x = 0; y = 0 }\n\n",
        "let main () = S.answer\n",
    );
    let (mut server, _) = ready("utf-16");
    server
        .publish_disk_snapshot(support_uri, support)
        .expect("support snapshot");
    open(&mut server, main_uri, 1, main);

    let imported_module = completion(&mut server, 2, main_uri, 2, 8);
    assert_eq!(labels(&imported_module), vec!["Support"]);
    assert_eq!(
        imported_module["result"]["items"][0]["textEdit"]["range"],
        json!({
            "end": {"character": 14, "line": 2},
            "start": {"character": 7, "line": 2},
        })
    );

    let type_member = completion(&mut server, 3, main_uri, 4, 14);
    assert_eq!(labels(&type_member), vec!["Point"]);
    let value_member = completion(&mut server, 4, main_uri, 6, 18);
    assert_eq!(labels(&value_member), vec!["answer"]);
}

#[test]
fn record_member_pattern_and_keyword_contexts_are_deterministic() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n\n",
        "type Point = { x: Int; y: Int }\n\n",
        "type State =\n",
        "    | Healthy\n",
        "    | Hurt\n\n",
        "let coordinate point = point.x\n\n",
        "let describe state =\n",
        "    match state with\n",
        "    | Healthy -> 1\n",
        "    | _ -> 2\n\n",
        "let main () = coordinate { x = 0; y = 1 }\n",
    );
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, 1, source);

    let fields = completion(&mut server, 2, uri, 8, 29);
    assert_eq!(labels(&fields), vec!["x", "y"]);
    let pattern = completion(&mut server, 3, uri, 12, 6);
    assert_eq!(labels(&pattern), vec!["Healthy", "Hurt", "None", "_"]);
    let keyword = completion(&mut server, 4, uri, 15, 2);
    assert_eq!(labels(&keyword), vec!["let"]);
    assert_eq!(keyword["result"]["items"][0]["kind"], 14);
    let wildcard = completion(&mut server, 5, uri, 13, 6);
    assert!(labels(&wildcard).contains(&"_"));
    assert_eq!(
        wildcard["result"]["items"]
            .as_array()
            .expect("wildcard items")
            .iter()
            .find(|item| item["label"] == "_")
            .expect("wildcard item")["kind"],
        14
    );
    assert!(
        fields["result"]["items"]
            .as_array()
            .expect("items")
            .iter()
            .all(|item| item.get("data").is_none() && item.get("documentation").is_none())
    );
}

#[test]
fn malformed_context_notifications_and_incomplete_sources_are_atomic() {
    let uri = "ling://workspace/src/Main.ling";
    let source = "module Main\n\nlet main () = 1\n";
    let (mut server, _) = ready("utf-8");
    open(&mut server, uri, 1, source);

    let malformed = response(
        &mut server,
        2,
        "textDocument/completion",
        json!({
            "context": {"triggerCharacter": ".", "triggerKind": 1},
            "position": {"character": 4, "line": 2},
            "textDocument": {"uri": uri},
        }),
    );
    assert_eq!(malformed["error"]["code"], -32602);
    assert_eq!(
        server.handle_json(&notification(
            "textDocument/completion",
            json!({
                "position": {"character": 4, "line": 2},
                "textDocument": {"uri": uri},
            }),
        )),
        HandleOutcome::NoResponse
    );

    open(
        &mut server,
        "untitled://ling/completion.ling",
        1,
        "module Main\n\nlet helper = 1\n\nlet main () = helper + \"x\"\n",
    );
    let incomplete = completion(&mut server, 3, "untitled://ling/completion.ling", 4, 15);
    assert_eq!(incomplete["error"]["code"], -32803, "{incomplete}");
}
