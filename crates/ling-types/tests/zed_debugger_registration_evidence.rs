use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedZedDebuggerBoundary {
    ExtensionPackage,
    ManifestIdentity,
    ManifestVersion,
    LanguageConfig,
    DebuggerRegistration,
    AdapterDiscovery,
    AdapterInstallation,
    AdapterVersion,
    ZedVersion,
    DapVersion,
    LaunchMapping,
    AttachMapping,
    BuildTask,
    RunTask,
    RootSelection,
    SourceExtensionLing,
    ProjectRoot,
    WorkingDirectory,
    EnvironmentPropagation,
    ProfileSelection,
    TargetSelection,
    CapabilitySelection,
    SourceIdentity,
    BinaryIdentity,
    SessionOwnership,
    RestartPolicy,
    Cancellation,
    Timeout,
    PermissionBoundary,
    TrustBoundary,
    MissingExecutable,
    InvalidConfig,
    UnknownConfig,
    OfflineWorkflow,
    LockedDependencies,
    PlatformTarget,
    UpdateRollback,
    ErrorReporting,
    Utf8SourceMapping,
    SourceMap,
    BreakpointMapping,
    StepMapping,
    StackMapping,
    ScopeVariableMapping,
    FaultMapping,
    OwnershipResourceView,
    NativeVmChoice,
    TypedCoreInput,
    HostPathExclusion,
    AddressExclusion,
    RustLayoutExclusion,
    DebugStringExclusion,
    BilingualDiagnostic,
    Unicode17,
    SemanticIdPreservation,
    PositiveFixture,
    NegativeFixture,
    SmokeEvidence,
    DeterministicEvidence,
    PublicProtocolInventory,
}

