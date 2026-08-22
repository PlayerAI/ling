use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedBackendAdapterBoundary {
    AdapterBoundary,
    Compile,
    DeviceIrInput,
    TargetSpec,
    DeviceBinary,
    VersionNegotiation,
    BinarySchema,
    BinaryCanonicalBytes,
    BinaryOwnership,
    BinaryCorruption,
    BinaryMigration,
    CacheIdentity,
    CacheInvalidation,
    Allocate,
    BufferHandle,
    BufferOwnership,
    Transfer,
    TransferVisibility,
    Queue,
    Launch,
    LaunchDimensions,
    WorkgroupLimits,
    Synchronization,
    SyncScope,
    CapabilityQuery,
    CapabilityIdentity,
    RequiredFeature,
    OptionalFeature,
    UnsupportedFeature,
    Fallback,
    Rejection,
    Fault,
    FaultCode,
    DeviceLoss,
    Cancellation,
    ResourceLimit,
    Cleanup,
    VendorIsolation,
    FrontendIsolation,
    TargetIdentity,
    ToolchainIdentity,
    DriverIdentity,
    NumericMode,
    Determinism,
    SourceMap,
    Utf8Spans,
    SemanticId,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    MigrationFixture,
    LifecycleFixture,
    DifferentialFixture,
    DiagnosticCode,
    DiagnosticFacts,
    HostPathExclusion,
    DriverPathExclusion,
    AddressExclusion,
    TimestampExclusion,
    ProtocolInventory,
}

impl PlannedBackendAdapterBoundary {
    const ALL: [Self; 60] = [
        Self::AdapterBoundary,
        Self::Compile,
        Self::DeviceIrInput,
        Self::TargetSpec,
        Self::DeviceBinary,
        Self::VersionNegotiation,
        Self::BinarySchema,
        Self::BinaryCanonicalBytes,
        Self::BinaryOwnership,
        Self::BinaryCorruption,
        Self::BinaryMigration,
        Self::CacheIdentity,
        Self::CacheInvalidation,
        Self::Allocate,
        Self::BufferHandle,
        Self::BufferOwnership,
        Self::Transfer,
        Self::TransferVisibility,
        Self::Queue,
        Self::Launch,
        Self::LaunchDimensions,
        Self::WorkgroupLimits,
        Self::Synchronization,
        Self::SyncScope,
        Self::CapabilityQuery,
        Self::CapabilityIdentity,
        Self::RequiredFeature,
        Self::OptionalFeature,
        Self::UnsupportedFeature,
        Self::Fallback,
        Self::Rejection,
        Self::Fault,
        Self::FaultCode,
        Self::DeviceLoss,
        Self::Cancellation,
        Self::ResourceLimit,
        Self::Cleanup,
        Self::VendorIsolation,
        Self::FrontendIsolation,
        Self::TargetIdentity,
        Self::ToolchainIdentity,
        Self::DriverIdentity,
        Self::NumericMode,
        Self::Determinism,
        Self::SourceMap,
        Self::Utf8Spans,
        Self::SemanticId,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::MigrationFixture,
        Self::LifecycleFixture,
        Self::DifferentialFixture,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::HostPathExclusion,
        Self::DriverPathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BackendAdapterInventory {
    boundaries: Box<[PlannedBackendAdapterBoundary]>,
}

impl BackendAdapterInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedBackendAdapterBoundary>,
    ) -> Result<Self, PlannedBackendAdapterBoundary> {
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
        let mut bytes = b"ling.backend-adapter-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_backend_adapter_boundaries_are_complete_and_ordered() {
    let inventory = BackendAdapterInventory::new(PlannedBackendAdapterBoundary::ALL)
        .expect("planned backend adapter boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedBackendAdapterBoundary::ALL
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
fn backend_adapter_evidence_is_order_independent_and_duplicate_checked() {
    let forward = BackendAdapterInventory::new(PlannedBackendAdapterBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        BackendAdapterInventory::new(PlannedBackendAdapterBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = BackendAdapterInventory::new([
        PlannedBackendAdapterBoundary::AdapterBoundary,
        PlannedBackendAdapterBoundary::AdapterBoundary,
    ])
    .expect_err("duplicate backend adapter boundary must be rejected");
    assert_eq!(duplicate, PlannedBackendAdapterBoundary::AdapterBoundary);
}

#[test]
fn backend_adapter_evidence_has_no_adapter_authority() {
    let inventory = BackendAdapterInventory::new([
        PlannedBackendAdapterBoundary::AdapterBoundary,
        PlannedBackendAdapterBoundary::Compile,
        PlannedBackendAdapterBoundary::DeviceIrInput,
        PlannedBackendAdapterBoundary::DeviceBinary,
        PlannedBackendAdapterBoundary::Allocate,
        PlannedBackendAdapterBoundary::Launch,
        PlannedBackendAdapterBoundary::Fault,
        PlannedBackendAdapterBoundary::ProtocolInventory,
    ])
    .expect("bounded backend adapter evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.backend-adapter-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
