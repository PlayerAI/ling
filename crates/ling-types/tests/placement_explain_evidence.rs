use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedPlacementExplainBoundary {
    ExplainOutput,
    CandidateDevices,
    RejectedReasons,
    ChosenDevice,
    Transfers,
    NumericMode,
    Fallback,
    CacheHit,
    CacheMiss,
    RecordReplayIdentity,
    DecisionIdentity,
    SemanticId,
    SourceSpan,
    Provenance,
    RejectionTaxonomy,
    CandidateOrder,
    ChosenIdentity,
    TransferBytes,
    TransferDirection,
    NumericClass,
    FallbackReason,
    CacheIdentity,
    ReplayVersion,
    Profile,
    ProtocolVersion,
    CanonicalOrdering,
    StableField,
    DiagnosticOnlyField,
    BilingualRendering,
    JsonTransport,
    ExitBehavior,
    UnknownFieldPolicy,
    Migration,
    Privacy,
    PathExclusion,
    AddressExclusion,
    TimestampExclusion,
    AllocationOrderExclusion,
    DriverTextExclusion,
    DebugOutputExclusion,
    SolverExclusion,
    LingCli,
    LingSourceExtension,
    PositiveFixture,
    NegativeFixture,
    PrivacyFixture,
    MigrationFixture,
    ReplayFixture,
    ExplainFixture,
    DifferentialFixture,
    TopologyFixture,
    CapabilityFixture,
    PolicyFixture,
    CostFixture,
    FallbackFixture,
    UnicodeFixture,
    DeterminismFixture,
    DiagnosticCode,
    ProtocolInventory,
    SupportExclusion,
}

impl PlannedPlacementExplainBoundary {
    const ALL: [Self; 60] = [
        Self::ExplainOutput,
        Self::CandidateDevices,
        Self::RejectedReasons,
        Self::ChosenDevice,
        Self::Transfers,
        Self::NumericMode,
        Self::Fallback,
        Self::CacheHit,
        Self::CacheMiss,
        Self::RecordReplayIdentity,
        Self::DecisionIdentity,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Provenance,
        Self::RejectionTaxonomy,
        Self::CandidateOrder,
        Self::ChosenIdentity,
        Self::TransferBytes,
        Self::TransferDirection,
        Self::NumericClass,
        Self::FallbackReason,
        Self::CacheIdentity,
        Self::ReplayVersion,
        Self::Profile,
        Self::ProtocolVersion,
        Self::CanonicalOrdering,
        Self::StableField,
        Self::DiagnosticOnlyField,
        Self::BilingualRendering,
        Self::JsonTransport,
        Self::ExitBehavior,
        Self::UnknownFieldPolicy,
        Self::Migration,
        Self::Privacy,
        Self::PathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::AllocationOrderExclusion,
        Self::DriverTextExclusion,
        Self::DebugOutputExclusion,
        Self::SolverExclusion,
        Self::LingCli,
        Self::LingSourceExtension,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PrivacyFixture,
        Self::MigrationFixture,
        Self::ReplayFixture,
        Self::ExplainFixture,
        Self::DifferentialFixture,
        Self::TopologyFixture,
        Self::CapabilityFixture,
        Self::PolicyFixture,
        Self::CostFixture,
        Self::FallbackFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::DiagnosticCode,
        Self::ProtocolInventory,
        Self::SupportExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlacementExplainInventory {
    boundaries: Box<[PlannedPlacementExplainBoundary]>,
}

impl PlacementExplainInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedPlacementExplainBoundary>,
    ) -> Result<Self, PlannedPlacementExplainBoundary> {
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
        let mut bytes = b"ling.placement-explain-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_placement_explain_boundaries_are_complete_and_ordered() {
    let inventory = PlacementExplainInventory::new(PlannedPlacementExplainBoundary::ALL)
        .expect("planned placement explain boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedPlacementExplainBoundary::ALL
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
fn placement_explain_evidence_is_order_independent_and_duplicate_checked() {
    let forward = PlacementExplainInventory::new(PlannedPlacementExplainBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        PlacementExplainInventory::new(PlannedPlacementExplainBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = PlacementExplainInventory::new([
        PlannedPlacementExplainBoundary::ExplainOutput,
        PlannedPlacementExplainBoundary::ExplainOutput,
    ])
    .expect_err("duplicate placement explain boundary must be rejected");
    assert_eq!(duplicate, PlannedPlacementExplainBoundary::ExplainOutput);
}

#[test]
fn placement_explain_evidence_has_no_cli_authority() {
    let inventory = PlacementExplainInventory::new([
        PlannedPlacementExplainBoundary::ExplainOutput,
        PlannedPlacementExplainBoundary::CandidateDevices,
        PlannedPlacementExplainBoundary::RejectedReasons,
        PlannedPlacementExplainBoundary::ChosenDevice,
        PlannedPlacementExplainBoundary::BilingualRendering,
        PlannedPlacementExplainBoundary::LingCli,
        PlannedPlacementExplainBoundary::SolverExclusion,
        PlannedPlacementExplainBoundary::ProtocolInventory,
    ])
    .expect("bounded placement explain evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.placement-explain-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
