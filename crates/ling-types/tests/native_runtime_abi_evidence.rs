use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNativeRuntimeAbiBoundary {
    ValuePassing,
    PrimitiveLayout,
    AggregateLayout,
    RecordTupleLayout,
    AdtTag,
    AdtPayload,
    ClosureEnvironment,
    ClosureCallingConvention,
    StringRepresentation,
    TextEncoding,
    BytesRepresentation,
    FaultRepresentation,
    ResultRepresentation,
    FaultUnwind,
    Cancellation,
    Shutdown,
    ThreadAttachment,
    Reentry,
    GcHandleIdentity,
    GcRoot,
    GcBarrier,
    GcPin,
    ResourceOwnership,
    ResourceDrop,
    BorrowRegion,
    FfiTransfer,
    ForeignOwnership,
    TaskCall,
    ActorCall,
    MailboxBoundary,
    TurnBoundary,
    LayoutAlignment,
    Endianness,
    TargetException,
    CallingConvention,
    RuntimeLibrary,
    AbiVersion,
    CompilerRuntimeCompatibility,
    FeatureNegotiation,
    Migration,
    SymbolMangling,
    DebugSourceMapping,
    SchemaOwnership,
    DeterministicMetadata,
    BilingualDiagnostic,
    UnsupportedAbiForm,
    UnicodeSourceSpan,
    SecurityTcb,
    OfflineBuildInputs,
    InterpreterVmNativeDifferential,
    CrossTargetCompatibility,
    SanitizerEvidence,
    HostFailureSeparation,
    AllocationOrderingExclusion,
    AddressTimingExclusion,
    MapOrderExclusion,
    PublicAbiExclusion,
    SeedCompatibility,
}

