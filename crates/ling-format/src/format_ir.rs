//! A lossless, compiler-owned intermediate representation for Author Source formatting.
//!
//! This module deliberately stops at a projection of the compiler CST.  It does
//! not parse text, infer semantics, attach comments, or render formatted output.
//! Those boundaries keep a future formatter from becoming a second language
//! implementation while still giving later stages one immutable source snapshot.

use std::error::Error;
use std::fmt;
use std::ops::Range;

use ling_source::{LexicalSpan, SourceFile, SourceId, Span};
use ling_syntax::{CstNode, NodeKind, ParsedSource, Token, TokenKind};

use crate::comments::{CommentAttachment, attach_comments};

/// Schema identifier for the in-process Format IR projection.
pub const FORMAT_IR_SCHEMA: &str = "ling.format-ir/0.1";

/// A compiler-CST projection suitable as input to later formatting stages.
///
/// The document owns the exact source snapshot used to build the projection.
/// `original_text` is never normalized; `lexical_text` is the parser's BOM-free,
/// LF-normalized view.  No formatter is implied by this type alone.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatDocument {
    source_id: SourceId,
    original_text: String,
    lexical_text: String,
    had_bom: bool,
    tokens: Box<[FormatToken]>,
    root: FormatNode,
    comment_attachments: Box<[CommentAttachment]>,
    valid: bool,
}

impl FormatDocument {
    /// Returns the source identity carried by every projected span.
    #[must_use]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    /// Returns the exact original UTF-8 source snapshot, including a BOM if one
    /// was present.
    #[must_use]
    pub fn original_text(&self) -> &str {
        &self.original_text
    }

    /// Returns the parser's BOM-free, LF-normalized lexical source snapshot.
    #[must_use]
    pub fn lexical_text(&self) -> &str {
        &self.lexical_text
    }

    /// Returns whether the source had a leading UTF-8 BOM.
    #[must_use]
    pub const fn had_bom(&self) -> bool {
        self.had_bom
    }

    /// Returns the complete projected token stream in compiler order.
    #[must_use]
    pub fn tokens(&self) -> &[FormatToken] {
        &self.tokens
    }

    /// Returns the projected compiler CST root.
    #[must_use]
    pub const fn root(&self) -> &FormatNode {
        &self.root
    }

    /// Returns deterministic comment-to-CST associations in source order.
    #[must_use]
    pub fn comment_attachments(&self) -> &[CommentAttachment] {
        &self.comment_attachments
    }

    /// Returns whether parsing completed without lexical or syntax errors.
    ///
    /// An invalid document is still a valid lossless projection.  Later
    /// recovery work may inspect its complete spans, but this flag prevents it
    /// from being mistaken for a checked source file.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }
}

/// One projected compiler CST node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatNode {
    kind: NodeKind,
    token_range: Range<usize>,
    span: Option<Span>,
    children: Box<[FormatNode]>,
}

impl FormatNode {
    /// Returns the compiler-owned node kind.
    #[must_use]
    pub const fn kind(&self) -> NodeKind {
        self.kind
    }

    /// Returns the node's half-open range in [`FormatDocument::tokens`].
    #[must_use]
    pub fn token_range(&self) -> Range<usize> {
        self.token_range.clone()
    }

    /// Returns the original-byte span covered by this node, when it has tokens.
    /// Empty recovery nodes intentionally have no fabricated span.
    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        self.span
    }

    /// Returns child nodes in compiler CST order.
    #[must_use]
    pub fn children(&self) -> &[FormatNode] {
        &self.children
    }

    /// Returns the exact projected token slice covered by this node.
    #[must_use]
    pub fn tokens<'document>(
        &self,
        document: &'document FormatDocument,
    ) -> &'document [FormatToken] {
        &document.tokens[self.token_range.clone()]
    }
}

/// One compiler token with exact original spelling and both span domains.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatToken {
    kind: TokenKind,
    lexical_span: LexicalSpan,
    span: Span,
    text: String,
}

impl FormatToken {
    /// Returns the compiler token kind.
    #[must_use]
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Returns the token span in the normalized lexical source.
    #[must_use]
    pub const fn lexical_span(&self) -> LexicalSpan {
        self.lexical_span
    }

    /// Returns the token span in the exact original source bytes.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the exact original source spelling covered by this token.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns whether this token is author trivia such as a comment or space.
    #[must_use]
    pub const fn is_trivia(&self) -> bool {
        self.kind.is_trivia()
    }

    /// Returns whether this token participates in compiler layout.
    #[must_use]
    pub const fn is_layout(&self) -> bool {
        self.kind.is_layout()
    }
}

/// Failure while validating the source/CST boundary for a Format IR build.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatIrBuildError {
    kind: FormatIrBuildErrorKind,
}

impl FormatIrBuildError {
    /// Returns the structured validation failure.
    #[must_use]
    pub const fn kind(&self) -> &FormatIrBuildErrorKind {
        &self.kind
    }
}

