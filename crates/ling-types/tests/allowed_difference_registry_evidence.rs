use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedAllowedDifferenceBoundary {
    RegistrySchema,
    EntryIdentity,
    AuthorityClause,
    SourceReference,
    EngineScope,
    TargetScope,
    ProfileScope,
    ObservableField,
    ComparisonPredicate,
    Rationale,
    EntryStatus,
    Owner,
    ReviewDate,
    ExpiryDate,
    MigrationRule,
    VersionCompatibility,
    Provenance,
    TamperEvidence,
    MissingEntryFailClosed,
    UnknownEntryReject,
    UnauthorizedEntryReject,
    OutOfScopeReject,
    ExpiredEntryReject,
    OverlapReject,
    ContradictionReject,
    PerformanceUnobservable,
    AddressUnobservable,
    TimingUnobservable,
    AllocationUnobservable,
    GcTiming,
    CleanupObservation,
    BestEffortScheduling,
    NumericPrecision,
    NumericRounding,
    NanPayload,
    SignedZero,
    NumericOverflow,
    Endianness,
    DeclaredTolerance,
    ReplayEquivalence,
    EffectLog,
    EventOrder,
    Concurrency,
    FfiVariation,
    TargetVariation,
    PositiveFixture,
    NegativeFixture,
    CrossProcessDeterminism,
    OfflineDeterminism,
    CrossTargetEvidence,
    PropertyEvidence,
    FuzzEvidence,
    DiagnosticBilingual,
    UnicodeSpans,
    SemanticIdPreservation,
    HostOutputExclusion,
    PublicProtocolExclusion,
    SchemaMigration,
    RegistryReaderBoundary,
    DifferentialHarnessIntegration,
}

