//! Internal replay privacy and integrity boundary evidence.
//!
//! This test-only inventory names proposed privacy, trimming, and corruption
//! boundaries. It does not classify data, redact values, retain logs, decode
//! chunks, or implement offline replay.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedReplayPrivacyBoundary {
    FieldSensitivity,
    FieldRedaction,
    SecretPiiExclusion,
    CapabilityResourceExclusion,
    Authorization,
    KeyHandling,
    Retention,
    DependencyClosure,
    ChunkBoundary,
    ChecksumIntegrity,
    Truncation,
    Corruption,
    FailureDiagnostics,
    UnknownField,
    OfflineMode,
    Migration,
}

impl PlannedReplayPrivacyBoundary {
    const ALL: [Self; 16] = [
        Self::FieldSensitivity,
        Self::FieldRedaction,
        Self::SecretPiiExclusion,
        Self::CapabilityResourceExclusion,
        Self::Authorization,
        Self::KeyHandling,
        Self::Retention,
        Self::DependencyClosure,
        Self::ChunkBoundary,
        Self::ChecksumIntegrity,
        Self::Truncation,
        Self::Corruption,
        Self::FailureDiagnostics,
        Self::UnknownField,
        Self::OfflineMode,
        Self::Migration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::FieldSensitivity => 0,
            Self::FieldRedaction => 1,
            Self::SecretPiiExclusion => 2,
            Self::CapabilityResourceExclusion => 3,
            Self::Authorization => 4,
            Self::KeyHandling => 5,
            Self::Retention => 6,
            Self::DependencyClosure => 7,
            Self::ChunkBoundary => 8,
            Self::ChecksumIntegrity => 9,
            Self::Truncation => 10,
            Self::Corruption => 11,
            Self::FailureDiagnostics => 12,
            Self::UnknownField => 13,
            Self::OfflineMode => 14,
            Self::Migration => 15,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReplayPrivacyBoundaryInventory {
    boundaries: Box<[PlannedReplayPrivacyBoundary]>,
}

impl ReplayPrivacyBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedReplayPrivacyBoundary>,
    ) -> Result<Self, PlannedReplayPrivacyBoundary> {
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
        bytes.extend_from_slice(b"ling.replay-privacy-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_replay_privacy_boundaries_are_complete_and_ordered() {
    let inventory = ReplayPrivacyBoundaryInventory::new(PlannedReplayPrivacyBoundary::ALL)
        .expect("planned privacy boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedReplayPrivacyBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..16).collect::<Vec<_>>()
    );
}

#[test]
fn replay_privacy_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ReplayPrivacyBoundaryInventory::new(PlannedReplayPrivacyBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ReplayPrivacyBoundaryInventory::new(PlannedReplayPrivacyBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ReplayPrivacyBoundaryInventory::new([
        PlannedReplayPrivacyBoundary::ChecksumIntegrity,
        PlannedReplayPrivacyBoundary::ChecksumIntegrity,
    ])
    .expect_err("duplicate privacy boundary must be rejected");
    assert_eq!(duplicate, PlannedReplayPrivacyBoundary::ChecksumIntegrity);
}

#[test]
fn replay_privacy_boundary_evidence_has_no_privacy_authority() {
    let inventory = ReplayPrivacyBoundaryInventory::new([
        PlannedReplayPrivacyBoundary::FieldSensitivity,
        PlannedReplayPrivacyBoundary::OfflineMode,
        PlannedReplayPrivacyBoundary::Corruption,
    ])
    .expect("bounded privacy evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.replay-privacy-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 3);
}
