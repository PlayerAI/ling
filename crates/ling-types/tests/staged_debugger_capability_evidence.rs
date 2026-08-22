use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDebuggerCapabilityBoundary {
    StageLaunch,
    StageContinue,
    StageBreakpoint,
    StageStepIn,
    StageStepOver,
    StageStepOut,
    StageStack,
    StageScope,
    StageVariable,
    StageConditionalBreakpoint,
    StageLogpoint,
    StageAttach,
    StageActorTaskView,
    CapabilityNegotiation,
    StopReason,
    BreakpointIdentity,
    BreakpointLocation,
    StepGranularity,
    SourceIdentity,
    BinaryIdentity,
    ProgramSnapshot,
    SourceMap,
    Utf8ByteSpan,
    StackFrame,
    ScopeIdentity,
    VariableIdentity,
    ResourceManagedView,
    OwnershipView,
    FaultMapping,
    ConditionInput,
    ConditionSandbox,
    LogpointSandbox,
    SideEffectRejection,
    ForeignCallRejection,
    CapabilityEscalationRejection,
    HostIoRejection,
    AllocationLimit,
    AttachAuthentication,
    SessionIsolation,
    Cancellation,
    Timeout,
    TargetRestriction,
    ProfileRestriction,
    VmNativeChoice,
    TaskLifecycle,
    ActorLifecycle,
    SuspensionReentry,
    MailboxObservation,
    SupervisionObservation,
    MalformedMessage,
    UnknownMessage,
    PositiveFixture,
    NegativeFixture,
    DeterministicEvidence,
    CrossEngineEvidence,
    BilingualDiagnostic,
    Unicode17,
    SemanticIdPreservation,
    HostOutputExclusion,
    PublicProtocolInventory,
}

