//! REP-2503 private checked Effect-boundary evidence authorized by DEC-0281.
//!
//! This module observes only bounded `Console.Write.write` calls that escape
//! accepted lexical Handler dispatch and reach an injected test adapter. It
//! does not define a production recorder, Effect Log, Replay payload, privacy
//! policy, or public compatibility contract.

use std::collections::{BTreeSet, VecDeque};

use ling_ast::lower as lower_ast;
use ling_diagnostics::codes;
use ling_effects::locate_main;
use ling_hir::lower as lower_hir;
use ling_resolve::resolve;
use ling_semantic::{ProgramSnapshot, build};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;

use crate::{Console, HostError, HostErrorCategory, RuntimeFault, RuntimeFaultKind, execute_main};

const OPERATION: &str = "Console.Write.write";
const MAX_OBSERVATIONS: usize = 8;
const MAX_HOST_TEXT_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceCase {
    EscapedSuccessOrder,
    HandledElisionAndClauseEscape,
    FailureStopAndFaultSidecar,
    CheckedReconstructionAndSourceIndependence,
    DeferredBoundariesAndPublicSurfaceAbsence,
}

const EVIDENCE_CASES: [(EvidenceCase, &str); 5] = [
    (EvidenceCase::EscapedSuccessOrder, "escaped-success-order"),
    (
        EvidenceCase::HandledElisionAndClauseEscape,
        "handled-elision-and-clause-escape",
    ),
    (
        EvidenceCase::FailureStopAndFaultSidecar,
        "failure-stop-and-fault-sidecar",
    ),
    (
        EvidenceCase::CheckedReconstructionAndSourceIndependence,
        "checked-reconstruction-and-source-independence",
    ),
    (
        EvidenceCase::DeferredBoundariesAndPublicSurfaceAbsence,
        "deferred-boundaries-and-public-surface-absence",
    ),
];

