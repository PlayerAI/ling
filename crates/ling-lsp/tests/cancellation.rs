use std::collections::VecDeque;
use std::io;
use std::sync::{Arc, Condvar, Mutex};

use ling_lsp::{
    CancellationError, CancellationToken, HandleOutcome, JSON_RPC_VERSION, LspServer,
    REQUEST_CANCELLATION_PROTOCOL_VERSION, TransportError, run_stdio,
};
use serde_json::{Value, json};

#[test]
fn cancellation_is_clone_shared_monotonic_and_idempotent() {
    let token = CancellationToken::new();
    let worker = token.clone();

    assert!(!token.is_cancelled());
    assert_eq!(token.check(), Ok(()));
    assert_eq!(worker.check(), Ok(()));

    worker.cancel();
    assert!(token.is_cancelled());
    assert!(worker.is_cancelled());
    assert_eq!(token.check(), Err(CancellationError::Cancelled));
    assert_eq!(worker.check(), Err(CancellationError::Cancelled));

    token.cancel();
    worker.cancel();
    assert_eq!(token.check(), Err(CancellationError::Cancelled));
    assert_eq!(worker.check(), Err(CancellationError::Cancelled));
}

#[test]
fn independent_tokens_do_not_share_cancellation_state() {
    let cancelled = CancellationToken::new();
    let active = CancellationToken::new();

    cancelled.cancel();
    assert_eq!(cancelled.check(), Err(CancellationError::Cancelled));
    assert_eq!(active.check(), Ok(()));
}

fn frame(value: Value) -> Vec<u8> {
    let body = serde_json::to_vec(&value).expect("test JSON");
    let mut frame = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
    frame.extend(body);
    frame
}

fn request(id: Value, method: &str, params: Value) -> Value {
    json!({"jsonrpc": JSON_RPC_VERSION, "id": id, "method": method, "params": params})
}

fn notification(method: &str, params: Value) -> Value {
    json!({"jsonrpc": JSON_RPC_VERSION, "method": method, "params": params})
}

#[derive(Clone, Default)]
struct Progress(Arc<(Mutex<usize>, Condvar)>);

impl Progress {
    fn advance(&self) {
        let (value, wake) = &*self.0;
        *value.lock().expect("progress lock") += 1;
        wake.notify_all();
    }

    fn wait_for(&self, target: usize) {
        let (value, wake) = &*self.0;
        let mut value = value.lock().expect("progress lock");
        while *value < target {
            value = wake.wait(value).expect("progress wait");
        }
    }
}

struct ChunkReader {
    chunks: VecDeque<Vec<u8>>,
    progress: Progress,
}

impl io::Read for ChunkReader {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let Some(chunk) = self.chunks.pop_front() else {
            return Ok(0);
        };
        assert!(
            chunk.len() <= output.len(),
            "test frame fits BufReader buffer"
        );
        output[..chunk.len()].copy_from_slice(&chunk);
        self.progress.advance();
        Ok(chunk.len())
    }
}

struct BlockingWriter {
    bytes: Arc<Mutex<Vec<u8>>>,
    progress: Progress,
    release_at: usize,
    released: bool,
}

struct FailingWriter;

