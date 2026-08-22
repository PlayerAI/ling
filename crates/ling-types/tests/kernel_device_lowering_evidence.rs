use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedKernelDeviceLoweringBoundary {
    LoweringSchema,
    KernelSource,
    KernelProfile,
    TypedCoreInput,
    VerifiedDerivative,
    Elementwise,
    Index,
    Shape,
    Bounds,
    LocalMemory,
    Reduction,
    VectorOp,
    TensorOp,
    Synchronization,
    SourceDiagnosticMap,
    ControlFlow,
    KernelType,
    KernelEffect,
    EffectWitness,
    Ownership,
    OwnershipWitness,
    AliasProof,
    ProofWitness,
    SynchronizationWitness,
    MemoryOp,
    AddressSpace,
    Layout,
    NumericMode,
    Determinism,
    Fault,
    Cancellation,
    Capability,
    CapabilityWitness,
    RequiredFeature,
    UnsupportedTarget,
    TargetScope,
    Fallback,
    Rejection,
    LoweringPrecondition,
    LoweringPostcondition,
    Provenance,
    SemanticId,
    SourceSpan,
    Utf8Spans,
    ResourceLimit,
    CanonicalOrdering,
    VersionCompatibility,
    PositiveFixture,
    NegativeFixture,
    PropertyFixture,
    CorruptionFixture,
    Migration,
    CpuReference,
    DifferentialResult,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    HostPathExclusion,
    DriverLogExclusion,
    ProtocolInventory,
}

impl PlannedKernelDeviceLoweringBoundary {
    const ALL: [Self; 60] = [
        Self::LoweringSchema,
        Self::KernelSource,
        Self::KernelProfile,
        Self::TypedCoreInput,
        Self::VerifiedDerivative,
        Self::Elementwise,
        Self::Index,
        Self::Shape,
        Self::Bounds,
        Self::LocalMemory,
        Self::Reduction,
        Self::VectorOp,
        Self::TensorOp,
        Self::Synchronization,
        Self::SourceDiagnosticMap,
        Self::ControlFlow,
        Self::KernelType,
        Self::KernelEffect,
        Self::EffectWitness,
        Self::Ownership,
        Self::OwnershipWitness,
        Self::AliasProof,
        Self::ProofWitness,
        Self::SynchronizationWitness,
        Self::MemoryOp,
        Self::AddressSpace,
        Self::Layout,
        Self::NumericMode,
        Self::Determinism,
        Self::Fault,
        Self::Cancellation,
        Self::Capability,
        Self::CapabilityWitness,
        Self::RequiredFeature,
        Self::UnsupportedTarget,
        Self::TargetScope,
        Self::Fallback,
        Self::Rejection,
        Self::LoweringPrecondition,
        Self::LoweringPostcondition,
        Self::Provenance,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Utf8Spans,
        Self::ResourceLimit,
        Self::CanonicalOrdering,
        Self::VersionCompatibility,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PropertyFixture,
        Self::CorruptionFixture,
        Self::Migration,
        Self::CpuReference,
        Self::DifferentialResult,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::HostPathExclusion,
        Self::DriverLogExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct KernelDeviceLoweringInventory {
    boundaries: Box<[PlannedKernelDeviceLoweringBoundary]>,
}

impl KernelDeviceLoweringInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedKernelDeviceLoweringBoundary>,
    ) -> Result<Self, PlannedKernelDeviceLoweringBoundary> {
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
        let mut bytes = b"ling.kernel-device-lowering-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_kernel_device_lowering_boundaries_are_complete_and_ordered() {
    let inventory = KernelDeviceLoweringInventory::new(PlannedKernelDeviceLoweringBoundary::ALL)
        .expect("planned Kernel-to-Device lowering boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedKernelDeviceLoweringBoundary::ALL
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
fn kernel_device_lowering_evidence_is_order_independent_and_duplicate_checked() {
    let forward = KernelDeviceLoweringInventory::new(PlannedKernelDeviceLoweringBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = KernelDeviceLoweringInventory::new(
        PlannedKernelDeviceLoweringBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = KernelDeviceLoweringInventory::new([
        PlannedKernelDeviceLoweringBoundary::LoweringSchema,
        PlannedKernelDeviceLoweringBoundary::LoweringSchema,
    ])
    .expect_err("duplicate Kernel-to-Device lowering boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedKernelDeviceLoweringBoundary::LoweringSchema
    );
}

#[test]
fn kernel_device_lowering_evidence_has_no_lowering_authority() {
    let inventory = KernelDeviceLoweringInventory::new([
        PlannedKernelDeviceLoweringBoundary::LoweringSchema,
        PlannedKernelDeviceLoweringBoundary::TypedCoreInput,
        PlannedKernelDeviceLoweringBoundary::Elementwise,
        PlannedKernelDeviceLoweringBoundary::Reduction,
        PlannedKernelDeviceLoweringBoundary::Synchronization,
        PlannedKernelDeviceLoweringBoundary::Rejection,
        PlannedKernelDeviceLoweringBoundary::BilingualDiagnostic,
        PlannedKernelDeviceLoweringBoundary::ProtocolInventory,
    ])
    .expect("bounded Kernel-to-Device lowering evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.kernel-device-lowering-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
