use ling_ast::lower as lower_ast;
use ling_effects::{CheckedProgram, check};
use ling_eval::{
    HostErrorCategory, MemoryConsole, RuntimeFaultKind, TaskPath, TaskRuntime, TaskRuntimeLimits,
    TaskRuntimeState, TaskStepKind, TaskValue,
};
use ling_hir::lower as lower_hir;
use ling_resolve::{DefinitionId, DefinitionKind, resolve};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;
use num_bigint::BigInt;

fn checked(text: &str) -> CheckedProgram {
    checked_at(
        SourceId::new(0),
        "task-runtime.ling",
        text.as_bytes().to_vec(),
    )
}

fn checked_at(source_id: SourceId, source_name: &str, bytes: Vec<u8>) -> CheckedProgram {
    let source = SourceFile::from_bytes(source_id, source_name, bytes).expect("valid source");
    let parsed = parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = lower_ast(&source, &parsed).expect("valid AST");
    let hir = lower_hir(source.name(), &ast).expect("valid HIR");
    let resolved = resolve(vec![hir], "Main").expect("resolved program");
    let typed = check_types(resolved).expect("typed program");
    check(typed).expect("checked program")
}

fn task(checked: &CheckedProgram, name: &str) -> DefinitionId {
    checked
        .task_cores()
        .keys()
        .find(|definition| {
            checked
                .typed()
                .resolved()
                .definition(definition)
                .is_some_and(|info| info.name == name)
        })
        .cloned()
        .expect("Task definition")
}

fn type_definition(checked: &CheckedProgram, name: &str) -> DefinitionId {
    checked
        .typed()
        .resolved()
        .definitions()
        .values()
        .find(|definition| definition.name == name && definition.kind == DefinitionKind::Type)
        .map(|definition| definition.id.clone())
        .expect("type definition")
}

fn value_definition(checked: &CheckedProgram, name: &str) -> DefinitionId {
    checked
        .typed()
        .resolved()
        .definitions()
        .values()
        .find(|definition| definition.name == name && definition.kind == DefinitionKind::Value)
        .map(|definition| definition.id.clone())
        .expect("value definition")
}

fn limits() -> TaskRuntimeLimits {
    TaskRuntimeLimits::new(16, 16, 64, 8)
}

fn integer(value: i64) -> TaskValue {
    TaskValue::Int(BigInt::from(value))
}

#[test]
fn checked_root_runs_only_through_explicit_ready_selections() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task echo value =\n",
        "    scope\n",
        "        return value\n",
    ));
    let root = task(&checked, "echo");
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(&checked, &root, vec![integer(7)], &mut console, limits())
        .expect("runtime");

    assert_eq!(runtime.ready(), [TaskPath::root()]);
    assert!(matches!(
        runtime.step(&TaskPath::root()).expect("open").kind(),
        TaskStepKind::ScopeOpened { scope: 1 }
    ));
    assert!(matches!(
        runtime.step(&TaskPath::root()).expect("return").kind(),
        TaskStepKind::Completed
    ));
    assert_eq!(
        runtime.root_state(),
        TaskRuntimeState::Completed(integer(7))
    );
    assert_eq!(runtime.cleanup_count(&TaskPath::root()), Some(1));
    assert!(runtime.ready().is_empty());
}

#[test]
fn spawn_suspend_wake_and_join_are_scheduler_neutral() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value + 1\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let handle = spawn child value\n",
        "        let result = await handle\n",
        "        return result\n",
    ));
    let root = task(&checked, "parent");
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(&checked, &root, vec![integer(4)], &mut console, limits())
        .expect("runtime");
    let root_path = TaskPath::root();

    runtime.step(&root_path).expect("root scope");
    let child = match runtime.step(&root_path).expect("spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected spawn step: {other:?}"),
    };
    assert_eq!(runtime.ready(), [root_path.clone(), child.clone()]);
    assert!(matches!(
        runtime.step(&root_path).expect("await").kind(),
        TaskStepKind::Suspended { .. }
    ));
    runtime.step(&child).expect("child scope");
    runtime.step(&child).expect("child return");
    assert_eq!(runtime.ready().as_slice(), std::slice::from_ref(&root_path));
    runtime.step(&root_path).expect("parent return");
    assert_eq!(
        runtime.root_state(),
        TaskRuntimeState::Completed(integer(5))
    );
    assert_eq!(runtime.cleanup_count(&child), Some(1));
}

