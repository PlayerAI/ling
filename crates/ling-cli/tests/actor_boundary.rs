use ling_cli::{CompileFailure, checked_actor_implementation_boundary, compile_source};
use ling_diagnostics::codes;

const ACTOR_PROGRAM: &str = concat!(
    "module Main\n\n",
    "actor Counter : Int =\n",
    "    state Int = 0\n",
    "    receive state message =\n",
    "        state + message\n\n",
    "let main () = ()\n",
);

#[test]
fn actor_declaration_reaches_checked_core_but_execution_is_rejected() {
    let compiled = compile_source("actor.ling", ACTOR_PROGRAM.as_bytes().to_vec())
        .expect("the DEC-0270 checked Actor subset compiles");
    let checked = compiled.snapshot.checked();
    assert!(checked.has_actors());
    let core = checked
        .actor_cores()
        .values()
        .next()
        .expect("checked Actor Core");
    assert!(core.actor_type().is_valid());
    assert_eq!(core.reference_type().actor_type(), core.actor_type());
    assert_eq!(core.reference_type().message(), core.message_type());
    assert!(core.reference_type().is_local_and_invariant());
    let contract = core.message_contract();
    assert_eq!(contract.actor(), core.definition());
    assert_eq!(contract.message_type(), core.message_type());
    assert_eq!(contract.sendability().as_str(), "SendableLocal(Value)");
    assert!(
        contract
            .schema()
            .id()
            .as_str()
            .starts_with("experimental:blake3:")
    );
    assert_eq!(contract.schema().root(), 0);
    assert!(!contract.schema().nodes().is_empty());
    assert_eq!(contract.source_span(), core.source_spans().message_type);
    assert!(core.effects().is_pure());
    assert!(core.actor_id_contract().is_runtime_scoped());
    assert!(
        core.actor_id_contract()
            .requires_nonzero_unique_nonreusable_ids()
    );
    assert!(!core.actor_id_contract().allocates_instances());

    let diagnostic = checked_actor_implementation_boundary(checked, "run")
        .expect("Actor execution remains behind ACT-2305");
    assert_eq!(diagnostic.code(), codes::ACTOR_IMPLEMENTATION_BOUNDARY);
    let span = diagnostic.primary_span().expect("original Actor span");
    assert_eq!(span.file(), "actor.ling");
    let json = diagnostic.render_json().expect("diagnostic JSON");
    assert!(json.contains("checked Actor"));
    assert!(json.contains("已检查 Actor"));
    assert!(json.contains("ACT-2305"));
}

#[test]
fn actor_core_identity_ignores_source_name_and_line_endings() {
    let first = compile_source("first.ling", ACTOR_PROGRAM.as_bytes().to_vec())
        .expect("first Actor source compiles");
    let second_source = ACTOR_PROGRAM.replace('\n', "\r\n");
    let second = compile_source("second.ling", second_source.into_bytes())
        .expect("second Actor source compiles");
    let first_core = first
        .snapshot
        .checked()
        .actor_cores()
        .values()
        .next()
        .expect("first core");
    let second_core = second
        .snapshot
        .checked()
        .actor_cores()
        .values()
        .next()
        .expect("second core");
    assert_eq!(first_core.actor_type(), second_core.actor_type());
    assert_eq!(first_core.canonical_bytes(), second_core.canonical_bytes());
    assert_eq!(
        first_core.message_contract().schema().id(),
        second_core.message_contract().schema().id()
    );
}

