use std::sync::Arc;

use ling_db::{
    CompilerDb, SEMANTIC_TOKEN_GENERATION_VERSION, SemanticTokenEntry, SemanticTokenEvidence,
    SemanticTokenGenerationMode, SemanticTokenKind, SemanticTokenModifier,
};
use ling_source::{ChangeEvent, SourceId};

fn source_file(event: ChangeEvent) -> SourceId {
    match event {
        ChangeEvent::Added { file, .. }
        | ChangeEvent::Changed { file, .. }
        | ChangeEvent::Unchanged { file, .. } => file,
    }
}

fn set_source(db: &mut CompilerDb, name: &str, source: &str) -> SourceId {
    source_file(
        db.set_disk_snapshot(name, source.as_bytes().to_vec())
            .expect("fixture source is accepted"),
    )
}

fn spelling<'source>(entry: &SemanticTokenEntry, source: &'source str) -> &'source str {
    let start = usize::try_from(entry.span().start().get()).expect("span start fits");
    let end = usize::try_from(entry.span().end().get()).expect("span end fits");
    &source[start..end]
}

fn entries_with<'index>(
    entries: &'index [SemanticTokenEntry],
    source: &str,
    text: &str,
) -> Vec<&'index SemanticTokenEntry> {
    entries
        .iter()
        .filter(|entry| spelling(entry, source) == text)
        .collect()
}

#[test]
fn typed_generation_classifies_seed_roles_and_reuses_exact_snapshot() {
    let source = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "trait Renderable<'a> =\n",
        "    render: 'a -> Text\n\n",
        "type Item = { name: Text }\n\n",
        "impl Renderable Item =\n",
        "    let render item = item.name\n\n",
        "let main () = Console.write (Renderable.render { name = \"Ling\" })\n",
    );
    let mut db = CompilerDb::new();
    let file = set_source(&mut db, "fixtures/Main.ling", source);

    let first = db
        .semantic_token_index(file)
        .expect("checked semantic tokens build");
    let repeated = db
        .semantic_token_index(file)
        .expect("equal snapshot semantic tokens build");

    assert_eq!(
        SEMANTIC_TOKEN_GENERATION_VERSION,
        "ling.semantic-token-generation/0.1"
    );
    assert!(Arc::ptr_eq(&first, &repeated));
    assert_eq!(first.mode(), SemanticTokenGenerationMode::Typed);
    assert_eq!(first.source(), file);
    assert_eq!(first.revision().get(), 1);
    assert_eq!(first.source_name(), "fixtures/Main.ling");
    assert!(first.entries().windows(2).all(|pair| {
        pair[0].span().end() <= pair[1].span().start()
            && pair[0].span().source() == pair[1].span().source()
    }));

    let module = entries_with(first.entries(), source, "Main");
    assert_eq!(module.len(), 1);
    assert_eq!(module[0].kind(), SemanticTokenKind::Namespace);
    assert_eq!(module[0].modifiers(), [SemanticTokenModifier::Definition]);

    let renderable = entries_with(first.entries(), source, "Renderable");
    assert!(renderable.iter().any(|entry| {
        entry.kind() == SemanticTokenKind::Interface
            && entry
                .modifiers()
                .contains(&SemanticTokenModifier::Definition)
    }));
    assert!(
        renderable
            .iter()
            .filter(|entry| entry.kind() == SemanticTokenKind::Interface)
            .count()
            >= 3
    );

    let item_type = entries_with(first.entries(), source, "Item");
    assert!(item_type.iter().any(|entry| {
        entry.kind() == SemanticTokenKind::Struct
            && entry
                .modifiers()
                .contains(&SemanticTokenModifier::Definition)
    }));
    assert!(
        item_type
            .iter()
            .any(|entry| entry.kind() == SemanticTokenKind::Type)
    );
    assert!(item_type.iter().all(|entry| {
        !entry
            .modifiers()
            .contains(&SemanticTokenModifier::DefaultLibrary)
    }));

    let text_type = entries_with(first.entries(), source, "Text");
    assert!(!text_type.is_empty());
    assert!(text_type.iter().all(|entry| {
        entry.kind() == SemanticTokenKind::Type
            && entry
                .modifiers()
                .contains(&SemanticTokenModifier::DefaultLibrary)
    }));

    let render = entries_with(first.entries(), source, "render");
    assert!(render.iter().any(|entry| {
        entry.kind() == SemanticTokenKind::Method
            && entry.modifiers() == [SemanticTokenModifier::Declaration]
    }));
    assert!(render.iter().any(|entry| {
        entry.kind() == SemanticTokenKind::Method
            && entry
                .modifiers()
                .contains(&SemanticTokenModifier::Definition)
    }));
    assert!(
        render
            .iter()
            .any(|entry| entry.kind() == SemanticTokenKind::Method && entry.modifiers().is_empty())
    );

    let item = entries_with(first.entries(), source, "item");
    assert!(item.iter().any(|entry| {
        entry.kind() == SemanticTokenKind::Parameter
            && entry
                .modifiers()
                .contains(&SemanticTokenModifier::Definition)
            && entry.modifiers().contains(&SemanticTokenModifier::Readonly)
    }));
    assert!(item.iter().any(|entry| {
        entry.kind() == SemanticTokenKind::Parameter
            && entry.evidence() == SemanticTokenEvidence::CheckedIdentity
            && entry.modifiers() == [SemanticTokenModifier::Readonly]
    }));

    let fields = entries_with(first.entries(), source, "name");
    assert!(
        fields
            .iter()
            .all(|entry| entry.kind() == SemanticTokenKind::Property)
    );
    assert!(fields.iter().any(|entry| {
        entry
            .modifiers()
            .contains(&SemanticTokenModifier::Definition)
            && entry.modifiers().contains(&SemanticTokenModifier::Readonly)
    }));

    let write = entries_with(first.entries(), source, "write");
    assert_eq!(write.len(), 1);
    assert_eq!(write[0].kind(), SemanticTokenKind::Function);
    assert!(
        write[0]
            .modifiers()
            .contains(&SemanticTokenModifier::DefaultLibrary)
    );
    assert_eq!(write[0].evidence(), SemanticTokenEvidence::CheckedIdentity);
}

