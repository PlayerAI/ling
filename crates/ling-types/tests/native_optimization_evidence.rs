use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNativeOptimizationBoundary {
    ConstantFolding,
    NumericSemantics,
    EffectAwareFolding,
    FaultAwareFolding,
    DeadBlockElimination,
    ReachabilityProof,
    TrivialInlining,
    ClosureCapture,
    InliningRecursion,
    CopyPropagation,
    AliasProof,
    BoundsCheckElimination,
    BoundsProof,
    TailCall,
    TailCleanup,
    EvaluationOrder,
    CapabilityPreservation,
    ResourceDrop,
    ManagedRootBarrier,
    BorrowAlias,
    TaskActor,
    Cancellation,
    FfiBoundary,
    ProfileCapability,
    AbiPreservation,
    TargetNumericModel,
    Endianness,
    PreVerifier,
    PostVerifier,
    PassOrder,
    Invalidation,
    ProofCertificate,
    DiagnosticDeterminism,
    OptimizationFailure,
    SourceIdentity,
    DebugLocation,
    StackTrace,
    SemanticId,
    UnicodeSourceSpan,
    DeterministicMetadata,
    ReproduciblePass,
    ResourceCompileBounds,
    SecurityTcb,
    VersionMigration,
    PositiveFixtures,
    NegativeFixtures,
    PropertyCorpus,
    FuzzStress,
    InterpreterVmNativeDifferential,
    UnoptimizedOptimizedEquivalence,
    NativeDifferential,
    HostTimingExclusion,
    AllocationOrderExclusion,
    AddressMapExclusion,
    UnsupportedForm,
    SeedCompatibility,
    CleanupCoverage,
    FaultVisibility,
    NumericOverflow,
    FloatRules,
}

