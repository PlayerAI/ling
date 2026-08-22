use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNativeBackendSelectionBoundary {
    CandidateCranelift,
    CandidateLlvm,
    CandidateCTransition,
    CandidateWasmTransition,
    ComparisonOnly,
    NirInput,
    AbiInput,
    TargetMatrix,
    ProfileMatrix,
    CoreEligibility,
    UnsupportedForm,
    ToolchainVersion,
    CompilerVersion,
    TargetTriple,
    CompilerFlags,
    StandardLibrary,
    LinkerVersion,
    RuntimeLibrary,
    ColdBuild,
    WarmBuild,
    ResourceTimeBounds,
    DebugInfo,
    SourceMap,
    JitMode,
    AotMode,
    TargetCoverage,
    LicenseReview,
    SupplyChain,
    TcbBoundary,
    OfflineLock,
    GeneratedCode,
    ReproducibleInputs,
    BuildEnvironment,
    HostPathExclusion,
    ClockNoiseExclusion,
    AddressExclusion,
    MapOrderExclusion,
    DeterministicCorpus,
    DeterministicMetrics,
    CrossTargetMatrix,
    SemanticPreservation,
    AbiFfiEvidence,
    FaultUnwind,
    ResourceManaged,
    TaskActor,
    ArtifactSchema,
    ReviewLifecycle,
    RecommendationOnly,
    MigrationRollback,
    SeedCompatibility,
    UnicodeSourceSpans,
    DifferentialEvidence,
    SecurityEvidence,
    ToolchainOptionality,
}

