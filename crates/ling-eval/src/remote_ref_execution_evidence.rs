//! REM-2601 private local/remote reference boundary evidence under DEC-0285.
//!
//! This test-only module exercises the accepted local Actor runtime and keeps
//! candidate remote coordinate parts nominally separate. It does not define a
//! RemoteRef, endpoint address, token format, network Effect, delivery result,
//! wire protocol, or local-to-remote conversion.

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
    ActorRuntime, ActorRuntimeId, ActorRuntimeLimits, ActorSendErrorKind, ActorSenderId,
    ActorShutdownReason, ActorValue, LocalTaskControl, MemoryConsole,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum EvidenceCase {
    LocalReferenceRuntimeScope,
    SourceIndependentLocalTypeEvidence,
    CandidateRemoteDimensionSeparation,
    LocalToRemoteConversionAbsence,
    DeferredPublicRemoteSurfaceAbsence,
}

const EVIDENCE_CASES: [(EvidenceCase, &str); 5] = [
    (
        EvidenceCase::LocalReferenceRuntimeScope,
        "local-reference-runtime-scope",
    ),
    (
        EvidenceCase::SourceIndependentLocalTypeEvidence,
        "source-independent-local-type-evidence",
    ),
    (
        EvidenceCase::CandidateRemoteDimensionSeparation,
        "candidate-remote-dimension-separation",
    ),
    (
        EvidenceCase::LocalToRemoteConversionAbsence,
        "local-to-remote-conversion-absence",
    ),
    (
        EvidenceCase::DeferredPublicRemoteSurfaceAbsence,
        "deferred-public-remote-surface-absence",
    ),
];

fn assert_registered(case: EvidenceCase, expected_name: &str) {
    assert_eq!(EVIDENCE_CASES.len(), 5);
    let matches = EVIDENCE_CASES
        .iter()
        .filter(|(candidate, _)| *candidate == case)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "each DEC-0285 case occurs exactly once");
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

const COUNTER_LF: &str = concat!(
    "module Main\n\n",
    "actor Counter : Int =\n",
    "    mailbox capacity 2 overflow Reject\n",
    "    state Int = 0\n",
    "    receive state message =\n",
    "        state + message\n\n",
    "let main () = ()\n",
);

const COUNTER_BOM_CRLF: &str = concat!(
    "\u{feff}module Main\r\n\r\n",
    "actor Counter : Int =\r\n",
    "    mailbox capacity 2 overflow Reject\r\n",
    "    state Int = 0\r\n",
    "    receive state message =\r\n",
    "        state + message\r\n\r\n",
    "let main () = ()\r\n",
);

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
        .expect("checked Actor definition")
}

fn limits() -> ActorRuntimeLimits {
    ActorRuntimeLimits::new(4, 4, 8, 32, 32, 32, 4, 4)
}

fn integer(value: i64) -> ActorValue {
    ActorValue::Int(BigInt::from(value))
}

#[test]
fn local_reference_runtime_scope() {
    assert_registered(
        EvidenceCase::LocalReferenceRuntimeScope,
        "local-reference-runtime-scope",
    );
    let checked = checked_at(2_850, "remote/local-scope.ling", COUNTER_LF);
    let counter = actor(&checked, "Counter");
    let first_control = LocalTaskControl::new();
    let second_control = LocalTaskControl::new();
    let mut first = ActorRuntime::new(
        &checked,
        ActorRuntimeId::new(2_850),
        limits(),
        &first_control,
    )
    .expect("first local Actor runtime");
    let mut second = ActorRuntime::new(
        &checked,
        ActorRuntimeId::new(28_500),
        limits(),
        &second_control,
    )
    .expect("second local Actor runtime");
    let mut console = MemoryConsole::default();
    let local = first
        .spawn(&counter, &mut console)
        .expect("local Actor starts");

    assert_eq!(local.runtime(), ActorRuntimeId::new(2_850));
    assert_eq!(local.actor().get(), 1);
    let rejected = second
        .send(&local, ActorSenderId::new(1), integer(7))
        .expect_err("a local reference cannot cross runtime scope");
    assert_eq!(rejected.kind(), &ActorSendErrorKind::WrongRuntime);
    assert_eq!(rejected.into_payload(), integer(7));
    assert_eq!(second.metrics().commands(), 0);
    assert_eq!(second.metrics().queued_messages(), 0);

    first
        .shutdown(ActorShutdownReason::OwnerCompleted)
        .expect("first runtime shutdown");
    second
        .shutdown(ActorShutdownReason::OwnerCompleted)
        .expect("second runtime shutdown");
}

