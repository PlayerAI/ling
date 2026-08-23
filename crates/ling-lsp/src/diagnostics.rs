use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity};
use ling_source::{ByteOffset, LspPosition, PositionEncoding, PositionError, SourceFile};
use serde_json::{Value, json};

use super::document_identity;

/// Version marker placed in every Experimental diagnostic `data` object.
pub const DIAGNOSTIC_PROTOCOL_VERSION: &str = "ling.lsp.diagnostic/0.1";

/// One exact logical source and its public path-free editor URI.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticSource {
    uri: String,
    source: SourceFile,
}

impl DiagnosticSource {
    #[must_use]
    pub fn new(uri: impl Into<String>, source: SourceFile) -> Self {
        Self {
            uri: uri.into(),
            source,
        }
    }

    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    #[must_use]
    pub const fn source(&self) -> &SourceFile {
        &self.source
    }
}

/// One explicit secondary compiler label for LSP `relatedInformation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelatedDiagnosticLabel {
    span: DiagnosticSpan,
    message_zh: String,
    message_en: String,
}

impl RelatedDiagnosticLabel {
    #[must_use]
    pub fn new(
        span: DiagnosticSpan,
        message_zh: impl Into<String>,
        message_en: impl Into<String>,
    ) -> Self {
        Self {
            span,
            message_zh: message_zh.into(),
            message_en: message_en.into(),
        }
    }

    #[must_use]
    pub const fn span(&self) -> &DiagnosticSpan {
        &self.span
    }

    #[must_use]
    pub fn message_zh(&self) -> &str {
        &self.message_zh
    }

    #[must_use]
    pub fn message_en(&self) -> &str {
        &self.message_en
    }
}

/// One compiler diagnostic plus adapter-only related labels.
#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticAdapterInput {
    diagnostic: Diagnostic,
    related: Box<[RelatedDiagnosticLabel]>,
}

impl DiagnosticAdapterInput {
    #[must_use]
    pub fn new(diagnostic: Diagnostic) -> Self {
        Self {
            diagnostic,
            related: Box::new([]),
        }
    }

    #[must_use]
    pub fn with_related(mut self, related: Vec<RelatedDiagnosticLabel>) -> Self {
        self.related = related.into_boxed_slice();
        self
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }

    #[must_use]
    pub fn related(&self) -> &[RelatedDiagnosticLabel] {
        &self.related
    }
}

/// One adapted LSP diagnostic and the document URI that owns its primary span.
#[derive(Clone, Debug, PartialEq)]
pub struct AdaptedDiagnostic {
    uri: String,
    value: Value,
}

impl AdaptedDiagnostic {
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }

    #[must_use]
    pub fn into_value(self) -> Value {
        self.value
    }
}

/// Failure to build a complete RFC-0031 diagnostic projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiagnosticAdapterError {
    EmptySources,
    InvalidSourceUri { uri: String },
    SourceIdentityMismatch { uri: String, source: String },
    DuplicateSourceName { source: String },
    DuplicateSourceUri { uri: String },
    MissingPrimarySpan { code: String },
    UnknownSource { source: String },
    Projection(DiagnosticProjectionError),
}

