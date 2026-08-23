use ling_lsp::{
    CancellationToken, HandleOutcome, LspServer, MAX_WORKSPACE_SYMBOL_QUERY_BYTES,
    MAX_WORKSPACE_SYMBOLS, WORKSPACE_SYMBOL_PROTOCOL_VERSION,
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

fn notification(method: &str, params: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
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

fn response_with_cancellation(
    server: &mut LspServer,
    id: u64,
    params: Value,
    cancellation: &CancellationToken,
) -> Value {
    let HandleOutcome::Response(bytes) = server
        .handle_json_with_cancellation(&request(id, "workspace/symbol", params), cancellation)
    else {
        panic!("request must respond")
    };
    serde_json::from_slice(&bytes).expect("response JSON")
}

fn ready(encoding: &str) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let initialized = response(
        &mut server,
        1,
        "initialize",
        json!({"capabilities": {"general": {"positionEncodings": [encoding]}}}),
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
                },
            }),
        )),
        HandleOutcome::NoResponse
    );
}

fn change(server: &mut LspServer, uri: &str, version: i64, text: &str) {
    assert_eq!(
        server.handle_json(&notification(
            "textDocument/didChange",
            json!({
                "contentChanges": [{"text": text}],
                "textDocument": {"uri": uri, "version": version},
            }),
        )),
        HandleOutcome::NoResponse
    );
}

fn symbols(server: &mut LspServer, id: u64, query: &str) -> Value {
    response(server, id, "workspace/symbol", json!({"query": query}))
}

