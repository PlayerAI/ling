use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedBufferOwnershipBoundary {
    OwnershipSchema,
    BufferOwnership,
    AddressSpaceOwnership,
    OwnershipState,
    OwnershipCategory,
    Copy,
    Move,
    Borrow,
    SharedRead,
    ExclusiveWrite,
    BorrowRegion,
    AliasProof,
    NoAliasProof,
    Subview,
    SubviewBounds,
    SubviewLayout,
    SubviewIdentity,
    Mapping,
    Pinning,
    Visibility,
    Coherence,
    TransferOwnership,
    TransferToken,
    AsyncLifetime,
    TransferCompletion,
    Cancellation,
    Drop,
    DropWait,
    DropCancel,
    DeviceLoss,
    Fault,
    FaultSpan,
    TaskCrossing,
    ActorCrossing,
    CrossingMove,
    CrossingBorrow,
    CrossingShare,
    CrossingReject,
    TypedCoreInput,
    VerifiedDerivative,
    EffectWitness,
    CapabilityWitness,
    ResourceLimit,
    SemanticId,
    SourceSpan,
    Utf8Spans,
    Unicode17,
    CanonicalOrdering,
    VersionCompatibility,
    PositiveFixture,
    NegativeFixture,
    PropertyFixture,
    CorruptionFixture,
    Migration,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    HostPathExclusion,
    DriverLogExclusion,
    ProtocolInventory,
}

impl PlannedBufferOwnershipBoundary {
    const ALL: [Self; 60] = [
        Self::OwnershipSchema,
        Self::BufferOwnership,
        Self::AddressSpaceOwnership,
        Self::OwnershipState,
        Self::OwnershipCategory,
        Self::Copy,
        Self::Move,
        Self::Borrow,
        Self::SharedRead,
        Self::ExclusiveWrite,
        Self::BorrowRegion,
        Self::AliasProof,
        Self::NoAliasProof,
        Self::Subview,
        Self::SubviewBounds,
        Self::SubviewLayout,
        Self::SubviewIdentity,
        Self::Mapping,
        Self::Pinning,
        Self::Visibility,
        Self::Coherence,
        Self::TransferOwnership,
        Self::TransferToken,
        Self::AsyncLifetime,
        Self::TransferCompletion,
        Self::Cancellation,
        Self::Drop,
        Self::DropWait,
        Self::DropCancel,
        Self::DeviceLoss,
        Self::Fault,
        Self::FaultSpan,
        Self::TaskCrossing,
        Self::ActorCrossing,
        Self::CrossingMove,
        Self::CrossingBorrow,
        Self::CrossingShare,
        Self::CrossingReject,
        Self::TypedCoreInput,
        Self::VerifiedDerivative,
        Self::EffectWitness,
        Self::CapabilityWitness,
        Self::ResourceLimit,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Utf8Spans,
        Self::Unicode17,
        Self::CanonicalOrdering,
        Self::VersionCompatibility,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PropertyFixture,
        Self::CorruptionFixture,
        Self::Migration,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::HostPathExclusion,
        Self::DriverLogExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BufferOwnershipInventory {
    boundaries: Box<[PlannedBufferOwnershipBoundary]>,
}

impl BufferOwnershipInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedBufferOwnershipBoundary>,
    ) -> Result<Self, PlannedBufferOwnershipBoundary> {
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
        let mut bytes = b"ling.buffer-ownership-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_buffer_ownership_boundaries_are_complete_and_ordered() {
    let inventory = BufferOwnershipInventory::new(PlannedBufferOwnershipBoundary::ALL)
        .expect("planned Buffer ownership boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedBufferOwnershipBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..60).collect::<Vec<_>>()
    );
}

#[test]
fn buffer_ownership_evidence_is_order_independent_and_duplicate_checked() {
    let forward = BufferOwnershipInventory::new(PlannedBufferOwnershipBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        BufferOwnershipInventory::new(PlannedBufferOwnershipBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = BufferOwnershipInventory::new([
        PlannedBufferOwnershipBoundary::OwnershipSchema,
        PlannedBufferOwnershipBoundary::OwnershipSchema,
    ])
    .expect_err("duplicate Buffer ownership boundary must be rejected");
    assert_eq!(duplicate, PlannedBufferOwnershipBoundary::OwnershipSchema);
}

#[test]
fn buffer_ownership_evidence_has_no_ownership_authority() {
    let inventory = BufferOwnershipInventory::new([
        PlannedBufferOwnershipBoundary::OwnershipSchema,
        PlannedBufferOwnershipBoundary::BufferOwnership,
        PlannedBufferOwnershipBoundary::Move,
        PlannedBufferOwnershipBoundary::Subview,
        PlannedBufferOwnershipBoundary::DropCancel,
        PlannedBufferOwnershipBoundary::TaskCrossing,
        PlannedBufferOwnershipBoundary::BilingualDiagnostic,
        PlannedBufferOwnershipBoundary::ProtocolInventory,
    ])
    .expect("bounded Buffer ownership evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.buffer-ownership-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
