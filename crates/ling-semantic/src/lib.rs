//! Deterministic checked-program snapshots and `ling.semantic/0.1` JSON.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use ling_effects::CheckedProgram;
use ling_hir as hir;
use ling_resolve::{
    DefinitionId, DefinitionKind, DefinitionOrigin, ExpressionKey, ModuleId, ReferenceTarget,
};
use serde::Serialize;

pub const SEMANTIC_SCHEMA: &str = "ling.semantic/0.1";
pub const LANGUAGE_VERSION: &str = "0.0.1-dev";

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BodyId(String);

impl BodyId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BodyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProgramId(String);

impl ProgramId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProgramId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticGraph {
    pub schema: &'static str,
    pub language_version: &'static str,
    pub unicode_version: String,
    pub program_id: String,
    pub entry_module: String,
    pub modules: Vec<SemanticModule>,
    pub definitions: Vec<SemanticDefinition>,
    pub references: Vec<SemanticReference>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticModule {
    pub name: String,
    pub explicit: bool,
    pub requires: Vec<String>,
    pub imports: Vec<SemanticImport>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticImport {
    pub alias: String,
    pub module: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticDefinition {
    pub definition_id: String,
    pub body_id: String,
    pub module: String,
    pub name: String,
    pub kind: String,
    pub origin: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub effects: Vec<String>,
    pub capabilities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticReference {
    pub module: String,
    pub reference: u32,
    pub target_kind: String,
    pub target: String,
}

#[derive(Clone, Debug)]
pub struct ProgramSnapshot {
    checked: CheckedProgram,
    graph: SemanticGraph,
    body_ids: BTreeMap<DefinitionId, BodyId>,
    program_id: ProgramId,
    json: String,
}

impl ProgramSnapshot {
    #[must_use]
    pub const fn checked(&self) -> &CheckedProgram {
        &self.checked
    }

    #[must_use]
    pub const fn graph(&self) -> &SemanticGraph {
        &self.graph
    }

    #[must_use]
    pub fn body_id(&self, definition: &DefinitionId) -> Option<&BodyId> {
        self.body_ids.get(definition)
    }

    #[must_use]
    pub const fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    #[must_use]
    pub fn json(&self) -> &str {
        &self.json
    }
}

#[derive(Debug)]
pub struct SnapshotError(serde_json::Error);

impl fmt::Display for SnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to serialize semantic snapshot: {}",
            self.0
        )
    }
}

impl Error for SnapshotError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

/// Builds a checked snapshot using versioned canonical binary hash inputs.
pub fn build(checked: CheckedProgram) -> Result<ProgramSnapshot, SnapshotError> {
    SnapshotBuilder::new(checked).build()
}

struct SnapshotBuilder {
    checked: CheckedProgram,
}

impl SnapshotBuilder {
    const fn new(checked: CheckedProgram) -> Self {
        Self { checked }
    }

    fn build(self) -> Result<ProgramSnapshot, SnapshotError> {
        let body_ids = self.body_ids();
        let program_id = self.program_id(&body_ids);
        let modules = self.modules();
        let definitions = self.definitions(&body_ids);
        let references = self.references();
        let graph = SemanticGraph {
            schema: SEMANTIC_SCHEMA,
            language_version: LANGUAGE_VERSION,
            unicode_version: ling_unicode::UNICODE_VERSION.to_string(),
            program_id: program_id.to_string(),
            entry_module: self
                .checked
                .typed()
                .resolved()
                .entry_module()
                .hir
                .module
                .name
                .normalized(),
            modules,
            definitions,
            references,
        };
        let json = serde_json::to_string(&graph).map_err(SnapshotError)?;
        Ok(ProgramSnapshot {
            checked: self.checked,
            graph,
            body_ids,
            program_id,
            json,
        })
    }

