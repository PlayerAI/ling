use ling_lsp::{
    CODE_ACTION_PROTOCOL_VERSION, FORMAT_ACTION_KIND, HandleOutcome, JSON_RPC_VERSION, LspServer,
};
use serde_json::{Value, json};

fn request(id: u64, method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "id": id,
        "jsonrpc": JSON_RPC_VERSION,
        "method": method,
        "params": params,
    }))
    .expect("request JSON")
}

fn notification(method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": JSON_RPC_VERSION,
        "method": method,
        "params": params,
    }))
    .expect("notification JSON")
}

fn response(server: &mut LspServer, id: u64, method: &str, params: Value) -> Value {
    let HandleOutcome::Response(bytes) = server.handle_json(&request(id, method, params)) else {
        panic!("request must respond")
    };
    serde_json::from_slice(&bytes).expect("response JSON")
}

fn initialize(capable: bool, encoding: &str) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let text_document = if capable {
        json!({
            "codeAction": {
                "codeActionLiteralSupport": {
                    "codeActionKind": {"valueSet": [FORMAT_ACTION_KIND]}
                },
                "dynamicRegistration": false,
            }
        })
    } else {
        json!({})
    };
    let initialized = response(
        &mut server,
        1,
        "initialize",
        json!({
            "capabilities": {
                "general": {"positionEncodings": [encoding]},
                "textDocument": text_document,
                "workspace": {
                    "workspaceEdit": {
                        "documentChanges": true,
                        "failureHandling": "transactional",
                    }
                },
            }
        }),
    );
    assert_eq!(
        server.handle_json(&notification("initialized", json!({}))),
        HandleOutcome::NoResponse
    );
    (server, initialized)
}

