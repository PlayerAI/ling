use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDifferentialHardwareMatrixBoundary {
    DifferentialMatrix,
    CpuReference,
    GpuReference,
    CheckedArtifact,
    KernelDeviceBoundary,
    InputCorpus,
    Seed,
    WorkItem,
    ReductionOrder,
    NumericMode,
    Precision,
    Rounding,
    NanPolicy,
    SignedZero,
    Overflow,
    Tolerance,
    ExactEquality,
    FaultEquivalence,
    CombinationIdentity,
    OperatingSystem,
    Device,
    Vendor,
    Architecture,
    Runtime,
    Driver,
    BackendCompiler,
    Toolchain,
    FeatureSet,
    Layout,
    ResourceLimits,
    KnownLimitations,
    Provenance,
    Expiry,
    EvidenceBundle,
    Reproducibility,
    UnsupportedStatus,
    ExperimentalStatus,
    PreviewStatus,
    StableStatus,
    FallbackPolicy,
    RejectionPolicy,
    PositiveFixture,
    NegativeFixture,
    DeterminismFixture,
    MalformedFixture,
    SourceMapFixture,
    UnicodeFixture,
    DifferentialFixture,
    CrossTargetFixture,
    ResourceFixture,
    FaultFixture,
    MigrationFixture,
    DiagnosticCode,
    DiagnosticFacts,
    HostPathExclusion,
    AddressExclusion,
    TimestampExclusion,
    DriverTextExclusion,
    LocalMachineExclusion,
    ProtocolInventory,
}

impl PlannedDifferentialHardwareMatrixBoundary {
    const ALL: [Self; 60] = [
        Self::DifferentialMatrix,
        Self::CpuReference,
        Self::GpuReference,
        Self::CheckedArtifact,
        Self::KernelDeviceBoundary,
        Self::InputCorpus,
        Self::Seed,
        Self::WorkItem,
        Self::ReductionOrder,
        Self::NumericMode,
        Self::Precision,
        Self::Rounding,
        Self::NanPolicy,
        Self::SignedZero,
        Self::Overflow,
        Self::Tolerance,
        Self::ExactEquality,
        Self::FaultEquivalence,
        Self::CombinationIdentity,
        Self::OperatingSystem,
        Self::Device,
        Self::Vendor,
        Self::Architecture,
        Self::Runtime,
        Self::Driver,
        Self::BackendCompiler,
        Self::Toolchain,
        Self::FeatureSet,
        Self::Layout,
        Self::ResourceLimits,
        Self::KnownLimitations,
        Self::Provenance,
        Self::Expiry,
        Self::EvidenceBundle,
        Self::Reproducibility,
        Self::UnsupportedStatus,
        Self::ExperimentalStatus,
        Self::PreviewStatus,
        Self::StableStatus,
        Self::FallbackPolicy,
        Self::RejectionPolicy,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::DeterminismFixture,
        Self::MalformedFixture,
        Self::SourceMapFixture,
        Self::UnicodeFixture,
        Self::DifferentialFixture,
        Self::CrossTargetFixture,
        Self::ResourceFixture,
        Self::FaultFixture,
        Self::MigrationFixture,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::DriverTextExclusion,
        Self::LocalMachineExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DifferentialHardwareMatrixInventory {
    boundaries: Box<[PlannedDifferentialHardwareMatrixBoundary]>,
}

impl DifferentialHardwareMatrixInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDifferentialHardwareMatrixBoundary>,
    ) -> Result<Self, PlannedDifferentialHardwareMatrixBoundary> {
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
        let mut bytes = b"ling.differential-hardware-matrix-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_differential_hardware_matrix_boundaries_are_complete_and_ordered() {
    let inventory =
        DifferentialHardwareMatrixInventory::new(PlannedDifferentialHardwareMatrixBoundary::ALL)
            .expect("planned differential and hardware matrix boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDifferentialHardwareMatrixBoundary::ALL
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
fn differential_hardware_matrix_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        DifferentialHardwareMatrixInventory::new(PlannedDifferentialHardwareMatrixBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = DifferentialHardwareMatrixInventory::new(
        PlannedDifferentialHardwareMatrixBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DifferentialHardwareMatrixInventory::new([
        PlannedDifferentialHardwareMatrixBoundary::DifferentialMatrix,
        PlannedDifferentialHardwareMatrixBoundary::DifferentialMatrix,
    ])
    .expect_err("duplicate differential matrix boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedDifferentialHardwareMatrixBoundary::DifferentialMatrix
    );
}

#[test]
fn differential_hardware_matrix_evidence_has_no_support_authority() {
    let inventory = DifferentialHardwareMatrixInventory::new([
        PlannedDifferentialHardwareMatrixBoundary::DifferentialMatrix,
        PlannedDifferentialHardwareMatrixBoundary::CpuReference,
        PlannedDifferentialHardwareMatrixBoundary::GpuReference,
        PlannedDifferentialHardwareMatrixBoundary::CombinationIdentity,
        PlannedDifferentialHardwareMatrixBoundary::ExperimentalStatus,
        PlannedDifferentialHardwareMatrixBoundary::DifferentialFixture,
        PlannedDifferentialHardwareMatrixBoundary::DiagnosticFacts,
        PlannedDifferentialHardwareMatrixBoundary::ProtocolInventory,
    ])
    .expect("bounded differential and hardware matrix evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.differential-hardware-matrix-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
