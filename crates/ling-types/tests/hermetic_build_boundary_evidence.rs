use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedHermeticBuildBoundary {
    HermeticBuild,
    LocalLockedProject,
    TypedBuildNode,
    BuildGraph,
    InputDeclaration,
    OutputDeclaration,
    CapabilityDeclaration,
    Sandbox,
    Determinism,
    Hash,
    OfflineReplay,
    EnvironmentAccess,
    NetworkAccess,
    FilesystemAccess,
    ProcessExecution,
    ShellScript,
    BuildScript,
    Plugin,
    CodeGenerator,
    GeneratedSource,
    Toolchain,
    CompilerVersion,
    LanguageVersion,
    UnicodeVersion,
    PackageIdentity,
    DependencyLock,
    Profile,
    Target,
    SourceInput,
    TypedCoreInput,
    Artifact,
    ArtifactIdentity,
    BuildMetadata,
    Provenance,
    Cache,
    CacheKey,
    CacheMiss,
    CacheCorruption,
    Retry,
    Cancellation,
    FailureAtomicity,
    ResourceLimit,
    Clock,
    Random,
    Credential,
    Symlink,
    PathRedaction,
    EnvironmentRedaction,
    Command,
    ExitCode,
    DiagnosticCode,
    BilingualDiagnostic,
    Schema,
    Migration,
    CrossProcess,
    CrossPlatform,
    PositiveFixture,
    NegativeFixture,
    AcceptedAuthority,
    ExplicitExclusion,
}

impl PlannedHermeticBuildBoundary {
    const ALL: [Self; 60] = [
        Self::HermeticBuild,
        Self::LocalLockedProject,
        Self::TypedBuildNode,
        Self::BuildGraph,
        Self::InputDeclaration,
        Self::OutputDeclaration,
        Self::CapabilityDeclaration,
        Self::Sandbox,
        Self::Determinism,
        Self::Hash,
        Self::OfflineReplay,
        Self::EnvironmentAccess,
        Self::NetworkAccess,
        Self::FilesystemAccess,
        Self::ProcessExecution,
        Self::ShellScript,
        Self::BuildScript,
        Self::Plugin,
        Self::CodeGenerator,
        Self::GeneratedSource,
        Self::Toolchain,
        Self::CompilerVersion,
        Self::LanguageVersion,
        Self::UnicodeVersion,
        Self::PackageIdentity,
        Self::DependencyLock,
        Self::Profile,
        Self::Target,
        Self::SourceInput,
        Self::TypedCoreInput,
        Self::Artifact,
        Self::ArtifactIdentity,
        Self::BuildMetadata,
        Self::Provenance,
        Self::Cache,
        Self::CacheKey,
        Self::CacheMiss,
        Self::CacheCorruption,
        Self::Retry,
        Self::Cancellation,
        Self::FailureAtomicity,
        Self::ResourceLimit,
        Self::Clock,
        Self::Random,
        Self::Credential,
        Self::Symlink,
        Self::PathRedaction,
        Self::EnvironmentRedaction,
        Self::Command,
        Self::ExitCode,
        Self::DiagnosticCode,
        Self::BilingualDiagnostic,
        Self::Schema,
        Self::Migration,
        Self::CrossProcess,
        Self::CrossPlatform,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::AcceptedAuthority,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HermeticBuildBoundaryInventory {
    boundaries: Box<[PlannedHermeticBuildBoundary]>,
}

impl HermeticBuildBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedHermeticBuildBoundary>,
    ) -> Result<Self, PlannedHermeticBuildBoundary> {
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
        let mut bytes = b"ling.hermetic-build-boundary-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_hermetic_build_boundaries_are_complete_and_ordered() {
    let inventory = HermeticBuildBoundaryInventory::new(PlannedHermeticBuildBoundary::ALL)
        .expect("planned hermetic-build boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedHermeticBuildBoundary::ALL
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
fn hermetic_build_evidence_is_order_independent_and_duplicate_checked() {
    let forward = HermeticBuildBoundaryInventory::new(PlannedHermeticBuildBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        HermeticBuildBoundaryInventory::new(PlannedHermeticBuildBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = HermeticBuildBoundaryInventory::new([
        PlannedHermeticBuildBoundary::BuildScript,
        PlannedHermeticBuildBoundary::BuildScript,
    ])
    .expect_err("duplicate hermetic-build boundary must be rejected");
    assert_eq!(duplicate, PlannedHermeticBuildBoundary::BuildScript);
}

#[test]
fn observation_has_no_build_graph_executor_or_sandbox_authority() {
    let inventory = HermeticBuildBoundaryInventory::new([
        PlannedHermeticBuildBoundary::HermeticBuild,
        PlannedHermeticBuildBoundary::TypedBuildNode,
        PlannedHermeticBuildBoundary::BuildGraph,
        PlannedHermeticBuildBoundary::CapabilityDeclaration,
        PlannedHermeticBuildBoundary::Sandbox,
        PlannedHermeticBuildBoundary::ShellScript,
        PlannedHermeticBuildBoundary::BuildScript,
        PlannedHermeticBuildBoundary::EnvironmentAccess,
        PlannedHermeticBuildBoundary::NetworkAccess,
        PlannedHermeticBuildBoundary::Artifact,
        PlannedHermeticBuildBoundary::AcceptedAuthority,
        PlannedHermeticBuildBoundary::ExplicitExclusion,
    ])
    .expect("bounded hermetic-build boundary evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.hermetic-build-boundary-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
