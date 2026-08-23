use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedLoweringValidatorBoundary {
    LoweringValidator,
    Version,
    ValidationBoundary,
    SourceRepresentation,
    TargetRepresentation,
    CheckedTypedCoreInput,
    BackendNeutralIr,
    NativeIr,
    TargetCode,
    ComposedValidation,
    SupportedCore,
    SupportedTarget,
    SupportedProfile,
    TypeCheck,
    LayoutCheck,
    ControlFlowCheck,
    ValueMapping,
    EvaluationOrder,
    EffectCapability,
    ContractPreservation,
    MemoryAlias,
    OwnershipResource,
    FaultUnwinding,
    ThreadReentry,
    FfiAbi,
    SourceBinaryCorrespondence,
    SourceId,
    SemanticId,
    ArtifactId,
    TargetId,
    ToolchainId,
    SourceSpan,
    OriginalUtf8Bytes,
    UnicodeVersion,
    Equivalence,
    Soundness,
    ProofObligation,
    Certificate,
    IndependentChecker,
    TrustBoundary,
    TcbIdentity,
    Assumption,
    OptimizationLimit,
    ResourceBound,
    FailClosed,
    UnsupportedConstruct,
    InvalidLowering,
    TypeLayoutMismatch,
    ControlFlowMismatch,
    ValueMappingMismatch,
    MissingPreservationFact,
    AliasViolation,
    SourceMapMismatch,
    UnavailableTarget,
    ValidatorFailure,
    PositiveFixture,
    NegativeFixture,
    DifferentialFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedLoweringValidatorBoundary {
    const ALL: [Self; 60] = [
        Self::LoweringValidator,
        Self::Version,
        Self::ValidationBoundary,
        Self::SourceRepresentation,
        Self::TargetRepresentation,
        Self::CheckedTypedCoreInput,
        Self::BackendNeutralIr,
        Self::NativeIr,
        Self::TargetCode,
        Self::ComposedValidation,
        Self::SupportedCore,
        Self::SupportedTarget,
        Self::SupportedProfile,
        Self::TypeCheck,
        Self::LayoutCheck,
        Self::ControlFlowCheck,
        Self::ValueMapping,
        Self::EvaluationOrder,
        Self::EffectCapability,
        Self::ContractPreservation,
        Self::MemoryAlias,
        Self::OwnershipResource,
        Self::FaultUnwinding,
        Self::ThreadReentry,
        Self::FfiAbi,
        Self::SourceBinaryCorrespondence,
        Self::SourceId,
        Self::SemanticId,
        Self::ArtifactId,
        Self::TargetId,
        Self::ToolchainId,
        Self::SourceSpan,
        Self::OriginalUtf8Bytes,
        Self::UnicodeVersion,
        Self::Equivalence,
        Self::Soundness,
        Self::ProofObligation,
        Self::Certificate,
        Self::IndependentChecker,
        Self::TrustBoundary,
        Self::TcbIdentity,
        Self::Assumption,
        Self::OptimizationLimit,
        Self::ResourceBound,
        Self::FailClosed,
        Self::UnsupportedConstruct,
        Self::InvalidLowering,
        Self::TypeLayoutMismatch,
        Self::ControlFlowMismatch,
        Self::ValueMappingMismatch,
        Self::MissingPreservationFact,
        Self::AliasViolation,
        Self::SourceMapMismatch,
        Self::UnavailableTarget,
        Self::ValidatorFailure,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::DifferentialFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LoweringValidatorInventory {
    boundaries: Box<[PlannedLoweringValidatorBoundary]>,
}

impl LoweringValidatorInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedLoweringValidatorBoundary>,
    ) -> Result<Self, PlannedLoweringValidatorBoundary> {
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
        let mut bytes = b"ling.lowering-validator-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_lowering_validator_boundaries_are_complete_and_ordered() {
    let inventory = LoweringValidatorInventory::new(PlannedLoweringValidatorBoundary::ALL)
        .expect("planned lowering-validator boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedLoweringValidatorBoundary::ALL
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
fn lowering_validator_evidence_is_order_independent_and_duplicate_checked() {
    let forward = LoweringValidatorInventory::new(PlannedLoweringValidatorBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        LoweringValidatorInventory::new(PlannedLoweringValidatorBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = LoweringValidatorInventory::new([
        PlannedLoweringValidatorBoundary::LoweringValidator,
        PlannedLoweringValidatorBoundary::LoweringValidator,
    ])
    .expect_err("duplicate lowering-validator boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedLoweringValidatorBoundary::LoweringValidator
    );
}

#[test]
fn lowering_validator_evidence_has_no_native_validation_authority() {
    let inventory = LoweringValidatorInventory::new([
        PlannedLoweringValidatorBoundary::LoweringValidator,
        PlannedLoweringValidatorBoundary::CheckedTypedCoreInput,
        PlannedLoweringValidatorBoundary::TypeCheck,
        PlannedLoweringValidatorBoundary::LayoutCheck,
        PlannedLoweringValidatorBoundary::ControlFlowCheck,
        PlannedLoweringValidatorBoundary::ValueMapping,
        PlannedLoweringValidatorBoundary::ContractPreservation,
        PlannedLoweringValidatorBoundary::MemoryAlias,
        PlannedLoweringValidatorBoundary::SourceBinaryCorrespondence,
        PlannedLoweringValidatorBoundary::UnsupportedConstruct,
        PlannedLoweringValidatorBoundary::ProtocolInventory,
    ])
    .expect("bounded lowering-validator evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.lowering-validator-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 11);
}
