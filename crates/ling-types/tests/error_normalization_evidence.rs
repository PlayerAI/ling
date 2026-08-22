use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedErrorNormalizationBoundary {
    ErrorNormalization,
    FaultTaxonomy,
    SourceProvenance,
    VerifierFailure,
    CompilerFailure,
    RuntimeFailure,
    BackendFailure,
    DeviceFailure,
    ResourceFailure,
    CancellationFailure,
    HostFailure,
    CategoryPrecedence,
    UnsupportedFeature,
    DeviceUnavailable,
    CompileFailure,
    LaunchFailure,
    OutOfDeviceMemory,
    DeviceLost,
    NumericModeUnsupported,
    Retryability,
    Cancellation,
    Severity,
    SourceSpan,
    Utf8Spans,
    SemanticId,
    StructuredFacts,
    BilingualRendering,
    ErrorCodeRegistry,
    ErrorCodeLock,
    Redaction,
    VendorDetail,
    UnknownVendorEvent,
    NumericMismatch,
    CapabilityMismatch,
    MalformedModule,
    QueueFailure,
    SynchronizationFailure,
    CleanupFailure,
    Migration,
    Determinism,
    HostBackendIsolation,
    PositiveFixture,
    NegativeFixture,
    MalformedFixture,
    CorruptVendorFixture,
    BilingualFixture,
    SourceMapFixture,
    UnicodeFixture,
    RedactionFixture,
    DeterminismFixture,
    MigrationFixture,
    CrossBackendFixture,
    DiagnosticCode,
    DiagnosticFacts,
    HostPathExclusion,
    AddressExclusion,
    TimestampExclusion,
    DriverTextExclusion,
    DebugOutputExclusion,
    ProtocolInventory,
}

impl PlannedErrorNormalizationBoundary {
    const ALL: [Self; 60] = [
        Self::ErrorNormalization,
        Self::FaultTaxonomy,
        Self::SourceProvenance,
        Self::VerifierFailure,
        Self::CompilerFailure,
        Self::RuntimeFailure,
        Self::BackendFailure,
        Self::DeviceFailure,
        Self::ResourceFailure,
        Self::CancellationFailure,
        Self::HostFailure,
        Self::CategoryPrecedence,
        Self::UnsupportedFeature,
        Self::DeviceUnavailable,
        Self::CompileFailure,
        Self::LaunchFailure,
        Self::OutOfDeviceMemory,
        Self::DeviceLost,
        Self::NumericModeUnsupported,
        Self::Retryability,
        Self::Cancellation,
        Self::Severity,
        Self::SourceSpan,
        Self::Utf8Spans,
        Self::SemanticId,
        Self::StructuredFacts,
        Self::BilingualRendering,
        Self::ErrorCodeRegistry,
        Self::ErrorCodeLock,
        Self::Redaction,
        Self::VendorDetail,
        Self::UnknownVendorEvent,
        Self::NumericMismatch,
        Self::CapabilityMismatch,
        Self::MalformedModule,
        Self::QueueFailure,
        Self::SynchronizationFailure,
        Self::CleanupFailure,
        Self::Migration,
        Self::Determinism,
        Self::HostBackendIsolation,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MalformedFixture,
        Self::CorruptVendorFixture,
        Self::BilingualFixture,
        Self::SourceMapFixture,
        Self::UnicodeFixture,
        Self::RedactionFixture,
        Self::DeterminismFixture,
        Self::MigrationFixture,
        Self::CrossBackendFixture,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::HostPathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::DriverTextExclusion,
        Self::DebugOutputExclusion,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ErrorNormalizationInventory {
    boundaries: Box<[PlannedErrorNormalizationBoundary]>,
}

impl ErrorNormalizationInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedErrorNormalizationBoundary>,
    ) -> Result<Self, PlannedErrorNormalizationBoundary> {
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
        let mut bytes = b"ling.error-normalization-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_error_normalization_boundaries_are_complete_and_ordered() {
    let inventory = ErrorNormalizationInventory::new(PlannedErrorNormalizationBoundary::ALL)
        .expect("planned error normalization boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedErrorNormalizationBoundary::ALL
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
fn error_normalization_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ErrorNormalizationInventory::new(PlannedErrorNormalizationBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ErrorNormalizationInventory::new(PlannedErrorNormalizationBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ErrorNormalizationInventory::new([
        PlannedErrorNormalizationBoundary::ErrorNormalization,
        PlannedErrorNormalizationBoundary::ErrorNormalization,
    ])
    .expect_err("duplicate error normalization boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedErrorNormalizationBoundary::ErrorNormalization
    );
}

#[test]
fn error_normalization_evidence_has_no_diagnostic_authority() {
    let inventory = ErrorNormalizationInventory::new([
        PlannedErrorNormalizationBoundary::ErrorNormalization,
        PlannedErrorNormalizationBoundary::FaultTaxonomy,
        PlannedErrorNormalizationBoundary::UnsupportedFeature,
        PlannedErrorNormalizationBoundary::DeviceLost,
        PlannedErrorNormalizationBoundary::NumericModeUnsupported,
        PlannedErrorNormalizationBoundary::BilingualRendering,
        PlannedErrorNormalizationBoundary::ErrorCodeRegistry,
        PlannedErrorNormalizationBoundary::ProtocolInventory,
    ])
    .expect("bounded error normalization evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.error-normalization-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