#[test]
fn source_independent_local_type_evidence() {
    assert_registered(
        EvidenceCase::SourceIndependentLocalTypeEvidence,
        "source-independent-local-type-evidence",
    );
    let left = checked_at(2_851, "remote/left.ling", COUNTER_LF);
    let right = checked_at(28_510, "different/right.ling", COUNTER_BOM_CRLF);
    let left_counter = actor(&left, "Counter");
    let right_counter = actor(&right, "Counter");
    let left_control = LocalTaskControl::new();
    let right_control = LocalTaskControl::new();
    let mut left_runtime =
        ActorRuntime::new(&left, ActorRuntimeId::new(2_851), limits(), &left_control)
            .expect("left local Actor runtime");
    let mut right_runtime =
        ActorRuntime::new(&right, ActorRuntimeId::new(2_852), limits(), &right_control)
            .expect("right local Actor runtime");
    let mut console = MemoryConsole::default();
    let left_ref = left_runtime
        .spawn(&left_counter, &mut console)
        .expect("left Actor starts");
    let right_ref = right_runtime
        .spawn(&right_counter, &mut console)
        .expect("right Actor starts");

    assert_eq!(left_counter, right_counter);
    assert_eq!(left_ref.actor_type(), right_ref.actor_type());
    assert_eq!(left_ref.message_schema(), right_ref.message_schema());
    assert_eq!(left_ref.actor(), right_ref.actor());
    assert_ne!(left_ref.runtime(), right_ref.runtime());

    left_runtime
        .shutdown(ActorShutdownReason::OwnerCompleted)
        .expect("left runtime shutdown");
    right_runtime
        .shutdown(ActorShutdownReason::OwnerCompleted)
        .expect("right runtime shutdown");
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CandidateEndpointId([u8; 8]);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CandidateRemoteActorId([u8; 8]);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CandidateProtocolVersion(u16);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CandidateCapabilityToken([u8; 8]);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CandidateRemoteParts {
    endpoint: CandidateEndpointId,
    actor: CandidateRemoteActorId,
    protocol: CandidateProtocolVersion,
    capability: CandidateCapabilityToken,
}

fn candidate_remote_parts() -> CandidateRemoteParts {
    CandidateRemoteParts {
        endpoint: CandidateEndpointId(*b"endpoint"),
        actor: CandidateRemoteActorId(*b"actor-id"),
        protocol: CandidateProtocolVersion(1),
        capability: CandidateCapabilityToken(*b"test-cap"),
    }
}

#[test]
fn candidate_remote_dimension_separation() {
    assert_registered(
        EvidenceCase::CandidateRemoteDimensionSeparation,
        "candidate-remote-dimension-separation",
    );
    let baseline = candidate_remote_parts();
    let changed_endpoint = CandidateRemoteParts {
        endpoint: CandidateEndpointId(*b"endpoin2"),
        ..baseline
    };
    let changed_actor = CandidateRemoteParts {
        actor: CandidateRemoteActorId(*b"actor-02"),
        ..baseline
    };
    let changed_protocol = CandidateRemoteParts {
        protocol: CandidateProtocolVersion(2),
        ..baseline
    };
    let changed_capability = CandidateRemoteParts {
        capability: CandidateCapabilityToken(*b"test-ca2"),
        ..baseline
    };

    for changed in [
        changed_endpoint,
        changed_actor,
        changed_protocol,
        changed_capability,
    ] {
        assert_ne!(baseline, changed);
    }
    assert_eq!(
        [
            baseline,
            changed_endpoint,
            changed_actor,
            changed_protocol,
            changed_capability,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
        .len(),
        5
    );
}

#[test]
fn local_to_remote_conversion_absence() {
    assert_registered(
        EvidenceCase::LocalToRemoteConversionAbsence,
        "local-to-remote-conversion-absence",
    );
    const ACTOR_RUNTIME_SOURCE: &str = include_str!("actor_runtime.rs");
    const ACTOR_CORE_SOURCE: &str = include_str!("../../ling-effects/src/actor_core.rs");

    for source in [ACTOR_RUNTIME_SOURCE, ACTOR_CORE_SOURCE] {
        for forbidden in [
            "impl From<LocalActorRef>",
            "impl TryFrom<LocalActorRef>",
            "pub fn to_remote",
            "pub fn into_remote",
            "pub fn serialize_actor_ref",
            "pub fn actor_network_address",
            "pub fn endpoint(&self)",
        ] {
            assert!(
                !source.contains(forbidden),
                "forbidden DEC-0285 local-to-remote surface: {forbidden}"
            );
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum RemoteConcern {
    LocalReferenceSeparation,
    RemoteReferenceIdentity,
    EndpointIdentity,
    RemoteActorIdentity,
    ProtocolVersion,
    CapabilityToken,
    EndpointAuthority,
    ProtocolNegotiation,
    NetworkEffect,
    ActorSendEffect,
    DeliveryOutcome,
    FaultOutcome,
    Incarnation,
    SerializationBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConcernDisposition {
    ExistingLocalEvidence,
    TestOnlyCandidateDimension,
    DeferredPublicContract,
}

const CONCERN_DISPOSITIONS: [(RemoteConcern, ConcernDisposition); 14] = [
    (
        RemoteConcern::LocalReferenceSeparation,
        ConcernDisposition::ExistingLocalEvidence,
    ),
    (
        RemoteConcern::RemoteReferenceIdentity,
        ConcernDisposition::TestOnlyCandidateDimension,
    ),
    (
        RemoteConcern::EndpointIdentity,
        ConcernDisposition::TestOnlyCandidateDimension,
    ),
    (
        RemoteConcern::RemoteActorIdentity,
        ConcernDisposition::TestOnlyCandidateDimension,
    ),
    (
        RemoteConcern::ProtocolVersion,
        ConcernDisposition::TestOnlyCandidateDimension,
    ),
    (
        RemoteConcern::CapabilityToken,
        ConcernDisposition::TestOnlyCandidateDimension,
    ),
    (
        RemoteConcern::EndpointAuthority,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        RemoteConcern::ProtocolNegotiation,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        RemoteConcern::NetworkEffect,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        RemoteConcern::ActorSendEffect,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        RemoteConcern::DeliveryOutcome,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        RemoteConcern::FaultOutcome,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        RemoteConcern::Incarnation,
        ConcernDisposition::DeferredPublicContract,
    ),
    (
        RemoteConcern::SerializationBoundary,
        ConcernDisposition::ExistingLocalEvidence,
    ),
];

fn assert_concern_dispositions() {
    assert_eq!(CONCERN_DISPOSITIONS.len(), 14);
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
            .filter(|(_, disposition)| *disposition == ConcernDisposition::ExistingLocalEvidence)
            .count(),
        2
    );
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .filter(|(_, disposition)| {
                *disposition == ConcernDisposition::TestOnlyCandidateDimension
            })
            .count(),
        5
    );
    assert_eq!(
        CONCERN_DISPOSITIONS
            .iter()
            .filter(|(_, disposition)| {
                *disposition == ConcernDisposition::DeferredPublicContract
            })
            .count(),
        7
    );
}

fn assert_public_remote_surfaces_absent() {
    const EVAL_SOURCE: &str = include_str!("lib.rs");
    const ACTOR_RUNTIME_SOURCE: &str = include_str!("actor_runtime.rs");
    const EFFECT_SOURCE: &str = include_str!("../../ling-effects/src/lib.rs");
    const TYPE_SOURCE: &str = include_str!("../../ling-types/src/lib.rs");
    const PROJECT_SOURCE: &str = include_str!("../../ling-project/src/lib.rs");
    const CLI_CATALOG_SOURCE: &str = include_str!("../../ling-cli/src/command_catalog.rs");
    const DIAGNOSTIC_CODES: &str = include_str!("../../../docs/ERROR-CODES.md");
    const SCHEMA_REGISTRY: &str = include_str!("../../../schemas/registry.toml");
    const PROTOCOL_INVENTORY: &str =
        include_str!("../../../docs/governance/protocol-inventory.toml");

    for production_source in [
        EVAL_SOURCE,
        ACTOR_RUNTIME_SOURCE,
        EFFECT_SOURCE,
        TYPE_SOURCE,
        PROJECT_SOURCE,
    ] {
        for forbidden_surface in [
            "pub struct RemoteRef",
            "pub struct EndpointId",
            "pub struct RemoteActorId",
            "pub struct CapabilityToken",
            "pub fn remote_send",
            "pub fn connect_endpoint",
            "pub fn authenticate_endpoint",
            "pub fn serialize_actor_ref",
        ] {
            assert!(
                !production_source.contains(forbidden_surface),
                "forbidden DEC-0285 production surface: {forbidden_surface}"
            );
        }
    }
    assert!(!CLI_CATALOG_SOURCE.contains("Command::Remote"));
    assert!(!CLI_CATALOG_SOURCE.contains("\"remote\" =>"));
    assert!(!DIAGNOSTIC_CODES.contains("L-REMOTE-"));
    assert!(!SCHEMA_REGISTRY.to_ascii_lowercase().contains("remote"));
    assert!(!PROTOCOL_INVENTORY.contains("id = \"PROTO-REMOTE"));
}

#[test]
fn deferred_public_remote_surface_absence() {
    assert_registered(
        EvidenceCase::DeferredPublicRemoteSurfaceAbsence,
        "deferred-public-remote-surface-absence",
    );
    assert_concern_dispositions();
    assert_public_remote_surfaces_absent();
}
