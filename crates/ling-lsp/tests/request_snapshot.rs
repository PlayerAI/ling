use ling_lsp::{FileOrigin, LspServer, PositionEncoding};
use serde_json::{Value, json};

fn request(id: u64, method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params,
    }))
    .expect("request is serializable")
}

fn notification(method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    }))
    .expect("notification is serializable")
}

fn ready_server() -> LspServer {
    let mut server = LspServer::new();
    let _ = server.handle_json(&request(
        1,
        "initialize",
        json!({"capabilities": {"general": {"positionEncodings": ["utf-8"]}}}),
    ));
    let _ = server.handle_json(&notification("initialized", json!({})));
    assert_eq!(server.position_encoding(), PositionEncoding::Utf8);
    server
}

#[test]
fn request_snapshot_is_immutable_and_separates_versions_from_revisions() {
    let mut server = ready_server();
    let disk_uri = "ling://workspace/main.ling";
    let overlay_uri = "ling://workspace/overlay.ling";
    server
        .publish_disk_snapshot(disk_uri, "disk")
        .expect("disk snapshot publishes");
    let _ = server.handle_json(&request(
        2,
        "textDocument/didOpen",
        json!({
            "textDocument": {"uri": overlay_uri, "version": 7, "text": "overlay-1"}
        }),
    ));

    let first = server
        .capture_request_snapshot()
        .expect("request snapshot captures");
    assert_eq!(first.state(), ling_lsp::LifecycleState::Ready);
    assert_eq!(first.position_encoding(), PositionEncoding::Utf8);
    assert_eq!(first.documents().len(), 2);
    assert_eq!(first.documents()[0].uri(), disk_uri);
    assert_eq!(first.documents()[1].uri(), overlay_uri);
    assert_eq!(first.document(disk_uri).unwrap().bytes(), b"disk");
    assert_eq!(first.document(disk_uri).unwrap().origin(), FileOrigin::Disk);
    assert!(!first.document(disk_uri).unwrap().is_open());
    assert_eq!(first.document(disk_uri).unwrap().client_version(), None);
    assert_eq!(first.document(overlay_uri).unwrap().bytes(), b"overlay-1");
    assert_eq!(
        first.document(overlay_uri).unwrap().origin(),
        FileOrigin::Overlay
    );
    assert!(first.document(overlay_uri).unwrap().is_open());
    assert_eq!(
        first.document(overlay_uri).unwrap().client_version(),
        Some(7)
    );
    assert!(first.document(overlay_uri).unwrap().revision() <= first.revision());

    let _ = server.handle_json(&request(
        3,
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": overlay_uri, "version": 8},
            "contentChanges": [{"text": "overlay-2"}]
        }),
    ));
    let second = server
        .capture_request_snapshot()
        .expect("second request snapshot captures");
    assert_eq!(first.document(overlay_uri).unwrap().bytes(), b"overlay-1");
    assert_eq!(second.document(overlay_uri).unwrap().bytes(), b"overlay-2");
    assert_eq!(
        second.document(overlay_uri).unwrap().client_version(),
        Some(8)
    );
    assert!(second.revision() > first.revision());
    assert!(
        second.document(overlay_uri).unwrap().revision()
            > first.document(overlay_uri).unwrap().revision()
    );
}

#[test]
fn close_and_disk_publication_are_visible_only_to_later_captures() {
    let mut server = ready_server();
    let uri = "ling://workspace/main.ling";
    server
        .publish_disk_snapshot(uri, "disk-1")
        .expect("disk snapshot publishes");
    let _ = server.handle_json(&request(
        2,
        "textDocument/didOpen",
        json!({"textDocument": {"uri": uri, "version": 1, "text": "edit"}}),
    ));
    let before_close = server
        .capture_request_snapshot()
        .expect("capture before close");
    assert_eq!(before_close.document(uri).unwrap().bytes(), b"edit");
    assert_eq!(
        before_close.document(uri).unwrap().client_version(),
        Some(1)
    );

    server
        .publish_disk_snapshot(uri, "disk-2")
        .expect("disk update publishes behind overlay");
    let hidden = server
        .capture_request_snapshot()
        .expect("capture while overlay is open");
    assert_eq!(hidden.document(uri).unwrap().bytes(), b"edit");

    let _ = server.handle_json(&request(
        3,
        "textDocument/didClose",
        json!({"textDocument": {"uri": uri}}),
    ));
    let after_close = server
        .capture_request_snapshot()
        .expect("capture after close");
    assert_eq!(after_close.document(uri).unwrap().bytes(), b"disk-2");
    assert_eq!(
        after_close.document(uri).unwrap().origin(),
        FileOrigin::Disk
    );
    assert!(!after_close.document(uri).unwrap().is_open());
    assert_eq!(after_close.document(uri).unwrap().client_version(), None);
}
