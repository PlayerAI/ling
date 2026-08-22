//! Internal Place and Move-analysis boundary evidence.
//!
//! This test-only inventory names proposed ownership-analysis boundaries. It
//! does not implement move/borrow states, dataflow, diagnostics, or Typed
//! Core ownership semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedPlaceMoveBoundary {
    LocalPlace,
    FieldPlace,
    IndexPlace,
    Projection,
    Move,
    Copy,
    Borrow,
    BorrowMut,
    Initialization,
    PartialMove,
    Reinitialization,
    Destructuring,
    ClosureCapture,
    Aggregate,
    Generic,
    BranchJoin,
    LoopFixedPoint,
    MatchJoin,
    ErrorFault,
    Cancellation,
    TaskBoundary,
    ActorTurnBoundary,
    SuspensionBoundary,
    ResourceBoundary,
    ManagedBoundary,
    LifetimeRegion,
    FfiNativeBoundary,
    Diagnostic,
    SemanticGraphProjection,
    AuditSourceProjection,
    UnicodeSourceSpans,
    InterpreterVmNativeDifferential,
    DeterministicTermination,
    SeedMigration,
}

impl PlannedPlaceMoveBoundary {
    const ALL: [Self; 34] = [
        Self::LocalPlace,
        Self::FieldPlace,
        Self::IndexPlace,
        Self::Projection,
        Self::Move,
        Self::Copy,
        Self::Borrow,
        Self::BorrowMut,
        Self::Initialization,
        Self::PartialMove,
        Self::Reinitialization,
        Self::Destructuring,
        Self::ClosureCapture,
        Self::Aggregate,
        Self::Generic,
        Self::BranchJoin,
        Self::LoopFixedPoint,
        Self::MatchJoin,
        Self::ErrorFault,
        Self::Cancellation,
        Self::TaskBoundary,
        Self::ActorTurnBoundary,
        Self::SuspensionBoundary,
        Self::ResourceBoundary,
        Self::ManagedBoundary,
        Self::LifetimeRegion,
        Self::FfiNativeBoundary,
        Self::Diagnostic,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::UnicodeSourceSpans,
        Self::InterpreterVmNativeDifferential,
        Self::DeterministicTermination,
        Self::SeedMigration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::LocalPlace => 0,
            Self::FieldPlace => 1,
            Self::IndexPlace => 2,
            Self::Projection => 3,
            Self::Move => 4,
            Self::Copy => 5,
            Self::Borrow => 6,
            Self::BorrowMut => 7,
            Self::Initialization => 8,
            Self::PartialMove => 9,
            Self::Reinitialization => 10,
            Self::Destructuring => 11,
            Self::ClosureCapture => 12,
            Self::Aggregate => 13,
            Self::Generic => 14,
            Self::BranchJoin => 15,
            Self::LoopFixedPoint => 16,
            Self::MatchJoin => 17,
            Self::ErrorFault => 18,
            Self::Cancellation => 19,
            Self::TaskBoundary => 20,
            Self::ActorTurnBoundary => 21,
            Self::SuspensionBoundary => 22,
            Self::ResourceBoundary => 23,
            Self::ManagedBoundary => 24,
            Self::LifetimeRegion => 25,
            Self::FfiNativeBoundary => 26,
            Self::Diagnostic => 27,
            Self::SemanticGraphProjection => 28,
            Self::AuditSourceProjection => 29,
            Self::UnicodeSourceSpans => 30,
            Self::InterpreterVmNativeDifferential => 31,
            Self::DeterministicTermination => 32,
            Self::SeedMigration => 33,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlaceMoveBoundaryInventory {
    boundaries: Box<[PlannedPlaceMoveBoundary]>,
}

impl PlaceMoveBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedPlaceMoveBoundary>,
    ) -> Result<Self, PlannedPlaceMoveBoundary> {
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
        bytes.extend_from_slice(b"ling.place-move-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_place_move_boundaries_are_complete_and_ordered() {
    let inventory = PlaceMoveBoundaryInventory::new(PlannedPlaceMoveBoundary::ALL)
        .expect("planned Place/Move boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedPlaceMoveBoundary::ALL
    );
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
fn place_move_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = PlaceMoveBoundaryInventory::new(PlannedPlaceMoveBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = PlaceMoveBoundaryInventory::new(PlannedPlaceMoveBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = PlaceMoveBoundaryInventory::new([
        PlannedPlaceMoveBoundary::Projection,
        PlannedPlaceMoveBoundary::Projection,
    ])
    .expect_err("duplicate Place/Move boundary must be rejected");
    assert_eq!(duplicate, PlannedPlaceMoveBoundary::Projection);
}

#[test]
fn place_move_boundary_evidence_has_no_ownership_authority() {
    let inventory = PlaceMoveBoundaryInventory::new([
        PlannedPlaceMoveBoundary::LocalPlace,
        PlannedPlaceMoveBoundary::Move,
        PlannedPlaceMoveBoundary::BorrowMut,
        PlannedPlaceMoveBoundary::ActorTurnBoundary,
    ])
    .expect("bounded Place/Move evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.place-move-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
