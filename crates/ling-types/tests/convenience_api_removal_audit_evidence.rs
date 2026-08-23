use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedConvenienceApiRemovalBoundary {
    ConvenienceApiRemovalAudit,
    CurrentPublicSurface,
    ExactRemovalSet,
    Delete,
    Hide,
    Reject,
    Retain,
    Deprecate,
    Migrate,
    Replacement,
    CorePackage,
    PreviewPackage,
    BuiltinOnly,
    Stable,
    Preview,
    Experimental,
    ImplicitIo,
    Clock,
    Random,
    Network,
    ImplicitRetry,
    GlobalRuntime,
    UnboundedCollection,
    DynamicReflection,
    TextEncoding,
    FfiOwnership,
    Complexity,
    ResourceBound,
    Effect,
    Capability,
    Fault,
    Ownership,
    Determinism,
    Termination,
    Panic,
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
    PackageVersion,
    ProfileAvailability,
    DiagnosticCode,
    BilingualDiagnostic,
    SemanticId,
    SourceSpan,
    UnicodeVersion,
    OfflineEvidence,
    PositiveFixture,
    NegativeFixture,
    MigrationFixture,
    AcceptedAuthority,
    ExplicitExclusion,
}

impl PlannedConvenienceApiRemovalBoundary {
    const ALL: [Self; 60] = [
        Self::ConvenienceApiRemovalAudit,
        Self::CurrentPublicSurface,
        Self::ExactRemovalSet,
        Self::Delete,
        Self::Hide,
        Self::Reject,
        Self::Retain,
        Self::Deprecate,
        Self::Migrate,
        Self::Replacement,
        Self::CorePackage,
        Self::PreviewPackage,
        Self::BuiltinOnly,
        Self::Stable,
        Self::Preview,
        Self::Experimental,
        Self::ImplicitIo,
        Self::Clock,
        Self::Random,
        Self::Network,
        Self::ImplicitRetry,
        Self::GlobalRuntime,
        Self::UnboundedCollection,
        Self::DynamicReflection,
        Self::TextEncoding,
        Self::FfiOwnership,
        Self::Complexity,
        Self::ResourceBound,
        Self::Effect,
        Self::Capability,
        Self::Fault,
        Self::Ownership,
        Self::Determinism,
        Self::Termination,
        Self::Panic,
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
        Self::PackageVersion,
        Self::ProfileAvailability,
        Self::DiagnosticCode,
        Self::BilingualDiagnostic,
        Self::SemanticId,
        Self::SourceSpan,
        Self::UnicodeVersion,
        Self::OfflineEvidence,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::MigrationFixture,
        Self::AcceptedAuthority,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ConvenienceApiRemovalInventory {
    boundaries: Box<[PlannedConvenienceApiRemovalBoundary]>,
}

impl ConvenienceApiRemovalInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedConvenienceApiRemovalBoundary>,
    ) -> Result<Self, PlannedConvenienceApiRemovalBoundary> {
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
        let mut bytes = b"ling.convenience-api-removal-audit-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_convenience_api_removal_boundaries_are_complete_and_ordered() {
    let inventory = ConvenienceApiRemovalInventory::new(PlannedConvenienceApiRemovalBoundary::ALL)
        .expect("planned convenience-API removal boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedConvenienceApiRemovalBoundary::ALL
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
fn removal_audit_evidence_is_order_independent_and_duplicate_checked() {
    let forward = ConvenienceApiRemovalInventory::new(PlannedConvenienceApiRemovalBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = ConvenienceApiRemovalInventory::new(
        PlannedConvenienceApiRemovalBoundary::ALL.into_iter().rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ConvenienceApiRemovalInventory::new([
        PlannedConvenienceApiRemovalBoundary::ExactRemovalSet,
        PlannedConvenienceApiRemovalBoundary::ExactRemovalSet,
    ])
    .expect_err("duplicate removal boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedConvenienceApiRemovalBoundary::ExactRemovalSet
    );
}

#[test]
fn audit_evidence_has_no_api_removal_authority() {
    let inventory = ConvenienceApiRemovalInventory::new([
        PlannedConvenienceApiRemovalBoundary::ConvenienceApiRemovalAudit,
        PlannedConvenienceApiRemovalBoundary::ExactRemovalSet,
        PlannedConvenienceApiRemovalBoundary::Delete,
        PlannedConvenienceApiRemovalBoundary::Retain,
        PlannedConvenienceApiRemovalBoundary::Deprecate,
        PlannedConvenienceApiRemovalBoundary::CorePackage,
        PlannedConvenienceApiRemovalBoundary::PreviewPackage,
        PlannedConvenienceApiRemovalBoundary::ImplicitIo,
        PlannedConvenienceApiRemovalBoundary::DynamicReflection,
        PlannedConvenienceApiRemovalBoundary::MigrationFixture,
        PlannedConvenienceApiRemovalBoundary::AcceptedAuthority,
        PlannedConvenienceApiRemovalBoundary::ExplicitExclusion,
    ])
    .expect("bounded convenience-API removal audit evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.convenience-api-removal-audit-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
