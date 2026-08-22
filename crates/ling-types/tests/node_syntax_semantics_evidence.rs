use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedNodeSyntaxSemanticsBoundary {
    NodeSyntaxSemantics,
    NodeDeclaration,
    InputPort,
    OutputPort,
    State,
    EveryPeriod,
    Deadline,
    DurationUnit,
    LogicalTick,
    InputSamplingOutputCommit,
    Initialization,
    Reinitialization,
    Absence,
    Presence,
    ClockDomain,
    ClockConversion,
    EvaluationOrder,
    Composition,
    Feedback,
    EffectRelation,
    CapabilityRelation,
    TaskActorRelation,
    KernelDeviceRelation,
    FfiRelation,
    PersistentState,
    StateVisibility,
    TickMiss,
    Overrun,
    FaultRecovery,
    Cancellation,
    Fallback,
    Scheduler,
    VirtualClock,
    TargetWcet,
    CacheBusAssumption,
    TargetCompiler,
    EvidenceIdentity,
    CriticalProfile,
    Allocation,
    Gc,
    Recursion,
    Mailbox,
    Ownership,
    UnknownTiming,
    UnknownResource,
    DiagnosticCode,
    DiagnosticFacts,
    SemanticId,
    SourceSpan,
    PositiveFixture,
    NegativeFixture,
    VirtualClockFixture,
    StateRestartFixture,
    AbsencePresenceFixture,
    CompositionFixture,
    DeadlineOverrunFixture,
    TargetMigrationFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedNodeSyntaxSemanticsBoundary {
    const ALL: [Self; 60] = [
        Self::NodeSyntaxSemantics,
        Self::NodeDeclaration,
        Self::InputPort,
        Self::OutputPort,
        Self::State,
        Self::EveryPeriod,
        Self::Deadline,
        Self::DurationUnit,
        Self::LogicalTick,
        Self::InputSamplingOutputCommit,
        Self::Initialization,
        Self::Reinitialization,
        Self::Absence,
        Self::Presence,
        Self::ClockDomain,
        Self::ClockConversion,
        Self::EvaluationOrder,
        Self::Composition,
        Self::Feedback,
        Self::EffectRelation,
        Self::CapabilityRelation,
        Self::TaskActorRelation,
        Self::KernelDeviceRelation,
        Self::FfiRelation,
        Self::PersistentState,
        Self::StateVisibility,
        Self::TickMiss,
        Self::Overrun,
        Self::FaultRecovery,
        Self::Cancellation,
        Self::Fallback,
        Self::Scheduler,
        Self::VirtualClock,
        Self::TargetWcet,
        Self::CacheBusAssumption,
        Self::TargetCompiler,
        Self::EvidenceIdentity,
        Self::CriticalProfile,
        Self::Allocation,
        Self::Gc,
        Self::Recursion,
        Self::Mailbox,
        Self::Ownership,
        Self::UnknownTiming,
        Self::UnknownResource,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::SemanticId,
        Self::SourceSpan,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::VirtualClockFixture,
        Self::StateRestartFixture,
        Self::AbsencePresenceFixture,
        Self::CompositionFixture,
        Self::DeadlineOverrunFixture,
        Self::TargetMigrationFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeSyntaxSemanticsInventory {
    boundaries: Box<[PlannedNodeSyntaxSemanticsBoundary]>,
}

impl NodeSyntaxSemanticsInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedNodeSyntaxSemanticsBoundary>,
    ) -> Result<Self, PlannedNodeSyntaxSemanticsBoundary> {
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
        let mut bytes = b"ling.node-syntax-semantics-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_node_syntax_semantics_boundaries_are_complete_and_ordered() {
    let inventory = NodeSyntaxSemanticsInventory::new(PlannedNodeSyntaxSemanticsBoundary::ALL)
        .expect("planned Node syntax/semantics boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedNodeSyntaxSemanticsBoundary::ALL
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
fn node_syntax_semantics_evidence_is_order_independent_and_duplicate_checked() {
    let forward = NodeSyntaxSemanticsInventory::new(PlannedNodeSyntaxSemanticsBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = NodeSyntaxSemanticsInventory::new(
        PlannedNodeSyntaxSemanticsBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = NodeSyntaxSemanticsInventory::new([
        PlannedNodeSyntaxSemanticsBoundary::NodeSyntaxSemantics,
        PlannedNodeSyntaxSemanticsBoundary::NodeSyntaxSemantics,
    ])
    .expect_err("duplicate Node syntax/semantics boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedNodeSyntaxSemanticsBoundary::NodeSyntaxSemantics
    );
}

#[test]
fn node_syntax_semantics_evidence_has_no_node_authority() {
    let inventory = NodeSyntaxSemanticsInventory::new([
        PlannedNodeSyntaxSemanticsBoundary::NodeSyntaxSemantics,
        PlannedNodeSyntaxSemanticsBoundary::EveryPeriod,
        PlannedNodeSyntaxSemanticsBoundary::UnknownTiming,
        PlannedNodeSyntaxSemanticsBoundary::Scheduler,
        PlannedNodeSyntaxSemanticsBoundary::DiagnosticCode,
        PlannedNodeSyntaxSemanticsBoundary::ProtocolInventory,
    ])
    .expect("bounded Node syntax/semantics evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.node-syntax-semantics-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