impl PlannedNativeRuntimeAbiBoundary {
    const ALL: [Self; 58] = [
        Self::ValuePassing,
        Self::PrimitiveLayout,
        Self::AggregateLayout,
        Self::RecordTupleLayout,
        Self::AdtTag,
        Self::AdtPayload,
        Self::ClosureEnvironment,
        Self::ClosureCallingConvention,
        Self::StringRepresentation,
        Self::TextEncoding,
        Self::BytesRepresentation,
        Self::FaultRepresentation,
        Self::ResultRepresentation,
        Self::FaultUnwind,
        Self::Cancellation,
        Self::Shutdown,
        Self::ThreadAttachment,
        Self::Reentry,
        Self::GcHandleIdentity,
        Self::GcRoot,
        Self::GcBarrier,
        Self::GcPin,
        Self::ResourceOwnership,
        Self::ResourceDrop,
        Self::BorrowRegion,
        Self::FfiTransfer,
        Self::ForeignOwnership,
        Self::TaskCall,
        Self::ActorCall,
        Self::MailboxBoundary,
        Self::TurnBoundary,
        Self::LayoutAlignment,
        Self::Endianness,
        Self::TargetException,
        Self::CallingConvention,
        Self::RuntimeLibrary,
        Self::AbiVersion,
        Self::CompilerRuntimeCompatibility,
        Self::FeatureNegotiation,
        Self::Migration,
        Self::SymbolMangling,
        Self::DebugSourceMapping,
        Self::SchemaOwnership,
        Self::DeterministicMetadata,
        Self::BilingualDiagnostic,
        Self::UnsupportedAbiForm,
        Self::UnicodeSourceSpan,
        Self::SecurityTcb,
        Self::OfflineBuildInputs,
        Self::InterpreterVmNativeDifferential,
        Self::CrossTargetCompatibility,
        Self::SanitizerEvidence,
        Self::HostFailureSeparation,
        Self::AllocationOrderingExclusion,
        Self::AddressTimingExclusion,
        Self::MapOrderExclusion,
        Self::PublicAbiExclusion,
        Self::SeedCompatibility,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ValuePassing => 0,
            Self::PrimitiveLayout => 1,
            Self::AggregateLayout => 2,
            Self::RecordTupleLayout => 3,
            Self::AdtTag => 4,
            Self::AdtPayload => 5,
            Self::ClosureEnvironment => 6,
            Self::ClosureCallingConvention => 7,
            Self::StringRepresentation => 8,
            Self::TextEncoding => 9,
            Self::BytesRepresentation => 10,
            Self::FaultRepresentation => 11,
            Self::ResultRepresentation => 12,
            Self::FaultUnwind => 13,
            Self::Cancellation => 14,
            Self::Shutdown => 15,
            Self::ThreadAttachment => 16,
            Self::Reentry => 17,
            Self::GcHandleIdentity => 18,
            Self::GcRoot => 19,
            Self::GcBarrier => 20,
            Self::GcPin => 21,
            Self::ResourceOwnership => 22,
            Self::ResourceDrop => 23,
            Self::BorrowRegion => 24,
            Self::FfiTransfer => 25,
            Self::ForeignOwnership => 26,
            Self::TaskCall => 27,
            Self::ActorCall => 28,
            Self::MailboxBoundary => 29,
            Self::TurnBoundary => 30,
            Self::LayoutAlignment => 31,
            Self::Endianness => 32,
            Self::TargetException => 33,
            Self::CallingConvention => 34,
            Self::RuntimeLibrary => 35,
            Self::AbiVersion => 36,
            Self::CompilerRuntimeCompatibility => 37,
            Self::FeatureNegotiation => 38,
            Self::Migration => 39,
            Self::SymbolMangling => 40,
            Self::DebugSourceMapping => 41,
            Self::SchemaOwnership => 42,
            Self::DeterministicMetadata => 43,
            Self::BilingualDiagnostic => 44,
            Self::UnsupportedAbiForm => 45,
            Self::UnicodeSourceSpan => 46,
            Self::SecurityTcb => 47,
            Self::OfflineBuildInputs => 48,
            Self::InterpreterVmNativeDifferential => 49,
            Self::CrossTargetCompatibility => 50,
            Self::SanitizerEvidence => 51,
            Self::HostFailureSeparation => 52,
            Self::AllocationOrderingExclusion => 53,
            Self::AddressTimingExclusion => 54,
            Self::MapOrderExclusion => 55,
            Self::PublicAbiExclusion => 56,
            Self::SeedCompatibility => 57,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NativeRuntimeAbiBoundaryInventory {
    boundaries: Box<[PlannedNativeRuntimeAbiBoundary]>,
}

impl NativeRuntimeAbiBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNativeRuntimeAbiBoundary>,
    ) -> Result<Self, PlannedNativeRuntimeAbiBoundary> {
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
        bytes.extend_from_slice(b"ling.native-runtime-abi-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_native_runtime_abi_boundaries_are_complete_and_ordered() {
    let inventory = NativeRuntimeAbiBoundaryInventory::new(PlannedNativeRuntimeAbiBoundary::ALL)
        .expect("planned Native runtime ABI boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNativeRuntimeAbiBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..58).collect::<Vec<_>>()
    );
}

#[test]
fn native_runtime_abi_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NativeRuntimeAbiBoundaryInventory::new(PlannedNativeRuntimeAbiBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = NativeRuntimeAbiBoundaryInventory::new(
        PlannedNativeRuntimeAbiBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NativeRuntimeAbiBoundaryInventory::new([
        PlannedNativeRuntimeAbiBoundary::ValuePassing,
        PlannedNativeRuntimeAbiBoundary::ValuePassing,
    ])
    .expect_err("duplicate Native runtime ABI boundary must be rejected");
    assert_eq!(duplicate, PlannedNativeRuntimeAbiBoundary::ValuePassing);
}

#[test]
fn native_runtime_abi_evidence_has_no_layout_or_public_abi_authority() {
    let inventory = NativeRuntimeAbiBoundaryInventory::new([
        PlannedNativeRuntimeAbiBoundary::ValuePassing,
        PlannedNativeRuntimeAbiBoundary::AdtTag,
        PlannedNativeRuntimeAbiBoundary::ClosureEnvironment,
        PlannedNativeRuntimeAbiBoundary::FaultRepresentation,
        PlannedNativeRuntimeAbiBoundary::GcHandleIdentity,
        PlannedNativeRuntimeAbiBoundary::ResourceDrop,
        PlannedNativeRuntimeAbiBoundary::TaskCall,
        PlannedNativeRuntimeAbiBoundary::StringRepresentation,
    ])
    .expect("bounded Native runtime ABI evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.native-runtime-abi-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
