//! Internal Value-layout and Copy/Move boundary evidence.
//!
//! This test-only inventory names proposed memory and ownership boundaries.
//! It does not implement layouts, Copy/Move rules, ownership checking, ABI,
//! serialization, or optimization semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedMemoryBoundary {
    ValueKind,
    InlineRepresentation,
    RegisterOptimization,
    StackOptimization,
    LingCopyRule,
    LingMoveRule,
    ImplicitCopy,
    ExplicitMove,
    ClosureCapture,
    RecursiveAggregate,
    GenericConstraint,
    TraitInteraction,
    ResourceInteraction,
    SeparateCompilation,
    Equality,
    Hash,
    Serialization,
    SemanticProjection,
    AuditSourceProjection,
    CanonicalBytes,
    Size,
    Alignment,
    Overflow,
    Padding,
    Endianness,
    Discriminant,
    NicheOptimization,
    PointerIdentity,
    OptimizationEquivalence,
    CopyMoveDiagnostic,
    UseAfterMove,
    OwnershipBoundary,
    NativeAbi,
    ProfileConstraint,
    InterpreterVmNativeDifferential,
    UnicodeSourceSpans,
    MigrationCompatibility,
}

impl PlannedMemoryBoundary {
    const ALL: [Self; 37] = [
        Self::ValueKind,
        Self::InlineRepresentation,
        Self::RegisterOptimization,
        Self::StackOptimization,
        Self::LingCopyRule,
        Self::LingMoveRule,
        Self::ImplicitCopy,
        Self::ExplicitMove,
        Self::ClosureCapture,
        Self::RecursiveAggregate,
        Self::GenericConstraint,
        Self::TraitInteraction,
        Self::ResourceInteraction,
        Self::SeparateCompilation,
        Self::Equality,
        Self::Hash,
        Self::Serialization,
        Self::SemanticProjection,
        Self::AuditSourceProjection,
        Self::CanonicalBytes,
        Self::Size,
        Self::Alignment,
        Self::Overflow,
        Self::Padding,
        Self::Endianness,
        Self::Discriminant,
        Self::NicheOptimization,
        Self::PointerIdentity,
        Self::OptimizationEquivalence,
        Self::CopyMoveDiagnostic,
        Self::UseAfterMove,
        Self::OwnershipBoundary,
        Self::NativeAbi,
        Self::ProfileConstraint,
        Self::InterpreterVmNativeDifferential,
        Self::UnicodeSourceSpans,
        Self::MigrationCompatibility,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ValueKind => 0,
            Self::InlineRepresentation => 1,
            Self::RegisterOptimization => 2,
            Self::StackOptimization => 3,
            Self::LingCopyRule => 4,
            Self::LingMoveRule => 5,
            Self::ImplicitCopy => 6,
            Self::ExplicitMove => 7,
            Self::ClosureCapture => 8,
            Self::RecursiveAggregate => 9,
            Self::GenericConstraint => 10,
            Self::TraitInteraction => 11,
            Self::ResourceInteraction => 12,
            Self::SeparateCompilation => 13,
            Self::Equality => 14,
            Self::Hash => 15,
            Self::Serialization => 16,
            Self::SemanticProjection => 17,
            Self::AuditSourceProjection => 18,
            Self::CanonicalBytes => 19,
            Self::Size => 20,
            Self::Alignment => 21,
            Self::Overflow => 22,
            Self::Padding => 23,
            Self::Endianness => 24,
            Self::Discriminant => 25,
            Self::NicheOptimization => 26,
            Self::PointerIdentity => 27,
            Self::OptimizationEquivalence => 28,
            Self::CopyMoveDiagnostic => 29,
            Self::UseAfterMove => 30,
            Self::OwnershipBoundary => 31,
            Self::NativeAbi => 32,
            Self::ProfileConstraint => 33,
            Self::InterpreterVmNativeDifferential => 34,
            Self::UnicodeSourceSpans => 35,
            Self::MigrationCompatibility => 36,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MemoryBoundaryInventory {
    boundaries: Box<[PlannedMemoryBoundary]>,
}

impl MemoryBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedMemoryBoundary>,
    ) -> Result<Self, PlannedMemoryBoundary> {
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
        bytes.extend_from_slice(b"ling.memory-layout-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_memory_boundaries_are_complete_and_ordered() {
    let inventory = MemoryBoundaryInventory::new(PlannedMemoryBoundary::ALL)
        .expect("planned memory boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedMemoryBoundary::ALL);
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..37).collect::<Vec<_>>()
    );
}

#[test]
fn memory_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = MemoryBoundaryInventory::new(PlannedMemoryBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = MemoryBoundaryInventory::new(PlannedMemoryBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = MemoryBoundaryInventory::new([
        PlannedMemoryBoundary::LingCopyRule,
        PlannedMemoryBoundary::LingCopyRule,
    ])
    .expect_err("duplicate memory boundary must be rejected");
    assert_eq!(duplicate, PlannedMemoryBoundary::LingCopyRule);
}

#[test]
fn memory_boundary_evidence_has_no_layout_or_ownership_authority() {
    let inventory = MemoryBoundaryInventory::new([
        PlannedMemoryBoundary::InlineRepresentation,
        PlannedMemoryBoundary::LingCopyRule,
        PlannedMemoryBoundary::NativeAbi,
        PlannedMemoryBoundary::UseAfterMove,
    ])
    .expect("bounded memory evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.memory-layout-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
