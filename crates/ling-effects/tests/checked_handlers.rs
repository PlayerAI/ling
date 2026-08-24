use ling_ast::lower as lower_ast;
use ling_diagnostics::codes;
use ling_effects::{Capability, CheckedProgram, EffectError, check};
use ling_hir::lower as lower_hir;
use ling_resolve::{ResolveError, resolve};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;

fn hir(text: &str) -> ling_hir::Program {
    let source = SourceFile::from_bytes(SourceId::new(0), "handler.ling", text.as_bytes().to_vec())
        .expect("valid UTF-8 source");
    let parsed = parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = lower_ast(&source, &parsed).expect("valid handler AST");
    lower_hir(source.name(), &ast).expect("valid handler HIR")
}

fn resolve_source(text: &str) -> Result<ling_resolve::ResolvedProgram, Vec<ResolveError>> {
    resolve(vec![hir(text)], "Main")
}

fn compile_checked(text: &str) -> Result<CheckedProgram, Vec<EffectError>> {
    let resolved = resolve_source(text).expect("source resolves");
    let typed = ling_types::check(resolved).expect("source type-checks");
    check(typed)
}

#[test]
fn source_handler_publishes_core_and_subtracts_transitive_effects() {
    let source = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let emit () = Console.write \"x\"\n\n",
        "let main () =\n",
        "    handle emit () with\n",
        "        operation Console.Write.write(message, resume) -> resume ()\n",
    );
    let checked = compile_checked(source).expect("checked handler");
    let module = checked.typed().resolved().entry_module();
    let main = checked
        .typed()
        .resolved()
        .definition_id(module.id, "main")
        .expect("main definition");
    assert!(checked.definition_effect(main).expect("main row").is_pure());
    assert!(
        checked
            .module_capabilities(module.id)
            .expect("module capabilities")
            .contains(&Capability::ConsoleWrite)
    );

    assert_eq!(checked.handler_cores().len(), 1);
    let core = checked
        .handler_cores()
        .values()
        .next()
        .expect("handler Core");
    assert_eq!(core.input().canonical_name(), "{Console.Write}");
    assert_eq!(core.residual().canonical_name(), "{}");
    assert_eq!(core.return_type().as_str(), "Unit");
    assert_eq!(core.clauses().len(), 1);
    assert!(core.source_span().is_some());

    let repeated = compile_checked(source).expect("repeat checked handler");
    assert_eq!(
        core.canonical_bytes(),
        repeated
            .handler_cores()
            .values()
            .next()
            .expect("repeated Core")
            .canonical_bytes()
    );
}

#[test]
fn handled_console_still_requires_host_capability() {
    let source = concat!(
        "module Main\n\n",
        "let main () =\n",
        "    handle Console.write \"x\" with\n",
        "        operation Console.Write.write(message, resume) -> resume ()\n",
    );
    let errors = compile_checked(source).expect_err("handling never grants Capability authority");
    assert!(
        errors
            .iter()
            .any(|error| error.to_diagnostic().code() == codes::MISSING_CAPABILITY)
    );
}

