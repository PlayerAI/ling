use ling_ast::lower as lower_ast;
use ling_effects::{CheckedProgram, check};
use ling_eval::{
    ActorFaultPhase, ActorInstanceState, ActorRuntime, ActorRuntimeEventKind, ActorRuntimeId,
    ActorRuntimeLimits, ActorRuntimeState, ActorSendErrorKind, ActorSenderId, ActorShutdownReason,
    ActorShutdownResult, ActorSpawnError, ActorStopResult, ActorTurnResult, ActorValue,
    LocalTaskControl, MemoryConsole, RuntimeFaultKind,
};
use ling_hir::lower as lower_hir;
use ling_resolve::{DefinitionId, resolve};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;
use num_bigint::BigInt;

const COUNTER: &str = concat!(
    "module Main\n\n",
    "actor Counter : Int =\n",
    "    mailbox capacity 2 overflow Reject\n",
    "    state Int = 0\n",
    "    receive state message =\n",
    "        state + message\n\n",
    "let main () = ()\n",
);

fn checked(source_name: &str, text: &str) -> CheckedProgram {
    checked_bytes(source_name, text.as_bytes().to_vec())
}

fn checked_bytes(source_name: &str, bytes: Vec<u8>) -> CheckedProgram {
    let source =
        SourceFile::from_bytes(SourceId::new(0), source_name, bytes).expect("valid source");
    let parsed = parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = lower_ast(&source, &parsed).expect("valid AST");
    let hir = lower_hir(source.name(), &ast).expect("valid HIR");
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

fn limits() -> ActorRuntimeLimits {
    ActorRuntimeLimits::new(8, 8, 16, 128, 128, 128, 8, 8)
}

fn integer(value: i64) -> ActorValue {
    ActorValue::Int(BigInt::from(value))
}

#[test]
fn checked_actor_runs_only_by_explicit_identity_and_commits_one_fifo_turn() {
    let checked = checked("actor-runtime.ling", COUNTER);
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(7), limits(), &control).expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime.spawn(&counter, &mut console).expect("Actor starts");

    assert_eq!(reference.actor().get(), 1);
    assert_eq!(runtime.ready(), []);
    assert_eq!(
        runtime
            .snapshot(reference.actor())
            .expect("snapshot")
            .state(),
        Some(&integer(0))
    );

    let sender = ActorSenderId::new(11);
    runtime
        .send(&reference, sender, integer(2))
        .expect("first send");
    runtime
        .send(&reference, sender, integer(3))
        .expect("second send");
    assert_eq!(runtime.ready(), [reference.actor()]);

    assert_eq!(
        runtime
            .step(reference.actor(), &mut console)
            .expect("first turn"),
        ActorTurnResult::Completed {
            actor: reference.actor(),
            state: integer(2),
            remaining_messages: 1,
        }
    );
    assert_eq!(
        runtime
            .step(reference.actor(), &mut console)
            .expect("second turn"),
        ActorTurnResult::Completed {
            actor: reference.actor(),
            state: integer(5),
            remaining_messages: 0,
        }
    );
    assert_eq!(runtime.ready(), []);
    let accepted = runtime
        .events()
        .into_iter()
        .filter_map(|event| match event.kind().clone() {
            ActorRuntimeEventKind::MessageAccepted {
                sender_sequence,
                admission_sequence,
                ..
            } => Some((sender_sequence, admission_sequence)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(accepted, [(1, 1), (2, 2)]);
}

#[test]
fn full_mailbox_rejects_without_consuming_payload_or_ordering_identity() {
    let checked = checked("mailbox.ling", COUNTER);
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(1), limits(), &control).expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime.spawn(&counter, &mut console).expect("Actor starts");
    let sender = ActorSenderId::new(1);
    runtime
        .send(&reference, sender, integer(1))
        .expect("first send");
    runtime
        .send(&reference, sender, integer(2))
        .expect("second send");

    let rejected = runtime
        .send(&reference, sender, integer(3))
        .expect_err("bounded mailbox is full");
    assert!(matches!(
        rejected.kind(),
        ActorSendErrorKind::Full {
            resource: "actor_mailbox",
            limit: 2
        }
    ));
    assert_eq!(rejected.into_payload(), integer(3));
    assert_eq!(runtime.metrics().commands(), 3);
    assert_eq!(runtime.metrics().queued_messages(), 2);
    let accepted = runtime
        .events()
        .into_iter()
        .filter(|event| matches!(event.kind(), ActorRuntimeEventKind::MessageAccepted { .. }))
        .count();
    assert_eq!(accepted, 2);
}

#[test]
fn wrong_payload_type_is_rejected_before_mailbox_mutation() {
    let checked = checked("typed-send.ling", COUNTER);
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(1), limits(), &control).expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime.spawn(&counter, &mut console).expect("Actor starts");

    let rejected = runtime
        .send(
            &reference,
            ActorSenderId::new(1),
            ActorValue::Text("not-an-int".to_owned()),
        )
        .expect_err("message type mismatch");
    assert_eq!(rejected.kind(), &ActorSendErrorKind::PayloadTypeMismatch);
    assert_eq!(
        rejected.into_payload(),
        ActorValue::Text("not-an-int".to_owned())
    );
    assert_eq!(runtime.metrics().queued_messages(), 0);
    assert_eq!(runtime.metrics().commands(), 1);
}

#[test]
fn cross_run_reference_is_rejected_with_unchanged_payload() {
    let checked = checked("cross-run.ling", COUNTER);
    let counter = actor(&checked, "Counter");
    let first_control = LocalTaskControl::new();
    let second_control = LocalTaskControl::new();
    let mut first = ActorRuntime::new(&checked, ActorRuntimeId::new(1), limits(), &first_control)
        .expect("first runtime");
    let mut second = ActorRuntime::new(&checked, ActorRuntimeId::new(2), limits(), &second_control)
        .expect("second runtime");
    let mut console = MemoryConsole::default();
    let first_reference = first
        .spawn(&counter, &mut console)
        .expect("Actor starts in first runtime");

    let rejected = second
        .send(&first_reference, ActorSenderId::new(1), integer(4))
        .expect_err("cross-run reference is invalid");
    assert_eq!(rejected.kind(), &ActorSendErrorKind::WrongRuntime);
    assert_eq!(rejected.into_payload(), integer(4));
    assert_eq!(second.metrics().commands(), 0);
    assert_eq!(second.metrics().queued_messages(), 0);
}

#[test]
fn ready_set_is_stable_by_actor_identity_not_send_order() {
    let checked = checked("ready-order.ling", COUNTER);
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(1), limits(), &control).expect("runtime");
    let mut console = MemoryConsole::default();
    let first = runtime.spawn(&counter, &mut console).expect("first Actor");
    let second = runtime.spawn(&counter, &mut console).expect("second Actor");
    runtime
        .send(&second, ActorSenderId::new(2), integer(20))
        .expect("send to second");
    runtime
        .send(&first, ActorSenderId::new(1), integer(10))
        .expect("send to first");

    assert_eq!(runtime.ready(), [first.actor(), second.actor()]);
    runtime
        .step(second.actor(), &mut console)
        .expect("caller explicitly selects second Actor");
    assert_eq!(runtime.ready(), [first.actor()]);
}

