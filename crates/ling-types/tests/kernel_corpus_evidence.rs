use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedKernelCorpusBoundary {
    CorpusManifest,
    FixtureIdentity,
    ProgramIdentity,
    SemanticId,
    SourceBytes,
    SourceExtension,
    SourceSpan,
    Utf8Spans,
    Unicode17,
    Profile,
    Target,
    InputValue,
    ExpectedValue,
    ExpectedShape,
    FaultExpectation,
    TraceReference,
    VectorAdd,
    MatrixMultiply,
    ImageFilter,
    Reduction,
    Histogram,
    Atomic,
    InvalidBounds,
    AliasConflict,
    FloatingPointEdge,
    NumericTolerance,
    Determinism,
    ShapeLayout,
    IndexBounds,
    BufferAccess,
    EffectWitness,
    CapabilityWitness,
    OwnershipWitness,
    ResourceLimit,
    PositiveFixture,
    NegativeFixture,
    PropertyFixture,
    CorruptionFixture,
    ResourceFixture,
    MigrationFixture,
    UnicodeFixture,
    SourceMapFixture,
    RoundTrip,
    GoldenFile,
    CanonicalOrdering,
    ManifestVersion,
    UnknownFieldReject,
    MissingFieldReject,
    DuplicateFixtureReject,
    CpuReference,
    CpuDifferential,
    DeviceDifferential,
    UnsupportedTarget,
    DiagnosticCode,
    DiagnosticFacts,
    BilingualDiagnostic,
    HostPathExclusion,
    AddressExclusion,
    ProtocolInventory,
    PublicCorpusBoundary,
}

impl PlannedKernelCorpusBoundary {
    const ALL: [Self; 60] = [
        Self::CorpusManifest,
        Self::FixtureIdentity,
        Self::ProgramIdentity,
        Self::SemanticId,
        Self::SourceBytes,
        Self::SourceExtension,
        Self::SourceSpan,
        Self::Utf8Spans,
        Self::Unicode17,
        Self::Profile,
        Self::Target,
        Self::InputValue,
        Self::ExpectedValue,
        Self::ExpectedShape,
        Self::FaultExpectation,
        Self::TraceReference,
        Self::VectorAdd,
        Self::MatrixMultiply,
        Self::ImageFilter,
        Self::Reduction,
        Self::Histogram,
        Self::Atomic,
        Self::InvalidBounds,
        Self::AliasConflict,
        Self::FloatingPointEdge,
        Self::NumericTolerance,
        Self::Determinism,
        Self::ShapeLayout,
        Self::IndexBounds,
        Self::BufferAccess,
        Self::EffectWitness,
        Self::CapabilityWitness,
        Self::OwnershipWitness,
        Self::ResourceLimit,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::PropertyFixture,
        Self::CorruptionFixture,
        Self::ResourceFixture,
        Self::MigrationFixture,
        Self::UnicodeFixture,
        Self::SourceMapFixture,
        Self::RoundTrip,
        Self::GoldenFile,
        Self::CanonicalOrdering,
        Self::ManifestVersion,
        Self::UnknownFieldReject,
        Self::MissingFieldReject,
        Self::DuplicateFixtureReject,
        Self::CpuReference,
        Self::CpuDifferential,
        Self::DeviceDifferential,
        Self::UnsupportedTarget,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::BilingualDiagnostic,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::ProtocolInventory,
        Self::PublicCorpusBoundary,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct KernelCorpusInventory {
    boundaries: Box<[PlannedKernelCorpusBoundary]>,
}

impl KernelCorpusInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedKernelCorpusBoundary>,
    ) -> Result<Self, PlannedKernelCorpusBoundary> {
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
        let mut bytes = b"ling.kernel-corpus-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_kernel_corpus_boundaries_are_complete_and_ordered() {
    let inventory = KernelCorpusInventory::new(PlannedKernelCorpusBoundary::ALL)
        .expect("planned Kernel corpus boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedKernelCorpusBoundary::ALL
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
fn kernel_corpus_evidence_is_order_independent_and_duplicate_checked() {
    let forward = KernelCorpusInventory::new(PlannedKernelCorpusBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = KernelCorpusInventory::new(PlannedKernelCorpusBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = KernelCorpusInventory::new([
        PlannedKernelCorpusBoundary::CorpusManifest,
        PlannedKernelCorpusBoundary::CorpusManifest,
    ])
    .expect_err("duplicate Kernel corpus boundary must be rejected");
    assert_eq!(duplicate, PlannedKernelCorpusBoundary::CorpusManifest);
}

#[test]
fn kernel_corpus_evidence_has_no_fixture_authority() {
    let inventory = KernelCorpusInventory::new([
        PlannedKernelCorpusBoundary::CorpusManifest,
        PlannedKernelCorpusBoundary::VectorAdd,
        PlannedKernelCorpusBoundary::MatrixMultiply,
        PlannedKernelCorpusBoundary::InvalidBounds,
        PlannedKernelCorpusBoundary::NumericTolerance,
        PlannedKernelCorpusBoundary::CpuDifferential,
        PlannedKernelCorpusBoundary::BilingualDiagnostic,
        PlannedKernelCorpusBoundary::ProtocolInventory,
    ])
    .expect("bounded Kernel corpus evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.kernel-corpus-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
