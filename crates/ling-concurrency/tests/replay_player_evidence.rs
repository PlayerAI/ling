//! Internal replay-player boundary evidence.
//!
//! This test-only inventory names proposed preflight and comparison boundaries.
//! It does not read logs, validate checkpoints, apply events, or restore state.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedReplayPlayerBoundary {
    CheckpointIdentity,
    ProgramCanonicalBytes,
    PreflightBinding,
    EventApplication,
    Ordering,
    Divergence,
    Fault,
    Cancellation,
    Privacy,
    Integrity,
    Migration,
}

impl PlannedReplayPlayerBoundary {
    const ALL: [Self; 11] = [
        Self::CheckpointIdentity,
        Self::ProgramCanonicalBytes,
        Self::PreflightBinding,
        Self::EventApplication,
        Self::Ordering,
        Self::Divergence,
        Self::Fault,
        Self::Cancellation,
        Self::Privacy,
        Self::Integrity,
        Self::Migration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::CheckpointIdentity => 0,
            Self::ProgramCanonicalBytes => 1,
            Self::PreflightBinding => 2,
            Self::EventApplication => 3,
            Self::Ordering => 4,
            Self::Divergence => 5,
            Self::Fault => 6,
            Self::Cancellation => 7,
            Self::Privacy => 8,
            Self::Integrity => 9,
            Self::Migration => 10,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReplayPlayerBoundaryInventory {
    boundaries: Box<[PlannedReplayPlayerBoundary]>,
}

impl ReplayPlayerBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedReplayPlayerBoundary>,
    ) -> Result<Self, PlannedReplayPlayerBoundary> {
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
        bytes.extend_from_slice(b"ling.replay-player-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_replay_player_boundaries_are_complete_and_ordered() {
    let inventory = ReplayPlayerBoundaryInventory::new(PlannedReplayPlayerBoundary::ALL)
        .expect("planned player boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedReplayPlayerBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..11).collect::<Vec<_>>()
    );
}

#[test]
fn replay_player_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ReplayPlayerBoundaryInventory::new(PlannedReplayPlayerBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ReplayPlayerBoundaryInventory::new(PlannedReplayPlayerBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ReplayPlayerBoundaryInventory::new([
        PlannedReplayPlayerBoundary::Ordering,
        PlannedReplayPlayerBoundary::Ordering,
    ])
    .expect_err("duplicate player boundary must be rejected");
    assert_eq!(duplicate, PlannedReplayPlayerBoundary::Ordering);
}

#[test]
fn replay_player_boundary_evidence_has_no_player_authority() {
    let inventory = ReplayPlayerBoundaryInventory::new([
        PlannedReplayPlayerBoundary::CheckpointIdentity,
        PlannedReplayPlayerBoundary::PreflightBinding,
    ])
    .expect("bounded player evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.replay-player-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 2);
}
