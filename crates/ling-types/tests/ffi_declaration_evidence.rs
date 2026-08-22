use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedFfiDeclarationBoundary {
    DeclarationIdentity,
    DeclarationName,
    ForeignSymbol,
    AbiSelection,
    AbiVersion,
    ArgumentTypes,
    ResultType,
    LayoutDescription,
    ScalarRepresentation,
    AggregateRepresentation,
    Alignment,
    Endianness,
    CallingConvention,
    TargetTriple,
    TargetConstraints,
    ProfileAvailability,
    CapabilityRequirement,
    OwnershipTransfer,
    BorrowDuration,
    Mutability,
    ResourceLifetime,
    ManagedBoundary,
    AllocatorPair,
    PointerProvenance,
    SpanBounds,
    Nullability,
    Encoding,
    CallbackSignature,
    CallbackLifetime,
    CallbackThreading,
    ThreadAffinity,
    Reentrancy,
    Blocking,
    Cancellation,
    ErrorMapping,
    FaultMapping,
    UnwindPolicy,
    OpaqueHandle,
    SymbolVersion,
    NameMangling,
    DeclarationSourceSpan,
    SemanticIdProjection,
    ViewSeparation,
    GrammarBoundary,
    AstNodeBoundary,
    HirNodeBoundary,
    CheckedCoreBoundary,
    VerifiedLowering,
    DiagnosticCode,
    BilingualDiagnostic,
    UnicodeSourceSpans,
    UnsupportedConstructs,
    DeterministicOrdering,
    CanonicalSchema,
    SchemaVersion,
    MigrationPolicy,
    ProvenanceTcb,
    SanitizerFuzzEvidence,
    CrossTargetEvidence,
    SeedCompatibility,
}

