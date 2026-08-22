use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedIndependentCheckerBoundary {
    IndependentChecker,
    ProofIrVersion,
    QueryVersion,
    CertificateVersion,
    CandidateInput,
    CheckerIdentity,
    CheckerVersion,
    CheckerConfig,
    ReplayableQuery,
    QueryIdentity,
    ProofCertificate,
    CertificateIdentity,
    WellFormedness,
    CanonicalSerialization,
    ObligationId,
    ContractId,
    SemanticId,
    SourceSpan,
    ResultStatus,
    Timeout,
    Unknown,
    Malformed,
    Corrupt,
    UnsupportedVersion,
    Checker,
    CheckerVersionEvidence,
    IndependentValidation,
    Soundness,
    TrustedAssumption,
    TcbEntry,
    TcbScope,
    Bound,
    Cancellation,
    FailClosed,
    ResourceLimit,
    DeterministicOrdering,
    OfflineDependency,
    Provenance,
    Checksum,
    Signature,
    Redaction,
    Migration,
    Replay,
    Counterexample,
    EvidenceLink,
    DiagnosticCode,
    DiagnosticFacts,
    MachineReadableResult,
    ExitCode,
    FuzzCorpus,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    DeepFixture,
    TimeoutFixture,
    UnknownFixture,
    CorruptionFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedIndependentCheckerBoundary {
    const ALL: [Self; 60] = [
        Self::IndependentChecker,
        Self::ProofIrVersion,
        Self::QueryVersion,
        Self::CertificateVersion,
        Self::CandidateInput,
        Self::CheckerIdentity,
        Self::CheckerVersion,
        Self::CheckerConfig,
        Self::ReplayableQuery,
        Self::QueryIdentity,
        Self::ProofCertificate,
        Self::CertificateIdentity,
        Self::WellFormedness,
        Self::CanonicalSerialization,
        Self::ObligationId,
        Self::ContractId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::ResultStatus,
        Self::Timeout,
        Self::Unknown,
        Self::Malformed,
        Self::Corrupt,
        Self::UnsupportedVersion,
        Self::Checker,
        Self::CheckerVersionEvidence,
        Self::IndependentValidation,
        Self::Soundness,
        Self::TrustedAssumption,
        Self::TcbEntry,
        Self::TcbScope,
        Self::Bound,
        Self::Cancellation,
        Self::FailClosed,
        Self::ResourceLimit,
        Self::DeterministicOrdering,
        Self::OfflineDependency,
        Self::Provenance,
        Self::Checksum,
        Self::Signature,
        Self::Redaction,
        Self::Migration,
        Self::Replay,
        Self::Counterexample,
        Self::EvidenceLink,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::MachineReadableResult,
        Self::ExitCode,
        Self::FuzzCorpus,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::DeepFixture,
        Self::TimeoutFixture,
        Self::UnknownFixture,
        Self::CorruptionFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IndependentCheckerInventory {
    boundaries: Box<[PlannedIndependentCheckerBoundary]>,
}

impl IndependentCheckerInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedIndependentCheckerBoundary>,
    ) -> Result<Self, PlannedIndependentCheckerBoundary> {
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
        let mut bytes = b"ling.independent-checker-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_independent_checker_boundaries_are_complete_and_ordered() {
    let inventory = IndependentCheckerInventory::new(PlannedIndependentCheckerBoundary::ALL)
        .expect("planned independent-checker boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedIndependentCheckerBoundary::ALL
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
fn independent_checker_evidence_is_order_independent_and_duplicate_checked() {
    let forward = IndependentCheckerInventory::new(PlannedIndependentCheckerBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        IndependentCheckerInventory::new(PlannedIndependentCheckerBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = IndependentCheckerInventory::new([
        PlannedIndependentCheckerBoundary::IndependentChecker,
        PlannedIndependentCheckerBoundary::IndependentChecker,
    ])
    .expect_err("duplicate independent-checker boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedIndependentCheckerBoundary::IndependentChecker
    );
}

#[test]
fn independent_checker_evidence_has_no_checker_authority() {
    let inventory = IndependentCheckerInventory::new([
        PlannedIndependentCheckerBoundary::IndependentChecker,
        PlannedIndependentCheckerBoundary::ProofCertificate,
        PlannedIndependentCheckerBoundary::IndependentValidation,
        PlannedIndependentCheckerBoundary::TcbEntry,
        PlannedIndependentCheckerBoundary::MachineReadableResult,
        PlannedIndependentCheckerBoundary::ProtocolInventory,
    ])
    .expect("bounded independent-checker evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.independent-checker-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
