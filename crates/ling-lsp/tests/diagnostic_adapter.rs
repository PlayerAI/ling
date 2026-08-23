use ling_diagnostics::{Diagnostic, DiagnosticSpan, Repair, Severity, codes};
use ling_lsp::{
    DiagnosticAdapterError, DiagnosticAdapterInput, DiagnosticProjectionError, DiagnosticSource,
    PositionEncoding, RelatedDiagnosticLabel, adapt_diagnostics,
};
use ling_source::{SourceFile, SourceId};
use serde_json::{Value, json};

const MAIN_URI: &str = "ling://workspace/src/Main.ling";
const MAIN_NAME: &str = "src/Main.ling";
const DEPENDENCY_URI: &str = "ling://dependency/math/src/Math.ling";
const DEPENDENCY_NAME: &str = "dependencies/math/src/Math.ling";

fn source(id: u32, uri: &str, name: &str, text: &str) -> DiagnosticSource {
    DiagnosticSource::new(
        uri,
        SourceFile::from_bytes(SourceId::new(id), name, text.as_bytes().to_vec())
            .expect("fixture source is valid UTF-8"),
    )
}

fn diagnostic(
    code: ling_diagnostics::DiagnosticCode,
    severity: Severity,
    file: &str,
    start: u32,
    end: u32,
) -> Diagnostic {
    Diagnostic::new(code, severity, "中文消息", "English message")
        .with_primary_span(DiagnosticSpan::at(file, start, end))
}

#[test]
fn maps_the_complete_public_shape_and_related_sources_exactly() {
    let sources = [
        source(1, MAIN_URI, MAIN_NAME, "\u{feff}a\r\n凌😀e\u{301}\n"),
        source(2, DEPENDENCY_URI, DEPENDENCY_NAME, "x\nβ\n"),
    ];
    let input = DiagnosticAdapterInput::new(
        diagnostic(codes::TYPE_MISMATCH, Severity::Error, MAIN_NAME, 6, 16)
            .with_semantic_id("sid:type:7")
            .with_fact("actual", "Text")
            .with_fact("expected", "Int")
            .with_repair(Repair::new("replace_expression", true).with_fact("replacement", "42"))
            .with_repair(Repair::new("add_annotation", false)),
    )
    .with_related(vec![RelatedDiagnosticLabel::new(
        DiagnosticSpan::at(DEPENDENCY_NAME, 2, 4),
        "定义于此",
        "defined here",
    )]);

    let adapted = adapt_diagnostics(PositionEncoding::Utf16, &sources, &[input])
        .expect("valid compiler diagnostic adapts");
    assert_eq!(adapted.len(), 1);
    assert_eq!(adapted[0].uri(), MAIN_URI);
    assert_eq!(
        adapted[0].value(),
        &json!({
            "code": "L-TYPE-0001",
            "data": {
                "facts": {"actual": "Text", "expected": "Int"},
                "repairs": [
                    {
                        "changes_semantics": true,
                        "facts": {"replacement": "42"},
                        "kind": "replace_expression"
                    },
                    {"changes_semantics": false, "kind": "add_annotation"}
                ],
                "semanticId": "sid:type:7",
                "version": "ling.lsp.diagnostic/0.1"
            },
            "message": "中文消息 / English message",
            "range": {
                "end": {"character": 5, "line": 1},
                "start": {"character": 0, "line": 1}
            },
            "relatedInformation": [{
                "location": {
                    "range": {
                        "end": {"character": 1, "line": 1},
                        "start": {"character": 0, "line": 1}
                    },
                    "uri": DEPENDENCY_URI
                },
                "message": "定义于此 / defined here"
            }],
            "severity": 1,
            "source": "ling"
        })
    );
}

