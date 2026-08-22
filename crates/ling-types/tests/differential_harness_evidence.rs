use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDifferentialBoundary {
    SourceProgram,
    CheckedCore,
    BytecodeArtifact,
    InterpreterEngine,
    VmEngine,
    NativeEngine,
    EngineIdentity,
    InputSnapshot,
    EntryIdentity,
    Arguments,
    ResultValue,
    RuntimeFault,
    DiagnosticProjection,
    SourceSpan,
    SemanticId,
    EffectTrace,
    CapabilityTrace,
    ResourceTrace,
    ActorTrace,
    ScheduleTrace,
    MailboxTrace,
    Cancellation,
    StepBoundary,
    InstructionBoundary,
    EventOrdering,
    HeapObservation,
    AllocationExclusion,
    AddressExclusion,
    TimingExclusion,
    MapOrderExclusion,
    OutputNormalization,
    ErrorNormalization,
    FaultNormalization,
    TextNormalization,
    FloatNormalization,
    IntegerNormalization,
    TupleRecordNormalization,
    VariantNormalization,
    ClosureNormalization,
    ConsoleNormalization,
    HostFailureNormalization,
    UnsupportedFeature,
    MalformedArtifact,
    LimitExceeded,
    SeedCorpus,
    PositiveCase,
    NegativeCase,
    PropertyCase,
    ReplayCase,
    CanonicalEncoding,
    DeterministicRun,
    CrossProcessRun,
    CrossTargetRun,
    CrossCompilerRun,
    DifferentialClassification,
    AllowedDifference,
    BilingualDiagnostic,
    UnicodeSpan,
    SchemaMigration,
    PublicProtocolExclusion,
}

