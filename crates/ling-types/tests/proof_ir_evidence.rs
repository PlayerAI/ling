use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedProofIrBoundary {
    ProofIr,
    Version,
    Sort,
    Type,
    Term,
    Variable,
    Constant,
    Application,
    Hypothesis,
    Theorem,
    Goal,
    Rule,
    ProofStep,
    CertificateRef,
    ArithmeticAxiom,
    MemoryAxiom,
    AliasAxiom,
    EffectAxiom,
    OwnershipAxiom,
    Bound,
    Termination,
    NodeTaskActor,
    FfiAbi,
    ExternalFact,
    Assumption,
    Provenance,
    SourceSpan,
    SemanticId,
    ProofId,
    CanonicalBytes,
    Normalization,
    WellFormedness,
    Translation,
    ContractCore,
    TypedCore,
    StatusSeparation,
    RuntimeEvidence,
    TestEvidence,
    ModelEvidence,
    Unknown,
    Malformed,
    Corrupt,
    FailClosed,
    Tcb,
    Kernel,
    Soundness,
    IndependentChecker,
    ResourceLimit,
    DeterministicOrdering,
    Migration,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    AdversarialFixture,
    ProofRejectionFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedProofIrBoundary {
    const ALL: [Self; 60] = [
        Self::ProofIr,
        Self::Version,
        Self::Sort,
        Self::Type,
        Self::Term,
        Self::Variable,
        Self::Constant,
        Self::Application,
        Self::Hypothesis,
        Self::Theorem,
        Self::Goal,
        Self::Rule,
        Self::ProofStep,
        Self::CertificateRef,
        Self::ArithmeticAxiom,
        Self::MemoryAxiom,
        Self::AliasAxiom,
        Self::EffectAxiom,
        Self::OwnershipAxiom,
        Self::Bound,
        Self::Termination,
        Self::NodeTaskActor,
        Self::FfiAbi,
        Self::ExternalFact,
        Self::Assumption,
        Self::Provenance,
        Self::SourceSpan,
        Self::SemanticId,
        Self::ProofId,
        Self::CanonicalBytes,
        Self::Normalization,
        Self::WellFormedness,
        Self::Translation,
        Self::ContractCore,
        Self::TypedCore,
        Self::StatusSeparation,
        Self::RuntimeEvidence,
        Self::TestEvidence,
        Self::ModelEvidence,
        Self::Unknown,
        Self::Malformed,
        Self::Corrupt,
        Self::FailClosed,
        Self::Tcb,
        Self::Kernel,
        Self::Soundness,
        Self::IndependentChecker,
        Self::ResourceLimit,
        Self::DeterministicOrdering,
        Self::Migration,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::AdversarialFixture,
        Self::ProofRejectionFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProofIrInventory {
    boundaries: Box<[PlannedProofIrBoundary]>,
}

impl ProofIrInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedProofIrBoundary>,
    ) -> Result<Self, PlannedProofIrBoundary> {
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
        let mut bytes = b"ling.proof-ir-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_proof_ir_boundaries_are_complete_and_ordered() {
    let inventory = ProofIrInventory::new(PlannedProofIrBoundary::ALL)
        .expect("planned Proof IR boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedProofIrBoundary::ALL);
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
fn proof_ir_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ProofIrInventory::new(PlannedProofIrBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ProofIrInventory::new(PlannedProofIrBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ProofIrInventory::new([
        PlannedProofIrBoundary::ProofIr,
        PlannedProofIrBoundary::ProofIr,
    ])
    .expect_err("duplicate Proof IR boundary must be rejected");
    assert_eq!(duplicate, PlannedProofIrBoundary::ProofIr);
}

#[test]
fn proof_ir_evidence_has_no_representation_authority() {
    let inventory = ProofIrInventory::new([
        PlannedProofIrBoundary::ProofIr,
        PlannedProofIrBoundary::Theorem,
        PlannedProofIrBoundary::ContractCore,
        PlannedProofIrBoundary::Kernel,
        PlannedProofIrBoundary::DiagnosticCode,
        PlannedProofIrBoundary::ProtocolInventory,
    ])
    .expect("bounded Proof IR evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.proof-ir-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