    fn body_ids(&self) -> BTreeMap<DefinitionId, BodyId> {
        self.checked
            .typed()
            .resolved()
            .definitions()
            .keys()
            .map(|definition| {
                let mut encoder = Encoder::new("ling.body-id/v1");
                encoder.string(LANGUAGE_VERSION);
                encoder.string(SEMANTIC_SCHEMA);
                self.encode_definition(definition, &mut encoder);
                (definition.clone(), BodyId(hash(encoder.finish())))
            })
            .collect()
    }

    fn program_id(&self, body_ids: &BTreeMap<DefinitionId, BodyId>) -> ProgramId {
        let mut encoder = Encoder::new("ling.program-id/v1");
        encoder.string(LANGUAGE_VERSION);
        encoder.string(SEMANTIC_SCHEMA);
        encoder.string(&ling_unicode::UNICODE_VERSION.to_string());
        encoder.u32(u32::try_from(body_ids.len()).unwrap_or(u32::MAX));
        for (definition, body) in body_ids {
            encoder.string(definition.as_str());
            encoder.string(body.as_str());
        }
        ProgramId(hash(encoder.finish()))
    }

    fn modules(&self) -> Vec<SemanticModule> {
        let resolved = self.checked.typed().resolved();
        let mut modules = resolved
            .modules()
            .iter()
            .map(|module| {
                let mut requires = module
                    .hir
                    .module
                    .requires
                    .iter()
                    .map(hir::QualifiedName::normalized)
                    .collect::<Vec<_>>();
                requires.sort();
                let mut imports = module
                    .imports
                    .iter()
                    .filter_map(|(alias, target)| {
                        resolved.module(*target).map(|target| SemanticImport {
                            alias: alias.clone(),
                            module: target.hir.module.name.normalized(),
                        })
                    })
                    .collect::<Vec<_>>();
                imports.sort_by(|left, right| {
                    (&left.alias, &left.module).cmp(&(&right.alias, &right.module))
                });
                SemanticModule {
                    name: module.hir.module.name.normalized(),
                    explicit: module.hir.module.explicit,
                    requires,
                    imports,
                }
            })
            .collect::<Vec<_>>();
        modules.sort_by(|left, right| left.name.cmp(&right.name));
        modules
    }

    fn definitions(&self, body_ids: &BTreeMap<DefinitionId, BodyId>) -> Vec<SemanticDefinition> {
        let typed = self.checked.typed();
        typed
            .resolved()
            .definitions()
            .iter()
            .map(|(id, definition)| {
                let effects = self
                    .checked
                    .definition_effect(id)
                    .map_or_else(Vec::new, ling_effects::EffectRow::names);
                let capabilities = effects
                    .iter()
                    .filter(|effect| effect.as_str() == "Console.Write")
                    .map(|_| "Console.Write".to_owned())
                    .collect();
                let type_name = typed.definition_type(id).map_or_else(
                    || "<untyped>".to_owned(),
                    |value| typed.arena().display(value),
                );
                SemanticDefinition {
                    definition_id: id.to_string(),
                    body_id: body_ids[id].to_string(),
                    module: definition.module_name.clone(),
                    name: definition.name.clone(),
                    kind: definition_kind(definition.kind).to_owned(),
                    origin: match definition.origin {
                        DefinitionOrigin::User { .. } => "user".to_owned(),
                        DefinitionOrigin::Builtin(_) => "builtin".to_owned(),
                    },
                    type_name,
                    effects,
                    capabilities,
                }
            })
            .collect()
    }

    fn references(&self) -> Vec<SemanticReference> {
        let resolved = self.checked.typed().resolved();
        resolved
            .references()
            .iter()
            .filter_map(|(key, target)| {
                resolved.module(key.module()).map(|module| {
                    let (target_kind, target) = match target {
                        ReferenceTarget::Definition(definition) => {
                            ("definition".to_owned(), definition.to_string())
                        }
                        ReferenceTarget::Binding(binding) => (
                            "binding".to_owned(),
                            format!("local:{}", binding.local().get()),
                        ),
                    };
                    SemanticReference {
                        module: module.hir.module.name.normalized(),
                        reference: key.local().get(),
                        target_kind,
                        target,
                    }
                })
            })
            .collect()
    }

