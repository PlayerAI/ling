use std::{fmt::Write as _, sync::mpsc};

use ling_ast::lower as lower_ast;
use ling_effects::{CheckedProgram, RunEntry, check, locate_run_entry};
use ling_eval::{
    Console, HostError, HostErrorCategory, LocalTaskControl, LocalTaskSchedulerConfig,
    LocalTaskSchedulerError, LocalTaskTerminal, MemoryConsole, RuntimeFaultKind, TaskRuntimeLimits,
    TaskRuntimeState, TaskValue, run_local_task,
};
use ling_hir::lower as lower_hir;
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;

fn checked(text: &str) -> CheckedProgram {
    checked_at("task-local-scheduler.ling", text.as_bytes().to_vec())
}

fn checked_at(source_name: &str, bytes: Vec<u8>) -> CheckedProgram {
    let source =
        SourceFile::from_bytes(SourceId::new(0), source_name, bytes).expect("valid source");
    let parsed = parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = lower_ast(&source, &parsed).expect("valid AST");
    let hir = lower_hir(source.name(), &ast).expect("valid HIR");
    let resolved = ling_resolve::resolve(vec![hir], "Main").expect("resolved program");
    let typed = check_types(resolved).expect("typed program");
    check(typed).expect("checked program")
}

fn task_main(checked: &CheckedProgram) -> ling_resolve::DefinitionId {
    match locate_run_entry(checked).expect("valid run entry") {
        RunEntry::Task(definition) => definition,
        RunEntry::Value(_) => panic!("expected Task main"),
    }
}

fn config(worker_count: usize) -> LocalTaskSchedulerConfig {
    LocalTaskSchedulerConfig::new(
        worker_count,
        16,
        8,
        128,
        128,
        64,
        TaskRuntimeLimits::new(16, 16, 128, 16),
    )
}

const NESTED_MAIN: &str = concat!(
    "module Main\n\n",
    "task child () =\n",
    "    scope\n",
    "        return ()\n\n",
    "task main () =\n",
    "    scope\n",
    "        let handle = spawn child ()\n",
    "        let value = await handle\n",
    "        return value\n",
);

const STRESS_SHORT_TASKS: usize = 64;
const STRESS_REPETITIONS: usize = 8;

fn generated_short_task_program(task_count: usize) -> String {
    let mut source = String::from(concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value\n\n",
        "task main () =\n",
        "    scope\n",
    ));
    for index in 0..task_count {
        writeln!(source, "        let handle{index} = spawn child {index}")
            .expect("write to String");
    }
    for index in 0..task_count {
        writeln!(source, "        let value{index} = await handle{index}")
            .expect("write to String");
    }
    source.push_str("        return ()\n");
    source
}

fn assert_terminal_cleanup(run: &ling_eval::LocalTaskRun, expected_tasks: usize) {
    assert_eq!(run.snapshot().records().len(), expected_tasks);
    assert!(run.snapshot().records().iter().all(|record| {
        record.cleanup_count() == 1
            && matches!(
                record.state(),
                TaskRuntimeState::Completed(_)
                    | TaskRuntimeState::Cancelled
                    | TaskRuntimeState::Faulted { .. }
            )
    }));
}

#[test]
fn fixed_worker_counts_preserve_terminal_tree_and_cleanup() {
    let checked = checked(NESTED_MAIN);
    let main = task_main(&checked);
    let mut outcomes = Vec::new();
    for workers in [1, 1, 4, 4] {
        let mut console = MemoryConsole::default();
        let run = run_local_task(
            &checked,
            &main,
            vec![TaskValue::Unit],
            &mut console,
            config(workers),
            &LocalTaskControl::new(),
        )
        .expect("local Task run");
        assert_eq!(
            run.terminal(),
            &LocalTaskTerminal::Completed(TaskValue::Unit)
        );
        assert_eq!(
            run.snapshot().root(),
            &TaskRuntimeState::Completed(TaskValue::Unit)
        );
        assert_eq!(run.snapshot().records().len(), 2);
        assert!(
            run.snapshot()
                .records()
                .iter()
                .all(|record| record.cleanup_count() == 1)
        );
        assert!(run.metrics().completed_steps() > 0);
        assert_eq!(run.metrics().worker_exits(), workers as u64);
        if workers > 1 {
            assert!(run.metrics().parks() > 0);
            assert!(run.metrics().wakes() > 0);
        }
        assert!(console.output().is_empty());
        outcomes.push(
            run.snapshot()
                .records()
                .iter()
                .map(|record| {
                    (
                        record.path().clone(),
                        record.state().clone(),
                        record.cleanup_count(),
                    )
                })
                .collect::<Vec<_>>(),
        );
    }
    assert!(outcomes.windows(2).all(|pair| pair[0] == pair[1]));
}

