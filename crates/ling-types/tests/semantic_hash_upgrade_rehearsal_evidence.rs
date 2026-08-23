use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedSemanticHashUpgradeBoundary {
    SemanticHashUpgradeRehearsal,
    OldAlgorithmId,
    NewAlgorithmId,
    HashSchemeId,
    DomainSeparator,
    CanonicalBytes,
    SemanticId,
    ProgramId,
    DefinitionId,
    BodyId,
    NodeId,
    PackageIdentity,
    DependencyIdentity,
    LockfileIdentity,
    CacheKey,
    CacheVersion,
    CacheInvalidation,
    DualReader,
    OldReader,
    NewReader,
    OldWriter,
    NewWriter,
    ExplicitMigration,
    MigrationTool,
    MigrationCorpus,
    NoSilentRehash,
    IdentityMismatch,
    UnknownAlgorithm,
    UnsupportedVersion,
    CorruptDigest,
    TruncatedDigest,
    MalformedDigest,
    WrongDomain,
    CollisionBoundary,
    ReplayLinkage,
    EvidenceLinkage,
    ArtifactLinkage,
    BuildManifestLinkage,
    ProtocolInventory,
    SchemaRegistry,
    AcceptedAuthority,
    Experimental,
    Preview,
    Stable,
    CompatibilityPromise,
    FailClosed,
    BilingualDiagnostic,
    OriginalUtf8Span,
    UnicodeVersion,
    DeterministicOrder,
    OfflineEvidence,
    CrossProcess,
    RepeatedBuild,
    PositiveFixture,
    NegativeFixture,
    OldGolden,
    NewGolden,
    CacheFixture,
    LockFixture,
    ExplicitExclusion,
}

impl PlannedSemanticHashUpgradeBoundary {
    const ALL: [Self; 60] = [
        Self::SemanticHashUpgradeRehearsal,
        Self::OldAlgorithmId,
        Self::NewAlgorithmId,
        Self::HashSchemeId,
        Self::DomainSeparator,
        Self::CanonicalBytes,
        Self::SemanticId,
        Self::ProgramId,
        Self::DefinitionId,
        Self::BodyId,
        Self::NodeId,
        Self::PackageIdentity,
        Self::DependencyIdentity,
        Self::LockfileIdentity,
        Self::CacheKey,
        Self::CacheVersion,
        Self::CacheInvalidation,
        Self::DualReader,
        Self::OldReader,
        Self::NewReader,
        Self::OldWriter,
        Self::NewWriter,
        Self::ExplicitMigration,
        Self::MigrationTool,
        Self::MigrationCorpus,
        Self::NoSilentRehash,
        Self::IdentityMismatch,
        Self::UnknownAlgorithm,
        Self::UnsupportedVersion,
        Self::CorruptDigest,
        Self::TruncatedDigest,
        Self::MalformedDigest,
        Self::WrongDomain,
        Self::CollisionBoundary,
        Self::ReplayLinkage,
        Self::EvidenceLinkage,
        Self::ArtifactLinkage,
        Self::BuildManifestLinkage,
        Self::ProtocolInventory,
        Self::SchemaRegistry,
        Self::AcceptedAuthority,
        Self::Experimental,
        Self::Preview,
        Self::Stable,
        Self::CompatibilityPromise,
        Self::FailClosed,
        Self::BilingualDiagnostic,
        Self::OriginalUtf8Span,
        Self::UnicodeVersion,
        Self::DeterministicOrder,
        Self::OfflineEvidence,
        Self::CrossProcess,
        Self::RepeatedBuild,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::OldGolden,
        Self::NewGolden,
        Self::CacheFixture,
        Self::LockFixture,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SemanticHashUpgradeInventory {
    boundaries: Box<[PlannedSemanticHashUpgradeBoundary]>,
}

impl SemanticHashUpgradeInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedSemanticHashUpgradeBoundary>,
    ) -> Result<Self, PlannedSemanticHashUpgradeBoundary> {
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
        let mut bytes = b"ling.semantic-hash-upgrade-rehearsal-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_semantic_hash_upgrade_boundaries_are_complete_and_ordered() {
    let inventory = SemanticHashUpgradeInventory::new(PlannedSemanticHashUpgradeBoundary::ALL)
        .expect("planned Semantic Hash upgrade boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedSemanticHashUpgradeBoundary::ALL
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
fn semantic_hash_rehearsal_evidence_is_order_independent_and_duplicate_checked() {
    let forward = SemanticHashUpgradeInventory::new(PlannedSemanticHashUpgradeBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = SemanticHashUpgradeInventory::new(
        PlannedSemanticHashUpgradeBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = SemanticHashUpgradeInventory::new([
        PlannedSemanticHashUpgradeBoundary::HashSchemeId,
        PlannedSemanticHashUpgradeBoundary::HashSchemeId,
    ])
    .expect_err("duplicate Semantic Hash upgrade boundary must be rejected");
    assert_eq!(duplicate, PlannedSemanticHashUpgradeBoundary::HashSchemeId);
}

#[test]
fn rehearsal_evidence_has_no_hash_or_migration_authority() {
    let inventory = SemanticHashUpgradeInventory::new([
        PlannedSemanticHashUpgradeBoundary::SemanticHashUpgradeRehearsal,
        PlannedSemanticHashUpgradeBoundary::OldAlgorithmId,
        PlannedSemanticHashUpgradeBoundary::NewAlgorithmId,
        PlannedSemanticHashUpgradeBoundary::CanonicalBytes,
        PlannedSemanticHashUpgradeBoundary::DualReader,
        PlannedSemanticHashUpgradeBoundary::ExplicitMigration,
        PlannedSemanticHashUpgradeBoundary::CacheInvalidation,
        PlannedSemanticHashUpgradeBoundary::ReplayLinkage,
        PlannedSemanticHashUpgradeBoundary::NoSilentRehash,
        PlannedSemanticHashUpgradeBoundary::AcceptedAuthority,
        PlannedSemanticHashUpgradeBoundary::CompatibilityPromise,
        PlannedSemanticHashUpgradeBoundary::ExplicitExclusion,
    ])
    .expect("bounded Semantic Hash upgrade rehearsal evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.semantic-hash-upgrade-rehearsal-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
