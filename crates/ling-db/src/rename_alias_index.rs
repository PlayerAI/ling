use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use ling_effects::CheckedProgram;
use ling_hir::{
    Expression, ExpressionKind, LocalBinding, Name, Pattern, PatternKind, SequenceElement,
};
use ling_resolve::{ModuleId, ResolvedProgram};
use ling_source::{ByteOffset, Span};

use crate::MAX_REFERENCE_SEARCH_ENTRIES;

/// One exact import-alias declaration or checked qualified-use root.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenameAliasLocation {
    source_name: String,
    span: Span,
    declaration: bool,
}

impl RenameAliasLocation {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub const fn is_declaration(&self) -> bool {
        self.declaration
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct RenameAliasKey {
    module_id: u32,
    normalized: String,
    target_module_id: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenameAliasGroup {
    key: RenameAliasKey,
    locations: Box<[RenameAliasLocation]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenameAliasSelector {
    source_name: String,
    span: Span,
    group: usize,
}

/// One borrowed checked alias target and its complete owned occurrence set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenameAliasSelection<'index> {
    key: &'index RenameAliasKey,
    selected_source_name: &'index str,
    selected_span: Span,
    locations: &'index [RenameAliasLocation],
}

impl<'index> RenameAliasSelection<'index> {
    #[must_use]
    pub const fn module_id(self) -> u32 {
        self.key.module_id
    }

    #[must_use]
    pub fn normalized(self) -> &'index str {
        &self.key.normalized
    }

    #[must_use]
    pub const fn target_module_id(self) -> u32 {
        self.key.target_module_id
    }

    #[must_use]
    pub const fn selected_source_name(self) -> &'index str {
        self.selected_source_name
    }

    #[must_use]
    pub const fn selected_span(self) -> Span {
        self.selected_span
    }

    #[must_use]
    pub const fn locations(self) -> &'index [RenameAliasLocation] {
        self.locations
    }
}

/// Checked, deterministic import-alias declarations and qualified-use roots.
///
/// The index owns compiler source identity only. It has no URI, document
/// version, editor range, mutation, cache, persistence, or protocol state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedRenameAliasIndex {
    groups: Box<[RenameAliasGroup]>,
    selectors: Box<[RenameAliasSelector]>,
}

impl CheckedRenameAliasIndex {
    #[must_use]
    pub fn selection_at(
        &self,
        source_name: &str,
        offset: ByteOffset,
    ) -> Option<RenameAliasSelection<'_>> {
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
        Some(RenameAliasSelection {
            key: &group.key,
            selected_source_name: &selector.source_name,
            selected_span: selector.span,
            locations: &group.locations,
        })
    }

    pub(crate) fn from_checked(checked: &CheckedProgram) -> Result<Self, RenameAliasIndexError> {
        Self::from_resolved(checked.typed().resolved())
    }

    fn from_resolved(resolved: &ResolvedProgram) -> Result<Self, RenameAliasIndexError> {
        let mut grouped = BTreeMap::<RenameAliasKey, Vec<RenameAliasLocation>>::new();
        for module in resolved.modules() {
            let mut keys = BTreeMap::<String, RenameAliasKey>::new();
            for import in &module.hir.imports {
                let normalized = import.alias.normalized.clone();
                let target = module
                    .imports
                    .get(&normalized)
                    .ok_or(RenameAliasIndexError::MissingResolvedAlias)?;
                let key = RenameAliasKey {
                    module_id: module.id.get(),
                    normalized: normalized.clone(),
                    target_module_id: target.get(),
                };
                if keys.insert(normalized, key.clone()).is_some() {
                    return Err(RenameAliasIndexError::DuplicateAlias);
                }
                grouped.entry(key).or_default().push(RenameAliasLocation {
                    source_name: module.hir.source_name.clone(),
                    span: import.alias.span,
                    declaration: true,
                });
            }
            collect_program_alias_uses(resolved, module.id, &module.hir, &keys, &mut grouped)?;
        }

        let mut groups = Vec::with_capacity(grouped.len());
        for (key, mut locations) in grouped {
            locations.sort_by(location_order);
            if locations.len() > MAX_REFERENCE_SEARCH_ENTRIES {
                return Err(RenameAliasIndexError::TooManyLocations {
                    maximum: MAX_REFERENCE_SEARCH_ENTRIES,
                });
            }
            if locations.windows(2).any(|pair| {
                pair[0].source_name == pair[1].source_name && pair[0].span == pair[1].span
            }) {
                return Err(RenameAliasIndexError::DuplicateLocation);
            }
            if locations
                .iter()
                .filter(|location| location.declaration)
                .count()
                != 1
            {
                return Err(RenameAliasIndexError::InvalidDeclarationCount);
            }
            groups.push(RenameAliasGroup {
                key,
                locations: locations.into_boxed_slice(),
            });
        }
        groups.sort_by(|left, right| left.key.cmp(&right.key));

        let mut selectors = Vec::new();
        for (group, entry) in groups.iter().enumerate() {
            selectors.extend(entry.locations.iter().map(|location| RenameAliasSelector {
                source_name: location.source_name.clone(),
                span: location.span,
                group,
            }));
        }
        selectors.sort_by(selector_order);
        if selectors.len() > MAX_REFERENCE_SEARCH_ENTRIES {
            return Err(RenameAliasIndexError::TooManyEntries {
                maximum: MAX_REFERENCE_SEARCH_ENTRIES,
            });
        }
        Ok(Self {
            groups: groups.into_boxed_slice(),
            selectors: selectors.into_boxed_slice(),
        })
    }
}