#[test]
fn mutable_binding_and_exact_assignment_target_propagate_modifiers() {
    let source = concat!(
        "module Main\n\n",
        "let main () =\n",
        "    let mutable counter = 0\n",
        "    counter <- counter + 1\n",
        "    counter\n",
    );
    let mut db = CompilerDb::new();
    let file = set_source(&mut db, "fixtures/Mutable.ling", source);
    let index = db
        .semantic_token_index(file)
        .expect("mutable fixture type-checks");

    let counters = entries_with(index.entries(), source, "counter");
    assert_eq!(counters.len(), 4);
    assert!(counters.iter().all(|entry| {
        entry.kind() == SemanticTokenKind::Variable
            && entry.modifiers().contains(&SemanticTokenModifier::Mutable)
            && !entry.modifiers().contains(&SemanticTokenModifier::Readonly)
    }));
    assert!(counters.iter().any(|entry| {
        entry
            .modifiers()
            .contains(&SemanticTokenModifier::Definition)
            && !entry
                .modifiers()
                .contains(&SemanticTokenModifier::Modification)
    }));
    assert_eq!(
        counters
            .iter()
            .filter(|entry| entry
                .modifiers()
                .contains(&SemanticTokenModifier::Modification))
            .count(),
        1
    );
}

#[test]
fn failed_analysis_uses_only_unmodified_lexical_families() {
    let source = "module Main\n\nlet broken =\n/// docs\n\"text\" + 1\n";
    let mut db = CompilerDb::new();
    let file = set_source(&mut db, "fixtures/Broken.ling", source);
    let index = db
        .semantic_token_index(file)
        .expect("failed analysis still yields conservative fallback");

    assert_eq!(index.mode(), SemanticTokenGenerationMode::LexicalFallback);
    assert!(!index.entries().is_empty());
    assert!(index.entries().iter().all(|entry| {
        entry.evidence() == SemanticTokenEvidence::LexicalFallback
            && entry.modifiers().is_empty()
            && matches!(
                entry.kind(),
                SemanticTokenKind::Keyword
                    | SemanticTokenKind::Comment
                    | SemanticTokenKind::String
                    | SemanticTokenKind::Number
                    | SemanticTokenKind::Operator
            )
    }));
    assert!(entries_with(index.entries(), source, "broken").is_empty());
    assert!(entries_with(index.entries(), source, "Main").is_empty());
    assert!(entries_with(index.entries(), source, "docs").is_empty());
    assert!(index.entries().iter().any(|entry| {
        spelling(entry, source) == "/// docs" && entry.kind() == SemanticTokenKind::Comment
    }));
}

