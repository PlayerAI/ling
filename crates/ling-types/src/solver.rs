//! Internal RFC-0005 first-slice obligation solver.
//!
//! This module deliberately stops at immutable selection evidence. It does not
//! alter inference substitutions or perform runtime dispatch; RFC-0021 lowers
//! its selections into the checked dictionary boundary.

use std::collections::{BTreeMap, BTreeSet};

use ling_resolve::ResolvedProgram;
use ling_source::Span;

use crate::coherence::{self, CoherenceIndex, ImplId, TraitId};
use crate::constraints::{ConstraintType, Obligation, ObligationOrigin};

/// Accepted RFC-0005 maximum nested Trait-obligation depth.
pub const MAX_NESTED_TRAIT_OBLIGATIONS: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SolvedObligation {
    pub(crate) obligation_order: usize,
    pub(crate) trait_id: TraitId,
    pub(crate) impl_id: ImplId,
    pub(crate) receiver: ConstraintType,
    pub(crate) member_names: Vec<String>,
    pub(crate) origin: ObligationOrigin,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SolverError {
    pub(crate) source_name: String,
    pub(crate) span: Span,
    pub(crate) kind: SolverErrorKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SolverErrorKind {
    UnknownTrait {
        name: String,
    },
    InvalidReceiverArity {
        actual: usize,
    },
    Unsatisfied {
        trait_name: String,
        receiver: String,
    },
    Ambiguous {
        trait_name: String,
        receiver: String,
        candidates: Vec<ImplId>,
    },
    Cycle {
        trait_name: String,
        receiver: String,
        depth: usize,
    },
    DepthLimit {
        trait_name: String,
        receiver: String,
        depth: usize,
    },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ActiveObligation {
    trait_id: TraitId,
    receiver_head: String,
    canonical_arguments: String,
}

/// Selects legal first-slice implementations and recursively checks their
/// internal requirements. The requirement map is an internal seam for later
/// HIR integration and for proving cycle/depth behavior without exposing a
/// public protocol.
pub(crate) fn solve_obligations(
    resolved: &ResolvedProgram,
    index: &CoherenceIndex,
    obligations: &[Obligation],
    requirements: &BTreeMap<ImplId, Vec<Obligation>>,
) -> Result<Vec<SolvedObligation>, Vec<SolverError>> {
    match solve_obligations_with_cancellation(resolved, index, obligations, requirements, &|| false)
    {
        Ok(selections) => Ok(selections),
        Err(SolveFailure::Errors(errors)) => Err(errors),
        Err(SolveFailure::Cancelled) => unreachable!("the default probe never cancels"),
    }
}

pub(crate) enum SolveFailure {
    Cancelled,
    Errors(Vec<SolverError>),
}

pub(crate) fn solve_obligations_with_cancellation(
    resolved: &ResolvedProgram,
    index: &CoherenceIndex,
    obligations: &[Obligation],
    requirements: &BTreeMap<ImplId, Vec<Obligation>>,
    cancelled: &dyn Fn() -> bool,
) -> Result<Vec<SolvedObligation>, SolveFailure> {
    let mut solver = Solver {
        resolved,
        index,
        requirements,
        active: BTreeSet::new(),
        selections: Vec::new(),
        errors: Vec::new(),
        cancelled,
        cancellation_observed: false,
    };
    for obligation in obligations {
        solver.solve_one(obligation, 0);
        if solver.cancellation_observed {
            return Err(SolveFailure::Cancelled);
        }
    }
    if solver.errors.is_empty() {
        Ok(solver.selections)
    } else {
        solver.errors.sort_by(|left, right| {
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
        Err(SolveFailure::Errors(solver.errors))
    }
}

struct Solver<'a> {
    resolved: &'a ResolvedProgram,
    index: &'a CoherenceIndex,
    requirements: &'a BTreeMap<ImplId, Vec<Obligation>>,
    active: BTreeSet<ActiveObligation>,
    selections: Vec<SolvedObligation>,
    errors: Vec<SolverError>,
    cancelled: &'a dyn Fn() -> bool,
    cancellation_observed: bool,
}

impl Solver<'_> {
    fn solve_one(&mut self, obligation: &Obligation, depth: usize) {
        if (self.cancelled)() {
            self.cancellation_observed = true;
            return;
        }
        let Some(module) = self.resolved.module(obligation.module) else {
            self.errors.push(SolverError {
                source_name: obligation.origin.source_name.clone(),
                span: obligation.origin.span,
                kind: SolverErrorKind::UnknownTrait {
                    name: obligation.trait_name.clone(),
                },
            });
            return;
        };
        let Some(trait_id) =
            coherence::resolve_trait_id_name(module, &obligation.trait_name, &self.index.traits)
        else {
            self.errors.push(SolverError {
                source_name: obligation.origin.source_name.clone(),
                span: obligation.origin.span,
                kind: SolverErrorKind::UnknownTrait {
                    name: obligation.trait_name.clone(),
                },
            });
            return;
        };
        if obligation.arguments.len() != 1 {
            self.errors.push(SolverError {
                source_name: obligation.origin.source_name.clone(),
                span: obligation.origin.span,
                kind: SolverErrorKind::InvalidReceiverArity {
                    actual: obligation.arguments.len(),
                },
            });
            return;
        }

        let receiver = obligation.arguments[0].clone();
        let receiver_text = coherence::canonical_type(&receiver);
        let receiver_head = coherence::receiver_head(&receiver).unwrap_or_default();
        let active = ActiveObligation {
            trait_id: trait_id.clone(),
            receiver_head,
            canonical_arguments: obligation
                .arguments
                .iter()
                .map(coherence::canonical_type)
                .collect::<Vec<_>>()
                .join(","),
        };
        if !self.active.insert(active.clone()) {
            self.errors.push(SolverError {
                source_name: obligation.origin.source_name.clone(),
                span: obligation.origin.span,
                kind: SolverErrorKind::Cycle {
                    trait_name: trait_id.name.clone(),
                    receiver: receiver_text,
                    depth,
                },
            });
            return;
        }

        if depth >= MAX_NESTED_TRAIT_OBLIGATIONS {
            self.errors.push(SolverError {
                source_name: obligation.origin.source_name.clone(),
                span: obligation.origin.span,
                kind: SolverErrorKind::DepthLimit {
                    trait_name: trait_id.name.clone(),
                    receiver: receiver_text,
                    depth,
                },
            });
            self.active.remove(&active);
            return;
        }

        let mut candidates = if coherence::contains_variable(&receiver) {
            Vec::new()
        } else {
            self.index
                .impls
                .iter()
                .filter(|implementation| {
                    implementation.trait_id == trait_id
                        && coherence::canonical_type(&implementation.receiver)
                            == coherence::canonical_type(&receiver)
                })
                .collect::<Vec<_>>()
        };
        candidates.sort_by_key(|candidate| candidate.id.clone());

        let Some(candidate) = (match candidates.as_slice() {
            [] => {
                self.errors.push(SolverError {
                    source_name: obligation.origin.source_name.clone(),
                    span: obligation.origin.span,
                    kind: SolverErrorKind::Unsatisfied {
                        trait_name: trait_id.name.clone(),
                        receiver: receiver_text.clone(),
                    },
                });
                None
            }
            [candidate] => Some(*candidate),
            candidates => {
                self.errors.push(SolverError {
                    source_name: obligation.origin.source_name.clone(),
                    span: obligation.origin.span,
                    kind: SolverErrorKind::Ambiguous {
                        trait_name: trait_id.name.clone(),
                        receiver: receiver_text.clone(),
                        candidates: candidates.iter().map(|value| value.id.clone()).collect(),
                    },
                });
                None
            }
        }) else {
            self.active.remove(&active);
            return;
        };

        self.selections.push(SolvedObligation {
            obligation_order: obligation.source_order,
            trait_id,
            impl_id: candidate.id.clone(),
            receiver,
            member_names: candidate.member_names.clone(),
            origin: obligation.origin.clone(),
        });

        if let Some(nested) = self.requirements.get(&candidate.id) {
            for requirement in nested {
                self.solve_one(requirement, depth.saturating_add(1));
                if self.cancellation_observed {
                    break;
                }
            }
        }
        self.active.remove(&active);
    }
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

    fn resolved_once(text: &str) -> ResolvedProgram {
        resolved_named("solver.ling", text)
    }

    fn resolved_named(name: &str, text: &str) -> ResolvedProgram {
        let source = SourceFile::from_bytes(SourceId::new(0), name, text.as_bytes().to_vec())
            .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = hir::lower(source.name(), &ast).expect("valid HIR");
        ling_resolve::resolve(vec![hir], "Main").expect("resolves")
    }

    fn fixture() -> ResolvedProgram {
        resolved_once(concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Item = { name: Text }\n",
            "type Other = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "let show requires { Renderable<Item> } value = value\n",
        ))
    }

    #[test]
    fn selects_one_concrete_candidate_with_ordered_members() {
        let program = fixture();
        let index = coherence::build_index(&program).expect("coherence");
        let obligations = constraints::collect_obligations(&program).expect("obligations");
        let selections = solve_obligations(&program, &index, &obligations, &BTreeMap::new())
            .expect("unique candidate");
        assert_eq!(selections.len(), 1);
        assert_eq!(selections[0].trait_id.name, "Renderable");
        assert_eq!(
            selections[0].receiver,
            ConstraintType::Named("Item".to_owned())
        );
        assert_eq!(selections[0].member_names, ["render"]);
        assert_eq!(selections[0].obligation_order, 0);
    }

    #[test]
    fn distinguishes_unsatisfied_variable_and_ambiguous_candidates() {
        let program = fixture();
        let index = coherence::build_index(&program).expect("coherence");
        let mut obligations = constraints::collect_obligations(&program).expect("obligations");
        obligations[0].arguments = vec![ConstraintType::Named("Other".to_owned())];
        let errors = solve_obligations(&program, &index, &obligations, &BTreeMap::new())
            .expect_err("Other has no implementation");
        assert!(matches!(
            errors[0].kind,
            SolverErrorKind::Unsatisfied { ref receiver, .. } if receiver == "Other"
        ));

        obligations[0].arguments = vec![ConstraintType::Variable("a".to_owned())];
        let errors = solve_obligations(&program, &index, &obligations, &BTreeMap::new())
            .expect_err("generic receiver cannot select a concrete impl");
        assert!(matches!(
            errors[0].kind,
            SolverErrorKind::Unsatisfied { ref receiver, .. } if receiver == "'a"
        ));

        let mut ambiguous = index.clone();
        let mut duplicate = ambiguous.impls[0].clone();
        duplicate.id.ordinal = duplicate.id.ordinal.saturating_add(1);
        ambiguous.impls.push(duplicate);
        let errors = solve_obligations(
            &program,
            &ambiguous,
            &constraints::collect_obligations(&program).expect("obligations"),
            &BTreeMap::new(),
        )
        .expect_err("two candidates are ambiguous");
        assert!(matches!(
            errors[0].kind,
            SolverErrorKind::Ambiguous { ref candidates, .. } if candidates.len() == 2
        ));
    }

    #[test]
    fn rejects_active_cycles_and_the_bounded_depth_limit() {
        let program = fixture();
        let index = coherence::build_index(&program).expect("coherence");
        let obligations = constraints::collect_obligations(&program).expect("obligations");
        let root = obligations[0].clone();
        let candidate = index.impls[0].id.clone();
        let mut requirements = BTreeMap::new();
        requirements.insert(candidate.clone(), vec![root.clone()]);
        let errors = solve_obligations(&program, &index, &obligations, &requirements)
            .expect_err("recursive requirement must be rejected");
        assert!(
            errors
                .iter()
                .any(|error| matches!(error.kind, SolverErrorKind::Cycle { depth: 1, .. }))
        );

        let mut text = String::from("module Main\n\n");
        for index in 0..=64 {
            text.push_str(&format!("trait T{index}<'a> =\n    run: 'a -> Text\n\n"));
            text.push_str(&format!("type A{index} = {{ value: Text }}\n\n"));
            text.push_str(&format!(
                "impl T{index} A{index} =\n    let run value = value.value\n\n"
            ));
        }
        text.push_str("let root requires { T0<A0> } value = value\n");
        let deep_program = resolved_once(&text);
        let deep_index = coherence::build_index(&deep_program).expect("deep coherence");
        let deep_obligations =
            constraints::collect_obligations(&deep_program).expect("deep obligations");
        let root = deep_obligations[0].clone();
        let mut requirements = BTreeMap::new();
        for index in 0..64 {
            let mut nested = root.clone();
            nested.trait_name = format!("T{}", index + 1);
            nested.arguments = vec![ConstraintType::Named(format!("A{}", index + 1))];
            nested.source_order = index + 1;
            let implementation = deep_index
                .impls
                .iter()
                .find(|implementation| implementation.trait_id.name == format!("T{index}"))
                .expect("deep implementation");
            requirements.insert(implementation.id.clone(), vec![nested]);
        }
        let errors =
            solve_obligations(&deep_program, &deep_index, &deep_obligations, &requirements)
                .expect_err("64 nested obligations are bounded");
        assert!(
            errors
                .iter()
                .any(|error| matches!(error.kind, SolverErrorKind::DepthLimit { depth: 64, .. }))
        );
    }

    #[test]
    fn reports_invalid_receiver_arity_without_selecting_an_impl() {
        let program = fixture();
        let index = coherence::build_index(&program).expect("coherence");
        let mut obligations = constraints::collect_obligations(&program).expect("obligations");
        obligations[0].arguments = vec![
            ConstraintType::Named("Item".to_owned()),
            ConstraintType::Named("Other".to_owned()),
        ];
        let errors = solve_obligations(&program, &index, &obligations, &BTreeMap::new())
            .expect_err("only one receiver argument is supported");
        assert!(matches!(
            errors[0].kind,
            SolverErrorKind::InvalidReceiverArity { actual: 2 }
        ));
    }

    #[test]
    fn bounded_termination_projection_ignores_source_evidence() {
        let text = concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "let show requires { Renderable<Item> } value = value\n",
        );
        let project = |source_name: &str| {
            let program = resolved_named(source_name, text);
            let index = coherence::build_index(&program).expect("coherence");
            let obligations = constraints::collect_obligations(&program).expect("obligations");
            solve_obligations(&program, &index, &obligations, &BTreeMap::new())
                .expect("bounded selection")
                .into_iter()
                .map(|selection| {
                    (
                        selection.obligation_order,
                        selection.trait_id.name,
                        selection.impl_id.ordinal,
                        selection.receiver,
                        selection.member_names,
                    )
                })
                .collect::<Vec<_>>()
        };

        let first = project("first.ling");
        let second = project("second.ling");
        assert_eq!(first, second);
    }
}
