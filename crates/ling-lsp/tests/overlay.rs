use ling_lsp::{
    HandleOutcome, JSON_RPC_VERSION, LifecycleState, LspServer, OVERLAY_PROTOCOL_VERSION,
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

fn ready_server() -> LspServer {
    let mut server = LspServer::new();
    let initialize = server.handle_json(&request(1, "initialize", json!({})));
    assert!(matches!(initialize, HandleOutcome::Response(_)));
    assert_eq!(
        server.handle_json(&notification("initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    assert_eq!(server.state(), LifecycleState::Ready);
    server
}

fn open_params(uri: &str, version: i64, text: &str) -> Value {
    json!({
        "textDocument": {"uri": uri, "version": version, "text": text}
    })
}

#[test]
fn workspace_overlay_is_full_sync_and_disk_changes_are_revealed_on_close() {
    assert_eq!(OVERLAY_PROTOCOL_VERSION, "ling.lsp.overlay/0.2");
    let mut server = ready_server();
    let uri = "ling://workspace/src/Main.ling";
    let opened = server.handle_json(&request(
        2,
        "textDocument/didOpen",
        open_params(uri, 1, "editor-1"),
    ));
    assert!(matches!(opened, HandleOutcome::Response(_)));
    assert_eq!(server.document(uri).unwrap().text(), "editor-1");
    assert!(server.document(uri).unwrap().is_open());

    let changed = server.handle_json(&notification(
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": uri, "version": 2},
            "contentChanges": [{"text": "editor-2"}]
        }),
    ));
    assert_eq!(changed, HandleOutcome::NoResponse);
    assert_eq!(server.document(uri).unwrap().text(), "editor-2");
    assert_eq!(server.document(uri).unwrap().version(), 2);

    server
        .publish_disk_snapshot(uri, "disk-2")
        .expect("disk update is accepted");
    assert_eq!(server.document(uri).unwrap().text(), "editor-2");

    let closed = server.handle_json(&request(
        3,
        "textDocument/didClose",
        json!({"textDocument": {"uri": uri}}),
    ));
    assert!(matches!(closed, HandleOutcome::Response(_)));
    let view = server.document(uri).unwrap();
    assert!(!view.is_open());
    assert_eq!(view.text(), "disk-2");
    assert_eq!(server.documents().len(), 1);
}

#[test]
fn stale_and_read_only_changes_do_not_mutate_the_overlay() {
    let mut server = ready_server();
    let workspace = "ling://workspace/src/Main.ling";
    let _ = server.handle_json(&request(
        2,
        "textDocument/didOpen",
        open_params(workspace, 4, "original"),
    ));
    let stale = server.handle_json(&request(
        3,
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": workspace, "version": 4},
            "contentChanges": [{"text": "must-not-apply"}]
        }),
    ));
    let HandleOutcome::Response(stale) = stale else {
        panic!("stale request must produce a response")
    };
    let stale: Value = serde_json::from_slice(&stale).expect("stale response is JSON");
    assert_eq!(stale["error"]["code"], -32_004);
    assert_eq!(server.document(workspace).unwrap().text(), "original");

    let dependency = "ling://dependency/core/src/Prelude.ling";
    let _ = server.handle_json(&request(
        4,
        "textDocument/didOpen",
        open_params(dependency, 1, "dependency"),
    ));
    let read_only = server.handle_json(&request(
        5,
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": dependency, "version": 2},
            "contentChanges": [{"text": "must-not-apply"}]
        }),
    ));
    let HandleOutcome::Response(read_only) = read_only else {
        panic!("read-only request must produce a response")
    };
    let read_only: Value = serde_json::from_slice(&read_only).expect("read-only response is JSON");
    assert_eq!(read_only["error"]["code"], -32_005);
    assert_eq!(server.document(dependency).unwrap().text(), "dependency");
    assert_eq!(server.document(dependency).unwrap().version(), 1);
}

#[test]
fn temporary_documents_are_removed_on_close_and_versions_remain_monotonic() {
    let mut server = ready_server();
    let uri = "untitled://ling/Buffer.ling";
    let _ = server.handle_json(&request(
        2,
        "textDocument/didOpen",
        open_params(uri, 7, "temporary"),
    ));
    let _ = server.handle_json(&request(
        3,
        "textDocument/didClose",
        json!({"textDocument": {"uri": uri}}),
    ));
    assert!(server.document(uri).is_none());

    let reopened = server.handle_json(&request(
        4,
        "textDocument/didOpen",
        open_params(uri, 8, "temporary-again"),
    ));
    assert!(matches!(reopened, HandleOutcome::Response(_)));
    let stale = server.handle_json(&request(
        5,
        "textDocument/didOpen",
        open_params(uri, 8, "duplicate"),
    ));
    let HandleOutcome::Response(stale) = stale else {
        panic!("duplicate open must produce a response")
    };
    let stale: Value = serde_json::from_slice(&stale).expect("duplicate response is JSON");
    assert_eq!(stale["error"]["code"], -32_004);
}

#[test]
fn invalid_uri_and_range_changes_are_rejected_before_vfs_mutation() {
    let mut server = ready_server();
    let invalid = server.handle_json(&request(
        2,
        "textDocument/didOpen",
        open_params("file:///host/Main.ling", 1, "text"),
    ));
    let HandleOutcome::Response(invalid) = invalid else {
        panic!("invalid URI must produce a response")
    };
    let invalid: Value = serde_json::from_slice(&invalid).expect("invalid URI response is JSON");
    assert_eq!(invalid["error"]["code"], -32_006);

    let uri = "ling://workspace/src/Main.ling";
    let _ = server.handle_json(&request(
        3,
        "textDocument/didOpen",
        open_params(uri, 1, "text"),
    ));
    let ranged = server.handle_json(&request(
        4,
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": uri, "version": 2},
            "contentChanges": [{"range": {}, "text": "bad"}]
        }),
    ));
    let HandleOutcome::Response(ranged) = ranged else {
        panic!("range change must produce a response")
    };
    let ranged: Value = serde_json::from_slice(&ranged).expect("range response is JSON");
    assert_eq!(ranged["error"]["code"], -32_602);
    assert_eq!(server.document(uri).unwrap().text(), "text");
    assert_eq!(server.document(uri).unwrap().version(), 1);
}
