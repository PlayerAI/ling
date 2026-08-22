use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedForbiddenCapabilityBoundary {
    ForbiddenCapability,
    CriticalPolicy,
    CapabilityTaxonomy,
    EffectTaxonomy,
    ManagedGc,
    AllocationBound,
    Clock,
    Random,
    Io,
    Network,
    Device,
    DynamicLoading,
    Reflection,
    Shell,
    BuildStep,
    TaskTopology,
    ActorTopology,
    MailboxBound,
    FfiAudit,
    NumericDeterminism,
    PlacementDeterminism,
    FaultHandling,
    FallbackHandling,
    TypedCoreInput,
    LoweringBoundary,
    TransitiveSummary,
    TargetPackage,
    Forbidden,
    Unavailable,
    Unknown,
    Assumed,
    RuntimeChecked,
    Proved,
    Experimental,
    ConflictPrecedence,
    ProfileSelection,
    Migration,
    SemanticId,
    SourceSpan,
    DiagnosticCode,
    DiagnosticFacts,
    PathExclusion,
    AddressExclusion,
    TimestampExclusion,
    OwnershipExclusion,
    AllocationOrderExclusion,
    DebugOutputExclusion,
    PositiveFixture,
    NegativeFixture,
    TransitiveFixture,
    SourceSpanFixture,
    ProfileMatrixFixture,
    BoundFixture,
    EffectFixture,
    NumericFaultFixture,
    FfiTargetFixture,
    DeterminismFixture,
    DifferentialFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedForbiddenCapabilityBoundary {
    const ALL: [Self; 60] = [
        Self::ForbiddenCapability,
        Self::CriticalPolicy,
        Self::CapabilityTaxonomy,
        Self::EffectTaxonomy,
        Self::ManagedGc,
        Self::AllocationBound,
        Self::Clock,
        Self::Random,
        Self::Io,
        Self::Network,
        Self::Device,
        Self::DynamicLoading,
        Self::Reflection,
        Self::Shell,
        Self::BuildStep,
        Self::TaskTopology,
        Self::ActorTopology,
        Self::MailboxBound,
        Self::FfiAudit,
        Self::NumericDeterminism,
        Self::PlacementDeterminism,
        Self::FaultHandling,
        Self::FallbackHandling,
        Self::TypedCoreInput,
        Self::LoweringBoundary,
        Self::TransitiveSummary,
        Self::TargetPackage,
        Self::Forbidden,
        Self::Unavailable,
        Self::Unknown,
        Self::Assumed,
        Self::RuntimeChecked,
        Self::Proved,
        Self::Experimental,
        Self::ConflictPrecedence,
        Self::ProfileSelection,
        Self::Migration,
        Self::SemanticId,
        Self::SourceSpan,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::OwnershipExclusion,
        Self::AllocationOrderExclusion,
        Self::DebugOutputExclusion,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::TransitiveFixture,
        Self::SourceSpanFixture,
        Self::ProfileMatrixFixture,
        Self::BoundFixture,
        Self::EffectFixture,
        Self::NumericFaultFixture,
        Self::FfiTargetFixture,
        Self::DeterminismFixture,
        Self::DifferentialFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ForbiddenCapabilityInventory {
    boundaries: Box<[PlannedForbiddenCapabilityBoundary]>,
}

impl ForbiddenCapabilityInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedForbiddenCapabilityBoundary>,
    ) -> Result<Self, PlannedForbiddenCapabilityBoundary> {
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
        let mut bytes = b"ling.forbidden-capability-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_forbidden_capability_boundaries_are_complete_and_ordered() {
    let inventory = ForbiddenCapabilityInventory::new(PlannedForbiddenCapabilityBoundary::ALL)
        .expect("planned forbidden capability boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedForbiddenCapabilityBoundary::ALL
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
fn forbidden_capability_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ForbiddenCapabilityInventory::new(PlannedForbiddenCapabilityBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ForbiddenCapabilityInventory::new(
        PlannedForbiddenCapabilityBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ForbiddenCapabilityInventory::new([
        PlannedForbiddenCapabilityBoundary::ForbiddenCapability,
        PlannedForbiddenCapabilityBoundary::ForbiddenCapability,
    ])
    .expect_err("duplicate forbidden capability boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedForbiddenCapabilityBoundary::ForbiddenCapability
    );
}

#[test]
fn forbidden_capability_evidence_has_no_checker_authority() {
    let inventory = ForbiddenCapabilityInventory::new([
        PlannedForbiddenCapabilityBoundary::ForbiddenCapability,
        PlannedForbiddenCapabilityBoundary::CriticalPolicy,
        PlannedForbiddenCapabilityBoundary::TypedCoreInput,
        PlannedForbiddenCapabilityBoundary::Forbidden,
        PlannedForbiddenCapabilityBoundary::Unavailable,
        PlannedForbiddenCapabilityBoundary::Proved,
        PlannedForbiddenCapabilityBoundary::DiagnosticCode,
        PlannedForbiddenCapabilityBoundary::ProtocolInventory,
    ])
    .expect("bounded forbidden capability evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.forbidden-capability-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
