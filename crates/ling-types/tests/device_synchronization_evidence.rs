use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDeviceSynchronizationBoundary {
    SynchronizationSchema,
    CommandQueue,
    QueueIdentity,
    QueueSubmission,
    QueueOwnership,
    QueueCapability,
    Event,
    EventIdentity,
    EventScope,
    EventCompletion,
    Fence,
    FenceIdentity,
    FenceSignal,
    HostAwait,
    DeviceBarrier,
    BarrierScope,
    BarrierOrdering,
    CrossQueue,
    HappensBefore,
    MemoryVisibility,
    Acquire,
    Release,
    BufferHazard,
    ReadHazard,
    WriteHazard,
    ReadWriteConflict,
    HazardProof,
    BufferIdentity,
    Subview,
    TransferDependency,
    Cancellation,
    Timeout,
    DeviceLoss,
    Fault,
    FaultSpan,
    Cleanup,
    CommittedEffect,
    ResourceLimit,
    TypedCoreInput,
    VerifiedDerivative,
    EffectWitness,
    CapabilityWitness,
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
    HostThreadExclusion,
    DriverLogExclusion,
    ProtocolInventory,
}

impl PlannedDeviceSynchronizationBoundary {
    const ALL: [Self; 60] = [
        Self::SynchronizationSchema,
        Self::CommandQueue,
        Self::QueueIdentity,
        Self::QueueSubmission,
        Self::QueueOwnership,
        Self::QueueCapability,
        Self::Event,
        Self::EventIdentity,
        Self::EventScope,
        Self::EventCompletion,
        Self::Fence,
        Self::FenceIdentity,
        Self::FenceSignal,
        Self::HostAwait,
        Self::DeviceBarrier,
        Self::BarrierScope,
        Self::BarrierOrdering,
        Self::CrossQueue,
        Self::HappensBefore,
        Self::MemoryVisibility,
        Self::Acquire,
        Self::Release,
        Self::BufferHazard,
        Self::ReadHazard,
        Self::WriteHazard,
        Self::ReadWriteConflict,
        Self::HazardProof,
        Self::BufferIdentity,
        Self::Subview,
        Self::TransferDependency,
        Self::Cancellation,
        Self::Timeout,
        Self::DeviceLoss,
        Self::Fault,
        Self::FaultSpan,
        Self::Cleanup,
        Self::CommittedEffect,
        Self::ResourceLimit,
        Self::TypedCoreInput,
        Self::VerifiedDerivative,
        Self::EffectWitness,
        Self::CapabilityWitness,
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
        Self::HostThreadExclusion,
        Self::DriverLogExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DeviceSynchronizationInventory {
    boundaries: Box<[PlannedDeviceSynchronizationBoundary]>,
}

impl DeviceSynchronizationInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDeviceSynchronizationBoundary>,
    ) -> Result<Self, PlannedDeviceSynchronizationBoundary> {
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
        let mut bytes = b"ling.device-synchronization-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_device_synchronization_boundaries_are_complete_and_ordered() {
    let inventory = DeviceSynchronizationInventory::new(PlannedDeviceSynchronizationBoundary::ALL)
        .expect("planned Device synchronization boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDeviceSynchronizationBoundary::ALL
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
fn device_synchronization_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DeviceSynchronizationInventory::new(PlannedDeviceSynchronizationBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = DeviceSynchronizationInventory::new(
        PlannedDeviceSynchronizationBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DeviceSynchronizationInventory::new([
        PlannedDeviceSynchronizationBoundary::SynchronizationSchema,
        PlannedDeviceSynchronizationBoundary::SynchronizationSchema,
    ])
    .expect_err("duplicate Device synchronization boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedDeviceSynchronizationBoundary::SynchronizationSchema
    );
}

#[test]
fn device_synchronization_evidence_has_no_runtime_authority() {
    let inventory = DeviceSynchronizationInventory::new([
        PlannedDeviceSynchronizationBoundary::SynchronizationSchema,
        PlannedDeviceSynchronizationBoundary::CommandQueue,
        PlannedDeviceSynchronizationBoundary::Event,
        PlannedDeviceSynchronizationBoundary::Fence,
        PlannedDeviceSynchronizationBoundary::BufferHazard,
        PlannedDeviceSynchronizationBoundary::Cancellation,
        PlannedDeviceSynchronizationBoundary::BilingualDiagnostic,
        PlannedDeviceSynchronizationBoundary::ProtocolInventory,
    ])
    .expect("bounded Device synchronization evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.device-synchronization-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
