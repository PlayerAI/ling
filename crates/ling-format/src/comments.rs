//! Compiler-CST-backed attachment metadata for Author Source comments.
//!
//! The lexer deliberately exposes a lossless token stream rather than assigning
//! comments to declarations.  This module adds that presentation-level
//! association without reparsing text: boundaries come from compiler tokens and
//! the projected CST's top-level child ranges.  A multiline block comment is
//! represented by several lexer segments; those segments are grouped only when
//! their exact token spellings prove that they are one block-comment sequence.

use std::ops::Range;

use ling_syntax::{NodeKind, TokenKind};

use crate::{FormatNode, FormatToken};

/// The source-level role of an attached comment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommentKind {
    /// A `//` line comment.
    Line,
    /// A `///` documentation comment.
    Documentation,
    /// A `/* ... */` block comment, including nested/multiline blocks.
    Block,
}

/// The comment's position relative to its neighboring code region.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommentPlacement {
    /// The comment precedes the target region and has no code before it on its
    /// source line.
    Leading,
    /// The comment follows code on its source line.
    Trailing,
    /// The comment has no following code region (for example a file-ending
    /// comment) and therefore stands alone.
    Standalone,
}

/// A deterministic association between one source comment region and the
/// compiler-CST region it must remain attached to during formatting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommentAttachment {
    comment_token_range: Range<usize>,
    target_token_range: Option<Range<usize>>,
    target_kind: Option<NodeKind>,
    kind: CommentKind,
    placement: CommentPlacement,
}

impl CommentAttachment {
    /// Returns the contiguous token range occupied by this comment region.
    /// Multiline block-comment ranges include the compiler newline tokens that
    /// split lexer segments of the same block comment.
    #[must_use]
    pub fn comment_token_range(&self) -> Range<usize> {
        self.comment_token_range.clone()
    }

    /// Returns the neighboring CST region's token range, when one exists.
    #[must_use]
    pub fn target_token_range(&self) -> Option<Range<usize>> {
        self.target_token_range.clone()
    }

    /// Returns the compiler node kind of the neighboring CST region.
    #[must_use]
    pub const fn target_kind(&self) -> Option<NodeKind> {
        self.target_kind
    }

    /// Returns whether this is a normal, documentation, or block comment.
    #[must_use]
    pub const fn kind(&self) -> CommentKind {
        self.kind
    }

