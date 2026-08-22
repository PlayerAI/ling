use std::cmp::Ordering;

use ling_resolve::{DefinitionInfo, DefinitionKind, DefinitionOrigin, ResolvedProgram};
use ling_source::Span;

/// The resolver classification retained by the internal definition index.
///
/// Trait and implementation members are separated from ordinary values using
/// the resolver's existing member tables. No editor or wire-level symbol
/// taxonomy is inferred here.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResolvedDefinitionKind {
    Value,
    Type,
    Constructor,
    TraitMember,
    ImplementationMember,
}

impl ResolvedDefinitionKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::Type => "type",
            Self::Constructor => "constructor",
            Self::TraitMember => "trait_member",
            Self::ImplementationMember => "implementation_member",
        }
    }
}

/// One user definition copied from the validated resolver result.
///
/// The span is the resolver-owned original UTF-8 span. The index does not
/// reinterpret it as an editor range or synthesize a selection span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedDefinitionSymbol {
    definition_id: String,
    module_name: String,
    name: String,
    name_source: String,
    kind: ResolvedDefinitionKind,
    mutable: bool,
    source_name: String,
    span: Span,
}

impl ResolvedDefinitionSymbol {
    #[must_use]
    pub fn definition_id(&self) -> &str {
        &self.definition_id
    }

    #[must_use]
    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn name_source(&self) -> &str {
        &self.name_source
    }

    #[must_use]
    pub const fn kind(&self) -> ResolvedDefinitionKind {
        self.kind
    }

    #[must_use]
    pub const fn is_mutable(&self) -> bool {
        self.mutable
    }

    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }
}

/// Deterministic source-order inventory of user definitions.
///
/// This is an in-process compiler observation. It excludes builtins and
/// Prelude entries and contains no URI, document version, position encoding,
/// hierarchy, cancellation, or JSON-RPC state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedDefinitionIndex {
    symbols: Box<[ResolvedDefinitionSymbol]>,
}

impl ResolvedDefinitionIndex {
    #[must_use]
    pub fn symbols(&self) -> &[ResolvedDefinitionSymbol] {
        &self.symbols
    }

    #[must_use]
    pub fn definition(&self, definition_id: &str) -> Option<&ResolvedDefinitionSymbol> {
        self.symbols
            .iter()
            .find(|symbol| symbol.definition_id == definition_id)
    }

    #[must_use]
    pub fn source_symbols(&self, source_name: &str) -> Vec<&ResolvedDefinitionSymbol> {
        self.symbols
            .iter()
            .filter(|symbol| symbol.source_name == source_name)
            .collect()
    }

    pub(crate) fn from_resolved(resolved: &ResolvedProgram) -> Self {
        let mut symbols = resolved
            .definitions()
            .values()
            .filter_map(|definition| {
                if !matches!(definition.origin, DefinitionOrigin::User { .. }) {
                    return None;
                }
                let source_name = definition.source_name.clone()?;
                let span = definition.span?;
                let kind = classify_definition(resolved, definition)?;
                Some(ResolvedDefinitionSymbol {
                    definition_id: definition.id.as_str().to_owned(),
                    module_name: definition.module_name.clone(),
                    name: definition.name.clone(),
                    name_source: definition.name_source.clone(),
                    kind,
                    mutable: definition.mutable,
                    source_name,
                    span,
                })
            })
            .collect::<Vec<_>>();

        symbols.sort_by(|left, right| {
            left.source_name
                .cmp(&right.source_name)
                .then_with(|| left.span.source().cmp(&right.span.source()))
                .then_with(|| left.span.start().get().cmp(&right.span.start().get()))
                .then_with(|| left.span.end().get().cmp(&right.span.end().get()))
                .then_with(|| left.kind.cmp(&right.kind))
                .then_with(|| left.name.cmp(&right.name))
                .then_with(|| left.definition_id.cmp(&right.definition_id))
        });

        debug_assert!(symbols.windows(2).all(|pair| {
            let left = &pair[0];
            let right = &pair[1];
            symbol_order(left, right) != Ordering::Greater
        }));

        Self {
            symbols: symbols.into_boxed_slice(),
        }
    }
}

pub(crate) fn classify_definition(
    resolved: &ResolvedProgram,
    definition: &DefinitionInfo,
) -> Option<ResolvedDefinitionKind> {
    if resolved.trait_member(&definition.id).is_some() {
        Some(ResolvedDefinitionKind::TraitMember)
    } else if resolved.impl_member(&definition.id).is_some() {
        Some(ResolvedDefinitionKind::ImplementationMember)
    } else {
        match definition.kind {
            DefinitionKind::Value => Some(ResolvedDefinitionKind::Value),
            DefinitionKind::Type => Some(ResolvedDefinitionKind::Type),
            DefinitionKind::Constructor => Some(ResolvedDefinitionKind::Constructor),
            DefinitionKind::Builtin => None,
        }
    }
}

fn symbol_order(left: &ResolvedDefinitionSymbol, right: &ResolvedDefinitionSymbol) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.span.source().cmp(&right.span.source()))
        .then_with(|| left.span.start().get().cmp(&right.span.start().get()))
        .then_with(|| left.span.end().get().cmp(&right.span.end().get()))
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.name.cmp(&right.name))
        .then_with(|| left.definition_id.cmp(&right.definition_id))
}