impl std::fmt::Display for DiagnosticAdapterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySources => formatter.write_str("diagnostic source set is empty"),
            Self::InvalidSourceUri { uri } => write!(formatter, "invalid diagnostic URI {uri:?}"),
            Self::SourceIdentityMismatch { uri, source } => write!(
                formatter,
                "diagnostic URI {uri:?} does not identify source {source:?}"
            ),
            Self::DuplicateSourceName { source } => {
                write!(formatter, "duplicate diagnostic source {source:?}")
            }
            Self::DuplicateSourceUri { uri } => {
                write!(formatter, "duplicate diagnostic URI {uri:?}")
            }
            Self::MissingPrimarySpan { code } => {
                write!(formatter, "diagnostic {code} has no primary span")
            }
            Self::UnknownSource { source } => {
                write!(formatter, "unknown diagnostic source {source:?}")
            }
            Self::Projection(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for DiagnosticAdapterError {}

impl From<DiagnosticProjectionError> for DiagnosticAdapterError {
    fn from(error: DiagnosticProjectionError) -> Self {
        Self::Projection(error)
    }
}

/// Adapts a complete diagnostic set into canonical RFC-0031 LSP values.
pub fn adapt_diagnostics(
    encoding: PositionEncoding,
    sources: &[DiagnosticSource],
    inputs: &[DiagnosticAdapterInput],
) -> Result<Box<[AdaptedDiagnostic]>, DiagnosticAdapterError> {
    if sources.is_empty() {
        return Err(DiagnosticAdapterError::EmptySources);
    }
    let mut by_name = BTreeMap::new();
    let mut uris = BTreeSet::new();
    for source in sources {
        let identity = document_identity(source.uri()).map_err(|_| {
            DiagnosticAdapterError::InvalidSourceUri {
                uri: source.uri().to_owned(),
            }
        })?;
        if identity.temporary {
            return Err(DiagnosticAdapterError::InvalidSourceUri {
                uri: source.uri().to_owned(),
            });
        }
        if identity.logical_name != source.source().name() {
            return Err(DiagnosticAdapterError::SourceIdentityMismatch {
                uri: source.uri().to_owned(),
                source: source.source().name().to_owned(),
            });
        }
        if !uris.insert(source.uri()) {
            return Err(DiagnosticAdapterError::DuplicateSourceUri {
                uri: source.uri().to_owned(),
            });
        }
        if by_name.insert(source.source().name(), source).is_some() {
            return Err(DiagnosticAdapterError::DuplicateSourceName {
                source: source.source().name().to_owned(),
            });
        }
    }

    let mut output = Vec::with_capacity(inputs.len());
    for (ordinal, input) in inputs.iter().enumerate() {
        let diagnostic = input.diagnostic();
        let primary = diagnostic.primary_span().ok_or_else(|| {
            DiagnosticAdapterError::MissingPrimarySpan {
                code: diagnostic.code().as_str().to_owned(),
            }
        })?;
        let source =
            by_name
                .get(primary.file())
                .ok_or_else(|| DiagnosticAdapterError::UnknownSource {
                    source: primary.file().to_owned(),
                })?;
        let range = project_span(source.source(), primary, encoding)?;
        let mut related_information = Vec::with_capacity(input.related().len());
        for related in input.related() {
            let related_source = by_name.get(related.span().file()).ok_or_else(|| {
                DiagnosticAdapterError::UnknownSource {
                    source: related.span().file().to_owned(),
                }
            })?;
            let related_range = project_span(related_source.source(), related.span(), encoding)?;
            related_information.push(json!({
                "location": {
                    "range": range_value(related_range),
                    "uri": related_source.uri(),
                },
                "message": bilingual(related.message_zh(), related.message_en()),
            }));
        }
        let severity = match diagnostic.severity() {
            Severity::Error => 1,
            Severity::Warning => 2,
            Severity::Note => 3,
        };
        let value = json!({
            "code": diagnostic.code().as_str(),
            "data": {
                "facts": diagnostic.facts(),
                "repairs": diagnostic.repairs(),
                "semanticId": diagnostic.semantic_id(),
                "version": DIAGNOSTIC_PROTOCOL_VERSION,
            },
            "message": bilingual(diagnostic.message_zh(), diagnostic.message_en()),
            "range": range_value(range),
            "relatedInformation": related_information,
            "severity": severity,
            "source": "ling",
        });
        let tie_breaker = u64::try_from(ordinal).unwrap_or(u64::MAX);
        let key = DiagnosticOrderKey::new(
            primary.file(),
            primary.start_byte(),
            primary.end_byte(),
            diagnostic.code().as_str(),
            tie_breaker,
        );
        output.push((
            key,
            AdaptedDiagnostic {
                uri: source.uri().to_owned(),
                value,
            },
        ));
    }
    output.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(output
        .into_iter()
        .map(|(_, diagnostic)| diagnostic)
        .collect::<Vec<_>>()
        .into_boxed_slice())
}

fn bilingual(chinese: &str, english: &str) -> String {
    format!("{chinese} / {english}")
}

fn range_value(range: DiagnosticPositionRange) -> Value {
    json!({
        "end": {
            "character": range.end().character(),
            "line": range.end().line(),
        },
        "start": {
            "character": range.start().character(),
            "line": range.start().line(),
        },
    })
}

/// Internal canonical ordering key used by the public diagnostic adapter.
///
/// The key preserves logical names and original UTF-8 byte offsets. It carries
/// no protocol, snapshot, severity, message, or suppression state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DiagnosticOrderKey {
    file: String,
    start_byte: u64,
    code: String,
    end_byte: u64,
    tie_breaker: u64,
}