#[test]
fn unicode_bom_crlf_and_multiline_comment_keep_original_nonempty_segments() {
    let source = concat!(
        "\u{feff}module Main\r\n\r\n",
        "/* first\r\n第二 */\r\n",
        "let 人物 = \"😀e\u{301}\"\r\n",
    );
    let mut db = CompilerDb::new();
    let file = set_source(&mut db, "fixtures/Unicode.ling", source);
    let index = db
        .semantic_token_index(file)
        .expect("Unicode fixture type-checks");

    assert_eq!(index.mode(), SemanticTokenGenerationMode::Typed);
    let comment_segments = index
        .entries()
        .iter()
        .filter(|entry| entry.kind() == SemanticTokenKind::Comment)
        .collect::<Vec<_>>();
    assert_eq!(comment_segments.len(), 2);
    assert_eq!(spelling(comment_segments[0], source), "/* first");
    assert_eq!(spelling(comment_segments[1], source), "第二 */");
    assert!(index.entries().iter().all(|entry| {
        let text = spelling(entry, source);
        !text.is_empty() && !text.contains('\n') && !text.ends_with('\r')
    }));
    let person = entries_with(index.entries(), source, "人物");
    assert_eq!(person.len(), 1);
    assert_eq!(person[0].kind(), SemanticTokenKind::Variable);
    assert_eq!(
        entries_with(index.entries(), source, "\"😀e\u{301}\"").len(),
        1
    );
}

#[test]
fn dependency_failure_invalidates_typed_cache_without_source_edit() {
    let mut db = CompilerDb::new();
    let main_source = "module Main\n\nimport Library\n\nlet main () = Library.answer\n";
    let main = set_source(&mut db, "Main.ling", main_source);
    set_source(
        &mut db,
        "Library.ling",
        "module Library\n\nlet answer = 1\n",
    );

    let typed = db
        .semantic_token_index(main)
        .expect("initial workspace type-checks");
    assert_eq!(typed.mode(), SemanticTokenGenerationMode::Typed);

    set_source(&mut db, "Library.ling", "module Library\n\nlet answer =\n");
    let fallback = db
        .semantic_token_index(main)
        .expect("dependency failure yields source fallback");
    assert_eq!(
        fallback.mode(),
        SemanticTokenGenerationMode::LexicalFallback
    );
    assert!(!Arc::ptr_eq(&typed, &fallback));

    set_source(
        &mut db,
        "Library.ling",
        "module Library\n\nlet answer = 2\n",
    );
    let restored = db
        .semantic_token_index(main)
        .expect("repaired dependency restores checked generation");
    assert_eq!(restored.mode(), SemanticTokenGenerationMode::Typed);
    assert!(!Arc::ptr_eq(&typed, &restored));
    assert_eq!(typed.revision(), restored.revision());
    assert_eq!(typed.entries(), restored.entries());
}

