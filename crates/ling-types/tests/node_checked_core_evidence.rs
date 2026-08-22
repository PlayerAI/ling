use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNodeCheckedCoreBoundary {
    NodeCheckedCore,
    CoreSchemaVersion,
    PortType,
    PortIdentity,
    InputPort,
    OutputPort,
    StateCell,
    TickTransition,
    Clock,
    Period,
    Deadline,
    DependencyEdge,
    FeedbackDelay,
    InstantCycle,
    FixedPoint,
    FixedPointProof,
    GraphIdentity,
    GraphOrdering,
    CanonicalBytes,
    SourceSpan,
    SemanticId,
    Ownership,
    Mutability,
    Aliasing,
    Initialization,
    Presence,
    Sampling,
    Commit,
    StateVisibility,
    Restart,
    Cancellation,
    EffectRelation,
    CapabilityRelation,
    TaskActorRelation,
    KernelDeviceRelation,
    FfiRelation,
    FaultTransition,
    ContractHook,
    ResourceBound,
    RecursionBound,
    MailboxBound,
    TargetWcet,
    TargetCompiler,
    EvidenceIdentity,
    UnknownGraph,
    UnsupportedGraph,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    GraphFixture,
    CycleFixture,
    FixedPointFixture,
    StateFixture,
    ClockFixture,
    FaultFixture,
    TargetMigrationFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedNodeCheckedCoreBoundary {
    const ALL: [Self; 60] = [
        Self::NodeCheckedCore,
        Self::CoreSchemaVersion,
        Self::PortType,
        Self::PortIdentity,
        Self::InputPort,
        Self::OutputPort,
        Self::StateCell,
        Self::TickTransition,
        Self::Clock,
        Self::Period,
        Self::Deadline,
        Self::DependencyEdge,
        Self::FeedbackDelay,
        Self::InstantCycle,
        Self::FixedPoint,
        Self::FixedPointProof,
        Self::GraphIdentity,
        Self::GraphOrdering,
        Self::CanonicalBytes,
        Self::SourceSpan,
        Self::SemanticId,
        Self::Ownership,
        Self::Mutability,
        Self::Aliasing,
        Self::Initialization,
        Self::Presence,
        Self::Sampling,
        Self::Commit,
        Self::StateVisibility,
        Self::Restart,
        Self::Cancellation,
        Self::EffectRelation,
        Self::CapabilityRelation,
        Self::TaskActorRelation,
        Self::KernelDeviceRelation,
        Self::FfiRelation,
        Self::FaultTransition,
        Self::ContractHook,
        Self::ResourceBound,
        Self::RecursionBound,
        Self::MailboxBound,
        Self::TargetWcet,
        Self::TargetCompiler,
        Self::EvidenceIdentity,
        Self::UnknownGraph,
        Self::UnsupportedGraph,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::GraphFixture,
        Self::CycleFixture,
        Self::FixedPointFixture,
        Self::StateFixture,
        Self::ClockFixture,
        Self::FaultFixture,
        Self::TargetMigrationFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeCheckedCoreInventory {
    boundaries: Box<[PlannedNodeCheckedCoreBoundary]>,
}

impl NodeCheckedCoreInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNodeCheckedCoreBoundary>,
    ) -> Result<Self, PlannedNodeCheckedCoreBoundary> {
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
        let mut bytes = b"ling.node-checked-core-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_node_checked_core_boundaries_are_complete_and_ordered() {
    let inventory = NodeCheckedCoreInventory::new(PlannedNodeCheckedCoreBoundary::ALL)
        .expect("planned Node Checked Core boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNodeCheckedCoreBoundary::ALL
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
fn node_checked_core_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NodeCheckedCoreInventory::new(PlannedNodeCheckedCoreBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        NodeCheckedCoreInventory::new(PlannedNodeCheckedCoreBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NodeCheckedCoreInventory::new([
        PlannedNodeCheckedCoreBoundary::NodeCheckedCore,
        PlannedNodeCheckedCoreBoundary::NodeCheckedCore,
    ])
    .expect_err("duplicate Node Checked Core boundary must be rejected");
    assert_eq!(duplicate, PlannedNodeCheckedCoreBoundary::NodeCheckedCore);
}

#[test]
fn node_checked_core_evidence_has_no_core_authority() {
    let inventory = NodeCheckedCoreInventory::new([
        PlannedNodeCheckedCoreBoundary::NodeCheckedCore,
        PlannedNodeCheckedCoreBoundary::PortType,
        PlannedNodeCheckedCoreBoundary::InstantCycle,
        PlannedNodeCheckedCoreBoundary::UnknownGraph,
        PlannedNodeCheckedCoreBoundary::DiagnosticCode,
        PlannedNodeCheckedCoreBoundary::ProtocolInventory,
    ])
    .expect("bounded Node Checked Core evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.node-checked-core-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
