use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedLaunchRuntimeBoundary {
    RuntimeBoundary,
    DeviceDiscovery,
    StableDeviceIdentity,
    CapabilityMatch,
    ModuleLoad,
    BinaryValidation,
    BufferBind,
    BufferLayout,
    BufferOwnership,
    LaunchDimensions,
    WorkgroupGrid,
    QueueSubmit,
    QueueOrdering,
    Synchronization,
    Visibility,
    Cancellation,
    DeviceLost,
    Fault,
    CleanupSuccess,
    CleanupError,
    CleanupFault,
    ResourceBudget,
    Metrics,
    ExplainPlan,
    RuntimeAbi,
    HostIsolation,
    VendorIsolation,
    TargetIdentity,
    ToolchainIdentity,
    DriverIdentity,
    NumericMode,
    Determinism,
    SourceMap,
    Utf8Spans,
    SemanticId,
    UnsupportedHardware,
    FallbackPolicy,
    RejectionPolicy,
    PositiveFixture,
    NegativeFixture,
    DiscoveryFixture,
    CapabilityFixture,
    ModuleCorruptionFixture,
    BindingFixture,
    LaunchFixture,
    SynchronizationFixture,
    DeviceLossFixture,
    CleanupFixture,
    CancellationFixture,
    ResourceFixture,
    UnicodeFixture,
    MigrationFixture,
    DifferentialFixture,
    DiagnosticCode,
    DiagnosticFacts,
    HostPathExclusion,
    DriverPathExclusion,
    AddressExclusion,
    TimestampExclusion,
    DebugOutputExclusion,
}

impl PlannedLaunchRuntimeBoundary {
    const ALL: [Self; 60] = [
        Self::RuntimeBoundary,
        Self::DeviceDiscovery,
        Self::StableDeviceIdentity,
        Self::CapabilityMatch,
        Self::ModuleLoad,
        Self::BinaryValidation,
        Self::BufferBind,
        Self::BufferLayout,
        Self::BufferOwnership,
        Self::LaunchDimensions,
        Self::WorkgroupGrid,
        Self::QueueSubmit,
        Self::QueueOrdering,
        Self::Synchronization,
        Self::Visibility,
        Self::Cancellation,
        Self::DeviceLost,
        Self::Fault,
        Self::CleanupSuccess,
        Self::CleanupError,
        Self::CleanupFault,
        Self::ResourceBudget,
        Self::Metrics,
        Self::ExplainPlan,
        Self::RuntimeAbi,
        Self::HostIsolation,
        Self::VendorIsolation,
        Self::TargetIdentity,
        Self::ToolchainIdentity,
        Self::DriverIdentity,
        Self::NumericMode,
        Self::Determinism,
        Self::SourceMap,
        Self::Utf8Spans,
        Self::SemanticId,
        Self::UnsupportedHardware,
        Self::FallbackPolicy,
        Self::RejectionPolicy,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::DiscoveryFixture,
        Self::CapabilityFixture,
        Self::ModuleCorruptionFixture,
        Self::BindingFixture,
        Self::LaunchFixture,
        Self::SynchronizationFixture,
        Self::DeviceLossFixture,
        Self::CleanupFixture,
        Self::CancellationFixture,
        Self::ResourceFixture,
        Self::UnicodeFixture,
        Self::MigrationFixture,
        Self::DifferentialFixture,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::HostPathExclusion,
        Self::DriverPathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::DebugOutputExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LaunchRuntimeInventory {
    boundaries: Box<[PlannedLaunchRuntimeBoundary]>,
}

impl LaunchRuntimeInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedLaunchRuntimeBoundary>,
    ) -> Result<Self, PlannedLaunchRuntimeBoundary> {
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
        let mut bytes = b"ling.launch-runtime-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_launch_runtime_boundaries_are_complete_and_ordered() {
    let inventory = LaunchRuntimeInventory::new(PlannedLaunchRuntimeBoundary::ALL)
        .expect("planned launch and runtime boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedLaunchRuntimeBoundary::ALL
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
fn launch_runtime_evidence_is_order_independent_and_duplicate_checked() {
    let forward = LaunchRuntimeInventory::new(PlannedLaunchRuntimeBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = LaunchRuntimeInventory::new(PlannedLaunchRuntimeBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = LaunchRuntimeInventory::new([
        PlannedLaunchRuntimeBoundary::RuntimeBoundary,
        PlannedLaunchRuntimeBoundary::RuntimeBoundary,
    ])
    .expect_err("duplicate launch runtime boundary must be rejected");
    assert_eq!(duplicate, PlannedLaunchRuntimeBoundary::RuntimeBoundary);
}

#[test]
fn launch_runtime_evidence_has_no_runtime_authority() {
    let inventory = LaunchRuntimeInventory::new([
        PlannedLaunchRuntimeBoundary::RuntimeBoundary,
        PlannedLaunchRuntimeBoundary::DeviceDiscovery,
        PlannedLaunchRuntimeBoundary::ModuleLoad,
        PlannedLaunchRuntimeBoundary::BufferBind,
        PlannedLaunchRuntimeBoundary::LaunchDimensions,
        PlannedLaunchRuntimeBoundary::DeviceLost,
        PlannedLaunchRuntimeBoundary::DiagnosticFacts,
        PlannedLaunchRuntimeBoundary::DebugOutputExclusion,
    ])
    .expect("bounded launch and runtime evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.launch-runtime-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
