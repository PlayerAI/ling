use ling_lsp::{
    HandleOutcome, JSON_RPC_VERSION, LspServer, MAX_FRAME_BYTES, MAX_PULL_PREVIOUS_RESULTS,
    PULL_DIAGNOSTICS_PROTOCOL_VERSION,
};
use serde_json::{Value, json};

const URI: &str = "ling://workspace/src/Main.ling";

fn message(id: Option<u64>, method: &str, params: Value) -> Vec<u8> {
    let mut value = json!({
        "jsonrpc": JSON_RPC_VERSION,
        "method": method,
        "params": params,
    });
    if let Some(id) = id {
        value["id"] = json!(id);
    }
    serde_json::to_vec(&value).unwrap()
}

fn response(outcome: HandleOutcome) -> Value {
    let HandleOutcome::Response(bytes) = outcome else {
        panic!("request must produce a response")
    };
    serde_json::from_slice(&bytes).unwrap()
}

fn ready(pull: bool) -> (LspServer, Value) {
    ready_with_options(pull, json!({}))
}

fn ready_with_options(pull: bool, initialization_options: Value) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let capabilities = if pull {
        json!({"textDocument": {"diagnostic": {}}})
    } else {
        json!({})
    };
    let initialized = response(server.handle_json(&message(
        Some(1),
        "initialize",
        json!({
            "capabilities": capabilities,
            "initializationOptions": initialization_options,
        }),
    )));
    assert_eq!(
        server.handle_json(&message(None, "initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    (server, initialized)
}

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&message(
            None,
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "languageId": "ling",
                    "text": text,
                    "uri": uri,
                    "version": version,
                }
            }),
        )),
        HandleOutcome::NoResponse
    );
}

fn change(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&message(
            None,
            "textDocument/didChange",
            json!({
                "contentChanges": [{"text": text}],
                "textDocument": {"uri": uri, "version": version},
            }),
        )),
        HandleOutcome::NoResponse
    );
}

fn document_pull(server: &mut LspServer, previous: Option<&str>) -> Value {
    let mut params = json!({
        "identifier": PULL_DIAGNOSTICS_PROTOCOL_VERSION,
        "textDocument": {"uri": URI},
    });
    if let Some(previous) = previous {
        params["previousResultId"] = json!(previous);
    }
    response(server.handle_json(&message(Some(9), "textDocument/diagnostic", params)))
}

#[test]
fn capability_and_methods_are_available_only_when_negotiated() {
    let (mut supported, initialized) = ready(true);
    assert_eq!(
        initialized["result"]["capabilities"]["diagnosticProvider"],
        json!({
            "identifier": PULL_DIAGNOSTICS_PROTOCOL_VERSION,
            "interFileDependencies": true,
            "workDoneProgress": false,
            "workspaceDiagnostics": true,
        })
    );
    assert_eq!(
        supported.handle_json(&message(
            None,
            "textDocument/diagnostic",
            json!({"textDocument": {"uri": URI}}),
        )),
        HandleOutcome::NoResponse
    );

    let (mut unsupported, initialized) = ready(false);
    assert!(
        initialized["result"]["capabilities"]
            .get("diagnosticProvider")
            .is_none()
    );
    let rejected = response(unsupported.handle_json(&message(
        Some(2),
        "workspace/diagnostic",
        json!({"previousResultIds": []}),
    )));
    assert_eq!(rejected["error"]["code"], -32601);
}

#[test]
fn malformed_pull_capability_rejects_initialize_without_changing_lifecycle() {
    for diagnostic in [Value::Null, json!(false), json!([]), json!("yes")] {
        let mut server = LspServer::new();
        let rejected = response(server.handle_json(&message(
            Some(1),
            "initialize",
            json!({"capabilities": {"textDocument": {"diagnostic": diagnostic}}}),
        )));
        assert_eq!(rejected["error"]["code"], -32602);
        assert_eq!(server.state(), ling_lsp::LifecycleState::Uninitialized);
    }
}

