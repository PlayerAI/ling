//! ACT-2306 property and bounded-stress evidence for the local Actor runtime.
//!
//! This module is compiled only for `ling-eval` unit tests.  Its bounded
//! parallel-turn driver is deliberately not a Ling API, scheduler, trace, or
//! public protocol.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;

use ling_ast::lower as lower_ast;
use ling_concurrency::ActorId;
use ling_effects::{CheckedActorCore, CheckedProgram, check};
use ling_hir as hir;
use ling_resolve::{DefinitionId, ModuleId, resolve};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;
use num_bigint::BigInt;

use super::*;
use crate::MemoryConsole;

const MAX_TEST_WORKERS: usize = 4;
const MAX_TEST_ACTORS: usize = 4;
const MAX_TEST_QUEUED_PER_ACTOR: usize = 8;
const MAX_TEST_COMMANDS: usize = 256;
const GENERATED_COMMANDS: usize = 48;
const STRESS_SEEDS: [u64; 4] = [
    0x0d1f_4e5a_7b9c_1137,
    0x6a09_e667_f3bc_c909,
    0xbb67_ae85_84ca_a73b,
    0x3c6e_f372_fe94_f82b,
];

thread_local! {
    static TURN_PANIC_REQUESTED: Cell<bool> = const { Cell::new(false) };
}

pub(super) fn panic_turn_evaluation_if_requested() {
    let requested = TURN_PANIC_REQUESTED.with(|requested| requested.replace(false));
    if requested {
        panic!("ACT-2306 private evaluator panic");
    }
}

