use ling_ast::lower as lower_ast;
use ling_effects::{CheckedProgram, check};
use ling_eval::{
    HostErrorCategory, TaskDeadline, TaskExplorationLimit, TaskHostOutcome, TaskHostResponse,
    TaskHostScript, TaskPath, TaskRuntimeLimits, TaskScheduleConfig, TaskScheduleEventKind,
    TaskScheduleTerminal, TaskSchedulerError, TaskSchedulerLimits, TaskValue,
    explore_task_schedules, replay_task_schedule, run_task_schedule,
};
use ling_hir::lower as lower_hir;
use ling_resolve::{DefinitionId, resolve};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;
use num_bigint::BigInt;

fn checked(text: &str) -> CheckedProgram {
    checked_at(SourceId::new(0), "task-scheduler.ling", text)
}

fn checked_at(source_id: SourceId, source_name: &str, text: &str) -> CheckedProgram {
    let source = SourceFile::from_bytes(source_id, source_name, text.as_bytes().to_vec())
        .expect("valid source");
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

fn scheduler_limits() -> TaskSchedulerLimits {
    TaskSchedulerLimits::new(128, 128, 16, 512, 2_048, 128, 16)
}

fn config(seed: u64) -> TaskScheduleConfig {
    TaskScheduleConfig::new(
        seed,
        TaskRuntimeLimits::new(32, 32, 256, 16),
        scheduler_limits(),
    )
}

fn integer(value: i64) -> TaskValue {
    TaskValue::Int(BigInt::from(value))
}

fn terminal(trace: &ling_eval::TaskScheduleTrace) -> &TaskScheduleTerminal {
    match trace.events().last().expect("closure").kind() {
        TaskScheduleEventKind::Closure { terminal, .. } => terminal,
        other => panic!("expected closure, found {other:?}"),
    }
}

fn closure_faults(trace: &ling_eval::TaskScheduleTrace) -> &[ling_eval::TaskFaultSummary] {
    match trace.events().last().expect("closure").kind() {
        TaskScheduleEventKind::Closure { faults, .. } => faults,
        other => panic!("expected closure, found {other:?}"),
    }
}

const ECHO: &str = concat!(
    "module Main\n\n",
    "task echo value =\n",
    "    scope\n",
    "        return value\n",
);

const PARENT: &str = concat!(
    "module Main\n\n",
    "task child value =\n",
    "    scope\n",
    "        return value + 1\n\n",
    "task parent value =\n",
    "    scope\n",
    "        let handle = spawn child value\n",
    "        let result = await handle\n",
    "        return result\n",
);

const WRITER: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "task writer text =\n",
    "    scope\n",
    "        Console.write text\n",
    "        return text\n",
);

#[test]
fn repeated_seed_and_reconstructed_checked_input_have_identical_trace_bytes() {
    let left = checked_at(SourceId::new(1), "left/task.ling", PARENT);
    let right = checked_at(SourceId::new(99), "different/right.ling", PARENT);
    let left_root = task(&left, "parent");
    let right_root = task(&right, "parent");

    let first = run_task_schedule(
        &left,
        &left_root,
        vec![integer(4)],
        config(0x1234_5678),
        vec![],
        TaskHostScript::default(),
    )
    .expect("first run");
    let repeated = run_task_schedule(
        &left,
        &left_root,
        vec![integer(4)],
        config(0x1234_5678),
        vec![],
        TaskHostScript::default(),
    )
    .expect("repeated run");
    let reconstructed = run_task_schedule(
        &right,
        &right_root,
        vec![integer(4)],
        config(0x1234_5678),
        vec![],
        TaskHostScript::default(),
    )
    .expect("reconstructed run");

    assert_eq!(first.canonical_bytes(), repeated.canonical_bytes());
    assert_eq!(first.canonical_bytes(), reconstructed.canonical_bytes());
    assert_eq!(
        terminal(&first),
        &TaskScheduleTerminal::Completed(integer(5))
    );
}

