//! Bounded checked symbol facts for RFC-0042 completion analysis.
//!
//! The catalog is compiler-owned and wire-agnostic. It copies only facts from
//! one complete checked program; context selection, replacement validation,
//! ranking, position projection, and JSON publication remain LSP concerns.

use std::cmp::Ordering;

use ling_effects::CheckedProgram;
use ling_resolve::{DefinitionKind, DefinitionOrigin};
use ling_source::Span;

use crate::completion_source_index::ResolvedCompletionSourceIdentity;

/// Maximum number of compiler facts that one RFC-0042 request may inspect.
pub const MAX_CHECKED_COMPLETION_CANDIDATES: usize = 512;

/// Compiler category for one possible completion spelling.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CheckedCompletionKind {
    Value,
    Type,
    Constructor,
    Builtin,
    Binding,
    ImportAlias,
    Module,
    Field,
    Keyword,
}

/// One immutable checked candidate source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedCompletionCandidate {
    name: String,
    qualifier: Option<String>,
    kind: CheckedCompletionKind,
    module_id: Option<u32>,
    target_module_id: Option<u32>,
    source_name: Option<String>,
    span: Option<Span>,
    identity: String,
    metadata_identity: Option<ResolvedCompletionSourceIdentity>,
}

impl CheckedCompletionCandidate {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn qualifier(&self) -> Option<&str> {
        self.qualifier.as_deref()
    }

    #[must_use]
    pub const fn kind(&self) -> CheckedCompletionKind {
        self.kind
    }

    #[must_use]
    pub const fn module_id(&self) -> Option<u32> {
        self.module_id
    }

    #[must_use]
    pub const fn target_module_id(&self) -> Option<u32> {
        self.target_module_id
    }

    #[must_use]
    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        self.span
    }

    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Returns the existing resolver identity when DEC-0080 checked metadata
    /// may be selected for this candidate.
    #[must_use]
    pub fn metadata_identity(&self) -> Option<&ResolvedCompletionSourceIdentity> {
        self.metadata_identity.as_ref()
    }
}

/// Deterministic facts copied from one complete checked workspace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedCompletionCatalog {
    candidates: Box<[CheckedCompletionCandidate]>,
}

impl CheckedCompletionCatalog {
    #[must_use]
    pub fn candidates(&self) -> &[CheckedCompletionCandidate] {
        &self.candidates
    }

    pub(crate) fn from_checked(checked: &CheckedProgram) -> Self {
        let typed = checked.typed();
        let resolved = typed.resolved();
        let mut candidates = Vec::new();

        for definition in resolved.definitions().values() {
            let (kind, module_id) = match definition.kind {
                DefinitionKind::Value => (
                    CheckedCompletionKind::Value,
                    user_module_id(definition.origin),
                ),
                DefinitionKind::Type => (
                    CheckedCompletionKind::Type,
                    user_module_id(definition.origin),
                ),
                DefinitionKind::Constructor => (
                    CheckedCompletionKind::Constructor,
                    user_module_id(definition.origin),
                ),
                DefinitionKind::Builtin => (CheckedCompletionKind::Builtin, None),
            };
            let (qualifier, name) = split_qualified_builtin(&definition.name, definition.origin);
            let metadata_identity = matches!(definition.origin, DefinitionOrigin::User { .. })
                .then(|| ResolvedCompletionSourceIdentity::Definition {
                    definition_id: definition.id.as_str().to_owned(),
                });
            candidates.push(CheckedCompletionCandidate {
                name,
                qualifier,
                kind,
                module_id,
                target_module_id: None,
                source_name: definition.source_name.clone(),
                span: definition.span,
                identity: format!("definition:{}", definition.id.as_str()),
                metadata_identity,
            });
        }

        for (key, binding) in resolved.bindings() {
            let source_name = resolved
                .module(key.module())
                .map(|module| module.hir.source_name.clone());
            candidates.push(CheckedCompletionCandidate {
                name: binding.name.clone(),
                qualifier: None,
                kind: CheckedCompletionKind::Binding,
                module_id: Some(key.module().get()),
                target_module_id: None,
                source_name,
                span: Some(binding.span),
                identity: format!("binding:{}:{}", key.module().get(), key.local().get()),
                metadata_identity: Some(ResolvedCompletionSourceIdentity::Binding {
                    module_id: key.module().get(),
                    binding_id: key.local().get(),
                }),
            });
        }

        for module in resolved.modules() {
            candidates.push(CheckedCompletionCandidate {
                name: module.hir.module.name.normalized(),
                qualifier: None,
                kind: CheckedCompletionKind::Module,
                module_id: Some(module.id.get()),
                target_module_id: None,
                source_name: Some(module.hir.source_name.clone()),
                span: Some(module.hir.module.name.span),
                identity: format!("module:{}", module.id.get()),
                metadata_identity: None,
            });
            for import in &module.hir.imports {
                let Some(target) = module.imports.get(&import.alias.normalized) else {
                    continue;
                };
                candidates.push(CheckedCompletionCandidate {
                    name: import.alias.normalized.clone(),
                    qualifier: None,
                    kind: CheckedCompletionKind::ImportAlias,
                    module_id: Some(module.id.get()),
                    target_module_id: Some(target.get()),
                    source_name: Some(module.hir.source_name.clone()),
                    span: Some(import.alias.span),
                    identity: format!("alias:{}:{}", module.id.get(), target.get()),
                    metadata_identity: None,
                });
            }
        }

        for record in typed.records().values() {
            for field in &record.fields {
                candidates.push(CheckedCompletionCandidate {
                    name: field.name.clone(),
                    qualifier: Some(record.name.clone()),
                    kind: CheckedCompletionKind::Field,
                    module_id: None,
                    target_module_id: None,
                    source_name: None,
                    span: None,
                    identity: format!("field:{}:{}", record.definition.as_str(), field.name),
                    metadata_identity: None,
                });
            }
        }

        candidates.sort_by(candidate_order);
        candidates.dedup_by(|left, right| {
            left.kind == right.kind
                && left.name == right.name
                && left.qualifier == right.qualifier
                && left.identity == right.identity
        });
        Self {
            candidates: candidates.into_boxed_slice(),
        }
    }
}