/// Failure to build a complete checked import-alias occurrence index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RenameAliasIndexError {
    MissingResolvedAlias,
    DuplicateAlias,
    DuplicateLocation,
    InvalidDeclarationCount,
    TooManyLocations { maximum: usize },
    TooManyEntries { maximum: usize },
}

impl fmt::Display for RenameAliasIndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingResolvedAlias => formatter.write_str("import alias is unresolved"),
            Self::DuplicateAlias => formatter.write_str("import alias is duplicated"),
            Self::DuplicateLocation => formatter.write_str("import alias location is duplicated"),
            Self::InvalidDeclarationCount => {
                formatter.write_str("import alias declaration count is invalid")
            }
            Self::TooManyLocations { maximum } => {
                write!(formatter, "import alias exceeds {maximum} locations")
            }
            Self::TooManyEntries { maximum } => {
                write!(formatter, "import alias index exceeds {maximum} entries")
            }
        }
    }
}

impl Error for RenameAliasIndexError {}

fn collect_program_alias_uses(
    resolved: &ResolvedProgram,
    module: ModuleId,
    program: &ling_hir::Program,
    keys: &BTreeMap<String, RenameAliasKey>,
    grouped: &mut BTreeMap<RenameAliasKey, Vec<RenameAliasLocation>>,
) -> Result<(), RenameAliasIndexError> {
    for definition in &program.definitions {
        collect_patterns(resolved, module, &definition.parameters, keys, grouped)?;
        collect_expression(resolved, module, &definition.value, keys, grouped)?;
    }
    for task in &program.tasks {
        collect_patterns(resolved, module, &task.parameters, keys, grouped)?;
        collect_expression(resolved, module, &task.body, keys, grouped)?;
    }
    for implementation in &program.impls {
        for definition in &implementation.members {
            collect_patterns(resolved, module, &definition.parameters, keys, grouped)?;
            collect_expression(resolved, module, &definition.value, keys, grouped)?;
        }
    }
    Ok(())
}

fn collect_local(
    resolved: &ResolvedProgram,
    module: ModuleId,
    binding: &LocalBinding,
    keys: &BTreeMap<String, RenameAliasKey>,
    grouped: &mut BTreeMap<RenameAliasKey, Vec<RenameAliasLocation>>,
) -> Result<(), RenameAliasIndexError> {
    collect_patterns(resolved, module, &binding.parameters, keys, grouped)?;
    collect_expression(resolved, module, &binding.value, keys, grouped)
}

fn collect_patterns(
    resolved: &ResolvedProgram,
    module: ModuleId,
    patterns: &[Pattern],
    keys: &BTreeMap<String, RenameAliasKey>,
    grouped: &mut BTreeMap<RenameAliasKey, Vec<RenameAliasLocation>>,
) -> Result<(), RenameAliasIndexError> {
    for pattern in patterns {
        collect_pattern(resolved, module, pattern, keys, grouped)?;
    }
    Ok(())
}

