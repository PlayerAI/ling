//! REP-2502 private executable structure evidence authorized by Accepted DEC-0280.
//!
//! This module observes validated in-memory DEC-0267 Task traces. It does not
//! define an Effect Log, Replay wire schema, encoder, decoder, checksum,
//! privacy policy, migration format, or public compatibility contract.

use std::collections::BTreeSet;

use ling_ast::lower as lower_ast;
use ling_effects::{CheckedProgram, check};
use ling_hir::lower as lower_hir;
use ling_resolve::{DefinitionId, resolve};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;
use num_bigint::BigInt;

use crate::{
    HostErrorCategory, TaskDeadline, TaskFaultSummary, TaskHostOutcome, TaskHostResponse,
    TaskHostScript, TaskPath, TaskRuntimeLimits, TaskScheduleConfig, TaskScheduleEventKind,
    TaskScheduleTerminal, TaskScheduleTrace, TaskSchedulerError, TaskSchedulerLimits, TaskValue,
    run_task_schedule,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceCase {
    ValidatedPrivateEnvelopeProjection,
    EventIdentityKindOrderProjection,
    TypedPayloadTerminalProjection,
    MutationAndLimitRejection,
    PublicReplaySchemaAbsence,
}

const EVIDENCE_CASES: [(EvidenceCase, &str); 5] = [
    (
        EvidenceCase::ValidatedPrivateEnvelopeProjection,
        "validated-private-envelope-projection",
    ),
    (
        EvidenceCase::EventIdentityKindOrderProjection,
        "event-identity-kind-order-projection",
    ),
    (
        EvidenceCase::TypedPayloadTerminalProjection,
        "typed-payload-terminal-projection",
    ),
    (
        EvidenceCase::MutationAndLimitRejection,
        "mutation-and-limit-rejection",
    ),
    (
        EvidenceCase::PublicReplaySchemaAbsence,
        "public-replay-schema-absence",
    ),
];

fn assert_registered(case: EvidenceCase, expected_name: &str) {
    assert_eq!(EVIDENCE_CASES.len(), 5);
    let matches = EVIDENCE_CASES
        .iter()
        .filter(|(candidate, _)| *candidate == case)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "each DEC-0280 case occurs exactly once");
    assert_eq!(matches[0].1, expected_name);

    let names = EVIDENCE_CASES
        .iter()
        .map(|(_, name)| *name)
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), EVIDENCE_CASES.len());
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ReplayConcern {
    CanonicalEnvelope,
    EventId,
    EventKind,
    Ordering,
    Identity,
    Checksum,
    DeterminismClass,
    Toolchain,
    Profile,
    Schema,
    Payload,
    Migration,
    Privacy,
}

impl ReplayConcern {
    const fn name(self) -> &'static str {
        match self {
            Self::CanonicalEnvelope => "canonical-envelope",
            Self::EventId => "event-id",
            Self::EventKind => "event-kind",
            Self::Ordering => "ordering",
            Self::Identity => "identity",
            Self::Checksum => "checksum",
            Self::DeterminismClass => "determinism-class",
            Self::Toolchain => "toolchain",
            Self::Profile => "profile",
            Self::Schema => "schema",
            Self::Payload => "payload",
            Self::Migration => "migration",
            Self::Privacy => "privacy",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConcernDisposition {
    ExistingPrivateTraceEvidence,
    DeferredPublicContract,
}

const CONCERN_DISPOSITIONS: [(ReplayConcern, ConcernDisposition); 13] = [
    (
        ReplayConcern::CanonicalEnvelope,
        ConcernDisposition::ExistingPrivateTraceEvidence,
    ),
    (
        ReplayConcern::EventId,
        ConcernDisposition::ExistingPrivateTraceEvidence,
    ),
    (
        ReplayConcern::EventKind,
        ConcernDisposition::ExistingPrivateTraceEvidence,
    ),
    (
        ReplayConcern::Ordering,
        ConcernDisposition::ExistingPrivateTraceEvidence,
    ),
    (
        ReplayConcern::Identity,
        ConcernDisposition::ExistingPrivateTraceEvidence,
    ),
    (
        ReplayConcern::Checksum,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        ReplayConcern::DeterminismClass,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        ReplayConcern::Toolchain,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        ReplayConcern::Profile,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        ReplayConcern::Schema,
        ConcernDisposition::ExistingPrivateTraceEvidence,
    ),
    (
        ReplayConcern::Payload,
        ConcernDisposition::ExistingPrivateTraceEvidence,
    ),
    (
        ReplayConcern::Migration,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        ReplayConcern::Privacy,
        ConcernDisposition::DeferredPublicContract,
    ),
];

fn assert_concern_inventory() {
    assert_eq!(CONCERN_DISPOSITIONS.len(), 13);
    let concerns = CONCERN_DISPOSITIONS
        .iter()
        .map(|(concern, _)| *concern)
        .collect::<BTreeSet<_>>();
    assert_eq!(concerns.len(), CONCERN_DISPOSITIONS.len());
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .filter(|(_, disposition)| {
                *disposition == ConcernDisposition::ExistingPrivateTraceEvidence
            })
            .count(),
        7
    );
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .filter(|(_, disposition)| {
                *disposition == ConcernDisposition::DeferredPublicContract
            })
            .count(),
        6
    );
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .map(|(concern, _)| concern.name())
            .collect::<Vec<_>>(),
        [
            "canonical-envelope",
            "event-id",
            "event-kind",
            "ordering",
            "identity",
            "checksum",
            "determinism-class",
            "toolchain",
            "profile",
            "schema",
            "payload",
            "migration",
            "privacy",
        ]
    );
}

