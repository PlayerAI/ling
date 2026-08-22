use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNativeReproducibleBuildBoundary {
    ToolchainVersion,
    ToolchainDigest,
    TargetTriple,
    LinkerVersion,
    LinkerDigest,
    EnvironmentClosure,
    StandardLibrary,
    RuntimeLibrary,
    CodegenOptions,
    DependencyLock,
    SourceInput,
    TypedCoreInput,
    NirInput,
    ProfileManifest,
    ArtifactFormat,
    ArtifactIdentity,
    ObjectBytes,
    ExecutableBytes,
    DebugBytes,
    SymbolBytes,
    PathRemapping,
    TimestampPolicy,
    BuildIdPolicy,
    SectionOrdering,
    SymbolOrdering,
    CompressionPolicy,
    ArchiveOrdering,
    ManifestVersion,
    DifferenceManifest,
    ByteComparison,
    ManifestComparison,
    CleanBuild,
    RepeatBuild,
    OfflineBuild,
    CrossHostBuild,
    CrossTargetBuild,
    TamperedInput,
    MissingInput,
    Provenance,
    LicenseTcb,
    CacheBoundary,
    ReleaseBoundary,
    SemanticIdSeparation,
    SourceSpanSeparation,
    PerformanceSeparation,
    ResourceTimeBounds,
    DeterministicMetadata,
    HostPathExclusion,
    AddressExclusion,
    MapOrderExclusion,
    TimingExclusion,
    UnicodeSourceSpans,
    DiagnosticBilingual,
    UnsupportedInput,
    MigrationCompatibility,
    SecurityEvidence,
    InterpreterVmNativeEvidence,
    SeedCompatibility,
    ArtifactSchema,
    ToolchainOptionality,
}

