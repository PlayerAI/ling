use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedContractStatusModelBoundary {
    ContractStatusModel,
    StatusVersion,
    Obligation,
    ObligationId,
    ContractId,
    DefinitionId,
    SemanticId,
    SourceSpan,
    CanonicalBytes,
    Proved,
    RuntimeChecked,
    ModelChecked,
    Tested,
    Assumed,
    Unknown,
    Failed,
    NotApplicable,
    EvidenceArtifact,
    EvidenceProvenance,
    TrustLevel,
    TrustedAssumption,
    ProofCertificate,
    SolverIdentity,
    CheckerIdentity,
    RuntimeCheckIdentity,
    TestIdentity,
    Bound,
    Timeout,
    Cancellation,
    Staleness,
    Corruption,
    Revocation,
    Transition,
    TerminalState,
    Composition,
    Precedence,
    Aggregation,
    Invalidation,
    Migration,
    Compatibility,
    AuditProjection,
    GraphProjection,
    EvidenceProjection,
    UiText,
    Accessibility,
    DiagnosticCode,
    DiagnosticFacts,
    ProfileOptimization,
    EffectIsolation,
    Fault,
    DeterministicOrdering,
    PositiveFixture,
    NegativeFixture,
    TransitionFixture,
    StaleFixture,
    CorruptionFixture,
    MigrationFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedContractStatusModelBoundary {
    const ALL: [Self; 60] = [
        Self::ContractStatusModel,
        Self::StatusVersion,
        Self::Obligation,
        Self::ObligationId,
        Self::ContractId,
        Self::DefinitionId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::CanonicalBytes,
        Self::Proved,
        Self::RuntimeChecked,
        Self::ModelChecked,
        Self::Tested,
        Self::Assumed,
        Self::Unknown,
        Self::Failed,
        Self::NotApplicable,
        Self::EvidenceArtifact,
        Self::EvidenceProvenance,
        Self::TrustLevel,
        Self::TrustedAssumption,
        Self::ProofCertificate,
        Self::SolverIdentity,
        Self::CheckerIdentity,
        Self::RuntimeCheckIdentity,
        Self::TestIdentity,
        Self::Bound,
        Self::Timeout,
        Self::Cancellation,
        Self::Staleness,
        Self::Corruption,
        Self::Revocation,
        Self::Transition,
        Self::TerminalState,
        Self::Composition,
        Self::Precedence,
        Self::Aggregation,
        Self::Invalidation,
        Self::Migration,
        Self::Compatibility,
        Self::AuditProjection,
        Self::GraphProjection,
        Self::EvidenceProjection,
        Self::UiText,
        Self::Accessibility,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::ProfileOptimization,
        Self::EffectIsolation,
        Self::Fault,
        Self::DeterministicOrdering,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::TransitionFixture,
        Self::StaleFixture,
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
struct ContractStatusModelInventory {
    boundaries: Box<[PlannedContractStatusModelBoundary]>,
}

impl ContractStatusModelInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedContractStatusModelBoundary>,
    ) -> Result<Self, PlannedContractStatusModelBoundary> {
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
        let mut bytes = b"ling.contract-status-model-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_contract_status_model_boundaries_are_complete_and_ordered() {
    let inventory = ContractStatusModelInventory::new(PlannedContractStatusModelBoundary::ALL)
        .expect("planned Contract status-model boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedContractStatusModelBoundary::ALL
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
fn contract_status_model_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ContractStatusModelInventory::new(PlannedContractStatusModelBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ContractStatusModelInventory::new(
        PlannedContractStatusModelBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ContractStatusModelInventory::new([
        PlannedContractStatusModelBoundary::ContractStatusModel,
        PlannedContractStatusModelBoundary::ContractStatusModel,
    ])
    .expect_err("duplicate Contract status-model boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedContractStatusModelBoundary::ContractStatusModel
    );
}

#[test]
fn contract_status_model_evidence_has_no_lifecycle_authority() {
    let inventory = ContractStatusModelInventory::new([
        PlannedContractStatusModelBoundary::ContractStatusModel,
        PlannedContractStatusModelBoundary::Proved,
        PlannedContractStatusModelBoundary::Unknown,
        PlannedContractStatusModelBoundary::EvidenceProvenance,
        PlannedContractStatusModelBoundary::DiagnosticCode,
        PlannedContractStatusModelBoundary::ProtocolInventory,
    ])
    .expect("bounded Contract status-model evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.contract-status-model-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
