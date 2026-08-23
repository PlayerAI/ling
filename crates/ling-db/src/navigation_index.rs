use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

use ling_effects::CheckedProgram;
use ling_resolve::{DefinitionId, ReferenceTarget, ResolvedProgram};
use ling_source::{ByteOffset, Span};
use ling_types::{Type, TypeId, TypedProgram};

use crate::definition_projection::{DefinitionProjectionError, definition_projection};
use crate::reference_span_index::ResolvedReferenceSpanIndex;

/// Maximum resolved references in one navigation index.
pub const MAX_NAVIGATION_ENTRIES: usize = 16_384;

/// One exact user-source navigation destination.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationLocation {
    source_name: String,
    span: Span,
}

impl NavigationLocation {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }
}

/// Navigation facts selected by one exact resolver reference span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationEntry {
    source_name: String,
    source_span: Span,
    definition: Option<NavigationLocation>,
    type_definition: Option<NavigationLocation>,
    identity_order: String,
}

impl NavigationEntry {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn source_span(&self) -> Span {
        self.source_span
    }

    #[must_use]
    pub const fn definition(&self) -> Option<&NavigationLocation> {
        self.definition.as_ref()
    }

    #[must_use]
    pub const fn type_definition(&self) -> Option<&NavigationLocation> {
        self.type_definition.as_ref()
    }
}

/// Deterministic resolver-backed navigation targets for one workspace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationIndex {
    entries: Box<[NavigationEntry]>,
}

impl NavigationIndex {
    #[must_use]
    pub fn entries(&self) -> &[NavigationEntry] {
        &self.entries
    }

    #[must_use]
    pub fn source_entry_at(
        &self,
        source_name: &str,
        offset: ByteOffset,
    ) -> Option<&NavigationEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.source_name == source_name
                    && entry.source_span.start() <= offset
                    && offset < entry.source_span.end()
            })
            .min_by(|left, right| {
                span_width(left.source_span)
                    .cmp(&span_width(right.source_span))
                    .then_with(|| entry_order(left, right))
            })
    }

    pub(crate) fn from_resolved(resolved: &ResolvedProgram) -> Result<Self, NavigationIndexError> {
        Self::build(resolved, None)
    }

    pub(crate) fn from_checked(checked: &CheckedProgram) -> Result<Self, NavigationIndexError> {
        Self::build(checked.typed().resolved(), Some(checked.typed()))
    }

    fn build(
        resolved: &ResolvedProgram,
        typed: Option<&TypedProgram>,
    ) -> Result<Self, NavigationIndexError> {
        let spans = ResolvedReferenceSpanIndex::from_resolved(resolved);
        let mut entries = Vec::with_capacity(resolved.references().len());
        for (key, target) in resolved.references() {
            let source = spans
                .reference(key.module().get(), key.local().get())
                .ok_or(NavigationIndexError::MissingReferenceSpan)?;
            let definition = target_location(resolved, target)?;
            let type_definition = if let Some(typed) = typed {
                target_type(typed, target)
                    .and_then(|type_id| nominal_result_definition(typed, type_id))
                    .map(|definition| definition_location(resolved, definition))
                    .transpose()?
                    .flatten()
            } else {
                None
            };
            entries.push(NavigationEntry {
                source_name: source.source_name().to_owned(),
                source_span: source.span(),
                definition,
                type_definition,
                identity_order: format!("{}:{}", key.module().get(), key.local().get()),
            });
        }
        Self::from_entries(entries)
    }

    fn from_entries(mut entries: Vec<NavigationEntry>) -> Result<Self, NavigationIndexError> {
        entries.sort_by(entry_order);
        if entries.len() > MAX_NAVIGATION_ENTRIES {
            return Err(NavigationIndexError::TooManyEntries {
                maximum: MAX_NAVIGATION_ENTRIES,
            });
        }
        Ok(Self {
            entries: entries.into_boxed_slice(),
        })
    }
}

