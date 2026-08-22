use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNodeVirtualTimeRuntimeBoundary {
    VirtualTimeReferenceRuntime,
    ReferenceRuntime,
    CheckedNodeCore,
    Epoch,
    TimeUnit,
    ClockAdvance,
    TimeOverflow,
    Tick,
    Release,
    Deadline,
    TieBreak,
    InjectedInput,
    OutputTrace,
    PortSampling,
    StateCommit,
    EventIdentity,
    SourceSpan,
    SemanticId,
    TraceBound,
    CanonicalSerialization,
    Overrun,
    MissedTick,
    Fault,
    Fallback,
    Cancellation,
    Restart,
    Recovery,
    EffectRecord,
    InputRecord,
    OutputRecord,
    ReplayEquivalence,
    EventOrder,
    Privacy,
    Redaction,
    Corruption,
    Truncation,
    Divergence,
    Migration,
    CriticalProfile,
    TaskActor,
    NativeAbi,
    KernelDevice,
    TargetRuntime,
    UnknownTrace,
    UnsupportedNode,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    ClockTickFixture,
    InputOutputFixture,
    StateFixture,
    OverrunFaultFixture,
    ReplayFixture,
    CorruptionFixture,
    MigrationFixture,
    DeterminismFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedNodeVirtualTimeRuntimeBoundary {
    const ALL: [Self; 60] = [
        Self::VirtualTimeReferenceRuntime,
        Self::ReferenceRuntime,
        Self::CheckedNodeCore,
        Self::Epoch,
        Self::TimeUnit,
        Self::ClockAdvance,
        Self::TimeOverflow,
        Self::Tick,
        Self::Release,
        Self::Deadline,
        Self::TieBreak,
        Self::InjectedInput,
        Self::OutputTrace,
        Self::PortSampling,
        Self::StateCommit,
        Self::EventIdentity,
        Self::SourceSpan,
        Self::SemanticId,
        Self::TraceBound,
        Self::CanonicalSerialization,
        Self::Overrun,
        Self::MissedTick,
        Self::Fault,
        Self::Fallback,
        Self::Cancellation,
        Self::Restart,
        Self::Recovery,
        Self::EffectRecord,
        Self::InputRecord,
        Self::OutputRecord,
        Self::ReplayEquivalence,
        Self::EventOrder,
        Self::Privacy,
        Self::Redaction,
        Self::Corruption,
        Self::Truncation,
        Self::Divergence,
        Self::Migration,
        Self::CriticalProfile,
        Self::TaskActor,
        Self::NativeAbi,
        Self::KernelDevice,
        Self::TargetRuntime,
        Self::UnknownTrace,
        Self::UnsupportedNode,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::ClockTickFixture,
        Self::InputOutputFixture,
        Self::StateFixture,
        Self::OverrunFaultFixture,
        Self::ReplayFixture,
        Self::CorruptionFixture,
        Self::MigrationFixture,
        Self::DeterminismFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeVirtualTimeRuntimeInventory {
    boundaries: Box<[PlannedNodeVirtualTimeRuntimeBoundary]>,
}

impl NodeVirtualTimeRuntimeInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNodeVirtualTimeRuntimeBoundary>,
    ) -> Result<Self, PlannedNodeVirtualTimeRuntimeBoundary> {
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
        let mut bytes = b"ling.node-virtual-time-runtime-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_node_virtual_time_runtime_boundaries_are_complete_and_ordered() {
    let inventory =
        NodeVirtualTimeRuntimeInventory::new(PlannedNodeVirtualTimeRuntimeBoundary::ALL)
            .expect("planned virtual-time runtime boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNodeVirtualTimeRuntimeBoundary::ALL
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
fn node_virtual_time_runtime_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NodeVirtualTimeRuntimeInventory::new(PlannedNodeVirtualTimeRuntimeBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = NodeVirtualTimeRuntimeInventory::new(
        PlannedNodeVirtualTimeRuntimeBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NodeVirtualTimeRuntimeInventory::new([
        PlannedNodeVirtualTimeRuntimeBoundary::VirtualTimeReferenceRuntime,
        PlannedNodeVirtualTimeRuntimeBoundary::VirtualTimeReferenceRuntime,
    ])
    .expect_err("duplicate virtual-time runtime boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedNodeVirtualTimeRuntimeBoundary::VirtualTimeReferenceRuntime
    );
}

#[test]
fn node_virtual_time_runtime_evidence_has_no_runtime_authority() {
    let inventory = NodeVirtualTimeRuntimeInventory::new([
        PlannedNodeVirtualTimeRuntimeBoundary::VirtualTimeReferenceRuntime,
        PlannedNodeVirtualTimeRuntimeBoundary::Epoch,
        PlannedNodeVirtualTimeRuntimeBoundary::InjectedInput,
        PlannedNodeVirtualTimeRuntimeBoundary::UnknownTrace,
        PlannedNodeVirtualTimeRuntimeBoundary::DiagnosticCode,
        PlannedNodeVirtualTimeRuntimeBoundary::ProtocolInventory,
    ])
    .expect("bounded virtual-time runtime evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.node-virtual-time-runtime-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
