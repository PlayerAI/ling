//! UTF-8 source storage, newline normalization, and byte-accurate span mapping.

mod position;
mod vfs;

pub use position::{
    LspPosition, LspPositionEdit, LspPositionEditError, PositionEncoding, PositionError,
    SUPPORTED_POSITION_ENCODINGS, negotiate_position_encoding,
};
pub use vfs::{
    ChangeEvent, FileOrigin, FileSnapshot, InputChange, Revision, VfsError, VirtualFileSystem,
    WorkspaceInput, WorkspaceSnapshot, WorkspaceStateSnapshot, validate_logical_name,
};

use std::error::Error;
use std::fmt;

const BYTE_ORDER_MARK: char = '\u{feff}';
const BYTE_ORDER_MARK_LEN: usize = 3;

/// Identifies one source file within a compiler session.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceId(u32);

impl SourceId {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// A byte offset in the original, unnormalized UTF-8 source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ByteOffset(u32);

impl ByteOffset {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// One in-process replacement over the original UTF-8 byte snapshot.
///
/// The range is half-open and is interpreted against the source snapshot that
/// is current when the edit is applied. This value deliberately carries no
/// URI, document version, negotiated position, or transport information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Utf8Edit {
    start: ByteOffset,
    end: ByteOffset,
    replacement: Vec<u8>,
}

impl Utf8Edit {
    /// Creates a replacement over the original UTF-8 byte range `[start, end)`.
    /// Range and replacement validation is performed by `SourceFile` so a
    /// caller can construct a value before attempting an atomic application.
    #[must_use]
    pub fn new(start: ByteOffset, end: ByteOffset, replacement: impl Into<Vec<u8>>) -> Self {
        Self {
            start,
            end,
            replacement: replacement.into(),
        }
    }

    #[must_use]
    pub const fn start(&self) -> ByteOffset {
        self.start
    }

    #[must_use]
    pub const fn end(&self) -> ByteOffset {
        self.end
    }

    #[must_use]
    pub fn replacement(&self) -> &[u8] {
        &self.replacement
    }
}

/// A byte offset in the BOM-free, LF-normalized lexical view.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LexicalOffset(u32);

impl LexicalOffset {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// A half-open byte span in the original source.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Span {
    source: SourceId,
    start: ByteOffset,
    end: ByteOffset,
}

impl Span {
    pub fn new(source: SourceId, start: ByteOffset, end: ByteOffset) -> Result<Self, InvalidSpan> {
        if start > end {
            return Err(InvalidSpan { start, end });
        }

        Ok(Self { source, start, end })
    }

    #[must_use]
    pub const fn source(self) -> SourceId {
        self.source
    }

    #[must_use]
    pub const fn start(self) -> ByteOffset {
        self.start
    }

    #[must_use]
    pub const fn end(self) -> ByteOffset {
        self.end
    }
}

/// A half-open byte span in the normalized lexical view.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LexicalSpan {
    source: SourceId,
    start: LexicalOffset,
    end: LexicalOffset,
}

impl LexicalSpan {
    pub fn new(
        source: SourceId,
        start: LexicalOffset,
        end: LexicalOffset,
    ) -> Result<Self, InvalidLexicalSpan> {
        if start > end {
            return Err(InvalidLexicalSpan { start, end });
        }

        Ok(Self { source, start, end })
    }

    #[must_use]
    pub const fn source(self) -> SourceId {
        self.source
    }

    #[must_use]
    pub const fn start(self) -> LexicalOffset {
        self.start
    }

    #[must_use]
    pub const fn end(self) -> LexicalOffset {
        self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidSpan {
    pub start: ByteOffset,
    pub end: ByteOffset,
}

impl fmt::Display for InvalidSpan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "source span starts at byte {} but ends at byte {}",
            self.start.get(),
            self.end.get()
        )
    }
}

impl Error for InvalidSpan {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidLexicalSpan {
    pub start: LexicalOffset,
    pub end: LexicalOffset,
}

impl fmt::Display for InvalidLexicalSpan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "lexical span starts at byte {} but ends at byte {}",
            self.start.get(),
            self.end.get()
        )
    }
}

impl Error for InvalidLexicalSpan {}

/// One-based line and Unicode-scalar column for human diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineColumn {
    line: u32,
    column: u32,
}

impl LineColumn {
    #[must_use]
    pub const fn line(self) -> u32 {
        self.line
    }

    #[must_use]
    pub const fn column(self) -> u32 {
        self.column
    }
}