#[test]
fn multiple_same_scope_handles_survive_suspension_without_frame_slots() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let first = spawn child value\n",
        "        let second = spawn child (value + 1)\n",
        "        let left = await first\n",
        "        let right = await second\n",
        "        return left + right\n",
    ));
    let root = task(&checked, "parent");
    let core = checked.task_core(&root).expect("core");
    assert!(core.suspensions()[0].live().is_empty());
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(&checked, &root, vec![integer(10)], &mut console, limits())
        .expect("runtime");
    let root_path = TaskPath::root();

    runtime.step(&root_path).expect("scope");
    let first = match runtime.step(&root_path).expect("first spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    let second = match runtime.step(&root_path).expect("second spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    runtime.step(&root_path).expect("await first");
    runtime.step(&second).expect("second scope");
    runtime.step(&second).expect("second complete");
    runtime.step(&first).expect("first scope");
    runtime.step(&first).expect("first complete");
    runtime.step(&root_path).expect("await second");
    runtime.step(&root_path).expect("return");
    assert_eq!(
        runtime.root_state(),
        TaskRuntimeState::Completed(integer(21))
    );
}

#[test]
fn cancellation_drains_children_and_cleans_each_task_once() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let handle = spawn child value\n",
        "        let result = await handle\n",
        "        return result\n",
    ));
    let root = task(&checked, "parent");
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(&checked, &root, vec![integer(1)], &mut console, limits())
        .expect("runtime");
    let root_path = TaskPath::root();
    runtime.step(&root_path).expect("scope");
    let child = match runtime.step(&root_path).expect("spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };

    runtime.request_cancel(&root_path).expect("cancel request");
    runtime.step(&root_path).expect("propagate");
    runtime.step(&child).expect("child cancel");
    runtime.step(&root_path).expect("root cancel");
    assert_eq!(runtime.root_state(), TaskRuntimeState::Cancelled);
    assert_eq!(runtime.state(&child), Some(TaskRuntimeState::Cancelled));
    assert_eq!(runtime.cleanup_count(&root_path), Some(1));
    assert_eq!(runtime.cleanup_count(&child), Some(1));
}

#[test]
fn sibling_faults_are_aggregated_by_canonical_task_path() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task bad value =\n",
        "    scope\n",
        "        return value / 0\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let first = spawn bad value\n",
        "        let second = spawn bad value\n",
        "        let left = await first\n",
        "        let right = await second\n",
        "        return left + right\n",
    ));
    let root = task(&checked, "parent");
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(&checked, &root, vec![integer(1)], &mut console, limits())
        .expect("runtime");
    let root_path = TaskPath::root();
    runtime.step(&root_path).expect("scope");
    let first = match runtime.step(&root_path).expect("first spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    let second = match runtime.step(&root_path).expect("second spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    runtime.step(&root_path).expect("suspend");
    runtime.step(&first).expect("first scope");
    runtime.step(&first).expect("first fault detection");
    runtime.step(&second).expect("second scope");
    runtime.step(&second).expect("second fault detection");
    runtime.step(&first).expect("first fault cleanup");
    runtime.step(&second).expect("second fault cleanup");
    runtime.step(&root_path).expect("owner fault propagation");

    assert_eq!(
        runtime.root_state(),
        TaskRuntimeState::Faulted { fault_count: 2 }
    );
    let aggregate = runtime.root_fault().expect("aggregate");
    assert!(matches!(
        aggregate.kind,
        RuntimeFaultKind::TaskFaultAggregate {
            ref primary_task,
            fault_count: 2,
            ..
        } if primary_task == &first.to_string()
    ));
}

