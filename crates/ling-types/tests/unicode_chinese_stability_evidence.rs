use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedUnicodeChineseStabilityBoundary {
    UnicodeChineseStability,
    UnicodeVersion,
    VersionTuple,
    GeneratedTable,
    DataManifest,
    DataChecksum,
    ReproducibleGeneration,
    DependencyVersion,
    Utf8Decode,
    OriginalByteSpan,
    ScalarColumn,
    XidStart,
    XidContinue,
    Underscore,
    Nfc,
    OriginalSpelling,
    NormalizedName,
    ConfusableSkeleton,
    ScriptSet,
    MixedScript,
    IdentifierStatus,
    IdentifierType,
    ForbiddenControl,
    JoinControl,
    BidiControl,
    DefaultIgnorable,
    VariationSelector,
    PrivateUse,
    Noncharacter,
    Unassigned,
    Deprecated,
    PatternSyntax,
    PatternWhitespace,
    Text,
    Scalar,
    Byte,
    ChinesePackage,
    ChineseModule,
    ChineseSymbol,
    QualifiedName,
    Formatter,
    Lsp,
    Zed,
    Cli,
    WindowsPath,
    Utf16Projection,
    Crlf,
    Bom,
    AliasSyntax,
    LocalizedView,
    SemanticId,
    UpgradeRfc,
    MigrationReport,
    BilingualDiagnostic,
    OfflineEvidence,
    CrossProcess,
    PositiveFixture,
    NegativeFixture,
    AcceptedAuthority,
    ExplicitExclusion,
}

impl PlannedUnicodeChineseStabilityBoundary {
    const ALL: [Self; 60] = [
        Self::UnicodeChineseStability,
        Self::UnicodeVersion,
        Self::VersionTuple,
        Self::GeneratedTable,
        Self::DataManifest,
        Self::DataChecksum,
        Self::ReproducibleGeneration,
        Self::DependencyVersion,
        Self::Utf8Decode,
        Self::OriginalByteSpan,
        Self::ScalarColumn,
        Self::XidStart,
        Self::XidContinue,
        Self::Underscore,
        Self::Nfc,
        Self::OriginalSpelling,
        Self::NormalizedName,
        Self::ConfusableSkeleton,
        Self::ScriptSet,
        Self::MixedScript,
        Self::IdentifierStatus,
        Self::IdentifierType,
        Self::ForbiddenControl,
        Self::JoinControl,
        Self::BidiControl,
        Self::DefaultIgnorable,
        Self::VariationSelector,
        Self::PrivateUse,
        Self::Noncharacter,
        Self::Unassigned,
        Self::Deprecated,
        Self::PatternSyntax,
        Self::PatternWhitespace,
        Self::Text,
        Self::Scalar,
        Self::Byte,
        Self::ChinesePackage,
        Self::ChineseModule,
        Self::ChineseSymbol,
        Self::QualifiedName,
        Self::Formatter,
        Self::Lsp,
        Self::Zed,
        Self::Cli,
        Self::WindowsPath,
        Self::Utf16Projection,
        Self::Crlf,
        Self::Bom,
        Self::AliasSyntax,
        Self::LocalizedView,
        Self::SemanticId,
        Self::UpgradeRfc,
        Self::MigrationReport,
        Self::BilingualDiagnostic,
        Self::OfflineEvidence,
        Self::CrossProcess,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::AcceptedAuthority,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct UnicodeChineseStabilityInventory {
    boundaries: Box<[PlannedUnicodeChineseStabilityBoundary]>,
}

impl UnicodeChineseStabilityInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedUnicodeChineseStabilityBoundary>,
    ) -> Result<Self, PlannedUnicodeChineseStabilityBoundary> {
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
        let mut bytes = b"ling.unicode-chinese-stability-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_unicode_chinese_stability_boundaries_are_complete_and_ordered() {
    let inventory =
        UnicodeChineseStabilityInventory::new(PlannedUnicodeChineseStabilityBoundary::ALL)
            .expect("planned Unicode/Chinese stability boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedUnicodeChineseStabilityBoundary::ALL
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
fn unicode_stability_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        UnicodeChineseStabilityInventory::new(PlannedUnicodeChineseStabilityBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = UnicodeChineseStabilityInventory::new(
        PlannedUnicodeChineseStabilityBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = UnicodeChineseStabilityInventory::new([
        PlannedUnicodeChineseStabilityBoundary::UnicodeVersion,
        PlannedUnicodeChineseStabilityBoundary::UnicodeVersion,
    ])
    .expect_err("duplicate Unicode stability boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedUnicodeChineseStabilityBoundary::UnicodeVersion
    );
}

#[test]
fn observation_has_no_upgrade_localization_or_tool_protocol_authority() {
    let inventory = UnicodeChineseStabilityInventory::new([
        PlannedUnicodeChineseStabilityBoundary::UnicodeChineseStability,
        PlannedUnicodeChineseStabilityBoundary::UnicodeVersion,
        PlannedUnicodeChineseStabilityBoundary::ChineseSymbol,
        PlannedUnicodeChineseStabilityBoundary::Formatter,
        PlannedUnicodeChineseStabilityBoundary::Lsp,
        PlannedUnicodeChineseStabilityBoundary::Zed,
        PlannedUnicodeChineseStabilityBoundary::WindowsPath,
        PlannedUnicodeChineseStabilityBoundary::AliasSyntax,
        PlannedUnicodeChineseStabilityBoundary::LocalizedView,
        PlannedUnicodeChineseStabilityBoundary::UpgradeRfc,
        PlannedUnicodeChineseStabilityBoundary::AcceptedAuthority,
        PlannedUnicodeChineseStabilityBoundary::ExplicitExclusion,
    ])
    .expect("bounded Unicode/Chinese stability evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.unicode-chinese-stability-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
