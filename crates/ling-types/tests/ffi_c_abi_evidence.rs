use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedCAbiBoundary {
    CAbiVersion,
    TargetTriple,
    CallingConvention,
    ScalarBoolRepresentation,
    ScalarIntegerWidths,
    ScalarFloatRepresentation,
    Endianness,
    Alignment,
    RecordLayout,
    RecordPacking,
    UnionLayout,
    BitfieldRejection,
    FlexibleArrayRejection,
    VariadicRejection,
    SymbolNaming,
    SymbolVersion,
    HeaderImport,
    DeclarationIdentity,
    ArgumentLayout,
    ResultLayout,
    SpanPointer,
    SpanLength,
    SpanNullability,
    SpanOverflow,
    SpanProvenance,
    SpanMutability,
    Encoding,
    CallbackSignature,
    CallbackCalling,
    CallbackLifetime,
    CallbackThread,
    CallbackReentrancy,
    CallbackCancellation,
    OpaqueHandleIdentity,
    HandleThreadAffinity,
    AllocatorPair,
    AllocatorProvenance,
    AllocatorDeallocation,
    OwnershipTransfer,
    BorrowDuration,
    ResourceDrop,
    ManagedBoundary,
    ErrorCodeMapping,
    FaultMapping,
    UnwindBoundary,
    Blocking,
    Capability,
    Profile,
    TargetRejection,
    UnsupportedLayout,
    DeclarationSourceSpan,
    DiagnosticBilingual,
    UnicodeSourceSpan,
    CanonicalSchema,
    SchemaVersion,
    Migration,
    ProvenanceTcb,
    SanitizerFuzz,
    CrossTargetDifferential,
    SeedCompatibility,
}

