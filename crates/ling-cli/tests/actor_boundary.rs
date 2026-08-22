use ling_cli::{CompileFailure, compile_source};
use ling_diagnostics::codes;

#[test]
fn actor_declaration_is_rejected_before_checked_snapshot_or_execution() {
    let result = compile_source("actor.ling", b"module Main\n\nactor Logger = ()\n".to_vec());
    let Err(CompileFailure::Diagnostics(diagnostics)) = result else {
        panic!("unimplemented Actor syntax must not produce a checked snapshot: {result:?}");
    };
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == codes::UNEXPECTED_TOKEN)
        .expect("Actor syntax rejection diagnostic");
    assert_eq!(diagnostic.code(), codes::UNEXPECTED_TOKEN);
    let span = diagnostic.primary_span().expect("original Actor span");
    assert_eq!(span.file(), "actor.ling");
    assert_eq!(span.start_byte(), 13);
    assert_eq!(span.end_byte(), 18);
    let json = diagnostic.render_json().expect("diagnostic JSON");
    assert!(json.contains("syntax error"));
    assert!(json.contains("语法错误"));
}