#[test]
fn early_parent_fault_drains_every_registered_child_before_shutdown() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task child () =\n",
        "    scope\n",
        "        return ()\n\n",
        "task main () =\n",
        "    scope\n",
        "        let handle = spawn child ()\n",
        "        let failed = 1 / 0\n",
        "        let value = await handle\n",
        "        return value\n",
    ));
    let main = task_main(&checked);
    for workers in [1, 4] {
        let mut console = MemoryConsole::default();
        let run = run_local_task(
            &checked,
            &main,
            vec![TaskValue::Unit],
            &mut console,
            config(workers),
            &LocalTaskControl::new(),
        )
        .expect("parent Fault remains a structured terminal outcome");
        assert!(matches!(
            run.terminal(),
            LocalTaskTerminal::Faulted(ling_eval::RuntimeFault {
                kind: RuntimeFaultKind::TaskFaultAggregate { fault_count: 1, .. },
                ..
            })
        ));
        assert_terminal_cleanup(&run, 2);
        assert_eq!(run.metrics().worker_exits(), workers as u64);
        assert!(console.output().is_empty());
    }
}

#[test]
fn nested_scopes_and_descendants_close_before_root_publication() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task child () =\n",
        "    scope\n",
        "        scope\n",
        "            return ()\n",
        "        return ()\n\n",
        "task main () =\n",
        "    scope\n",
        "        scope\n",
        "            let handle = spawn child ()\n",
        "            let value = await handle\n",
        "            return value\n",
        "        return ()\n",
    ));
    let main = task_main(&checked);
    for workers in [1, 4] {
        let mut console = MemoryConsole::default();
        let run = run_local_task(
            &checked,
            &main,
            vec![TaskValue::Unit],
            &mut console,
            config(workers),
            &LocalTaskControl::new(),
        )
        .expect("nested scopes complete");
        assert_eq!(
            run.terminal(),
            &LocalTaskTerminal::Completed(TaskValue::Unit)
        );
        assert_terminal_cleanup(&run, 2);
        assert_eq!(run.metrics().worker_exits(), workers as u64);
    }
}

#[test]
fn independently_faulting_siblings_publish_one_canonical_aggregate() {
    let checked = checked(concat!(
        "module Main\n    requires Console.Write\n\n",
        "task bad value =\n",
        "    scope\n",
        "        let ignored = Console.write \"boundary\"\n",
        "        return value / 0\n\n",
        "task main () =\n",
        "    scope\n",
        "        let first = spawn bad 1\n",
        "        let second = spawn bad 2\n",
        "        let left = await first\n",
        "        let right = await second\n",
        "        return ()\n",
    ));
    let main = task_main(&checked);
    let mut projections = Vec::new();
    for workers in [1, 4] {
        let mut console = MemoryConsole::default();
        let run = run_local_task(
            &checked,
            &main,
            vec![TaskValue::Unit],
            &mut console,
            config(workers),
            &LocalTaskControl::new(),
        )
        .expect("sibling Faults remain a structured terminal outcome");
        let LocalTaskTerminal::Faulted(fault) = run.terminal() else {
            panic!("expected sibling Fault aggregate")
        };
        assert!(matches!(
            fault.kind,
            RuntimeFaultKind::TaskFaultAggregate { fault_count: 2, .. }
        ));
        assert_terminal_cleanup(&run, 3);
        assert_eq!(console.output(), "boundary\nboundary\n");
        projections.push((fault.clone(), run.snapshot().records().to_vec()));
    }
    assert_eq!(projections[0], projections[1]);
}

