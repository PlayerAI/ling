use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedFfiShimBoundary {
    ShimInputSchema,
    DeclarationVersion,
    AbiFacts,
    TargetIdentity,
    LayoutFacts,
    OwnershipFacts,
    CapabilityFacts,
    EncodingFacts,
    CallbackFacts,
    ErrorFacts,
    SourceSpanProjection,
    SemanticIdProjection,
    GeneratorIdentity,
    TemplateIdentity,
    TemplateTrustBoundary,
    GeneratorVersion,
    OutputLanguage,
    OutputSchema,
    GeneratedSource,
    PublicProtocolExclusion,
    GeneratedArtifactIdentity,
    GeneratedMetadata,
    LayoutAssertions,
    NullChecks,
    BoundsChecks,
    OverflowChecks,
    MutabilityChecks,
    EncodingChecks,
    OwnershipConversion,
    AllocatorConversion,
    DropConversion,
    CallbackTrampoline,
    CallbackLifetime,
    CallbackThread,
    CallbackReentry,
    Cancellation,
    FaultMapping,
    CapabilityChecks,
    UnsupportedConstructs,
    UnknownFieldPolicy,
    CanonicalOrdering,
    CanonicalBytes,
    DeterministicGeneration,
    CleanGeneration,
    RepeatGeneration,
    OfflineInputs,
    Provenance,
    LicenseTcb,
    TamperDetection,
    BuildHashSeparation,
    SemanticIdSeparation,
    PathTimestampExclusion,
    HostAddressExclusion,
    DiagnosticBilingual,
    UnicodeSpans,
    SchemaMigration,
    TargetCompatibility,
    SanitizerFuzz,
    CompilerDifferential,
    SeedCompatibility,
}