/// Failure to construct a complete navigation observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NavigationIndexError {
    MissingReferenceSpan,
    MissingBinding,
    MissingModule,
    InvalidDefinition,
    InvalidLocation,
    TooManyEntries { maximum: usize },
}

impl fmt::Display for NavigationIndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingReferenceSpan => {
                formatter.write_str("navigation reference span is absent")
            }
            Self::MissingBinding => formatter.write_str("navigation binding is absent"),
            Self::MissingModule => formatter.write_str("navigation binding module is absent"),
            Self::InvalidDefinition => formatter.write_str("navigation definition is inconsistent"),
            Self::InvalidLocation => formatter.write_str("navigation location is incomplete"),
            Self::TooManyEntries { maximum } => {
                write!(formatter, "navigation index exceeds {maximum} entries")
            }
        }
    }
}

impl Error for NavigationIndexError {}

fn target_location(
    resolved: &ResolvedProgram,
    target: &ReferenceTarget,
) -> Result<Option<NavigationLocation>, NavigationIndexError> {
    match target {
        ReferenceTarget::Definition(definition) => definition_location(resolved, definition),
        ReferenceTarget::Binding(binding) => {
            let info = resolved
                .bindings()
                .get(binding)
                .ok_or(NavigationIndexError::MissingBinding)?;
            let module = resolved
                .module(binding.module())
                .ok_or(NavigationIndexError::MissingModule)?;
            Ok(Some(NavigationLocation {
                source_name: module.hir.source_name.clone(),
                span: info.span,
            }))
        }
    }
}

fn definition_location(
    resolved: &ResolvedProgram,
    definition: &DefinitionId,
) -> Result<Option<NavigationLocation>, NavigationIndexError> {
    let projection = definition_projection(resolved, definition).map_err(|error| match error {
        DefinitionProjectionError::MissingDefinition | DefinitionProjectionError::InvalidMember => {
            NavigationIndexError::InvalidDefinition
        }
    })?;
    match (projection.source_name, projection.name_span) {
        (Some(source_name), Some(span)) => Ok(Some(NavigationLocation { source_name, span })),
        (None, None) => Ok(None),
        _ => Err(NavigationIndexError::InvalidLocation),
    }
}

fn target_type(typed: &TypedProgram, target: &ReferenceTarget) -> Option<TypeId> {
    match target {
        ReferenceTarget::Definition(definition) => typed.definition_type(definition),
        ReferenceTarget::Binding(binding) => typed.binding_type(*binding),
    }
}

fn nominal_result_definition(typed: &TypedProgram, mut type_id: TypeId) -> Option<&DefinitionId> {
    for _ in 0..64 {
        match typed.arena().get(type_id) {
            Type::Function { result, .. } => type_id = *result,
            Type::NominalRecord { definition, .. } | Type::NominalVariant { definition, .. } => {
                return Some(definition);
            }
            Type::Unit
            | Type::Bool
            | Type::Int
            | Type::Float64
            | Type::Text
            | Type::Tuple(_)
            | Type::List(_)
            | Type::Variable(_)
            | Type::Error => return None,
        }
    }
    None
}

fn span_width(span: Span) -> u32 {
    span.end().get().saturating_sub(span.start().get())
}

fn entry_order(left: &NavigationEntry, right: &NavigationEntry) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.source_span.source().cmp(&right.source_span.source()))
        .then_with(|| left.source_span.start().cmp(&right.source_span.start()))
        .then_with(|| left.source_span.end().cmp(&right.source_span.end()))
        .then_with(|| left.identity_order.cmp(&right.identity_order))
}

#[cfg(test)]
mod tests {
    use ling_ast::lower;
    use ling_effects::check as check_effects;
    use ling_hir::lower as lower_hir;
    use ling_resolve::resolve;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::{lex, parse};
    use ling_types::check as check_types;

    use super::*;

