use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use ling_effects::CheckedProgram;
use ling_hir::{Expression, ExpressionKind, Program, SequenceElement};
use ling_resolve::{
    DefinitionId, DefinitionKind, DefinitionOrigin, ExpressionKey, ModuleId, ReferenceKey,
    ReferenceTarget,
};
use ling_source::{ByteOffset, Span};

use crate::definition_projection::{DefinitionProjectionError, definition_projection};
use crate::reference_span_index::ResolvedReferenceSpanIndex;

/// Maximum declarations, bindings, and references in one checked hover index.
pub const MAX_CHECKED_HOVER_ENTRIES: usize = 16_384;

/// Compiler-owned classification for one hoverable checked target.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CheckedHoverKind {
    Value,
    Type,
    Constructor,
    Builtin,
    TraitMember,
    ImplementationMember,
    Binding,
    Parameter,
}

/// Exact checked Trait selection attached to one member-call reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedHoverTraitSelection {
    trait_name: String,
    receiver: String,
    member: String,
}

impl CheckedHoverTraitSelection {
    #[must_use]
    pub fn trait_name(&self) -> &str {
        &self.trait_name
    }

    #[must_use]
    pub fn receiver(&self) -> &str {
        &self.receiver
    }

    #[must_use]
    pub fn member(&self) -> &str {
        &self.member
    }
}

/// One hoverable compiler fact at an exact original UTF-8 identifier span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedHoverEntry {
    source_name: String,
    span: Span,
    name: String,
    kind: CheckedHoverKind,
    mutable: bool,
    type_display: Option<String>,
    effects: Box<[String]>,
    capabilities: Box<[String]>,
    trait_selection: Option<CheckedHoverTraitSelection>,
    identity_order: String,
}

impl CheckedHoverEntry {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn kind(&self) -> CheckedHoverKind {
        self.kind
    }

    #[must_use]
    pub const fn is_mutable(&self) -> bool {
        self.mutable
    }

    #[must_use]
    pub fn type_display(&self) -> Option<&str> {
        self.type_display.as_deref()
    }

    #[must_use]
    pub fn effects(&self) -> &[String] {
        &self.effects
    }

    #[must_use]
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    #[must_use]
    pub const fn trait_selection(&self) -> Option<&CheckedHoverTraitSelection> {
        self.trait_selection.as_ref()
    }
}

/// Deterministic checked hover targets for one complete resolved workspace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedHoverIndex {
    entries: Box<[CheckedHoverEntry]>,
}

impl CheckedHoverIndex {
    #[must_use]
    pub fn entries(&self) -> &[CheckedHoverEntry] {
        &self.entries
    }

