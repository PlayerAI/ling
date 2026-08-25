use ling_ast::lower as lower_ast;
use ling_effects::{Effect, EffectErrorKind, check};
use ling_hir::lower as lower_hir;
use ling_resolve::resolve;
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;

fn check_source(source_name: &str, text: &str) -> ling_effects::CheckedProgram {
    check(typed_source(source_name, text)).expect("checked program")
}

fn typed_source(source_name: &str, text: &str) -> ling_types::TypedProgram {
    let source = SourceFile::from_bytes(SourceId::new(0), source_name, text.as_bytes().to_vec())
        .expect("valid UTF-8 source");
    let parsed = parse(&source);
    assert!(
        parsed.is_valid(),
        "parse errors: {:?}",
        parsed.parse_errors()
    );
    let ast = lower_ast(&source, &parsed).expect("valid AST");
    let hir = lower_hir(source.name(), &ast).expect("valid HIR");
    let resolved = resolve(vec![hir], "Main").expect("resolved program");
    check_types(resolved).expect("typed program")
}

fn check_errors(text: &str) -> Vec<ling_effects::EffectError> {
    check(typed_source("invalid-task.ling", text)).expect_err("Task program must be rejected")
}

#[test]
fn task_scope_requires_a_return_on_every_reachable_path() {
    let errors = check_errors(concat!(
        "module Main\n\n",
        "task missing value =\n",
        "    scope\n",
        "        if true then return value else value\n",
    ));
    assert!(errors.iter().any(|error| matches!(
        error.kind,
        EffectErrorKind::InvalidTaskStructure {
            reason: "missing_final_return",
            ..
        }
    )));

    let errors = check_errors(concat!(
        "module Main\n\n",
        "task nonfinal value =\n",
        "    scope\n",
        "        return value\n",
        "        value\n",
    ));
    assert!(errors.iter().any(|error| matches!(
        error.kind,
        EffectErrorKind::InvalidTaskStructure {
            reason: "non_final_return",
            ..
        }
    )));
}

#[test]
fn task_handles_are_observed_exactly_once_in_their_own_scope() {
    let unobserved = check_errors(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let handle = spawn child value\n",
        "        return value\n",
    ));
    assert!(unobserved.iter().any(|error| matches!(
        error.kind,
        EffectErrorKind::InvalidTaskHandle {
            reason: "handle_not_observed_on_scope_exit",
            ..
        }
    )));

    let double = check_errors(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let handle = spawn child value\n",
        "        let first = await handle\n",
        "        let second = await handle\n",
        "        return second\n",
    ));
    assert!(double.iter().any(|error| matches!(
        error.kind,
        EffectErrorKind::InvalidTaskHandle {
            reason: "handle_observed_more_than_once",
            ..
        }
    )));
}

#[test]
fn task_handle_observation_is_checked_on_each_control_flow_path() {
    let checked = check_source(
        "branch-task.ling",
        concat!(
            "module Main\n\n",
            "task child value =\n",
            "    scope\n",
            "        return value\n\n",
            "task parent value =\n",
            "    scope\n",
            "        let handle = spawn child value\n",
            "        let result = if true then await handle else await handle\n",
            "        return result\n",
        ),
    );
    let parent = checked
        .task_cores()
        .values()
        .find(|core| !core.spawns().is_empty())
        .expect("parent Task Core");
    assert_eq!(parent.spawns().len(), 1);
    assert_eq!(parent.suspensions().len(), 2);
    assert!(
        parent
            .suspensions()
            .iter()
            .all(|suspension| suspension.awaited_task() == parent.spawns()[0].task())
    );

    let errors = check_errors(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let handle = spawn child value\n",
        "        let result = if true then await handle else value\n",
        "        return result\n",
    ));
    assert!(errors.iter().any(|error| matches!(
        error.kind,
        EffectErrorKind::InvalidTaskHandle {
            reason: "handle_observation_differs_across_paths",
            ..
        }
    )));
}

#[test]
fn task_suspension_rejects_mutable_and_other_handle_live_values() {
    let mutable = check_errors(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let mutable state = value\n",
        "        let handle = spawn child value\n",
        "        let result = await handle\n",
        "        return state\n",
    ));
    assert!(
        mutable.iter().any(|error| matches!(
            error.kind,
            EffectErrorKind::UnsafeTaskSuspension {
                reason: "mutable_binding_crosses_suspension",
                ..
            }
        )),
        "{mutable:?}"
    );

    let other_handle = check_errors(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let first = spawn child value\n",
        "        let second = spawn child value\n",
        "        let left = await first\n",
        "        let right = await second\n",
        "        return right\n",
    ));
    assert!(
        other_handle.iter().any(|error| matches!(
            error.kind,
            EffectErrorKind::UnsafeTaskSuspension {
                reason: "other_task_handle_crosses_suspension",
                ..
            }
        )),
        "{other_handle:?}"
    );
}

