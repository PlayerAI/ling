use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedTargetPackageBoundary {
    TargetIdentity,
    TargetTriple,
    PackageIdentity,
    PackageVersion,
    PackageSource,
    ManifestSchema,
    LingAbiSchema,
    PrimitiveIdentity,
    PrimitiveSignature,
    PrimitiveLayout,
    CapabilityRequirements,
    ProfileAvailability,
    TargetAvailability,
    DependencyClosure,
    LockEntry,
    CanonicalBytes,
    UnknownFieldPolicy,
    MigrationPolicy,
    PrimitiveImplementation,
    ImplementationLanguage,
    UnsafeBoundary,
    TrustOwner,
    SignatureVerification,
    TcbDeclaration,
    ProofStatus,
    CompilerAssumptions,
    BackendAssumptions,
    RuntimeAssumptions,
    LicenseProvenance,
    Revocation,
    UpdateCompatibility,
    ArtifactIdentity,
    ShimProvenance,
    ToolchainInputs,
    OfflineInputs,
    DeterministicOrdering,
    SemanticIdProjection,
    SourceSpanProjection,
    AbiSelection,
    CallingConvention,
    OwnershipTransfer,
    BorrowDuration,
    ResourceDrop,
    ManagedBoundary,
    ThreadSafety,
    Reentrancy,
    Blocking,
    ErrorMapping,
    FaultMapping,
    BoundsAliasing,
    CapabilityAdmission,
    ProfileRejection,
    UnknownTargetRejection,
    UnsupportedPrimitive,
    DiagnosticBilingual,
    UnicodeSpans,
    SanitizerFuzz,
    CrossTargetEvidence,
    SeedCompatibility,
    PublicProtocolExclusion,
}

