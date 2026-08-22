//! Internal Resource and Drop boundary evidence.
//!
//! This test-only inventory names proposed Resource and cleanup boundaries.
//! It does not implement ownership, Drop, cleanup, Effects, Faults, or FFI
//! transfer semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedResourceBoundary {
    ResourceIdentity,
    UniqueOwnership,
    Move,
    UseAfterMove,
    ExplicitDrop,
    DerivedDrop,
    AggregateDropOrder,
    BranchDropOrder,
    LoopDropOrder,
    ClosureDropOrder,
    GenericDrop,
    TraitDrop,
    ActorTurnBoundary,
    SuspensionBoundary,
    CleanupOnReturn,
    CleanupOnError,
    CleanupOnFault,
    CancellationCleanup,
    TimeoutCleanup,
    TerminationCleanup,
    PartialCleanup,
    DropEffect,
    DropFault,
    CapabilityRestriction,
    ManagedFinalizerSeparation,
    FfiTransferMode,
    FfiLifetime,
    FfiThreadBoundary,
    NativeAbi,
    Diagnostic,
    UnicodeSourceSpans,
    InterpreterVmNativeDifferential,
    DeterministicBoundedCleanup,
}

impl PlannedResourceBoundary {
    const ALL: [Self; 33] = [
        Self::ResourceIdentity,
        Self::UniqueOwnership,
        Self::Move,
        Self::UseAfterMove,
        Self::ExplicitDrop,
        Self::DerivedDrop,
        Self::AggregateDropOrder,
        Self::BranchDropOrder,
        Self::LoopDropOrder,
        Self::ClosureDropOrder,
        Self::GenericDrop,
        Self::TraitDrop,
        Self::ActorTurnBoundary,
        Self::SuspensionBoundary,
        Self::CleanupOnReturn,
        Self::CleanupOnError,
        Self::CleanupOnFault,
        Self::CancellationCleanup,
        Self::TimeoutCleanup,
        Self::TerminationCleanup,
        Self::PartialCleanup,
        Self::DropEffect,
        Self::DropFault,
        Self::CapabilityRestriction,
        Self::ManagedFinalizerSeparation,
        Self::FfiTransferMode,
        Self::FfiLifetime,
        Self::FfiThreadBoundary,
        Self::NativeAbi,
        Self::Diagnostic,
        Self::UnicodeSourceSpans,
        Self::InterpreterVmNativeDifferential,
        Self::DeterministicBoundedCleanup,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ResourceIdentity => 0,
            Self::UniqueOwnership => 1,
            Self::Move => 2,
            Self::UseAfterMove => 3,
            Self::ExplicitDrop => 4,
            Self::DerivedDrop => 5,
            Self::AggregateDropOrder => 6,
            Self::BranchDropOrder => 7,
            Self::LoopDropOrder => 8,
            Self::ClosureDropOrder => 9,
            Self::GenericDrop => 10,
            Self::TraitDrop => 11,
            Self::ActorTurnBoundary => 12,
            Self::SuspensionBoundary => 13,
            Self::CleanupOnReturn => 14,
            Self::CleanupOnError => 15,
            Self::CleanupOnFault => 16,
            Self::CancellationCleanup => 17,
            Self::TimeoutCleanup => 18,
            Self::TerminationCleanup => 19,
            Self::PartialCleanup => 20,
            Self::DropEffect => 21,
            Self::DropFault => 22,
            Self::CapabilityRestriction => 23,
            Self::ManagedFinalizerSeparation => 24,
            Self::FfiTransferMode => 25,
            Self::FfiLifetime => 26,
            Self::FfiThreadBoundary => 27,
            Self::NativeAbi => 28,
            Self::Diagnostic => 29,
            Self::UnicodeSourceSpans => 30,
            Self::InterpreterVmNativeDifferential => 31,
            Self::DeterministicBoundedCleanup => 32,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResourceBoundaryInventory {
    boundaries: Box<[PlannedResourceBoundary]>,
}

impl ResourceBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedResourceBoundary>,
    ) -> Result<Self, PlannedResourceBoundary> {
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
        bytes.extend_from_slice(b"ling.resource-drop-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_resource_boundaries_are_complete_and_ordered() {
    let inventory = ResourceBoundaryInventory::new(PlannedResourceBoundary::ALL)
        .expect("planned Resource boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedResourceBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..33).collect::<Vec<_>>()
    );
}

#[test]
fn resource_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ResourceBoundaryInventory::new(PlannedResourceBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ResourceBoundaryInventory::new(PlannedResourceBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ResourceBoundaryInventory::new([
        PlannedResourceBoundary::ExplicitDrop,
        PlannedResourceBoundary::ExplicitDrop,
    ])
    .expect_err("duplicate Resource boundary must be rejected");
    assert_eq!(duplicate, PlannedResourceBoundary::ExplicitDrop);
}

#[test]
fn resource_boundary_evidence_has_no_cleanup_or_ownership_authority() {
    let inventory = ResourceBoundaryInventory::new([
        PlannedResourceBoundary::UniqueOwnership,
        PlannedResourceBoundary::CleanupOnFault,
        PlannedResourceBoundary::FfiTransferMode,
        PlannedResourceBoundary::ManagedFinalizerSeparation,
    ])
    .expect("bounded Resource evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.resource-drop-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
