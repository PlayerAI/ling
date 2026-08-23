use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedStableLibraryAuditBoundary {
    StableStandardLibraryAudit,
    PublicSymbol,
    BuiltinSymbol,
    PreludeSymbol,
    LogicalModule,
    PackageIdentity,
    PackageVersion,
    BuiltinOnly,
    Packaged,
    ManifestSelection,
    RegistryDistribution,
    TypeSignature,
    EffectRow,
    CapabilityRequirement,
    RuntimeFault,
    Cancellation,
    Ownership,
    Kind,
    EvaluationOrder,
    Complexity,
    ResourceBound,
    Determinism,
    ProfileAvailability,
    TargetAvailability,
    PanicBehavior,
    Termination,
    UnicodeVersion,
    LocaleBehavior,
    TextEncoding,
    ConsoleWrite,
    TextFormat,
    Max,
    Min,
    Map,
    Sum,
    Option,
    Some,
    None,
    Result,
    Ok,
    Error,
    DefinitionId,
    Origin,
    SourceSpan,
    Shadowing,
    Redefinition,
    Preview,
    Stable,
    Migration,
    Deprecation,
    BilingualDiagnostic,
    CheckedTypedCore,
    OfflineEvidence,
    CrossProcess,
    PositiveFixture,
    NegativeFixture,
    SupportMatrix,
    ProtocolInventory,
    AcceptedAuthority,
    ExplicitExclusion,
}

impl PlannedStableLibraryAuditBoundary {
    const ALL: [Self; 60] = [
        Self::StableStandardLibraryAudit,
        Self::PublicSymbol,
        Self::BuiltinSymbol,
        Self::PreludeSymbol,
        Self::LogicalModule,
        Self::PackageIdentity,
        Self::PackageVersion,
        Self::BuiltinOnly,
        Self::Packaged,
        Self::ManifestSelection,
        Self::RegistryDistribution,
        Self::TypeSignature,
        Self::EffectRow,
        Self::CapabilityRequirement,
        Self::RuntimeFault,
        Self::Cancellation,
        Self::Ownership,
        Self::Kind,
        Self::EvaluationOrder,
        Self::Complexity,
        Self::ResourceBound,
        Self::Determinism,
        Self::ProfileAvailability,
        Self::TargetAvailability,
        Self::PanicBehavior,
        Self::Termination,
        Self::UnicodeVersion,
        Self::LocaleBehavior,
        Self::TextEncoding,
        Self::ConsoleWrite,
        Self::TextFormat,
        Self::Max,
        Self::Min,
        Self::Map,
        Self::Sum,
        Self::Option,
        Self::Some,
        Self::None,
        Self::Result,
        Self::Ok,
        Self::Error,
        Self::DefinitionId,
        Self::Origin,
        Self::SourceSpan,
        Self::Shadowing,
        Self::Redefinition,
        Self::Preview,
        Self::Stable,
        Self::Migration,
        Self::Deprecation,
        Self::BilingualDiagnostic,
        Self::CheckedTypedCore,
        Self::OfflineEvidence,
        Self::CrossProcess,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::SupportMatrix,
        Self::ProtocolInventory,
        Self::AcceptedAuthority,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StableLibraryAuditInventory {
    boundaries: Box<[PlannedStableLibraryAuditBoundary]>,
}

impl StableLibraryAuditInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedStableLibraryAuditBoundary>,
    ) -> Result<Self, PlannedStableLibraryAuditBoundary> {
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
        let mut bytes = b"ling.stable-standard-library-audit-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_stable_library_audit_boundaries_are_complete_and_ordered() {
    let inventory = StableLibraryAuditInventory::new(PlannedStableLibraryAuditBoundary::ALL)
        .expect("planned Stable library audit boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedStableLibraryAuditBoundary::ALL
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
fn stable_library_audit_evidence_is_order_independent_and_duplicate_checked() {
    let forward = StableLibraryAuditInventory::new(PlannedStableLibraryAuditBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        StableLibraryAuditInventory::new(PlannedStableLibraryAuditBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = StableLibraryAuditInventory::new([
        PlannedStableLibraryAuditBoundary::PublicSymbol,
        PlannedStableLibraryAuditBoundary::PublicSymbol,
    ])
    .expect_err("duplicate Stable library audit boundary must be rejected");
    assert_eq!(duplicate, PlannedStableLibraryAuditBoundary::PublicSymbol);
}

#[test]
fn audit_evidence_has_no_stable_library_authority() {
    let inventory = StableLibraryAuditInventory::new([
        PlannedStableLibraryAuditBoundary::StableStandardLibraryAudit,
        PlannedStableLibraryAuditBoundary::BuiltinSymbol,
        PlannedStableLibraryAuditBoundary::PreludeSymbol,
        PlannedStableLibraryAuditBoundary::BuiltinOnly,
        PlannedStableLibraryAuditBoundary::Packaged,
        PlannedStableLibraryAuditBoundary::ProfileAvailability,
        PlannedStableLibraryAuditBoundary::Preview,
        PlannedStableLibraryAuditBoundary::Stable,
        PlannedStableLibraryAuditBoundary::Migration,
        PlannedStableLibraryAuditBoundary::SupportMatrix,
        PlannedStableLibraryAuditBoundary::AcceptedAuthority,
        PlannedStableLibraryAuditBoundary::ExplicitExclusion,
    ])
    .expect("bounded Stable library audit evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.stable-standard-library-audit-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