impl PlannedTargetPackageBoundary {
    const ALL: [Self; 60] = [
        Self::TargetIdentity,
        Self::TargetTriple,
        Self::PackageIdentity,
        Self::PackageVersion,
        Self::PackageSource,
        Self::ManifestSchema,
        Self::LingAbiSchema,
        Self::PrimitiveIdentity,
        Self::PrimitiveSignature,
        Self::PrimitiveLayout,
        Self::CapabilityRequirements,
        Self::ProfileAvailability,
        Self::TargetAvailability,
        Self::DependencyClosure,
        Self::LockEntry,
        Self::CanonicalBytes,
        Self::UnknownFieldPolicy,
        Self::MigrationPolicy,
        Self::PrimitiveImplementation,
        Self::ImplementationLanguage,
        Self::UnsafeBoundary,
        Self::TrustOwner,
        Self::SignatureVerification,
        Self::TcbDeclaration,
        Self::ProofStatus,
        Self::CompilerAssumptions,
        Self::BackendAssumptions,
        Self::RuntimeAssumptions,
        Self::LicenseProvenance,
        Self::Revocation,
        Self::UpdateCompatibility,
        Self::ArtifactIdentity,
        Self::ShimProvenance,
        Self::ToolchainInputs,
        Self::OfflineInputs,
        Self::DeterministicOrdering,
        Self::SemanticIdProjection,
        Self::SourceSpanProjection,
        Self::AbiSelection,
        Self::CallingConvention,
        Self::OwnershipTransfer,
        Self::BorrowDuration,
        Self::ResourceDrop,
        Self::ManagedBoundary,
        Self::ThreadSafety,
        Self::Reentrancy,
        Self::Blocking,
        Self::ErrorMapping,
        Self::FaultMapping,
        Self::BoundsAliasing,
        Self::CapabilityAdmission,
        Self::ProfileRejection,
        Self::UnknownTargetRejection,
        Self::UnsupportedPrimitive,
        Self::DiagnosticBilingual,
        Self::UnicodeSpans,
        Self::SanitizerFuzz,
        Self::CrossTargetEvidence,
        Self::SeedCompatibility,
        Self::PublicProtocolExclusion,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::TargetIdentity => 0,
            Self::TargetTriple => 1,
            Self::PackageIdentity => 2,
            Self::PackageVersion => 3,
            Self::PackageSource => 4,
            Self::ManifestSchema => 5,
            Self::LingAbiSchema => 6,
            Self::PrimitiveIdentity => 7,
            Self::PrimitiveSignature => 8,
            Self::PrimitiveLayout => 9,
            Self::CapabilityRequirements => 10,
            Self::ProfileAvailability => 11,
            Self::TargetAvailability => 12,
            Self::DependencyClosure => 13,
            Self::LockEntry => 14,
            Self::CanonicalBytes => 15,
            Self::UnknownFieldPolicy => 16,
            Self::MigrationPolicy => 17,
            Self::PrimitiveImplementation => 18,
            Self::ImplementationLanguage => 19,
            Self::UnsafeBoundary => 20,
            Self::TrustOwner => 21,
            Self::SignatureVerification => 22,
            Self::TcbDeclaration => 23,
            Self::ProofStatus => 24,
            Self::CompilerAssumptions => 25,
            Self::BackendAssumptions => 26,
            Self::RuntimeAssumptions => 27,
            Self::LicenseProvenance => 28,
            Self::Revocation => 29,
            Self::UpdateCompatibility => 30,
            Self::ArtifactIdentity => 31,
            Self::ShimProvenance => 32,
            Self::ToolchainInputs => 33,
            Self::OfflineInputs => 34,
            Self::DeterministicOrdering => 35,
            Self::SemanticIdProjection => 36,
            Self::SourceSpanProjection => 37,
            Self::AbiSelection => 38,
            Self::CallingConvention => 39,
            Self::OwnershipTransfer => 40,
            Self::BorrowDuration => 41,
            Self::ResourceDrop => 42,
            Self::ManagedBoundary => 43,
            Self::ThreadSafety => 44,
            Self::Reentrancy => 45,
            Self::Blocking => 46,
            Self::ErrorMapping => 47,
            Self::FaultMapping => 48,
            Self::BoundsAliasing => 49,
            Self::CapabilityAdmission => 50,
            Self::ProfileRejection => 51,
            Self::UnknownTargetRejection => 52,
            Self::UnsupportedPrimitive => 53,
            Self::DiagnosticBilingual => 54,
            Self::UnicodeSpans => 55,
            Self::SanitizerFuzz => 56,
            Self::CrossTargetEvidence => 57,
            Self::SeedCompatibility => 58,
            Self::PublicProtocolExclusion => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TargetPackageBoundaryInventory {
    boundaries: Box<[PlannedTargetPackageBoundary]>,
}

impl TargetPackageBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedTargetPackageBoundary>,
    ) -> Result<Self, PlannedTargetPackageBoundary> {
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
        bytes.extend_from_slice(b"ling.target-primitive-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_target_package_boundaries_are_complete_and_ordered() {
    let inventory = TargetPackageBoundaryInventory::new(PlannedTargetPackageBoundary::ALL)
        .expect("planned target package boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedTargetPackageBoundary::ALL
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
fn target_package_evidence_is_order_independent_and_duplicate_checked() {
    let forward = TargetPackageBoundaryInventory::new(PlannedTargetPackageBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        TargetPackageBoundaryInventory::new(PlannedTargetPackageBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = TargetPackageBoundaryInventory::new([
        PlannedTargetPackageBoundary::TargetIdentity,
        PlannedTargetPackageBoundary::TargetIdentity,
    ])
    .expect_err("duplicate target package boundary must be rejected");
    assert_eq!(duplicate, PlannedTargetPackageBoundary::TargetIdentity);
}

#[test]
fn target_package_evidence_has_no_package_or_tcb_authority() {
    let inventory = TargetPackageBoundaryInventory::new([
        PlannedTargetPackageBoundary::TargetIdentity,
        PlannedTargetPackageBoundary::LingAbiSchema,
        PlannedTargetPackageBoundary::PrimitiveSignature,
        PlannedTargetPackageBoundary::CapabilityAdmission,
        PlannedTargetPackageBoundary::SignatureVerification,
        PlannedTargetPackageBoundary::TcbDeclaration,
        PlannedTargetPackageBoundary::UnknownTargetRejection,
        PlannedTargetPackageBoundary::SeedCompatibility,
    ])
    .expect("bounded target package evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.target-primitive-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