fn open(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&notification(
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

fn params(uri: &str, only: Option<Value>, diagnostics: Value) -> Value {
    let mut context = json!({"diagnostics": diagnostics});
    if let Some(only) = only {
        context["only"] = only;
    }
    json!({
        "context": context,
        "range": {
            "end": {"character": 0, "line": 0},
            "start": {"character": 0, "line": 0},
        },
        "textDocument": {"uri": uri},
    })
}

#[test]
fn initialize_negotiates_exact_bounded_contract() {
    assert_eq!(CODE_ACTION_PROTOCOL_VERSION, "ling.lsp.code-action/0.1");
    let (_, initialized) = initialize(true, "utf-16");
    assert_eq!(
        initialized["result"]["capabilities"]["codeActionProvider"],
        json!({
            "codeActionKinds": [FORMAT_ACTION_KIND],
            "resolveProvider": false,
        })
    );
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingCodeAction"],
        json!({
            "actionKinds": [FORMAT_ACTION_KIND],
            "maxActions": 1,
            "result": "versionedDocumentChanges",
            "source": "compiler-cst-format-plan",
            "transactional": true,
            "version": CODE_ACTION_PROTOCOL_VERSION,
        })
    );

    let (_, incapable) = initialize(false, "utf-16");
    assert!(incapable["result"]["capabilities"]["codeActionProvider"].is_null());
    assert!(incapable["result"]["capabilities"]["experimental"]["lingCodeAction"].is_null());

    let mut no_transaction = LspServer::new();
    let no_transaction = response(
        &mut no_transaction,
        1,
        "initialize",
        json!({
            "capabilities": {
                "textDocument": {
                    "codeAction": {
                        "codeActionLiteralSupport": {
                            "codeActionKind": {"valueSet": [FORMAT_ACTION_KIND]}
                        }
                    }
                }
            }
        }),
    );
    assert!(no_transaction["result"]["capabilities"]["codeActionProvider"].is_null());
}

#[test]
fn malformed_capabilities_fail_without_partial_initialization() {
    for code_action in [
        json!(true),
        json!({"dynamicRegistration": "no"}),
        json!({"codeActionLiteralSupport": true}),
        json!({"codeActionLiteralSupport": {"codeActionKind": {"valueSet": []}}}),
        json!({"codeActionLiteralSupport": {"codeActionKind": {"valueSet": [7]}}}),
    ] {
        let mut server = LspServer::new();
        let failure = response(
            &mut server,
            1,
            "initialize",
            json!({"capabilities": {"textDocument": {"codeAction": code_action}}}),
        );
        assert_eq!(failure["error"]["code"], -32_602);
    }
}

#[test]
fn formatter_action_is_exact_versioned_deterministic_and_non_mutating() {
    let cases = [("utf-8", 17), ("utf-16", 11), ("utf-32", 10)];
    for (index, (encoding, end_character)) in cases.into_iter().enumerate() {
        let (mut server, _) = initialize(true, encoding);
        let uri = format!("ling://workspace/src/Action{index}.ling");
        let source = "let 中文=\"😀\"";
        open(&mut server, &uri, 7, source);
        let action = response(
            &mut server,
            2,
            "textDocument/codeAction",
            params(&uri, None, json!([])),
        );
        assert_eq!(
            action["result"],
            json!([{
                "edit": {
                    "documentChanges": [{
                        "edits": [{
                            "newText": "let 中文 = \"😀\"\n",
                            "range": {
                                "end": {"character": end_character, "line": 0},
                                "start": {"character": 0, "line": 0},
                            },
                        }],
                        "textDocument": {"uri": uri, "version": 7},
                    }],
                },
                "isPreferred": true,
                "kind": FORMAT_ACTION_KIND,
                "title": "格式化文档 / Format document",
            }])
        );
        let repeated = response(
            &mut server,
            3,
            "textDocument/codeAction",
            params(&uri, None, json!([])),
        );
        assert_eq!(repeated["result"], action["result"]);
        assert_eq!(server.document(&uri).unwrap().text(), source);
        assert_eq!(server.document(&uri).unwrap().version(), 7);
    }
}

#[test]
fn bom_crlf_and_latest_version_are_projected_from_original_bytes() {
    let (mut server, _) = initialize(true, "utf-16");
    let uri = "ling://workspace/src/Bom.ling";
    open(&mut server, uri, 4, "\u{feff}let value=1\r\n");
    assert_eq!(
        server.handle_json(&notification(
            "textDocument/didChange",
            json!({
                "contentChanges": [{"text": "\u{feff}let value=2\r\n"}],
                "textDocument": {"uri": uri, "version": 5},
            }),
        )),
        HandleOutcome::NoResponse
    );
    let action = response(
        &mut server,
        2,
        "textDocument/codeAction",
        params(uri, None, json!([])),
    );
    let edit = &action["result"][0]["edit"]["documentChanges"][0];
    assert_eq!(edit["textDocument"], json!({"uri": uri, "version": 5}));
    assert_eq!(edit["edits"][0]["newText"], "let value = 2\n");
    assert_eq!(
        edit["edits"][0]["range"],
        json!({
            "end": {"character": 0, "line": 1},
            "start": {"character": 0, "line": 0},
        })
    );
    assert!(server.document(uri).unwrap().text().starts_with('\u{feff}'));
}

#[test]
fn kind_filters_and_opaque_diagnostics_cannot_change_the_plan() {
    let (mut server, _) = initialize(true, "utf-16");
    let uri = "untitled://ling/Opaque.ling";
    open(&mut server, uri, 9, "let value=1\n");
    let hostile_diagnostic = json!([{
        "code": "L-TYPE-0001",
        "data": {
            "repairs": [{
                "facts": {"replacement": "malicious"},
                "kind": "replace_everything"
            }]
        },
        "message": "parse me",
        "range": false,
    }]);
    let action = response(
        &mut server,
        2,
        "textDocument/codeAction",
        params(uri, Some(json!(["source"])), hostile_diagnostic),
    );
    assert_eq!(
        action["result"][0]["edit"]["documentChanges"][0]["edits"][0]["newText"],
        "let value = 1\n"
    );
    for only in ["quickfix", "refactor", "source.organizeImports"] {
        let filtered = response(
            &mut server,
            3,
            "textDocument/codeAction",
            params(uri, Some(json!([only])), json!([])),
        );
        assert_eq!(filtered["result"], json!([]));
    }
}

#[test]
fn empty_invalid_unavailable_and_notification_cases_are_exact() {
    let (mut server, _) = initialize(true, "utf-16");
    let formatted_uri = "ling://workspace/src/Formatted.ling";
    let invalid_uri = "ling://workspace/src/Invalid.ling";
    let dependency_uri = "ling://dependency/core/src/Prelude.ling";
    open(&mut server, formatted_uri, 1, "let value = 1\n");
    open(&mut server, invalid_uri, 1, "let value=\"unterminated\n");
    open(&mut server, dependency_uri, 1, "let value=1\n");
    for uri in [formatted_uri, invalid_uri] {
        assert_eq!(
            response(
                &mut server,
                2,
                "textDocument/codeAction",
                params(uri, None, json!([])),
            )["result"],
            json!([])
        );
    }
    assert_eq!(
        response(
            &mut server,
            3,
            "textDocument/codeAction",
            params(dependency_uri, None, json!([])),
        )["error"]["code"],
        -32_803
    );
    assert_eq!(
        response(
            &mut server,
            4,
            "textDocument/codeAction",
            params("ling://workspace/src/Missing.ling", None, json!([])),
        )["error"]["code"],
        -32_602
    );
    let malformed = params(formatted_uri, None, json!([]));
    for malformed in [
        json!(null),
        json!({"textDocument": {"uri": formatted_uri}}),
        {
            let mut value = malformed.clone();
            value["context"]["diagnostics"] = json!(false);
            value
        },
        {
            let mut value = malformed.clone();
            value["range"]["end"]["character"] = json!(-1);
            value
        },
        {
            let mut value = malformed.clone();
            value["range"]["start"]["character"] = json!(1);
            value
        },
        {
            let mut value = malformed.clone();
            value["range"]["end"]["character"] = json!(999);
            value
        },
        {
            let mut value = malformed.clone();
            value["context"]["only"] = json!([]);
            value
        },
        {
            let mut value = malformed.clone();
            value["context"]["triggerKind"] = json!(3);
            value
        },
    ] {
        assert_eq!(
            response(&mut server, 5, "textDocument/codeAction", malformed,)["error"]["code"],
            -32_602
        );
    }
    assert_eq!(
        server.handle_json(&notification(
            "textDocument/codeAction",
            params(formatted_uri, None, json!([])),
        )),
        HandleOutcome::NoResponse
    );

    let (mut incapable, _) = initialize(false, "utf-16");
    open(&mut incapable, formatted_uri, 1, "let value=1\n");
    assert_eq!(
        response(
            &mut incapable,
            6,
            "textDocument/codeAction",
            params(formatted_uri, None, json!([])),
        )["error"]["code"],
        -32_803
    );
}
