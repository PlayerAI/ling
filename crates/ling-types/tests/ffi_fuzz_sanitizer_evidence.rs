use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedFfiFuzzBoundary {
    FuzzInputCorpus,
    FuzzHarness,
    FuzzTarget,
    DeclarationParser,
    AbiReader,
    ShimGenerator,
    TargetPackageLoader,
    MalformedMetadata,
    UnknownFields,
    TruncatedInput,
    OversizedInput,
    IntegerOverflow,
    LengthOverflow,
    NullPointer,
    DanglingPointer,
    OutOfBounds,
    Misalignment,
    InvalidEncoding,
    InvalidLayout,
    VariadicReject,
    BitfieldReject,
    FlexibleArrayReject,
    CallbackLifetime,
    CallbackReentry,
    ThreadRace,
    AllocatorMismatch,
    DoubleFree,
    UseAfterFree,
    OwnershipTransfer,
    BorrowLifetime,
    FaultUnwind,
    Cancellation,
    CapabilityDenial,
    ProfileTargetMismatch,
    LinkerFailure,
    ArtifactTamper,
    ProvenanceMismatch,
    LicenseTcb,
    DeterministicSeeds,
    OrderIndependence,
    ReproducibleRun,
    OfflineRun,
    CrossTargetRun,
    CrossCompilerRun,
    SanitizerAddress,
    SanitizerUndefined,
    SanitizerThread,
    SanitizerMemory,
    LeakDetection,
    CoverageThreshold,
    CrashMinimization,
    CorpusDeduplication,
    TimeoutBound,
    MemoryBound,
    DiagnosticBilingual,
    UnicodeSpans,
    SeedCompatibility,
    PublicProtocolExclusion,
    SemanticIdSeparation,
    HostOutputExclusion,
}

