//! Internal cross-suspension and Actor-turn boundary evidence.
//!
//! This test-only inventory names proposed suspension and turn boundaries. It
//! does not implement await, pinning, borrow checking, Actor reentry, or
//! message semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedSuspensionBoundary {
    StackLocalBorrow,
    TurnLocalBorrow,
    SuspensionPoint,
    Await,
    BorrowAcrossSuspension,
    PinnedField,
    OwnedField,
    StateMachineLowering,
    ActorStateBorrow,
    TurnEnd,
    ActorReentry,
    RemoteMessageBorrow,
    MessageSendability,
    CopyValue,
    ShortenBorrow,
    MoveOwnership,
    SplitState,
    Cancellation,
    Timeout,
    Drop,
    Fault,
    PartialInitialization,
    RegionInteraction,
    BorrowInteraction,
    ResourceManaged,
    TaskBoundary,
    ActorBoundary,
    CrossPackage,
    FfiNativeAbi,
    CapabilitySecurity,
    DiagnosticRepair,
    SemanticGraphProjection,
    AuditSourceProjection,
    UnicodeSourceSpans,
    InterleavingReplay,
    DeterministicDifferential,
    SeedMigration,
}

impl PlannedSuspensionBoundary {
    const ALL: [Self; 37] = [
        Self::StackLocalBorrow,
        Self::TurnLocalBorrow,
        Self::SuspensionPoint,
        Self::Await,
        Self::BorrowAcrossSuspension,
        Self::PinnedField,
        Self::OwnedField,
        Self::StateMachineLowering,
        Self::ActorStateBorrow,
        Self::TurnEnd,
        Self::ActorReentry,
        Self::RemoteMessageBorrow,
        Self::MessageSendability,
        Self::CopyValue,
        Self::ShortenBorrow,
        Self::MoveOwnership,
        Self::SplitState,
        Self::Cancellation,
        Self::Timeout,
        Self::Drop,
        Self::Fault,
        Self::PartialInitialization,
        Self::RegionInteraction,
        Self::BorrowInteraction,
        Self::ResourceManaged,
        Self::TaskBoundary,
        Self::ActorBoundary,
        Self::CrossPackage,
        Self::FfiNativeAbi,
        Self::CapabilitySecurity,
        Self::DiagnosticRepair,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::UnicodeSourceSpans,
        Self::InterleavingReplay,
        Self::DeterministicDifferential,
        Self::SeedMigration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::StackLocalBorrow => 0,
            Self::TurnLocalBorrow => 1,
            Self::SuspensionPoint => 2,
            Self::Await => 3,
            Self::BorrowAcrossSuspension => 4,
            Self::PinnedField => 5,
            Self::OwnedField => 6,
            Self::StateMachineLowering => 7,
            Self::ActorStateBorrow => 8,
            Self::TurnEnd => 9,
            Self::ActorReentry => 10,
            Self::RemoteMessageBorrow => 11,
            Self::MessageSendability => 12,
            Self::CopyValue => 13,
            Self::ShortenBorrow => 14,
            Self::MoveOwnership => 15,
            Self::SplitState => 16,
            Self::Cancellation => 17,
            Self::Timeout => 18,
            Self::Drop => 19,
            Self::Fault => 20,
            Self::PartialInitialization => 21,
            Self::RegionInteraction => 22,
            Self::BorrowInteraction => 23,
            Self::ResourceManaged => 24,
            Self::TaskBoundary => 25,
            Self::ActorBoundary => 26,
            Self::CrossPackage => 27,
            Self::FfiNativeAbi => 28,
            Self::CapabilitySecurity => 29,
            Self::DiagnosticRepair => 30,
            Self::SemanticGraphProjection => 31,
            Self::AuditSourceProjection => 32,
            Self::UnicodeSourceSpans => 33,
            Self::InterleavingReplay => 34,
            Self::DeterministicDifferential => 35,
            Self::SeedMigration => 36,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SuspensionBoundaryInventory {
    boundaries: Box<[PlannedSuspensionBoundary]>,
}

impl SuspensionBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedSuspensionBoundary>,
    ) -> Result<Self, PlannedSuspensionBoundary> {
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
        bytes.extend_from_slice(b"ling.borrow-await-turn-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_suspension_boundaries_are_complete_and_ordered() {
    let inventory = SuspensionBoundaryInventory::new(PlannedSuspensionBoundary::ALL)
        .expect("planned suspension boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedSuspensionBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..37).collect::<Vec<_>>()
    );
}

#[test]
fn suspension_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = SuspensionBoundaryInventory::new(PlannedSuspensionBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        SuspensionBoundaryInventory::new(PlannedSuspensionBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = SuspensionBoundaryInventory::new([
        PlannedSuspensionBoundary::Await,
        PlannedSuspensionBoundary::Await,
    ])
    .expect_err("duplicate suspension boundary must be rejected");
    assert_eq!(duplicate, PlannedSuspensionBoundary::Await);
}

#[test]
fn suspension_boundary_evidence_has_no_await_or_actor_authority() {
    let inventory = SuspensionBoundaryInventory::new([
        PlannedSuspensionBoundary::StackLocalBorrow,
        PlannedSuspensionBoundary::PinnedField,
        PlannedSuspensionBoundary::ActorReentry,
        PlannedSuspensionBoundary::RemoteMessageBorrow,
    ])
    .expect("bounded suspension evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.borrow-await-turn-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
