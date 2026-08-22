//! Internal Core-to-Native IR lowering boundary evidence.
//!
//! This test-only inventory names proposed lowering slices and preservation
//! obligations. It does not implement a lowering pass, Native IR, ABI,
//! differential harness, or backend semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedLoweringBoundary {
    IntegerBoolCall,
    RecordTuple,
    AdtMatch,
    MutablePlace,
    Closure,
    EffectOperation,
    ResourceDrop,
    ManagedHandle,
    TaskActorAbi,
    CheckedCoreOnly,
    UnsupportedFormRejection,
    EvaluationOrder,
    ValueRepresentation,
    AggregateRepresentation,
    ClosureRepresentation,
    MemoryCategory,
    BorrowProvenance,
    AliasFacts,
    Cleanup,
    Drop,
    Allocation,
    GcBarrier,
    EffectCapability,
    FaultEdge,
    Cancellation,
    SourceSpan,
    SemanticId,
    AbiTarget,
    ProfileFfi,
    ThreadReentry,
    RuntimeLibrary,
    MigrationVersion,
    DeterministicOrdering,
    Serialization,
    MalformedInput,
    InterpreterVmNativeDifferential,
    NondeterminismExclusion,
    TargetDifference,
    HostFaultExclusion,
    MetricExclusion,
    DebugLocationExclusion,
    BilingualDiagnostic,
    UnicodeSourceSpans,
    ResourceBounds,
    NoUnresolvedAstHir,
    SeedCompatibility,
}

impl PlannedLoweringBoundary {
    const ALL: [Self; 46] = [
        Self::IntegerBoolCall,
        Self::RecordTuple,
        Self::AdtMatch,
        Self::MutablePlace,
        Self::Closure,
        Self::EffectOperation,
        Self::ResourceDrop,
        Self::ManagedHandle,
        Self::TaskActorAbi,
        Self::CheckedCoreOnly,
        Self::UnsupportedFormRejection,
        Self::EvaluationOrder,
        Self::ValueRepresentation,
        Self::AggregateRepresentation,
        Self::ClosureRepresentation,
        Self::MemoryCategory,
        Self::BorrowProvenance,
        Self::AliasFacts,
        Self::Cleanup,
        Self::Drop,
        Self::Allocation,
        Self::GcBarrier,
        Self::EffectCapability,
        Self::FaultEdge,
        Self::Cancellation,
        Self::SourceSpan,
        Self::SemanticId,
        Self::AbiTarget,
        Self::ProfileFfi,
        Self::ThreadReentry,
        Self::RuntimeLibrary,
        Self::MigrationVersion,
        Self::DeterministicOrdering,
        Self::Serialization,
        Self::MalformedInput,
        Self::InterpreterVmNativeDifferential,
        Self::NondeterminismExclusion,
        Self::TargetDifference,
        Self::HostFaultExclusion,
        Self::MetricExclusion,
        Self::DebugLocationExclusion,
        Self::BilingualDiagnostic,
        Self::UnicodeSourceSpans,
        Self::ResourceBounds,
        Self::NoUnresolvedAstHir,
        Self::SeedCompatibility,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::IntegerBoolCall => 0,
            Self::RecordTuple => 1,
            Self::AdtMatch => 2,
            Self::MutablePlace => 3,
            Self::Closure => 4,
            Self::EffectOperation => 5,
            Self::ResourceDrop => 6,
            Self::ManagedHandle => 7,
            Self::TaskActorAbi => 8,
            Self::CheckedCoreOnly => 9,
            Self::UnsupportedFormRejection => 10,
            Self::EvaluationOrder => 11,
            Self::ValueRepresentation => 12,
            Self::AggregateRepresentation => 13,
            Self::ClosureRepresentation => 14,
            Self::MemoryCategory => 15,
            Self::BorrowProvenance => 16,
            Self::AliasFacts => 17,
            Self::Cleanup => 18,
            Self::Drop => 19,
            Self::Allocation => 20,
            Self::GcBarrier => 21,
            Self::EffectCapability => 22,
            Self::FaultEdge => 23,
            Self::Cancellation => 24,
            Self::SourceSpan => 25,
            Self::SemanticId => 26,
            Self::AbiTarget => 27,
            Self::ProfileFfi => 28,
            Self::ThreadReentry => 29,
            Self::RuntimeLibrary => 30,
            Self::MigrationVersion => 31,
            Self::DeterministicOrdering => 32,
            Self::Serialization => 33,
            Self::MalformedInput => 34,
            Self::InterpreterVmNativeDifferential => 35,
            Self::NondeterminismExclusion => 36,
            Self::TargetDifference => 37,
            Self::HostFaultExclusion => 38,
            Self::MetricExclusion => 39,
            Self::DebugLocationExclusion => 40,
            Self::BilingualDiagnostic => 41,
            Self::UnicodeSourceSpans => 42,
            Self::ResourceBounds => 43,
            Self::NoUnresolvedAstHir => 44,
            Self::SeedCompatibility => 45,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LoweringBoundaryInventory {
    boundaries: Box<[PlannedLoweringBoundary]>,
}

impl LoweringBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedLoweringBoundary>,
    ) -> Result<Self, PlannedLoweringBoundary> {
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
        bytes.extend_from_slice(b"ling.native-ir-lowering-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_lowering_boundaries_are_complete_and_ordered() {
    let inventory = LoweringBoundaryInventory::new(PlannedLoweringBoundary::ALL)
        .expect("planned lowering boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedLoweringBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..46).collect::<Vec<_>>()
    );
}

#[test]
fn lowering_evidence_is_order_independent_and_duplicate_checked() {
    let forward = LoweringBoundaryInventory::new(PlannedLoweringBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = LoweringBoundaryInventory::new(PlannedLoweringBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = LoweringBoundaryInventory::new([
        PlannedLoweringBoundary::IntegerBoolCall,
        PlannedLoweringBoundary::IntegerBoolCall,
    ])
    .expect_err("duplicate lowering boundary must be rejected");
    assert_eq!(duplicate, PlannedLoweringBoundary::IntegerBoolCall);
}

#[test]
fn lowering_evidence_has_no_native_or_differential_authority() {
    let inventory = LoweringBoundaryInventory::new([
        PlannedLoweringBoundary::CheckedCoreOnly,
        PlannedLoweringBoundary::UnsupportedFormRejection,
        PlannedLoweringBoundary::ResourceDrop,
        PlannedLoweringBoundary::ManagedHandle,
        PlannedLoweringBoundary::InterpreterVmNativeDifferential,
        PlannedLoweringBoundary::NoUnresolvedAstHir,
    ])
    .expect("bounded lowering evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.native-ir-lowering-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
