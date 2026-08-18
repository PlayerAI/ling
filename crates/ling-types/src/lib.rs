//! Hindley-Milner-style Seed type inference and assignment validation.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_hir as hir;
use ling_resolve::{
    BindingKey, Builtin, DefinitionId, ExpressionKey, ModuleId, ReferenceTarget, ResolvedProgram,
};
use ling_source::Span;
use num_bigint::BigInt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeId(u32);

impl TypeId {
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    Unit,
    Bool,
    Int,
    Float64,
    Text,
    Tuple(Vec<TypeId>),
    List(TypeId),
    Function {
        parameters: Vec<TypeId>,
        result: TypeId,
    },
    NominalRecord {
        definition: DefinitionId,
        arguments: Vec<TypeId>,
    },
    NominalVariant {
        definition: DefinitionId,
        arguments: Vec<TypeId>,
    },
    Variable(u32),
    Error,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TypeArena {
    types: Vec<Type>,
}

impl TypeArena {
    #[must_use]
    pub fn get(&self, id: TypeId) -> &Type {
        &self.types[id.0 as usize]
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.types.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    #[must_use]
    pub fn display(&self, id: TypeId) -> String {
        match self.get(id) {
            Type::Unit => "Unit".to_owned(),
            Type::Bool => "Bool".to_owned(),
            Type::Int => "Int".to_owned(),
            Type::Float64 => "f64".to_owned(),
            Type::Text => "Text".to_owned(),
            Type::Tuple(elements) => format!(
                "({})",
                elements
                    .iter()
                    .map(|element| self.display(*element))
                    .collect::<Vec<_>>()
                    .join(" * ")
            ),
            Type::List(element) => format!("List<{}>", self.display(*element)),
            Type::Function { parameters, result } => {
                let mut parts = parameters
                    .iter()
                    .map(|parameter| self.display(*parameter))
                    .collect::<Vec<_>>();
                parts.push(self.display(*result));
                parts.join(" -> ")
            }
            Type::NominalRecord { definition, .. } | Type::NominalVariant { definition, .. } => {
                definition.to_string()
            }
            Type::Variable(variable) => format!("'t{variable}"),
            Type::Error => "<error>".to_owned(),
        }
    }

    fn intern(&mut self, value: Type, index: &mut BTreeMap<Type, TypeId>) -> TypeId {
        if let Some(id) = index.get(&value) {
            return *id;
        }
        let id = TypeId(u32::try_from(self.types.len()).unwrap_or(u32::MAX));
        self.types.push(value.clone());
        index.insert(value, id);
        id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordFieldInfo {
    pub name: String,
    pub mutable: bool,
    pub field_type: TypeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordInfo {
    pub definition: DefinitionId,
    pub name: String,
    pub fields: Vec<RecordFieldInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantCaseInfo {
    pub name: String,
    pub payload: Option<TypeId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantInfo {
    pub definition: DefinitionId,
    pub name: String,
    pub cases: Vec<VariantCaseInfo>,
}

#[derive(Clone, Debug)]
pub struct TypedProgram {
    resolved: ResolvedProgram,
    arena: TypeArena,
    expression_types: BTreeMap<ExpressionKey, TypeId>,
    definition_types: BTreeMap<DefinitionId, TypeId>,
    binding_types: BTreeMap<BindingKey, TypeId>,
    place_types: BTreeMap<ExpressionKey, TypeId>,
    integers: BTreeMap<ExpressionKey, BigInt>,
    records: BTreeMap<DefinitionId, RecordInfo>,
    variants: BTreeMap<DefinitionId, VariantInfo>,
}

impl TypedProgram {
    #[must_use]
    pub const fn resolved(&self) -> &ResolvedProgram {
        &self.resolved
    }

    #[must_use]
    pub const fn arena(&self) -> &TypeArena {
        &self.arena
    }

    #[must_use]
    pub fn expression_type(&self, key: ExpressionKey) -> Option<TypeId> {
        self.expression_types.get(&key).copied()
    }

    #[must_use]
    pub fn expression_types(&self) -> &BTreeMap<ExpressionKey, TypeId> {
        &self.expression_types
    }

    #[must_use]
    pub fn definition_type(&self, id: &DefinitionId) -> Option<TypeId> {
        self.definition_types.get(id).copied()
    }

    #[must_use]
    pub fn definition_types(&self) -> &BTreeMap<DefinitionId, TypeId> {
        &self.definition_types
    }

    #[must_use]
    pub fn binding_type(&self, key: BindingKey) -> Option<TypeId> {
        self.binding_types.get(&key).copied()
    }

    #[must_use]
    pub fn place_type(&self, key: ExpressionKey) -> Option<TypeId> {
        self.place_types.get(&key).copied()
    }

    #[must_use]
    pub fn integer(&self, key: ExpressionKey) -> Option<&BigInt> {
        self.integers.get(&key)
    }

    #[must_use]
    pub fn records(&self) -> &BTreeMap<DefinitionId, RecordInfo> {
        &self.records
    }

    #[must_use]
    pub fn variants(&self) -> &BTreeMap<DefinitionId, VariantInfo> {
        &self.variants
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeError {
    pub kind: TypeErrorKind,
    pub source_name: String,
    pub span: Span,
    pub restriction_reason: Option<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeErrorKind {
    Mismatch { expected: String, actual: String },
    InfiniteType,
    NotCallable { actual: String },
    Arity { expected: usize, actual: usize },
    UnknownField { field: String },
    AmbiguousRecord,
    InvalidInteger,
    InvalidAssignment { reason: &'static str },
    UnsupportedTypeSyntax,
}

impl TypeError {
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        let (code, zh, en) = match &self.kind {
            TypeErrorKind::Mismatch { expected, actual } => (
                codes::TYPE_MISMATCH,
                format!("类型不匹配：期望 {expected}，实际 {actual}"),
                format!("type mismatch: expected {expected}, found {actual}"),
            ),
            TypeErrorKind::InfiniteType => (
                codes::INFINITE_TYPE,
                "类型推导产生无限类型".to_owned(),
                "type inference produced an infinite type".to_owned(),
            ),
            TypeErrorKind::NotCallable { actual } => (
                codes::TYPE_MISMATCH,
                format!("类型 {actual} 不是函数"),
                format!("type {actual} is not callable"),
            ),
            TypeErrorKind::Arity { expected, actual } => (
                codes::CALL_ARITY,
                format!("参数数量不匹配：期望 {expected}，实际 {actual}"),
                format!("argument count mismatch: expected {expected}, found {actual}"),
            ),
            TypeErrorKind::UnknownField { field } => (
                codes::UNKNOWN_FIELD,
                format!("record 中不存在字段“{field}”"),
                format!("record has no field named `{field}`"),
            ),
            TypeErrorKind::AmbiguousRecord => (
                codes::AMBIGUOUS_RECORD,
                "无法从字段唯一确定 nominal record 类型".to_owned(),
                "record fields do not identify one nominal record type".to_owned(),
            ),
            TypeErrorKind::InvalidInteger => (
                codes::TYPE_MISMATCH,
                "整数 literal 无法解析".to_owned(),
                "integer literal could not be parsed".to_owned(),
            ),
            TypeErrorKind::InvalidAssignment { reason } => (
                codes::INVALID_ASSIGNMENT,
                format!("赋值左侧不可修改：{reason}"),
                format!("assignment target is not mutable: {reason}"),
            ),
            TypeErrorKind::UnsupportedTypeSyntax => (
                codes::TYPE_MISMATCH,
                "当前 Seed 类型语法不支持此注解".to_owned(),
                "this annotation is not supported by the current Seed type grammar".to_owned(),
            ),
        };
        let mut diagnostic = Diagnostic::new(code, Severity::Error, zh, en)
            .with_primary_span(DiagnosticSpan::new(&self.source_name, self.span));
        if let Some(reason) = self.restriction_reason {
            diagnostic = diagnostic
                .with_fact("generalization", "restricted")
                .with_fact("restriction_reason", reason);
        }
        diagnostic
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.kind)
    }
}

impl Error for TypeError {}

/// Infers all Seed types and validates assignment places.
pub fn check(resolved: ResolvedProgram) -> Result<TypedProgram, Vec<TypeError>> {
    Inferencer::new(resolved).run()
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum InferType {
    Unit,
    Bool,
    Int,
    Float64,
    Text,
    Tuple(Vec<Self>),
    List(Box<Self>),
    Function {
        parameters: Vec<Self>,
        result: Box<Self>,
    },
    Record {
        definition: DefinitionId,
        arguments: Vec<Self>,
    },
    Variant {
        definition: DefinitionId,
        arguments: Vec<Self>,
    },
    Variable(u32),
    Error,
}

#[derive(Clone, Debug)]
struct Scheme {
    quantified: BTreeSet<u32>,
    value: InferType,
}

impl Scheme {
    fn mono(value: InferType) -> Self {
        Self {
            quantified: BTreeSet::new(),
            value,
        }
    }
}

#[derive(Clone)]
struct InternalRecord {
    name: String,
    fields: BTreeMap<String, (bool, InferType)>,
}

#[derive(Clone)]
struct InternalVariant {
    name: String,
    cases: BTreeMap<String, Option<InferType>>,
}

struct Inferencer {
    resolved: ResolvedProgram,
    next_variable: u32,
    substitutions: BTreeMap<u32, InferType>,
    definitions: BTreeMap<DefinitionId, Scheme>,
    bindings: BTreeMap<BindingKey, Scheme>,
    inferred_expressions: BTreeMap<ExpressionKey, InferType>,
    inferred_definitions: BTreeMap<DefinitionId, InferType>,
    inferred_bindings: BTreeMap<BindingKey, InferType>,
    inferred_places: BTreeMap<ExpressionKey, InferType>,
    integers: BTreeMap<ExpressionKey, BigInt>,
    records: BTreeMap<DefinitionId, InternalRecord>,
    variants: BTreeMap<DefinitionId, InternalVariant>,
    errors: Vec<TypeError>,
    current_module: ModuleId,
}

impl Inferencer {
    fn new(resolved: ResolvedProgram) -> Self {
        let current_module = resolved.entry();
        Self {
            resolved,
            next_variable: 0,
            substitutions: BTreeMap::new(),
            definitions: BTreeMap::new(),
            bindings: BTreeMap::new(),
            inferred_expressions: BTreeMap::new(),
            inferred_definitions: BTreeMap::new(),
            inferred_bindings: BTreeMap::new(),
            inferred_places: BTreeMap::new(),
            integers: BTreeMap::new(),
            records: BTreeMap::new(),
            variants: BTreeMap::new(),
            errors: Vec::new(),
            current_module,
        }
    }

    fn run(mut self) -> Result<TypedProgram, Vec<TypeError>> {
        self.seed_builtins();
        self.index_nominal_types();
        self.predeclare_values();
        self.infer_definitions();

        if !self.errors.is_empty() {
            self.errors.sort_by(|left, right| {
                (
                    &left.source_name,
                    left.span.start(),
                    format!("{:?}", left.kind),
                )
                    .cmp(&(
                        &right.source_name,
                        right.span.start(),
                        format!("{:?}", right.kind),
                    ))
            });
            return Err(self.errors);
        }

        let mut arena = TypeArena::default();
        let mut index = BTreeMap::new();
        let expression_types = self
            .inferred_expressions
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let value = self.apply(value);
                (key, intern_type(&value, &mut arena, &mut index))
            })
            .collect();
        let definition_types = self
            .inferred_definitions
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let value = self.apply(value);
                (key, intern_type(&value, &mut arena, &mut index))
            })
            .collect();
        let binding_types = self
            .inferred_bindings
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let value = self.apply(value);
                (key, intern_type(&value, &mut arena, &mut index))
            })
            .collect();
        let place_types = self
            .inferred_places
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let value = self.apply(value);
                (key, intern_type(&value, &mut arena, &mut index))
            })
            .collect();
        let records = self
            .records
            .iter()
            .map(|(id, record)| {
                let fields = record
                    .fields
                    .iter()
                    .map(|(name, (mutable, field_type))| RecordFieldInfo {
                        name: name.clone(),
                        mutable: *mutable,
                        field_type: intern_type(
                            &self.apply(field_type.clone()),
                            &mut arena,
                            &mut index,
                        ),
                    })
                    .collect();
                (
                    id.clone(),
                    RecordInfo {
                        definition: id.clone(),
                        name: record.name.clone(),
                        fields,
                    },
                )
            })
            .collect();
        let variants = self
            .variants
            .iter()
            .map(|(id, variant)| {
                let cases = variant
                    .cases
                    .iter()
                    .map(|(name, payload)| VariantCaseInfo {
                        name: name.clone(),
                        payload: payload.clone().map(|payload| {
                            intern_type(&self.apply(payload), &mut arena, &mut index)
                        }),
                    })
                    .collect();
                (
                    id.clone(),
                    VariantInfo {
                        definition: id.clone(),
                        name: variant.name.clone(),
                        cases,
                    },
                )
            })
            .collect();

        Ok(TypedProgram {
            resolved: self.resolved,
            arena,
            expression_types,
            definition_types,
            binding_types,
            place_types,
            integers: self.integers,
            records,
            variants,
        })
    }

    fn seed_builtins(&mut self) {
        let insert = |this: &mut Self, builtin: Builtin, scheme: Scheme| {
            let id = this.resolved.builtin_id(builtin).clone();
            this.definitions.insert(id.clone(), scheme.clone());
            this.inferred_definitions.insert(id, scheme.value);
        };
        insert(
            self,
            Builtin::ConsoleWrite,
            Scheme::mono(function(vec![InferType::Text], InferType::Unit)),
        );
        insert(
            self,
            Builtin::TextFormat,
            Scheme::mono(function(
                vec![InferType::Text, InferType::Int],
                InferType::Text,
            )),
        );
        for builtin in [Builtin::Max, Builtin::Min] {
            insert(
                self,
                builtin,
                Scheme::mono(function(
                    vec![InferType::Int, InferType::Int],
                    InferType::Int,
                )),
            );
        }
        let a = self.fresh_variable_id();
        let b = self.fresh_variable_id();
        insert(
            self,
            Builtin::Map,
            Scheme {
                quantified: BTreeSet::from([a, b]),
                value: function(
                    vec![
                        function(vec![InferType::Variable(a)], InferType::Variable(b)),
                        InferType::List(Box::new(InferType::Variable(a))),
                    ],
                    InferType::List(Box::new(InferType::Variable(b))),
                ),
            },
        );
        insert(
            self,
            Builtin::Sum,
            Scheme::mono(function(
                vec![InferType::List(Box::new(InferType::Int))],
                InferType::Int,
            )),
        );
    }

    fn index_nominal_types(&mut self) {
        let modules = self.resolved.modules().to_vec();
        for module in &modules {
            self.current_module = module.id;
            for declaration in &module.hir.types {
                let Some(id) = self
                    .resolved
                    .definition_id(module.id, &declaration.name.normalized)
                    .cloned()
                else {
                    continue;
                };
                match &declaration.definition {
                    hir::TypeDefinition::Record(fields) => {
                        let fields = fields
                            .iter()
                            .map(|field| {
                                (
                                    field.name.normalized.clone(),
                                    (
                                        field.mutable,
                                        self.type_syntax(module.id, &field.field_type),
                                    ),
                                )
                            })
                            .collect();
                        self.records.insert(
                            id.clone(),
                            InternalRecord {
                                name: declaration.name.normalized.clone(),
                                fields,
                            },
                        );
                        let nominal = InferType::Record {
                            definition: id.clone(),
                            arguments: Vec::new(),
                        };
                        self.definitions
                            .insert(id.clone(), Scheme::mono(nominal.clone()));
                        self.inferred_definitions.insert(id, nominal);
                    }
                    hir::TypeDefinition::Variant(cases) => {
                        let cases_by_name = cases
                            .iter()
                            .map(|case| {
                                (
                                    case.name.normalized.clone(),
                                    case.payload
                                        .as_ref()
                                        .map(|payload| self.type_syntax(module.id, payload)),
                                )
                            })
                            .collect::<BTreeMap<_, _>>();
                        self.variants.insert(
                            id.clone(),
                            InternalVariant {
                                name: declaration.name.normalized.clone(),
                                cases: cases_by_name.clone(),
                            },
                        );
                        let nominal = InferType::Variant {
                            definition: id.clone(),
                            arguments: Vec::new(),
                        };
                        self.definitions
                            .insert(id.clone(), Scheme::mono(nominal.clone()));
                        self.inferred_definitions
                            .insert(id.clone(), nominal.clone());
                        for case in cases {
                            if let Some(constructor_id) = self
                                .resolved
                                .definition_id(module.id, &case.name.normalized)
                                .cloned()
                            {
                                let constructor_type = case.payload.as_ref().map_or_else(
                                    || nominal.clone(),
                                    |payload| {
                                        function(
                                            vec![self.type_syntax(module.id, payload)],
                                            nominal.clone(),
                                        )
                                    },
                                );
                                self.definitions.insert(
                                    constructor_id.clone(),
                                    Scheme::mono(constructor_type.clone()),
                                );
                                self.inferred_definitions
                                    .insert(constructor_id, constructor_type);
                            }
                        }
                    }
                    hir::TypeDefinition::Alias(alias) => {
                        let alias_type = self.type_syntax(module.id, alias);
                        self.definitions
                            .insert(id.clone(), Scheme::mono(alias_type.clone()));
                        self.inferred_definitions.insert(id, alias_type);
                    }
                }
            }
        }
    }

    fn predeclare_values(&mut self) {
        let modules = self.resolved.modules().to_vec();
        for module in &modules {
            for definition in &module.hir.definitions {
                let Some(id) = self
                    .resolved
                    .definition_id(module.id, &definition.name.normalized)
                    .cloned()
                else {
                    continue;
                };
                let variable = self.fresh();
                self.definitions
                    .insert(id.clone(), Scheme::mono(variable.clone()));
                self.inferred_definitions.insert(id, variable);
            }
        }
    }

    fn infer_definitions(&mut self) {
        let modules = self.resolved.modules().to_vec();
        for module in &modules {
            self.current_module = module.id;
            for definition in &module.hir.definitions {
                let Some(id) = self
                    .resolved
                    .definition_id(module.id, &definition.name.normalized)
                    .cloned()
                else {
                    continue;
                };
                let parameter_types = definition
                    .parameters
                    .iter()
                    .map(|pattern| {
                        let value = self.fresh();
                        self.bind_pattern(module.id, pattern, value.clone());
                        value
                    })
                    .collect::<Vec<_>>();
                let body = self.infer_expression(module.id, &definition.value);
                let inferred = if parameter_types.is_empty() {
                    body
                } else {
                    function(parameter_types, body)
                };
                let placeholder = self
                    .definitions
                    .get(&id)
                    .map(|scheme| scheme.value.clone())
                    .unwrap_or(InferType::Error);
                self.unify(
                    placeholder,
                    inferred.clone(),
                    module.id,
                    definition.span,
                    None,
                );
                let inferred = self.apply(inferred);
                let scheme = if definition.mutable {
                    Scheme::mono(inferred.clone())
                } else if !definition.parameters.is_empty() || non_expansive(&definition.value.kind)
                {
                    self.generalize(inferred.clone())
                } else {
                    Scheme::mono(inferred.clone())
                };
                self.definitions.insert(id.clone(), scheme);
                self.inferred_definitions.insert(id, inferred);
            }
        }
    }

    fn infer_expression(&mut self, module: ModuleId, expression: &hir::Expression) -> InferType {
        let key = ExpressionKey::new(module, expression.id);
        let value = match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                let mut result = InferType::Unit;
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            self.infer_local(module, binding);
                            result = InferType::Unit;
                        }
                        hir::SequenceElement::Expression(expression) => {
                            result = self.infer_expression(module, expression);
                        }
                    }
                }
                result
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_type = self.infer_expression(module, condition);
                self.unify(
                    InferType::Bool,
                    condition_type,
                    module,
                    condition.span,
                    None,
                );
                let then_type = self.infer_expression(module, then_branch);
                let else_type = self.infer_expression(module, else_branch);
                self.unify(then_type.clone(), else_type, module, expression.span, None);
                then_type
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                let scrutinee_type = self.infer_expression(module, scrutinee);
                let result = self.fresh();
                for case in cases {
                    self.bind_pattern(module, &case.pattern, scrutinee_type.clone());
                    if let Some(guard) = &case.guard {
                        let guard_type = self.infer_expression(module, guard);
                        self.unify(InferType::Bool, guard_type, module, guard.span, None);
                    }
                    let body = self.infer_expression(module, &case.body);
                    self.unify(result.clone(), body, module, case.body.span, None);
                }
                result
            }
            hir::ExpressionKind::Assignment { place, value } => {
                let place_type = self.infer_place(module, expression, place);
                let value_type = self.infer_expression(module, value);
                self.unify(place_type, value_type, module, value.span, None);
                InferType::Unit
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                let callable = self.infer_expression(module, function);
                let argument_types = arguments
                    .iter()
                    .map(|argument| self.infer_expression(module, argument))
                    .collect::<Vec<_>>();
                self.infer_application(callable, argument_types, module, expression.span)
            }
            hir::ExpressionKind::Projection {
                reference,
                target,
                field,
            } => {
                if let Some(resolved) = self.resolved.reference(module, *reference).cloned() {
                    self.reference_type(&resolved)
                } else {
                    let target_type = self.infer_expression(module, target);
                    self.project_field(target_type, field, module, expression.span, false)
                }
            }
            hir::ExpressionKind::Name { reference, .. } => self
                .resolved
                .reference(module, *reference)
                .cloned()
                .map_or(InferType::Error, |target| self.reference_type(&target)),
            hir::ExpressionKind::Binary {
                operator,
                left,
                right,
            } => {
                let left_type = self.infer_expression(module, left);
                let right_type = self.infer_expression(module, right);
                use hir::BinaryOperator as Operator;
                match operator {
                    Operator::Equal | Operator::NotEqual => {
                        self.unify(left_type, right_type, module, expression.span, None);
                        InferType::Bool
                    }
                    Operator::Less
                    | Operator::LessEqual
                    | Operator::Greater
                    | Operator::GreaterEqual => {
                        self.unify(InferType::Int, left_type, module, left.span, None);
                        self.unify(InferType::Int, right_type, module, right.span, None);
                        InferType::Bool
                    }
                    Operator::Add
                    | Operator::Subtract
                    | Operator::Multiply
                    | Operator::Divide
                    | Operator::Remainder => {
                        self.unify(InferType::Int, left_type, module, left.span, None);
                        self.unify(InferType::Int, right_type, module, right.span, None);
                        InferType::Int
                    }
                }
            }
            hir::ExpressionKind::Unary { operand, .. } => {
                let operand_type = self.infer_expression(module, operand);
                self.unify(InferType::Int, operand_type, module, operand.span, None);
                InferType::Int
            }
            hir::ExpressionKind::Literal(literal) => match literal {
                hir::Literal::Integer { radix, digits } => {
                    let canonical = digits.replace('_', "");
                    if let Some(integer) = BigInt::parse_bytes(canonical.as_bytes(), *radix) {
                        self.integers.insert(key, integer);
                    } else {
                        self.push_error(
                            module,
                            expression.span,
                            TypeErrorKind::InvalidInteger,
                            None,
                        );
                    }
                    InferType::Int
                }
                hir::Literal::Float(_) => InferType::Float64,
                hir::Literal::Text(_) => InferType::Text,
                hir::Literal::Boolean(_) => InferType::Bool,
            },
            hir::ExpressionKind::Unit => InferType::Unit,
            hir::ExpressionKind::Tuple(elements) => InferType::Tuple(
                elements
                    .iter()
                    .map(|element| self.infer_expression(module, element))
                    .collect(),
            ),
            hir::ExpressionKind::List(elements) => {
                let element_type = self.fresh();
                for element in elements {
                    let actual = self.infer_expression(module, element);
                    self.unify(element_type.clone(), actual, module, element.span, None);
                }
                InferType::List(Box::new(element_type))
            }
            hir::ExpressionKind::Record(fields) => {
                self.infer_record(module, fields, expression.span)
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                let base_type = self.infer_expression(module, base);
                for field in fields {
                    let expected = self.project_field(
                        base_type.clone(),
                        &field.name,
                        module,
                        field.span,
                        false,
                    );
                    let actual = self.infer_expression(module, &field.value);
                    self.unify(expected, actual, module, field.span, None);
                }
                base_type
            }
        };
        self.inferred_expressions.insert(key, value.clone());
        value
    }

    fn infer_local(&mut self, module: ModuleId, binding: &hir::LocalBinding) {
        let key = BindingKey::new(module, binding.id);
        let placeholder = self.fresh();
        if binding.recursive {
            self.bindings.insert(key, Scheme::mono(placeholder.clone()));
        }
        let parameter_types = binding
            .parameters
            .iter()
            .map(|pattern| {
                let value = self.fresh();
                self.bind_pattern(module, pattern, value.clone());
                value
            })
            .collect::<Vec<_>>();
        let body = self.infer_expression(module, &binding.value);
        let inferred = if parameter_types.is_empty() {
            body
        } else {
            function(parameter_types, body)
        };
        if binding.recursive {
            self.unify(placeholder, inferred.clone(), module, binding.span, None);
        }
        let inferred = self.apply(inferred);
        let restriction = if binding.mutable {
            Some("mutable_binding")
        } else if !binding.parameters.is_empty() || non_expansive(&binding.value.kind) {
            None
        } else {
            Some("expansive_rhs")
        };
        let scheme = if restriction.is_none() {
            self.generalize(inferred.clone())
        } else {
            Scheme::mono(inferred.clone())
        };
        self.bindings.insert(key, scheme);
        self.inferred_bindings.insert(key, inferred);
    }

    fn bind_pattern(&mut self, module: ModuleId, pattern: &hir::Pattern, value: InferType) {
        match &pattern.kind {
            hir::PatternKind::Binding { id, .. } => {
                let key = BindingKey::new(module, *id);
                self.bindings.insert(key, Scheme::mono(value.clone()));
                self.inferred_bindings.insert(key, value);
            }
            hir::PatternKind::Unit => {
                self.unify(InferType::Unit, value, module, pattern.span, None);
            }
            hir::PatternKind::Literal(literal) => {
                let expected = match literal {
                    hir::Literal::Integer { .. } => InferType::Int,
                    hir::Literal::Float(_) => InferType::Float64,
                    hir::Literal::Text(_) => InferType::Text,
                    hir::Literal::Boolean(_) => InferType::Bool,
                };
                self.unify(expected, value, module, pattern.span, None);
            }
            hir::PatternKind::Tuple(elements) => {
                let element_types = elements.iter().map(|_| self.fresh()).collect::<Vec<_>>();
                self.unify(
                    InferType::Tuple(element_types.clone()),
                    value,
                    module,
                    pattern.span,
                    None,
                );
                for (element, element_type) in elements.iter().zip(element_types) {
                    self.bind_pattern(module, element, element_type);
                }
            }
            hir::PatternKind::Constructor { name, arguments } => {
                let constructor = self
                    .resolved
                    .definition_id(module, &name.normalized)
                    .cloned();
                if let Some(constructor) = constructor {
                    let constructor_type =
                        self.reference_type(&ReferenceTarget::Definition(constructor));
                    let argument_types = arguments.iter().map(|_| self.fresh()).collect::<Vec<_>>();
                    let result = self.infer_application(
                        constructor_type,
                        argument_types.clone(),
                        module,
                        pattern.span,
                    );
                    self.unify(result, value, module, pattern.span, None);
                    for (argument, argument_type) in arguments.iter().zip(argument_types) {
                        self.bind_pattern(module, argument, argument_type);
                    }
                }
            }
        }
    }

    fn infer_place(
        &mut self,
        module: ModuleId,
        assignment: &hir::Expression,
        place: &hir::Place,
    ) -> InferType {
        let target = self
            .resolved
            .reference(module, place.root_reference)
            .cloned();
        let mut value = match target {
            Some(ReferenceTarget::Binding(binding)) => {
                let info = &self.resolved.bindings()[&binding];
                if !info.mutable {
                    let reason = if info.parameter {
                        "function parameters are immutable values"
                    } else {
                        "local binding is not declared mutable"
                    };
                    self.push_error(
                        module,
                        place.span,
                        TypeErrorKind::InvalidAssignment { reason },
                        None,
                    );
                }
                self.reference_type(&ReferenceTarget::Binding(binding))
            }
            Some(ReferenceTarget::Definition(_)) | None => {
                self.push_error(
                    module,
                    place.span,
                    TypeErrorKind::InvalidAssignment {
                        reason: "only current-function mutable locals are assignable",
                    },
                    None,
                );
                InferType::Error
            }
        };
        for field in &place.fields {
            value = self.project_field(value, field, module, field.span, true);
        }
        self.inferred_places
            .insert(ExpressionKey::new(module, assignment.id), value.clone());
        value
    }

    fn infer_record(
        &mut self,
        module: ModuleId,
        fields: &[hir::RecordField],
        span: Span,
    ) -> InferType {
        let field_names = fields
            .iter()
            .map(|field| field.name.normalized.as_str())
            .collect::<BTreeSet<_>>();
        let candidates = self
            .records
            .iter()
            .filter(|(_, record)| {
                record
                    .fields
                    .keys()
                    .map(String::as_str)
                    .collect::<BTreeSet<_>>()
                    == field_names
            })
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>();
        if candidates.len() != 1 {
            self.push_error(module, span, TypeErrorKind::AmbiguousRecord, None);
            for field in fields {
                self.infer_expression(module, &field.value);
            }
            return InferType::Error;
        }
        let definition = candidates[0].clone();
        let record = self.records[&definition].clone();
        for field in fields {
            let expected = record.fields[&field.name.normalized].1.clone();
            let actual = self.infer_expression(module, &field.value);
            self.unify(expected, actual, module, field.span, None);
        }
        InferType::Record {
            definition,
            arguments: Vec::new(),
        }
    }

    fn project_field(
        &mut self,
        target: InferType,
        field: &hir::Name,
        module: ModuleId,
        span: Span,
        require_mutable: bool,
    ) -> InferType {
        let target = self.apply(target);
        let InferType::Record { definition, .. } = target else {
            self.push_error(
                module,
                span,
                TypeErrorKind::UnknownField {
                    field: field.normalized.clone(),
                },
                None,
            );
            return InferType::Error;
        };
        let Some(record) = self.records.get(&definition) else {
            return InferType::Error;
        };
        let Some((mutable, field_type)) = record.fields.get(&field.normalized).cloned() else {
            self.push_error(
                module,
                span,
                TypeErrorKind::UnknownField {
                    field: field.normalized.clone(),
                },
                None,
            );
            return InferType::Error;
        };
        if require_mutable && !mutable {
            self.push_error(
                module,
                span,
                TypeErrorKind::InvalidAssignment {
                    reason: "record field is not declared mutable",
                },
                None,
            );
        }
        field_type
    }

    fn infer_application(
        &mut self,
        callable: InferType,
        arguments: Vec<InferType>,
        module: ModuleId,
        span: Span,
    ) -> InferType {
        let callable = self.apply(callable);
        match callable {
            InferType::Variable(variable) => {
                let result = self.fresh();
                let expected = function(arguments, result.clone());
                self.bind_variable(variable, expected, module, span, None);
                result
            }
            InferType::Function { parameters, result } => {
                if arguments.len() > parameters.len() {
                    self.push_error(
                        module,
                        span,
                        TypeErrorKind::Arity {
                            expected: parameters.len(),
                            actual: arguments.len(),
                        },
                        None,
                    );
                    return InferType::Error;
                }
                for (expected, actual) in parameters.iter().zip(&arguments) {
                    self.unify(expected.clone(), actual.clone(), module, span, None);
                }
                if arguments.len() == parameters.len() {
                    *result
                } else {
                    function(parameters[arguments.len()..].to_vec(), *result)
                }
            }
            InferType::Error => InferType::Error,
            actual => {
                self.push_error(
                    module,
                    span,
                    TypeErrorKind::NotCallable {
                        actual: display_infer(&self.apply(actual)),
                    },
                    None,
                );
                InferType::Error
            }
        }
    }

    fn reference_type(&mut self, target: &ReferenceTarget) -> InferType {
        match target {
            ReferenceTarget::Definition(definition) => self
                .definitions
                .get(definition)
                .cloned()
                .map_or(InferType::Error, |scheme| self.instantiate(&scheme)),
            ReferenceTarget::Binding(binding) => self
                .bindings
                .get(binding)
                .cloned()
                .map_or(InferType::Error, |scheme| self.instantiate(&scheme)),
        }
    }

    fn type_syntax(&mut self, module: ModuleId, syntax: &hir::TypeSyntax) -> InferType {
        let names = syntax
            .atoms
            .iter()
            .filter_map(|atom| match atom {
                hir::TypeAtom::Name(name) => Some(name.normalized.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        if syntax.atoms.len() == 1 {
            if let hir::TypeAtom::Name(name) = &syntax.atoms[0] {
                return match name.normalized.as_str() {
                    "Unit" => InferType::Unit,
                    "Bool" => InferType::Bool,
                    "Int" => InferType::Int,
                    "f64" => InferType::Float64,
                    "Text" => InferType::Text,
                    custom => self
                        .resolved
                        .definition_id(module, custom)
                        .cloned()
                        .and_then(|id| self.definitions.get(&id).map(|scheme| scheme.value.clone()))
                        .unwrap_or(InferType::Error),
                };
            }
            if matches!(syntax.atoms[0], hir::TypeAtom::Variable(_)) {
                return self.fresh();
            }
        }
        if syntax
            .atoms
            .iter()
            .any(|atom| matches!(atom, hir::TypeAtom::Arrow))
        {
            let mut parts = Vec::new();
            let mut start = 0;
            for (index, atom) in syntax.atoms.iter().enumerate() {
                if matches!(atom, hir::TypeAtom::Arrow) {
                    parts.push(self.simple_type_atom(module, &syntax.atoms[start..index]));
                    start = index + 1;
                }
            }
            parts.push(self.simple_type_atom(module, &syntax.atoms[start..]));
            if let Some(result) = parts.pop() {
                return function(parts, result);
            }
        }
        if names.first() == Some(&"List") {
            if let Some(hir::TypeAtom::Name(element)) = syntax.atoms.get(2) {
                return InferType::List(Box::new(self.named_type(module, &element.normalized)));
            }
        }
        self.push_error(
            module,
            syntax.span,
            TypeErrorKind::UnsupportedTypeSyntax,
            None,
        );
        InferType::Error
    }

    fn simple_type_atom(&mut self, module: ModuleId, atoms: &[hir::TypeAtom]) -> InferType {
        match atoms {
            [hir::TypeAtom::Name(name)] => self.named_type(module, &name.normalized),
            [hir::TypeAtom::Variable(_)] => self.fresh(),
            _ => InferType::Error,
        }
    }

    fn named_type(&self, module: ModuleId, name: &str) -> InferType {
        match name {
            "Unit" => InferType::Unit,
            "Bool" => InferType::Bool,
            "Int" => InferType::Int,
            "f64" => InferType::Float64,
            "Text" => InferType::Text,
            custom => self
                .resolved
                .definition_id(module, custom)
                .and_then(|id| self.definitions.get(id))
                .map_or(InferType::Error, |scheme| scheme.value.clone()),
        }
    }

    fn unify(
        &mut self,
        expected: InferType,
        actual: InferType,
        module: ModuleId,
        span: Span,
        restriction_reason: Option<&'static str>,
    ) {
        let expected = self.apply(expected);
        let actual = self.apply(actual);
        match (expected.clone(), actual.clone()) {
            (InferType::Error, _) | (_, InferType::Error) => {}
            (InferType::Variable(variable), value) | (value, InferType::Variable(variable)) => {
                self.bind_variable(variable, value, module, span, restriction_reason);
            }
            (InferType::Unit, InferType::Unit)
            | (InferType::Bool, InferType::Bool)
            | (InferType::Int, InferType::Int)
            | (InferType::Float64, InferType::Float64)
            | (InferType::Text, InferType::Text) => {}
            (InferType::Tuple(left), InferType::Tuple(right)) if left.len() == right.len() => {
                for (left, right) in left.into_iter().zip(right) {
                    self.unify(left, right, module, span, restriction_reason);
                }
            }
            (InferType::List(left), InferType::List(right)) => {
                self.unify(*left, *right, module, span, restriction_reason);
            }
            (
                InferType::Function {
                    parameters: left,
                    result: left_result,
                },
                InferType::Function {
                    parameters: right,
                    result: right_result,
                },
            ) if left.len() == right.len() => {
                for (left, right) in left.into_iter().zip(right) {
                    self.unify(left, right, module, span, restriction_reason);
                }
                self.unify(
                    *left_result,
                    *right_result,
                    module,
                    span,
                    restriction_reason,
                );
            }
            (
                InferType::Record {
                    definition: left, ..
                },
                InferType::Record {
                    definition: right, ..
                },
            ) if left == right => {}
            (
                InferType::Variant {
                    definition: left, ..
                },
                InferType::Variant {
                    definition: right, ..
                },
            ) if left == right => {}
            _ => self.push_error(
                module,
                span,
                TypeErrorKind::Mismatch {
                    expected: display_infer(&expected),
                    actual: display_infer(&actual),
                },
                restriction_reason,
            ),
        }
    }

    fn bind_variable(
        &mut self,
        variable: u32,
        value: InferType,
        module: ModuleId,
        span: Span,
        restriction_reason: Option<&'static str>,
    ) {
        if value == InferType::Variable(variable) {
            return;
        }
        if free_variables(&value).contains(&variable) {
            self.push_error(
                module,
                span,
                TypeErrorKind::InfiniteType,
                restriction_reason,
            );
            self.substitutions.insert(variable, InferType::Error);
        } else {
            self.substitutions.insert(variable, value);
        }
    }

    fn apply(&self, value: InferType) -> InferType {
        match value {
            InferType::Variable(variable) => self
                .substitutions
                .get(&variable)
                .cloned()
                .map_or(InferType::Variable(variable), |value| self.apply(value)),
            InferType::Tuple(elements) => InferType::Tuple(
                elements
                    .into_iter()
                    .map(|value| self.apply(value))
                    .collect(),
            ),
            InferType::List(element) => InferType::List(Box::new(self.apply(*element))),
            InferType::Function { parameters, result } => InferType::Function {
                parameters: parameters
                    .into_iter()
                    .map(|value| self.apply(value))
                    .collect(),
                result: Box::new(self.apply(*result)),
            },
            InferType::Record {
                definition,
                arguments,
            } => InferType::Record {
                definition,
                arguments: arguments
                    .into_iter()
                    .map(|value| self.apply(value))
                    .collect(),
            },
            InferType::Variant {
                definition,
                arguments,
            } => InferType::Variant {
                definition,
                arguments: arguments
                    .into_iter()
                    .map(|value| self.apply(value))
                    .collect(),
            },
            value => value,
        }
    }

    fn instantiate(&mut self, scheme: &Scheme) -> InferType {
        let replacements = scheme
            .quantified
            .iter()
            .map(|variable| (*variable, self.fresh()))
            .collect::<BTreeMap<_, _>>();
        replace_variables(&scheme.value, &replacements)
    }

    fn generalize(&self, value: InferType) -> Scheme {
        let environment = self
            .definitions
            .values()
            .chain(self.bindings.values())
            .flat_map(free_scheme_variables)
            .collect::<BTreeSet<_>>();
        let quantified = free_variables(&value)
            .difference(&environment)
            .copied()
            .collect();
        Scheme { quantified, value }
    }

    fn fresh(&mut self) -> InferType {
        InferType::Variable(self.fresh_variable_id())
    }

    fn fresh_variable_id(&mut self) -> u32 {
        let id = self.next_variable;
        self.next_variable = self.next_variable.saturating_add(1);
        id
    }

    fn push_error(
        &mut self,
        module: ModuleId,
        span: Span,
        kind: TypeErrorKind,
        restriction_reason: Option<&'static str>,
    ) {
        self.errors.push(TypeError {
            kind,
            source_name: self
                .resolved
                .module(module)
                .map(|module| module.hir.source_name.clone())
                .unwrap_or_default(),
            span,
            restriction_reason,
        });
    }
}

fn function(parameters: Vec<InferType>, result: InferType) -> InferType {
    InferType::Function {
        parameters,
        result: Box::new(result),
    }
}

fn non_expansive(kind: &hir::ExpressionKind) -> bool {
    match kind {
        hir::ExpressionKind::Name { .. }
        | hir::ExpressionKind::Literal(_)
        | hir::ExpressionKind::Unit => true,
        hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
            elements.iter().all(|element| non_expansive(&element.kind))
        }
        hir::ExpressionKind::Record(fields) => {
            fields.iter().all(|field| non_expansive(&field.value.kind))
        }
        _ => false,
    }
}

fn free_variables(value: &InferType) -> BTreeSet<u32> {
    let mut output = BTreeSet::new();
    collect_free_variables(value, &mut output);
    output
}

fn collect_free_variables(value: &InferType, output: &mut BTreeSet<u32>) {
    match value {
        InferType::Variable(variable) => {
            output.insert(*variable);
        }
        InferType::Tuple(elements) => {
            for element in elements {
                collect_free_variables(element, output);
            }
        }
        InferType::List(element) => collect_free_variables(element, output),
        InferType::Function { parameters, result } => {
            for parameter in parameters {
                collect_free_variables(parameter, output);
            }
            collect_free_variables(result, output);
        }
        InferType::Record { arguments, .. } | InferType::Variant { arguments, .. } => {
            for argument in arguments {
                collect_free_variables(argument, output);
            }
        }
        InferType::Unit
        | InferType::Bool
        | InferType::Int
        | InferType::Float64
        | InferType::Text
        | InferType::Error => {}
    }
}

fn free_scheme_variables(scheme: &Scheme) -> BTreeSet<u32> {
    free_variables(&scheme.value)
        .difference(&scheme.quantified)
        .copied()
        .collect()
}

fn replace_variables(value: &InferType, replacements: &BTreeMap<u32, InferType>) -> InferType {
    match value {
        InferType::Variable(variable) => replacements
            .get(variable)
            .cloned()
            .unwrap_or_else(|| value.clone()),
        InferType::Tuple(elements) => InferType::Tuple(
            elements
                .iter()
                .map(|element| replace_variables(element, replacements))
                .collect(),
        ),
        InferType::List(element) => {
            InferType::List(Box::new(replace_variables(element, replacements)))
        }
        InferType::Function { parameters, result } => InferType::Function {
            parameters: parameters
                .iter()
                .map(|parameter| replace_variables(parameter, replacements))
                .collect(),
            result: Box::new(replace_variables(result, replacements)),
        },
        InferType::Record {
            definition,
            arguments,
        } => InferType::Record {
            definition: definition.clone(),
            arguments: arguments
                .iter()
                .map(|argument| replace_variables(argument, replacements))
                .collect(),
        },
        InferType::Variant {
            definition,
            arguments,
        } => InferType::Variant {
            definition: definition.clone(),
            arguments: arguments
                .iter()
                .map(|argument| replace_variables(argument, replacements))
                .collect(),
        },
        value => value.clone(),
    }
}

fn intern_type(
    value: &InferType,
    arena: &mut TypeArena,
    index: &mut BTreeMap<Type, TypeId>,
) -> TypeId {
    let value = match value {
        InferType::Unit => Type::Unit,
        InferType::Bool => Type::Bool,
        InferType::Int => Type::Int,
        InferType::Float64 => Type::Float64,
        InferType::Text => Type::Text,
        InferType::Tuple(elements) => Type::Tuple(
            elements
                .iter()
                .map(|element| intern_type(element, arena, index))
                .collect(),
        ),
        InferType::List(element) => Type::List(intern_type(element, arena, index)),
        InferType::Function { parameters, result } => Type::Function {
            parameters: parameters
                .iter()
                .map(|parameter| intern_type(parameter, arena, index))
                .collect(),
            result: intern_type(result, arena, index),
        },
        InferType::Record {
            definition,
            arguments,
        } => Type::NominalRecord {
            definition: definition.clone(),
            arguments: arguments
                .iter()
                .map(|argument| intern_type(argument, arena, index))
                .collect(),
        },
        InferType::Variant {
            definition,
            arguments,
        } => Type::NominalVariant {
            definition: definition.clone(),
            arguments: arguments
                .iter()
                .map(|argument| intern_type(argument, arena, index))
                .collect(),
        },
        InferType::Variable(variable) => Type::Variable(*variable),
        InferType::Error => Type::Error,
    };
    arena.intern(value, index)
}

fn display_infer(value: &InferType) -> String {
    match value {
        InferType::Unit => "Unit".to_owned(),
        InferType::Bool => "Bool".to_owned(),
        InferType::Int => "Int".to_owned(),
        InferType::Float64 => "f64".to_owned(),
        InferType::Text => "Text".to_owned(),
        InferType::Tuple(elements) => format!(
            "({})",
            elements
                .iter()
                .map(display_infer)
                .collect::<Vec<_>>()
                .join(" * ")
        ),
        InferType::List(element) => format!("List<{}>", display_infer(element)),
        InferType::Function { parameters, result } => {
            let mut parts = parameters.iter().map(display_infer).collect::<Vec<_>>();
            parts.push(display_infer(result));
            parts.join(" -> ")
        }
        InferType::Record { definition, .. } | InferType::Variant { definition, .. } => {
            definition.to_string()
        }
        InferType::Variable(variable) => format!("'t{variable}"),
        InferType::Error => "<error>".to_owned(),
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
            SourceFile::from_bytes(SourceId::new(0), "test.ling", text.as_bytes().to_vec())
                .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = hir::lower(source.name(), &ast).expect("valid HIR");
        ling_resolve::resolve(vec![hir], "Main").expect("resolves")
    }

    #[test]
    fn infers_hello_main_and_console_write() {
        let typed = check(resolved(
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"你好，零\"\n",
        ))
        .expect("hello type-checks");
        let main = typed
            .resolved()
            .definition_id(typed.resolved().entry(), "main")
            .expect("main definition");
        let main_type = typed.definition_type(main).expect("main type");
        assert_eq!(typed.arena().display(main_type), "Unit -> Unit");
    }

    #[test]
    fn rejects_console_write_with_an_integer() {
        let errors = check(resolved(
            "module Main\n    requires Console.Write\n\nlet main () = Console.write 1\n",
        ))
        .expect_err("argument type must fail");
        assert!(
            errors
                .iter()
                .any(|error| matches!(error.kind, TypeErrorKind::Mismatch { .. }))
        );
    }

    #[test]
    fn preserves_integers_larger_than_i128() {
        let typed = check(resolved(
            "module Main\n\nlet huge = 340282366920938463463374607431768211456\n",
        ))
        .expect("arbitrary precision integer type-checks");
        assert!(
            typed
                .integers
                .values()
                .any(|value| value.to_string() == "340282366920938463463374607431768211456")
        );
    }

    #[test]
    fn occurs_check_rejects_self_application() {
        let errors = check(resolved("module Main\n\nlet omega f = f f\n"))
            .expect_err("self application must create an infinite type");
        assert!(
            errors
                .iter()
                .any(|error| matches!(error.kind, TypeErrorKind::InfiniteType))
        );
    }
}