/// Maps byte offsets in the lexical view back to original source bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceMap {
    source: SourceId,
    original_len: ByteOffset,
    lexical_to_original: Box<[ByteOffset]>,
}

impl SourceMap {
    #[must_use]
    pub const fn source(&self) -> SourceId {
        self.source
    }

    #[must_use]
    pub const fn original_len(&self) -> ByteOffset {
        self.original_len
    }

    #[must_use]
    pub fn lexical_len(&self) -> LexicalOffset {
        let length = self.lexical_to_original.len() - 1;
        LexicalOffset::new(u32::try_from(length).expect("source length was checked"))
    }

    pub fn original_offset(&self, lexical: LexicalOffset) -> Result<ByteOffset, SourceMapError> {
        self.lexical_to_original
            .get(lexical.get() as usize)
            .copied()
            .ok_or(SourceMapError::LexicalOffsetOutOfBounds {
                offset: lexical,
                lexical_len: self.lexical_len(),
            })
    }

    pub fn lexical_offset(&self, original: ByteOffset) -> Result<LexicalOffset, SourceMapError> {
        if original > self.original_len {
            return Err(SourceMapError::OriginalOffsetOutOfBounds {
                offset: original,
                original_len: self.original_len,
            });
        }

        let index = self
            .lexical_to_original
            .binary_search(&original)
            .map_err(|_| SourceMapError::UnmappedOriginalOffset { offset: original })?;
        Ok(LexicalOffset::new(
            u32::try_from(index).expect("source length was checked"),
        ))
    }

    pub fn original_span(&self, lexical: LexicalSpan) -> Result<Span, SourceMapError> {
        if lexical.source() != self.source {
            return Err(SourceMapError::WrongSource {
                expected: self.source,
                actual: lexical.source(),
            });
        }

        let start = self.original_offset(lexical.start())?;
        let end = self.original_offset(lexical.end())?;
        Span::new(self.source, start, end).map_err(|_| SourceMapError::InvalidMappedSpan)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceMapError {
    WrongSource {
        expected: SourceId,
        actual: SourceId,
    },
    LexicalOffsetOutOfBounds {
        offset: LexicalOffset,
        lexical_len: LexicalOffset,
    },
    OriginalOffsetOutOfBounds {
        offset: ByteOffset,
        original_len: ByteOffset,
    },
    UnmappedOriginalOffset {
        offset: ByteOffset,
    },
    NotCharacterBoundary {
        offset: ByteOffset,
    },
    InvalidMappedSpan,
}

impl fmt::Display for SourceMapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongSource { expected, actual } => write!(
                formatter,
                "source map belongs to source {} but span belongs to source {}",
                expected.get(),
                actual.get()
            ),
            Self::LexicalOffsetOutOfBounds {
                offset,
                lexical_len,
            } => write!(
                formatter,
                "lexical byte offset {} exceeds source length {}",
                offset.get(),
                lexical_len.get()
            ),
            Self::OriginalOffsetOutOfBounds {
                offset,
                original_len,
            } => write!(
                formatter,
                "original byte offset {} exceeds source length {}",
                offset.get(),
                original_len.get()
            ),
            Self::UnmappedOriginalOffset { offset } => write!(
                formatter,
                "original byte offset {} is inside a normalized source sequence",
                offset.get()
            ),
            Self::NotCharacterBoundary { offset } => write!(
                formatter,
                "original byte offset {} is not a UTF-8 character boundary",
                offset.get()
            ),
            Self::InvalidMappedSpan => formatter.write_str("normalized span mapped backwards"),
        }
    }
}

impl Error for SourceMapError {}

/// A validated source file and its normalized lexical view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceFile {
    id: SourceId,
    name: String,
    original: String,
    lexical: String,
    had_bom: bool,
    source_map: SourceMap,
    line_starts: Box<[LexicalOffset]>,
}