#[test]
fn resolution_type_and_effect_failures_never_publish_identifier_roles() {
    for (name, source, identifier) in [
        (
            "resolution",
            "module Main\n\nlet main () = missing\n",
            "missing",
        ),
        (
            "type",
            "module Main\n\nlet main () = 1 + \"text\"\n",
            "main",
        ),
        (
            "effect",
            "module Main\n\nlet main () = Console.write \"x\"\n",
            "write",
        ),
    ] {
        let mut db = CompilerDb::new();
        let logical_name = format!("fixtures/{name}.ling");
        let file = set_source(&mut db, &logical_name, source);
        let index = db
            .semantic_token_index(file)
            .expect("analysis failure has a conservative token result");
        assert_eq!(
            index.mode(),
            SemanticTokenGenerationMode::LexicalFallback,
            "{name} failure must not publish partial checked roles"
        );
        assert!(entries_with(index.entries(), source, identifier).is_empty());
        assert!(index.entries().iter().all(|entry| {
            entry.evidence() == SemanticTokenEvidence::LexicalFallback
                && entry.modifiers().is_empty()
        }));
    }
}

#[test]
fn prelude_identity_is_marked_but_user_shadowing_is_not() {
    let source = concat!(
        "module Main\n\n",
        "let main () =\n",
        "    let max left right = left\n",
        "    let option = Some (max 1 2)\n",
        "    option\n",
    );
    let mut db = CompilerDb::new();
    let file = set_source(&mut db, "fixtures/Prelude.ling", source);
    let index = db
        .semantic_token_index(file)
        .expect("Prelude and shadowing fixture checks");
    assert_eq!(index.mode(), SemanticTokenGenerationMode::Typed);

    let some = entries_with(index.entries(), source, "Some");
    assert_eq!(some.len(), 1);
    assert_eq!(some[0].kind(), SemanticTokenKind::EnumMember);
    assert!(
        some[0]
            .modifiers()
            .contains(&SemanticTokenModifier::DefaultLibrary)
    );

    let max = entries_with(index.entries(), source, "max");
    assert_eq!(max.len(), 2);
    assert!(
        max.iter()
            .all(|entry| entry.kind() == SemanticTokenKind::Function)
    );
    assert!(max.iter().all(|entry| {
        !entry
            .modifiers()
            .contains(&SemanticTokenModifier::DefaultLibrary)
    }));
}

#[test]
fn variant_declarations_references_and_patterns_use_enum_roles() {
    let source = concat!(
        "module Main\n\n",
        "type State =\n",
        "    | Ready\n",
        "    | Busy of Int\n\n",
        "let unwrap state =\n",
        "    match state with\n",
        "    | Ready -> 0\n",
        "    | Busy value -> value\n\n",
        "let main () = unwrap (Busy 7)\n",
    );
    let mut db = CompilerDb::new();
    let file = set_source(&mut db, "fixtures/Variant.ling", source);
    let index = db
        .semantic_token_index(file)
        .expect("variant fixture checks");

    let state_type = entries_with(index.entries(), source, "State");
    assert_eq!(state_type.len(), 1);
    assert_eq!(state_type[0].kind(), SemanticTokenKind::Enum);
    assert!(
        state_type[0]
            .modifiers()
            .contains(&SemanticTokenModifier::Definition)
    );

    for constructor in ["Ready", "Busy"] {
        let entries = entries_with(index.entries(), source, constructor);
        assert!(entries.len() >= 2);
        assert!(
            entries
                .iter()
                .all(|entry| entry.kind() == SemanticTokenKind::EnumMember),
            "{constructor}: {entries:?}"
        );
        assert!(entries.iter().any(|entry| {
            entry
                .modifiers()
                .contains(&SemanticTokenModifier::Definition)
        }));
    }
}
