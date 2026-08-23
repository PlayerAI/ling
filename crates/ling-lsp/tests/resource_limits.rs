use std::collections::VecDeque;
use std::io;
use std::sync::{Arc, Condvar, Mutex};

use ling_lsp::{
    HandleOutcome, JSON_RPC_VERSION, LifecycleState, LspServer, MAX_DOCUMENT_BYTES,
    MAX_LIVE_REQUESTS, MAX_OPEN_DOCUMENT_BYTES, RESOURCE_LIMITS_PROTOCOL_VERSION, run_stdio,
};
use serde_json::{Value, json};

fn request(id: Value, method: &str, params: Value) -> Value {
    json!({"jsonrpc": JSON_RPC_VERSION, "id": id, "method": method, "params": params})
}

fn notification(method: &str, params: Value) -> Value {
    json!({"jsonrpc": JSON_RPC_VERSION, "method": method, "params": params})
}

fn body(value: Value) -> Vec<u8> {
    serde_json::to_vec(&value).expect("test JSON")
}

fn frame(value: Value) -> Vec<u8> {
    let body = body(value);
    let mut frame = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
    frame.extend(body);
    frame
}

fn response(outcome: HandleOutcome) -> Value {
    let HandleOutcome::Response(bytes) = outcome else {
        panic!("request must produce a response")
    };
    serde_json::from_slice(&bytes).expect("response JSON")
}

fn ready_server() -> LspServer {
    let mut server = LspServer::new();
    let initialized = response(server.handle_json(&body(request(
        json!(1),
        "initialize",
        json!({"capabilities": {}}),
    ))));
    assert_eq!(initialized["result"]["serverInfo"]["name"], "ling");
    assert_eq!(
        server.handle_json(&body(notification("initialized", json!({})))),
        HandleOutcome::NoResponse
    );
    assert_eq!(server.state(), LifecycleState::Ready);
    server
}

fn open(server: &mut LspServer, id: u64, uri: &str, version: i64, text: &str) -> Value {
    response(server.handle_json(&body(request(
        json!(id),
        "textDocument/didOpen",
        json!({"textDocument": {"languageId": "ling", "text": text, "uri": uri, "version": version}}),
    ))))
}

fn change(server: &mut LspServer, id: u64, uri: &str, version: i64, text: &str) -> Value {
    response(server.handle_json(&body(request(
        json!(id),
        "textDocument/didChange",
        json!({
            "contentChanges": [{"text": text}],
            "textDocument": {"uri": uri, "version": version}
        }),
    ))))
}

fn close(server: &mut LspServer, id: u64, uri: &str) -> Value {
    response(server.handle_json(&body(request(
        json!(id),
        "textDocument/didClose",
        json!({"textDocument": {"uri": uri}}),
    ))))
}

fn assert_resource_error(
    response: &Value,
    actual: usize,
    maximum: usize,
    resource: &str,
    scope: &str,
) {
    assert_eq!(response["error"]["code"], -32_803);
    assert_eq!(
        response["error"]["message"],
        "资源上限已超出 / resource limit exceeded"
    );
    assert_eq!(response["error"]["data"]["code"], "L-LSP-0002");
    assert_eq!(response["error"]["data"]["facts"]["actual"], actual);
    assert_eq!(response["error"]["data"]["facts"]["maximum"], maximum);
    assert_eq!(response["error"]["data"]["facts"]["resource"], resource);
    assert_eq!(response["error"]["data"]["facts"]["scope"], scope);
    assert_eq!(
        response["error"]["data"]["version"],
        RESOURCE_LIMITS_PROTOCOL_VERSION
    );
}

#[test]
fn discovery_and_error_shape_match_the_exact_fixture() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../tests/protocols/lsp-resource-limits/fixtures/v1.json"
    ))
    .expect("resource fixture JSON");
    assert_eq!(fixture["protocol"], RESOURCE_LIMITS_PROTOCOL_VERSION);

    let mut server = LspServer::new();
    let initialize = response(server.handle_json(&body(request(
        json!(1),
        "initialize",
        json!({"capabilities": {}}),
    ))));
    assert_eq!(
        initialize["result"]["capabilities"]["experimental"]["lingResourceLimits"],
        fixture["discovery"]
    );

    let mut ready = ready_server();
    let oversized = "x".repeat(MAX_DOCUMENT_BYTES + 1);
    let rejected = open(
        &mut ready,
        2,
        "ling://workspace/src/Oversized.ling",
        1,
        &oversized,
    );
    assert_resource_error(
        &rejected,
        MAX_DOCUMENT_BYTES + 1,
        MAX_DOCUMENT_BYTES,
        "document_bytes",
        "document",
    );
    assert!(
        ready
            .document("ling://workspace/src/Oversized.ling")
            .is_none()
    );
}