fn assert_registered(case: EvidenceCase, expected_name: &str) {
    assert_eq!(EVIDENCE_CASES.len(), 5);
    let matches = EVIDENCE_CASES
        .iter()
        .filter(|(candidate, _)| *candidate == case)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "each DEC-0281 case occurs exactly once");
    assert_eq!(matches[0].1, expected_name);

    let names = EVIDENCE_CASES
        .iter()
        .map(|(_, name)| *name)
        .collect::<BTreeSet<_>>();
    assert_eq!(names.len(), EVIDENCE_CASES.len());
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PrivateOutcome {
    SucceededUnit,
    Failed(HostErrorCategory),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrivateObservation {
    ordinal: u64,
    operation: &'static str,
    host_text: String,
    outcome: PrivateOutcome,
}

struct ScriptedRecordingConsole {
    responses: VecDeque<Result<(), HostErrorCategory>>,
    observations: Vec<PrivateObservation>,
}

impl ScriptedRecordingConsole {
    fn new(responses: impl IntoIterator<Item = Result<(), HostErrorCategory>>) -> Self {
        let responses = responses.into_iter().collect::<VecDeque<_>>();
        assert!(responses.len() <= MAX_OBSERVATIONS);
        Self {
            responses,
            observations: Vec::new(),
        }
    }

    fn observations(&self) -> &[PrivateObservation] {
        &self.observations
    }

    fn remaining_responses(&self) -> usize {
        self.responses.len()
    }
}

impl Console for ScriptedRecordingConsole {
    fn write(&mut self, text: &str) -> Result<(), HostError> {
        assert!(text.len() <= MAX_HOST_TEXT_BYTES);
        assert!(self.observations.len() < MAX_OBSERVATIONS);
        let response = self
            .responses
            .pop_front()
            .expect("bounded fixture must declare every host response");
        let outcome = match response {
            Ok(()) => PrivateOutcome::SucceededUnit,
            Err(category) => PrivateOutcome::Failed(category),
        };
        let ordinal =
            u64::try_from(self.observations.len() + 1).expect("bounded observation count fits u64");
        self.observations.push(PrivateObservation {
            ordinal,
            operation: OPERATION,
            host_text: text.to_owned(),
            outcome,
        });
        response.map_err(HostError::new)
    }
}

fn snapshot_at(source_id: u32, source_name: &str, bytes: Vec<u8>) -> ProgramSnapshot {
    let source = SourceFile::from_bytes(SourceId::new(source_id), source_name, bytes)
        .expect("valid UTF-8 source");
    let parsed = parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = lower_ast(&source, &parsed).expect("valid AST");
    let hir = lower_hir(source.name(), &ast).expect("valid HIR");
    let resolved = resolve(vec![hir], "Main").expect("resolved program");
    let typed = check_types(resolved).expect("typed program");
    let checked = ling_effects::check(typed).expect("checked Effect/Capability program");
    build(checked).expect("semantic snapshot")
}

fn execute(
    snapshot: &ProgramSnapshot,
    responses: impl IntoIterator<Item = Result<(), HostErrorCategory>>,
) -> (Result<(), RuntimeFault>, ScriptedRecordingConsole) {
    let main = locate_main(snapshot.checked()).expect("checked Main.main");
    let mut console = ScriptedRecordingConsole::new(responses);
    let result = execute_main(snapshot, &main, &mut console);
    for (index, observation) in console.observations().iter().enumerate() {
        assert_eq!(observation.ordinal, index as u64 + 1);
        assert_eq!(observation.operation, OPERATION);
        assert!(observation.host_text.ends_with('\n'));
        assert!(!observation.host_text.contains('\r'));
    }
    (result, console)
}

fn successful(ordinal: u64, text: &str) -> PrivateObservation {
    PrivateObservation {
        ordinal,
        operation: OPERATION,
        host_text: text.to_owned(),
        outcome: PrivateOutcome::SucceededUnit,
    }
}

#[test]
fn escaped_success_order() {
    assert_registered(EvidenceCase::EscapedSuccessOrder, "escaped-success-order");
    const SOURCE: &str = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let pure value = value + 1\n\n",
        "let main () =\n",
        "    Console.write \"first\"\n",
        "    let ignored = pure 1\n",
        "    Console.write \"第二\"\n",
    );
    let snapshot = snapshot_at(281, "evidence/success.ling", SOURCE.as_bytes().to_vec());
    let (result, console) = execute(&snapshot, [Ok(()), Ok(())]);
    result.expect("escaped host operations succeed");
    assert_eq!(console.remaining_responses(), 0);
    assert_eq!(
        console.observations(),
        &[successful(1, "first\n"), successful(2, "第二\n")]
    );
}

fn assert_no_host_observation(source_id: u32, name: &str, source: &str) {
    let snapshot = snapshot_at(source_id, name, source.as_bytes().to_vec());
    let (result, console) = execute(&snapshot, []);
    result.expect("accepted Handler path executes");
    assert!(console.observations().is_empty());
    assert_eq!(console.remaining_responses(), 0);
}

#[test]
fn handled_elision_and_clause_escape() {
    assert_registered(
        EvidenceCase::HandledElisionAndClauseEscape,
        "handled-elision-and-clause-escape",
    );

    assert_no_host_observation(
        282,
        "evidence/handler-direct.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"direct\" with\n",
            "        operation Console.Write.write(message, resume) -> ()\n",
        ),
    );
    assert_no_host_observation(
        283,
        "evidence/handler-transitive.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let emit () = Console.write \"transitive\"\n\n",
            "let main () =\n",
            "    handle emit () with\n",
            "        operation Console.Write.write(message, resume) -> ()\n",
        ),
    );
    assert_no_host_observation(
        284,
        "evidence/handler-resumed.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let emitBoth () =\n",
            "    Console.write \"first\"\n",
            "    Console.write \"second\"\n\n",
            "let main () =\n",
            "    handle emitBoth () with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            if message == \"first\" then resume () else ()\n",
        ),
    );
    assert_no_host_observation(
        285,
        "evidence/handler-nested.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let inner () =\n",
            "    handle Console.write \"body\" with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            Console.write \"inner clause\"\n\n",
            "let main () =\n",
            "    handle inner () with\n",
            "        operation Console.Write.write(message, resume) -> ()\n",
        ),
    );

    let clause_escape = snapshot_at(
        286,
        "evidence/handler-clause-escape.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"hidden\" with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            Console.write \"escaped\"\n",
        )
        .as_bytes()
        .to_vec(),
    );
    let (result, console) = execute(&clause_escape, [Ok(())]);
    result.expect("clause Effect escapes the selected Handler");
    assert_eq!(console.remaining_responses(), 0);
    assert_eq!(console.observations(), &[successful(1, "escaped\n")]);
}