    /// Returns the unique smallest identifier span containing `offset`.
    #[must_use]
    pub fn source_entry_at(
        &self,
        source_name: &str,
        offset: ByteOffset,
    ) -> Option<&CheckedHoverEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.source_name == source_name
                    && entry.span.start() <= offset
                    && offset < entry.span.end()
            })
            .min_by(|left, right| {
                span_width(left.span)
                    .cmp(&span_width(right.span))
                    .then_with(|| entry_order(left, right))
            })
    }

    pub(crate) fn from_checked(checked: &CheckedProgram) -> Result<Self, CheckedHoverIndexError> {
        let resolved = checked.typed().resolved();
        let mut entries = Vec::new();

        for definition in resolved.definitions().values() {
            let (Some(source_name), Some(span)) =
                (definition.source_name.as_deref(), definition.span)
            else {
                continue;
            };
            if matches!(definition.origin, DefinitionOrigin::User { .. }) {
                entries.push(definition_entry(
                    checked,
                    definition.id.clone(),
                    source_name,
                    span,
                    true,
                    None,
                )?);
            }
        }

        for (key, binding) in resolved.bindings() {
            let module = resolved
                .module(key.module())
                .ok_or(CheckedHoverIndexError::MissingModule)?;
            entries.push(CheckedHoverEntry {
                source_name: module.hir.source_name.clone(),
                span: binding.span,
                name: binding.name.clone(),
                kind: if binding.parameter {
                    CheckedHoverKind::Parameter
                } else {
                    CheckedHoverKind::Binding
                },
                mutable: binding.mutable,
                type_display: checked
                    .typed()
                    .binding_type(*key)
                    .map(|type_id| canonical_type_display(&checked.typed().display_type(type_id))),
                effects: checked
                    .binding_effect(*key)
                    .map_or_else(Box::default, |row| row.canonical_names().into_boxed_slice()),
                capabilities: module_capabilities(checked, key.module()),
                trait_selection: None,
                identity_order: format!("binding:{}:{}", key.module().get(), key.local().get()),
            });
        }

        let spans = ResolvedReferenceSpanIndex::from_resolved(resolved);
        let selections = trait_selections(checked)?;
        for (key, target) in resolved.references() {
            let Some(reference) = spans.reference(key.module().get(), key.local().get()) else {
                return Err(CheckedHoverIndexError::MissingReferenceSpan);
            };
            let selection = selections.get(key).cloned();
            let entry = match target {
                ReferenceTarget::Definition(definition) => definition_entry(
                    checked,
                    definition.clone(),
                    reference.source_name(),
                    reference.span(),
                    false,
                    selection,
                )?,
                ReferenceTarget::Binding(binding) => {
                    let info = resolved
                        .bindings()
                        .get(binding)
                        .ok_or(CheckedHoverIndexError::MissingBinding)?;
                    CheckedHoverEntry {
                        source_name: reference.source_name().to_owned(),
                        span: reference.span(),
                        name: info.name.clone(),
                        kind: if info.parameter {
                            CheckedHoverKind::Parameter
                        } else {
                            CheckedHoverKind::Binding
                        },
                        mutable: info.mutable,
                        type_display: checked.typed().binding_type(*binding).map(|type_id| {
                            canonical_type_display(&checked.typed().display_type(type_id))
                        }),
                        effects: checked
                            .binding_effect(*binding)
                            .map_or_else(Box::default, |row| {
                                row.canonical_names().into_boxed_slice()
                            }),
                        capabilities: module_capabilities(checked, binding.module()),
                        trait_selection: selection,
                        identity_order: format!(
                            "binding:{}:{}",
                            binding.module().get(),
                            binding.local().get()
                        ),
                    }
                }
            };
            entries.push(entry);
        }

        entries.sort_by(entry_order);
        if entries.len() > MAX_CHECKED_HOVER_ENTRIES {
            return Err(CheckedHoverIndexError::TooManyEntries {
                maximum: MAX_CHECKED_HOVER_ENTRIES,
            });
        }
        Ok(Self {
            entries: entries.into_boxed_slice(),
        })
    }
}

/// Failure to construct complete checked hover observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckedHoverIndexError {
    MissingModule,
    MissingBinding,
    MissingDefinition,
    InvalidDefinitionSpan,
    MissingReferenceSpan,
    InvalidTraitSelection,
    TooManyEntries { maximum: usize },
}

impl fmt::Display for CheckedHoverIndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingModule => formatter.write_str("checked hover module is absent"),
            Self::MissingBinding => formatter.write_str("checked hover binding is absent"),
            Self::MissingDefinition => formatter.write_str("checked hover definition is absent"),
            Self::InvalidDefinitionSpan => {
                formatter.write_str("checked hover definition name span is inconsistent")
            }
            Self::MissingReferenceSpan => {
                formatter.write_str("checked hover reference span is absent")
            }
            Self::InvalidTraitSelection => {
                formatter.write_str("checked hover Trait selection is inconsistent")
            }
            Self::TooManyEntries { maximum } => {
                write!(formatter, "checked hover index exceeds {maximum} entries")
            }
        }
    }
}

