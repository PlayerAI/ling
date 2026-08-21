//! Internal RFC-0005 coherence and package-ownership index.
//!
//! The index is intentionally prior to solving.  It validates the restricted
//! declaration/impl shape and records deterministic candidates, but it never
//! chooses an implementation or produces an executable dictionary witness.

use std::collections::{BTreeMap, BTreeSet};

use ling_hir as hir;
use ling_resolve::{DefinitionKind, ModuleId, ResolvedModule, ResolvedProgram};
use ling_source::Span;

use crate::constraints::{self, ConstraintType};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct TraitId {
    pub(crate) module: ModuleId,
    pub(crate) name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TraitInfo {
    pub(crate) id: TraitId,
    pub(crate) module_name: String,
    pub(crate) package_key: Option<String>,
    pub(crate) members: Vec<String>,
    pub(crate) source_name: String,
    pub(crate) span: Span,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ImplId {
    pub(crate) module: ModuleId,
    pub(crate) ordinal: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ImplInfo {
    pub(crate) id: ImplId,
    pub(crate) trait_id: TraitId,
    pub(crate) receiver: ConstraintType,
    pub(crate) receiver_head: String,
    pub(crate) member_names: Vec<String>,
    pub(crate) package_key: Option<String>,
    pub(crate) source_name: String,
    pub(crate) span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoherenceIndex {
    pub(crate) project_graph_id: Option<String>,
    pub(crate) traits: BTreeMap<TraitId, TraitInfo>,
    /// Stable candidate order for diagnostics and future solver input only.
    pub(crate) impls: Vec<ImplInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoherenceError {
    pub(crate) source_name: String,
    pub(crate) span: Span,
    pub(crate) kind: CoherenceErrorKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CoherenceErrorKind {
    DuplicateTrait {
        name: String,
    },
    UnknownTrait {
        name: String,
    },
    InvalidReceiver {
        reason: &'static str,
    },
    UnknownReceiver {
        name: String,
    },
    GenericReceiver {
        name: String,
    },
    OrphanImpl {
        trait_name: String,
        receiver: String,
    },
    DuplicateImpl {
        trait_name: String,
        receiver: String,
    },
    OverlappingImpl {
        trait_name: String,
        receiver_head: String,
    },
    DuplicateMember {
        member: String,
    },
    MissingMember {
        member: String,
    },
    UnexpectedMember {
        member: String,
    },
}

/// Builds the deterministic first-slice coherence index.
pub(crate) fn build_index(
    resolved: &ResolvedProgram,
) -> Result<CoherenceIndex, Vec<CoherenceError>> {
    let mut errors = Vec::new();
    let mut traits = BTreeMap::new();

    for module in resolved.modules() {
        for declaration in &module.hir.traits {
            let id = TraitId {
                module: module.id,
                name: declaration.name.normalized.clone(),
            };
            if traits.contains_key(&id) {
                errors.push(CoherenceError {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.span,
                    kind: CoherenceErrorKind::DuplicateTrait {
                        name: declaration.name.normalized.clone(),
                    },
                });
                continue;
            }
            let mut members = Vec::new();
            let mut member_set = BTreeSet::new();
            for member in &declaration.members {
                if !member_set.insert(member.name.normalized.clone()) {
                    errors.push(CoherenceError {
                        source_name: module.hir.source_name.clone(),
                        span: member.span,
                        kind: CoherenceErrorKind::DuplicateMember {
                            member: member.name.normalized.clone(),
                        },
                    });
                } else {
                    members.push(member.name.normalized.clone());
                }
            }
            traits.insert(
                id.clone(),
                TraitInfo {
                    id,
                    module_name: module.hir.module.name.normalized(),
                    package_key: package_key(module),
                    members,
                    source_name: module.hir.source_name.clone(),
                    span: declaration.span,
                },
            );
        }
    }

    let mut impls = Vec::new();
    for module in resolved.modules() {
        for (ordinal, declaration) in module.hir.impls.iter().enumerate() {
            let Some(trait_id) = resolve_trait_id(module, &declaration.trait_name, &traits) else {
                errors.push(CoherenceError {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.span,
                    kind: CoherenceErrorKind::UnknownTrait {
                        name: declaration.trait_name.normalized(),
                    },
                });
                continue;
            };
            let receiver = match constraints::parse_type_expression(&declaration.receiver) {
                Ok(receiver) => receiver,
                Err(reason) => {
                    errors.push(CoherenceError {
                        source_name: module.hir.source_name.clone(),
                        span: declaration.receiver.span,
                        kind: CoherenceErrorKind::InvalidReceiver { reason },
                    });
                    continue;
                }
            };
            let receiver_head = receiver_head(&receiver).unwrap_or_default();
            if receiver_head.is_empty() {
                errors.push(CoherenceError {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.receiver.span,
                    kind: CoherenceErrorKind::InvalidReceiver {
                        reason: "receiver must be a nominal type",
                    },
                });
                continue;
            }
            if contains_variable(&receiver) {
                errors.push(CoherenceError {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.receiver.span,
                    kind: CoherenceErrorKind::GenericReceiver {
                        name: receiver_head.clone(),
                    },
                });
                continue;
            }
            let Some(receiver_owner) = receiver_owner(resolved, module, &receiver) else {
                errors.push(CoherenceError {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.receiver.span,
                    kind: CoherenceErrorKind::UnknownReceiver {
                        name: receiver_head.clone(),
                    },
                });
                continue;
            };
            let trait_info = traits
                .get(&trait_id)
                .expect("resolved TraitId came from the indexed Trait map");
            let current_package = package_key(module);
            if !owns_target(
                current_package.as_deref(),
                receiver_owner.package_key.as_deref(),
                receiver_owner.local,
            ) && !owns_target(
                current_package.as_deref(),
                trait_info.package_key.as_deref(),
                true,
            ) {
                errors.push(CoherenceError {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.span,
                    kind: CoherenceErrorKind::OrphanImpl {
                        trait_name: trait_id.name.clone(),
                        receiver: canonical_type(&receiver),
                    },
                });
            }

            let mut member_names = Vec::new();
            let mut member_set = BTreeSet::new();
            for member in &declaration.members {
                if !member_set.insert(member.name.normalized.clone()) {
                    errors.push(CoherenceError {
                        source_name: module.hir.source_name.clone(),
                        span: member.span,
                        kind: CoherenceErrorKind::DuplicateMember {
                            member: member.name.normalized.clone(),
                        },
                    });
                } else {
                    member_names.push(member.name.normalized.clone());
                }
            }
            for member in &trait_info.members {
                if !member_set.contains(member) {
                    errors.push(CoherenceError {
                        source_name: module.hir.source_name.clone(),
                        span: declaration.span,
                        kind: CoherenceErrorKind::MissingMember {
                            member: member.clone(),
                        },
                    });
                }
            }
            for member in &member_names {
                if !trait_info.members.contains(member) {
                    errors.push(CoherenceError {
                        source_name: module.hir.source_name.clone(),
                        span: declaration.span,
                        kind: CoherenceErrorKind::UnexpectedMember {
                            member: member.clone(),
                        },
                    });
                }
            }
            impls.push(ImplInfo {
                id: ImplId {
                    module: module.id,
                    ordinal,
                },
                trait_id,
                receiver,
                receiver_head,
                member_names,
                package_key: current_package,
                source_name: module.hir.source_name.clone(),
                span: declaration.span,
            });
        }
    }

    impls.sort_by(|left, right| {
        (
            &left.trait_id,
            canonical_type(&left.receiver),
            left.id.module,
            left.span.source(),
            left.span.start(),
            left.id.ordinal,
        )
            .cmp(&(
                &right.trait_id,
                canonical_type(&right.receiver),
                right.id.module,
                right.span.source(),
                right.span.start(),
                right.id.ordinal,
            ))
    });
    for left_index in 0..impls.len() {
        for right_index in (left_index + 1)..impls.len() {
            let left = &impls[left_index];
            let right = &impls[right_index];
            if left.trait_id != right.trait_id {
                continue;
            }
            let left_receiver = canonical_type(&left.receiver);
            let right_receiver = canonical_type(&right.receiver);
            if left_receiver == right_receiver {
                errors.push(CoherenceError {
                    source_name: right.source_name.clone(),
                    span: right.span,
                    kind: CoherenceErrorKind::DuplicateImpl {
                        trait_name: right.trait_id.name.clone(),
                        receiver: right_receiver,
                    },
                });
            } else if left.receiver_head == right.receiver_head
                && may_overlap(&left.receiver, &right.receiver)
            {
                errors.push(CoherenceError {
                    source_name: right.source_name.clone(),
                    span: right.span,
                    kind: CoherenceErrorKind::OverlappingImpl {
                        trait_name: right.trait_id.name.clone(),
                        receiver_head: right.receiver_head.clone(),
                    },
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(CoherenceIndex {
            project_graph_id: resolved
                .project()
                .map(|project| project.graph_id().as_str().to_owned()),
            traits,
            impls,
        })
    } else {
        errors.sort_by(|left, right| {
            (
                &left.source_name,
                left.span.source(),
                left.span.start(),
                format!("{:?}", left.kind),
            )
                .cmp(&(
                    &right.source_name,
                    right.span.source(),
                    right.span.start(),
                    format!("{:?}", right.kind),
                ))
        });
        Err(errors)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReceiverOwner {
    package_key: Option<String>,
    local: bool,
}

fn resolve_trait_id(
    module: &ResolvedModule,
    name: &hir::QualifiedName,
    traits: &BTreeMap<TraitId, TraitInfo>,
) -> Option<TraitId> {
    resolve_trait_id_name(module, &name.normalized(), traits)
}

pub(crate) fn resolve_trait_id_name(
    module: &ResolvedModule,
    normalized_name: &str,
    traits: &BTreeMap<TraitId, TraitInfo>,
) -> Option<TraitId> {
    let segments = normalized_name.split('.').collect::<Vec<_>>();
    let name = segments.last().copied()?;
    if segments.len() == 1 {
        return traits
            .keys()
            .find(|id| id.module == module.id && id.name == name)
            .cloned();
    }

    let first = segments[0];
    let tail = segments[1..].join(".");
    if let Some(imported) = module.imports.get(first) {
        return traits
            .keys()
            .find(|id| id.module == *imported && id.name == tail)
            .cloned();
    }

    let module_name = segments[..segments.len() - 1].join(".");
    traits
        .values()
        .find(|info| info.module_name == module_name && info.id.name == name)
        .map(|info| info.id.clone())
}

fn receiver_owner(
    resolved: &ResolvedProgram,
    module: &ResolvedModule,
    receiver: &ConstraintType,
) -> Option<ReceiverOwner> {
    let head = receiver_head(receiver)?;
    let (target_module, local_name) = resolve_type_name(module, &head);
    if let Some(target_module) = target_module {
        let target = resolved.module(target_module)?;
        if target
            .hir
            .types
            .iter()
            .any(|declaration| declaration.name.normalized == local_name)
        {
            return Some(ReceiverOwner {
                package_key: package_key(target),
                local: true,
            });
        }
        return None;
    }
    if resolved.prelude_definition(&local_name).is_some_and(|id| {
        resolved
            .definition(id)
            .is_some_and(|info| info.kind == DefinitionKind::Type)
    }) {
        return Some(ReceiverOwner {
            package_key: None,
            local: false,
        });
    }
    None
}

fn resolve_type_name(module: &ResolvedModule, name: &str) -> (Option<ModuleId>, String) {
    let mut segments = name.split('.');
    let first = segments.next().unwrap_or(name);
    let rest = segments.collect::<Vec<_>>();
    if rest.is_empty() {
        return (Some(module.id), first.to_owned());
    }
    if let Some(imported) = module.imports.get(first) {
        return (Some(*imported), rest.join("."));
    }
    (None, name.to_owned())
}

fn package_key(module: &ResolvedModule) -> Option<String> {
    module.package.as_ref().map(|package| {
        format!(
            "{}@{}#{}",
            package.name().as_str(),
            package.version(),
            package.source().as_str()
        )
    })
}

fn owns_target(current: Option<&str>, target: Option<&str>, local: bool) -> bool {
    local && current == target
}

pub(crate) fn receiver_head(receiver: &ConstraintType) -> Option<String> {
    match receiver {
        ConstraintType::Named(name) | ConstraintType::Variable(name) => {
            (!name.is_empty()).then(|| name.clone())
        }
        ConstraintType::Applied { name, .. } => (!name.is_empty()).then(|| name.clone()),
    }
}

pub(crate) fn contains_variable(value: &ConstraintType) -> bool {
    match value {
        ConstraintType::Variable(_) => true,
        ConstraintType::Named(_) => false,
        ConstraintType::Applied { arguments, .. } => arguments.iter().any(contains_variable),
    }
}

fn may_overlap(left: &ConstraintType, right: &ConstraintType) -> bool {
    contains_variable(left) || contains_variable(right)
}

pub(crate) fn canonical_type(value: &ConstraintType) -> String {
    match value {
        ConstraintType::Named(name) => name.clone(),
        ConstraintType::Variable(name) => format!("'{name}"),
        ConstraintType::Applied { name, arguments } => format!(
            "{}<{}>",
            name,
            arguments
                .iter()
                .map(canonical_type)
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;

    fn resolved(text: &str) -> ResolvedProgram {
        let source =
            SourceFile::from_bytes(SourceId::new(0), "coherence.ling", text.as_bytes().to_vec())
                .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = hir::lower(source.name(), &ast).expect("valid HIR");
        ling_resolve::resolve(vec![hir], "Main").expect("resolves")
    }

    #[test]
    fn indexes_nominal_impls_and_preserves_member_order() {
        let index = build_index(&resolved(concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n",
            "    label: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n",
            "    let label item = item.name\n",
        )))
        .expect("coherence is valid");
        assert_eq!(index.traits.len(), 1);
        assert_eq!(index.impls.len(), 1);
        assert_eq!(index.impls[0].receiver_head, "Item");
        assert_eq!(index.impls[0].member_names, ["render", "label"]);
        assert!(index.project_graph_id.is_none());
    }

    #[test]
    fn rejects_duplicate_receiver_impls_and_missing_members() {
        let errors = build_index(&resolved(concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n",
            "    label: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n",
        )))
        .expect_err("duplicate and incomplete impls must fail");
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            CoherenceErrorKind::MissingMember { ref member } if member == "label"
        )));
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            CoherenceErrorKind::DuplicateImpl { ref receiver, .. } if receiver == "Item"
        )));
    }

    #[test]
    fn rejects_generic_receivers_before_overlap_or_selection() {
        let errors = build_index(&resolved(concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Box<'a> = { value: 'a }\n\n",
            "impl Renderable Box<'a> =\n",
            "    let render box = box.value\n",
        )))
        .expect_err("generic impl receivers are outside the first slice");
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            CoherenceErrorKind::GenericReceiver { ref name } if name == "Box"
        )));
    }

    #[test]
    fn ownership_matrix_keeps_orphan_check_package_local_and_deterministic() {
        assert!(owns_target(None, None, true));
        assert!(!owns_target(Some("app@1#root"), Some("dep@1#dep"), true));
        assert!(!owns_target(Some("app@1#root"), None, false));
        assert!(may_overlap(
            &ConstraintType::Applied {
                name: "Box".to_owned(),
                arguments: vec![ConstraintType::Variable("a".to_owned())],
            },
            &ConstraintType::Applied {
                name: "Box".to_owned(),
                arguments: vec![ConstraintType::Named("Int".to_owned())],
            }
        ));
    }
}