    fn encode_definition(&self, definition: &DefinitionId, encoder: &mut Encoder) {
        let typed = self.checked.typed();
        let info = &typed.resolved().definitions()[definition];
        encoder.string(definition_kind(info.kind));
        if let Some(type_id) = typed.definition_type(definition) {
            encoder.string(&typed.arena().display(type_id));
        } else {
            encoder.string("<untyped>");
        }
        let effects = self
            .checked
            .definition_effect(definition)
            .map_or_else(Vec::new, ling_effects::EffectRow::names);
        encoder.strings(&effects);
        let capabilities = effects
            .iter()
            .filter(|effect| effect.as_str() == "Console.Write")
            .cloned()
            .collect::<Vec<_>>();
        encoder.strings(&capabilities);
        match info.origin {
            DefinitionOrigin::Builtin(builtin) => {
                encoder.u8(0);
                encoder.string(builtin.qualified_name());
            }
            DefinitionOrigin::User { module } => {
                encoder.u8(1);
                let resolved_module = typed
                    .resolved()
                    .module(module)
                    .expect("definition module exists");
                if let Some(value) = resolved_module
                    .hir
                    .definitions
                    .iter()
                    .find(|value| value.name.normalized == info.name)
                {
                    encoder.u8(0);
                    encoder.bool(value.recursive);
                    encoder.bool(value.mutable);
                    encoder.u32(u32::try_from(value.parameters.len()).unwrap_or(u32::MAX));
                    for pattern in &value.parameters {
                        encode_pattern(pattern, encoder);
                    }
                    self.encode_expression(module, &value.value, encoder);
                } else if let Some(value) = resolved_module
                    .hir
                    .types
                    .iter()
                    .find(|value| value.name.normalized == info.name)
                {
                    encoder.u8(1);
                    encode_type_declaration(value, encoder);
                } else {
                    encoder.u8(2);
                    encoder.string(&info.name);
                }
            }
        }
    }

