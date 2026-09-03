//! REP-2504 private Task replay-player evidence authorized by DEC-0282.
//!
//! This module exercises only the existing in-memory DEC-0267 Task trace and
//! fresh-runtime replay path. It does not define a checkpoint, persisted log
//! reader, public Replay Player, CLI command, wire protocol, privacy policy,
//! integrity mechanism, migration rule, or cross-process guarantee.

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
    HostErrorCategory, TaskDeadline, TaskHostOutcome, TaskHostResponse, TaskHostScript, TaskPath,
    TaskRuntimeLimits, TaskScheduleConfig, TaskScheduleEventKind, TaskScheduleTerminal,
    TaskScheduleTrace, TaskSchedulerLimits, TaskValue, replay_task_schedule, run_task_schedule,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceCase {
    ValidatedTraceExactReplay,
    CheckedRecipePreflight,
    FirstEventDivergence,
    FaultAndCancellationReplay,
    DeferredCheckpointAndPublicSurfaceAbsence,
}

const EVIDENCE_CASES: [(EvidenceCase, &str); 5] = [
    (
        EvidenceCase::ValidatedTraceExactReplay,
        "validated-trace-exact-replay",
    ),
    (
        EvidenceCase::CheckedRecipePreflight,
        "checked-recipe-preflight",
    ),
    (EvidenceCase::FirstEventDivergence, "first-event-divergence"),
    (
        EvidenceCase::FaultAndCancellationReplay,
        "fault-and-cancellation-replay",
    ),
    (
        EvidenceCase::DeferredCheckpointAndPublicSurfaceAbsence,
        "deferred-checkpoint-and-public-surface-absence",
    ),
];