#[test]
fn registry_arity_and_resume_cardinality_fail_before_checked_publication() {
    let unknown = concat!(
        "module Main\n\n",
        "let value =\n",
        "    handle 1 with\n",
        "        operation Missing.run() -> 1\n",
    );
    let errors = resolve_source(unknown).expect_err("unknown operation");
    assert_eq!(
        errors[0].to_diagnostic().code(),
        codes::INVALID_HANDLER_CONTRACT
    );

    let arity = concat!(
        "module Main\n\n",
        "let value =\n",
        "    handle 1 with\n",
        "        operation Random.next() -> 1\n",
    );
    let errors = resolve_source(arity).expect_err("operation arity");
    assert!(errors.iter().any(|error| {
        error.to_diagnostic().code() == codes::INVALID_HANDLER_CONTRACT
            && error
                .to_diagnostic()
                .render_json()
                .expect("diagnostic JSON")
                .contains("parameter_arity")
    }));

    let repeated_once = concat!(
        "module Main\n\n",
        "let value =\n",
        "    handle () with\n",
        "        operation Console.Write.write(message, resume) ->\n",
        "            resume ()\n",
        "            resume ()\n",
    );
    let errors = resolve_source(repeated_once).expect_err("Once resume used twice");
    assert!(errors.iter().any(|error| {
        error.to_diagnostic().code() == codes::INVALID_HANDLER_CONTRACT
            && error
                .to_diagnostic()
                .render_json()
                .expect("diagnostic JSON")
                .contains("resume_uses")
    }));

    let duplicate = concat!(
        "module Main\n\n",
        "let value =\n",
        "    handle () with\n",
        "        operation Console.Write.write(first, resume) -> ()\n",
        "        operation Console.Write.write(second, resume) -> ()\n",
    );
    let errors = resolve_source(duplicate).expect_err("duplicate handled label");
    assert!(errors.iter().any(|error| {
        matches!(
            error.kind,
            ling_resolve::ResolveErrorKind::InvalidHandlerContract {
                reason: "duplicate_handled_label",
                ..
            }
        )
    }));
}

#[test]
fn handler_operation_inputs_accept_only_bindings_and_wildcards() {
    let accepted = concat!(
        "module Main\n\n",
        "let bound =\n",
        "    handle 1 with\n",
        "        operation Random.next(seed, resume) -> seed\n\n",
        "let ignored =\n",
        "    handle () with\n",
        "        operation Console.Write.write(_, resume) -> ()\n",
    );
    let checked = compile_checked(accepted).expect("binding and wildcard inputs are total");
    assert_eq!(checked.handler_cores().len(), 2);

    let cases = [
        (
            concat!(
                "module Main\n\n",
                "let value =\n",
                "    handle 1 with\n",
                "        operation Random.next(0, resume) -> 1\n",
            ),
            "0",
        ),
        (
            concat!(
                "module Main\n\n",
                "let value =\n",
                "    handle 1 with\n",
                "        operation Random.next((left, right), resume) -> left\n",
            ),
            "(left, right)",
        ),
        (
            concat!(
                "module Main\n\n",
                "let value =\n",
                "    handle 1 with\n",
                "        operation Random.next({ value = item }, resume) -> item\n",
            ),
            "{ value = item }",
        ),
        (
            concat!(
                "\u{feff}module Main\r\n\r\n",
                "type 包装 =\r\n",
                "    | 包裹 of Int\r\n\r\n",
                "let value =\r\n",
                "    handle 1 with\r\n",
                "        operation Random.next((包裹 值), resume) -> 值\r\n",
            ),
            "(包裹 值)",
        ),
    ];

    for (source, rejected_pattern) in cases {
        let errors = resolve_source(source).expect_err("refutable input prevents publication");
        let error = errors
            .iter()
            .find(|error| {
                matches!(
                    error.kind,
                    ling_resolve::ResolveErrorKind::InvalidHandlerContract {
                        reason: "refutable_parameter",
                        ..
                    }
                )
            })
            .expect("registered refutable-parameter diagnostic");
        let expected_start = source
            .find(rejected_pattern)
            .expect("pattern spelling occurs in source");
        let diagnostic = error.to_diagnostic();
        let span = diagnostic.primary_span().expect("pattern span");

        assert_eq!(diagnostic.code(), codes::INVALID_HANDLER_CONTRACT);
        assert_eq!(span.start_byte(), expected_start as u64);
        assert_eq!(
            span.end_byte(),
            (expected_start + rejected_pattern.len()) as u64
        );
        assert_eq!(
            diagnostic
                .facts()
                .get("operation")
                .and_then(|value| value.as_str()),
            Some("Random.next")
        );
        assert_eq!(
            diagnostic
                .facts()
                .get("reason")
                .and_then(|value| value.as_str()),
            Some("refutable_parameter")
        );
        assert!(
            diagnostic
                .message_zh()
                .contains("Handler clause contract 无效")
        );
        assert!(
            diagnostic
                .message_en()
                .contains("handler clause contract is invalid")
        );
    }
}

