use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedFeatureStateMetadataBoundary {
    FeatureStateMetadata,
    Version,
    FeatureIdentity,
    ProfileIdentity,
    TargetIdentity,
    CurrentState,
    Stability,
    Implemented,
    Tested,
    Documented,
    Experimental,
    Preview,
    Stable,
    Deprecated,
    Removed,
    Unavailable,
    Partial,
    Blocker,
    Owner,
    Authority,
    SourceOfTruth,
    Release,
    SchemaVersion,
    PublicSchema,
    InternalFixture,
    PublicContract,
    ProposedCommand,
    CliConsumer,
    BuildManifestConsumer,
    PackageConsumer,
    LspConsumer,
    ZedConsumer,
    DocumentationConsumer,
    UnknownField,
    UnknownState,
    MissingField,
    ConflictingState,
    Transition,
    Promotion,
    Demotion,
    Deprecation,
    Removal,
    Migration,
    ReaderPolicy,
    WriterPolicy,
    NMinusOne,
    CanonicalOrdering,
    DeterministicGeneration,
    OfflineGeneration,
    BilingualDiagnostic,
    OriginalUtf8Span,
    UnicodeVersion,
    SemanticId,
    SupportedProfile,
    SupportedTarget,
    SupportMatrix,
    Traceability,
    PositiveFixture,
    NegativeFixture,
    ExplicitExclusion,
}

impl PlannedFeatureStateMetadataBoundary {
    const ALL: [Self; 60] = [
        Self::FeatureStateMetadata,
        Self::Version,
        Self::FeatureIdentity,
        Self::ProfileIdentity,
        Self::TargetIdentity,
        Self::CurrentState,
        Self::Stability,
        Self::Implemented,
        Self::Tested,
        Self::Documented,
        Self::Experimental,
        Self::Preview,
        Self::Stable,
        Self::Deprecated,
        Self::Removed,
        Self::Unavailable,
        Self::Partial,
        Self::Blocker,
        Self::Owner,
        Self::Authority,
        Self::SourceOfTruth,
        Self::Release,
        Self::SchemaVersion,
        Self::PublicSchema,
        Self::InternalFixture,
        Self::PublicContract,
        Self::ProposedCommand,
        Self::CliConsumer,
        Self::BuildManifestConsumer,
        Self::PackageConsumer,
        Self::LspConsumer,
        Self::ZedConsumer,
        Self::DocumentationConsumer,
        Self::UnknownField,
        Self::UnknownState,
        Self::MissingField,
        Self::ConflictingState,
        Self::Transition,
        Self::Promotion,
        Self::Demotion,
        Self::Deprecation,
        Self::Removal,
        Self::Migration,
        Self::ReaderPolicy,
        Self::WriterPolicy,
        Self::NMinusOne,
        Self::CanonicalOrdering,
        Self::DeterministicGeneration,
        Self::OfflineGeneration,
        Self::BilingualDiagnostic,
        Self::OriginalUtf8Span,
        Self::UnicodeVersion,
        Self::SemanticId,
        Self::SupportedProfile,
        Self::SupportedTarget,
        Self::SupportMatrix,
        Self::Traceability,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FeatureStateMetadataInventory {
    boundaries: Box<[PlannedFeatureStateMetadataBoundary]>,
}

impl FeatureStateMetadataInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedFeatureStateMetadataBoundary>,
    ) -> Result<Self, PlannedFeatureStateMetadataBoundary> {
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
        let mut bytes = b"ling.feature-state-metadata-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_feature_state_metadata_boundaries_are_complete_and_ordered() {
    let inventory = FeatureStateMetadataInventory::new(PlannedFeatureStateMetadataBoundary::ALL)
        .expect("planned feature-state-metadata boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedFeatureStateMetadataBoundary::ALL
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
fn feature_state_metadata_evidence_is_order_independent_and_duplicate_checked() {
    let forward = FeatureStateMetadataInventory::new(PlannedFeatureStateMetadataBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = FeatureStateMetadataInventory::new(
        PlannedFeatureStateMetadataBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = FeatureStateMetadataInventory::new([
        PlannedFeatureStateMetadataBoundary::FeatureStateMetadata,
        PlannedFeatureStateMetadataBoundary::FeatureStateMetadata,
    ])
    .expect_err("duplicate feature-state-metadata boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedFeatureStateMetadataBoundary::FeatureStateMetadata
    );
}

#[test]
fn feature_state_metadata_evidence_has_no_public_protocol_authority() {
    let inventory = FeatureStateMetadataInventory::new([
        PlannedFeatureStateMetadataBoundary::FeatureStateMetadata,
        PlannedFeatureStateMetadataBoundary::CurrentState,
        PlannedFeatureStateMetadataBoundary::Stability,
        PlannedFeatureStateMetadataBoundary::Experimental,
        PlannedFeatureStateMetadataBoundary::Preview,
        PlannedFeatureStateMetadataBoundary::Stable,
        PlannedFeatureStateMetadataBoundary::Deprecated,
        PlannedFeatureStateMetadataBoundary::Removed,
        PlannedFeatureStateMetadataBoundary::InternalFixture,
        PlannedFeatureStateMetadataBoundary::PublicContract,
        PlannedFeatureStateMetadataBoundary::ProposedCommand,
        PlannedFeatureStateMetadataBoundary::ExplicitExclusion,
    ])
    .expect("bounded feature-state-metadata evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.feature-state-metadata-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
