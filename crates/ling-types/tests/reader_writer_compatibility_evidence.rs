use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedReaderWriterCompatibilityBoundary {
    ReaderWriterCompatibility,
    ProtocolIdentity,
    SchemaIdentity,
    SchemaVersion,
    Marker,
    CurrentWriter,
    CurrentReader,
    CurrentCurrent,
    NMinusOne,
    NoPreviousVersion,
    PreviousVersion,
    PreviousMarker,
    ReaderVersion,
    WriterEvidence,
    ReaderAdapter,
    MigrationAdapter,
    CompatibilityDirectory,
    UnknownField,
    RejectUnknown,
    NamespacedExtension,
    UnspecifiedUnknown,
    MissingField,
    RejectRequired,
    ReaderDefault,
    DefaultValue,
    FutureVersion,
    CorruptInput,
    TruncatedInput,
    MalformedJson,
    WrongType,
    MissingRequired,
    ExtraField,
    CanonicalReencoding,
    CompactJsonLf,
    GoldenBytes,
    HashScheme,
    SizeLimit,
    DepthLimit,
    ResourceLimit,
    SecurityLimit,
    DeterministicMutation,
    CurrentOnly,
    ReaderNone,
    VersionGraph,
    CompatibilityEdge,
    UnsupportedEdge,
    FailClosed,
    BilingualDiagnostic,
    OriginalUtf8Span,
    UnicodeVersion,
    Bom,
    Crlf,
    CrossProcess,
    RepeatedBuild,
    PositiveFixture,
    NegativeFixture,
    CanonicalFixture,
    SchemaRegistry,
    ProtocolInventory,
    ExplicitExclusion,
}

impl PlannedReaderWriterCompatibilityBoundary {
    const ALL: [Self; 60] = [
        Self::ReaderWriterCompatibility,
        Self::ProtocolIdentity,
        Self::SchemaIdentity,
        Self::SchemaVersion,
        Self::Marker,
        Self::CurrentWriter,
        Self::CurrentReader,
        Self::CurrentCurrent,
        Self::NMinusOne,
        Self::NoPreviousVersion,
        Self::PreviousVersion,
        Self::PreviousMarker,
        Self::ReaderVersion,
        Self::WriterEvidence,
        Self::ReaderAdapter,
        Self::MigrationAdapter,
        Self::CompatibilityDirectory,
        Self::UnknownField,
        Self::RejectUnknown,
        Self::NamespacedExtension,
        Self::UnspecifiedUnknown,
        Self::MissingField,
        Self::RejectRequired,
        Self::ReaderDefault,
        Self::DefaultValue,
        Self::FutureVersion,
        Self::CorruptInput,
        Self::TruncatedInput,
        Self::MalformedJson,
        Self::WrongType,
        Self::MissingRequired,
        Self::ExtraField,
        Self::CanonicalReencoding,
        Self::CompactJsonLf,
        Self::GoldenBytes,
        Self::HashScheme,
        Self::SizeLimit,
        Self::DepthLimit,
        Self::ResourceLimit,
        Self::SecurityLimit,
        Self::DeterministicMutation,
        Self::CurrentOnly,
        Self::ReaderNone,
        Self::VersionGraph,
        Self::CompatibilityEdge,
        Self::UnsupportedEdge,
        Self::FailClosed,
        Self::BilingualDiagnostic,
        Self::OriginalUtf8Span,
        Self::UnicodeVersion,
        Self::Bom,
        Self::Crlf,
        Self::CrossProcess,
        Self::RepeatedBuild,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CanonicalFixture,
        Self::SchemaRegistry,
        Self::ProtocolInventory,
        Self::ExplicitExclusion,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReaderWriterCompatibilityInventory {
    boundaries: Box<[PlannedReaderWriterCompatibilityBoundary]>,
}

impl ReaderWriterCompatibilityInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedReaderWriterCompatibilityBoundary>,
    ) -> Result<Self, PlannedReaderWriterCompatibilityBoundary> {
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
        let mut bytes = b"ling.reader-writer-compatibility-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_reader_writer_compatibility_boundaries_are_complete_and_ordered() {
    let inventory =
        ReaderWriterCompatibilityInventory::new(PlannedReaderWriterCompatibilityBoundary::ALL)
            .expect("planned reader/writer compatibility boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedReaderWriterCompatibilityBoundary::ALL
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
fn reader_writer_evidence_is_order_independent_and_duplicate_checked() {
    let forward =
        ReaderWriterCompatibilityInventory::new(PlannedReaderWriterCompatibilityBoundary::ALL)
            .expect("forward inventory")
            .canonical_bytes();
    let reverse = ReaderWriterCompatibilityInventory::new(
        PlannedReaderWriterCompatibilityBoundary::ALL
            .into_iter()
            .rev(),
    )
    .expect("reverse inventory")
    .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = ReaderWriterCompatibilityInventory::new([
        PlannedReaderWriterCompatibilityBoundary::ReaderWriterCompatibility,
        PlannedReaderWriterCompatibilityBoundary::ReaderWriterCompatibility,
    ])
    .expect_err("duplicate reader/writer compatibility boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedReaderWriterCompatibilityBoundary::ReaderWriterCompatibility
    );
}

#[test]
fn reader_writer_evidence_has_no_compatibility_edge_authority() {
    let inventory = ReaderWriterCompatibilityInventory::new([
        PlannedReaderWriterCompatibilityBoundary::ReaderWriterCompatibility,
        PlannedReaderWriterCompatibilityBoundary::CurrentCurrent,
        PlannedReaderWriterCompatibilityBoundary::NMinusOne,
        PlannedReaderWriterCompatibilityBoundary::NoPreviousVersion,
        PlannedReaderWriterCompatibilityBoundary::MigrationAdapter,
        PlannedReaderWriterCompatibilityBoundary::UnsupportedEdge,
        PlannedReaderWriterCompatibilityBoundary::CorruptInput,
        PlannedReaderWriterCompatibilityBoundary::CanonicalReencoding,
        PlannedReaderWriterCompatibilityBoundary::SizeLimit,
        PlannedReaderWriterCompatibilityBoundary::DepthLimit,
        PlannedReaderWriterCompatibilityBoundary::ProtocolInventory,
        PlannedReaderWriterCompatibilityBoundary::ExplicitExclusion,
    ])
    .expect("bounded reader/writer compatibility evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.reader-writer-compatibility-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 12);
}
