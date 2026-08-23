use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedSupportMatrixItemAuditBoundary {
    SupportMatrixItemAudit,
    Version,
    MatrixTarget,
    CandidateStableSet,
    Feature,
    Profile,
    Target,
    StableIdentity,
    StableVersion,
    ExactName,
    AcceptedAuthority,
    ParserImplementation,
    CheckerImplementation,
    TypedCoreImplementation,
    InterpreterSupport,
    VmSupport,
    NativeSupport,
    DeviceSupport,
    PositiveConformance,
    NegativeConformance,
    DifferentialConformance,
    DiagnosticCode,
    LspSupport,
    ZedSupport,
    CompatibilityPromise,
    KnownLimitation,
    CiEvidence,
    ReleaseArtifact,
    Traceability,
    ClauseToSymbol,
    SymbolToClause,
    EvidencePolarity,
    Supported,
    Experimental,
    Preview,
    Unavailable,
    Unsupported,
    Stable,
    Demotion,
    ExplicitExclusion,
    MissingField,
    ConflictingField,
    StaleField,
    UnverifiableField,
    FailClosed,
    ProtocolCompatibility,
    MigrationCorpus,
    CanonicalIdentity,
    SemanticId,
    UnicodeVersion,
    OriginalUtf8Span,
    DeterministicBuild,
    OfflineBuild,
    IndependentReview,
    TierScope,
    TargetToolchainIdentity,
    PositiveFixture,
    NegativeFixture,
    UnicodeFixture,
    ProtocolInventory,
}

impl PlannedSupportMatrixItemAuditBoundary {
    const ALL: [Self; 60] = [
        Self::SupportMatrixItemAudit,
        Self::Version,
        Self::MatrixTarget,
        Self::CandidateStableSet,
        Self::Feature,
        Self::Profile,
        Self::Target,
        Self::StableIdentity,
        Self::StableVersion,
        Self::ExactName,
        Self::AcceptedAuthority,
        Self::ParserImplementation,
        Self::CheckerImplementation,
        Self::TypedCoreImplementation,
        Self::InterpreterSupport,
        Self::VmSupport,
        Self::NativeSupport,
        Self::DeviceSupport,
        Self::PositiveConformance,
        Self::NegativeConformance,
        Self::DifferentialConformance,
        Self::DiagnosticCode,
        Self::LspSupport,
        Self::ZedSupport,
        Self::CompatibilityPromise,
        Self::KnownLimitation,
        Self::CiEvidence,
        Self::ReleaseArtifact,
        Self::Traceability,
        Self::ClauseToSymbol,
        Self::SymbolToClause,
        Self::EvidencePolarity,
        Self::Supported,
        Self::Experimental,
        Self::Preview,
        Self::Unavailable,
        Self::Unsupported,
        Self::Stable,
        Self::Demotion,
        Self::ExplicitExclusion,
        Self::MissingField,
        Self::ConflictingField,
        Self::StaleField,
        Self::UnverifiableField,
        Self::FailClosed,
        Self::ProtocolCompatibility,
        Self::MigrationCorpus,
        Self::CanonicalIdentity,
        Self::SemanticId,
        Self::UnicodeVersion,
        Self::OriginalUtf8Span,
        Self::DeterministicBuild,
        Self::OfflineBuild,
        Self::IndependentReview,
        Self::TierScope,
        Self::TargetToolchainIdentity,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::UnicodeFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SupportMatrixItemAuditInventory {
    boundaries: Box<[PlannedSupportMatrixItemAuditBoundary]>,
}

impl SupportMatrixItemAuditInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedSupportMatrixItemAuditBoundary>,
    ) -> Result<Self, PlannedSupportMatrixItemAuditBoundary> {
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
        let mut bytes = b"ling.support-matrix-item-audit-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_support_matrix_item_audit_boundaries_are_complete_and_ordered() {
    let inventory =
        SupportMatrixItemAuditInventory::new(PlannedSupportMatrixItemAuditBoundary::ALL)
            .expect("planned support-matrix-item-audit boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedSupportMatrixItemAuditBoundary::ALL
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
fn support_matrix_item_audit_evidence_is_order_independent_and_duplicate_checked() {
    let forward = SupportMatrixItemAuditInventory::new(PlannedSupportMatrixItemAuditBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = SupportMatrixItemAuditInventory::new(
        PlannedSupportMatrixItemAuditBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = SupportMatrixItemAuditInventory::new([
        PlannedSupportMatrixItemAuditBoundary::SupportMatrixItemAudit,
        PlannedSupportMatrixItemAuditBoundary::SupportMatrixItemAudit,
    ])
    .expect_err("duplicate support-matrix-item-audit boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedSupportMatrixItemAuditBoundary::SupportMatrixItemAudit
    );
}

#[test]
fn support_matrix_item_audit_evidence_has_no_stable_promotion_authority() {
    let inventory = SupportMatrixItemAuditInventory::new([
        PlannedSupportMatrixItemAuditBoundary::SupportMatrixItemAudit,
        PlannedSupportMatrixItemAuditBoundary::CandidateStableSet,
        PlannedSupportMatrixItemAuditBoundary::AcceptedAuthority,
        PlannedSupportMatrixItemAuditBoundary::Experimental,
        PlannedSupportMatrixItemAuditBoundary::Preview,
        PlannedSupportMatrixItemAuditBoundary::Unavailable,
        PlannedSupportMatrixItemAuditBoundary::Unsupported,
        PlannedSupportMatrixItemAuditBoundary::Stable,
        PlannedSupportMatrixItemAuditBoundary::MissingField,
        PlannedSupportMatrixItemAuditBoundary::FailClosed,
        PlannedSupportMatrixItemAuditBoundary::ProtocolInventory,
    ])
    .expect("bounded support-matrix-item-audit evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.support-matrix-item-audit-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 11);
}