impl PlannedNativeOptimizationBoundary {
    const ALL: [Self; 60] = [
        Self::ConstantFolding,
        Self::NumericSemantics,
        Self::EffectAwareFolding,
        Self::FaultAwareFolding,
        Self::DeadBlockElimination,
        Self::ReachabilityProof,
        Self::TrivialInlining,
        Self::ClosureCapture,
        Self::InliningRecursion,
        Self::CopyPropagation,
        Self::AliasProof,
        Self::BoundsCheckElimination,
        Self::BoundsProof,
        Self::TailCall,
        Self::TailCleanup,
        Self::EvaluationOrder,
        Self::CapabilityPreservation,
        Self::ResourceDrop,
        Self::ManagedRootBarrier,
        Self::BorrowAlias,
        Self::TaskActor,
        Self::Cancellation,
        Self::FfiBoundary,
        Self::ProfileCapability,
        Self::AbiPreservation,
        Self::TargetNumericModel,
        Self::Endianness,
        Self::PreVerifier,
        Self::PostVerifier,
        Self::PassOrder,
        Self::Invalidation,
        Self::ProofCertificate,
        Self::DiagnosticDeterminism,
        Self::OptimizationFailure,
        Self::SourceIdentity,
        Self::DebugLocation,
        Self::StackTrace,
        Self::SemanticId,
        Self::UnicodeSourceSpan,
        Self::DeterministicMetadata,
        Self::ReproduciblePass,
        Self::ResourceCompileBounds,
        Self::SecurityTcb,
        Self::VersionMigration,
        Self::PositiveFixtures,
        Self::NegativeFixtures,
        Self::PropertyCorpus,
        Self::FuzzStress,
        Self::InterpreterVmNativeDifferential,
        Self::UnoptimizedOptimizedEquivalence,
        Self::NativeDifferential,
        Self::HostTimingExclusion,
        Self::AllocationOrderExclusion,
        Self::AddressMapExclusion,
        Self::UnsupportedForm,
        Self::SeedCompatibility,
        Self::CleanupCoverage,
        Self::FaultVisibility,
        Self::NumericOverflow,
        Self::FloatRules,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ConstantFolding => 0,
            Self::NumericSemantics => 1,
            Self::EffectAwareFolding => 2,
            Self::FaultAwareFolding => 3,
            Self::DeadBlockElimination => 4,
            Self::ReachabilityProof => 5,
            Self::TrivialInlining => 6,
            Self::ClosureCapture => 7,
            Self::InliningRecursion => 8,
            Self::CopyPropagation => 9,
            Self::AliasProof => 10,
            Self::BoundsCheckElimination => 11,
            Self::BoundsProof => 12,
            Self::TailCall => 13,
            Self::TailCleanup => 14,
            Self::EvaluationOrder => 15,
            Self::CapabilityPreservation => 16,
            Self::ResourceDrop => 17,
            Self::ManagedRootBarrier => 18,
            Self::BorrowAlias => 19,
            Self::TaskActor => 20,
            Self::Cancellation => 21,
            Self::FfiBoundary => 22,
            Self::ProfileCapability => 23,
            Self::AbiPreservation => 24,
            Self::TargetNumericModel => 25,
            Self::Endianness => 26,
            Self::PreVerifier => 27,
            Self::PostVerifier => 28,
            Self::PassOrder => 29,
            Self::Invalidation => 30,
            Self::ProofCertificate => 31,
            Self::DiagnosticDeterminism => 32,
            Self::OptimizationFailure => 33,
            Self::SourceIdentity => 34,
            Self::DebugLocation => 35,
            Self::StackTrace => 36,
            Self::SemanticId => 37,
            Self::UnicodeSourceSpan => 38,
            Self::DeterministicMetadata => 39,
            Self::ReproduciblePass => 40,
            Self::ResourceCompileBounds => 41,
            Self::SecurityTcb => 42,
            Self::VersionMigration => 43,
            Self::PositiveFixtures => 44,
            Self::NegativeFixtures => 45,
            Self::PropertyCorpus => 46,
            Self::FuzzStress => 47,
            Self::InterpreterVmNativeDifferential => 48,
            Self::UnoptimizedOptimizedEquivalence => 49,
            Self::NativeDifferential => 50,
            Self::HostTimingExclusion => 51,
            Self::AllocationOrderExclusion => 52,
            Self::AddressMapExclusion => 53,
            Self::UnsupportedForm => 54,
            Self::SeedCompatibility => 55,
            Self::CleanupCoverage => 56,
            Self::FaultVisibility => 57,
            Self::NumericOverflow => 58,
            Self::FloatRules => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NativeOptimizationBoundaryInventory {
    boundaries: Box<[PlannedNativeOptimizationBoundary]>,
}

impl NativeOptimizationBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNativeOptimizationBoundary>,
    ) -> Result<Self, PlannedNativeOptimizationBoundary> {
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
        bytes.extend_from_slice(b"ling.native-optimization-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_native_optimization_boundaries_are_complete_and_ordered() {
    let inventory =
        NativeOptimizationBoundaryInventory::new(PlannedNativeOptimizationBoundary::ALL)
            .expect("planned Native optimization boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNativeOptimizationBoundary::ALL
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
fn native_optimization_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NativeOptimizationBoundaryInventory::new(PlannedNativeOptimizationBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = NativeOptimizationBoundaryInventory::new(
        PlannedNativeOptimizationBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NativeOptimizationBoundaryInventory::new([
        PlannedNativeOptimizationBoundary::ConstantFolding,
        PlannedNativeOptimizationBoundary::ConstantFolding,
    ])
    .expect_err("duplicate Native optimization boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedNativeOptimizationBoundary::ConstantFolding
    );
}

#[test]
fn native_optimization_evidence_has_no_pass_or_proof_authority() {
    let inventory = NativeOptimizationBoundaryInventory::new([
        PlannedNativeOptimizationBoundary::ConstantFolding,
        PlannedNativeOptimizationBoundary::DeadBlockElimination,
        PlannedNativeOptimizationBoundary::TrivialInlining,
        PlannedNativeOptimizationBoundary::BoundsProof,
        PlannedNativeOptimizationBoundary::CopyPropagation,
        PlannedNativeOptimizationBoundary::TailCall,
        PlannedNativeOptimizationBoundary::PreVerifier,
        PlannedNativeOptimizationBoundary::PostVerifier,
        PlannedNativeOptimizationBoundary::SeedCompatibility,
    ])
    .expect("bounded Native optimization evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.native-optimization-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 9);
}
