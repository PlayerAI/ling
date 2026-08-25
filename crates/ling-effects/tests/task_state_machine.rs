use ling_ast::lower as lower_ast;
use ling_effects::{
    CHECKED_TASK_MACHINE_VERSION, CheckedProgram, CheckedTaskMachineEdgeKind,
    CheckedTaskMachineStateKind, check,
};
use ling_hir::lower as lower_hir;
use ling_resolve::resolve;
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;

fn check_source(source_id: u32, source_name: &str, text: &str) -> CheckedProgram {
    let source = SourceFile::from_bytes(
        SourceId::new(source_id),
        source_name,
        text.as_bytes().to_vec(),
    )
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
    let typed = check_types(resolved).expect("typed program");
    check(typed).expect("checked program")
}

fn parent(checked: &CheckedProgram) -> &ling_effects::CheckedTaskMachine {
    checked
        .task_machines()
        .values()
        .find(|machine| {
            machine
                .states()
                .iter()
                .any(|state| matches!(state.kind(), CheckedTaskMachineStateKind::Suspend { .. }))
        })
        .expect("parent task machine")
}

#[test]
fn task_without_suspension_has_explicit_cleanup_and_terminal_paths() {
    let checked = check_source(
        0,
        "simple-task.ling",
        concat!(
            "module Main\n\n",
            "task echo value =\n",
            "    scope\n",
            "        return value\n",
        ),
    );
    let machine = checked
        .task_machines()
        .values()
        .next()
        .expect("task machine");

    assert_eq!(machine.version(), CHECKED_TASK_MACHINE_VERSION);
    assert_eq!(machine.states().len(), 7);
    assert_eq!(machine.projection().states().len(), machine.states().len());
    assert!(matches!(
        machine.state(machine.entry()).map(|state| state.kind()),
        Some(CheckedTaskMachineStateKind::Entry)
    ));

    for state in machine.states() {
        let outgoing = machine
            .edges()
            .iter()
            .filter(|edge| edge.from() == state.id())
            .collect::<Vec<_>>();
        match state.kind() {
            CheckedTaskMachineStateKind::Entry => {
                assert!(outgoing.iter().any(|edge| {
                    edge.kind() == CheckedTaskMachineEdgeKind::Continue
                        && matches!(
                            machine.state(edge.to()).map(|target| target.kind()),
                            Some(CheckedTaskMachineStateKind::ReturnCleanup)
                        )
                }));
                assert!(
                    outgoing
                        .iter()
                        .any(|edge| edge.kind() == CheckedTaskMachineEdgeKind::Cancel)
                );
                assert!(
                    outgoing
                        .iter()
                        .any(|edge| edge.kind() == CheckedTaskMachineEdgeKind::Fault)
                );
            }
            CheckedTaskMachineStateKind::ReturnCleanup
            | CheckedTaskMachineStateKind::CancelCleanup
            | CheckedTaskMachineStateKind::FaultCleanup => {
                assert_eq!(outgoing.len(), 1);
                assert_eq!(outgoing[0].kind(), CheckedTaskMachineEdgeKind::Cleanup);
            }
            CheckedTaskMachineStateKind::Completed
            | CheckedTaskMachineStateKind::Cancelled
            | CheckedTaskMachineStateKind::Faulted => assert!(outgoing.is_empty()),
            CheckedTaskMachineStateKind::Suspend { .. } => unreachable!(),
        }
    }
}

#[test]
fn sequential_suspensions_resume_in_order_with_exact_checked_frames() {
    let checked = check_source(
        0,
        "sequential-task.ling",
        concat!(
            "module Main\n\n",
            "task child value =\n",
            "    scope\n",
            "        return value\n\n",
            "task parent value =\n",
            "    scope\n",
            "        let seed = value\n",
            "        let! first = child value\n",
            "        let! second = child first\n",
            "        return seed\n",
        ),
    );
    let machine = parent(&checked);
    let core = checked
        .task_core(machine.definition())
        .expect("matching Task Core");
    let suspensions = machine
        .states()
        .iter()
        .filter(|state| matches!(state.kind(), CheckedTaskMachineStateKind::Suspend { .. }))
        .collect::<Vec<_>>();

    assert_eq!(suspensions.len(), 2);
    for (state, suspension) in suspensions.iter().zip(core.suspensions()) {
        let CheckedTaskMachineStateKind::Suspend {
            suspension: id,
            scope,
            awaited_task,
            continuation,
        } = state.kind()
        else {
            unreachable!();
        };
        assert_eq!(id, suspension.id());
        assert_eq!(scope, suspension.scope());
        assert_eq!(awaited_task, suspension.awaited_task());
        assert_eq!(continuation, suspension.continuation());
        assert_eq!(state.source_span(), Some(suspension.span()));
        assert_eq!(state.frame().len(), suspension.live().len());
        for (slot, live) in state.frame().iter().zip(suspension.live()) {
            assert_eq!(slot.binding(), live.binding());
            assert_eq!(slot.value_type(), live.value_type());
        }
    }
    assert!(machine.edges().iter().any(|edge| {
        edge.from() == suspensions[0].id()
            && edge.to() == suspensions[1].id()
            && edge.kind() == CheckedTaskMachineEdgeKind::Resume
    }));
}

