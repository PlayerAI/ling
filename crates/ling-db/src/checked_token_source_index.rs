//! Exact joins between lexical tokens and existing checked definition facts.
//!
//! This is a compiler-owned observation for future semantic-token design. It
//! does not classify non-definition tokens or project any editor presentation.

use std::fmt;

use crate::{TokenSource, TokenSourceIndex};

use super::TypedDefinitionIndex;

/// One lexical token with optional checked facts for an exact definition span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedTokenSource {
    token: TokenSource,
    definition_id: Option<String>,
    type_display: Option<String>,
    effects: Option<Box<[String]>>,
    capabilities: Option<Box<[String]>>,
}

impl CheckedTokenSource {
    #[must_use]
    pub const fn token(&self) -> &TokenSource {
        &self.token
    }

    #[must_use]
    pub fn definition_id(&self) -> Option<&str> {
        self.definition_id.as_deref()
    }

    #[must_use]
    pub fn type_display(&self) -> Option<&str> {
        self.type_display.as_deref()
    }

    #[must_use]
    pub fn effects(&self) -> Option<&[String]> {
        self.effects.as_deref()
    }

    #[must_use]
    pub fn capabilities(&self) -> Option<&[String]> {
        self.capabilities.as_deref()
    }
}

/// Source-order lexical tokens joined to exact checked definition spans.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedTokenSourceIndex {
    source_name: String,
    entries: Box<[CheckedTokenSource]>,
}

impl CheckedTokenSourceIndex {
    pub(crate) fn from_indexes(lexical: &TokenSourceIndex, typed: &TypedDefinitionIndex) -> Self {
        let entries = lexical
            .tokens()
            .iter()
            .map(|token| {
                let definition = typed.symbols().iter().find(|symbol| {
                    symbol.source_name() == lexical.source_name() && symbol.span() == token.span()
                });
                CheckedTokenSource {
                    token: token.clone(),
                    definition_id: definition.map(|symbol| symbol.definition_id().to_owned()),
                    type_display: definition
                        .and_then(|symbol| symbol.type_display().map(str::to_owned)),
                    effects: definition.map(|symbol| {
                        symbol
                            .effects()
                            .unwrap_or_default()
                            .to_vec()
                            .into_boxed_slice()
                    }),
                    capabilities: definition.map(|symbol| {
                        symbol
                            .capabilities()
                            .unwrap_or_default()
                            .to_vec()
                            .into_boxed_slice()
                    }),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self {
            source_name: lexical.source_name().to_owned(),
            entries,
        }
    }

    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub fn entries(&self) -> &[CheckedTokenSource] {
        &self.entries
    }
}

impl fmt::Display for CheckedTokenSourceIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "checked token source index for {} ({} entries)",
            self.source_name,
            self.entries.len()
        )
    }
}