impl Error for CheckedHoverIndexError {}

fn definition_entry(
    checked: &CheckedProgram,
    id: DefinitionId,
    source_name: &str,
    span: Span,
    declaration: bool,
    trait_selection: Option<CheckedHoverTraitSelection>,
) -> Result<CheckedHoverEntry, CheckedHoverIndexError> {
    let resolved = checked.typed().resolved();
    let definition = resolved
        .definition(&id)
        .ok_or(CheckedHoverIndexError::MissingDefinition)?;
    let projection = definition_projection(resolved, &id).map_err(|error| match error {
        DefinitionProjectionError::MissingDefinition => CheckedHoverIndexError::MissingDefinition,
        DefinitionProjectionError::InvalidMember => CheckedHoverIndexError::InvalidDefinitionSpan,
    })?;
    let kind = if resolved.trait_member(&id).is_some() {
        CheckedHoverKind::TraitMember
    } else if resolved.impl_member(&id).is_some() {
        CheckedHoverKind::ImplementationMember
    } else {
        match definition.kind {
            DefinitionKind::Value | DefinitionKind::Task => CheckedHoverKind::Value,
            DefinitionKind::Type | DefinitionKind::Actor => CheckedHoverKind::Type,
            DefinitionKind::Constructor => CheckedHoverKind::Constructor,
            DefinitionKind::Builtin => CheckedHoverKind::Builtin,
        }
    };
    let span = if declaration {
        projection
            .name_span
            .ok_or(CheckedHoverIndexError::InvalidDefinitionSpan)?
    } else {
        span
    };
    let module = match definition.origin {
        DefinitionOrigin::User { module } => Some(module),
        DefinitionOrigin::Builtin(_) | DefinitionOrigin::Prelude(_) => None,
    };
    Ok(CheckedHoverEntry {
        source_name: source_name.to_owned(),
        span,
        name: projection.name,
        kind,
        mutable: definition.mutable,
        type_display: checked
            .typed()
            .definition_type(&id)
            .map(|type_id| canonical_type_display(&checked.typed().display_type(type_id))),
        effects: checked
            .definition_effect(&id)
            .map_or_else(Box::default, |row| row.canonical_names().into_boxed_slice()),
        capabilities: module
            .map_or_else(Box::default, |module| module_capabilities(checked, module)),
        trait_selection,
        identity_order: format!("definition:{}", id.as_str()),
    })
}

