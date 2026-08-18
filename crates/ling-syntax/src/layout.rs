use ling_source::{SourceFile, Span};

use crate::lexer::{LexError, LexErrorKind};
use crate::token::{Token, TokenKind};

const MAX_LAYOUT_DEPTH: usize = 256;
const MAX_DELIMITER_DEPTH: usize = 256;

pub(crate) fn apply(
    source: &SourceFile,
    raw_tokens: Vec<Token>,
    errors: &mut Vec<LexError>,
) -> Vec<Token> {
    let mut layout = Layout::new(source, errors);
    for token in raw_tokens {
        layout.push(token);
    }
    layout.finish()
}

struct Layout<'source, 'errors> {
    source: &'source SourceFile,
    errors: &'errors mut Vec<LexError>,
    output: Vec<Token>,
    indent_stack: Vec<u32>,
    delimiters: Vec<(TokenKind, Span)>,
    at_line_start: bool,
    indent_collecting: bool,
    line_indent: u32,
    tab_reported: bool,
    eof_offset: usize,
}

impl<'source, 'errors> Layout<'source, 'errors> {
    fn new(source: &'source SourceFile, errors: &'errors mut Vec<LexError>) -> Self {
        Self {
            source,
            errors,
            output: Vec::new(),
            indent_stack: vec![0],
            delimiters: Vec::new(),
            at_line_start: true,
            indent_collecting: true,
            line_indent: 0,
            tab_reported: false,
            eof_offset: source.lexical_text().len(),
        }
    }

    fn push(&mut self, mut token: Token) {
        if token.kind() == TokenKind::Eof {
            self.eof_offset = token.lexical_span().start().get() as usize;
            return;
        }

        if token.kind() == TokenKind::Newline {
            token.kind = if self.delimiters.is_empty() {
                TokenKind::Newline
            } else {
                TokenKind::SoftNewline
            };
            self.output.push(token);
            self.reset_line();
            return;
        }

        if self.at_line_start {
            match token.kind() {
                TokenKind::Whitespace if self.indent_collecting => {
                    self.collect_indentation(&token);
                    self.output.push(token);
                    return;
                }
                TokenKind::Whitespace => {
                    self.output.push(token);
                    return;
                }
                TokenKind::LineComment | TokenKind::DocComment | TokenKind::BlockComment => {
                    self.indent_collecting = false;
                    self.output.push(token);
                    return;
                }
                _ => {
                    if self.delimiters.is_empty() {
                        self.apply_indentation(&token);
                    }
                    self.at_line_start = false;
                    self.indent_collecting = false;
                }
            }
        }

        self.update_delimiters(&token);
        self.output.push(token);
    }

    fn finish(mut self) -> Vec<Token> {
        if !self.at_line_start {
            self.output.push(Token::synthetic(
                TokenKind::Newline,
                self.source,
                self.eof_offset,
            ));
        }

        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.output.push(Token::synthetic(
                TokenKind::Dedent,
                self.source,
                self.eof_offset,
            ));
        }

