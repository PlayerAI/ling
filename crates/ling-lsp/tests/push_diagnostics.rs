use ling_lsp::{
    DIAGNOSTIC_PROTOCOL_VERSION, DiagnosticAnalysisError, HandleOutcome, LspServer,
    PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION, PositionEncoding, run_stdio,
};
use serde_json::{Value, json};

fn message(method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    }))
    .unwrap()
}

fn request(id: u64, method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params,
    }))
    .unwrap()
}

fn ready() -> LspServer {
    ready_with_options(json!({}))
}

fn ready_with_options(initialization_options: Value) -> LspServer {
    let mut server = LspServer::new();
    assert!(matches!(
        server.handle_json(&request(
            1,
            "initialize",
            json!({
                "capabilities": {"general": {"positionEncodings": ["utf-16"]}},
                "initializationOptions": initialization_options,
            }),
        )),
        HandleOutcome::Response(_)
    ));
    assert_eq!(server.position_encoding(), PositionEncoding::Utf16);
    assert_eq!(
        server.handle_json(&message("initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    server
}

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&message(
            "textDocument/didOpen",
            json!({"textDocument": {"languageId": "ling", "text": text, "uri": uri, "version": version}}),
        )),
        HandleOutcome::NoResponse
    );
}

fn change(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&message(
            "textDocument/didChange",
            json!({
                "contentChanges": [{"text": text}],
                "textDocument": {"uri": uri, "version": version}
            }),
        )),
        HandleOutcome::NoResponse
    );
}

fn close(server: &mut LspServer, uri: &str) {
    assert_eq!(
        server.handle_json(&message(
            "textDocument/didClose",
            json!({"textDocument": {"uri": uri}}),
        )),
        HandleOutcome::NoResponse
    );
}

fn notifications(server: &mut LspServer) -> Vec<Value> {
    server
        .take_notifications()
        .iter()
        .map(|bytes| serde_json::from_slice(bytes).expect("notification is JSON"))
        .collect()
}

fn frame(body: &[u8]) -> Vec<u8> {
    let mut frame = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
    frame.extend_from_slice(body);
    frame
}

fn decode_frames(mut bytes: &[u8]) -> Vec<Value> {
    let mut values = Vec::new();
    while !bytes.is_empty() {
        let header_end = bytes
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .expect("complete frame header");
        let header = std::str::from_utf8(&bytes[..header_end]).unwrap();
        let length = header
            .strip_prefix("Content-Length: ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let body_start = header_end + 4;
        let body_end = body_start + length;
        values.push(serde_json::from_slice(&bytes[body_start..body_end]).unwrap());
        bytes = &bytes[body_end..];
    }
    values
}

#[test]
fn initialize_advertises_exact_push_and_adapter_markers() {
    let mut server = LspServer::new();
    let HandleOutcome::Response(bytes) = server.handle_json(&request(1, "initialize", json!({})))
    else {
        panic!("initialize responds")
    };
    let value: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        value["result"]["capabilities"]["experimental"]["lingPublishDiagnostics"],
        json!({
            "adapterVersion": DIAGNOSTIC_PROTOCOL_VERSION,
            "debounce": "message-boundary",
            "version": PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION,
        })
    );
    assert_eq!(DIAGNOSTIC_PROTOCOL_VERSION, "ling.lsp.diagnostic/0.2");
}

#[test]
fn publishes_registered_diagnostics_then_versioned_empty_replacement() {
    let uri = "ling://workspace/src/Main.ling";
    let mut server = ready();
    open(
        &mut server,
        uri,
        1,
        "module Main\n\nlet value: Int = \"text\"\n",
    );
    assert!(server.diagnostics_pending());
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let first = notifications(&mut server);
    assert_eq!(first.len(), 1);
    assert_eq!(first[0]["method"], "textDocument/publishDiagnostics");
    assert_eq!(first[0]["params"]["uri"], uri);
    assert_eq!(first[0]["params"]["version"], 1);
    assert_eq!(first[0]["params"]["diagnostics"][0]["code"], "L-TYPE-0001");

    change(&mut server, uri, 2, "module Main\n\nlet value: Int = 1\n");
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let replacement = notifications(&mut server);
    assert_eq!(replacement[0]["params"]["version"], 2);
    assert_eq!(replacement[0]["params"]["diagnostics"], json!([]));
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 0);
    assert!(notifications(&mut server).is_empty());
}

#[test]
fn multiple_mutations_coalesce_to_the_newest_complete_state() {
    let uri = "ling://workspace/src/Main.ling";
    let mut server = ready();
    open(&mut server, uri, 1, "module Main\n\nlet value = missing\n");
    change(&mut server, uri, 2, "module Main\n\nlet value = @\n");
    change(
        &mut server,
        uri,
        3,
        "module Main\n\nlet value: Int = \"x\"\n",
    );

    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let output = notifications(&mut server);
    assert_eq!(output.len(), 1);
    assert_eq!(output[0]["params"]["version"], 3);
    assert_eq!(output[0]["params"]["diagnostics"][0]["code"], "L-TYPE-0001");
}