impl SourceFile {
    pub fn from_bytes(
        id: SourceId,
        name: impl Into<String>,
        bytes: Vec<u8>,
    ) -> Result<Self, SourceError> {
        if bytes.len() > u32::MAX as usize {
            return Err(SourceError::TooLarge {
                byte_len: bytes.len(),
            });
        }

        let original = String::from_utf8(bytes).map_err(|error| {
            let utf8_error = error.utf8_error();
            SourceError::InvalidUtf8 {
                valid_up_to: utf8_error.valid_up_to(),
                error_len: utf8_error.error_len(),
            }
        })?;
        let had_bom = original.starts_with(BYTE_ORDER_MARK);
        let content_start = usize::from(had_bom) * BYTE_ORDER_MARK_LEN;

        if let Some((relative_offset, _)) = original[content_start..]
            .char_indices()
            .find(|(_, character)| *character == BYTE_ORDER_MARK)
        {
            return Err(SourceError::MisplacedByteOrderMark {
                byte_offset: content_start + relative_offset,
            });
        }

        let (lexical, lexical_to_original) = normalize_newlines(&original, content_start);
        let line_starts = lexical_line_starts(&lexical);
        let original_len = ByteOffset::new(
            u32::try_from(original.len()).expect("source length was checked before decoding"),
        );

        Ok(Self {
            id,
            name: name.into(),
            original,
            lexical,
            had_bom,
            source_map: SourceMap {
                source: id,
                original_len,
                lexical_to_original: lexical_to_original.into_boxed_slice(),
            },
            line_starts: line_starts.into_boxed_slice(),
        })
    }

    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn original_text(&self) -> &str {
        &self.original
    }

    /// Consumes this validated source and returns its exact original UTF-8 bytes.
    ///
    /// This is useful for pipeline stages that must retain the same byte snapshot
    /// after parsing without rereading a mutable host file.
    #[must_use]
    pub fn into_original_bytes(self) -> Vec<u8> {
        self.original.into_bytes()
    }

    /// Applies one UTF-8 byte-range replacement and returns a new source
    /// snapshot.
    ///
    /// The original source is immutable: invalid ranges, non-character
    /// boundaries, invalid replacement text, and oversized results return an
    /// error without producing a partially updated snapshot. This is an
    /// in-process source primitive, not an LSP edit or transaction API.
    pub fn apply_utf8_edit(&self, edit: &Utf8Edit) -> Result<Self, SourceEditError> {
        self.apply_utf8_edits(std::slice::from_ref(edit))
    }

    /// Applies edits in the supplied order, validating each edit against the
    /// snapshot produced by the preceding edit. The operation is atomic from
    /// the caller's perspective because `self` is never mutated and no
    /// intermediate snapshot is returned on failure.
    pub fn apply_utf8_edits(&self, edits: &[Utf8Edit]) -> Result<Self, SourceEditError> {
        let mut current = self.clone();
        for edit in edits {
            current = current.apply_one_utf8_edit(edit)?;
        }
        Ok(current)
    }

    fn apply_one_utf8_edit(&self, edit: &Utf8Edit) -> Result<Self, SourceEditError> {
        if edit.start > edit.end {
            return Err(SourceEditError::ReversedRange {
                start: edit.start,
                end: edit.end,
            });
        }

        let source_len = self.original.len();
        let start = edit.start.get() as usize;
        let end = edit.end.get() as usize;
        if start > source_len {
            return Err(SourceEditError::OffsetOutOfBounds {
                offset: edit.start,
                source_len: ByteOffset::new(
                    u32::try_from(source_len).expect("source length is bounded"),
                ),
            });
        }
        if end > source_len {
            return Err(SourceEditError::OffsetOutOfBounds {
                offset: edit.end,
                source_len: ByteOffset::new(
                    u32::try_from(source_len).expect("source length is bounded"),
                ),
            });
        }
        if !is_edit_boundary(&self.original, start) {
            return Err(SourceEditError::NotCharacterBoundary { offset: edit.start });
        }
        if !is_edit_boundary(&self.original, end) {
            return Err(SourceEditError::NotCharacterBoundary { offset: edit.end });
        }

        let retained_len = source_len - (end - start);
        let maximum = u32::MAX as usize;
        if retained_len > maximum || edit.replacement.len() > maximum - retained_len {
            return Err(SourceEditError::ResultTooLarge);
        }
        let result_len = retained_len + edit.replacement.len();
        let original = self.original.as_bytes();
        let mut bytes = Vec::with_capacity(result_len);
        bytes.extend_from_slice(&original[..start]);
        bytes.extend_from_slice(&edit.replacement);
        bytes.extend_from_slice(&original[end..]);

        debug_assert_eq!(bytes.len(), result_len);
        Self::from_bytes(self.id, self.name.clone(), bytes).map_err(SourceEditError::InvalidSource)
    }

    #[must_use]
    pub fn lexical_text(&self) -> &str {
        &self.lexical
    }

    #[must_use]
    pub const fn had_bom(&self) -> bool {
        self.had_bom
    }

    #[must_use]
    pub const fn source_map(&self) -> &SourceMap {
        &self.source_map
    }

