//! Conservative core-syntax Author Source formatting over [`FormatDocument`].
//!
//! The renderer consumes the compiler token stream projected by `format_ir`.
//! It does not inspect text with a second parser, infer semantic structure, or
//! move comments between CST regions.  Comment attachment and incomplete-source
//! recovery remain separate execution-plan tasks.

use ling_source::SourceFile;
use ling_syntax::{TokenKind, parse};

use crate::FormatDocument;

/// Formats valid Author Source core syntax using the compiler-owned Format IR.
///
/// The core slice emits four ASCII spaces for compiler `Indent` depth, uses LF
/// line endings, and applies deterministic spacing around core punctuation and
/// operators.  Original literal, identifier, and comment spelling is copied
/// byte-for-byte.  Invalid or incomplete input is returned unchanged; this is a
/// conservative boundary until the dedicated recovery task is implemented.
///
/// The candidate output is reparsed by the existing compiler parser before it
/// is returned.  If the compiler rejects the candidate, the original snapshot
/// is returned and no partial rewrite is published.
#[must_use]
pub fn format_core(document: &FormatDocument) -> String {
    if !document.is_valid() {
        return document.original_text().to_owned();
    }
    let candidate = Renderer::new(document.had_bom()).render(document);
    if parses_valid_source(document.source_id(), &candidate) {
        candidate
    } else {
        document.original_text().to_owned()
    }
}

fn parses_valid_source(source_id: ling_source::SourceId, candidate: &str) -> bool {
    let Ok(source) =
        SourceFile::from_bytes(source_id, "formatted.ling", candidate.as_bytes().to_vec())
    else {
        return false;
    };
    parse(&source).is_valid()
}

struct Renderer {
    output: String,
    indent_depth: usize,
    at_line_start: bool,
    soft_line: bool,
    pending_whitespace: String,
    previous: Option<TokenKind>,
    previous_unary: bool,
    last_comment: bool,
}

impl Renderer {
    fn new(had_bom: bool) -> Self {
        let mut output = String::new();
        if had_bom {
            output.push('\u{feff}');
        }
        Self {
            output,
            indent_depth: 0,
            at_line_start: true,
            soft_line: false,
            pending_whitespace: String::new(),
            previous: None,
            previous_unary: false,
            last_comment: false,
        }
    }

    fn render(mut self, document: &FormatDocument) -> String {
        for token in document.tokens() {
            match token.kind() {
                TokenKind::Whitespace => self.whitespace(token.text()),
                TokenKind::Newline | TokenKind::SoftNewline => self.newline(token.kind()),
                TokenKind::Indent => self.indent_depth = self.indent_depth.saturating_add(1),
                TokenKind::Dedent => self.indent_depth = self.indent_depth.saturating_sub(1),
                TokenKind::LineComment | TokenKind::DocComment | TokenKind::BlockComment => {
                    self.comment(token.text())
                }
                TokenKind::Eof => {}
                kind => self.significant(kind, token.text()),
            }
        }
        if !self.output.ends_with('\n') {
            self.output.push('\n');
        }
        self.output
    }

    fn whitespace(&mut self, text: &str) {
        if self.at_line_start {
            self.pending_whitespace.push_str(text);
        }
    }

    fn newline(&mut self, kind: TokenKind) {
        self.output.push('\n');
        self.at_line_start = true;
        self.soft_line = kind == TokenKind::SoftNewline;
        self.pending_whitespace.clear();
        self.previous = None;
        self.previous_unary = false;
        self.last_comment = false;
    }

    fn begin_line(&mut self) {
        if !self.at_line_start {
            return;
        }
        if self.soft_line {
            self.output.push_str(&self.pending_whitespace);
        } else {
            for _ in 0..self.indent_depth {
                self.output.push_str("    ");
            }
        }
        self.pending_whitespace.clear();
        self.at_line_start = false;
    }

    fn comment(&mut self, text: &str) {
        if self.at_line_start {
            // Comment-only lines retain their original relative indentation;
            // FMT-1504 owns a richer attachment model.
            self.output.push_str(&self.pending_whitespace);
            self.pending_whitespace.clear();
            self.at_line_start = false;
        } else if !self.output.ends_with([' ', '\n']) {
            self.output.push(' ');
        }
        self.output.push_str(text);
        self.last_comment = true;
    }