    fn checked(source_text: &str) -> CheckedProgram {
        let source = SourceFile::from_bytes(
            SourceId::new(0),
            "Main.ling",
            source_text.as_bytes().to_vec(),
        )
        .expect("valid source");
        assert!(lex(&source).errors().is_empty());
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower(&source, &parsed).expect("valid AST");
        let hir = lower_hir(source.name().to_owned(), &ast).expect("valid HIR");
        let resolved = resolve(vec![hir], "Main").expect("valid resolution");
        check_effects(check_types(resolved).expect("valid types")).expect("valid effects")
    }

    fn offset(source: &str, needle: &str, occurrence: usize) -> ByteOffset {
        let byte = source
            .match_indices(needle)
            .nth(occurrence)
            .unwrap_or_else(|| panic!("missing occurrence {occurrence} of {needle}"))
            .0;
        ByteOffset::new(u32::try_from(byte).expect("fixture fits u32"))
    }

    fn span_text<'source>(source: &'source str, location: &NavigationLocation) -> &'source str {
        let start = location.span().start().get() as usize;
        let end = location.span().end().get() as usize;
        &source[start..end]
    }

    #[test]
    fn resolved_and_checked_targets_preserve_exact_definition_and_type_locations() {
        let source = concat!(
            "module Main\n\n",
            "type Item = { name: Text }\n",
            "type State =\n",
            "    | Idle\n",
            "    | Ready of Int\n\n",
            "let make name = { name = name }\n",
            "let main () =\n",
            "    let local = make \"Ling\"\n",
            "    let state = Ready 1\n",
            "    local\n",
        );
        let checked = checked(source);
        let resolved = NavigationIndex::from_resolved(checked.typed().resolved()).unwrap();
        let typed = NavigationIndex::from_checked(&checked).unwrap();

        let resolved_make = resolved
            .source_entry_at("Main.ling", offset(source, "make", 1))
            .expect("make reference");
        assert_eq!(
            span_text(source, resolved_make.definition().expect("make definition")),
            "make"
        );
        assert!(resolved_make.type_definition().is_none());

        let typed_make = typed
            .source_entry_at("Main.ling", offset(source, "make", 1))
            .expect("checked make reference");
        assert_eq!(
            span_text(source, typed_make.type_definition().expect("Item type")),
            "Item"
        );

        let constructor = typed
            .source_entry_at("Main.ling", offset(source, "Ready", 1))
            .expect("constructor reference");
        assert_eq!(
            span_text(source, constructor.definition().expect("Ready definition")),
            "Ready"
        );
        assert_eq!(
            span_text(source, constructor.type_definition().expect("State type")),
            "State"
        );

        let local = typed
            .source_entry_at("Main.ling", offset(source, "local", 1))
            .expect("local reference");
        assert_eq!(
            span_text(source, local.definition().expect("local binding")),
            "local"
        );
        assert_eq!(
            span_text(source, local.type_definition().expect("local Item type")),
            "Item"
        );
        assert_eq!(typed, NavigationIndex::from_checked(&checked).unwrap());
    }

    #[test]
    fn source_less_and_non_nominal_targets_remain_absent() {
        let source = concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () = Console.write \"Ling\"\n",
        );
        let checked = checked(source);
        let index = NavigationIndex::from_checked(&checked).unwrap();
        let builtin = index
            .source_entry_at("Main.ling", offset(source, "write", 0))
            .expect("builtin reference");
        assert!(builtin.definition().is_none());
        assert!(builtin.type_definition().is_none());
    }

    #[test]
    fn entry_bound_is_atomic() {
        let source = "module Main\n\nlet value = 1\nlet main = value\n";
        let checked = checked(source);
        let index = NavigationIndex::from_checked(&checked).unwrap();
        let entry = index.entries()[0].clone();
        assert_eq!(
            NavigationIndex::from_entries(vec![entry; MAX_NAVIGATION_ENTRIES + 1]),
            Err(NavigationIndexError::TooManyEntries {
                maximum: MAX_NAVIGATION_ENTRIES,
            })
        );
    }
}
