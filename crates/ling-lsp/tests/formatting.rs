// experimental: RFC-0026 closes only the synchronous formatting slice; general
// snapshot/transaction behavior remains GAP-LSP-TRANSACTION-PROTOCOL-001.
use ling_lsp::{
    FORMATTING_PROTOCOL_VERSION, HandleOutcome, JSON_RPC_VERSION, LifecycleState, LspServer,
};
use serde_json::{Value, json};

fn request(id: u64, method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": JSON_RPC_VERSION,
        "id": id,
        "method": method,
        "params": params,
    }))
    .expect("request is serializable")
}

fn notification(method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": JSON_RPC_VERSION,
        "method": method,
        "params": params,
    }))
    .expect("notification is serializable")
}

fn response(outcome: HandleOutcome) -> Value {
    let HandleOutcome::Response(bytes) = outcome else {
        panic!("request must produce a response")
    };
    serde_json::from_slice(&bytes).expect("response is JSON")
}

fn ready_server(encoding: &str) -> LspServer {
    let mut server = LspServer::new();
    let initialized = response(server.handle_json(&request(
        1,
        "initialize",
        json!({"capabilities": {"general": {"positionEncodings": [encoding]}}}),
    )));
    assert_eq!(
        initialized["result"]["capabilities"]["documentFormattingProvider"],
        true
    );
    assert_eq!(
        server.handle_json(&notification("initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    assert_eq!(server.state(), LifecycleState::Ready);
    server
}

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    let outcome = server.handle_json(&notification(
        "textDocument/didOpen",
        json!({
            "textDocument": {"uri": uri, "version": version, "text": text}
        }),
    ));
    assert_eq!(outcome, HandleOutcome::NoResponse);
}

fn formatting_params(uri: &str) -> Value {
    json!({
        "textDocument": {"uri": uri},
        "options": {"tabSize": 4, "insertSpaces": true}
    })
}

#[test]
fn formatting_projects_one_whole_document_edit_for_each_encoding() {
    assert_eq!(FORMATTING_PROTOCOL_VERSION, "ling.lsp.formatting/0.1");
    let expectations = [("utf-8", 17), ("utf-16", 11), ("utf-32", 10)];
    for (index, (encoding, end_character)) in expectations.into_iter().enumerate() {
        let mut server = ready_server(encoding);
        let uri = format!("ling://workspace/src/Main{index}.ling");
        let original = "let 中文=\"😀\"";
        open(&mut server, &uri, 1, original);

        let formatted = response(server.handle_json(&request(
            2,
            "textDocument/formatting",
            formatting_params(&uri),
        )));
        assert_eq!(
            formatted["result"],
            json!([{
                "newText": "let 中文 = \"😀\"\n",
                "range": {
                    "start": {"line": 0, "character": 0},
                    "end": {"line": 0, "character": end_character},
                }
            }])
        );
        assert_eq!(server.document(&uri).unwrap().text(), original);
        assert_eq!(server.document(&uri).unwrap().version(), 1);

        let repeated = response(server.handle_json(&request(
            3,
            "textDocument/formatting",
            formatting_params(&uri),
        )));
        assert_eq!(repeated["result"], formatted["result"]);
    }
}

#[test]
fn formatting_preserves_bom_and_uses_the_latest_immutable_overlay() {
    let mut server = ready_server("utf-16");
    let uri = "ling://workspace/src/Main.ling";
    open(&mut server, uri, 4, "\u{feff}let value=1\r\n");
    assert_eq!(
        server.handle_json(&notification(
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": 5},
                "contentChanges": [{"text": "\u{feff}let value=2\r\n"}]
            }),
        )),
        HandleOutcome::NoResponse
    );

    let formatted = response(server.handle_json(&request(
        2,
        "textDocument/formatting",
        formatting_params(uri),
    )));
    assert_eq!(formatted["result"][0]["newText"], "let value = 2\n");
    assert_eq!(
        formatted["result"][0]["range"],
        json!({
            "start": {"line": 0, "character": 0},
            "end": {"line": 1, "character": 0}
        })
    );
    assert_eq!(server.document(uri).unwrap().version(), 5);
    assert!(server.document(uri).unwrap().text().starts_with('\u{feff}'));
}

