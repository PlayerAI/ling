use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedFiniteStateProjectionBoundary {
    FiniteStateProjection,
    Version,
    CheckedCore,
    Task,
    Actor,
    Node,
    StateVariable,
    StateType,
    StateValue,
    Mailbox,
    Queue,
    MailboxBound,
    QueueBound,
    Transition,
    SchedulerChoice,
    Fairness,
    Fault,
    Restart,
    RestartIdentity,
    TimeAbstraction,
    ExternalInput,
    Ownership,
    Property,
    PropertyLanguage,
    ExplicitBound,
    DepthBound,
    StateBound,
    TimeBound,
    MemoryBound,
    CanonicalState,
    StateId,
    SemanticId,
    SourceSpan,
    StateHash,
    ProjectionRelation,
    SoundnessNonClaim,
    ModelCheckedStatus,
    BoundedEvidence,
    Assumption,
    ProfileAdmission,
    DeterministicOrdering,
    ResourceLimit,
    Timeout,
    MemoryExhaustion,
    Unknown,
    Incomplete,
    Malformed,
    Corrupt,
    Migration,
    CounterexampleLink,
    ReplayLink,
    Provenance,
    Checksum,
    Redaction,
    DiagnosticCode,
    DiagnosticFacts,
    InterleavingFixture,
    FaultRestartFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedFiniteStateProjectionBoundary {
    const ALL: [Self; 60] = [
        Self::FiniteStateProjection,
        Self::Version,
        Self::CheckedCore,
        Self::Task,
        Self::Actor,
        Self::Node,
        Self::StateVariable,
        Self::StateType,
        Self::StateValue,
        Self::Mailbox,
        Self::Queue,
        Self::MailboxBound,
        Self::QueueBound,
        Self::Transition,
        Self::SchedulerChoice,
        Self::Fairness,
        Self::Fault,
        Self::Restart,
        Self::RestartIdentity,
        Self::TimeAbstraction,
        Self::ExternalInput,
        Self::Ownership,
        Self::Property,
        Self::PropertyLanguage,
        Self::ExplicitBound,
        Self::DepthBound,
        Self::StateBound,
        Self::TimeBound,
        Self::MemoryBound,
        Self::CanonicalState,
        Self::StateId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::StateHash,
        Self::ProjectionRelation,
        Self::SoundnessNonClaim,
        Self::ModelCheckedStatus,
        Self::BoundedEvidence,
        Self::Assumption,
        Self::ProfileAdmission,
        Self::DeterministicOrdering,
        Self::ResourceLimit,
        Self::Timeout,
        Self::MemoryExhaustion,
        Self::Unknown,
        Self::Incomplete,
        Self::Malformed,
        Self::Corrupt,
        Self::Migration,
        Self::CounterexampleLink,
        Self::ReplayLink,
        Self::Provenance,
        Self::Checksum,
        Self::Redaction,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::InterleavingFixture,
        Self::FaultRestartFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FiniteStateProjectionInventory {
    boundaries: Box<[PlannedFiniteStateProjectionBoundary]>,
}

impl FiniteStateProjectionInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedFiniteStateProjectionBoundary>,
    ) -> Result<Self, PlannedFiniteStateProjectionBoundary> {
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
        let mut bytes = b"ling.finite-state-projection-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_finite_state_projection_boundaries_are_complete_and_ordered() {
    let inventory = FiniteStateProjectionInventory::new(PlannedFiniteStateProjectionBoundary::ALL)
        .expect("planned finite-state projection boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedFiniteStateProjectionBoundary::ALL
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
fn finite_state_projection_evidence_is_order_independent_and_duplicate_checked() {
    let forward = FiniteStateProjectionInventory::new(PlannedFiniteStateProjectionBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = FiniteStateProjectionInventory::new(
        PlannedFiniteStateProjectionBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = FiniteStateProjectionInventory::new([
        PlannedFiniteStateProjectionBoundary::FiniteStateProjection,
        PlannedFiniteStateProjectionBoundary::FiniteStateProjection,
    ])
    .expect_err("duplicate finite-state projection boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedFiniteStateProjectionBoundary::FiniteStateProjection
    );
}

#[test]
fn finite_state_projection_evidence_has_no_projection_authority() {
    let inventory = FiniteStateProjectionInventory::new([
        PlannedFiniteStateProjectionBoundary::FiniteStateProjection,
        PlannedFiniteStateProjectionBoundary::CheckedCore,
        PlannedFiniteStateProjectionBoundary::Transition,
        PlannedFiniteStateProjectionBoundary::ExplicitBound,
        PlannedFiniteStateProjectionBoundary::SoundnessNonClaim,
        PlannedFiniteStateProjectionBoundary::ProtocolInventory,
    ])
    .expect("bounded finite-state projection evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.finite-state-projection-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
