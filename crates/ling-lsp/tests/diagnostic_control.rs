use ling_lsp::{
    DEFAULT_MAX_DIAGNOSTICS_PER_DOCUMENT, DEFAULT_MAX_DIAGNOSTICS_PER_WORKSPACE,
    DIAGNOSTIC_CONTROL_PROTOCOL_VERSION, HandleOutcome, JSON_RPC_VERSION, LspServer,
    MAX_DIAGNOSTICS_PER_DOCUMENT, MAX_DIAGNOSTICS_PER_WORKSPACE,
    PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION, PULL_DIAGNOSTICS_PROTOCOL_VERSION,
};
use serde_json::{Value, json};

const A_URI: &str = "ling://workspace/a.ling";
const B_URI: &str = "ling://workspace/b.ling";

fn message(id: Option<u64>, method: &str, params: Value) -> Vec<u8> {
    let mut value = json!({"jsonrpc": JSON_RPC_VERSION, "method": method, "params": params});
    if let Some(id) = id {
        value["id"] = json!(id);
    }
    serde_json::to_vec(&value).unwrap()
}

fn response(outcome: HandleOutcome) -> Value {
    let HandleOutcome::Response(bytes) = outcome else {
        panic!("request must respond")
    };
    serde_json::from_slice(&bytes).unwrap()
}

fn initialize(options: Value, pull: bool) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let capabilities = if pull {
        json!({"textDocument": {"diagnostic": {}}})
    } else {
        json!({})
    };
    let initialized = response(server.handle_json(&message(
        Some(1),
        "initialize",
        json!({"capabilities": capabilities, "initializationOptions": options}),
    )));
    if initialized.get("result").is_some() {
        assert_eq!(
            server.handle_json(&message(None, "initialized", json!({}))),
            HandleOutcome::NoResponse
        );
    }
    (server, initialized)
}

fn ready(per_document: usize, per_workspace: usize, pull: bool) -> LspServer {
    initialize(
        json!({
            "lingDiagnosticControl": {
                "maxPerDocument": per_document,
                "maxPerWorkspace": per_workspace,
            }
        }),
        pull,
    )
    .0
}

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&message(
            None,
            "textDocument/didOpen",
            json!({"textDocument": {
                "languageId": "ling",
                "text": text,
                "uri": uri,
                "version": version,
            }}),
        )),
        HandleOutcome::NoResponse
    );
}

fn workspace_pull(server: &mut LspServer) -> Value {
    response(server.handle_json(&message(
        Some(8),
        "workspace/diagnostic",
        json!({"previousResultIds": []}),
    )))
}

#[test]
fn initialize_advertises_defaults_custom_limits_and_protocol_migrations() {
    let (_, defaults) = initialize(json!({}), true);
    assert_eq!(
        defaults["result"]["capabilities"]["experimental"]["lingDiagnosticControl"],
        json!({
            "maxPerDocument": DEFAULT_MAX_DIAGNOSTICS_PER_DOCUMENT,
            "maxPerWorkspace": DEFAULT_MAX_DIAGNOSTICS_PER_WORKSPACE,
            "version": DIAGNOSTIC_CONTROL_PROTOCOL_VERSION,
        })
    );
    assert_eq!(
        defaults["result"]["capabilities"]["experimental"]["lingPublishDiagnostics"]["version"],
        PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION
    );
    assert_eq!(
        defaults["result"]["capabilities"]["diagnosticProvider"]["identifier"],
        PULL_DIAGNOSTICS_PROTOCOL_VERSION
    );
    assert_eq!(
        PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION,
        "ling.lsp.publish-diagnostics/0.2"
    );
    assert_eq!(
        PULL_DIAGNOSTICS_PROTOCOL_VERSION,
        "ling.lsp.pull-diagnostics/0.2"
    );

    let (_, custom) = initialize(
        json!({
            "future": true,
            "lingDiagnosticControl": {
                "maxPerDocument": 7,
                "maxPerWorkspace": 11,
                "unknownFutureField": "ignored",
            }
        }),
        false,
    );
    assert_eq!(
        custom["result"]["capabilities"]["experimental"]["lingDiagnosticControl"],
        json!({
            "maxPerDocument": 7,
            "maxPerWorkspace": 11,
            "version": DIAGNOSTIC_CONTROL_PROTOCOL_VERSION,
        })
    );

    let (_, partial) = initialize(
        json!({"lingDiagnosticControl": {"maxPerDocument": 9}}),
        false,
    );
    assert_eq!(
        partial["result"]["capabilities"]["experimental"]["lingDiagnosticControl"],
        json!({
            "maxPerDocument": 9,
            "maxPerWorkspace": DEFAULT_MAX_DIAGNOSTICS_PER_WORKSPACE,
            "version": DIAGNOSTIC_CONTROL_PROTOCOL_VERSION,
        })
    );
}

