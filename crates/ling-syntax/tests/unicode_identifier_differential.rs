use ling_source::{SourceFile, SourceId};
use ling_syntax::{TokenKind, TokenValue, lex};

const CORPUS: &str =
    include_str!("../../../editors/tree-sitter-ling/test/fixtures/unicode-identifiers.tsv");

#[derive(Debug)]
struct Case {
    id: &'static str,
    spelling: String,
    compiler: &'static str,
    normalized: Option<String>,
    diagnostic_code: Option<&'static str>,
}

#[test]
fn compiler_lexer_matches_the_shared_tree_sitter_identifier_corpus() {
    let cases = parse_cases();
    assert!(
        cases.len() >= 18,
        "the differential corpus unexpectedly shrank"
    );

    for (index, case) in cases.iter().enumerate() {
        let source = SourceFile::from_bytes(
            SourceId::new(u32::try_from(index).expect("small fixture index")),
            format!("{}.ling", case.id),
            case.spelling.as_bytes().to_vec(),
        )
        .expect("fixture spelling is valid UTF-8");
        let lexed = lex(&source);
        let significant = lexed
            .tokens()
            .iter()
            .filter(|token| {
                let kind = token.kind();
                kind != TokenKind::Eof && !kind.is_trivia() && !kind.is_layout()
            })
            .collect::<Vec<_>>();

        assert_eq!(
            significant.len(),
            1,
            "{} must produce one significant compiler token: {significant:?}",
            case.id
        );

        match case.compiler {
            "identifier" => {
                assert!(
                    lexed.errors().is_empty(),
                    "{}: {:#?}",
                    case.id,
                    lexed.errors()
                );
                assert_eq!(significant[0].kind(), TokenKind::Identifier, "{}", case.id);
                let Some(TokenValue::Identifier(identifier)) = significant[0].value() else {
                    panic!("{} identifier token must carry security metadata", case.id);
                };
                assert_eq!(
                    identifier.identifier().normalized(),
                    case.normalized
                        .as_deref()
                        .expect("identifier normalization"),
                    "{}",
                    case.id
                );
            }
            "keyword:and" => {
                assert!(
                    lexed.errors().is_empty(),
                    "{}: {:#?}",
                    case.id,
                    lexed.errors()
                );
                assert_eq!(significant[0].kind(), TokenKind::And, "{}", case.id);
                assert!(case.normalized.is_none(), "keyword has no identifier value");
            }
            "invalid" => {
                assert_eq!(significant[0].kind(), TokenKind::Error, "{}", case.id);
                assert_eq!(
                    lexed.errors().len(),
                    1,
                    "{}: {:#?}",
                    case.id,
                    lexed.errors()
                );
                let diagnostic = lexed.errors()[0].to_diagnostic(source.name());
                assert_eq!(
                    diagnostic.code().as_str(),
                    case.diagnostic_code.expect("invalid case diagnostic code"),
                    "{}",
                    case.id
                );
                let span = diagnostic
                    .primary_span()
                    .expect("public lexical diagnostics carry an original-byte span");
                assert_eq!(span.start_byte(), 0, "{}", case.id);
                assert_eq!(
                    span.end_byte(),
                    u32::try_from(case.spelling.len()).expect("small fixture spelling"),
                    "{}",
                    case.id
                );
                let json = diagnostic
                    .render_json()
                    .expect("diagnostic renders as JSON");
                assert!(json.contains("\"message_zh\""), "{}: {json}", case.id);
                assert!(json.contains("\"message_en\""), "{}: {json}", case.id);
            }
            unexpected => panic!("{} has unknown compiler expectation {unexpected}", case.id),
        }
    }
}

fn parse_cases() -> Vec<Case> {
    CORPUS
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| {
            let fields = line.split('\t').collect::<Vec<_>>();
            assert_eq!(fields.len(), 6, "invalid differential row: {line}");
            Case {
                id: fields[0],
                spelling: decode_codepoints(fields[1]),
                compiler: fields[3],
                normalized: (fields[4] != "-").then(|| decode_codepoints(fields[4])),
                diagnostic_code: (fields[5] != "-").then_some(fields[5]),
            }
        })
        .collect()
}

fn decode_codepoints(input: &str) -> String {
    input
        .split('+')
        .map(|value| {
            let codepoint = u32::from_str_radix(value, 16)
                .unwrap_or_else(|error| panic!("invalid codepoint {value}: {error}"));
            char::from_u32(codepoint)
                .unwrap_or_else(|| panic!("U+{codepoint:04X} is not a Unicode scalar"))
        })
        .collect()
}
