use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedCpuReferenceTraceBoundary {
    TraceSchema,
    LogicalWorkItem,
    WorkItemOrdering,
    BufferRead,
    BufferWrite,
    BufferIdentity,
    ViewIdentity,
    Index,
    IndexNormalization,
    Operation,
    MapOperation,
    ConditionalOperation,
    LoopOperation,
    ReductionOperation,
    AtomicOperation,
    BarrierOperation,
    Fault,
    FaultOrdering,
    FaultProvenance,
    EffectWitness,
    CapabilityWitness,
    ShapeWitness,
    BoundsWitness,
    AliasWitness,
    RaceWitness,
    NumericMode,
    Determinism,
    SourceSpan,
    SemanticId,
    Utf8Spans,
    Unicode17,
    CanonicalOrdering,
    EventIdentity,
    EventSequence,
    EventPayload,
    EventSampling,
    EventLimit,
    ByteLimit,
    Truncation,
    Redaction,
    SensitiveDataExclusion,
    HostPathExclusion,
    AddressExclusion,
    DriverLogExclusion,
    CorruptionReject,
    UnknownEventReject,
    VersionCompatibility,
    Migration,
    PositiveFixture,
    NegativeFixture,
    RoundTrip,
    GoldenFile,
    CpuReference,
    CpuDifferential,
    DeviceDifferential,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    ProtocolInventory,
    PublicTraceBoundary,
}

impl PlannedCpuReferenceTraceBoundary {
    const ALL: [Self; 60] = [
        Self::TraceSchema,
        Self::LogicalWorkItem,
        Self::WorkItemOrdering,
        Self::BufferRead,
        Self::BufferWrite,
        Self::BufferIdentity,
        Self::ViewIdentity,
        Self::Index,
        Self::IndexNormalization,
        Self::Operation,
        Self::MapOperation,
        Self::ConditionalOperation,
        Self::LoopOperation,
        Self::ReductionOperation,
        Self::AtomicOperation,
        Self::BarrierOperation,
        Self::Fault,
        Self::FaultOrdering,
        Self::FaultProvenance,
        Self::EffectWitness,
        Self::CapabilityWitness,
        Self::ShapeWitness,
        Self::BoundsWitness,
        Self::AliasWitness,
        Self::RaceWitness,
        Self::NumericMode,
        Self::Determinism,
        Self::SourceSpan,
        Self::SemanticId,
        Self::Utf8Spans,
        Self::Unicode17,
        Self::CanonicalOrdering,
        Self::EventIdentity,
        Self::EventSequence,
        Self::EventPayload,
        Self::EventSampling,
        Self::EventLimit,
        Self::ByteLimit,
        Self::Truncation,
        Self::Redaction,
        Self::SensitiveDataExclusion,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::DriverLogExclusion,
        Self::CorruptionReject,
        Self::UnknownEventReject,
        Self::VersionCompatibility,
        Self::Migration,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::RoundTrip,
        Self::GoldenFile,
        Self::CpuReference,
        Self::CpuDifferential,
        Self::DeviceDifferential,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::ProtocolInventory,
        Self::PublicTraceBoundary,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CpuReferenceTraceInventory {
    boundaries: Box<[PlannedCpuReferenceTraceBoundary]>,
}

impl CpuReferenceTraceInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCpuReferenceTraceBoundary>,
    ) -> Result<Self, PlannedCpuReferenceTraceBoundary> {
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
        let mut bytes = b"ling.cpu-reference-trace-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_cpu_reference_trace_boundaries_are_complete_and_ordered() {
    let inventory = CpuReferenceTraceInventory::new(PlannedCpuReferenceTraceBoundary::ALL)
        .expect("planned CPU reference-trace boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedCpuReferenceTraceBoundary::ALL
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
fn cpu_reference_trace_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CpuReferenceTraceInventory::new(PlannedCpuReferenceTraceBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        CpuReferenceTraceInventory::new(PlannedCpuReferenceTraceBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CpuReferenceTraceInventory::new([
        PlannedCpuReferenceTraceBoundary::TraceSchema,
        PlannedCpuReferenceTraceBoundary::TraceSchema,
    ])
    .expect_err("duplicate CPU reference-trace boundary must be rejected");
    assert_eq!(duplicate, PlannedCpuReferenceTraceBoundary::TraceSchema);
}

#[test]
fn cpu_reference_trace_evidence_has_no_trace_authority() {
    let inventory = CpuReferenceTraceInventory::new([
        PlannedCpuReferenceTraceBoundary::TraceSchema,
        PlannedCpuReferenceTraceBoundary::LogicalWorkItem,
        PlannedCpuReferenceTraceBoundary::BufferRead,
        PlannedCpuReferenceTraceBoundary::Fault,
        PlannedCpuReferenceTraceBoundary::Redaction,
        PlannedCpuReferenceTraceBoundary::CpuDifferential,
        PlannedCpuReferenceTraceBoundary::BilingualDiagnostic,
        PlannedCpuReferenceTraceBoundary::ProtocolInventory,
    ])
    .expect("bounded CPU reference-trace evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.cpu-reference-trace-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
