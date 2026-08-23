use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedCriticalRuntimeTargetPackageBoundary {
    CriticalRuntimeTargetPackage,
    Version,
    CriticalProfile,
    CriticalCore,
    StaticScheduler,
    ScheduleIdentity,
    TaskAdmission,
    Priority,
    TieBreak,
    ClockSource,
    TickSource,
    Deadline,
    WcetAssumption,
    Interrupt,
    IoTreatment,
    QueueBound,
    RecursionBound,
    NoGeneralHeap,
    HeapBound,
    BoundedStack,
    StackBound,
    FrameLayout,
    AllocationBound,
    OwnershipAlias,
    ResourceDrop,
    Cleanup,
    DeterministicStartup,
    Initialization,
    Shutdown,
    Reset,
    Cancellation,
    Fault,
    Overrun,
    Watchdog,
    SafeState,
    Recovery,
    FailClosed,
    TargetPrimitivePackage,
    TargetIdentity,
    PrimitiveIdentity,
    QualifiedPrimitiveList,
    DevicePrimitive,
    ClockPrimitive,
    WatchdogPrimitive,
    Capability,
    HostServiceDeclaration,
    UndeclaredHostService,
    AbiIdentity,
    ToolchainIdentity,
    ArtifactIdentity,
    TargetSpecificEvidence,
    TimingEvidence,
    MemoryEvidence,
    Assumption,
    TcbIdentity,
    IndependentVerifier,
    PositiveFixture,
    NegativeFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedCriticalRuntimeTargetPackageBoundary {
    const ALL: [Self; 60] = [
        Self::CriticalRuntimeTargetPackage,
        Self::Version,
        Self::CriticalProfile,
        Self::CriticalCore,
        Self::StaticScheduler,
        Self::ScheduleIdentity,
        Self::TaskAdmission,
        Self::Priority,
        Self::TieBreak,
        Self::ClockSource,
        Self::TickSource,
        Self::Deadline,
        Self::WcetAssumption,
        Self::Interrupt,
        Self::IoTreatment,
        Self::QueueBound,
        Self::RecursionBound,
        Self::NoGeneralHeap,
        Self::HeapBound,
        Self::BoundedStack,
        Self::StackBound,
        Self::FrameLayout,
        Self::AllocationBound,
        Self::OwnershipAlias,
        Self::ResourceDrop,
        Self::Cleanup,
        Self::DeterministicStartup,
        Self::Initialization,
        Self::Shutdown,
        Self::Reset,
        Self::Cancellation,
        Self::Fault,
        Self::Overrun,
        Self::Watchdog,
        Self::SafeState,
        Self::Recovery,
        Self::FailClosed,
        Self::TargetPrimitivePackage,
        Self::TargetIdentity,
        Self::PrimitiveIdentity,
        Self::QualifiedPrimitiveList,
        Self::DevicePrimitive,
        Self::ClockPrimitive,
        Self::WatchdogPrimitive,
        Self::Capability,
        Self::HostServiceDeclaration,
        Self::UndeclaredHostService,
        Self::AbiIdentity,
        Self::ToolchainIdentity,
        Self::ArtifactIdentity,
        Self::TargetSpecificEvidence,
        Self::TimingEvidence,
        Self::MemoryEvidence,
        Self::Assumption,
        Self::TcbIdentity,
        Self::IndependentVerifier,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CriticalRuntimeTargetPackageInventory {
    boundaries: Box<[PlannedCriticalRuntimeTargetPackageBoundary]>,
}

impl CriticalRuntimeTargetPackageInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCriticalRuntimeTargetPackageBoundary>,
    ) -> Result<Self, PlannedCriticalRuntimeTargetPackageBoundary> {
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
        let mut bytes = b"ling.critical-runtime-target-package-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_critical_runtime_target_package_boundaries_are_complete_and_ordered() {
    let inventory = CriticalRuntimeTargetPackageInventory::new(
        PlannedCriticalRuntimeTargetPackageBoundary::ALL,
    )
    .expect("planned Critical-runtime/Target-Package boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedCriticalRuntimeTargetPackageBoundary::ALL
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
fn critical_runtime_target_package_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CriticalRuntimeTargetPackageInventory::new(
        PlannedCriticalRuntimeTargetPackageBoundary::ALL,
    )
    .expect("forward inventory")
    .canonical_bytes();
    let reverse = CriticalRuntimeTargetPackageInventory::new(
        PlannedCriticalRuntimeTargetPackageBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CriticalRuntimeTargetPackageInventory::new([
        PlannedCriticalRuntimeTargetPackageBoundary::CriticalRuntimeTargetPackage,
        PlannedCriticalRuntimeTargetPackageBoundary::CriticalRuntimeTargetPackage,
    ])
    .expect_err("duplicate Critical-runtime/Target-Package boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedCriticalRuntimeTargetPackageBoundary::CriticalRuntimeTargetPackage
    );
}

#[test]
fn critical_runtime_target_package_evidence_has_no_runtime_or_target_authority() {
    let inventory = CriticalRuntimeTargetPackageInventory::new([
        PlannedCriticalRuntimeTargetPackageBoundary::CriticalRuntimeTargetPackage,
        PlannedCriticalRuntimeTargetPackageBoundary::StaticScheduler,
        PlannedCriticalRuntimeTargetPackageBoundary::NoGeneralHeap,
        PlannedCriticalRuntimeTargetPackageBoundary::BoundedStack,
        PlannedCriticalRuntimeTargetPackageBoundary::DeterministicStartup,
        PlannedCriticalRuntimeTargetPackageBoundary::Watchdog,
        PlannedCriticalRuntimeTargetPackageBoundary::SafeState,
        PlannedCriticalRuntimeTargetPackageBoundary::TargetPrimitivePackage,
        PlannedCriticalRuntimeTargetPackageBoundary::HostServiceDeclaration,
        PlannedCriticalRuntimeTargetPackageBoundary::TargetSpecificEvidence,
        PlannedCriticalRuntimeTargetPackageBoundary::ProtocolInventory,
    ])
    .expect("bounded Critical-runtime/Target-Package evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.critical-runtime-target-package-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 11);
}