#[test]
fn host_effect_is_a_step_boundary_and_resource_failures_are_faults() {
    let checked_program = checked(concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "task writer text =\n",
        "    scope\n",
        "        Console.write text\n",
        "        return text\n",
    ));
    let root = task(&checked_program, "writer");
    let mut console = MemoryConsole::default();
    {
        let mut runtime = TaskRuntime::new(
            &checked_program,
            &root,
            vec![TaskValue::Text("ok".to_owned())],
            &mut console,
            limits(),
        )
        .expect("runtime");
        runtime.step(&TaskPath::root()).expect("scope");
        assert!(matches!(
            runtime.step(&TaskPath::root()).expect("effect").kind(),
            TaskStepKind::HostEffectCompleted
        ));
        runtime.step(&TaskPath::root()).expect("return");
    }
    assert_eq!(console.output(), "ok\n");

    let spawn_checked = checked(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let handle = spawn child value\n",
        "        let result = await handle\n",
        "        return result\n",
    ));
    let parent = task(&spawn_checked, "parent");
    let mut resource_console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(
        &spawn_checked,
        &parent,
        vec![integer(1)],
        &mut resource_console,
        TaskRuntimeLimits::new(1, 4, 16, 4),
    )
    .expect("bounded runtime");
    runtime.step(&TaskPath::root()).expect("scope");
    runtime.step(&TaskPath::root()).expect("task limit fault");
    runtime.step(&TaskPath::root()).expect("fault cleanup");
    let fault = runtime.root_fault().expect("root fault");
    let causes = runtime.faults(&TaskPath::root()).expect("causes");
    assert!(matches!(
        causes[0].1.kind,
        RuntimeFaultKind::TaskResourceLimit {
            resource: "runtime_tasks",
            limit: 1
        }
    ));
    assert!(matches!(
        fault.kind,
        RuntimeFaultKind::TaskFaultAggregate { fault_count: 1, .. }
    ));
}

#[test]
fn fused_let_await_executes_as_distinct_spawn_and_suspend_boundaries() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value + 1\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let! result = child value\n",
        "        return result\n",
    ));
    let root = task(&checked, "parent");
    let root_path = TaskPath::root();
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(&checked, &root, vec![integer(8)], &mut console, limits())
        .expect("runtime");

    runtime.step(&root_path).expect("scope");
    let child = match runtime.step(&root_path).expect("fused spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected fused spawn: {other:?}"),
    };
    assert!(matches!(
        runtime.step(&root_path).expect("fused await").kind(),
        TaskStepKind::Suspended { .. }
    ));
    runtime.step(&child).expect("child scope");
    runtime.step(&child).expect("child return");
    runtime.step(&root_path).expect("parent return");
    assert_eq!(
        runtime.root_state(),
        TaskRuntimeState::Completed(integer(9))
    );
}

#[test]
fn cancellation_is_idempotent_before_start_and_while_suspended() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let! result = child value\n",
        "        return result\n",
    ));
    let parent = task(&checked, "parent");
    let root = TaskPath::root();

    let mut prestart_console = MemoryConsole::default();
    let mut prestart = TaskRuntime::new(
        &checked,
        &parent,
        vec![integer(1)],
        &mut prestart_console,
        limits(),
    )
    .expect("runtime");
    prestart.request_cancel(&root).expect("first request");
    prestart.request_cancel(&root).expect("idempotent request");
    prestart.step(&root).expect("cancel before start");
    assert_eq!(prestart.root_state(), TaskRuntimeState::Cancelled);
    assert_eq!(prestart.cleanup_count(&root), Some(1));

    let mut suspended_console = MemoryConsole::default();
    let mut suspended = TaskRuntime::new(
        &checked,
        &parent,
        vec![integer(2)],
        &mut suspended_console,
        limits(),
    )
    .expect("runtime");
    suspended.step(&root).expect("scope");
    let child = match suspended.step(&root).expect("spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    suspended.step(&root).expect("suspend");
    assert!(matches!(
        suspended.root_state(),
        TaskRuntimeState::Suspended { .. }
    ));
    suspended.request_cancel(&root).expect("cancel suspended");
    suspended.step(&root).expect("propagate");
    suspended.step(&child).expect("cancel child");
    suspended.step(&root).expect("cancel root");
    assert_eq!(suspended.root_state(), TaskRuntimeState::Cancelled);
    assert_eq!(suspended.cleanup_count(&child), Some(1));
}

