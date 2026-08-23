use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use ling_effects::CheckedProgram;
use ling_resolve::{DefinitionId, ResolvedProgram};
use ling_source::{ByteOffset, Span};

use crate::definition_projection::{DefinitionProjectionError, definition_projection};
use crate::reference_index::{ResolvedReferenceIndex, ResolvedReferenceTargetKey};
use crate::reference_span_index::{ResolvedReferenceRelation, ResolvedReferenceSpanIndex};

/// Maximum selectable declarations and references in one checked observation.
pub const MAX_REFERENCE_SEARCH_ENTRIES: usize = 16_384;

/// One exact declaration or resolver-reference location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceSearchLocation {
    source_name: String,
    span: Span,
    relation: Option<ResolvedReferenceRelation>,
}

impl ReferenceSearchLocation {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub const fn relation(&self) -> Option<ResolvedReferenceRelation> {
        self.relation
    }

    #[must_use]
    pub const fn is_declaration(&self) -> bool {
        self.relation.is_none()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReferenceSearchGroup {
    target: ResolvedReferenceTargetKey,
    declaration: Option<ReferenceSearchLocation>,
    references: Box<[ReferenceSearchLocation]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReferenceSearchSelector {
    source_name: String,
    span: Span,
    group: usize,
}

/// Checked, deterministic target-to-source locations for LSP references.
///
/// The value owns no URI, document version, cache, publication, or wire state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceSearchIndex {
    groups: Box<[ReferenceSearchGroup]>,
    selectors: Box<[ReferenceSearchSelector]>,
}

impl ReferenceSearchIndex {
    /// Selects either a declaration identifier or resolver reference and
    /// returns the target's canonical source-ordered locations.
    #[must_use]
    pub fn locations_at(
        &self,
        source_name: &str,
        offset: ByteOffset,
        include_declaration: bool,
    ) -> Option<Vec<&ReferenceSearchLocation>> {
        let selector = self
            .selectors
            .iter()
            .filter(|selector| {
                selector.source_name == source_name
                    && selector.span.start() <= offset
                    && offset < selector.span.end()
            })
            .min_by(|left, right| {
                span_width(left.span)
                    .cmp(&span_width(right.span))
                    .then_with(|| selector_order(left, right))
            })?;
        let group = &self.groups[selector.group];
        let mut locations = Vec::with_capacity(
            group.references.len()
                + usize::from(include_declaration && group.declaration.is_some()),
        );
        if include_declaration {
            locations.extend(group.declaration.iter());
        }
        locations.extend(group.references.iter());
        locations.sort_by(|left, right| location_order(left, right));
        Some(locations)
    }

    pub(crate) fn from_checked(
        checked: &CheckedProgram,
    ) -> Result<Self, ReferenceSearchIndexError> {
        Self::build(checked.typed().resolved())
    }

    fn build(resolved: &ResolvedProgram) -> Result<Self, ReferenceSearchIndexError> {
        let forward = ResolvedReferenceIndex::from_resolved(resolved);
        let spans = ResolvedReferenceSpanIndex::from_resolved(resolved);
        if forward.entries().len() != spans.entries().len() {
            return Err(ReferenceSearchIndexError::IncompleteReferenceJoin);
        }
        let targets = forward
            .entries()
            .iter()
            .map(|entry| {
                (
                    (entry.source_module_id(), entry.reference_id()),
                    entry.target().key(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        if targets.len() != forward.entries().len() {
            return Err(ReferenceSearchIndexError::IncompleteReferenceJoin);
        }

        let mut grouped =
            BTreeMap::<ResolvedReferenceTargetKey, Vec<ReferenceSearchLocation>>::new();
        for span in spans.entries() {
            let target = targets
                .get(&(span.module_id(), span.reference_id()))
                .ok_or(ReferenceSearchIndexError::IncompleteReferenceJoin)?;
            grouped
                .entry(target.clone())
                .or_default()
                .push(ReferenceSearchLocation {
                    source_name: span.source_name().to_owned(),
                    span: span.span(),
                    relation: Some(span.relation()),
                });
        }

        let mut groups = Vec::with_capacity(grouped.len());
        for (target, mut references) in grouped {
            references.sort_by(location_order);
            if references.windows(2).any(|pair| {
                pair[0].source_name == pair[1].source_name && pair[0].span == pair[1].span
            }) {
                return Err(ReferenceSearchIndexError::DuplicateLocation);
            }
            let declaration = declaration_location(resolved, &target)?;
            groups.push(ReferenceSearchGroup {
                target,
                declaration,
                references: references.into_boxed_slice(),
            });
        }
        Self::from_groups(groups)
    }

    fn from_groups(
        mut groups: Vec<ReferenceSearchGroup>,
    ) -> Result<Self, ReferenceSearchIndexError> {
        if groups.iter().any(|group| {
            group.references.len() + usize::from(group.declaration.is_some())
                > MAX_REFERENCE_SEARCH_ENTRIES
        }) {
            return Err(ReferenceSearchIndexError::TooManyLocations {
                maximum: MAX_REFERENCE_SEARCH_ENTRIES,
            });
        }
        if groups.iter().any(|group| {
            group.declaration.as_ref().is_some_and(|declaration| {
                group.references.iter().any(|reference| {
                    reference.source_name == declaration.source_name
                        && reference.span == declaration.span
                })
            })
        }) {
            return Err(ReferenceSearchIndexError::DuplicateLocation);
        }
        groups.sort_by(|left, right| left.target.cmp(&right.target));

        let mut selectors = Vec::new();
        for (group, entry) in groups.iter().enumerate() {
            if let Some(declaration) = &entry.declaration {
                selectors.push(ReferenceSearchSelector {
                    source_name: declaration.source_name.clone(),
                    span: declaration.span,
                    group,
                });
            }
            selectors.extend(
                entry
                    .references
                    .iter()
                    .map(|reference| ReferenceSearchSelector {
                        source_name: reference.source_name.clone(),
                        span: reference.span,
                        group,
                    }),
            );
        }
        selectors.sort_by(selector_order);
        if selectors.len() > MAX_REFERENCE_SEARCH_ENTRIES {
            return Err(ReferenceSearchIndexError::TooManyEntries {
                maximum: MAX_REFERENCE_SEARCH_ENTRIES,
            });
        }
        Ok(Self {
            groups: groups.into_boxed_slice(),
            selectors: selectors.into_boxed_slice(),
        })
    }
}

/// Failure to build a complete checked references observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceSearchIndexError {
    IncompleteReferenceJoin,
    MissingDefinition,
    InvalidDefinition,
    InvalidLocation,
    MissingBinding,
    MissingModule,
    DuplicateLocation,
    TooManyLocations { maximum: usize },
    TooManyEntries { maximum: usize },
}

impl fmt::Display for ReferenceSearchIndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompleteReferenceJoin => formatter.write_str("reference join is incomplete"),
            Self::MissingDefinition => formatter.write_str("reference definition is absent"),
            Self::InvalidDefinition => formatter.write_str("reference definition is inconsistent"),
            Self::InvalidLocation => formatter.write_str("reference location is incomplete"),
            Self::MissingBinding => formatter.write_str("reference binding is absent"),
            Self::MissingModule => formatter.write_str("reference binding module is absent"),
            Self::DuplicateLocation => formatter.write_str("reference location is duplicated"),
            Self::TooManyLocations { maximum } => {
                write!(formatter, "reference result exceeds {maximum} locations")
            }
            Self::TooManyEntries { maximum } => {
                write!(formatter, "reference index exceeds {maximum} entries")
            }
        }
    }
}

impl Error for ReferenceSearchIndexError {}

fn declaration_location(
    resolved: &ResolvedProgram,
    target: &ResolvedReferenceTargetKey,
) -> Result<Option<ReferenceSearchLocation>, ReferenceSearchIndexError> {
    match target {
        ResolvedReferenceTargetKey::Definition(raw) => {
            let definition = resolved
                .definitions()
                .keys()
                .find(|definition| definition.as_str() == raw)
                .ok_or(ReferenceSearchIndexError::MissingDefinition)?;
            definition_location(resolved, definition)
        }
        ResolvedReferenceTargetKey::Binding {
            module_id,
            binding_id,
        } => {
            let (key, binding) = resolved
                .bindings()
                .iter()
                .find(|(key, _)| {
                    key.module().get() == *module_id && key.local().get() == *binding_id
                })
                .ok_or(ReferenceSearchIndexError::MissingBinding)?;
            let module = resolved
                .module(key.module())
                .ok_or(ReferenceSearchIndexError::MissingModule)?;
            Ok(Some(ReferenceSearchLocation {
                source_name: module.hir.source_name.clone(),
                span: binding.span,
                relation: None,
            }))
        }
    }
}

fn definition_location(
    resolved: &ResolvedProgram,
    definition: &DefinitionId,
) -> Result<Option<ReferenceSearchLocation>, ReferenceSearchIndexError> {
    let projection = definition_projection(resolved, definition).map_err(|error| match error {
        DefinitionProjectionError::MissingDefinition => {
            ReferenceSearchIndexError::MissingDefinition
        }
        DefinitionProjectionError::InvalidMember => ReferenceSearchIndexError::InvalidDefinition,
    })?;
    match (projection.source_name, projection.name_span) {
        (Some(source_name), Some(span)) => Ok(Some(ReferenceSearchLocation {
            source_name,
            span,
            relation: None,
        })),
        (None, None) => Ok(None),
        _ => Err(ReferenceSearchIndexError::InvalidLocation),
    }
}

fn span_width(span: Span) -> u32 {
    span.end().get().saturating_sub(span.start().get())
}

fn selector_order(left: &ReferenceSearchSelector, right: &ReferenceSearchSelector) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.span.source().cmp(&right.span.source()))
        .then_with(|| left.span.start().cmp(&right.span.start()))
        .then_with(|| left.span.end().cmp(&right.span.end()))
        .then_with(|| left.group.cmp(&right.group))
}

