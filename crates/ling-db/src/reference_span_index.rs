use std::cmp::Ordering;
use std::collections::BTreeMap;

use ling_hir::{Expression, ExpressionKind, Program, SequenceElement};
use ling_resolve::{ModuleId, ReferenceKey, ResolvedProgram};
use ling_source::Span;

/// Syntax-directed relation of one resolver-owned expression reference.
///
/// `Type` and `Implementation` are reserved by the accepted references
/// taxonomy, but Seed's resolver does not assign reference identities to those
/// surfaces yet. They therefore cannot be fabricated by this index.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ResolvedReferenceRelation {
    Read,
    Write,
    Call,
    Type,
    Implementation,
}

/// One resolver reference paired with its exact original UTF-8 identifier span.
///
/// The span is copied from HIR name metadata. It is not converted to an LSP
/// position and does not authorize an edit or a rename target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedReferenceSpan {
    module_id: u32,
    reference_id: u32,
    source_name: String,
    span: Span,
    relation: ResolvedReferenceRelation,
}

impl ResolvedReferenceSpan {
    #[must_use]
    pub const fn module_id(&self) -> u32 {
        self.module_id
    }

    #[must_use]
    pub const fn reference_id(&self) -> u32 {
        self.reference_id
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
    pub const fn relation(&self) -> ResolvedReferenceRelation {
        self.relation
    }
}

/// Deterministic, resolver-filtered source-span observations for references.
///
/// This is an in-process compiler value. It contains only logical source
/// names, resolver reference identities, and original-byte spans; it has no
/// URI/version, position encoding, edit, cache, persistence, or protocol state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedReferenceSpanIndex {
    entries: Box<[ResolvedReferenceSpan]>,
}

impl ResolvedReferenceSpanIndex {
    #[must_use]
    pub fn entries(&self) -> &[ResolvedReferenceSpan] {
        &self.entries
    }

    #[must_use]
    pub fn reference(&self, module_id: u32, reference_id: u32) -> Option<&ResolvedReferenceSpan> {
        self.entries
            .iter()
            .find(|entry| entry.module_id == module_id && entry.reference_id == reference_id)
    }

    #[must_use]
    pub fn source_entries(&self, source_name: &str) -> Vec<&ResolvedReferenceSpan> {
        self.entries
            .iter()
            .filter(|entry| entry.source_name == source_name)
            .collect()
    }

    pub(crate) fn from_resolved(resolved: &ResolvedProgram) -> Self {
        let mut spans = BTreeMap::<ReferenceKey, (Span, ResolvedReferenceRelation)>::new();
        for module in resolved.modules() {
            collect_program(module.id, &module.hir, &mut spans);
        }

        let mut entries = spans
            .into_iter()
            .filter_map(|(key, (span, relation))| {
                if !resolved.references().contains_key(&key) {
                    return None;
                }
                let source_name = resolved.module(key.module())?.hir.source_name.clone();
                Some(ResolvedReferenceSpan {
                    module_id: key.module().get(),
                    reference_id: key.local().get(),
                    source_name,
                    span,
                    relation,
                })
            })
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

fn collect_program(
    module: ModuleId,
    program: &Program,
    spans: &mut BTreeMap<ReferenceKey, (Span, ResolvedReferenceRelation)>,
) {
    for definition in &program.definitions {
        collect_expression(
            module,
            &definition.value,
            ResolvedReferenceRelation::Read,
            spans,
        );
    }
    for task in &program.tasks {
        collect_expression(module, &task.body, ResolvedReferenceRelation::Read, spans);
    }
    for implementation in &program.impls {
        for definition in &implementation.members {
            collect_expression(
                module,
                &definition.value,
                ResolvedReferenceRelation::Read,
                spans,
            );
        }
    }
}

fn collect_expression(
    module: ModuleId,
    expression: &Expression,
    root_relation: ResolvedReferenceRelation,
    spans: &mut BTreeMap<ReferenceKey, (Span, ResolvedReferenceRelation)>,
) {
    match &expression.kind {
        ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    SequenceElement::Let(binding) => {
                        collect_expression(
                            module,
                            &binding.value,
                            ResolvedReferenceRelation::Read,
                            spans,
                        );
                    }
                    SequenceElement::LetAwait(binding) => {
                        collect_expression(
                            module,
                            &binding.call,
                            ResolvedReferenceRelation::Call,
                            spans,
                        );
                    }
                    SequenceElement::Expression(expression) => {
                        collect_expression(
                            module,
                            expression,
                            ResolvedReferenceRelation::Read,
                            spans,
                        );
                    }
                }
            }
        }
        ExpressionKind::TaskScope { body, .. } => {
            collect_expression(module, body, ResolvedReferenceRelation::Read, spans);
        }
        ExpressionKind::TaskSpawn { call, .. } => {
            collect_expression(module, call, ResolvedReferenceRelation::Call, spans);
        }
        ExpressionKind::TaskAwait { handle, .. } => {
            collect_expression(module, handle, ResolvedReferenceRelation::Read, spans);
        }
        ExpressionKind::TaskReturn { value, .. } => {
            collect_expression(module, value, ResolvedReferenceRelation::Read, spans);
        }
        ExpressionKind::Handle { body, clauses } => {
            collect_expression(module, body, ResolvedReferenceRelation::Read, spans);
            for clause in clauses {
                collect_expression(module, &clause.body, ResolvedReferenceRelation::Read, spans);
            }
        }
        ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_expression(module, condition, ResolvedReferenceRelation::Read, spans);
            collect_expression(module, then_branch, ResolvedReferenceRelation::Read, spans);
            collect_expression(module, else_branch, ResolvedReferenceRelation::Read, spans);
        }
        ExpressionKind::Match { scrutinee, cases } => {
            collect_expression(module, scrutinee, ResolvedReferenceRelation::Read, spans);
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_expression(module, guard, ResolvedReferenceRelation::Read, spans);
                }
                collect_expression(module, &case.body, ResolvedReferenceRelation::Read, spans);
            }
        }
        ExpressionKind::Assignment { place, value } => {
            spans
                .entry(ReferenceKey::new(module, place.root_reference))
                .or_insert((place.root.span, ResolvedReferenceRelation::Write));
            collect_expression(module, value, ResolvedReferenceRelation::Read, spans);
        }
        ExpressionKind::Application {
            function,
            arguments,
        } => {
            collect_expression(module, function, ResolvedReferenceRelation::Call, spans);
            for argument in arguments {
                collect_expression(module, argument, ResolvedReferenceRelation::Read, spans);
            }
        }
        ExpressionKind::Projection {
            reference,
            target,
            field,
        } => {
            spans
                .entry(ReferenceKey::new(module, *reference))
                .or_insert((field.span, root_relation));
            collect_expression(module, target, ResolvedReferenceRelation::Read, spans);
        }
        ExpressionKind::Name { reference, name } => {
            spans
                .entry(ReferenceKey::new(module, *reference))
                .or_insert((name.span, root_relation));
        }
        ExpressionKind::Binary { left, right, .. } => {
            collect_expression(module, left, ResolvedReferenceRelation::Read, spans);
            collect_expression(module, right, ResolvedReferenceRelation::Read, spans);
        }
        ExpressionKind::Unary { operand, .. } => {
            collect_expression(module, operand, ResolvedReferenceRelation::Read, spans);
        }
        ExpressionKind::Tuple(elements) | ExpressionKind::List(elements) => {
            for element in elements {
                collect_expression(module, element, ResolvedReferenceRelation::Read, spans);
            }
        }
        ExpressionKind::Record(fields) => {
            for field in fields {
                collect_expression(module, &field.value, ResolvedReferenceRelation::Read, spans);
            }
        }
        ExpressionKind::RecordUpdate { base, fields } => {
            collect_expression(module, base, ResolvedReferenceRelation::Read, spans);
            for field in fields {
                collect_expression(module, &field.value, ResolvedReferenceRelation::Read, spans);
            }
        }
        ExpressionKind::Literal(_) | ExpressionKind::Unit => {}
    }
}

