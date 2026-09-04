//! REP-2506 private cross-process reconstruction evidence under DEC-0284.
//!
//! Parent tests start fresh copies of the current unit-test executable with an
//! empty inherited environment. Ignored child probes rebuild one checked Task
//! trace from fixed in-memory inputs and emit its exact private canonical bytes.
//! This is not a persisted log, Replay reader/player, public process protocol,
//! cache/toolchain certification, or cross-platform compatibility guarantee.

use std::collections::BTreeSet;
use std::process::Command;

use ling_ast::lower as lower_ast;
use ling_effects::{CheckedProgram, check};
use ling_hir::lower as lower_hir;
use ling_resolve::{DefinitionId, resolve};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;
use num_bigint::BigInt;

use crate::{
    TaskHostScript, TaskRuntimeLimits, TaskScheduleConfig, TaskSchedulerLimits, TaskValue,
    run_task_schedule,
};

const REPEAT_COUNT: usize = 3;
const TRACE_MARKER: &str = "LING_REP_2506_TRACE=";
const ENV_MARKER: &str = "LING_REP_2506_ENV_COUNT=";
const LF_PROBE: &str = "replay_cross_process_execution_evidence::child_lf_probe";
const BOM_CRLF_PROBE: &str = "replay_cross_process_execution_evidence::child_bom_crlf_probe";
const CHANGED_BODY_PROBE: &str =
    "replay_cross_process_execution_evidence::child_changed_body_probe";
const CHANGED_ARGUMENT_PROBE: &str =
    "replay_cross_process_execution_evidence::child_changed_argument_probe";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceCase {
    IndependentProcessRepeatability,
    SourceIndependentProcessEquivalence,
    ChangedRecipeProcessDistinction,
    EmptyEnvironmentBoundedProcess,
    DeferredCrossProcessPublicSurfaceAbsence,
}

const EVIDENCE_CASES: [(EvidenceCase, &str); 5] = [
    (
        EvidenceCase::IndependentProcessRepeatability,
        "independent-process-repeatability",
    ),
    (
        EvidenceCase::SourceIndependentProcessEquivalence,
        "source-independent-process-equivalence",
    ),
    (
        EvidenceCase::ChangedRecipeProcessDistinction,
        "changed-recipe-process-distinction",
    ),
    (
        EvidenceCase::EmptyEnvironmentBoundedProcess,
        "empty-environment-bounded-process",
    ),
    (
        EvidenceCase::DeferredCrossProcessPublicSurfaceAbsence,
        "deferred-cross-process-public-surface-absence",
    ),
];

fn assert_registered(case: EvidenceCase, expected_name: &str) {
    assert_eq!(EVIDENCE_CASES.len(), 5);
    let matches = EVIDENCE_CASES
        .iter()
        .filter(|(candidate, _)| *candidate == case)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "each DEC-0284 case occurs exactly once");
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

fn checked_at(source_id: u32, source_name: &str, source_text: &str) -> CheckedProgram {
    let source = SourceFile::from_bytes(
        SourceId::new(source_id),
        source_name,
        source_text.as_bytes().to_vec(),
    )
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

fn config() -> TaskScheduleConfig {
    TaskScheduleConfig::new(
        0x0284_0001,
        TaskRuntimeLimits::new(32, 32, 256, 16),
        TaskSchedulerLimits::new(128, 128, 16, 512, 2_048, 128, 16),
    )
}

const PROGRAM_LF: &str = concat!(
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

const PROGRAM_BOM_CRLF: &str = concat!(
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

const CHANGED_BODY: &str = concat!(
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

fn trace_bytes(source_id: u32, source_name: &str, source_text: &str, argument: i64) -> Vec<u8> {
    let checked = checked_at(source_id, source_name, source_text);
    let root = task(&checked, "parent");
    let trace = run_task_schedule(
        &checked,
        &root,
        vec![TaskValue::Int(BigInt::from(argument))],
        config(),
        vec![],
        TaskHostScript::default(),
    )
    .expect("bounded private Task trace");
    trace.validate().expect("validated private Task trace");
    trace.canonical_bytes()
}

fn encode_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    output
}

fn emit_probe(source_id: u32, source_name: &str, source_text: &str, argument: i64) {
    println!("{ENV_MARKER}{}", std::env::vars_os().count());
    println!(
        "{TRACE_MARKER}{}",
        encode_hex(&trace_bytes(source_id, source_name, source_text, argument))
    );
}

#[test]
#[ignore = "private child-process probe invoked by the REP-2506 parent matrix"]
fn child_lf_probe() {
    emit_probe(2_840, "process/left.ling", PROGRAM_LF, 4);
}

#[test]
#[ignore = "private child-process probe invoked by the REP-2506 parent matrix"]
fn child_bom_crlf_probe() {
    emit_probe(28_400, "different/process-right.ling", PROGRAM_BOM_CRLF, 4);
}

#[test]
#[ignore = "private child-process probe invoked by the REP-2506 parent matrix"]
fn child_changed_body_probe() {
    emit_probe(2_841, "process/changed-body.ling", CHANGED_BODY, 4);
}

#[test]
#[ignore = "private child-process probe invoked by the REP-2506 parent matrix"]
fn child_changed_argument_probe() {
    emit_probe(2_842, "process/changed-argument.ling", PROGRAM_LF, 5);
}

#[derive(Debug, Eq, PartialEq)]
struct ProbeOutput {
    environment_count: usize,
    trace_hex: String,
}

fn marker_value<'a>(stdout: &'a str, marker: &str) -> &'a str {
    let matches = stdout
        .lines()
        .filter_map(|line| line.find(marker).map(|index| &line[index + marker.len()..]))
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "one {marker} line in child output");
    matches[0]
}

