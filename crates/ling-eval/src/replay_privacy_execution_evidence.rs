//! REP-2505 private Task replay privacy/corruption evidence under DEC-0283.
//!
//! This module demonstrates that the existing in-memory Task trace retains raw
//! fixture payloads, rejects truncation and mutation, and can be reconstructed
//! from explicit bounded inputs. It does not classify or redact data, trim a
//! log, frame chunks, calculate checksums, decode external bytes, manage keys
//! or retention, or define a public offline Replay tool.

use std::collections::BTreeSet;

use ling_ast::lower as lower_ast;
use ling_effects::{CheckedProgram, check};
use ling_hir::lower as lower_hir;
use ling_resolve::{DefinitionId, resolve};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;

use crate::{
    TaskDeadline, TaskHostOutcome, TaskHostResponse, TaskHostScript, TaskPath, TaskRuntimeLimits,
    TaskScheduleConfig, TaskScheduleEventKind, TaskScheduleTrace, TaskSchedulerLimits, TaskValue,
    replay_task_schedule, run_task_schedule,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceCase {
    RawPayloadRetentionBoundary,
    PrefixTruncationRefusal,
    ValidatedMutationRefusal,
    ExplicitInputOfflineReconstruction,
    DeferredPrivacyIntegritySurfaceAbsence,
}

const EVIDENCE_CASES: [(EvidenceCase, &str); 5] = [
    (
        EvidenceCase::RawPayloadRetentionBoundary,
        "raw-payload-retention-boundary",
    ),
    (
        EvidenceCase::PrefixTruncationRefusal,
        "prefix-truncation-refusal",
    ),
    (
        EvidenceCase::ValidatedMutationRefusal,
        "validated-mutation-refusal",
    ),
    (
        EvidenceCase::ExplicitInputOfflineReconstruction,
        "explicit-input-offline-reconstruction",
    ),
    (
        EvidenceCase::DeferredPrivacyIntegritySurfaceAbsence,
        "deferred-privacy-integrity-surface-absence",
    ),
];

fn assert_registered(case: EvidenceCase, expected_name: &str) {
    assert_eq!(EVIDENCE_CASES.len(), 5);
    let matches = EVIDENCE_CASES
        .iter()
        .filter(|(candidate, _)| *candidate == case)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "each DEC-0283 case occurs exactly once");
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

fn scheduler_limits() -> TaskSchedulerLimits {
    TaskSchedulerLimits::new(128, 128, 16, 512, 2_048, 128, 16)
}

fn config() -> TaskScheduleConfig {
    TaskScheduleConfig::new(
        0x0283_0001,
        TaskRuntimeLimits::new(32, 32, 256, 16),
        scheduler_limits(),
    )
}

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

const RAW_FIXTURE_SENTINEL: &str = "fixture-private-boundary-敏感🙂";

fn writer_trace(checked: &CheckedProgram, root: &DefinitionId) -> TaskScheduleTrace {
    run_task_schedule(
        checked,
        root,
        vec![TaskValue::Text(RAW_FIXTURE_SENTINEL.to_owned())],
        config(),
        vec![TaskDeadline::new(64, TaskPath::root())],
        TaskHostScript::new([TaskHostResponse::Complete]),
    )
    .expect("bounded writer trace")
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

#[test]
fn raw_payload_retention_boundary() {
    assert_registered(
        EvidenceCase::RawPayloadRetentionBoundary,
        "raw-payload-retention-boundary",
    );
    let checked = checked_at(
        2_830,
        "privacy/raw-fixture.ling",
        WRITER_LF.as_bytes().to_vec(),
    );
    let root = task(&checked, "writer");
    let trace = writer_trace(&checked, &root);
    trace.validate().expect("validated private trace");

    assert!(trace.events().iter().any(|event| matches!(
        event.kind(),
        TaskScheduleEventKind::Host { text, outcome }
            if text == &format!("{RAW_FIXTURE_SENTINEL}\n")
                && outcome == &TaskHostOutcome::Completed
    )));
    assert!(contains_bytes(
        &trace.canonical_bytes(),
        RAW_FIXTURE_SENTINEL.as_bytes()
    ));
    assert!(!contains_bytes(
        &trace.canonical_bytes(),
        b"privacy/raw-fixture.ling"
    ));
}

#[test]
fn prefix_truncation_refusal() {
    assert_registered(
        EvidenceCase::PrefixTruncationRefusal,
        "prefix-truncation-refusal",
    );
    crate::task_scheduler::tests::trace_validation_rejects_truncated_and_gapped_event_sequences();
}

#[test]
fn validated_mutation_refusal() {
    assert_registered(
        EvidenceCase::ValidatedMutationRefusal,
        "validated-mutation-refusal",
    );
    crate::task_scheduler::tests::trace_validation_rejects_version_event_and_closure_corruption();
    crate::task_scheduler::tests::replay_reports_the_first_mutated_selection_field();
    crate::task_scheduler::tests::replay_reports_deadline_host_and_terminal_mutations_at_their_event();
}

#[test]
fn explicit_input_offline_reconstruction() {
    assert_registered(
        EvidenceCase::ExplicitInputOfflineReconstruction,
        "explicit-input-offline-reconstruction",
    );
    let original = checked_at(
        2_831,
        "offline/original.ling",
        WRITER_LF.as_bytes().to_vec(),
    );
    let rebuilt = checked_at(
        28_310,
        "different/offline-rebuilt.ling",
        WRITER_BOM_CRLF.as_bytes().to_vec(),
    );
    let original_root = task(&original, "writer");
    let rebuilt_root = task(&rebuilt, "writer");
    let expected = writer_trace(&original, &original_root);
    let replayed = replay_task_schedule(
        &rebuilt,
        &rebuilt_root,
        vec![TaskValue::Text(RAW_FIXTURE_SENTINEL.to_owned())],
        &expected,
    )
    .expect("fresh-runtime replay from explicit inputs");

    assert_eq!(expected.canonical_bytes(), replayed.canonical_bytes());
    assert!(expected.events().len() <= config().scheduler_limits().max_trace_events());
    assert_eq!(expected.deadlines().len(), 1);
    assert_eq!(expected.host_script().responses().len(), 1);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PrivacyConcern {
    FieldSensitivity,
    FieldRedaction,
    SecretPiiExclusion,
    CapabilityResourceExclusion,
    Authorization,
    KeyHandling,
    Retention,
    DependencyClosure,
    ChunkBoundary,
    ChecksumIntegrity,
    Truncation,
    Corruption,
    FailureDiagnostics,
    UnknownField,
    OfflineMode,
    Migration,
}

impl PrivacyConcern {
    const fn name(self) -> &'static str {
        match self {
            Self::FieldSensitivity => "field-sensitivity",
            Self::FieldRedaction => "field-redaction",
            Self::SecretPiiExclusion => "secret-pii-exclusion",
            Self::CapabilityResourceExclusion => "capability-resource-exclusion",
            Self::Authorization => "authorization",
            Self::KeyHandling => "key-handling",
            Self::Retention => "retention",
            Self::DependencyClosure => "dependency-closure",
            Self::ChunkBoundary => "chunk-boundary",
            Self::ChecksumIntegrity => "checksum-integrity",
            Self::Truncation => "truncation",
            Self::Corruption => "corruption",
            Self::FailureDiagnostics => "failure-diagnostics",
            Self::UnknownField => "unknown-field",
            Self::OfflineMode => "offline-mode",
            Self::Migration => "migration",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConcernDisposition {
    RawPayloadRiskEvidence,
    PrivateFailClosedEvidence,
    ExplicitInputHermeticEvidence,
    DeferredPublicContract,
}

const CONCERN_DISPOSITIONS: [(PrivacyConcern, ConcernDisposition); 16] = [
    (
        PrivacyConcern::FieldSensitivity,
        ConcernDisposition::RawPayloadRiskEvidence,
    ),
    (
        PrivacyConcern::FieldRedaction,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::SecretPiiExclusion,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::CapabilityResourceExclusion,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::Authorization,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::KeyHandling,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::Retention,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::DependencyClosure,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::ChunkBoundary,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::ChecksumIntegrity,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::Truncation,
        ConcernDisposition::PrivateFailClosedEvidence,
    ),
    (
        PrivacyConcern::Corruption,
        ConcernDisposition::PrivateFailClosedEvidence,
    ),
    (
        PrivacyConcern::FailureDiagnostics,
        ConcernDisposition::PrivateFailClosedEvidence,
    ),
    (
        PrivacyConcern::UnknownField,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        PrivacyConcern::OfflineMode,
        ConcernDisposition::ExplicitInputHermeticEvidence,
    ),
    (
        PrivacyConcern::Migration,
        ConcernDisposition::DeferredPublicContract,
    ),
];

fn assert_concern_dispositions() {
    assert_eq!(CONCERN_DISPOSITIONS.len(), 16);
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
                *disposition == ConcernDisposition::DeferredPublicContract
            })
            .count(),
        11
    );
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .map(|(concern, _)| concern.name())
            .collect::<Vec<_>>(),
        [
            "field-sensitivity",
            "field-redaction",
            "secret-pii-exclusion",
            "capability-resource-exclusion",
            "authorization",
            "key-handling",
            "retention",
            "dependency-closure",
            "chunk-boundary",
            "checksum-integrity",
            "truncation",
            "corruption",
            "failure-diagnostics",
            "unknown-field",
            "offline-mode",
            "migration",
        ]
    );
}

fn assert_public_privacy_surfaces_absent() {
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
            "pub struct ReplayPrivacyPolicy",
            "pub enum ReplaySensitivity",
            "pub struct ReplayRedactor",
            "pub struct ReplayTrimmer",
            "pub struct ReplayChunk",
            "pub struct ReplayChecksum",
            "pub struct ReplayRetention",
            "pub fn redact_replay",
            "pub fn trim_replay",
            "pub fn decode_replay",
            "pub fn verify_replay_checksum",
            "replay_redaction:",
            "replay_retention:",
            "replay_integrity:",
        ] {
            assert!(
                !production_source.contains(forbidden_surface),
                "forbidden DEC-0283 production surface: {forbidden_surface}"
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
fn deferred_privacy_integrity_surface_absence() {
    assert_registered(
        EvidenceCase::DeferredPrivacyIntegritySurfaceAbsence,
        "deferred-privacy-integrity-surface-absence",
    );
    assert_concern_dispositions();
    assert_public_privacy_surfaces_absent();
}