fn entry_order(left: &ResolvedReferenceSpan, right: &ResolvedReferenceSpan) -> Ordering {
    left.source_name
        .cmp(&right.source_name)
        .then_with(|| left.module_id.cmp(&right.module_id))
        .then_with(|| left.reference_id.cmp(&right.reference_id))
        .then_with(|| {
            (
                left.span.source().get(),
                left.span.start().get(),
                left.span.end().get(),
            )
                .cmp(&(
                    right.span.source().get(),
                    right.span.start().get(),
                    right.span.end().get(),
                ))
        })
}

#[cfg(test)]
mod tests {
    use ling_ast::lower;
    use ling_hir::lower as lower_hir;
    use ling_resolve::resolve;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::{lex, parse};

    use super::*;

    fn resolved(source_name: &str, source: &str) -> ResolvedProgram {
        let source =
            SourceFile::from_bytes(SourceId::new(0), source_name, source.as_bytes().to_vec())
                .expect("valid source");
        let lexed = lex(&source);
        assert!(lexed.errors().is_empty(), "{:?}", lexed.errors());
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower(&source, &parsed).expect("valid AST");
        let hir = lower_hir(source.name().to_owned(), &ast).expect("valid HIR");
        resolve(vec![hir], "Main").expect("valid resolution")
    }

    #[test]
    fn records_exact_name_spans_after_bom_and_crlf() {
        let resolved = resolved(
            "unicode/Main.ling",
            "\u{feff}module Main\r\n\r\nlet helper = 1\r\n\r\nlet main () = helper\r\n",
        );
        let index = ResolvedReferenceSpanIndex::from_resolved(&resolved);

        assert_eq!(index.source_entries("unicode/Main.ling").len(), 1);
        let entry = &index.entries()[0];
        let bytes =
            "\u{feff}module Main\r\n\r\nlet helper = 1\r\n\r\nlet main () = helper\r\n".as_bytes();
        let start = entry.span.start().get() as usize;
        let end = entry.span.end().get() as usize;
        assert_eq!(&bytes[start..end], b"helper");
        assert_eq!(entry.source_name(), "unicode/Main.ling");
    }

    #[test]
    fn construction_is_repeatable_and_lookup_is_source_scoped() {
        let resolved = resolved(
            "Main.ling",
            "module Main\n\nlet helper = 1\n\nlet main () =\n    let local = helper\n    local\n",
        );
        let first = ResolvedReferenceSpanIndex::from_resolved(&resolved);
        let second = ResolvedReferenceSpanIndex::from_resolved(&resolved);

        assert_eq!(first, second);
        assert_eq!(first.entries().len(), 2);
        assert!(
            first
                .reference(0, first.entries()[0].reference_id())
                .is_some()
        );
        assert!(first.source_entries("missing.ling").is_empty());
    }

    #[test]
    fn ignores_hir_reference_ids_not_present_in_resolver() {
        let resolved = resolved("Main.ling", "let main () = ()\n");
        let index = ResolvedReferenceSpanIndex::from_resolved(&resolved);

        assert!(index.entries().is_empty());
    }
}