    fn encode_expression(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
        encoder: &mut Encoder,
    ) {
        let typed = self.checked.typed();
        if let Some(type_id) = typed.expression_type(ExpressionKey::new(module, expression.id)) {
            encoder.string(&typed.arena().display(type_id));
        } else {
            encoder.string("<untyped>");
        }
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                encoder.u8(0);
                encoder.u32(u32::try_from(elements.len()).unwrap_or(u32::MAX));
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            encoder.u8(0);
                            encoder.u32(binding.id.get());
                            encoder.bool(binding.recursive);
                            encoder.bool(binding.mutable);
                            encoder
                                .u32(u32::try_from(binding.parameters.len()).unwrap_or(u32::MAX));
                            for pattern in &binding.parameters {
                                encode_pattern(pattern, encoder);
                            }
                            self.encode_expression(module, &binding.value, encoder);
                        }
                        hir::SequenceElement::Expression(expression) => {
                            encoder.u8(1);
                            self.encode_expression(module, expression, encoder);
                        }
                    }
                }
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                encoder.u8(1);
                self.encode_expression(module, condition, encoder);
                self.encode_expression(module, then_branch, encoder);
                self.encode_expression(module, else_branch, encoder);
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                encoder.u8(2);
                self.encode_expression(module, scrutinee, encoder);
                encoder.u32(u32::try_from(cases.len()).unwrap_or(u32::MAX));
                for case in cases {
                    encode_pattern(&case.pattern, encoder);
                    encoder.bool(case.guard.is_some());
                    if let Some(guard) = &case.guard {
                        self.encode_expression(module, guard, encoder);
                    }
                    self.encode_expression(module, &case.body, encoder);
                }
            }
            hir::ExpressionKind::Assignment { place, value } => {
                encoder.u8(3);
                self.encode_reference(module, place.root_reference, encoder);
                encoder.u32(u32::try_from(place.fields.len()).unwrap_or(u32::MAX));
                for field in &place.fields {
                    encoder.string(&field.normalized);
                }
                self.encode_expression(module, value, encoder);
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                encoder.u8(4);
                self.encode_expression(module, function, encoder);
                encoder.u32(u32::try_from(arguments.len()).unwrap_or(u32::MAX));
                for argument in arguments {
                    self.encode_expression(module, argument, encoder);
                }
            }
            hir::ExpressionKind::Projection {
                reference,
                target,
                field,
            } => {
                encoder.u8(5);
                if typed.resolved().reference(module, *reference).is_some() {
                    encoder.u8(0);
                    self.encode_reference(module, *reference, encoder);
                } else {
                    encoder.u8(1);
                    self.encode_expression(module, target, encoder);
                    encoder.string(&field.normalized);
                }
            }
            hir::ExpressionKind::Name { reference, .. } => {
                encoder.u8(6);
                self.encode_reference(module, *reference, encoder);
            }
            hir::ExpressionKind::Binary {
                operator,
                left,
                right,
            } => {
                encoder.u8(7);
                encoder.u8(binary_tag(*operator));
                self.encode_expression(module, left, encoder);
                self.encode_expression(module, right, encoder);
            }
            hir::ExpressionKind::Unary { operator, operand } => {
                encoder.u8(8);
                encoder.u8(match operator {
                    hir::UnaryOperator::Positive => 0,
                    hir::UnaryOperator::Negative => 1,
                });
                self.encode_expression(module, operand, encoder);
            }
            hir::ExpressionKind::Literal(literal) => {
                encoder.u8(9);
                match literal {
                    hir::Literal::Integer { .. } => {
                        encoder.u8(0);
                        let bytes = typed
                            .integer(ExpressionKey::new(module, expression.id))
                            .map(num_bigint_bytes)
                            .unwrap_or_default();
                        encoder.bytes(&bytes);
                    }
                    hir::Literal::Float(value) => {
                        encoder.u8(1);
                        encoder.bytes(
                            &value
                                .parse::<f64>()
                                .unwrap_or(f64::NAN)
                                .to_bits()
                                .to_be_bytes(),
                        );
                    }
                    hir::Literal::Text(value) => {
                        encoder.u8(2);
                        encoder.string(value);
                    }
                    hir::Literal::Boolean(value) => {
                        encoder.u8(3);
                        encoder.bool(*value);
                    }
                }
            }
            hir::ExpressionKind::Unit => encoder.u8(10),
            hir::ExpressionKind::Tuple(elements) => {
                encoder.u8(11);
                encoder.u32(u32::try_from(elements.len()).unwrap_or(u32::MAX));
                for element in elements {
                    self.encode_expression(module, element, encoder);
                }
            }
            hir::ExpressionKind::Record(fields) => {
                encoder.u8(12);
                self.encode_record_fields(module, fields, encoder);
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                encoder.u8(13);
                self.encode_expression(module, base, encoder);
                self.encode_record_fields(module, fields, encoder);
            }
            hir::ExpressionKind::List(elements) => {
                encoder.u8(14);
                encoder.u32(u32::try_from(elements.len()).unwrap_or(u32::MAX));
                for element in elements {
                    self.encode_expression(module, element, encoder);
                }
            }
        }
    }

    fn encode_record_fields(
        &self,
        module: ModuleId,
        fields: &[hir::RecordField],
        encoder: &mut Encoder,
    ) {
        encoder.u32(u32::try_from(fields.len()).unwrap_or(u32::MAX));
        for field in fields {
            encoder.string(&field.name.normalized);
            self.encode_expression(module, &field.value, encoder);
        }
    }

    fn encode_reference(
        &self,
        module: ModuleId,
        reference: hir::ReferenceId,
        encoder: &mut Encoder,
    ) {
        match self.checked.typed().resolved().reference(module, reference) {
            Some(ReferenceTarget::Definition(definition)) => {
                encoder.u8(0);
                encoder.string(definition.as_str());
            }
            Some(ReferenceTarget::Binding(binding)) => {
                encoder.u8(1);
                encoder.u32(binding.local().get());
            }
            None => encoder.u8(u8::MAX),
        }
    }
}