#[test]
fn stale_completion_emits_nothing_and_preserves_newer_pending_work() {
    let uri = "ling://workspace/src/Main.ling";
    let mut server = ready();
    open(&mut server, uri, 1, "module Main\n\nlet value = missing\n");
    let ticket = server
        .begin_diagnostic_analysis()
        .unwrap()
        .expect("analysis pending");
    let result = ticket
        .compile()
        .expect("old snapshot compiles to diagnostics");
    change(&mut server, uri, 2, "module Main\n\nlet value = @\n");

    assert_eq!(
        server.complete_diagnostic_analysis(result),
        Err(DiagnosticAnalysisError::Stale)
    );
    assert!(notifications(&mut server).is_empty());
    assert!(server.diagnostics_pending());
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let current = notifications(&mut server);
    assert_eq!(current[0]["params"]["version"], 2);
    assert_eq!(current[0]["params"]["diagnostics"][0]["code"], "L-LEX-0004");
}

#[test]
fn temporary_documents_are_syntax_only_and_clear_on_close() {
    let workspace = "ling://workspace/src/Main.ling";
    let temporary = "untitled://ling/scratch/Main.ling";
    let mut server = ready();
    open(
        &mut server,
        workspace,
        1,
        "module Main\n\nlet value: Int = \"text\"\n",
    );
    open(
        &mut server,
        temporary,
        1,
        "module Main\n\nlet scratch = 1\n",
    );
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 2);
    let first = notifications(&mut server);
    assert_eq!(first.len(), 2);
    assert_eq!(first[0]["params"]["uri"], workspace);
    assert_eq!(first[0]["params"]["diagnostics"][0]["code"], "L-TYPE-0001");
    assert_eq!(first[1]["params"]["uri"], temporary);
    assert_eq!(first[1]["params"]["diagnostics"], json!([]));

    close(&mut server, temporary);
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let cleared = notifications(&mut server);
    assert_eq!(cleared[0]["params"]["uri"], temporary);
    assert_eq!(cleared[0]["params"]["version"], 1);
    assert_eq!(cleared[0]["params"]["diagnostics"], json!([]));
}

#[test]
fn close_republishes_visible_disk_diagnostics_without_a_version() {
    let uri = "ling://workspace/src/Main.ling";
    let mut server = ready();
    server
        .publish_disk_snapshot(uri, "module Main\n\nlet value = missing\n")
        .unwrap();
    open(&mut server, uri, 1, "module Main\n\nlet value = 1\n");
    server.flush_pending_diagnostics().unwrap();
    let opened = notifications(&mut server);
    assert_eq!(opened[0]["params"]["version"], 1);
    assert_eq!(opened[0]["params"]["diagnostics"], json!([]));

    close(&mut server, uri);
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let closed = notifications(&mut server);
    assert!(closed[0]["params"].get("version").is_none());
    assert_eq!(closed[0]["params"]["diagnostics"][0]["code"], "L-NAME-0001");
}

#[test]
fn workspace_reload_noop_is_suppressed_and_removal_clears() {
    let uri = "ling://workspace/src/Main.ling";
    let text = "module Main\n\nlet value = missing\n";
    let mut server = ready();
    let revision = server.capture_request_snapshot().unwrap().revision().get();
    assert!(matches!(
        server.handle_json(&request(
            2,
            "ling/workspace/reload",
            json!({"baseRevision": revision.to_string(), "inputs": [], "sources": [{"text": text, "uri": uri}]})
        )),
        HandleOutcome::Response(_)
    ));
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    assert_eq!(
        notifications(&mut server)[0]["params"]["diagnostics"][0]["code"],
        "L-NAME-0001"
    );

    let revision = server.capture_request_snapshot().unwrap().revision().get();
    let HandleOutcome::Response(noop) = server.handle_json(&request(
        3,
        "ling/workspace/reload",
        json!({"baseRevision": revision.to_string(), "inputs": [], "sources": [{"text": text, "uri": uri}]}),
    )) else {
        panic!("reload responds")
    };
    assert_eq!(
        serde_json::from_slice::<Value>(&noop).unwrap()["result"]["changed"],
        false
    );
    assert!(!server.diagnostics_pending());

    let revision = server.capture_request_snapshot().unwrap().revision().get();
    let _ = server.handle_json(&request(
        4,
        "ling/workspace/reload",
        json!({"baseRevision": revision.to_string(), "inputs": [], "sources": [{"text": null, "uri": uri}]}),
    ));
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let clear = notifications(&mut server);
    assert_eq!(clear[0]["params"]["uri"], uri);
    assert_eq!(clear[0]["params"]["diagnostics"], json!([]));
    assert!(clear[0]["params"].get("version").is_none());
}

