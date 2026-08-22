use std::cmp::Ordering;

use ling_effects::CheckedProgram;
use ling_resolve::{DefinitionInfo, DefinitionOrigin, ResolvedProgram};
use ling_source::Span;

use crate::definition_index::{ResolvedDefinitionKind, classify_definition};

/// One user definition joined with exact checked type/effect observations.
///
/// Optional fields preserve missing checked facts instead of inventing a
/// hover placeholder or a presentation policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedDefinitionSymbol {
    definition_id: String,
    module_name: String,
    name: String,
    name_source: String,
    kind: ResolvedDefinitionKind,
    mutable: bool,
    source_name: String,
    span: Span,
    type_display: Option<String>,
    effects: Option<Box<[String]>>,
    capabilities: Option<Box<[String]>>,
}

impl TypedDefinitionSymbol {
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

/// Deterministic source-order observations for checked user definitions.
///
/// This is an in-process compiler observation. It contains no hover markup,
/// editor range, URI/version, cancellation, publication, or JSON-RPC state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedDefinitionIndex {
    symbols: Box<[TypedDefinitionSymbol]>,
}

impl TypedDefinitionIndex {
    #[must_use]
    pub fn symbols(&self) -> &[TypedDefinitionSymbol] {
        &self.symbols
    }

    #[must_use]
    pub fn definition(&self, definition_id: &str) -> Option<&TypedDefinitionSymbol> {
        self.symbols
            .iter()
            .find(|symbol| symbol.definition_id == definition_id)
    }

    #[must_use]
    pub fn source_symbols(&self, source_name: &str) -> Vec<&TypedDefinitionSymbol> {
        self.symbols
            .iter()
            .filter(|symbol| symbol.source_name == source_name)
            .collect()
    }

    pub(crate) fn from_checked(checked: &CheckedProgram) -> Self {
        let resolved = checked.typed().resolved();
        let mut symbols = resolved
            .definitions()
            .values()
            .filter_map(|definition| typed_symbol(checked, resolved, definition))
            .collect::<Vec<_>>();
        symbols.sort_by(symbol_order);
        debug_assert!(
            symbols
                .windows(2)
                .all(|pair| { symbol_order(&pair[0], &pair[1]) != Ordering::Greater })
        );
        Self {
            symbols: symbols.into_boxed_slice(),
        }
    }
}

fn typed_symbol(
    checked: &CheckedProgram,
    resolved: &ResolvedProgram,
    definition: &DefinitionInfo,
) -> Option<TypedDefinitionSymbol> {
    let DefinitionOrigin::User { module } = definition.origin else {
        return None;
    };
    let source_name = definition.source_name.clone()?;
    let span = definition.span?;
    let kind = classify_definition(resolved, definition)?;
    let type_display = checked
        .typed()
        .definition_type(&definition.id)
        .map(|type_id| checked.typed().display_type(type_id));
    let effects = checked
        .definition_effect(&definition.id)
        .map(|row| row.canonical_names().into_boxed_slice());
    let capabilities = checked.module_capabilities(module).map(|values| {
        values
            .iter()
            .map(|capability| capability.name().to_owned())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    });
    Some(TypedDefinitionSymbol {
        definition_id: definition.id.as_str().to_owned(),
        module_name: definition.module_name.clone(),
        name: definition.name.clone(),
        name_source: definition.name_source.clone(),
        kind,
        mutable: definition.mutable,
        source_name,
        span,
        type_display,
        effects,
        capabilities,
    })
}

fn symbol_order(left: &TypedDefinitionSymbol, right: &TypedDefinitionSymbol) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.span.source().cmp(&right.span.source()))
        .then_with(|| left.span.start().get().cmp(&right.span.start().get()))
        .then_with(|| left.span.end().get().cmp(&right.span.end().get()))
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.name.cmp(&right.name))
        .then_with(|| left.definition_id.cmp(&right.definition_id))
}