impl PlannedFfiFuzzBoundary {
    const ALL: [Self; 60] = [
        Self::FuzzInputCorpus,
        Self::FuzzHarness,
        Self::FuzzTarget,
        Self::DeclarationParser,
        Self::AbiReader,
        Self::ShimGenerator,
        Self::TargetPackageLoader,
        Self::MalformedMetadata,
        Self::UnknownFields,
        Self::TruncatedInput,
        Self::OversizedInput,
        Self::IntegerOverflow,
        Self::LengthOverflow,
        Self::NullPointer,
        Self::DanglingPointer,
        Self::OutOfBounds,
        Self::Misalignment,
        Self::InvalidEncoding,
        Self::InvalidLayout,
        Self::VariadicReject,
        Self::BitfieldReject,
        Self::FlexibleArrayReject,
        Self::CallbackLifetime,
        Self::CallbackReentry,
        Self::ThreadRace,
        Self::AllocatorMismatch,
        Self::DoubleFree,
        Self::UseAfterFree,
        Self::OwnershipTransfer,
        Self::BorrowLifetime,
        Self::FaultUnwind,
        Self::Cancellation,
        Self::CapabilityDenial,
        Self::ProfileTargetMismatch,
        Self::LinkerFailure,
        Self::ArtifactTamper,
        Self::ProvenanceMismatch,
        Self::LicenseTcb,
        Self::DeterministicSeeds,
        Self::OrderIndependence,
        Self::ReproducibleRun,
        Self::OfflineRun,
        Self::CrossTargetRun,
        Self::CrossCompilerRun,
        Self::SanitizerAddress,
        Self::SanitizerUndefined,
        Self::SanitizerThread,
        Self::SanitizerMemory,
        Self::LeakDetection,
        Self::CoverageThreshold,
        Self::CrashMinimization,
        Self::CorpusDeduplication,
        Self::TimeoutBound,
        Self::MemoryBound,
        Self::DiagnosticBilingual,
        Self::UnicodeSpans,
        Self::SeedCompatibility,
        Self::PublicProtocolExclusion,
        Self::SemanticIdSeparation,
        Self::HostOutputExclusion,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::FuzzInputCorpus => 0,
            Self::FuzzHarness => 1,
            Self::FuzzTarget => 2,
            Self::DeclarationParser => 3,
            Self::AbiReader => 4,
            Self::ShimGenerator => 5,
            Self::TargetPackageLoader => 6,
            Self::MalformedMetadata => 7,
            Self::UnknownFields => 8,
            Self::TruncatedInput => 9,
            Self::OversizedInput => 10,
            Self::IntegerOverflow => 11,
            Self::LengthOverflow => 12,
            Self::NullPointer => 13,
            Self::DanglingPointer => 14,
            Self::OutOfBounds => 15,
            Self::Misalignment => 16,
            Self::InvalidEncoding => 17,
            Self::InvalidLayout => 18,
            Self::VariadicReject => 19,
            Self::BitfieldReject => 20,
            Self::FlexibleArrayReject => 21,
            Self::CallbackLifetime => 22,
            Self::CallbackReentry => 23,
            Self::ThreadRace => 24,
            Self::AllocatorMismatch => 25,
            Self::DoubleFree => 26,
            Self::UseAfterFree => 27,
            Self::OwnershipTransfer => 28,
            Self::BorrowLifetime => 29,
            Self::FaultUnwind => 30,
            Self::Cancellation => 31,
            Self::CapabilityDenial => 32,
            Self::ProfileTargetMismatch => 33,
            Self::LinkerFailure => 34,
            Self::ArtifactTamper => 35,
            Self::ProvenanceMismatch => 36,
            Self::LicenseTcb => 37,
            Self::DeterministicSeeds => 38,
            Self::OrderIndependence => 39,
            Self::ReproducibleRun => 40,
            Self::OfflineRun => 41,
            Self::CrossTargetRun => 42,
            Self::CrossCompilerRun => 43,
            Self::SanitizerAddress => 44,
            Self::SanitizerUndefined => 45,
            Self::SanitizerThread => 46,
            Self::SanitizerMemory => 47,
            Self::LeakDetection => 48,
            Self::CoverageThreshold => 49,
            Self::CrashMinimization => 50,
            Self::CorpusDeduplication => 51,
            Self::TimeoutBound => 52,
            Self::MemoryBound => 53,
            Self::DiagnosticBilingual => 54,
            Self::UnicodeSpans => 55,
            Self::SeedCompatibility => 56,
            Self::PublicProtocolExclusion => 57,
            Self::SemanticIdSeparation => 58,
            Self::HostOutputExclusion => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FfiFuzzBoundaryInventory {
    boundaries: Box<[PlannedFfiFuzzBoundary]>,
}

impl FfiFuzzBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedFfiFuzzBoundary>,
    ) -> Result<Self, PlannedFfiFuzzBoundary> {
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
        bytes.extend_from_slice(b"ling.ffi-fuzz-sanitizer-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_ffi_fuzz_boundaries_are_complete_and_ordered() {
    let inventory = FfiFuzzBoundaryInventory::new(PlannedFfiFuzzBoundary::ALL)
        .expect("planned FFI fuzz and sanitizer boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedFfiFuzzBoundary::ALL);
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
fn ffi_fuzz_evidence_is_order_independent_and_duplicate_checked() {
    let forward = FfiFuzzBoundaryInventory::new(PlannedFfiFuzzBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = FfiFuzzBoundaryInventory::new(PlannedFfiFuzzBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = FfiFuzzBoundaryInventory::new([
        PlannedFfiFuzzBoundary::FuzzInputCorpus,
        PlannedFfiFuzzBoundary::FuzzInputCorpus,
    ])
    .expect_err("duplicate FFI fuzz boundary must be rejected");
    assert_eq!(duplicate, PlannedFfiFuzzBoundary::FuzzInputCorpus);
}

#[test]
fn ffi_fuzz_evidence_has_no_fuzz_or_sanitizer_authority() {
    let inventory = FfiFuzzBoundaryInventory::new([
        PlannedFfiFuzzBoundary::FuzzInputCorpus,
        PlannedFfiFuzzBoundary::MalformedMetadata,
        PlannedFfiFuzzBoundary::OutOfBounds,
        PlannedFfiFuzzBoundary::CallbackLifetime,
        PlannedFfiFuzzBoundary::SanitizerAddress,
        PlannedFfiFuzzBoundary::DeterministicSeeds,
        PlannedFfiFuzzBoundary::DiagnosticBilingual,
        PlannedFfiFuzzBoundary::SeedCompatibility,
    ])
    .expect("bounded FFI fuzz and sanitizer evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.ffi-fuzz-sanitizer-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
