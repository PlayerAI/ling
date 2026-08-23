use ling_lsp::{
    DOCUMENT_SYMBOL_PROTOCOL_VERSION, HandleOutcome, JSON_RPC_VERSION, LspServer,
    MAX_DOCUMENT_SYMBOLS,
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

fn ready(hierarchical: Option<bool>) -> (LspServer, Value) {
    ready_with_encoding(hierarchical, "utf-16")
}

fn ready_with_encoding(hierarchical: Option<bool>, encoding: &str) -> (LspServer, Value) {
    let mut server = LspServer::new();
    let document_symbol = hierarchical.map_or_else(
        || json!({}),
        |value| json!({"hierarchicalDocumentSymbolSupport": value}),
    );
    let initialized = response(
        &mut server,
        1,
        "initialize",
        json!({
            "capabilities": {
                "general": {"positionEncodings": [encoding]},
                "textDocument": {"documentSymbol": document_symbol},
            },
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
                },
            }),
        )),
        HandleOutcome::NoResponse
    );
}

fn symbols(server: &mut LspServer, id: u64, uri: &str) -> Value {
    response(
        server,
        id,
        "textDocument/documentSymbol",
        json!({"textDocument": {"uri": uri}}),
    )
}

fn child<'a>(node: &'a Value, name: &str) -> &'a Value {
    node["children"]
        .as_array()
        .expect("children array")
        .iter()
        .find(|child| child["name"] == name)
        .unwrap_or_else(|| panic!("missing child {name}"))
}