impl io::Write for FailingWriter {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "test writer"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Write for BlockingWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if !self.released {
            self.progress.wait_for(self.release_at);
            self.released = true;
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
fn framed_cancel_notification_cancels_live_request_before_publication() {
    let progress = Progress::default();
    let uri = "ling://workspace/src/Main.ling";
    let chunks = [
        frame(request(json!(1), "initialize", json!({"capabilities": {}}))),
        frame(notification("initialized", json!({}))),
        frame(notification(
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "languageId": "ling",
                    "text": "module Main\n\nlet answer = 42\n",
                    "uri": uri,
                    "version": 1
                }
            }),
        )),
        frame(request(
            json!("symbols-1"),
            "workspace/symbol",
            json!({"query": "answer"}),
        )),
        frame(notification("$/cancelRequest", json!({"id": "symbols-1"}))),
        frame(notification("exit", json!({}))),
    ];
    let reader = ChunkReader {
        chunks: chunks.into_iter().collect(),
        progress: progress.clone(),
    };
    let output = Arc::new(Mutex::new(Vec::new()));
    let writer = BlockingWriter {
        bytes: Arc::clone(&output),
        progress,
        release_at: 6,
        released: false,
    };

    let result = run_stdio(reader, writer).expect("cancellation transcript");
    assert_eq!(result.exit_code(), 1);
    let output = output.lock().expect("output lock");
    let responses = response_bodies(&output);
    let initialize = responses
        .iter()
        .find(|value| value["id"] == 1)
        .expect("initialize response");
    assert_eq!(
        initialize["result"]["capabilities"]["experimental"]["lingRequestCancellation"]["version"],
        REQUEST_CANCELLATION_PROTOCOL_VERSION
    );
    let cancelled = responses
        .iter()
        .find(|value| value["id"] == "symbols-1")
        .expect("cancelled request response");
    assert_eq!(cancelled["error"]["code"], -32_800);
    assert_eq!(
        responses
            .iter()
            .filter(|value| value.get("id") == Some(&Value::Null))
            .count(),
        0,
        "cancel notifications remain response-free"
    );
}

#[test]
fn request_form_cancel_is_rejected_and_notification_forms_stay_response_free() {
    let mut server = LspServer::new();
    let request_form = serde_json::to_vec(&request(json!(9), "$/cancelRequest", json!({"id": 1})))
        .expect("test JSON");
    let HandleOutcome::Response(response) = server.handle_json(&request_form) else {
        panic!("request-form cancellation must receive Invalid Request");
    };
    let response: Value = serde_json::from_slice(&response).expect("response JSON");
    assert_eq!(response["error"]["code"], -32_600);

    for params in [json!({"id": null}), json!({"id": true}), json!({})] {
        let body = serde_json::to_vec(&notification("$/cancelRequest", params)).expect("test JSON");
        assert_eq!(server.handle_json(&body), HandleOutcome::NoResponse);
    }
}

#[test]
fn cancellation_fixture_corpus_matches_exact_envelope_contract() {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../tests/protocols/lsp-request-cancellation/fixtures/v1.json"
    ))
    .expect("cancellation fixture corpus JSON");
    assert_eq!(
        corpus["fixtureFormat"],
        "ling.test.lsp-request-cancellation/1"
    );
    assert_eq!(corpus["protocol"], REQUEST_CANCELLATION_PROTOCOL_VERSION);

    let mut initialized = LspServer::new();
    let initialize = serde_json::to_vec(&request(
        json!(1),
        "initialize",
        json!({"capabilities": {}}),
    ))
    .expect("initialize JSON");
    let HandleOutcome::Response(bytes) = initialized.handle_json(&initialize) else {
        panic!("initialize response")
    };
    let response: Value = serde_json::from_slice(&bytes).expect("initialize response JSON");
    assert_eq!(
        response["result"]["capabilities"]["experimental"]["lingRequestCancellation"],
        corpus["discovery"]
    );

    let mut names = Vec::new();
    for case in corpus["cases"].as_array().expect("fixture cases") {
        let name = case["name"].as_str().expect("fixture name");
        assert!(!names.contains(&name), "duplicate fixture name: {name}");
        names.push(name);
        let body = serde_json::to_vec(&case["message"]).expect("fixture message JSON");
        let actual = match LspServer::new().handle_json(&body) {
            HandleOutcome::Response(bytes) => {
                serde_json::from_slice(&bytes).expect("fixture response JSON")
            }
            HandleOutcome::NoResponse => Value::Null,
            outcome => panic!("unexpected fixture outcome for {name}: {outcome:?}"),
        };
        assert_eq!(actual, case["expected"], "fixture case: {name}");
    }
    assert_eq!(names.len(), 4);
}

#[test]
fn executor_writer_failure_aborts_without_a_guessed_protocol_response() {
    let input = frame(request(json!(1), "initialize", json!({"capabilities": {}})));
    let error = run_stdio(input.as_slice(), FailingWriter).expect_err("writer must fail");
    assert!(
        matches!(error, TransportError::Io(error) if error.kind() == io::ErrorKind::BrokenPipe)
    );
}
