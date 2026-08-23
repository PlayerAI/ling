use ling_lsp::{
    HandleOutcome, JSON_RPC_VERSION, LspServer, MAX_CONTENT_CHANGES, MAX_DOCUMENT_BYTES,
    OVERLAY_PROTOCOL_VERSION,
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

fn ready_server(encoding: &str) -> LspServer {
    let mut server = LspServer::new();
    let initialized = response(server.handle_json(&message(
        Some(1),
        "initialize",
        json!({
            "capabilities": {"general": {"positionEncodings": [encoding]}},
        }),
    )));
    assert_eq!(
        initialized["result"]["capabilities"]["textDocumentSync"],
        json!({"change": 2, "openClose": true})
    );
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingOverlay"],
        json!({"changeLimit": MAX_CONTENT_CHANGES, "version": OVERLAY_PROTOCOL_VERSION})
    );
    assert_eq!(
        server.handle_json(&message(None, "initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    server
}

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&message(
            None,
            "textDocument/didOpen",
            json!({"textDocument": {"text": text, "uri": uri, "version": version}}),
        )),
        HandleOutcome::NoResponse
    );
}

fn change(server: &mut LspServer, uri: &str, version: i64, changes: Value) -> Value {
    response(server.handle_json(&message(
        Some(9),
        "textDocument/didChange",
        json!({
            "contentChanges": changes,
            "textDocument": {"uri": uri, "version": version},
        }),
    )))
}

fn ranged(line: u32, start: u32, end: u32, text: &str) -> Value {
    json!({
        "range": {
            "end": {"character": end, "line": line},
            "start": {"character": start, "line": line},
        },
        "text": text,
    })
}

#[test]
fn negotiated_utf8_utf16_and_utf32_ranges_match_full_replacement() {
    let cases = [("utf-8", 1, 5), ("utf-16", 1, 3), ("utf-32", 1, 2)];
    for (encoding, start, end) in cases {
        let incremental_uri = format!("untitled://ling/{encoding}-incremental.ling");
        let full_uri = format!("untitled://ling/{encoding}-full.ling");
        let mut server = ready_server(encoding);
        open(&mut server, &incremental_uri, 1, "A😀B");
        open(&mut server, &full_uri, 1, "A😀B");

        let incremental = change(
            &mut server,
            &incremental_uri,
            2,
            json!([ranged(0, start, end, "零")]),
        );
        let full = change(&mut server, &full_uri, 2, json!([{"text": "A零B"}]));

        assert_eq!(incremental["result"], Value::Null);
        assert_eq!(full["result"], Value::Null);
        assert_eq!(server.document(&incremental_uri).unwrap().text(), "A零B");
        assert_eq!(
            server.document(&incremental_uri).unwrap().text(),
            server.document(&full_uri).unwrap().text()
        );
    }
}

#[test]
fn ordered_batches_rebuild_lines_and_allow_mixed_full_and_range_changes() {
    let uri = "ling://workspace/src/Ordered.ling";
    let mut server = ready_server("utf-16");
    open(&mut server, uri, 4, "ab\r\n终");

    let ordered = change(
        &mut server,
        uri,
        5,
        json!([
            ranged(0, 1, 1, "x\n"),
            ranged(1, 0, 1, "文"),
            {"text": "😀x"},
            ranged(0, 2, 3, "零"),
        ]),
    );

    assert_eq!(ordered["result"], Value::Null);
    let view = server.document(uri).unwrap();
    assert_eq!(view.text(), "😀零");
    assert_eq!(view.version(), 5);
}

#[test]
fn bom_crlf_combining_and_supplementary_text_preserve_exact_bytes() {
    let uri = "ling://workspace/src/Unicode.ling";
    let mut server = ready_server("utf-16");
    open(&mut server, uri, 1, "\u{feff}零😀\r\ne\u{301}\r\n");

    let changed = change(
        &mut server,
        uri,
        2,
        json!([ranged(0, 1, 3, "语言"), ranged(1, 0, 2, "é"),]),
    );

    assert_eq!(changed["result"], Value::Null);
    assert_eq!(
        server.document(uri).unwrap().text(),
        "\u{feff}零语言\r\né\r\n"
    );
}

#[test]
fn a_later_invalid_range_leaves_bytes_version_and_revision_unchanged() {
    let uri = "ling://workspace/src/Atomic.ling";
    let mut server = ready_server("utf-16");
    open(&mut server, uri, 7, "A😀B");
    let before = server.capture_request_snapshot().expect("snapshot before");

    let invalid = change(
        &mut server,
        uri,
        8,
        json!([ranged(0, 0, 1, "Z"), ranged(0, 2, 3, "must-not-apply"),]),
    );

    assert_eq!(invalid["error"]["code"], -32_602);
    assert_eq!(
        invalid["error"]["message"],
        "文档编辑范围无效 / invalid document edit range"
    );
    let view = server.document(uri).unwrap();
    assert_eq!(view.text(), "A😀B");
    assert_eq!(view.version(), 7);
    let after = server.capture_request_snapshot().expect("snapshot after");
    assert_eq!(after.revision(), before.revision());
    assert_eq!(after, before);
}

