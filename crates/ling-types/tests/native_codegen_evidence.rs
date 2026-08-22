use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNativeCodegenBoundary {
    TargetMachine,
    TargetTriple,
    TargetProfile,
    DataLayout,
    Endianness,
    FunctionEmission,
    DataEmission,
    ClosureEmission,
    AdtEmission,
    StringEmission,
    ValueRepresentation,
    ResourceRepresentation,
    ManagedHandleRepresentation,
    CallingConvention,
    FaultResultConvention,
    TaskActorCall,
    FfiBoundary,
    Relocation,
    RuntimeLinking,
    RuntimeLibrary,
    ObjectFormat,
    ExecutableFormat,
    SectionLayout,
    SymbolTable,
    DebugInfo,
    SourceMap,
    SourceIdentity,
    DeterministicMetadata,
    SymbolOrdering,
    SectionOrdering,
    UnsupportedForm,
    UnsupportedDiagnostic,
    DiagnosticBilingual,
    SourceByteSpan,
    NirVerifierInput,
    AbiVerifierInput,
    ProfileCapability,
    AllocationGcIntegration,
    CleanupDrop,
    RuntimeFaultUnwind,
    CrossTargetEvidence,
    AbiEvidence,
    ReproducibleArtifacts,
    SemanticPreservation,
    DifferentialEvidence,
    SanitizerEvidence,
    SecurityTcb,
    LicenseOfflinePolicy,
    ArtifactSchema,
    MigrationCompatibility,
    SeedCompatibility,
    UnicodeSourceSpans,
    HostOutputExclusion,
    TimestampAddressExclusion,
    MapOrderExclusion,
    MalformedInputRejection,
    ToolchainLinkerInputs,
    BuildResourceBounds,
}