#[test]
fn hierarchical_symbols_cover_every_seed_structure_and_original_ranges() {
    let uri = "ling://workspace/src/Main.ling";
    let source = concat!(
        "\u{feff}module 主程序\r\n\r\n",
        "type 人物 = { 名称: Text; 年龄: Int }\r\n\r\n",
        "type 选择 =\r\n",
        "    | 首项\r\n",
        "    | 次项 of Int\r\n\r\n",
        "type 标识 = Int\r\n\r\n",
        "trait 显示<'a> =\r\n",
        "    显示值: 'a -> Text\r\n\r\n",
        "impl 显示 人物 =\r\n",
        "    let 显示值 person = person.名称\r\n\r\n",
        "let 计算 value = value\r\n",
        "let 常量 = 1\r\n",
        "let 图标 = \"🙂\"\r\n",
    );
    let (mut server, initialized) = ready(Some(true));
    assert_eq!(
        initialized["result"]["capabilities"]["documentSymbolProvider"],
        true
    );
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingDocumentSymbols"],
        json!({
            "maxSymbols": MAX_DOCUMENT_SYMBOLS,
            "mode": "hierarchical",
            "version": DOCUMENT_SYMBOL_PROTOCOL_VERSION,
        })
    );
    open(&mut server, uri, 1, source);

    let result = symbols(&mut server, 2, uri);
    let roots = result["result"]
        .as_array()
        .unwrap_or_else(|| panic!("symbol array: {result}"));
    assert_eq!(roots.len(), 1);
    let module = &roots[0];
    assert_eq!(module["name"], "主程序");
    assert_eq!(module["kind"], 2);
    assert_ne!(module["range"], module["selectionRange"]);

    let record = child(module, "人物");
    assert_eq!(record["kind"], 23);
    assert_eq!(child(record, "名称")["kind"], 8);
    assert_eq!(child(record, "年龄")["kind"], 8);
    let variant = child(module, "选择");
    assert_eq!(variant["kind"], 10);
    assert_eq!(child(variant, "首项")["kind"], 22);
    assert_eq!(child(variant, "次项")["kind"], 22);
    assert_eq!(child(module, "标识")["kind"], 5);
    let interface = child(module, "显示");
    assert_eq!(interface["kind"], 11);
    assert_eq!(child(interface, "显示值")["kind"], 6);
    let implementation = child(module, "impl 显示 人物");
    assert_eq!(implementation["kind"], 19);
    assert_eq!(child(implementation, "显示值")["kind"], 6);
    assert_eq!(child(module, "计算")["kind"], 12);
    assert_eq!(child(module, "常量")["kind"], 14);
    assert_eq!(child(module, "图标")["kind"], 14);

    let starts = module["children"]
        .as_array()
        .unwrap()
        .iter()
        .map(|symbol| {
            (
                symbol["range"]["start"]["line"].as_u64().unwrap(),
                symbol["range"]["start"]["character"].as_u64().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    assert!(starts.windows(2).all(|pair| pair[0] <= pair[1]));
    assert_eq!(symbols(&mut server, 3, uri)["result"], result["result"]);
}

#[test]
fn flat_fallback_is_preorder_with_exact_uri_and_containers() {
    let uri = "ling://workspace/src/Main.ling";
    let (mut server, initialized) = ready(None);
    assert_eq!(
        initialized["result"]["capabilities"]["experimental"]["lingDocumentSymbols"]["mode"],
        "flat"
    );
    open(
        &mut server,
        uri,
        1,
        "module Main\n\ntype Point = { x: Int; y: Int }\n\nlet make value = value\n",
    );
    let result = symbols(&mut server, 2, uri);
    let items = result["result"].as_array().expect("flat symbols");
    let names = items
        .iter()
        .map(|item| item["name"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(names, ["Main", "Point", "x", "y", "make"]);
    assert!(items.iter().all(|item| item.get("children").is_none()));
    assert!(
        items
            .iter()
            .all(|item| item.get("selectionRange").is_none())
    );
    assert!(items.iter().all(|item| item["location"]["uri"] == uri));
    assert!(items[0].get("containerName").is_none());
    assert_eq!(items[1]["containerName"], "Main");
    assert_eq!(items[2]["containerName"], "Point");
}

#[test]
fn request_validation_resolution_failure_and_recovery_are_atomic() {
    let uri = "ling://workspace/src/Main.ling";
    let (mut server, _) = ready(Some(true));
    open(&mut server, uri, 1, "module Main\n\nlet value = missing\n");

    let invalid = response(
        &mut server,
        2,
        "textDocument/documentSymbol",
        json!({"textDocument": {"uri": 7}}),
    );
    assert_eq!(invalid["error"]["code"], -32602);
    let unknown = symbols(&mut server, 3, "ling://workspace/src/Missing.ling");
    assert_eq!(unknown["error"]["code"], -32602);
    let unresolved = symbols(&mut server, 4, uri);
    assert_eq!(unresolved["error"]["code"], -32803);
    assert_eq!(
        server.handle_json(&notification(
            "textDocument/documentSymbol",
            json!({"textDocument": {"uri": uri}}),
        )),
        HandleOutcome::NoResponse
    );

    assert_eq!(
        server.handle_json(&notification(
            "textDocument/didChange",
            json!({
                "contentChanges": [{"text": "module Main\n\nlet value = 1\n"}],
                "textDocument": {"uri": uri, "version": 2},
            }),
        )),
        HandleOutcome::NoResponse
    );
    assert_eq!(symbols(&mut server, 5, uri)["result"][0]["name"], "Main");
}

#[test]
fn malformed_capability_and_symbol_limit_fail_without_partial_success() {
    for capability in [
        json!(true),
        json!({"hierarchicalDocumentSymbolSupport": "yes"}),
    ] {
        let mut server = LspServer::new();
        let result = response(
            &mut server,
            1,
            "initialize",
            json!({"capabilities": {"textDocument": {"documentSymbol": capability}}}),
        );
        assert_eq!(result["error"]["code"], -32602);
    }

    let uri = "ling://workspace/src/Main.ling";
    let mut source = String::from("module Main\n\n");
    for index in 0..MAX_DOCUMENT_SYMBOLS {
        source.push_str(&format!("let value{index} = {index}\n"));
    }
    let (mut server, _) = ready(Some(true));
    open(&mut server, uri, 1, &source);
    let result = symbols(&mut server, 2, uri);
    assert_eq!(result["error"]["code"], -32803);
    assert!(result.get("result").is_none());
}

#[test]
fn temporary_document_is_resolved_in_isolation() {
    let uri = "untitled://ling/scratch/Main.ling";
    let (mut server, _) = ready(Some(true));
    open(
        &mut server,
        "ling://workspace/src/Broken.ling",
        1,
        "module Scratch\n\nlet broken = missing\n",
    );
    open(
        &mut server,
        uri,
        1,
        "module Scratch\n\nlet 临时 value = value\n",
    );
    let result = symbols(&mut server, 2, uri);
    assert_eq!(result["result"][0]["name"], "Scratch");
    assert_eq!(result["result"][0]["children"][0]["name"], "临时");
}

#[test]
fn selection_ranges_follow_the_negotiated_position_encoding() {
    let uri = "ling://workspace/src/Main.ling";
    let source = "module Main\n\n/*🙂e\u{301}*/ let value = 1\n";
    for (encoding, expected_character) in [("utf-8", 16), ("utf-16", 13), ("utf-32", 12)] {
        let (mut server, _) = ready_with_encoding(Some(true), encoding);
        open(&mut server, uri, 1, source);
        let result = symbols(&mut server, 2, uri);
        assert_eq!(
            result["result"][0]["children"][0]["selectionRange"]["start"],
            json!({"character": expected_character, "line": 2}),
            "encoding {encoding}: {result}"
        );
    }
}

#[test]
fn dependency_resolution_is_invariant_to_document_insertion_order() {
    let main_uri = "ling://workspace/src/Main.ling";
    let support_uri = "ling://workspace/src/Support.ling";
    let main = "module Main\n\nimport Support as S\n\nlet answer = S.answer\n";
    let support = "module Support\n\nlet answer = 42\n";

    let mut results = Vec::new();
    for reverse in [false, true] {
        let (mut server, _) = ready(Some(true));
        if reverse {
            open(&mut server, support_uri, 1, support);
            open(&mut server, main_uri, 1, main);
        } else {
            open(&mut server, main_uri, 1, main);
            open(&mut server, support_uri, 1, support);
        }
        results.push(symbols(&mut server, 2, main_uri)["result"].clone());
    }

    assert_eq!(results[0], results[1]);
    assert_eq!(results[0][0]["name"], "Main");
    assert_eq!(results[0][0]["children"][0]["name"], "answer");
}
