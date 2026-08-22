use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNodeActorBoundary {
    NodeActorBoundary,
    ActorIdentity,
    NodeIdentity,
    CheckedNodeCore,
    CheckedActorCore,
    BridgeIdentity,
    InputPort,
    OutputPort,
    MessageType,
    EnvelopeVersion,
    SemanticId,
    SourceSpan,
    Serialization,
    Ownership,
    BorrowMove,
    Managed,
    Mailbox,
    QueueCapacity,
    Admission,
    Backpressure,
    DropExpiry,
    StaleInput,
    Sampling,
    Commit,
    ClockConversion,
    DeliveryOrder,
    TieBreak,
    BoundedMemory,
    OutputEvent,
    ActorTurn,
    AwaitReentry,
    Cancellation,
    Supervision,
    Restart,
    Shutdown,
    Fault,
    Fallback,
    HardRealtimeNonWait,
    NetworkServiceBoundary,
    CriticalProfile,
    NativeDevice,
    TargetProfile,
    Replay,
    EffectRecord,
    PrivacyRedaction,
    CorruptionDivergence,
    Migration,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    CapacityFixture,
    StaleDropFixture,
    OrderingFixture,
    SamplingFixture,
    OwnershipFixture,
    RestartReplayFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedNodeActorBoundary {
    const ALL: [Self; 60] = [
        Self::NodeActorBoundary,
        Self::ActorIdentity,
        Self::NodeIdentity,
        Self::CheckedNodeCore,
        Self::CheckedActorCore,
        Self::BridgeIdentity,
        Self::InputPort,
        Self::OutputPort,
        Self::MessageType,
        Self::EnvelopeVersion,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Serialization,
        Self::Ownership,
        Self::BorrowMove,
        Self::Managed,
        Self::Mailbox,
        Self::QueueCapacity,
        Self::Admission,
        Self::Backpressure,
        Self::DropExpiry,
        Self::StaleInput,
        Self::Sampling,
        Self::Commit,
        Self::ClockConversion,
        Self::DeliveryOrder,
        Self::TieBreak,
        Self::BoundedMemory,
        Self::OutputEvent,
        Self::ActorTurn,
        Self::AwaitReentry,
        Self::Cancellation,
        Self::Supervision,
        Self::Restart,
        Self::Shutdown,
        Self::Fault,
        Self::Fallback,
        Self::HardRealtimeNonWait,
        Self::NetworkServiceBoundary,
        Self::CriticalProfile,
        Self::NativeDevice,
        Self::TargetProfile,
        Self::Replay,
        Self::EffectRecord,
        Self::PrivacyRedaction,
        Self::CorruptionDivergence,
        Self::Migration,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CapacityFixture,
        Self::StaleDropFixture,
        Self::OrderingFixture,
        Self::SamplingFixture,
        Self::OwnershipFixture,
        Self::RestartReplayFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeActorBoundaryInventory {
    boundaries: Box<[PlannedNodeActorBoundary]>,
}

impl NodeActorBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNodeActorBoundary>,
    ) -> Result<Self, PlannedNodeActorBoundary> {
        let mut boundaries = boundaries.into_iter().collect::<Vec<_>>();
        boundaries.sort_unstable_by_key(|boundary| boundary.rank());
        let mut seen = BTreeSet::new();
        for boundary in &boundaries {
            if !seen.insert(*boundary) {
                return Err(*boundary);
            }
        }
        Ok(Self {
            boundaries: boundaries.into_boxed_slice(),
        })
    }

    fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = b"ling.node-actor-boundary-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_node_actor_boundaries_are_complete_and_ordered() {
    let inventory = NodeActorBoundaryInventory::new(PlannedNodeActorBoundary::ALL)
        .expect("planned Node/Actor boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNodeActorBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..60).collect::<Vec<_>>()
    );
}

#[test]
fn node_actor_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NodeActorBoundaryInventory::new(PlannedNodeActorBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = NodeActorBoundaryInventory::new(PlannedNodeActorBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NodeActorBoundaryInventory::new([
        PlannedNodeActorBoundary::NodeActorBoundary,
        PlannedNodeActorBoundary::NodeActorBoundary,
    ])
    .expect_err("duplicate Node/Actor boundary must be rejected");
    assert_eq!(duplicate, PlannedNodeActorBoundary::NodeActorBoundary);
}

#[test]
fn node_actor_boundary_evidence_has_no_bridge_authority() {
    let inventory = NodeActorBoundaryInventory::new([
        PlannedNodeActorBoundary::NodeActorBoundary,
        PlannedNodeActorBoundary::Mailbox,
        PlannedNodeActorBoundary::HardRealtimeNonWait,
        PlannedNodeActorBoundary::Replay,
        PlannedNodeActorBoundary::DiagnosticCode,
        PlannedNodeActorBoundary::ProtocolInventory,
    ])
    .expect("bounded Node/Actor boundary evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.node-actor-boundary-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
