use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedModelCheckReportBoundary {
    ModelCheckReport,
    Version,
    CounterexampleFound,
    NoCounterexampleWithinBounds,
    Inconclusive,
    InvalidModel,
    InvalidProperty,
    Timeout,
    MemoryExhaustion,
    ResourceExhaustion,
    Unknown,
    Malformed,
    Corrupt,
    UnsupportedVersion,
    FailClosed,
    BoundedNonProof,
    SafetyClaimProhibited,
    ModelId,
    PropertyId,
    BoundId,
    AssumptionId,
    CounterexampleId,
    ReplayId,
    SemanticId,
    SourceSpan,
    SchedulerConfig,
    TimeConfig,
    ExploredStateCount,
    ExploredTransitionCount,
    ResourceLimit,
    ExhaustionReason,
    ToolIdentity,
    ToolVersion,
    Provenance,
    Checksum,
    Signature,
    Redaction,
    UnknownField,
    Migration,
    CanonicalBytes,
    DeterministicOrdering,
    CounterexamplePayload,
    ReplayLink,
    ProofDistinction,
    EvidenceLink,
    IndependentVerification,
    DiagnosticCode,
    DiagnosticFacts,
    ExitCode,
    PositiveFixture,
    NegativeFixture,
    CounterexampleFixture,
    BoundedAbsenceFixture,
    InconclusiveFixture,
    InvalidModelFixture,
    InvalidPropertyFixture,
    MalformedFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedModelCheckReportBoundary {
    const ALL: [Self; 60] = [
        Self::ModelCheckReport,
        Self::Version,
        Self::CounterexampleFound,
        Self::NoCounterexampleWithinBounds,
        Self::Inconclusive,
        Self::InvalidModel,
        Self::InvalidProperty,
        Self::Timeout,
        Self::MemoryExhaustion,
        Self::ResourceExhaustion,
        Self::Unknown,
        Self::Malformed,
        Self::Corrupt,
        Self::UnsupportedVersion,
        Self::FailClosed,
        Self::BoundedNonProof,
        Self::SafetyClaimProhibited,
        Self::ModelId,
        Self::PropertyId,
        Self::BoundId,
        Self::AssumptionId,
        Self::CounterexampleId,
        Self::ReplayId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::SchedulerConfig,
        Self::TimeConfig,
        Self::ExploredStateCount,
        Self::ExploredTransitionCount,
        Self::ResourceLimit,
        Self::ExhaustionReason,
        Self::ToolIdentity,
        Self::ToolVersion,
        Self::Provenance,
        Self::Checksum,
        Self::Signature,
        Self::Redaction,
        Self::UnknownField,
        Self::Migration,
        Self::CanonicalBytes,
        Self::DeterministicOrdering,
        Self::CounterexamplePayload,
        Self::ReplayLink,
        Self::ProofDistinction,
        Self::EvidenceLink,
        Self::IndependentVerification,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::ExitCode,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CounterexampleFixture,
        Self::BoundedAbsenceFixture,
        Self::InconclusiveFixture,
        Self::InvalidModelFixture,
        Self::InvalidPropertyFixture,
        Self::MalformedFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ModelCheckReportInventory {
    boundaries: Box<[PlannedModelCheckReportBoundary]>,
}

impl ModelCheckReportInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedModelCheckReportBoundary>,
    ) -> Result<Self, PlannedModelCheckReportBoundary> {
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
        let mut bytes = b"ling.model-check-report-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_model_check_report_boundaries_are_complete_and_ordered() {
    let inventory = ModelCheckReportInventory::new(PlannedModelCheckReportBoundary::ALL)
        .expect("planned model-check report boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedModelCheckReportBoundary::ALL
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
fn model_check_report_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ModelCheckReportInventory::new(PlannedModelCheckReportBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ModelCheckReportInventory::new(PlannedModelCheckReportBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ModelCheckReportInventory::new([
        PlannedModelCheckReportBoundary::ModelCheckReport,
        PlannedModelCheckReportBoundary::ModelCheckReport,
    ])
    .expect_err("duplicate model-check report boundary must be rejected");
    assert_eq!(duplicate, PlannedModelCheckReportBoundary::ModelCheckReport);
}

#[test]
fn bounded_absence_is_recorded_as_non_proof_evidence_only() {
    let inventory = ModelCheckReportInventory::new([
        PlannedModelCheckReportBoundary::NoCounterexampleWithinBounds,
        PlannedModelCheckReportBoundary::BoundedNonProof,
        PlannedModelCheckReportBoundary::SafetyClaimProhibited,
        PlannedModelCheckReportBoundary::BoundId,
        PlannedModelCheckReportBoundary::ProofDistinction,
        PlannedModelCheckReportBoundary::ProtocolInventory,
    ])
    .expect("bounded report evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.model-check-report-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