impl PlannedNativeReproducibleBuildBoundary {
    const ALL: [Self; 60] = [
        Self::ToolchainVersion,
        Self::ToolchainDigest,
        Self::TargetTriple,
        Self::LinkerVersion,
        Self::LinkerDigest,
        Self::EnvironmentClosure,
        Self::StandardLibrary,
        Self::RuntimeLibrary,
        Self::CodegenOptions,
        Self::DependencyLock,
        Self::SourceInput,
        Self::TypedCoreInput,
        Self::NirInput,
        Self::ProfileManifest,
        Self::ArtifactFormat,
        Self::ArtifactIdentity,
        Self::ObjectBytes,
        Self::ExecutableBytes,
        Self::DebugBytes,
        Self::SymbolBytes,
        Self::PathRemapping,
        Self::TimestampPolicy,
        Self::BuildIdPolicy,
        Self::SectionOrdering,
        Self::SymbolOrdering,
        Self::CompressionPolicy,
        Self::ArchiveOrdering,
        Self::ManifestVersion,
        Self::DifferenceManifest,
        Self::ByteComparison,
        Self::ManifestComparison,
        Self::CleanBuild,
        Self::RepeatBuild,
        Self::OfflineBuild,
        Self::CrossHostBuild,
        Self::CrossTargetBuild,
        Self::TamperedInput,
        Self::MissingInput,
        Self::Provenance,
        Self::LicenseTcb,
        Self::CacheBoundary,
        Self::ReleaseBoundary,
        Self::SemanticIdSeparation,
        Self::SourceSpanSeparation,
        Self::PerformanceSeparation,
        Self::ResourceTimeBounds,
        Self::DeterministicMetadata,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::MapOrderExclusion,
        Self::TimingExclusion,
        Self::UnicodeSourceSpans,
        Self::DiagnosticBilingual,
        Self::UnsupportedInput,
        Self::MigrationCompatibility,
        Self::SecurityEvidence,
        Self::InterpreterVmNativeEvidence,
        Self::SeedCompatibility,
        Self::ArtifactSchema,
        Self::ToolchainOptionality,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ToolchainVersion => 0,
            Self::ToolchainDigest => 1,
            Self::TargetTriple => 2,
            Self::LinkerVersion => 3,
            Self::LinkerDigest => 4,
            Self::EnvironmentClosure => 5,
            Self::StandardLibrary => 6,
            Self::RuntimeLibrary => 7,
            Self::CodegenOptions => 8,
            Self::DependencyLock => 9,
            Self::SourceInput => 10,
            Self::TypedCoreInput => 11,
            Self::NirInput => 12,
            Self::ProfileManifest => 13,
            Self::ArtifactFormat => 14,
            Self::ArtifactIdentity => 15,
            Self::ObjectBytes => 16,
            Self::ExecutableBytes => 17,
            Self::DebugBytes => 18,
            Self::SymbolBytes => 19,
            Self::PathRemapping => 20,
            Self::TimestampPolicy => 21,
            Self::BuildIdPolicy => 22,
            Self::SectionOrdering => 23,
            Self::SymbolOrdering => 24,
            Self::CompressionPolicy => 25,
            Self::ArchiveOrdering => 26,
            Self::ManifestVersion => 27,
            Self::DifferenceManifest => 28,
            Self::ByteComparison => 29,
            Self::ManifestComparison => 30,
            Self::CleanBuild => 31,
            Self::RepeatBuild => 32,
            Self::OfflineBuild => 33,
            Self::CrossHostBuild => 34,
            Self::CrossTargetBuild => 35,
            Self::TamperedInput => 36,
            Self::MissingInput => 37,
            Self::Provenance => 38,
            Self::LicenseTcb => 39,
            Self::CacheBoundary => 40,
            Self::ReleaseBoundary => 41,
            Self::SemanticIdSeparation => 42,
            Self::SourceSpanSeparation => 43,
            Self::PerformanceSeparation => 44,
            Self::ResourceTimeBounds => 45,
            Self::DeterministicMetadata => 46,
            Self::HostPathExclusion => 47,
            Self::AddressExclusion => 48,
            Self::MapOrderExclusion => 49,
            Self::TimingExclusion => 50,
            Self::UnicodeSourceSpans => 51,
            Self::DiagnosticBilingual => 52,
            Self::UnsupportedInput => 53,
            Self::MigrationCompatibility => 54,
            Self::SecurityEvidence => 55,
            Self::InterpreterVmNativeEvidence => 56,
            Self::SeedCompatibility => 57,
            Self::ArtifactSchema => 58,
            Self::ToolchainOptionality => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NativeReproducibleBuildBoundaryInventory {
    boundaries: Box<[PlannedNativeReproducibleBuildBoundary]>,
}

impl NativeReproducibleBuildBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNativeReproducibleBuildBoundary>,
    ) -> Result<Self, PlannedNativeReproducibleBuildBoundary> {
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
        bytes.extend_from_slice(b"ling.native-reproducible-build-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_native_reproducible_build_boundaries_are_complete_and_ordered() {
    let inventory =
        NativeReproducibleBuildBoundaryInventory::new(PlannedNativeReproducibleBuildBoundary::ALL)
            .expect("planned Native reproducible-build boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNativeReproducibleBuildBoundary::ALL
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
fn native_reproducible_build_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        NativeReproducibleBuildBoundaryInventory::new(PlannedNativeReproducibleBuildBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = NativeReproducibleBuildBoundaryInventory::new(
        PlannedNativeReproducibleBuildBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NativeReproducibleBuildBoundaryInventory::new([
        PlannedNativeReproducibleBuildBoundary::ToolchainVersion,
        PlannedNativeReproducibleBuildBoundary::ToolchainVersion,
    ])
    .expect_err("duplicate Native reproducible-build boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedNativeReproducibleBuildBoundary::ToolchainVersion
    );
}

#[test]
fn native_reproducible_build_evidence_has_no_build_or_release_authority() {
    let inventory = NativeReproducibleBuildBoundaryInventory::new([
        PlannedNativeReproducibleBuildBoundary::ToolchainVersion,
        PlannedNativeReproducibleBuildBoundary::TargetTriple,
        PlannedNativeReproducibleBuildBoundary::DependencyLock,
        PlannedNativeReproducibleBuildBoundary::ArtifactIdentity,
        PlannedNativeReproducibleBuildBoundary::ByteComparison,
        PlannedNativeReproducibleBuildBoundary::DifferenceManifest,
        PlannedNativeReproducibleBuildBoundary::OfflineBuild,
        PlannedNativeReproducibleBuildBoundary::SeedCompatibility,
    ])
    .expect("bounded Native reproducible-build evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.native-reproducible-build-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