impl PlannedFfiShimBoundary {
    const ALL: [Self; 60] = [
        Self::ShimInputSchema,
        Self::DeclarationVersion,
        Self::AbiFacts,
        Self::TargetIdentity,
        Self::LayoutFacts,
        Self::OwnershipFacts,
        Self::CapabilityFacts,
        Self::EncodingFacts,
        Self::CallbackFacts,
        Self::ErrorFacts,
        Self::SourceSpanProjection,
        Self::SemanticIdProjection,
        Self::GeneratorIdentity,
        Self::TemplateIdentity,
        Self::TemplateTrustBoundary,
        Self::GeneratorVersion,
        Self::OutputLanguage,
        Self::OutputSchema,
        Self::GeneratedSource,
        Self::PublicProtocolExclusion,
        Self::GeneratedArtifactIdentity,
        Self::GeneratedMetadata,
        Self::LayoutAssertions,
        Self::NullChecks,
        Self::BoundsChecks,
        Self::OverflowChecks,
        Self::MutabilityChecks,
        Self::EncodingChecks,
        Self::OwnershipConversion,
        Self::AllocatorConversion,
        Self::DropConversion,
        Self::CallbackTrampoline,
        Self::CallbackLifetime,
        Self::CallbackThread,
        Self::CallbackReentry,
        Self::Cancellation,
        Self::FaultMapping,
        Self::CapabilityChecks,
        Self::UnsupportedConstructs,
        Self::UnknownFieldPolicy,
        Self::CanonicalOrdering,
        Self::CanonicalBytes,
        Self::DeterministicGeneration,
        Self::CleanGeneration,
        Self::RepeatGeneration,
        Self::OfflineInputs,
        Self::Provenance,
        Self::LicenseTcb,
        Self::TamperDetection,
        Self::BuildHashSeparation,
        Self::SemanticIdSeparation,
        Self::PathTimestampExclusion,
        Self::HostAddressExclusion,
        Self::DiagnosticBilingual,
        Self::UnicodeSpans,
        Self::SchemaMigration,
        Self::TargetCompatibility,
        Self::SanitizerFuzz,
        Self::CompilerDifferential,
        Self::SeedCompatibility,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ShimInputSchema => 0,
            Self::DeclarationVersion => 1,
            Self::AbiFacts => 2,
            Self::TargetIdentity => 3,
            Self::LayoutFacts => 4,
            Self::OwnershipFacts => 5,
            Self::CapabilityFacts => 6,
            Self::EncodingFacts => 7,
            Self::CallbackFacts => 8,
            Self::ErrorFacts => 9,
            Self::SourceSpanProjection => 10,
            Self::SemanticIdProjection => 11,
            Self::GeneratorIdentity => 12,
            Self::TemplateIdentity => 13,
            Self::TemplateTrustBoundary => 14,
            Self::GeneratorVersion => 15,
            Self::OutputLanguage => 16,
            Self::OutputSchema => 17,
            Self::GeneratedSource => 18,
            Self::PublicProtocolExclusion => 19,
            Self::GeneratedArtifactIdentity => 20,
            Self::GeneratedMetadata => 21,
            Self::LayoutAssertions => 22,
            Self::NullChecks => 23,
            Self::BoundsChecks => 24,
            Self::OverflowChecks => 25,
            Self::MutabilityChecks => 26,
            Self::EncodingChecks => 27,
            Self::OwnershipConversion => 28,
            Self::AllocatorConversion => 29,
            Self::DropConversion => 30,
            Self::CallbackTrampoline => 31,
            Self::CallbackLifetime => 32,
            Self::CallbackThread => 33,
            Self::CallbackReentry => 34,
            Self::Cancellation => 35,
            Self::FaultMapping => 36,
            Self::CapabilityChecks => 37,
            Self::UnsupportedConstructs => 38,
            Self::UnknownFieldPolicy => 39,
            Self::CanonicalOrdering => 40,
            Self::CanonicalBytes => 41,
            Self::DeterministicGeneration => 42,
            Self::CleanGeneration => 43,
            Self::RepeatGeneration => 44,
            Self::OfflineInputs => 45,
            Self::Provenance => 46,
            Self::LicenseTcb => 47,
            Self::TamperDetection => 48,
            Self::BuildHashSeparation => 49,
            Self::SemanticIdSeparation => 50,
            Self::PathTimestampExclusion => 51,
            Self::HostAddressExclusion => 52,
            Self::DiagnosticBilingual => 53,
            Self::UnicodeSpans => 54,
            Self::SchemaMigration => 55,
            Self::TargetCompatibility => 56,
            Self::SanitizerFuzz => 57,
            Self::CompilerDifferential => 58,
            Self::SeedCompatibility => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FfiShimBoundaryInventory {
    boundaries: Box<[PlannedFfiShimBoundary]>,
}

impl FfiShimBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedFfiShimBoundary>,
    ) -> Result<Self, PlannedFfiShimBoundary> {
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
        bytes.extend_from_slice(b"ling.ffi-shim-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_ffi_shim_boundaries_are_complete_and_ordered() {
    let inventory = FfiShimBoundaryInventory::new(PlannedFfiShimBoundary::ALL)
        .expect("planned FFI shim boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedFfiShimBoundary::ALL);
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
fn ffi_shim_evidence_is_order_independent_and_duplicate_checked() {
    let forward = FfiShimBoundaryInventory::new(PlannedFfiShimBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = FfiShimBoundaryInventory::new(PlannedFfiShimBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = FfiShimBoundaryInventory::new([
        PlannedFfiShimBoundary::ShimInputSchema,
        PlannedFfiShimBoundary::ShimInputSchema,
    ])
    .expect_err("duplicate FFI shim boundary must be rejected");
    assert_eq!(duplicate, PlannedFfiShimBoundary::ShimInputSchema);
}

#[test]
fn ffi_shim_evidence_has_no_generator_or_artifact_authority() {
    let inventory = FfiShimBoundaryInventory::new([
        PlannedFfiShimBoundary::ShimInputSchema,
        PlannedFfiShimBoundary::GeneratorIdentity,
        PlannedFfiShimBoundary::LayoutAssertions,
        PlannedFfiShimBoundary::OwnershipConversion,
        PlannedFfiShimBoundary::CallbackTrampoline,
        PlannedFfiShimBoundary::Provenance,
        PlannedFfiShimBoundary::BuildHashSeparation,
        PlannedFfiShimBoundary::SeedCompatibility,
    ])
    .expect("bounded FFI shim evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.ffi-shim-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