#[test]
fn failure_stop_and_fault_sidecar() {
    assert_registered(
        EvidenceCase::FailureStopAndFaultSidecar,
        "failure-stop-and-fault-sidecar",
    );
    const SOURCE: &str = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    Console.write \"first\"\n",
        "    Console.write \"fail\"\n",
        "    Console.write \"after\"\n",
    );
    let snapshot = snapshot_at(287, "evidence/failure.ling", SOURCE.as_bytes().to_vec());
    let (result, console) = execute(
        &snapshot,
        [Ok(()), Err(HostErrorCategory::BrokenPipe), Ok(())],
    );
    let fault = result.expect_err("scripted host failure stops execution");
    assert_eq!(console.remaining_responses(), 1);
    assert_eq!(
        console.observations(),
        &[
            successful(1, "first\n"),
            PrivateObservation {
                ordinal: 2,
                operation: OPERATION,
                host_text: "fail\n".to_owned(),
                outcome: PrivateOutcome::Failed(HostErrorCategory::BrokenPipe),
            },
        ]
    );
    assert!(matches!(
        fault.kind,
        RuntimeFaultKind::HostCapability {
            operation: "Console.write",
            category: HostErrorCategory::BrokenPipe,
        }
    ));
    let failing_start = SOURCE
        .find("Console.write \"fail\"")
        .expect("failing operation exists") as u32;
    assert_eq!(fault.source_name, "evidence/failure.ling");
    assert_eq!(fault.span.start().get(), failing_start);
    assert!(fault.span.end().get() > failing_start);
    assert_eq!(fault.to_diagnostic().code(), codes::RUNTIME_FAULT);
}

#[test]
fn checked_reconstruction_and_source_independence() {
    assert_registered(
        EvidenceCase::CheckedReconstructionAndSourceIndependence,
        "checked-reconstruction-and-source-independence",
    );
    const LF: &str = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () = Console.write \"你好🙂\"\n",
    );
    const BOM_CRLF: &str = concat!(
        "\u{feff}module Main\r\n",
        "    requires Console.Write\r\n\r\n",
        "let main () = Console.write \"你好🙂\"\r\n",
    );
    let left = snapshot_at(288, "left/源.ling", LF.as_bytes().to_vec());
    let right = snapshot_at(2_880, "different/right.ling", BOM_CRLF.as_bytes().to_vec());
    let (left_result, left_console) = execute(&left, [Err(HostErrorCategory::Interrupted)]);
    let (right_result, right_console) = execute(&right, [Err(HostErrorCategory::Interrupted)]);
    let left_fault = left_result.expect_err("left host operation fails");
    let right_fault = right_result.expect_err("right host operation fails");

    assert_eq!(left_console.observations(), right_console.observations());
    assert_eq!(left_console.remaining_responses(), 0);
    assert_eq!(right_console.remaining_responses(), 0);
    assert_eq!(left_console.observations().len(), 1);
    assert_eq!(left_console.observations()[0].host_text, "你好🙂\n");
    assert_eq!(left_fault.source_name, "left/源.ling");
    assert_eq!(right_fault.source_name, "different/right.ling");
    assert_eq!(
        left_fault.span.start().get() as usize,
        LF.rfind("Console.write").expect("left operation")
    );
    assert_eq!(
        right_fault.span.start().get() as usize,
        BOM_CRLF.rfind("Console.write").expect("right operation")
    );
    assert_ne!(left_fault.span, right_fault.span);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum DeferredBoundary {
    Clock,
    Random,
    ExternalInput,
    NetworkReceive,
    FileDeviceRead,
    SchedulingNondeterminism,
}

