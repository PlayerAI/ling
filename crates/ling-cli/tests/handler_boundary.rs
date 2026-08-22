use ling_cli::{CompileFailure, compile_source};
use ling_diagnostics::codes;

#[test]
fn unresolved_handler_is_rejected_before_checked_snapshot_or_execution() {
    let result = compile_source(
        "handler.ling",
        b"let value =\n    handle value with\n        operation Clock.now() -> 1\n".to_vec(),
    );
    let Err(CompileFailure::Diagnostics(diagnostics)) = result else {
        panic!("unresolved handler must not produce a checked snapshot: {result:?}");
    };
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == codes::UNSUPPORTED_HANDLER)
        .expect("handler rejection diagnostic");
    assert_eq!(diagnostic.code(), codes::UNSUPPORTED_HANDLER);
    let json = diagnostic.render_json().expect("diagnostic JSON");
    assert!(json.contains("handler does not yet have checked semantics"));
    assert!(diagnostic.primary_span().is_some());
}