fn definition_kind(kind: DefinitionKind) -> &'static str {
    match kind {
        DefinitionKind::Value => "value",
        DefinitionKind::Type => "type",
        DefinitionKind::Constructor => "constructor",
        DefinitionKind::Builtin => "builtin",
    }
}

fn encode_pattern(pattern: &hir::Pattern, encoder: &mut Encoder) {
    match &pattern.kind {
        hir::PatternKind::Binding { id, .. } => {
            encoder.u8(0);
            encoder.u32(id.get());
        }
        hir::PatternKind::Unit => encoder.u8(1),
        hir::PatternKind::Literal(literal) => {
            encoder.u8(2);
            encode_literal_without_context(literal, encoder);
        }
        hir::PatternKind::Tuple(elements) => {
            encoder.u8(3);
            encoder.u32(u32::try_from(elements.len()).unwrap_or(u32::MAX));
            for element in elements {
                encode_pattern(element, encoder);
            }
        }
        hir::PatternKind::Constructor { name, arguments } => {
            encoder.u8(4);
            encoder.string(&name.normalized);
            encoder.u32(u32::try_from(arguments.len()).unwrap_or(u32::MAX));
            for argument in arguments {
                encode_pattern(argument, encoder);
            }
        }
    }
}

fn encode_literal_without_context(literal: &hir::Literal, encoder: &mut Encoder) {
    match literal {
        hir::Literal::Integer { radix, digits } => {
            encoder.u8(0);
            encoder.u32(*radix);
            encoder.string(&digits.replace('_', ""));
        }
        hir::Literal::Float(value) => {
            encoder.u8(1);
            encoder.bytes(
                &value
                    .parse::<f64>()
                    .unwrap_or(f64::NAN)
                    .to_bits()
                    .to_be_bytes(),
            );
        }
        hir::Literal::Text(value) => {
            encoder.u8(2);
            encoder.string(value);
        }
        hir::Literal::Boolean(value) => {
            encoder.u8(3);
            encoder.bool(*value);
        }
    }
}

fn encode_type_declaration(declaration: &hir::TypeDeclaration, encoder: &mut Encoder) {
    encoder.u32(u32::try_from(declaration.parameters.len()).unwrap_or(u32::MAX));
    match &declaration.definition {
        hir::TypeDefinition::Record(fields) => {
            encoder.u8(0);
            encoder.u32(u32::try_from(fields.len()).unwrap_or(u32::MAX));
            for field in fields {
                encoder.string(&field.name.normalized);
                encoder.bool(field.mutable);
                encode_type_syntax(&field.field_type, encoder);
            }
        }
        hir::TypeDefinition::Variant(cases) => {
            encoder.u8(1);
            encoder.u32(u32::try_from(cases.len()).unwrap_or(u32::MAX));
            for case in cases {
                encoder.string(&case.name.normalized);
                encoder.bool(case.payload.is_some());
                if let Some(payload) = &case.payload {
                    encode_type_syntax(payload, encoder);
                }
            }
        }
        hir::TypeDefinition::Alias(alias) => {
            encoder.u8(2);
            encode_type_syntax(alias, encoder);
        }
    }
}

