use ling_lsp::{
    HandleOutcome, JSON_RPC_VERSION, LspServer, MAX_RELOAD_SOURCES, MAX_RELOAD_TEXT_BYTES,
    WORKSPACE_PROTOCOL_VERSION, WorkspaceInput,
};
use serde_json::{Value, json};

fn message(id: Option<u64>, method: &str, params: Value) -> Vec<u8> {
    let mut value = json!({
        "jsonrpc": JSON_RPC_VERSION,
        "method": method,
        "params": params,
    });
    if let Some(id) = id {
        value["id"] = json!(id);
    }
    serde_json::to_vec(&value).expect("message is serializable")
}

fn response(outcome: HandleOutcome) -> Value {
    let HandleOutcome::Response(bytes) = outcome else {
        panic!("request must produce a response")
    };
    serde_json::from_slice(&bytes).expect("response is JSON")
}

fn ready_server() -> LspServer {
    let mut server = LspServer::new();
    let initialized =
        response(server.handle_json(&message(Some(1), "initialize", json!({"capabilities": {}}))));
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingWorkspaceReload"],
        json!({
            "inputLimit": 5,
            "sourceLimit": MAX_RELOAD_SOURCES,
            "totalByteLimit": MAX_RELOAD_TEXT_BYTES,
            "version": WORKSPACE_PROTOCOL_VERSION,
        })
    );
    assert_eq!(
        server.handle_json(&message(None, "initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    server
}

fn reload(server: &mut LspServer, base: &str, sources: Value, inputs: Value) -> Value {
    response(server.handle_json(&message(
        Some(9),
        "ling/workspace/reload",
        json!({
            "baseRevision": base,
            "inputs": inputs,
            "sources": sources,
        }),
    )))
}

#[test]
fn canonical_reload_publishes_exact_sources_inputs_and_noops() {
    let sources = json!([
        {"uri": "ling://workspace/z/Main.ling", "text": "\u{feff}零\r\n"},
        {"uri": "ling://dependency/math/src/Math.ling", "text": "let n = 1\n"},
        {"uri": "ling://workspace/a/Main.ling", "text": "let a = 1\n"},
    ]);
    let inputs = json!([
        {"name": "target", "text": "host"},
        {"name": "lock", "text": "lock-v1"},
        {"name": "manifest", "text": "manifest-v1"},
    ]);
    let mut left = ready_server();
    let mut right = ready_server();

    let accepted = reload(&mut left, "0", sources.clone(), inputs.clone());
    let reversed = reload(
        &mut right,
        "0",
        Value::Array(sources.as_array().unwrap().iter().rev().cloned().collect()),
        Value::Array(inputs.as_array().unwrap().iter().rev().cloned().collect()),
    );

    assert_eq!(accepted["result"]["changed"], true);
    assert_eq!(accepted["result"], reversed["result"]);
    let snapshot = left.capture_request_snapshot().expect("snapshot");
    assert_eq!(
        snapshot,
        right.capture_request_snapshot().expect("snapshot")
    );
    assert_eq!(
        snapshot
            .documents()
            .iter()
            .map(|document| document.uri())
            .collect::<Vec<_>>(),
        [
            "ling://dependency/math/src/Math.ling",
            "ling://workspace/a/Main.ling",
            "ling://workspace/z/Main.ling",
        ]
    );
    assert_eq!(
        snapshot
            .document("ling://workspace/z/Main.ling")
            .unwrap()
            .bytes(),
        "\u{feff}零\r\n".as_bytes()
    );
    assert_eq!(
        snapshot
            .inputs()
            .iter()
            .map(|input| input.kind())
            .collect::<Vec<_>>(),
        [
            WorkspaceInput::PackageManifest,
            WorkspaceInput::PackageLock,
            WorkspaceInput::Target,
        ]
    );
    assert_eq!(
        snapshot.input(WorkspaceInput::PackageLock).unwrap().bytes(),
        b"lock-v1"
    );

    let revision = accepted["result"]["revision"].as_str().unwrap();
    let no_op = reload(&mut left, revision, sources, inputs);
    assert_eq!(
        no_op["result"],
        json!({"changed": false, "revision": revision})
    );
}

#[test]
fn disk_reload_stays_below_overlay_and_removals_are_atomic() {
    let uri = "ling://workspace/src/Main.ling";
    let mut server = ready_server();
    let added = reload(
        &mut server,
        "0",
        json!([{"uri": uri, "text": "disk-1"}]),
        json!([]),
    );
    let revision = added["result"]["revision"].as_str().unwrap();
    assert_eq!(
        server.handle_json(&message(
            None,
            "textDocument/didOpen",
            json!({"textDocument": {"uri": uri, "version": 1, "text": "editor"}}),
        )),
        HandleOutcome::NoResponse
    );
    let open_revision = server
        .capture_request_snapshot()
        .expect("open snapshot")
        .revision()
        .get()
        .to_string();
    assert_ne!(open_revision, revision);

    let disk_changed = reload(
        &mut server,
        &open_revision,
        json!([{"uri": uri, "text": "disk-2"}]),
        json!([{"name": "config", "text": "config-v1"}]),
    );
    assert_eq!(disk_changed["result"]["changed"], true);
    assert_eq!(server.document(uri).unwrap().text(), "editor");

    let before_failed_delete = server.capture_request_snapshot().expect("snapshot");
    let rejected = reload(
        &mut server,
        disk_changed["result"]["revision"].as_str().unwrap(),
        json!([{"uri": uri, "text": null}]),
        json!([{"name": "manifest", "text": "must-not-publish"}]),
    );
    assert_eq!(rejected["error"]["code"], -32_602);
    assert_eq!(
        server.capture_request_snapshot().expect("atomic snapshot"),
        before_failed_delete
    );

    assert_eq!(
        server.handle_json(&message(
            None,
            "textDocument/didClose",
            json!({"textDocument": {"uri": uri}}),
        )),
        HandleOutcome::NoResponse
    );
    assert_eq!(server.document(uri).unwrap().text(), "disk-2");
    let closed_revision = server
        .capture_request_snapshot()
        .expect("closed snapshot")
        .revision()
        .get()
        .to_string();
    let removed = reload(
        &mut server,
        &closed_revision,
        json!([{"uri": uri, "text": null}]),
        json!([{"name": "config", "text": null}]),
    );
    assert_eq!(removed["result"]["changed"], true);
    assert!(server.document(uri).is_none());
    assert!(
        server
            .capture_request_snapshot()
            .expect("removed snapshot")
            .input(WorkspaceInput::Config)
            .is_none()
    );
}

#[test]
fn stale_malformed_duplicate_and_oversized_reloads_do_not_publish() {
    let mut server = ready_server();
    let before = server.capture_request_snapshot().expect("initial snapshot");
    let invalid = [
        json!({"baseRevision": "1", "inputs": [], "sources": [{"uri": "ling://workspace/a.ling", "text": "a"}]}),
        json!({"baseRevision": "01", "inputs": [], "sources": [{"uri": "ling://workspace/a.ling", "text": "a"}]}),
        json!({"baseRevision": "0", "inputs": [], "sources": []}),
        json!({"baseRevision": "0", "inputs": [], "sources": [
            {"uri": "ling://workspace/a.ling", "text": "a"},
            {"uri": "ling://workspace/a.ling", "text": "b"}
        ]}),
        json!({"baseRevision": "0", "inputs": [], "sources": [{"uri": "untitled://ling/a.ling", "text": "a"}]}),
        json!({"baseRevision": "0", "inputs": [{"name": "manifest", "text": "x".repeat(MAX_RELOAD_TEXT_BYTES)}, {"name": "lock", "text": "y"}], "sources": []}),
    ];
    for (index, params) in invalid.into_iter().enumerate() {
        let rejected = response(server.handle_json(&message(
            Some(u64::try_from(index).unwrap() + 20),
            "ling/workspace/reload",
            params,
        )));
        assert!(matches!(
            rejected["error"]["code"].as_i64(),
            Some(-32_007 | -32_602)
        ));
        assert_eq!(
            server
                .capture_request_snapshot()
                .expect("unchanged snapshot"),
            before
        );
    }

    assert_eq!(
        server.handle_json(&message(
            None,
            "ling/workspace/reload",
            json!({
                "baseRevision": "0",
                "inputs": [{"name": "manifest", "text": "ignored"}],
                "sources": [],
            }),
        )),
        HandleOutcome::NoResponse
    );
    assert_eq!(server.capture_request_snapshot().unwrap(), before);
}