    pub fn line_column(&self, offset: ByteOffset) -> Result<LineColumn, SourceMapError> {
        let lexical_offset = self.source_map.lexical_offset(offset)?;
        let lexical_index = lexical_offset.get() as usize;
        if !self.lexical.is_char_boundary(lexical_index) {
            return Err(SourceMapError::NotCharacterBoundary { offset });
        }

        let line_index = self
            .line_starts
            .partition_point(|line_start| *line_start <= lexical_offset)
            .saturating_sub(1);
        let line_start = self.line_starts[line_index].get() as usize;
        let column = self.lexical[line_start..lexical_index].chars().count() + 1;

        Ok(LineColumn {
            line: u32::try_from(line_index + 1).expect("line count is bounded by source length"),
            column: u32::try_from(column).expect("column is bounded by source length"),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceError {
    InvalidUtf8 {
        valid_up_to: usize,
        error_len: Option<usize>,
    },
    MisplacedByteOrderMark {
        byte_offset: usize,
    },
    TooLarge {
        byte_len: usize,
    },
}

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUtf8 {
                valid_up_to,
                error_len,
            } => write!(
                formatter,
                "source is not valid UTF-8 at byte {valid_up_to} (invalid length {error_len:?})"
            ),
            Self::MisplacedByteOrderMark { byte_offset } => write!(
                formatter,
                "UTF-8 byte-order mark is only allowed at byte 0, found at byte {byte_offset}"
            ),
            Self::TooLarge { byte_len } => write!(
                formatter,
                "source contains {byte_len} bytes; the maximum supported size is {}",
                u32::MAX
            ),
        }
    }
}

impl Error for SourceError {}

/// Failure while applying an in-process UTF-8 source edit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceEditError {
    ReversedRange {
        start: ByteOffset,
        end: ByteOffset,
    },
    OffsetOutOfBounds {
        offset: ByteOffset,
        source_len: ByteOffset,
    },
    NotCharacterBoundary {
        offset: ByteOffset,
    },
    ResultTooLarge,
    InvalidSource(SourceError),
}

impl fmt::Display for SourceEditError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReversedRange { start, end } => write!(
                formatter,
                "UTF-8 edit range starts at byte {} but ends at byte {}",
                start.get(),
                end.get()
            ),
            Self::OffsetOutOfBounds { offset, source_len } => write!(
                formatter,
                "UTF-8 edit byte offset {} exceeds source length {}",
                offset.get(),
                source_len.get()
            ),
            Self::NotCharacterBoundary { offset } => write!(
                formatter,
                "UTF-8 edit byte offset {} is not a character boundary",
                offset.get()
            ),
            Self::ResultTooLarge => {
                formatter.write_str("UTF-8 edit result exceeds the u32 source limit")
            }
            Self::InvalidSource(error) => write!(formatter, "edited source is invalid: {error}"),
        }
    }
}

impl Error for SourceEditError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidSource(error) => Some(error),
            _ => None,
        }
    }
}

fn is_edit_boundary(original: &str, offset: usize) -> bool {
    if !original.is_char_boundary(offset) {
        return false;
    }

    let bytes = original.as_bytes();
    !(offset > 0 && offset < bytes.len() && bytes[offset - 1] == b'\r' && bytes[offset] == b'\n')
}

fn normalize_newlines(original: &str, content_start: usize) -> (String, Vec<ByteOffset>) {
    let mut lexical = String::with_capacity(original.len() - content_start);
    let mut lexical_to_original = Vec::with_capacity(original.len() - content_start + 1);
    lexical_to_original.push(ByteOffset::new(
        u32::try_from(content_start).expect("source length was checked"),
    ));

    let bytes = original.as_bytes();
    let mut original_index = content_start;
    while original_index < bytes.len() {
        if bytes[original_index] == b'\r' {
            lexical.push('\n');
            original_index += 1;
            if original_index < bytes.len() && bytes[original_index] == b'\n' {
                original_index += 1;
            }
            lexical_to_original.push(ByteOffset::new(
                u32::try_from(original_index).expect("source length was checked"),
            ));
            continue;
        }

        let character = original[original_index..]
            .chars()
            .next()
            .expect("index is before the end of a valid UTF-8 string");
        lexical.push(character);
        for consumed in 1..=character.len_utf8() {
            lexical_to_original.push(ByteOffset::new(
                u32::try_from(original_index + consumed).expect("source length was checked"),
            ));
        }
        original_index += character.len_utf8();
    }

    debug_assert_eq!(lexical_to_original.len(), lexical.len() + 1);
    (lexical, lexical_to_original)
}