        for (opening, span) in self.delimiters.drain(..).rev() {
            self.errors.push(LexError::new(
                LexErrorKind::UnclosedDelimiter {
                    expected: opening
                        .matching_close()
                        .expect("delimiter stack contains opening tokens"),
                },
                span,
            ));
        }
        self.output.push(Token::synthetic(
            TokenKind::Eof,
            self.source,
            self.eof_offset,
        ));
        self.output
    }

    fn reset_line(&mut self) {
        self.at_line_start = true;
        self.indent_collecting = true;
        self.line_indent = 0;
        self.tab_reported = false;
    }

    fn collect_indentation(&mut self, whitespace: &Token) {
        let start = whitespace.lexical_span().start().get() as usize;
        let end = whitespace.lexical_span().end().get() as usize;
        for character in self.source.lexical_text()[start..end].chars() {
            if character == '\t' && !self.tab_reported {
                self.errors.push(LexError::new(
                    LexErrorKind::TabInIndentation,
                    whitespace.span(),
                ));
                self.tab_reported = true;
            }
            self.line_indent = self.line_indent.saturating_add(1);
        }
    }

    fn apply_indentation(&mut self, token: &Token) {
        let current = *self
            .indent_stack
            .last()
            .expect("indent stack always contains the root");
        if self.line_indent > current {
            if self.indent_stack.len() >= MAX_LAYOUT_DEPTH {
                self.errors.push(LexError::new(
                    LexErrorKind::LayoutNestingTooDeep,
                    token.span(),
                ));
                return;
            }
            self.indent_stack.push(self.line_indent);
            self.output.push(Token::synthetic(
                TokenKind::Indent,
                self.source,
                token.lexical_span().start().get() as usize,
            ));
            return;
        }

        if self.line_indent == current {
            return;
        }

        while self
            .indent_stack
            .last()
            .is_some_and(|indent| *indent > self.line_indent)
        {
            self.indent_stack.pop();
            self.output.push(Token::synthetic(
                TokenKind::Dedent,
                self.source,
                token.lexical_span().start().get() as usize,
            ));
        }
        let recovered_to = *self
            .indent_stack
            .last()
            .expect("indent stack always contains the root");
        if recovered_to != self.line_indent {
            self.errors.push(LexError::new(
                LexErrorKind::InconsistentDedent {
                    actual: self.line_indent,
                    recovered_to,
                },
                token.span(),
            ));
        }
    }

    fn update_delimiters(&mut self, token: &Token) {
        if token.kind().opens_delimiter() {
            if self.delimiters.len() >= MAX_DELIMITER_DEPTH {
                self.errors.push(LexError::new(
                    LexErrorKind::LayoutNestingTooDeep,
                    token.span(),
                ));
                return;
            }
            self.delimiters.push((token.kind(), token.span()));
            return;
        }
        if !token.kind().closes_delimiter() {
            return;
        }

        let Some((opening, _)) = self.delimiters.pop() else {
            self.errors.push(LexError::new(
                LexErrorKind::UnmatchedClosingDelimiter {
                    found: token.kind(),
                },
                token.span(),
            ));
            return;
        };
        let expected = opening
            .matching_close()
            .expect("delimiter stack contains opening tokens");
        if token.kind() != expected {
            self.errors.push(LexError::new(
                LexErrorKind::MismatchedClosingDelimiter {
                    expected,
                    found: token.kind(),
                },
                token.span(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use ling_source::SourceId;

    use super::*;
    use crate::lexer::lex;

    fn layout_kinds(input: &str) -> (Vec<TokenKind>, Vec<LexErrorKind>) {
        let source =
            SourceFile::from_bytes(SourceId::new(0), "layout.ling", input.as_bytes().to_vec())
                .unwrap();
        let lexed = lex(&source);
        let kinds = lexed
            .tokens()
            .iter()
            .map(Token::kind)
            .filter(|kind| !kind.is_trivia())
            .collect();
        let errors = lexed
            .errors()
            .iter()
            .map(|error| error.kind().clone())
            .collect();
        (kinds, errors)
    }

    #[test]
    fn emits_relative_indent_and_dedent_without_four_space_requirement() {
        let (kinds, errors) = layout_kinds("let x =\n  1\nlet y = 2\n");

        assert!(errors.is_empty());
        assert_eq!(
            kinds,
            [
                TokenKind::Let,
                TokenKind::Identifier,
                TokenKind::Equals,
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Integer,
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Let,
                TokenKind::Identifier,
                TokenKind::Equals,
                TokenKind::Integer,
                TokenKind::Newline,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn delimiters_produce_soft_newlines_without_layout_changes() {
        let (kinds, errors) = layout_kinds("let x = {\n  a = 1\n  b = 2\n}\n");

        assert!(errors.is_empty());
        assert_eq!(
            kinds
                .iter()
                .filter(|kind| matches!(kind, TokenKind::Indent | TokenKind::Dedent))
                .count(),
            0
        );
        assert_eq!(
            kinds
                .iter()
                .filter(|kind| **kind == TokenKind::SoftNewline)
                .count(),
            3
        );
    }

    #[test]
    fn ignores_blank_and_comment_only_line_indentation() {
        let (kinds, errors) = layout_kinds("let x =\n  // comment\n\n  1\n");

        assert!(errors.is_empty());
        assert_eq!(
            kinds
                .iter()
                .filter(|kind| **kind == TokenKind::Indent)
                .count(),
            1
        );
    }

    #[test]
    fn reports_tabs_and_inconsistent_dedent() {
        let (_, errors) = layout_kinds("let x =\n\t1\nlet y =\n  2\n    3\n 1\n");

        assert!(errors.contains(&LexErrorKind::TabInIndentation));
        assert!(errors.iter().any(|error| matches!(
            error,
            LexErrorKind::InconsistentDedent {
                actual: 1,
                recovered_to: 0
            }
        )));
    }

    #[test]
    fn enforces_the_documented_delimiter_depth_boundary() {
        let at_limit = format!("{}{}", "[".repeat(256), "]".repeat(256));
        let beyond_limit = format!("{}{}", "[".repeat(257), "]".repeat(257));

        assert!(layout_kinds(&at_limit).1.is_empty());
        assert!(
            layout_kinds(&beyond_limit)
                .1
                .contains(&LexErrorKind::LayoutNestingTooDeep)
        );
    }

    #[test]
    fn enforces_the_documented_layout_depth_boundary() {
        fn nested_lines(levels: usize) -> String {
            let mut source = String::new();
            for level in 1..=levels {
                source.push_str(&" ".repeat(level));
                source.push_str("x\n");
            }
            source
        }

        assert!(layout_kinds(&nested_lines(255)).1.is_empty());
        assert!(
            layout_kinds(&nested_lines(256))
                .1
                .contains(&LexErrorKind::LayoutNestingTooDeep)
        );
    }
}