#[test]
fn change_limits_and_invalid_position_shapes_are_failure_atomic() {
    let uri = "ling://workspace/src/Limits.ling";
    let mut server = ready_server("utf-8");
    open(&mut server, uri, 1, "a");

    for changes in [
        json!([]),
        Value::Array(
            (0..=MAX_CONTENT_CHANGES)
                .map(|_| json!({"text": "a"}))
                .collect(),
        ),
        json!([{"rangeLength": 1, "text": "b"}]),
        json!([{
            "range": {
                "end": {"character": 1, "line": 0},
                "start": {"character": -1, "line": 0}
            },
            "text": "b"
        }]),
        json!([{
            "range": {
                "end": {"character": 1, "line": 0},
                "start": {"character": 4_294_967_296_u64, "line": 0}
            },
            "text": "b"
        }]),
    ] {
        let rejected = change(&mut server, uri, 2, changes);
        assert_eq!(rejected["error"]["code"], -32_602);
        assert_eq!(server.document(uri).unwrap().text(), "a");
        assert_eq!(server.document(uri).unwrap().version(), 1);
    }

    let oversized = change(
        &mut server,
        uri,
        2,
        Value::Array(vec![
            json!({"text": "x".repeat(MAX_DOCUMENT_BYTES)}),
            ranged(0, 0, 0, "y"),
        ]),
    );
    assert_eq!(oversized["error"]["code"], -32_602);
    assert_eq!(server.document(uri).unwrap().text(), "a");
    assert_eq!(server.document(uri).unwrap().version(), 1);
}

#[test]
fn public_range_failures_cover_boundaries_and_suppress_notification_errors() {
    let uri = "ling://workspace/src/Boundaries.ling";
    let mut server = ready_server("utf-16");
    open(&mut server, uri, 1, "a😀\r\nb");
    let before = server.capture_request_snapshot().expect("snapshot before");

    let invalid_batches = [
        json!([ranged(0, 2, 3, "surrogate-interior")]),
        json!([{
            "range": {
                "end": {"character": 0, "line": 0},
                "start": {"character": 1, "line": 1}
            },
            "text": "reversed"
        }]),
        json!([ranged(9, 0, 0, "unknown-line")]),
        json!([ranged(1, 0, 2, "overlong")]),
        json!([ranged(1, 0, 0, "\u{feff}")]),
    ];
    for changes in invalid_batches {
        let rejected = change(&mut server, uri, 2, changes);
        assert_eq!(rejected["error"]["code"], -32_602);
        assert_eq!(
            server
                .capture_request_snapshot()
                .expect("unchanged snapshot"),
            before
        );
    }

    let notification = server.handle_json(&message(
        None,
        "textDocument/didChange",
        json!({
            "contentChanges": [ranged(0, 2, 3, "ignored")],
            "textDocument": {"uri": uri, "version": 2},
        }),
    ));
    assert_eq!(notification, HandleOutcome::NoResponse);
    assert_eq!(
        server
            .capture_request_snapshot()
            .expect("notification atomic"),
        before
    );

    let utf8_uri = "ling://workspace/src/Utf8Boundary.ling";
    let mut utf8_server = ready_server("utf-8");
    open(&mut utf8_server, utf8_uri, 1, "a😀b");
    let utf8_rejected = change(
        &mut utf8_server,
        utf8_uri,
        2,
        json!([ranged(0, 2, 5, "scalar-interior")]),
    );
    assert_eq!(utf8_rejected["error"]["code"], -32_602);
    assert_eq!(utf8_server.document(utf8_uri).unwrap().text(), "a😀b");
    assert_eq!(utf8_server.document(utf8_uri).unwrap().version(), 1);
}

#[test]
fn legacy_full_replacement_preserves_exact_utf8_that_is_not_valid_ling_source() {
    let uri = "ling://workspace/src/Incomplete.ling";
    let mut server = ready_server("utf-16");
    open(&mut server, uri, 1, "let value = 1\n");

    let exact_overlay = "let \u{feff}unfinished =";
    let accepted = change(&mut server, uri, 2, json!([{"text": exact_overlay}]));

    assert_eq!(accepted["result"], Value::Null);
    assert_eq!(server.document(uri).unwrap().text(), exact_overlay);
    assert_eq!(server.document(uri).unwrap().version(), 2);

    let before_range = server
        .capture_request_snapshot()
        .expect("snapshot before range");
    let rejected = change(&mut server, uri, 3, json!([ranged(0, 0, 0, "prefix")]));
    assert_eq!(rejected["error"]["code"], -32_602);
    assert_eq!(
        server
            .capture_request_snapshot()
            .expect("unchanged snapshot"),
        before_range
    );
}