#[test]
fn projects_original_bytes_for_all_encodings_and_maps_all_severities() {
    let sources = [source(
        1,
        MAIN_URI,
        MAIN_NAME,
        "\u{feff}a\r\n凌😀e\u{301}\n",
    )];
    let inputs = [
        DiagnosticAdapterInput::new(diagnostic(
            codes::INVALID_NUMBER,
            Severity::Error,
            MAIN_NAME,
            6,
            16,
        )),
        DiagnosticAdapterInput::new(diagnostic(
            codes::UNREACHABLE_MATCH_CASE,
            Severity::Warning,
            MAIN_NAME,
            6,
            16,
        )),
        DiagnosticAdapterInput::new(diagnostic(
            codes::UNUSED_CAPABILITY,
            Severity::Note,
            MAIN_NAME,
            6,
            16,
        )),
    ];
    let cases = [
        (PositionEncoding::Utf8, 10),
        (PositionEncoding::Utf16, 5),
        (PositionEncoding::Utf32, 4),
    ];

    for (encoding, end_character) in cases {
        let adapted = adapt_diagnostics(encoding, &sources, &inputs).expect("spans project");
        let severities = adapted
            .iter()
            .map(|item| item.value()["severity"].as_u64())
            .collect::<Vec<_>>();
        assert_eq!(severities, vec![Some(3), Some(1), Some(2)]);
        for item in &adapted {
            assert_eq!(
                item.value()["range"]["start"],
                json!({"character": 0, "line": 1})
            );
            assert_eq!(
                item.value()["range"]["end"],
                json!({"character": end_character, "line": 1})
            );
            assert_eq!(item.value()["data"]["semanticId"], Value::Null);
            assert_eq!(item.value()["relatedInformation"], json!([]));
        }
    }
}

#[test]
fn sorts_by_the_accepted_key_and_serializes_repeatably() {
    let sources = [
        source(1, MAIN_URI, MAIN_NAME, "abcdef"),
        source(2, DEPENDENCY_URI, DEPENDENCY_NAME, "abcdef"),
    ];
    let inputs = [
        DiagnosticAdapterInput::new(diagnostic(
            codes::TYPE_MISMATCH,
            Severity::Error,
            MAIN_NAME,
            2,
            5,
        )),
        DiagnosticAdapterInput::new(diagnostic(
            codes::INVALID_NUMBER,
            Severity::Error,
            DEPENDENCY_NAME,
            4,
            5,
        )),
        DiagnosticAdapterInput::new(diagnostic(
            codes::UNEXPECTED_CHARACTER,
            Severity::Error,
            MAIN_NAME,
            2,
            4,
        )),
        DiagnosticAdapterInput::new(diagnostic(
            codes::INVALID_NUMBER,
            Severity::Error,
            MAIN_NAME,
            0,
            1,
        )),
    ];

    let left = adapt_diagnostics(PositionEncoding::Utf8, &sources, &inputs).expect("valid set");
    let right = adapt_diagnostics(PositionEncoding::Utf8, &sources, &inputs).expect("valid set");
    let reversed_inputs = inputs.iter().rev().cloned().collect::<Vec<_>>();
    let reversed = adapt_diagnostics(PositionEncoding::Utf8, &sources, &reversed_inputs)
        .expect("reordered valid set");
    let observed = left
        .iter()
        .map(|item| (item.uri(), item.value()["code"].as_str().unwrap()))
        .collect::<Vec<_>>();
    assert_eq!(
        observed,
        vec![
            (DEPENDENCY_URI, "L-LEX-0011"),
            (MAIN_URI, "L-LEX-0011"),
            (MAIN_URI, "L-LEX-0005"),
            (MAIN_URI, "L-TYPE-0001"),
        ]
    );
    assert_eq!(
        serde_json::to_vec(&left.iter().map(|item| item.value()).collect::<Vec<_>>()).unwrap(),
        serde_json::to_vec(&right.iter().map(|item| item.value()).collect::<Vec<_>>()).unwrap()
    );
    assert_eq!(
        left.iter().map(|item| item.value()).collect::<Vec<_>>(),
        reversed.iter().map(|item| item.value()).collect::<Vec<_>>()
    );
}

