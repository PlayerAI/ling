use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedProfileAuditLspBoundary {
    ProfileAudit,
    AuditSchema,
    Explanation,
    CheckedFacts,
    EffectFinding,
    CapabilityFinding,
    UnboundedSource,
    ProfileIdentity,
    TargetIdentity,
    Provenance,
    SemanticId,
    Utf8ByteSpan,
    RelatedSpan,
    Severity,
    DiagnosticCode,
    DiagnosticFacts,
    Localization,
    Redaction,
    UnknownField,
    SchemaVersion,
    Migration,
    LingCli,
    ProfileSelection,
    ManifestPrecedence,
    ConfigPrecedence,
    CheckCommand,
    AuditCommand,
    ExplainCommand,
    HumanFormat,
    JsonFormat,
    ExitStatus,
    OfflineMode,
    LspLifecycle,
    CapabilityNegotiation,
    DocumentUri,
    DocumentVersion,
    PositionEncoding,
    WorkspaceIdentity,
    ProfileContext,
    Cancellation,
    StaleResult,
    DeterministicPublication,
    RequestLimits,
    ErrorMapping,
    DiagnosticPublication,
    SourceMapping,
    QuickFixSafety,
    PositiveFixture,
    NegativeFixture,
    UnicodeFixture,
    CrLfFixture,
    TransitiveFixture,
    BoundsFixture,
    RevisionFixture,
    CancellationFixture,
    StaleResultFixture,
    JsonMigrationFixture,
    PrivacyFixture,
    DifferentialFixture,
    ProtocolInventory,
}

impl PlannedProfileAuditLspBoundary {
    const ALL: [Self; 60] = [
        Self::ProfileAudit,
        Self::AuditSchema,
        Self::Explanation,
        Self::CheckedFacts,
        Self::EffectFinding,
        Self::CapabilityFinding,
        Self::UnboundedSource,
        Self::ProfileIdentity,
        Self::TargetIdentity,
        Self::Provenance,
        Self::SemanticId,
        Self::Utf8ByteSpan,
        Self::RelatedSpan,
        Self::Severity,
        Self::DiagnosticCode,
        Self::DiagnosticFacts,
        Self::Localization,
        Self::Redaction,
        Self::UnknownField,
        Self::SchemaVersion,
        Self::Migration,
        Self::LingCli,
        Self::ProfileSelection,
        Self::ManifestPrecedence,
        Self::ConfigPrecedence,
        Self::CheckCommand,
        Self::AuditCommand,
        Self::ExplainCommand,
        Self::HumanFormat,
        Self::JsonFormat,
        Self::ExitStatus,
        Self::OfflineMode,
        Self::LspLifecycle,
        Self::CapabilityNegotiation,
        Self::DocumentUri,
        Self::DocumentVersion,
        Self::PositionEncoding,
        Self::WorkspaceIdentity,
        Self::ProfileContext,
        Self::Cancellation,
        Self::StaleResult,
        Self::DeterministicPublication,
        Self::RequestLimits,
        Self::ErrorMapping,
        Self::DiagnosticPublication,
        Self::SourceMapping,
        Self::QuickFixSafety,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::UnicodeFixture,
        Self::CrLfFixture,
        Self::TransitiveFixture,
        Self::BoundsFixture,
        Self::RevisionFixture,
        Self::CancellationFixture,
        Self::StaleResultFixture,
        Self::JsonMigrationFixture,
        Self::PrivacyFixture,
        Self::DifferentialFixture,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProfileAuditLspInventory {
    boundaries: Box<[PlannedProfileAuditLspBoundary]>,
}

impl ProfileAuditLspInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedProfileAuditLspBoundary>,
    ) -> Result<Self, PlannedProfileAuditLspBoundary> {
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
        let mut bytes = b"ling.profile-audit-lsp-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_profile_audit_lsp_boundaries_are_complete_and_ordered() {
    let inventory = ProfileAuditLspInventory::new(PlannedProfileAuditLspBoundary::ALL)
        .expect("planned Profile Audit/LSP boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedProfileAuditLspBoundary::ALL
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
fn profile_audit_lsp_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ProfileAuditLspInventory::new(PlannedProfileAuditLspBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        ProfileAuditLspInventory::new(PlannedProfileAuditLspBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ProfileAuditLspInventory::new([
        PlannedProfileAuditLspBoundary::ProfileAudit,
        PlannedProfileAuditLspBoundary::ProfileAudit,
    ])
    .expect_err("duplicate Profile Audit/LSP boundary must be rejected");
    assert_eq!(duplicate, PlannedProfileAuditLspBoundary::ProfileAudit);
}

#[test]
fn profile_audit_lsp_evidence_has_no_public_protocol_authority() {
    let inventory = ProfileAuditLspInventory::new([
        PlannedProfileAuditLspBoundary::ProfileAudit,
        PlannedProfileAuditLspBoundary::AuditSchema,
        PlannedProfileAuditLspBoundary::DiagnosticCode,
        PlannedProfileAuditLspBoundary::LingCli,
        PlannedProfileAuditLspBoundary::LspLifecycle,
        PlannedProfileAuditLspBoundary::DocumentVersion,
        PlannedProfileAuditLspBoundary::SemanticId,
        PlannedProfileAuditLspBoundary::ProtocolInventory,
    ])
    .expect("bounded Profile Audit/LSP evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.profile-audit-lsp-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