#[test]
fn bounded_generated_short_task_stress_is_worker_count_independent() {
    let source = generated_short_task_program(STRESS_SHORT_TASKS);
    let checked = checked_at("generated-task-stress.ling", source.into_bytes());
    let main = task_main(&checked);
    let mut baseline = None;
    for workers in [1, 4] {
        for _ in 0..STRESS_REPETITIONS {
            let mut console = MemoryConsole::default();
            let run = run_local_task(
                &checked,
                &main,
                vec![TaskValue::Unit],
                &mut console,
                LocalTaskSchedulerConfig::new(
                    workers,
                    128,
                    STRESS_SHORT_TASKS,
                    4_096,
                    4_096,
                    128,
                    TaskRuntimeLimits::new(128, 128, 4_096, 128),
                ),
                &LocalTaskControl::new(),
            )
            .expect("bounded generated stress run");
            assert_eq!(
                run.terminal(),
                &LocalTaskTerminal::Completed(TaskValue::Unit)
            );
            assert_terminal_cleanup(&run, STRESS_SHORT_TASKS + 1);
            assert_eq!(run.metrics().worker_exits(), workers as u64);
            assert!(console.output().is_empty());
            let projection = run.snapshot().records().to_vec();
            if let Some(expected) = &baseline {
                assert_eq!(&projection, expected);
            } else {
                baseline = Some(projection);
            }
        }
    }
}

#[test]
fn invalid_configuration_fails_before_host_effects_or_workers() {
    let checked = checked(NESTED_MAIN);
    let main = task_main(&checked);
    let mut console = MemoryConsole::default();
    let error = run_local_task(
        &checked,
        &main,
        vec![TaskValue::Unit],
        &mut console,
        LocalTaskSchedulerConfig::new(0, 1, 1, 1, 1, 1, TaskRuntimeLimits::new(1, 1, 1, 1)),
        &LocalTaskControl::new(),
    )
    .expect_err("zero workers are invalid");
    assert_eq!(
        error,
        LocalTaskSchedulerError::InvalidConfiguration {
            reason: "worker_count_zero"
        }
    );
    assert!(console.output().is_empty());
}

#[test]
fn exact_million_task_ceiling_is_accepted_and_larger_limit_is_rejected_atomically() {
    let accepted = checked(NESTED_MAIN);
    let accepted_main = task_main(&accepted);
    let mut accepted_console = MemoryConsole::default();
    let run = run_local_task(
        &accepted,
        &accepted_main,
        vec![TaskValue::Unit],
        &mut accepted_console,
        LocalTaskSchedulerConfig::new(
            2,
            16,
            8,
            128,
            128,
            64,
            TaskRuntimeLimits::new(1_000_000, 16, 128, 16),
        ),
        &LocalTaskControl::new(),
    )
    .expect("the exact one-million Task ceiling is a valid capacity bound");
    assert_eq!(
        run.terminal(),
        &LocalTaskTerminal::Completed(TaskValue::Unit)
    );
    assert!(
        run.snapshot()
            .records()
            .iter()
            .all(|record| record.cleanup_count() == 1)
    );

    let rejected = checked(concat!(
        "module Main\n    requires Console.Write\n\n",
        "task main () =\n",
        "    scope\n",
        "        let ignored = Console.write \"must not run\"\n",
        "        return ()\n",
    ));
    let rejected_main = task_main(&rejected);
    for task_limit in [1_000_001, usize::MAX] {
        let mut rejected_console = MemoryConsole::default();
        let error = run_local_task(
            &rejected,
            &rejected_main,
            vec![TaskValue::Unit],
            &mut rejected_console,
            LocalTaskSchedulerConfig::new(
                2,
                16,
                8,
                128,
                128,
                64,
                TaskRuntimeLimits::new(task_limit, 16, 128, 16),
            ),
            &LocalTaskControl::new(),
        )
        .expect_err("a capacity above one million must fail before execution");
        assert_eq!(
            error,
            LocalTaskSchedulerError::InvalidConfiguration {
                reason: "runtime_task_limit_exceeds_1000000"
            }
        );
        assert!(rejected_console.output().is_empty());
    }
}