fn request_turn_panic() {
    TURN_PANIC_REQUESTED.with(|requested| requested.set(true));
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParallelTurnTestError {
    EmptyBatch,
    InvalidWorkerCount { worker_count: usize },
    BatchExceedsActorBound { actors: usize },
    DuplicateActor { actor: ActorId },
    Runtime(ActorRuntimeError),
    CandidateFault { actor: ActorId },
    CandidateValue { actor: ActorId },
    EffectfulTurn { actor: ActorId },
    WorkerPanicked,
}

impl From<ActorRuntimeError> for ParallelTurnTestError {
    fn from(error: ActorRuntimeError) -> Self {
        Self::Runtime(error)
    }
}

#[derive(Clone)]
struct ParallelTurnProbe {
    barrier: Arc<Barrier>,
    entered: Arc<AtomicUsize>,
    released: Arc<AtomicUsize>,
}

impl ParallelTurnProbe {
    fn new(participants: usize) -> Self {
        Self {
            barrier: Arc::new(Barrier::new(participants)),
            entered: Arc::new(AtomicUsize::new(0)),
            released: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn arrive(&self) {
        self.entered.fetch_add(1, Ordering::SeqCst);
        self.barrier.wait();
        self.released.fetch_add(1, Ordering::SeqCst);
    }

    fn entered(&self) -> usize {
        self.entered.load(Ordering::SeqCst)
    }

    fn released(&self) -> usize {
        self.released.load(Ordering::SeqCst)
    }
}

#[derive(Clone)]
struct PreparedTurn {
    actor: ActorId,
    core: CheckedActorCore,
    module: ModuleId,
    transition: hir::Expression,
    previous_state: ActorValue,
}

#[derive(Clone)]
struct ReservedTurn {
    actor: ActorId,
    core: CheckedActorCore,
    module: ModuleId,
    transition: hir::Expression,
    previous_state: ActorValue,
    envelope: Envelope,
}

impl<'checked> ActorRuntime<'checked> {
    /// Test-only ACT-2306 driver for the pure normal-return Actor profile.
    ///
    /// Reservations and commits stay on this coordinator.  Workers evaluate
    /// only copied inputs for distinct actors, and state commits are canonical
    /// by ActorId after all worker groups have completed.
    fn test_step_parallel(
        &mut self,
        actors: &[ActorId],
        worker_count: usize,
        probe: Option<&ParallelTurnProbe>,
    ) -> Result<Vec<ActorTurnResult>, ParallelTurnTestError> {
        if actors.is_empty() {
            return Err(ParallelTurnTestError::EmptyBatch);
        }
        if !(1..=MAX_TEST_WORKERS).contains(&worker_count) {
            return Err(ParallelTurnTestError::InvalidWorkerCount { worker_count });
        }
        if actors.len() > MAX_TEST_ACTORS {
            return Err(ParallelTurnTestError::BatchExceedsActorBound {
                actors: actors.len(),
            });
        }

        let mut canonical = actors.to_vec();
        canonical.sort_unstable();
        if let Some(duplicate) = canonical
            .windows(2)
            .find_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
        {
            return Err(ParallelTurnTestError::DuplicateActor { actor: duplicate });
        }

        self.observe_owner_cancellation()?;
        self.ensure_running()?;
        self.require_parallel_capacity(canonical.len())?;

        let prepared = canonical
            .iter()
            .map(|actor| self.prepare_parallel_turn(*actor))
            .collect::<Result<Vec<_>, _>>()?;
        let reservations = self.reserve_parallel_turns(prepared);

        let candidates = match evaluate_reserved_turns(
            self.checked,
            &reservations,
            worker_count,
            probe.cloned(),
        ) {
            Ok(candidates) => candidates,
            Err(error) => {
                self.restore_parallel_reservations(reservations);
                return Err(error);
            }
        };
        if reservations
            .iter()
            .any(|reservation| !candidates.contains_key(&reservation.actor))
        {
            self.restore_parallel_reservations(reservations);
            return Err(ParallelTurnTestError::WorkerPanicked);
        }

        let mut results = Vec::with_capacity(reservations.len());
        for reservation in reservations {
            let next_state = candidates
                .get(&reservation.actor)
                .expect("all reserved Actors have one checked candidate")
                .clone();
            let remaining_messages = {
                let instance = self
                    .actors
                    .get_mut(&reservation.actor)
                    .expect("reserved Actor remains registered until coordinator commit");
                instance.state = Some(next_state.clone());
                instance.mailbox.len()
            };
            self.commands += 1;
            self.turns += 1;
            self.push_event(ActorRuntimeEventKind::TurnStarted {
                actor: reservation.actor,
                sender: reservation.envelope.sender,
                sender_sequence: reservation.envelope.sender_sequence,
                admission_sequence: reservation.envelope.admission_sequence,
            });
            self.push_event(ActorRuntimeEventKind::TurnCompleted {
                actor: reservation.actor,
            });
            results.push(ActorTurnResult::Completed {
                actor: reservation.actor,
                state: next_state,
                remaining_messages,
            });
        }
        Ok(results)
    }

    fn require_parallel_capacity(&self, actors: usize) -> Result<(), ActorRuntimeError> {
        if actors > self.limits.max_turns.saturating_sub(self.turns) {
            return Err(ActorRuntimeError::ResourceExhausted {
                resource: "turns",
                limit: self.limits.max_turns,
            });
        }
        if actors > self.limits.max_commands.saturating_sub(self.commands) {
            return Err(ActorRuntimeError::ResourceExhausted {
                resource: "commands",
                limit: self.limits.max_commands,
            });
        }
        let event_count = actors
            .checked_mul(2)
            .expect("ACT-2306 test Actor batch is bounded by four");
        if event_count > self.limits.max_events.saturating_sub(self.events.len()) {
            return Err(ActorRuntimeError::ResourceExhausted {
                resource: "events",
                limit: self.limits.max_events,
            });
        }
        Ok(())
    }

    fn prepare_parallel_turn(&self, actor: ActorId) -> Result<PreparedTurn, ActorRuntimeError> {
        let instance = self
            .actors
            .get(&actor)
            .ok_or(ActorRuntimeError::UnknownActor { actor })?;
        if instance.lifecycle != ActorInstanceState::Running || instance.mailbox.is_empty() {
            return Err(ActorRuntimeError::ActorNotReady { actor });
        }
        let (module, transition) =
            actor_expression(self.checked, &instance.core, ActorFaultPhase::Turn)
                .map(|(module, expression)| (module, expression.clone()))
                .ok_or(ActorRuntimeError::InvalidCheckedCore {
                    invariant: "Actor transition expression is absent",
                })?;
        let previous_state =
            instance
                .state
                .clone()
                .ok_or(ActorRuntimeError::InvalidCheckedCore {
                    invariant: "running Actor has no committed state",
                })?;
        Ok(PreparedTurn {
            actor,
            core: instance.core.clone(),
            module,
            transition,
            previous_state,
        })
    }

    fn reserve_parallel_turns(&mut self, prepared: Vec<PreparedTurn>) -> Vec<ReservedTurn> {
        let mut reservations = Vec::with_capacity(prepared.len());
        for prepared in prepared {
            let envelope = self
                .actors
                .get_mut(&prepared.actor)
                .expect("preflighted Actor remains registered")
                .mailbox
                .pop_front()
                .expect("preflighted ready Actor retains its envelope");
            reservations.push(ReservedTurn {
                actor: prepared.actor,
                core: prepared.core,
                module: prepared.module,
                transition: prepared.transition,
                previous_state: prepared.previous_state,
                envelope,
            });
        }
        debug_assert!(self.queued_messages >= reservations.len());
        self.queued_messages -= reservations.len();
        reservations
    }

    fn restore_parallel_reservations(&mut self, reservations: Vec<ReservedTurn>) {
        let count = reservations.len();
        for reservation in reservations {
            self.actors
                .get_mut(&reservation.actor)
                .expect("reserved Actor remains registered after worker failure")
                .mailbox
                .push_front(reservation.envelope);
        }
        self.queued_messages += count;
    }
}

fn evaluate_reserved_turns(
    checked: &CheckedProgram,
    reservations: &[ReservedTurn],
    worker_count: usize,
    probe: Option<ParallelTurnProbe>,
) -> Result<BTreeMap<ActorId, ActorValue>, ParallelTurnTestError> {
    let mut candidates = BTreeMap::new();
    for group in reservations.chunks(worker_count) {
        let outcomes = thread::scope(|scope| {
            let mut workers = Vec::with_capacity(group.len());
            for reservation in group.iter().cloned() {
                let probe = probe.clone();
                workers
                    .push(scope.spawn(move || evaluate_reserved_turn(checked, reservation, probe)));
            }
            let mut outcomes = Vec::with_capacity(workers.len());
            for worker in workers {
                outcomes.push(
                    worker
                        .join()
                        .map_err(|_| ParallelTurnTestError::WorkerPanicked)?,
                );
            }
            Ok::<_, ParallelTurnTestError>(outcomes)
        })?;
        for outcome in outcomes {
            let (actor, candidate) = outcome?;
            if candidates.insert(actor, candidate).is_some() {
                return Err(ParallelTurnTestError::DuplicateActor { actor });
            }
        }
    }
    Ok(candidates)
}

fn evaluate_reserved_turn(
    checked: &CheckedProgram,
    reservation: ReservedTurn,
    probe: Option<ParallelTurnProbe>,
) -> Result<(ActorId, ActorValue), ParallelTurnTestError> {
    let actor = reservation.actor;
    let mut console = MemoryConsole::default();
    let value = catch_unwind(AssertUnwindSafe(|| {
        if let Some(probe) = probe {
            probe.arrive();
        }
        let mut environment = Environment::new();
        environment.insert(
            reservation.core.state_binding(),
            Arc::new(Mutex::new(Value::from(reservation.previous_state.clone()))),
        );
        environment.insert(
            reservation.core.message_binding(),
            Arc::new(Mutex::new(Value::from(
                reservation.envelope.payload.clone(),
            ))),
        );
        let mut interpreter = Interpreter::new(checked, &mut console);
        interpreter.eval_expression(
            reservation.module,
            &reservation.transition,
            &mut environment,
        )
    }))
    .map_err(|_| ParallelTurnTestError::WorkerPanicked)?
    .map_err(|_| ParallelTurnTestError::CandidateFault { actor })?;
    if !console.output().is_empty() {
        return Err(ParallelTurnTestError::EffectfulTurn { actor });
    }
    let value =
        ActorValue::try_from(value).map_err(|_| ParallelTurnTestError::CandidateValue { actor })?;
    if !closed_value_matches_type(&value, reservation.core.state_type(), checked) {
        return Err(ParallelTurnTestError::CandidateValue { actor });
    }
    Ok((actor, value))
}

#[derive(Clone, Debug, PartialEq)]
struct ActorProjection {
    actor: ActorId,
    lifecycle: ActorInstanceState,
    state: Option<ActorValue>,
    queued_messages: usize,
    cleanup_count: usize,
    terminal_reason: Option<ActorShutdownReason>,
    fault: Option<(ActorFaultPhase, RuntimeFaultKind, u32, u32)>,
}

#[derive(Clone, Debug, PartialEq)]
struct OutcomeProjection {
    state: ActorRuntimeState,
    metrics: ActorRuntimeMetrics,
    actors: Vec<ActorProjection>,
    accepted: Vec<(ActorId, ActorSenderId, u64, u64)>,
}

fn projection(runtime: &ActorRuntime<'_>) -> OutcomeProjection {
    let metrics = runtime.metrics();
    let actors = (1..=metrics.created_actors())
        .filter_map(|raw| {
            let actor = ActorId::new(
                u32::try_from(raw).expect("created Actor test bound fits its identity"),
            );
            runtime.snapshot(actor).map(|snapshot| ActorProjection {
                actor,
                lifecycle: snapshot.lifecycle(),
                state: snapshot.state().cloned(),
                queued_messages: snapshot.queued_messages(),
                cleanup_count: snapshot.cleanup_count(),
                terminal_reason: snapshot.terminal_reason(),
                fault: snapshot.fault().map(|fault| {
                    (
                        fault.phase(),
                        fault.cause().kind.clone(),
                        fault.cause().span.start().get(),
                        fault.cause().span.end().get(),
                    )
                }),
            })
        })
        .collect();
    let accepted = runtime
        .events()
        .into_iter()
        .filter_map(|event| match event.kind() {
            ActorRuntimeEventKind::MessageAccepted {
                actor,
                sender,
                sender_sequence,
                admission_sequence,
            } => Some((*actor, *sender, *sender_sequence, *admission_sequence)),
            _ => None,
        })
        .collect();
    OutcomeProjection {
        state: runtime.state(),
        metrics,
        actors,
        accepted,
    }
}

fn checked(source_name: &str, text: &str) -> CheckedProgram {
    checked_bytes(source_name, text.as_bytes().to_vec())
}

fn checked_bytes(source_name: &str, bytes: Vec<u8>) -> CheckedProgram {
    let source =
        SourceFile::from_bytes(SourceId::new(0), source_name, bytes).expect("valid test source");
    let parsed = parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = lower_ast(&source, &parsed).expect("valid AST");
    let hir = hir::lower(source.name(), &ast).expect("valid HIR");
    let resolved = resolve(vec![hir], "Main").expect("resolved program");
    let typed = check_types(resolved).expect("typed program");
    check(typed).expect("checked program")
}

fn actor(checked: &CheckedProgram, name: &str) -> DefinitionId {
    checked
        .actor_cores()
        .keys()
        .find(|definition| {
            checked
                .typed()
                .resolved()
                .definition(definition)
                .is_some_and(|info| info.name == name)
        })
        .cloned()
        .expect("Actor definition")
}

fn integer(value: i64) -> ActorValue {
    ActorValue::Int(BigInt::from(value))
}

fn actor_declaration(name: &str, capacity: usize) -> String {
    format!(
        "actor {name} : Int =\n    mailbox capacity {capacity} overflow Reject\n    state Int = 0\n    receive state message =\n        state + message\n"
    )
}

fn counter_source(name: &str, capacity: usize, spare_first: bool) -> String {
    let counter = actor_declaration(name, capacity);
    let spare = actor_declaration("Spare", capacity);
    let declarations = if spare_first {
        format!("{spare}\n{counter}")
    } else {
        counter
    };
    format!("module Main\n\n{declarations}\nlet main () = ()\n")
}

fn limits(max_queued_messages: usize) -> ActorRuntimeLimits {
    ActorRuntimeLimits::new(
        MAX_TEST_ACTORS,
        MAX_TEST_ACTORS,
        max_queued_messages,
        MAX_TEST_COMMANDS,
        MAX_TEST_COMMANDS,
        1_024,
        MAX_TEST_ACTORS,
        MAX_TEST_ACTORS,
    )
}

fn assert_no_same_actor_overlap(runtime: &ActorRuntime<'_>) {
    let mut active = BTreeSet::new();
    for event in runtime.events() {
        match event.kind() {
            ActorRuntimeEventKind::TurnStarted { actor, .. } => {
                assert!(active.insert(*actor), "Actor turn overlap: {}", actor.get());
            }
            ActorRuntimeEventKind::TurnCompleted { actor } => {
                assert!(active.remove(actor), "turn completion without reservation");
            }
            _ => {}
        }
    }
    assert!(active.is_empty(), "all reserved turns reach a boundary");
}

#[test]
fn same_actor_generated_turns_are_serial_and_consume_one_fifo_envelope() {
    let checked = checked("serial.ling", &counter_source("Counter", 8, false));
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime = ActorRuntime::new(&checked, ActorRuntimeId::new(41), limits(32), &control)
        .expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime.spawn(&counter, &mut console).expect("Actor starts");
    let sender = ActorSenderId::new(7);
    let mut generator = SplitMix64::new(STRESS_SEEDS[0]);
    let mut expected = 0_i64;

    for _ in 0..32 {
        let message = i64::try_from(generator.next() % 7 + 1).expect("small test message");
        runtime
            .send(&reference, sender, integer(message))
            .expect("one FIFO envelope is accepted");
        expected += message;
        let ActorTurnResult::Completed {
            state,
            remaining_messages,
            ..
        } = runtime
            .step(reference.actor(), &mut console)
            .expect("serial turn")
        else {
            panic!("pure Counter turn must complete");
        };
        assert_eq!(state, integer(expected));
        assert_eq!(remaining_messages, 0);
    }

    assert_eq!(runtime.metrics().turns(), 32);
    assert_eq!(runtime.metrics().queued_messages(), 0);
    assert_no_same_actor_overlap(&runtime);
    let accepted = projection(&runtime).accepted;
    assert_eq!(accepted.len(), 32);
    assert_eq!(
        accepted
            .iter()
            .map(|(_, _, sender_sequence, _)| *sender_sequence)
            .collect::<Vec<_>>(),
        (1..=32).collect::<Vec<_>>(),
    );
}

fn run_parallel_case(worker_count: usize, probe: Option<&ParallelTurnProbe>) -> OutcomeProjection {
    let checked = checked("parallel.ling", &counter_source("Counter", 8, false));
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime = ActorRuntime::new(&checked, ActorRuntimeId::new(42), limits(32), &control)
        .expect("runtime");
    let mut console = MemoryConsole::default();
    let first = runtime.spawn(&counter, &mut console).expect("first Actor");
    let second = runtime.spawn(&counter, &mut console).expect("second Actor");
    runtime
        .send(&first, ActorSenderId::new(1), integer(3))
        .expect("first message");
    runtime
        .send(&second, ActorSenderId::new(2), integer(5))
        .expect("second message");

    let results = runtime
        .test_step_parallel(&[second.actor(), first.actor()], worker_count, probe)
        .expect("pure independent Actors complete");
    assert_eq!(
        results
            .iter()
            .map(|result| match result {
                ActorTurnResult::Completed { actor, .. }
                | ActorTurnResult::Faulted { actor, .. } => {
                    *actor
                }
            })
            .collect::<Vec<_>>(),
        [first.actor(), second.actor()],
    );
    let committed = runtime
        .events()
        .into_iter()
        .filter_map(|event| match event.kind() {
            ActorRuntimeEventKind::TurnStarted { actor, .. } => Some(*actor),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(committed, [first.actor(), second.actor()]);
    assert_no_same_actor_overlap(&runtime);
    projection(&runtime)
}

#[test]
fn independent_actor_turns_reach_a_barrier_and_commit_in_actor_id_order() {
    let one_worker = run_parallel_case(1, None);
    let probe = ParallelTurnProbe::new(2);
    let two_workers = run_parallel_case(2, Some(&probe));

    assert_eq!(probe.entered(), 2);
    assert_eq!(probe.released(), 2);
    assert_eq!(one_worker, two_workers);
}

#[test]
fn parallel_driver_configuration_errors_are_typed_and_failure_atomic() {
    let checked = checked("parallel-config.ling", &counter_source("Counter", 8, false));
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime = ActorRuntime::new(&checked, ActorRuntimeId::new(43), limits(32), &control)
        .expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime.spawn(&counter, &mut console).expect("Actor starts");
    runtime
        .send(&reference, ActorSenderId::new(1), integer(9))
        .expect("message queued");
    let before = projection(&runtime);

    assert_eq!(
        runtime
            .test_step_parallel(&[reference.actor()], 0, None)
            .expect_err("zero workers are invalid"),
        ParallelTurnTestError::InvalidWorkerCount { worker_count: 0 }
    );
    assert_eq!(projection(&runtime), before);
    assert_eq!(
        runtime
            .test_step_parallel(&[reference.actor(), reference.actor()], 2, None)
            .expect_err("one Actor cannot reserve two concurrent turns"),
        ParallelTurnTestError::DuplicateActor {
            actor: reference.actor()
        }
    );
    assert_eq!(projection(&runtime), before);
    assert_eq!(
        runtime
            .test_step_parallel(&[reference.actor()], MAX_TEST_WORKERS + 1, None)
            .expect_err("worker bound is four"),
        ParallelTurnTestError::InvalidWorkerCount {
            worker_count: MAX_TEST_WORKERS + 1,
        }
    );
    assert_eq!(projection(&runtime), before);
}

#[test]
fn parallel_resource_preflight_rejects_without_dequeue_or_publication() {
    let checked = checked(
        "parallel-resource.ling",
        &counter_source("Counter", 8, false),
    );
    let counter = actor(&checked, "Counter");
    let mut console = MemoryConsole::default();

    let command_control = LocalTaskControl::new();
    let command_limits = ActorRuntimeLimits::new(2, 2, 4, 2, 4, 16, 2, 2);
    let mut command_runtime = ActorRuntime::new(
        &checked,
        ActorRuntimeId::new(431),
        command_limits,
        &command_control,
    )
    .expect("command-bounded runtime");
    let command_reference = command_runtime
        .spawn(&counter, &mut console)
        .expect("Actor starts");
    command_runtime
        .send(&command_reference, ActorSenderId::new(1), integer(1))
        .expect("message fits command limit");
    let command_before = projection(&command_runtime);
    assert_eq!(
        command_runtime
            .test_step_parallel(&[command_reference.actor()], 1, None)
            .expect_err("parallel turn cannot exceed command bound"),
        ParallelTurnTestError::Runtime(ActorRuntimeError::ResourceExhausted {
            resource: "commands",
            limit: 2,
        })
    );
    assert_eq!(projection(&command_runtime), command_before);

    let event_control = LocalTaskControl::new();
    let event_limits = ActorRuntimeLimits::new(2, 2, 4, 16, 16, 3, 2, 2);
    let mut event_runtime = ActorRuntime::new(
        &checked,
        ActorRuntimeId::new(432),
        event_limits,
        &event_control,
    )
    .expect("event-bounded runtime");
    let event_reference = event_runtime
        .spawn(&counter, &mut console)
        .expect("Actor starts");
    event_runtime
        .send(&event_reference, ActorSenderId::new(1), integer(1))
        .expect("message consumes the second event");
    let event_before = projection(&event_runtime);
    assert_eq!(
        event_runtime
            .test_step_parallel(&[event_reference.actor()], 1, None)
            .expect_err("parallel turn must reserve both canonical events"),
        ParallelTurnTestError::Runtime(ActorRuntimeError::ResourceExhausted {
            resource: "events",
            limit: 3,
        })
    );
    assert_eq!(projection(&event_runtime), event_before);
}

#[test]
fn slow_consumer_backpressure_preserves_payload_and_contiguous_sender_order() {
    let checked = checked("backpressure.ling", &counter_source("Counter", 2, false));
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(44), limits(3), &control).expect("runtime");
    let mut console = MemoryConsole::default();
    let first = runtime.spawn(&counter, &mut console).expect("first Actor");
    let second = runtime.spawn(&counter, &mut console).expect("second Actor");
    let first_sender = ActorSenderId::new(1);
    let second_sender = ActorSenderId::new(2);

    runtime
        .send(&first, first_sender, integer(11))
        .expect("first mailbox message");
    runtime
        .send(&first, first_sender, integer(12))
        .expect("second mailbox message");
    let full_mailbox = runtime
        .send(&first, first_sender, integer(13))
        .expect_err("slow consumer does not step");
    assert_eq!(
        full_mailbox.kind(),
        &ActorSendErrorKind::Full {
            resource: "actor_mailbox",
            limit: 2,
        }
    );
    assert_eq!(full_mailbox.into_payload(), integer(13));
    assert_eq!(runtime.metrics().queued_messages(), 2);

    runtime
        .step(first.actor(), &mut console)
        .expect("drain one FIFO envelope");
    runtime
        .send(&first, first_sender, integer(13))
        .expect("retry after drain is accepted");
    runtime
        .send(&second, second_sender, integer(21))
        .expect("fill run-wide queue");
    let full_runtime = runtime
        .send(&second, second_sender, integer(22))
        .expect_err("run-wide queue is full without a step");
    assert_eq!(
        full_runtime.kind(),
        &ActorSendErrorKind::Full {
            resource: "queued_messages",
            limit: 3,
        }
    );
    assert_eq!(full_runtime.into_payload(), integer(22));
    assert_eq!(runtime.metrics().queued_messages(), 3);

    let accepted = projection(&runtime).accepted;
    assert_eq!(
        accepted
            .iter()
            .filter(|(_, sender, _, _)| *sender == first_sender)
            .map(|(_, _, sequence, _)| *sequence)
            .collect::<Vec<_>>(),
        [1, 2, 3],
    );
    assert_eq!(
        accepted
            .iter()
            .filter(|(_, sender, _, _)| *sender == second_sender)
            .map(|(_, _, sequence, _)| *sequence)
            .collect::<Vec<_>>(),
        [1],
    );
}

#[test]
fn test_only_panic_is_contained_as_a_turn_fault_without_payload_leakage() {
    let source = counter_source("计数器", 2, false);
    let checked = checked("故障边界.ling", &source);
    let counter = actor(&checked, "计数器");
    let control = LocalTaskControl::new();
    let mut runtime = ActorRuntime::new(&checked, ActorRuntimeId::new(45), limits(16), &control)
        .expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime.spawn(&counter, &mut console).expect("Actor starts");
    runtime
        .send(&reference, ActorSenderId::new(1), integer(1))
        .expect("message queued");

    request_turn_panic();
    let ActorTurnResult::Faulted {
        previous_state,
        fault,
        discarded_messages,
        ..
    } = runtime
        .step(reference.actor(), &mut console)
        .expect("panic is contained")
    else {
        panic!("test injection must produce a contained Actor Fault");
    };
    assert_eq!(previous_state, integer(0));
    assert_eq!(discarded_messages, 0);
    assert_eq!(fault.phase(), ActorFaultPhase::Turn);
    assert!(matches!(
        fault.cause().kind,
        RuntimeFaultKind::InvalidCheckedCore {
            invariant: "Actor transition evaluation panicked"
        }
    ));
    assert_eq!(fault.cause().source_name, "故障边界.ling");
    assert_eq!(
        fault.cause().span.start().get(),
        u32::try_from(source.find("state + message").expect("transition text")).expect("span")
    );
    assert!(!format!("{fault:?}").contains("ACT-2306 private evaluator panic"));
    assert_eq!(runtime.state(), ActorRuntimeState::Failed);
    assert!(control.is_cancelled());
    let closed = runtime
        .send(&reference, ActorSenderId::new(1), integer(2))
        .expect_err("terminal Actor rejects later messages");
    assert_eq!(closed.kind(), &ActorSendErrorKind::Closed);
    assert_eq!(closed.into_payload(), integer(2));
}

#[test]
fn owner_cancellation_cleans_each_actor_once_in_canonical_actor_order() {
    let checked = checked("cleanup-order.ling", &counter_source("Counter", 8, false));
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime = ActorRuntime::new(&checked, ActorRuntimeId::new(451), limits(32), &control)
        .expect("runtime");
    let mut console = MemoryConsole::default();
    let actors = (0..3)
        .map(|_| runtime.spawn(&counter, &mut console).expect("Actor starts"))
        .collect::<Vec<_>>();
    for (index, reference) in actors.iter().enumerate() {
        runtime
            .send(
                reference,
                ActorSenderId::new(1),
                integer(i64::try_from(index + 1).expect("small payload")),
            )
            .expect("queued message");
    }

    control.cancel();
    assert!(
        runtime
            .observe_owner_cancellation()
            .expect("root cancellation drains the runtime")
    );
    assert_eq!(runtime.state(), ActorRuntimeState::Stopped);
    assert_eq!(runtime.metrics().queued_messages(), 0);
    assert_eq!(runtime.metrics().cleanups(), 3);
    assert_eq!(runtime.metrics().discarded_messages(), 3);
    assert_eq!(
        runtime
            .events()
            .into_iter()
            .filter_map(|event| match event.kind() {
                ActorRuntimeEventKind::ActorStopped { actor, .. } => Some(*actor),
                _ => None,
            })
            .collect::<Vec<_>>(),
        actors.iter().map(LocalActorRef::actor).collect::<Vec<_>>(),
    );
    for reference in actors {
        let snapshot = runtime
            .snapshot(reference.actor())
            .expect("terminal record");
        assert_eq!(snapshot.lifecycle(), ActorInstanceState::Stopped);
        assert_eq!(snapshot.cleanup_count(), 1);
        assert_eq!(snapshot.queued_messages(), 0);
        let closed = runtime
            .send(&reference, ActorSenderId::new(1), integer(9))
            .expect_err("terminal Actor rejects later messages");
        assert_eq!(closed.kind(), &ActorSendErrorKind::Closed);
        assert_eq!(closed.into_payload(), integer(9));
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum GeneratedCommand {
    Spawn,
    Send {
        selector: usize,
        sender: u32,
        value: i64,
    },
    Step {
        selector: usize,
    },
    Parallel,
    Stop {
        selector: usize,
    },
    OwnerCancel,
    Shutdown,
}

#[derive(Clone, Copy)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }
}

fn generated_schedule(seed: u64) -> Vec<GeneratedCommand> {
    let mut generator = SplitMix64::new(seed);
    let mut commands = vec![GeneratedCommand::Spawn, GeneratedCommand::Spawn];
    for _ in 0..GENERATED_COMMANDS - 3 {
        let command = match generator.next() % 6 {
            0 | 1 => GeneratedCommand::Send {
                selector: (generator.next() as usize) % MAX_TEST_ACTORS,
                sender: u32::try_from(generator.next() % 3 + 1).expect("small sender identity"),
                value: i64::try_from(generator.next() % 7 + 1).expect("small message value"),
            },
            2 => GeneratedCommand::Step {
                selector: (generator.next() as usize) % MAX_TEST_ACTORS,
            },
            3 => GeneratedCommand::Parallel,
            4 => GeneratedCommand::Stop {
                selector: (generator.next() as usize) % MAX_TEST_ACTORS,
            },
            _ => GeneratedCommand::Spawn,
        };
        commands.push(command);
    }
    commands.push(if seed & 1 == 0 {
        GeneratedCommand::Shutdown
    } else {
        GeneratedCommand::OwnerCancel
    });
    assert_eq!(commands.len(), GENERATED_COMMANDS);
    commands
}

fn running_references(
    runtime: &ActorRuntime<'_>,
    references: &[LocalActorRef],
) -> Vec<LocalActorRef> {
    references
        .iter()
        .filter(|reference| {
            runtime
                .snapshot(reference.actor())
                .is_some_and(|snapshot| snapshot.lifecycle() == ActorInstanceState::Running)
        })
        .cloned()
        .collect()
}

fn assert_generated_bounds(runtime: &ActorRuntime<'_>) {
    let metrics = runtime.metrics();
    assert!(metrics.created_actors() <= MAX_TEST_ACTORS);
    assert!(metrics.live_actors() <= MAX_TEST_ACTORS);
    assert!(metrics.queued_messages() <= MAX_TEST_ACTORS * MAX_TEST_QUEUED_PER_ACTOR);
    assert!(metrics.commands() <= MAX_TEST_COMMANDS);
    assert!(metrics.turns() <= MAX_TEST_COMMANDS);
    for raw in 1..=metrics.created_actors() {
        let actor =
            ActorId::new(u32::try_from(raw).expect("created Actor test bound fits its identity"));
        let snapshot = runtime
            .snapshot(actor)
            .expect("created Actor keeps a terminal record");
        assert!(snapshot.queued_messages() <= MAX_TEST_QUEUED_PER_ACTOR);
        assert!(snapshot.cleanup_count() <= 1);
    }
}

fn run_generated(
    source_name: &str,
    bytes: Vec<u8>,
    actor_name: &str,
    seed: u64,
    worker_count: usize,
) -> OutcomeProjection {
    let checked = checked_bytes(source_name, bytes);
    let counter = actor(&checked, actor_name);
    let control = LocalTaskControl::new();
    let mut runtime = ActorRuntime::new(&checked, ActorRuntimeId::new(46), limits(32), &control)
        .expect("runtime");
    let mut console = MemoryConsole::default();
    let mut references = Vec::new();
    let schedule = generated_schedule(seed);

    for command in &schedule {
        if runtime.state() != ActorRuntimeState::Running {
            break;
        }
        match command {
            GeneratedCommand::Spawn => {
                if references.len() < MAX_TEST_ACTORS {
                    references.push(
                        runtime
                            .spawn(&counter, &mut console)
                            .expect("bounded generated Actor spawn"),
                    );
                }
            }
            GeneratedCommand::Send {
                selector,
                sender,
                value,
            } => {
                let running = running_references(&runtime, &references);
                if let Some(reference) = running.get(selector % running.len().max(1)) {
                    let payload = integer(*value);
                    if let Err(rejected) =
                        runtime.send(reference, ActorSenderId::new(*sender), payload.clone())
                    {
                        assert!(matches!(rejected.kind(), ActorSendErrorKind::Full { .. }));
                        assert_eq!(rejected.into_payload(), payload);
                    }
                }
            }
            GeneratedCommand::Step { selector } => {
                let ready = runtime.ready();
                if let Some(actor) = ready.get(selector % ready.len().max(1)) {
                    runtime
                        .step(*actor, &mut console)
                        .expect("generated selected pure turn");
                }
            }
            GeneratedCommand::Parallel => {
                let ready = runtime.ready();
                if ready.len() >= 2 {
                    let batch = ready.into_iter().take(MAX_TEST_ACTORS).collect::<Vec<_>>();
                    runtime
                        .test_step_parallel(&batch, worker_count, None)
                        .expect("generated pure parallel turn batch");
                } else if let Some(actor) = ready.first() {
                    runtime
                        .step(*actor, &mut console)
                        .expect("generated single ready turn");
                }
            }
            GeneratedCommand::Stop { selector } => {
                let running = running_references(&runtime, &references);
                if let Some(reference) = running.get(selector % running.len().max(1)) {
                    runtime.stop(reference).expect("generated explicit stop");
                }
            }
            GeneratedCommand::OwnerCancel => {
                control.cancel();
                assert!(
                    runtime
                        .observe_owner_cancellation()
                        .expect("generated owner cancellation")
                );
            }
            GeneratedCommand::Shutdown => {
                runtime
                    .shutdown(ActorShutdownReason::Explicit)
                    .expect("generated explicit shutdown");
            }
        }
        assert_generated_bounds(&runtime);
    }
    if runtime.state() == ActorRuntimeState::Running {
        runtime
            .shutdown(ActorShutdownReason::Explicit)
            .expect("bounded run terminal shutdown");
    }
    assert!(
        runtime.metrics().commands() <= MAX_TEST_COMMANDS,
        "seed {seed:#x} exceeded the ACT-2306 command bound: {:?}",
        runtime.metrics(),
    );
    assert!(
        runtime.events().iter().all(|event| event.sequence() > 0),
        "event identities remain run-relative and nonzero"
    );
    assert_no_same_actor_overlap(&runtime);
    projection(&runtime)
}

#[test]
fn bounded_splitmix_stress_has_the_same_projection_with_one_or_two_workers() {
    let source = counter_source("Counter", MAX_TEST_QUEUED_PER_ACTOR, false);
    for seed in STRESS_SEEDS {
        let schedule = generated_schedule(seed);
        let one_worker = run_generated(
            "stress-one.ling",
            source.as_bytes().to_vec(),
            "Counter",
            seed,
            1,
        );
        let two_workers = run_generated(
            "stress-two.ling",
            source.as_bytes().to_vec(),
            "Counter",
            seed,
            2,
        );
        assert_eq!(
            one_worker, two_workers,
            "seed {seed:#x}; full command sequence: {schedule:?}"
        );
    }
}

#[test]
fn reconstructed_unicode_bom_crlf_source_and_actor_insertion_order_keep_outcomes() {
    let baseline_source = counter_source("Counter", MAX_TEST_QUEUED_PER_ACTOR, false);
    let baseline = run_generated(
        "reconstructed-base.ling",
        baseline_source.as_bytes().to_vec(),
        "Counter",
        STRESS_SEEDS[1],
        2,
    );
    let ordered = counter_source("Counter", MAX_TEST_QUEUED_PER_ACTOR, true);
    let unicode = counter_source("计数器", MAX_TEST_QUEUED_PER_ACTOR, true);
    let variants = [
        (
            "reconstructed-order.ling",
            ordered.as_bytes().to_vec(),
            "Counter",
        ),
        (
            "重建-crlf.ling",
            unicode.replace('\n', "\r\n").into_bytes(),
            "计数器",
        ),
        (
            "重建-bom.ling",
            format!("\u{feff}{unicode}").into_bytes(),
            "计数器",
        ),
    ];
    for (source_name, bytes, actor_name) in variants {
        let outcome = run_generated(source_name, bytes, actor_name, STRESS_SEEDS[1], 2);
        assert_eq!(baseline, outcome, "{source_name}");
    }
}

#[test]
fn zero_one_and_maximum_valid_runtime_limits_remain_explicit() {
    let checked = checked("limits.ling", &counter_source("Counter", 1, false));
    let control = LocalTaskControl::new();
    let invalid = match ActorRuntime::new(
        &checked,
        ActorRuntimeId::new(47),
        ActorRuntimeLimits::new(0, 1, 1, 1, 1, 1, 1, 1),
        &control,
    ) {
        Ok(_) => panic!("zero created-Actor bound must be invalid"),
        Err(error) => error,
    };
    assert_eq!(
        invalid,
        ActorRuntimeError::InvalidLimit {
            resource: "created_actors"
        }
    );
    ActorRuntime::new(
        &checked,
        ActorRuntimeId::new(48),
        ActorRuntimeLimits::new(1, 1, 1, 1, 1, 2, 1, 1),
        &control,
    )
    .expect("all one-valued runtime bounds are valid");
    let maximum = u32::MAX as usize;
    ActorRuntime::new(
        &checked,
        ActorRuntimeId::new(49),
        ActorRuntimeLimits::new(
            maximum, maximum, maximum, maximum, maximum, maximum, maximum, maximum,
        ),
        &control,
    )
    .expect("the maximum non-overflow Actor identity bound is valid without allocation");
}
