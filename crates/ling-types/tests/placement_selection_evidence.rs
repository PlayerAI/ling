use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedPlacementSelectionBoundary {
    SelectionPipeline,
    StaticLegality,
    CapabilityFilter,
    VerifiedArtifact,
    ArtifactPreparation,
    RuntimeAvailability,
    DeviceIdentity,
    TargetIdentity,
    CapabilityIdentity,
    ToolchainIdentity,
    FeatureVersion,
    TopologySnapshot,
    BufferLocation,
    RemoteBoundary,
    Policy,
    CostInput,
    CostEstimate,
    PolicyPrecedence,
    DeterministicTieBreak,
    Fallback,
    Rejection,
    Conflict,
    MissingDevice,
    Cancellation,
    ResourceLimit,
    Fault,
    CriticalProfile,
    StrictProfile,
    NativeProfile,
    FixedPlacement,
    RecordDecision,
    ReplayDecision,
    ReplayMismatch,
    StaleDecision,
    Migration,
    DecisionSchema,
    ExplainSchema,
    CacheIdentity,
    Provenance,
    Privacy,
    PathExclusion,
    AddressExclusion,
    TimestampExclusion,
    AllocationOrderExclusion,
    DriverTextExclusion,
    DebugOutputExclusion,
    PositiveFixture,
    NegativeFixture,
    TopologyFixture,
    CapabilityFixture,
    PolicyFixture,
    CostFixture,
    FallbackFixture,
    ReplayFixture,
    MigrationFixture,
    DifferentialFixture,
    UnicodeFixture,
    DeterminismFixture,
    DiagnosticCode,
    ProtocolInventory,
}

impl PlannedPlacementSelectionBoundary {
    const ALL: [Self; 60] = [
        Self::SelectionPipeline,
        Self::StaticLegality,
        Self::CapabilityFilter,
        Self::VerifiedArtifact,
        Self::ArtifactPreparation,
        Self::RuntimeAvailability,
        Self::DeviceIdentity,
        Self::TargetIdentity,
        Self::CapabilityIdentity,
        Self::ToolchainIdentity,
        Self::FeatureVersion,
        Self::TopologySnapshot,
        Self::BufferLocation,
        Self::RemoteBoundary,
        Self::Policy,
        Self::CostInput,
        Self::CostEstimate,
        Self::PolicyPrecedence,
        Self::DeterministicTieBreak,
        Self::Fallback,
        Self::Rejection,
        Self::Conflict,
        Self::MissingDevice,
        Self::Cancellation,
        Self::ResourceLimit,
        Self::Fault,
        Self::CriticalProfile,
        Self::StrictProfile,
        Self::NativeProfile,
        Self::FixedPlacement,
        Self::RecordDecision,
        Self::ReplayDecision,
        Self::ReplayMismatch,
        Self::StaleDecision,
        Self::Migration,
        Self::DecisionSchema,
        Self::ExplainSchema,
        Self::CacheIdentity,
        Self::Provenance,
        Self::Privacy,
        Self::PathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::AllocationOrderExclusion,
        Self::DriverTextExclusion,
        Self::DebugOutputExclusion,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::TopologyFixture,
        Self::CapabilityFixture,
        Self::PolicyFixture,
        Self::CostFixture,
        Self::FallbackFixture,
        Self::ReplayFixture,
        Self::MigrationFixture,
        Self::DifferentialFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::DiagnosticCode,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlacementSelectionInventory {
    boundaries: Box<[PlannedPlacementSelectionBoundary]>,
}

impl PlacementSelectionInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedPlacementSelectionBoundary>,
    ) -> Result<Self, PlannedPlacementSelectionBoundary> {
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
        let mut bytes = b"ling.placement-selection-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_placement_selection_boundaries_are_complete_and_ordered() {
    let inventory = PlacementSelectionInventory::new(PlannedPlacementSelectionBoundary::ALL)
        .expect("planned placement selection boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedPlacementSelectionBoundary::ALL
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
fn placement_selection_evidence_is_order_independent_and_duplicate_checked() {
    let forward = PlacementSelectionInventory::new(PlannedPlacementSelectionBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        PlacementSelectionInventory::new(PlannedPlacementSelectionBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = PlacementSelectionInventory::new([
        PlannedPlacementSelectionBoundary::SelectionPipeline,
        PlannedPlacementSelectionBoundary::SelectionPipeline,
    ])
    .expect_err("duplicate placement selection boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedPlacementSelectionBoundary::SelectionPipeline
    );
}

#[test]
fn placement_selection_evidence_has_no_selector_authority() {
    let inventory = PlacementSelectionInventory::new([
        PlannedPlacementSelectionBoundary::SelectionPipeline,
        PlannedPlacementSelectionBoundary::StaticLegality,
        PlannedPlacementSelectionBoundary::RuntimeAvailability,
        PlannedPlacementSelectionBoundary::Policy,
        PlannedPlacementSelectionBoundary::RecordDecision,
        PlannedPlacementSelectionBoundary::ReplayDecision,
        PlannedPlacementSelectionBoundary::CriticalProfile,
        PlannedPlacementSelectionBoundary::ProtocolInventory,
    ])
    .expect("bounded placement selection evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.placement-selection-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
