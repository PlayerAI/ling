use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedCostModelBoundary {
    CostModel,
    InputBytes,
    TransferBytes,
    OperationCount,
    MemoryFootprint,
    LaunchOverhead,
    OccupancyHint,
    DeadlineMetadata,
    EnergyMetadata,
    Units,
    StaticInput,
    DynamicInput,
    Calibration,
    Provenance,
    Confidence,
    Uncertainty,
    Estimate,
    GuaranteeExclusion,
    Overflow,
    UnknownValue,
    InvalidUnit,
    HardwareModel,
    DeviceIdentity,
    CapabilityContext,
    PlacementContext,
    BufferContext,
    Policy,
    PolicyPrecedence,
    CostLimit,
    Fallback,
    Rejection,
    DeterministicSerialization,
    SelectionInput,
    DiagnosticOnly,
    CriticalProfile,
    StrictProfile,
    NativeProfile,
    ReplayInput,
    CacheIdentity,
    ExplainField,
    Versioning,
    Migration,
    Corruption,
    Privacy,
    PathExclusion,
    AddressExclusion,
    TimestampExclusion,
    DriverTextExclusion,
    DebugOutputExclusion,
    PositiveFixture,
    NegativeFixture,
    CalibrationFixture,
    UncertaintyFixture,
    DeterminismFixture,
    TopologyFixture,
    FallbackFixture,
    DifferentialFixture,
    UnicodeFixture,
    DiagnosticCode,
    ProtocolInventory,
}

impl PlannedCostModelBoundary {
    const ALL: [Self; 60] = [
        Self::CostModel,
        Self::InputBytes,
        Self::TransferBytes,
        Self::OperationCount,
        Self::MemoryFootprint,
        Self::LaunchOverhead,
        Self::OccupancyHint,
        Self::DeadlineMetadata,
        Self::EnergyMetadata,
        Self::Units,
        Self::StaticInput,
        Self::DynamicInput,
        Self::Calibration,
        Self::Provenance,
        Self::Confidence,
        Self::Uncertainty,
        Self::Estimate,
        Self::GuaranteeExclusion,
        Self::Overflow,
        Self::UnknownValue,
        Self::InvalidUnit,
        Self::HardwareModel,
        Self::DeviceIdentity,
        Self::CapabilityContext,
        Self::PlacementContext,
        Self::BufferContext,
        Self::Policy,
        Self::PolicyPrecedence,
        Self::CostLimit,
        Self::Fallback,
        Self::Rejection,
        Self::DeterministicSerialization,
        Self::SelectionInput,
        Self::DiagnosticOnly,
        Self::CriticalProfile,
        Self::StrictProfile,
        Self::NativeProfile,
        Self::ReplayInput,
        Self::CacheIdentity,
        Self::ExplainField,
        Self::Versioning,
        Self::Migration,
        Self::Corruption,
        Self::Privacy,
        Self::PathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::DriverTextExclusion,
        Self::DebugOutputExclusion,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CalibrationFixture,
        Self::UncertaintyFixture,
        Self::DeterminismFixture,
        Self::TopologyFixture,
        Self::FallbackFixture,
        Self::DifferentialFixture,
        Self::UnicodeFixture,
        Self::DiagnosticCode,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CostModelInventory {
    boundaries: Box<[PlannedCostModelBoundary]>,
}

impl CostModelInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCostModelBoundary>,
    ) -> Result<Self, PlannedCostModelBoundary> {
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
        let mut bytes = b"ling.cost-model-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_cost_model_boundaries_are_complete_and_ordered() {
    let inventory = CostModelInventory::new(PlannedCostModelBoundary::ALL)
        .expect("planned cost model boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedCostModelBoundary::ALL
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
fn cost_model_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CostModelInventory::new(PlannedCostModelBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = CostModelInventory::new(PlannedCostModelBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CostModelInventory::new([
        PlannedCostModelBoundary::CostModel,
        PlannedCostModelBoundary::CostModel,
    ])
    .expect_err("duplicate cost model boundary must be rejected");
    assert_eq!(duplicate, PlannedCostModelBoundary::CostModel);
}

#[test]
fn cost_model_evidence_has_no_estimator_authority() {
    let inventory = CostModelInventory::new([
        PlannedCostModelBoundary::CostModel,
        PlannedCostModelBoundary::InputBytes,
        PlannedCostModelBoundary::Calibration,
        PlannedCostModelBoundary::Uncertainty,
        PlannedCostModelBoundary::SelectionInput,
        PlannedCostModelBoundary::DiagnosticOnly,
        PlannedCostModelBoundary::GuaranteeExclusion,
        PlannedCostModelBoundary::ProtocolInventory,
    ])
    .expect("bounded cost model evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.cost-model-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