#[test]
fn turn_fault_preserves_previous_commit_and_cancels_the_root_task() {
    let checked = checked(
        "actor-fault.ling",
        concat!(
            "module Main\n\n",
            "actor Divider : Int =\n",
            "    mailbox capacity 4 overflow Reject\n",
            "    state Int = 10\n",
            "    receive state message =\n",
            "        state / message\n\n",
            "let main () = ()\n",
        ),
    );
    let divider = actor(&checked, "Divider");
    let control = LocalTaskControl::new();
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(1), limits(), &control).expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime.spawn(&divider, &mut console).expect("Actor starts");
    runtime
        .send(&reference, ActorSenderId::new(1), integer(0))
        .expect("zero reaches mailbox");

    let result = runtime
        .step(reference.actor(), &mut console)
        .expect("fault is a contained turn result");
    let ActorTurnResult::Faulted {
        previous_state,
        fault,
        discarded_messages,
        ..
    } = result
    else {
        panic!("division by zero must fault the Actor turn");
    };
    assert_eq!(previous_state, integer(10));
    assert_eq!(fault.phase(), ActorFaultPhase::Turn);
    assert_eq!(fault.runtime(), ActorRuntimeId::new(1));
    assert_eq!(fault.actor_type(), reference.actor_type());
    assert!(matches!(
        fault.cause().kind,
        RuntimeFaultKind::DivisionByZero
    ));
    assert_eq!(discarded_messages, 0);
    assert!(control.is_cancelled());
    assert_eq!(runtime.state(), ActorRuntimeState::Failed);
    let snapshot = runtime
        .snapshot(reference.actor())
        .expect("terminal record");
    assert_eq!(snapshot.lifecycle(), ActorInstanceState::Failed);
    assert_eq!(snapshot.state(), None);
    assert_eq!(snapshot.cleanup_count(), 1);
    assert_eq!(snapshot.fault(), Some(&fault));
}

