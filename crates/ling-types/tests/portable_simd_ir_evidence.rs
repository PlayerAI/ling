use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedPortableSimdIrBoundary {
    PortableSimdIrSchema,
    CheckedTypedCore,
    VerifiedDerivative,
    LaneType,
    LaneCount,
    VectorValue,
    VectorLoad,
    VectorStore,
    Addressing,
    Alignment,
    Bounds,
    Alias,
    Mask,
    MaskTruth,
    Shuffle,
    ShuffleIndex,
    HorizontalReduction,
    ReductionOrder,
    Scalarization,
    Fallback,
    ElementEffect,
    MemoryEffect,
    Fault,
    Shape,
    Layout,
    Index,
    Overflow,
    StrictFloatingPoint,
    RelaxedFloatingPoint,
    Determinism,
    TargetCapability,
    FeatureIdentity,
    RequiredFeature,
    OptionalFeature,
    UnsupportedTarget,
    CanonicalOrdering,
    SemanticId,
    SourceSpan,
    Utf8Spans,
    Unicode17,
    Version,
    UnknownOpcodeReject,
    MalformedInstructionReject,
    ResourceLimit,
    PositiveFixture,
    NegativeFixture,
    PropertyFixture,
    CorruptionFixture,
    Migration,
    RoundTrip,
    GoldenFile,
    CpuReference,
    Differential,
    ToleranceRule,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    HostOutputExclusion,
    ProtocolInventory,
    PublicIrBoundary,
}

impl PlannedPortableSimdIrBoundary {
    const ALL: [Self; 60] = [
        Self::PortableSimdIrSchema,
        Self::CheckedTypedCore,
        Self::VerifiedDerivative,
        Self::LaneType,
        Self::LaneCount,
        Self::VectorValue,
        Self::VectorLoad,
        Self::VectorStore,
        Self::Addressing,
        Self::Alignment,
        Self::Bounds,
        Self::Alias,
        Self::Mask,
        Self::MaskTruth,
        Self::Shuffle,
        Self::ShuffleIndex,
        Self::HorizontalReduction,
        Self::ReductionOrder,
        Self::Scalarization,
        Self::Fallback,
        Self::ElementEffect,
        Self::MemoryEffect,
        Self::Fault,
        Self::Shape,
        Self::Layout,
        Self::Index,
        Self::Overflow,
        Self::StrictFloatingPoint,
        Self::RelaxedFloatingPoint,
        Self::Determinism,
        Self::TargetCapability,
        Self::FeatureIdentity,
        Self::RequiredFeature,
        Self::OptionalFeature,
        Self::UnsupportedTarget,
        Self::CanonicalOrdering,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Utf8Spans,
        Self::Unicode17,
        Self::Version,
        Self::UnknownOpcodeReject,
        Self::MalformedInstructionReject,
        Self::ResourceLimit,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PropertyFixture,
        Self::CorruptionFixture,
        Self::Migration,
        Self::RoundTrip,
        Self::GoldenFile,
        Self::CpuReference,
        Self::Differential,
        Self::ToleranceRule,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::HostOutputExclusion,
        Self::ProtocolInventory,
        Self::PublicIrBoundary,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PortableSimdIrInventory {
    boundaries: Box<[PlannedPortableSimdIrBoundary]>,
}

impl PortableSimdIrInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedPortableSimdIrBoundary>,
    ) -> Result<Self, PlannedPortableSimdIrBoundary> {
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
        let mut bytes = b"ling.portable-simd-ir-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_portable_simd_ir_boundaries_are_complete_and_ordered() {
    let inventory = PortableSimdIrInventory::new(PlannedPortableSimdIrBoundary::ALL)
        .expect("planned portable SIMD IR boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedPortableSimdIrBoundary::ALL
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
fn portable_simd_ir_evidence_is_order_independent_and_duplicate_checked() {
    let forward = PortableSimdIrInventory::new(PlannedPortableSimdIrBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        PortableSimdIrInventory::new(PlannedPortableSimdIrBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = PortableSimdIrInventory::new([
        PlannedPortableSimdIrBoundary::PortableSimdIrSchema,
        PlannedPortableSimdIrBoundary::PortableSimdIrSchema,
    ])
    .expect_err("duplicate portable SIMD IR boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedPortableSimdIrBoundary::PortableSimdIrSchema
    );
}

#[test]
fn portable_simd_ir_evidence_has_no_ir_authority() {
    let inventory = PortableSimdIrInventory::new([
        PlannedPortableSimdIrBoundary::PortableSimdIrSchema,
        PlannedPortableSimdIrBoundary::LaneType,
        PlannedPortableSimdIrBoundary::VectorLoad,
        PlannedPortableSimdIrBoundary::MaskTruth,
        PlannedPortableSimdIrBoundary::Scalarization,
        PlannedPortableSimdIrBoundary::CpuReference,
        PlannedPortableSimdIrBoundary::BilingualDiagnostic,
        PlannedPortableSimdIrBoundary::ProtocolInventory,
    ])
    .expect("bounded portable SIMD IR evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.portable-simd-ir-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
