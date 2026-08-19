use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_source::{LexicalOffset, LexicalSpan, SourceFile, Span};
use ling_unicode::{
    IdentifierError, inspect_identifier, is_identifier_continue, is_identifier_start,
};

use crate::layout;
use crate::token::{FloatLiteral, IntegerLiteral, Token, TokenKind, TokenValue};

const MAX_COMMENT_DEPTH: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LexedSource {
    tokens: Vec<Token>,
    errors: Vec<LexError>,
}

impl LexedSource {
    #[must_use]
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    #[must_use]
    pub fn errors(&self) -> &[LexError] {
        &self.errors
    }

    #[must_use]
    pub fn into_parts(self) -> (Vec<Token>, Vec<LexError>) {
        (self.tokens, self.errors)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LexError {
    kind: LexErrorKind,
    span: Span,
}

impl LexError {
    pub(crate) fn new(kind: LexErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub(crate) fn at_lexical(
        kind: LexErrorKind,
        source: &SourceFile,
        start: usize,
        end: usize,
    ) -> Self {
        let lexical_span = LexicalSpan::new(
            source.id(),
            LexicalOffset::new(u32::try_from(start).expect("source length is bounded")),
            LexicalOffset::new(u32::try_from(end).expect("source length is bounded")),
        )
        .expect("lexer emits forward spans");
        let span = source
            .source_map()
            .original_span(lexical_span)
            .expect("lexical boundaries always map to original source");
        Self { kind, span }
    }

    #[must_use]
    pub const fn kind(&self) -> &LexErrorKind {
        &self.kind
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub fn to_diagnostic(&self, file: &str) -> Diagnostic {
        let span = DiagnosticSpan::new(file, self.span);
        match &self.kind {
            LexErrorKind::InvalidIdentifier(error) => Diagnostic::new(
                codes::INVALID_IDENTIFIER,
                Severity::Error,
                "标识符包含不允许的 Unicode 字符",
                "identifier contains a disallowed Unicode character",
            )
            .with_primary_span(span)
            .with_fact("reason", error.to_string()),
            LexErrorKind::UnexpectedCharacter(character) => Diagnostic::new(
                codes::UNEXPECTED_CHARACTER,
                Severity::Error,
                format!("无法识别字符 U+{:04X}", u32::from(*character)),
                format!("unrecognized character U+{:04X}", u32::from(*character)),
            )
            .with_primary_span(span)
            .with_fact("codepoint", format!("U+{:04X}", u32::from(*character))),
            LexErrorKind::UnterminatedBlockComment => Diagnostic::new(
                codes::UNTERMINATED_BLOCK_COMMENT,
                Severity::Error,
                "块注释未闭合",
                "unterminated block comment",
            )
            .with_primary_span(span),
            LexErrorKind::CommentNestingTooDeep => Diagnostic::new(
                codes::COMMENT_NESTING_TOO_DEEP,
                Severity::Error,
                "块注释嵌套超过 256 层",
                "block-comment nesting exceeds 256 levels",
            )
            .with_primary_span(span)
            .with_fact("maximum_depth", 256_u64),
            LexErrorKind::UnterminatedText => Diagnostic::new(
                codes::UNTERMINATED_TEXT,
                Severity::Error,
                "Text 字面量未闭合",
                "unterminated Text literal",
            )
            .with_primary_span(span),
            LexErrorKind::InvalidTextEscape => Diagnostic::new(
                codes::INVALID_TEXT_ESCAPE,
                Severity::Error,
                "Text 字面量包含无效转义",
                "Text literal contains an invalid escape",
            )
            .with_primary_span(span),
            LexErrorKind::InvalidUnicodeEscape => Diagnostic::new(
                codes::INVALID_UNICODE_ESCAPE,
                Severity::Error,
                "Text 字面量包含无效 Unicode 转义",
                "Text literal contains an invalid Unicode escape",
            )
            .with_primary_span(span),
            LexErrorKind::InvalidNumber => Diagnostic::new(
                codes::INVALID_NUMBER,
                Severity::Error,
                "数字字面量格式无效",
                "invalid numeric literal",
            )
            .with_primary_span(span),
            LexErrorKind::UnsupportedCharacterLiteral => Diagnostic::new(
                codes::UNSUPPORTED_CHARACTER_LITERAL,
                Severity::Error,
                "Ling Seed 不支持 Char 字面量；请使用 Text",
                "Ling Seed does not support Char literals; use Text instead",
            )
            .with_primary_span(span),
            LexErrorKind::TabInIndentation => Diagnostic::new(
                codes::TAB_IN_INDENTATION,
                Severity::Error,
                "语义缩进不能使用 Tab",
                "tabs are not allowed in semantic indentation",
            )
            .with_primary_span(span),
            LexErrorKind::InconsistentDedent {
                actual,
                recovered_to,
            } => Diagnostic::new(
                codes::INCONSISTENT_DEDENT,
                Severity::Error,
                "Dedent 未对齐到已有缩进层级",
                "dedent does not align with an existing indentation level",
            )
            .with_primary_span(span)
            .with_fact("actual_column", u64::from(*actual))
            .with_fact("recovered_column", u64::from(*recovered_to)),
            LexErrorKind::LayoutNestingTooDeep => Diagnostic::new(
                codes::LAYOUT_NESTING_TOO_DEEP,
                Severity::Error,
                "Layout 或 delimiter 嵌套超过 256 层",
                "layout or delimiter nesting exceeds 256 levels",
            )
            .with_primary_span(span)
            .with_fact("maximum_depth", 256_u64),
            LexErrorKind::UnmatchedClosingDelimiter { found } => Diagnostic::new(
                codes::UNMATCHED_CLOSING_DELIMITER,
                Severity::Error,
                "存在没有对应开始符号的结束 delimiter",
                "closing delimiter has no matching opening delimiter",
            )
            .with_primary_span(span)
            .with_fact("found", format!("{found:?}")),
            LexErrorKind::MismatchedClosingDelimiter { expected, found } => Diagnostic::new(
                codes::MISMATCHED_CLOSING_DELIMITER,
                Severity::Error,
                "结束 delimiter 与开始 delimiter 不匹配",
                "closing delimiter does not match the opening delimiter",
            )
            .with_primary_span(span)
            .with_fact("expected", format!("{expected:?}"))
            .with_fact("found", format!("{found:?}")),
            LexErrorKind::UnclosedDelimiter { expected } => Diagnostic::new(
                codes::UNCLOSED_DELIMITER,
                Severity::Error,
                "delimiter 在文件结尾前未闭合",
                "delimiter is not closed before the end of the file",
            )
            .with_primary_span(span)
            .with_fact("expected", format!("{expected:?}")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexErrorKind {
    InvalidIdentifier(IdentifierError),
    UnexpectedCharacter(char),
    UnterminatedBlockComment,
    CommentNestingTooDeep,
    UnterminatedText,
    InvalidTextEscape,
    InvalidUnicodeEscape,
    InvalidNumber,
    UnsupportedCharacterLiteral,
    TabInIndentation,
    InconsistentDedent {
        actual: u32,
        recovered_to: u32,
    },
    LayoutNestingTooDeep,
    UnmatchedClosingDelimiter {
        found: TokenKind,
    },
    MismatchedClosingDelimiter {
        expected: TokenKind,
        found: TokenKind,
    },
    UnclosedDelimiter {
        expected: TokenKind,
    },
}

#[must_use]
pub fn lex(source: &SourceFile) -> LexedSource {
    let (raw_tokens, mut errors) = RawLexer::new(source).run();
    let tokens = layout::apply(source, raw_tokens, &mut errors);
    LexedSource { tokens, errors }
}

struct RawLexer<'source> {
    source: &'source SourceFile,
    input: &'source str,
    offset: usize,
    tokens: Vec<Token>,
    errors: Vec<LexError>,
}

impl<'source> RawLexer<'source> {
    fn new(source: &'source SourceFile) -> Self {
        Self {
            source,
            input: source.lexical_text(),
            offset: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn run(mut self) -> (Vec<Token>, Vec<LexError>) {
        while self.offset < self.input.len() {
            let start = self.offset;
            let character = self.current_character();
            match character {
                ' ' | '\t' => self.lex_whitespace(),
                '\n' => {
                    self.advance_character();
                    self.push(TokenKind::Newline, start, self.offset, None);
                }
                '/' if self.remaining().starts_with("//") => self.lex_line_comment(),
                '/' if self.remaining().starts_with("/*") => self.lex_block_comment(),
                '"' => self.lex_text(),
                '0'..='9' => self.lex_number(),
                '\'' => self.lex_apostrophe(),
                '_' => self.lex_identifier(),
                character if is_identifier_start(character) => self.lex_identifier(),
                _ => self.lex_punctuation_or_error(),
            }
        }

        self.tokens.push(Token::synthetic(
            TokenKind::Eof,
            self.source,
            self.input.len(),
        ));
        (self.tokens, self.errors)
    }

    fn lex_whitespace(&mut self) {
        let start = self.offset;
        while matches!(self.peek_character(), Some(' ' | '\t')) {
            self.advance_character();
        }
        self.push(TokenKind::Whitespace, start, self.offset, None);
    }

    fn lex_line_comment(&mut self) {
        let start = self.offset;
        let kind = if self.remaining().starts_with("///") {
            TokenKind::DocComment
        } else {
            TokenKind::LineComment
        };
        while self
            .peek_character()
            .is_some_and(|character| character != '\n')
        {
            self.advance_character();
        }
        self.push(kind, start, self.offset, None);
    }

    fn lex_block_comment(&mut self) {
        let comment_start = self.offset;
        let mut segment_start = self.offset;
        let mut depth = 0_usize;

        while self.offset < self.input.len() {
            if self.remaining().starts_with("/*") {
                depth += 1;
                if depth > MAX_COMMENT_DEPTH {
                    self.error_at(
                        LexErrorKind::CommentNestingTooDeep,
                        self.offset,
                        self.offset + 2,
                    );
                }
                self.offset += 2;
                continue;
            }
            if self.remaining().starts_with("*/") {
                self.offset += 2;
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    self.push(TokenKind::BlockComment, segment_start, self.offset, None);
                    return;
                }
                continue;
            }
            if self.current_character() == '\n' {
                if segment_start < self.offset {
                    self.push(TokenKind::BlockComment, segment_start, self.offset, None);
                }
                let newline_start = self.offset;
                self.advance_character();
                self.push(TokenKind::Newline, newline_start, self.offset, None);
                segment_start = self.offset;
                continue;
            }
            self.advance_character();
        }

        if segment_start < self.offset {
            self.push(TokenKind::BlockComment, segment_start, self.offset, None);
        }
        self.error_at(
            LexErrorKind::UnterminatedBlockComment,
            comment_start,
            self.offset,
        );
    }

    fn lex_text(&mut self) {
        let start = self.offset;
        self.advance_character();
        let mut decoded = String::new();
        let mut terminated = false;

        while self.offset < self.input.len() {
            let character = self.current_character();
            match character {
                '"' => {
                    self.advance_character();
                    terminated = true;
                    break;
                }
                '\n' => break,
                '\\' => self.lex_escape(&mut decoded),
                _ => {
                    decoded.push(character);
                    self.advance_character();
                }
            }
        }

        if !terminated {
            self.error_at(LexErrorKind::UnterminatedText, start, self.offset);
        }
        self.push(
            TokenKind::Text,
            start,
            self.offset,
            Some(TokenValue::Text(decoded)),
        );
    }

    fn lex_escape(&mut self, decoded: &mut String) {
        let escape_start = self.offset;
        self.advance_character();
        let Some(character) = self.peek_character() else {
            self.error_at(LexErrorKind::InvalidTextEscape, escape_start, self.offset);
            return;
        };
        self.advance_character();
        match character {
            '\\' => decoded.push('\\'),
            '"' => decoded.push('"'),
            'n' => decoded.push('\n'),
            'r' => decoded.push('\r'),
            't' => decoded.push('\t'),
            '0' => decoded.push('\0'),
            'u' => self.lex_unicode_escape(escape_start, decoded),
            _ => self.error_at(LexErrorKind::InvalidTextEscape, escape_start, self.offset),
        }
    }

    fn lex_unicode_escape(&mut self, escape_start: usize, decoded: &mut String) {
        if self.peek_character() != Some('{') {
            self.error_at(
                LexErrorKind::InvalidUnicodeEscape,
                escape_start,
                self.offset,
            );
            return;
        }
        self.advance_character();
        let digits_start = self.offset;
        while self
            .peek_character()
            .is_some_and(|character| character.is_ascii_hexdigit())
            && self.offset - digits_start < 6
        {
            self.advance_character();
        }
        let digits_end = self.offset;
        let valid_shape = digits_end > digits_start && self.peek_character() == Some('}');
        if self.peek_character() == Some('}') {
            self.advance_character();
        }
        let value = valid_shape
            .then(|| u32::from_str_radix(&self.input[digits_start..digits_end], 16).ok())
            .flatten()
            .and_then(char::from_u32);
        if let Some(value) = value {
            decoded.push(value);
        } else {
            self.error_at(
                LexErrorKind::InvalidUnicodeEscape,
                escape_start,
                self.offset,
            );
        }
    }

    fn lex_identifier(&mut self) {
        let start = self.offset;
        self.advance_character();
        while self.peek_character().is_some_and(is_identifier_continue) {
            self.advance_character();
        }
        let spelling = &self.input[start..self.offset];
        match inspect_identifier(spelling) {
            Ok(identifier) => {
                let kind = keyword(identifier.identifier().normalized());
                let value = (kind == TokenKind::Identifier)
                    .then(|| TokenValue::Identifier(Box::new(identifier)));
                self.push(kind, start, self.offset, value);
            }
            Err(error) => {
                self.error_at(LexErrorKind::InvalidIdentifier(error), start, self.offset);
                self.push(TokenKind::Error, start, self.offset, None);
            }
        }
    }

    fn lex_apostrophe(&mut self) {
        let start = self.offset;
        let mut characters = self.remaining().chars();
        debug_assert_eq!(characters.next(), Some('\''));
        let value = characters.next();
        let closing = characters.next();

        if value.is_some_and(|value| value != '\n' && value != '\'') && closing == Some('\'') {
            self.advance_character();
            self.advance_character();
            self.advance_character();
            self.error_at(
                LexErrorKind::UnsupportedCharacterLiteral,
                start,
                self.offset,
            );
            self.push(TokenKind::Error, start, self.offset, None);
            return;
        }

        self.advance_character();
        self.push(TokenKind::Apostrophe, start, self.offset, None);
    }

    fn lex_number(&mut self) {
        let start = self.offset;
        if self.remaining().starts_with("0b")
            || self.remaining().starts_with("0o")
            || self.remaining().starts_with("0x")
        {
            self.lex_based_integer(start);
            return;
        }

        let integer_end = self.scan_digit_segment(10);
        let mut is_float = false;
        if self.peek_character() == Some('.')
            && self
                .peek_second_character()
                .is_some_and(|character| character.is_ascii_digit())
        {
            is_float = true;
            self.advance_character();
            self.scan_digit_segment(10);
        }
        if matches!(self.peek_character(), Some('e' | 'E')) {
            is_float = true;
            self.advance_character();
            if matches!(self.peek_character(), Some('+' | '-')) {
                self.advance_character();
            }
            let exponent_start = self.offset;
            self.scan_digit_segment(10);
            if exponent_start == self.offset {
                self.consume_identifier_suffix();
                self.error_at(LexErrorKind::InvalidNumber, start, self.offset);
            }
        }

        let end = self.offset;
        let spelling = &self.input[start..end];
        let segments_valid = number_underscores_are_valid(spelling);
        if !segments_valid || self.consume_identifier_suffix() {
            self.error_at(LexErrorKind::InvalidNumber, start, self.offset);
            self.push(TokenKind::Error, start, self.offset, None);
            return;
        }

        let normalized = spelling.replace('_', "");
        let value_is_valid = if is_float {
            normalized.parse::<f64>().is_ok_and(f64::is_finite)
        } else {
            normalized == "0" || !normalized.starts_with('0')
        };
        if !value_is_valid {
            self.error_at(LexErrorKind::InvalidNumber, start, end);
            self.push(TokenKind::Error, start, end, None);
            return;
        }
        let value = if is_float {
            TokenValue::Float(FloatLiteral::new(normalized))
        } else {
            debug_assert_eq!(integer_end, end);
            TokenValue::Integer(IntegerLiteral::new(10, normalized))
        };
        self.push(
            if is_float {
                TokenKind::Float
            } else {
                TokenKind::Integer
            },
            start,
            end,
            Some(value),
        );
    }

    fn lex_based_integer(&mut self, start: usize) {
        let radix = match self.input.as_bytes()[start + 1] {
            b'b' => 2,
            b'o' => 8,
            b'x' => 16,
            _ => unreachable!("caller checked integer base prefix"),
        };
        self.offset += 2;
        let digits_start = self.offset;
        while self
            .peek_character()
            .is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
        {
            self.advance_character();
        }
        let end = self.offset;
        let spelling = &self.input[digits_start..end];
        let valid = !spelling.is_empty()
            && number_underscores_are_valid(spelling)
            && spelling
                .chars()
                .filter(|character| *character != '_')
                .all(|character| character.is_digit(radix));
        if !valid {
            self.error_at(LexErrorKind::InvalidNumber, start, end);
            self.push(TokenKind::Error, start, end, None);
            return;
        }
        self.push(
            TokenKind::Integer,
            start,
            end,
            Some(TokenValue::Integer(IntegerLiteral::new(
                radix,
                spelling.replace('_', ""),
            ))),
        );
    }

    fn scan_digit_segment(&mut self, radix: u32) -> usize {
        while self
            .peek_character()
            .is_some_and(|character| character == '_' || character.is_digit(radix))
        {
            self.advance_character();
        }
        self.offset
    }

    fn consume_identifier_suffix(&mut self) -> bool {
        let start = self.offset;
        while self
            .peek_character()
            .is_some_and(|character| character == '_' || is_identifier_continue(character))
        {
            self.advance_character();
        }
        self.offset > start
    }

    fn lex_punctuation_or_error(&mut self) {
        let start = self.offset;
        let punctuation = [
            ("==", TokenKind::EqualEqual),
            ("!=", TokenKind::BangEqual),
            ("<=", TokenKind::LessEqual),
            (">=", TokenKind::GreaterEqual),
            ("<-", TokenKind::LeftArrow),
            ("->", TokenKind::RightArrow),
            ("|>", TokenKind::PipeGreater),
            ("&&", TokenKind::AmpAmp),
            ("||", TokenKind::PipePipe),
        ];
        if let Some((spelling, kind)) = punctuation
            .into_iter()
            .find(|(spelling, _)| self.remaining().starts_with(spelling))
        {
            self.offset += spelling.len();
            self.push(kind, start, self.offset, None);
            return;
        }

        let kind = match self.current_character() {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            ':' => TokenKind::Colon,
            '.' => TokenKind::Dot,
            '=' => TokenKind::Equals,
            '<' => TokenKind::Less,
            '>' => TokenKind::Greater,
            '|' => TokenKind::Pipe,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            character => {
                self.advance_character();
                let single = character.len_utf8();
                let identifier_error = inspect_identifier(&character.to_string()).err();
                let error = identifier_error.map_or(
                    LexErrorKind::UnexpectedCharacter(character),
                    LexErrorKind::InvalidIdentifier,
                );
                self.error_at(error, start, start + single);
                self.push(TokenKind::Error, start, self.offset, None);
                return;
            }
        };
        self.advance_character();
        self.push(kind, start, self.offset, None);
    }

    fn current_character(&self) -> char {
        self.peek_character()
            .expect("current offset is before the end of input")
    }

    fn peek_character(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn peek_second_character(&self) -> Option<char> {
        self.remaining().chars().nth(1)
    }

    fn advance_character(&mut self) {
        self.offset += self.current_character().len_utf8();
    }

    fn remaining(&self) -> &str {
        &self.input[self.offset..]
    }

    fn push(&mut self, kind: TokenKind, start: usize, end: usize, value: Option<TokenValue>) {
        self.tokens
            .push(Token::new(kind, self.source, start, end, value));
    }

    fn error_at(&mut self, kind: LexErrorKind, start: usize, end: usize) {
        self.errors
            .push(LexError::at_lexical(kind, self.source, start, end));
    }
}

fn keyword(identifier: &str) -> TokenKind {
    match identifier {
        "let" => TokenKind::Let,
        "mutable" => TokenKind::Mutable,
        "rec" => TokenKind::Rec,
        "and" => TokenKind::And,
        "type" => TokenKind::Type,
        "of" => TokenKind::Of,
        "match" => TokenKind::Match,
        "with" => TokenKind::With,
        "when" => TokenKind::When,
        "if" => TokenKind::If,
        "then" => TokenKind::Then,
        "else" => TokenKind::Else,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "module" => TokenKind::Module,
        "import" => TokenKind::Import,
        "as" => TokenKind::As,
        "requires" => TokenKind::Requires,
        _ => TokenKind::Identifier,
    }
}

fn number_underscores_are_valid(input: &str) -> bool {
    !input.starts_with('_')
        && !input.ends_with('_')
        && !input.contains("__")
        && !input.contains("._")
        && !input.contains("_.")
        && !input.contains("e_")
        && !input.contains("E_")
        && !input.contains("_e")
        && !input.contains("_E")
        && !input.contains("+_")
        && !input.contains("-_")
}

#[cfg(test)]
mod tests {
    use ling_source::SourceId;

    use super::*;

    fn lex_text(input: &str) -> LexedSource {
        let source =
            SourceFile::from_bytes(SourceId::new(0), "test.ling", input.as_bytes().to_vec())
                .unwrap();
        lex(&source)
    }

    fn significant_kinds(lexed: &LexedSource) -> Vec<TokenKind> {
        lexed
            .tokens()
            .iter()
            .map(Token::kind)
            .filter(|kind| !kind.is_trivia())
            .collect()
    }

    #[test]
    fn lexes_chinese_identifiers_keywords_and_pipeline() {
        let lexed = lex_text("let 人物ID = 人物 |> map 计算\n");

        assert!(lexed.errors().is_empty());
        assert_eq!(
            significant_kinds(&lexed),
            [
                TokenKind::Let,
                TokenKind::Identifier,
                TokenKind::Equals,
                TokenKind::Identifier,
                TokenKind::PipeGreater,
                TokenKind::Identifier,
                TokenKind::Identifier,
                TokenKind::Newline,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn decodes_seed_literals_without_fixed_width_integers() {
        let lexed = lex_text("0xff 1_000 1.5e-2 \"你好\\n\"\n");

        assert!(lexed.errors().is_empty());
        let values = lexed
            .tokens()
            .iter()
            .filter_map(Token::value)
            .collect::<Vec<_>>();
        assert_eq!(
            values,
            [
                &TokenValue::Integer(IntegerLiteral::new(16, "ff".to_owned())),
                &TokenValue::Integer(IntegerLiteral::new(10, "1000".to_owned())),
                &TokenValue::Float(FloatLiteral::new("1.5e-2".to_owned())),
                &TokenValue::Text("你好\n".to_owned()),
            ]
        );
    }

    #[test]
    fn reports_invalid_number_and_text_escape() {
        let lexed = lex_text("0b102 01 0_1 1e309 \"bad\\q\"\n");

        assert_eq!(
            lexed
                .errors()
                .iter()
                .map(LexError::kind)
                .collect::<Vec<_>>(),
            [
                &LexErrorKind::InvalidNumber,
                &LexErrorKind::InvalidNumber,
                &LexErrorKind::InvalidNumber,
                &LexErrorKind::InvalidNumber,
                &LexErrorKind::InvalidTextEscape,
            ]
        );
    }

    #[test]
    fn rejects_character_literals_without_consuming_type_variable_apostrophes() {
        let lexed = lex_text("'x' '人' 'a\n");

        assert_eq!(
            lexed
                .errors()
                .iter()
                .map(LexError::kind)
                .collect::<Vec<_>>(),
            [
                &LexErrorKind::UnsupportedCharacterLiteral,
                &LexErrorKind::UnsupportedCharacterLiteral,
            ]
        );
        assert_eq!(
            significant_kinds(&lexed),
            [
                TokenKind::Error,
                TokenKind::Error,
                TokenKind::Apostrophe,
                TokenKind::Identifier,
                TokenKind::Newline,
                TokenKind::Eof,
            ]
        );
        assert_eq!(
            lexed.errors()[0].to_diagnostic("test.ling").code().as_str(),
            "L-LEX-0012"
        );
    }

    #[test]
    fn preserves_newlines_inside_nested_block_comments() {
        let lexed = lex_text("/* outer\n /* inner */\n*/\nlet x = 1\n");

        assert!(lexed.errors().is_empty());
        assert_eq!(
            significant_kinds(&lexed),
            [
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Newline,
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
    fn enforces_the_documented_block_comment_depth_boundary() {
        let at_limit = format!("{}{}", "/*".repeat(256), "*/".repeat(256));
        let beyond_limit = format!("{}{}", "/*".repeat(257), "*/".repeat(257));

        assert!(lex_text(&at_limit).errors().is_empty());
        assert!(
            lex_text(&beyond_limit)
                .errors()
                .iter()
                .any(|error| error.kind() == &LexErrorKind::CommentNestingTooDeep)
        );
    }
}