#[test]
fn lexical_direct_child_limit_is_preflighted() {
    let checked = checked(concat!(
        "module Main\n\n",
        "task child () =\n",
        "    scope\n",
        "        return ()\n\n",
        "task main () =\n",
        "    scope\n",
        "        let first = spawn child ()\n",
        "        let second = spawn child ()\n",
        "        let left = await first\n",
        "        let right = await second\n",
        "        return right\n",
    ));
    let main = task_main(&checked);
    let mut console = MemoryConsole::default();
    let error = run_local_task(
        &checked,
        &main,
        vec![TaskValue::Unit],
        &mut console,
        LocalTaskSchedulerConfig::new(
            2,
            16,
            1,
            128,
            128,
            64,
            TaskRuntimeLimits::new(16, 16, 128, 16),
        ),
        &LocalTaskControl::new(),
    )
    .expect_err("two lexical children exceed the configured limit");
    assert!(matches!(
        error,
        LocalTaskSchedulerError::Runtime {
            fault: ling_eval::RuntimeFault {
                kind: RuntimeFaultKind::TaskResourceLimit {
                    resource: "scope_direct_children",
                    limit: 1,
                },
                ..
            }
        }
    ));
    assert!(console.output().is_empty());
}

#[test]
fn queue_and_transition_limits_fail_at_scheduling_boundaries() {
    let checked = checked(NESTED_MAIN);
    let main = task_main(&checked);
    let runtime_limits = TaskRuntimeLimits::new(16, 16, 128, 16);

    let mut queue_console = MemoryConsole::default();
    let queue_error = run_local_task(
        &checked,
        &main,
        vec![TaskValue::Unit],
        &mut queue_console,
        LocalTaskSchedulerConfig::new(2, 1, 8, 128, 128, 64, runtime_limits),
        &LocalTaskControl::new(),
    )
    .expect_err("root plus child exceeds a queue of one");
    assert!(matches!(
        queue_error,
        LocalTaskSchedulerError::Runtime {
            fault: ling_eval::RuntimeFault {
                kind: RuntimeFaultKind::TaskResourceLimit {
                    resource: "local_queue",
                    limit: 1,
                },
                ..
            }
        }
    ));
    assert!(queue_console.output().is_empty());

    let mut transition_console = MemoryConsole::default();
    let transition_error = run_local_task(
        &checked,
        &main,
        vec![TaskValue::Unit],
        &mut transition_console,
        LocalTaskSchedulerConfig::new(2, 16, 8, 1, 128, 64, runtime_limits),
        &LocalTaskControl::new(),
    )
    .expect_err("one scheduler transition cannot finish the root");
    assert!(matches!(
        transition_error,
        LocalTaskSchedulerError::Runtime {
            fault: ling_eval::RuntimeFault {
                kind: RuntimeFaultKind::TaskResourceLimit {
                    resource: "scheduler_transitions",
                    limit: 1,
                },
                ..
            }
        }
    ));
    assert!(transition_console.output().is_empty());
}

struct BlockingConsole {
    entered: mpsc::Sender<()>,
    release: mpsc::Receiver<()>,
}

impl Console for BlockingConsole {
    fn write(&mut self, _text: &str) -> Result<(), HostError> {
        self.entered.send(()).expect("test observer remains live");
        self.release.recv().expect("test releases host boundary");
        Ok(())
    }
}

#[test]
fn host_cancellation_wakes_scheduler_and_drains_cleanup() {
    let checked = checked(concat!(
        "module Main\n    requires Console.Write\n\n",
        "task main () =\n",
        "    scope\n",
        "        let ignored = Console.write \"entered\"\n",
        "        return ()\n",
    ));
    let main = task_main(&checked);
    let control = LocalTaskControl::new();
    let cancel = control.clone();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        let mut console = BlockingConsole {
            entered: entered_tx,
            release: release_rx,
        };
        run_local_task(
            &checked,
            &main,
            vec![TaskValue::Unit],
            &mut console,
            config(2),
            &control,
        )
    });
    entered_rx.recv().expect("host boundary reached");
    cancel.cancel();
    release_tx.send(()).expect("release host boundary");
    let run = worker.join().expect("scheduler thread joins").expect("run");
    assert_eq!(run.terminal(), &LocalTaskTerminal::Cancelled);
    assert_eq!(run.metrics().cancellation_observations(), 1);
    assert!(
        run.snapshot()
            .records()
            .iter()
            .all(|record| record.cleanup_count() == 1)
    );
}

