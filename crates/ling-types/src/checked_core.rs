//! Internal RFC-0005 Checked Core dictionary witness lowering.
//!
//! This module consumes immutable solver selections and turns them into an
//! immutable witness table. It never performs candidate selection. The table
//! is attached to `TypedProgram` and consumed by the checked interpreter and
//! bytecode lowerer through the RFC-0021 static Trait boundary.

use std::collections::{BTreeMap, BTreeSet};

use ling_resolve::{DefinitionId, ResolvedProgram};
use ling_source::Span;

use crate::coherence::{self, CoherenceIndex, ImplId, TraitId};
use crate::constraints::{ConstraintType, ObligationOrigin};
use crate::solver::SolvedObligation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DictionaryMember {
    pub(crate) ordinal: usize,
    pub(crate) name: String,
    pub(crate) definition: DefinitionId,
}

impl DictionaryMember {
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn definition(&self) -> &DefinitionId {
        &self.definition
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DictionaryWitness {
    pub(crate) obligation_order: usize,
    pub(crate) trait_id: TraitId,
    pub(crate) impl_id: ImplId,
    pub(crate) receiver: ConstraintType,
    pub(crate) members: Vec<DictionaryMember>,
    pub(crate) origin: ObligationOrigin,
}

impl DictionaryWitness {
    #[must_use]
    pub const fn obligation_order(&self) -> usize {
        self.obligation_order
    }

    #[must_use]
    pub fn trait_name(&self) -> &str {
        &self.trait_id.name
    }

    #[must_use]
    pub fn members(&self) -> &[DictionaryMember] {
        &self.members
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DictionaryTable {
    witnesses: Vec<DictionaryWitness>,
}

/// Backend-facing selection for one checked `Trait.member` application.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraitMemberCall {
    pub(crate) witness_index: usize,
    pub(crate) member_ordinal: usize,
    pub(crate) implementation: DefinitionId,
}

impl TraitMemberCall {
    #[must_use]
    pub const fn witness_index(&self) -> usize {
        self.witness_index
    }

    #[must_use]
    pub const fn member_ordinal(&self) -> usize {
        self.member_ordinal
    }

    #[must_use]
    pub const fn implementation(&self) -> &DefinitionId {
        &self.implementation
    }
}

impl DictionaryTable {
    #[must_use]
    pub(crate) const fn empty() -> Self {
        Self {
            witnesses: Vec::new(),
        }
    }

    #[must_use]
    pub fn witnesses(&self) -> &[DictionaryWitness] {
        &self.witnesses
    }

    /// Returns deterministic semantic bytes for the witness table.
    ///
    /// Source names and byte spans are intentionally absent: they are origin
    /// metadata for diagnostics, not semantic identity. Length-prefixed UTF-8
    /// fields avoid delimiter ambiguity without depending on host formatting.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ling.checked-core.dictionary/0\0");
        append_usize(&mut bytes, self.witnesses.len());
        for witness in &self.witnesses {
            append_usize(&mut bytes, witness.obligation_order);
            append_u64(&mut bytes, u64::from(witness.trait_id.module.get()));
            append_string(&mut bytes, &witness.trait_id.name);
            append_u64(&mut bytes, u64::from(witness.impl_id.module.get()));
            append_usize(&mut bytes, witness.impl_id.ordinal);
            append_string(&mut bytes, &coherence::canonical_type(&witness.receiver));
            append_usize(&mut bytes, witness.members.len());
            for member in &witness.members {
                append_usize(&mut bytes, member.ordinal);
                append_string(&mut bytes, &member.name);
                append_string(&mut bytes, member.definition.as_str());
            }
        }
        bytes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DictionaryLoweringError {
    pub(crate) source_name: String,
    pub(crate) span: Span,
    pub(crate) kind: DictionaryLoweringErrorKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum DictionaryLoweringErrorKind {
    DuplicateObligationOrder {
        order: usize,
    },
    UnknownImplementation {
        module: u32,
        ordinal: usize,
    },
    TraitMismatch {
        selected: TraitId,
        indexed: TraitId,
    },
    ReceiverMismatch {
        selected: String,
        indexed: String,
    },
    MemberMismatch {
        selected: Vec<String>,
        indexed: Vec<String>,
    },
}

/// Lowers solver selections into an immutable, backend-consumable witness
/// table without re-running Trait selection.
pub(crate) fn lower_dictionary_witnesses(
    selections: &[SolvedObligation],
    index: &CoherenceIndex,
    resolved: &ResolvedProgram,
) -> Result<DictionaryTable, Vec<DictionaryLoweringError>> {
    let implementations = index
        .impls
        .iter()
        .map(|implementation| (&implementation.id, implementation))
        .collect::<BTreeMap<_, _>>();
    let mut errors = Vec::new();
    let mut orders = BTreeSet::new();
    let mut witnesses = Vec::with_capacity(selections.len());

    for selection in selections {
        if !orders.insert(selection.obligation_order) {
            errors.push(DictionaryLoweringError {
                source_name: selection.origin.source_name.clone(),
                span: selection.origin.span,
                kind: DictionaryLoweringErrorKind::DuplicateObligationOrder {
                    order: selection.obligation_order,
                },
            });
            continue;
        }

        let Some(indexed_impl) = implementations.get(&selection.impl_id) else {
            errors.push(DictionaryLoweringError {
                source_name: selection.origin.source_name.clone(),
                span: selection.origin.span,
                kind: DictionaryLoweringErrorKind::UnknownImplementation {
                    module: selection.impl_id.module.get(),
                    ordinal: selection.impl_id.ordinal,
                },
            });
            continue;
        };

        if indexed_impl.trait_id != selection.trait_id {
            errors.push(DictionaryLoweringError {
                source_name: selection.origin.source_name.clone(),
                span: selection.origin.span,
                kind: DictionaryLoweringErrorKind::TraitMismatch {
                    selected: selection.trait_id.clone(),
                    indexed: indexed_impl.trait_id.clone(),
                },
            });
            continue;
        }

        let selected_receiver = coherence::canonical_type(&selection.receiver);
        let indexed_receiver = coherence::canonical_type(&indexed_impl.receiver);
        if selected_receiver != indexed_receiver {
            errors.push(DictionaryLoweringError {
                source_name: selection.origin.source_name.clone(),
                span: selection.origin.span,
                kind: DictionaryLoweringErrorKind::ReceiverMismatch {
                    selected: selected_receiver,
                    indexed: indexed_receiver,
                },
            });
            continue;
        }

        let Some(trait_info) = index.traits.get(&selection.trait_id) else {
            // A valid coherence index always has this entry. Treat a malformed
            // index as a member mismatch rather than inventing a new public
            // error category.
            errors.push(DictionaryLoweringError {
                source_name: selection.origin.source_name.clone(),
                span: selection.origin.span,
                kind: DictionaryLoweringErrorKind::MemberMismatch {
                    selected: selection.member_names.clone(),
                    indexed: Vec::new(),
                },
            });
            continue;
        };
        if selection.member_names != indexed_impl.member_names
            || selection.member_names != trait_info.members
        {
            errors.push(DictionaryLoweringError {
                source_name: selection.origin.source_name.clone(),
                span: selection.origin.span,
                kind: DictionaryLoweringErrorKind::MemberMismatch {
                    selected: selection.member_names.clone(),
                    indexed: trait_info.members.clone(),
                },
            });
            continue;
        }

        let Some(members) = selection
            .member_names
            .iter()
            .enumerate()
            .map(|(ordinal, name)| {
                resolved
                    .impl_members()
                    .values()
                    .find(|member| {
                        member.module == selection.impl_id.module
                            && member.impl_ordinal == selection.impl_id.ordinal
                            && member.member_name == *name
                    })
                    .map(|member| DictionaryMember {
                        ordinal,
                        name: name.clone(),
                        definition: member.definition.clone(),
                    })
            })
            .collect::<Option<Vec<_>>>()
        else {
            errors.push(DictionaryLoweringError {
                source_name: selection.origin.source_name.clone(),
                span: selection.origin.span,
                kind: DictionaryLoweringErrorKind::MemberMismatch {
                    selected: selection.member_names.clone(),
                    indexed: Vec::new(),
                },
            });
            continue;
        };
        witnesses.push(DictionaryWitness {
            obligation_order: selection.obligation_order,
            trait_id: selection.trait_id.clone(),
            impl_id: selection.impl_id.clone(),
            receiver: selection.receiver.clone(),
            members,
            origin: selection.origin.clone(),
        });
    }

    if !errors.is_empty() {
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
        return Err(errors);
    }

    witnesses.sort_by(|left, right| {
        (
            left.obligation_order,
            &left.trait_id,
            &left.impl_id,
            coherence::canonical_type(&left.receiver),
        )
            .cmp(&(
                right.obligation_order,
                &right.trait_id,
                &right.impl_id,
                coherence::canonical_type(&right.receiver),
            ))
    });
    Ok(DictionaryTable { witnesses })
}

fn append_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn append_usize(bytes: &mut Vec<u8>, value: usize) {
    append_u64(bytes, u64::try_from(value).unwrap_or(u64::MAX));
}

fn append_string(bytes: &mut Vec<u8>, value: &str) {
    append_usize(bytes, value.len());
    bytes.extend_from_slice(value.as_bytes());
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_hir as hir;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;
    use crate::coherence;
    use crate::constraints;
    use crate::solver;

    fn resolved_once(text: &str) -> ling_resolve::ResolvedProgram {
        let source = SourceFile::from_bytes(
            SourceId::new(0),
            "checked-core.ling",
            text.as_bytes().to_vec(),
        )
        .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = hir::lower(source.name(), &ast).expect("valid HIR");
        ling_resolve::resolve(vec![hir], "Main").expect("resolves")
    }

    fn selection_fixture() -> (
        ling_resolve::ResolvedProgram,
        CoherenceIndex,
        Vec<SolvedObligation>,
    ) {
        let program = resolved_once(concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n",
            "    label: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n",
            "    let label item = item.name\n\n",
            "let show requires { Renderable<Item> } value = value\n",
        ));
        let index = coherence::build_index(&program).expect("coherence");
        let obligations = constraints::collect_obligations(&program).expect("obligations");
        let selections =
            solver::solve_obligations(&program, &index, &obligations, &BTreeMap::new())
                .expect("selection");
        (program, index, selections)
    }

    #[test]
    fn lowers_identity_member_order_and_origin() {
        let (program, index, selections) = selection_fixture();
        let table =
            lower_dictionary_witnesses(&selections, &index, &program).expect("dictionary table");
        assert_eq!(table.witnesses().len(), 1);
        let witness = &table.witnesses()[0];
        assert_eq!(witness.trait_id.name, "Renderable");
        assert_eq!(witness.impl_id, index.impls[0].id);
        assert_eq!(witness.receiver, ConstraintType::Named("Item".to_owned()));
        assert_eq!(
            witness
                .members
                .iter()
                .map(|member| member.name.as_str())
                .collect::<Vec<_>>(),
            ["render", "label"]
        );
        assert_eq!(witness.origin.source_name, "checked-core.ling");
    }

    #[test]
    fn rejects_duplicate_or_unknown_selection_without_reselecting() {
        let (program, index, selections) = selection_fixture();
        let mut duplicate = selections.clone();
        duplicate.push(selections[0].clone());
        let errors =
            lower_dictionary_witnesses(&duplicate, &index, &program).expect_err("duplicate order");
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            DictionaryLoweringErrorKind::DuplicateObligationOrder { order: 0 }
        )));

        let mut unknown = selections;
        unknown[0].impl_id.ordinal = unknown[0].impl_id.ordinal.saturating_add(1);
        let errors =
            lower_dictionary_witnesses(&unknown, &index, &program).expect_err("unknown impl");
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            DictionaryLoweringErrorKind::UnknownImplementation { .. }
        )));
    }

    #[test]
    fn canonical_bytes_exclude_origin_presentation_and_remain_repeatable() {
        let (program, index, selections) = selection_fixture();
        let first = lower_dictionary_witnesses(&selections, &index, &program)
            .expect("first table")
            .canonical_bytes();
        let mut changed_origin = selections;
        changed_origin[0].origin.source_name = "different-host-path.ling".to_owned();
        let second = lower_dictionary_witnesses(&changed_origin, &index, &program)
            .expect("second table")
            .canonical_bytes();
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_receiver_and_member_identity_mismatches() {
        let (program, index, selections) = selection_fixture();
        let mut receiver = selections.clone();
        receiver[0].receiver = ConstraintType::Named("Other".to_owned());
        let errors =
            lower_dictionary_witnesses(&receiver, &index, &program).expect_err("receiver mismatch");
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            DictionaryLoweringErrorKind::ReceiverMismatch { .. }
        )));

        let mut members = selections;
        members[0].member_names.reverse();
        let errors =
            lower_dictionary_witnesses(&members, &index, &program).expect_err("member mismatch");
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            DictionaryLoweringErrorKind::MemberMismatch { .. }
        )));
    }
}
