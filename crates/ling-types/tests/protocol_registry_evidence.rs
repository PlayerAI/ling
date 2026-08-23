use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedProtocolRegistryBoundary {
    ProtocolRegistry,
    SingleSourceOfTruth,
    RegistrySchemaVersion,
    ProtocolIdentity,
    Title,
    Category,
    Visibility,
    CurrentVersion,
    Stability,
    Implemented,
    PublicSchema,
    Canonical,
    Owner,
    Producer,
    Consumer,
    ReaderPolicy,
    WriterPolicy,
    UnknownFieldPolicy,
    MissingFieldPolicy,
    MigrationTool,
    Fixture,
    Source,
    VersionMarker,
    Authority,
    Note,
    Public,
    Internal,
    PlannedPublic,
    Experimental,
    Preview,
    Stable,
    Future,
    Json,
    Cli,
    Lsp,
    HumanOutput,
    CanonicalIdentity,
    TextProtocol,
    Incident,
    Transaction,
    PackageMetadata,
    Bytecode,
    RuntimeControl,
    Replay,
    Abi,
    Evidence,
    DuplicateId,
    MissingRequiredRecord,
    InvalidPath,
    MissingVersion,
    MissingMarker,
    UnacceptedAuthority,
    FalseStableClaim,
    FutureOverclaim,
    DeterministicRender,
    SchemaRegistryLink,
    SupportMatrixLink,
    TraceabilityLink,
    GoldenCorpus,
    ExplicitExclusion,
}

impl PlannedProtocolRegistryBoundary {
    const ALL: [Self; 60] = [
        Self::ProtocolRegistry,
        Self::SingleSourceOfTruth,
        Self::RegistrySchemaVersion,
        Self::ProtocolIdentity,
        Self::Title,
        Self::Category,
        Self::Visibility,
        Self::CurrentVersion,
        Self::Stability,
        Self::Implemented,
        Self::PublicSchema,
        Self::Canonical,
        Self::Owner,
        Self::Producer,
        Self::Consumer,
        Self::ReaderPolicy,
        Self::WriterPolicy,
        Self::UnknownFieldPolicy,
        Self::MissingFieldPolicy,
        Self::MigrationTool,
        Self::Fixture,
        Self::Source,
        Self::VersionMarker,
        Self::Authority,
        Self::Note,
        Self::Public,
        Self::Internal,
        Self::PlannedPublic,
        Self::Experimental,
        Self::Preview,
        Self::Stable,
        Self::Future,
        Self::Json,
        Self::Cli,
        Self::Lsp,
        Self::HumanOutput,
        Self::CanonicalIdentity,
        Self::TextProtocol,
        Self::Incident,
        Self::Transaction,
        Self::PackageMetadata,
        Self::Bytecode,
        Self::RuntimeControl,
        Self::Replay,
        Self::Abi,
        Self::Evidence,
        Self::DuplicateId,
        Self::MissingRequiredRecord,
        Self::InvalidPath,
        Self::MissingVersion,
        Self::MissingMarker,
        Self::UnacceptedAuthority,
        Self::FalseStableClaim,
        Self::FutureOverclaim,
        Self::DeterministicRender,
        Self::SchemaRegistryLink,
        Self::SupportMatrixLink,
        Self::TraceabilityLink,
        Self::GoldenCorpus,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProtocolRegistryInventory {
    boundaries: Box<[PlannedProtocolRegistryBoundary]>,
}

impl ProtocolRegistryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedProtocolRegistryBoundary>,
    ) -> Result<Self, PlannedProtocolRegistryBoundary> {
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
        let mut bytes = b"ling.protocol-registry-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_protocol_registry_boundaries_are_complete_and_ordered() {
    let inventory = ProtocolRegistryInventory::new(PlannedProtocolRegistryBoundary::ALL)
        .expect("planned protocol-registry boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedProtocolRegistryBoundary::ALL
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
fn protocol_registry_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ProtocolRegistryInventory::new(PlannedProtocolRegistryBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ProtocolRegistryInventory::new(PlannedProtocolRegistryBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ProtocolRegistryInventory::new([
        PlannedProtocolRegistryBoundary::ProtocolRegistry,
        PlannedProtocolRegistryBoundary::ProtocolRegistry,
    ])
    .expect_err("duplicate protocol-registry boundary must be rejected");
    assert_eq!(duplicate, PlannedProtocolRegistryBoundary::ProtocolRegistry);
}

#[test]
fn protocol_registry_evidence_has_no_public_registry_authority() {
    let inventory = ProtocolRegistryInventory::new([
        PlannedProtocolRegistryBoundary::ProtocolRegistry,
        PlannedProtocolRegistryBoundary::SingleSourceOfTruth,
        PlannedProtocolRegistryBoundary::Owner,
        PlannedProtocolRegistryBoundary::Stable,
        PlannedProtocolRegistryBoundary::Future,
        PlannedProtocolRegistryBoundary::FalseStableClaim,
        PlannedProtocolRegistryBoundary::FutureOverclaim,
        PlannedProtocolRegistryBoundary::SchemaRegistryLink,
        PlannedProtocolRegistryBoundary::SupportMatrixLink,
        PlannedProtocolRegistryBoundary::TraceabilityLink,
        PlannedProtocolRegistryBoundary::GoldenCorpus,
        PlannedProtocolRegistryBoundary::ExplicitExclusion,
    ])
    .expect("bounded protocol-registry evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.protocol-registry-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