#[test]
fn disk_snapshots_publish_in_uri_order_and_identical_bytes_are_a_noop() {
    let first_uri = "ling://workspace/a/Main.ling";
    let second_uri = "ling://workspace/z/Main.ling";
    let mut server = ready();
    server
        .publish_disk_snapshot(second_uri, "module Z\n\nlet z = 1\n")
        .unwrap();
    server
        .publish_disk_snapshot(first_uri, "module A\n\nlet a = 1\n")
        .unwrap();

    assert_eq!(server.flush_pending_diagnostics().unwrap(), 2);
    let initial = notifications(&mut server);
    assert_eq!(initial[0]["params"]["uri"], first_uri);
    assert_eq!(initial[1]["params"]["uri"], second_uri);
    assert!(
        initial
            .iter()
            .all(|notification| notification["params"].get("version").is_none())
    );

    server
        .publish_disk_snapshot(first_uri, "module A\n\nlet a = 1\n")
        .unwrap();
    assert!(!server.diagnostics_pending());

    server
        .publish_disk_snapshot(first_uri, "module A\n\nlet a = missing\n")
        .unwrap();
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let changed = notifications(&mut server);
    assert_eq!(changed[0]["params"]["uri"], first_uri);
    assert_eq!(
        changed[0]["params"]["diagnostics"][0]["code"],
        "L-NAME-0001"
    );
}

#[test]
fn workspace_input_change_makes_an_older_completion_stale() {
    let uri = "ling://workspace/src/Main.ling";
    let mut server = ready();
    open(&mut server, uri, 1, "module Main\n\nlet value = missing\n");
    let result = server
        .begin_diagnostic_analysis()
        .unwrap()
        .expect("analysis pending")
        .compile()
        .expect("snapshot compiles to diagnostics");
    let revision = server.capture_request_snapshot().unwrap().revision().get();
    assert!(matches!(
        server.handle_json(&request(
            2,
            "ling/workspace/reload",
            json!({
                "baseRevision": revision.to_string(),
                "inputs": [{"name": "config", "text": "profile = seed"}],
                "sources": []
            }),
        )),
        HandleOutcome::Response(_)
    ));

    assert_eq!(
        server.complete_diagnostic_analysis(result),
        Err(DiagnosticAnalysisError::Stale)
    );
    assert!(server.diagnostics_pending());
    assert!(notifications(&mut server).is_empty());
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
}

#[test]
fn stdio_writes_request_response_before_caused_publication() {
    let initialize = request(1, "initialize", json!({}));
    let initialized = message("initialized", json!({}));
    let reload = request(
        2,
        "ling/workspace/reload",
        json!({
            "baseRevision": "0",
            "inputs": [],
            "sources": [{
                "text": "module Main\n\nlet value = missing\n",
                "uri": "ling://workspace/src/Main.ling"
            }]
        }),
    );
    let shutdown = request(3, "shutdown", Value::Null);
    let exit = message("exit", json!({}));
    let input = [initialize, initialized, reload, shutdown, exit]
        .into_iter()
        .flat_map(|body| frame(&body))
        .collect::<Vec<_>>();
    let mut output = Vec::new();

    let result = run_stdio(input.as_slice(), &mut output).expect("stdio run succeeds");
    assert_eq!(result.exit_code(), 0);
    let frames = decode_frames(&output);
    assert_eq!(frames.len(), 4);
    assert_eq!(frames[0]["id"], 1);
    assert_eq!(frames[1]["id"], 2);
    assert_eq!(frames[2]["method"], "textDocument/publishDiagnostics");
    assert_eq!(frames[2]["params"]["diagnostics"][0]["code"], "L-NAME-0001");
    assert_eq!(frames[3]["id"], 3);
}

#[test]
fn oversized_notification_is_rejected_without_mutating_publication_state() {
    let uri = "ling://workspace/src/Main.ling";
    let mut server = ready_with_options(json!({"lingDiagnosticControl": {
        "maxPerDocument": 4_096,
        "maxPerWorkspace": 65_536,
    }}));
    let text = format!("module Main\n\nlet value = {}\n", "@".repeat(20_000));
    open(&mut server, uri, 1, &text);

    let error = server
        .flush_pending_diagnostics()
        .expect_err("the encoded notification exceeds the transport frame limit");
    assert!(matches!(
        error,
        DiagnosticAnalysisError::NotificationTooLarge { length }
            if length > ling_lsp::MAX_FRAME_BYTES
    ));
    assert!(server.diagnostics_pending());
    assert!(notifications(&mut server).is_empty());

    change(&mut server, uri, 2, "module Main\n\nlet value = 1\n");
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let published = notifications(&mut server);
    assert_eq!(published[0]["params"]["version"], 2);
    assert_eq!(published[0]["params"]["diagnostics"], json!([]));
}