fn checked_at(source_id: u32, source_name: &str, bytes: Vec<u8>) -> CheckedProgram {
    let source = SourceFile::from_bytes(SourceId::new(source_id), source_name, bytes)
        .expect("valid UTF-8 source");
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
        .expect("checked Task definition")
}

fn scheduler_limits() -> TaskSchedulerLimits {
    TaskSchedulerLimits::new(128, 128, 16, 512, 2_048, 128, 16)
}

fn schedule_config(seed: u64) -> TaskScheduleConfig {
    TaskScheduleConfig::new(
        seed,
        TaskRuntimeLimits::new(32, 32, 256, 16),
        scheduler_limits(),
    )
}

fn integer(value: i64) -> TaskValue {
    TaskValue::Int(BigInt::from(value))
}

const TASK_LF: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "task 子任务 text =\n",
    "    scope\n",
    "        Console.write text\n",
    "        return text\n\n",
    "task 父任务 text =\n",
    "    scope\n",
    "        let handle = spawn 子任务 text\n",
    "        let result = await handle\n",
    "        return result\n",
);

const TASK_BOM_CRLF: &str = concat!(
    "\u{feff}module Main\r\n",
    "    requires Console.Write\r\n\r\n",
    "task 子任务 text =\r\n",
    "    scope\r\n",
    "        Console.write text\r\n",
    "        return text\r\n\r\n",
    "task 父任务 text =\r\n",
    "    scope\r\n",
    "        let handle = spawn 子任务 text\r\n",
    "        let result = await handle\r\n",
    "        return result\r\n",
);

const ECHO: &str = concat!(
    "module Main\n\n",
    "task echo value =\n",
    "    scope\n",
    "        return value\n",
);