#[test]
fn event_exhaustion_precedes_turn_dequeue_or_command_publication() {
    let checked = checked("event-limit.ling", COUNTER);
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let event_bounded = ActorRuntimeLimits::new(1, 1, 2, 16, 16, 2, 1, 1);
    let mut runtime = ActorRuntime::new(&checked, ActorRuntimeId::new(1), event_bounded, &control)
        .expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime
        .spawn(&counter, &mut console)
        .expect("spawn reserves its fault/shutdown event budget");
    runtime
        .send(&reference, ActorSenderId::new(1), integer(5))
        .expect("send consumes the final event slot");

    let error = runtime
        .step(reference.actor(), &mut console)
        .expect_err("turn cannot reserve success/fault events");
    assert!(matches!(
        error,
        ling_eval::ActorRuntimeError::ResourceExhausted {
            resource: "events",
            limit: 2
        }
    ));
    assert_eq!(runtime.metrics().commands(), 2);
    assert_eq!(runtime.metrics().turns(), 0);
    assert_eq!(runtime.metrics().queued_messages(), 1);
    assert_eq!(
        runtime
            .snapshot(reference.actor())
            .expect("snapshot")
            .state(),
        Some(&integer(0))
    );
}

#[test]
fn unicode_bom_and_crlf_reconstruct_the_same_explicit_runtime_trace() {
    let source = concat!(
        "module Main\n\n",
        "actor 计数器 : Int =\n",
        "    mailbox capacity 2 overflow Reject\n",
        "    state Int = 1\n",
        "    receive 状态 消息 =\n",
        "        状态 + 消息\n\n",
        "let main () = ()\n",
    );
    let variants = [
        ("unicode-lf.ling", source.as_bytes().to_vec()),
        (
            "unicode-crlf.ling",
            source.replace('\n', "\r\n").into_bytes(),
        ),
        ("unicode-bom.ling", format!("\u{feff}{source}").into_bytes()),
    ];
    let traces = variants
        .into_iter()
        .map(|(source_name, bytes)| {
            let checked = checked_bytes(source_name, bytes);
            let counter = actor(&checked, "计数器");
            let control = LocalTaskControl::new();
            let mut runtime =
                ActorRuntime::new(&checked, ActorRuntimeId::new(9), limits(), &control)
                    .expect("runtime");
            let mut console = MemoryConsole::default();
            let reference = runtime
                .spawn(&counter, &mut console)
                .expect("Unicode Actor starts");
            runtime
                .send(&reference, ActorSenderId::new(7), integer(4))
                .expect("typed message");
            let turn = runtime.step(reference.actor(), &mut console).expect("turn");
            (
                counter,
                reference.actor_type(),
                reference.message_schema().to_owned(),
                turn,
                runtime
                    .events()
                    .into_iter()
                    .map(|event| event.kind().clone())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(traces[0], traces[1]);
    assert_eq!(traces[0], traces[2]);
}

#[test]
fn actor_fault_preserves_original_bom_crlf_utf8_byte_span() {
    let source = concat!(
        "\u{feff}module Main\r\n\r\n",
        "actor 除法器 : Int =\r\n",
        "    mailbox capacity 1 overflow Reject\r\n",
        "    state Int = 10\r\n",
        "    receive 状态 消息 =\r\n",
        "        状态 / 消息\r\n\r\n",
        "let main () = ()\r\n",
    );
    let checked = checked_bytes("故障.ling", source.as_bytes().to_vec());
    let divider = actor(&checked, "除法器");
    let control = LocalTaskControl::new();
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(1), limits(), &control).expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime.spawn(&divider, &mut console).expect("Actor starts");
    runtime
        .send(&reference, ActorSenderId::new(1), integer(0))
        .expect("zero reaches mailbox");
    let ActorTurnResult::Faulted { fault, .. } = runtime
        .step(reference.actor(), &mut console)
        .expect("contained fault")
    else {
        panic!("division by zero must fault");
    };
    let expression_start = u32::try_from(source.find("状态 / 消息").expect("expression"))
        .expect("test source fits a span");
    assert_eq!(fault.cause().source_name, "故障.ling");
    assert_eq!(fault.cause().span.start().get(), expression_start);
    assert!(fault.cause().span.end().get() > expression_start);
}

#[test]
fn initializer_fault_retires_identity_without_publishing_an_actor() {
    let checked = checked(
        "spawn-fault.ling",
        concat!(
            "module Main\n\n",
            "actor Broken : Int =\n",
            "    mailbox capacity 1 overflow Reject\n",
            "    state Int = 1 / 0\n",
            "    receive state message =\n",
            "        state\n\n",
            "let main () = ()\n",
        ),
    );
    let broken = actor(&checked, "Broken");
    let control = LocalTaskControl::new();
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(1), limits(), &control).expect("runtime");
    let mut console = MemoryConsole::default();

    let ActorSpawnError::Fault(fault) = runtime
        .spawn(&broken, &mut console)
        .expect_err("initializer faults")
    else {
        panic!("initializer must return an Actor Fault");
    };
    assert_eq!(fault.actor().get(), 1);
    assert_eq!(fault.phase(), ActorFaultPhase::Initializer);
    assert!(matches!(
        fault.cause().kind,
        RuntimeFaultKind::DivisionByZero
    ));
    assert!(runtime.reference(fault.actor()).is_none());
    assert_eq!(runtime.metrics().created_actors(), 1);
    assert_eq!(runtime.metrics().live_actors(), 0);
    assert_eq!(runtime.metrics().faults(), 1);
    assert!(control.is_cancelled());
    assert_eq!(runtime.state(), ActorRuntimeState::Failed);
}

#[test]
fn owner_cancellation_discards_mailboxes_and_runs_cleanup_exactly_once() {
    let checked = checked("cancel.ling", COUNTER);
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(1), limits(), &control).expect("runtime");
    let mut console = MemoryConsole::default();
    let first = runtime.spawn(&counter, &mut console).expect("first Actor");
    let second = runtime.spawn(&counter, &mut console).expect("second Actor");
    runtime
        .send(&first, ActorSenderId::new(1), integer(1))
        .expect("first message");
    runtime
        .send(&second, ActorSenderId::new(1), integer(2))
        .expect("second message");

    control.cancel();
    assert!(
        runtime
            .observe_owner_cancellation()
            .expect("cancellation has event capacity")
    );
    assert!(
        !runtime
            .observe_owner_cancellation()
            .expect("terminal observation is inert")
    );
    assert_eq!(runtime.state(), ActorRuntimeState::Stopped);
    assert_eq!(runtime.metrics().queued_messages(), 0);
    assert_eq!(runtime.metrics().discarded_messages(), 2);
    assert_eq!(runtime.metrics().cleanups(), 2);
    assert_eq!(
        runtime
            .snapshot(first.actor())
            .expect("first record")
            .cleanup_count(),
        1
    );
    assert_eq!(
        runtime
            .snapshot(second.actor())
            .expect("second record")
            .cleanup_count(),
        1
    );
    assert_eq!(
        runtime
            .shutdown(ActorShutdownReason::OwnerCancelled)
            .expect("idempotent shutdown"),
        ActorShutdownResult::AlreadyStopped
    );
    let rejected = runtime
        .send(&first, ActorSenderId::new(1), integer(9))
        .expect_err("closed mailbox rejects");
    assert_eq!(rejected.kind(), &ActorSendErrorKind::Closed);
    assert_eq!(rejected.into_payload(), integer(9));
}