#[test]
fn cancellation_after_host_effect_preserves_committed_output() {
    let checked = checked(concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "task writer text =\n",
        "    scope\n",
        "        Console.write text\n",
        "        return text\n",
    ));
    let writer = task(&checked, "writer");
    let root = TaskPath::root();
    let mut console = MemoryConsole::default();
    {
        let mut runtime = TaskRuntime::new(
            &checked,
            &writer,
            vec![TaskValue::Text("committed".to_owned())],
            &mut console,
            limits(),
        )
        .expect("runtime");
        runtime.step(&root).expect("scope");
        runtime.step(&root).expect("host effect");
        runtime.request_cancel(&root).expect("cancel");
        runtime.step(&root).expect("cleanup");
        assert_eq!(runtime.root_state(), TaskRuntimeState::Cancelled);
        assert_eq!(runtime.cleanup_count(&root), Some(1));
    }
    assert_eq!(console.output(), "committed\n");
}

#[test]
fn invalid_driver_selection_is_atomic() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let! result = child value\n",
        "        return result\n",
    ));
    let parent = task(&checked, "parent");
    let root = TaskPath::root();
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(&checked, &parent, vec![integer(1)], &mut console, limits())
        .expect("runtime");
    runtime.step(&root).expect("scope");
    let child = match runtime.step(&root).expect("spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    runtime.step(&root).expect("suspend");
    let before_state = runtime.root_state();
    let before_ready = runtime.ready();
    let fault = runtime
        .step(&root)
        .expect_err("suspended root is not ready");
    assert!(matches!(
        fault.kind,
        RuntimeFaultKind::TaskDriver {
            reason: "task_not_ready",
            ..
        }
    ));
    assert_eq!(runtime.root_state(), before_state);
    assert_eq!(runtime.ready(), before_ready);
    assert_eq!(runtime.cleanup_count(&root), Some(0));
    assert_eq!(runtime.cleanup_count(&child), Some(0));
}

#[test]
fn every_explicit_runtime_bound_faults_at_its_boundary() {
    let echo_checked = checked(concat!(
        "module Main\n\n",
        "task echo value =\n",
        "    scope\n",
        "        return value\n",
    ));
    let echo = task(&echo_checked, "echo");
    for limits in [
        TaskRuntimeLimits::new(0, 1, 1, 1),
        TaskRuntimeLimits::new(1, 0, 1, 1),
        TaskRuntimeLimits::new(1, 1, 0, 1),
        TaskRuntimeLimits::new(1, 1, 1, 0),
    ] {
        let mut console = MemoryConsole::default();
        let fault = TaskRuntime::new(&echo_checked, &echo, vec![integer(1)], &mut console, limits)
            .err()
            .expect("zero bound rejected");
        assert!(matches!(
            fault.kind,
            RuntimeFaultKind::InvalidCheckedCore {
                invariant: "TaskRuntime limits must all be non-zero"
            }
        ));
    }

    let mut step_console = MemoryConsole::default();
    let mut step_limited = TaskRuntime::new(
        &echo_checked,
        &echo,
        vec![integer(1)],
        &mut step_console,
        TaskRuntimeLimits::new(2, 2, 1, 2),
    )
    .expect("runtime");
    step_limited.step(&TaskPath::root()).expect("first step");
    step_limited
        .step(&TaskPath::root())
        .expect("step-limit fault cleanup");
    assert!(matches!(
        step_limited.faults(&TaskPath::root()).unwrap()[0].1.kind,
        RuntimeFaultKind::TaskResourceLimit {
            resource: "lifecycle_steps",
            limit: 1
        }
    ));

    let nested_checked = checked(concat!(
        "module Main\n\n",
        "task nested value =\n",
        "    scope\n",
        "        scope\n",
        "            return value\n",
        "        return value\n",
    ));
    let nested = task(&nested_checked, "nested");
    let mut scope_console = MemoryConsole::default();
    let mut scope_limited = TaskRuntime::new(
        &nested_checked,
        &nested,
        vec![integer(1)],
        &mut scope_console,
        TaskRuntimeLimits::new(2, 1, 8, 2),
    )
    .expect("runtime");
    scope_limited.step(&TaskPath::root()).expect("outer scope");
    scope_limited
        .step(&TaskPath::root())
        .expect("scope-limit detection");
    scope_limited
        .step(&TaskPath::root())
        .expect("scope-limit cleanup");
    assert!(matches!(
        scope_limited.faults(&TaskPath::root()).unwrap()[0].1.kind,
        RuntimeFaultKind::TaskResourceLimit {
            resource: "runtime_scopes",
            limit: 1
        }
    ));
}

