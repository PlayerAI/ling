//! Internal Managed-graph and island-boundary evidence.
//!
//! This test-only inventory names proposed Managed and isolation boundaries.
//! It does not implement references, graphs, collection, pinning, borrowed
//! views, cross-domain transfer, or runtime semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedManagedBoundary {
    ManagedIdentity,
    ManagedGraph,
    ValueToManagedEdge,
    ResourceToManagedEdge,
    ManagedToResourceEdge,
    IslandRoot,
    RootDiscovery,
    Reachability,
    Cycle,
    SharingAliasing,
    Equality,
    Hash,
    Serialization,
    Collection,
    Finalization,
    OomFault,
    Pinning,
    BorrowedView,
    BorrowedViewExpiry,
    MutationConcurrency,
    CancellationBoundary,
    ActorTurnBoundary,
    TaskBoundary,
    CrossThreadTransfer,
    FfiTransfer,
    NativeAbi,
    TargetPrimitive,
    ProfileConstraint,
    CapabilitySecurity,
    CheckedCoreProjection,
    SemanticGraphProjection,
    AuditSourceProjection,
    CanonicalBytes,
    Diagnostic,
    UnicodeSourceSpans,
    InterpreterVmNativeDifferential,
    DeterministicObservability,
    IslandEscape,
}

impl PlannedManagedBoundary {
    const ALL: [Self; 38] = [
        Self::ManagedIdentity,
        Self::ManagedGraph,
        Self::ValueToManagedEdge,
        Self::ResourceToManagedEdge,
        Self::ManagedToResourceEdge,
        Self::IslandRoot,
        Self::RootDiscovery,
        Self::Reachability,
        Self::Cycle,
        Self::SharingAliasing,
        Self::Equality,
        Self::Hash,
        Self::Serialization,
        Self::Collection,
        Self::Finalization,
        Self::OomFault,
        Self::Pinning,
        Self::BorrowedView,
        Self::BorrowedViewExpiry,
        Self::MutationConcurrency,
        Self::CancellationBoundary,
        Self::ActorTurnBoundary,
        Self::TaskBoundary,
        Self::CrossThreadTransfer,
        Self::FfiTransfer,
        Self::NativeAbi,
        Self::TargetPrimitive,
        Self::ProfileConstraint,
        Self::CapabilitySecurity,
        Self::CheckedCoreProjection,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::CanonicalBytes,
        Self::Diagnostic,
        Self::UnicodeSourceSpans,
        Self::InterpreterVmNativeDifferential,
        Self::DeterministicObservability,
        Self::IslandEscape,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ManagedIdentity => 0,
            Self::ManagedGraph => 1,
            Self::ValueToManagedEdge => 2,
            Self::ResourceToManagedEdge => 3,
            Self::ManagedToResourceEdge => 4,
            Self::IslandRoot => 5,
            Self::RootDiscovery => 6,
            Self::Reachability => 7,
            Self::Cycle => 8,
            Self::SharingAliasing => 9,
            Self::Equality => 10,
            Self::Hash => 11,
            Self::Serialization => 12,
            Self::Collection => 13,
            Self::Finalization => 14,
            Self::OomFault => 15,
            Self::Pinning => 16,
            Self::BorrowedView => 17,
            Self::BorrowedViewExpiry => 18,
            Self::MutationConcurrency => 19,
            Self::CancellationBoundary => 20,
            Self::ActorTurnBoundary => 21,
            Self::TaskBoundary => 22,
            Self::CrossThreadTransfer => 23,
            Self::FfiTransfer => 24,
            Self::NativeAbi => 25,
            Self::TargetPrimitive => 26,
            Self::ProfileConstraint => 27,
            Self::CapabilitySecurity => 28,
            Self::CheckedCoreProjection => 29,
            Self::SemanticGraphProjection => 30,
            Self::AuditSourceProjection => 31,
            Self::CanonicalBytes => 32,
            Self::Diagnostic => 33,
            Self::UnicodeSourceSpans => 34,
            Self::InterpreterVmNativeDifferential => 35,
            Self::DeterministicObservability => 36,
            Self::IslandEscape => 37,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ManagedBoundaryInventory {
    boundaries: Box<[PlannedManagedBoundary]>,
}

impl ManagedBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedManagedBoundary>,
    ) -> Result<Self, PlannedManagedBoundary> {
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
        bytes.extend_from_slice(b"ling.managed-island-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_managed_boundaries_are_complete_and_ordered() {
    let inventory = ManagedBoundaryInventory::new(PlannedManagedBoundary::ALL)
        .expect("planned Managed boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedManagedBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..38).collect::<Vec<_>>()
    );
}

#[test]
fn managed_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ManagedBoundaryInventory::new(PlannedManagedBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ManagedBoundaryInventory::new(PlannedManagedBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ManagedBoundaryInventory::new([
        PlannedManagedBoundary::ManagedGraph,
        PlannedManagedBoundary::ManagedGraph,
    ])
    .expect_err("duplicate Managed boundary must be rejected");
    assert_eq!(duplicate, PlannedManagedBoundary::ManagedGraph);
}

#[test]
fn managed_boundary_evidence_has_no_graph_or_isolation_authority() {
    let inventory = ManagedBoundaryInventory::new([
        PlannedManagedBoundary::ManagedIdentity,
        PlannedManagedBoundary::IslandRoot,
        PlannedManagedBoundary::Pinning,
        PlannedManagedBoundary::IslandEscape,
    ])
    .expect("bounded Managed evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.managed-island-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
