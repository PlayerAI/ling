use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedSimdDifferentialBoundary {
    DifferentialSchema,
    VerifiedInput,
    CheckedTypedCore,
    ScalarReference,
    SimdArtifact,
    CpuResult,
    SimdResult,
    IntegerExact,
    StructuralExact,
    StrictFloatingPoint,
    RoundingMode,
    Nan,
    Infinity,
    SignedZero,
    FloatOverflow,
    RelaxedFloatingPoint,
    ToleranceRule,
    ErrorMetric,
    ReductionOrder,
    DeterminismClass,
    Tail,
    UnalignedAccess,
    Alignment,
    Overflow,
    Fault,
    FaultIdentity,
    FaultSpan,
    EffectEquivalence,
    CommittedEffects,
    Cancellation,
    ResourceLimit,
    UnsupportedCapability,
    Mismatch,
    EvidenceFailure,
    TargetFeature,
    CapabilityEvidence,
    InputCorpus,
    SourceIdentity,
    ProgramIdentity,
    SemanticId,
    Utf8Spans,
    Unicode17,
    OutputCanonicalization,
    TraceReference,
    Redaction,
    CorruptionReject,
    VersionCompatibility,
    Migration,
    PositiveFixture,
    NegativeFixture,
    PropertyFixture,
    GoldenFile,
    CpuReferenceDifferential,
    CrossTargetDifferential,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    HostOutputExclusion,
    ProtocolInventory,
    PublicDifferentialBoundary,
}

impl PlannedSimdDifferentialBoundary {
    const ALL: [Self; 60] = [
        Self::DifferentialSchema,
        Self::VerifiedInput,
        Self::CheckedTypedCore,
        Self::ScalarReference,
        Self::SimdArtifact,
        Self::CpuResult,
        Self::SimdResult,
        Self::IntegerExact,
        Self::StructuralExact,
        Self::StrictFloatingPoint,
        Self::RoundingMode,
        Self::Nan,
        Self::Infinity,
        Self::SignedZero,
        Self::FloatOverflow,
        Self::RelaxedFloatingPoint,
        Self::ToleranceRule,
        Self::ErrorMetric,
        Self::ReductionOrder,
        Self::DeterminismClass,
        Self::Tail,
        Self::UnalignedAccess,
        Self::Alignment,
        Self::Overflow,
        Self::Fault,
        Self::FaultIdentity,
        Self::FaultSpan,
        Self::EffectEquivalence,
        Self::CommittedEffects,
        Self::Cancellation,
        Self::ResourceLimit,
        Self::UnsupportedCapability,
        Self::Mismatch,
        Self::EvidenceFailure,
        Self::TargetFeature,
        Self::CapabilityEvidence,
        Self::InputCorpus,
        Self::SourceIdentity,
        Self::ProgramIdentity,
        Self::SemanticId,
        Self::Utf8Spans,
        Self::Unicode17,
        Self::OutputCanonicalization,
        Self::TraceReference,
        Self::Redaction,
        Self::CorruptionReject,
        Self::VersionCompatibility,
        Self::Migration,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PropertyFixture,
        Self::GoldenFile,
        Self::CpuReferenceDifferential,
        Self::CrossTargetDifferential,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::HostOutputExclusion,
        Self::ProtocolInventory,
        Self::PublicDifferentialBoundary,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SimdDifferentialInventory {
    boundaries: Box<[PlannedSimdDifferentialBoundary]>,
}

impl SimdDifferentialInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedSimdDifferentialBoundary>,
    ) -> Result<Self, PlannedSimdDifferentialBoundary> {
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
        let mut bytes = b"ling.simd-differential-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_simd_differential_boundaries_are_complete_and_ordered() {
    let inventory = SimdDifferentialInventory::new(PlannedSimdDifferentialBoundary::ALL)
        .expect("planned SIMD differential boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedSimdDifferentialBoundary::ALL
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
fn simd_differential_evidence_is_order_independent_and_duplicate_checked() {
    let forward = SimdDifferentialInventory::new(PlannedSimdDifferentialBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        SimdDifferentialInventory::new(PlannedSimdDifferentialBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = SimdDifferentialInventory::new([
        PlannedSimdDifferentialBoundary::DifferentialSchema,
        PlannedSimdDifferentialBoundary::DifferentialSchema,
    ])
    .expect_err("duplicate SIMD differential boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedSimdDifferentialBoundary::DifferentialSchema
    );
}

#[test]
fn simd_differential_evidence_has_no_comparison_authority() {
    let inventory = SimdDifferentialInventory::new([
        PlannedSimdDifferentialBoundary::DifferentialSchema,
        PlannedSimdDifferentialBoundary::IntegerExact,
        PlannedSimdDifferentialBoundary::StrictFloatingPoint,
        PlannedSimdDifferentialBoundary::ToleranceRule,
        PlannedSimdDifferentialBoundary::FaultSpan,
        PlannedSimdDifferentialBoundary::CpuReferenceDifferential,
        PlannedSimdDifferentialBoundary::BilingualDiagnostic,
        PlannedSimdDifferentialBoundary::ProtocolInventory,
    ])
    .expect("bounded SIMD differential evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.simd-differential-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