impl PlannedNativeCodegenBoundary {
    const ALL: [Self; 58] = [
        Self::TargetMachine,
        Self::TargetTriple,
        Self::TargetProfile,
        Self::DataLayout,
        Self::Endianness,
        Self::FunctionEmission,
        Self::DataEmission,
        Self::ClosureEmission,
        Self::AdtEmission,
        Self::StringEmission,
        Self::ValueRepresentation,
        Self::ResourceRepresentation,
        Self::ManagedHandleRepresentation,
        Self::CallingConvention,
        Self::FaultResultConvention,
        Self::TaskActorCall,
        Self::FfiBoundary,
        Self::Relocation,
        Self::RuntimeLinking,
        Self::RuntimeLibrary,
        Self::ObjectFormat,
        Self::ExecutableFormat,
        Self::SectionLayout,
        Self::SymbolTable,
        Self::DebugInfo,
        Self::SourceMap,
        Self::SourceIdentity,
        Self::DeterministicMetadata,
        Self::SymbolOrdering,
        Self::SectionOrdering,
        Self::UnsupportedForm,
        Self::UnsupportedDiagnostic,
        Self::DiagnosticBilingual,
        Self::SourceByteSpan,
        Self::NirVerifierInput,
        Self::AbiVerifierInput,
        Self::ProfileCapability,
        Self::AllocationGcIntegration,
        Self::CleanupDrop,
        Self::RuntimeFaultUnwind,
        Self::CrossTargetEvidence,
        Self::AbiEvidence,
        Self::ReproducibleArtifacts,
        Self::SemanticPreservation,
        Self::DifferentialEvidence,
        Self::SanitizerEvidence,
        Self::SecurityTcb,
        Self::LicenseOfflinePolicy,
        Self::ArtifactSchema,
        Self::MigrationCompatibility,
        Self::SeedCompatibility,
        Self::UnicodeSourceSpans,
        Self::HostOutputExclusion,
        Self::TimestampAddressExclusion,
        Self::MapOrderExclusion,
        Self::MalformedInputRejection,
        Self::ToolchainLinkerInputs,
        Self::BuildResourceBounds,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::TargetMachine => 0,
            Self::TargetTriple => 1,
            Self::TargetProfile => 2,
            Self::DataLayout => 3,
            Self::Endianness => 4,
            Self::FunctionEmission => 5,
            Self::DataEmission => 6,
            Self::ClosureEmission => 7,
            Self::AdtEmission => 8,
            Self::StringEmission => 9,
            Self::ValueRepresentation => 10,
            Self::ResourceRepresentation => 11,
            Self::ManagedHandleRepresentation => 12,
            Self::CallingConvention => 13,
            Self::FaultResultConvention => 14,
            Self::TaskActorCall => 15,
            Self::FfiBoundary => 16,
            Self::Relocation => 17,
            Self::RuntimeLinking => 18,
            Self::RuntimeLibrary => 19,
            Self::ObjectFormat => 20,
            Self::ExecutableFormat => 21,
            Self::SectionLayout => 22,
            Self::SymbolTable => 23,
            Self::DebugInfo => 24,
            Self::SourceMap => 25,
            Self::SourceIdentity => 26,
            Self::DeterministicMetadata => 27,
            Self::SymbolOrdering => 28,
            Self::SectionOrdering => 29,
            Self::UnsupportedForm => 30,
            Self::UnsupportedDiagnostic => 31,
            Self::DiagnosticBilingual => 32,
            Self::SourceByteSpan => 33,
            Self::NirVerifierInput => 34,
            Self::AbiVerifierInput => 35,
            Self::ProfileCapability => 36,
            Self::AllocationGcIntegration => 37,
            Self::CleanupDrop => 38,
            Self::RuntimeFaultUnwind => 39,
            Self::CrossTargetEvidence => 40,
            Self::AbiEvidence => 41,
            Self::ReproducibleArtifacts => 42,
            Self::SemanticPreservation => 43,
            Self::DifferentialEvidence => 44,
            Self::SanitizerEvidence => 45,
            Self::SecurityTcb => 46,
            Self::LicenseOfflinePolicy => 47,
            Self::ArtifactSchema => 48,
            Self::MigrationCompatibility => 49,
            Self::SeedCompatibility => 50,
            Self::UnicodeSourceSpans => 51,
            Self::HostOutputExclusion => 52,
            Self::TimestampAddressExclusion => 53,
            Self::MapOrderExclusion => 54,
            Self::MalformedInputRejection => 55,
            Self::ToolchainLinkerInputs => 56,
            Self::BuildResourceBounds => 57,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NativeCodegenBoundaryInventory {
    boundaries: Box<[PlannedNativeCodegenBoundary]>,
}

impl NativeCodegenBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNativeCodegenBoundary>,
    ) -> Result<Self, PlannedNativeCodegenBoundary> {
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
        bytes.extend_from_slice(b"ling.native-codegen-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_native_codegen_boundaries_are_complete_and_ordered() {
    let inventory = NativeCodegenBoundaryInventory::new(PlannedNativeCodegenBoundary::ALL)
        .expect("planned Native codegen boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNativeCodegenBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..58).collect::<Vec<_>>()
    );
}

#[test]
fn native_codegen_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NativeCodegenBoundaryInventory::new(PlannedNativeCodegenBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        NativeCodegenBoundaryInventory::new(PlannedNativeCodegenBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NativeCodegenBoundaryInventory::new([
        PlannedNativeCodegenBoundary::TargetMachine,
        PlannedNativeCodegenBoundary::TargetMachine,
    ])
    .expect_err("duplicate Native codegen boundary must be rejected");
    assert_eq!(duplicate, PlannedNativeCodegenBoundary::TargetMachine);
}

#[test]
fn native_codegen_evidence_has_no_emission_or_artifact_authority() {
    let inventory = NativeCodegenBoundaryInventory::new([
        PlannedNativeCodegenBoundary::TargetMachine,
        PlannedNativeCodegenBoundary::FunctionEmission,
        PlannedNativeCodegenBoundary::Relocation,
        PlannedNativeCodegenBoundary::ObjectFormat,
        PlannedNativeCodegenBoundary::DebugInfo,
        PlannedNativeCodegenBoundary::UnsupportedDiagnostic,
        PlannedNativeCodegenBoundary::SeedCompatibility,
    ])
    .expect("bounded Native codegen evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.native-codegen-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 7);
}