#[test]
fn recursive_task_spawn_chain_is_rejected() {
    let errors = check_errors(concat!(
        "module Main\n\n",
        "task recurse value =\n",
        "    scope\n",
        "        let! next = recurse value\n",
        "        return next\n",
    ));
    assert!(
        errors.iter().any(|error| matches!(
            error.kind,
            EffectErrorKind::InvalidTaskStructure {
                reason: "recursive_spawn_chain",
                ..
            }
        )),
        "{errors:?}"
    );
}

#[test]
fn task_declaration_publishes_checked_task_core() {
    let checked = check_source(
        "task.ling",
        concat!(
            "module Main\n\n",
            "task echo value =\n",
            "    scope\n",
            "        return value\n",
        ),
    );

    let core = checked
        .task_cores()
        .values()
        .next()
        .expect("checked Task Core");
    assert_eq!(checked.task_cores().len(), 1);
    assert_eq!(core.root_scope().get(), 1);
    assert_eq!(core.scopes().len(), 1);
    assert!(core.spawns().is_empty());
    assert!(core.suspensions().is_empty());
    assert!(core.signature().effects().effects().next().is_none());
}

#[test]
fn explicit_and_fused_task_observation_publish_structural_effects() {
    let checked = check_source(
        "tasks.ling",
        concat!(
            "module Main\n\n",
            "task child value =\n",
            "    scope\n",
            "        return value\n\n",
            "task parent value =\n",
            "    scope\n",
            "        let handle = spawn child value\n",
            "        let first = await handle\n",
            "        let! second = child first\n",
            "        return second\n",
        ),
    );

    let parent = checked
        .task_cores()
        .values()
        .find(|core| core.spawns().len() == 2)
        .expect("parent Task Core");
    assert_eq!(parent.suspensions().len(), 2);
    assert!(
        parent
            .signature()
            .effects()
            .effects()
            .any(|effect| effect == &Effect::TaskSpawn)
    );
    assert!(
        parent
            .signature()
            .effects()
            .effects()
            .any(|effect| effect == &Effect::TaskAwait)
    );
}

#[test]
fn checked_task_bytes_are_path_independent_and_follow_alpha_renaming_rules() {
    const FIRST: &str = concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let handle = spawn child value\n",
        "        let result = await handle\n",
        "        return result\n",
    );
    const LOCALS_RENAMED: &str = concat!(
        "module Main\n\n",
        "task child input =\n",
        "    scope\n",
        "        return input\n\n",
        "task parent input =\n",
        "    scope\n",
        "        let pending = spawn child input\n",
        "        let output = await pending\n",
        "        return output\n",
    );
    const TASK_RENAMED: &str = concat!(
        "module Main\n\n",
        "task renamed value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let handle = spawn renamed value\n",
        "        let result = await handle\n",
        "        return result\n",
    );

    let first = check_source("C:/one/task.ling", FIRST);
    let reconstructed = check_source("D:/elsewhere/task.ling", FIRST);
    let locals_renamed = check_source("renamed-locals.ling", LOCALS_RENAMED);
    let task_renamed = check_source("renamed-task.ling", TASK_RENAMED);
    let bytes = |checked: &ling_effects::CheckedProgram| {
        checked
            .task_cores()
            .values()
            .map(|core| core.canonical_bytes(checked.typed()))
            .collect::<Vec<_>>()
    };

    assert_eq!(bytes(&first), bytes(&reconstructed));
    assert_eq!(bytes(&first), bytes(&locals_renamed));
    assert_ne!(bytes(&first), bytes(&task_renamed));
}

#[test]
fn task_core_preserves_bom_crlf_and_chinese_syntax_spans() {
    let source = concat!(
        "\u{feff}module Main\r\n\r\n",
        "task 子任务 值 =\r\n",
        "    scope\r\n",
        "        return 值\r\n\r\n",
        "task 父任务 值 =\r\n",
        "    scope\r\n",
        "        let! 结果 = 子任务 值\r\n",
        "        return 结果\r\n",
    );
    let checked = check_source("任务.ling", source);
    let parent = checked
        .task_cores()
        .values()
        .find(|core| !core.spawns().is_empty())
        .expect("parent Task Core");
    let spawn = &parent.spawns()[0];

    assert_eq!(
        usize::try_from(parent.source_span().start().get()).expect("span fits usize"),
        source.find("task 父任务").expect("parent declaration")
    );
    assert_eq!(
        usize::try_from(spawn.operator_span().start().get()).expect("span fits usize"),
        source.find("let! 结果").expect("let! operator")
    );
    let pattern = spawn.pattern_span().expect("fused pattern span");
    assert_eq!(
        &source[usize::try_from(pattern.start().get()).expect("span fits usize")
            ..usize::try_from(pattern.end().get()).expect("span fits usize")],
        "结果"
    );
}
