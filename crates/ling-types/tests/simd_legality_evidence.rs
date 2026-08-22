use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedSimdLegalityBoundary {
    LegalitySchema,
    CheckedTypedCore,
    VerifiedDerivative,
    IndependentIterations,
    Dependence,
    LoopInvariant,
    IterationSpace,
    ShapeWitness,
    IndexWitness,
    BoundsWitness,
    AliasWitness,
    OwnershipWitness,
    MutationWitness,
    EffectWitness,
    ReductionWitness,
    Alignment,
    VectorWidth,
    TailStrategy,
    Overflow,
    IntegerSemantics,
    StrictFloatingPoint,
    RelaxedFloatingPoint,
    NanInfinity,
    ReductionOrder,
    Determinism,
    TargetFeature,
    FeatureIdentity,
    FeatureNegotiation,
    UnsupportedWidth,
    UnsupportedFeature,
    FallbackAllowed,
    FallbackReason,
    FallbackEvidence,
    ScalarEquivalence,
    CpuReference,
    Differential,
    ToleranceRule,
    CanonicalOrdering,
    Provenance,
    SourceSpan,
    SemanticId,
    Utf8Spans,
    Unicode17,
    PositiveFixture,
    NegativeFixture,
    PropertyFixture,
    CorruptionFixture,
    Migration,
    GoldenFile,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    ResourceLimit,
    HostPathExclusion,
    AddressExclusion,
    TimingExclusion,
    DriverLogExclusion,
    ProtocolInventory,
    PublicLegalityBoundary,
    TargetBackendBoundary,
}

impl PlannedSimdLegalityBoundary {
    const ALL: [Self; 60] = [
        Self::LegalitySchema,
        Self::CheckedTypedCore,
        Self::VerifiedDerivative,
        Self::IndependentIterations,
        Self::Dependence,
        Self::LoopInvariant,
        Self::IterationSpace,
        Self::ShapeWitness,
        Self::IndexWitness,
        Self::BoundsWitness,
        Self::AliasWitness,
        Self::OwnershipWitness,
        Self::MutationWitness,
        Self::EffectWitness,
        Self::ReductionWitness,
        Self::Alignment,
        Self::VectorWidth,
        Self::TailStrategy,
        Self::Overflow,
        Self::IntegerSemantics,
        Self::StrictFloatingPoint,
        Self::RelaxedFloatingPoint,
        Self::NanInfinity,
        Self::ReductionOrder,
        Self::Determinism,
        Self::TargetFeature,
        Self::FeatureIdentity,
        Self::FeatureNegotiation,
        Self::UnsupportedWidth,
        Self::UnsupportedFeature,
        Self::FallbackAllowed,
        Self::FallbackReason,
        Self::FallbackEvidence,
        Self::ScalarEquivalence,
        Self::CpuReference,
        Self::Differential,
        Self::ToleranceRule,
        Self::CanonicalOrdering,
        Self::Provenance,
        Self::SourceSpan,
        Self::SemanticId,
        Self::Utf8Spans,
        Self::Unicode17,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PropertyFixture,
        Self::CorruptionFixture,
        Self::Migration,
        Self::GoldenFile,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::ResourceLimit,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::TimingExclusion,
        Self::DriverLogExclusion,
        Self::ProtocolInventory,
        Self::PublicLegalityBoundary,
        Self::TargetBackendBoundary,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SimdLegalityInventory {
    boundaries: Box<[PlannedSimdLegalityBoundary]>,
}

impl SimdLegalityInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedSimdLegalityBoundary>,
    ) -> Result<Self, PlannedSimdLegalityBoundary> {
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
        let mut bytes = b"ling.simd-legality-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_simd_legality_boundaries_are_complete_and_ordered() {
    let inventory = SimdLegalityInventory::new(PlannedSimdLegalityBoundary::ALL)
        .expect("planned SIMD legality boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedSimdLegalityBoundary::ALL
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
fn simd_legality_evidence_is_order_independent_and_duplicate_checked() {
    let forward = SimdLegalityInventory::new(PlannedSimdLegalityBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = SimdLegalityInventory::new(PlannedSimdLegalityBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = SimdLegalityInventory::new([
        PlannedSimdLegalityBoundary::LegalitySchema,
        PlannedSimdLegalityBoundary::LegalitySchema,
    ])
    .expect_err("duplicate SIMD legality boundary must be rejected");
    assert_eq!(duplicate, PlannedSimdLegalityBoundary::LegalitySchema);
}

#[test]
fn simd_legality_evidence_has_no_optimizer_authority() {
    let inventory = SimdLegalityInventory::new([
        PlannedSimdLegalityBoundary::LegalitySchema,
        PlannedSimdLegalityBoundary::IndependentIterations,
        PlannedSimdLegalityBoundary::Alignment,
        PlannedSimdLegalityBoundary::FallbackReason,
        PlannedSimdLegalityBoundary::StrictFloatingPoint,
        PlannedSimdLegalityBoundary::CpuReference,
        PlannedSimdLegalityBoundary::BilingualDiagnostic,
        PlannedSimdLegalityBoundary::ProtocolInventory,
    ])
    .expect("bounded SIMD legality evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.simd-legality-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
