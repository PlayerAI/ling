use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedBackendSpikeSelectionBoundary {
    BackendSpike,
    CandidatePath,
    Spirv,
    Vulkan,
    Wgsl,
    WebGpu,
    Cuda,
    Ptx,
    MlirBridge,
    VendorSdk,
    InputArtifact,
    VerifiedArtifact,
    DeviceIrBoundary,
    KernelBoundary,
    TargetIdentity,
    TargetProfile,
    CapabilityDiscovery,
    RequiredFeature,
    OptionalFeature,
    UnsupportedTarget,
    FallbackPolicy,
    RejectionPolicy,
    PlatformCoverage,
    CompilerApi,
    RuntimeApi,
    SourceDebugSupport,
    NumericControl,
    Determinism,
    Synchronization,
    TransferEffects,
    FaultMapping,
    ResourceLimits,
    LaunchContract,
    AbiBoundary,
    ToolchainIdentity,
    DriverIdentity,
    HardwareAvailability,
    CiEvidence,
    LicenseEvidence,
    Reproducibility,
    CacheIdentity,
    MaintenanceCost,
    BenchmarkEvidence,
    DifferentialEvidence,
    PositiveFixture,
    NegativeFixture,
    CrossTargetFixture,
    MalformedFixture,
    UnicodeFixture,
    MigrationFixture,
    ExperimentalStatus,
    PreviewStatus,
    SupportedStatus,
    DiagnosticCode,
    DiagnosticFacts,
    TimestampExclusion,
    AddressExclusion,
    HostPathExclusion,
    DriverPathExclusion,
    ProtocolInventory,
}

impl PlannedBackendSpikeSelectionBoundary {
    const ALL: [Self; 60] = [
        Self::BackendSpike,
        Self::CandidatePath,
        Self::Spirv,
        Self::Vulkan,
        Self::Wgsl,
        Self::WebGpu,
        Self::Cuda,
        Self::Ptx,
        Self::MlirBridge,
        Self::VendorSdk,
        Self::InputArtifact,
        Self::VerifiedArtifact,
        Self::DeviceIrBoundary,
        Self::KernelBoundary,
        Self::TargetIdentity,
        Self::TargetProfile,
        Self::CapabilityDiscovery,
        Self::RequiredFeature,
        Self::OptionalFeature,
        Self::UnsupportedTarget,
        Self::FallbackPolicy,
        Self::RejectionPolicy,
        Self::PlatformCoverage,
        Self::CompilerApi,
        Self::RuntimeApi,
        Self::SourceDebugSupport,
        Self::NumericControl,
        Self::Determinism,
        Self::Synchronization,
        Self::TransferEffects,
        Self::FaultMapping,
        Self::ResourceLimits,
        Self::LaunchContract,
        Self::AbiBoundary,
        Self::ToolchainIdentity,
        Self::DriverIdentity,
        Self::HardwareAvailability,
        Self::CiEvidence,
        Self::LicenseEvidence,
        Self::Reproducibility,
        Self::CacheIdentity,
        Self::MaintenanceCost,
        Self::BenchmarkEvidence,
        Self::DifferentialEvidence,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CrossTargetFixture,
        Self::MalformedFixture,
        Self::UnicodeFixture,
        Self::MigrationFixture,
        Self::ExperimentalStatus,
        Self::PreviewStatus,
        Self::SupportedStatus,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::TimestampExclusion,
        Self::AddressExclusion,
        Self::HostPathExclusion,
        Self::DriverPathExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BackendSpikeSelectionInventory {
    boundaries: Box<[PlannedBackendSpikeSelectionBoundary]>,
}

impl BackendSpikeSelectionInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedBackendSpikeSelectionBoundary>,
    ) -> Result<Self, PlannedBackendSpikeSelectionBoundary> {
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
        let mut bytes = b"ling.backend-spike-selection-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_backend_spike_selection_boundaries_are_complete_and_ordered() {
    let inventory = BackendSpikeSelectionInventory::new(PlannedBackendSpikeSelectionBoundary::ALL)
        .expect("planned backend spike and selection boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedBackendSpikeSelectionBoundary::ALL
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
fn backend_spike_selection_evidence_is_order_independent_and_duplicate_checked() {
    let forward = BackendSpikeSelectionInventory::new(PlannedBackendSpikeSelectionBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = BackendSpikeSelectionInventory::new(
        PlannedBackendSpikeSelectionBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = BackendSpikeSelectionInventory::new([
        PlannedBackendSpikeSelectionBoundary::BackendSpike,
        PlannedBackendSpikeSelectionBoundary::BackendSpike,
    ])
    .expect_err("duplicate backend spike boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedBackendSpikeSelectionBoundary::BackendSpike
    );
}

#[test]
fn backend_spike_selection_evidence_has_no_backend_authority() {
    let inventory = BackendSpikeSelectionInventory::new([
        PlannedBackendSpikeSelectionBoundary::BackendSpike,
        PlannedBackendSpikeSelectionBoundary::Spirv,
        PlannedBackendSpikeSelectionBoundary::Wgsl,
        PlannedBackendSpikeSelectionBoundary::Cuda,
        PlannedBackendSpikeSelectionBoundary::CapabilityDiscovery,
        PlannedBackendSpikeSelectionBoundary::DifferentialEvidence,
        PlannedBackendSpikeSelectionBoundary::DiagnosticFacts,
        PlannedBackendSpikeSelectionBoundary::ProtocolInventory,
    ])
    .expect("bounded backend spike and selection evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.backend-spike-selection-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