impl PlannedDifferentialBoundary {
    const ALL: [Self; 60] = [
        Self::SourceProgram,
        Self::CheckedCore,
        Self::BytecodeArtifact,
        Self::InterpreterEngine,
        Self::VmEngine,
        Self::NativeEngine,
        Self::EngineIdentity,
        Self::InputSnapshot,
        Self::EntryIdentity,
        Self::Arguments,
        Self::ResultValue,
        Self::RuntimeFault,
        Self::DiagnosticProjection,
        Self::SourceSpan,
        Self::SemanticId,
        Self::EffectTrace,
        Self::CapabilityTrace,
        Self::ResourceTrace,
        Self::ActorTrace,
        Self::ScheduleTrace,
        Self::MailboxTrace,
        Self::Cancellation,
        Self::StepBoundary,
        Self::InstructionBoundary,
        Self::EventOrdering,
        Self::HeapObservation,
        Self::AllocationExclusion,
        Self::AddressExclusion,
        Self::TimingExclusion,
        Self::MapOrderExclusion,
        Self::OutputNormalization,
        Self::ErrorNormalization,
        Self::FaultNormalization,
        Self::TextNormalization,
        Self::FloatNormalization,
        Self::IntegerNormalization,
        Self::TupleRecordNormalization,
        Self::VariantNormalization,
        Self::ClosureNormalization,
        Self::ConsoleNormalization,
        Self::HostFailureNormalization,
        Self::UnsupportedFeature,
        Self::MalformedArtifact,
        Self::LimitExceeded,
        Self::SeedCorpus,
        Self::PositiveCase,
        Self::NegativeCase,
        Self::PropertyCase,
        Self::ReplayCase,
        Self::CanonicalEncoding,
        Self::DeterministicRun,
        Self::CrossProcessRun,
        Self::CrossTargetRun,
        Self::CrossCompilerRun,
        Self::DifferentialClassification,
        Self::AllowedDifference,
        Self::BilingualDiagnostic,
        Self::UnicodeSpan,
        Self::SchemaMigration,
        Self::PublicProtocolExclusion,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::SourceProgram => 0,
            Self::CheckedCore => 1,
            Self::BytecodeArtifact => 2,
            Self::InterpreterEngine => 3,
            Self::VmEngine => 4,
            Self::NativeEngine => 5,
            Self::EngineIdentity => 6,
            Self::InputSnapshot => 7,
            Self::EntryIdentity => 8,
            Self::Arguments => 9,
            Self::ResultValue => 10,
            Self::RuntimeFault => 11,
            Self::DiagnosticProjection => 12,
            Self::SourceSpan => 13,
            Self::SemanticId => 14,
            Self::EffectTrace => 15,
            Self::CapabilityTrace => 16,
            Self::ResourceTrace => 17,
            Self::ActorTrace => 18,
            Self::ScheduleTrace => 19,
            Self::MailboxTrace => 20,
            Self::Cancellation => 21,
            Self::StepBoundary => 22,
            Self::InstructionBoundary => 23,
            Self::EventOrdering => 24,
            Self::HeapObservation => 25,
            Self::AllocationExclusion => 26,
            Self::AddressExclusion => 27,
            Self::TimingExclusion => 28,
            Self::MapOrderExclusion => 29,
            Self::OutputNormalization => 30,
            Self::ErrorNormalization => 31,
            Self::FaultNormalization => 32,
            Self::TextNormalization => 33,
            Self::FloatNormalization => 34,
            Self::IntegerNormalization => 35,
            Self::TupleRecordNormalization => 36,
            Self::VariantNormalization => 37,
            Self::ClosureNormalization => 38,
            Self::ConsoleNormalization => 39,
            Self::HostFailureNormalization => 40,
            Self::UnsupportedFeature => 41,
            Self::MalformedArtifact => 42,
            Self::LimitExceeded => 43,
            Self::SeedCorpus => 44,
            Self::PositiveCase => 45,
            Self::NegativeCase => 46,
            Self::PropertyCase => 47,
            Self::ReplayCase => 48,
            Self::CanonicalEncoding => 49,
            Self::DeterministicRun => 50,
            Self::CrossProcessRun => 51,
            Self::CrossTargetRun => 52,
            Self::CrossCompilerRun => 53,
            Self::DifferentialClassification => 54,
            Self::AllowedDifference => 55,
            Self::BilingualDiagnostic => 56,
            Self::UnicodeSpan => 57,
            Self::SchemaMigration => 58,
            Self::PublicProtocolExclusion => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DifferentialBoundaryInventory {
    boundaries: Box<[PlannedDifferentialBoundary]>,
}

impl DifferentialBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDifferentialBoundary>,
    ) -> Result<Self, PlannedDifferentialBoundary> {
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
        bytes.extend_from_slice(b"ling.differential-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_differential_boundaries_are_complete_and_ordered() {
    let inventory = DifferentialBoundaryInventory::new(PlannedDifferentialBoundary::ALL)
        .expect("planned differential boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDifferentialBoundary::ALL
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
fn differential_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DifferentialBoundaryInventory::new(PlannedDifferentialBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        DifferentialBoundaryInventory::new(PlannedDifferentialBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DifferentialBoundaryInventory::new([
        PlannedDifferentialBoundary::SourceProgram,
        PlannedDifferentialBoundary::SourceProgram,
    ])
    .expect_err("duplicate differential boundary must be rejected");
    assert_eq!(duplicate, PlannedDifferentialBoundary::SourceProgram);
}

#[test]
fn differential_evidence_has_no_execution_or_equivalence_authority() {
    let inventory = DifferentialBoundaryInventory::new([
        PlannedDifferentialBoundary::SourceProgram,
        PlannedDifferentialBoundary::CheckedCore,
        PlannedDifferentialBoundary::InterpreterEngine,
        PlannedDifferentialBoundary::VmEngine,
        PlannedDifferentialBoundary::RuntimeFault,
        PlannedDifferentialBoundary::DifferentialClassification,
        PlannedDifferentialBoundary::DeterministicRun,
        PlannedDifferentialBoundary::PublicProtocolExclusion,
    ])
    .expect("bounded differential evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.differential-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
