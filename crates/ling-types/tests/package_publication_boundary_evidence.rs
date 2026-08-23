use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedPackagePublicationBoundary {
    PackagePublication,
    LocalProtocol,
    ManifestV1,
    LockV1,
    PackageName,
    DisplayName,
    PackageVersion,
    LanguageVersion,
    PackageSourceId,
    PackageGraphId,
    Sha256,
    SemanticHash,
    ContentHash,
    BuildArtifactId,
    Namespace,
    PublisherIdentity,
    SourceCoordinate,
    Ownership,
    Transfer,
    KeyRotation,
    KeyRevocation,
    Registry,
    Upload,
    Download,
    Installation,
    Update,
    Rollback,
    NetworkDependency,
    GitDependency,
    VersionRange,
    MultipleVersions,
    Archive,
    Artifact,
    FileInclusion,
    SymlinkPolicy,
    PermissionPolicy,
    Checksum,
    Signature,
    TrustRoot,
    Provenance,
    Sbom,
    License,
    TransparencyLog,
    Yanked,
    Deprecated,
    Mirror,
    OfflineCache,
    CacheFreshness,
    Retry,
    Credential,
    Manifest,
    Lock,
    CanonicalBytes,
    Determinism,
    OfflineResolution,
    BilingualDiagnostic,
    PositiveFixture,
    NegativeFixture,
    AcceptedAuthority,
    ExplicitExclusion,
}

impl PlannedPackagePublicationBoundary {
    const ALL: [Self; 60] = [
        Self::PackagePublication,
        Self::LocalProtocol,
        Self::ManifestV1,
        Self::LockV1,
        Self::PackageName,
        Self::DisplayName,
        Self::PackageVersion,
        Self::LanguageVersion,
        Self::PackageSourceId,
        Self::PackageGraphId,
        Self::Sha256,
        Self::SemanticHash,
        Self::ContentHash,
        Self::BuildArtifactId,
        Self::Namespace,
        Self::PublisherIdentity,
        Self::SourceCoordinate,
        Self::Ownership,
        Self::Transfer,
        Self::KeyRotation,
        Self::KeyRevocation,
        Self::Registry,
        Self::Upload,
        Self::Download,
        Self::Installation,
        Self::Update,
        Self::Rollback,
        Self::NetworkDependency,
        Self::GitDependency,
        Self::VersionRange,
        Self::MultipleVersions,
        Self::Archive,
        Self::Artifact,
        Self::FileInclusion,
        Self::SymlinkPolicy,
        Self::PermissionPolicy,
        Self::Checksum,
        Self::Signature,
        Self::TrustRoot,
        Self::Provenance,
        Self::Sbom,
        Self::License,
        Self::TransparencyLog,
        Self::Yanked,
        Self::Deprecated,
        Self::Mirror,
        Self::OfflineCache,
        Self::CacheFreshness,
        Self::Retry,
        Self::Credential,
        Self::Manifest,
        Self::Lock,
        Self::CanonicalBytes,
        Self::Determinism,
        Self::OfflineResolution,
        Self::BilingualDiagnostic,
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
struct PackagePublicationBoundaryInventory {
    boundaries: Box<[PlannedPackagePublicationBoundary]>,
}

impl PackagePublicationBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedPackagePublicationBoundary>,
    ) -> Result<Self, PlannedPackagePublicationBoundary> {
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
        let mut bytes = b"ling.package-publication-boundary-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_package_publication_boundaries_are_complete_and_ordered() {
    let inventory =
        PackagePublicationBoundaryInventory::new(PlannedPackagePublicationBoundary::ALL)
            .expect("planned package-publication boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedPackagePublicationBoundary::ALL
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
fn package_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = PackagePublicationBoundaryInventory::new(PlannedPackagePublicationBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = PackagePublicationBoundaryInventory::new(
        PlannedPackagePublicationBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = PackagePublicationBoundaryInventory::new([
        PlannedPackagePublicationBoundary::Registry,
        PlannedPackagePublicationBoundary::Registry,
    ])
    .expect_err("duplicate package-publication boundary must be rejected");
    assert_eq!(duplicate, PlannedPackagePublicationBoundary::Registry);
}

#[test]
fn observation_has_no_publication_registry_or_supply_chain_authority() {
    let inventory = PackagePublicationBoundaryInventory::new([
        PlannedPackagePublicationBoundary::PackagePublication,
        PlannedPackagePublicationBoundary::LocalProtocol,
        PlannedPackagePublicationBoundary::PublisherIdentity,
        PlannedPackagePublicationBoundary::Registry,
        PlannedPackagePublicationBoundary::Upload,
        PlannedPackagePublicationBoundary::Installation,
        PlannedPackagePublicationBoundary::Artifact,
        PlannedPackagePublicationBoundary::Signature,
        PlannedPackagePublicationBoundary::Provenance,
        PlannedPackagePublicationBoundary::Mirror,
        PlannedPackagePublicationBoundary::AcceptedAuthority,
        PlannedPackagePublicationBoundary::ExplicitExclusion,
    ])
    .expect("bounded package-publication boundary evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.package-publication-boundary-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