fn module_capabilities(checked: &CheckedProgram, module: ModuleId) -> Box<[String]> {
    checked
        .module_capabilities(module)
        .map_or_else(Box::default, |values| {
            values
                .iter()
                .map(|capability| capability.name().to_owned())
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
}

fn trait_selections(
    checked: &CheckedProgram,
) -> Result<BTreeMap<ReferenceKey, CheckedHoverTraitSelection>, CheckedHoverIndexError> {
    let mut selections = BTreeMap::new();
    for module in checked.typed().resolved().modules() {
        collect_program_selections(checked, module.id, &module.hir, &mut selections)?;
    }
    Ok(selections)
}

fn collect_program_selections(
    checked: &CheckedProgram,
    module: ModuleId,
    program: &Program,
    output: &mut BTreeMap<ReferenceKey, CheckedHoverTraitSelection>,
) -> Result<(), CheckedHoverIndexError> {
    for definition in &program.definitions {
        collect_expression_selections(checked, module, &definition.value, output)?;
    }
    for task in &program.tasks {
        collect_expression_selections(checked, module, &task.body, output)?;
    }
    for implementation in &program.impls {
        for definition in &implementation.members {
            collect_expression_selections(checked, module, &definition.value, output)?;
        }
    }
    Ok(())
}

fn collect_expression_selections(
    checked: &CheckedProgram,
    module: ModuleId,
    expression: &Expression,
    output: &mut BTreeMap<ReferenceKey, CheckedHoverTraitSelection>,
) -> Result<(), CheckedHoverIndexError> {
    match &expression.kind {
        ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    SequenceElement::Let(binding) => {
                        collect_expression_selections(checked, module, &binding.value, output)?;
                    }
                    SequenceElement::LetAwait(binding) => {
                        collect_expression_selections(checked, module, &binding.call, output)?;
                    }
                    SequenceElement::Expression(expression) => {
                        collect_expression_selections(checked, module, expression, output)?;
                    }
                }
            }
        }
        ExpressionKind::TaskScope { body, .. } => {
            collect_expression_selections(checked, module, body, output)?;
        }
        ExpressionKind::TaskSpawn { call, .. } => {
            collect_expression_selections(checked, module, call, output)?;
        }
        ExpressionKind::TaskAwait { handle, .. } => {
            collect_expression_selections(checked, module, handle, output)?;
        }
        ExpressionKind::TaskReturn { value, .. } => {
            collect_expression_selections(checked, module, value, output)?;
        }
        ExpressionKind::Handle { body, clauses } => {
            collect_expression_selections(checked, module, body, output)?;
            for clause in clauses {
                collect_expression_selections(checked, module, &clause.body, output)?;
            }
        }
        ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_expression_selections(checked, module, condition, output)?;
            collect_expression_selections(checked, module, then_branch, output)?;
            collect_expression_selections(checked, module, else_branch, output)?;
        }
        ExpressionKind::Match { scrutinee, cases } => {
            collect_expression_selections(checked, module, scrutinee, output)?;
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_expression_selections(checked, module, guard, output)?;
                }
                collect_expression_selections(checked, module, &case.body, output)?;
            }
        }
        ExpressionKind::Assignment { value, .. } => {
            collect_expression_selections(checked, module, value, output)?;
        }
        ExpressionKind::Application {
            function,
            arguments,
        } => {
            if let Some(call) = checked.trait_member_call(ExpressionKey::new(module, expression.id))
            {
                let reference = direct_reference(function)
                    .ok_or(CheckedHoverIndexError::InvalidTraitSelection)?;
                let witness = checked
                    .dictionary()
                    .witnesses()
                    .get(call.witness_index())
                    .ok_or(CheckedHoverIndexError::InvalidTraitSelection)?;
                let member = witness
                    .members()
                    .get(call.member_ordinal())
                    .ok_or(CheckedHoverIndexError::InvalidTraitSelection)?;
                if member.definition() != call.implementation() {
                    return Err(CheckedHoverIndexError::InvalidTraitSelection);
                }
                let selection = CheckedHoverTraitSelection {
                    trait_name: witness.trait_name().to_owned(),
                    receiver: witness.receiver(),
                    member: member.name().to_owned(),
                };
                if output
                    .insert(ReferenceKey::new(module, reference), selection.clone())
                    .is_some_and(|previous| previous != selection)
                {
                    return Err(CheckedHoverIndexError::InvalidTraitSelection);
                }
            }
            collect_expression_selections(checked, module, function, output)?;
            for argument in arguments {
                collect_expression_selections(checked, module, argument, output)?;
            }
        }
        ExpressionKind::Projection { target, .. } => {
            collect_expression_selections(checked, module, target, output)?;
        }
        ExpressionKind::Binary { left, right, .. } => {
            collect_expression_selections(checked, module, left, output)?;
            collect_expression_selections(checked, module, right, output)?;
        }
        ExpressionKind::Unary { operand, .. } => {
            collect_expression_selections(checked, module, operand, output)?;
        }
        ExpressionKind::Tuple(elements) | ExpressionKind::List(elements) => {
            for element in elements {
                collect_expression_selections(checked, module, element, output)?;
            }
        }
        ExpressionKind::Record(fields) => {
            for field in fields {
                collect_expression_selections(checked, module, &field.value, output)?;
            }
        }
        ExpressionKind::RecordUpdate { base, fields } => {
            collect_expression_selections(checked, module, base, output)?;
            for field in fields {
                collect_expression_selections(checked, module, &field.value, output)?;
            }
        }
        ExpressionKind::Name { .. } | ExpressionKind::Literal(_) | ExpressionKind::Unit => {}
    }
    Ok(())
}