fn drive_two_faults(second_first: bool) -> (Vec<(TaskPath, ling_eval::RuntimeFault)>, Vec<usize>) {
    let checked = checked(concat!(
        "module Main\n\n",
        "task bad value =\n",
        "    scope\n",
        "        return value / 0\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let first = spawn bad value\n",
        "        let second = spawn bad value\n",
        "        let left = await first\n",
        "        let right = await second\n",
        "        return left + right\n",
    ));
    let parent = task(&checked, "parent");
    let root = TaskPath::root();
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(&checked, &parent, vec![integer(1)], &mut console, limits())
        .expect("runtime");
    runtime.step(&root).expect("scope");
    let first = match runtime.step(&root).expect("spawn first").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    let second = match runtime.step(&root).expect("spawn second").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    runtime.step(&root).expect("suspend");
    runtime.step(&first).expect("first scope");
    runtime.step(&second).expect("second scope");
    let order = if second_first {
        [second.clone(), first.clone()]
    } else {
        [first.clone(), second.clone()]
    };
    for child in &order {
        runtime.step(child).expect("fault detection");
    }
    assert!(runtime.ready().contains(&root));
    for child in order.iter().rev() {
        runtime.step(child).expect("fault cleanup");
    }
    runtime.step(&root).expect("owner propagation");
    (
        runtime.faults(&root).expect("fault set"),
        vec![
            runtime.cleanup_count(&root).unwrap(),
            runtime.cleanup_count(&first).unwrap(),
            runtime.cleanup_count(&second).unwrap(),
        ],
    )
}

#[test]
fn opposite_explicit_schedules_produce_the_same_canonical_fault_set() {
    let (left_faults, left_cleanup) = drive_two_faults(false);
    let (right_faults, right_cleanup) = drive_two_faults(true);
    assert_eq!(left_faults, right_faults);
    assert_eq!(left_cleanup, [1, 1, 1]);
    assert_eq!(left_cleanup, right_cleanup);
}

#[test]
fn fault_dominates_a_cancellation_requested_during_drain() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task bad value =\n",
        "    scope\n",
        "        return value / 0\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let! result = bad value\n",
        "        return result\n",
    ));
    let parent = task(&checked, "parent");
    let root = TaskPath::root();
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(&checked, &parent, vec![integer(1)], &mut console, limits())
        .expect("runtime");
    runtime.step(&root).expect("scope");
    let child = match runtime.step(&root).expect("spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    runtime.step(&root).expect("suspend");
    runtime.step(&child).expect("child scope");
    runtime.step(&child).expect("fault detection");
    assert!(runtime.ready().contains(&root));
    runtime.request_cancel(&root).expect("late cancellation");
    runtime.step(&child).expect("child cleanup");
    runtime.step(&root).expect("fault cleanup");
    assert!(matches!(
        runtime.root_state(),
        TaskRuntimeState::Faulted { fault_count: 1 }
    ));
}

#[test]
fn fault_spans_preserve_bom_crlf_unicode_bytes_and_bilingual_diagnostics() {
    let source = concat!(
        "\u{feff}module Main\r\n\r\n",
        "task 错误 值 =\r\n",
        "    scope\r\n",
        "        return 值 / 0\r\n",
    );
    let checked = checked_at(
        SourceId::new(73),
        "物理/路径/任务.ling",
        source.as_bytes().to_vec(),
    );
    let root_definition = task(&checked, "错误");
    let root = TaskPath::root();
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(
        &checked,
        &root_definition,
        vec![integer(1)],
        &mut console,
        limits(),
    )
    .expect("runtime");
    runtime.step(&root).expect("scope");
    runtime.step(&root).expect("fault detection");
    runtime.step(&root).expect("fault cleanup");
    let aggregate = runtime.root_fault().expect("aggregate");
    assert_eq!(aggregate.source_name, "物理/路径/任务.ling");
    assert_eq!(aggregate.span.source(), SourceId::new(73));
    assert_eq!(
        aggregate.span.start().get() as usize,
        source.find("值 / 0").expect("fault expression")
    );
    let diagnostic = aggregate.to_diagnostic();
    assert!(!diagnostic.message_zh().is_empty());
    assert!(!diagnostic.message_en().is_empty());
    assert_eq!(diagnostic.facts()["category"], "task_fault_aggregate");
    assert_eq!(diagnostic.facts()["fault_count"], "1");
}

