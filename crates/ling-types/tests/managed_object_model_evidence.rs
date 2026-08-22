//! Internal Managed object-model boundary evidence.
//!
//! This test-only inventory names the contracts that a future Managed object
//! model must settle. It does not implement object headers, metadata, roots,
//! collection, barriers, weak references, finalizers, allocation, or runtime
//! semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedObjectModelBoundary {
    ObjectIdentity,
    PrivateObjectHeader,
    TypeMetadata,
    RootInterface,
    RootRegistration,
    RootLifetime,
    HandleIdentity,
    Reachability,
    CycleHandling,
    SharedEdges,
    WriteBarrier,
    MutationOrdering,
    WeakReference,
    Finalization,
    OomFault,
    AllocationPolicy,
    PointerIdentity,
    AddressOpacity,
    ManagedValueBoundary,
    ResourceDropBoundary,
    OwnershipRegionBoundary,
    Pinning,
    BorrowedView,
    ExploreProfile,
    NativeProfile,
    CriticalProfile,
    FfiBoundary,
    CallbackRoot,
    ThreadAttachment,
    CheckedCoreProjection,
    SemanticGraphProjection,
    AuditSourceProjection,
    PublicProtocol,
    Diagnostic,
    UnicodeSourceSpans,
    DeterministicTraversal,
    InterpreterVmNativeDifferential,
    SeedMigration,
    RuntimeSecurityBoundary,
    CollectionBoundary,
}

impl PlannedObjectModelBoundary {
    const ALL: [Self; 40] = [
        Self::ObjectIdentity,
        Self::PrivateObjectHeader,
        Self::TypeMetadata,
        Self::RootInterface,
        Self::RootRegistration,
        Self::RootLifetime,
        Self::HandleIdentity,
        Self::Reachability,
        Self::CycleHandling,
        Self::SharedEdges,
        Self::WriteBarrier,
        Self::MutationOrdering,
        Self::WeakReference,
        Self::Finalization,
        Self::OomFault,
        Self::AllocationPolicy,
        Self::PointerIdentity,
        Self::AddressOpacity,
        Self::ManagedValueBoundary,
        Self::ResourceDropBoundary,
        Self::OwnershipRegionBoundary,
        Self::Pinning,
        Self::BorrowedView,
        Self::ExploreProfile,
        Self::NativeProfile,
        Self::CriticalProfile,
        Self::FfiBoundary,
        Self::CallbackRoot,
        Self::ThreadAttachment,
        Self::CheckedCoreProjection,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::PublicProtocol,
        Self::Diagnostic,
        Self::UnicodeSourceSpans,
        Self::DeterministicTraversal,
        Self::InterpreterVmNativeDifferential,
        Self::SeedMigration,
        Self::RuntimeSecurityBoundary,
        Self::CollectionBoundary,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ObjectIdentity => 0,
            Self::PrivateObjectHeader => 1,
            Self::TypeMetadata => 2,
            Self::RootInterface => 3,
            Self::RootRegistration => 4,
            Self::RootLifetime => 5,
            Self::HandleIdentity => 6,
            Self::Reachability => 7,
            Self::CycleHandling => 8,
            Self::SharedEdges => 9,
            Self::WriteBarrier => 10,
            Self::MutationOrdering => 11,
            Self::WeakReference => 12,
            Self::Finalization => 13,
            Self::OomFault => 14,
            Self::AllocationPolicy => 15,
            Self::PointerIdentity => 16,
            Self::AddressOpacity => 17,
            Self::ManagedValueBoundary => 18,
            Self::ResourceDropBoundary => 19,
            Self::OwnershipRegionBoundary => 20,
            Self::Pinning => 21,
            Self::BorrowedView => 22,
            Self::ExploreProfile => 23,
            Self::NativeProfile => 24,
            Self::CriticalProfile => 25,
            Self::FfiBoundary => 26,
            Self::CallbackRoot => 27,
            Self::ThreadAttachment => 28,
            Self::CheckedCoreProjection => 29,
            Self::SemanticGraphProjection => 30,
            Self::AuditSourceProjection => 31,
            Self::PublicProtocol => 32,
            Self::Diagnostic => 33,
            Self::UnicodeSourceSpans => 34,
            Self::DeterministicTraversal => 35,
            Self::InterpreterVmNativeDifferential => 36,
            Self::SeedMigration => 37,
            Self::RuntimeSecurityBoundary => 38,
            Self::CollectionBoundary => 39,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ObjectModelBoundaryInventory {
    boundaries: Box<[PlannedObjectModelBoundary]>,
}

impl ObjectModelBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedObjectModelBoundary>,
    ) -> Result<Self, PlannedObjectModelBoundary> {
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
        bytes.extend_from_slice(b"ling.managed-object-model-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_object_model_boundaries_are_complete_and_ordered() {
    let inventory = ObjectModelBoundaryInventory::new(PlannedObjectModelBoundary::ALL)
        .expect("planned object-model boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedObjectModelBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..40).collect::<Vec<_>>()
    );
}

#[test]
fn object_model_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ObjectModelBoundaryInventory::new(PlannedObjectModelBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ObjectModelBoundaryInventory::new(PlannedObjectModelBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ObjectModelBoundaryInventory::new([
        PlannedObjectModelBoundary::PrivateObjectHeader,
        PlannedObjectModelBoundary::PrivateObjectHeader,
    ])
    .expect_err("duplicate object-model boundary must be rejected");
    assert_eq!(duplicate, PlannedObjectModelBoundary::PrivateObjectHeader);
}

#[test]
fn object_model_evidence_has_no_runtime_authority() {
    let inventory = ObjectModelBoundaryInventory::new([
        PlannedObjectModelBoundary::PrivateObjectHeader,
        PlannedObjectModelBoundary::RootInterface,
        PlannedObjectModelBoundary::WriteBarrier,
        PlannedObjectModelBoundary::OomFault,
        PlannedObjectModelBoundary::PointerIdentity,
    ])
    .expect("bounded object-model evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.managed-object-model-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 5);
}