fn lexical_line_starts(lexical: &str) -> Vec<LexicalOffset> {
    let mut line_starts = vec![LexicalOffset::new(0)];
    line_starts.extend(
        lexical
            .bytes()
            .enumerate()
            .filter(|(_, byte)| *byte == b'\n')
            .map(|(index, _)| {
                LexicalOffset::new(
                    u32::try_from(index + 1).expect("source length was checked before decoding"),
                )
            }),
    );
    line_starts
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE_ID: SourceId = SourceId::new(7);

    #[test]
    fn rejects_invalid_utf8() {
        let error = SourceFile::from_bytes(SOURCE_ID, "invalid.ling", vec![0xff]).unwrap_err();

        assert_eq!(
            error,
            SourceError::InvalidUtf8 {
                valid_up_to: 0,
                error_len: Some(1)
            }
        );
    }

    #[test]
    fn removes_only_a_leading_bom_from_the_lexical_view() {
        let source =
            SourceFile::from_bytes(SOURCE_ID, "bom.ling", "\u{feff}人物".as_bytes().to_vec())
                .unwrap();

        assert!(source.had_bom());
        assert_eq!(source.original_text(), "\u{feff}人物");
        assert_eq!(source.lexical_text(), "人物");
        assert_eq!(
            source
                .source_map()
                .original_offset(LexicalOffset::new(0))
                .unwrap(),
            ByteOffset::new(3)
        );
    }

    #[test]
    fn rejects_a_bom_after_the_start() {
        let error = SourceFile::from_bytes(SOURCE_ID, "bom.ling", "x\u{feff}y".as_bytes().to_vec())
            .unwrap_err();

        assert_eq!(
            error,
            SourceError::MisplacedByteOrderMark { byte_offset: 1 }
        );
    }

    #[test]
    fn normalizes_all_line_endings_and_preserves_original_spans() {
        let source =
            SourceFile::from_bytes(SOURCE_ID, "lines.ling", b"a\r\nb\rc\n".to_vec()).unwrap();

        assert_eq!(source.lexical_text(), "a\nb\nc\n");
        let lexical_b =
            LexicalSpan::new(SOURCE_ID, LexicalOffset::new(2), LexicalOffset::new(3)).unwrap();
        assert_eq!(
            source.source_map().original_span(lexical_b).unwrap(),
            Span::new(SOURCE_ID, ByteOffset::new(3), ByteOffset::new(4)).unwrap()
        );
    }

    #[test]
    fn reports_scalar_columns_after_newline_normalization() {
        let source = SourceFile::from_bytes(
            SOURCE_ID,
            "chinese.ling",
            "人物\r\n血量".as_bytes().to_vec(),
        )
        .unwrap();

        assert_eq!(
            source.line_column(ByteOffset::new(8)).unwrap(),
            LineColumn { line: 2, column: 1 }
        );
        assert_eq!(
            source.line_column(ByteOffset::new(11)).unwrap(),
            LineColumn { line: 2, column: 2 }
        );
    }

    #[test]
    fn rejects_offsets_inside_utf8_scalars() {
        let source =
            SourceFile::from_bytes(SOURCE_ID, "chinese.ling", "人物".as_bytes().to_vec()).unwrap();

        assert_eq!(
            source.line_column(ByteOffset::new(1)).unwrap_err(),
            SourceMapError::NotCharacterBoundary {
                offset: ByteOffset::new(1)
            }
        );
    }

    #[test]
    fn applies_unicode_edit_without_normalizing_original_bytes() {
        let source = SourceFile::from_bytes(
            SOURCE_ID,
            "edit.ling",
            "\u{feff}人物 = 😀\r\n".as_bytes().to_vec(),
        )
        .unwrap();
        let original = source.original_text().as_bytes();
        let start = original
            .windows("人物".len())
            .position(|window| window == "人物".as_bytes())
            .unwrap();
        let edited = source
            .apply_utf8_edit(&Utf8Edit::new(
                ByteOffset::new(start as u32),
                ByteOffset::new((start + "人物".len()) as u32),
                "将军".as_bytes().to_vec(),
            ))
            .unwrap();

        assert_eq!(edited.id(), SOURCE_ID);
        assert_eq!(edited.name(), "edit.ling");
        assert_eq!(edited.original_text(), "\u{feff}将军 = 😀\r\n");
        assert_eq!(edited.lexical_text(), "将军 = 😀\n");
        assert!(edited.had_bom());
    }

    #[test]
    fn applies_edits_in_order_and_matches_full_replacement() {
        let source =
            SourceFile::from_bytes(SOURCE_ID, "edit.ling", b"alpha beta".to_vec()).unwrap();
        let edits = [
            Utf8Edit::new(ByteOffset::new(0), ByteOffset::new(5), b"one".to_vec()),
            Utf8Edit::new(ByteOffset::new(4), ByteOffset::new(8), b"two".to_vec()),
        ];

        let edited = source.apply_utf8_edits(&edits).unwrap();
        let expected = SourceFile::from_bytes(SOURCE_ID, "edit.ling", b"one two".to_vec()).unwrap();
        assert_eq!(edited, expected);
    }

    #[test]
    fn rejects_reversed_out_of_bounds_and_non_boundary_ranges_atomically() {
        let source =
            SourceFile::from_bytes(SOURCE_ID, "edit.ling", "人物".as_bytes().to_vec()).unwrap();

        assert_eq!(
            source
                .apply_utf8_edit(&Utf8Edit::new(
                    ByteOffset::new(2),
                    ByteOffset::new(1),
                    Vec::new(),
                ))
                .unwrap_err(),
            SourceEditError::ReversedRange {
                start: ByteOffset::new(2),
                end: ByteOffset::new(1),
            }
        );
        assert!(matches!(
            source
                .apply_utf8_edit(&Utf8Edit::new(
                    ByteOffset::new(0),
                    ByteOffset::new(7),
                    Vec::new(),
                ))
                .unwrap_err(),
            SourceEditError::OffsetOutOfBounds { .. }
        ));
        assert_eq!(
            source
                .apply_utf8_edit(&Utf8Edit::new(
                    ByteOffset::new(1),
                    ByteOffset::new(2),
                    Vec::new(),
                ))
                .unwrap_err(),
            SourceEditError::NotCharacterBoundary {
                offset: ByteOffset::new(1)
            }
        );
        let crlf = SourceFile::from_bytes(SOURCE_ID, "edit.ling", b"a\r\nb".to_vec()).unwrap();
        assert_eq!(
            crlf.apply_utf8_edit(&Utf8Edit::new(
                ByteOffset::new(2),
                ByteOffset::new(2),
                Vec::new(),
            ))
            .unwrap_err(),
            SourceEditError::NotCharacterBoundary {
                offset: ByteOffset::new(2)
            }
        );
        assert_eq!(source.original_text(), "人物");
    }

    #[test]
    fn rejects_invalid_replacement_without_publishing_a_partial_batch() {
        let source = SourceFile::from_bytes(SOURCE_ID, "edit.ling", b"ab".to_vec()).unwrap();
        let edits = [
            Utf8Edit::new(ByteOffset::new(0), ByteOffset::new(1), b"x".to_vec()),
            Utf8Edit::new(ByteOffset::new(0), ByteOffset::new(1), vec![0xff]),
        ];

        assert!(matches!(
            source.apply_utf8_edits(&edits).unwrap_err(),
            SourceEditError::InvalidSource(SourceError::InvalidUtf8 { .. })
        ));
        assert_eq!(source.original_text(), "ab");
    }

    #[test]
    fn rejects_a_replacement_that_moves_the_bom() {
        let source = SourceFile::from_bytes(SOURCE_ID, "edit.ling", b"ab".to_vec()).unwrap();
        assert!(matches!(
            source
                .apply_utf8_edit(&Utf8Edit::new(
                    ByteOffset::new(1),
                    ByteOffset::new(1),
                    "\u{feff}".as_bytes().to_vec(),
                ))
                .unwrap_err(),
            SourceEditError::InvalidSource(SourceError::MisplacedByteOrderMark { .. })
        ));
    }

    #[test]
    fn full_replacement_rebuilds_the_same_validated_snapshot() {
        let source =
            SourceFile::from_bytes(SOURCE_ID, "edit.ling", "\u{feff}旧\r\n".as_bytes().to_vec())
                .unwrap();
        let replacement = "\u{feff}新\r\n".as_bytes().to_vec();
        let edited = source
            .apply_utf8_edit(&Utf8Edit::new(
                ByteOffset::new(0),
                ByteOffset::new(source.original_text().len() as u32),
                replacement.clone(),
            ))
            .unwrap();
        let expected = SourceFile::from_bytes(SOURCE_ID, "edit.ling", replacement).unwrap();

        assert_eq!(edited, expected);
    }
}