#[test]
fn aggregate_overlay_budget_is_exact_failure_atomic_and_released_on_close() {
    let mut server = ready_server();
    let full = "x".repeat(MAX_DOCUMENT_BYTES);
    let half = "零".repeat((MAX_DOCUMENT_BYTES / 2) / "零".len());
    let half_len = half.len();
    assert!(half_len <= MAX_DOCUMENT_BYTES / 2);
    let filler = "y".repeat(MAX_DOCUMENT_BYTES - half_len);

    for index in 0..7 {
        let uri = format!("ling://workspace/src/Full{index}.ling");
        assert_eq!(
            open(&mut server, 10 + index, &uri, 1, &full)["result"],
            Value::Null
        );
    }
    let left = "ling://workspace/src/UnicodeBomCrLf.ling";
    let right = "untitled://ling/Filler.ling";
    assert_eq!(open(&mut server, 20, left, 1, &half)["result"], Value::Null);
    assert_eq!(
        open(&mut server, 21, right, 1, &filler)["result"],
        Value::Null
    );
    assert_eq!(
        7 * MAX_DOCUMENT_BYTES + half_len + filler.len(),
        MAX_OPEN_DOCUMENT_BYTES
    );

    let before = server.capture_request_snapshot().expect("snapshot before");
    let grown = format!("{half}z");
    let rejected = change(&mut server, 22, left, 2, &grown);
    assert_resource_error(
        &rejected,
        MAX_OPEN_DOCUMENT_BYTES + 1,
        MAX_OPEN_DOCUMENT_BYTES,
        "open_document_bytes",
        "session",
    );
    assert_eq!(server.document(left).expect("left document").text(), half);
    assert_eq!(server.document(left).expect("left document").version(), 1);
    assert_eq!(
        server
            .capture_request_snapshot()
            .expect("unchanged snapshot"),
        before
    );

    assert_eq!(close(&mut server, 23, right)["result"], Value::Null);
    assert_eq!(
        change(&mut server, 24, left, 2, &grown)["result"],
        Value::Null
    );
    assert_eq!(
        server.document(left).expect("changed document").text(),
        grown
    );

    let reopened = "ling://workspace/src/Reopened.ling";
    assert_eq!(
        open(&mut server, 25, reopened, 1, "\u{feff}\r\n😀")["result"],
        Value::Null
    );
}

#[derive(Default)]
struct GateState {
    writer_waiting: bool,
    reader_done: bool,
}

#[derive(Clone, Default)]
struct Gate(Arc<(Mutex<GateState>, Condvar)>);

struct CoordinatedReader {
    chunks: VecDeque<Vec<u8>>,
    gate: Gate,
    first: bool,
}

impl io::Read for CoordinatedReader {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if !self.first {
            let (state, wake) = &*self.gate.0;
            let mut state = state.lock().expect("gate lock");
            while !state.writer_waiting {
                state = wake.wait(state).expect("gate wait");
            }
        }
        self.first = false;
        let Some(chunk) = self.chunks.pop_front() else {
            let (state, wake) = &*self.gate.0;
            state.lock().expect("gate lock").reader_done = true;
            wake.notify_all();
            return Ok(0);
        };
        assert!(chunk.len() <= output.len());
        output[..chunk.len()].copy_from_slice(&chunk);
        Ok(chunk.len())
    }
}

struct CoordinatedWriter {
    bytes: Arc<Mutex<Vec<u8>>>,
    gate: Gate,
    first: bool,
}

impl io::Write for CoordinatedWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.first {
            self.first = false;
            let (state, wake) = &*self.gate.0;
            let mut state = state.lock().expect("gate lock");
            state.writer_waiting = true;
            wake.notify_all();
            while !state.reader_done {
                state = wake.wait(state).expect("gate wait");
            }
        }
        self.bytes
            .lock()
            .expect("output lock")
            .extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn response_bodies(mut bytes: &[u8]) -> Vec<Value> {
    let mut values = Vec::new();
    while !bytes.is_empty() {
        let boundary = bytes
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .expect("complete response header");
        let length = std::str::from_utf8(&bytes[16..boundary])
            .expect("ASCII length")
            .parse::<usize>()
            .expect("numeric length");
        let start = boundary + 4;
        let end = start + length;
        values.push(serde_json::from_slice(&bytes[start..end]).expect("response JSON"));
        bytes = &bytes[end..];
    }
    values
}

#[test]
fn framed_129th_live_request_is_rejected_deterministically() {
    let gate = Gate::default();
    let mut chunks = VecDeque::from([frame(request(
        json!("initialize"),
        "initialize",
        json!({"capabilities": {}}),
    ))]);
    chunks.push_back(frame(notification("initialized", json!({}))));
    for id in 0..=MAX_LIVE_REQUESTS {
        chunks.push_back(frame(request(json!(id), "unknown/bounded", json!({}))));
    }
    let reader = CoordinatedReader {
        chunks,
        gate: gate.clone(),
        first: true,
    };
    let output = Arc::new(Mutex::new(Vec::new()));
    let writer = CoordinatedWriter {
        bytes: Arc::clone(&output),
        gate,
        first: true,
    };

    let result = run_stdio(reader, writer).expect("bounded request transcript");
    assert_eq!(result.exit_code(), 0);
    assert_eq!(result.state(), LifecycleState::Ready);
    let output = output.lock().expect("output lock");
    let responses = response_bodies(&output);
    let rejected = responses
        .iter()
        .find(|value| value["id"] == MAX_LIVE_REQUESTS)
        .expect("129th response");
    assert_resource_error(
        rejected,
        MAX_LIVE_REQUESTS + 1,
        MAX_LIVE_REQUESTS,
        "live_requests",
        "session",
    );
    assert_eq!(
        responses
            .iter()
            .filter(|value| value["error"]["code"] == -32_803)
            .count(),
        1
    );
}