#[test]
fn unicode_bom_crlf_reconstruction_preserves_logical_trace_identity() {
    let lf = concat!(
        "module Main\n\n",
        "task 工作 value =\n",
        "    scope\n",
        "        return value\n",
    );
    let bom_crlf = concat!(
        "\u{feff}module Main\r\n\r\n",
        "task 工作 value =\r\n",
        "    scope\r\n",
        "        return value\r\n",
    );
    let left = checked_at(SourceId::new(7), "左侧.ling", lf);
    let right = checked_at(SourceId::new(77), "目录/右侧.ling", bom_crlf);
    let left_root = task(&left, "工作");
    let right_root = task(&right, "工作");
    let left_trace = run_task_schedule(
        &left,
        &left_root,
        vec![integer(6)],
        config(42),
        vec![],
        TaskHostScript::default(),
    )
    .expect("LF run");
    let right_trace = run_task_schedule(
        &right,
        &right_root,
        vec![integer(6)],
        config(42),
        vec![],
        TaskHostScript::default(),
    )
    .expect("BOM/CRLF run");
    assert_eq!(left_trace.canonical_bytes(), right_trace.canonical_bytes());
}

#[test]
fn fault_source_spans_remain_sidecar_evidence_not_logical_trace_bytes() {
    let lf = concat!(
        "module Main\n\n",
        "task 错误 value =\n",
        "    scope\n",
        "        return value / 0\n",
    );
    let bom_crlf = concat!(
        "\u{feff}module Main\r\n\r\n",
        "task 错误 value =\r\n",
        "    scope\r\n",
        "        return value / 0\r\n",
    );
    let left = checked_at(SourceId::new(8), "left-fault.ling", lf);
    let right = checked_at(SourceId::new(88), "目录/right-fault.ling", bom_crlf);
    let left_root = task(&left, "错误");
    let right_root = task(&right, "错误");
    let left_trace = run_task_schedule(
        &left,
        &left_root,
        vec![integer(6)],
        config(42),
        vec![],
        TaskHostScript::default(),
    )
    .expect("LF fault run");
    let right_trace = run_task_schedule(
        &right,
        &right_root,
        vec![integer(6)],
        config(42),
        vec![],
        TaskHostScript::default(),
    )
    .expect("BOM/CRLF fault run");
    let left_fault = &closure_faults(&left_trace)[0];
    let right_fault = &closure_faults(&right_trace)[0];
    assert_eq!(left_fault.source_name(), "left-fault.ling");
    assert_eq!(right_fault.source_name(), "目录/right-fault.ling");
    assert_eq!(
        left_fault.source_span().start().get(),
        lf.find("value / 0").expect("fault expression") as u32
    );
    assert_eq!(
        right_fault.source_span().start().get(),
        bom_crlf.find("value / 0").expect("fault expression") as u32
    );
    assert_eq!(left_trace.canonical_bytes(), right_trace.canonical_bytes());
}

#[test]
fn deadline_at_zero_cancels_before_the_first_runtime_step() {
    let checked = checked(ECHO);
    let root = task(&checked, "echo");
    let trace = run_task_schedule(
        &checked,
        &root,
        vec![integer(7)],
        config(7),
        vec![TaskDeadline::new(0, TaskPath::root())],
        TaskHostScript::default(),
    )
    .expect("deadline run");

    assert!(matches!(
        trace.events()[0].kind(),
        TaskScheduleEventKind::Deadline {
            task,
            applied: true
        } if task == &TaskPath::root()
    ));
    assert_eq!(terminal(&trace), &TaskScheduleTerminal::Cancelled);
}

#[test]
fn terminal_deadline_is_recorded_without_changing_the_completed_task() {
    let checked = checked(ECHO);
    let root = task(&checked, "echo");
    let trace = run_task_schedule(
        &checked,
        &root,
        vec![integer(7)],
        config(7),
        vec![TaskDeadline::new(2, TaskPath::root())],
        TaskHostScript::default(),
    )
    .expect("terminal deadline run");
    assert!(trace.events().iter().any(|event| matches!(
        event.kind(),
        TaskScheduleEventKind::Deadline {
            task,
            applied: false
        } if task == &TaskPath::root()
    )));
    assert_eq!(
        terminal(&trace),
        &TaskScheduleTerminal::Completed(integer(7))
    );
}

