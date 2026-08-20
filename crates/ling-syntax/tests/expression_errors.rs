use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;

#[test]
fn boolean_operators_require_both_operands() {
    for (index, text) in [
        "let value = true &&\n",
        "let value = && true\n",
        "let value = false ||\n",
        "let value = || false\n",
    ]
    .into_iter()
    .enumerate()
    {
        let source = SourceFile::from_bytes(
            SourceId::new(u32::try_from(index).expect("four cases fit u32")),
            format!("missing-boolean-operand-{index}.ling"),
            text.as_bytes().to_vec(),
        )
        .expect("source is valid UTF-8");
        let parsed = parse(&source);
        assert!(!parsed.is_valid(), "source unexpectedly parsed: {text:?}");
        assert!(!parsed.parse_errors().is_empty(), "expected a parser error");
    }
}

#[test]
fn reserved_and_is_not_a_boolean_operator_alias() {
    let source = SourceFile::from_bytes(
        SourceId::new(0),
        "and-alias.ling",
        b"let value = true and false\n".to_vec(),
    )
    .expect("source is valid UTF-8");

    assert!(!parse(&source).is_valid());
}

#[test]
fn missing_boolean_operand_uses_the_original_utf8_eof_span() {
    let text = "let 结果 = true &&\n";
    let source = SourceFile::from_bytes(
        SourceId::new(0),
        "utf8-missing-operand.ling",
        text.as_bytes().to_vec(),
    )
    .expect("source is valid UTF-8");

    let parsed = parse(&source);
    assert!(!parsed.is_valid());
    let expected = u32::try_from(text.len()).expect("test source length fits u32");
    assert!(parsed.parse_errors().iter().all(
        |error| error.span().start().get() == expected && error.span().end().get() == expected
    ));
}