impl PlannedNativeBackendSelectionBoundary {
    const ALL: [Self; 54] = [
        Self::CandidateCranelift,
        Self::CandidateLlvm,
        Self::CandidateCTransition,
        Self::CandidateWasmTransition,
        Self::ComparisonOnly,
        Self::NirInput,
        Self::AbiInput,
        Self::TargetMatrix,
        Self::ProfileMatrix,
        Self::CoreEligibility,
        Self::UnsupportedForm,
        Self::ToolchainVersion,
        Self::CompilerVersion,
        Self::TargetTriple,
        Self::CompilerFlags,
        Self::StandardLibrary,
        Self::LinkerVersion,
        Self::RuntimeLibrary,
        Self::ColdBuild,
        Self::WarmBuild,
        Self::ResourceTimeBounds,
        Self::DebugInfo,
        Self::SourceMap,
        Self::JitMode,
        Self::AotMode,
        Self::TargetCoverage,
        Self::LicenseReview,
        Self::SupplyChain,
        Self::TcbBoundary,
        Self::OfflineLock,
        Self::GeneratedCode,
        Self::ReproducibleInputs,
        Self::BuildEnvironment,
        Self::HostPathExclusion,
        Self::ClockNoiseExclusion,
        Self::AddressExclusion,
        Self::MapOrderExclusion,
        Self::DeterministicCorpus,
        Self::DeterministicMetrics,
        Self::CrossTargetMatrix,
        Self::SemanticPreservation,
        Self::AbiFfiEvidence,
        Self::FaultUnwind,
        Self::ResourceManaged,
        Self::TaskActor,
        Self::ArtifactSchema,
        Self::ReviewLifecycle,
        Self::RecommendationOnly,
        Self::MigrationRollback,
        Self::SeedCompatibility,
        Self::UnicodeSourceSpans,
        Self::DifferentialEvidence,
        Self::SecurityEvidence,
        Self::ToolchainOptionality,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::CandidateCranelift => 0,
            Self::CandidateLlvm => 1,
            Self::CandidateCTransition => 2,
            Self::CandidateWasmTransition => 3,
            Self::ComparisonOnly => 4,
            Self::NirInput => 5,
            Self::AbiInput => 6,
            Self::TargetMatrix => 7,
            Self::ProfileMatrix => 8,
            Self::CoreEligibility => 9,
            Self::UnsupportedForm => 10,
            Self::ToolchainVersion => 11,
            Self::CompilerVersion => 12,
            Self::TargetTriple => 13,
            Self::CompilerFlags => 14,
            Self::StandardLibrary => 15,
            Self::LinkerVersion => 16,
            Self::RuntimeLibrary => 17,
            Self::ColdBuild => 18,
            Self::WarmBuild => 19,
            Self::ResourceTimeBounds => 20,
            Self::DebugInfo => 21,
            Self::SourceMap => 22,
            Self::JitMode => 23,
            Self::AotMode => 24,
            Self::TargetCoverage => 25,
            Self::LicenseReview => 26,
            Self::SupplyChain => 27,
            Self::TcbBoundary => 28,
            Self::OfflineLock => 29,
            Self::GeneratedCode => 30,
            Self::ReproducibleInputs => 31,
            Self::BuildEnvironment => 32,
            Self::HostPathExclusion => 33,
            Self::ClockNoiseExclusion => 34,
            Self::AddressExclusion => 35,
            Self::MapOrderExclusion => 36,
            Self::DeterministicCorpus => 37,
            Self::DeterministicMetrics => 38,
            Self::CrossTargetMatrix => 39,
            Self::SemanticPreservation => 40,
            Self::AbiFfiEvidence => 41,
            Self::FaultUnwind => 42,
            Self::ResourceManaged => 43,
            Self::TaskActor => 44,
            Self::ArtifactSchema => 45,
            Self::ReviewLifecycle => 46,
            Self::RecommendationOnly => 47,
            Self::MigrationRollback => 48,
            Self::SeedCompatibility => 49,
            Self::UnicodeSourceSpans => 50,
            Self::DifferentialEvidence => 51,
            Self::SecurityEvidence => 52,
            Self::ToolchainOptionality => 53,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NativeBackendSelectionBoundaryInventory {
    boundaries: Box<[PlannedNativeBackendSelectionBoundary]>,
}

impl NativeBackendSelectionBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNativeBackendSelectionBoundary>,
    ) -> Result<Self, PlannedNativeBackendSelectionBoundary> {
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
        bytes.extend_from_slice(b"ling.native-backend-selection-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_native_backend_selection_boundaries_are_complete_and_ordered() {
    let inventory =
        NativeBackendSelectionBoundaryInventory::new(PlannedNativeBackendSelectionBoundary::ALL)
            .expect("planned Native backend selection boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNativeBackendSelectionBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..54).collect::<Vec<_>>()
    );
}

#[test]
fn native_backend_selection_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        NativeBackendSelectionBoundaryInventory::new(PlannedNativeBackendSelectionBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = NativeBackendSelectionBoundaryInventory::new(
        PlannedNativeBackendSelectionBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NativeBackendSelectionBoundaryInventory::new([
        PlannedNativeBackendSelectionBoundary::CandidateCranelift,
        PlannedNativeBackendSelectionBoundary::CandidateCranelift,
    ])
    .expect_err("duplicate Native backend selection boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedNativeBackendSelectionBoundary::CandidateCranelift
    );
}

#[test]
fn native_backend_selection_evidence_has_no_backend_or_support_authority() {
    let inventory = NativeBackendSelectionBoundaryInventory::new([
        PlannedNativeBackendSelectionBoundary::CandidateCranelift,
        PlannedNativeBackendSelectionBoundary::CandidateLlvm,
        PlannedNativeBackendSelectionBoundary::NirInput,
        PlannedNativeBackendSelectionBoundary::LicenseReview,
        PlannedNativeBackendSelectionBoundary::ReproducibleInputs,
        PlannedNativeBackendSelectionBoundary::RecommendationOnly,
        PlannedNativeBackendSelectionBoundary::SeedCompatibility,
    ])
    .expect("bounded Native backend selection evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.native-backend-selection-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 7);
}
