use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNodeNativeRuntimeBoundary {
    NativeNodeRuntime,
    CheckedNodeCore,
    VerifiedNativeIr,
    AbiIdentity,
    CallingConvention,
    DataLayout,
    Endianness,
    Alignment,
    FfiBoundary,
    TargetPrimitive,
    TargetIdentity,
    ToolchainIdentity,
    StaticMemory,
    StackBudget,
    ArenaBudget,
    BufferBudget,
    NoGeneralAllocation,
    StateCell,
    Ownership,
    Region,
    Resource,
    DropCleanup,
    Aliasing,
    SafeState,
    Schedule,
    Clock,
    Timer,
    Tick,
    Deadline,
    Jitter,
    Overrun,
    Watchdog,
    Interrupt,
    Preemption,
    Startup,
    Shutdown,
    Fault,
    Fallback,
    Telemetry,
    CriticalProfile,
    KernelDevice,
    Placement,
    UnsupportedTarget,
    SourceSpan,
    SemanticId,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    AbiLayoutFixture,
    OwnershipDropFixture,
    StaticMemoryFixture,
    TimerWatchdogFixture,
    LifecycleFixture,
    SafeStateFixture,
    TargetArtifactFixture,
    DifferentialFixture,
    MigrationFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedNodeNativeRuntimeBoundary {
    const ALL: [Self; 60] = [
        Self::NativeNodeRuntime,
        Self::CheckedNodeCore,
        Self::VerifiedNativeIr,
        Self::AbiIdentity,
        Self::CallingConvention,
        Self::DataLayout,
        Self::Endianness,
        Self::Alignment,
        Self::FfiBoundary,
        Self::TargetPrimitive,
        Self::TargetIdentity,
        Self::ToolchainIdentity,
        Self::StaticMemory,
        Self::StackBudget,
        Self::ArenaBudget,
        Self::BufferBudget,
        Self::NoGeneralAllocation,
        Self::StateCell,
        Self::Ownership,
        Self::Region,
        Self::Resource,
        Self::DropCleanup,
        Self::Aliasing,
        Self::SafeState,
        Self::Schedule,
        Self::Clock,
        Self::Timer,
        Self::Tick,
        Self::Deadline,
        Self::Jitter,
        Self::Overrun,
        Self::Watchdog,
        Self::Interrupt,
        Self::Preemption,
        Self::Startup,
        Self::Shutdown,
        Self::Fault,
        Self::Fallback,
        Self::Telemetry,
        Self::CriticalProfile,
        Self::KernelDevice,
        Self::Placement,
        Self::UnsupportedTarget,
        Self::SourceSpan,
        Self::SemanticId,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::AbiLayoutFixture,
        Self::OwnershipDropFixture,
        Self::StaticMemoryFixture,
        Self::TimerWatchdogFixture,
        Self::LifecycleFixture,
        Self::SafeStateFixture,
        Self::TargetArtifactFixture,
        Self::DifferentialFixture,
        Self::MigrationFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeNativeRuntimeInventory {
    boundaries: Box<[PlannedNodeNativeRuntimeBoundary]>,
}

impl NodeNativeRuntimeInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNodeNativeRuntimeBoundary>,
    ) -> Result<Self, PlannedNodeNativeRuntimeBoundary> {
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
        let mut bytes = b"ling.node-native-runtime-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_node_native_runtime_boundaries_are_complete_and_ordered() {
    let inventory = NodeNativeRuntimeInventory::new(PlannedNodeNativeRuntimeBoundary::ALL)
        .expect("planned Native Node runtime boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNodeNativeRuntimeBoundary::ALL
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
fn node_native_runtime_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NodeNativeRuntimeInventory::new(PlannedNodeNativeRuntimeBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        NodeNativeRuntimeInventory::new(PlannedNodeNativeRuntimeBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NodeNativeRuntimeInventory::new([
        PlannedNodeNativeRuntimeBoundary::NativeNodeRuntime,
        PlannedNodeNativeRuntimeBoundary::NativeNodeRuntime,
    ])
    .expect_err("duplicate Native Node runtime boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedNodeNativeRuntimeBoundary::NativeNodeRuntime
    );
}

#[test]
fn node_native_runtime_evidence_has_no_backend_authority() {
    let inventory = NodeNativeRuntimeInventory::new([
        PlannedNodeNativeRuntimeBoundary::NativeNodeRuntime,
        PlannedNodeNativeRuntimeBoundary::AbiIdentity,
        PlannedNodeNativeRuntimeBoundary::StaticMemory,
        PlannedNodeNativeRuntimeBoundary::UnsupportedTarget,
        PlannedNodeNativeRuntimeBoundary::DiagnosticCode,
        PlannedNodeNativeRuntimeBoundary::ProtocolInventory,
    ])
    .expect("bounded Native Node runtime evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.node-native-runtime-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