fn assert_registered(case: EvidenceCase, expected_name: &str) {
    assert_eq!(EVIDENCE_CASES.len(), 5);
    let matches = EVIDENCE_CASES
        .iter()
        .filter(|(candidate, _)| *candidate == case)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "each DEC-0282 case occurs exactly once");
    assert_eq!(matches[0].1, expected_name);
    assert_eq!(
        EVIDENCE_CASES
            .iter()
            .map(|(_, name)| *name)
            .collect::<BTreeSet<_>>()
            .len(),
        EVIDENCE_CASES.len()
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
    check(typed).expect("checked Effect/Capability program")
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

fn integer(value: i64) -> TaskValue {
    TaskValue::Int(BigInt::from(value))
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

fn terminal(trace: &TaskScheduleTrace) -> &TaskScheduleTerminal {
    match trace.events().last().expect("finite trace closure").kind() {
        TaskScheduleEventKind::Closure { terminal, .. } => terminal,
        other => panic!("expected closure, found {other:?}"),
    }
}

const PARENT_LF: &str = concat!(
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

const PARENT_BOM_CRLF: &str = concat!(
    "\u{feff}module Main\r\n\r\n",
    "task child value =\r\n",
    "    scope\r\n",
    "        return value + 1\r\n\r\n",
    "task parent value =\r\n",
    "    scope\r\n",
    "        let handle = spawn child value\r\n",
    "        let result = await handle\r\n",
    "        return result\r\n",
);

const ALTERED_PARENT: &str = concat!(
    "module Main\n\n",
    "task child value =\n",
    "    scope\n",
    "        return value + 2\n\n",
    "task parent value =\n",
    "    scope\n",
    "        let handle = spawn child value\n",
    "        let result = await handle\n",
    "        return result\n",
);

const WRITER_LF: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "task writer text =\n",
    "    scope\n",
    "        Console.write text\n",
    "        return text\n",
);

const WRITER_BOM_CRLF: &str = concat!(
    "\u{feff}module Main\r\n",
    "    requires Console.Write\r\n\r\n",
    "task writer text =\r\n",
    "    scope\r\n",
    "        Console.write text\r\n",
    "        return text\r\n",
);

const ECHO: &str = concat!(
    "module Main\n\n",
    "task echo value =\n",
    "    scope\n",
    "        return value\n",
);

fn parent_trace(checked: &CheckedProgram, root: &DefinitionId) -> TaskScheduleTrace {
    run_task_schedule(
        checked,
        root,
        vec![integer(10)],
        config(u64::MAX),
        vec![],
        TaskHostScript::default(),
    )
    .expect("validated parent trace")
}

#[test]
fn validated_trace_exact_replay() {
    assert_registered(
        EvidenceCase::ValidatedTraceExactReplay,
        "validated-trace-exact-replay",
    );
    let original = checked_at(2_820, "record/源.ling", PARENT_LF.as_bytes().to_vec());
    let rebuilt = checked_at(
        28_200,
        "different/replay.ling",
        PARENT_BOM_CRLF.as_bytes().to_vec(),
    );
    let original_root = task(&original, "parent");
    let rebuilt_root = task(&rebuilt, "parent");
    let expected = parent_trace(&original, &original_root);
    expected.validate().expect("valid private Task trace");

    let replayed = replay_task_schedule(&rebuilt, &rebuilt_root, vec![integer(10)], &expected)
        .expect("strict fresh-runtime replay");
    replayed.validate().expect("valid reconstructed trace");
    assert_eq!(expected.canonical_bytes(), replayed.canonical_bytes());
    assert_eq!(terminal(&expected), terminal(&replayed));
    assert_eq!(
        terminal(&replayed),
        &TaskScheduleTerminal::Completed(integer(11))
    );
}

fn assert_identity_mismatch(
    label: &str,
    checked: &CheckedProgram,
    root: &DefinitionId,
    arguments: Vec<TaskValue>,
    expected: &TaskScheduleTrace,
) {
    let error = replay_task_schedule(checked, root, arguments, expected)
        .expect_err("changed checked recipe must fail preflight");
    assert_eq!(error.event_id(), 0, "{label}");
    assert_eq!(error.reason(), "runtime_identity_mismatch", "{label}");
}

#[test]
fn checked_recipe_preflight() {
    assert_registered(
        EvidenceCase::CheckedRecipePreflight,
        "checked-recipe-preflight",
    );
    let original = checked_at(
        2_821,
        "preflight/original.ling",
        PARENT_LF.as_bytes().to_vec(),
    );
    let equivalent = checked_at(
        28_210,
        "preflight/equivalent.ling",
        PARENT_BOM_CRLF.as_bytes().to_vec(),
    );
    let altered = checked_at(
        2_822,
        "preflight/altered.ling",
        ALTERED_PARENT.as_bytes().to_vec(),
    );
    let original_root = task(&original, "parent");
    let equivalent_root = task(&equivalent, "parent");
    let altered_root = task(&altered, "parent");
    let changed_root = task(&equivalent, "child");
    let expected = parent_trace(&original, &original_root);

    replay_task_schedule(&equivalent, &equivalent_root, vec![integer(10)], &expected)
        .expect("source spelling and identity are not recipe identity");
    assert_identity_mismatch(
        "changed argument",
        &equivalent,
        &equivalent_root,
        vec![integer(11)],
        &expected,
    );
    assert_identity_mismatch(
        "changed Task Core",
        &altered,
        &altered_root,
        vec![integer(10)],
        &expected,
    );
    assert_identity_mismatch(
        "changed root Task",
        &equivalent,
        &changed_root,
        vec![integer(10)],
        &expected,
    );
}

#[test]
fn first_event_divergence() {
    assert_registered(EvidenceCase::FirstEventDivergence, "first-event-divergence");
    crate::task_scheduler::tests::trace_validation_rejects_version_event_and_closure_corruption();
    crate::task_scheduler::tests::replay_reports_the_first_mutated_selection_field();
    crate::task_scheduler::tests::replay_reports_deadline_host_and_terminal_mutations_at_their_event();
}

fn closure_fault_count(trace: &TaskScheduleTrace) -> usize {
    match trace.events().last().expect("finite trace closure").kind() {
        TaskScheduleEventKind::Closure {
            faults, cleanup, ..
        } => {
            assert!(cleanup.iter().all(|(_, count)| *count == 1));
            faults.len()
        }
        other => panic!("expected closure, found {other:?}"),
    }
}

#[test]
fn fault_and_cancellation_replay() {
    assert_registered(
        EvidenceCase::FaultAndCancellationReplay,
        "fault-and-cancellation-replay",
    );

    let writer = checked_at(2_823, "fault/writer.ling", WRITER_LF.as_bytes().to_vec());
    let rebuilt_writer = checked_at(
        28_230,
        "different/fault-writer.ling",
        WRITER_BOM_CRLF.as_bytes().to_vec(),
    );
    let writer_root = task(&writer, "writer");
    let rebuilt_writer_root = task(&rebuilt_writer, "writer");
    let failed = run_task_schedule(
        &writer,
        &writer_root,
        vec![TaskValue::Text("失败🙂".to_owned())],
        config(0x0282_0001),
        vec![],
        TaskHostScript::new([TaskHostResponse::Fail(HostErrorCategory::BrokenPipe)]),
    )
    .expect("bounded host-Fault trace");
    let replayed_failure = replay_task_schedule(
        &rebuilt_writer,
        &rebuilt_writer_root,
        vec![TaskValue::Text("失败🙂".to_owned())],
        &failed,
    )
    .expect("host-Fault reconstruction");
    assert_eq!(failed.canonical_bytes(), replayed_failure.canonical_bytes());
    assert_eq!(
        terminal(&replayed_failure),
        &TaskScheduleTerminal::Faulted { fault_count: 1 }
    );
    assert_eq!(closure_fault_count(&replayed_failure), 1);
    assert!(replayed_failure.events().iter().any(|event| matches!(
        event.kind(),
        TaskScheduleEventKind::Host { text, outcome }
            if text == "失败🙂\n"
                && outcome == &TaskHostOutcome::Failed(HostErrorCategory::BrokenPipe)
    )));

    let echo = checked_at(2_824, "cancel/echo.ling", ECHO.as_bytes().to_vec());
    let rebuilt_echo = checked_at(28_240, "different/cancel.ling", ECHO.as_bytes().to_vec());
    let echo_root = task(&echo, "echo");
    let rebuilt_echo_root = task(&rebuilt_echo, "echo");
    let cancelled = run_task_schedule(
        &echo,
        &echo_root,
        vec![integer(7)],
        config(0x0282_0002),
        vec![TaskDeadline::new(0, TaskPath::root())],
        TaskHostScript::default(),
    )
    .expect("bounded deadline-cancellation trace");
    let replayed_cancellation = replay_task_schedule(
        &rebuilt_echo,
        &rebuilt_echo_root,
        vec![integer(7)],
        &cancelled,
    )
    .expect("deadline-cancellation reconstruction");
    assert_eq!(
        cancelled.canonical_bytes(),
        replayed_cancellation.canonical_bytes()
    );
    assert_eq!(
        terminal(&replayed_cancellation),
        &TaskScheduleTerminal::Cancelled
    );
    assert_eq!(closure_fault_count(&replayed_cancellation), 0);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlayerConcern {
    CheckpointIdentity,
    ProgramCanonicalBytes,
    PreflightBinding,
    EventApplication,
    Ordering,
    Divergence,
    Fault,
    Cancellation,
    Privacy,
    Integrity,
    Migration,
}

impl PlayerConcern {
    const fn name(self) -> &'static str {
        match self {
            Self::CheckpointIdentity => "checkpoint-identity",
            Self::ProgramCanonicalBytes => "program-canonical-bytes",
            Self::PreflightBinding => "preflight-binding",
            Self::EventApplication => "event-application",
            Self::Ordering => "ordering",
            Self::Divergence => "divergence",
            Self::Fault => "fault",
            Self::Cancellation => "cancellation",
            Self::Privacy => "privacy",
            Self::Integrity => "integrity",
            Self::Migration => "migration",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConcernDisposition {
    ExistingPrivateTaskReplayEvidence,
    DeferredPublicContract,
}

const CONCERN_DISPOSITIONS: [(PlayerConcern, ConcernDisposition); 11] = [
    (
        PlayerConcern::CheckpointIdentity,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PlayerConcern::ProgramCanonicalBytes,
        ConcernDisposition::ExistingPrivateTaskReplayEvidence,
    ),
    (
        PlayerConcern::PreflightBinding,
        ConcernDisposition::ExistingPrivateTaskReplayEvidence,
    ),
    (
        PlayerConcern::EventApplication,
        ConcernDisposition::ExistingPrivateTaskReplayEvidence,
    ),
    (
        PlayerConcern::Ordering,
        ConcernDisposition::ExistingPrivateTaskReplayEvidence,
    ),
    (
        PlayerConcern::Divergence,
        ConcernDisposition::ExistingPrivateTaskReplayEvidence,
    ),
    (
        PlayerConcern::Fault,
        ConcernDisposition::ExistingPrivateTaskReplayEvidence,
    ),
    (
        PlayerConcern::Cancellation,
        ConcernDisposition::ExistingPrivateTaskReplayEvidence,
    ),
    (
        PlayerConcern::Privacy,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PlayerConcern::Integrity,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PlayerConcern::Migration,
        ConcernDisposition::DeferredPublicContract,
    ),
];

fn assert_concern_dispositions() {
    assert_eq!(CONCERN_DISPOSITIONS.len(), 11);
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .map(|(concern, _)| *concern)
            .collect::<BTreeSet<_>>()
            .len(),
        CONCERN_DISPOSITIONS.len()
    );
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .filter(|(_, disposition)| {
                *disposition == ConcernDisposition::ExistingPrivateTaskReplayEvidence
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
        4
    );
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .map(|(concern, _)| concern.name())
            .collect::<Vec<_>>(),
        [
            "checkpoint-identity",
            "program-canonical-bytes",
            "preflight-binding",
            "event-application",
            "ordering",
            "divergence",
            "fault",
            "cancellation",
            "privacy",
            "integrity",
            "migration",
        ]
    );
}

fn assert_public_player_surfaces_absent() {
    const EVAL_SOURCE: &str = include_str!("lib.rs");
    const SEMANTIC_SOURCE: &str = include_str!("../../ling-semantic/src/lib.rs");
    const PROJECT_SOURCE: &str = include_str!("../../ling-project/src/lib.rs");
    const BYTECODE_SOURCE: &str = include_str!("../../ling-bytecode/src/lib.rs");
    const VM_SOURCE: &str = include_str!("../../ling-vm/src/lib.rs");
    const CLI_CATALOG_SOURCE: &str = include_str!("../../ling-cli/src/command_catalog.rs");
    const DIAGNOSTIC_CODES: &str = include_str!("../../../docs/ERROR-CODES.md");
    const SCHEMA_REGISTRY: &str = include_str!("../../../schemas/registry.toml");
    const PROTOCOL_INVENTORY: &str =
        include_str!("../../../docs/governance/protocol-inventory.toml");

    for production_source in [
        EVAL_SOURCE,
        SEMANTIC_SOURCE,
        PROJECT_SOURCE,
        BYTECODE_SOURCE,
        VM_SOURCE,
    ] {
        for forbidden_surface in [
            "pub struct ReplayPlayer",
            "pub enum ReplayPlayer",
            "pub struct ReplayCheckpoint",
            "pub struct ReplayLogReader",
            "pub fn decode_replay",
            "pub fn restore_checkpoint",
            "pub fn seek_replay",
            "pub fn load_replay",
            "replay_reader:",
            "checkpoint_restore:",
            "replay_integrity:",
            "replay_migration:",
        ] {
            assert!(
                !production_source.contains(forbidden_surface),
                "forbidden DEC-0282 production surface: {forbidden_surface}"
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
fn deferred_checkpoint_and_public_surface_absence() {
    assert_registered(
        EvidenceCase::DeferredCheckpointAndPublicSurfaceAbsence,
        "deferred-checkpoint-and-public-surface-absence",
    );
    assert_concern_dispositions();
    assert_public_player_surfaces_absent();
}