impl PlannedZedDebuggerBoundary {
    const ALL: [Self; 60] = [
        Self::ExtensionPackage,
        Self::ManifestIdentity,
        Self::ManifestVersion,
        Self::LanguageConfig,
        Self::DebuggerRegistration,
        Self::AdapterDiscovery,
        Self::AdapterInstallation,
        Self::AdapterVersion,
        Self::ZedVersion,
        Self::DapVersion,
        Self::LaunchMapping,
        Self::AttachMapping,
        Self::BuildTask,
        Self::RunTask,
        Self::RootSelection,
        Self::SourceExtensionLing,
        Self::ProjectRoot,
        Self::WorkingDirectory,
        Self::EnvironmentPropagation,
        Self::ProfileSelection,
        Self::TargetSelection,
        Self::CapabilitySelection,
        Self::SourceIdentity,
        Self::BinaryIdentity,
        Self::SessionOwnership,
        Self::RestartPolicy,
        Self::Cancellation,
        Self::Timeout,
        Self::PermissionBoundary,
        Self::TrustBoundary,
        Self::MissingExecutable,
        Self::InvalidConfig,
        Self::UnknownConfig,
        Self::OfflineWorkflow,
        Self::LockedDependencies,
        Self::PlatformTarget,
        Self::UpdateRollback,
        Self::ErrorReporting,
        Self::Utf8SourceMapping,
        Self::SourceMap,
        Self::BreakpointMapping,
        Self::StepMapping,
        Self::StackMapping,
        Self::ScopeVariableMapping,
        Self::FaultMapping,
        Self::OwnershipResourceView,
        Self::NativeVmChoice,
        Self::TypedCoreInput,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::RustLayoutExclusion,
        Self::DebugStringExclusion,
        Self::BilingualDiagnostic,
        Self::Unicode17,
        Self::SemanticIdPreservation,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::SmokeEvidence,
        Self::DeterministicEvidence,
        Self::PublicProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ExtensionPackage => 0,
            Self::ManifestIdentity => 1,
            Self::ManifestVersion => 2,
            Self::LanguageConfig => 3,
            Self::DebuggerRegistration => 4,
            Self::AdapterDiscovery => 5,
            Self::AdapterInstallation => 6,
            Self::AdapterVersion => 7,
            Self::ZedVersion => 8,
            Self::DapVersion => 9,
            Self::LaunchMapping => 10,
            Self::AttachMapping => 11,
            Self::BuildTask => 12,
            Self::RunTask => 13,
            Self::RootSelection => 14,
            Self::SourceExtensionLing => 15,
            Self::ProjectRoot => 16,
            Self::WorkingDirectory => 17,
            Self::EnvironmentPropagation => 18,
            Self::ProfileSelection => 19,
            Self::TargetSelection => 20,
            Self::CapabilitySelection => 21,
            Self::SourceIdentity => 22,
            Self::BinaryIdentity => 23,
            Self::SessionOwnership => 24,
            Self::RestartPolicy => 25,
            Self::Cancellation => 26,
            Self::Timeout => 27,
            Self::PermissionBoundary => 28,
            Self::TrustBoundary => 29,
            Self::MissingExecutable => 30,
            Self::InvalidConfig => 31,
            Self::UnknownConfig => 32,
            Self::OfflineWorkflow => 33,
            Self::LockedDependencies => 34,
            Self::PlatformTarget => 35,
            Self::UpdateRollback => 36,
            Self::ErrorReporting => 37,
            Self::Utf8SourceMapping => 38,
            Self::SourceMap => 39,
            Self::BreakpointMapping => 40,
            Self::StepMapping => 41,
            Self::StackMapping => 42,
            Self::ScopeVariableMapping => 43,
            Self::FaultMapping => 44,
            Self::OwnershipResourceView => 45,
            Self::NativeVmChoice => 46,
            Self::TypedCoreInput => 47,
            Self::HostPathExclusion => 48,
            Self::AddressExclusion => 49,
            Self::RustLayoutExclusion => 50,
            Self::DebugStringExclusion => 51,
            Self::BilingualDiagnostic => 52,
            Self::Unicode17 => 53,
            Self::SemanticIdPreservation => 54,
            Self::PositiveFixture => 55,
            Self::NegativeFixture => 56,
            Self::SmokeEvidence => 57,
            Self::DeterministicEvidence => 58,
            Self::PublicProtocolInventory => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ZedDebuggerBoundaryInventory {
    boundaries: Box<[PlannedZedDebuggerBoundary]>,
}

impl ZedDebuggerBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedZedDebuggerBoundary>,
    ) -> Result<Self, PlannedZedDebuggerBoundary> {
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
        bytes.extend_from_slice(b"ling.zed-debugger-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_zed_debugger_boundaries_are_complete_and_ordered() {
    let inventory = ZedDebuggerBoundaryInventory::new(PlannedZedDebuggerBoundary::ALL)
        .expect("planned Zed debugger boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedZedDebuggerBoundary::ALL
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
fn zed_debugger_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ZedDebuggerBoundaryInventory::new(PlannedZedDebuggerBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ZedDebuggerBoundaryInventory::new(PlannedZedDebuggerBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ZedDebuggerBoundaryInventory::new([
        PlannedZedDebuggerBoundary::ExtensionPackage,
        PlannedZedDebuggerBoundary::ExtensionPackage,
    ])
    .expect_err("duplicate Zed debugger boundary must be rejected");
    assert_eq!(duplicate, PlannedZedDebuggerBoundary::ExtensionPackage);
}

#[test]
fn zed_debugger_evidence_has_no_extension_or_protocol_authority() {
    let inventory = ZedDebuggerBoundaryInventory::new([
        PlannedZedDebuggerBoundary::ExtensionPackage,
        PlannedZedDebuggerBoundary::DebuggerRegistration,
        PlannedZedDebuggerBoundary::LaunchMapping,
        PlannedZedDebuggerBoundary::SourceExtensionLing,
        PlannedZedDebuggerBoundary::TypedCoreInput,
        PlannedZedDebuggerBoundary::BilingualDiagnostic,
        PlannedZedDebuggerBoundary::Unicode17,
        PlannedZedDebuggerBoundary::PublicProtocolInventory,
    ])
    .expect("bounded Zed debugger evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.zed-debugger-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
