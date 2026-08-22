use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedContractVcBoundary {
    ProofIrVc,
    Version,
    WellFormedness,
    CanonicalBytes,
    ObligationId,
    ContractId,
    SemanticId,
    SourceSpan,
    Ssa,
    PathCondition,
    Branch,
    LoopInvariant,
    Recursion,
    Termination,
    ArithmeticModel,
    Overflow,
    Rounding,
    MemoryFact,
    AliasFact,
    OwnershipFact,
    EffectFact,
    CapabilityFact,
    TimingFact,
    FfiFact,
    ExternalAssumption,
    TrustedAssumption,
    TcbEntry,
    TranslationRule,
    SoundnessClaim,
    BoundedReasoning,
    UnboundedReasoning,
    SolverCandidate,
    CheckedCertificate,
    Timeout,
    Unknown,
    InvalidModel,
    FailClosed,
    ResourceLimit,
    DeterministicOrdering,
    EvidenceLink,
    Counterexample,
    Replay,
    Provenance,
    Redaction,
    Revocation,
    Migration,
    DiagnosticCode,
    DiagnosticFacts,
    ProfileGate,
    OptimizationGate,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    ArithmeticFixture,
    AliasEffectFixture,
    AssumptionFixture,
    TimeoutUnknownFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedContractVcBoundary {
    const ALL: [Self; 60] = [
        Self::ProofIrVc,
        Self::Version,
        Self::WellFormedness,
        Self::CanonicalBytes,
        Self::ObligationId,
        Self::ContractId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Ssa,
        Self::PathCondition,
        Self::Branch,
        Self::LoopInvariant,
        Self::Recursion,
        Self::Termination,
        Self::ArithmeticModel,
        Self::Overflow,
        Self::Rounding,
        Self::MemoryFact,
        Self::AliasFact,
        Self::OwnershipFact,
        Self::EffectFact,
        Self::CapabilityFact,
        Self::TimingFact,
        Self::FfiFact,
        Self::ExternalAssumption,
        Self::TrustedAssumption,
        Self::TcbEntry,
        Self::TranslationRule,
        Self::SoundnessClaim,
        Self::BoundedReasoning,
        Self::UnboundedReasoning,
        Self::SolverCandidate,
        Self::CheckedCertificate,
        Self::Timeout,
        Self::Unknown,
        Self::InvalidModel,
        Self::FailClosed,
        Self::ResourceLimit,
        Self::DeterministicOrdering,
        Self::EvidenceLink,
        Self::Counterexample,
        Self::Replay,
        Self::Provenance,
        Self::Redaction,
        Self::Revocation,
        Self::Migration,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::ProfileGate,
        Self::OptimizationGate,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::ArithmeticFixture,
        Self::AliasEffectFixture,
        Self::AssumptionFixture,
        Self::TimeoutUnknownFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContractVcInventory {
    boundaries: Box<[PlannedContractVcBoundary]>,
}

impl ContractVcInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedContractVcBoundary>,
    ) -> Result<Self, PlannedContractVcBoundary> {
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
        let mut bytes = b"ling.contract-vc-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_contract_vc_boundaries_are_complete_and_ordered() {
    let inventory = ContractVcInventory::new(PlannedContractVcBoundary::ALL)
        .expect("planned Contract VC boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedContractVcBoundary::ALL
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
fn contract_vc_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ContractVcInventory::new(PlannedContractVcBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ContractVcInventory::new(PlannedContractVcBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ContractVcInventory::new([
        PlannedContractVcBoundary::ProofIrVc,
        PlannedContractVcBoundary::ProofIrVc,
    ])
    .expect_err("duplicate Contract VC boundary must be rejected");
    assert_eq!(duplicate, PlannedContractVcBoundary::ProofIrVc);
}

#[test]
fn contract_vc_evidence_has_no_proof_authority() {
    let inventory = ContractVcInventory::new([
        PlannedContractVcBoundary::ProofIrVc,
        PlannedContractVcBoundary::Ssa,
        PlannedContractVcBoundary::TrustedAssumption,
        PlannedContractVcBoundary::CheckedCertificate,
        PlannedContractVcBoundary::DiagnosticCode,
        PlannedContractVcBoundary::ProtocolInventory,
    ])
    .expect("bounded Contract VC evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.contract-vc-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
