use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNodeConformanceBoundary {
    NodeConformance,
    ConformanceVersion,
    CheckedNodeCore,
    FixtureManifest,
    ConformanceProtocol,
    Oracle,
    InitialState,
    Tick,
    StateTransition,
    MultiTick,
    MultiRate,
    ClockRate,
    InputPresence,
    StaleInput,
    InputOrder,
    Deadline,
    Overrun,
    Fault,
    Fallback,
    Restart,
    SafeMode,
    StaticSchedule,
    Wcet,
    MemoryBound,
    AbiTarget,
    ReferenceSimulation,
    TargetExecution,
    EventIdentity,
    EffectIdentity,
    InputTrace,
    OutputTrace,
    Replay,
    Divergence,
    Corruption,
    Privacy,
    Migration,
    NodeActorBridge,
    TaskBoundary,
    Ownership,
    Mailbox,
    SemanticId,
    SourceSpan,
    DiagnosticCode,
    DiagnosticFacts,
    DeterministicOrdering,
    PositiveFixture,
    NegativeFixture,
    InitializationFixture,
    TickStateFixture,
    MultiRateFixture,
    StaleInputFixture,
    DeadlineFixture,
    FallbackFixture,
    RestartSafeModeFixture,
    ReplayFixture,
    ScheduleFixture,
    DifferentialFixture,
    UnicodeFixture,
    MigrationFixture,
    ProtocolInventory,
}

impl PlannedNodeConformanceBoundary {
    const ALL: [Self; 60] = [
        Self::NodeConformance,
        Self::ConformanceVersion,
        Self::CheckedNodeCore,
        Self::FixtureManifest,
        Self::ConformanceProtocol,
        Self::Oracle,
        Self::InitialState,
        Self::Tick,
        Self::StateTransition,
        Self::MultiTick,
        Self::MultiRate,
        Self::ClockRate,
        Self::InputPresence,
        Self::StaleInput,
        Self::InputOrder,
        Self::Deadline,
        Self::Overrun,
        Self::Fault,
        Self::Fallback,
        Self::Restart,
        Self::SafeMode,
        Self::StaticSchedule,
        Self::Wcet,
        Self::MemoryBound,
        Self::AbiTarget,
        Self::ReferenceSimulation,
        Self::TargetExecution,
        Self::EventIdentity,
        Self::EffectIdentity,
        Self::InputTrace,
        Self::OutputTrace,
        Self::Replay,
        Self::Divergence,
        Self::Corruption,
        Self::Privacy,
        Self::Migration,
        Self::NodeActorBridge,
        Self::TaskBoundary,
        Self::Ownership,
        Self::Mailbox,
        Self::SemanticId,
        Self::SourceSpan,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::DeterministicOrdering,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::InitializationFixture,
        Self::TickStateFixture,
        Self::MultiRateFixture,
        Self::StaleInputFixture,
        Self::DeadlineFixture,
        Self::FallbackFixture,
        Self::RestartSafeModeFixture,
        Self::ReplayFixture,
        Self::ScheduleFixture,
        Self::DifferentialFixture,
        Self::UnicodeFixture,
        Self::MigrationFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeConformanceInventory {
    boundaries: Box<[PlannedNodeConformanceBoundary]>,
}

impl NodeConformanceInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNodeConformanceBoundary>,
    ) -> Result<Self, PlannedNodeConformanceBoundary> {
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
        let mut bytes = b"ling.node-conformance-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_node_conformance_boundaries_are_complete_and_ordered() {
    let inventory = NodeConformanceInventory::new(PlannedNodeConformanceBoundary::ALL)
        .expect("planned Node conformance boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNodeConformanceBoundary::ALL
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
fn node_conformance_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NodeConformanceInventory::new(PlannedNodeConformanceBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        NodeConformanceInventory::new(PlannedNodeConformanceBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NodeConformanceInventory::new([
        PlannedNodeConformanceBoundary::NodeConformance,
        PlannedNodeConformanceBoundary::NodeConformance,
    ])
    .expect_err("duplicate Node conformance boundary must be rejected");
    assert_eq!(duplicate, PlannedNodeConformanceBoundary::NodeConformance);
}

#[test]
fn node_conformance_evidence_has_no_oracle_authority() {
    let inventory = NodeConformanceInventory::new([
        PlannedNodeConformanceBoundary::NodeConformance,
        PlannedNodeConformanceBoundary::FixtureManifest,
        PlannedNodeConformanceBoundary::ReferenceSimulation,
        PlannedNodeConformanceBoundary::TargetExecution,
        PlannedNodeConformanceBoundary::DiagnosticCode,
        PlannedNodeConformanceBoundary::ProtocolInventory,
    ])
    .expect("bounded Node conformance evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.node-conformance-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
