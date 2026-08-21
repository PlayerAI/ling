use std::error::Error;
use std::fmt;

use super::{ByteOffset, LexicalOffset, SourceFile, SourceMapError};

/// A negotiated position encoding used by an editor adapter.
///
/// The labels are the wire values defined by the LSP position-encoding
/// negotiation.  This type only describes conversion in the source layer; it
/// does not define an LSP server or document lifecycle.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PositionEncoding {
    Utf8,
    Utf16,
    Utf32,
}

impl PositionEncoding {
    /// Returns the protocol label for this encoding.
    #[must_use]
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::Utf8 => "utf-8",
            Self::Utf16 => "utf-16",
            Self::Utf32 => "utf-32",
        }
    }

    /// Parses one of the supported protocol labels.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "utf-8" => Some(Self::Utf8),
            "utf-16" => Some(Self::Utf16),
            "utf-32" => Some(Self::Utf32),
            _ => None,
        }
    }
}

/// Supported encodings in the deterministic advertisement order.
pub const SUPPORTED_POSITION_ENCODINGS: [PositionEncoding; 3] = [
    PositionEncoding::Utf8,
    PositionEncoding::Utf16,
    PositionEncoding::Utf32,
];

/// Selects the first supported encoding advertised by a client.
///
/// Unknown labels are ignored.  An empty list, or a list containing only
/// unknown labels, uses the LSP-compatible UTF-16 fallback.
#[must_use]
pub fn negotiate_position_encoding(client_labels: &[&str]) -> PositionEncoding {
    client_labels
        .iter()
        .find_map(|label| PositionEncoding::parse(label))
        .unwrap_or(PositionEncoding::Utf16)
}

/// A zero-based editor position in a normalized lexical source view.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LspPosition {
    line: u32,
    character: u32,
}

impl LspPosition {
    /// Creates a zero-based line/character position.
    #[must_use]
    pub const fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }

    /// Returns the zero-based line.
    #[must_use]
    pub const fn line(self) -> u32 {
        self.line
    }

    /// Returns the encoding-specific zero-based character position.
    #[must_use]
    pub const fn character(self) -> u32 {
        self.character
    }
}

/// A failure while converting between original bytes and an editor position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PositionError {
    /// The source map could not project the requested original/lexical offset.
    SourceMap(SourceMapError),
    /// The requested line is not present in the lexical source.
    LineOutOfBounds { line: u32, line_count: u32 },
    /// The requested character is beyond the content of its lexical line.
    CharacterOutOfBounds {
        line: u32,
        character: u32,
        maximum: u32,
        encoding: PositionEncoding,
    },
    /// The requested character falls inside a UTF-8 scalar or UTF-16 pair.
    InvalidCharacterBoundary {
        position: LspPosition,
        encoding: PositionEncoding,
    },
}

impl fmt::Display for PositionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceMap(error) => write!(formatter, "source position mapping failed: {error}"),
            Self::LineOutOfBounds { line, line_count } => write!(
                formatter,
                "position line {} is outside the {}-line lexical source",
                line, line_count
            ),
            Self::CharacterOutOfBounds {
                line,
                character,
                maximum,
                encoding,
            } => write!(
                formatter,
                "position ({line}, {character}) exceeds the {encoding} line maximum {maximum}"
            ),
            Self::InvalidCharacterBoundary { position, encoding } => write!(
                formatter,
                "position ({}, {}) is inside a character boundary for {encoding}",
                position.line(),
                position.character()
            ),
        }
    }
}

impl Error for PositionError {}

impl From<SourceMapError> for PositionError {
    fn from(error: SourceMapError) -> Self {
        Self::SourceMap(error)
    }
}

impl fmt::Display for PositionEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.wire_name())
    }
}

impl SourceFile {
    /// Converts an original UTF-8 byte offset into a normalized editor position.
    ///
    /// The source map remains authoritative: the returned position is always
    /// derived from the BOM-free, LF-normalized lexical view, while the input
    /// offset is required to be an exact original character boundary.
    pub fn lsp_position(
        &self,
        offset: ByteOffset,
        encoding: PositionEncoding,
    ) -> Result<LspPosition, PositionError> {
        let lexical_offset = self.source_map.lexical_offset(offset)?;
        let lexical_index = lexical_offset.get() as usize;
        let line_index = self.line_index(lexical_offset);
        let line_start = self.line_starts[line_index].get() as usize;
        let character = if encoding == PositionEncoding::Utf8 {
            lexical_index - line_start
        } else {
            let mut prefix_end = lexical_index;
            while prefix_end > line_start && !self.lexical.is_char_boundary(prefix_end) {
                prefix_end -= 1;
            }
            measure_units(&self.lexical[line_start..prefix_end], encoding)
        };
        let position = LspPosition::new(
            u32::try_from(line_index).expect("line count is bounded by source length"),
            u32::try_from(character).expect("character count is bounded by source length"),
        );

        if !self.lexical.is_char_boundary(lexical_index) {
            return Err(PositionError::InvalidCharacterBoundary { position, encoding });
        }

        Ok(position)
    }