impl PlannedDebuggerCapabilityBoundary {
    const ALL: [Self; 60] = [
        Self::StageLaunch,
        Self::StageContinue,
        Self::StageBreakpoint,
        Self::StageStepIn,
        Self::StageStepOver,
        Self::StageStepOut,
        Self::StageStack,
        Self::StageScope,
        Self::StageVariable,
        Self::StageConditionalBreakpoint,
        Self::StageLogpoint,
        Self::StageAttach,
        Self::StageActorTaskView,
        Self::CapabilityNegotiation,
        Self::StopReason,
        Self::BreakpointIdentity,
        Self::BreakpointLocation,
        Self::StepGranularity,
        Self::SourceIdentity,
        Self::BinaryIdentity,
        Self::ProgramSnapshot,
        Self::SourceMap,
        Self::Utf8ByteSpan,
        Self::StackFrame,
        Self::ScopeIdentity,
        Self::VariableIdentity,
        Self::ResourceManagedView,
        Self::OwnershipView,
        Self::FaultMapping,
        Self::ConditionInput,
        Self::ConditionSandbox,
        Self::LogpointSandbox,
        Self::SideEffectRejection,
        Self::ForeignCallRejection,
        Self::CapabilityEscalationRejection,
        Self::HostIoRejection,
        Self::AllocationLimit,
        Self::AttachAuthentication,
        Self::SessionIsolation,
        Self::Cancellation,
        Self::Timeout,
        Self::TargetRestriction,
        Self::ProfileRestriction,
        Self::VmNativeChoice,
        Self::TaskLifecycle,
        Self::ActorLifecycle,
        Self::SuspensionReentry,
        Self::MailboxObservation,
        Self::SupervisionObservation,
        Self::MalformedMessage,
        Self::UnknownMessage,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::DeterministicEvidence,
        Self::CrossEngineEvidence,
        Self::BilingualDiagnostic,
        Self::Unicode17,
        Self::SemanticIdPreservation,
        Self::HostOutputExclusion,
        Self::PublicProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::StageLaunch => 0,
            Self::StageContinue => 1,
            Self::StageBreakpoint => 2,
            Self::StageStepIn => 3,
            Self::StageStepOver => 4,
            Self::StageStepOut => 5,
            Self::StageStack => 6,
            Self::StageScope => 7,
            Self::StageVariable => 8,
            Self::StageConditionalBreakpoint => 9,
            Self::StageLogpoint => 10,
            Self::StageAttach => 11,
            Self::StageActorTaskView => 12,
            Self::CapabilityNegotiation => 13,
            Self::StopReason => 14,
            Self::BreakpointIdentity => 15,
            Self::BreakpointLocation => 16,
            Self::StepGranularity => 17,
            Self::SourceIdentity => 18,
            Self::BinaryIdentity => 19,
            Self::ProgramSnapshot => 20,
            Self::SourceMap => 21,
            Self::Utf8ByteSpan => 22,
            Self::StackFrame => 23,
            Self::ScopeIdentity => 24,
            Self::VariableIdentity => 25,
            Self::ResourceManagedView => 26,
            Self::OwnershipView => 27,
            Self::FaultMapping => 28,
            Self::ConditionInput => 29,
            Self::ConditionSandbox => 30,
            Self::LogpointSandbox => 31,
            Self::SideEffectRejection => 32,
            Self::ForeignCallRejection => 33,
            Self::CapabilityEscalationRejection => 34,
            Self::HostIoRejection => 35,
            Self::AllocationLimit => 36,
            Self::AttachAuthentication => 37,
            Self::SessionIsolation => 38,
            Self::Cancellation => 39,
            Self::Timeout => 40,
            Self::TargetRestriction => 41,
            Self::ProfileRestriction => 42,
            Self::VmNativeChoice => 43,
            Self::TaskLifecycle => 44,
            Self::ActorLifecycle => 45,
            Self::SuspensionReentry => 46,
            Self::MailboxObservation => 47,
            Self::SupervisionObservation => 48,
            Self::MalformedMessage => 49,
            Self::UnknownMessage => 50,
            Self::PositiveFixture => 51,
            Self::NegativeFixture => 52,
            Self::DeterministicEvidence => 53,
            Self::CrossEngineEvidence => 54,
            Self::BilingualDiagnostic => 55,
            Self::Unicode17 => 56,
            Self::SemanticIdPreservation => 57,
            Self::HostOutputExclusion => 58,
            Self::PublicProtocolInventory => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DebuggerCapabilityInventory {
    boundaries: Box<[PlannedDebuggerCapabilityBoundary]>,
}

impl DebuggerCapabilityInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDebuggerCapabilityBoundary>,
    ) -> Result<Self, PlannedDebuggerCapabilityBoundary> {
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
        bytes.extend_from_slice(b"ling.staged-debugger-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_debugger_capabilities_are_complete_and_ordered() {
    let inventory = DebuggerCapabilityInventory::new(PlannedDebuggerCapabilityBoundary::ALL)
        .expect("planned debugger capability boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDebuggerCapabilityBoundary::ALL
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
fn staged_debugger_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DebuggerCapabilityInventory::new(PlannedDebuggerCapabilityBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        DebuggerCapabilityInventory::new(PlannedDebuggerCapabilityBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DebuggerCapabilityInventory::new([
        PlannedDebuggerCapabilityBoundary::StageLaunch,
        PlannedDebuggerCapabilityBoundary::StageLaunch,
    ])
    .expect_err("duplicate debugger capability boundary must be rejected");
    assert_eq!(duplicate, PlannedDebuggerCapabilityBoundary::StageLaunch);
}

#[test]
fn staged_debugger_evidence_has_no_capability_authority() {
    let inventory = DebuggerCapabilityInventory::new([
        PlannedDebuggerCapabilityBoundary::StageLaunch,
        PlannedDebuggerCapabilityBoundary::StageBreakpoint,
        PlannedDebuggerCapabilityBoundary::ConditionSandbox,
        PlannedDebuggerCapabilityBoundary::AttachAuthentication,
        PlannedDebuggerCapabilityBoundary::TaskLifecycle,
        PlannedDebuggerCapabilityBoundary::BilingualDiagnostic,
        PlannedDebuggerCapabilityBoundary::Unicode17,
        PlannedDebuggerCapabilityBoundary::PublicProtocolInventory,
    ])
    .expect("bounded staged debugger evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.staged-debugger-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
