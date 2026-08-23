use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedCliExitFreezeBoundary {
    CliExitFreeze,
    CliName,
    SourceExtension,
    RootCommand,
    HierarchicalCommand,
    HelpFlag,
    VersionFlag,
    Run,
    Check,
    Repl,
    Semantic,
    Audit,
    Format,
    Init,
    Test,
    ProjectCheck,
    Lsp,
    Build,
    Query,
    Patch,
    Replay,
    Explain,
    Evidence,
    Support,
    OutputFormatHuman,
    OutputFormatJson,
    SuccessExit,
    CompileExit,
    InvalidUsageExit,
    ReservedExit,
    RuntimeFaultExit,
    InternalErrorExit,
    SnapshotMismatchExit,
    Stdout,
    Stderr,
    HumanMode,
    JsonMode,
    ColorPolicy,
    PathNormalization,
    OfflineBehavior,
    LockedBehavior,
    DefaultValue,
    FlagArity,
    UnknownCommand,
    UnknownOption,
    InvalidFormat,
    DiagnosticCode,
    BilingualDiagnostic,
    SchemaVersion,
    ProtocolInventory,
    SupportMatrix,
    Preview,
    Experimental,
    Stable,
    DeterministicOrder,
    OriginalUtf8Span,
    UnicodeVersion,
    PositiveFixture,
    NegativeFixture,
    ExplicitExclusion,
}

impl PlannedCliExitFreezeBoundary {
    const ALL: [Self; 60] = [
        Self::CliExitFreeze,
        Self::CliName,
        Self::SourceExtension,
        Self::RootCommand,
        Self::HierarchicalCommand,
        Self::HelpFlag,
        Self::VersionFlag,
        Self::Run,
        Self::Check,
        Self::Repl,
        Self::Semantic,
        Self::Audit,
        Self::Format,
        Self::Init,
        Self::Test,
        Self::ProjectCheck,
        Self::Lsp,
        Self::Build,
        Self::Query,
        Self::Patch,
        Self::Replay,
        Self::Explain,
        Self::Evidence,
        Self::Support,
        Self::OutputFormatHuman,
        Self::OutputFormatJson,
        Self::SuccessExit,
        Self::CompileExit,
        Self::InvalidUsageExit,
        Self::ReservedExit,
        Self::RuntimeFaultExit,
        Self::InternalErrorExit,
        Self::SnapshotMismatchExit,
        Self::Stdout,
        Self::Stderr,
        Self::HumanMode,
        Self::JsonMode,
        Self::ColorPolicy,
        Self::PathNormalization,
        Self::OfflineBehavior,
        Self::LockedBehavior,
        Self::DefaultValue,
        Self::FlagArity,
        Self::UnknownCommand,
        Self::UnknownOption,
        Self::InvalidFormat,
        Self::DiagnosticCode,
        Self::BilingualDiagnostic,
        Self::SchemaVersion,
        Self::ProtocolInventory,
        Self::SupportMatrix,
        Self::Preview,
        Self::Experimental,
        Self::Stable,
        Self::DeterministicOrder,
        Self::OriginalUtf8Span,
        Self::UnicodeVersion,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CliExitFreezeInventory {
    boundaries: Box<[PlannedCliExitFreezeBoundary]>,
}

impl CliExitFreezeInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedCliExitFreezeBoundary>,
    ) -> Result<Self, PlannedCliExitFreezeBoundary> {
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
        let mut bytes = b"ling.cli-exit-freeze-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_cli_exit_freeze_boundaries_are_complete_and_ordered() {
    let inventory = CliExitFreezeInventory::new(PlannedCliExitFreezeBoundary::ALL)
        .expect("planned CLI/exit freeze boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedCliExitFreezeBoundary::ALL
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
fn cli_exit_freeze_evidence_is_order_independent_and_duplicate_checked() {
    let forward = CliExitFreezeInventory::new(PlannedCliExitFreezeBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = CliExitFreezeInventory::new(PlannedCliExitFreezeBoundary::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = CliExitFreezeInventory::new([
        PlannedCliExitFreezeBoundary::InvalidUsageExit,
        PlannedCliExitFreezeBoundary::InvalidUsageExit,
    ])
    .expect_err("duplicate CLI/exit freeze boundary must be rejected");
    assert_eq!(duplicate, PlannedCliExitFreezeBoundary::InvalidUsageExit);
}

#[test]
fn freeze_evidence_has_no_stable_cli_authority() {
    let inventory = CliExitFreezeInventory::new([
        PlannedCliExitFreezeBoundary::CliExitFreeze,
        PlannedCliExitFreezeBoundary::RootCommand,
        PlannedCliExitFreezeBoundary::Build,
        PlannedCliExitFreezeBoundary::Query,
        PlannedCliExitFreezeBoundary::Replay,
        PlannedCliExitFreezeBoundary::SuccessExit,
        PlannedCliExitFreezeBoundary::ReservedExit,
        PlannedCliExitFreezeBoundary::ColorPolicy,
        PlannedCliExitFreezeBoundary::PathNormalization,
        PlannedCliExitFreezeBoundary::ProtocolInventory,
        PlannedCliExitFreezeBoundary::Stable,
        PlannedCliExitFreezeBoundary::ExplicitExclusion,
    ])
    .expect("bounded CLI/exit freeze evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.cli-exit-freeze-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
