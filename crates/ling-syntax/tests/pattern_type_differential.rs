use std::fs;
use std::path::Path;

use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;

const FIXTURE_PATH: &str = "../../editors/tree-sitter-ling/test/fixtures/pattern-types.tsv";

#[test]
fn compiler_matches_the_shared_pattern_and_type_syntax_corpus() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
    let fixture = fs::read_to_string(&fixture_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", fixture_path.display()));
    let mut case_count = 0;

    for (index, line) in fixture.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let [case, expectation, source] = line
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("invalid fixture row {}: {line}", index + 1));
        let source = SourceFile::from_bytes(
            SourceId::new(u32::try_from(index).expect("fixture row count fits u32")),
            format!("{case}.ling"),
            format!("{source}\n").into_bytes(),
        )
        .expect("fixture source is valid UTF-8");
        let parsed = parse(&source);
        let expected_valid = match expectation {
            "valid" => true,
            "invalid" => false,
            other => panic!("unknown expectation {other:?} in case {case}"),
        };
        assert_eq!(
            parsed.is_valid(),
            expected_valid,
            "{case}: lexical={:?}, parse={:?}",
            parsed.lexical_errors(),
            parsed.parse_errors()
        );
        case_count += 1;
    }

    assert_eq!(case_count, 41, "the TS-3106 corpus changed unexpectedly");
}

#[test]
fn rejected_pattern_delimiters_keep_registered_diagnostics_at_original_utf8_spans() {
    for (index, (case, text, rejected_delimiter)) in [
        (
            "singleton tuple",
            "let 解包 value = match value with | (项目,) -> 项目\n",
            ')',
        ),
        (
            "empty record",
            "let 检查 value = match value with | {} -> 0\n",
            '}',
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let source = SourceFile::from_bytes(
            SourceId::new(u32::try_from(index).expect("two cases fit u32")),
            format!("{case}.ling"),
            text.as_bytes().to_vec(),
        )
        .expect("fixture source is valid UTF-8");
        let parsed = parse(&source);
        assert_eq!(parsed.parse_errors().len(), 1, "{case}");

        let diagnostic = parsed.parse_errors()[0].to_diagnostic(source.name());
        assert_eq!(diagnostic.code().as_str(), "L-SYNTAX-0010", "{case}");
        let span = diagnostic
            .primary_span()
            .expect("public parser diagnostics carry an original-byte span");
        let expected_start = u32::try_from(
            text.find(rejected_delimiter)
                .expect("fixture contains its rejected delimiter"),
        )
        .expect("small fixture byte offset fits u32");
        assert_eq!(span.start_byte(), u64::from(expected_start), "{case}");
        assert_eq!(span.end_byte(), u64::from(expected_start + 1), "{case}");

        let json = diagnostic
            .render_json()
            .expect("diagnostic renders as JSON");
        assert!(json.contains("\"message_zh\""), "{case}: {json}");
        assert!(json.contains("\"message_en\""), "{case}: {json}");
    }
}
