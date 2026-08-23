use ling_cli::compile_source;

#[test]
fn checked_handler_reaches_the_public_semantic_and_audit_snapshot() {
    let compiled = compile_source(
        "handler.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"x\" with\n",
            "        operation Console.Write.write(message, resume) -> resume ()\n",
        )
        .as_bytes()
        .to_vec(),
    )
    .expect("checked handler compiles through the snapshot boundary");
    let node = compiled
        .snapshot
        .graph()
        .nodes
        .iter()
        .find(|node| node.name.as_deref() == Some("handler"))
        .expect("handler expression node");
    let audit = compiled.snapshot.audit_model();
    let handler = audit
        .modules
        .iter()
        .find(|module| module.name == "Main")
        .expect("Main Audit module")
        .handlers
        .first()
        .expect("handler Audit projection");
    assert_eq!(handler.handler_id, node.node_id);
    assert_eq!(handler.input_row, "{Console.Write}");
    assert_eq!(handler.eliminated_effects, ["Console.Write"]);
    assert_eq!(handler.residual_row, "{}");
    assert_eq!(
        handler.clauses[0].operation,
        "Console.Write::write(Text)->Unit::Once"
    );
    let rendered = ling_format::render_audit(&audit).expect("handler Audit renders");
    assert!(rendered.starts_with("audit ling.audit/0.2 {\n"));
    assert_eq!(
        ling_format::parse_audit(&rendered).expect("handler Audit parses"),
        audit
    );
}

#[test]
fn handler_audit_preserves_unicode_crlf_and_bom_byte_spans() {
    let source = concat!(
        "\u{feff}module Main\r\n",
        "    requires Console.Write\r\n\r\n",
        "let main () =\r\n",
        "    handle Console.write \"你好\" with\r\n",
        "        operation Console.Write.write(消息, resume) -> resume ()\r\n",
    );
    let compiled = compile_source("handler.ling", source.as_bytes().to_vec())
        .expect("Unicode CRLF/BOM Handler compiles");
    let audit = compiled.snapshot.audit_model();
    let handler = audit
        .modules
        .iter()
        .find(|module| module.name == "Main")
        .and_then(|module| module.handlers.first())
        .expect("handler Audit projection");
    assert_eq!(
        usize::try_from(handler.source_start).expect("span fits usize"),
        source.find("handle").expect("handler source token")
    );
    assert!(handler.source_end > handler.source_start);

    let rendered = ling_format::render_audit(&audit).expect("handler Audit renders");
    assert!(!rendered.starts_with('\u{feff}'));
    assert!(!rendered.contains('\r'));
    assert_eq!(
        ling_format::parse_audit(&rendered).expect("handler Audit parses"),
        audit
    );
}