#[test]
fn actor_message_schema_closes_generic_recursive_nominal_graphs() {
    let source = concat!(
        "module Main\n\n",
        "type Envelope<'a> = { payload: 'a }\n",
        "type Tree<'a> =\n",
        "    | Branch of List<Tree<'a>>\n",
        "    | Leaf of 'a\n\n",
        "actor Inbox : Envelope<Tree<Int>> =\n",
        "    state Int = 0\n",
        "    receive state message =\n",
        "        state\n",
    );
    let first = compile_source("tree.ling", source.as_bytes().to_vec())
        .expect("closed recursive message graph compiles");
    let second = compile_source(
        "renamed.ling",
        source.replace('\n', "\r\n").as_bytes().to_vec(),
    )
    .expect("source evidence does not change schema identity");
    let first_schema = first
        .snapshot
        .checked()
        .actor_cores()
        .values()
        .next()
        .expect("first Actor")
        .message_contract()
        .schema();
    let second_schema = second
        .snapshot
        .checked()
        .actor_cores()
        .values()
        .next()
        .expect("second Actor")
        .message_contract()
        .schema();
    assert_eq!(first_schema.id(), second_schema.id());
    assert_eq!(
        first_schema.canonical_bytes(),
        second_schema.canonical_bytes()
    );
    assert!(first_schema.nodes().len() >= 4);
    ling_semantic::read_json(first.snapshot.json()).expect("Actor graph reader accepts output");
}

#[test]
fn actor_core_preserves_original_bom_crlf_unicode_spans() {
    let source = concat!(
        "\u{feff}module Main\r\n\r\n",
        "actor 计数器 : Int =\r\n",
        "    state Int = 0\r\n",
        "    receive 状态 消息 =\r\n",
        "        状态 + 消息\r\n",
    );
    let compiled = compile_source("unicode-actor.ling", source.as_bytes().to_vec())
        .expect("Unicode Actor source compiles");
    let core = compiled
        .snapshot
        .checked()
        .actor_cores()
        .values()
        .next()
        .expect("checked Actor Core");
    let spans = core.source_spans();
    let text =
        |span: ling_source::Span| &source[span.start().get() as usize..span.end().get() as usize];
    assert_eq!(text(spans.actor_keyword), "actor");
    assert_eq!(text(spans.message_type), "Int");
    assert_eq!(text(spans.state_keyword), "state");
    assert_eq!(text(spans.state_pattern), "状态");
    assert_eq!(text(spans.message_pattern), "消息");
    assert_eq!(text(spans.receive_keyword), "receive");
    assert_eq!(text(spans.transition_body), "状态 + 消息");
}

#[test]
fn actor_rejects_effectful_transition_and_non_value_message_type() {
    for source in [
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "actor Logger : Text =\n",
            "    state Int = 0\n",
            "    receive state message =\n",
            "        Console.write message\n",
            "        state\n",
        ),
        concat!(
            "module Main\n\n",
            "actor Callback : Int -> Int =\n",
            "    state Int = 0\n",
            "    receive state callback =\n",
            "        state\n",
        ),
    ] {
        let Err(CompileFailure::Diagnostics(diagnostics)) =
            compile_source("invalid-actor.ling", source.as_bytes().to_vec())
        else {
            panic!("invalid Actor declaration must be rejected");
        };
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code() == codes::INVALID_ACTOR_DECLARATION),
            "{diagnostics:?}"
        );
        if source.contains("Callback") {
            assert!(
                diagnostics.iter().any(|diagnostic| diagnostic
                    .render_json()
                    .is_ok_and(|json| json.contains("message_type_function_not_sendable_local"))),
                "{diagnostics:?}"
            );
        }
    }
}

#[test]
fn actor_declaration_is_not_a_first_class_source_value() {
    let source = format!("{ACTOR_PROGRAM}\nlet copy = Counter\n");
    let Err(CompileFailure::Diagnostics(diagnostics)) =
        compile_source("actor-value.ling", source.into_bytes())
    else {
        panic!("an Actor declaration must not become a first-class value");
    };
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code() == codes::INVALID_ACTOR_DECLARATION),
        "{diagnostics:?}"
    );
}

#[test]
fn actor_words_remain_contextual_identifiers() {
    compile_source(
        "contextual.ling",
        concat!(
            "module Main\n\n",
            "let actor = 1\n",
            "let state = actor + 1\n",
            "let receive = state + 1\n",
        )
        .as_bytes()
        .to_vec(),
    )
    .expect("Actor contextual words remain ordinary identifiers elsewhere");
}