fn direct_reference(expression: &Expression) -> Option<ling_hir::ReferenceId> {
    match &expression.kind {
        ExpressionKind::Name { reference, .. } | ExpressionKind::Projection { reference, .. } => {
            Some(*reference)
        }
        _ => None,
    }
}

fn span_width(span: Span) -> u32 {
    span.end().get().saturating_sub(span.start().get())
}

fn canonical_type_display(display: &str) -> String {
    let bytes = display.as_bytes();
    let mut variables = BTreeMap::<u32, usize>::new();
    let mut output = String::with_capacity(display.len());
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] == b'\''
            && bytes.get(cursor + 1) == Some(&b't')
            && bytes.get(cursor + 2).is_some_and(u8::is_ascii_digit)
        {
            let mut end = cursor + 2;
            while bytes.get(end).is_some_and(u8::is_ascii_digit) {
                end += 1;
            }
            let variable = display[cursor + 2..end]
                .parse::<u32>()
                .expect("type display variable is bounded u32");
            let next = variables.len();
            let ordinal = *variables.entry(variable).or_insert(next);
            output.push('\'');
            if ordinal < 26 {
                output.push(char::from(
                    b'a' + u8::try_from(ordinal).expect("ordinal below 26"),
                ));
            } else {
                output.push('t');
                output.push_str(&ordinal.to_string());
            }
            cursor = end;
            continue;
        }
        let character = display[cursor..]
            .chars()
            .next()
            .expect("cursor is a character boundary");
        output.push(character);
        cursor += character.len_utf8();
    }
    output
}

fn entry_order(left: &CheckedHoverEntry, right: &CheckedHoverEntry) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.span.source().cmp(&right.span.source()))
        .then_with(|| left.span.start().cmp(&right.span.start()))
        .then_with(|| left.span.end().cmp(&right.span.end()))
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.name.cmp(&right.name))
        .then_with(|| left.identity_order.cmp(&right.identity_order))
}

