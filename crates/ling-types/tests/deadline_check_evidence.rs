use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDeadlineCheckBoundary {
    DeadlineCheck,
    Version,
    NodeDeadline,
    NodePeriod,
    LogicalClockUnit,
    ActivationRule,
    ReleaseRule,
    OverrunBehavior,
    WcetBound,
    WcetStatus,
    SchedulerInterference,
    IoBound,
    Margin,
    ComparisonEquation,
    InequalityRule,
    RoundingRule,
    ValidityCondition,
    Satisfied,
    Missed,
    Unknown,
    Invalid,
    Unsupported,
    TargetId,
    ProfileId,
    BuildId,
    ProcessorId,
    ToolchainId,
    SchedulerId,
    ClockId,
    DevicePackageId,
    TcbId,
    TimingIrId,
    PathId,
    AssumptionId,
    EvidenceId,
    SemanticId,
    SourceSpan,
    Fault,
    Cancellation,
    SkipPolicy,
    QueuePolicy,
    AbortPolicy,
    DegradePolicy,
    ResourceContention,
    StaleBuild,
    TargetMismatch,
    MissingAssumption,
    ContradictoryAssumption,
    UnknownPath,
    Malformed,
    UnsupportedVersion,
    Migration,
    FailClosed,
    DiagnosticCode,
    PositiveFixture,
    NegativeFixture,
    OverrunFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedDeadlineCheckBoundary {
    const ALL: [Self; 60] = [
        Self::DeadlineCheck,
        Self::Version,
        Self::NodeDeadline,
        Self::NodePeriod,
        Self::LogicalClockUnit,
        Self::ActivationRule,
        Self::ReleaseRule,
        Self::OverrunBehavior,
        Self::WcetBound,
        Self::WcetStatus,
        Self::SchedulerInterference,
        Self::IoBound,
        Self::Margin,
        Self::ComparisonEquation,
        Self::InequalityRule,
        Self::RoundingRule,
        Self::ValidityCondition,
        Self::Satisfied,
        Self::Missed,
        Self::Unknown,
        Self::Invalid,
        Self::Unsupported,
        Self::TargetId,
        Self::ProfileId,
        Self::BuildId,
        Self::ProcessorId,
        Self::ToolchainId,
        Self::SchedulerId,
        Self::ClockId,
        Self::DevicePackageId,
        Self::TcbId,
        Self::TimingIrId,
        Self::PathId,
        Self::AssumptionId,
        Self::EvidenceId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Fault,
        Self::Cancellation,
        Self::SkipPolicy,
        Self::QueuePolicy,
        Self::AbortPolicy,
        Self::DegradePolicy,
        Self::ResourceContention,
        Self::StaleBuild,
        Self::TargetMismatch,
        Self::MissingAssumption,
        Self::ContradictoryAssumption,
        Self::UnknownPath,
        Self::Malformed,
        Self::UnsupportedVersion,
        Self::Migration,
        Self::FailClosed,
        Self::DiagnosticCode,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::OverrunFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DeadlineCheckInventory {
    boundaries: Box<[PlannedDeadlineCheckBoundary]>,
}

impl DeadlineCheckInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDeadlineCheckBoundary>,
    ) -> Result<Self, PlannedDeadlineCheckBoundary> {
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
        let mut bytes = b"ling.deadline-check-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_deadline_check_boundaries_are_complete_and_ordered() {
    let inventory = DeadlineCheckInventory::new(PlannedDeadlineCheckBoundary::ALL)
        .expect("planned deadline-check boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDeadlineCheckBoundary::ALL
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
fn deadline_check_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DeadlineCheckInventory::new(PlannedDeadlineCheckBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = DeadlineCheckInventory::new(PlannedDeadlineCheckBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DeadlineCheckInventory::new([
        PlannedDeadlineCheckBoundary::DeadlineCheck,
        PlannedDeadlineCheckBoundary::DeadlineCheck,
    ])
    .expect_err("duplicate deadline-check boundary must be rejected");
    assert_eq!(duplicate, PlannedDeadlineCheckBoundary::DeadlineCheck);
}

#[test]
fn deadline_check_evidence_has_no_schedulability_authority() {
    let inventory = DeadlineCheckInventory::new([
        PlannedDeadlineCheckBoundary::NodeDeadline,
        PlannedDeadlineCheckBoundary::WcetBound,
        PlannedDeadlineCheckBoundary::SchedulerInterference,
        PlannedDeadlineCheckBoundary::IoBound,
        PlannedDeadlineCheckBoundary::Margin,
        PlannedDeadlineCheckBoundary::ValidityCondition,
        PlannedDeadlineCheckBoundary::TargetId,
        PlannedDeadlineCheckBoundary::ProfileId,
        PlannedDeadlineCheckBoundary::BuildId,
        PlannedDeadlineCheckBoundary::ProtocolInventory,
    ])
    .expect("bounded deadline-check evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.deadline-check-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 10);
}
