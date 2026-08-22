use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedExplorationEngineBoundary {
    ExplorationEngine,
    Version,
    ProjectedModel,
    StateCanonicalBytes,
    StateHash,
    HashVersion,
    HashCollision,
    TransitionOrder,
    EventOrder,
    Bfs,
    Dfs,
    WorkQueue,
    VisitedSet,
    Deduplication,
    TieBreak,
    DeterministicSearch,
    PartialOrderReduction,
    IndependenceRelation,
    ReductionSoundness,
    ReductionDisabled,
    DepthBound,
    StepBound,
    StateBound,
    QueueBound,
    TimeBound,
    MemoryBound,
    Cancellation,
    Timeout,
    ResourceExhaustion,
    Incomplete,
    Unknown,
    InvalidModel,
    Malformed,
    Corrupt,
    ResultStatus,
    CounterexampleTrace,
    ReplayLink,
    SourceSpan,
    SemanticId,
    Provenance,
    Checksum,
    Redaction,
    BoundedNonProof,
    Assumption,
    ProfileAdmission,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    InterleavingFixture,
    ReductionFixture,
    HashCollisionFixture,
    BoundEdgeFixture,
    TimeoutFixture,
    MemoryFixture,
    DeterminismFixture,
    ReplayFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedExplorationEngineBoundary {
    const ALL: [Self; 60] = [
        Self::ExplorationEngine,
        Self::Version,
        Self::ProjectedModel,
        Self::StateCanonicalBytes,
        Self::StateHash,
        Self::HashVersion,
        Self::HashCollision,
        Self::TransitionOrder,
        Self::EventOrder,
        Self::Bfs,
        Self::Dfs,
        Self::WorkQueue,
        Self::VisitedSet,
        Self::Deduplication,
        Self::TieBreak,
        Self::DeterministicSearch,
        Self::PartialOrderReduction,
        Self::IndependenceRelation,
        Self::ReductionSoundness,
        Self::ReductionDisabled,
        Self::DepthBound,
        Self::StepBound,
        Self::StateBound,
        Self::QueueBound,
        Self::TimeBound,
        Self::MemoryBound,
        Self::Cancellation,
        Self::Timeout,
        Self::ResourceExhaustion,
        Self::Incomplete,
        Self::Unknown,
        Self::InvalidModel,
        Self::Malformed,
        Self::Corrupt,
        Self::ResultStatus,
        Self::CounterexampleTrace,
        Self::ReplayLink,
        Self::SourceSpan,
        Self::SemanticId,
        Self::Provenance,
        Self::Checksum,
        Self::Redaction,
        Self::BoundedNonProof,
        Self::Assumption,
        Self::ProfileAdmission,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::InterleavingFixture,
        Self::ReductionFixture,
        Self::HashCollisionFixture,
        Self::BoundEdgeFixture,
        Self::TimeoutFixture,
        Self::MemoryFixture,
        Self::DeterminismFixture,
        Self::ReplayFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExplorationEngineInventory {
    boundaries: Box<[PlannedExplorationEngineBoundary]>,
}

impl ExplorationEngineInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedExplorationEngineBoundary>,
    ) -> Result<Self, PlannedExplorationEngineBoundary> {
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
        let mut bytes = b"ling.exploration-engine-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_exploration_engine_boundaries_are_complete_and_ordered() {
    let inventory = ExplorationEngineInventory::new(PlannedExplorationEngineBoundary::ALL)
        .expect("planned exploration-engine boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedExplorationEngineBoundary::ALL
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
fn exploration_engine_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ExplorationEngineInventory::new(PlannedExplorationEngineBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ExplorationEngineInventory::new(PlannedExplorationEngineBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ExplorationEngineInventory::new([
        PlannedExplorationEngineBoundary::ExplorationEngine,
        PlannedExplorationEngineBoundary::ExplorationEngine,
    ])
    .expect_err("duplicate exploration-engine boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedExplorationEngineBoundary::ExplorationEngine
    );
}

#[test]
fn exploration_engine_evidence_has_no_engine_authority() {
    let inventory = ExplorationEngineInventory::new([
        PlannedExplorationEngineBoundary::ExplorationEngine,
        PlannedExplorationEngineBoundary::ProjectedModel,
        PlannedExplorationEngineBoundary::StateHash,
        PlannedExplorationEngineBoundary::PartialOrderReduction,
        PlannedExplorationEngineBoundary::BoundedNonProof,
        PlannedExplorationEngineBoundary::ProtocolInventory,
    ])
    .expect("bounded exploration-engine evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.exploration-engine-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
