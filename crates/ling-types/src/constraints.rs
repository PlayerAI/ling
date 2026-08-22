//! Internal first-slice collection of RFC-0005 trait obligations.
//!
//! This module deliberately stops before name resolution, coherence, solving,
//! or dictionary construction. The normal type-checking entry point uses the
//! collected result to reject only unresolved generic obligations; concrete
//! member-call obligations are collected by the RFC-0021 checker boundary.

use ling_hir as hir;
use ling_resolve::{BindingKey, DefinitionId, ModuleId, ResolvedProgram};
use ling_source::Span;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ConstraintType {
    Named(String),
    Variable(String),
    Applied { name: String, arguments: Vec<Self> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ObligationOwner {
    Definition(DefinitionId),
    Binding(BindingKey),
    ImplMember { trait_name: String, member: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ObligationOrigin {
    pub(crate) source_name: String,
    pub(crate) span: Span,
    pub(crate) parent: Option<Box<Self>>,
}

impl ObligationOrigin {
    fn direct(source_name: &str, span: Span) -> Self {
        Self {
            source_name: source_name.to_owned(),
            span,
            parent: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Obligation {
    pub(crate) module: ModuleId,
    pub(crate) owner: ObligationOwner,
    pub(crate) trait_name: String,
    pub(crate) arguments: Vec<ConstraintType>,
    pub(crate) origin: ObligationOrigin,
    pub(crate) source_order: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ConstraintCollectionError {
    pub(crate) source_name: String,
    pub(crate) span: Span,
    pub(crate) reason: &'static str,
}

/// Collects normalized obligations in deterministic source order.
///
/// The order is the order already established by the resolver: resolved
/// modules, HIR definitions, expression children, and constraint vectors are
/// all walked as ordered slices.  No map iteration participates in this
/// result, and this function never chooses an implementation candidate.
pub(crate) fn collect_obligations(
    resolved: &ResolvedProgram,
) -> Result<Vec<Obligation>, Vec<ConstraintCollectionError>> {
    let mut collector = Collector {
        obligations: Vec::new(),
        errors: Vec::new(),
        next_source_order: 0,
    };

    for module in resolved.modules() {
        for definition in &module.hir.definitions {
            let Some(owner) = resolved
                .definition_id(module.id, &definition.name.normalized)
                .cloned()
            else {
                collector.errors.push(ConstraintCollectionError {
                    source_name: module.hir.source_name.clone(),
                    span: definition.span,
                    reason: "top-level definition is missing from the resolved index",
                });
                continue;
            };
            collector.collect_constraints(
                module.id,
                &module.hir.source_name,
                ObligationOwner::Definition(owner),
                &definition.constraints,
            );
            collector.visit_expression(module.id, &module.hir.source_name, &definition.value);
        }

        for implementation in &module.hir.impls {
            let trait_name = implementation.trait_name.normalized();
            for member in &implementation.members {
                let owner = ObligationOwner::ImplMember {
                    trait_name: trait_name.clone(),
                    member: member.name.normalized.clone(),
                };
                collector.collect_constraints(
                    module.id,
                    &module.hir.source_name,
                    owner,
                    &member.constraints,
                );
                collector.visit_expression(module.id, &module.hir.source_name, &member.value);
            }
        }
    }

    if collector.errors.is_empty() {
        collector.obligations.sort_by_key(|obligation| {
            (
                obligation.module.get(),
                obligation.origin.span.source().get(),
                obligation.origin.span.start().get(),
                obligation.source_order,
            )
        });
        for (source_order, obligation) in collector.obligations.iter_mut().enumerate() {
            obligation.source_order = source_order;
        }
        Ok(collector.obligations)
    } else {
        collector.errors.sort_by_key(|error| {
            (
                error.source_name.clone(),
                error.span.source().get(),
                error.span.start().get(),
                error.reason,
            )
        });
        Err(collector.errors)
    }
}

struct Collector {
    obligations: Vec<Obligation>,
    errors: Vec<ConstraintCollectionError>,
    next_source_order: usize,
}

impl Collector {
    fn collect_constraints(
        &mut self,
        module: ModuleId,
        source_name: &str,
        owner: ObligationOwner,
        constraints: &[hir::TypeSyntax],
    ) {
        for constraint in constraints {
            let source_order = self.next_source_order;
            self.next_source_order = self.next_source_order.saturating_add(1);
            match parse_obligation(constraint) {
                Ok((trait_name, arguments)) => self.obligations.push(Obligation {
                    module,
                    owner: owner.clone(),
                    trait_name,
                    arguments,
                    origin: ObligationOrigin::direct(source_name, constraint.span),
                    source_order,
                }),
                Err(reason) => self.errors.push(ConstraintCollectionError {
                    source_name: source_name.to_owned(),
                    span: constraint.span,
                    reason,
                }),
            }
        }
    }

    fn visit_expression(
        &mut self,
        module: ModuleId,
        source_name: &str,
        expression: &hir::Expression,
    ) {
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            self.collect_constraints(
                                module,
                                source_name,
                                ObligationOwner::Binding(BindingKey::new(module, binding.id)),
                                &binding.constraints,
                            );
                            self.visit_expression(module, source_name, &binding.value);
                        }
                        hir::SequenceElement::Expression(expression) => {
                            self.visit_expression(module, source_name, expression);
                        }
                    }
                }
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(module, source_name, condition);
                self.visit_expression(module, source_name, then_branch);
                self.visit_expression(module, source_name, else_branch);
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                self.visit_expression(module, source_name, scrutinee);
                for case in cases {
                    if let Some(guard) = &case.guard {
                        self.visit_expression(module, source_name, guard);
                    }
                    self.visit_expression(module, source_name, &case.body);
                }
            }
            hir::ExpressionKind::Assignment { value, .. } => {
                self.visit_expression(module, source_name, value);
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                self.visit_expression(module, source_name, function);
                for argument in arguments {
                    self.visit_expression(module, source_name, argument);
                }
            }
            hir::ExpressionKind::Projection { target, .. } => {
                self.visit_expression(module, source_name, target);
            }
            hir::ExpressionKind::Binary { left, right, .. } => {
                self.visit_expression(module, source_name, left);
                self.visit_expression(module, source_name, right);
            }
            hir::ExpressionKind::Unary { operand, .. } => {
                self.visit_expression(module, source_name, operand);
            }
            hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
                for element in elements {
                    self.visit_expression(module, source_name, element);
                }
            }
            hir::ExpressionKind::Record(fields) => {
                for field in fields {
                    self.visit_expression(module, source_name, &field.value);
                }
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                self.visit_expression(module, source_name, base);
                for field in fields {
                    self.visit_expression(module, source_name, &field.value);
                }
            }
            hir::ExpressionKind::Name { .. }
            | hir::ExpressionKind::Literal(_)
            | hir::ExpressionKind::Unit => {}
        }
    }
}

fn parse_obligation(
    syntax: &hir::TypeSyntax,
) -> Result<(String, Vec<ConstraintType>), &'static str> {
    let mut parser = TypeParser {
        atoms: &syntax.atoms,
        position: 0,
    };
    let trait_name = parser.qualified_name()?;
    let arguments = if parser.peek_is(&hir::TypeAtom::LeftAngle) {
        parser.type_arguments()?
    } else {
        Vec::new()
    };
    if parser.position != parser.atoms.len() {
        return Err("obligation contains trailing or unsupported type syntax");
    }
    Ok((trait_name, arguments))
}

pub(crate) fn parse_type_expression(
    syntax: &hir::TypeSyntax,
) -> Result<ConstraintType, &'static str> {
    let mut parser = TypeParser {
        atoms: &syntax.atoms,
        position: 0,
    };
    let value = parser.type_primary()?;
    if parser.position != parser.atoms.len() {
        return Err("type expression contains trailing or unsupported syntax");
    }
    Ok(value)
}

