use std::cmp::Ordering;

use ling_resolve::{DefinitionOrigin, ResolvedProgram};
use ling_source::Span;

/// The resolver-owned source of one future completion candidate.
///
/// This is an internal source category, not an editor completion kind. It
/// deliberately does not decide visibility, ranking, insertion text, or
/// whether the source is valid for a particular completion context.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResolvedCompletionSourceKind {
    Definition,
    Binding,
    ImportAlias,
}

impl ResolvedCompletionSourceKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Definition => "definition",
            Self::Binding => "binding",
            Self::ImportAlias => "import_alias",
        }
    }
}

/// Existing resolver identity for an internal completion-source observation.
///
/// No new identity is created here: definitions retain their resolver
/// `DefinitionId`, bindings retain their `(ModuleId, BindingId)`, and import
/// aliases retain the resolved target module.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResolvedCompletionSourceIdentity {
    Definition { definition_id: String },
    Binding { module_id: u32, binding_id: u32 },
    ImportAlias { target_module_id: u32 },
}

/// One resolver-backed name source retained for future completion analysis.
///
/// The span is the original UTF-8 alias/name span. It is not an editor range,
/// insertion edit, or request-position projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCompletionSource {
    module_id: u32,
    name: String,
    source_name: String,
    span: Span,
    kind: ResolvedCompletionSourceKind,
    identity: ResolvedCompletionSourceIdentity,
}

impl ResolvedCompletionSource {
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
}

/// Deterministic, resolver-backed name sources for future completion work.
///
/// The index contains only facts already present in a validated resolver
/// result. It performs no context classification, scope-distance calculation,
/// type/effect ranking, visibility filtering, insertion-text generation,
/// request-position conversion, snapshot binding, or protocol publication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCompletionSourceIndex {
    entries: Box<[ResolvedCompletionSource]>,
}

impl ResolvedCompletionSourceIndex {
    #[must_use]
    pub fn entries(&self) -> &[ResolvedCompletionSource] {
        &self.entries
    }

    #[must_use]
    pub fn module_entries(&self, module_id: u32) -> Vec<&ResolvedCompletionSource> {
        self.entries
            .iter()
            .filter(|entry| entry.module_id == module_id)
            .collect()
    }

    #[must_use]
    pub fn source_entries(&self, source_name: &str) -> Vec<&ResolvedCompletionSource> {
        self.entries
            .iter()
            .filter(|entry| entry.source_name == source_name)
            .collect()
    }

    #[must_use]
    pub fn name_entries(&self, name: &str) -> Vec<&ResolvedCompletionSource> {
        self.entries
            .iter()
            .filter(|entry| entry.name == name)
            .collect()
    }

    pub(crate) fn from_resolved(resolved: &ResolvedProgram) -> Self {
        let mut entries = Vec::new();

        for definition in resolved.definitions().values() {
            let DefinitionOrigin::User { module } = &definition.origin else {
                continue;
            };
            let Some(source_name) = definition.source_name.clone() else {
                continue;
            };
            let Some(span) = definition.span else {
                continue;
            };
            entries.push(ResolvedCompletionSource {
                module_id: module.get(),
                name: definition.name.clone(),
                source_name,
                span,
                kind: ResolvedCompletionSourceKind::Definition,
                identity: ResolvedCompletionSourceIdentity::Definition {
                    definition_id: definition.id.as_str().to_owned(),
                },
            });
        }

        for (key, binding) in resolved.bindings() {
            let Some(module) = resolved.module(key.module()) else {
                continue;
            };
            entries.push(ResolvedCompletionSource {
                module_id: key.module().get(),
                name: binding.name.clone(),
                source_name: module.hir.source_name.clone(),
                span: binding.span,
                kind: ResolvedCompletionSourceKind::Binding,
                identity: ResolvedCompletionSourceIdentity::Binding {
                    module_id: key.module().get(),
                    binding_id: key.local().get(),
                },
            });
        }

        for module in resolved.modules() {
            for import in &module.hir.imports {
                let alias = import.alias.normalized.clone();
                let Some(target_module_id) = module.imports.get(&alias) else {
                    continue;
                };
                entries.push(ResolvedCompletionSource {
                    module_id: module.id.get(),
                    name: alias,
                    source_name: module.hir.source_name.clone(),
                    span: import.alias.span,
                    kind: ResolvedCompletionSourceKind::ImportAlias,
                    identity: ResolvedCompletionSourceIdentity::ImportAlias {
                        target_module_id: target_module_id.get(),
                    },
                });
            }
        }

        entries.sort_by(source_order);
        debug_assert!(
            entries
                .windows(2)
                .all(|pair| { source_order(&pair[0], &pair[1]) != Ordering::Greater })
        );
        Self {
            entries: entries.into_boxed_slice(),
        }
    }
}

fn source_order(left: &ResolvedCompletionSource, right: &ResolvedCompletionSource) -> Ordering {
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
    use ling_hir::lower as lower_hir;
    use ling_resolve::resolve;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::{lex, parse};

    use super::*;

    fn resolved(sources: &[(&str, &str)]) -> ResolvedProgram {
        let programs = sources
            .iter()
            .enumerate()
            .map(|(index, (source_name, text))| {
                let source = SourceFile::from_bytes(
                    SourceId::new(u32::try_from(index).expect("test source index fits")),
                    *source_name,
                    text.as_bytes().to_vec(),
                )
                .expect("valid source");
                let lexed = lex(&source);
                assert!(lexed.errors().is_empty(), "{:?}", lexed.errors());
                let parsed = parse(&source);
                assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
                let ast = lower(&source, &parsed).expect("valid AST");
                lower_hir(source.name().to_owned(), &ast).expect("valid HIR")
            })
            .collect();
        resolve(programs, "Main").expect("valid resolution")
    }

    #[test]
    fn records_definitions_bindings_and_import_aliases_with_original_spans() {
        let sources = [
            ("Library.ling", "module Library\n\nlet answer = 1\n"),
            (
                "Main.ling",
                "module Main\n\nimport Library as Lib\n\nlet main value = Lib.answer\n",
            ),
        ];
        let resolved = resolved(&sources);
        let index = ResolvedCompletionSourceIndex::from_resolved(&resolved);

        assert!(index.entries().iter().any(|entry| entry.name() == "answer"
            && entry.kind() == ResolvedCompletionSourceKind::Definition));
        let alias = index
            .name_entries("Lib")
            .into_iter()
            .find(|entry| entry.kind() == ResolvedCompletionSourceKind::ImportAlias)
            .expect("resolved import alias");
        assert_eq!(alias.source_name(), "Main.ling");
        let bytes = sources[1].1.as_bytes();
        let start = alias.span().start().get() as usize;
        let end = alias.span().end().get() as usize;
        assert_eq!(&bytes[start..end], b"Lib");
        assert!(index.entries().iter().any(|entry| entry.name() == "value"
            && entry.kind() == ResolvedCompletionSourceKind::Binding));
    }

    #[test]
    fn construction_and_lookups_are_deterministic_and_source_scoped() {
        let resolved = resolved(&[("Main.ling", "module Main\n\nlet main value = value\n")]);
        let first = ResolvedCompletionSourceIndex::from_resolved(&resolved);
        let second = ResolvedCompletionSourceIndex::from_resolved(&resolved);

        assert_eq!(first, second);
        assert!(!first.source_entries("Main.ling").is_empty());
        assert!(first.source_entries("missing.ling").is_empty());
        assert_eq!(first.name_entries("value").len(), 1);
    }
}
