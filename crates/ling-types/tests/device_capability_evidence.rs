use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDeviceCapabilityBoundary {
    DeviceSchema,
    DeviceId,
    DeviceKind,
    DeviceCapability,
    CapabilityVersion,
    CapabilitySet,
    AddressSpace,
    DeviceAddressSpace,
    SharedAddressSpace,
    BufferType,
    BufferElementType,
    BufferShape,
    BufferLayout,
    BufferIdentity,
    ReadView,
    WriteView,
    TransferToken,
    Fence,
    Event,
    RawPointerReject,
    TypedCoreInput,
    VerifiedDerivative,
    Profile,
    Target,
    EffectWitness,
    CapabilityWitness,
    OwnershipWitness,
    AliasWitness,
    BoundsWitness,
    ResourceLimit,
    Transfer,
    TransferDirection,
    TransferOrdering,
    AsyncLifetime,
    Cancellation,
    Drop,
    DeviceLoss,
    Fault,
    FaultSpan,
    SemanticId,
    SourceSpan,
    Utf8Spans,
    Unicode17,
    CanonicalOrdering,
    VersionCompatibility,
    UnknownCapabilityReject,
    UnsupportedCapabilityReject,
    PositiveFixture,
    NegativeFixture,
    PropertyFixture,
    CorruptionFixture,
    Migration,
    CpuReference,
    DeviceDifferential,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    HostPathExclusion,
    DriverLogExclusion,
    ProtocolInventory,
}

impl PlannedDeviceCapabilityBoundary {
    const ALL: [Self; 60] = [
        Self::DeviceSchema,
        Self::DeviceId,
        Self::DeviceKind,
        Self::DeviceCapability,
        Self::CapabilityVersion,
        Self::CapabilitySet,
        Self::AddressSpace,
        Self::DeviceAddressSpace,
        Self::SharedAddressSpace,
        Self::BufferType,
        Self::BufferElementType,
        Self::BufferShape,
        Self::BufferLayout,
        Self::BufferIdentity,
        Self::ReadView,
        Self::WriteView,
        Self::TransferToken,
        Self::Fence,
        Self::Event,
        Self::RawPointerReject,
        Self::TypedCoreInput,
        Self::VerifiedDerivative,
        Self::Profile,
        Self::Target,
        Self::EffectWitness,
        Self::CapabilityWitness,
        Self::OwnershipWitness,
        Self::AliasWitness,
        Self::BoundsWitness,
        Self::ResourceLimit,
        Self::Transfer,
        Self::TransferDirection,
        Self::TransferOrdering,
        Self::AsyncLifetime,
        Self::Cancellation,
        Self::Drop,
        Self::DeviceLoss,
        Self::Fault,
        Self::FaultSpan,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Utf8Spans,
        Self::Unicode17,
        Self::CanonicalOrdering,
        Self::VersionCompatibility,
        Self::UnknownCapabilityReject,
        Self::UnsupportedCapabilityReject,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PropertyFixture,
        Self::CorruptionFixture,
        Self::Migration,
        Self::CpuReference,
        Self::DeviceDifferential,
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
struct DeviceCapabilityInventory {
    boundaries: Box<[PlannedDeviceCapabilityBoundary]>,
}

impl DeviceCapabilityInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDeviceCapabilityBoundary>,
    ) -> Result<Self, PlannedDeviceCapabilityBoundary> {
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
        let mut bytes = b"ling.device-capability-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_device_capability_boundaries_are_complete_and_ordered() {
    let inventory = DeviceCapabilityInventory::new(PlannedDeviceCapabilityBoundary::ALL)
        .expect("planned Device capability boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDeviceCapabilityBoundary::ALL
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
fn device_capability_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DeviceCapabilityInventory::new(PlannedDeviceCapabilityBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        DeviceCapabilityInventory::new(PlannedDeviceCapabilityBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DeviceCapabilityInventory::new([
        PlannedDeviceCapabilityBoundary::DeviceSchema,
        PlannedDeviceCapabilityBoundary::DeviceSchema,
    ])
    .expect_err("duplicate Device capability boundary must be rejected");
    assert_eq!(duplicate, PlannedDeviceCapabilityBoundary::DeviceSchema);
}

#[test]
fn device_capability_evidence_has_no_device_api_authority() {
    let inventory = DeviceCapabilityInventory::new([
        PlannedDeviceCapabilityBoundary::DeviceSchema,
        PlannedDeviceCapabilityBoundary::DeviceId,
        PlannedDeviceCapabilityBoundary::DeviceCapability,
        PlannedDeviceCapabilityBoundary::RawPointerReject,
        PlannedDeviceCapabilityBoundary::TransferToken,
        PlannedDeviceCapabilityBoundary::DeviceDifferential,
        PlannedDeviceCapabilityBoundary::BilingualDiagnostic,
        PlannedDeviceCapabilityBoundary::ProtocolInventory,
    ])
    .expect("bounded Device capability evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.device-capability-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
