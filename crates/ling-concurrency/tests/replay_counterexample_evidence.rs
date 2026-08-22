use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedReplayCounterexampleBoundary {
    ReplayCounterexample,
    Version,
    CheckedModel,
    ExplorationResult,
    Counterexample,
    ConversionBoundary,
    ReplayFixture,
    ReplaySchema,
    ReplayReader,
    ReplayWriter,
    ReferenceRuntime,
    SchedulerIdentity,
    SchedulerPolicy,
    LogicalClock,
    InputEvent,
    HostEffect,
    StateSnapshot,
    StateChecksum,
    FaultEvent,
    RestartEvent,
    EventOrder,
    MailboxOrder,
    Backpressure,
    Reentry,
    DropPolicy,
    Expiry,
    Cancellation,
    Ownership,
    Capability,
    ResourceBound,
    ModelId,
    RuntimeId,
    CounterexampleId,
    ReplayId,
    SemanticId,
    SourceSpan,
    CheckedSnapshotId,
    Provenance,
    Checksum,
    Signature,
    Redaction,
    Privacy,
    Divergence,
    Malformed,
    Corrupt,
    UnknownField,
    UnsupportedVersion,
    Migration,
    UnavailableInput,
    UnsupportedFault,
    FailClosed,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    DivergenceFixture,
    SourceLinkFixture,
    UnicodeFixture,
    DeterminismFixture,
    ProtocolInventory,
}

impl PlannedReplayCounterexampleBoundary {
    const ALL: [Self; 60] = [
        Self::ReplayCounterexample,
        Self::Version,
        Self::CheckedModel,
        Self::ExplorationResult,
        Self::Counterexample,
        Self::ConversionBoundary,
        Self::ReplayFixture,
        Self::ReplaySchema,
        Self::ReplayReader,
        Self::ReplayWriter,
        Self::ReferenceRuntime,
        Self::SchedulerIdentity,
        Self::SchedulerPolicy,
        Self::LogicalClock,
        Self::InputEvent,
        Self::HostEffect,
        Self::StateSnapshot,
        Self::StateChecksum,
        Self::FaultEvent,
        Self::RestartEvent,
        Self::EventOrder,
        Self::MailboxOrder,
        Self::Backpressure,
        Self::Reentry,
        Self::DropPolicy,
        Self::Expiry,
        Self::Cancellation,
        Self::Ownership,
        Self::Capability,
        Self::ResourceBound,
        Self::ModelId,
        Self::RuntimeId,
        Self::CounterexampleId,
        Self::ReplayId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::CheckedSnapshotId,
        Self::Provenance,
        Self::Checksum,
        Self::Signature,
        Self::Redaction,
        Self::Privacy,
        Self::Divergence,
        Self::Malformed,
        Self::Corrupt,
        Self::UnknownField,
        Self::UnsupportedVersion,
        Self::Migration,
        Self::UnavailableInput,
        Self::UnsupportedFault,
        Self::FailClosed,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::DivergenceFixture,
        Self::SourceLinkFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReplayCounterexampleInventory {
    boundaries: Box<[PlannedReplayCounterexampleBoundary]>,
}

impl ReplayCounterexampleInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedReplayCounterexampleBoundary>,
    ) -> Result<Self, PlannedReplayCounterexampleBoundary> {
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
        let mut bytes = b"ling.replay-counterexample-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_replay_counterexample_boundaries_are_complete_and_ordered() {
    let inventory = ReplayCounterexampleInventory::new(PlannedReplayCounterexampleBoundary::ALL)
        .expect("planned replay-counterexample boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedReplayCounterexampleBoundary::ALL
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
fn replay_counterexample_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ReplayCounterexampleInventory::new(PlannedReplayCounterexampleBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ReplayCounterexampleInventory::new(
        PlannedReplayCounterexampleBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ReplayCounterexampleInventory::new([
        PlannedReplayCounterexampleBoundary::ReplayCounterexample,
        PlannedReplayCounterexampleBoundary::ReplayCounterexample,
    ])
    .expect_err("duplicate replay-counterexample boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedReplayCounterexampleBoundary::ReplayCounterexample
    );
}

#[test]
fn replay_counterexample_evidence_has_no_replay_authority() {
    let inventory = ReplayCounterexampleInventory::new([
        PlannedReplayCounterexampleBoundary::ReplayCounterexample,
        PlannedReplayCounterexampleBoundary::ConversionBoundary,
        PlannedReplayCounterexampleBoundary::ReferenceRuntime,
        PlannedReplayCounterexampleBoundary::SchedulerIdentity,
        PlannedReplayCounterexampleBoundary::SourceSpan,
        PlannedReplayCounterexampleBoundary::ProtocolInventory,
    ])
    .expect("bounded replay-counterexample evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.replay-counterexample-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
