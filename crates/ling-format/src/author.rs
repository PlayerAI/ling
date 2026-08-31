//! Conservative core-syntax Author Source formatting over [`FormatDocument`].
//!
//! The renderer consumes the compiler token stream projected by `format_ir`.
//! It does not inspect text with a second parser, infer semantic structure, or
//! move comments between CST regions.  Comment attachment metadata is consumed
//! as a preservation guard.  Incomplete-source recovery is deliberately
//! conservative: invalid input is returned unchanged with an explicit
//! disposition rather than receiving a partial edit.

use ling_source::SourceFile;
use ling_syntax::{TokenKind, parse};

use crate::FormatDocument;

/// The safe publication decision made for one formatting request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatDisposition {
    /// The complete source passed comment preservation and compiler
    /// revalidation, so the formatted candidate is published.
    Formatted,
    /// The source was incomplete or invalid; the exact original bytes were
    /// returned and no partial region was rewritten.
    OriginalInvalidSource,
    /// A valid-source candidate failed a preservation or compiler gate; the
    /// exact original bytes were returned instead of publishing partial output.
    OriginalRejectedCandidate,
}

/// Formatting text together with its conservative publication disposition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatResult {
    text: String,
    disposition: FormatDisposition,
}

impl FormatResult {
    /// Returns the formatted or preserved source text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Consumes the result and returns its source text.
    #[must_use]
    pub fn into_text(self) -> String {
        self.text
    }

    /// Returns the safe publication decision.
    #[must_use]
    pub const fn disposition(&self) -> FormatDisposition {
        self.disposition
    }
}

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
    format_core_with_disposition(document).into_text()
}

/// Formats valid Author Source or preserves incomplete/invalid source exactly.
///
/// The invalid-source branch is intentionally a no-op policy for this slice.
/// It provides a stable recovery boundary while FMT-1505 has no authority to
/// infer missing syntax or choose edits around error regions.  A caller can use
/// [`FormatResult::disposition`] to distinguish that safe fallback from a
/// published formatting result.
#[must_use]
pub fn format_core_with_disposition(document: &FormatDocument) -> FormatResult {
    if !document.is_valid() {
        return FormatResult {
            text: document.original_text().to_owned(),
            disposition: FormatDisposition::OriginalInvalidSource,
        };
    }
    let candidate = Renderer::new(document.had_bom()).render(document);
    if comments_are_preserved(document, &candidate)
        && parses_valid_source(document.source_id(), &candidate)
    {
        FormatResult {
            text: candidate,
            disposition: FormatDisposition::Formatted,
        }
    } else {
        FormatResult {
            text: document.original_text().to_owned(),
            disposition: FormatDisposition::OriginalRejectedCandidate,
        }
    }
}