#[test]
fn explicit_stop_is_idempotent_and_command_exhaustion_is_failure_atomic() {
    let checked = checked("limits.ling", COUNTER);
    let counter = actor(&checked, "Counter");
    let control = LocalTaskControl::new();
    let bounded = ActorRuntimeLimits::new(2, 2, 4, 2, 4, 16, 2, 2);
    let mut runtime =
        ActorRuntime::new(&checked, ActorRuntimeId::new(1), bounded, &control).expect("runtime");
    let mut console = MemoryConsole::default();
    let reference = runtime.spawn(&counter, &mut console).expect("Actor starts");
    runtime
        .send(&reference, ActorSenderId::new(1), integer(1))
        .expect("one send fits command bound");
    let rejected = runtime
        .send(&reference, ActorSenderId::new(1), integer(2))
        .expect_err("command bound rejects before enqueue");
    assert!(matches!(
        rejected.kind(),
        ActorSendErrorKind::ResourceExhausted {
            resource: "commands",
            limit: 2
        }
    ));
    assert_eq!(rejected.into_payload(), integer(2));
    assert_eq!(runtime.metrics().queued_messages(), 1);
    assert_eq!(runtime.metrics().commands(), 2);

    let roomy_control = LocalTaskControl::new();
    let mut roomy = ActorRuntime::new(&checked, ActorRuntimeId::new(2), limits(), &roomy_control)
        .expect("roomy runtime");
    let reference = roomy.spawn(&counter, &mut console).expect("Actor starts");
    roomy
        .send(&reference, ActorSenderId::new(1), integer(1))
        .expect("queued message");
    assert_eq!(
        roomy.stop(&reference).expect("first stop"),
        ActorStopResult::Stopped {
            discarded_messages: 1
        }
    );
    assert_eq!(
        roomy.stop(&reference).expect("repeated stop"),
        ActorStopResult::AlreadyStopped
    );
    assert_eq!(
        roomy
            .snapshot(reference.actor())
            .expect("record")
            .cleanup_count(),
        1
    );
}
