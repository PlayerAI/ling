use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedCpuScalarReferenceBoundary {
    ScalarReferenceSchema,
    VerifiedKernelInput,
    CheckedTypedCore,
    ElementWiseMap,
    MultiDimIndex,
    IndexNormalization,
    Conditional,
    BoundedLoop,
    BufferRead,
    BufferWrite,
    ShapeExtent,
    Stride,
    BoundsCheck,
    BoundsReject,
    AliasCheck,
    RaceCheck,
    Reduction,
    ReductionOrder,
    Atomic,
    Barrier,
    EffectWitness,
    CapabilityWitness,
    OwnershipWitness,
    ResourceLimit,
    Allocation,
    Cancellation,
    Fault,
    FaultKind,
    FaultSpan,
    FaultSemanticId,
    NumericMode,
    IntegerSemantics,
    FloatSemantics,
    NanInfinity,
    Overflow,
    Determinism,
    OutputValue,
    OutputShape,
    ReferenceTrace,
    CanonicalBytes,
    Provenance,
    Utf8Spans,
    SemanticId,
    Unicode17,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    PositiveFixture,
    NegativeFixture,
    CorruptionFixture,
    RoundTrip,
    Migration,
    CpuDifferential,
    DeviceDifferential,
    ToleranceRule,
    UnsupportedTarget,
    HostPathExclusion,
    AddressExclusion,
    DriverLogExclusion,
    ProtocolInventory,
}

impl PlannedCpuScalarReferenceBoundary {
    const ALL: [Self; 60] = [
        Self::ScalarReferenceSchema,
        Self::VerifiedKernelInput,
        Self::CheckedTypedCore,
        Self::ElementWiseMap,
        Self::MultiDimIndex,
        Self::IndexNormalization,
        Self::Conditional,
        Self::BoundedLoop,
        Self::BufferRead,
        Self::BufferWrite,
        Self::ShapeExtent,
        Self::Stride,
        Self::BoundsCheck,
        Self::BoundsReject,
        Self::AliasCheck,
        Self::RaceCheck,
        Self::Reduction,
        Self::ReductionOrder,
        Self::Atomic,
        Self::Barrier,
        Self::EffectWitness,
        Self::CapabilityWitness,
        Self::OwnershipWitness,
        Self::ResourceLimit,
        Self::Allocation,
        Self::Cancellation,
        Self::Fault,
        Self::FaultKind,
        Self::FaultSpan,
        Self::FaultSemanticId,
        Self::NumericMode,
        Self::IntegerSemantics,
        Self::FloatSemantics,
        Self::NanInfinity,
        Self::Overflow,
        Self::Determinism,
        Self::OutputValue,
        Self::OutputShape,
        Self::ReferenceTrace,
        Self::CanonicalBytes,
        Self::Provenance,
        Self::Utf8Spans,
        Self::SemanticId,
        Self::Unicode17,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CorruptionFixture,
        Self::RoundTrip,
        Self::Migration,
        Self::CpuDifferential,
        Self::DeviceDifferential,
        Self::ToleranceRule,
        Self::UnsupportedTarget,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::DriverLogExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CpuScalarReferenceInventory {
    boundaries: Box<[PlannedCpuScalarReferenceBoundary]>,
}

impl CpuScalarReferenceInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCpuScalarReferenceBoundary>,
    ) -> Result<Self, PlannedCpuScalarReferenceBoundary> {
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
        let mut bytes = b"ling.cpu-scalar-reference-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_cpu_scalar_reference_boundaries_are_complete_and_ordered() {
    let inventory = CpuScalarReferenceInventory::new(PlannedCpuScalarReferenceBoundary::ALL)
        .expect("planned CPU scalar-reference boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedCpuScalarReferenceBoundary::ALL
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
fn cpu_scalar_reference_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CpuScalarReferenceInventory::new(PlannedCpuScalarReferenceBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        CpuScalarReferenceInventory::new(PlannedCpuScalarReferenceBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CpuScalarReferenceInventory::new([
        PlannedCpuScalarReferenceBoundary::ScalarReferenceSchema,
        PlannedCpuScalarReferenceBoundary::ScalarReferenceSchema,
    ])
    .expect_err("duplicate CPU scalar-reference boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedCpuScalarReferenceBoundary::ScalarReferenceSchema
    );
}

#[test]
fn cpu_scalar_reference_evidence_has_no_backend_authority() {
    let inventory = CpuScalarReferenceInventory::new([
        PlannedCpuScalarReferenceBoundary::ScalarReferenceSchema,
        PlannedCpuScalarReferenceBoundary::VerifiedKernelInput,
        PlannedCpuScalarReferenceBoundary::BoundsReject,
        PlannedCpuScalarReferenceBoundary::ReductionOrder,
        PlannedCpuScalarReferenceBoundary::Fault,
        PlannedCpuScalarReferenceBoundary::CpuDifferential,
        PlannedCpuScalarReferenceBoundary::BilingualDiagnostic,
        PlannedCpuScalarReferenceBoundary::ProtocolInventory,
    ])
    .expect("bounded CPU scalar-reference evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.cpu-scalar-reference-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
