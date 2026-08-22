use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedContractRuntimeCheckBoundary {
    RuntimeContractCheck,
    CheckedContractCore,
    Obligation,
    ContractId,
    SemanticId,
    SourceSpan,
    CanonicalBytes,
    Precondition,
    Postcondition,
    Invariant,
    InstanceValue,
    CallBoundary,
    ReturnBoundary,
    DeclaredBoundary,
    SuspensionBoundary,
    NodeActorBoundary,
    FfiBoundary,
    CleanupBoundary,
    EvaluationOrder,
    ShortCircuit,
    Pure,
    Total,
    EffectCapability,
    AllocationBound,
    TerminationBound,
    AssumeRestriction,
    CheckedInput,
    UnknownClaim,
    MalformedClaim,
    DeterministicOrder,
    EffectIsolation,
    Atomicity,
    CommittedState,
    FaultCategory,
    FaultCode,
    FaultFacts,
    ContractViolation,
    StatusProjection,
    ObligationProvenance,
    CapturedValue,
    PrivacyLimit,
    SizeLimit,
    ProfileGate,
    CriticalNonWeakening,
    ReferenceRuntime,
    VmDifferential,
    NativeDifferential,
    RuntimeEvidence,
    Replay,
    Migration,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    BoundaryFixture,
    IsolationFixture,
    ProfileFixture,
    ReplayFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedContractRuntimeCheckBoundary {
    const ALL: [Self; 60] = [
        Self::RuntimeContractCheck,
        Self::CheckedContractCore,
        Self::Obligation,
        Self::ContractId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::CanonicalBytes,
        Self::Precondition,
        Self::Postcondition,
        Self::Invariant,
        Self::InstanceValue,
        Self::CallBoundary,
        Self::ReturnBoundary,
        Self::DeclaredBoundary,
        Self::SuspensionBoundary,
        Self::NodeActorBoundary,
        Self::FfiBoundary,
        Self::CleanupBoundary,
        Self::EvaluationOrder,
        Self::ShortCircuit,
        Self::Pure,
        Self::Total,
        Self::EffectCapability,
        Self::AllocationBound,
        Self::TerminationBound,
        Self::AssumeRestriction,
        Self::CheckedInput,
        Self::UnknownClaim,
        Self::MalformedClaim,
        Self::DeterministicOrder,
        Self::EffectIsolation,
        Self::Atomicity,
        Self::CommittedState,
        Self::FaultCategory,
        Self::FaultCode,
        Self::FaultFacts,
        Self::ContractViolation,
        Self::StatusProjection,
        Self::ObligationProvenance,
        Self::CapturedValue,
        Self::PrivacyLimit,
        Self::SizeLimit,
        Self::ProfileGate,
        Self::CriticalNonWeakening,
        Self::ReferenceRuntime,
        Self::VmDifferential,
        Self::NativeDifferential,
        Self::RuntimeEvidence,
        Self::Replay,
        Self::Migration,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::BoundaryFixture,
        Self::IsolationFixture,
        Self::ProfileFixture,
        Self::ReplayFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContractRuntimeCheckInventory {
    boundaries: Box<[PlannedContractRuntimeCheckBoundary]>,
}

impl ContractRuntimeCheckInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedContractRuntimeCheckBoundary>,
    ) -> Result<Self, PlannedContractRuntimeCheckBoundary> {
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
        let mut bytes = b"ling.contract-runtime-check-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_contract_runtime_check_boundaries_are_complete_and_ordered() {
    let inventory = ContractRuntimeCheckInventory::new(PlannedContractRuntimeCheckBoundary::ALL)
        .expect("planned Contract runtime-check boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedContractRuntimeCheckBoundary::ALL
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
fn contract_runtime_check_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ContractRuntimeCheckInventory::new(PlannedContractRuntimeCheckBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ContractRuntimeCheckInventory::new(
        PlannedContractRuntimeCheckBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ContractRuntimeCheckInventory::new([
        PlannedContractRuntimeCheckBoundary::RuntimeContractCheck,
        PlannedContractRuntimeCheckBoundary::RuntimeContractCheck,
    ])
    .expect_err("duplicate Contract runtime-check boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedContractRuntimeCheckBoundary::RuntimeContractCheck
    );
}

#[test]
fn contract_runtime_check_evidence_has_no_evaluator_authority() {
    let inventory = ContractRuntimeCheckInventory::new([
        PlannedContractRuntimeCheckBoundary::RuntimeContractCheck,
        PlannedContractRuntimeCheckBoundary::Precondition,
        PlannedContractRuntimeCheckBoundary::FaultCategory,
        PlannedContractRuntimeCheckBoundary::ProfileGate,
        PlannedContractRuntimeCheckBoundary::DiagnosticCode,
        PlannedContractRuntimeCheckBoundary::ProtocolInventory,
    ])
    .expect("bounded Contract runtime-check evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.contract-runtime-check-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
