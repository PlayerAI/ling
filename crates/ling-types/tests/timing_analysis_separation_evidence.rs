use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedTimingAnalysisSeparationBoundary {
    TimingResult,
    Version,
    Measured,
    Estimated,
    StaticallyBounded,
    Assumed,
    Unknown,
    Invalid,
    Unsupported,
    MeasurementBoundary,
    EstimationBoundary,
    StaticAnalysisBoundary,
    ProofBoundary,
    AssumptionBoundary,
    WcetClaimExclusion,
    ObservedAverage,
    ObservedMaximum,
    SampleCount,
    AggregationRule,
    Confidence,
    Uncertainty,
    Calibration,
    ClockIdentity,
    Instrumentation,
    InstrumentationPerturbation,
    TargetIdentity,
    ProfileIdentity,
    BuildIdentity,
    ToolchainIdentity,
    SchedulerIdentity,
    InterruptModel,
    CacheModel,
    MemoryModel,
    DeviceIdentity,
    FfiIdentity,
    InputIdentity,
    EnvironmentIdentity,
    TcbIdentity,
    TimingIrId,
    PathId,
    SemanticId,
    SourceSpan,
    Provenance,
    Checksum,
    Signature,
    Redaction,
    DeterministicOrdering,
    UnknownField,
    Malformed,
    Contradictory,
    UnsupportedVersion,
    Migration,
    FailClosed,
    DiagnosticCode,
    PositiveFixture,
    NegativeFixture,
    CalibrationFixture,
    UnicodeFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedTimingAnalysisSeparationBoundary {
    const ALL: [Self; 60] = [
        Self::TimingResult,
        Self::Version,
        Self::Measured,
        Self::Estimated,
        Self::StaticallyBounded,
        Self::Assumed,
        Self::Unknown,
        Self::Invalid,
        Self::Unsupported,
        Self::MeasurementBoundary,
        Self::EstimationBoundary,
        Self::StaticAnalysisBoundary,
        Self::ProofBoundary,
        Self::AssumptionBoundary,
        Self::WcetClaimExclusion,
        Self::ObservedAverage,
        Self::ObservedMaximum,
        Self::SampleCount,
        Self::AggregationRule,
        Self::Confidence,
        Self::Uncertainty,
        Self::Calibration,
        Self::ClockIdentity,
        Self::Instrumentation,
        Self::InstrumentationPerturbation,
        Self::TargetIdentity,
        Self::ProfileIdentity,
        Self::BuildIdentity,
        Self::ToolchainIdentity,
        Self::SchedulerIdentity,
        Self::InterruptModel,
        Self::CacheModel,
        Self::MemoryModel,
        Self::DeviceIdentity,
        Self::FfiIdentity,
        Self::InputIdentity,
        Self::EnvironmentIdentity,
        Self::TcbIdentity,
        Self::TimingIrId,
        Self::PathId,
        Self::SemanticId,
        Self::SourceSpan,
        Self::Provenance,
        Self::Checksum,
        Self::Signature,
        Self::Redaction,
        Self::DeterministicOrdering,
        Self::UnknownField,
        Self::Malformed,
        Self::Contradictory,
        Self::UnsupportedVersion,
        Self::Migration,
        Self::FailClosed,
        Self::DiagnosticCode,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CalibrationFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TimingAnalysisSeparationInventory {
    boundaries: Box<[PlannedTimingAnalysisSeparationBoundary]>,
}

impl TimingAnalysisSeparationInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedTimingAnalysisSeparationBoundary>,
    ) -> Result<Self, PlannedTimingAnalysisSeparationBoundary> {
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
        let mut bytes = b"ling.timing-analysis-separation-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_timing_analysis_separation_boundaries_are_complete_and_ordered() {
    let inventory =
        TimingAnalysisSeparationInventory::new(PlannedTimingAnalysisSeparationBoundary::ALL)
            .expect("planned timing-analysis separation boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedTimingAnalysisSeparationBoundary::ALL
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
fn timing_analysis_separation_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        TimingAnalysisSeparationInventory::new(PlannedTimingAnalysisSeparationBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = TimingAnalysisSeparationInventory::new(
        PlannedTimingAnalysisSeparationBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = TimingAnalysisSeparationInventory::new([
        PlannedTimingAnalysisSeparationBoundary::TimingResult,
        PlannedTimingAnalysisSeparationBoundary::TimingResult,
    ])
    .expect_err("duplicate timing-analysis separation boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedTimingAnalysisSeparationBoundary::TimingResult
    );
}

#[test]
fn timing_analysis_separation_evidence_has_no_wcet_authority() {
    let inventory = TimingAnalysisSeparationInventory::new([
        PlannedTimingAnalysisSeparationBoundary::Measured,
        PlannedTimingAnalysisSeparationBoundary::Estimated,
        PlannedTimingAnalysisSeparationBoundary::StaticallyBounded,
        PlannedTimingAnalysisSeparationBoundary::Assumed,
        PlannedTimingAnalysisSeparationBoundary::Unknown,
        PlannedTimingAnalysisSeparationBoundary::WcetClaimExclusion,
        PlannedTimingAnalysisSeparationBoundary::ObservedMaximum,
        PlannedTimingAnalysisSeparationBoundary::ProtocolInventory,
    ])
    .expect("bounded timing-analysis separation evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.timing-analysis-separation-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