    /// Converts a normalized editor position back into an original byte offset.
    ///
    /// Positions are validated against the lexical line without clamping.  A
    /// valid position is then projected through the source map so BOM and
    /// newline normalization remain lossless in the original source.
    pub fn original_offset(
        &self,
        position: LspPosition,
        encoding: PositionEncoding,
    ) -> Result<ByteOffset, PositionError> {
        let line_count =
            u32::try_from(self.line_starts.len()).expect("line count is bounded by source length");
        if position.line >= line_count {
            return Err(PositionError::LineOutOfBounds {
                line: position.line,
                line_count,
            });
        }

        let line_index = position.line as usize;
        let (line_start, line_end) = self.line_bounds(line_index);
        let line = &self.lexical[line_start..line_end];
        let requested = position.character as usize;
        let byte_in_line = match encoding {
            PositionEncoding::Utf8 => {
                if requested > line.len() {
                    return Err(character_out_of_bounds(position, encoding, line.len()));
                }
                if !line.is_char_boundary(requested) {
                    return Err(PositionError::InvalidCharacterBoundary { position, encoding });
                }
                requested
            }
            PositionEncoding::Utf16 => utf16_byte_offset(line, requested, position, encoding)?,
            PositionEncoding::Utf32 => utf32_byte_offset(line, requested, position, encoding)?,
        };

        let lexical_index = line_start + byte_in_line;
        let lexical_offset = LexicalOffset::new(
            u32::try_from(lexical_index).expect("source length is bounded by u32"),
        );
        Ok(self.source_map.original_offset(lexical_offset)?)
    }

    fn line_index(&self, offset: LexicalOffset) -> usize {
        self.line_starts
            .partition_point(|line_start| *line_start <= offset)
            .saturating_sub(1)
    }

    fn line_bounds(&self, line_index: usize) -> (usize, usize) {
        let line_start = self.line_starts[line_index].get() as usize;
        let line_end = self
            .line_starts
            .get(line_index + 1)
            .map_or_else(|| self.lexical.len(), |next| next.get() as usize - 1);
        (line_start, line_end)
    }
}

fn measure_units(text: &str, encoding: PositionEncoding) -> usize {
    match encoding {
        PositionEncoding::Utf8 => text.len(),
        PositionEncoding::Utf16 => text.chars().map(char::len_utf16).sum(),
        PositionEncoding::Utf32 => text.chars().count(),
    }
}

fn character_out_of_bounds(
    position: LspPosition,
    encoding: PositionEncoding,
    maximum: usize,
) -> PositionError {
    PositionError::CharacterOutOfBounds {
        line: position.line,
        character: position.character,
        maximum: u32::try_from(maximum).expect("character count is bounded by source length"),
        encoding,
    }
}

fn utf16_byte_offset(
    line: &str,
    requested: usize,
    position: LspPosition,
    encoding: PositionEncoding,
) -> Result<usize, PositionError> {
    let mut units = 0usize;
    for (byte, character) in line.char_indices() {
        if requested == units {
            return Ok(byte);
        }

        let width = character.len_utf16();
        if requested < units + width {
            return Err(PositionError::InvalidCharacterBoundary { position, encoding });
        }
        units += width;
    }

    if requested == units {
        Ok(line.len())
    } else {
        Err(character_out_of_bounds(position, encoding, units))
    }
}

