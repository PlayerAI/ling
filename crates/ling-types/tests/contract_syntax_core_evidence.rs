use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedContractSyntaxCoreBoundary {
    ContractSyntax,
    Requires,
    Ensures,
    Invariant,
    Assert,
    RestrictedAssume,
    Expression,
    Precedence,
    LogicalOperators,
    ShortCircuit,
    CoreValue,
    Context,
    SourceSpan,
    MalformedRecovery,
    Pure,
    Total,
    EffectCapability,
    AllocationBound,
    TerminationBound,
    AssumeProvenance,
    ObligationIdentity,
    ContractId,
    DefinitionId,
    CanonicalBytes,
    AlphaNormalization,
    SemanticDiff,
    StatusLifecycle,
    Proved,
    RuntimeChecked,
    ModelChecked,
    Tested,
    Assumed,
    Unknown,
    Failed,
    NotApplicable,
    Timeout,
    Migration,
    VcGeneration,
    SolverCertificate,
    TrustedAssumption,
    ProofChecker,
    RuntimeCheck,
    AssertionOrder,
    EffectIsolation,
    Fault,
    DiagnosticCode,
    DiagnosticFacts,
    ProfileOptimization,
    OwnershipBorrow,
    NodeTaskActor,
    MemoryTiming,
    EvidenceBundle,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    UnicodeFixture,
    MigrationFixture,
    DeterminismFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedContractSyntaxCoreBoundary {
    const ALL: [Self; 60] = [
        Self::ContractSyntax,
        Self::Requires,
        Self::Ensures,
        Self::Invariant,
        Self::Assert,
        Self::RestrictedAssume,
        Self::Expression,
        Self::Precedence,
        Self::LogicalOperators,
        Self::ShortCircuit,
        Self::CoreValue,
        Self::Context,
        Self::SourceSpan,
        Self::MalformedRecovery,
        Self::Pure,
        Self::Total,
        Self::EffectCapability,
        Self::AllocationBound,
        Self::TerminationBound,
        Self::AssumeProvenance,
        Self::ObligationIdentity,
        Self::ContractId,
        Self::DefinitionId,
        Self::CanonicalBytes,
        Self::AlphaNormalization,
        Self::SemanticDiff,
        Self::StatusLifecycle,
        Self::Proved,
        Self::RuntimeChecked,
        Self::ModelChecked,
        Self::Tested,
        Self::Assumed,
        Self::Unknown,
        Self::Failed,
        Self::NotApplicable,
        Self::Timeout,
        Self::Migration,
        Self::VcGeneration,
        Self::SolverCertificate,
        Self::TrustedAssumption,
        Self::ProofChecker,
        Self::RuntimeCheck,
        Self::AssertionOrder,
        Self::EffectIsolation,
        Self::Fault,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::ProfileOptimization,
        Self::OwnershipBorrow,
        Self::NodeTaskActor,
        Self::MemoryTiming,
        Self::EvidenceBundle,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::UnicodeFixture,
        Self::MigrationFixture,
        Self::DeterminismFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContractSyntaxCoreInventory {
    boundaries: Box<[PlannedContractSyntaxCoreBoundary]>,
}

impl ContractSyntaxCoreInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedContractSyntaxCoreBoundary>,
    ) -> Result<Self, PlannedContractSyntaxCoreBoundary> {
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
        let mut bytes = b"ling.contract-syntax-core-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_contract_syntax_core_boundaries_are_complete_and_ordered() {
    let inventory = ContractSyntaxCoreInventory::new(PlannedContractSyntaxCoreBoundary::ALL)
        .expect("planned Contract syntax/Core boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedContractSyntaxCoreBoundary::ALL
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
fn contract_syntax_core_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ContractSyntaxCoreInventory::new(PlannedContractSyntaxCoreBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ContractSyntaxCoreInventory::new(PlannedContractSyntaxCoreBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ContractSyntaxCoreInventory::new([
        PlannedContractSyntaxCoreBoundary::ContractSyntax,
        PlannedContractSyntaxCoreBoundary::ContractSyntax,
    ])
    .expect_err("duplicate Contract syntax/Core boundary must be rejected");
    assert_eq!(duplicate, PlannedContractSyntaxCoreBoundary::ContractSyntax);
}

#[test]
fn contract_syntax_core_evidence_has_no_contract_authority() {
    let inventory = ContractSyntaxCoreInventory::new([
        PlannedContractSyntaxCoreBoundary::ContractSyntax,
        PlannedContractSyntaxCoreBoundary::Requires,
        PlannedContractSyntaxCoreBoundary::StatusLifecycle,
        PlannedContractSyntaxCoreBoundary::Unknown,
        PlannedContractSyntaxCoreBoundary::DiagnosticCode,
        PlannedContractSyntaxCoreBoundary::ProtocolInventory,
    ])
    .expect("bounded Contract syntax/Core evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.contract-syntax-core-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
