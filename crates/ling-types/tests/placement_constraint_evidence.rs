use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedPlacementConstraintBoundary {
    PlacementConstraint,
    RequiresGpu,
    PrefersGpu,
    ForbidsRemote,
    SameNodeAs,
    NearBuffer,
    FallbackCpu,
    HardConstraint,
    SoftConstraint,
    CapabilityPredicate,
    DeviceIdentity,
    Topology,
    BufferLocation,
    AddressSpace,
    RemoteBoundary,
    Ownership,
    Transfer,
    Synchronization,
    NumericClass,
    Effects,
    Fault,
    Availability,
    Cost,
    CostLimit,
    DeterministicTieBreak,
    StaticFilter,
    RuntimeSelection,
    Rejection,
    FallbackLegality,
    UserIntent,
    Explain,
    Replay,
    DecisionSchema,
    CacheIdentity,
    Provenance,
    Versioning,
    Migration,
    Corruption,
    Privacy,
    PathExclusion,
    AddressExclusion,
    TimestampExclusion,
    AllocationOrderExclusion,
    DriverTextExclusion,
    DebugOutputExclusion,
    PositiveFixture,
    NegativeFixture,
    ConflictFixture,
    UnsatisfiableFixture,
    TopologyFixture,
    CapabilityFixture,
    FallbackFixture,
    ExplainFixture,
    ReplayFixture,
    CacheFixture,
    DifferentialFixture,
    UnicodeFixture,
    DeterminismFixture,
    DiagnosticCode,
    ProtocolInventory,
}

impl PlannedPlacementConstraintBoundary {
    const ALL: [Self; 60] = [
        Self::PlacementConstraint,
        Self::RequiresGpu,
        Self::PrefersGpu,
        Self::ForbidsRemote,
        Self::SameNodeAs,
        Self::NearBuffer,
        Self::FallbackCpu,
        Self::HardConstraint,
        Self::SoftConstraint,
        Self::CapabilityPredicate,
        Self::DeviceIdentity,
        Self::Topology,
        Self::BufferLocation,
        Self::AddressSpace,
        Self::RemoteBoundary,
        Self::Ownership,
        Self::Transfer,
        Self::Synchronization,
        Self::NumericClass,
        Self::Effects,
        Self::Fault,
        Self::Availability,
        Self::Cost,
        Self::CostLimit,
        Self::DeterministicTieBreak,
        Self::StaticFilter,
        Self::RuntimeSelection,
        Self::Rejection,
        Self::FallbackLegality,
        Self::UserIntent,
        Self::Explain,
        Self::Replay,
        Self::DecisionSchema,
        Self::CacheIdentity,
        Self::Provenance,
        Self::Versioning,
        Self::Migration,
        Self::Corruption,
        Self::Privacy,
        Self::PathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::AllocationOrderExclusion,
        Self::DriverTextExclusion,
        Self::DebugOutputExclusion,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::ConflictFixture,
        Self::UnsatisfiableFixture,
        Self::TopologyFixture,
        Self::CapabilityFixture,
        Self::FallbackFixture,
        Self::ExplainFixture,
        Self::ReplayFixture,
        Self::CacheFixture,
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
struct PlacementConstraintInventory {
    boundaries: Box<[PlannedPlacementConstraintBoundary]>,
}

impl PlacementConstraintInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedPlacementConstraintBoundary>,
    ) -> Result<Self, PlannedPlacementConstraintBoundary> {
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
        let mut bytes = b"ling.placement-constraint-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_placement_constraint_boundaries_are_complete_and_ordered() {
    let inventory = PlacementConstraintInventory::new(PlannedPlacementConstraintBoundary::ALL)
        .expect("planned placement constraint boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedPlacementConstraintBoundary::ALL
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
fn placement_constraint_evidence_is_order_independent_and_duplicate_checked() {
    let forward = PlacementConstraintInventory::new(PlannedPlacementConstraintBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = PlacementConstraintInventory::new(
        PlannedPlacementConstraintBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = PlacementConstraintInventory::new([
        PlannedPlacementConstraintBoundary::PlacementConstraint,
        PlannedPlacementConstraintBoundary::PlacementConstraint,
    ])
    .expect_err("duplicate placement constraint must be rejected");
    assert_eq!(
        duplicate,
        PlannedPlacementConstraintBoundary::PlacementConstraint
    );
}

#[test]
fn placement_constraint_evidence_has_no_solver_authority() {
    let inventory = PlacementConstraintInventory::new([
        PlannedPlacementConstraintBoundary::PlacementConstraint,
        PlannedPlacementConstraintBoundary::RequiresGpu,
        PlannedPlacementConstraintBoundary::HardConstraint,
        PlannedPlacementConstraintBoundary::CapabilityPredicate,
        PlannedPlacementConstraintBoundary::FallbackLegality,
        PlannedPlacementConstraintBoundary::Explain,
        PlannedPlacementConstraintBoundary::CacheIdentity,
        PlannedPlacementConstraintBoundary::ProtocolInventory,
    ])
    .expect("bounded placement constraint evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.placement-constraint-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