#[test]
fn branch_suspensions_are_alternatives_not_a_false_sequence() {
    let checked = check_source(
        0,
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
    let machine = parent(&checked);
    let suspend_ids = machine
        .states()
        .iter()
        .filter_map(|state| {
            matches!(state.kind(), CheckedTaskMachineStateKind::Suspend { .. })
                .then_some(state.id())
        })
        .collect::<Vec<_>>();

    assert_eq!(suspend_ids.len(), 2);
    for id in &suspend_ids {
        assert!(machine.edges().iter().any(|edge| {
            edge.from() == machine.entry()
                && edge.to() == *id
                && edge.kind() == CheckedTaskMachineEdgeKind::Continue
        }));
    }
    assert!(!machine.edges().iter().any(|edge| {
        suspend_ids.contains(&edge.from())
            && suspend_ids.contains(&edge.to())
            && edge.kind() == CheckedTaskMachineEdgeKind::Resume
    }));
}

#[test]
fn nested_scope_suspension_remains_in_the_owner_machine() {
    let checked = check_source(
        0,
        "nested-scope-task.ling",
        concat!(
            "module Main\n\n",
            "task child value =\n",
            "    scope\n",
            "        return value\n\n",
            "task parent value =\n",
            "    scope\n",
            "        scope\n",
            "            let! observed = child value\n",
            "            return observed\n",
            "        let! outer = child value\n",
            "        return outer\n",
        ),
    );
    let machine = parent(&checked);
    let core = checked
        .task_core(machine.definition())
        .expect("matching Task Core");
    let nested = machine
        .states()
        .iter()
        .find(|state| {
            matches!(
                state.kind(),
                CheckedTaskMachineStateKind::Suspend { scope, .. }
                    if scope != core.root_scope()
            )
        })
        .expect("nested suspension");
    let outer = machine
        .states()
        .iter()
        .find(|state| {
            matches!(
                state.kind(),
                CheckedTaskMachineStateKind::Suspend { scope, .. }
                    if scope == core.root_scope()
            )
        })
        .expect("outer suspension");

    assert_eq!(core.scopes().len(), 2);
    assert!(machine.edges().iter().any(|edge| {
        edge.from() == nested.id()
            && edge.to() == outer.id()
            && edge.kind() == CheckedTaskMachineEdgeKind::Resume
    }));
    assert_eq!(checked.task_machines().len(), checked.task_cores().len());
}

#[test]
fn match_branch_suspensions_converge_only_after_resumption() {
    let checked = check_source(
        0,
        "match-task.ling",
        concat!(
            "module Main\n\n",
            "task child value =\n",
            "    scope\n",
            "        return value\n\n",
            "task parent value =\n",
            "    scope\n",
            "        let handle = spawn child value\n",
            "        let result = match true with\n",
            "            | true -> await handle\n",
            "            | false -> await handle\n",
            "        return result\n",
        ),
    );
    let machine = parent(&checked);
    let suspend_ids = machine
        .states()
        .iter()
        .filter_map(|state| {
            matches!(state.kind(), CheckedTaskMachineStateKind::Suspend { .. })
                .then_some(state.id())
        })
        .collect::<Vec<_>>();

    assert_eq!(suspend_ids.len(), 2);
    assert!(!machine.edges().iter().any(|edge| {
        suspend_ids.contains(&edge.from())
            && suspend_ids.contains(&edge.to())
            && edge.kind() == CheckedTaskMachineEdgeKind::Resume
    }));
    let resumed_targets = suspend_ids
        .iter()
        .map(|id| {
            machine
                .edges()
                .iter()
                .filter(|edge| {
                    edge.from() == *id && edge.kind() == CheckedTaskMachineEdgeKind::Resume
                })
                .map(|edge| edge.to())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(resumed_targets[0], resumed_targets[1]);
}

#[test]
fn canonical_machine_bytes_ignore_paths_source_ids_and_span_encoding() {
    const LF: &str = concat!(
        "module Main\n\n",
        "task 子任务 值 =\n",
        "    scope\n",
        "        return 值\n\n",
        "task 父任务 值 =\n",
        "    scope\n",
        "        let! 结果 = 子任务 值\n",
        "        return 结果\n",
    );
    let crlf = format!("\u{feff}{}", LF.replace('\n', "\r\n"));
    let first = check_source(3, "C:/one/tasks.ling", LF);
    let reconstructed = check_source(91, "D:/elsewhere/tasks.ling", &crlf);
    let bytes = |checked: &CheckedProgram| {
        checked
            .task_machines()
            .values()
            .map(|machine| machine.canonical_bytes(checked.typed()))
            .collect::<Vec<_>>()
    };

    assert_eq!(bytes(&first), bytes(&reconstructed));
    let reconstructed_parent = parent(&reconstructed);
    assert!(reconstructed_parent.source_span().source().get() == 91);
    assert!(reconstructed_parent.states().iter().any(|state| {
        matches!(state.kind(), CheckedTaskMachineStateKind::Suspend { .. })
            && state.source_span().is_some()
    }));
}
