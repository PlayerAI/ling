use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedEvidenceBundleBoundary {
    EvidenceBundle,
    Version,
    Manifest,
    ProgramId,
    SourceId,
    SemanticId,
    AuthorityVersion,
    DependencyLock,
    BuildGraph,
    ProfileId,
    TargetId,
    ToolchainId,
    TcbId,
    AuditSource,
    ConformanceResult,
    PropertyResult,
    FuzzResult,
    ContractObligation,
    ProofResult,
    ModelCheckReport,
    CounterexampleTrace,
    ReplayEvidence,
    TimingEvidence,
    MemoryEvidence,
    FfiEvidence,
    TargetPackage,
    Assumption,
    AiProvenance,
    HumanReview,
    Artifact,
    ArtifactDigest,
    CanonicalOrdering,
    SizeLimit,
    Passed,
    Failed,
    Skipped,
    Unavailable,
    Assumed,
    Unknown,
    Bounded,
    NonClaim,
    SourceSpan,
    CrossReference,
    Privacy,
    Redaction,
    Signature,
    TrustRoot,
    OfflineVerification,
    NoCodeExecution,
    UnknownField,
    Malformed,
    Corrupt,
    UnsupportedVersion,
    Migration,
    DiagnosticCode,
    PositiveFixture,
    NegativeFixture,
    UnicodeFixture,
    DeterminismFixture,
    ProtocolInventory,
}

impl PlannedEvidenceBundleBoundary {
    const ALL: [Self; 60] = [
        Self::EvidenceBundle,
        Self::Version,
        Self::Manifest,
        Self::ProgramId,
        Self::SourceId,
        Self::SemanticId,
        Self::AuthorityVersion,
        Self::DependencyLock,
        Self::BuildGraph,
        Self::ProfileId,
        Self::TargetId,
        Self::ToolchainId,
        Self::TcbId,
        Self::AuditSource,
        Self::ConformanceResult,
        Self::PropertyResult,
        Self::FuzzResult,
        Self::ContractObligation,
        Self::ProofResult,
        Self::ModelCheckReport,
        Self::CounterexampleTrace,
        Self::ReplayEvidence,
        Self::TimingEvidence,
        Self::MemoryEvidence,
        Self::FfiEvidence,
        Self::TargetPackage,
        Self::Assumption,
        Self::AiProvenance,
        Self::HumanReview,
        Self::Artifact,
        Self::ArtifactDigest,
        Self::CanonicalOrdering,
        Self::SizeLimit,
        Self::Passed,
        Self::Failed,
        Self::Skipped,
        Self::Unavailable,
        Self::Assumed,
        Self::Unknown,
        Self::Bounded,
        Self::NonClaim,
        Self::SourceSpan,
        Self::CrossReference,
        Self::Privacy,
        Self::Redaction,
        Self::Signature,
        Self::TrustRoot,
        Self::OfflineVerification,
        Self::NoCodeExecution,
        Self::UnknownField,
        Self::Malformed,
        Self::Corrupt,
        Self::UnsupportedVersion,
        Self::Migration,
        Self::DiagnosticCode,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EvidenceBundleInventory {
    boundaries: Box<[PlannedEvidenceBundleBoundary]>,
}

impl EvidenceBundleInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedEvidenceBundleBoundary>,
    ) -> Result<Self, PlannedEvidenceBundleBoundary> {
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
        let mut bytes = b"ling.evidence-bundle-schema-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_evidence_bundle_boundaries_are_complete_and_ordered() {
    let inventory = EvidenceBundleInventory::new(PlannedEvidenceBundleBoundary::ALL)
        .expect("planned evidence-bundle boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedEvidenceBundleBoundary::ALL
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
fn evidence_bundle_evidence_is_order_independent_and_duplicate_checked() {
    let forward = EvidenceBundleInventory::new(PlannedEvidenceBundleBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        EvidenceBundleInventory::new(PlannedEvidenceBundleBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = EvidenceBundleInventory::new([
        PlannedEvidenceBundleBoundary::EvidenceBundle,
        PlannedEvidenceBundleBoundary::EvidenceBundle,
    ])
    .expect_err("duplicate evidence-bundle boundary must be rejected");
    assert_eq!(duplicate, PlannedEvidenceBundleBoundary::EvidenceBundle);
}

#[test]
fn evidence_bundle_evidence_has_no_schema_or_verifier_authority() {
    let inventory = EvidenceBundleInventory::new([
        PlannedEvidenceBundleBoundary::EvidenceBundle,
        PlannedEvidenceBundleBoundary::Manifest,
        PlannedEvidenceBundleBoundary::ArtifactDigest,
        PlannedEvidenceBundleBoundary::NonClaim,
        PlannedEvidenceBundleBoundary::Privacy,
        PlannedEvidenceBundleBoundary::OfflineVerification,
        PlannedEvidenceBundleBoundary::NoCodeExecution,
        PlannedEvidenceBundleBoundary::ProtocolInventory,
    ])
    .expect("bounded evidence-bundle evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.evidence-bundle-schema-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