fn comments_are_preserved(document: &FormatDocument, candidate: &str) -> bool {
    let mut cursor = 0;
    for attachment in document.comment_attachments() {
        for token in document.tokens()[attachment.comment_token_range()]
            .iter()
            .filter(|token| {
                matches!(
                    token.kind(),
                    TokenKind::LineComment | TokenKind::DocComment | TokenKind::BlockComment
                )
            })
        {
            let Some(relative) = candidate[cursor..].find(token.text()) else {
                return false;
            };
            cursor += relative + token.text().len();
        }
    }
    true
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
    use ling_ast::lower as lower_ast;
    use ling_effects::check as check_effects;
    use ling_hir::lower as lower_hir;
    use ling_resolve::resolve;
    use ling_semantic::build as build_snapshot;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;
    use ling_types::check as check_types;

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

    fn checked_snapshot(input: &str) -> ling_semantic::ProgramSnapshot {
        let source = SourceFile::from_bytes(
            SourceId::new(15106),
            "property.ling",
            input.as_bytes().to_vec(),
        )
        .expect("valid property source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:#?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = lower_hir(source.name(), &ast).expect("valid HIR");
        let resolved = resolve(vec![hir], "Main").expect("valid resolution");
        let typed = check_types(resolved).expect("valid types");
        let checked = check_effects(typed).expect("valid effects");
        build_snapshot(checked).expect("semantic snapshot builds")
    }

    fn semantic_snapshot(input: &str) -> String {
        checked_snapshot(input).json().to_owned()
    }

    fn audit_source(input: &str) -> String {
        let snapshot = checked_snapshot(input);
        crate::render_audit(&snapshot.audit_model()).expect("canonical Audit Source renders")
    }

    fn syntax_signature(input: &str) -> Vec<(TokenKind, String)> {
        let source = SourceFile::from_bytes(
            SourceId::new(15107),
            "signature.ling",
            input.as_bytes().to_vec(),
        )
        .expect("valid signature source");
        parse(&source)
            .tree()
            .tokens()
            .iter()
            .filter(|token| !token.kind().is_trivia() && !token.kind().is_layout())
            .filter(|token| token.kind() != TokenKind::Eof)
            .map(|token| {
                let span = token.span();
                (
                    token.kind(),
                    source.original_text()[span.start().get() as usize..span.end().get() as usize]
                        .to_owned(),
                )
            })
            .collect()
    }

    fn comment_signature(input: &str) -> Vec<String> {
        let source = SourceFile::from_bytes(
            SourceId::new(15108),
            "comments-property.ling",
            input.as_bytes().to_vec(),
        )
        .expect("valid comment source");
        let parsed = parse(&source);
        let document = build_format_ir(&source, &parsed).expect("Format IR builds");
        document
            .tokens()
            .iter()
            .filter(|token| {
                matches!(
                    token.kind(),
                    TokenKind::LineComment | TokenKind::DocComment | TokenKind::BlockComment
                )
            })
            .map(|token| token.text().to_owned())
            .collect()
    }

    fn property_corpus() -> [&'static str; 3] {
        [
            "module Main\n\nlet identity value=value\n\nlet main ()=identity ()\n",
            concat!(
                "module Main\r\n",
                "    requires Console.Write\r\n",
                "\r\n",
                "/// 输出\r\n",
                "let main ()=Console.write \"你好，零\"// 行尾\r\n",
            ),
            concat!(
                "module Main\n\n",
                "/* 外层\n",
                "   /* 内层 */\n",
                "*/\n",
                "let main ()=if true then () else ()\n",
            ),
        ]
    }

    #[test]
    fn formats_core_spacing_and_four_space_layout() {
        assert_eq!(
            format("let add a b=\n  a+b\nlet main ()=add 1 2\n"),
            "let add a b =\n    a + b\nlet main () = add 1 2\n"
        );
    }

    #[test]
    fn formats_checked_actor_declarations_without_changing_their_shape() {
        assert_eq!(
            format(concat!(
                "actor Counter:Int=\n",
                "  mailbox capacity 16 overflow Reject\n",
                "  state Int=0\n",
                "  receive state message=\n",
                "    state+message\n",
            )),
            concat!(
                "actor Counter: Int =\n",
                "    mailbox capacity 16 overflow Reject\n",
                "    state Int = 0\n",
                "    receive state message =\n",
                "        state + message\n",
            )
        );
    }

    #[test]
    fn property_corpus_is_idempotent_parse_equivalent_and_semantically_equivalent() {
        for original in property_corpus() {
            let formatted = format(original);
            assert_eq!(
                format(&formatted),
                formatted,
                "not idempotent: {original:?}"
            );
            assert_eq!(
                syntax_signature(original),
                syntax_signature(&formatted),
                "syntax signature changed: {original:?}"
            );
            assert_eq!(
                comment_signature(original),
                comment_signature(&formatted),
                "comment spelling/order changed: {original:?}"
            );
            assert_eq!(
                semantic_snapshot(original),
                semantic_snapshot(&formatted),
                "checked semantic snapshot changed: {original:?}"
            );
        }
    }

    #[test]
    fn author_formatting_does_not_replace_canonical_audit_source() {
        for original in property_corpus() {
            let formatted = format(original);
            assert_eq!(
                audit_source(original),
                audit_source(&formatted),
                "canonical Audit Source changed after formatting: {original:?}"
            );
        }
    }

    #[test]
    fn reports_a_published_disposition_for_a_valid_source() {
        let source =
            SourceFile::from_bytes(SourceId::new(84), "author.ling", b"let value=1\n".to_vec())
                .expect("valid test source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:#?}", parsed.parse_errors());
        let document = build_format_ir(&source, &parsed).expect("Format IR builds");
        let result = format_core_with_disposition(&document);
        assert_eq!(result.disposition(), FormatDisposition::Formatted);
        assert_eq!(result.text(), "let value = 1\n");
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
        let result = format_core_with_disposition(&document);
        assert_eq!(
            result.disposition(),
            FormatDisposition::OriginalInvalidSource
        );
        assert_eq!(result.text(), source.original_text());
    }

    #[test]
    fn never_partially_rewrites_a_valid_prefix_before_an_incomplete_region() {
        let original = concat!("let good=1\r\n", "let broken=\"unterminated\r\n");
        let source = SourceFile::from_bytes(
            SourceId::new(85),
            "author.ling",
            original.as_bytes().to_vec(),
        )
        .expect("valid UTF-8 source");
        let parsed = parse(&source);
        assert!(!parsed.is_valid());
        let document = build_format_ir(&source, &parsed).expect("invalid IR still builds");
        let result = format_core_with_disposition(&document);
        assert_eq!(
            result.disposition(),
            FormatDisposition::OriginalInvalidSource
        );
        assert_eq!(result.text(), original);
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
