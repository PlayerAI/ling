//! Internal Drop-order and cleanup boundary evidence.
//!
//! This test-only inventory names proposed cleanup boundaries. It does not
//! implement Resource ownership, Drop insertion, Cleanup Core, or failure
//! semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedDropBoundary {
    ResourceIdentity,
    OwnershipTransfer,
    MoveBorrowRegion,
    ImplicitDrop,
    ExplicitDrop,
    AggregateOrder,
    BranchOrder,
    LoopOrder,
    ClosureOrder,
    ReverseDeclarationOrder,
    RfcDefinedOrder,
    PartialInitialization,
    ReplaceSemantics,
    CleanupCore,
    NormalReturn,
    EarlyReturn,
    TryError,
    Fault,
    Cancellation,
    Timeout,
    TaskTermination,
    ActorTermination,
    ProcessShutdown,
    PanicUnwindRejection,
    Idempotence,
    PartialCleanup,
    FailureAggregation,
    EffectsFaults,
    BoundedCleanup,
    ManagedSeparation,
    CapabilityNetwork,
    NativeFfiAbi,
    ProfileCritical,
    Migration,
    DeterministicOptimization,
    Diagnostic,
    SemanticGraphProjection,
    AuditSourceProjection,
    UnicodeSourceSpans,
    InterpreterVmNativeDifferential,
    SeedMigration,
}

impl PlannedDropBoundary {
    const ALL: [Self; 41] = [
        Self::ResourceIdentity,
        Self::OwnershipTransfer,
        Self::MoveBorrowRegion,
        Self::ImplicitDrop,
        Self::ExplicitDrop,
        Self::AggregateOrder,
        Self::BranchOrder,
        Self::LoopOrder,
        Self::ClosureOrder,
        Self::ReverseDeclarationOrder,
        Self::RfcDefinedOrder,
        Self::PartialInitialization,
        Self::ReplaceSemantics,
        Self::CleanupCore,
        Self::NormalReturn,
        Self::EarlyReturn,
        Self::TryError,
        Self::Fault,
        Self::Cancellation,
        Self::Timeout,
        Self::TaskTermination,
        Self::ActorTermination,
        Self::ProcessShutdown,
        Self::PanicUnwindRejection,
        Self::Idempotence,
        Self::PartialCleanup,
        Self::FailureAggregation,
        Self::EffectsFaults,
        Self::BoundedCleanup,
        Self::ManagedSeparation,
        Self::CapabilityNetwork,
        Self::NativeFfiAbi,
        Self::ProfileCritical,
        Self::Migration,
        Self::DeterministicOptimization,
        Self::Diagnostic,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::UnicodeSourceSpans,
        Self::InterpreterVmNativeDifferential,
        Self::SeedMigration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ResourceIdentity => 0,
            Self::OwnershipTransfer => 1,
            Self::MoveBorrowRegion => 2,
            Self::ImplicitDrop => 3,
            Self::ExplicitDrop => 4,
            Self::AggregateOrder => 5,
            Self::BranchOrder => 6,
            Self::LoopOrder => 7,
            Self::ClosureOrder => 8,
            Self::ReverseDeclarationOrder => 9,
            Self::RfcDefinedOrder => 10,
            Self::PartialInitialization => 11,
            Self::ReplaceSemantics => 12,
            Self::CleanupCore => 13,
            Self::NormalReturn => 14,
            Self::EarlyReturn => 15,
            Self::TryError => 16,
            Self::Fault => 17,
            Self::Cancellation => 18,
            Self::Timeout => 19,
            Self::TaskTermination => 20,
            Self::ActorTermination => 21,
            Self::ProcessShutdown => 22,
            Self::PanicUnwindRejection => 23,
            Self::Idempotence => 24,
            Self::PartialCleanup => 25,
            Self::FailureAggregation => 26,
            Self::EffectsFaults => 27,
            Self::BoundedCleanup => 28,
            Self::ManagedSeparation => 29,
            Self::CapabilityNetwork => 30,
            Self::NativeFfiAbi => 31,
            Self::ProfileCritical => 32,
            Self::Migration => 33,
            Self::DeterministicOptimization => 34,
            Self::Diagnostic => 35,
            Self::SemanticGraphProjection => 36,
            Self::AuditSourceProjection => 37,
            Self::UnicodeSourceSpans => 38,
            Self::InterpreterVmNativeDifferential => 39,
            Self::SeedMigration => 40,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DropBoundaryInventory {
    boundaries: Box<[PlannedDropBoundary]>,
}

impl DropBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDropBoundary>,
    ) -> Result<Self, PlannedDropBoundary> {
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
        bytes.extend_from_slice(b"ling.drop-order-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_drop_boundaries_are_complete_and_ordered() {
    let inventory = DropBoundaryInventory::new(PlannedDropBoundary::ALL)
        .expect("planned Drop boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedDropBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..41).collect::<Vec<_>>()
    );
}

#[test]
fn drop_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DropBoundaryInventory::new(PlannedDropBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = DropBoundaryInventory::new(PlannedDropBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DropBoundaryInventory::new([
        PlannedDropBoundary::ImplicitDrop,
        PlannedDropBoundary::ImplicitDrop,
    ])
    .expect_err("duplicate Drop boundary must be rejected");
    assert_eq!(duplicate, PlannedDropBoundary::ImplicitDrop);
}

#[test]
fn drop_boundary_evidence_has_no_cleanup_authority() {
    let inventory = DropBoundaryInventory::new([
        PlannedDropBoundary::ResourceIdentity,
        PlannedDropBoundary::ReverseDeclarationOrder,
        PlannedDropBoundary::Cancellation,
        PlannedDropBoundary::NativeFfiAbi,
    ])
    .expect("bounded Drop evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.drop-order-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