    fn significant(&mut self, current: TokenKind, text: &str) {
        self.begin_line();
        if self.last_comment {
            if !self.output.ends_with([' ', '\n']) {
                self.output.push(' ');
            }
        } else if let Some(previous) = self.previous {
            let current_unary = is_unary(current, self.previous);
            if needs_space(previous, current, self.previous_unary, current_unary) {
                self.output.push(' ');
            }
        }
        self.output.push_str(text);
        self.previous_unary = is_unary(current, self.previous);
        self.previous = Some(current);
        self.last_comment = false;
    }
}

fn is_unary(current: TokenKind, previous: Option<TokenKind>) -> bool {
    matches!(current, TokenKind::Plus | TokenKind::Minus)
        && previous.is_none_or(|kind| {
            is_operator(kind)
                || matches!(
                    kind,
                    TokenKind::LeftParen
                        | TokenKind::LeftBracket
                        | TokenKind::LeftBrace
                        | TokenKind::Comma
                        | TokenKind::Colon
                        | TokenKind::Apostrophe
                        | TokenKind::Then
                        | TokenKind::Else
                        | TokenKind::With
                        | TokenKind::Of
                        | TokenKind::As
                        | TokenKind::If
                        | TokenKind::Match
                        | TokenKind::When
                )
        })
}

fn needs_space(
    previous: TokenKind,
    current: TokenKind,
    previous_unary: bool,
    current_unary: bool,
) -> bool {
    if previous_unary {
        return false;
    }
    if matches!(
        current,
        TokenKind::RightParen
            | TokenKind::RightBracket
            | TokenKind::Comma
            | TokenKind::Semicolon
            | TokenKind::Dot
            | TokenKind::Colon
    ) {
        return false;
    }
    if current == TokenKind::RightBrace {
        return previous != TokenKind::LeftBrace;
    }
    if current == TokenKind::Apostrophe {
        return !matches!(
            previous,
            TokenKind::LeftParen
                | TokenKind::LeftBracket
                | TokenKind::LeftBrace
                | TokenKind::Less
                | TokenKind::Dot
                | TokenKind::Apostrophe
        );
    }
    if matches!(
        previous,
        TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::Dot | TokenKind::Apostrophe
    ) {
        return false;
    }
    if previous == TokenKind::LeftBrace {
        return current != TokenKind::RightBrace;
    }
    if matches!(
        previous,
        TokenKind::Comma | TokenKind::Semicolon | TokenKind::Colon
    ) {
        return true;
    }
    if matches!(
        current,
        TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::LeftBrace
    ) {
        return is_word_like(previous)
            || is_closing_delimiter(previous)
            || is_operator(previous)
            || matches!(previous, TokenKind::Comma | TokenKind::Colon);
    }
    if is_operator(previous) || is_operator(current) {
        return true;
    }
    if current_unary {
        return !matches!(
            previous,
            TokenKind::LeftParen
                | TokenKind::LeftBracket
                | TokenKind::LeftBrace
                | TokenKind::Dot
                | TokenKind::Apostrophe
        );
    }
    if is_word_like(previous) && is_word_like(current) {
        return true;
    }
    is_closing_delimiter(previous) && (is_word_like(current) || is_opening_delimiter(current))
}

const fn is_opening_delimiter(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::LeftBrace
    )
}

const fn is_closing_delimiter(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace
    )
}

const fn is_operator(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Equals
            | TokenKind::EqualEqual
            | TokenKind::BangEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::LeftArrow
            | TokenKind::RightArrow
            | TokenKind::Pipe
            | TokenKind::PipeGreater
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::AmpAmp
            | TokenKind::PipePipe
    )
}

const fn is_word_like(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Identifier
            | TokenKind::Integer
            | TokenKind::Float
            | TokenKind::Text
            | TokenKind::Let
            | TokenKind::Mutable
            | TokenKind::Rec
            | TokenKind::And
            | TokenKind::Type
            | TokenKind::Of
            | TokenKind::Match
            | TokenKind::With
            | TokenKind::When
            | TokenKind::If
            | TokenKind::Then
            | TokenKind::Else
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Module
            | TokenKind::Import
            | TokenKind::As
            | TokenKind::Requires
    )
}

