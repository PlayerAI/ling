use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedAcceleratorPluginInterfaceBoundary {
    PluginBoundary,
    VerifiedDeviceIr,
    VerifiedKernelCore,
    InputValidation,
    PluginAbi,
    PluginVersion,
    VersionNegotiation,
    DeclarationSchema,
    SupportedOps,
    SupportedTypes,
    ShapeConstraints,
    LayoutConstraints,
    NumericModes,
    Determinism,
    DeviceCapabilities,
    CapabilityIdentity,
    TargetIdentity,
    CacheIdentity,
    CacheInvalidation,
    FallbackPolicy,
    RejectionPolicy,
    FaultPropagation,
    DiagnosticMapper,
    ResourceOwnership,
    Cleanup,
    Cancellation,
    HostIsolation,
    FrontendIsolation,
    PluginIsolation,
    TrustBoundary,
    LoadingPolicy,
    Dependency,
    Signature,
    Provenance,
    License,
    OfflineBuild,
    Sandbox,
    ExperimentalStatus,
    SupportedStatus,
    UnsupportedStatus,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    MigrationFixture,
    SourceMapFixture,
    UnicodeFixture,
    DeterminismFixture,
    CacheFixture,
    CapabilityFixture,
    FallbackFixture,
    SecurityFixture,
    DifferentialFixture,
    DiagnosticCode,
    DiagnosticFacts,
    HostPathExclusion,
    AddressExclusion,
    TimestampExclusion,
    DriverTextExclusion,
    DebugOutputExclusion,
    ProtocolInventory,
}

impl PlannedAcceleratorPluginInterfaceBoundary {
    const ALL: [Self; 60] = [
        Self::PluginBoundary,
        Self::VerifiedDeviceIr,
        Self::VerifiedKernelCore,
        Self::InputValidation,
        Self::PluginAbi,
        Self::PluginVersion,
        Self::VersionNegotiation,
        Self::DeclarationSchema,
        Self::SupportedOps,
        Self::SupportedTypes,
        Self::ShapeConstraints,
        Self::LayoutConstraints,
        Self::NumericModes,
        Self::Determinism,
        Self::DeviceCapabilities,
        Self::CapabilityIdentity,
        Self::TargetIdentity,
        Self::CacheIdentity,
        Self::CacheInvalidation,
        Self::FallbackPolicy,
        Self::RejectionPolicy,
        Self::FaultPropagation,
        Self::DiagnosticMapper,
        Self::ResourceOwnership,
        Self::Cleanup,
        Self::Cancellation,
        Self::HostIsolation,
        Self::FrontendIsolation,
        Self::PluginIsolation,
        Self::TrustBoundary,
        Self::LoadingPolicy,
        Self::Dependency,
        Self::Signature,
        Self::Provenance,
        Self::License,
        Self::OfflineBuild,
        Self::Sandbox,
        Self::ExperimentalStatus,
        Self::SupportedStatus,
        Self::UnsupportedStatus,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::MigrationFixture,
        Self::SourceMapFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::CacheFixture,
        Self::CapabilityFixture,
        Self::FallbackFixture,
        Self::SecurityFixture,
        Self::DifferentialFixture,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::DriverTextExclusion,
        Self::DebugOutputExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AcceleratorPluginInterfaceInventory {
    boundaries: Box<[PlannedAcceleratorPluginInterfaceBoundary]>,
}

impl AcceleratorPluginInterfaceInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedAcceleratorPluginInterfaceBoundary>,
    ) -> Result<Self, PlannedAcceleratorPluginInterfaceBoundary> {
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
        let mut bytes = b"ling.accelerator-plugin-interface-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_accelerator_plugin_interface_boundaries_are_complete_and_ordered() {
    let inventory =
        AcceleratorPluginInterfaceInventory::new(PlannedAcceleratorPluginInterfaceBoundary::ALL)
            .expect("planned accelerator plugin interface boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedAcceleratorPluginInterfaceBoundary::ALL
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
fn accelerator_plugin_interface_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        AcceleratorPluginInterfaceInventory::new(PlannedAcceleratorPluginInterfaceBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = AcceleratorPluginInterfaceInventory::new(
        PlannedAcceleratorPluginInterfaceBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = AcceleratorPluginInterfaceInventory::new([
        PlannedAcceleratorPluginInterfaceBoundary::PluginBoundary,
        PlannedAcceleratorPluginInterfaceBoundary::PluginBoundary,
    ])
    .expect_err("duplicate accelerator plugin boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedAcceleratorPluginInterfaceBoundary::PluginBoundary
    );
}

#[test]
fn accelerator_plugin_interface_evidence_has_no_plugin_authority() {
    let inventory = AcceleratorPluginInterfaceInventory::new([
        PlannedAcceleratorPluginInterfaceBoundary::PluginBoundary,
        PlannedAcceleratorPluginInterfaceBoundary::VerifiedDeviceIr,
        PlannedAcceleratorPluginInterfaceBoundary::DeclarationSchema,
        PlannedAcceleratorPluginInterfaceBoundary::SupportedOps,
        PlannedAcceleratorPluginInterfaceBoundary::DeviceCapabilities,
        PlannedAcceleratorPluginInterfaceBoundary::ExperimentalStatus,
        PlannedAcceleratorPluginInterfaceBoundary::DiagnosticMapper,
        PlannedAcceleratorPluginInterfaceBoundary::ProtocolInventory,
    ])
    .expect("bounded accelerator plugin interface evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.accelerator-plugin-interface-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
