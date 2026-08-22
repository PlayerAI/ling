use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDapBoundary {
    ProtocolSchema,
    StdioFraming,
    MessageLimit,
    InitializeRequest,
    CapabilityNegotiation,
    LaunchRequest,
    AttachRequest,
    DisconnectRequest,
    CancelRequest,
    SessionIdentity,
    SessionIsolation,
    TransportSecurity,
    UnknownMessage,
    MalformedMessage,
    OversizedMessage,
    VersionCompatibility,
    MigrationRule,
    ReaderBoundary,
    WriterBoundary,
    SourceMap,
    Utf8ByteSpan,
    LspPosition,
    ProgramSnapshot,
    BinaryIdentity,
    BreakpointLocation,
    BreakpointCondition,
    Logpoint,
    ContinueBoundary,
    StepBoundary,
    PauseBoundary,
    StackFrame,
    Scope,
    Variable,
    MutableVisibility,
    FaultCategory,
    ExceptionMapping,
    ResourceManagedView,
    OwnershipView,
    ActorTaskView,
    CapabilityRestriction,
    ProfileRestriction,
    TargetRestriction,
    VmDebugMetadata,
    NativeDebugMetadata,
    TypedCoreInput,
    HostPathExclusion,
    AddressExclusion,
    AllocationExclusion,
    DebugStringExclusion,
    Cancellation,
    Timeout,
    ConcurrentClient,
    Redaction,
    BilingualDiagnostic,
    Unicode17,
    SemanticIdPreservation,
    PositiveFixture,
    NegativeFixture,
    DeterministicEvidence,
    PublicProtocolInventory,
}

impl PlannedDapBoundary {
    const ALL: [Self; 60] = [
        Self::ProtocolSchema,
        Self::StdioFraming,
        Self::MessageLimit,
        Self::InitializeRequest,
        Self::CapabilityNegotiation,
        Self::LaunchRequest,
        Self::AttachRequest,
        Self::DisconnectRequest,
        Self::CancelRequest,
        Self::SessionIdentity,
        Self::SessionIsolation,
        Self::TransportSecurity,
        Self::UnknownMessage,
        Self::MalformedMessage,
        Self::OversizedMessage,
        Self::VersionCompatibility,
        Self::MigrationRule,
        Self::ReaderBoundary,
        Self::WriterBoundary,
        Self::SourceMap,
        Self::Utf8ByteSpan,
        Self::LspPosition,
        Self::ProgramSnapshot,
        Self::BinaryIdentity,
        Self::BreakpointLocation,
        Self::BreakpointCondition,
        Self::Logpoint,
        Self::ContinueBoundary,
        Self::StepBoundary,
        Self::PauseBoundary,
        Self::StackFrame,
        Self::Scope,
        Self::Variable,
        Self::MutableVisibility,
        Self::FaultCategory,
        Self::ExceptionMapping,
        Self::ResourceManagedView,
        Self::OwnershipView,
        Self::ActorTaskView,
        Self::CapabilityRestriction,
        Self::ProfileRestriction,
        Self::TargetRestriction,
        Self::VmDebugMetadata,
        Self::NativeDebugMetadata,
        Self::TypedCoreInput,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::AllocationExclusion,
        Self::DebugStringExclusion,
        Self::Cancellation,
        Self::Timeout,
        Self::ConcurrentClient,
        Self::Redaction,
        Self::BilingualDiagnostic,
        Self::Unicode17,
        Self::SemanticIdPreservation,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::DeterministicEvidence,
        Self::PublicProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ProtocolSchema => 0,
            Self::StdioFraming => 1,
            Self::MessageLimit => 2,
            Self::InitializeRequest => 3,
            Self::CapabilityNegotiation => 4,
            Self::LaunchRequest => 5,
            Self::AttachRequest => 6,
            Self::DisconnectRequest => 7,
            Self::CancelRequest => 8,
            Self::SessionIdentity => 9,
            Self::SessionIsolation => 10,
            Self::TransportSecurity => 11,
            Self::UnknownMessage => 12,
            Self::MalformedMessage => 13,
            Self::OversizedMessage => 14,
            Self::VersionCompatibility => 15,
            Self::MigrationRule => 16,
            Self::ReaderBoundary => 17,
            Self::WriterBoundary => 18,
            Self::SourceMap => 19,
            Self::Utf8ByteSpan => 20,
            Self::LspPosition => 21,
            Self::ProgramSnapshot => 22,
            Self::BinaryIdentity => 23,
            Self::BreakpointLocation => 24,
            Self::BreakpointCondition => 25,
            Self::Logpoint => 26,
            Self::ContinueBoundary => 27,
            Self::StepBoundary => 28,
            Self::PauseBoundary => 29,
            Self::StackFrame => 30,
            Self::Scope => 31,
            Self::Variable => 32,
            Self::MutableVisibility => 33,
            Self::FaultCategory => 34,
            Self::ExceptionMapping => 35,
            Self::ResourceManagedView => 36,
            Self::OwnershipView => 37,
            Self::ActorTaskView => 38,
            Self::CapabilityRestriction => 39,
            Self::ProfileRestriction => 40,
            Self::TargetRestriction => 41,
            Self::VmDebugMetadata => 42,
            Self::NativeDebugMetadata => 43,
            Self::TypedCoreInput => 44,
            Self::HostPathExclusion => 45,
            Self::AddressExclusion => 46,
            Self::AllocationExclusion => 47,
            Self::DebugStringExclusion => 48,
            Self::Cancellation => 49,
            Self::Timeout => 50,
            Self::ConcurrentClient => 51,
            Self::Redaction => 52,
            Self::BilingualDiagnostic => 53,
            Self::Unicode17 => 54,
            Self::SemanticIdPreservation => 55,
            Self::PositiveFixture => 56,
            Self::NegativeFixture => 57,
            Self::DeterministicEvidence => 58,
            Self::PublicProtocolInventory => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DapBoundaryInventory {
    boundaries: Box<[PlannedDapBoundary]>,
}

impl DapBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDapBoundary>,
    ) -> Result<Self, PlannedDapBoundary> {
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
        bytes.extend_from_slice(b"ling.dap-debugger-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_dap_boundaries_are_complete_and_ordered() {
    let inventory = DapBoundaryInventory::new(PlannedDapBoundary::ALL)
        .expect("planned DAP boundaries have no duplicates");
    assert_eq!(inventory.boundaries.as_ref(), &PlannedDapBoundary::ALL);
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
fn dap_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DapBoundaryInventory::new(PlannedDapBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = DapBoundaryInventory::new(PlannedDapBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DapBoundaryInventory::new([
        PlannedDapBoundary::ProtocolSchema,
        PlannedDapBoundary::ProtocolSchema,
    ])
    .expect_err("duplicate DAP boundary must be rejected");
    assert_eq!(duplicate, PlannedDapBoundary::ProtocolSchema);
}

#[test]
fn dap_evidence_has_no_debugger_protocol_authority() {
    let inventory = DapBoundaryInventory::new([
        PlannedDapBoundary::ProtocolSchema,
        PlannedDapBoundary::StdioFraming,
        PlannedDapBoundary::SourceMap,
        PlannedDapBoundary::FaultCategory,
        PlannedDapBoundary::TypedCoreInput,
        PlannedDapBoundary::BilingualDiagnostic,
        PlannedDapBoundary::Unicode17,
        PlannedDapBoundary::PublicProtocolInventory,
    ])
    .expect("bounded DAP evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.dap-debugger-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