#[test]
fn many_resume_and_clause_body_effects_are_checked() {
    let many = concat!(
        "module Main\n\n",
        "let value =\n",
        "    handle 1 with\n",
        "        operation Random.next(seed, resume) ->\n",
        "            resume seed\n",
        "            resume seed\n",
    );
    let checked = compile_checked(many).expect("Many permits multiple source references");
    let core = checked
        .handler_cores()
        .values()
        .next()
        .expect("handler Core");
    assert_eq!(
        core.clauses()[0].resume_use(),
        ling_effects::ResumeUse::Many
    );

    let shadowed = concat!(
        "module Main\n\n",
        "let seed = \"outer\"\n",
        "let value =\n",
        "    handle 1 with\n",
        "        operation Random.next(seed, resume) -> seed\n",
    );
    let checked = compile_checked(shadowed).expect("clause parameter shadows outer definition");
    let module = checked.typed().resolved().entry_module();
    let value = checked
        .typed()
        .resolved()
        .definition_id(module.id, "value")
        .expect("value definition");
    let value_type = checked.typed().definition_type(value).expect("value type");
    assert_eq!(checked.typed().display_type(value_type), "Int");

    let clause_effect = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let value =\n",
        "    handle () with\n",
        "        operation Clock.now() -> Console.write \"clause\"\n",
    );
    let checked = compile_checked(clause_effect).expect("clause body effect checks");
    let module = checked.typed().resolved().entry_module();
    let value = checked
        .typed()
        .resolved()
        .definition_id(module.id, "value")
        .expect("value definition");
    assert_eq!(
        checked
            .definition_effect(value)
            .expect("value effects")
            .canonical_names(),
        ["Console.Write"]
    );
}

#[test]
fn nested_handlers_publish_inner_before_outer_without_path_identity() {
    let source = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let inner () =\n",
        "    handle Console.write \"x\" with\n",
        "        operation Console.Write.write(message, resume) -> resume ()\n\n",
        "let value =\n",
        "    handle inner () with\n",
        "        operation Clock.now() -> ()\n",
    );
    let checked = compile_checked(source).expect("nested handlers check");
    assert_eq!(checked.handler_cores().len(), 2);
    let rows = checked
        .handler_cores()
        .values()
        .map(|core| {
            (
                core.input().canonical_name(),
                core.residual().canonical_name(),
            )
        })
        .collect::<Vec<_>>();
    assert!(rows.contains(&("{Console.Write}".to_owned(), "{}".to_owned())));
    assert!(rows.contains(&("{}".to_owned(), "{}".to_owned())));
    assert!(
        checked
            .handler_cores()
            .values()
            .all(|core| !String::from_utf8_lossy(&core.canonical_bytes()).contains("handler.ling"))
    );
}

#[test]
fn state_remains_visible_and_clause_results_share_the_handler_type() {
    let state = concat!(
        "module Main\n\n",
        "let mutate () =\n",
        "    let mutable cell = 0\n",
        "    cell <- 1\n",
        "    cell\n\n",
        "let value =\n",
        "    handle mutate () with\n",
        "        operation Clock.now() -> 1\n",
    );
    let checked = compile_checked(state).expect("State remains a checked residual");
    let core = checked
        .handler_cores()
        .values()
        .next()
        .expect("handler Core");
    assert_eq!(core.input().canonical_name(), "{State<Int>}");
    assert_eq!(core.residual().canonical_name(), "{State<Int>}");

    let mismatch = concat!(
        "module Main\n\n",
        "let value =\n",
        "    handle 1 with\n",
        "        operation Random.next(seed, resume) -> \"wrong\"\n",
    );
    let resolved = resolve_source(mismatch).expect("mismatched handler resolves");
    let errors = ling_types::check(resolved).expect_err("clause result type must match body");
    assert!(
        errors
            .iter()
            .any(|error| error.to_diagnostic().code() == codes::TYPE_MISMATCH)
    );
}
