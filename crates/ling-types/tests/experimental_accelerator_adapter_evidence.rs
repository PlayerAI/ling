use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedExperimentalAcceleratorAdapterBoundary {
    ExperimentalAdapter,
    ExperimentalStatus,
    VerifiedArtifact,
    DeviceIrBoundary,
    KernelCoreBoundary,
    PluginAbi,
    InputValidation,
    SupportedOps,
    SupportedTypes,
    ShapeConstraints,
    LayoutConstraints,
    NumericMode,
    Determinism,
    CapabilityIdentity,
    TargetIdentity,
    ResourceOwnership,
    Synchronization,
    Fault,
    FallbackPolicy,
    RejectionPolicy,
    CacheIdentity,
    Versioning,
    LimitationSet,
    EvidenceRequirement,
    Reproducibility,
    Promotion,
    Deprecation,
    Revocation,
    Removal,
    FrontendIsolation,
    VendorGraphExclusion,
    TrustBoundary,
    DependencyAudit,
    Signature,
    Provenance,
    Sandbox,
    License,
    OfflineBuild,
    HostDeviceIsolation,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    MigrationFixture,
    CapabilityFixture,
    CacheFixture,
    SecurityFixture,
    SourceMapFixture,
    UnicodeFixture,
    DeterminismFixture,
    DifferentialFixture,
    LifecycleFixture,
    DiagnosticCode,
    DiagnosticFacts,
    HostPathExclusion,
    AddressExclusion,
    TimestampExclusion,
    DriverTextExclusion,
    DebugOutputExclusion,
    PublicSupportExclusion,
    ProtocolInventory,
}

impl PlannedExperimentalAcceleratorAdapterBoundary {
    const ALL: [Self; 60] = [
        Self::ExperimentalAdapter,
        Self::ExperimentalStatus,
        Self::VerifiedArtifact,
        Self::DeviceIrBoundary,
        Self::KernelCoreBoundary,
        Self::PluginAbi,
        Self::InputValidation,
        Self::SupportedOps,
        Self::SupportedTypes,
        Self::ShapeConstraints,
        Self::LayoutConstraints,
        Self::NumericMode,
        Self::Determinism,
        Self::CapabilityIdentity,
        Self::TargetIdentity,
        Self::ResourceOwnership,
        Self::Synchronization,
        Self::Fault,
        Self::FallbackPolicy,
        Self::RejectionPolicy,
        Self::CacheIdentity,
        Self::Versioning,
        Self::LimitationSet,
        Self::EvidenceRequirement,
        Self::Reproducibility,
        Self::Promotion,
        Self::Deprecation,
        Self::Revocation,
        Self::Removal,
        Self::FrontendIsolation,
        Self::VendorGraphExclusion,
        Self::TrustBoundary,
        Self::DependencyAudit,
        Self::Signature,
        Self::Provenance,
        Self::Sandbox,
        Self::License,
        Self::OfflineBuild,
        Self::HostDeviceIsolation,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::MigrationFixture,
        Self::CapabilityFixture,
        Self::CacheFixture,
        Self::SecurityFixture,
        Self::SourceMapFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::DifferentialFixture,
        Self::LifecycleFixture,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::DriverTextExclusion,
        Self::DebugOutputExclusion,
        Self::PublicSupportExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExperimentalAcceleratorAdapterInventory {
    boundaries: Box<[PlannedExperimentalAcceleratorAdapterBoundary]>,
}

impl ExperimentalAcceleratorAdapterInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedExperimentalAcceleratorAdapterBoundary>,
    ) -> Result<Self, PlannedExperimentalAcceleratorAdapterBoundary> {
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
        let mut bytes = b"ling.experimental-accelerator-adapter-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_experimental_accelerator_adapter_boundaries_are_complete_and_ordered() {
    let inventory = ExperimentalAcceleratorAdapterInventory::new(
        PlannedExperimentalAcceleratorAdapterBoundary::ALL,
    )
    .expect("planned experimental accelerator adapter boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedExperimentalAcceleratorAdapterBoundary::ALL
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
fn experimental_accelerator_adapter_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ExperimentalAcceleratorAdapterInventory::new(
        PlannedExperimentalAcceleratorAdapterBoundary::ALL,
    )
    .expect("forward inventory")
    .canonical_bytes();
    let reverse = ExperimentalAcceleratorAdapterInventory::new(
        PlannedExperimentalAcceleratorAdapterBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ExperimentalAcceleratorAdapterInventory::new([
        PlannedExperimentalAcceleratorAdapterBoundary::ExperimentalAdapter,
        PlannedExperimentalAcceleratorAdapterBoundary::ExperimentalAdapter,
    ])
    .expect_err("duplicate experimental accelerator boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedExperimentalAcceleratorAdapterBoundary::ExperimentalAdapter
    );
}

#[test]
fn experimental_accelerator_adapter_evidence_has_no_adapter_authority() {
    let inventory = ExperimentalAcceleratorAdapterInventory::new([
        PlannedExperimentalAcceleratorAdapterBoundary::ExperimentalAdapter,
        PlannedExperimentalAcceleratorAdapterBoundary::ExperimentalStatus,
        PlannedExperimentalAcceleratorAdapterBoundary::VerifiedArtifact,
        PlannedExperimentalAcceleratorAdapterBoundary::PluginAbi,
        PlannedExperimentalAcceleratorAdapterBoundary::LimitationSet,
        PlannedExperimentalAcceleratorAdapterBoundary::VendorGraphExclusion,
        PlannedExperimentalAcceleratorAdapterBoundary::PublicSupportExclusion,
        PlannedExperimentalAcceleratorAdapterBoundary::ProtocolInventory,
    ])
    .expect("bounded experimental accelerator adapter evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.experimental-accelerator-adapter-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
