use std::cmp::Ordering;

use ling_diagnostics::DiagnosticSpan;
use ling_source::{ByteOffset, LspPosition, PositionEncoding, PositionError, SourceFile};

/// Internal canonical ordering key for a future LSP diagnostic projection.
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
/// representation. The public diagnostic adapter remains deferred.
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
pub(crate) enum DiagnosticProjectionError {
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