#[test]
fn document_full_unchanged_and_push_values_are_identical() {
    let (mut server, _) = ready(true);
    open(
        &mut server,
        URI,
        1,
        "module Main\n\nlet value: Int = \"text\"\n",
    );
    assert!(server.diagnostics_pending());

    let full = document_pull(&mut server, None);
    assert_eq!(full["result"]["kind"], "full");
    assert_eq!(full["result"]["items"][0]["code"], "L-TYPE-0001");
    assert!(server.diagnostics_pending());
    assert!(server.take_notifications().is_empty());

    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let notifications = server.take_notifications();
    let push: Value = serde_json::from_slice(&notifications[0]).unwrap();
    assert_eq!(
        serde_json::to_vec(&full["result"]["items"]).unwrap(),
        serde_json::to_vec(&push["params"]["diagnostics"]).unwrap()
    );

    let result_id = full["result"]["resultId"].as_str().unwrap();
    let unchanged = document_pull(&mut server, Some(result_id));
    assert_eq!(
        unchanged["result"],
        json!({"kind": "unchanged", "resultId": result_id})
    );
}

#[test]
fn result_identity_ignores_version_but_changes_with_diagnostics() {
    let (mut server, _) = ready(true);
    let invalid = "module Main\n\nlet value: Int = \"text\"\n";
    open(&mut server, URI, 1, invalid);
    let first = document_pull(&mut server, None);
    let result_id = first["result"]["resultId"].as_str().unwrap().to_owned();

    change(&mut server, URI, 2, invalid);
    let same = document_pull(&mut server, Some(&result_id));
    assert_eq!(same["result"]["kind"], "unchanged");

    change(&mut server, URI, 3, "module Main\n\nlet value: Int = 1\n");
    let fixed = document_pull(&mut server, Some(&result_id));
    assert_eq!(fixed["result"]["kind"], "full");
    assert_eq!(fixed["result"]["items"], json!([]));
    assert_ne!(fixed["result"]["resultId"], result_id);
}