fn encode_type_syntax(syntax: &hir::TypeSyntax, encoder: &mut Encoder) {
    encoder.u32(u32::try_from(syntax.atoms.len()).unwrap_or(u32::MAX));
    for atom in &syntax.atoms {
        match atom {
            hir::TypeAtom::Name(name) => {
                encoder.u8(0);
                encoder.string(&name.normalized);
            }
            hir::TypeAtom::Variable(_) => encoder.u8(1),
            hir::TypeAtom::Arrow => encoder.u8(2),
            hir::TypeAtom::Product => encoder.u8(3),
            hir::TypeAtom::LeftParen => encoder.u8(4),
            hir::TypeAtom::RightParen => encoder.u8(5),
            hir::TypeAtom::LeftAngle => encoder.u8(6),
            hir::TypeAtom::RightAngle => encoder.u8(7),
            hir::TypeAtom::Comma => encoder.u8(8),
            hir::TypeAtom::Dot => encoder.u8(9),
        }
    }
}

const fn binary_tag(operator: hir::BinaryOperator) -> u8 {
    match operator {
        hir::BinaryOperator::Equal => 0,
        hir::BinaryOperator::NotEqual => 1,
        hir::BinaryOperator::Less => 2,
        hir::BinaryOperator::LessEqual => 3,
        hir::BinaryOperator::Greater => 4,
        hir::BinaryOperator::GreaterEqual => 5,
        hir::BinaryOperator::Add => 6,
        hir::BinaryOperator::Subtract => 7,
        hir::BinaryOperator::Multiply => 8,
        hir::BinaryOperator::Divide => 9,
        hir::BinaryOperator::Remainder => 10,
    }
}

fn num_bigint_bytes(value: &num_bigint::BigInt) -> Vec<u8> {
    let (sign, bytes) = value.to_bytes_be();
    let mut output = vec![match sign {
        num_bigint::Sign::Minus => 0,
        num_bigint::Sign::NoSign => 1,
        num_bigint::Sign::Plus => 2,
    }];
    output.extend_from_slice(&bytes);
    output
}

fn hash(bytes: Vec<u8>) -> String {
    format!("experimental:blake3:{}", blake3::hash(&bytes).to_hex())
}

struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    fn new(domain: &str) -> Self {
        let mut encoder = Self { bytes: Vec::new() };
        encoder.string(domain);
        encoder
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }

    fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn bool(&mut self, value: bool) {
        self.u8(u8::from(value));
    }

    fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn bytes(&mut self, value: &[u8]) {
        self.u32(u32::try_from(value.len()).unwrap_or(u32::MAX));
        self.bytes.extend_from_slice(value);
    }

    fn string(&mut self, value: &str) {
        self.bytes(value.as_bytes());
    }

    fn strings(&mut self, values: &[String]) {
        self.u32(u32::try_from(values.len()).unwrap_or(u32::MAX));
        for value in values {
            self.string(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;

    fn snapshot(text: &str) -> ProgramSnapshot {
        let source =
            SourceFile::from_bytes(SourceId::new(0), "test.ling", text.as_bytes().to_vec())
                .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = hir::lower(source.name(), &ast).expect("valid HIR");
        let resolved = ling_resolve::resolve(vec![hir], "Main").expect("resolves");
        let typed = ling_types::check(resolved).expect("type-checks");
        let checked = ling_effects::check(typed).expect("effects check");
        build(checked).expect("snapshot builds")
    }

    #[test]
    fn hello_snapshot_is_byte_deterministic() {
        let source =
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"你好，零\"\n";
        let first = snapshot(source);
        let second = snapshot(source);
        assert_eq!(first.json(), second.json());
        assert_eq!(first.program_id(), second.program_id());
        assert!(first.json().contains("\"schema\":\"ling.semantic/0.1\""));
    }

    #[test]
    fn whitespace_does_not_change_body_or_program_ids() {
        let compact = snapshot(
            "module Main\n    requires Console.Write\nlet main () = Console.write \"x\"\n",
        );
        let spaced = snapshot(
            "module Main\n    requires Console.Write\n\n\nlet main () =\n    Console.write \"x\"\n",
        );
        assert_eq!(compact.program_id(), spaced.program_id());
    }
}