#[test]
fn rejects_invalid_source_sets_and_missing_or_unknown_primary_spans() {
    assert_eq!(
        adapt_diagnostics(PositionEncoding::Utf8, &[], &[]),
        Err(DiagnosticAdapterError::EmptySources)
    );

    let main = source(1, MAIN_URI, MAIN_NAME, "abc");
    assert!(matches!(
        adapt_diagnostics(
            PositionEncoding::Utf8,
            &[source(
                1,
                "untitled://ling/src/Main.ling",
                "untitled/src/Main.ling",
                "abc"
            )],
            &[]
        ),
        Err(DiagnosticAdapterError::InvalidSourceUri { .. })
    ));
    assert!(matches!(
        adapt_diagnostics(
            PositionEncoding::Utf8,
            &[source(1, MAIN_URI, "wrong.ling", "abc")],
            &[]
        ),
        Err(DiagnosticAdapterError::SourceIdentityMismatch { .. })
    ));
    assert!(matches!(
        adapt_diagnostics(PositionEncoding::Utf8, &[main.clone(), main.clone()], &[]),
        Err(DiagnosticAdapterError::DuplicateSourceUri { .. })
    ));
    assert!(matches!(
        adapt_diagnostics(
            PositionEncoding::Utf8,
            &[
                source(
                    1,
                    "ling://workspace/dependencies/math/src/Math.ling",
                    DEPENDENCY_NAME,
                    "abc"
                ),
                source(2, DEPENDENCY_URI, DEPENDENCY_NAME, "abc")
            ],
            &[]
        ),
        Err(DiagnosticAdapterError::DuplicateSourceName { .. })
    ));
    assert!(matches!(
        adapt_diagnostics(
            PositionEncoding::Utf8,
            &[
                main.clone(),
                source(2, "ling://workspace/src/Other.ling", MAIN_NAME, "abc")
            ],
            &[]
        ),
        Err(DiagnosticAdapterError::SourceIdentityMismatch { .. })
    ));

    let missing = DiagnosticAdapterInput::new(Diagnostic::new(
        codes::INVALID_NUMBER,
        Severity::Error,
        "错",
        "error",
    ));
    assert!(matches!(
        adapt_diagnostics(
            PositionEncoding::Utf8,
            std::slice::from_ref(&main),
            &[missing]
        ),
        Err(DiagnosticAdapterError::MissingPrimarySpan { .. })
    ));
    let unknown = DiagnosticAdapterInput::new(diagnostic(
        codes::INVALID_NUMBER,
        Severity::Error,
        "missing.ling",
        0,
        1,
    ));
    assert!(matches!(
        adapt_diagnostics(PositionEncoding::Utf8, &[main], &[unknown]),
        Err(DiagnosticAdapterError::UnknownSource { .. })
    ));
}

#[test]
fn rejects_invalid_later_input_atomically_without_clamping() {
    let sources = [source(1, MAIN_URI, MAIN_NAME, "a\r\n凌😀\n")];
    let valid = DiagnosticAdapterInput::new(diagnostic(
        codes::INVALID_NUMBER,
        Severity::Error,
        MAIN_NAME,
        0,
        1,
    ));
    let invalid_spans = [
        DiagnosticSpan::at_u64(MAIN_NAME, 5, 4),
        DiagnosticSpan::at_u64(MAIN_NAME, u64::from(u32::MAX) + 1, u64::from(u32::MAX) + 1),
        DiagnosticSpan::at(MAIN_NAME, 99, 99),
        DiagnosticSpan::at(MAIN_NAME, 4, 5),
        DiagnosticSpan::at(MAIN_NAME, 7, 8),
        DiagnosticSpan::at(MAIN_NAME, 1, 2),
    ];

    for span in invalid_spans {
        let invalid = DiagnosticAdapterInput::new(
            Diagnostic::new(codes::TYPE_MISMATCH, Severity::Error, "错", "error")
                .with_primary_span(span),
        );
        let result =
            adapt_diagnostics(PositionEncoding::Utf16, &sources, &[valid.clone(), invalid]);
        assert!(matches!(
            result,
            Err(DiagnosticAdapterError::Projection(
                DiagnosticProjectionError::ReversedSpan { .. }
                    | DiagnosticProjectionError::OffsetOutOfRange { .. }
                    | DiagnosticProjectionError::Position(_)
            ))
        ));
    }

    let invalid_related = DiagnosticAdapterInput::new(diagnostic(
        codes::INVALID_NUMBER,
        Severity::Error,
        MAIN_NAME,
        0,
        1,
    ))
    .with_related(vec![RelatedDiagnosticLabel::new(
        DiagnosticSpan::at("missing.ling", 0, 1),
        "相关",
        "related",
    )]);
    assert!(matches!(
        adapt_diagnostics(PositionEncoding::Utf8, &sources, &[invalid_related]),
        Err(DiagnosticAdapterError::UnknownSource { .. })
    ));
}
