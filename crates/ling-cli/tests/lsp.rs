use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::{Value, json};

fn frame(value: Value) -> Vec<u8> {
    let body = serde_json::to_vec(&value).expect("fixture JSON is serializable");
    let mut bytes = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
    bytes.extend_from_slice(&body);
    bytes
}

fn response_bodies(mut bytes: &[u8]) -> Vec<Value> {
    let mut values = Vec::new();
    while !bytes.is_empty() {
        let marker = b"\r\n\r\n";
        let header_end = bytes
            .windows(marker.len())
            .position(|window| window == marker)
            .expect("response has a complete header");
        let length = std::str::from_utf8(&bytes[..header_end])
            .expect("response headers are ASCII")
            .strip_prefix("Content-Length: ")
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
fn lsp_stdio_cli_keeps_stdout_framed_and_stderr_quiet() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ling"))
        .args(["lsp", "--stdio"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn ling lsp --stdio");
    let mut input = Vec::new();
    input.extend_from_slice(&frame(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {"capabilities": {"general": {"positionEncodings": ["utf-16"]}}},
    })));
    input.extend_from_slice(&frame(json!({
        "jsonrpc": "2.0",
        "method": "initialized",
    })));
    input.extend_from_slice(&frame(json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "shutdown",
    })));
    input.extend_from_slice(&frame(json!({
        "jsonrpc": "2.0",
        "method": "exit",
    })));
    child
        .stdin
        .take()
        .expect("child stdin")
        .write_all(&input)
        .expect("write lifecycle transcript");

    let output = child.wait_with_output().expect("wait for lifecycle server");
    assert_eq!(output.status.code(), Some(0));
    assert!(
        output.stderr.is_empty(),
        "unexpected stderr: {:?}",
        output.stderr
    );
    let responses = response_bodies(&output.stdout);
    assert_eq!(responses.len(), 2);
    assert_eq!(
        responses[0]["result"]["capabilities"]["positionEncoding"],
        "utf-16"
    );
    assert_eq!(
        responses[1],
        json!({"id": 2, "jsonrpc": "2.0", "result": null})
    );
}
