use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedContractOptimizerBoundary {
    ContractOptimizer,
    CheckedContractProof,
    StatusTrust,
    Proved,
    RuntimeChecked,
    ModelChecked,
    Tested,
    Assumed,
    Unknown,
    Stale,
    Corrupt,
    Unverifiable,
    Admission,
    ProfileGate,
    CriticalNonWeakening,
    Transformation,
    ConstantFolding,
    CheckElimination,
    DeadCode,
    Inlining,
    EffectPreservation,
    CapabilityPreservation,
    EvaluationOrder,
    ShortCircuit,
    FaultVisibility,
    CleanupVisibility,
    Ownership,
    Resource,
    Timing,
    Node,
    Task,
    Actor,
    NumericOverflow,
    FfiAbi,
    StackMapping,
    DebugMapping,
    SemanticId,
    SourceSpan,
    Invalidation,
    DependencyChange,
    PassPrecondition,
    PassPostcondition,
    DeterministicOrder,
    FailClosed,
    ProofLink,
    AssumptionLink,
    EvidenceLink,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    RejectionFixture,
    StaleFixture,
    CorruptFixture,
    UnknownFixture,
    EffectFaultFixture,
    UnicodeFixture,
    DifferentialFixture,
    OptimizationFixture,
    ProtocolInventory,
}

impl PlannedContractOptimizerBoundary {
    const ALL: [Self; 60] = [
        Self::ContractOptimizer,
        Self::CheckedContractProof,
        Self::StatusTrust,
        Self::Proved,
        Self::RuntimeChecked,
        Self::ModelChecked,
        Self::Tested,
        Self::Assumed,
        Self::Unknown,
        Self::Stale,
        Self::Corrupt,
        Self::Unverifiable,
        Self::Admission,
        Self::ProfileGate,
        Self::CriticalNonWeakening,
        Self::Transformation,
        Self::ConstantFolding,
        Self::CheckElimination,
        Self::DeadCode,
        Self::Inlining,
        Self::EffectPreservation,
        Self::CapabilityPreservation,
        Self::EvaluationOrder,
        Self::ShortCircuit,
        Self::FaultVisibility,
        Self::CleanupVisibility,
        Self::Ownership,
        Self::Resource,
        Self::Timing,
        Self::Node,
        Self::Task,
        Self::Actor,
        Self::NumericOverflow,
        Self::FfiAbi,
        Self::StackMapping,
        Self::DebugMapping,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Invalidation,
        Self::DependencyChange,
        Self::PassPrecondition,
        Self::PassPostcondition,
        Self::DeterministicOrder,
        Self::FailClosed,
        Self::ProofLink,
        Self::AssumptionLink,
        Self::EvidenceLink,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::RejectionFixture,
        Self::StaleFixture,
        Self::CorruptFixture,
        Self::UnknownFixture,
        Self::EffectFaultFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::OptimizationFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContractOptimizerInventory {
    boundaries: Box<[PlannedContractOptimizerBoundary]>,
}

impl ContractOptimizerInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedContractOptimizerBoundary>,
    ) -> Result<Self, PlannedContractOptimizerBoundary> {
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
        let mut bytes = b"ling.contract-optimizer-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_contract_optimizer_boundaries_are_complete_and_ordered() {
    let inventory = ContractOptimizerInventory::new(PlannedContractOptimizerBoundary::ALL)
        .expect("planned Contract optimizer boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedContractOptimizerBoundary::ALL
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
fn contract_optimizer_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ContractOptimizerInventory::new(PlannedContractOptimizerBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ContractOptimizerInventory::new(PlannedContractOptimizerBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ContractOptimizerInventory::new([
        PlannedContractOptimizerBoundary::ContractOptimizer,
        PlannedContractOptimizerBoundary::ContractOptimizer,
    ])
    .expect_err("duplicate Contract optimizer boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedContractOptimizerBoundary::ContractOptimizer
    );
}

#[test]
fn contract_optimizer_evidence_has_no_pass_authority() {
    let inventory = ContractOptimizerInventory::new([
        PlannedContractOptimizerBoundary::ContractOptimizer,
        PlannedContractOptimizerBoundary::Proved,
        PlannedContractOptimizerBoundary::CheckElimination,
        PlannedContractOptimizerBoundary::FailClosed,
        PlannedContractOptimizerBoundary::DiagnosticCode,
        PlannedContractOptimizerBoundary::ProtocolInventory,
    ])
    .expect("bounded Contract optimizer evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.contract-optimizer-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
