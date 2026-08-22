//! Internal Managed collector boundary evidence.
//!
//! This test-only inventory names the contracts a future collector must
//! satisfy. It does not implement a heap, collector, roots, pauses, safe
//! points, scheduler hooks, allocation limits, metrics, or runtime semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedCollectorBoundary {
    CollectorChoice,
    CollectorStrategyOpacity,
    HeapBoundary,
    RootRegistration,
    RootDeregistration,
    RootLifetime,
    StackRoot,
    ClosureRoot,
    GlobalRoot,
    TaskRoot,
    ActorRoot,
    CallbackRoot,
    NativeIslandRoot,
    SuspensionRoot,
    Reachability,
    CycleCollection,
    WriteBarrier,
    MutationOrdering,
    SafePoint,
    PauseBehavior,
    ProgressFairness,
    ThreadAttachment,
    TaskInteraction,
    ActorInteraction,
    Cancellation,
    Restart,
    Shutdown,
    MemoryLimit,
    AllocationFailure,
    RetryRecovery,
    OomFault,
    Metrics,
    MetricsSchema,
    DeterministicMetrics,
    StressEvidence,
    PropertyEvidence,
    FuzzEvidence,
    DeterministicSeedsAndBounds,
    InterpreterVmNativeDifferential,
    UnicodeSourceSpans,
    AddressTimingOpacity,
    ProfileConstraint,
    ResourceDropBoundary,
}

impl PlannedCollectorBoundary {
    const ALL: [Self; 43] = [
        Self::CollectorChoice,
        Self::CollectorStrategyOpacity,
        Self::HeapBoundary,
        Self::RootRegistration,
        Self::RootDeregistration,
        Self::RootLifetime,
        Self::StackRoot,
        Self::ClosureRoot,
        Self::GlobalRoot,
        Self::TaskRoot,
        Self::ActorRoot,
        Self::CallbackRoot,
        Self::NativeIslandRoot,
        Self::SuspensionRoot,
        Self::Reachability,
        Self::CycleCollection,
        Self::WriteBarrier,
        Self::MutationOrdering,
        Self::SafePoint,
        Self::PauseBehavior,
        Self::ProgressFairness,
        Self::ThreadAttachment,
        Self::TaskInteraction,
        Self::ActorInteraction,
        Self::Cancellation,
        Self::Restart,
        Self::Shutdown,
        Self::MemoryLimit,
        Self::AllocationFailure,
        Self::RetryRecovery,
        Self::OomFault,
        Self::Metrics,
        Self::MetricsSchema,
        Self::DeterministicMetrics,
        Self::StressEvidence,
        Self::PropertyEvidence,
        Self::FuzzEvidence,
        Self::DeterministicSeedsAndBounds,
        Self::InterpreterVmNativeDifferential,
        Self::UnicodeSourceSpans,
        Self::AddressTimingOpacity,
        Self::ProfileConstraint,
        Self::ResourceDropBoundary,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::CollectorChoice => 0,
            Self::CollectorStrategyOpacity => 1,
            Self::HeapBoundary => 2,
            Self::RootRegistration => 3,
            Self::RootDeregistration => 4,
            Self::RootLifetime => 5,
            Self::StackRoot => 6,
            Self::ClosureRoot => 7,
            Self::GlobalRoot => 8,
            Self::TaskRoot => 9,
            Self::ActorRoot => 10,
            Self::CallbackRoot => 11,
            Self::NativeIslandRoot => 12,
            Self::SuspensionRoot => 13,
            Self::Reachability => 14,
            Self::CycleCollection => 15,
            Self::WriteBarrier => 16,
            Self::MutationOrdering => 17,
            Self::SafePoint => 18,
            Self::PauseBehavior => 19,
            Self::ProgressFairness => 20,
            Self::ThreadAttachment => 21,
            Self::TaskInteraction => 22,
            Self::ActorInteraction => 23,
            Self::Cancellation => 24,
            Self::Restart => 25,
            Self::Shutdown => 26,
            Self::MemoryLimit => 27,
            Self::AllocationFailure => 28,
            Self::RetryRecovery => 29,
            Self::OomFault => 30,
            Self::Metrics => 31,
            Self::MetricsSchema => 32,
            Self::DeterministicMetrics => 33,
            Self::StressEvidence => 34,
            Self::PropertyEvidence => 35,
            Self::FuzzEvidence => 36,
            Self::DeterministicSeedsAndBounds => 37,
            Self::InterpreterVmNativeDifferential => 38,
            Self::UnicodeSourceSpans => 39,
            Self::AddressTimingOpacity => 40,
            Self::ProfileConstraint => 41,
            Self::ResourceDropBoundary => 42,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CollectorBoundaryInventory {
    boundaries: Box<[PlannedCollectorBoundary]>,
}

impl CollectorBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCollectorBoundary>,
    ) -> Result<Self, PlannedCollectorBoundary> {
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
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ling.managed-collector-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_collector_boundaries_are_complete_and_ordered() {
    let inventory = CollectorBoundaryInventory::new(PlannedCollectorBoundary::ALL)
        .expect("planned collector boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedCollectorBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..43).collect::<Vec<_>>()
    );
}

#[test]
fn collector_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CollectorBoundaryInventory::new(PlannedCollectorBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = CollectorBoundaryInventory::new(PlannedCollectorBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CollectorBoundaryInventory::new([
        PlannedCollectorBoundary::RootRegistration,
        PlannedCollectorBoundary::RootRegistration,
    ])
    .expect_err("duplicate collector boundary must be rejected");
    assert_eq!(duplicate, PlannedCollectorBoundary::RootRegistration);
}

#[test]
fn collector_evidence_has_no_heap_or_scheduler_authority() {
    let inventory = CollectorBoundaryInventory::new([
        PlannedCollectorBoundary::CollectorChoice,
        PlannedCollectorBoundary::SafePoint,
        PlannedCollectorBoundary::TaskInteraction,
        PlannedCollectorBoundary::MemoryLimit,
        PlannedCollectorBoundary::Metrics,
        PlannedCollectorBoundary::FuzzEvidence,
    ])
    .expect("bounded collector evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.managed-collector-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