impl PlannedCAbiBoundary {
    const ALL: [Self; 60] = [
        Self::CAbiVersion,
        Self::TargetTriple,
        Self::CallingConvention,
        Self::ScalarBoolRepresentation,
        Self::ScalarIntegerWidths,
        Self::ScalarFloatRepresentation,
        Self::Endianness,
        Self::Alignment,
        Self::RecordLayout,
        Self::RecordPacking,
        Self::UnionLayout,
        Self::BitfieldRejection,
        Self::FlexibleArrayRejection,
        Self::VariadicRejection,
        Self::SymbolNaming,
        Self::SymbolVersion,
        Self::HeaderImport,
        Self::DeclarationIdentity,
        Self::ArgumentLayout,
        Self::ResultLayout,
        Self::SpanPointer,
        Self::SpanLength,
        Self::SpanNullability,
        Self::SpanOverflow,
        Self::SpanProvenance,
        Self::SpanMutability,
        Self::Encoding,
        Self::CallbackSignature,
        Self::CallbackCalling,
        Self::CallbackLifetime,
        Self::CallbackThread,
        Self::CallbackReentrancy,
        Self::CallbackCancellation,
        Self::OpaqueHandleIdentity,
        Self::HandleThreadAffinity,
        Self::AllocatorPair,
        Self::AllocatorProvenance,
        Self::AllocatorDeallocation,
        Self::OwnershipTransfer,
        Self::BorrowDuration,
        Self::ResourceDrop,
        Self::ManagedBoundary,
        Self::ErrorCodeMapping,
        Self::FaultMapping,
        Self::UnwindBoundary,
        Self::Blocking,
        Self::Capability,
        Self::Profile,
        Self::TargetRejection,
        Self::UnsupportedLayout,
        Self::DeclarationSourceSpan,
        Self::DiagnosticBilingual,
        Self::UnicodeSourceSpan,
        Self::CanonicalSchema,
        Self::SchemaVersion,
        Self::Migration,
        Self::ProvenanceTcb,
        Self::SanitizerFuzz,
        Self::CrossTargetDifferential,
        Self::SeedCompatibility,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::CAbiVersion => 0,
            Self::TargetTriple => 1,
            Self::CallingConvention => 2,
            Self::ScalarBoolRepresentation => 3,
            Self::ScalarIntegerWidths => 4,
            Self::ScalarFloatRepresentation => 5,
            Self::Endianness => 6,
            Self::Alignment => 7,
            Self::RecordLayout => 8,
            Self::RecordPacking => 9,
            Self::UnionLayout => 10,
            Self::BitfieldRejection => 11,
            Self::FlexibleArrayRejection => 12,
            Self::VariadicRejection => 13,
            Self::SymbolNaming => 14,
            Self::SymbolVersion => 15,
            Self::HeaderImport => 16,
            Self::DeclarationIdentity => 17,
            Self::ArgumentLayout => 18,
            Self::ResultLayout => 19,
            Self::SpanPointer => 20,
            Self::SpanLength => 21,
            Self::SpanNullability => 22,
            Self::SpanOverflow => 23,
            Self::SpanProvenance => 24,
            Self::SpanMutability => 25,
            Self::Encoding => 26,
            Self::CallbackSignature => 27,
            Self::CallbackCalling => 28,
            Self::CallbackLifetime => 29,
            Self::CallbackThread => 30,
            Self::CallbackReentrancy => 31,
            Self::CallbackCancellation => 32,
            Self::OpaqueHandleIdentity => 33,
            Self::HandleThreadAffinity => 34,
            Self::AllocatorPair => 35,
            Self::AllocatorProvenance => 36,
            Self::AllocatorDeallocation => 37,
            Self::OwnershipTransfer => 38,
            Self::BorrowDuration => 39,
            Self::ResourceDrop => 40,
            Self::ManagedBoundary => 41,
            Self::ErrorCodeMapping => 42,
            Self::FaultMapping => 43,
            Self::UnwindBoundary => 44,
            Self::Blocking => 45,
            Self::Capability => 46,
            Self::Profile => 47,
            Self::TargetRejection => 48,
            Self::UnsupportedLayout => 49,
            Self::DeclarationSourceSpan => 50,
            Self::DiagnosticBilingual => 51,
            Self::UnicodeSourceSpan => 52,
            Self::CanonicalSchema => 53,
            Self::SchemaVersion => 54,
            Self::Migration => 55,
            Self::ProvenanceTcb => 56,
            Self::SanitizerFuzz => 57,
            Self::CrossTargetDifferential => 58,
            Self::SeedCompatibility => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CAbiBoundaryInventory {
    boundaries: Box<[PlannedCAbiBoundary]>,
}

impl CAbiBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCAbiBoundary>,
    ) -> Result<Self, PlannedCAbiBoundary> {
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
        bytes.extend_from_slice(b"ling.ffi-c-abi-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_c_abi_boundaries_are_complete_and_ordered() {
    let inventory = CAbiBoundaryInventory::new(PlannedCAbiBoundary::ALL)
        .expect("planned C ABI boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedCAbiBoundary::ALL);
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
fn c_abi_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CAbiBoundaryInventory::new(PlannedCAbiBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = CAbiBoundaryInventory::new(PlannedCAbiBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CAbiBoundaryInventory::new([
        PlannedCAbiBoundary::CAbiVersion,
        PlannedCAbiBoundary::CAbiVersion,
    ])
    .expect_err("duplicate C ABI boundary must be rejected");
    assert_eq!(duplicate, PlannedCAbiBoundary::CAbiVersion);
}

#[test]
fn c_abi_evidence_has_no_layout_or_linker_authority() {
    let inventory = CAbiBoundaryInventory::new([
        PlannedCAbiBoundary::CAbiVersion,
        PlannedCAbiBoundary::ScalarIntegerWidths,
        PlannedCAbiBoundary::RecordLayout,
        PlannedCAbiBoundary::SpanPointer,
        PlannedCAbiBoundary::CallbackLifetime,
        PlannedCAbiBoundary::AllocatorPair,
        PlannedCAbiBoundary::ErrorCodeMapping,
        PlannedCAbiBoundary::SeedCompatibility,
    ])
    .expect("bounded C ABI evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.ffi-c-abi-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