const fn user_module_id(origin: DefinitionOrigin) -> Option<u32> {
    match origin {
        DefinitionOrigin::User { module } => Some(module.get()),
        DefinitionOrigin::Builtin(_) | DefinitionOrigin::Prelude(_) => None,
    }
}

fn split_qualified_builtin(name: &str, origin: DefinitionOrigin) -> (Option<String>, String) {
    if matches!(origin, DefinitionOrigin::Builtin(_)) {
        if let Some((qualifier, member)) = name.rsplit_once('.') {
            return (Some(qualifier.to_owned()), member.to_owned());
        }
    }
    (None, name.to_owned())
}

fn candidate_order(
    left: &CheckedCompletionCandidate,
    right: &CheckedCompletionCandidate,
) -> Ordering {
    left.name
        .cmp(&right.name)
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.qualifier.cmp(&right.qualifier))
        .then_with(|| left.module_id.cmp(&right.module_id))
        .then_with(|| left.target_module_id.cmp(&right.target_module_id))
        .then_with(|| left.source_name.cmp(&right.source_name))
        .then_with(|| span_key(left.span).cmp(&span_key(right.span)))
        .then_with(|| left.identity.cmp(&right.identity))
}

fn span_key(span: Option<Span>) -> Option<(u32, u32, u32)> {
    span.map(|span| (span.source().get(), span.start().get(), span.end().get()))
}

#[cfg(test)]
mod tests {
    use ling_ast::lower;
    use ling_effects::check as check_effects;
    use ling_hir::lower as lower_hir;
    use ling_resolve::resolve;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;
    use ling_types::check as check_types;

    use super::*;

    fn checked(sources: &[(&str, &str)], entry: &str) -> CheckedProgram {
        let programs = sources
            .iter()
            .enumerate()
            .map(|(index, (name, text))| {
                let source = SourceFile::from_bytes(
                    SourceId::new(u32::try_from(index).expect("source count fits")),
                    *name,
                    text.as_bytes().to_vec(),
                )
                .expect("valid source");
                let parsed = parse(&source);
                assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
                let ast = lower(&source, &parsed).expect("valid AST");
                lower_hir(source.name(), &ast).expect("valid HIR")
            })
            .collect();
        let resolved = resolve(programs, entry).expect("valid resolution");
        let typed = check_types(resolved).expect("valid types");
        check_effects(typed).expect("valid effects")
    }

    #[test]
    fn records_checked_symbols_modules_aliases_and_fields_deterministically() {
        let checked = checked(
            &[
                (
                    "Library.ling",
                    "module Library\n\ntype Point = { x: Int; y: Int }\n\nlet answer = 1\n",
                ),
                (
                    "Main.ling",
                    "module Main\n\nimport Library as Lib\n\nlet main value = value\n",
                ),
            ],
            "Main",
        );
        let first = CheckedCompletionCatalog::from_checked(&checked);
        let second = CheckedCompletionCatalog::from_checked(&checked);

        assert_eq!(first, second);
        assert!(first.candidates().iter().any(|candidate| {
            candidate.kind() == CheckedCompletionKind::ImportAlias
                && candidate.name() == "Lib"
                && candidate.target_module_id().is_some()
        }));
        assert!(first.candidates().iter().any(|candidate| {
            candidate.kind() == CheckedCompletionKind::Module && candidate.name() == "Library"
        }));
        assert!(first.candidates().iter().any(|candidate| {
            candidate.kind() == CheckedCompletionKind::Field && candidate.name() == "x"
        }));
        assert!(first.candidates().iter().any(|candidate| {
            candidate.kind() == CheckedCompletionKind::Binding
                && candidate.name() == "value"
                && candidate.span().is_some()
                && matches!(
                    candidate.metadata_identity(),
                    Some(ResolvedCompletionSourceIdentity::Binding { .. })
                )
        }));
        assert!(first.candidates().iter().any(|candidate| {
            candidate.name() == "answer"
                && matches!(
                    candidate.metadata_identity(),
                    Some(ResolvedCompletionSourceIdentity::Definition { .. })
                )
        }));
        assert!(first.candidates().iter().all(|candidate| {
            matches!(
                candidate.kind(),
                CheckedCompletionKind::Value
                    | CheckedCompletionKind::Type
                    | CheckedCompletionKind::Constructor
                    | CheckedCompletionKind::Binding
            ) || candidate.metadata_identity().is_none()
        }));
    }

    #[test]
    fn splits_only_qualified_builtins_for_member_completion() {
        let checked = checked(
            &[(
                "Main.ling",
                "module Main\n    requires Console.Write\n\nlet main () = Console.write \"ok\"\n",
            )],
            "Main",
        );
        let catalog = CheckedCompletionCatalog::from_checked(&checked);
        let write = catalog
            .candidates()
            .iter()
            .find(|candidate| candidate.qualifier() == Some("Console"))
            .expect("qualified Console builtin");
        assert_eq!(write.name(), "write");
        assert_eq!(write.kind(), CheckedCompletionKind::Builtin);
        assert!(write.metadata_identity().is_none());
    }
}
