use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum RegistryDefermentBoundary {
    RegistryStrategy,
    Deferred,
    Unsupported,
    VersionOneScope,
    LocalManifest,
    LocalDependency,
    LocalSource,
    LockedResolution,
    OfflineResolution,
    PackageName,
    ContentIdentity,
    GraphIdentity,
    LockProtocol,
    Experimental,
    StableClaim,
    PreviewClaim,
    ReopeningCriteria,
    AcceptedAuthority,
    PublisherCoordinate,
    PublisherAuthentication,
    NamespaceOwnership,
    NamespaceTransfer,
    RegistryIndex,
    Upload,
    Download,
    Installation,
    Update,
    Rollback,
    Yanked,
    Deprecated,
    Archive,
    Artifact,
    Checksum,
    Signature,
    Provenance,
    Sbom,
    License,
    Transparency,
    Mirror,
    Cache,
    Availability,
    RateLimit,
    Retry,
    Credential,
    TrustRoot,
    ThreatModel,
    CliCommand,
    ExitCode,
    JsonProtocol,
    BilingualDiagnostic,
    Schema,
    CompatibilityReader,
    Migration,
    UnicodeVersion,
    OriginalByteSpan,
    Determinism,
    PositiveFixture,
    NegativeFixture,
    SecurityFixture,
    ExplicitExclusion,
}

impl RegistryDefermentBoundary {
    const ALL: [Self; 60] = [
        Self::RegistryStrategy,
        Self::Deferred,
        Self::Unsupported,
        Self::VersionOneScope,
        Self::LocalManifest,
        Self::LocalDependency,
        Self::LocalSource,
        Self::LockedResolution,
        Self::OfflineResolution,
        Self::PackageName,
        Self::ContentIdentity,
        Self::GraphIdentity,
        Self::LockProtocol,
        Self::Experimental,
        Self::StableClaim,
        Self::PreviewClaim,
        Self::ReopeningCriteria,
        Self::AcceptedAuthority,
        Self::PublisherCoordinate,
        Self::PublisherAuthentication,
        Self::NamespaceOwnership,
        Self::NamespaceTransfer,
        Self::RegistryIndex,
        Self::Upload,
        Self::Download,
        Self::Installation,
        Self::Update,
        Self::Rollback,
        Self::Yanked,
        Self::Deprecated,
        Self::Archive,
        Self::Artifact,
        Self::Checksum,
        Self::Signature,
        Self::Provenance,
        Self::Sbom,
        Self::License,
        Self::Transparency,
        Self::Mirror,
        Self::Cache,
        Self::Availability,
        Self::RateLimit,
        Self::Retry,
        Self::Credential,
        Self::TrustRoot,
        Self::ThreatModel,
        Self::CliCommand,
        Self::ExitCode,
        Self::JsonProtocol,
        Self::BilingualDiagnostic,
        Self::Schema,
        Self::CompatibilityReader,
        Self::Migration,
        Self::UnicodeVersion,
        Self::OriginalByteSpan,
        Self::Determinism,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::SecurityFixture,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegistryDefermentInventory {
    boundaries: Box<[RegistryDefermentBoundary]>,
}

impl RegistryDefermentInventory {
    fn new(
        boundaries: impl IntoIterator<Item = RegistryDefermentBoundary>,
    ) -> Result<Self, RegistryDefermentBoundary> {
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
        let mut bytes = b"ling.registry-deferment-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn registry_deferment_boundaries_are_complete_and_ordered() {
    let inventory = RegistryDefermentInventory::new(RegistryDefermentBoundary::ALL)
        .expect("registry deferment boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &RegistryDefermentBoundary::ALL
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
fn registry_deferment_evidence_is_order_independent_and_duplicate_checked() {
    let forward = RegistryDefermentInventory::new(RegistryDefermentBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = RegistryDefermentInventory::new(RegistryDefermentBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = RegistryDefermentInventory::new([
        RegistryDefermentBoundary::Deferred,
        RegistryDefermentBoundary::Deferred,
    ])
    .expect_err("duplicate registry-deferment boundary must be rejected");
    assert_eq!(duplicate, RegistryDefermentBoundary::Deferred);
}

#[test]
fn deferment_does_not_authorize_a_registry_protocol() {
    let inventory = RegistryDefermentInventory::new([
        RegistryDefermentBoundary::RegistryStrategy,
        RegistryDefermentBoundary::Deferred,
        RegistryDefermentBoundary::Unsupported,
        RegistryDefermentBoundary::PublisherCoordinate,
        RegistryDefermentBoundary::RegistryIndex,
        RegistryDefermentBoundary::Upload,
        RegistryDefermentBoundary::Download,
        RegistryDefermentBoundary::Installation,
        RegistryDefermentBoundary::Signature,
        RegistryDefermentBoundary::CliCommand,
        RegistryDefermentBoundary::AcceptedAuthority,
        RegistryDefermentBoundary::ExplicitExclusion,
    ])
    .expect("bounded registry-deferment evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.registry-deferment-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
