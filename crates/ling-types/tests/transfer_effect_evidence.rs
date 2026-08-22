use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedTransferEffectBoundary {
    TransferSchema,
    TransferSyntax,
    TransferExpression,
    TransferSource,
    TransferDestination,
    TransferResult,
    EffectRow,
    EffectWitness,
    DeviceTransferCapability,
    CapabilityVersion,
    CapabilityIdentity,
    CapabilityDiscovery,
    TransferDirection,
    AddressSpace,
    SourceAddressSpace,
    DestinationAddressSpace,
    ByteCount,
    LogicalBytes,
    Layout,
    Shape,
    Alignment,
    Bounds,
    OwnershipTransition,
    CopyTransition,
    MoveTransition,
    BorrowTransition,
    ViewTransition,
    TransferToken,
    Synchronization,
    Visibility,
    Coherence,
    AsyncLifetime,
    Completion,
    Cancellation,
    HostPathExclusion,
    DriverLogExclusion,
    DeviceLoss,
    Fault,
    FaultSpan,
    Drop,
    ResourceLimit,
    CostEvidence,
    TypedCoreInput,
    VerifiedDerivative,
    SemanticId,
    SourceSpan,
    Utf8Spans,
    Unicode17,
    Determinism,
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
    ProtocolInventory,
}

impl PlannedTransferEffectBoundary {
    const ALL: [Self; 60] = [
        Self::TransferSchema,
        Self::TransferSyntax,
        Self::TransferExpression,
        Self::TransferSource,
        Self::TransferDestination,
        Self::TransferResult,
        Self::EffectRow,
        Self::EffectWitness,
        Self::DeviceTransferCapability,
        Self::CapabilityVersion,
        Self::CapabilityIdentity,
        Self::CapabilityDiscovery,
        Self::TransferDirection,
        Self::AddressSpace,
        Self::SourceAddressSpace,
        Self::DestinationAddressSpace,
        Self::ByteCount,
        Self::LogicalBytes,
        Self::Layout,
        Self::Shape,
        Self::Alignment,
        Self::Bounds,
        Self::OwnershipTransition,
        Self::CopyTransition,
        Self::MoveTransition,
        Self::BorrowTransition,
        Self::ViewTransition,
        Self::TransferToken,
        Self::Synchronization,
        Self::Visibility,
        Self::Coherence,
        Self::AsyncLifetime,
        Self::Completion,
        Self::Cancellation,
        Self::HostPathExclusion,
        Self::DriverLogExclusion,
        Self::DeviceLoss,
        Self::Fault,
        Self::FaultSpan,
        Self::Drop,
        Self::ResourceLimit,
        Self::CostEvidence,
        Self::TypedCoreInput,
        Self::VerifiedDerivative,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Utf8Spans,
        Self::Unicode17,
        Self::Determinism,
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
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TransferEffectInventory {
    boundaries: Box<[PlannedTransferEffectBoundary]>,
}

impl TransferEffectInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedTransferEffectBoundary>,
    ) -> Result<Self, PlannedTransferEffectBoundary> {
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
        let mut bytes = b"ling.transfer-effect-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_transfer_effect_boundaries_are_complete_and_ordered() {
    let inventory = TransferEffectInventory::new(PlannedTransferEffectBoundary::ALL)
        .expect("planned Transfer Effect boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedTransferEffectBoundary::ALL
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
fn transfer_effect_evidence_is_order_independent_and_duplicate_checked() {
    let forward = TransferEffectInventory::new(PlannedTransferEffectBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        TransferEffectInventory::new(PlannedTransferEffectBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = TransferEffectInventory::new([
        PlannedTransferEffectBoundary::TransferSchema,
        PlannedTransferEffectBoundary::TransferSchema,
    ])
    .expect_err("duplicate Transfer Effect boundary must be rejected");
    assert_eq!(duplicate, PlannedTransferEffectBoundary::TransferSchema);
}

#[test]
fn transfer_effect_evidence_has_no_transfer_authority() {
    let inventory = TransferEffectInventory::new([
        PlannedTransferEffectBoundary::TransferSchema,
        PlannedTransferEffectBoundary::TransferExpression,
        PlannedTransferEffectBoundary::EffectRow,
        PlannedTransferEffectBoundary::DeviceTransferCapability,
        PlannedTransferEffectBoundary::Synchronization,
        PlannedTransferEffectBoundary::Fault,
        PlannedTransferEffectBoundary::BilingualDiagnostic,
        PlannedTransferEffectBoundary::ProtocolInventory,
    ])
    .expect("bounded Transfer Effect evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.transfer-effect-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
