use std::io::Cursor;

use ling_lsp::{
    HandleOutcome, JSON_RPC_VERSION, LifecycleState, MAX_FRAME_BYTES, PROTOCOL_VERSION,
    PositionEncoding, TransportError, run_stdio,
};
use serde_json::{Value, json};

fn framed(body: &[u8]) -> Vec<u8> {
    let mut frame = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
    frame.extend_from_slice(body);
    frame
}

fn request(id: u64, method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": JSON_RPC_VERSION,
        "id": id,
        "method": method,
        "params": params,
    }))
    .expect("test request is serializable")
}

fn notification(method: &str) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": JSON_RPC_VERSION,
        "method": method,
    }))
    .expect("test notification is serializable")
}

fn response_bodies(mut bytes: &[u8]) -> Vec<Value> {
    let mut values = Vec::new();
    while !bytes.is_empty() {
        let marker = b"\r\n\r\n";
        let header_end = bytes
            .windows(marker.len())
            .position(|window| window == marker)
            .expect("response has a complete header");
        let header = &bytes[..header_end];
        let length = header
            .strip_prefix(b"Content-Length: ")
            .and_then(|value| std::str::from_utf8(value).ok())
            .and_then(|value| value.parse::<usize>().ok())
            .expect("response has a numeric content length");
        let body_start = header_end + marker.len();
        let body_end = body_start + length;
        values.push(serde_json::from_slice(&bytes[body_start..body_end]).expect("response JSON"));
        bytes = &bytes[body_end..];
    }
    values
}

#[test]
fn framed_lifecycle_transcript_is_deterministic_and_stdout_pure() {
    let mut input = Vec::new();
    input.extend_from_slice(&framed(&request(
        1,
        "initialize",
        json!({
            "capabilities": {"general": {"positionEncodings": ["utf-8", "utf-16"]}},
            "workspaceFolders": [{"uri": "file:///ling", "name": "Ling"}],
        }),
    )));
    input.extend_from_slice(&framed(&notification("initialized")));
    input.extend_from_slice(&framed(&request(2, "shutdown", Value::Null)));
    input.extend_from_slice(&framed(&notification("exit")));

    let mut output = Vec::new();
    let result = run_stdio(Cursor::new(input), &mut output).expect("lifecycle transcript");
    assert_eq!(result.exit_code(), 0);
    assert_eq!(result.state(), LifecycleState::Exited);
    let responses = response_bodies(&output);
    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0]["id"], 1);
    assert_eq!(
        responses[0]["result"]["capabilities"]["positionEncoding"],
        "utf-8"
    );
    assert_eq!(responses[0]["result"]["serverInfo"]["name"], "ling");
    assert_eq!(
        responses[1],
        json!({"id": 2, "jsonrpc": JSON_RPC_VERSION, "result": null})
    );
}

#[test]
fn fallback_and_unicode_workspace_metadata_are_preserved() {
    let mut input = Vec::new();
    input.extend_from_slice(&framed(&request(
        1,
        "initialize",
        json!({
            "capabilities": {"general": {"positionEncodings": ["unknown"]}},
            "workspaceFolders": [{"uri": "file:///凌", "name": "编程"}],
        }),
    )));
    input.extend_from_slice(&framed(&notification("exit")));
    let mut output = Vec::new();
    let result = run_stdio(Cursor::new(input), &mut output).expect("fallback transcript");
    assert_eq!(result.exit_code(), 1);
    let response = response_bodies(&output).remove(0);
    assert_eq!(
        response["result"]["capabilities"]["positionEncoding"],
        "utf-16"
    );
}

#[test]
fn malformed_json_and_preinitialize_request_have_protocol_errors() {
    let mut input = Vec::new();
    input.extend_from_slice(&framed(&request(1, "shutdown", Value::Null)));
    input.extend_from_slice(&framed(b"{"));
    input.extend_from_slice(&framed(&notification("exit")));
    let mut output = Vec::new();
    let result = run_stdio(Cursor::new(input), &mut output).expect("error transcript");
    assert_eq!(result.exit_code(), 1);
    let responses = response_bodies(&output);
    assert_eq!(responses[0]["error"]["code"], -32002);
    assert_eq!(responses[1]["error"]["code"], -32700);
}

#[test]
fn malformed_transport_is_rejected_without_a_guessed_response() {
    let error = run_stdio(Cursor::new(b"Content-Length: 1\n\n{}".to_vec()), Vec::new())
        .expect_err("bare LF must be rejected");
    assert!(matches!(error, TransportError::InvalidHeader));

    let too_large = format!("Content-Length: {}\r\n\r\n", MAX_FRAME_BYTES + 1);
    let error = run_stdio(Cursor::new(too_large.into_bytes()), Vec::new())
        .expect_err("oversized frame must be rejected");
    assert!(matches!(error, TransportError::FrameTooLarge { .. }));
}

#[test]
fn public_position_encoding_type_is_the_source_projection_type() {
    assert_eq!(PositionEncoding::Utf8.wire_name(), "utf-8");
    assert_eq!(PROTOCOL_VERSION, "ling.lsp.lifecycle/0.1");
}

#[test]
fn library_dispatch_keeps_notifications_response_free() {
    let mut server = ling_lsp::LspServer::new();
    assert_eq!(
        server.handle_json(&notification("initialized")),
        HandleOutcome::NoResponse
    );
}