#[test]
fn retained_fault_limit_is_bounded_and_deterministic() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task bad value =\n",
        "    scope\n",
        "        return value / 0\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let first = spawn bad value\n",
        "        let second = spawn bad value\n",
        "        let left = await first\n",
        "        let right = await second\n",
        "        return left + right\n",
    ));
    let parent = task(&checked, "parent");
    let root = TaskPath::root();
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(
        &checked,
        &parent,
        vec![integer(1)],
        &mut console,
        TaskRuntimeLimits::new(4, 4, 32, 1),
    )
    .expect("runtime");
    runtime.step(&root).expect("scope");
    let first = match runtime.step(&root).expect("first spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    let second = match runtime.step(&root).expect("second spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    runtime.step(&root).expect("suspend");
    runtime.step(&first).expect("first scope");
    runtime.step(&second).expect("second scope");
    runtime.step(&first).expect("first fault");
    runtime.step(&second).expect("second fault");
    runtime.step(&first).expect("first cleanup");
    runtime.step(&second).expect("second cleanup");
    runtime.step(&root).expect("root cleanup");
    let faults = runtime.faults(&root).expect("bounded fault set");
    assert_eq!(faults.len(), 1);
    assert!(matches!(
        faults[0].1.kind,
        RuntimeFaultKind::TaskResourceLimit {
            resource: "retained_faults",
            limit: 1
        }
    ));
}

#[test]
fn polymorphic_boundary_rejects_a_non_ling_heterogeneous_list() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task echo value =\n",
        "    scope\n",
        "        return value\n",
    ));
    let echo = task(&checked, "echo");
    let mut console = MemoryConsole::default();
    let result = TaskRuntime::new(
        &checked,
        &echo,
        vec![TaskValue::List(vec![integer(1), TaskValue::Bool(true)])],
        &mut console,
        limits(),
    );
    let fault = result.err().expect("heterogeneous list rejected");
    assert!(matches!(
        fault.kind,
        RuntimeFaultKind::InvalidCheckedCore {
            invariant: "TaskRuntime argument type disagrees with Checked Task signature"
        }
    ));
}

#[test]
fn checked_argument_boundary_validates_nominal_record_shape() {
    let checked = checked(concat!(
        "module Main\n\n",
        "type Point = { x: Int; y: Int }\n\n",
        "task echo value =\n",
        "    scope\n",
        "        return value\n",
    ));
    let echo = task(&checked, "echo");
    let point = type_definition(&checked, "Point");
    let mut console = MemoryConsole::default();
    let missing_field = TaskValue::Record {
        definition: point.clone(),
        fields: [("x".to_owned(), integer(1))].into(),
    };
    let fault = TaskRuntime::new(&checked, &echo, vec![missing_field], &mut console, limits())
        .err()
        .expect("incomplete record rejected");
    assert!(matches!(
        fault.kind,
        RuntimeFaultKind::InvalidCheckedCore {
            invariant: "TaskRuntime argument type disagrees with Checked Task signature"
        }
    ));

    let valid = TaskValue::Record {
        definition: point,
        fields: [("x".to_owned(), integer(1)), ("y".to_owned(), integer(2))].into(),
    };
    let mut valid_console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(
        &checked,
        &echo,
        vec![valid.clone()],
        &mut valid_console,
        limits(),
    )
    .expect("complete record accepted");
    runtime.step(&TaskPath::root()).expect("scope");
    runtime.step(&TaskPath::root()).expect("return");
    assert_eq!(runtime.root_state(), TaskRuntimeState::Completed(valid));
}

