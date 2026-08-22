use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedKernelMatrixBoundary {
    MatrixSchema,
    CapabilityIdentifier,
    Condition,
    RejectionCategory,
    ProfileScope,
    TargetScope,
    SourceProvenance,
    SemanticIdProvenance,
    GraphProjection,
    AuditProjection,
    CanonicalBytes,
    Migration,
    DeterministicOrdering,
    ValueTypes,
    RecordTypes,
    RestrictedAdt,
    ManagedValue,
    ResourceValue,
    Allocation,
    Recursion,
    Loop,
    Calls,
    StaticDispatch,
    EffectRows,
    CapabilityRows,
    TaskReject,
    ActorReject,
    NetworkReject,
    IoReject,
    DeviceCapability,
    BufferOwnership,
    AddressSpace,
    AliasProof,
    RaceProof,
    Bounds,
    Overflow,
    NumericMode,
    ReductionOrder,
    TargetDiscovery,
    Fallback,
    CheckedTypedCore,
    VerifiedDerivative,
    Utf8Spans,
    BilingualDiagnostic,
    Unicode17,
    PositiveFixture,
    NegativeFixture,
    GoldenFile,
    RoundTrip,
    UnsupportedTarget,
    CpuReference,
    DeviceDifferential,
    HostPathExclusion,
    AddressExclusion,
    AllocationOrderExclusion,
    DriverLogExclusion,
    ProtocolInventory,
    PublicSchemaBoundary,
    ErrorFacts,
    VersionCompatibility,
}

impl PlannedKernelMatrixBoundary {
    const ALL: [Self; 60] = [
        Self::MatrixSchema,
        Self::CapabilityIdentifier,
        Self::Condition,
        Self::RejectionCategory,
        Self::ProfileScope,
        Self::TargetScope,
        Self::SourceProvenance,
        Self::SemanticIdProvenance,
        Self::GraphProjection,
        Self::AuditProjection,
        Self::CanonicalBytes,
        Self::Migration,
        Self::DeterministicOrdering,
        Self::ValueTypes,
        Self::RecordTypes,
        Self::RestrictedAdt,
        Self::ManagedValue,
        Self::ResourceValue,
        Self::Allocation,
        Self::Recursion,
        Self::Loop,
        Self::Calls,
        Self::StaticDispatch,
        Self::EffectRows,
        Self::CapabilityRows,
        Self::TaskReject,
        Self::ActorReject,
        Self::NetworkReject,
        Self::IoReject,
        Self::DeviceCapability,
        Self::BufferOwnership,
        Self::AddressSpace,
        Self::AliasProof,
        Self::RaceProof,
        Self::Bounds,
        Self::Overflow,
        Self::NumericMode,
        Self::ReductionOrder,
        Self::TargetDiscovery,
        Self::Fallback,
        Self::CheckedTypedCore,
        Self::VerifiedDerivative,
        Self::Utf8Spans,
        Self::BilingualDiagnostic,
        Self::Unicode17,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::GoldenFile,
        Self::RoundTrip,
        Self::UnsupportedTarget,
        Self::CpuReference,
        Self::DeviceDifferential,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::AllocationOrderExclusion,
        Self::DriverLogExclusion,
        Self::ProtocolInventory,
        Self::PublicSchemaBoundary,
        Self::ErrorFacts,
        Self::VersionCompatibility,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::MatrixSchema => 0,
            Self::CapabilityIdentifier => 1,
            Self::Condition => 2,
            Self::RejectionCategory => 3,
            Self::ProfileScope => 4,
            Self::TargetScope => 5,
            Self::SourceProvenance => 6,
            Self::SemanticIdProvenance => 7,
            Self::GraphProjection => 8,
            Self::AuditProjection => 9,
            Self::CanonicalBytes => 10,
            Self::Migration => 11,
            Self::DeterministicOrdering => 12,
            Self::ValueTypes => 13,
            Self::RecordTypes => 14,
            Self::RestrictedAdt => 15,
            Self::ManagedValue => 16,
            Self::ResourceValue => 17,
            Self::Allocation => 18,
            Self::Recursion => 19,
            Self::Loop => 20,
            Self::Calls => 21,
            Self::StaticDispatch => 22,
            Self::EffectRows => 23,
            Self::CapabilityRows => 24,
            Self::TaskReject => 25,
            Self::ActorReject => 26,
            Self::NetworkReject => 27,
            Self::IoReject => 28,
            Self::DeviceCapability => 29,
            Self::BufferOwnership => 30,
            Self::AddressSpace => 31,
            Self::AliasProof => 32,
            Self::RaceProof => 33,
            Self::Bounds => 34,
            Self::Overflow => 35,
            Self::NumericMode => 36,
            Self::ReductionOrder => 37,
            Self::TargetDiscovery => 38,
            Self::Fallback => 39,
            Self::CheckedTypedCore => 40,
            Self::VerifiedDerivative => 41,
            Self::Utf8Spans => 42,
            Self::BilingualDiagnostic => 43,
            Self::Unicode17 => 44,
            Self::PositiveFixture => 45,
            Self::NegativeFixture => 46,
            Self::GoldenFile => 47,
            Self::RoundTrip => 48,
            Self::UnsupportedTarget => 49,
            Self::CpuReference => 50,
            Self::DeviceDifferential => 51,
            Self::HostPathExclusion => 52,
            Self::AddressExclusion => 53,
            Self::AllocationOrderExclusion => 54,
            Self::DriverLogExclusion => 55,
            Self::ProtocolInventory => 56,
            Self::PublicSchemaBoundary => 57,
            Self::ErrorFacts => 58,
            Self::VersionCompatibility => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct KernelMatrixInventory {
    boundaries: Box<[PlannedKernelMatrixBoundary]>,
}

impl KernelMatrixInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedKernelMatrixBoundary>,
    ) -> Result<Self, PlannedKernelMatrixBoundary> {
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
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ling.kernel-capability-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_kernel_matrix_boundaries_are_complete_and_ordered() {
    let inventory = KernelMatrixInventory::new(PlannedKernelMatrixBoundary::ALL)
        .expect("planned Kernel matrix boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedKernelMatrixBoundary::ALL
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
fn kernel_matrix_evidence_is_order_independent_and_duplicate_checked() {
    let forward = KernelMatrixInventory::new(PlannedKernelMatrixBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = KernelMatrixInventory::new(PlannedKernelMatrixBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = KernelMatrixInventory::new([
        PlannedKernelMatrixBoundary::MatrixSchema,
        PlannedKernelMatrixBoundary::MatrixSchema,
    ])
    .expect_err("duplicate Kernel matrix boundary must be rejected");
    assert_eq!(duplicate, PlannedKernelMatrixBoundary::MatrixSchema);
}

#[test]
fn kernel_matrix_evidence_has_no_kernel_authority() {
    let inventory = KernelMatrixInventory::new([
        PlannedKernelMatrixBoundary::MatrixSchema,
        PlannedKernelMatrixBoundary::CapabilityIdentifier,
        PlannedKernelMatrixBoundary::CheckedTypedCore,
        PlannedKernelMatrixBoundary::CpuReference,
        PlannedKernelMatrixBoundary::DeviceDifferential,
        PlannedKernelMatrixBoundary::BilingualDiagnostic,
        PlannedKernelMatrixBoundary::Unicode17,
        PlannedKernelMatrixBoundary::ProtocolInventory,
    ])
    .expect("bounded Kernel matrix evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.kernel-capability-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