impl fmt::Display for FormatIrBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            FormatIrBuildErrorKind::SourceMismatch { expected, actual } => write!(
                formatter,
                "Format IR source mismatch: expected source {}, found {}",
                expected.get(),
                actual.get()
            ),
            FormatIrBuildErrorKind::OriginalSpanOutOfBounds {
                token_index,
                start,
                end,
            } => write!(
                formatter,
                "Format IR token {token_index} has original span {start}..{end} outside the source"
            ),
            FormatIrBuildErrorKind::LexicalSpanOutOfBounds {
                token_index,
                start,
                end,
            } => write!(
                formatter,
                "Format IR token {token_index} has lexical span {start}..{end} outside the source"
            ),
            FormatIrBuildErrorKind::InvalidNodeRange {
                kind,
                start,
                end,
                token_count,
            } => write!(
                formatter,
                "Format IR {kind:?} node range {start}..{end} exceeds {token_count} tokens"
            ),
            FormatIrBuildErrorKind::InvalidNodeSpan { kind, start, end } => write!(
                formatter,
                "Format IR {kind:?} node span {start}..{end} is reversed"
            ),
        }
    }
}

impl Error for FormatIrBuildError {}

/// Structured validation failures produced before a Format IR is published.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormatIrBuildErrorKind {
    /// A parsed tree belongs to a different source identity than the snapshot.
    SourceMismatch {
        expected: SourceId,
        actual: SourceId,
    },
    /// A projected token's original span cannot be sliced from the snapshot.
    OriginalSpanOutOfBounds {
        token_index: usize,
        start: u32,
        end: u32,
    },
    /// A projected token's lexical span cannot be sliced from the normalized view.
    LexicalSpanOutOfBounds {
        token_index: usize,
        start: u32,
        end: u32,
    },
    /// A compiler CST node range is outside the projected token stream.
    InvalidNodeRange {
        kind: NodeKind,
        start: usize,
        end: usize,
        token_count: usize,
    },
    /// A non-empty node range does not produce a forward original span.
    InvalidNodeSpan {
        kind: NodeKind,
        start: u32,
        end: u32,
    },
}

/// Builds a lossless Format IR directly from the authoritative compiler CST.
///
/// This function never reparses source text.  It validates that the supplied
/// `SourceFile` owns every token span, projects exact original token spelling,
/// and recursively copies the compiler node structure.  Parse errors remain
/// observable through [`FormatDocument::is_valid`] and are not interpreted.
pub fn build_format_ir(
    source: &SourceFile,
    parsed: &ParsedSource,
) -> Result<FormatDocument, FormatIrBuildError> {
    let tree = parsed.tree();
    let tokens = tree
        .tokens()
        .iter()
        .enumerate()
        .map(|(index, token)| project_token(source, index, token))
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    let root = project_node(tree.root(), &tokens)?;
    let comment_attachments = attach_comments(&tokens, &root);
    Ok(FormatDocument {
        source_id: source.id(),
        original_text: source.original_text().to_owned(),
        lexical_text: source.lexical_text().to_owned(),
        had_bom: source.had_bom(),
        tokens,
        root,
        comment_attachments,
        valid: parsed.is_valid(),
    })
}

fn project_token(
    source: &SourceFile,
    token_index: usize,
    token: &Token,
) -> Result<FormatToken, FormatIrBuildError> {
    let span = token.span();
    if span.source() != source.id() {
        return Err(FormatIrBuildError {
            kind: FormatIrBuildErrorKind::SourceMismatch {
                expected: source.id(),
                actual: span.source(),
            },
        });
    }
    let original_start = span.start().get();
    let original_end = span.end().get();
    let original_text = source
        .original_text()
        .get(original_start as usize..original_end as usize)
        .ok_or(FormatIrBuildError {
            kind: FormatIrBuildErrorKind::OriginalSpanOutOfBounds {
                token_index,
                start: original_start,
                end: original_end,
            },
        })?;
    let lexical_span = token.lexical_span();
    if lexical_span.source() != source.id() {
        return Err(FormatIrBuildError {
            kind: FormatIrBuildErrorKind::SourceMismatch {
                expected: source.id(),
                actual: lexical_span.source(),
            },
        });
    }
    let lexical_start = lexical_span.start().get();
    let lexical_end = lexical_span.end().get();
    source
        .lexical_text()
        .get(lexical_start as usize..lexical_end as usize)
        .ok_or(FormatIrBuildError {
            kind: FormatIrBuildErrorKind::LexicalSpanOutOfBounds {
                token_index,
                start: lexical_start,
                end: lexical_end,
            },
        })?;
    Ok(FormatToken {
        kind: token.kind(),
        lexical_span,
        span,
        text: original_text.to_owned(),
    })
}

