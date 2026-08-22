//! Lossless lexical token observations for future semantic-token work.
//!
//! The index preserves the existing lexer taxonomy, exact original UTF-8
//! spans, and source spelling. It deliberately does not classify tokens into
//! an LSP semantic-token legend or produce an editor response.

use std::fmt;

use ling_source::{SourceId, Span};
use ling_syntax::{LexedSource, TokenKind};

/// A single lexer-owned token with its original source spelling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenSource {
    kind: TokenKind,
    span: Span,
    text: String,
}

impl TokenSource {
    #[must_use]
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// An immutable source-order lexical inventory for one compiler snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenSourceIndex {
    source: SourceId,
    source_name: String,
    valid: bool,
    tokens: Box<[TokenSource]>,
}

impl TokenSourceIndex {
    pub(crate) fn from_lexed(
        source: SourceId,
        source_name: String,
        original_text: &str,
        lexed: &LexedSource,
    ) -> Result<Self, TokenSourceIndexError> {
        let mut tokens = Vec::with_capacity(lexed.tokens().len());
        for token in lexed.tokens() {
            let span = token.span();
            let start = usize::try_from(span.start().get()).expect("span fits usize");
            let end = usize::try_from(span.end().get()).expect("span fits usize");
            let text = original_text
                .get(start..end)
                .ok_or(TokenSourceIndexError::SpanOutOfBounds { span })?;
            tokens.push(TokenSource {
                kind: token.kind(),
                span,
                text: text.to_owned(),
            });
        }
        Ok(Self {
            source,
            source_name,
            valid: lexed.errors().is_empty(),
            tokens: tokens.into_boxed_slice(),
        })
    }

    #[must_use]
    pub const fn source(&self) -> SourceId {
        self.source
    }

    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Returns whether lexing completed without a lexical error.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    #[must_use]
    pub fn tokens(&self) -> &[TokenSource] {
        &self.tokens
    }
}

/// Failure while projecting a lexer token into its original source bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenSourceIndexError {
    SpanOutOfBounds { span: Span },
}

impl fmt::Display for TokenSourceIndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SpanOutOfBounds { span } => write!(
                formatter,
                "lexer token span {}..{} is outside the original source",
                span.start().get(),
                span.end().get()
            ),
        }
    }
}

impl std::error::Error for TokenSourceIndexError {}