impl PlannedFfiDeclarationBoundary {
    const ALL: [Self; 60] = [
        Self::DeclarationIdentity,
        Self::DeclarationName,
        Self::ForeignSymbol,
        Self::AbiSelection,
        Self::AbiVersion,
        Self::ArgumentTypes,
        Self::ResultType,
        Self::LayoutDescription,
        Self::ScalarRepresentation,
        Self::AggregateRepresentation,
        Self::Alignment,
        Self::Endianness,
        Self::CallingConvention,
        Self::TargetTriple,
        Self::TargetConstraints,
        Self::ProfileAvailability,
        Self::CapabilityRequirement,
        Self::OwnershipTransfer,
        Self::BorrowDuration,
        Self::Mutability,
        Self::ResourceLifetime,
        Self::ManagedBoundary,
        Self::AllocatorPair,
        Self::PointerProvenance,
        Self::SpanBounds,
        Self::Nullability,
        Self::Encoding,
        Self::CallbackSignature,
        Self::CallbackLifetime,
        Self::CallbackThreading,
        Self::ThreadAffinity,
        Self::Reentrancy,
        Self::Blocking,
        Self::Cancellation,
        Self::ErrorMapping,
        Self::FaultMapping,
        Self::UnwindPolicy,
        Self::OpaqueHandle,
        Self::SymbolVersion,
        Self::NameMangling,
        Self::DeclarationSourceSpan,
        Self::SemanticIdProjection,
        Self::ViewSeparation,
        Self::GrammarBoundary,
        Self::AstNodeBoundary,
        Self::HirNodeBoundary,
        Self::CheckedCoreBoundary,
        Self::VerifiedLowering,
        Self::DiagnosticCode,
        Self::BilingualDiagnostic,
        Self::UnicodeSourceSpans,
        Self::UnsupportedConstructs,
        Self::DeterministicOrdering,
        Self::CanonicalSchema,
        Self::SchemaVersion,
        Self::MigrationPolicy,
        Self::ProvenanceTcb,
        Self::SanitizerFuzzEvidence,
        Self::CrossTargetEvidence,
        Self::SeedCompatibility,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::DeclarationIdentity => 0,
            Self::DeclarationName => 1,
            Self::ForeignSymbol => 2,
            Self::AbiSelection => 3,
            Self::AbiVersion => 4,
            Self::ArgumentTypes => 5,
            Self::ResultType => 6,
            Self::LayoutDescription => 7,
            Self::ScalarRepresentation => 8,
            Self::AggregateRepresentation => 9,
            Self::Alignment => 10,
            Self::Endianness => 11,
            Self::CallingConvention => 12,
            Self::TargetTriple => 13,
            Self::TargetConstraints => 14,
            Self::ProfileAvailability => 15,
            Self::CapabilityRequirement => 16,
            Self::OwnershipTransfer => 17,
            Self::BorrowDuration => 18,
            Self::Mutability => 19,
            Self::ResourceLifetime => 20,
            Self::ManagedBoundary => 21,
            Self::AllocatorPair => 22,
            Self::PointerProvenance => 23,
            Self::SpanBounds => 24,
            Self::Nullability => 25,
            Self::Encoding => 26,
            Self::CallbackSignature => 27,
            Self::CallbackLifetime => 28,
            Self::CallbackThreading => 29,
            Self::ThreadAffinity => 30,
            Self::Reentrancy => 31,
            Self::Blocking => 32,
            Self::Cancellation => 33,
            Self::ErrorMapping => 34,
            Self::FaultMapping => 35,
            Self::UnwindPolicy => 36,
            Self::OpaqueHandle => 37,
            Self::SymbolVersion => 38,
            Self::NameMangling => 39,
            Self::DeclarationSourceSpan => 40,
            Self::SemanticIdProjection => 41,
            Self::ViewSeparation => 42,
            Self::GrammarBoundary => 43,
            Self::AstNodeBoundary => 44,
            Self::HirNodeBoundary => 45,
            Self::CheckedCoreBoundary => 46,
            Self::VerifiedLowering => 47,
            Self::DiagnosticCode => 48,
            Self::BilingualDiagnostic => 49,
            Self::UnicodeSourceSpans => 50,
            Self::UnsupportedConstructs => 51,
            Self::DeterministicOrdering => 52,
            Self::CanonicalSchema => 53,
            Self::SchemaVersion => 54,
            Self::MigrationPolicy => 55,
            Self::ProvenanceTcb => 56,
            Self::SanitizerFuzzEvidence => 57,
            Self::CrossTargetEvidence => 58,
            Self::SeedCompatibility => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FfiDeclarationBoundaryInventory {
    boundaries: Box<[PlannedFfiDeclarationBoundary]>,
}

impl FfiDeclarationBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedFfiDeclarationBoundary>,
    ) -> Result<Self, PlannedFfiDeclarationBoundary> {
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
        bytes.extend_from_slice(b"ling.ffi-declaration-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_ffi_declaration_boundaries_are_complete_and_ordered() {
    let inventory = FfiDeclarationBoundaryInventory::new(PlannedFfiDeclarationBoundary::ALL)
        .expect("planned FFI declaration boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedFfiDeclarationBoundary::ALL
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
fn ffi_declaration_evidence_is_order_independent_and_duplicate_checked() {
    let forward = FfiDeclarationBoundaryInventory::new(PlannedFfiDeclarationBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        FfiDeclarationBoundaryInventory::new(PlannedFfiDeclarationBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = FfiDeclarationBoundaryInventory::new([
        PlannedFfiDeclarationBoundary::DeclarationIdentity,
        PlannedFfiDeclarationBoundary::DeclarationIdentity,
    ])
    .expect_err("duplicate FFI declaration boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedFfiDeclarationBoundary::DeclarationIdentity
    );
}

#[test]
fn ffi_declaration_evidence_has_no_syntax_or_abi_authority() {
    let inventory = FfiDeclarationBoundaryInventory::new([
        PlannedFfiDeclarationBoundary::DeclarationIdentity,
        PlannedFfiDeclarationBoundary::AbiSelection,
        PlannedFfiDeclarationBoundary::ArgumentTypes,
        PlannedFfiDeclarationBoundary::OwnershipTransfer,
        PlannedFfiDeclarationBoundary::CallbackLifetime,
        PlannedFfiDeclarationBoundary::ErrorMapping,
        PlannedFfiDeclarationBoundary::CanonicalSchema,
        PlannedFfiDeclarationBoundary::SeedCompatibility,
    ])
    .expect("bounded FFI declaration evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.ffi-declaration-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
