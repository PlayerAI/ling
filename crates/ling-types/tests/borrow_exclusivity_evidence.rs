//! Internal borrow-exclusivity boundary evidence.
//!
//! This test-only inventory names proposed borrowing and alias boundaries. It
//! does not implement borrow types, overlap analysis, lifetimes, or
//! diagnostics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedBorrowBoundary {
    ImmutableBorrow,
    MutableBorrow,
    AliasIdentity,
    PlaceOverlap,
    FieldSplitting,
    IndexAlias,
    DynamicProjection,
    AutomaticBorrow,
    Reborrow,
    CallSiteCoercion,
    TemporaryLifetime,
    LifetimeExtension,
    IteratorMutation,
    ContainerMutation,
    MutablePlace,
    ClosureCapture,
    BranchLoopLifetime,
    ReturnEscape,
    PublicLifetime,
    TaskBoundary,
    ActorTurnBoundary,
    SuspensionBoundary,
    PinRegion,
    FfiNativeBoundary,
    CopyMoveInteraction,
    ResourceManagedInteraction,
    TraitInteraction,
    Diagnostic,
    SemanticGraphProjection,
    AuditSourceProjection,
    UnicodeSourceSpans,
    InterpreterVmNativeDifferential,
    DeterministicApproximation,
    SeedMigration,
}

impl PlannedBorrowBoundary {
    const ALL: [Self; 34] = [
        Self::ImmutableBorrow,
        Self::MutableBorrow,
        Self::AliasIdentity,
        Self::PlaceOverlap,
        Self::FieldSplitting,
        Self::IndexAlias,
        Self::DynamicProjection,
        Self::AutomaticBorrow,
        Self::Reborrow,
        Self::CallSiteCoercion,
        Self::TemporaryLifetime,
        Self::LifetimeExtension,
        Self::IteratorMutation,
        Self::ContainerMutation,
        Self::MutablePlace,
        Self::ClosureCapture,
        Self::BranchLoopLifetime,
        Self::ReturnEscape,
        Self::PublicLifetime,
        Self::TaskBoundary,
        Self::ActorTurnBoundary,
        Self::SuspensionBoundary,
        Self::PinRegion,
        Self::FfiNativeBoundary,
        Self::CopyMoveInteraction,
        Self::ResourceManagedInteraction,
        Self::TraitInteraction,
        Self::Diagnostic,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::UnicodeSourceSpans,
        Self::InterpreterVmNativeDifferential,
        Self::DeterministicApproximation,
        Self::SeedMigration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ImmutableBorrow => 0,
            Self::MutableBorrow => 1,
            Self::AliasIdentity => 2,
            Self::PlaceOverlap => 3,
            Self::FieldSplitting => 4,
            Self::IndexAlias => 5,
            Self::DynamicProjection => 6,
            Self::AutomaticBorrow => 7,
            Self::Reborrow => 8,
            Self::CallSiteCoercion => 9,
            Self::TemporaryLifetime => 10,
            Self::LifetimeExtension => 11,
            Self::IteratorMutation => 12,
            Self::ContainerMutation => 13,
            Self::MutablePlace => 14,
            Self::ClosureCapture => 15,
            Self::BranchLoopLifetime => 16,
            Self::ReturnEscape => 17,
            Self::PublicLifetime => 18,
            Self::TaskBoundary => 19,
            Self::ActorTurnBoundary => 20,
            Self::SuspensionBoundary => 21,
            Self::PinRegion => 22,
            Self::FfiNativeBoundary => 23,
            Self::CopyMoveInteraction => 24,
            Self::ResourceManagedInteraction => 25,
            Self::TraitInteraction => 26,
            Self::Diagnostic => 27,
            Self::SemanticGraphProjection => 28,
            Self::AuditSourceProjection => 29,
            Self::UnicodeSourceSpans => 30,
            Self::InterpreterVmNativeDifferential => 31,
            Self::DeterministicApproximation => 32,
            Self::SeedMigration => 33,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BorrowBoundaryInventory {
    boundaries: Box<[PlannedBorrowBoundary]>,
}

impl BorrowBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedBorrowBoundary>,
    ) -> Result<Self, PlannedBorrowBoundary> {
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
        bytes.extend_from_slice(b"ling.borrow-exclusivity-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_borrow_boundaries_are_complete_and_ordered() {
    let inventory = BorrowBoundaryInventory::new(PlannedBorrowBoundary::ALL)
        .expect("planned borrow boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedBorrowBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..34).collect::<Vec<_>>()
    );
}

#[test]
fn borrow_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = BorrowBoundaryInventory::new(PlannedBorrowBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = BorrowBoundaryInventory::new(PlannedBorrowBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = BorrowBoundaryInventory::new([
        PlannedBorrowBoundary::PlaceOverlap,
        PlannedBorrowBoundary::PlaceOverlap,
    ])
    .expect_err("duplicate borrow boundary must be rejected");
    assert_eq!(duplicate, PlannedBorrowBoundary::PlaceOverlap);
}

#[test]
fn borrow_boundary_evidence_has_no_exclusivity_authority() {
    let inventory = BorrowBoundaryInventory::new([
        PlannedBorrowBoundary::ImmutableBorrow,
        PlannedBorrowBoundary::MutableBorrow,
        PlannedBorrowBoundary::IndexAlias,
        PlannedBorrowBoundary::SuspensionBoundary,
    ])
    .expect("bounded borrow evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.borrow-exclusivity-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
