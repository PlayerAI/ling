//! Internal backend-neutral Native IR design boundary evidence.
//!
//! This test-only inventory names proposed NIR contracts. It does not
//! implement an IR, instruction set, ABI, serializer, verifier, or lowering
//! semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedNativeIrBoundary {
    TypedSsa,
    ControlFlow,
    BasicBlock,
    Phi,
    EvaluationOrder,
    ValueRepresentation,
    ResourceRepresentation,
    ManagedRepresentation,
    AggregateLayout,
    ClosureLayout,
    BorrowProvenance,
    AliasFacts,
    OwnershipFacts,
    ExplicitDrop,
    Cleanup,
    FunctionAbi,
    CallingConvention,
    DataLayout,
    Alignment,
    Discriminant,
    FaultEdge,
    Unwind,
    EffectBoundary,
    CapabilityBoundary,
    TaskRuntimeAbi,
    ActorRuntimeAbi,
    FfiBoundary,
    TargetPackage,
    TargetEndianness,
    SourceMapping,
    DebugVariableLocation,
    DefinitionMapping,
    DeterministicSerialization,
    SchemaVersion,
    UnknownVersionRejection,
    MalformedInput,
    BackendNeutrality,
    UnresolvedOperationRejection,
    SemanticId,
    TypedCoreInput,
    SemanticGraphProjection,
    AuditSourceProjection,
    InterpreterVmNativeDifferential,
    UnicodeSourceSpans,
    MigrationCompatibility,
    SecurityBounds,
}

impl PlannedNativeIrBoundary {
    const ALL: [Self; 46] = [
        Self::TypedSsa,
        Self::ControlFlow,
        Self::BasicBlock,
        Self::Phi,
        Self::EvaluationOrder,
        Self::ValueRepresentation,
        Self::ResourceRepresentation,
        Self::ManagedRepresentation,
        Self::AggregateLayout,
        Self::ClosureLayout,
        Self::BorrowProvenance,
        Self::AliasFacts,
        Self::OwnershipFacts,
        Self::ExplicitDrop,
        Self::Cleanup,
        Self::FunctionAbi,
        Self::CallingConvention,
        Self::DataLayout,
        Self::Alignment,
        Self::Discriminant,
        Self::FaultEdge,
        Self::Unwind,
        Self::EffectBoundary,
        Self::CapabilityBoundary,
        Self::TaskRuntimeAbi,
        Self::ActorRuntimeAbi,
        Self::FfiBoundary,
        Self::TargetPackage,
        Self::TargetEndianness,
        Self::SourceMapping,
        Self::DebugVariableLocation,
        Self::DefinitionMapping,
        Self::DeterministicSerialization,
        Self::SchemaVersion,
        Self::UnknownVersionRejection,
        Self::MalformedInput,
        Self::BackendNeutrality,
        Self::UnresolvedOperationRejection,
        Self::SemanticId,
        Self::TypedCoreInput,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::InterpreterVmNativeDifferential,
        Self::UnicodeSourceSpans,
        Self::MigrationCompatibility,
        Self::SecurityBounds,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::TypedSsa => 0,
            Self::ControlFlow => 1,
            Self::BasicBlock => 2,
            Self::Phi => 3,
            Self::EvaluationOrder => 4,
            Self::ValueRepresentation => 5,
            Self::ResourceRepresentation => 6,
            Self::ManagedRepresentation => 7,
            Self::AggregateLayout => 8,
            Self::ClosureLayout => 9,
            Self::BorrowProvenance => 10,
            Self::AliasFacts => 11,
            Self::OwnershipFacts => 12,
            Self::ExplicitDrop => 13,
            Self::Cleanup => 14,
            Self::FunctionAbi => 15,
            Self::CallingConvention => 16,
            Self::DataLayout => 17,
            Self::Alignment => 18,
            Self::Discriminant => 19,
            Self::FaultEdge => 20,
            Self::Unwind => 21,
            Self::EffectBoundary => 22,
            Self::CapabilityBoundary => 23,
            Self::TaskRuntimeAbi => 24,
            Self::ActorRuntimeAbi => 25,
            Self::FfiBoundary => 26,
            Self::TargetPackage => 27,
            Self::TargetEndianness => 28,
            Self::SourceMapping => 29,
            Self::DebugVariableLocation => 30,
            Self::DefinitionMapping => 31,
            Self::DeterministicSerialization => 32,
            Self::SchemaVersion => 33,
            Self::UnknownVersionRejection => 34,
            Self::MalformedInput => 35,
            Self::BackendNeutrality => 36,
            Self::UnresolvedOperationRejection => 37,
            Self::SemanticId => 38,
            Self::TypedCoreInput => 39,
            Self::SemanticGraphProjection => 40,
            Self::AuditSourceProjection => 41,
            Self::InterpreterVmNativeDifferential => 42,
            Self::UnicodeSourceSpans => 43,
            Self::MigrationCompatibility => 44,
            Self::SecurityBounds => 45,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NativeIrBoundaryInventory {
    boundaries: Box<[PlannedNativeIrBoundary]>,
}

impl NativeIrBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNativeIrBoundary>,
    ) -> Result<Self, PlannedNativeIrBoundary> {
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
        bytes.extend_from_slice(b"ling.native-ir-design-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_native_ir_boundaries_are_complete_and_ordered() {
    let inventory = NativeIrBoundaryInventory::new(PlannedNativeIrBoundary::ALL)
        .expect("planned Native IR boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedNativeIrBoundary::ALL);
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
fn native_ir_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NativeIrBoundaryInventory::new(PlannedNativeIrBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = NativeIrBoundaryInventory::new(PlannedNativeIrBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NativeIrBoundaryInventory::new([
        PlannedNativeIrBoundary::TypedSsa,
        PlannedNativeIrBoundary::TypedSsa,
    ])
    .expect_err("duplicate Native IR boundary must be rejected");
    assert_eq!(duplicate, PlannedNativeIrBoundary::TypedSsa);
}

#[test]
fn native_ir_evidence_has_no_instruction_or_abi_authority() {
    let inventory = NativeIrBoundaryInventory::new([
        PlannedNativeIrBoundary::TypedSsa,
        PlannedNativeIrBoundary::BorrowProvenance,
        PlannedNativeIrBoundary::FunctionAbi,
        PlannedNativeIrBoundary::FaultEdge,
        PlannedNativeIrBoundary::DeterministicSerialization,
        PlannedNativeIrBoundary::BackendNeutrality,
    ])
    .expect("bounded Native IR evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.native-ir-design-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