impl DiagnosticOrderKey {
    #[must_use]
    pub(crate) fn new(
        file: impl Into<String>,
        start_byte: u64,
        end_byte: u64,
        code: impl Into<String>,
        tie_breaker: u64,
    ) -> Self {
        Self {
            file: file.into(),
            start_byte,
            code: code.into(),
            end_byte,
            tie_breaker,
        }
    }

    #[must_use]
    pub(crate) fn file(&self) -> &str {
        &self.file
    }

    #[must_use]
    pub(crate) const fn start_byte(&self) -> u64 {
        self.start_byte
    }

    #[must_use]
    pub(crate) const fn end_byte(&self) -> u64 {
        self.end_byte
    }

    #[must_use]
    pub(crate) fn code(&self) -> &str {
        &self.code
    }

    #[must_use]
    pub(crate) const fn tie_breaker(&self) -> u64 {
        self.tie_breaker
    }
}

impl Ord for DiagnosticOrderKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file
            .cmp(&other.file)
            .then_with(|| self.start_byte.cmp(&other.start_byte))
            .then_with(|| self.code.cmp(&other.code))
            .then_with(|| self.end_byte.cmp(&other.end_byte))
            .then_with(|| self.tie_breaker.cmp(&other.tie_breaker))
    }
}

impl PartialOrd for DiagnosticOrderKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// An internal source-layer projection of one diagnostic span.
///
/// This value carries no severity, message, snapshot, URI, version, or wire
/// representation; RFC-0031's adapter owns those independent fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DiagnosticPositionRange {
    start: LspPosition,
    end: LspPosition,
}

impl DiagnosticPositionRange {
    #[must_use]
    pub(crate) const fn start(self) -> LspPosition {
        self.start
    }

    #[must_use]
    pub(crate) const fn end(self) -> LspPosition {
        self.end
    }
}

/// A failure while projecting a compiler diagnostic span into editor
/// positions. No clamping or path/URI interpretation is performed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiagnosticProjectionError {
    FileMismatch { expected: String, actual: String },
    OffsetOutOfRange { offset: u64 },
    ReversedSpan { start: u64, end: u64 },
    Position(PositionError),
}

impl std::fmt::Display for DiagnosticProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileMismatch { expected, actual } => write!(
                formatter,
                "diagnostic span names `{actual}` but source is `{expected}`"
            ),
            Self::OffsetOutOfRange { offset } => {
                write!(formatter, "diagnostic byte offset {offset} exceeds u32")
            }
            Self::ReversedSpan { start, end } => {
                write!(formatter, "diagnostic span is reversed: {start}..{end}")
            }
            Self::Position(error) => {
                write!(formatter, "diagnostic position projection failed: {error}")
            }
        }
    }
}

impl std::error::Error for DiagnosticProjectionError {}

impl From<PositionError> for DiagnosticProjectionError {
    fn from(error: PositionError) -> Self {
        Self::Position(error)
    }
}

