use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedFalseEntryPointAuditBoundary {
    FalseEntryPointAudit,
    PublicSurfaceInventory,
    CliCommand,
    CliHelp,
    CliExitStatus,
    LibraryApi,
    Diagnostic,
    Protocol,
    Schema,
    Profile,
    Backend,
    BuildDefault,
    Documentation,
    EditorGrammar,
    EditorHighlight,
    EditorCompletion,
    SyntaxAlias,
    StaleLegacyName,
    EmptyImplementation,
    TodoMacro,
    UnimplementedMacro,
    UnreachableDispatchInvariant,
    SuccessfulNoOp,
    AdvertisedFutureCommand,
    UnavailableCapability,
    UnsupportedCapability,
    NegativeEvidence,
    RecoveryNode,
    NegativeFixture,
    InternalPlaceholder,
    Implemented,
    Experimental,
    Preview,
    Stable,
    Deprecated,
    Removed,
    Owner,
    AcceptedAuthority,
    CompatibilityStatus,
    Delete,
    Hide,
    Reject,
    Retain,
    Migrate,
    FailClosed,
    BilingualDiagnostic,
    OriginalUtf8Span,
    UnicodeVersion,
    CanonicalIdentity,
    SemanticId,
    DeterministicOrder,
    OfflineEvidence,
    PositiveFixture,
    NegativeCommandFixture,
    HelpFixture,
    CompletionFixture,
    GrammarFixture,
    SupportMatrixFixture,
    ProtocolInventory,
    ExplicitExclusion,
}

impl PlannedFalseEntryPointAuditBoundary {
    const ALL: [Self; 60] = [
        Self::FalseEntryPointAudit,
        Self::PublicSurfaceInventory,
        Self::CliCommand,
        Self::CliHelp,
        Self::CliExitStatus,
        Self::LibraryApi,
        Self::Diagnostic,
        Self::Protocol,
        Self::Schema,
        Self::Profile,
        Self::Backend,
        Self::BuildDefault,
        Self::Documentation,
        Self::EditorGrammar,
        Self::EditorHighlight,
        Self::EditorCompletion,
        Self::SyntaxAlias,
        Self::StaleLegacyName,
        Self::EmptyImplementation,
        Self::TodoMacro,
        Self::UnimplementedMacro,
        Self::UnreachableDispatchInvariant,
        Self::SuccessfulNoOp,
        Self::AdvertisedFutureCommand,
        Self::UnavailableCapability,
        Self::UnsupportedCapability,
        Self::NegativeEvidence,
        Self::RecoveryNode,
        Self::NegativeFixture,
        Self::InternalPlaceholder,
        Self::Implemented,
        Self::Experimental,
        Self::Preview,
        Self::Stable,
        Self::Deprecated,
        Self::Removed,
        Self::Owner,
        Self::AcceptedAuthority,
        Self::CompatibilityStatus,
        Self::Delete,
        Self::Hide,
        Self::Reject,
        Self::Retain,
        Self::Migrate,
        Self::FailClosed,
        Self::BilingualDiagnostic,
        Self::OriginalUtf8Span,
        Self::UnicodeVersion,
        Self::CanonicalIdentity,
        Self::SemanticId,
        Self::DeterministicOrder,
        Self::OfflineEvidence,
        Self::PositiveFixture,
        Self::NegativeCommandFixture,
        Self::HelpFixture,
        Self::CompletionFixture,
        Self::GrammarFixture,
        Self::SupportMatrixFixture,
        Self::ProtocolInventory,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FalseEntryPointAuditInventory {
    boundaries: Box<[PlannedFalseEntryPointAuditBoundary]>,
}

impl FalseEntryPointAuditInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedFalseEntryPointAuditBoundary>,
    ) -> Result<Self, PlannedFalseEntryPointAuditBoundary> {
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
        let mut bytes = b"ling.false-entry-point-audit-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_false_entry_point_audit_boundaries_are_complete_and_ordered() {
    let inventory = FalseEntryPointAuditInventory::new(PlannedFalseEntryPointAuditBoundary::ALL)
        .expect("planned false-entry-point-audit boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedFalseEntryPointAuditBoundary::ALL
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
fn false_entry_point_audit_evidence_is_order_independent_and_duplicate_checked() {
    let forward = FalseEntryPointAuditInventory::new(PlannedFalseEntryPointAuditBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = FalseEntryPointAuditInventory::new(
        PlannedFalseEntryPointAuditBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = FalseEntryPointAuditInventory::new([
        PlannedFalseEntryPointAuditBoundary::FalseEntryPointAudit,
        PlannedFalseEntryPointAuditBoundary::FalseEntryPointAudit,
    ])
    .expect_err("duplicate false-entry-point-audit boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedFalseEntryPointAuditBoundary::FalseEntryPointAudit
    );
}

#[test]
fn false_entry_point_audit_evidence_has_no_cleanup_authority() {
    let inventory = FalseEntryPointAuditInventory::new([
        PlannedFalseEntryPointAuditBoundary::FalseEntryPointAudit,
        PlannedFalseEntryPointAuditBoundary::AcceptedAuthority,
        PlannedFalseEntryPointAuditBoundary::AdvertisedFutureCommand,
        PlannedFalseEntryPointAuditBoundary::UnavailableCapability,
        PlannedFalseEntryPointAuditBoundary::UnsupportedCapability,
        PlannedFalseEntryPointAuditBoundary::NegativeEvidence,
        PlannedFalseEntryPointAuditBoundary::UnreachableDispatchInvariant,
        PlannedFalseEntryPointAuditBoundary::Delete,
        PlannedFalseEntryPointAuditBoundary::Hide,
        PlannedFalseEntryPointAuditBoundary::Retain,
        PlannedFalseEntryPointAuditBoundary::Migrate,
        PlannedFalseEntryPointAuditBoundary::ExplicitExclusion,
    ])
    .expect("bounded false-entry-point-audit evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.false-entry-point-audit-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