#[test]
fn invalid_known_limit_shapes_and_bounds_are_failure_atomic() {
    let cases = [
        Value::Null,
        json!([]),
        json!({"maxPerDocument": 0}),
        json!({"maxPerDocument": MAX_DIAGNOSTICS_PER_DOCUMENT + 1}),
        json!({"maxPerDocument": 1.5}),
        json!({"maxPerWorkspace": 0}),
        json!({"maxPerWorkspace": MAX_DIAGNOSTICS_PER_WORKSPACE + 1}),
        json!({"maxPerWorkspace": "100"}),
    ];
    for control in cases {
        let mut server = LspServer::new();
        let rejected = response(server.handle_json(&message(
            Some(1),
            "initialize",
            json!({
                "capabilities": {},
                "initializationOptions": {"lingDiagnosticControl": control},
            }),
        )));
        assert_eq!(rejected["error"]["code"], -32602);
        assert_eq!(server.state(), ling_lsp::LifecycleState::Uninitialized);
    }
}

#[test]
fn document_cap_keeps_roots_and_appends_exact_summary_at_first_omission() {
    let mut server = ready(2, 10, true);
    open(&mut server, A_URI, 1, "@@@@");
    let pulled = workspace_pull(&mut server);
    let items = pulled["result"]["items"][0]["items"].as_array().unwrap();
    assert_eq!(items.len(), 3);
    assert_eq!(items[0]["code"], "L-LEX-0004");
    assert_eq!(items[1]["code"], "L-LEX-0004");
    assert_eq!(items[2]["code"], "L-LSP-0001");
    assert_eq!(items[2]["severity"], 2);
    assert_eq!(
        items[2]["range"],
        json!({
            "end": {"character": 3, "line": 0},
            "start": {"character": 2, "line": 0},
        })
    );
    assert_eq!(
        items[2]["data"]["version"],
        DIAGNOSTIC_CONTROL_PROTOCOL_VERSION
    );
    assert_eq!(
        items[2]["data"]["facts"],
        json!({
            "capped": 2,
            "deduplicated": 0,
            "maximum": 2,
            "omitted": 2,
            "scope": "document",
        })
    );
}

#[test]
fn workspace_cap_uses_uri_order_and_reports_one_global_summary() {
    let mut server = ready(10, 3, true);
    open(&mut server, B_URI, 1, "@@");
    open(&mut server, A_URI, 1, "@@");
    let pulled = workspace_pull(&mut server);
    let reports = pulled["result"]["items"].as_array().unwrap();
    assert_eq!(reports[0]["uri"], A_URI);
    assert_eq!(reports[0]["items"].as_array().unwrap().len(), 2);
    assert_eq!(reports[1]["uri"], B_URI);
    let b = reports[1]["items"].as_array().unwrap();
    assert_eq!(b.len(), 2);
    assert_eq!(b[0]["code"], "L-LEX-0004");
    assert_eq!(b[1]["code"], "L-LSP-0001");
    assert_eq!(b[1]["data"]["facts"]["scope"], "workspace");
    assert_eq!(b[1]["data"]["facts"]["omitted"], 1);
    assert_eq!(b[1]["data"]["facts"]["maximum"], 3);

    let mut reversed = ready(10, 3, true);
    open(&mut reversed, A_URI, 1, "@@");
    open(&mut reversed, B_URI, 1, "@@");
    assert_eq!(workspace_pull(&mut reversed)["result"], pulled["result"]);
}

#[test]
fn controlled_push_and_pull_arrays_match_and_recovery_clears_summary() {
    let mut server = ready(2, 10, true);
    open(&mut server, A_URI, 1, "@@@@");
    let pulled = workspace_pull(&mut server);
    let pull_items = &pulled["result"]["items"][0]["items"];
    assert!(server.diagnostics_pending());
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let notification: Value = serde_json::from_slice(&server.take_notifications()[0]).unwrap();
    assert_eq!(
        serde_json::to_vec(pull_items).unwrap(),
        serde_json::to_vec(&notification["params"]["diagnostics"]).unwrap()
    );

    assert_eq!(
        server.handle_json(&message(
            None,
            "textDocument/didChange",
            json!({
                "contentChanges": [{"text": "module A\n\nlet value = 1\n"}],
                "textDocument": {"uri": A_URI, "version": 2},
            }),
        )),
        HandleOutcome::NoResponse
    );
    assert_eq!(server.flush_pending_diagnostics().unwrap(), 1);
    let cleared: Value = serde_json::from_slice(&server.take_notifications()[0]).unwrap();
    assert_eq!(cleared["params"]["diagnostics"], json!([]));
}