#[cfg(test)]
mod tests {
    use ling_ast::lower;
    use ling_effects::check as check_effects;
    use ling_hir::lower as lower_hir;
    use ling_resolve::resolve;
    use ling_source::{ByteOffset, SourceFile, SourceId};
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
        let lexed = lex(&source);
        assert!(lexed.errors().is_empty(), "{:?}", lexed.errors());
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower(&source, &parsed).expect("valid AST");
        let hir = lower_hir(source.name().to_owned(), &ast).expect("valid HIR");
        let resolved = resolve(vec![hir], "Main").expect("valid resolution");
        let typed = check_types(resolved).expect("valid types");
        check_effects(typed).expect("valid effects")
    }

    fn offset(source: &str, needle: &str, occurrence: usize) -> ByteOffset {
        let byte = source
            .match_indices(needle)
            .nth(occurrence)
            .unwrap_or_else(|| panic!("missing occurrence {occurrence} of {needle}"))
            .0;
        ByteOffset::new(u32::try_from(byte).expect("fixture fits u32"))
    }

    #[test]
    fn declarations_bindings_and_references_share_checked_facts() {
        let source = "module Main\n\nlet identity value = value\n";
        let index = CheckedHoverIndex::from_checked(&checked(source)).expect("hover index");

        let definition = index
            .source_entry_at("Main.ling", offset(source, "identity", 0))
            .expect("definition hover");
        assert_eq!(definition.name(), "identity");
        assert_eq!(definition.kind(), CheckedHoverKind::Value);
        assert_eq!(definition.type_display(), Some("'a -> 'a"));

        let parameter = index
            .source_entry_at("Main.ling", offset(source, "value", 0))
            .expect("parameter declaration hover");
        let reference = index
            .source_entry_at("Main.ling", offset(source, "value", 1))
            .expect("parameter reference hover");
        assert_eq!(parameter.kind(), CheckedHoverKind::Parameter);
        assert_eq!(reference.kind(), CheckedHoverKind::Parameter);
        assert_eq!(parameter.type_display(), reference.type_display());
        assert_eq!(
            index,
            CheckedHoverIndex::from_checked(&checked(source)).unwrap()
        );
    }

    #[test]
    fn builtin_effects_and_module_capabilities_are_exact() {
        let source = concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () = Console.write \"Ling\"\n",
        );
        let index = CheckedHoverIndex::from_checked(&checked(source)).expect("hover index");
        let main = index
            .source_entry_at("Main.ling", offset(source, "main", 0))
            .expect("main hover");
        assert_eq!(main.effects(), ["Console.Write"]);
        assert_eq!(main.capabilities(), ["Console.Write"]);

        let builtin = index
            .source_entry_at("Main.ling", offset(source, "write", 0))
            .expect("builtin reference hover");
        assert_eq!(builtin.kind(), CheckedHoverKind::Builtin);
        assert_eq!(builtin.name(), "Console.write");
        assert_eq!(builtin.effects(), ["Console.Write"]);
    }

    #[test]
    fn concrete_trait_member_reference_retains_checked_selection() {
        let source = concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "let main () = Renderable.render { name = \"Ling\" }\n",
        );
        let index = CheckedHoverIndex::from_checked(&checked(source)).expect("hover index");
        let call = index
            .source_entry_at("Main.ling", offset(source, "render", 2))
            .expect("Trait call hover");
        let selection = call.trait_selection().expect("checked Trait selection");
        assert_eq!(selection.trait_name(), "Renderable");
        assert_eq!(selection.receiver(), "Item");
        assert_eq!(selection.member(), "render");
    }

    #[test]
    fn checked_seed_target_taxonomy_is_complete() {
        let source = concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Item = { name: Text }\n",
            "type State =\n",
            "    | Idle\n",
            "    | Ready of Int\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "let main value =\n",
            "    let local = Renderable.render { name = value }\n",
            "    Console.write local\n",
        );
        let index = CheckedHoverIndex::from_checked(&checked(source)).expect("hover index");
        let has = |name: &str, kind: CheckedHoverKind| {
            index
                .entries()
                .iter()
                .any(|entry| entry.name() == name && entry.kind() == kind)
        };

        assert!(has("main", CheckedHoverKind::Value));
        assert!(has("Item", CheckedHoverKind::Type));
        assert!(has("State", CheckedHoverKind::Type));
        assert!(has("Idle", CheckedHoverKind::Constructor));
        assert!(has("Console.write", CheckedHoverKind::Builtin));
        assert!(has("Renderable.render", CheckedHoverKind::TraitMember));
        assert!(has("render", CheckedHoverKind::ImplementationMember));
        assert!(has("local", CheckedHoverKind::Binding));
        assert!(has("value", CheckedHoverKind::Parameter));

        for kind in [
            CheckedHoverKind::TraitMember,
            CheckedHoverKind::ImplementationMember,
        ] {
            let declaration = index
                .entries()
                .iter()
                .find(|entry| entry.kind() == kind && entry.trait_selection().is_none())
                .expect("member declaration");
            let start = declaration.span().start().get() as usize;
            let end = declaration.span().end().get() as usize;
            assert_eq!(&source.as_bytes()[start..end], b"render");
        }
    }

    #[test]
    fn type_variables_are_alpha_canonicalized_without_touching_unicode_names() {
        assert_eq!(
            canonical_type_display("'t91 -> 't7 -> 't91"),
            "'a -> 'b -> 'a"
        );
        assert_eq!(canonical_type_display("容器<'t12>"), "容器<'a>");
    }
}