#[test]
fn unchanged_or_invalid_sources_return_no_edits() {
    let mut server = ready_server("utf-16");
    for (index, text) in ["let value = 1\n", "let value=\"unterminated\n"]
        .into_iter()
        .enumerate()
    {
        let uri = format!("untitled://ling/Buffer{index}.ling");
        open(&mut server, &uri, 1, text);
        let formatted = response(server.handle_json(&request(
            10 + index as u64,
            "textDocument/formatting",
            formatting_params(&uri),
        )));
        assert_eq!(formatted["result"], json!([]));
        assert_eq!(server.document(&uri).unwrap().text(), text);
    }
}

#[test]
fn formatting_rejects_invalid_state_read_only_and_options_without_mutation() {
    let mut server = ready_server("utf-16");
    let workspace = "ling://workspace/src/Main.ling";
    let dependency = "ling://dependency/core/src/Prelude.ling";
    open(&mut server, workspace, 1, "let value=1\n");
    open(&mut server, dependency, 1, "let value=1\n");

    let read_only = response(server.handle_json(&request(
        2,
        "textDocument/formatting",
        formatting_params(dependency),
    )));
    assert_eq!(read_only["error"]["code"], -32_005);

    let missing = response(server.handle_json(&request(
        3,
        "textDocument/formatting",
        formatting_params("ling://workspace/src/Missing.ling"),
    )));
    assert_eq!(missing["error"]["code"], -32_602);

    let invalid_options = response(server.handle_json(&request(
        4,
        "textDocument/formatting",
        json!({
            "textDocument": {"uri": workspace},
            "options": {"tabSize": 2, "insertSpaces": true}
        }),
    )));
    assert_eq!(invalid_options["error"]["code"], -32_602);
    assert_eq!(server.document(workspace).unwrap().text(), "let value=1\n");

    let extra_option = response(server.handle_json(&request(
        5,
        "textDocument/formatting",
        json!({
            "textDocument": {"uri": workspace},
            "options": {"tabSize": 4, "insertSpaces": true, "trimFinalNewlines": true}
        }),
    )));
    assert_eq!(extra_option["error"]["code"], -32_602);

    let invalid_uri = response(server.handle_json(&request(
        6,
        "textDocument/formatting",
        formatting_params("file:///tmp/Main.ling"),
    )));
    assert_eq!(invalid_uri["error"]["code"], -32_006);

    assert_eq!(
        server.handle_json(&notification(
            "textDocument/formatting",
            formatting_params(workspace),
        )),
        HandleOutcome::NoResponse
    );
    assert_eq!(server.document(workspace).unwrap().text(), "let value=1\n");

    assert_eq!(
        server.handle_json(&notification(
            "textDocument/didClose",
            json!({"textDocument": {"uri": workspace}}),
        )),
        HandleOutcome::NoResponse
    );
    let closed = response(server.handle_json(&request(
        7,
        "textDocument/formatting",
        formatting_params(workspace),
    )));
    assert_eq!(closed["error"]["code"], -32_602);
}

#[test]
fn formatting_obeys_preinitialize_and_shutdown_lifecycle_without_work() {
    let uri = "ling://workspace/src/Main.ling";
    let mut uninitialized = LspServer::new();
    let preinitialize = response(uninitialized.handle_json(&request(
        1,
        "textDocument/formatting",
        formatting_params(uri),
    )));
    assert_eq!(preinitialize["error"]["code"], -32_002);
    assert!(uninitialized.document(uri).is_none());

    let mut server = ready_server("utf-16");
    open(&mut server, uri, 7, "let value=1\n");
    let shutdown = response(server.handle_json(&request(2, "shutdown", Value::Null)));
    assert_eq!(shutdown["result"], Value::Null);
    assert_eq!(server.state(), LifecycleState::ShutdownRequested);

    let post_shutdown = response(server.handle_json(&request(
        3,
        "textDocument/formatting",
        formatting_params(uri),
    )));
    assert_eq!(post_shutdown["error"]["code"], -32_003);
    let document = server
        .document(uri)
        .expect("open overlay remains observable");
    assert_eq!(document.text(), "let value=1\n");
    assert_eq!(document.version(), 7);
}
