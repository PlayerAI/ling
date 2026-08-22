use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNodeStaticSchedulingBoundary {
    StaticNodeScheduling,
    GraphIdentity,
    GraphElements,
    TopologicalOrder,
    LegalCycle,
    CycleBreak,
    ClockDomain,
    Rate,
    Period,
    Phase,
    MultiRateBridge,
    RateConversion,
    Buffering,
    Backpressure,
    Loss,
    StateOwnership,
    Priority,
    Release,
    Deadline,
    Jitter,
    Preemption,
    CooperativeExecution,
    Admission,
    Schedulability,
    Wcet,
    TargetCompiler,
    InterruptAssumption,
    CacheBusAssumption,
    ReleaseOverrun,
    Fault,
    Cancellation,
    Restart,
    RecoveryFallback,
    Replay,
    Determinism,
    ManifestLifecycle,
    CriticalProfile,
    TaskActor,
    KernelDevice,
    MemoryQueue,
    UnknownSchedulability,
    UnsupportedBridge,
    TargetMismatch,
    DiagnosticCode,
    DiagnosticFacts,
    SemanticId,
    SourceSpan,
    PositiveFixture,
    NegativeFixture,
    GraphScheduleFixture,
    RateClockFixture,
    BridgeFixture,
    CycleFixture,
    DeadlineOverrunFixture,
    TargetMigrationFixture,
    VirtualClockFixture,
    ReplayFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedNodeStaticSchedulingBoundary {
    const ALL: [Self; 60] = [
        Self::StaticNodeScheduling,
        Self::GraphIdentity,
        Self::GraphElements,
        Self::TopologicalOrder,
        Self::LegalCycle,
        Self::CycleBreak,
        Self::ClockDomain,
        Self::Rate,
        Self::Period,
        Self::Phase,
        Self::MultiRateBridge,
        Self::RateConversion,
        Self::Buffering,
        Self::Backpressure,
        Self::Loss,
        Self::StateOwnership,
        Self::Priority,
        Self::Release,
        Self::Deadline,
        Self::Jitter,
        Self::Preemption,
        Self::CooperativeExecution,
        Self::Admission,
        Self::Schedulability,
        Self::Wcet,
        Self::TargetCompiler,
        Self::InterruptAssumption,
        Self::CacheBusAssumption,
        Self::ReleaseOverrun,
        Self::Fault,
        Self::Cancellation,
        Self::Restart,
        Self::RecoveryFallback,
        Self::Replay,
        Self::Determinism,
        Self::ManifestLifecycle,
        Self::CriticalProfile,
        Self::TaskActor,
        Self::KernelDevice,
        Self::MemoryQueue,
        Self::UnknownSchedulability,
        Self::UnsupportedBridge,
        Self::TargetMismatch,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::SemanticId,
        Self::SourceSpan,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::GraphScheduleFixture,
        Self::RateClockFixture,
        Self::BridgeFixture,
        Self::CycleFixture,
        Self::DeadlineOverrunFixture,
        Self::TargetMigrationFixture,
        Self::VirtualClockFixture,
        Self::ReplayFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeStaticSchedulingInventory {
    boundaries: Box<[PlannedNodeStaticSchedulingBoundary]>,
}

impl NodeStaticSchedulingInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNodeStaticSchedulingBoundary>,
    ) -> Result<Self, PlannedNodeStaticSchedulingBoundary> {
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
        let mut bytes = b"ling.node-static-scheduling-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_node_static_scheduling_boundaries_are_complete_and_ordered() {
    let inventory = NodeStaticSchedulingInventory::new(PlannedNodeStaticSchedulingBoundary::ALL)
        .expect("planned Node scheduling boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNodeStaticSchedulingBoundary::ALL
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
fn node_static_scheduling_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NodeStaticSchedulingInventory::new(PlannedNodeStaticSchedulingBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = NodeStaticSchedulingInventory::new(
        PlannedNodeStaticSchedulingBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NodeStaticSchedulingInventory::new([
        PlannedNodeStaticSchedulingBoundary::StaticNodeScheduling,
        PlannedNodeStaticSchedulingBoundary::StaticNodeScheduling,
    ])
    .expect_err("duplicate Node scheduling boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedNodeStaticSchedulingBoundary::StaticNodeScheduling
    );
}

#[test]
fn node_static_scheduling_evidence_has_no_scheduler_authority() {
    let inventory = NodeStaticSchedulingInventory::new([
        PlannedNodeStaticSchedulingBoundary::StaticNodeScheduling,
        PlannedNodeStaticSchedulingBoundary::TopologicalOrder,
        PlannedNodeStaticSchedulingBoundary::UnknownSchedulability,
        PlannedNodeStaticSchedulingBoundary::ReleaseOverrun,
        PlannedNodeStaticSchedulingBoundary::DiagnosticCode,
        PlannedNodeStaticSchedulingBoundary::ProtocolInventory,
    ])
    .expect("bounded Node scheduling evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.node-static-scheduling-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
