use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedAssumptionRegistryBoundary {
    AssumptionRegistry,
    Version,
    AssumptionId,
    Description,
    Source,
    SourceDigest,
    Scope,
    Owner,
    Reviewer,
    Expiry,
    VersionConstraint,
    RiskClass,
    AffectedObligation,
    ContractId,
    ProofId,
    SemanticId,
    SourceSpan,
    Status,
    Proposed,
    Approved,
    Revoked,
    Expired,
    Stale,
    Missing,
    Duplicate,
    Conflict,
    OutOfScope,
    Unreviewed,
    Unverifiable,
    Malformed,
    Corrupt,
    UnknownField,
    Migration,
    Assumption,
    Hypothesis,
    Axiom,
    RuntimeCheck,
    TestEvidence,
    ModelCheckEvidence,
    SolverCandidate,
    ProvedFact,
    TcbEntry,
    ProofEffect,
    OptimizationGate,
    ProfileGate,
    FailClosed,
    Provenance,
    Checksum,
    Signature,
    Redaction,
    EvidenceBundle,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    ExpiredFixture,
    RevokedFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedAssumptionRegistryBoundary {
    const ALL: [Self; 60] = [
        Self::AssumptionRegistry,
        Self::Version,
        Self::AssumptionId,
        Self::Description,
        Self::Source,
        Self::SourceDigest,
        Self::Scope,
        Self::Owner,
        Self::Reviewer,
        Self::Expiry,
        Self::VersionConstraint,
        Self::RiskClass,
        Self::AffectedObligation,
        Self::ContractId,
        Self::ProofId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Status,
        Self::Proposed,
        Self::Approved,
        Self::Revoked,
        Self::Expired,
        Self::Stale,
        Self::Missing,
        Self::Duplicate,
        Self::Conflict,
        Self::OutOfScope,
        Self::Unreviewed,
        Self::Unverifiable,
        Self::Malformed,
        Self::Corrupt,
        Self::UnknownField,
        Self::Migration,
        Self::Assumption,
        Self::Hypothesis,
        Self::Axiom,
        Self::RuntimeCheck,
        Self::TestEvidence,
        Self::ModelCheckEvidence,
        Self::SolverCandidate,
        Self::ProvedFact,
        Self::TcbEntry,
        Self::ProofEffect,
        Self::OptimizationGate,
        Self::ProfileGate,
        Self::FailClosed,
        Self::Provenance,
        Self::Checksum,
        Self::Signature,
        Self::Redaction,
        Self::EvidenceBundle,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::ExpiredFixture,
        Self::RevokedFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AssumptionRegistryInventory {
    boundaries: Box<[PlannedAssumptionRegistryBoundary]>,
}

impl AssumptionRegistryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedAssumptionRegistryBoundary>,
    ) -> Result<Self, PlannedAssumptionRegistryBoundary> {
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
        let mut bytes = b"ling.assumption-registry-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_assumption_registry_boundaries_are_complete_and_ordered() {
    let inventory = AssumptionRegistryInventory::new(PlannedAssumptionRegistryBoundary::ALL)
        .expect("planned assumption-registry boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedAssumptionRegistryBoundary::ALL
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
fn assumption_registry_evidence_is_order_independent_and_duplicate_checked() {
    let forward = AssumptionRegistryInventory::new(PlannedAssumptionRegistryBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        AssumptionRegistryInventory::new(PlannedAssumptionRegistryBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = AssumptionRegistryInventory::new([
        PlannedAssumptionRegistryBoundary::AssumptionRegistry,
        PlannedAssumptionRegistryBoundary::AssumptionRegistry,
    ])
    .expect_err("duplicate assumption-registry boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedAssumptionRegistryBoundary::AssumptionRegistry
    );
}

#[test]
fn assumption_registry_evidence_has_no_registry_authority() {
    let inventory = AssumptionRegistryInventory::new([
        PlannedAssumptionRegistryBoundary::AssumptionRegistry,
        PlannedAssumptionRegistryBoundary::AssumptionId,
        PlannedAssumptionRegistryBoundary::Reviewer,
        PlannedAssumptionRegistryBoundary::ProofEffect,
        PlannedAssumptionRegistryBoundary::EvidenceBundle,
        PlannedAssumptionRegistryBoundary::ProtocolInventory,
    ])
    .expect("bounded assumption-registry evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.assumption-registry-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
