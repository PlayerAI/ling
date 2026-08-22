use std::cmp::Ordering;

use ling_resolve::{ReferenceTarget, ResolvedProgram};
use ling_source::Span;

/// The existing resolver target category retained by the internal index.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResolvedReferenceTargetKind {
    Definition,
    Binding,
}

/// A definition target copied from the resolver without inventing a location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedReferenceDefinitionTarget {
    definition_id: String,
    name: Option<String>,
    source_name: Option<String>,
    span: Option<Span>,
}

impl ResolvedReferenceDefinitionTarget {
    #[must_use]
    pub fn definition_id(&self) -> &str {
        &self.definition_id
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[must_use]
    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        self.span
    }
}

/// A local binding target copied from the resolver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedReferenceBindingTarget {
    module_id: u32,
    binding_id: u32,
    name: String,
    source_name: String,
    span: Span,
}

impl ResolvedReferenceBindingTarget {
    #[must_use]
    pub const fn module_id(&self) -> u32 {
        self.module_id
    }

    #[must_use]
    pub const fn binding_id(&self) -> u32 {
        self.binding_id
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
}

/// Existing resolver target data for one reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResolvedReferenceTarget {
    Definition(ResolvedReferenceDefinitionTarget),
    Binding(ResolvedReferenceBindingTarget),
}

impl ResolvedReferenceTarget {
    #[must_use]
    pub const fn kind(&self) -> ResolvedReferenceTargetKind {
        match self {
            Self::Definition(_) => ResolvedReferenceTargetKind::Definition,
            Self::Binding(_) => ResolvedReferenceTargetKind::Binding,
        }
    }
}

/// One source reference and its resolver-owned target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedReferenceEntry {
    source_module_id: u32,
    source_module: String,
    source_name: String,
    reference_id: u32,
    target: ResolvedReferenceTarget,
}

impl ResolvedReferenceEntry {
    #[must_use]
    pub const fn source_module_id(&self) -> u32 {
        self.source_module_id
    }

    #[must_use]
    pub fn source_module(&self) -> &str {
        &self.source_module
    }

    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn reference_id(&self) -> u32 {
        self.reference_id
    }

    #[must_use]
    pub const fn target(&self) -> &ResolvedReferenceTarget {
        &self.target
    }
}

/// Deterministic source/module-order inventory of resolved references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedReferenceIndex {
    entries: Box<[ResolvedReferenceEntry]>,
}

impl ResolvedReferenceIndex {
    #[must_use]
    pub fn entries(&self) -> &[ResolvedReferenceEntry] {
        &self.entries
    }

    #[must_use]
    pub fn source_entries(&self, source_name: &str) -> Vec<&ResolvedReferenceEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.source_name == source_name)
            .collect()
    }

    pub(crate) fn from_resolved(resolved: &ResolvedProgram) -> Self {
        let mut entries = resolved
            .references()
            .iter()
            .filter_map(|(key, target)| resolved_reference(resolved, *key, target))
            .collect::<Vec<_>>();
        entries.sort_by(entry_order);
        debug_assert!(
            entries
                .windows(2)
                .all(|pair| { entry_order(&pair[0], &pair[1]) != Ordering::Greater })
        );
        Self {
            entries: entries.into_boxed_slice(),
        }
    }
}

fn resolved_reference(
    resolved: &ResolvedProgram,
    key: ling_resolve::ReferenceKey,
    target: &ReferenceTarget,
) -> Option<ResolvedReferenceEntry> {
    let source_module = resolved.module(key.module())?;
    let target = match target {
        ReferenceTarget::Definition(definition_id) => {
            let definition = resolved.definition(definition_id);
            ResolvedReferenceTarget::Definition(ResolvedReferenceDefinitionTarget {
                definition_id: definition_id.as_str().to_owned(),
                name: definition.map(|value| value.name.clone()),
                source_name: definition.and_then(|value| value.source_name.clone()),
                span: definition.and_then(|value| value.span),
            })
        }
        ReferenceTarget::Binding(binding_key) => {
            let binding = resolved.bindings().get(binding_key)?;
            let binding_module = resolved.module(binding_key.module())?;
            ResolvedReferenceTarget::Binding(ResolvedReferenceBindingTarget {
                module_id: binding_key.module().get(),
                binding_id: binding_key.local().get(),
                name: binding.name.clone(),
                source_name: binding_module.hir.source_name.clone(),
                span: binding.span,
            })
        }
    };
    Some(ResolvedReferenceEntry {
        source_module_id: key.module().get(),
        source_module: source_module.hir.module.name.normalized().to_owned(),
        source_name: source_module.hir.source_name.clone(),
        reference_id: key.local().get(),
        target,
    })
}

fn entry_order(left: &ResolvedReferenceEntry, right: &ResolvedReferenceEntry) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.source_module_id.cmp(&right.source_module_id))
        .then_with(|| left.source_module.cmp(&right.source_module))
        .then_with(|| left.reference_id.cmp(&right.reference_id))
        .then_with(|| left.target.kind().cmp(&right.target.kind()))
        .then_with(|| target_order(&left.target, &right.target))
}

fn target_order(left: &ResolvedReferenceTarget, right: &ResolvedReferenceTarget) -> Ordering {
    match (left, right) {
        (ResolvedReferenceTarget::Definition(left), ResolvedReferenceTarget::Definition(right)) => {
            left.definition_id
                .cmp(&right.definition_id)
                .then_with(|| left.name.cmp(&right.name))
                .then_with(|| left.source_name.cmp(&right.source_name))
                .then_with(|| span_order(left.span, right.span))
        }
        (ResolvedReferenceTarget::Binding(left), ResolvedReferenceTarget::Binding(right)) => (
            left.module_id,
            left.binding_id,
            &left.name,
            &left.source_name,
        )
            .cmp(&(
                right.module_id,
                right.binding_id,
                &right.name,
                &right.source_name,
            ))
            .then_with(|| span_order(Some(left.span), Some(right.span))),
        _ => Ordering::Equal,
    }
}

fn span_order(left: Option<Span>, right: Option<Span>) -> Ordering {
    left.map(|span| (span.source().get(), span.start().get(), span.end().get()))
        .cmp(&right.map(|span| (span.source().get(), span.start().get(), span.end().get())))
}
