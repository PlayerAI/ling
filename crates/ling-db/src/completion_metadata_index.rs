use std::cmp::Ordering;

use ling_effects::CheckedProgram;
use ling_resolve::DefinitionOrigin;
use ling_source::Span;

use crate::completion_source_index::{
    ResolvedCompletionSourceIdentity, ResolvedCompletionSourceKind,
};

/// Checked type/effect/capability facts joined to one resolver-backed
/// completion source.
///
/// These are compiler observations only. They are not a completion-item
/// presentation, documentation body, capability disclosure, or insertion
/// contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCompletionMetadata {
    module_id: u32,
    name: String,
    source_name: String,
    span: Span,
    kind: ResolvedCompletionSourceKind,
    identity: ResolvedCompletionSourceIdentity,
    type_display: Option<String>,
    effects: Option<Box<[String]>>,
    capabilities: Option<Box<[String]>>,
}

impl ResolvedCompletionMetadata {
    #[must_use]
    pub const fn module_id(&self) -> u32 {
        self.module_id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
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
    pub const fn kind(&self) -> ResolvedCompletionSourceKind {
        self.kind
    }

    #[must_use]
    pub fn identity(&self) -> &ResolvedCompletionSourceIdentity {
        &self.identity
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

/// Deterministic checked metadata for resolver-backed definitions and
/// bindings.
///
/// The index does not render signatures or documentation, choose a completion
/// candidate, disclose a capability to an editor, or retain protocol state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCompletionMetadataIndex {
    entries: Box<[ResolvedCompletionMetadata]>,
}

impl ResolvedCompletionMetadataIndex {
    #[must_use]
    pub fn entries(&self) -> &[ResolvedCompletionMetadata] {
        &self.entries
    }

    #[must_use]
    pub fn source_entries(&self, source_name: &str) -> Vec<&ResolvedCompletionMetadata> {
        self.entries
            .iter()
            .filter(|entry| entry.source_name == source_name)
            .collect()
    }

    #[must_use]
    pub fn name_entries(&self, name: &str) -> Vec<&ResolvedCompletionMetadata> {
        self.entries
            .iter()
            .filter(|entry| entry.name == name)
            .collect()
    }

    #[must_use]
    pub fn identity(
        &self,
        identity: &ResolvedCompletionSourceIdentity,
    ) -> Option<&ResolvedCompletionMetadata> {
        self.entries
            .iter()
            .find(|entry| &entry.identity == identity)
    }

    pub(crate) fn from_checked(checked: &CheckedProgram) -> Self {
        let resolved = checked.typed().resolved();
        let mut entries = Vec::new();

        for definition in resolved.definitions().values() {
            let DefinitionOrigin::User { module } = definition.origin else {
                continue;
            };
            let Some(source_name) = definition.source_name.clone() else {
                continue;
            };
            let Some(span) = definition.span else {
                continue;
            };
            entries.push(ResolvedCompletionMetadata {
                module_id: module.get(),
                name: definition.name.clone(),
                source_name,
                span,
                kind: ResolvedCompletionSourceKind::Definition,
                identity: ResolvedCompletionSourceIdentity::Definition {
                    definition_id: definition.id.as_str().to_owned(),
                },
                type_display: checked
                    .typed()
                    .definition_type(&definition.id)
                    .map(|type_id| checked.typed().display_type(type_id)),
                effects: checked
                    .definition_effect(&definition.id)
                    .map(|row| row.canonical_names().into_boxed_slice()),
                capabilities: checked.module_capabilities(module).map(|values| {
                    values
                        .iter()
                        .map(|capability| capability.name().to_owned())
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                }),
            });
        }

        for (key, binding) in resolved.bindings() {
            let Some(module) = resolved.module(key.module()) else {
                continue;
            };
            entries.push(ResolvedCompletionMetadata {
                module_id: key.module().get(),
                name: binding.name.clone(),
                source_name: module.hir.source_name.clone(),
                span: binding.span,
                kind: ResolvedCompletionSourceKind::Binding,
                identity: ResolvedCompletionSourceIdentity::Binding {
                    module_id: key.module().get(),
                    binding_id: key.local().get(),
                },
                type_display: checked
                    .typed()
                    .binding_type(*key)
                    .map(|type_id| checked.typed().display_type(type_id)),
                effects: checked
                    .binding_effect(*key)
                    .map(|row| row.canonical_names().into_boxed_slice()),
                capabilities: checked.module_capabilities(key.module()).map(|values| {
                    values
                        .iter()
                        .map(|capability| capability.name().to_owned())
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                }),
            });
        }

        entries.sort_by(metadata_order);
        debug_assert!(
            entries
                .windows(2)
                .all(|pair| { metadata_order(&pair[0], &pair[1]) != Ordering::Greater })
        );
        Self {
            entries: entries.into_boxed_slice(),
        }
    }
}

fn metadata_order(
    left: &ResolvedCompletionMetadata,
    right: &ResolvedCompletionMetadata,
) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.span.source().cmp(&right.span.source()))
        .then_with(|| left.span.start().get().cmp(&right.span.start().get()))
        .then_with(|| left.span.end().get().cmp(&right.span.end().get()))
        .then_with(|| left.module_id.cmp(&right.module_id))
        .then_with(|| left.name.cmp(&right.name))
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.identity.cmp(&right.identity))
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

    #[test]
    fn joins_checked_definition_and_binding_facts_without_presentation_policy() {
        let checked = checked("module Main\n\nlet main value = value\n");
        let index = ResolvedCompletionMetadataIndex::from_checked(&checked);

        let main = index
            .name_entries("main")
            .into_iter()
            .find(|entry| entry.kind() == ResolvedCompletionSourceKind::Definition)
            .expect("definition metadata");
        assert!(main.type_display().is_some());
        assert!(main.effects().is_some());
        assert!(main.capabilities().is_some());

        let value = index
            .name_entries("value")
            .into_iter()
            .find(|entry| entry.kind() == ResolvedCompletionSourceKind::Binding)
            .expect("binding metadata");
        assert!(value.type_display().is_some());
        assert!(value.effects().is_none());
    }

    #[test]
    fn construction_and_identity_lookup_are_repeatable() {
        let checked = checked("module Main\n\nlet helper = 1\n\nlet main () = helper\n");
        let first = ResolvedCompletionMetadataIndex::from_checked(&checked);
        let second = ResolvedCompletionMetadataIndex::from_checked(&checked);

        assert_eq!(first, second);
        let identity = first.entries()[0].identity().clone();
        assert!(first.identity(&identity).is_some());
        assert!(first.source_entries("missing.ling").is_empty());
    }
}