struct TypeParser<'a> {
    atoms: &'a [hir::TypeAtom],
    position: usize,
}

impl TypeParser<'_> {
    fn peek_is(&self, expected: &hir::TypeAtom) -> bool {
        self.atoms
            .get(self.position)
            .is_some_and(|actual| actual == expected)
    }

    fn qualified_name(&mut self) -> Result<String, &'static str> {
        let mut segments = Vec::new();
        let Some(hir::TypeAtom::Name(name)) = self.atoms.get(self.position) else {
            return Err("obligation head must start with a nominal name");
        };
        segments.push(name.normalized.clone());
        self.position += 1;
        while self.peek_is(&hir::TypeAtom::Dot) {
            self.position += 1;
            let Some(hir::TypeAtom::Name(name)) = self.atoms.get(self.position) else {
                return Err("qualified obligation name must have a name after each dot");
            };
            segments.push(name.normalized.clone());
            self.position += 1;
        }
        Ok(segments.join("."))
    }

    fn type_arguments(&mut self) -> Result<Vec<ConstraintType>, &'static str> {
        if !self.peek_is(&hir::TypeAtom::LeftAngle) {
            return Err("type argument list must start with <");
        }
        self.position += 1;
        if self.peek_is(&hir::TypeAtom::RightAngle) {
            return Err("type argument list cannot be empty");
        }

        let mut arguments = Vec::new();
        loop {
            arguments.push(self.type_primary()?);
            if self.peek_is(&hir::TypeAtom::Comma) {
                self.position += 1;
                if self.peek_is(&hir::TypeAtom::RightAngle) {
                    return Err("type argument list cannot have a trailing comma");
                }
                continue;
            }
            if self.peek_is(&hir::TypeAtom::RightAngle) {
                self.position += 1;
                return Ok(arguments);
            }
            return Err("type argument list requires comma or >");
        }
    }

    fn type_primary(&mut self) -> Result<ConstraintType, &'static str> {
        let Some(atom) = self.atoms.get(self.position) else {
            return Err("type argument is missing");
        };
        match atom {
            hir::TypeAtom::Name(_) => {
                let name = self.qualified_name()?;
                if self.peek_is(&hir::TypeAtom::LeftAngle) {
                    Ok(ConstraintType::Applied {
                        name,
                        arguments: self.type_arguments()?,
                    })
                } else {
                    Ok(ConstraintType::Named(name))
                }
            }
            hir::TypeAtom::Variable(name) => {
                self.position += 1;
                if self.peek_is(&hir::TypeAtom::LeftAngle) {
                    return Err("type variables cannot have type arguments");
                }
                Ok(ConstraintType::Variable(name.normalized.clone()))
            }
            hir::TypeAtom::LeftParen => {
                self.position += 1;
                let value = self.type_primary()?;
                if !self.peek_is(&hir::TypeAtom::RightParen) {
                    return Err("parenthesized obligation type must contain one type");
                }
                self.position += 1;
                Ok(value)
            }
            hir::TypeAtom::Arrow | hir::TypeAtom::Product => {
                Err("function and product types are not first-slice obligation arguments")
            }
            hir::TypeAtom::RightAngle
            | hir::TypeAtom::Comma
            | hir::TypeAtom::Dot
            | hir::TypeAtom::RightParen
            | hir::TypeAtom::LeftAngle => Err("malformed obligation type argument"),
        }
    }
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_source::{ByteOffset, SourceFile, SourceId, Span};
    use ling_syntax::parse;

    use super::*;

    fn resolved(text: &str) -> ResolvedProgram {
        let source = SourceFile::from_bytes(
            SourceId::new(0),
            "constraints.ling",
            text.as_bytes().to_vec(),
        )
        .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = hir::lower(source.name(), &ast).expect("valid HIR");
        ling_resolve::resolve(vec![hir], "Main").expect("resolves")
    }

    fn name(value: &str, span: Span) -> hir::Name {
        hir::Name {
            span,
            source: value.to_owned(),
            normalized: value.to_owned(),
            skeleton: value.to_owned(),
            scripts: vec!["Latin".to_owned()],
            suspicious_mixed_script: false,
        }
    }

    #[test]
    fn collects_normalized_nested_obligations_and_local_bindings() {
        let program = resolved(concat!(
            "module Main\n\n",
            "let show<'a> requires { UI.Renderable<'a>, Comparable<List<'a>> } value =\n",
            "    let local<'b> requires { Local<'b> } item = item\n",
            "    value\n",
        ));
        let obligations = collect_obligations(&program).expect("valid obligations");
        assert_eq!(obligations.len(), 3);
        assert_eq!(obligations[0].trait_name, "UI.Renderable");
        assert_eq!(obligations[1].trait_name, "Comparable");
        assert_eq!(obligations[2].trait_name, "Local");
        assert_eq!(obligations[0].source_order, 0);
        assert_eq!(obligations[1].source_order, 1);
        assert_eq!(obligations[2].source_order, 2);
        assert!(matches!(
            &obligations[0].arguments[0],
            ConstraintType::Variable(name) if name == "a"
        ));
        assert!(matches!(
            &obligations[1].arguments[0],
            ConstraintType::Applied { name, arguments }
                if name == "List"
                    && arguments == &vec![ConstraintType::Variable("a".to_owned())]
        ));
        assert!(matches!(
            obligations[0].owner,
            ObligationOwner::Definition(_)
        ));
        assert!(matches!(obligations[2].owner, ObligationOwner::Binding(_)));
        assert_eq!(
            obligations[0].origin.span.start().get(),
            u32::try_from("module Main\n\nlet show<'a> requires { ".len()).expect("offset fits")
        );
        assert!(obligations[0].origin.parent.is_none());
    }

    #[test]
    fn collection_is_repeatable_and_does_not_depend_on_map_iteration() {
        let program = resolved(concat!(
            "module Main\n\n",
            "let first<'a> requires { Renderable<'a> } value = value\n",
            "let second<'a> requires { Renderable<'a> } value = value\n",
        ));
        assert_eq!(collect_obligations(&program), collect_obligations(&program));
    }

    #[test]
    fn rejects_malformed_first_slice_argument_syntax() {
        let span = Span::new(SourceId::new(0), ByteOffset::new(0), ByteOffset::new(8))
            .expect("valid span");
        let syntax = hir::TypeSyntax {
            span,
            atoms: vec![
                hir::TypeAtom::Name(name("Renderable", span)),
                hir::TypeAtom::LeftAngle,
                hir::TypeAtom::Arrow,
                hir::TypeAtom::RightAngle,
            ],
        };
        assert_eq!(
            parse_obligation(&syntax),
            Err("function and product types are not first-slice obligation arguments")
        );
    }

    #[test]
    fn check_keeps_unresolved_generic_obligations_out_of_typed_core() {
        let program = resolved(concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "let show<'a> requires { Renderable<'a> } value = value\n",
        ));
        let errors = crate::check(program).expect_err("Trait support is not executable yet");
        assert_eq!(errors.len(), 1);
        assert!(
            errors
                .iter()
                .all(|error| matches!(error.kind, crate::TypeErrorKind::UnsupportedTypeSyntax))
        );
    }
}
