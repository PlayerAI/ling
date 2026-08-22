//! Internal region-inference boundary evidence.
//!
//! This test-only inventory names proposed region and lifetime boundaries. It
//! does not implement regions, lifetime inference, escape checking, or
//! diagnostics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedRegionBoundary {
    RegionVariable,
    LifetimeVariable,
    LexicalScope,
    NonLexicalScope,
    OutlivesConstraint,
    Inference,
    FixedPoint,
    Termination,
    Reborrow,
    PlaceInteraction,
    CopyMoveInteraction,
    BorrowInteraction,
    ResourceInteraction,
    ManagedInteraction,
    TraitInteraction,
    ReturnedBorrow,
    ClosureCapture,
    LocalEscape,
    ActorEscape,
    TaskEscape,
    Suspension,
    Await,
    Pinning,
    Cancellation,
    DropInteraction,
    PublicRegionParameter,
    ExplicitLifetime,
    InferredLifetime,
    SeparateCompilation,
    CrossPackage,
    FfiBoundary,
    NativeAbi,
    Diagnostic,
    SemanticGraphProjection,
    AuditSourceProjection,
    UnicodeSourceSpans,
    InterpreterVmNativeDifferential,
    DeterministicInference,
    SeedMigration,
}

impl PlannedRegionBoundary {
    const ALL: [Self; 39] = [
        Self::RegionVariable,
        Self::LifetimeVariable,
        Self::LexicalScope,
        Self::NonLexicalScope,
        Self::OutlivesConstraint,
        Self::Inference,
        Self::FixedPoint,
        Self::Termination,
        Self::Reborrow,
        Self::PlaceInteraction,
        Self::CopyMoveInteraction,
        Self::BorrowInteraction,
        Self::ResourceInteraction,
        Self::ManagedInteraction,
        Self::TraitInteraction,
        Self::ReturnedBorrow,
        Self::ClosureCapture,
        Self::LocalEscape,
        Self::ActorEscape,
        Self::TaskEscape,
        Self::Suspension,
        Self::Await,
        Self::Pinning,
        Self::Cancellation,
        Self::DropInteraction,
        Self::PublicRegionParameter,
        Self::ExplicitLifetime,
        Self::InferredLifetime,
        Self::SeparateCompilation,
        Self::CrossPackage,
        Self::FfiBoundary,
        Self::NativeAbi,
        Self::Diagnostic,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::UnicodeSourceSpans,
        Self::InterpreterVmNativeDifferential,
        Self::DeterministicInference,
        Self::SeedMigration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::RegionVariable => 0,
            Self::LifetimeVariable => 1,
            Self::LexicalScope => 2,
            Self::NonLexicalScope => 3,
            Self::OutlivesConstraint => 4,
            Self::Inference => 5,
            Self::FixedPoint => 6,
            Self::Termination => 7,
            Self::Reborrow => 8,
            Self::PlaceInteraction => 9,
            Self::CopyMoveInteraction => 10,
            Self::BorrowInteraction => 11,
            Self::ResourceInteraction => 12,
            Self::ManagedInteraction => 13,
            Self::TraitInteraction => 14,
            Self::ReturnedBorrow => 15,
            Self::ClosureCapture => 16,
            Self::LocalEscape => 17,
            Self::ActorEscape => 18,
            Self::TaskEscape => 19,
            Self::Suspension => 20,
            Self::Await => 21,
            Self::Pinning => 22,
            Self::Cancellation => 23,
            Self::DropInteraction => 24,
            Self::PublicRegionParameter => 25,
            Self::ExplicitLifetime => 26,
            Self::InferredLifetime => 27,
            Self::SeparateCompilation => 28,
            Self::CrossPackage => 29,
            Self::FfiBoundary => 30,
            Self::NativeAbi => 31,
            Self::Diagnostic => 32,
            Self::SemanticGraphProjection => 33,
            Self::AuditSourceProjection => 34,
            Self::UnicodeSourceSpans => 35,
            Self::InterpreterVmNativeDifferential => 36,
            Self::DeterministicInference => 37,
            Self::SeedMigration => 38,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegionBoundaryInventory {
    boundaries: Box<[PlannedRegionBoundary]>,
}

impl RegionBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedRegionBoundary>,
    ) -> Result<Self, PlannedRegionBoundary> {
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
        bytes.extend_from_slice(b"ling.region-inference-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_region_boundaries_are_complete_and_ordered() {
    let inventory = RegionBoundaryInventory::new(PlannedRegionBoundary::ALL)
        .expect("planned region boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedRegionBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..39).collect::<Vec<_>>()
    );
}

#[test]
fn region_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = RegionBoundaryInventory::new(PlannedRegionBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = RegionBoundaryInventory::new(PlannedRegionBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = RegionBoundaryInventory::new([
        PlannedRegionBoundary::RegionVariable,
        PlannedRegionBoundary::RegionVariable,
    ])
    .expect_err("duplicate region boundary must be rejected");
    assert_eq!(duplicate, PlannedRegionBoundary::RegionVariable);
}

#[test]
fn region_boundary_evidence_has_no_region_authority() {
    let inventory = RegionBoundaryInventory::new([
        PlannedRegionBoundary::RegionVariable,
        PlannedRegionBoundary::LexicalScope,
        PlannedRegionBoundary::ReturnedBorrow,
        PlannedRegionBoundary::Suspension,
    ])
    .expect("bounded region evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.region-inference-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
