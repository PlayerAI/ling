//! Internal ownership negative-corpus and property-test boundary evidence.
//!
//! This test-only inventory names proposed corpus and property boundaries. It
//! does not implement ownership oracles, generators, fuzz targets, or
//! diagnostics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedCorpusBoundary {
    LegalIllegalOracle,
    Value,
    Managed,
    Resource,
    CopyMove,
    BorrowRegion,
    Drop,
    Alias,
    PartialMove,
    Match,
    Loop,
    Closure,
    Fault,
    Cancellation,
    Task,
    Actor,
    Await,
    FfiTransfer,
    AutomaticBorrow,
    PublicLifetime,
    Profile,
    Generator,
    Shrinking,
    Bounds,
    StateMachineInterleaving,
    FailureCancellationRestart,
    DeterministicSeeds,
    ResourceLimits,
    HostFailureSeparation,
    NegativeDiagnostic,
    RepairFixture,
    UnicodeSourceSpans,
    DeterministicOrdering,
    InterpreterVmNativeDifferential,
    CorpusMigration,
    SeedPreservation,
}

impl PlannedCorpusBoundary {
    const ALL: [Self; 36] = [
        Self::LegalIllegalOracle,
        Self::Value,
        Self::Managed,
        Self::Resource,
        Self::CopyMove,
        Self::BorrowRegion,
        Self::Drop,
        Self::Alias,
        Self::PartialMove,
        Self::Match,
        Self::Loop,
        Self::Closure,
        Self::Fault,
        Self::Cancellation,
        Self::Task,
        Self::Actor,
        Self::Await,
        Self::FfiTransfer,
        Self::AutomaticBorrow,
        Self::PublicLifetime,
        Self::Profile,
        Self::Generator,
        Self::Shrinking,
        Self::Bounds,
        Self::StateMachineInterleaving,
        Self::FailureCancellationRestart,
        Self::DeterministicSeeds,
        Self::ResourceLimits,
        Self::HostFailureSeparation,
        Self::NegativeDiagnostic,
        Self::RepairFixture,
        Self::UnicodeSourceSpans,
        Self::DeterministicOrdering,
        Self::InterpreterVmNativeDifferential,
        Self::CorpusMigration,
        Self::SeedPreservation,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::LegalIllegalOracle => 0,
            Self::Value => 1,
            Self::Managed => 2,
            Self::Resource => 3,
            Self::CopyMove => 4,
            Self::BorrowRegion => 5,
            Self::Drop => 6,
            Self::Alias => 7,
            Self::PartialMove => 8,
            Self::Match => 9,
            Self::Loop => 10,
            Self::Closure => 11,
            Self::Fault => 12,
            Self::Cancellation => 13,
            Self::Task => 14,
            Self::Actor => 15,
            Self::Await => 16,
            Self::FfiTransfer => 17,
            Self::AutomaticBorrow => 18,
            Self::PublicLifetime => 19,
            Self::Profile => 20,
            Self::Generator => 21,
            Self::Shrinking => 22,
            Self::Bounds => 23,
            Self::StateMachineInterleaving => 24,
            Self::FailureCancellationRestart => 25,
            Self::DeterministicSeeds => 26,
            Self::ResourceLimits => 27,
            Self::HostFailureSeparation => 28,
            Self::NegativeDiagnostic => 29,
            Self::RepairFixture => 30,
            Self::UnicodeSourceSpans => 31,
            Self::DeterministicOrdering => 32,
            Self::InterpreterVmNativeDifferential => 33,
            Self::CorpusMigration => 34,
            Self::SeedPreservation => 35,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CorpusBoundaryInventory {
    boundaries: Box<[PlannedCorpusBoundary]>,
}

impl CorpusBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCorpusBoundary>,
    ) -> Result<Self, PlannedCorpusBoundary> {
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
        bytes.extend_from_slice(b"ling.ownership-corpus-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_corpus_boundaries_are_complete_and_ordered() {
    let inventory = CorpusBoundaryInventory::new(PlannedCorpusBoundary::ALL)
        .expect("planned corpus boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedCorpusBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..36).collect::<Vec<_>>()
    );
}

#[test]
fn corpus_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CorpusBoundaryInventory::new(PlannedCorpusBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = CorpusBoundaryInventory::new(PlannedCorpusBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CorpusBoundaryInventory::new([
        PlannedCorpusBoundary::LegalIllegalOracle,
        PlannedCorpusBoundary::LegalIllegalOracle,
    ])
    .expect_err("duplicate corpus boundary must be rejected");
    assert_eq!(duplicate, PlannedCorpusBoundary::LegalIllegalOracle);
}

#[test]
fn corpus_boundary_evidence_has_no_ownership_oracle_authority() {
    let inventory = CorpusBoundaryInventory::new([
        PlannedCorpusBoundary::LegalIllegalOracle,
        PlannedCorpusBoundary::Generator,
        PlannedCorpusBoundary::NegativeDiagnostic,
        PlannedCorpusBoundary::InterpreterVmNativeDifferential,
    ])
    .expect("bounded corpus evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.ownership-corpus-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
