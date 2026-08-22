use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedTimingIrPathBoundary {
    TimingIr,
    Version,
    TargetArchitecture,
    TargetInstruction,
    BasicBlock,
    ControlFlowPath,
    EntryPoint,
    ExitPoint,
    Loop,
    LoopBound,
    RecursionBound,
    PathCondition,
    Branch,
    Call,
    Return,
    CacheAssumption,
    MemoryAssumption,
    InterruptModel,
    SchedulerModel,
    DeviceCall,
    FfiCall,
    CallBound,
    SourceMap,
    SemanticId,
    SourceSpan,
    CheckedCore,
    Bytecode,
    MachineCode,
    CostUnit,
    InstructionCost,
    BlockCost,
    PathCost,
    WorstCaseBound,
    BoundSource,
    AssumptionId,
    EvidenceId,
    ToolIdentity,
    ToolVersion,
    Provenance,
    Checksum,
    Signature,
    Redaction,
    DeterministicOrdering,
    ResourceLimit,
    Unknown,
    Incomplete,
    Malformed,
    Corrupt,
    UnsupportedVersion,
    Migration,
    FailClosed,
    DiagnosticCode,
    DiagnosticFacts,
    PositiveFixture,
    NegativeFixture,
    LoopFixture,
    CallFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedTimingIrPathBoundary {
    const ALL: [Self; 60] = [
        Self::TimingIr,
        Self::Version,
        Self::TargetArchitecture,
        Self::TargetInstruction,
        Self::BasicBlock,
        Self::ControlFlowPath,
        Self::EntryPoint,
        Self::ExitPoint,
        Self::Loop,
        Self::LoopBound,
        Self::RecursionBound,
        Self::PathCondition,
        Self::Branch,
        Self::Call,
        Self::Return,
        Self::CacheAssumption,
        Self::MemoryAssumption,
        Self::InterruptModel,
        Self::SchedulerModel,
        Self::DeviceCall,
        Self::FfiCall,
        Self::CallBound,
        Self::SourceMap,
        Self::SemanticId,
        Self::SourceSpan,
        Self::CheckedCore,
        Self::Bytecode,
        Self::MachineCode,
        Self::CostUnit,
        Self::InstructionCost,
        Self::BlockCost,
        Self::PathCost,
        Self::WorstCaseBound,
        Self::BoundSource,
        Self::AssumptionId,
        Self::EvidenceId,
        Self::ToolIdentity,
        Self::ToolVersion,
        Self::Provenance,
        Self::Checksum,
        Self::Signature,
        Self::Redaction,
        Self::DeterministicOrdering,
        Self::ResourceLimit,
        Self::Unknown,
        Self::Incomplete,
        Self::Malformed,
        Self::Corrupt,
        Self::UnsupportedVersion,
        Self::Migration,
        Self::FailClosed,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::LoopFixture,
        Self::CallFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TimingIrPathInventory {
    boundaries: Box<[PlannedTimingIrPathBoundary]>,
}

impl TimingIrPathInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedTimingIrPathBoundary>,
    ) -> Result<Self, PlannedTimingIrPathBoundary> {
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
        let mut bytes = b"ling.timing-ir-path-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_timing_ir_path_boundaries_are_complete_and_ordered() {
    let inventory = TimingIrPathInventory::new(PlannedTimingIrPathBoundary::ALL)
        .expect("planned Timing IR/path boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedTimingIrPathBoundary::ALL
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
fn timing_ir_path_evidence_is_order_independent_and_duplicate_checked() {
    let forward = TimingIrPathInventory::new(PlannedTimingIrPathBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = TimingIrPathInventory::new(PlannedTimingIrPathBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = TimingIrPathInventory::new([
        PlannedTimingIrPathBoundary::TimingIr,
        PlannedTimingIrPathBoundary::TimingIr,
    ])
    .expect_err("duplicate Timing IR/path boundary must be rejected");
    assert_eq!(duplicate, PlannedTimingIrPathBoundary::TimingIr);
}

#[test]
fn timing_ir_path_evidence_has_no_timing_authority() {
    let inventory = TimingIrPathInventory::new([
        PlannedTimingIrPathBoundary::TimingIr,
        PlannedTimingIrPathBoundary::TargetInstruction,
        PlannedTimingIrPathBoundary::ControlFlowPath,
        PlannedTimingIrPathBoundary::WorstCaseBound,
        PlannedTimingIrPathBoundary::SourceMap,
        PlannedTimingIrPathBoundary::ProtocolInventory,
    ])
    .expect("bounded Timing IR/path evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.timing-ir-path-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 6);
}
