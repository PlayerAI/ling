use ling_lsp::{
    HandleOutcome, LspServer, MAX_INTERACTIVE_BURST, MAX_NON_BACKGROUND_BURST,
    SCHEDULING_PROTOCOL_VERSION,
};
use serde_json::{Value, json};

fn request(id: u64, method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "id": id,
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    }))
    .expect("request JSON")
}

#[test]
fn initialize_advertises_the_exact_preview_scheduling_contract() {
    let mut server = LspServer::new();
    let HandleOutcome::Response(body) =
        server.handle_json(&request(1, "initialize", json!({"capabilities": {}})))
    else {
        panic!("initialize response");
    };
    let response: Value = serde_json::from_slice(&body).expect("initialize response JSON");

    assert_eq!(
        response["result"]["capabilities"]["experimental"]["lingScheduling"],
        json!({
            "classes": ["interactive", "analysis", "background"],
            "debounce": "message-boundary",
            "maxInteractiveBurst": MAX_INTERACTIVE_BURST,
            "maxNonBackgroundBurst": MAX_NON_BACKGROUND_BURST,
            "requestOrder": "wire-order",
            "supersession": "cancel-stale-analysis",
            "version": SCHEDULING_PROTOCOL_VERSION,
        })
    );
}

#[test]
fn protocol_fixture_matches_discovery_and_fixed_bounds() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../tests/protocols/lsp-scheduling/fixtures/v1.json"
    ))
    .expect("scheduling fixture JSON");

    assert_eq!(fixture["schema"], "ling.test.lsp-scheduling/1");
    assert_eq!(fixture["discovery"]["version"], SCHEDULING_PROTOCOL_VERSION);
    assert_eq!(
        fixture["discovery"]["maxInteractiveBurst"],
        MAX_INTERACTIVE_BURST
    );
    assert_eq!(
        fixture["discovery"]["maxNonBackgroundBurst"],
        MAX_NON_BACKGROUND_BURST
    );
    assert_eq!(fixture["methodClasses"]["workspace/symbol"], "background");
    assert_eq!(
        fixture["methodClasses"]["textDocument/diagnostic"],
        "analysis"
    );
    assert_eq!(
        fixture["methodClasses"]["textDocument/hover"],
        "interactive"
    );
}
