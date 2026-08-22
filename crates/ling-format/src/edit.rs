//! Deterministic in-process source-edit projection for Author Source.
//!
//! This module deliberately stops before any LSP or JSON-RPC contract.  It
//! turns one accepted formatter result into one byte-range replacement that a
//! later protocol adapter may validate and project under its own authority.

use std::error::Error;
use std::fmt;
use std::ops::Range;

use ling_source::SourceId;

use crate::{FormatDisposition, FormatDocument, format_core_with_disposition};

/// One deterministic replacement over the original UTF-8 byte span.
///
/// The range is always measured against the exact original source snapshot.
/// This value is an in-process formatter result, not an LSP `TextEdit`,
/// `WorkspaceEdit`, or serialized protocol object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatEdit {
    source_id: SourceId,
    range: Range<u32>,
    replacement: String,
}

impl FormatEdit {
    /// Returns the source identity carried by the formatter document.
    #[must_use]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    /// Returns the replaced range in original UTF-8 byte offsets.
    #[must_use]
    pub fn range(&self) -> Range<u32> {
        self.range.clone()
    }

    /// Returns the exact replacement text, including any formatter-preserved
    /// BOM and the formatter's LF line endings.
    #[must_use]
    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

/// A source snapshot cannot be represented by the accepted `u32` span unit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatEditError {
    byte_length: usize,
}

impl FormatEditError {
    /// Returns the original source length that exceeded the span unit.
    #[must_use]
    pub const fn byte_length(&self) -> usize {
        self.byte_length
    }
}

impl fmt::Display for FormatEditError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "formatted source is too large for a u32 byte span: {} bytes",
            self.byte_length
        )
    }
}

impl Error for FormatEditError {}

/// Computes the single whole-document edit for a safely formatted source.
///
/// Invalid or rejected candidates produce `Ok(None)`, preserving the existing
/// formatter publication boundary.  A valid candidate that differs from the
/// original source produces exactly one replacement covering the original
/// source bytes.  No diff algorithm, URI, version, position encoding,
/// transaction, or wire serialization is introduced here.
pub fn format_core_edit(document: &FormatDocument) -> Result<Option<FormatEdit>, FormatEditError> {
    let result = format_core_with_disposition(document);
    if result.disposition() != FormatDisposition::Formatted
        || result.text() == document.original_text()
    {
        return Ok(None);
    }

    let end = u32::try_from(document.original_text().len()).map_err(|_| FormatEditError {
        byte_length: document.original_text().len(),
    })?;
    Ok(Some(FormatEdit {
        source_id: document.source_id(),
        range: 0..end,
        replacement: result.into_text(),
    }))
}

#[cfg(test)]
mod tests {
    use ling_source::SourceFile;
    use ling_syntax::parse;

    use super::*;
    use crate::build_format_ir;

    fn document(source_id: u32, input: &str) -> FormatDocument {
        let source = SourceFile::from_bytes(
            SourceId::new(source_id),
            "edit.ling",
            input.as_bytes().to_vec(),
        )
        .expect("test source is valid UTF-8");
        let parsed = parse(&source);
        build_format_ir(&source, &parsed).expect("format IR builds")
    }

    #[test]
    fn changed_valid_source_returns_one_whole_document_edit() {
        let input = "let value=1\n";
        let edit = format_core_edit(&document(16001, input))
            .expect("edit projection succeeds")
            .expect("source changes");

        assert_eq!(edit.source_id(), SourceId::new(16001));
        assert_eq!(edit.range(), 0..u32::try_from(input.len()).unwrap());
        assert_eq!(edit.replacement(), "let value = 1\n");
    }

    #[test]
    fn already_formatted_source_has_no_edit() {
        assert_eq!(
            format_core_edit(&document(16002, "let value = 1\n"))
                .expect("edit projection succeeds"),
            None
        );
    }

    #[test]
    fn invalid_source_preserves_the_no_edit_boundary() {
        assert_eq!(
            format_core_edit(&document(16003, "let value=\"unterminated\r\n"))
                .expect("edit projection succeeds"),
            None
        );
    }

    #[test]
    fn original_bom_and_crlf_bytes_define_the_edit_range() {
        let input = "\u{feff}let 中文=1\r\n";
        let edit = format_core_edit(&document(16004, input))
            .expect("edit projection succeeds")
            .expect("source changes");

        assert_eq!(edit.range(), 0..u32::try_from(input.len()).unwrap());
        assert_eq!(edit.replacement(), "\u{feff}let 中文 = 1\n");
    }
}
