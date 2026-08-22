use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedKernelAliasBoundary {
    AliasSchema,
    AliasIdentity,
    ReadAlias,
    WriteAlias,
    SharedAlias,
    MutableAlias,
    BorrowScope,
    OwnershipTransfer,
    BorrowEnd,
    ParallelWrite,
    WriteReadConflict,
    ReadWriteConflict,
    WriteWriteConflict,
    DisjointProof,
    OverlapProof,
    RangeProof,
    ShapeProof,
    IndexProof,
    BoundsProof,
    AddressSpace,
    BufferOwnership,
    DeviceCapability,
    KernelProfile,
    TargetScope,
    EffectRow,
    CapabilityRow,
    TaskReject,
    ActorReject,
    IoReject,
    NetworkReject,
    Synchronization,
    Barrier,
    Atomicity,
    RaceDetection,
    Determinism,
    SchedulingOrder,
    MemoryOrdering,
    Mutation,
    LoopCarriedAlias,
    CallAlias,
    RecursionAlias,
    TypedCoreInput,
    VerifiedDerivative,
    DiagnosticCode,
    DiagnosticFacts,
    Utf8Spans,
    SemanticId,
    Unicode17,
    PositiveFixture,
    NegativeFixture,
    UnknownAliasReject,
    ConflictingAliasReject,
    Migration,
    CanonicalOrdering,
    CrossModule,
    CrossPackage,
    CpuReference,
    DeviceDifferential,
    HostPathExclusion,
    ProtocolInventory,
}

impl PlannedKernelAliasBoundary {
    const ALL: [Self; 60] = [
        Self::AliasSchema,
        Self::AliasIdentity,
        Self::ReadAlias,
        Self::WriteAlias,
        Self::SharedAlias,
        Self::MutableAlias,
        Self::BorrowScope,
        Self::OwnershipTransfer,
        Self::BorrowEnd,
        Self::ParallelWrite,
        Self::WriteReadConflict,
        Self::ReadWriteConflict,
        Self::WriteWriteConflict,
        Self::DisjointProof,
        Self::OverlapProof,
        Self::RangeProof,
        Self::ShapeProof,
        Self::IndexProof,
        Self::BoundsProof,
        Self::AddressSpace,
        Self::BufferOwnership,
        Self::DeviceCapability,
        Self::KernelProfile,
        Self::TargetScope,
        Self::EffectRow,
        Self::CapabilityRow,
        Self::TaskReject,
        Self::ActorReject,
        Self::IoReject,
        Self::NetworkReject,
        Self::Synchronization,
        Self::Barrier,
        Self::Atomicity,
        Self::RaceDetection,
        Self::Determinism,
        Self::SchedulingOrder,
        Self::MemoryOrdering,
        Self::Mutation,
        Self::LoopCarriedAlias,
        Self::CallAlias,
        Self::RecursionAlias,
        Self::TypedCoreInput,
        Self::VerifiedDerivative,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::Utf8Spans,
        Self::SemanticId,
        Self::Unicode17,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::UnknownAliasReject,
        Self::ConflictingAliasReject,
        Self::Migration,
        Self::CanonicalOrdering,
        Self::CrossModule,
        Self::CrossPackage,
        Self::CpuReference,
        Self::DeviceDifferential,
        Self::HostPathExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct KernelAliasInventory {
    boundaries: Box<[PlannedKernelAliasBoundary]>,
}

impl KernelAliasInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedKernelAliasBoundary>,
    ) -> Result<Self, PlannedKernelAliasBoundary> {
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
        let mut bytes = b"ling.kernel-alias-write-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_kernel_alias_boundaries_are_complete_and_ordered() {
    let inventory = KernelAliasInventory::new(PlannedKernelAliasBoundary::ALL)
        .expect("planned Kernel alias boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedKernelAliasBoundary::ALL
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
fn kernel_alias_evidence_is_order_independent_and_duplicate_checked() {
    let forward = KernelAliasInventory::new(PlannedKernelAliasBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = KernelAliasInventory::new(PlannedKernelAliasBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);
    let duplicate = KernelAliasInventory::new([
        PlannedKernelAliasBoundary::AliasSchema,
        PlannedKernelAliasBoundary::AliasSchema,
    ])
    .expect_err("duplicate Kernel alias boundary must be rejected");
    assert_eq!(duplicate, PlannedKernelAliasBoundary::AliasSchema);
}

#[test]
fn kernel_alias_evidence_has_no_conflict_authority() {
    let inventory = KernelAliasInventory::new([
        PlannedKernelAliasBoundary::AliasSchema,
        PlannedKernelAliasBoundary::WriteWriteConflict,
        PlannedKernelAliasBoundary::RaceDetection,
        PlannedKernelAliasBoundary::TypedCoreInput,
        PlannedKernelAliasBoundary::CpuReference,
        PlannedKernelAliasBoundary::DiagnosticCode,
        PlannedKernelAliasBoundary::Unicode17,
        PlannedKernelAliasBoundary::ProtocolInventory,
    ])
    .expect("bounded Kernel alias evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.kernel-alias-write-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
