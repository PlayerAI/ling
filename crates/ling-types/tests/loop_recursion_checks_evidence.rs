use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedLoopRecursionChecksBoundary {
    LoopRecursionChecks,
    StaticallyBounded,
    ProvedTerminating,
    RuntimeGuarded,
    Forbidden,
    Unknown,
    ProfileStatePolicy,
    LoopSyntax,
    DirectRecursion,
    MutualRecursion,
    HigherOrderRecursion,
    DataSizeRelation,
    RankingFunction,
    SizeChange,
    Assumption,
    ProofObligation,
    CheckerTrust,
    EffectRelation,
    FaultRelation,
    ConcurrencyRelation,
    TaskRelation,
    ActorRelation,
    MailboxRelation,
    Backpressure,
    Cancellation,
    DeviceRelation,
    NativeRelation,
    NumericDeterminism,
    StackResource,
    HeapResource,
    ArenaResource,
    FrameLimitNonProof,
    RuntimeGuard,
    GuardFailure,
    LimitExhaustion,
    UnsupportedCase,
    WorkQueueAction,
    Eligibility,
    StateEquivalence,
    OwnershipEquivalence,
    EffectEquivalence,
    OrderingEquivalence,
    AllocationEquivalence,
    CancellationEquivalence,
    FaultEquivalence,
    SourceMapPreservation,
    UserConsent,
    Rollback,
    DiagnosticCode,
    DiagnosticFacts,
    SemanticId,
    SourceSpan,
    PositiveFixture,
    NegativeFixture,
    CounterexampleFixture,
    TransformationFixture,
    UnicodeFixture,
    DeterminismFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedLoopRecursionChecksBoundary {
    const ALL: [Self; 60] = [
        Self::LoopRecursionChecks,
        Self::StaticallyBounded,
        Self::ProvedTerminating,
        Self::RuntimeGuarded,
        Self::Forbidden,
        Self::Unknown,
        Self::ProfileStatePolicy,
        Self::LoopSyntax,
        Self::DirectRecursion,
        Self::MutualRecursion,
        Self::HigherOrderRecursion,
        Self::DataSizeRelation,
        Self::RankingFunction,
        Self::SizeChange,
        Self::Assumption,
        Self::ProofObligation,
        Self::CheckerTrust,
        Self::EffectRelation,
        Self::FaultRelation,
        Self::ConcurrencyRelation,
        Self::TaskRelation,
        Self::ActorRelation,
        Self::MailboxRelation,
        Self::Backpressure,
        Self::Cancellation,
        Self::DeviceRelation,
        Self::NativeRelation,
        Self::NumericDeterminism,
        Self::StackResource,
        Self::HeapResource,
        Self::ArenaResource,
        Self::FrameLimitNonProof,
        Self::RuntimeGuard,
        Self::GuardFailure,
        Self::LimitExhaustion,
        Self::UnsupportedCase,
        Self::WorkQueueAction,
        Self::Eligibility,
        Self::StateEquivalence,
        Self::OwnershipEquivalence,
        Self::EffectEquivalence,
        Self::OrderingEquivalence,
        Self::AllocationEquivalence,
        Self::CancellationEquivalence,
        Self::FaultEquivalence,
        Self::SourceMapPreservation,
        Self::UserConsent,
        Self::Rollback,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::SemanticId,
        Self::SourceSpan,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CounterexampleFixture,
        Self::TransformationFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LoopRecursionChecksInventory {
    boundaries: Box<[PlannedLoopRecursionChecksBoundary]>,
}

impl LoopRecursionChecksInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedLoopRecursionChecksBoundary>,
    ) -> Result<Self, PlannedLoopRecursionChecksBoundary> {
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
        let mut bytes = b"ling.loop-recursion-checks-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_loop_recursion_checks_boundaries_are_complete_and_ordered() {
    let inventory = LoopRecursionChecksInventory::new(PlannedLoopRecursionChecksBoundary::ALL)
        .expect("planned loop/recursion boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedLoopRecursionChecksBoundary::ALL
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
fn loop_recursion_checks_evidence_is_order_independent_and_duplicate_checked() {
    let forward = LoopRecursionChecksInventory::new(PlannedLoopRecursionChecksBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = LoopRecursionChecksInventory::new(
        PlannedLoopRecursionChecksBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = LoopRecursionChecksInventory::new([
        PlannedLoopRecursionChecksBoundary::LoopRecursionChecks,
        PlannedLoopRecursionChecksBoundary::LoopRecursionChecks,
    ])
    .expect_err("duplicate loop/recursion boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedLoopRecursionChecksBoundary::LoopRecursionChecks
    );
}

#[test]
fn loop_recursion_checks_evidence_has_no_termination_authority() {
    let inventory = LoopRecursionChecksInventory::new([
        PlannedLoopRecursionChecksBoundary::LoopRecursionChecks,
        PlannedLoopRecursionChecksBoundary::StaticallyBounded,
        PlannedLoopRecursionChecksBoundary::ProvedTerminating,
        PlannedLoopRecursionChecksBoundary::RuntimeGuarded,
        PlannedLoopRecursionChecksBoundary::Forbidden,
        PlannedLoopRecursionChecksBoundary::WorkQueueAction,
        PlannedLoopRecursionChecksBoundary::DiagnosticCode,
        PlannedLoopRecursionChecksBoundary::ProtocolInventory,
    ])
    .expect("bounded loop/recursion evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.loop-recursion-checks-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