fn location_order(left: &ReferenceSearchLocation, right: &ReferenceSearchLocation) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.span.source().cmp(&right.span.source()))
        .then_with(|| left.span.start().cmp(&right.span.start()))
        .then_with(|| left.span.end().cmp(&right.span.end()))
        .then_with(|| left.relation.cmp(&right.relation))
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
        ByteOffset::new(
            u32::try_from(
                source
                    .match_indices(needle)
                    .nth(occurrence)
                    .unwrap_or_else(|| panic!("missing occurrence {occurrence} of {needle}"))
                    .0,
            )
            .expect("fixture fits u32"),
        )
    }

    fn text<'source>(source: &'source str, location: &ReferenceSearchLocation) -> &'source str {
        &source[location.span.start().get() as usize..location.span.end().get() as usize]
    }

    #[test]
    fn selects_declarations_and_references_with_exact_relations() {
        let source = concat!(
            "module Main\n\n",
            "let helper value = value\n",
            "let main () =\n",
            "    let mutable count = 0\n",
            "    count <- helper count\n",
            "    count\n",
        );
        let index = ReferenceSearchIndex::from_checked(&checked(source)).unwrap();
        let helper = index
            .locations_at("Main.ling", offset(source, "helper", 0), true)
            .expect("helper declaration selectable");
        assert_eq!(helper.len(), 2);
        assert!(helper[0].is_declaration());
        assert_eq!(helper[1].relation(), Some(ResolvedReferenceRelation::Call));
        assert!(
            helper
                .iter()
                .all(|location| text(source, location) == "helper")
        );

        let count = index
            .locations_at("Main.ling", offset(source, "count", 2), true)
            .expect("count reference selectable");
        assert_eq!(
            count
                .iter()
                .filter_map(|location| location.relation())
                .collect::<Vec<_>>(),
            vec![
                ResolvedReferenceRelation::Write,
                ResolvedReferenceRelation::Read,
                ResolvedReferenceRelation::Read,
            ]
        );
        assert_eq!(
            index,
            ReferenceSearchIndex::from_checked(&checked(source)).unwrap()
        );
    }

    #[test]
    fn include_declaration_and_source_less_targets_are_exact() {
        let source = concat!(
            "module Main\n    requires Console.Write\n\n",
            "let helper = 1\n",
            "let main () =\n",
            "    let value = helper + helper\n",
            "    Console.write \"Ling\"\n",
        );
        let index = ReferenceSearchIndex::from_checked(&checked(source)).unwrap();
        let helper = index
            .locations_at("Main.ling", offset(source, "helper", 1), false)
            .expect("helper reference");
        assert_eq!(helper.len(), 2);
        assert!(helper.iter().all(|location| !location.is_declaration()));

        let builtin = index
            .locations_at("Main.ling", offset(source, "write", 0), true)
            .expect("builtin reference");
        assert_eq!(builtin.len(), 1);
        assert_eq!(builtin[0].relation(), Some(ResolvedReferenceRelation::Call));
    }

    #[test]
    fn location_and_selector_bounds_fail_atomically() {
        let source = "module Main\n\nlet value = 1\nlet main = value\n";
        let index = ReferenceSearchIndex::from_checked(&checked(source)).unwrap();
        let group = index.groups[0].clone();
        let location = group.references[0].clone();
        let oversized_group = ReferenceSearchGroup {
            target: group.target.clone(),
            declaration: group.declaration.clone(),
            references: vec![location; MAX_REFERENCE_SEARCH_ENTRIES].into_boxed_slice(),
        };
        assert_eq!(
            ReferenceSearchIndex::from_groups(vec![oversized_group]),
            Err(ReferenceSearchIndexError::TooManyLocations {
                maximum: MAX_REFERENCE_SEARCH_ENTRIES,
            })
        );

        let groups = (0..(MAX_REFERENCE_SEARCH_ENTRIES / 2 + 1))
            .map(|ordinal| ReferenceSearchGroup {
                target: ResolvedReferenceTargetKey::Definition(format!("definition-{ordinal}")),
                declaration: group.declaration.clone(),
                references: group.references.clone(),
            })
            .collect();
        assert_eq!(
            ReferenceSearchIndex::from_groups(groups),
            Err(ReferenceSearchIndexError::TooManyEntries {
                maximum: MAX_REFERENCE_SEARCH_ENTRIES,
            })
        );
    }
}