fn run_probe(test_name: &str) -> ProbeOutput {
    let executable = std::env::current_exe().expect("current unit-test executable");
    let output = Command::new(executable)
        .env_clear()
        .args([
            "--ignored",
            "--exact",
            test_name,
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .expect("start isolated child test process");
    assert!(
        output.status.success(),
        "child process failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "child process must keep stderr empty"
    );
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 child output");
    let environment_count = marker_value(&stdout, ENV_MARKER)
        .parse::<usize>()
        .expect("decimal environment count");
    let trace_hex = marker_value(&stdout, TRACE_MARKER).to_owned();
    assert!(!trace_hex.is_empty());
    assert!(trace_hex.bytes().all(|byte| byte.is_ascii_hexdigit()));
    ProbeOutput {
        environment_count,
        trace_hex,
    }
}

#[test]
fn independent_process_repeatability() {
    assert_registered(
        EvidenceCase::IndependentProcessRepeatability,
        "independent-process-repeatability",
    );
    let probes = (0..REPEAT_COUNT)
        .map(|_| run_probe(LF_PROBE))
        .collect::<Vec<_>>();
    assert_eq!(probes.len(), REPEAT_COUNT);
    assert!(probes.windows(2).all(|pair| pair[0] == pair[1]));
}

#[test]
fn source_independent_process_equivalence() {
    assert_registered(
        EvidenceCase::SourceIndependentProcessEquivalence,
        "source-independent-process-equivalence",
    );
    assert_eq!(run_probe(LF_PROBE), run_probe(BOM_CRLF_PROBE));
}

#[test]
fn changed_recipe_process_distinction() {
    assert_registered(
        EvidenceCase::ChangedRecipeProcessDistinction,
        "changed-recipe-process-distinction",
    );
    let baseline = run_probe(LF_PROBE);
    let changed_body = run_probe(CHANGED_BODY_PROBE);
    let changed_argument = run_probe(CHANGED_ARGUMENT_PROBE);
    assert_ne!(baseline.trace_hex, changed_body.trace_hex);
    assert_ne!(baseline.trace_hex, changed_argument.trace_hex);
    assert_ne!(changed_body.trace_hex, changed_argument.trace_hex);
}

#[test]
fn empty_environment_bounded_process() {
    assert_registered(
        EvidenceCase::EmptyEnvironmentBoundedProcess,
        "empty-environment-bounded-process",
    );
    let probe = run_probe(LF_PROBE);
    assert_eq!(probe.environment_count, 0);
    assert_eq!(REPEAT_COUNT, 3);
    assert!(config().scheduler_limits().max_trace_events() > 0);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CrossProcessConcern {
    ProcessIsolation,
    ToolchainIdentity,
    ProfileIdentity,
    TargetIdentity,
    CacheIsolation,
    InputSnapshot,
    LogGeneration,
    ReplayPlayback,
    ProgramBinding,
    SchemaBinding,
    MutationRejection,
    ObservableEquivalence,
    Repeatability,
    Divergence,
    Provenance,
    ResourceLimits,
    PlatformBoundary,
    OfflineMode,
}

impl CrossProcessConcern {
    const fn name(self) -> &'static str {
        match self {
            Self::ProcessIsolation => "process-isolation",
            Self::ToolchainIdentity => "toolchain-identity",
            Self::ProfileIdentity => "profile-identity",
            Self::TargetIdentity => "target-identity",
            Self::CacheIsolation => "cache-isolation",
            Self::InputSnapshot => "input-snapshot",
            Self::LogGeneration => "log-generation",
            Self::ReplayPlayback => "replay-playback",
            Self::ProgramBinding => "program-binding",
            Self::SchemaBinding => "schema-binding",
            Self::MutationRejection => "mutation-rejection",
            Self::ObservableEquivalence => "observable-equivalence",
            Self::Repeatability => "repeatability",
            Self::Divergence => "divergence",
            Self::Provenance => "provenance",
            Self::ResourceLimits => "resource-limits",
            Self::PlatformBoundary => "platform-boundary",
            Self::OfflineMode => "offline-mode",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConcernDisposition {
    SameBinaryChildEvidence,
    PrivateTraceComparisonEvidence,
    DeferredPublicContract,
}

const CONCERN_DISPOSITIONS: [(CrossProcessConcern, ConcernDisposition); 18] = [
    (
        CrossProcessConcern::ProcessIsolation,
        ConcernDisposition::SameBinaryChildEvidence,
    ),
    (
        CrossProcessConcern::ToolchainIdentity,
        ConcernDisposition::SameBinaryChildEvidence,
    ),
    (
        CrossProcessConcern::ProfileIdentity,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        CrossProcessConcern::TargetIdentity,
        ConcernDisposition::SameBinaryChildEvidence,
    ),
    (
        CrossProcessConcern::CacheIsolation,
        ConcernDisposition::PrivateTraceComparisonEvidence,
    ),
    (
        CrossProcessConcern::InputSnapshot,
        ConcernDisposition::PrivateTraceComparisonEvidence,
    ),
    (
        CrossProcessConcern::LogGeneration,
        ConcernDisposition::PrivateTraceComparisonEvidence,
    ),
    (
        CrossProcessConcern::ReplayPlayback,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        CrossProcessConcern::ProgramBinding,
        ConcernDisposition::PrivateTraceComparisonEvidence,
    ),
    (
        CrossProcessConcern::SchemaBinding,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        CrossProcessConcern::MutationRejection,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        CrossProcessConcern::ObservableEquivalence,
        ConcernDisposition::PrivateTraceComparisonEvidence,
    ),
    (
        CrossProcessConcern::Repeatability,
        ConcernDisposition::PrivateTraceComparisonEvidence,
    ),
    (
        CrossProcessConcern::Divergence,
        ConcernDisposition::PrivateTraceComparisonEvidence,
    ),
    (
        CrossProcessConcern::Provenance,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        CrossProcessConcern::ResourceLimits,
        ConcernDisposition::PrivateTraceComparisonEvidence,
    ),
    (
        CrossProcessConcern::PlatformBoundary,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        CrossProcessConcern::OfflineMode,
        ConcernDisposition::PrivateTraceComparisonEvidence,
    ),
];

fn assert_concern_dispositions() {
    assert_eq!(CONCERN_DISPOSITIONS.len(), 18);
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
        6
    );
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .map(|(concern, _)| concern.name())
            .collect::<Vec<_>>(),
        [
            "process-isolation",
            "toolchain-identity",
            "profile-identity",
            "target-identity",
            "cache-isolation",
            "input-snapshot",
            "log-generation",
            "replay-playback",
            "program-binding",
            "schema-binding",
            "mutation-rejection",
            "observable-equivalence",
            "repeatability",
            "divergence",
            "provenance",
            "resource-limits",
            "platform-boundary",
            "offline-mode",
        ]
    );
}

fn assert_public_cross_process_surfaces_absent() {
    const EVAL_SOURCE: &str = include_str!("lib.rs");
    const PROJECT_SOURCE: &str = include_str!("../../ling-project/src/lib.rs");
    const BYTECODE_SOURCE: &str = include_str!("../../ling-bytecode/src/lib.rs");
    const VM_SOURCE: &str = include_str!("../../ling-vm/src/lib.rs");
    const CLI_CATALOG_SOURCE: &str = include_str!("../../ling-cli/src/command_catalog.rs");
    const DIAGNOSTIC_CODES: &str = include_str!("../../../docs/ERROR-CODES.md");
    const SCHEMA_REGISTRY: &str = include_str!("../../../schemas/registry.toml");
    const PROTOCOL_INVENTORY: &str =
        include_str!("../../../docs/governance/protocol-inventory.toml");

    for production_source in [EVAL_SOURCE, PROJECT_SOURCE, BYTECODE_SOURCE, VM_SOURCE] {
        for forbidden_surface in [
            "pub struct ReplayProcessHarness",
            "pub struct ReplayAcceptance",
            "pub struct ReplayProcessResult",
            "pub struct ReplayEnvironment",
            "pub fn run_replay_process",
            "pub fn compare_replay_processes",
            "pub fn certify_replay",
            "cross_process_replay:",
            "replay_acceptance:",
        ] {
            assert!(
                !production_source.contains(forbidden_surface),
                "forbidden DEC-0284 production surface: {forbidden_surface}"
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
fn deferred_cross_process_public_surface_absence() {
    assert_registered(
        EvidenceCase::DeferredCrossProcessPublicSurfaceAbsence,
        "deferred-cross-process-public-surface-absence",
    );
    assert_concern_dispositions();
    assert_public_cross_process_surfaces_absent();
}