impl DeferredBoundary {
    const fn name(self) -> &'static str {
        match self {
            Self::Clock => "Clock",
            Self::Random => "Random",
            Self::ExternalInput => "ExternalInput",
            Self::NetworkReceive => "NetworkReceive",
            Self::FileDeviceRead => "FileDeviceRead",
            Self::SchedulingNondeterminism => "SchedulingNondeterminism",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeferredDisposition {
    CheckedContractWithoutProducer,
    PlanOnlyWithoutOperation,
    SeparatePrivateSchedulingEvidence,
}

const DEFERRED_BOUNDARIES: [(DeferredBoundary, DeferredDisposition); 6] = [
    (
        DeferredBoundary::Clock,
        DeferredDisposition::CheckedContractWithoutProducer,
    ),
    (
        DeferredBoundary::Random,
        DeferredDisposition::CheckedContractWithoutProducer,
    ),
    (
        DeferredBoundary::ExternalInput,
        DeferredDisposition::PlanOnlyWithoutOperation,
    ),
    (
        DeferredBoundary::NetworkReceive,
        DeferredDisposition::PlanOnlyWithoutOperation,
    ),
    (
        DeferredBoundary::FileDeviceRead,
        DeferredDisposition::PlanOnlyWithoutOperation,
    ),
    (
        DeferredBoundary::SchedulingNondeterminism,
        DeferredDisposition::SeparatePrivateSchedulingEvidence,
    ),
];

fn assert_deferred_boundary_inventory() {
    assert_eq!(DEFERRED_BOUNDARIES.len(), 6);
    let boundaries = DEFERRED_BOUNDARIES
        .iter()
        .map(|(boundary, _)| *boundary)
        .collect::<BTreeSet<_>>();
    assert_eq!(boundaries.len(), DEFERRED_BOUNDARIES.len());
    assert_eq!(
        DEFERRED_BOUNDARIES
            .iter()
            .map(|(boundary, _)| boundary.name())
            .collect::<Vec<_>>(),
        [
            "Clock",
            "Random",
            "ExternalInput",
            "NetworkReceive",
            "FileDeviceRead",
            "SchedulingNondeterminism",
        ]
    );
    assert_eq!(
        DEFERRED_BOUNDARIES
            .iter()
            .filter(|(_, disposition)| {
                *disposition == DeferredDisposition::CheckedContractWithoutProducer
            })
            .count(),
        2
    );
    assert_eq!(
        DEFERRED_BOUNDARIES
            .iter()
            .filter(|(_, disposition)| {
                *disposition == DeferredDisposition::PlanOnlyWithoutOperation
            })
            .count(),
        3
    );
    assert_eq!(
        DEFERRED_BOUNDARIES
            .iter()
            .filter(|(_, disposition)| {
                *disposition == DeferredDisposition::SeparatePrivateSchedulingEvidence
            })
            .count(),
        1
    );
    assert!(
        !DEFERRED_BOUNDARIES
            .iter()
            .any(|(boundary, _)| boundary.name() == OPERATION)
    );
}

fn assert_public_recording_surfaces_absent() {
    const EVAL_SOURCE: &str = include_str!("lib.rs");
    const EFFECT_SOURCE: &str = include_str!("../../ling-effects/src/lib.rs");
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
        EFFECT_SOURCE,
        SEMANTIC_SOURCE,
        PROJECT_SOURCE,
        BYTECODE_SOURCE,
        VM_SOURCE,
    ] {
        for forbidden_surface in [
            "pub trait EffectRecorder",
            "pub struct EffectRecorder",
            "pub enum EffectRecorder",
            "pub struct EffectRecord",
            "pub enum EffectRecord",
            "pub struct EffectLog",
            "pub enum EffectLog",
            "pub fn record_effect",
            "effect_recorder:",
            "recorder_hook:",
            "replay_payload:",
            "replay_redaction:",
        ] {
            assert!(
                !production_source.contains(forbidden_surface),
                "forbidden DEC-0281 production surface: {forbidden_surface}"
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
fn deferred_boundaries_and_public_surface_absence() {
    assert_registered(
        EvidenceCase::DeferredBoundariesAndPublicSurfaceAbsence,
        "deferred-boundaries-and-public-surface-absence",
    );
    assert_deferred_boundary_inventory();
    assert_public_recording_surfaces_absent();
}
