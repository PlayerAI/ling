use ling_lsp::{HandleOutcome, JSON_RPC_VERSION, LifecycleState, LspServer, PositionEncoding};
use serde_json::{Value, json};

fn request(id: u64, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": JSON_RPC_VERSION,
        "id": id,
        "method": "initialize",
        "params": params,
    }))
    .expect("initialize request is serializable")
}

fn response(outcome: HandleOutcome) -> Value {
    let HandleOutcome::Response(bytes) = outcome else {
        panic!("initialize must return a response")
    };
    serde_json::from_slice(&bytes).expect("initialize response is JSON")
}

#[test]
fn negotiation_selects_the_first_supported_client_label() {
    let mut server = LspServer::new();
    let response = response(server.handle_json(&request(
        1,
        json!({
            "capabilities": {
                "general": {
                    "positionEncodings": ["unknown", "utf-32", "utf-8"]
                }
            }
        }),
    )));

    assert_eq!(server.position_encoding(), PositionEncoding::Utf32);
    assert_eq!(server.state(), LifecycleState::AwaitingInitialized);
    assert_eq!(
        response["result"]["capabilities"]["positionEncoding"],
        "utf-32"
    );
}

#[test]
fn negotiation_uses_utf16_for_absent_or_empty_client_lists() {
    for capabilities in [json!({}), json!({"general": {"positionEncodings": []}})] {
        let mut server = LspServer::new();
        let response =
            response(server.handle_json(&request(1, json!({"capabilities": capabilities}))));

        assert_eq!(server.position_encoding(), PositionEncoding::Utf16);
        assert_eq!(
            response["result"]["capabilities"]["positionEncoding"],
            "utf-16"
        );
    }
}

#[test]
fn malformed_position_encoding_metadata_is_rejected_before_lifecycle_transition() {
    let mut server = LspServer::new();
    let response = response(server.handle_json(&request(
        1,
        json!({
            "capabilities": {
                "general": {
                    "positionEncodings": ["utf-8", 16]
                }
            }
        }),
    )));

    assert_eq!(response["error"]["code"], -32602);
    assert_eq!(server.state(), LifecycleState::Uninitialized);
    assert_eq!(server.position_encoding(), PositionEncoding::Utf16);
}