#[test]
fn workspace_reports_are_uri_sorted_and_preserve_open_closed_versions() {
    let (mut server, _) = ready(true);
    let closed = "ling://workspace/a/Closed.ling";
    let open_uri = "ling://workspace/z/Open.ling";
    server
        .publish_disk_snapshot(closed, "module Closed\n\nlet value: Int = 1\n")
        .unwrap();
    open(
        &mut server,
        open_uri,
        7,
        "module Open\n\nlet value: Int = \"bad\"\n",
    );

    let first = response(server.handle_json(&message(
        Some(2),
        "workspace/diagnostic",
        json!({
            "identifier": PULL_DIAGNOSTICS_PROTOCOL_VERSION,
            "partialResultToken": {"ignored": true},
            "previousResultIds": [],
            "unknownFutureField": 42,
            "workDoneToken": ["ignored"],
        }),
    )));
    let reports = first["result"]["items"].as_array().unwrap();
    assert_eq!(reports.len(), 2);
    assert_eq!(reports[0]["uri"], closed);
    assert_eq!(reports[0]["version"], Value::Null);
    assert_eq!(reports[1]["uri"], open_uri);
    assert_eq!(reports[1]["version"], 7);

    let previous = reports
        .iter()
        .rev()
        .map(|report| {
            json!({
                "uri": report["uri"],
                "value": report["resultId"],
                "future": true,
            })
        })
        .collect::<Vec<_>>();
    let repeated = response(server.handle_json(&message(
        Some(3),
        "workspace/diagnostic",
        json!({"previousResultIds": previous}),
    )));
    assert!(
        repeated["result"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .all(|report| report["kind"] == "unchanged" && report.get("items").is_none())
    );
}

#[test]
fn removed_workspace_uri_gets_a_full_empty_clearance() {
    let (mut server, _) = ready(true);
    server
        .publish_disk_snapshot(URI, "module Main\n\nlet value: Int = \"bad\"\n")
        .unwrap();
    let initial = response(server.handle_json(&message(
        Some(2),
        "workspace/diagnostic",
        json!({"previousResultIds": []}),
    )));
    let old_id = initial["result"]["items"][0]["resultId"]
        .as_str()
        .unwrap()
        .to_owned();
    let base = server.capture_request_snapshot().unwrap().revision().get();
    let removed = response(server.handle_json(&message(
        Some(3),
        "ling/workspace/reload",
        json!({
            "baseRevision": base.to_string(),
            "inputs": [],
            "sources": [{"uri": URI, "text": null}],
        }),
    )));
    assert_eq!(removed["result"]["changed"], true, "{removed}");

    let cleared = response(server.handle_json(&message(
        Some(4),
        "workspace/diagnostic",
        json!({"previousResultIds": [{"uri": URI, "value": old_id}]}),
    )));
    assert_eq!(cleared["result"]["items"][0]["uri"], URI);
    assert_eq!(cleared["result"]["items"][0]["version"], Value::Null);
    assert_eq!(cleared["result"]["items"][0]["kind"], "full");
    assert_eq!(cleared["result"]["items"][0]["items"], json!([]));
}

#[test]
fn invalid_params_are_rejected_before_analysis_and_leave_pending_work() {
    let (mut server, _) = ready(true);
    open(&mut server, URI, 1, "module Main\n\nlet value = @\n");
    let cases = [
        json!({"textDocument": {"uri": URI}, "identifier": "other"}),
        json!({"textDocument": {"uri": URI}, "previousResultId": null}),
        json!({"textDocument": {"uri": "ling://workspace/missing.ling"}}),
        json!({"textDocument": {"uri": "file:///tmp/Main.ling"}}),
    ];
    for params in cases {
        let rejected =
            response(server.handle_json(&message(Some(8), "textDocument/diagnostic", params)));
        assert_eq!(rejected["error"]["code"], -32602);
        assert!(server.diagnostics_pending());
        assert!(server.take_notifications().is_empty());
    }

    let duplicate = response(server.handle_json(&message(
        Some(9),
        "workspace/diagnostic",
        json!({"previousResultIds": [
            {"uri": URI, "value": "a"},
            {"uri": URI, "value": "b"},
        ]}),
    )));
    assert_eq!(duplicate["error"]["code"], -32602);
}

#[test]
fn workspace_previous_result_bound_and_empty_workspace_are_exact() {
    let (mut server, _) = ready(true);
    let empty = response(server.handle_json(&message(
        Some(2),
        "workspace/diagnostic",
        json!({"previousResultIds": []}),
    )));
    assert_eq!(empty["result"], json!({"items": []}));

    let previous = (0..=MAX_PULL_PREVIOUS_RESULTS)
        .map(|index| {
            json!({
                "uri": format!("ling://workspace/generated/{index}.ling"),
                "value": "x",
            })
        })
        .collect::<Vec<_>>();
    let rejected = response(server.handle_json(&message(
        Some(3),
        "workspace/diagnostic",
        json!({"previousResultIds": previous}),
    )));
    assert_eq!(rejected["error"]["code"], -32602);
}

#[test]
fn temporary_document_pull_is_syntax_only_and_uses_its_exact_uri() {
    let temporary = "untitled://ling/Scratch.ling";
    let (mut server, _) = ready(true);
    open(
        &mut server,
        temporary,
        1,
        "module Scratch\n\nlet value = @\n",
    );
    let pulled = response(server.handle_json(&message(
        Some(2),
        "textDocument/diagnostic",
        json!({"textDocument": {"uri": temporary}}),
    )));
    assert_eq!(pulled["result"]["kind"], "full");
    assert_eq!(pulled["result"]["items"][0]["code"], "L-LEX-0004");
    assert!(
        pulled["result"]["resultId"]
            .as_str()
            .unwrap()
            .starts_with("ling.lsp.pull-result/0.1:blake3:")
    );
}

#[test]
fn oversized_success_becomes_bounded_request_failed() {
    let (mut server, _) = ready_with_options(
        true,
        json!({"lingDiagnosticControl": {
            "maxPerDocument": 4_096,
            "maxPerWorkspace": 65_536,
        }}),
    );
    let text = "@".repeat(20_000);
    assert!(text.len() < MAX_FRAME_BYTES);
    open(&mut server, URI, 1, &text);
    let rejected = document_pull(&mut server, None);
    assert_eq!(rejected["error"]["code"], -32803);
    assert!(server.diagnostics_pending());
    assert!(server.take_notifications().is_empty());
}