const ORDERED_TASKS: &str = concat!(
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

fn run_writer_trace(
    checked: &CheckedProgram,
    root: &DefinitionId,
    response: TaskHostResponse,
) -> TaskScheduleTrace {
    run_task_schedule(
        checked,
        root,
        vec![TaskValue::Text("你好".to_owned())],
        schedule_config(0x0280_0001),
        vec![TaskDeadline::new(64, TaskPath::root())],
        TaskHostScript::new([response]),
    )
    .expect("bounded Task trace")
}

fn closure(
    trace: &TaskScheduleTrace,
) -> (
    &TaskScheduleTerminal,
    &[(TaskPath, usize)],
    &[TaskFaultSummary],
) {
    match trace.events().last().expect("closure event").kind() {
        TaskScheduleEventKind::Closure {
            terminal,
            cleanup,
            faults,
        } => (terminal, cleanup, faults),
        other => panic!("expected closure, found {other:?}"),
    }
}

#[derive(Debug, PartialEq)]
struct EnvelopeProjection {
    version: String,
    config: TaskScheduleConfig,
    runtime_identity: Vec<u8>,
    deadlines: Vec<(u64, TaskPath)>,
    host_responses: Vec<TaskHostResponse>,
    event_count: usize,
    canonical_bytes: Vec<u8>,
}

fn envelope_projection(trace: &TaskScheduleTrace) -> EnvelopeProjection {
    trace.validate().expect("validated DEC-0267 trace");
    let projection = EnvelopeProjection {
        version: trace.version().to_owned(),
        config: trace.config(),
        runtime_identity: trace.runtime_identity().to_vec(),
        deadlines: trace
            .deadlines()
            .iter()
            .map(|deadline| (deadline.tick(), deadline.task().clone()))
            .collect(),
        host_responses: trace.host_script().responses().to_vec(),
        event_count: trace.events().len(),
        canonical_bytes: trace.canonical_bytes(),
    };
    assert!(!projection.runtime_identity.is_empty());
    assert!(!projection.canonical_bytes.is_empty());
    assert!(projection.event_count > 0);
    assert!(projection.event_count <= projection.config.scheduler_limits().max_trace_events());
    projection
}

#[test]
fn validated_private_envelope_projection() {
    assert_registered(
        EvidenceCase::ValidatedPrivateEnvelopeProjection,
        "validated-private-envelope-projection",
    );
    assert_concern_inventory();

    let left = checked_at(28, "左侧/record.ling", TASK_LF.as_bytes().to_vec());
    let reconstructed = checked_at(
        280,
        "different/right-record.ling",
        TASK_BOM_CRLF.as_bytes().to_vec(),
    );
    let left_root = task(&left, "父任务");
    let reconstructed_root = task(&reconstructed, "父任务");

    for response in [
        TaskHostResponse::Complete,
        TaskHostResponse::Fail(HostErrorCategory::BrokenPipe),
    ] {
        let first = run_writer_trace(&left, &left_root, response);
        let repeated = run_writer_trace(&left, &left_root, response);
        let rebuilt = run_writer_trace(&reconstructed, &reconstructed_root, response);
        assert_eq!(envelope_projection(&first), envelope_projection(&repeated));
        assert_eq!(envelope_projection(&first), envelope_projection(&rebuilt));
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PrivateEventKind {
    Selection,
    Deadline,
    Host,
    Closure,
}

fn assert_event_invariants(trace: &TaskScheduleTrace) -> BTreeSet<PrivateEventKind> {
    trace.validate().expect("validated event trace");
    let mut kinds = BTreeSet::new();
    let mut last_tick = 0;
    let mut closures = 0;
    for (index, event) in trace.events().iter().enumerate() {
        assert_eq!(event.id(), index as u64 + 1);
        assert!(event.tick() >= last_tick);
        last_tick = event.tick();
        match event.kind() {
            TaskScheduleEventKind::Selection {
                ready, selected, ..
            } => {
                assert!(!ready.is_empty());
                assert!(ready.windows(2).all(|pair| pair[0] < pair[1]));
                assert!(ready.binary_search(selected).is_ok());
                kinds.insert(PrivateEventKind::Selection);
            }
            TaskScheduleEventKind::Deadline { .. } => {
                kinds.insert(PrivateEventKind::Deadline);
            }
            TaskScheduleEventKind::Host { .. } => {
                kinds.insert(PrivateEventKind::Host);
            }
            TaskScheduleEventKind::Closure { .. } => {
                closures += 1;
                assert_eq!(index + 1, trace.events().len());
                kinds.insert(PrivateEventKind::Closure);
            }
        }
    }
    assert_eq!(closures, 1);
    kinds
}

#[test]
fn event_identity_kind_order_projection() {
    assert_registered(
        EvidenceCase::EventIdentityKindOrderProjection,
        "event-identity-kind-order-projection",
    );
    let writer = checked_at(29, "events/writer.ling", TASK_LF.as_bytes().to_vec());
    let writer_root = task(&writer, "父任务");
    let writer_trace = run_writer_trace(&writer, &writer_root, TaskHostResponse::Complete);

    let echo = checked_at(30, "events/deadline.ling", ECHO.as_bytes().to_vec());
    let echo_root = task(&echo, "echo");
    let deadline_trace = run_task_schedule(
        &echo,
        &echo_root,
        vec![integer(7)],
        schedule_config(0x0280_0002),
        vec![TaskDeadline::new(0, TaskPath::root())],
        TaskHostScript::default(),
    )
    .expect("deadline trace");

    let ordered = checked_at(31, "events/ordered.ling", ORDERED_TASKS.as_bytes().to_vec());
    let ordered_root = task(&ordered, "parent");
    let child = TaskPath::from_segments([2]).expect("canonical child path");
    let ordered_trace = run_task_schedule(
        &ordered,
        &ordered_root,
        vec![integer(1)],
        schedule_config(0x0280_0003),
        vec![
            TaskDeadline::new(2, child.clone()),
            TaskDeadline::new(2, TaskPath::root()),
        ],
        TaskHostScript::default(),
    )
    .expect("canonical deadline trace");
    assert_eq!(ordered_trace.deadlines()[0].task(), &TaskPath::root());
    assert_eq!(ordered_trace.deadlines()[1].task(), &child);
    assert!(
        ordered_trace
            .deadlines()
            .windows(2)
            .all(|pair| { (pair[0].tick(), pair[0].task()) < (pair[1].tick(), pair[1].task()) })
    );

    let kinds = [writer_trace, deadline_trace, ordered_trace]
        .iter()
        .flat_map(assert_event_invariants)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        kinds,
        BTreeSet::from([
            PrivateEventKind::Selection,
            PrivateEventKind::Deadline,
            PrivateEventKind::Host,
            PrivateEventKind::Closure,
        ])
    );
}

#[test]
fn typed_payload_terminal_projection() {
    assert_registered(
        EvidenceCase::TypedPayloadTerminalProjection,
        "typed-payload-terminal-projection",
    );
    let left = checked_at(32, "payload/左侧.ling", TASK_LF.as_bytes().to_vec());
    let right = checked_at(
        320,
        "different/payload-right.ling",
        TASK_BOM_CRLF.as_bytes().to_vec(),
    );
    let left_root = task(&left, "父任务");
    let right_root = task(&right, "父任务");

    let success = run_writer_trace(&left, &left_root, TaskHostResponse::Complete);
    assert!(success.events().iter().any(|event| matches!(
        event.kind(),
        TaskScheduleEventKind::Host { text, outcome }
            if text == "你好\n" && outcome == &TaskHostOutcome::Completed
    )));
    let (success_terminal, success_cleanup, success_faults) = closure(&success);
    assert_eq!(
        success_terminal,
        &TaskScheduleTerminal::Completed(TaskValue::Text("你好".to_owned()))
    );
    assert!(success_faults.is_empty());
    assert!(success_cleanup.iter().all(|(_, count)| *count == 1));

    let failed = run_writer_trace(
        &left,
        &left_root,
        TaskHostResponse::Fail(HostErrorCategory::BrokenPipe),
    );
    let rebuilt_failed = run_writer_trace(
        &right,
        &right_root,
        TaskHostResponse::Fail(HostErrorCategory::BrokenPipe),
    );
    assert!(failed.events().iter().any(|event| matches!(
        event.kind(),
        TaskScheduleEventKind::Host { text, outcome }
            if text == "你好\n"
                && outcome == &TaskHostOutcome::Failed(HostErrorCategory::BrokenPipe)
    )));
    let (failed_terminal, failed_cleanup, failed_faults) = closure(&failed);
    assert_eq!(
        failed_terminal,
        &TaskScheduleTerminal::Faulted { fault_count: 1 }
    );
    assert_eq!(failed_faults.len(), 1);
    assert_eq!(failed_faults[0].category(), "host_capability");
    assert_eq!(failed_faults[0].operation(), "Console.write");
    assert_eq!(failed_faults[0].detail(), "broken_pipe");
    assert!(failed_cleanup.iter().all(|(_, count)| *count == 1));
    assert!(failed_cleanup.windows(2).all(|pair| pair[0].0 < pair[1].0));
    assert!(
        failed_faults
            .windows(2)
            .all(|pair| pair[0].task() < pair[1].task())
    );

    let (_, rebuilt_cleanup, rebuilt_faults) = closure(&rebuilt_failed);
    assert!(rebuilt_cleanup.iter().all(|(_, count)| *count == 1));
    assert_eq!(failed.canonical_bytes(), rebuilt_failed.canonical_bytes());
    assert_eq!(failed_faults[0].source_name(), "payload/左侧.ling");
    assert_eq!(
        rebuilt_faults[0].source_name(),
        "different/payload-right.ling"
    );
    assert_ne!(
        failed_faults[0].source_span(),
        rebuilt_faults[0].source_span()
    );
}

#[test]
fn mutation_and_limit_rejection() {
    assert_registered(
        EvidenceCase::MutationAndLimitRejection,
        "mutation-and-limit-rejection",
    );
    crate::task_scheduler::tests::trace_validation_rejects_version_event_and_closure_corruption();
    crate::task_scheduler::tests::replay_reports_the_first_mutated_selection_field();
    crate::task_scheduler::tests::replay_reports_deadline_host_and_terminal_mutations_at_their_event();

    let checked = checked_at(33, "limits/echo.ling", ECHO.as_bytes().to_vec());
    let root = task(&checked, "echo");
    let scheduler_base = [16usize, 16, 4, 64, 32, 16, 4];
    for index in 0..scheduler_base.len() {
        let mut values = scheduler_base;
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

    let runtime_base = [8usize, 8, 32, 8];
    for index in 0..runtime_base.len() {
        let mut values = runtime_base;
        values[index] = 0;
        let limits = TaskRuntimeLimits::new(values[0], values[1], values[2], values[3]);
        assert!(matches!(
            run_task_schedule(
                &checked,
                &root,
                vec![integer(1)],
                TaskScheduleConfig::new(0, limits, scheduler_limits()),
                vec![],
                TaskHostScript::default(),
            ),
            Err(TaskSchedulerError::InvalidLimit {
                limit: "TaskRuntimeLimits"
            })
        ));
    }
}

fn assert_public_replay_surfaces_absent() {
    const EVAL_SOURCE: &str = include_str!("lib.rs");
    const EFFECT_SOURCE: &str = include_str!("../../ling-effects/src/lib.rs");
    const SEMANTIC_SOURCE: &str = include_str!("../../ling-semantic/src/lib.rs");
    const PROJECT_SOURCE: &str = include_str!("../../ling-project/src/lib.rs");
    const BYTECODE_SOURCE: &str = include_str!("../../ling-bytecode/src/lib.rs");
    const VM_SOURCE: &str = include_str!("../../ling-vm/src/lib.rs");
    const SOURCE_SOURCE: &str = include_str!("../../ling-source/src/lib.rs");
    const CLI_CATALOG_SOURCE: &str = include_str!("../../ling-cli/src/command_catalog.rs");
    const DIAGNOSTIC_CODES: &str = include_str!("../../../docs/ERROR-CODES.md");
    const SCHEMA_REGISTRY: &str = include_str!("../../../schemas/registry.toml");
    const PROTOCOL_INVENTORY: &str =
        include_str!("../../../docs/governance/protocol-inventory.toml");

    for production_source in [
        EVAL_SOURCE,
        EFFECT_SOURCE,
        SEMANTIC_SOURCE,
        PROJECT_SOURCE,
        BYTECODE_SOURCE,
        VM_SOURCE,
        SOURCE_SOURCE,
    ] {
        for forbidden_surface in [
            "pub struct ReplayEnvelope",
            "pub enum ReplayEvent",
            "pub struct EffectLog",
            "pub enum EffectLog",
            "pub struct ReplayHeader",
            "pub struct ReplayWriter",
            "pub struct ReplayReader",
            "pub fn encode_replay",
            "pub fn decode_replay",
            "pub fn write_replay",
            "pub fn read_replay",
            "replay_checksum:",
            "replay_privacy:",
            "replay_migration:",
            "replay_header:",
        ] {
            assert!(
                !production_source.contains(forbidden_surface),
                "forbidden DEC-0280 production surface: {forbidden_surface}"
            );
        }
    }
    assert!(!CLI_CATALOG_SOURCE.contains("Command::Replay"));
    assert!(!CLI_CATALOG_SOURCE.contains("\"replay\" =>"));
    assert!(!DIAGNOSTIC_CODES.contains("L-REPLAY-"));
    assert!(!SCHEMA_REGISTRY.to_ascii_lowercase().contains("replay"));

    let replay_protocol = PROTOCOL_INVENTORY
        .split("[[protocol]]")
        .find(|block| block.contains("id = \"PROTO-REPLAY\""))
        .expect("planned Replay inventory record");
    for future_boundary in [
        "current_version = \"\"",
        "stability = \"Future\"",
        "implemented = false",
        "public_schema = false",
        "canonical = false",
        "fixtures = []",
        "version_markers = []",
    ] {
        assert!(replay_protocol.contains(future_boundary));
    }
}

#[test]
fn public_replay_schema_absence() {
    assert_registered(
        EvidenceCase::PublicReplaySchemaAbsence,
        "public-replay-schema-absence",
    );
    assert_concern_inventory();
    assert_public_replay_surfaces_absent();
}
