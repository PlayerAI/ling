use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedBoundTypesExpressionsBoundary {
    BoundTypesExpressions,
    ConstantExpression,
    ProfileParameter,
    RangeType,
    CapacityType,
    LoopTripBound,
    RecursionDepth,
    TaskCount,
    ActorCount,
    StackBudget,
    ArenaBudget,
    MessageSize,
    Unit,
    Domain,
    Variance,
    NatDomain,
    IntDomain,
    Arithmetic,
    Overflow,
    Underflow,
    UnknownBound,
    SymbolicBound,
    CanonicalBytes,
    CheckedCoreNode,
    TypedCoreBoundary,
    ConstantEval,
    ConstraintSolver,
    Relation,
    CollectionSoundness,
    LoopSoundness,
    RecursionSoundness,
    TaskSoundness,
    ActorSoundness,
    StackSoundness,
    ArenaSoundness,
    MessageSoundness,
    DeviceSoundness,
    OwnershipRelation,
    EffectRelation,
    CapabilityRelation,
    SchedulerRelation,
    CancellationRelation,
    FaultRelation,
    FallbackRelation,
    ProofState,
    RuntimeGuarded,
    Forbidden,
    Assumed,
    ProfileLimit,
    TargetLimit,
    DiagnosticCode,
    DiagnosticFacts,
    SourceSpan,
    PositiveFixture,
    NegativeFixture,
    ArithmeticFixture,
    SymbolicFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedBoundTypesExpressionsBoundary {
    const ALL: [Self; 60] = [
        Self::BoundTypesExpressions,
        Self::ConstantExpression,
        Self::ProfileParameter,
        Self::RangeType,
        Self::CapacityType,
        Self::LoopTripBound,
        Self::RecursionDepth,
        Self::TaskCount,
        Self::ActorCount,
        Self::StackBudget,
        Self::ArenaBudget,
        Self::MessageSize,
        Self::Unit,
        Self::Domain,
        Self::Variance,
        Self::NatDomain,
        Self::IntDomain,
        Self::Arithmetic,
        Self::Overflow,
        Self::Underflow,
        Self::UnknownBound,
        Self::SymbolicBound,
        Self::CanonicalBytes,
        Self::CheckedCoreNode,
        Self::TypedCoreBoundary,
        Self::ConstantEval,
        Self::ConstraintSolver,
        Self::Relation,
        Self::CollectionSoundness,
        Self::LoopSoundness,
        Self::RecursionSoundness,
        Self::TaskSoundness,
        Self::ActorSoundness,
        Self::StackSoundness,
        Self::ArenaSoundness,
        Self::MessageSoundness,
        Self::DeviceSoundness,
        Self::OwnershipRelation,
        Self::EffectRelation,
        Self::CapabilityRelation,
        Self::SchedulerRelation,
        Self::CancellationRelation,
        Self::FaultRelation,
        Self::FallbackRelation,
        Self::ProofState,
        Self::RuntimeGuarded,
        Self::Forbidden,
        Self::Assumed,
        Self::ProfileLimit,
        Self::TargetLimit,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::SourceSpan,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::ArithmeticFixture,
        Self::SymbolicFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BoundTypesExpressionsInventory {
    boundaries: Box<[PlannedBoundTypesExpressionsBoundary]>,
}

impl BoundTypesExpressionsInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedBoundTypesExpressionsBoundary>,
    ) -> Result<Self, PlannedBoundTypesExpressionsBoundary> {
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
        let mut bytes = b"ling.bound-types-expressions-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_bound_types_expressions_boundaries_are_complete_and_ordered() {
    let inventory = BoundTypesExpressionsInventory::new(PlannedBoundTypesExpressionsBoundary::ALL)
        .expect("planned bound/type/expression boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedBoundTypesExpressionsBoundary::ALL
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
fn bound_types_expressions_evidence_is_order_independent_and_duplicate_checked() {
    let forward = BoundTypesExpressionsInventory::new(PlannedBoundTypesExpressionsBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = BoundTypesExpressionsInventory::new(
        PlannedBoundTypesExpressionsBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = BoundTypesExpressionsInventory::new([
        PlannedBoundTypesExpressionsBoundary::BoundTypesExpressions,
        PlannedBoundTypesExpressionsBoundary::BoundTypesExpressions,
    ])
    .expect_err("duplicate bound/type/expression boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedBoundTypesExpressionsBoundary::BoundTypesExpressions
    );
}

#[test]
fn bound_types_expressions_evidence_has_no_bound_authority() {
    let inventory = BoundTypesExpressionsInventory::new([
        PlannedBoundTypesExpressionsBoundary::BoundTypesExpressions,
        PlannedBoundTypesExpressionsBoundary::ConstantExpression,
        PlannedBoundTypesExpressionsBoundary::RangeType,
        PlannedBoundTypesExpressionsBoundary::UnknownBound,
        PlannedBoundTypesExpressionsBoundary::TypedCoreBoundary,
        PlannedBoundTypesExpressionsBoundary::ProofState,
        PlannedBoundTypesExpressionsBoundary::DiagnosticCode,
        PlannedBoundTypesExpressionsBoundary::ProtocolInventory,
    ])
    .expect("bounded bound/type/expression evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.bound-types-expressions-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
