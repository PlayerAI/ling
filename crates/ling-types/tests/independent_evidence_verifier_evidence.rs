use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedIndependentEvidenceVerifierBoundary {
    IndependentVerifier,
    Version,
    BundleInput,
    SchemaCheck,
    VersionCheck,
    CanonicalCheck,
    HashCheck,
    ArtifactLinkage,
    ProofCertificate,
    TestReportIdentity,
    TestReportSignature,
    DependencyLock,
    ToolchainIdentity,
    ProfileIdentity,
    TargetIdentity,
    BuildIdentity,
    SemanticId,
    SourceSpan,
    EvidencePolarity,
    NonClaim,
    TrustRoot,
    VerifierIdentity,
    VerifierVersion,
    TcbIdentity,
    KeyIdentity,
    Revocation,
    OfflineMode,
    NetworkDenied,
    NoCodeExecution,
    UnicodeFixture,
    CommandDenied,
    FfiDenied,
    DeterminismFixture,
    ResourceLimit,
    DeterministicOrdering,
    Valid,
    Invalid,
    Unknown,
    Unsupported,
    MissingField,
    UnknownField,
    HashMismatch,
    LinkMismatch,
    InvalidCertificate,
    UnavailableInput,
    StaleIdentity,
    TrustFailure,
    ResourceExhaustion,
    Malformed,
    Corrupt,
    UnsupportedVersion,
    Migration,
    FailClosed,
    DiagnosticCode,
    PositiveFixture,
    NegativeFixture,
    TamperedLinkFixture,
    InvalidCertificateFixture,
    NoCodeExecutionFixture,
    ProtocolInventory,
}

impl PlannedIndependentEvidenceVerifierBoundary {
    const ALL: [Self; 60] = [
        Self::IndependentVerifier,
        Self::Version,
        Self::BundleInput,
        Self::SchemaCheck,
        Self::VersionCheck,
        Self::CanonicalCheck,
        Self::HashCheck,
        Self::ArtifactLinkage,
        Self::ProofCertificate,
        Self::TestReportIdentity,
        Self::TestReportSignature,
        Self::DependencyLock,
        Self::ToolchainIdentity,
        Self::ProfileIdentity,
        Self::TargetIdentity,
        Self::BuildIdentity,
        Self::SemanticId,
        Self::SourceSpan,
        Self::EvidencePolarity,
        Self::NonClaim,
        Self::TrustRoot,
        Self::VerifierIdentity,
        Self::VerifierVersion,
        Self::TcbIdentity,
        Self::KeyIdentity,
        Self::Revocation,
        Self::OfflineMode,
        Self::NetworkDenied,
        Self::NoCodeExecution,
        Self::UnicodeFixture,
        Self::CommandDenied,
        Self::FfiDenied,
        Self::DeterminismFixture,
        Self::ResourceLimit,
        Self::DeterministicOrdering,
        Self::Valid,
        Self::Invalid,
        Self::Unknown,
        Self::Unsupported,
        Self::MissingField,
        Self::UnknownField,
        Self::HashMismatch,
        Self::LinkMismatch,
        Self::InvalidCertificate,
        Self::UnavailableInput,
        Self::StaleIdentity,
        Self::TrustFailure,
        Self::ResourceExhaustion,
        Self::Malformed,
        Self::Corrupt,
        Self::UnsupportedVersion,
        Self::Migration,
        Self::FailClosed,
        Self::DiagnosticCode,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::TamperedLinkFixture,
        Self::InvalidCertificateFixture,
        Self::NoCodeExecutionFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IndependentEvidenceVerifierInventory {
    boundaries: Box<[PlannedIndependentEvidenceVerifierBoundary]>,
}

impl IndependentEvidenceVerifierInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedIndependentEvidenceVerifierBoundary>,
    ) -> Result<Self, PlannedIndependentEvidenceVerifierBoundary> {
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
        let mut bytes = b"ling.independent-evidence-verifier-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_independent_evidence_verifier_boundaries_are_complete_and_ordered() {
    let inventory =
        IndependentEvidenceVerifierInventory::new(PlannedIndependentEvidenceVerifierBoundary::ALL)
            .expect("planned independent-evidence-verifier boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedIndependentEvidenceVerifierBoundary::ALL
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
fn independent_evidence_verifier_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        IndependentEvidenceVerifierInventory::new(PlannedIndependentEvidenceVerifierBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = IndependentEvidenceVerifierInventory::new(
        PlannedIndependentEvidenceVerifierBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = IndependentEvidenceVerifierInventory::new([
        PlannedIndependentEvidenceVerifierBoundary::IndependentVerifier,
        PlannedIndependentEvidenceVerifierBoundary::IndependentVerifier,
    ])
    .expect_err("duplicate independent-evidence-verifier boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedIndependentEvidenceVerifierBoundary::IndependentVerifier
    );
}

#[test]
fn independent_evidence_verifier_evidence_has_no_verification_authority() {
    let inventory = IndependentEvidenceVerifierInventory::new([
        PlannedIndependentEvidenceVerifierBoundary::IndependentVerifier,
        PlannedIndependentEvidenceVerifierBoundary::TrustRoot,
        PlannedIndependentEvidenceVerifierBoundary::OfflineMode,
        PlannedIndependentEvidenceVerifierBoundary::NetworkDenied,
        PlannedIndependentEvidenceVerifierBoundary::NoCodeExecution,
        PlannedIndependentEvidenceVerifierBoundary::CommandDenied,
        PlannedIndependentEvidenceVerifierBoundary::FfiDenied,
        PlannedIndependentEvidenceVerifierBoundary::FailClosed,
        PlannedIndependentEvidenceVerifierBoundary::ProtocolInventory,
    ])
    .expect("bounded independent-evidence-verifier evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.independent-evidence-verifier-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 9);
}