fn collect_pattern(
    resolved: &ResolvedProgram,
    module: ModuleId,
    pattern: &Pattern,
    keys: &BTreeMap<String, RenameAliasKey>,
    grouped: &mut BTreeMap<RenameAliasKey, Vec<RenameAliasLocation>>,
) -> Result<(), RenameAliasIndexError> {
    match &pattern.kind {
        PatternKind::Tuple(elements) => {
            collect_patterns(resolved, module, elements, keys, grouped)?;
        }
        PatternKind::Record(fields) => {
            for field in fields {
                collect_pattern(resolved, module, &field.pattern, keys, grouped)?;
            }
        }
        PatternKind::Constructor {
            qualifier,
            arguments,
            ..
        } => {
            if resolved.pattern_constructor(module, pattern.id).is_some() {
                if let Some(qualifier) = qualifier {
                    push_use(
                        qualifier,
                        &resolved
                            .module(module)
                            .ok_or(RenameAliasIndexError::MissingResolvedAlias)?
                            .hir
                            .source_name,
                        keys,
                        grouped,
                    )?;
                }
            }
            collect_patterns(resolved, module, arguments, keys, grouped)?;
        }
        PatternKind::Binding { .. }
        | PatternKind::Wildcard
        | PatternKind::Unit
        | PatternKind::Literal(_) => {}
    }
    Ok(())
}

fn collect_expression(
    resolved: &ResolvedProgram,
    module: ModuleId,
    expression: &Expression,
    keys: &BTreeMap<String, RenameAliasKey>,
    grouped: &mut BTreeMap<RenameAliasKey, Vec<RenameAliasLocation>>,
) -> Result<(), RenameAliasIndexError> {
    let source_name = &resolved
        .module(module)
        .ok_or(RenameAliasIndexError::MissingResolvedAlias)?
        .hir
        .source_name;
    match &expression.kind {
        ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    SequenceElement::Let(binding) => {
                        collect_local(resolved, module, binding, keys, grouped)?;
                    }
                    SequenceElement::LetAwait(binding) => {
                        collect_pattern(resolved, module, &binding.pattern, keys, grouped)?;
                        collect_expression(resolved, module, &binding.call, keys, grouped)?;
                    }
                    SequenceElement::Expression(expression) => {
                        collect_expression(resolved, module, expression, keys, grouped)?;
                    }
                }
            }
        }
        ExpressionKind::TaskScope { body, .. } => {
            collect_expression(resolved, module, body, keys, grouped)?;
        }
        ExpressionKind::TaskSpawn { call, .. } => {
            collect_expression(resolved, module, call, keys, grouped)?;
        }
        ExpressionKind::TaskAwait { handle, .. } => {
            collect_expression(resolved, module, handle, keys, grouped)?;
        }
        ExpressionKind::TaskReturn { value, .. } => {
            collect_expression(resolved, module, value, keys, grouped)?;
        }
        ExpressionKind::Handle { body, clauses } => {
            collect_expression(resolved, module, body, keys, grouped)?;
            for clause in clauses {
                collect_patterns(resolved, module, &clause.parameters, keys, grouped)?;
                collect_expression(resolved, module, &clause.body, keys, grouped)?;
            }
        }
        ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_expression(resolved, module, condition, keys, grouped)?;
            collect_expression(resolved, module, then_branch, keys, grouped)?;
            collect_expression(resolved, module, else_branch, keys, grouped)?;
        }
        ExpressionKind::Match { scrutinee, cases } => {
            collect_expression(resolved, module, scrutinee, keys, grouped)?;
            for case in cases {
                collect_pattern(resolved, module, &case.pattern, keys, grouped)?;
                if let Some(guard) = &case.guard {
                    collect_expression(resolved, module, guard, keys, grouped)?;
                }
                collect_expression(resolved, module, &case.body, keys, grouped)?;
            }
        }
        ExpressionKind::Assignment { value, .. } => {
            collect_expression(resolved, module, value, keys, grouped)?;
        }
        ExpressionKind::Application {
            function,
            arguments,
        } => {
            collect_expression(resolved, module, function, keys, grouped)?;
            for argument in arguments {
                collect_expression(resolved, module, argument, keys, grouped)?;
            }
        }
        ExpressionKind::Projection {
            reference, target, ..
        } => {
            if resolved.reference(module, *reference).is_some() {
                if let Some(root) = qualified_root(expression) {
                    push_use(root, source_name, keys, grouped)?;
                }
            }
            collect_expression(resolved, module, target, keys, grouped)?;
        }
        ExpressionKind::Binary { left, right, .. } => {
            collect_expression(resolved, module, left, keys, grouped)?;
            collect_expression(resolved, module, right, keys, grouped)?;
        }
        ExpressionKind::Unary { operand, .. } => {
            collect_expression(resolved, module, operand, keys, grouped)?;
        }
        ExpressionKind::Tuple(elements) | ExpressionKind::List(elements) => {
            for element in elements {
                collect_expression(resolved, module, element, keys, grouped)?;
            }
        }
        ExpressionKind::Record(fields) => {
            for field in fields {
                collect_expression(resolved, module, &field.value, keys, grouped)?;
            }
        }
        ExpressionKind::RecordUpdate { base, fields } => {
            collect_expression(resolved, module, base, keys, grouped)?;
            for field in fields {
                collect_expression(resolved, module, &field.value, keys, grouped)?;
            }
        }
        ExpressionKind::Name { .. } | ExpressionKind::Literal(_) | ExpressionKind::Unit => {}
    }
    Ok(())
}