struct PanickingConsole;

impl Console for PanickingConsole {
    fn write(&mut self, _text: &str) -> Result<(), HostError> {
        panic!("private test panic payload")
    }
}

#[test]
fn host_panic_is_contained_as_typed_scheduler_failure() {
    let checked = checked(concat!(
        "module Main\n    requires Console.Write\n\n",
        "task main () =\n",
        "    scope\n",
        "        let ignored = Console.write \"panic\"\n",
        "        return ()\n",
    ));
    let main = task_main(&checked);
    let error = run_local_task(
        &checked,
        &main,
        vec![TaskValue::Unit],
        &mut PanickingConsole,
        config(2),
        &LocalTaskControl::new(),
    )
    .expect_err("host panic cannot escape the scheduler");
    assert_eq!(
        error,
        LocalTaskSchedulerError::Internal {
            reason: "worker_or_host_panic"
        }
    );
    assert!(!error.to_string().contains("private test panic payload"));
}

struct FailingConsole;

impl Console for FailingConsole {
    fn write(&mut self, _text: &str) -> Result<(), HostError> {
        Err(HostError::new(HostErrorCategory::PermissionDenied))
    }
}

#[test]
fn host_failure_retains_original_runtime_fault_and_span() {
    let checked = checked(concat!(
        "module Main\n    requires Console.Write\n\n",
        "task main () =\n",
        "    scope\n",
        "        let ignored = Console.write \"fail\"\n",
        "        return ()\n",
    ));
    let main = task_main(&checked);
    let run = run_local_task(
        &checked,
        &main,
        vec![TaskValue::Unit],
        &mut FailingConsole,
        config(2),
        &LocalTaskControl::new(),
    )
    .expect("host failure is retained by the structured Task fault set");
    let LocalTaskTerminal::Faulted(fault) = run.terminal() else {
        panic!("expected faulted terminal state")
    };
    assert_eq!(fault.source_name, "task-local-scheduler.ling");
    assert!(matches!(
        fault.kind,
        RuntimeFaultKind::TaskFaultAggregate { fault_count: 1, .. }
    ));
    assert_eq!(run.snapshot().records()[0].cleanup_count(), 1);
}

#[test]
fn unicode_bom_and_crlf_source_evidence_survives_worker_execution() {
    let source = concat!(
        "module Main\r\n    requires Console.Write\r\n\r\n",
        "task 子任务 () =\r\n",
        "    scope\r\n",
        "        return ()\r\n\r\n",
        "task main () =\r\n",
        "    scope\r\n",
        "        let child = spawn 子任务 ()\r\n",
        "        let value = await child\r\n",
        "        let ignored = Console.write \"失败\"\r\n",
        "        return value\r\n",
    );
    let mut bytes = vec![0xef, 0xbb, 0xbf];
    bytes.extend_from_slice(source.as_bytes());
    let checked = checked_at("目录/任务.ling", bytes);
    let main = task_main(&checked);
    let run = run_local_task(
        &checked,
        &main,
        vec![TaskValue::Unit],
        &mut FailingConsole,
        config(4),
        &LocalTaskControl::new(),
    )
    .expect("checked host failure becomes a structured Fault");
    let LocalTaskTerminal::Faulted(fault) = run.terminal() else {
        panic!("expected faulted terminal state")
    };
    assert_eq!(fault.source_name, "目录/任务.ling");
    assert!(fault.span.start().get() > 3);
    assert!(
        run.snapshot()
            .records()
            .iter()
            .all(|record| record.cleanup_count() == 1)
    );
}
