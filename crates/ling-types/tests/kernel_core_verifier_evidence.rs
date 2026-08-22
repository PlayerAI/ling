use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedKernelCoreVerifierBoundary {
    KernelCoreSchema,
    CheckedTypedCore,
    VerifiedDerivative,
    CoreNodeIdentity,
    CoreSpan,
    SemanticId,
    EffectWitness,
    CapabilityWitness,
    ShapeWitness,
    BoundsWitness,
    AliasWitness,
    RaceWitness,
    OwnershipWitness,
    ResourceWitness,
    ManagedWitness,
    ProfileWitness,
    TargetWitness,
    DeviceWitness,
    TypeRule,
    ValueRule,
    AggregateRule,
    ControlFlowRule,
    LoopRule,
    CallRule,
    RecursionRule,
    StaticDispatchRule,
    EffectRule,
    CapabilityRule,
    BoundsRule,
    NumericRule,
    DeterminismRule,
    UnsupportedNode,
    InvalidWitness,
    MissingWitness,
    ConflictingWitness,
    RejectionCategory,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    Utf8Spans,
    Unicode17,
    CanonicalOrdering,
    Provenance,
    GraphProjection,
    AuditProjection,
    GoldenFile,
    RoundTrip,
    Migration,
    PositiveFixture,
    NegativeFixture,
    UnknownRuleReject,
    VersionCompatibility,
    CpuReference,
    DeviceDifferential,
    BackendBoundary,
    HostPathExclusion,
    AddressExclusion,
    DriverLogExclusion,
    ProtocolInventory,
    PublicSchemaBoundary,
}

impl PlannedKernelCoreVerifierBoundary {
    const ALL: [Self; 60] = [
        Self::KernelCoreSchema,
        Self::CheckedTypedCore,
        Self::VerifiedDerivative,
        Self::CoreNodeIdentity,
        Self::CoreSpan,
        Self::SemanticId,
        Self::EffectWitness,
        Self::CapabilityWitness,
        Self::ShapeWitness,
        Self::BoundsWitness,
        Self::AliasWitness,
        Self::RaceWitness,
        Self::OwnershipWitness,
        Self::ResourceWitness,
        Self::ManagedWitness,
        Self::ProfileWitness,
        Self::TargetWitness,
        Self::DeviceWitness,
        Self::TypeRule,
        Self::ValueRule,
        Self::AggregateRule,
        Self::ControlFlowRule,
        Self::LoopRule,
        Self::CallRule,
        Self::RecursionRule,
        Self::StaticDispatchRule,
        Self::EffectRule,
        Self::CapabilityRule,
        Self::BoundsRule,
        Self::NumericRule,
        Self::DeterminismRule,
        Self::UnsupportedNode,
        Self::InvalidWitness,
        Self::MissingWitness,
        Self::ConflictingWitness,
        Self::RejectionCategory,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::Utf8Spans,
        Self::Unicode17,
        Self::CanonicalOrdering,
        Self::Provenance,
        Self::GraphProjection,
        Self::AuditProjection,
        Self::GoldenFile,
        Self::RoundTrip,
        Self::Migration,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::UnknownRuleReject,
        Self::VersionCompatibility,
        Self::CpuReference,
        Self::DeviceDifferential,
        Self::BackendBoundary,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::DriverLogExclusion,
        Self::ProtocolInventory,
        Self::PublicSchemaBoundary,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct KernelCoreVerifierInventory {
    boundaries: Box<[PlannedKernelCoreVerifierBoundary]>,
}

impl KernelCoreVerifierInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedKernelCoreVerifierBoundary>,
    ) -> Result<Self, PlannedKernelCoreVerifierBoundary> {
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
        let mut bytes = b"ling.kernel-core-verifier-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_kernel_core_verifier_boundaries_are_complete_and_ordered() {
    let inventory = KernelCoreVerifierInventory::new(PlannedKernelCoreVerifierBoundary::ALL)
        .expect("planned Kernel Core/verifier boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedKernelCoreVerifierBoundary::ALL
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
fn kernel_core_verifier_evidence_is_order_independent_and_duplicate_checked() {
    let forward = KernelCoreVerifierInventory::new(PlannedKernelCoreVerifierBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        KernelCoreVerifierInventory::new(PlannedKernelCoreVerifierBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = KernelCoreVerifierInventory::new([
        PlannedKernelCoreVerifierBoundary::KernelCoreSchema,
        PlannedKernelCoreVerifierBoundary::KernelCoreSchema,
    ])
    .expect_err("duplicate Kernel Core/verifier boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedKernelCoreVerifierBoundary::KernelCoreSchema
    );
}

#[test]
fn kernel_core_verifier_evidence_has_no_checker_authority() {
    let inventory = KernelCoreVerifierInventory::new([
        PlannedKernelCoreVerifierBoundary::KernelCoreSchema,
        PlannedKernelCoreVerifierBoundary::CheckedTypedCore,
        PlannedKernelCoreVerifierBoundary::InvalidWitness,
        PlannedKernelCoreVerifierBoundary::DeterminismRule,
        PlannedKernelCoreVerifierBoundary::CpuReference,
        PlannedKernelCoreVerifierBoundary::BilingualDiagnostic,
        PlannedKernelCoreVerifierBoundary::Unicode17,
        PlannedKernelCoreVerifierBoundary::ProtocolInventory,
    ])
    .expect("bounded Kernel Core/verifier evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.kernel-core-verifier-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