#[cfg(test)]
mod tests {
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;
    use crate::build_format_ir;

    fn format(input: &str) -> String {
        let source =
            SourceFile::from_bytes(SourceId::new(80), "author.ling", input.as_bytes().to_vec())
                .expect("valid test source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:#?}", parsed.parse_errors());
        let document = build_format_ir(&source, &parsed).expect("Format IR builds");
        format_core(&document)
    }

    #[test]
    fn formats_core_spacing_and_four_space_layout() {
        assert_eq!(
            format("let add a b=\n  a+b\nlet main ()=add 1 2\n"),
            "let add a b =\n    a + b\nlet main () = add 1 2\n"
        );
    }

    #[test]
    fn formats_records_variants_matches_and_pipelines() {
        let formatted = format(concat!(
            "type Person={name:Text;mutable hp:Int}\n",
            "type Status=|Healthy|Hurt of Int\n",
            "let describe status=\n",
            "  match status with\n",
            "  |Healthy->\"ok\"\n",
            "  |Hurt amount->amount\n",
            "let total xs=\n",
            "  xs\n",
            "  |>sum\n",
        ));
        assert_eq!(
            formatted,
            concat!(
                "type Person = { name: Text; mutable hp: Int }\n",
                "type Status = | Healthy | Hurt of Int\n",
                "let describe status =\n",
                "    match status with\n",
                "    | Healthy -> \"ok\"\n",
                "    | Hurt amount -> amount\n",
                "let total xs =\n",
                "    xs\n",
                "    |> sum\n",
            )
        );
    }

    #[test]
    fn formats_modules_imports_and_mutable_assignments() {
        assert_eq!(
            format(concat!(
                "module Main\n",
                "  requires Console.Write\n",
                "\n",
                "import Game.Math as Math\n",
                "\n",
                "let main ()=\n",
                "  let mutable total=0\n",
                "  total<-total+1\n",
                "  Console.write\"ok\"\n",
            )),
            concat!(
                "module Main\n",
                "    requires Console.Write\n",
                "\n",
                "import Game.Math as Math\n",
                "\n",
                "let main () =\n",
                "    let mutable total = 0\n",
                "    total <- total + 1\n",
                "    Console.write \"ok\"\n",
            )
        );
    }

    #[test]
    fn preserves_comments_blank_lines_unicode_and_bom_while_normalizing_lf() {
        let source = SourceFile::from_bytes(
            SourceId::new(81),
            "author.ling",
            concat!(
                "\u{feff}let 中文=1//尾注\r\n",
                "\r\n",
                "  // 独立注释\r\n",
                "let 文本=\"保留\\n\"\r\n",
            )
            .as_bytes()
            .to_vec(),
        )
        .expect("valid test source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:#?}", parsed.parse_errors());
        let document = build_format_ir(&source, &parsed).expect("Format IR builds");
        assert_eq!(
            format_core(&document),
            concat!(
                "\u{feff}let 中文 = 1 //尾注\n",
                "\n",
                "  // 独立注释\n",
                "let 文本 = \"保留\\n\"\n",
            )
        );
    }

    #[test]
    fn returns_invalid_source_without_rewriting_or_normalizing_it() {
        let source = SourceFile::from_bytes(
            SourceId::new(82),
            "author.ling",
            b"let value=\"unterminated\r\n".to_vec(),
        )
        .expect("valid UTF-8 source");
        let parsed = parse(&source);
        assert!(!parsed.is_valid());
        let document = build_format_ir(&source, &parsed).expect("invalid IR still builds");
        assert_eq!(format_core(&document), source.original_text());
    }

    #[test]
    fn formatting_is_idempotent_for_the_core_slice() {
        let first = format("let f x=\n  if x>0 then\n    x+1\n  else\n    -x\n");
        let source =
            SourceFile::from_bytes(SourceId::new(83), "author.ling", first.as_bytes().to_vec())
                .expect("formatted source remains UTF-8");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:#?}", parsed.parse_errors());
        let document = build_format_ir(&source, &parsed).expect("Format IR builds");
        assert_eq!(format_core(&document), first);
    }
}