    /// Returns the comment's leading, trailing, or standalone placement.
    #[must_use]
    pub const fn placement(&self) -> CommentPlacement {
        self.placement
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NodeTarget {
    kind: NodeKind,
    range: RangeBounds,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RangeBounds {
    start: usize,
    end: usize,
}

impl RangeBounds {
    const fn new(range: &Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }

    const fn contains(self, token_index: usize) -> bool {
        self.start <= token_index && token_index < self.end
    }

    fn to_range(self) -> Range<usize> {
        self.start..self.end
    }
}

/// Builds deterministic attachments directly from the projected compiler CST.
pub(crate) fn attach_comments(
    tokens: &[FormatToken],
    root: &FormatNode,
) -> Box<[CommentAttachment]> {
    let mut attachments = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let Some((end, kind)) = comment_group_end(tokens, index) else {
            index += 1;
            continue;
        };
        let previous = previous_significant(tokens, index);
        let next = next_significant(tokens, end);
        let placement = if has_code_before_line(tokens, index) {
            CommentPlacement::Trailing
        } else if next.is_some() {
            CommentPlacement::Leading
        } else {
            CommentPlacement::Standalone
        };
        let target = target_for_comment(root, tokens, index, previous, next);
        attachments.push(CommentAttachment {
            comment_token_range: index..end,
            target_token_range: target.map(|target| target.range.to_range()),
            target_kind: target.map(|target| target.kind),
            kind,
            placement,
        });
        index = end;
    }
    attachments.into_boxed_slice()
}

fn comment_group_end(tokens: &[FormatToken], start: usize) -> Option<(usize, CommentKind)> {
    let token = tokens.get(start)?;
    let kind = match token.kind() {
        TokenKind::LineComment => CommentKind::Line,
        TokenKind::DocComment => CommentKind::Documentation,
        TokenKind::BlockComment => CommentKind::Block,
        _ => return None,
    };
    if kind != CommentKind::Block || !token.text().starts_with("/*") {
        return Some((start + 1, kind));
    }

    let mut depth = 0_i32;
    for (offset, token) in tokens[start..].iter().enumerate() {
        if token.kind() == TokenKind::BlockComment {
            depth += block_depth_delta(token.text());
            if depth <= 0 {
                return Some((start + offset + 1, kind));
            }
        }
    }
    Some((tokens.len(), kind))
}

fn block_depth_delta(text: &str) -> i32 {
    let opens = text.match_indices("/*").count() as i32;
    let closes = text.match_indices("*/").count() as i32;
    opens - closes
}

fn previous_significant(tokens: &[FormatToken], start: usize) -> Option<usize> {
    tokens[..start.min(tokens.len())]
        .iter()
        .enumerate()
        .rev()
        .find_map(|(index, token)| is_significant(token).then_some(index))
}

fn next_significant(tokens: &[FormatToken], start: usize) -> Option<usize> {
    tokens
        .iter()
        .enumerate()
        .skip(start)
        .find_map(|(index, token)| is_significant(token).then_some(index))
}

fn has_code_before_line(tokens: &[FormatToken], start: usize) -> bool {
    for token in tokens[..start.min(tokens.len())].iter().rev() {
        match token.kind() {
            TokenKind::Newline | TokenKind::SoftNewline => return false,
            _ if is_significant(token) => return true,
            _ => {}
        }
    }
    false
}

fn is_significant(token: &FormatToken) -> bool {
    !token.kind().is_trivia() && !token.kind().is_layout() && token.kind() != TokenKind::Eof
}

fn target_for_comment(
    root: &FormatNode,
    tokens: &[FormatToken],
    comment_start: usize,
    previous: Option<usize>,
    next: Option<usize>,
) -> Option<NodeTarget> {
    // A comment token already inside a top-level CST child belongs to that
    // declaration even when its neighboring significant token is on another
    // line (for example a comment inside an offside block).
    if let Some(target) = top_level_target(root, comment_start) {
        return Some(target);
    }
    if let Some(previous) = previous.filter(|_| has_code_before_line(tokens, comment_start))
        && let Some(target) = top_level_target(root, previous)
    {
        return Some(target);
    }
    if let Some(next) = next
        && let Some(target) = top_level_target(root, next)
    {
        return Some(target);
    }
    previous
        .and_then(|index| top_level_target(root, index))
        .or_else(|| next.and_then(|index| top_level_target(root, index)))
        .or_else(|| {
            let range = root.token_range();
            (!range.is_empty()).then(|| NodeTarget {
                kind: root.kind(),
                range: RangeBounds::new(&range),
            })
        })
}

fn top_level_target(root: &FormatNode, token_index: usize) -> Option<NodeTarget> {
    root.children().iter().find_map(|child| {
        let range = child.token_range();
        RangeBounds::new(&range)
            .contains(token_index)
            .then(|| NodeTarget {
                kind: child.kind(),
                range: RangeBounds::new(&range),
            })
    })
}

#[cfg(test)]
mod tests {
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;
    use crate::build_format_ir;

    fn document(input: &str) -> crate::FormatDocument {
        let source = SourceFile::from_bytes(
            SourceId::new(15104),
            "comments.ling",
            input.as_bytes().to_vec(),
        )
        .expect("valid UTF-8 source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:#?}", parsed.parse_errors());
        build_format_ir(&source, &parsed).expect("Format IR builds")
    }

    #[test]
    fn attaches_documentation_and_trailing_comments_to_their_declarations() {
        let document = document(concat!(
            "/// 文档\n",
            "let first=1// 行尾\n",
            "\n",
            "// 第二个\n",
            "let second=2\n",
        ));
        let attachments = document.comment_attachments();
        assert_eq!(attachments.len(), 3);
        assert_eq!(attachments[0].kind(), CommentKind::Documentation);
        assert_eq!(attachments[0].placement(), CommentPlacement::Leading);
        assert_eq!(attachments[0].target_kind(), Some(NodeKind::LetDeclaration));
        assert_eq!(attachments[1].placement(), CommentPlacement::Trailing);
        assert_eq!(attachments[1].target_kind(), Some(NodeKind::LetDeclaration));
        assert_eq!(attachments[2].placement(), CommentPlacement::Leading);
        assert_eq!(attachments[2].target_kind(), Some(NodeKind::LetDeclaration));
        assert!(
            attachments[1].comment_token_range().start < attachments[2].comment_token_range().start
        );
    }

    #[test]
    fn groups_multiline_nested_block_comment_segments_without_crossing_definitions() {
        let document = document(concat!(
            "/* 外层\n",
            "   /* 内层 */\n",
            "*/\n",
            "let value=1\n",
        ));
        let attachments = document.comment_attachments();
        assert_eq!(attachments.len(), 1);
        assert_eq!(attachments[0].kind(), CommentKind::Block);
        assert_eq!(attachments[0].target_kind(), Some(NodeKind::LetDeclaration));
        let range = attachments[0].comment_token_range();
        assert!(range.end - range.start >= 3);
        assert_eq!(
            document.tokens()[range]
                .iter()
                .filter(|token| token.kind() == TokenKind::BlockComment)
                .count(),
            3
        );
    }

    #[test]
    fn keeps_file_only_comments_standalone_and_in_source_order() {
        let document = document("// 一\n// 二\n");
        let attachments = document.comment_attachments();
        assert_eq!(attachments.len(), 2);
        assert!(attachments.iter().all(|attachment| {
            attachment.placement() == CommentPlacement::Standalone
                && attachment.target_kind() == Some(NodeKind::Program)
        }));
        assert!(
            attachments[0].comment_token_range().start < attachments[1].comment_token_range().start
        );
    }
}
