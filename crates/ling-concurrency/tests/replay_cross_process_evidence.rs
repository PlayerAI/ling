//! Internal cross-process replay acceptance boundary evidence.
//!
//! This test-only inventory names proposed process, provenance, and
//! comparison boundaries. It does not spawn processes, generate logs, replay
//! events, compare runtime results, or claim cross-process reproducibility.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedCrossProcessBoundary {
    ProcessIsolation,
    ToolchainIdentity,
    ProfileIdentity,
    TargetIdentity,
    CacheIsolation,
    InputSnapshot,
    LogGeneration,
    ReplayPlayback,
    ProgramBinding,
    SchemaBinding,
    MutationRejection,
    ObservableEquivalence,
    Repeatability,
    Divergence,
    Provenance,
    ResourceLimits,
    PlatformBoundary,
    OfflineMode,
}

impl PlannedCrossProcessBoundary {
    const ALL: [Self; 18] = [
        Self::ProcessIsolation,
        Self::ToolchainIdentity,
        Self::ProfileIdentity,
        Self::TargetIdentity,
        Self::CacheIsolation,
        Self::InputSnapshot,
        Self::LogGeneration,
        Self::ReplayPlayback,
        Self::ProgramBinding,
        Self::SchemaBinding,
        Self::MutationRejection,
        Self::ObservableEquivalence,
        Self::Repeatability,
        Self::Divergence,
        Self::Provenance,
        Self::ResourceLimits,
        Self::PlatformBoundary,
        Self::OfflineMode,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ProcessIsolation => 0,
            Self::ToolchainIdentity => 1,
            Self::ProfileIdentity => 2,
            Self::TargetIdentity => 3,
            Self::CacheIsolation => 4,
            Self::InputSnapshot => 5,
            Self::LogGeneration => 6,
            Self::ReplayPlayback => 7,
            Self::ProgramBinding => 8,
            Self::SchemaBinding => 9,
            Self::MutationRejection => 10,
            Self::ObservableEquivalence => 11,
            Self::Repeatability => 12,
            Self::Divergence => 13,
            Self::Provenance => 14,
            Self::ResourceLimits => 15,
            Self::PlatformBoundary => 16,
            Self::OfflineMode => 17,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CrossProcessBoundaryInventory {
    boundaries: Box<[PlannedCrossProcessBoundary]>,
}

impl CrossProcessBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCrossProcessBoundary>,
    ) -> Result<Self, PlannedCrossProcessBoundary> {
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
        bytes.extend_from_slice(b"ling.replay-cross-process-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_cross_process_boundaries_are_complete_and_ordered() {
    let inventory = CrossProcessBoundaryInventory::new(PlannedCrossProcessBoundary::ALL)
        .expect("planned cross-process boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedCrossProcessBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..18).collect::<Vec<_>>()
    );
}

#[test]
fn cross_process_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CrossProcessBoundaryInventory::new(PlannedCrossProcessBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        CrossProcessBoundaryInventory::new(PlannedCrossProcessBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CrossProcessBoundaryInventory::new([
        PlannedCrossProcessBoundary::ProgramBinding,
        PlannedCrossProcessBoundary::ProgramBinding,
    ])
    .expect_err("duplicate cross-process boundary must be rejected");
    assert_eq!(duplicate, PlannedCrossProcessBoundary::ProgramBinding);
}

#[test]
fn cross_process_boundary_evidence_has_no_acceptance_authority() {
    let inventory = CrossProcessBoundaryInventory::new([
        PlannedCrossProcessBoundary::ProcessIsolation,
        PlannedCrossProcessBoundary::ObservableEquivalence,
        PlannedCrossProcessBoundary::OfflineMode,
    ])
    .expect("bounded cross-process evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.replay-cross-process-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 3);
}
