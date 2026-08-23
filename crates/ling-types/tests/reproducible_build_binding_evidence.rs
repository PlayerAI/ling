use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedReproducibleBuildBindingBoundary {
    ReproducibleBuildBinding,
    Version,
    BundleManifest,
    ControlledEnvironment,
    HermeticBuild,
    SourceId,
    SemanticId,
    BuildGraphId,
    DependencyLockId,
    ProfileId,
    TargetId,
    ToolchainId,
    CompilerId,
    BuildId,
    TcbId,
    ObjectArtifact,
    BinaryArtifact,
    ObjectHash,
    BinaryHash,
    GeneratedSource,
    ProofArtifact,
    GeneratedProvenance,
    ArtifactLinkage,
    BuildRecipe,
    EnvironmentIdentity,
    InputIdentity,
    OutputIdentity,
    AcceptedNondeterminism,
    NondeterminismReason,
    NondeterminismScope,
    DeterministicOrdering,
    RepeatedBuild,
    CrossHostBuild,
    OfflineBuild,
    NetworkDenied,
    TimestampExclusion,
    HostPathExclusion,
    AddressExclusion,
    DebugOutputExclusion,
    SecretExclusion,
    Equivalent,
    Mismatch,
    Unknown,
    Unsupported,
    StaleInput,
    MissingArtifact,
    HashMismatch,
    IdentityMismatch,
    UnapprovedNondeterminism,
    Malformed,
    Corrupt,
    UnsupportedVersion,
    Migration,
    FailClosed,
    DiagnosticCode,
    PositiveFixture,
    NegativeFixture,
    RepeatedBuildFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedReproducibleBuildBindingBoundary {
    const ALL: [Self; 60] = [
        Self::ReproducibleBuildBinding,
        Self::Version,
        Self::BundleManifest,
        Self::ControlledEnvironment,
        Self::HermeticBuild,
        Self::SourceId,
        Self::SemanticId,
        Self::BuildGraphId,
        Self::DependencyLockId,
        Self::ProfileId,
        Self::TargetId,
        Self::ToolchainId,
        Self::CompilerId,
        Self::BuildId,
        Self::TcbId,
        Self::ObjectArtifact,
        Self::BinaryArtifact,
        Self::ObjectHash,
        Self::BinaryHash,
        Self::GeneratedSource,
        Self::ProofArtifact,
        Self::GeneratedProvenance,
        Self::ArtifactLinkage,
        Self::BuildRecipe,
        Self::EnvironmentIdentity,
        Self::InputIdentity,
        Self::OutputIdentity,
        Self::AcceptedNondeterminism,
        Self::NondeterminismReason,
        Self::NondeterminismScope,
        Self::DeterministicOrdering,
        Self::RepeatedBuild,
        Self::CrossHostBuild,
        Self::OfflineBuild,
        Self::NetworkDenied,
        Self::TimestampExclusion,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::DebugOutputExclusion,
        Self::SecretExclusion,
        Self::Equivalent,
        Self::Mismatch,
        Self::Unknown,
        Self::Unsupported,
        Self::StaleInput,
        Self::MissingArtifact,
        Self::HashMismatch,
        Self::IdentityMismatch,
        Self::UnapprovedNondeterminism,
        Self::Malformed,
        Self::Corrupt,
        Self::UnsupportedVersion,
        Self::Migration,
        Self::FailClosed,
        Self::DiagnosticCode,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::RepeatedBuildFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReproducibleBuildBindingInventory {
    boundaries: Box<[PlannedReproducibleBuildBindingBoundary]>,
}

impl ReproducibleBuildBindingInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedReproducibleBuildBindingBoundary>,
    ) -> Result<Self, PlannedReproducibleBuildBindingBoundary> {
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
        let mut bytes = b"ling.reproducible-build-binding-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_reproducible_build_binding_boundaries_are_complete_and_ordered() {
    let inventory =
        ReproducibleBuildBindingInventory::new(PlannedReproducibleBuildBindingBoundary::ALL)
            .expect("planned reproducible-build-binding boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedReproducibleBuildBindingBoundary::ALL
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
fn reproducible_build_binding_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        ReproducibleBuildBindingInventory::new(PlannedReproducibleBuildBindingBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = ReproducibleBuildBindingInventory::new(
        PlannedReproducibleBuildBindingBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ReproducibleBuildBindingInventory::new([
        PlannedReproducibleBuildBindingBoundary::ReproducibleBuildBinding,
        PlannedReproducibleBuildBindingBoundary::ReproducibleBuildBinding,
    ])
    .expect_err("duplicate reproducible-build-binding boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedReproducibleBuildBindingBoundary::ReproducibleBuildBinding
    );
}

#[test]
fn reproducible_build_binding_evidence_has_no_reproducibility_authority() {
    let inventory = ReproducibleBuildBindingInventory::new([
        PlannedReproducibleBuildBindingBoundary::ReproducibleBuildBinding,
        PlannedReproducibleBuildBindingBoundary::HermeticBuild,
        PlannedReproducibleBuildBindingBoundary::SourceId,
        PlannedReproducibleBuildBindingBoundary::SemanticId,
        PlannedReproducibleBuildBindingBoundary::ObjectHash,
        PlannedReproducibleBuildBindingBoundary::BinaryHash,
        PlannedReproducibleBuildBindingBoundary::AcceptedNondeterminism,
        PlannedReproducibleBuildBindingBoundary::ProtocolInventory,
    ])
    .expect("bounded reproducible-build-binding evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.reproducible-build-binding-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