fn project_node(node: &CstNode, tokens: &[FormatToken]) -> Result<FormatNode, FormatIrBuildError> {
    let token_range = node.token_range();
    if token_range.start > token_range.end || token_range.end > tokens.len() {
        return Err(FormatIrBuildError {
            kind: FormatIrBuildErrorKind::InvalidNodeRange {
                kind: node.kind(),
                start: token_range.start,
                end: token_range.end,
                token_count: tokens.len(),
            },
        });
    }
    let span = node_span(node.kind(), &token_range, tokens)?;
    let children = node
        .children()
        .iter()
        .map(|child| project_node(child, tokens))
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    Ok(FormatNode {
        kind: node.kind(),
        token_range,
        span,
        children,
    })
}

fn node_span(
    kind: NodeKind,
    token_range: &Range<usize>,
    tokens: &[FormatToken],
) -> Result<Option<Span>, FormatIrBuildError> {
    let Some(first) = tokens.get(token_range.start) else {
        return Ok(None);
    };
    let Some(last) = tokens.get(token_range.end.saturating_sub(1)) else {
        return Ok(None);
    };
    let start = first.span.start();
    let end = last.span.end();
    if start > end {
        return Err(FormatIrBuildError {
            kind: FormatIrBuildErrorKind::InvalidNodeSpan {
                kind,
                start: start.get(),
                end: end.get(),
            },
        });
    }
    Ok(Some(
        Span::new(first.span.source(), start, end).expect("validated node span is forward"),
    ))
}

#[cfg(test)]
mod tests {
    use ling_source::SourceId;
    use ling_syntax::{NodeKind, TokenKind, parse};

    use super::*;

    fn source(id: u32, text: &[u8]) -> SourceFile {
        SourceFile::from_bytes(SourceId::new(id), "format-ir.ling", text.to_vec())
            .expect("test source is valid UTF-8")
    }

    #[test]
    fn projects_cst_spans_and_exact_original_tokens() {
        let source = source(
            41,
            concat!(
                "\u{feff}module Main\r\n",
                "\r\n",
                "// 保留注释\r\n",
                "let main () =\r\n",
                "    \"保留\"\r\n",
            )
            .as_bytes(),
        );
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:#?}", parsed.parse_errors());

        let document = build_format_ir(&source, &parsed).expect("CST projection succeeds");
        assert_eq!(document.source_id(), source.id());
        assert_eq!(document.original_text(), source.original_text());
        assert_eq!(document.lexical_text(), source.lexical_text());
        assert!(document.had_bom());
        assert!(document.is_valid());
        assert_eq!(document.root().kind(), NodeKind::Program);
        assert_eq!(
            document.root().token_range(),
            parsed.tree().root().token_range()
        );
        assert_eq!(
            document.root().tokens(&document).len(),
            document.tokens().len()
        );
        assert!(
            document.tokens().iter().any(
                |token| token.kind() == TokenKind::LineComment && token.text() == "// 保留注释"
            )
        );
        assert!(
            document
                .tokens()
                .iter()
                .any(|token| token.kind() == TokenKind::Newline && token.text() == "\r\n")
        );
        assert!(
            document
                .tokens()
                .iter()
                .any(|token| token.kind() == TokenKind::Text && token.text() == "\"保留\"")
        );
        let declaration = document
            .root()
            .children()
            .iter()
            .find(|node| node.kind() == NodeKind::LetDeclaration)
            .expect("let declaration is represented");
        assert!(declaration.span().is_some());
        assert!(declaration.children().iter().any(|node| matches!(
            node.kind(),
            NodeKind::Pattern | NodeKind::UnitExpression | NodeKind::Expression
        )));
    }

    #[test]
    fn retains_an_invalid_source_without_fabricating_tokens() {
        let source = source(42, b"let value = \"unterminated\n");
        let parsed = parse(&source);
        assert!(!parsed.is_valid());

        let document = build_format_ir(&source, &parsed).expect("invalid CST still projects");
        assert!(!document.is_valid());
        assert_eq!(document.original_text(), source.original_text());
        assert!(
            document
                .tokens()
                .iter()
                .all(|token| token.span().source() == source.id())
        );
    }

    #[test]
    fn rejects_a_tree_from_another_source_snapshot() {
        let parsed_source = source(43, b"let value = 1\n");
        let other_source = source(44, b"let value = 1\n");
        let parsed = parse(&parsed_source);

        let error = build_format_ir(&other_source, &parsed).expect_err("source identity matters");
        assert_eq!(
            error.kind(),
            &FormatIrBuildErrorKind::SourceMismatch {
                expected: other_source.id(),
                actual: parsed_source.id(),
            }
        );
    }

    #[test]
    fn projection_is_deterministic() {
        let source = source(45, b"module Main\n\nlet main () = ()\n");
        let parsed = parse(&source);
        let first = build_format_ir(&source, &parsed).expect("first projection");
        let second = build_format_ir(&source, &parsed).expect("second projection");
        assert_eq!(first, second);
    }
}
