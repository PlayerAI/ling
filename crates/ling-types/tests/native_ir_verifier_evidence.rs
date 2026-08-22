use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNativeIrVerifierBoundary {
    BasicBlock,
    BlockEntry,
    BlockExit,
    ControlFlow,
    Phi,
    SsaDefinition,
    SsaUse,
    Dominance,
    TypeConsistency,
    ValueType,
    AggregateType,
    ResourceOwnership,
    ManagedHandle,
    BorrowProvenance,
    CleanupCoverage,
    DropEdge,
    FaultEdge,
    EffectEdge,
    LegalAbi,
    CallingConvention,
    SourceId,
    SourceSpan,
    DefinitionMapping,
    ReferenceValidity,
    NoDanglingReference,
    NoUnresolvedOperation,
    BackendNeutrality,
    UnknownVersion,
    MalformedInput,
    DuplicateIdentifier,
    MissingBlock,
    InvalidPhi,
    InvalidType,
    InvalidOwnership,
    InvalidCleanup,
    InvalidAbi,
    DeterministicDiagnostics,
    SafeRejection,
    NoHostUb,
    ResourceBounds,
    UnicodeSourceSpans,
    SemanticId,
    DifferentialEvidence,
    MigrationCompatibility,
}

impl PlannedNativeIrVerifierBoundary {
    const ALL: [Self; 44] = [
        Self::BasicBlock,
        Self::BlockEntry,
        Self::BlockExit,
        Self::ControlFlow,
        Self::Phi,
        Self::SsaDefinition,
        Self::SsaUse,
        Self::Dominance,
        Self::TypeConsistency,
        Self::ValueType,
        Self::AggregateType,
        Self::ResourceOwnership,
        Self::ManagedHandle,
        Self::BorrowProvenance,
        Self::CleanupCoverage,
        Self::DropEdge,
        Self::FaultEdge,
        Self::EffectEdge,
        Self::LegalAbi,
        Self::CallingConvention,
        Self::SourceId,
        Self::SourceSpan,
        Self::DefinitionMapping,
        Self::ReferenceValidity,
        Self::NoDanglingReference,
        Self::NoUnresolvedOperation,
        Self::BackendNeutrality,
        Self::UnknownVersion,
        Self::MalformedInput,
        Self::DuplicateIdentifier,
        Self::MissingBlock,
        Self::InvalidPhi,
        Self::InvalidType,
        Self::InvalidOwnership,
        Self::InvalidCleanup,
        Self::InvalidAbi,
        Self::DeterministicDiagnostics,
        Self::SafeRejection,
        Self::NoHostUb,
        Self::ResourceBounds,
        Self::UnicodeSourceSpans,
        Self::SemanticId,
        Self::DifferentialEvidence,
        Self::MigrationCompatibility,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::BasicBlock => 0,
            Self::BlockEntry => 1,
            Self::BlockExit => 2,
            Self::ControlFlow => 3,
            Self::Phi => 4,
            Self::SsaDefinition => 5,
            Self::SsaUse => 6,
            Self::Dominance => 7,
            Self::TypeConsistency => 8,
            Self::ValueType => 9,
            Self::AggregateType => 10,
            Self::ResourceOwnership => 11,
            Self::ManagedHandle => 12,
            Self::BorrowProvenance => 13,
            Self::CleanupCoverage => 14,
            Self::DropEdge => 15,
            Self::FaultEdge => 16,
            Self::EffectEdge => 17,
            Self::LegalAbi => 18,
            Self::CallingConvention => 19,
            Self::SourceId => 20,
            Self::SourceSpan => 21,
            Self::DefinitionMapping => 22,
            Self::ReferenceValidity => 23,
            Self::NoDanglingReference => 24,
            Self::NoUnresolvedOperation => 25,
            Self::BackendNeutrality => 26,
            Self::UnknownVersion => 27,
            Self::MalformedInput => 28,
            Self::DuplicateIdentifier => 29,
            Self::MissingBlock => 30,
            Self::InvalidPhi => 31,
            Self::InvalidType => 32,
            Self::InvalidOwnership => 33,
            Self::InvalidCleanup => 34,
            Self::InvalidAbi => 35,
            Self::DeterministicDiagnostics => 36,
            Self::SafeRejection => 37,
            Self::NoHostUb => 38,
            Self::ResourceBounds => 39,
            Self::UnicodeSourceSpans => 40,
            Self::SemanticId => 41,
            Self::DifferentialEvidence => 42,
            Self::MigrationCompatibility => 43,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NativeIrVerifierBoundaryInventory {
    boundaries: Box<[PlannedNativeIrVerifierBoundary]>,
}

impl NativeIrVerifierBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNativeIrVerifierBoundary>,
    ) -> Result<Self, PlannedNativeIrVerifierBoundary> {
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
        bytes.extend_from_slice(b"ling.native-ir-verifier-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_native_ir_verifier_boundaries_are_complete_and_ordered() {
    let inventory = NativeIrVerifierBoundaryInventory::new(PlannedNativeIrVerifierBoundary::ALL)
        .expect("planned Native IR verifier boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNativeIrVerifierBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..44).collect::<Vec<_>>()
    );
}

#[test]
fn native_ir_verifier_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NativeIrVerifierBoundaryInventory::new(PlannedNativeIrVerifierBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = NativeIrVerifierBoundaryInventory::new(
        PlannedNativeIrVerifierBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NativeIrVerifierBoundaryInventory::new([
        PlannedNativeIrVerifierBoundary::BasicBlock,
        PlannedNativeIrVerifierBoundary::BasicBlock,
    ])
    .expect_err("duplicate Native IR verifier boundary must be rejected");
    assert_eq!(duplicate, PlannedNativeIrVerifierBoundary::BasicBlock);
}

#[test]
fn native_ir_verifier_evidence_has_no_validation_or_execution_authority() {
    let inventory = NativeIrVerifierBoundaryInventory::new([
        PlannedNativeIrVerifierBoundary::Phi,
        PlannedNativeIrVerifierBoundary::TypeConsistency,
        PlannedNativeIrVerifierBoundary::ResourceOwnership,
        PlannedNativeIrVerifierBoundary::CleanupCoverage,
        PlannedNativeIrVerifierBoundary::LegalAbi,
        PlannedNativeIrVerifierBoundary::NoHostUb,
        PlannedNativeIrVerifierBoundary::NoUnresolvedOperation,
    ])
    .expect("bounded Native IR verifier evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.native-ir-verifier-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 7);
}