impl PlannedAllowedDifferenceBoundary {
    const ALL: [Self; 60] = [
        Self::RegistrySchema,
        Self::EntryIdentity,
        Self::AuthorityClause,
        Self::SourceReference,
        Self::EngineScope,
        Self::TargetScope,
        Self::ProfileScope,
        Self::ObservableField,
        Self::ComparisonPredicate,
        Self::Rationale,
        Self::EntryStatus,
        Self::Owner,
        Self::ReviewDate,
        Self::ExpiryDate,
        Self::MigrationRule,
        Self::VersionCompatibility,
        Self::Provenance,
        Self::TamperEvidence,
        Self::MissingEntryFailClosed,
        Self::UnknownEntryReject,
        Self::UnauthorizedEntryReject,
        Self::OutOfScopeReject,
        Self::ExpiredEntryReject,
        Self::OverlapReject,
        Self::ContradictionReject,
        Self::PerformanceUnobservable,
        Self::AddressUnobservable,
        Self::TimingUnobservable,
        Self::AllocationUnobservable,
        Self::GcTiming,
        Self::CleanupObservation,
        Self::BestEffortScheduling,
        Self::NumericPrecision,
        Self::NumericRounding,
        Self::NanPayload,
        Self::SignedZero,
        Self::NumericOverflow,
        Self::Endianness,
        Self::DeclaredTolerance,
        Self::ReplayEquivalence,
        Self::EffectLog,
        Self::EventOrder,
        Self::Concurrency,
        Self::FfiVariation,
        Self::TargetVariation,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CrossProcessDeterminism,
        Self::OfflineDeterminism,
        Self::CrossTargetEvidence,
        Self::PropertyEvidence,
        Self::FuzzEvidence,
        Self::DiagnosticBilingual,
        Self::UnicodeSpans,
        Self::SemanticIdPreservation,
        Self::HostOutputExclusion,
        Self::PublicProtocolExclusion,
        Self::SchemaMigration,
        Self::RegistryReaderBoundary,
        Self::DifferentialHarnessIntegration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::RegistrySchema => 0,
            Self::EntryIdentity => 1,
            Self::AuthorityClause => 2,
            Self::SourceReference => 3,
            Self::EngineScope => 4,
            Self::TargetScope => 5,
            Self::ProfileScope => 6,
            Self::ObservableField => 7,
            Self::ComparisonPredicate => 8,
            Self::Rationale => 9,
            Self::EntryStatus => 10,
            Self::Owner => 11,
            Self::ReviewDate => 12,
            Self::ExpiryDate => 13,
            Self::MigrationRule => 14,
            Self::VersionCompatibility => 15,
            Self::Provenance => 16,
            Self::TamperEvidence => 17,
            Self::MissingEntryFailClosed => 18,
            Self::UnknownEntryReject => 19,
            Self::UnauthorizedEntryReject => 20,
            Self::OutOfScopeReject => 21,
            Self::ExpiredEntryReject => 22,
            Self::OverlapReject => 23,
            Self::ContradictionReject => 24,
            Self::PerformanceUnobservable => 25,
            Self::AddressUnobservable => 26,
            Self::TimingUnobservable => 27,
            Self::AllocationUnobservable => 28,
            Self::GcTiming => 29,
            Self::CleanupObservation => 30,
            Self::BestEffortScheduling => 31,
            Self::NumericPrecision => 32,
            Self::NumericRounding => 33,
            Self::NanPayload => 34,
            Self::SignedZero => 35,
            Self::NumericOverflow => 36,
            Self::Endianness => 37,
            Self::DeclaredTolerance => 38,
            Self::ReplayEquivalence => 39,
            Self::EffectLog => 40,
            Self::EventOrder => 41,
            Self::Concurrency => 42,
            Self::FfiVariation => 43,
            Self::TargetVariation => 44,
            Self::PositiveFixture => 45,
            Self::NegativeFixture => 46,
            Self::CrossProcessDeterminism => 47,
            Self::OfflineDeterminism => 48,
            Self::CrossTargetEvidence => 49,
            Self::PropertyEvidence => 50,
            Self::FuzzEvidence => 51,
            Self::DiagnosticBilingual => 52,
            Self::UnicodeSpans => 53,
            Self::SemanticIdPreservation => 54,
            Self::HostOutputExclusion => 55,
            Self::PublicProtocolExclusion => 56,
            Self::SchemaMigration => 57,
            Self::RegistryReaderBoundary => 58,
            Self::DifferentialHarnessIntegration => 59,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AllowedDifferenceBoundaryInventory {
    boundaries: Box<[PlannedAllowedDifferenceBoundary]>,
}

impl AllowedDifferenceBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedAllowedDifferenceBoundary>,
    ) -> Result<Self, PlannedAllowedDifferenceBoundary> {
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
        bytes.extend_from_slice(b"ling.allowed-difference-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_allowed_difference_boundaries_are_complete_and_ordered() {
    let inventory = AllowedDifferenceBoundaryInventory::new(PlannedAllowedDifferenceBoundary::ALL)
        .expect("planned allowed-difference boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedAllowedDifferenceBoundary::ALL
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
fn allowed_difference_evidence_is_order_independent_and_duplicate_checked() {
    let forward = AllowedDifferenceBoundaryInventory::new(PlannedAllowedDifferenceBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = AllowedDifferenceBoundaryInventory::new(
        PlannedAllowedDifferenceBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = AllowedDifferenceBoundaryInventory::new([
        PlannedAllowedDifferenceBoundary::RegistrySchema,
        PlannedAllowedDifferenceBoundary::RegistrySchema,
    ])
    .expect_err("duplicate allowed-difference boundary must be rejected");
    assert_eq!(duplicate, PlannedAllowedDifferenceBoundary::RegistrySchema);
}

#[test]
fn allowed_difference_evidence_has_no_registry_or_equivalence_authority() {
    let inventory = AllowedDifferenceBoundaryInventory::new([
        PlannedAllowedDifferenceBoundary::RegistrySchema,
        PlannedAllowedDifferenceBoundary::AuthorityClause,
        PlannedAllowedDifferenceBoundary::MissingEntryFailClosed,
        PlannedAllowedDifferenceBoundary::NumericPrecision,
        PlannedAllowedDifferenceBoundary::ReplayEquivalence,
        PlannedAllowedDifferenceBoundary::PositiveFixture,
        PlannedAllowedDifferenceBoundary::DiagnosticBilingual,
        PlannedAllowedDifferenceBoundary::SemanticIdPreservation,
    ])
    .expect("bounded allowed-difference evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.allowed-difference-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