fn utf32_byte_offset(
    line: &str,
    requested: usize,
    position: LspPosition,
    encoding: PositionEncoding,
) -> Result<usize, PositionError> {
    let maximum = line.chars().count();
    if requested > maximum {
        return Err(character_out_of_bounds(position, encoding, maximum));
    }

    Ok(line
        .char_indices()
        .nth(requested)
        .map_or(line.len(), |(byte, _)| byte))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceId;

    const SOURCE_ID: SourceId = SourceId::new(42);

    fn source(text: &str) -> SourceFile {
        SourceFile::from_bytes(SOURCE_ID, "position.ling", text.as_bytes().to_vec())
            .expect("test source is valid UTF-8")
    }

    #[test]
    fn negotiates_the_first_supported_label_and_falls_back_to_utf16() {
        assert_eq!(
            negotiate_position_encoding(&["unknown", "utf-32", "utf-8"]),
            PositionEncoding::Utf32
        );
        assert_eq!(
            negotiate_position_encoding(&["unknown", "also-unknown"]),
            PositionEncoding::Utf16
        );
        assert_eq!(
            SUPPORTED_POSITION_ENCODINGS,
            [
                PositionEncoding::Utf8,
                PositionEncoding::Utf16,
                PositionEncoding::Utf32
            ]
        );
        assert_eq!(PositionEncoding::Utf8.wire_name(), "utf-8");
        assert_eq!(
            PositionEncoding::parse("utf-16"),
            Some(PositionEncoding::Utf16)
        );
        assert_eq!(PositionEncoding::parse("UTF-8"), None);
    }

    #[test]
    fn projects_bom_crlf_unicode_and_combining_scalars() {
        let source = source("\u{feff}人物 😀e\u{301}\r\n\r\n末");

        for encoding in SUPPORTED_POSITION_ENCODINGS {
            let lexical_start = LspPosition::new(0, 0);
            assert_eq!(
                source.original_offset(lexical_start, encoding).unwrap(),
                ByteOffset::new(3)
            );
            assert_eq!(
                source.lsp_position(ByteOffset::new(3), encoding).unwrap(),
                lexical_start
            );

            let lexical_len = source.source_map().lexical_len().get() as usize;
            for lexical_index in 0..=lexical_len {
                if !source.lexical_text().is_char_boundary(lexical_index) {
                    continue;
                }
                let lexical_offset = LexicalOffset::new(lexical_index as u32);
                let original = source.source_map().original_offset(lexical_offset).unwrap();
                let position = source.lsp_position(original, encoding).unwrap();
                assert_eq!(
                    source.original_offset(position, encoding).unwrap(),
                    original,
                    "round-trip failed for {encoding:?} at lexical offset {lexical_index}"
                );
            }
        }

        assert_eq!(
            source.lsp_position(ByteOffset::new(3), PositionEncoding::Utf8),
            Ok(LspPosition::new(0, 0))
        );
        assert_eq!(
            source.lsp_position(ByteOffset::new(10), PositionEncoding::Utf8),
            Ok(LspPosition::new(0, 7))
        );
        assert_eq!(
            source.lsp_position(ByteOffset::new(14), PositionEncoding::Utf16),
            Ok(LspPosition::new(0, 5))
        );
        assert_eq!(
            source.original_offset(LspPosition::new(2, 0), PositionEncoding::Utf8),
            Ok(ByteOffset::new(21))
        );
    }

    #[test]
    fn rejects_inside_utf8_scalars_and_utf16_surrogate_pairs() {
        let source = source("人物 😀");

        assert_eq!(
            source.lsp_position(ByteOffset::new(1), PositionEncoding::Utf8),
            Err(PositionError::InvalidCharacterBoundary {
                position: LspPosition::new(0, 1),
                encoding: PositionEncoding::Utf8
            })
        );
        assert_eq!(
            source.original_offset(LspPosition::new(0, 1), PositionEncoding::Utf8),
            Err(PositionError::InvalidCharacterBoundary {
                position: LspPosition::new(0, 1),
                encoding: PositionEncoding::Utf8
            })
        );
        assert_eq!(
            source.original_offset(LspPosition::new(0, 4), PositionEncoding::Utf16),
            Err(PositionError::InvalidCharacterBoundary {
                position: LspPosition::new(0, 4),
                encoding: PositionEncoding::Utf16
            })
        );
        assert!(matches!(
            source.original_offset(LspPosition::new(0, 99), PositionEncoding::Utf32),
            Err(PositionError::CharacterOutOfBounds {
                maximum: 4,
                encoding: PositionEncoding::Utf32,
                ..
            })
        ));
    }

    #[test]
    fn rejects_crlf_interior_and_unknown_lines_without_clamping() {
        let source = source("a\r\nb");

        assert_eq!(
            source.lsp_position(ByteOffset::new(2), PositionEncoding::Utf8),
            Err(PositionError::SourceMap(
                SourceMapError::UnmappedOriginalOffset {
                    offset: ByteOffset::new(2)
                }
            ))
        );
        assert_eq!(
            source.original_offset(LspPosition::new(0, 2), PositionEncoding::Utf8),
            Err(PositionError::CharacterOutOfBounds {
                line: 0,
                character: 2,
                maximum: 1,
                encoding: PositionEncoding::Utf8
            })
        );
        assert_eq!(
            source.original_offset(LspPosition::new(3, 0), PositionEncoding::Utf8),
            Err(PositionError::LineOutOfBounds {
                line: 3,
                line_count: 2
            })
        );
        assert_eq!(
            source.original_offset(LspPosition::new(1, 1), PositionEncoding::Utf8),
            Ok(ByteOffset::new(4))
        );
    }
}
