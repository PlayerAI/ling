use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedMemoryBudgetsBoundary {
    MemoryBudgets,
    StaticData,
    Stack,
    Arena,
    Buffer,
    QueueMailbox,
    TaskActorState,
    DeviceMemory,
    TransientPeak,
    ErrorFallbackPath,
    Allocation,
    Ownership,
    Region,
    Layout,
    Units,
    Alignment,
    Lifetime,
    DropTiming,
    Aliasing,
    Sharing,
    FragmentationNonSemantic,
    ValueMemory,
    ManagedMemory,
    ResourceMemory,
    DevicePlacement,
    ControlFlowJoin,
    WorstCasePath,
    RecursionContribution,
    Cancellation,
    Concurrency,
    Backpressure,
    TargetCompilerBinding,
    Migration,
    CacheReplayIdentity,
    Proof,
    Estimate,
    Assumption,
    Unknown,
    Overflow,
    Unsupported,
    TargetMismatch,
    RuntimeGuard,
    OutOfMemory,
    QueueOverflow,
    DeviceAllocationFailure,
    FallbackPolicy,
    GuaranteeBoundary,
    SemanticId,
    SourceSpan,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    BoundaryFixture,
    OwnershipAliasFixture,
    QueueTaskDeviceFixture,
    TargetMigrationFixture,
    DeterminismFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedMemoryBudgetsBoundary {
    const ALL: [Self; 60] = [
        Self::MemoryBudgets,
        Self::StaticData,
        Self::Stack,
        Self::Arena,
        Self::Buffer,
        Self::QueueMailbox,
        Self::TaskActorState,
        Self::DeviceMemory,
        Self::TransientPeak,
        Self::ErrorFallbackPath,
        Self::Allocation,
        Self::Ownership,
        Self::Region,
        Self::Layout,
        Self::Units,
        Self::Alignment,
        Self::Lifetime,
        Self::DropTiming,
        Self::Aliasing,
        Self::Sharing,
        Self::FragmentationNonSemantic,
        Self::ValueMemory,
        Self::ManagedMemory,
        Self::ResourceMemory,
        Self::DevicePlacement,
        Self::ControlFlowJoin,
        Self::WorstCasePath,
        Self::RecursionContribution,
        Self::Cancellation,
        Self::Concurrency,
        Self::Backpressure,
        Self::TargetCompilerBinding,
        Self::Migration,
        Self::CacheReplayIdentity,
        Self::Proof,
        Self::Estimate,
        Self::Assumption,
        Self::Unknown,
        Self::Overflow,
        Self::Unsupported,
        Self::TargetMismatch,
        Self::RuntimeGuard,
        Self::OutOfMemory,
        Self::QueueOverflow,
        Self::DeviceAllocationFailure,
        Self::FallbackPolicy,
        Self::GuaranteeBoundary,
        Self::SemanticId,
        Self::SourceSpan,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::BoundaryFixture,
        Self::OwnershipAliasFixture,
        Self::QueueTaskDeviceFixture,
        Self::TargetMigrationFixture,
        Self::DeterminismFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MemoryBudgetsInventory {
    boundaries: Box<[PlannedMemoryBudgetsBoundary]>,
}

impl MemoryBudgetsInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedMemoryBudgetsBoundary>,
    ) -> Result<Self, PlannedMemoryBudgetsBoundary> {
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
        let mut bytes = b"ling.memory-budgets-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_memory_budget_boundaries_are_complete_and_ordered() {
    let inventory = MemoryBudgetsInventory::new(PlannedMemoryBudgetsBoundary::ALL)
        .expect("planned memory-budget boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedMemoryBudgetsBoundary::ALL
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
fn memory_budgets_evidence_is_order_independent_and_duplicate_checked() {
    let forward = MemoryBudgetsInventory::new(PlannedMemoryBudgetsBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = MemoryBudgetsInventory::new(PlannedMemoryBudgetsBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = MemoryBudgetsInventory::new([
        PlannedMemoryBudgetsBoundary::MemoryBudgets,
        PlannedMemoryBudgetsBoundary::MemoryBudgets,
    ])
    .expect_err("duplicate memory-budget boundary must be rejected");
    assert_eq!(duplicate, PlannedMemoryBudgetsBoundary::MemoryBudgets);
}

#[test]
fn memory_budgets_evidence_has_no_budget_authority() {
    let inventory = MemoryBudgetsInventory::new([
        PlannedMemoryBudgetsBoundary::MemoryBudgets,
        PlannedMemoryBudgetsBoundary::Allocation,
        PlannedMemoryBudgetsBoundary::Proof,
        PlannedMemoryBudgetsBoundary::Unknown,
        PlannedMemoryBudgetsBoundary::GuaranteeBoundary,
        PlannedMemoryBudgetsBoundary::DiagnosticCode,
        PlannedMemoryBudgetsBoundary::ProtocolInventory,
    ])
    .expect("bounded memory-budget evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.memory-budgets-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 7);
}