/// Projects one compiler diagnostic's original-byte span into an explicit
/// position encoding using the authoritative source map.
pub(crate) fn project_span(
    source: &SourceFile,
    span: &DiagnosticSpan,
    encoding: PositionEncoding,
) -> Result<DiagnosticPositionRange, DiagnosticProjectionError> {
    if span.file() != source.name() {
        return Err(DiagnosticProjectionError::FileMismatch {
            expected: source.name().to_owned(),
            actual: span.file().to_owned(),
        });
    }
    if span.start_byte() > span.end_byte() {
        return Err(DiagnosticProjectionError::ReversedSpan {
            start: span.start_byte(),
            end: span.end_byte(),
        });
    }
    let start = u32::try_from(span.start_byte())
        .map(ByteOffset::new)
        .map_err(|_| DiagnosticProjectionError::OffsetOutOfRange {
            offset: span.start_byte(),
        })?;
    let end = u32::try_from(span.end_byte())
        .map(ByteOffset::new)
        .map_err(|_| DiagnosticProjectionError::OffsetOutOfRange {
            offset: span.end_byte(),
        })?;
    Ok(DiagnosticPositionRange {
        start: source.lsp_position(start, encoding)?,
        end: source.lsp_position(end, encoding)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(text: &[u8]) -> SourceFile {
        SourceFile::from_bytes(ling_source::SourceId::new(7), "main.ling", text.to_vec())
            .expect("test source is valid UTF-8")
    }

    #[test]
    fn canonical_order_preserves_file_bytes_span_and_code() {
        let mut keys = [
            DiagnosticOrderKey::new("z.ling", 0, 1, "L-TYPE-0001", 0),
            DiagnosticOrderKey::new("a.ling", 8, 9, "L-TYPE-0001", 0),
            DiagnosticOrderKey::new("a.ling", 2, 4, "L-TYPE-0001", 0),
            DiagnosticOrderKey::new("a.ling", 2, 4, "L-LEX-0001", 0),
        ];
        keys.sort();
        assert_eq!(
            keys.iter().map(|key| key.file()).collect::<Vec<_>>(),
            vec!["a.ling", "a.ling", "a.ling", "z.ling"]
        );
        assert_eq!(keys[0].start_byte(), 2);
        assert_eq!(keys[0].code(), "L-LEX-0001");
        assert_eq!(keys[1].code(), "L-TYPE-0001");
        assert_eq!(keys[2].start_byte(), 8);
    }

    #[test]
    fn tie_breaker_distinguishes_equal_diagnostic_facts() {
        let first = DiagnosticOrderKey::new("凌.ling", 3, 7, "L-LEX-0001", 1);
        let second = DiagnosticOrderKey::new("凌.ling", 3, 7, "L-LEX-0001", 2);
        assert!(first < second);
        assert_eq!(first.end_byte(), 7);
        assert_eq!(second.tie_breaker(), 2);
    }

    #[test]
    fn repeated_sorting_is_deterministic_for_crlf_byte_offsets() {
        let input = vec![
            DiagnosticOrderKey::new("main.ling", 5, 7, "L-SYNTAX-0002", 0),
            DiagnosticOrderKey::new("main.ling", 5, 6, "L-SYNTAX-0001", 0),
            DiagnosticOrderKey::new("main.ling", 12, 13, "L-SYNTAX-0001", 0),
        ];
        let mut left = input.clone();
        let mut right = input;
        left.sort();
        right.sort();
        assert_eq!(left, right);
    }

    #[test]
    fn projects_original_diagnostic_bytes_for_each_encoding() {
        let source = source("\u{feff}a\r\n凌😀z\n".as_bytes());
        let span = DiagnosticSpan::at("main.ling", 6, 14);
        let expected = [
            (
                PositionEncoding::Utf8,
                LspPosition::new(1, 0),
                LspPosition::new(1, 8),
            ),
            (
                PositionEncoding::Utf16,
                LspPosition::new(1, 0),
                LspPosition::new(1, 4),
            ),
            (
                PositionEncoding::Utf32,
                LspPosition::new(1, 0),
                LspPosition::new(1, 3),
            ),
        ];
        for (encoding, start, end) in expected {
            let range = project_span(&source, &span, encoding).expect("span projects");
            assert_eq!((range.start(), range.end()), (start, end));
        }
    }

    #[test]
    fn projection_rejects_identity_range_and_offset_failures_without_clamping() {
        let source = source(b"hello\n");
        assert!(matches!(
            project_span(
                &source,
                &DiagnosticSpan::at("other.ling", 0, 1),
                PositionEncoding::Utf8,
            ),
            Err(DiagnosticProjectionError::FileMismatch { .. })
        ));
        assert!(matches!(
            project_span(
                &source,
                &DiagnosticSpan::at_u64("main.ling", 4, 2),
                PositionEncoding::Utf8,
            ),
            Err(DiagnosticProjectionError::ReversedSpan { .. })
        ));
        assert!(matches!(
            project_span(
                &source,
                &DiagnosticSpan::at_u64(
                    "main.ling",
                    u64::from(u32::MAX) + 1,
                    u64::from(u32::MAX) + 1
                ),
                PositionEncoding::Utf8,
            ),
            Err(DiagnosticProjectionError::OffsetOutOfRange { .. })
        ));
        assert!(matches!(
            project_span(
                &source,
                &DiagnosticSpan::at("main.ling", 0, 99),
                PositionEncoding::Utf8,
            ),
            Err(DiagnosticProjectionError::Position(_))
        ));
    }
}