#[test]
fn equal_tick_deadlines_are_canonicalized_by_task_path() {
    let checked = checked(PARENT);
    let root = task(&checked, "parent");
    let child = TaskPath::from_segments([2]).expect("canonical child path");
    let trace = run_task_schedule(
        &checked,
        &root,
        vec![integer(1)],
        config(5),
        vec![
            TaskDeadline::new(2, child.clone()),
            TaskDeadline::new(2, TaskPath::root()),
        ],
        TaskHostScript::default(),
    )
    .expect("equal-tick deadlines");
    assert_eq!(trace.deadlines()[0].task(), &TaskPath::root());
    assert_eq!(trace.deadlines()[1].task(), &child);
    let due = trace
        .events()
        .iter()
        .filter_map(|event| match event.kind() {
            TaskScheduleEventKind::Deadline { task, .. } => Some(task.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(due, [TaskPath::root(), child]);
}

#[test]
fn unknown_due_child_is_rejected_without_executing_the_root() {
    let checked = checked(PARENT);
    let root = task(&checked, "parent");
    let child = TaskPath::from_segments([2]).expect("canonical child path");
    let error = run_task_schedule(
        &checked,
        &root,
        vec![integer(1)],
        config(1),
        vec![TaskDeadline::new(0, child.clone())],
        TaskHostScript::default(),
    )
    .expect_err("unregistered child deadline");
    assert!(matches!(
        error,
        TaskSchedulerError::UnknownDeadlineTask { tick: 0, task } if task == child
    ));
}

#[test]
fn host_success_and_failure_are_canonical_trace_events() {
    let checked = checked(WRITER);
    let root = task(&checked, "writer");
    for (response, expected, faulted) in [
        (
            TaskHostResponse::Complete,
            TaskHostOutcome::Completed,
            false,
        ),
        (
            TaskHostResponse::Fail(HostErrorCategory::BrokenPipe),
            TaskHostOutcome::Failed(HostErrorCategory::BrokenPipe),
            true,
        ),
    ] {
        let trace = run_task_schedule(
            &checked,
            &root,
            vec![TaskValue::Text("你好".to_owned())],
            config(9),
            vec![],
            TaskHostScript::new([response]),
        )
        .expect("host run is contained");
        assert!(trace.events().iter().any(|event| matches!(
            event.kind(),
            TaskScheduleEventKind::Host { text, outcome }
                if text == "你好\n" && outcome == &expected
        )));
        assert_eq!(
            matches!(terminal(&trace), TaskScheduleTerminal::Faulted { .. }),
            faulted
        );
    }
}

#[test]
fn host_panic_becomes_a_runtime_fault_without_a_guessed_host_event() {
    let checked = checked(WRITER);
    let root = task(&checked, "writer");
    let trace = run_task_schedule(
        &checked,
        &root,
        vec![TaskValue::Text("panic".to_owned())],
        config(9),
        vec![],
        TaskHostScript::new([TaskHostResponse::Panic]),
    )
    .expect("test-host panic is contained");

    assert!(
        !trace
            .events()
            .iter()
            .any(|event| matches!(event.kind(), TaskScheduleEventKind::Host { .. }))
    );
    assert!(matches!(
        terminal(&trace),
        TaskScheduleTerminal::Faulted { fault_count: 1 }
    ));
    assert_eq!(closure_faults(&trace)[0].category(), "host_capability");
    assert_eq!(closure_faults(&trace)[0].detail(), "other");
}

#[test]
fn replay_reconstructs_the_same_trace_without_seed_fallback() {
    let original = checked_at(SourceId::new(3), "original.ling", PARENT);
    let reconstructed = checked_at(SourceId::new(44), "elsewhere/replay.ling", PARENT);
    let original_root = task(&original, "parent");
    let replay_root = task(&reconstructed, "parent");
    let trace = run_task_schedule(
        &original,
        &original_root,
        vec![integer(10)],
        config(u64::MAX),
        vec![],
        TaskHostScript::default(),
    )
    .expect("recorded run");
    let replayed = replay_task_schedule(&reconstructed, &replay_root, vec![integer(10)], &trace)
        .expect("strict replay");
    assert_eq!(trace.canonical_bytes(), replayed.canonical_bytes());

    let mismatch = replay_task_schedule(&reconstructed, &replay_root, vec![integer(11)], &trace)
        .expect_err("changed recipe identity");
    assert_eq!(mismatch.event_id(), 0);
    assert_eq!(mismatch.reason(), "runtime_identity_mismatch");
}

#[test]
fn breadth_first_exploration_is_complete_and_canonical_when_bounded_sufficiently() {
    let checked = checked(PARENT);
    let root = task(&checked, "parent");
    let result = explore_task_schedules(
        &checked,
        &root,
        vec![integer(2)],
        config(0),
        vec![],
        TaskHostScript::default(),
    )
    .expect("bounded exploration");
    assert!(result.is_complete());
    assert!(result.runs() > result.traces().len());
    assert!(result.traces().len() > 1);
    assert!(
        result
            .traces()
            .iter()
            .all(|trace| terminal(trace) == &TaskScheduleTerminal::Completed(integer(3)))
    );
    assert!(result.first_failure().is_none());
}

#[test]
fn exploration_depth_exhaustion_is_explicitly_incomplete() {
    let checked = checked(ECHO);
    let root = task(&checked, "echo");
    let limits = TaskSchedulerLimits::new(16, 16, 4, 64, 32, 1, 4);
    let result = explore_task_schedules(
        &checked,
        &root,
        vec![integer(2)],
        TaskScheduleConfig::new(0, TaskRuntimeLimits::new(8, 8, 32, 8), limits),
        vec![],
        TaskHostScript::default(),
    )
    .expect("bounded result");
    assert!(!result.is_complete());
    assert_eq!(result.limit(), Some(TaskExplorationLimit::Depth));
}

#[test]
fn every_zero_scheduler_limit_is_rejected_before_a_run() {
    let checked = checked(ECHO);
    let root = task(&checked, "echo");
    let base = [16usize, 16, 4, 64, 32, 16, 4];
    for index in 0..base.len() {
        let mut values = base;
        values[index] = 0;
        let limits = TaskSchedulerLimits::new(
            values[0],
            values[1] as u64,
            values[2],
            values[3],
            values[4],
            values[5],
            values[6],
        );
        assert!(matches!(
            run_task_schedule(
                &checked,
                &root,
                vec![integer(1)],
                TaskScheduleConfig::new(0, TaskRuntimeLimits::new(8, 8, 32, 8), limits),
                vec![],
                TaskHostScript::default(),
            ),
            Err(TaskSchedulerError::InvalidLimit { .. })
        ));
    }
}

#[test]
fn decision_tick_trace_ready_and_exploration_run_bounds_are_explicit() {
    let echo = checked(ECHO);
    let echo_root = task(&echo, "echo");
    let runtime = TaskRuntimeLimits::new(8, 8, 32, 8);
    for (limits, expected) in [
        (
            TaskSchedulerLimits::new(1, 16, 4, 64, 32, 16, 4),
            "decision",
        ),
        (TaskSchedulerLimits::new(16, 1, 4, 64, 32, 16, 4), "tick"),
        (TaskSchedulerLimits::new(16, 16, 4, 3, 32, 16, 4), "trace"),
    ] {
        let error = run_task_schedule(
            &echo,
            &echo_root,
            vec![integer(1)],
            TaskScheduleConfig::new(0, runtime, limits),
            vec![],
            TaskHostScript::default(),
        )
        .expect_err("bound must terminate the run");
        assert!(
            matches!(
                (&error, expected),
                (TaskSchedulerError::DecisionLimit { .. }, "decision")
                    | (TaskSchedulerError::TickLimit { .. }, "tick")
                    | (TaskSchedulerError::TraceEventLimit { .. }, "trace")
            ),
            "unexpected {expected} error: {error:?}"
        );
    }

    let parent = checked(PARENT);
    let parent_root = task(&parent, "parent");
    let width_error = run_task_schedule(
        &parent,
        &parent_root,
        vec![integer(1)],
        TaskScheduleConfig::new(
            0,
            runtime,
            TaskSchedulerLimits::new(32, 32, 4, 128, 32, 32, 1),
        ),
        vec![],
        TaskHostScript::default(),
    )
    .expect_err("two ready Tasks exceed width one");
    assert!(matches!(
        width_error,
        TaskSchedulerError::ReadyWidthLimit { width: 2, limit: 1 }
    ));

    let run_limited = explore_task_schedules(
        &echo,
        &echo_root,
        vec![integer(1)],
        TaskScheduleConfig::new(
            0,
            runtime,
            TaskSchedulerLimits::new(16, 16, 4, 64, 1, 16, 4),
        ),
        vec![],
        TaskHostScript::default(),
    )
    .expect("explicit incomplete exploration");
    assert!(!run_limited.is_complete());
    assert_eq!(run_limited.limit(), Some(TaskExplorationLimit::Runs));
}
