use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedSolverProofCheckerBoundary {
    SolverProofChecker,
    ProofIrVersion,
    QueryVersion,
    CertificateVersion,
    CandidateSolver,
    SolverIdentity,
    SolverVersion,
    SolverConfig,
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
    InvalidModel,
    Checker,
    CheckerVersion,
    IndependentChecker,
    Soundness,
    TrustedAssumption,
    TcbEntry,
    TcbScope,
    Bound,
    Cancellation,
    FailClosed,
    ResourceLimit,
    DeterministicOrdering,
    StdoutUntrusted,
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
    ProfileGate,
    OptimizationGate,
    VerifierError,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    TimeoutFixture,
    UnknownFixture,
    CorruptionFixture,
    MigrationFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedSolverProofCheckerBoundary {
    const ALL: [Self; 60] = [
        Self::SolverProofChecker,
        Self::ProofIrVersion,
        Self::QueryVersion,
        Self::CertificateVersion,
        Self::CandidateSolver,
        Self::SolverIdentity,
        Self::SolverVersion,
        Self::SolverConfig,
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
        Self::InvalidModel,
        Self::Checker,
        Self::CheckerVersion,
        Self::IndependentChecker,
        Self::Soundness,
        Self::TrustedAssumption,
        Self::TcbEntry,
        Self::TcbScope,
        Self::Bound,
        Self::Cancellation,
        Self::FailClosed,
        Self::ResourceLimit,
        Self::DeterministicOrdering,
        Self::StdoutUntrusted,
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
        Self::ProfileGate,
        Self::OptimizationGate,
        Self::VerifierError,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::TimeoutFixture,
        Self::UnknownFixture,
        Self::CorruptionFixture,
        Self::MigrationFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SolverProofCheckerInventory {
    boundaries: Box<[PlannedSolverProofCheckerBoundary]>,
}

impl SolverProofCheckerInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedSolverProofCheckerBoundary>,
    ) -> Result<Self, PlannedSolverProofCheckerBoundary> {
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
        let mut bytes = b"ling.solver-proof-checker-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_solver_proof_checker_boundaries_are_complete_and_ordered() {
    let inventory = SolverProofCheckerInventory::new(PlannedSolverProofCheckerBoundary::ALL)
        .expect("planned solver/proof-checker boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedSolverProofCheckerBoundary::ALL
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
fn solver_proof_checker_evidence_is_order_independent_and_duplicate_checked() {
    let forward = SolverProofCheckerInventory::new(PlannedSolverProofCheckerBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        SolverProofCheckerInventory::new(PlannedSolverProofCheckerBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = SolverProofCheckerInventory::new([
        PlannedSolverProofCheckerBoundary::SolverProofChecker,
        PlannedSolverProofCheckerBoundary::SolverProofChecker,
    ])
    .expect_err("duplicate solver/proof-checker boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedSolverProofCheckerBoundary::SolverProofChecker
    );
}

#[test]
fn solver_proof_checker_evidence_has_no_adapter_authority() {
    let inventory = SolverProofCheckerInventory::new([
        PlannedSolverProofCheckerBoundary::SolverProofChecker,
        PlannedSolverProofCheckerBoundary::CandidateSolver,
        PlannedSolverProofCheckerBoundary::ProofCertificate,
        PlannedSolverProofCheckerBoundary::IndependentChecker,
        PlannedSolverProofCheckerBoundary::DiagnosticCode,
        PlannedSolverProofCheckerBoundary::ProtocolInventory,
    ])
    .expect("bounded solver/proof-checker evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.solver-proof-checker-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
