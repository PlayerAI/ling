use ling_source::{LexicalOffset, LexicalSpan, SourceFile, Span};
use ling_unicode::IdentifierSecurity;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TokenKind {
    Whitespace,
    LineComment,
    DocComment,
    BlockComment,
    Newline,
    SoftNewline,
    Indent,
    Dedent,
    Identifier,
    Integer,
    Float,
    Text,
    Let,
    Mutable,
    Rec,
    And,
    Type,
    Of,
    Match,
    With,
    When,
    If,
    Then,
    Else,
    True,
    False,
    Module,
    Import,
    As,
    Requires,
    Trait,
    Impl,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Apostrophe,
    Equals,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftArrow,
    RightArrow,
    Pipe,
    PipeGreater,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    AmpAmp,
    PipePipe,
    Error,
    Eof,
}

impl TokenKind {
    /// Stable spelling used by deterministic token-stream projections.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Whitespace => "whitespace",
            Self::LineComment => "line_comment",
            Self::DocComment => "doc_comment",
            Self::BlockComment => "block_comment",
            Self::Newline => "newline",
            Self::SoftNewline => "soft_newline",
            Self::Indent => "indent",
            Self::Dedent => "dedent",
            Self::Identifier => "identifier",
            Self::Integer => "integer",
            Self::Float => "float",
            Self::Text => "text",
            Self::Let => "let",
            Self::Mutable => "mutable",
            Self::Rec => "rec",
            Self::And => "and",
            Self::Type => "type",
            Self::Of => "of",
            Self::Match => "match",
            Self::With => "with",
            Self::When => "when",
            Self::If => "if",
            Self::Then => "then",
            Self::Else => "else",
            Self::True => "true",
            Self::False => "false",
            Self::Module => "module",
            Self::Import => "import",
            Self::As => "as",
            Self::Requires => "requires",
            Self::Trait => "trait",
            Self::Impl => "impl",
            Self::LeftParen => "left_paren",
            Self::RightParen => "right_paren",
            Self::LeftBracket => "left_bracket",
            Self::RightBracket => "right_bracket",
            Self::LeftBrace => "left_brace",
            Self::RightBrace => "right_brace",
            Self::Comma => "comma",
            Self::Semicolon => "semicolon",
            Self::Colon => "colon",
            Self::Dot => "dot",
            Self::Apostrophe => "apostrophe",
            Self::Equals => "equals",
            Self::EqualEqual => "equal_equal",
            Self::Bang => "bang",
            Self::BangEqual => "bang_equal",
            Self::Less => "less",
            Self::LessEqual => "less_equal",
            Self::Greater => "greater",
            Self::GreaterEqual => "greater_equal",
            Self::LeftArrow => "left_arrow",
            Self::RightArrow => "right_arrow",
            Self::Pipe => "pipe",
            Self::PipeGreater => "pipe_greater",
            Self::Plus => "plus",
            Self::Minus => "minus",
            Self::Star => "star",
            Self::Slash => "slash",
            Self::Percent => "percent",
            Self::AmpAmp => "amp_amp",
            Self::PipePipe => "pipe_pipe",
            Self::Error => "error",
            Self::Eof => "eof",
        }
    }

    #[must_use]
    pub const fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::Whitespace | Self::LineComment | Self::DocComment | Self::BlockComment
        )
    }

    #[must_use]
    pub const fn is_layout(self) -> bool {
        matches!(
            self,
            Self::Newline | Self::SoftNewline | Self::Indent | Self::Dedent
        )
    }

    pub(crate) const fn opens_delimiter(self) -> bool {
        matches!(self, Self::LeftParen | Self::LeftBracket | Self::LeftBrace)
    }

    pub(crate) const fn closes_delimiter(self) -> bool {
        matches!(
            self,
            Self::RightParen | Self::RightBracket | Self::RightBrace
        )
    }

    pub(crate) const fn matching_close(self) -> Option<Self> {
        match self {
            Self::LeftParen => Some(Self::RightParen),
            Self::LeftBracket => Some(Self::RightBracket),
            Self::LeftBrace => Some(Self::RightBrace),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerLiteral {
    radix: u32,
    digits: String,
}

impl IntegerLiteral {
    #[must_use]
    pub(crate) fn new(radix: u32, digits: String) -> Self {
        Self { radix, digits }
    }

    #[must_use]
    pub const fn radix(&self) -> u32 {
        self.radix
    }

    #[must_use]
    pub fn digits(&self) -> &str {
        &self.digits
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloatLiteral {
    normalized: String,
}

impl FloatLiteral {
    #[must_use]
    pub(crate) fn new(normalized: String) -> Self {
        Self { normalized }
    }

    #[must_use]
    pub fn normalized(&self) -> &str {
        &self.normalized
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenValue {
    Identifier(Box<IdentifierSecurity>),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Text(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub(crate) kind: TokenKind,
    lexical_span: LexicalSpan,
    span: Span,
    value: Option<TokenValue>,
}

impl Token {
    pub(crate) fn new(
        kind: TokenKind,
        source: &SourceFile,
        start: usize,
        end: usize,
        value: Option<TokenValue>,
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
        Self {
            kind,
            lexical_span,
            span,
            value,
        }
    }

    pub(crate) fn synthetic(kind: TokenKind, source: &SourceFile, offset: usize) -> Self {
        Self::new(kind, source, offset, offset, None)
    }

    #[must_use]
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    #[must_use]
    pub const fn lexical_span(&self) -> LexicalSpan {
        self.lexical_span
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub const fn value(&self) -> Option<&TokenValue> {
        self.value.as_ref()
    }
}