#[test]
fn host_failure_becomes_a_task_fault_without_fabricated_output() {
    let checked = checked(concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "task writer text =\n",
        "    scope\n",
        "        Console.write text\n",
        "        return text\n",
    ));
    let writer = task(&checked, "writer");
    let root = TaskPath::root();
    let mut console = MemoryConsole::failing(HostErrorCategory::PermissionDenied);
    let mut runtime = TaskRuntime::new(
        &checked,
        &writer,
        vec![TaskValue::Text("never".to_owned())],
        &mut console,
        limits(),
    )
    .expect("runtime");
    runtime.step(&root).expect("scope");
    runtime.step(&root).expect("host fault detection");
    runtime.step(&root).expect("fault cleanup");
    let faults = runtime.faults(&root).expect("faults");
    assert!(matches!(
        faults[0].1.kind,
        RuntimeFaultKind::HostCapability {
            operation: "Console.write",
            category: HostErrorCategory::PermissionDenied
        }
    ));
    drop(runtime);
    assert_eq!(console.output(), "");
}

#[test]
fn transitive_child_fault_propagates_before_descendant_cleanup_finishes() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task leaf value =\n",
        "    scope\n",
        "        return value / 0\n\n",
        "task middle value =\n",
        "    scope\n",
        "        let! result = leaf value\n",
        "        return result\n\n",
        "task root_task value =\n",
        "    scope\n",
        "        let! result = middle value\n",
        "        return result\n",
    ));
    let definition = task(&checked, "root_task");
    let root = TaskPath::root();
    let mut console = MemoryConsole::default();
    let mut runtime = TaskRuntime::new(
        &checked,
        &definition,
        vec![integer(1)],
        &mut console,
        limits(),
    )
    .expect("runtime");
    runtime.step(&root).expect("root scope");
    let middle = match runtime.step(&root).expect("middle spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    runtime.step(&root).expect("root suspend");
    runtime.step(&middle).expect("middle scope");
    let leaf = match runtime.step(&middle).expect("leaf spawn").kind() {
        TaskStepKind::ChildRegistered { child } => child.clone(),
        other => panic!("unexpected: {other:?}"),
    };
    runtime.step(&middle).expect("middle suspend");
    runtime.step(&leaf).expect("leaf scope");
    runtime.step(&leaf).expect("leaf fault detection");
    assert!(runtime.ready().contains(&middle));
    runtime.step(&middle).expect("middle propagation");
    assert!(runtime.ready().contains(&root));
    runtime.step(&root).expect("root propagation");
    runtime.step(&leaf).expect("leaf cleanup");
    runtime.step(&middle).expect("middle cleanup");
    runtime.step(&root).expect("root cleanup");
    assert_eq!(runtime.cleanup_count(&leaf), Some(1));
    assert_eq!(runtime.cleanup_count(&middle), Some(1));
    assert_eq!(runtime.cleanup_count(&root), Some(1));
    assert!(matches!(
        runtime.root_state(),
        TaskRuntimeState::Faulted { fault_count: 1 }
    ));
    assert_eq!(runtime.faults(&root).unwrap()[0].0, leaf);
}

#[test]
fn construction_rejects_non_task_roots_and_signature_mismatches() {
    let checked = checked(concat!(
        "module Main\n\n",
        "let ordinary = 1\n\n",
        "task increment value =\n",
        "    scope\n",
        "        return value + 1\n",
    ));
    let ordinary = value_definition(&checked, "ordinary");
    let increment = task(&checked, "increment");

    let mut non_task_console = MemoryConsole::default();
    let non_task = TaskRuntime::new(&checked, &ordinary, vec![], &mut non_task_console, limits())
        .err()
        .expect("non-Task root rejected");
    assert!(matches!(
        non_task.kind,
        RuntimeFaultKind::InvalidCheckedCore {
            invariant: "TaskRuntime root has no Checked Task Core"
        }
    ));

    let mut arity_console = MemoryConsole::default();
    let arity = TaskRuntime::new(&checked, &increment, vec![], &mut arity_console, limits())
        .err()
        .expect("arity mismatch rejected");
    assert!(matches!(
        arity.kind,
        RuntimeFaultKind::InvalidCheckedCore {
            invariant: "TaskRuntime argument arity disagrees with Checked Task signature"
        }
    ));

    let mut type_console = MemoryConsole::default();
    let argument_type = TaskRuntime::new(
        &checked,
        &increment,
        vec![TaskValue::Bool(true)],
        &mut type_console,
        limits(),
    )
    .err()
    .expect("type mismatch rejected");
    assert!(matches!(
        argument_type.kind,
        RuntimeFaultKind::InvalidCheckedCore {
            invariant: "TaskRuntime argument type disagrees with Checked Task signature"
        }
    ));
}
