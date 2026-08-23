use ling_semantic::{SemanticReadErrorKind, read_json, read_project_json};

const VALID_SEED: &str =
    include_str!("../../../fuzz/corpus/semantic_schema_bytes/minimal-valid-seed.json");
const MALFORMED: &str = include_str!("../../../fuzz/corpus/semantic_schema_bytes/malformed.json");

#[test]
fn semantic_fuzz_corpus_covers_success_and_deterministic_failure() {
    let graph = read_json(VALID_SEED).expect("the Seed Semantic Graph corpus entry is valid");
    assert_eq!(graph.schema, "ling.semantic/0.1");
    assert_eq!(graph.entry_module, "Main");

    let wrong_reader = read_project_json(VALID_SEED)
        .expect_err("the project reader must not guess the Seed schema");
    let repeated_wrong_reader = read_project_json(VALID_SEED)
        .expect_err("the project reader must reject the Seed schema repeatedly");
    assert_eq!(wrong_reader, repeated_wrong_reader);

    let first = read_json(MALFORMED).expect_err("malformed JSON must be rejected");
    let second = read_json(MALFORMED).expect_err("malformed JSON must be rejected repeatedly");
    assert_eq!(first, second);
    assert!(matches!(
        first.kind,
        SemanticReadErrorKind::InvalidJson { .. }
    ));
}
