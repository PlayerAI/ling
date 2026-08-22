//! Internal ownership-diagnostic and repair boundary evidence.
//!
//! This test-only inventory names proposed diagnostic and repair boundaries.
//! It does not allocate codes, publish diagnostics, rank repairs, or define
//! ownership semantics.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedDiagnosticBoundary {
    ResourceOrigin,
    MoveBorrowStart,
    ConflictingUse,
    RegionBoundary,
    ConflictPersistence,
    RootCause,
    SecondaryFact,
    Severity,
    DeterministicOrdering,
    ErrorWarningBoundary,
    SeedDiagnosticInteraction,
    RepairSchema,
    RepairRanking,
    SourceEdit,
    Preconditions,
    StaleSpan,
    StaleVersion,
    Applicability,
    Alternative,
    Localization,
    SafetyProof,
    LspMapping,
    SourceProjection,
    CstProjection,
    AstProjection,
    HirProjection,
    CheckedCoreProjection,
    SemanticGraphProjection,
    AuditSourceProjection,
    Utf8ByteSpan,
    SemanticId,
    PublicSchemaVersion,
    Migration,
    ProfileBoundary,
    FfiNativeBoundary,
    TaskActorBoundary,
    FaultCompileBoundary,
    UnicodeSourceSpans,
    RawHostTextRejection,
    UncheckedAstRejection,
    MoveAliasFixture,
    PartialMoveFixture,
    EscapeFixture,
    RepairDifferential,
    SeedMigration,
}

impl PlannedDiagnosticBoundary {
    const ALL: [Self; 45] = [
        Self::ResourceOrigin,
        Self::MoveBorrowStart,
        Self::ConflictingUse,
        Self::RegionBoundary,
        Self::ConflictPersistence,
        Self::RootCause,
        Self::SecondaryFact,
        Self::Severity,
        Self::DeterministicOrdering,
        Self::ErrorWarningBoundary,
        Self::SeedDiagnosticInteraction,
        Self::RepairSchema,
        Self::RepairRanking,
        Self::SourceEdit,
        Self::Preconditions,
        Self::StaleSpan,
        Self::StaleVersion,
        Self::Applicability,
        Self::Alternative,
        Self::Localization,
        Self::SafetyProof,
        Self::LspMapping,
        Self::SourceProjection,
        Self::CstProjection,
        Self::AstProjection,
        Self::HirProjection,
        Self::CheckedCoreProjection,
        Self::SemanticGraphProjection,
        Self::AuditSourceProjection,
        Self::Utf8ByteSpan,
        Self::SemanticId,
        Self::PublicSchemaVersion,
        Self::Migration,
        Self::ProfileBoundary,
        Self::FfiNativeBoundary,
        Self::TaskActorBoundary,
        Self::FaultCompileBoundary,
        Self::UnicodeSourceSpans,
        Self::RawHostTextRejection,
        Self::UncheckedAstRejection,
        Self::MoveAliasFixture,
        Self::PartialMoveFixture,
        Self::EscapeFixture,
        Self::RepairDifferential,
        Self::SeedMigration,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::ResourceOrigin => 0,
            Self::MoveBorrowStart => 1,
            Self::ConflictingUse => 2,
            Self::RegionBoundary => 3,
            Self::ConflictPersistence => 4,
            Self::RootCause => 5,
            Self::SecondaryFact => 6,
            Self::Severity => 7,
            Self::DeterministicOrdering => 8,
            Self::ErrorWarningBoundary => 9,
            Self::SeedDiagnosticInteraction => 10,
            Self::RepairSchema => 11,
            Self::RepairRanking => 12,
            Self::SourceEdit => 13,
            Self::Preconditions => 14,
            Self::StaleSpan => 15,
            Self::StaleVersion => 16,
            Self::Applicability => 17,
            Self::Alternative => 18,
            Self::Localization => 19,
            Self::SafetyProof => 20,
            Self::LspMapping => 21,
            Self::SourceProjection => 22,
            Self::CstProjection => 23,
            Self::AstProjection => 24,
            Self::HirProjection => 25,
            Self::CheckedCoreProjection => 26,
            Self::SemanticGraphProjection => 27,
            Self::AuditSourceProjection => 28,
            Self::Utf8ByteSpan => 29,
            Self::SemanticId => 30,
            Self::PublicSchemaVersion => 31,
            Self::Migration => 32,
            Self::ProfileBoundary => 33,
            Self::FfiNativeBoundary => 34,
            Self::TaskActorBoundary => 35,
            Self::FaultCompileBoundary => 36,
            Self::UnicodeSourceSpans => 37,
            Self::RawHostTextRejection => 38,
            Self::UncheckedAstRejection => 39,
            Self::MoveAliasFixture => 40,
            Self::PartialMoveFixture => 41,
            Self::EscapeFixture => 42,
            Self::RepairDifferential => 43,
            Self::SeedMigration => 44,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DiagnosticBoundaryInventory {
    boundaries: Box<[PlannedDiagnosticBoundary]>,
}

impl DiagnosticBoundaryInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDiagnosticBoundary>,
    ) -> Result<Self, PlannedDiagnosticBoundary> {
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
        bytes.extend_from_slice(b"ling.ownership-diagnostic-observation/0");
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_diagnostic_boundaries_are_complete_and_ordered() {
    let inventory = DiagnosticBoundaryInventory::new(PlannedDiagnosticBoundary::ALL)
        .expect("planned diagnostic boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDiagnosticBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..45).collect::<Vec<_>>()
    );
}

#[test]
fn diagnostic_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DiagnosticBoundaryInventory::new(PlannedDiagnosticBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        DiagnosticBoundaryInventory::new(PlannedDiagnosticBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DiagnosticBoundaryInventory::new([
        PlannedDiagnosticBoundary::ResourceOrigin,
        PlannedDiagnosticBoundary::ResourceOrigin,
    ])
    .expect_err("duplicate diagnostic boundary must be rejected");
    assert_eq!(duplicate, PlannedDiagnosticBoundary::ResourceOrigin);
}

#[test]
fn diagnostic_boundary_evidence_has_no_ownership_diagnostic_authority() {
    let inventory = DiagnosticBoundaryInventory::new([
        PlannedDiagnosticBoundary::ResourceOrigin,
        PlannedDiagnosticBoundary::RepairRanking,
        PlannedDiagnosticBoundary::LspMapping,
        PlannedDiagnosticBoundary::UnicodeSourceSpans,
    ])
    .expect("bounded diagnostic evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.ownership-diagnostic-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 4);
}
