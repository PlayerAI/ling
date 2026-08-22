use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedKernelShapeBoundary {
    ShapeSchema,
    Dimension,
    Rank,
    Extent,
    Stride,
    Layout,
    IndexType,
    IndexArity,
    IndexOrigin,
    IndexNormalization,
    Slice,
    Gather,
    Scatter,
    Broadcast,
    Reshape,
    Transpose,
    BoundsCheck,
    LowerBound,
    UpperBound,
    NegativeIndexReject,
    OverflowCheck,
    DivisionByZero,
    OutOfRangeReject,
    EmptyShape,
    ZeroExtent,
    DynamicShape,
    StaticShape,
    SymbolicShape,
    ShapeInference,
    IndexInference,
    BoundsProof,
    AliasInteraction,
    RaceInteraction,
    BufferView,
    AddressSpace,
    OwnershipView,
    DeviceCapability,
    KernelProfile,
    TargetScope,
    TypedCoreInput,
    VerifiedDerivative,
    CanonicalOrdering,
    Provenance,
    Utf8Spans,
    SemanticId,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    Unicode17,
    PositiveFixture,
    NegativeFixture,
    GoldenFile,
    RoundTrip,
    UnknownShapeReject,
    UnknownIndexReject,
    Migration,
    CpuReference,
    DeviceDifferential,
    HostPathExclusion,
    ProtocolInventory,
}

impl PlannedKernelShapeBoundary {
    const ALL: [Self; 60] = [
        Self::ShapeSchema,
        Self::Dimension,
        Self::Rank,
        Self::Extent,
        Self::Stride,
        Self::Layout,
        Self::IndexType,
        Self::IndexArity,
        Self::IndexOrigin,
        Self::IndexNormalization,
        Self::Slice,
        Self::Gather,
        Self::Scatter,
        Self::Broadcast,
        Self::Reshape,
        Self::Transpose,
        Self::BoundsCheck,
        Self::LowerBound,
        Self::UpperBound,
        Self::NegativeIndexReject,
        Self::OverflowCheck,
        Self::DivisionByZero,
        Self::OutOfRangeReject,
        Self::EmptyShape,
        Self::ZeroExtent,
        Self::DynamicShape,
        Self::StaticShape,
        Self::SymbolicShape,
        Self::ShapeInference,
        Self::IndexInference,
        Self::BoundsProof,
        Self::AliasInteraction,
        Self::RaceInteraction,
        Self::BufferView,
        Self::AddressSpace,
        Self::OwnershipView,
        Self::DeviceCapability,
        Self::KernelProfile,
        Self::TargetScope,
        Self::TypedCoreInput,
        Self::VerifiedDerivative,
        Self::CanonicalOrdering,
        Self::Provenance,
        Self::Utf8Spans,
        Self::SemanticId,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::Unicode17,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::GoldenFile,
        Self::RoundTrip,
        Self::UnknownShapeReject,
        Self::UnknownIndexReject,
        Self::Migration,
        Self::CpuReference,
        Self::DeviceDifferential,
        Self::HostPathExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct KernelShapeInventory {
    boundaries: Box<[PlannedKernelShapeBoundary]>,
}

impl KernelShapeInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedKernelShapeBoundary>,
    ) -> Result<Self, PlannedKernelShapeBoundary> {
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
        bytes.extend_from_slice(b"ling.kernel-shape-index-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_kernel_shape_boundaries_are_complete_and_ordered() {
    let inventory = KernelShapeInventory::new(PlannedKernelShapeBoundary::ALL)
        .expect("planned Kernel shape boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedKernelShapeBoundary::ALL
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
fn kernel_shape_evidence_is_order_independent_and_duplicate_checked() {
    let forward = KernelShapeInventory::new(PlannedKernelShapeBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = KernelShapeInventory::new(PlannedKernelShapeBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = KernelShapeInventory::new([
        PlannedKernelShapeBoundary::ShapeSchema,
        PlannedKernelShapeBoundary::ShapeSchema,
    ])
    .expect_err("duplicate Kernel shape boundary must be rejected");
    assert_eq!(duplicate, PlannedKernelShapeBoundary::ShapeSchema);
}

#[test]
fn kernel_shape_evidence_has_no_bounds_authority() {
    let inventory = KernelShapeInventory::new([
        PlannedKernelShapeBoundary::ShapeSchema,
        PlannedKernelShapeBoundary::BoundsCheck,
        PlannedKernelShapeBoundary::BoundsProof,
        PlannedKernelShapeBoundary::TypedCoreInput,
        PlannedKernelShapeBoundary::CpuReference,
        PlannedKernelShapeBoundary::BilingualDiagnostic,
        PlannedKernelShapeBoundary::Unicode17,
        PlannedKernelShapeBoundary::ProtocolInventory,
    ])
    .expect("bounded Kernel shape evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.kernel-shape-index-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