#[test]
fn provider_matching_kinds_and_wire_shape_follow_rfc_0045() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "module Main\n\n",
        "type Person = { name: Text; age: Int }\n\n",
        "type Choice =\n",
        "    | First\n",
        "    | Next of Int\n\n",
        "trait Show<'a> =\n",
        "    show: 'a -> Text\n\n",
        "impl Show Person =\n",
        "    let show person = person.name\n\n",
        "let alpha = 1\n",
        "let alphabet = 2\n",
    );
    let (mut server, initialized) = ready("utf-16");
    assert_eq!(
        initialized["result"]["capabilities"]["workspaceSymbolProvider"],
        json!({"resolveProvider": false, "workDoneProgress": false})
    );
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingWorkspaceSymbols"],
        json!({
            "cache": "complete-snapshot",
            "cancellation": "cooperative-host-token",
            "matching": "exact-or-prefix-case-sensitive",
            "maxQueryBytes": MAX_WORKSPACE_SYMBOL_QUERY_BYTES,
            "maxSymbols": MAX_WORKSPACE_SYMBOLS,
            "scope": "tracked-workspace-sources",
            "version": WORKSPACE_SYMBOL_PROTOCOL_VERSION,
        })
    );
    open(&mut server, uri, 1, source);

    let result = symbols(&mut server, 2, "alpha");
    let items = result["result"].as_array().expect("symbol array");
    assert_eq!(
        items
            .iter()
            .map(|item| item["name"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["alpha", "alphabet"]
    );
    for item in items {
        assert_eq!(item.as_object().expect("symbol object").len(), 4);
        assert_eq!(item["containerName"], "Main");
        assert_eq!(item["kind"], 13);
        assert_eq!(item["location"]["uri"], uri);
        assert_eq!(item["location"].as_object().unwrap().len(), 2);
    }
    assert!(
        symbols(&mut server, 3, "Alpha")["result"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(symbols(&mut server, 4, "Person")["result"][0]["kind"], 5);
    assert_eq!(symbols(&mut server, 5, "First")["result"][0]["kind"], 22);
    assert!(
        symbols(&mut server, 6, "show")["result"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item["kind"] == 6)
    );
}

#[test]
fn scope_positions_and_insertion_order_are_deterministic() {
    let main_uri = "ling://workspace/src/Main.ling";
    let support_uri = "ling://workspace/src/Support.ling";
    let auxiliary_uri = "ling://workspace/src/Aux.ling";
    let dependency_uri = "ling://dependency/pkg/src/Dep.ling";
    let temporary_uri = "untitled://ling/Scratch.ling";
    let main = concat!(
        "module Main\n\n",
        "import Support as S\n",
        "import Dep as D\n\n",
        "let answerMain = S.answer\n",
        "let dependencyValue = D.secret\n",
    );
    let support = "\u{feff}module Support\r\n\r\n/*🙂e\u{301}*/ let answer = 42\r\n";
    let auxiliary = "module Aux\n\nlet answerAux = 0\n";
    let dependency = "module Dep\n\nlet secret = 7\n";
    let temporary = "module Scratch\n\nlet answerTemporary = 9\n";

    for (encoding, expected_character) in [("utf-8", 16), ("utf-16", 13), ("utf-32", 12)] {
        let mut insertion_results = Vec::new();
        for reverse in [false, true] {
            let (mut server, _) = ready(encoding);
            server
                .publish_disk_snapshot(dependency_uri, dependency)
                .unwrap();
            server
                .publish_disk_snapshot(auxiliary_uri, auxiliary)
                .unwrap();
            if reverse {
                server.publish_disk_snapshot(support_uri, support).unwrap();
                server.publish_disk_snapshot(main_uri, main).unwrap();
            } else {
                server.publish_disk_snapshot(main_uri, main).unwrap();
                server.publish_disk_snapshot(support_uri, support).unwrap();
            }
            open(&mut server, temporary_uri, 1, temporary);
            let result = symbols(&mut server, 2, "answer")["result"].clone();
            let items = result.as_array().expect("symbol array");
            assert_eq!(items.len(), 3);
            assert_eq!(items[0]["name"], "answer");
            assert_eq!(items[0]["containerName"], "Support");
            assert_eq!(items[0]["location"]["uri"], support_uri);
            assert_eq!(
                items[0]["location"]["range"]["start"],
                json!({"character": expected_character, "line": 2})
            );
            assert_eq!(items[1]["name"], "answerAux");
            assert_eq!(items[1]["containerName"], "Aux");
            assert_eq!(items[1]["location"]["uri"], auxiliary_uri);
            assert_eq!(items[2]["name"], "answerMain");
            insertion_results.push(result);
        }
        assert_eq!(insertion_results[0], insertion_results[1]);
    }
}

#[test]
fn equal_snapshot_repeats_and_document_change_invalidates_the_plan() {
    let uri = "ling://workspace/src/Main.ling";
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, 1, "module Main\n\nlet cached = 1\n");
    let first = symbols(&mut server, 2, "cached")["result"].clone();
    assert_eq!(symbols(&mut server, 3, "cached")["result"], first);

    change(&mut server, uri, 2, "module Main\n\nlet changed = 1\n");
    assert!(
        symbols(&mut server, 4, "cached")["result"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        symbols(&mut server, 5, "changed")["result"][0]["name"],
        "changed"
    );
}

#[test]
fn result_limit_is_stable_and_keeps_the_first_sorted_matches() {
    let uri = "ling://workspace/src/Main.ling";
    let mut source = String::from("module Main\n\n");
    for index in (0..=MAX_WORKSPACE_SYMBOLS).rev() {
        source.push_str(&format!("let item{index:03} = {index}\n"));
    }
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, 1, &source);
    let first = symbols(&mut server, 2, "item")["result"].clone();
    let items = first.as_array().expect("symbol array");
    assert_eq!(items.len(), MAX_WORKSPACE_SYMBOLS);
    assert_eq!(items[0]["name"], "item000");
    assert_eq!(items[MAX_WORKSPACE_SYMBOLS - 1]["name"], "item255");
    assert_eq!(symbols(&mut server, 3, "item")["result"], first);
}

#[test]
fn lifecycle_params_cancellation_and_compiler_failures_are_atomic() {
    let mut uninitialized = LspServer::new();
    assert_eq!(symbols(&mut uninitialized, 1, "")["error"]["code"], -32002);

    let uri = "ling://workspace/src/Main.ling";
    let (mut server, _) = ready("utf-16");
    open(&mut server, uri, 1, "module Main\n\nlet value = 1\n");
    assert_eq!(
        server.handle_json(&notification("workspace/symbol", json!({"query": ""}))),
        HandleOutcome::NoResponse
    );
    for params in [
        json!({}),
        json!({"query": 7}),
        json!({"query": "bad\u{0}query"}),
        json!({"query": "x".repeat(MAX_WORKSPACE_SYMBOL_QUERY_BYTES + 1)}),
    ] {
        assert_eq!(
            response(&mut server, 2, "workspace/symbol", params)["error"]["code"],
            -32602
        );
    }

    let cancellation = CancellationToken::new();
    cancellation.cancel();
    let cancelled = response_with_cancellation(&mut server, 3, json!({"query": ""}), &cancellation);
    assert_eq!(cancelled["error"]["code"], -32800);
    assert!(cancelled.get("result").is_none());
    assert_eq!(
        symbols(&mut server, 4, "value")["result"][0]["name"],
        "value"
    );

    change(&mut server, uri, 2, "module Main\n\nlet value = missing\n");
    let failed = symbols(&mut server, 5, "value");
    assert_eq!(failed["error"]["code"], -32803);
    assert!(failed.get("result").is_none());
    change(&mut server, uri, 3, "module Main\n\nlet value = 2\n");
    assert_eq!(
        symbols(&mut server, 6, "value")["result"][0]["name"],
        "value"
    );

    assert!(response(&mut server, 7, "shutdown", Value::Null)["result"].is_null());
    assert_eq!(symbols(&mut server, 8, "")["error"]["code"], -32003);
}