fn qualified_root(expression: &Expression) -> Option<&Name> {
    match &expression.kind {
        ExpressionKind::Name { name, .. } => Some(name),
        ExpressionKind::Projection { target, .. } => qualified_root(target),
        _ => None,
    }
}

fn push_use(
    name: &Name,
    source_name: &str,
    keys: &BTreeMap<String, RenameAliasKey>,
    grouped: &mut BTreeMap<RenameAliasKey, Vec<RenameAliasLocation>>,
) -> Result<(), RenameAliasIndexError> {
    let Some(key) = keys.get(&name.normalized) else {
        return Ok(());
    };
    grouped
        .get_mut(key)
        .ok_or(RenameAliasIndexError::MissingResolvedAlias)?
        .push(RenameAliasLocation {
            source_name: source_name.to_owned(),
            span: name.span,
            declaration: false,
        });
    Ok(())
}

fn span_width(span: Span) -> u32 {
    span.end().get().saturating_sub(span.start().get())
}

fn selector_order(left: &RenameAliasSelector, right: &RenameAliasSelector) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.span.source().cmp(&right.span.source()))
        .then_with(|| left.span.start().cmp(&right.span.start()))
        .then_with(|| left.span.end().cmp(&right.span.end()))
        .then_with(|| left.group.cmp(&right.group))
}

fn location_order(left: &RenameAliasLocation, right: &RenameAliasLocation) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.span.source().cmp(&right.span.source()))
        .then_with(|| left.span.start().cmp(&right.span.start()))
        .then_with(|| left.span.end().cmp(&right.span.end()))
        .then_with(|| left.declaration.cmp(&right.declaration).reverse())
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

    fn checked(sources: &[(&str, &str)]) -> CheckedProgram {
        let programs = sources
            .iter()
            .enumerate()
            .map(|(index, (name, text))| {
                let source = SourceFile::from_bytes(
                    SourceId::new(u32::try_from(index).expect("fixture count fits u32")),
                    *name,
                    text.as_bytes().to_vec(),
                )
                .expect("valid source");
                assert!(lex(&source).errors().is_empty());
                let parsed = parse(&source);
                assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
                let ast = lower(&source, &parsed).expect("valid AST");
                lower_hir(source.name().to_owned(), &ast).expect("valid HIR")
            })
            .collect::<Vec<_>>();
        let resolved = resolve(programs, "Main").expect("valid resolution");
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

    #[test]
    fn selects_resolved_alias_declaration_and_qualified_uses() {
        let main = concat!(
            "module Main\n\n",
            "import Support as S\n\n",
            "let main () = S.answer + S.answer\n",
        );
        let checked = checked(&[
            ("Main.ling", main),
            ("Support.ling", "module Support\n\nlet answer = 42\n"),
        ]);
        let index = CheckedRenameAliasIndex::from_checked(&checked).unwrap();

        let offsets = [
            ByteOffset::new(offset(main, "as S", 0).get() + 3),
            offset(main, "S.answer", 0),
            offset(main, "S.answer", 1),
        ];
        for offset in offsets {
            let selection = index
                .selection_at("Main.ling", offset)
                .expect("alias occurrence is selected");
            assert_eq!(selection.normalized(), "S");
            assert_eq!(selection.locations().len(), 3);
            assert_eq!(
                selection
                    .locations()
                    .iter()
                    .filter(|location| location.is_declaration())
                    .count(),
                1
            );
        }
        assert_eq!(
            index,
            CheckedRenameAliasIndex::from_checked(&checked).unwrap()
        );
    }

    #[test]
    fn unrelated_names_and_imported_members_are_not_alias_selectors() {
        let main = concat!(
            "module Main\n\n",
            "import Support as S\n\n",
            "let main S = S + S.answer\n",
        );
        let checked = checked(&[
            ("Main.ling", main),
            ("Support.ling", "module Support\n\nlet answer = 1\n"),
        ]);
        let index = CheckedRenameAliasIndex::from_checked(&checked).unwrap();
        assert!(
            index
                .selection_at("Main.ling", offset(main, "S =", 0))
                .is_none()
        );
        assert!(
            index
                .selection_at("Main.ling", offset(main, "answer", 0))
                .is_none()
        );
    }
}
