//! Hindley-Milner-style Seed type inference and assignment validation.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_hir as hir;
use ling_resolve::{
    BindingKey, Builtin, DefinitionId, DefinitionKind, ExpressionKey, ModuleId, PreludeDefinition,
    ReferenceTarget, ResolvedProgram,
};
use ling_source::Span;
use num_bigint::BigInt;

mod coherence;
mod constraints;
// Solver evidence remains crate-private until dictionary lowering integrates it.
#[allow(dead_code)]
mod solver;

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
                    .map(|parameter| match self.get(*parameter) {
                        Type::Function { .. } => format!("({})", self.display(*parameter)),
                        _ => self.display(*parameter),
                    })
                    .collect::<Vec<_>>();
                parts.push(self.display(*result));
                parts.join(" -> ")
            }
            Type::NominalRecord {
                definition,
                arguments,
            }
            | Type::NominalVariant {
                definition,
                arguments,
            } => display_nominal(self, definition, arguments),
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
    pub parameters: Vec<String>,
    pub fields: Vec<RecordFieldInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantCaseInfo {
    pub definition: DefinitionId,
    pub name: String,
    pub payload: Option<TypeId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantInfo {
    pub definition: DefinitionId,
    pub name: String,
    pub parameters: Vec<String>,
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
    place_root_types: BTreeMap<ExpressionKey, TypeId>,
    integers: BTreeMap<ExpressionKey, BigInt>,
    records: BTreeMap<DefinitionId, RecordInfo>,
    variants: BTreeMap<DefinitionId, VariantInfo>,
    warnings: Vec<Diagnostic>,
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
    pub fn place_root_type(&self, key: ExpressionKey) -> Option<TypeId> {
        self.place_root_types.get(&key).copied()
    }

    #[must_use]
    pub fn display_type(&self, id: TypeId) -> String {
        display_resolved_type(&self.arena, &self.resolved, id)
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

    #[must_use]
    pub fn warnings(&self) -> &[Diagnostic] {
        &self.warnings
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
    Mismatch {
        expected: String,
        actual: String,
    },
    InfiniteType,
    NotCallable {
        actual: String,
    },
    Arity {
        expected: usize,
        actual: usize,
    },
    UnknownField {
        field: String,
    },
    AmbiguousRecord,
    DuplicateRecordField {
        field: String,
    },
    MissingRecordFields {
        fields: Vec<String>,
    },
    NonExhaustiveMatch {
        witness: String,
    },
    InvalidConstructorPattern {
        constructor: String,
        expected: usize,
        actual: usize,
    },
    InvalidInteger,
    InvalidAssignment {
        reason: &'static str,
        mutability: &'static str,
        field: Option<String>,
    },
    UnsupportedEquality {
        actual: String,
    },
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
            TypeErrorKind::DuplicateRecordField { field } => (
                codes::DUPLICATE_RECORD_FIELD,
                format!("record 字段“{field}”重复"),
                format!("record field `{field}` is duplicated"),
            ),
            TypeErrorKind::MissingRecordFields { fields } => (
                codes::MISSING_RECORD_FIELDS,
                format!("record 缺少字段：{}", fields.join("、")),
                format!("record is missing fields: {}", fields.join(", ")),
            ),
            TypeErrorKind::NonExhaustiveMatch { witness } => (
                codes::NON_EXHAUSTIVE_MATCH,
                format!("match 非穷尽；缺少模式 {witness}"),
                format!("match is non-exhaustive; missing pattern {witness}"),
            ),
            TypeErrorKind::InvalidConstructorPattern {
                constructor,
                expected,
                actual,
            } => (
                codes::INVALID_CONSTRUCTOR_PATTERN,
                format!(
                    "constructor 模式“{constructor}”参数数量不匹配：期望 {expected}，实际 {actual}"
                ),
                format!(
                    "constructor pattern `{constructor}` has wrong arity: expected {expected}, found {actual}"
                ),
            ),
            TypeErrorKind::InvalidInteger => (
                codes::TYPE_MISMATCH,
                "整数 literal 无法解析".to_owned(),
                "integer literal could not be parsed".to_owned(),
            ),
            TypeErrorKind::InvalidAssignment { reason, .. } => (
                codes::INVALID_ASSIGNMENT,
                format!("赋值左侧不可修改：{reason}"),
                format!("assignment target is not mutable: {reason}"),
            ),
            TypeErrorKind::UnsupportedEquality { actual } => (
                codes::UNSUPPORTED_EQUALITY,
                format!("类型 {actual} 不支持相等性比较"),
                format!("type {actual} does not support equality"),
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
        match &self.kind {
            TypeErrorKind::UnknownField { field }
            | TypeErrorKind::DuplicateRecordField { field } => {
                diagnostic = diagnostic.with_fact("field", field.as_str());
            }
            TypeErrorKind::MissingRecordFields { fields } => {
                diagnostic = diagnostic.with_fact("fields", fields.join(","));
            }
            TypeErrorKind::NonExhaustiveMatch { witness } => {
                diagnostic = diagnostic.with_fact("witness", witness.as_str());
            }
            TypeErrorKind::InvalidConstructorPattern {
                constructor,
                expected,
                actual,
            } => {
                diagnostic = diagnostic
                    .with_fact("constructor", constructor.as_str())
                    .with_fact(
                        "expected_arity",
                        u64::try_from(*expected).unwrap_or(u64::MAX),
                    )
                    .with_fact("actual_arity", u64::try_from(*actual).unwrap_or(u64::MAX));
            }
            TypeErrorKind::InvalidAssignment {
                reason,
                mutability,
                field,
            } => {
                diagnostic = diagnostic
                    .with_fact("reason", *reason)
                    .with_fact("mutability", *mutability);
                if let Some(field) = field {
                    diagnostic = diagnostic.with_fact("field", field.as_str());
                }
            }
            TypeErrorKind::UnsupportedEquality { actual } => {
                diagnostic = diagnostic.with_fact("type", actual.as_str());
            }
            _ => {}
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
    let mut trait_errors = constraints::trait_item_spans(&resolved)
        .into_iter()
        .map(|(source_name, span)| TypeError {
            kind: TypeErrorKind::UnsupportedTypeSyntax,
            source_name,
            span,
            restriction_reason: None,
        })
        .collect::<Vec<_>>();
    match constraints::collect_obligations(&resolved) {
        Ok(obligations) => {
            trait_errors.extend(obligations.into_iter().map(|obligation| TypeError {
                kind: TypeErrorKind::UnsupportedTypeSyntax,
                source_name: obligation.origin.source_name,
                span: obligation.origin.span,
                restriction_reason: None,
            }))
        }
        Err(collection_errors) => {
            trait_errors.extend(collection_errors.into_iter().map(|error| TypeError {
                kind: TypeErrorKind::UnsupportedTypeSyntax,
                source_name: error.source_name,
                span: error.span,
                restriction_reason: None,
            }))
        }
    }
    if let Err(coherence_errors) = coherence::build_index(&resolved) {
        trait_errors.extend(coherence_errors.into_iter().map(|error| TypeError {
            kind: TypeErrorKind::UnsupportedTypeSyntax,
            source_name: error.source_name,
            span: error.span,
            restriction_reason: None,
        }));
    }
    if !trait_errors.is_empty() {
        trait_errors.sort_by(|left, right| {
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
        return Err(trait_errors);
    }
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

#[derive(Clone, Debug, Eq, PartialEq)]
enum PatternCoverage {
    CatchAll,
    Boolean(bool),
    Constructor(DefinitionId),
    Other,
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
    parameters: Vec<(String, u32)>,
    fields: BTreeMap<String, (bool, InferType)>,
}

#[derive(Clone)]
struct InternalVariant {
    name: String,
    parameters: Vec<(String, u32)>,
    cases: BTreeMap<String, Option<InferType>>,
    constructors: BTreeMap<String, DefinitionId>,
}

struct Inferencer {
    resolved: ResolvedProgram,
    next_variable: u32,
    substitutions: BTreeMap<u32, InferType>,
    restricted_variables: BTreeMap<u32, &'static str>,
    definitions: BTreeMap<DefinitionId, Scheme>,
    bindings: BTreeMap<BindingKey, Scheme>,
    inferred_expressions: BTreeMap<ExpressionKey, InferType>,
    inferred_definitions: BTreeMap<DefinitionId, InferType>,
    inferred_bindings: BTreeMap<BindingKey, InferType>,
    inferred_places: BTreeMap<ExpressionKey, InferType>,
    inferred_place_roots: BTreeMap<ExpressionKey, InferType>,
    equality_constraints: Vec<(ModuleId, Span, InferType)>,
    integers: BTreeMap<ExpressionKey, BigInt>,
    records: BTreeMap<DefinitionId, InternalRecord>,
    variants: BTreeMap<DefinitionId, InternalVariant>,
    errors: Vec<TypeError>,
    warnings: Vec<Diagnostic>,
    current_module: ModuleId,
}

impl Inferencer {
    fn new(resolved: ResolvedProgram) -> Self {
        let current_module = resolved.entry();
        Self {
            resolved,
            next_variable: 0,
            substitutions: BTreeMap::new(),
            restricted_variables: BTreeMap::new(),
            definitions: BTreeMap::new(),
            bindings: BTreeMap::new(),
            inferred_expressions: BTreeMap::new(),
            inferred_definitions: BTreeMap::new(),
            inferred_bindings: BTreeMap::new(),
            inferred_places: BTreeMap::new(),
            inferred_place_roots: BTreeMap::new(),
            equality_constraints: Vec::new(),
            integers: BTreeMap::new(),
            records: BTreeMap::new(),
            variants: BTreeMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            current_module,
        }
    }

    fn run(mut self) -> Result<TypedProgram, Vec<TypeError>> {
        self.seed_builtins();
        self.seed_prelude();
        self.index_nominal_types();
        self.predeclare_values();
        self.infer_definitions();
        self.check_equality_constraints();

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
        let place_root_types = self
            .inferred_place_roots
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
                        parameters: record
                            .parameters
                            .iter()
                            .map(|(name, _)| name.clone())
                            .collect(),
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
                        definition: variant.constructors[name].clone(),
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
                        parameters: variant
                            .parameters
                            .iter()
                            .map(|(name, _)| name.clone())
                            .collect(),
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
            place_root_types,
            integers: self.integers,
            records,
            variants,
            warnings: self.warnings,
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

    fn seed_prelude(&mut self) {
        let option_parameter = self.fresh_variable_id();
        let option_id = self.resolved.prelude_id(PreludeDefinition::Option).clone();
        let some_id = self.resolved.prelude_id(PreludeDefinition::Some).clone();
        let none_id = self.resolved.prelude_id(PreludeDefinition::None).clone();
        let option = InferType::Variant {
            definition: option_id.clone(),
            arguments: vec![InferType::Variable(option_parameter)],
        };
        let option_quantified = BTreeSet::from([option_parameter]);
        self.variants.insert(
            option_id.clone(),
            InternalVariant {
                name: "Option".to_owned(),
                parameters: vec![("'a".to_owned(), option_parameter)],
                cases: BTreeMap::from([
                    ("None".to_owned(), None),
                    (
                        "Some".to_owned(),
                        Some(InferType::Variable(option_parameter)),
                    ),
                ]),
                constructors: BTreeMap::from([
                    ("None".to_owned(), none_id.clone()),
                    ("Some".to_owned(), some_id.clone()),
                ]),
            },
        );
        self.insert_prelude_scheme(
            option_id,
            Scheme {
                quantified: option_quantified.clone(),
                value: option.clone(),
            },
        );
        self.insert_prelude_scheme(
            some_id,
            Scheme {
                quantified: option_quantified.clone(),
                value: function(vec![InferType::Variable(option_parameter)], option.clone()),
            },
        );
        self.insert_prelude_scheme(
            none_id,
            Scheme {
                quantified: option_quantified,
                value: option,
            },
        );

        let result_value_parameter = self.fresh_variable_id();
        let result_error_parameter = self.fresh_variable_id();
        let result_id = self.resolved.prelude_id(PreludeDefinition::Result).clone();
        let ok_id = self.resolved.prelude_id(PreludeDefinition::Ok).clone();
        let error_id = self.resolved.prelude_id(PreludeDefinition::Error).clone();
        let result = InferType::Variant {
            definition: result_id.clone(),
            arguments: vec![
                InferType::Variable(result_value_parameter),
                InferType::Variable(result_error_parameter),
            ],
        };
        let result_quantified = BTreeSet::from([result_value_parameter, result_error_parameter]);
        self.variants.insert(
            result_id.clone(),
            InternalVariant {
                name: "Result".to_owned(),
                parameters: vec![
                    ("'a".to_owned(), result_value_parameter),
                    ("'e".to_owned(), result_error_parameter),
                ],
                cases: BTreeMap::from([
                    (
                        "Error".to_owned(),
                        Some(InferType::Variable(result_error_parameter)),
                    ),
                    (
                        "Ok".to_owned(),
                        Some(InferType::Variable(result_value_parameter)),
                    ),
                ]),
                constructors: BTreeMap::from([
                    ("Error".to_owned(), error_id.clone()),
                    ("Ok".to_owned(), ok_id.clone()),
                ]),
            },
        );
        self.insert_prelude_scheme(
            result_id,
            Scheme {
                quantified: result_quantified.clone(),
                value: result.clone(),
            },
        );
        self.insert_prelude_scheme(
            ok_id,
            Scheme {
                quantified: result_quantified.clone(),
                value: function(
                    vec![InferType::Variable(result_value_parameter)],
                    result.clone(),
                ),
            },
        );
        self.insert_prelude_scheme(
            error_id,
            Scheme {
                quantified: result_quantified,
                value: function(vec![InferType::Variable(result_error_parameter)], result),
            },
        );
    }

    fn insert_prelude_scheme(&mut self, id: DefinitionId, scheme: Scheme) {
        self.definitions.insert(id.clone(), scheme.clone());
        self.inferred_definitions.insert(id, scheme.value);
    }

    fn index_nominal_types(&mut self) {
        let modules = self.resolved.modules().to_vec();
        let mut declaration_parameters = BTreeMap::new();

        // First register every type name so declarations may refer to types
        // that appear later in the same module. Declaration parameters use a
        // distinct, stable inference variable per name and are quantified at
        // the nominal boundary.
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
                let mut seen = BTreeSet::new();
                let parameters = declaration
                    .parameters
                    .iter()
                    .filter_map(|parameter| {
                        if seen.insert(parameter.normalized.clone()) {
                            Some((parameter.normalized.clone(), self.fresh_variable_id()))
                        } else {
                            self.push_error(
                                module.id,
                                parameter.span,
                                TypeErrorKind::UnsupportedTypeSyntax,
                                None,
                            );
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let quantified = parameters
                    .iter()
                    .map(|(_, variable)| *variable)
                    .collect::<BTreeSet<_>>();
                let arguments = parameters
                    .iter()
                    .map(|(_, variable)| InferType::Variable(*variable))
                    .collect::<Vec<_>>();
                declaration_parameters.insert(id.clone(), parameters.clone());
                match &declaration.definition {
                    hir::TypeDefinition::Record(_) => {
                        self.records.insert(
                            id.clone(),
                            InternalRecord {
                                name: declaration.name.normalized.clone(),
                                parameters,
                                fields: BTreeMap::new(),
                            },
                        );
                        let nominal = InferType::Record {
                            definition: id.clone(),
                            arguments,
                        };
                        self.definitions.insert(
                            id.clone(),
                            Scheme {
                                quantified,
                                value: nominal.clone(),
                            },
                        );
                        self.inferred_definitions.insert(id, nominal);
                    }
                    hir::TypeDefinition::Variant(_) => {
                        self.variants.insert(
                            id.clone(),
                            InternalVariant {
                                name: declaration.name.normalized.clone(),
                                parameters,
                                cases: BTreeMap::new(),
                                constructors: BTreeMap::new(),
                            },
                        );
                        let nominal = InferType::Variant {
                            definition: id.clone(),
                            arguments,
                        };
                        self.definitions.insert(
                            id.clone(),
                            Scheme {
                                quantified,
                                value: nominal.clone(),
                            },
                        );
                        self.inferred_definitions
                            .insert(id.clone(), nominal.clone());
                    }
                    hir::TypeDefinition::Alias(_) => {
                        let placeholder = self.fresh();
                        self.definitions.insert(
                            id.clone(),
                            Scheme {
                                quantified,
                                value: placeholder.clone(),
                            },
                        );
                        self.inferred_definitions.insert(id, placeholder);
                    }
                }
            }
        }

        // Resolve alias templates in dependency order. Record and variant
        // nominal identities are already available from the first pass, while
        // an alias never observes another alias's unresolved placeholder.
        let mut pending_aliases = modules
            .iter()
            .flat_map(|module| {
                module.hir.types.iter().filter_map(|declaration| {
                    if !matches!(declaration.definition, hir::TypeDefinition::Alias(_)) {
                        return None;
                    }
                    self.resolved
                        .definition_id(module.id, &declaration.name.normalized)
                        .cloned()
                        .map(|id| (id, (module.id, declaration.clone())))
                })
            })
            .collect::<BTreeMap<_, _>>();
        let alias_ids = pending_aliases.keys().cloned().collect::<BTreeSet<_>>();
        while !pending_aliases.is_empty() {
            let ready = pending_aliases
                .iter()
                .filter_map(|(id, (module, declaration))| {
                    let hir::TypeDefinition::Alias(alias) = &declaration.definition else {
                        unreachable!("pending entries are aliases")
                    };
                    self.alias_dependencies(*module, alias, &alias_ids)
                        .iter()
                        .all(|dependency| !pending_aliases.contains_key(dependency))
                        .then(|| id.clone())
                })
                .collect::<Vec<_>>();
            if ready.is_empty() {
                for (module, declaration) in pending_aliases.values() {
                    self.push_error(
                        *module,
                        declaration.span,
                        TypeErrorKind::UnsupportedTypeSyntax,
                        None,
                    );
                }
                break;
            }
            for id in ready {
                let (module, declaration) = pending_aliases
                    .remove(&id)
                    .expect("ready alias remains pending");
                let hir::TypeDefinition::Alias(alias) = &declaration.definition else {
                    unreachable!("pending entries are aliases")
                };
                let parameters = declaration_parameters.get(&id).cloned().unwrap_or_default();
                let parameter_environment = parameters
                    .iter()
                    .map(|(name, variable)| (name.clone(), InferType::Variable(*variable)))
                    .collect::<BTreeMap<_, _>>();
                let quantified = parameters
                    .iter()
                    .map(|(_, variable)| *variable)
                    .collect::<BTreeSet<_>>();
                let alias_type =
                    self.type_syntax_with_parameters(module, alias, &parameter_environment);
                let placeholder = self
                    .definitions
                    .get(&id)
                    .map(|scheme| scheme.value.clone())
                    .unwrap_or(InferType::Error);
                self.unify(placeholder, alias_type.clone(), module, alias.span, None);
                let alias_type = self.apply(alias_type);
                self.definitions.insert(
                    id.clone(),
                    Scheme {
                        quantified,
                        value: alias_type.clone(),
                    },
                );
                self.inferred_definitions.insert(id, alias_type);
            }
        }

        // Populate fields, constructor signatures, and aliases only after all
        // names are available. Every occurrence of a declaration parameter is
        // resolved through the same environment.
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
                let parameters = declaration_parameters.get(&id).cloned().unwrap_or_default();
                let parameter_environment = parameters
                    .iter()
                    .map(|(name, variable)| (name.clone(), InferType::Variable(*variable)))
                    .collect::<BTreeMap<_, _>>();
                let quantified = parameters
                    .iter()
                    .map(|(_, variable)| *variable)
                    .collect::<BTreeSet<_>>();
                match &declaration.definition {
                    hir::TypeDefinition::Record(fields) => {
                        let mut indexed = BTreeMap::new();
                        for field in fields {
                            let field_type = self.type_syntax_with_parameters(
                                module.id,
                                &field.field_type,
                                &parameter_environment,
                            );
                            if indexed
                                .insert(field.name.normalized.clone(), (field.mutable, field_type))
                                .is_some()
                            {
                                self.push_error(
                                    module.id,
                                    field.span,
                                    TypeErrorKind::DuplicateRecordField {
                                        field: field.name.normalized.clone(),
                                    },
                                    None,
                                );
                            }
                        }
                        if let Some(record) = self.records.get_mut(&id) {
                            record.fields = indexed;
                        }
                    }
                    hir::TypeDefinition::Variant(cases) => {
                        let nominal = InferType::Variant {
                            definition: id.clone(),
                            arguments: parameters
                                .iter()
                                .map(|(_, variable)| InferType::Variable(*variable))
                                .collect(),
                        };
                        let mut indexed = BTreeMap::new();
                        let mut constructors = BTreeMap::new();
                        for case in cases {
                            let payload = case.payload.as_ref().map(|payload| {
                                self.type_syntax_with_parameters(
                                    module.id,
                                    payload,
                                    &parameter_environment,
                                )
                            });
                            indexed.insert(case.name.normalized.clone(), payload.clone());
                            if let Some(constructor_id) = self
                                .resolved
                                .definition_id(module.id, &case.name.normalized)
                                .cloned()
                            {
                                constructors
                                    .insert(case.name.normalized.clone(), constructor_id.clone());
                                let constructor_type = payload.map_or_else(
                                    || nominal.clone(),
                                    |payload| function(vec![payload], nominal.clone()),
                                );
                                self.definitions.insert(
                                    constructor_id.clone(),
                                    Scheme {
                                        quantified: quantified.clone(),
                                        value: constructor_type.clone(),
                                    },
                                );
                                self.inferred_definitions
                                    .insert(constructor_id, constructor_type);
                            }
                        }
                        if let Some(variant) = self.variants.get_mut(&id) {
                            variant.cases = indexed;
                            variant.constructors = constructors;
                        }
                    }
                    hir::TypeDefinition::Alias(_) => {}
                }
            }
        }
    }

    fn alias_dependencies(
        &self,
        module: ModuleId,
        syntax: &hir::TypeSyntax,
        aliases: &BTreeSet<DefinitionId>,
    ) -> BTreeSet<DefinitionId> {
        let mut dependencies = BTreeSet::new();
        let mut position = 0;
        while position < syntax.atoms.len() {
            let hir::TypeAtom::Name(first) = &syntax.atoms[position] else {
                position += 1;
                continue;
            };
            let mut names = vec![first.normalized.clone()];
            position += 1;
            while syntax
                .atoms
                .get(position)
                .is_some_and(|atom| matches!(atom, hir::TypeAtom::Dot))
            {
                position += 1;
                let Some(hir::TypeAtom::Name(segment)) = syntax.atoms.get(position) else {
                    break;
                };
                names.push(segment.normalized.clone());
                position += 1;
            }
            if let Some(definition) = self.resolve_type_definition(module, &names) {
                if aliases.contains(&definition) {
                    dependencies.insert(definition);
                }
            }
        }
        dependencies
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
                let annotation = definition
                    .annotation
                    .as_ref()
                    .map(|annotation| (annotation, self.type_syntax(module.id, annotation)));
                let body = match &annotation {
                    Some((_, expected)) => self.infer_expression_with_expected(
                        module.id,
                        &definition.value,
                        expected.clone(),
                    ),
                    None => self.infer_expression(module.id, &definition.value),
                };
                if let Some((annotation, expected)) = annotation {
                    self.unify(expected, body.clone(), module.id, annotation.span, None);
                }
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
                let restriction = self.value_restriction(
                    module.id,
                    definition.mutable,
                    &definition.parameters,
                    &definition.value,
                );
                let scheme = match restriction {
                    None => self.generalize(inferred.clone()),
                    Some(reason) => {
                        self.register_restriction(&inferred, reason);
                        Scheme::mono(inferred.clone())
                    }
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
                self.check_match_coverage(module, scrutinee_type, cases, expression.span);
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
                    Operator::BooleanAnd | Operator::BooleanOr => {
                        self.unify(InferType::Bool, left_type, module, left.span, None);
                        self.unify(InferType::Bool, right_type, module, right.span, None);
                        InferType::Bool
                    }
                    Operator::Equal | Operator::NotEqual => {
                        self.equality_constraints.push((
                            module,
                            expression.span,
                            left_type.clone(),
                        ));
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
                self.infer_record(module, fields, expression.span, None)
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                let base_type = self.infer_expression(module, base);
                let mut seen = BTreeSet::new();
                for field in fields {
                    if !seen.insert(field.name.normalized.as_str()) {
                        self.push_error(
                            module,
                            field.span,
                            TypeErrorKind::DuplicateRecordField {
                                field: field.name.normalized.clone(),
                            },
                            None,
                        );
                    }
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
        let annotation = binding
            .annotation
            .as_ref()
            .map(|annotation| (annotation, self.type_syntax(module, annotation)));
        let body = match &annotation {
            Some((_, expected)) => {
                self.infer_expression_with_expected(module, &binding.value, expected.clone())
            }
            None => self.infer_expression(module, &binding.value),
        };
        if let Some((annotation, expected)) = annotation {
            self.unify(expected, body.clone(), module, annotation.span, None);
        }
        let inferred = if parameter_types.is_empty() {
            body
        } else {
            function(parameter_types, body)
        };
        if binding.recursive {
            self.unify(placeholder, inferred.clone(), module, binding.span, None);
        }
        let inferred = self.apply(inferred);
        let restriction =
            self.value_restriction(module, binding.mutable, &binding.parameters, &binding.value);
        let scheme = match restriction {
            None => self.generalize(inferred.clone()),
            Some(reason) => {
                self.register_restriction(&inferred, reason);
                Scheme::mono(inferred.clone())
            }
        };
        self.bindings.insert(key, scheme);
        self.inferred_bindings.insert(key, inferred);
    }

    fn bind_pattern(&mut self, module: ModuleId, pattern: &hir::Pattern, value: InferType) {
        match &pattern.kind {
            hir::PatternKind::Binding { id, name } => {
                if let Some(constructor) = self
                    .resolved
                    .pattern_constructor(module, pattern.id)
                    .cloned()
                {
                    self.bind_constructor_pattern(module, pattern, name, &constructor, &[], value);
                } else {
                    let key = BindingKey::new(module, *id);
                    self.bindings.insert(key, Scheme::mono(value.clone()));
                    self.inferred_bindings.insert(key, value);
                }
            }
            hir::PatternKind::Wildcard => {}
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
            hir::PatternKind::Record(fields) => {
                self.bind_record_pattern(module, pattern, fields, value);
            }
            hir::PatternKind::Constructor {
                name, arguments, ..
            } => {
                let constructor = self
                    .resolved
                    .pattern_constructor(module, pattern.id)
                    .cloned();
                if let Some(constructor) = constructor {
                    self.bind_constructor_pattern(
                        module,
                        pattern,
                        name,
                        &constructor,
                        arguments,
                        value,
                    );
                }
            }
        }
    }

    fn bind_constructor_pattern(
        &mut self,
        module: ModuleId,
        pattern: &hir::Pattern,
        name: &hir::Name,
        constructor: &DefinitionId,
        arguments: &[hir::Pattern],
        value: InferType,
    ) {
        let constructor_type =
            self.reference_type(&ReferenceTarget::Definition(constructor.clone()));
        match constructor_type {
            InferType::Function { parameters, result } => {
                if arguments.len() != parameters.len() {
                    self.push_error(
                        module,
                        pattern.span,
                        TypeErrorKind::InvalidConstructorPattern {
                            constructor: name.normalized.clone(),
                            expected: parameters.len(),
                            actual: arguments.len(),
                        },
                        None,
                    );
                }
                self.unify(*result, value, module, pattern.span, None);
                for (index, argument) in arguments.iter().enumerate() {
                    let argument_type = parameters
                        .get(index)
                        .cloned()
                        .unwrap_or_else(|| self.fresh());
                    self.bind_pattern(module, argument, argument_type);
                }
            }
            constructor_type => {
                if !arguments.is_empty() {
                    self.push_error(
                        module,
                        pattern.span,
                        TypeErrorKind::InvalidConstructorPattern {
                            constructor: name.normalized.clone(),
                            expected: 0,
                            actual: arguments.len(),
                        },
                        None,
                    );
                    for argument in arguments {
                        let argument_type = self.fresh();
                        self.bind_pattern(module, argument, argument_type);
                    }
                }
                self.unify(constructor_type, value, module, pattern.span, None);
            }
        }
    }

    fn bind_record_pattern(
        &mut self,
        module: ModuleId,
        pattern: &hir::Pattern,
        fields: &[hir::RecordPatternField],
        value: InferType,
    ) {
        let mut names = BTreeSet::new();
        for field in fields {
            if !names.insert(field.name.normalized.as_str()) {
                self.push_error(
                    module,
                    field.span,
                    TypeErrorKind::DuplicateRecordField {
                        field: field.name.normalized.clone(),
                    },
                    None,
                );
            }
        }

        let (definition, arguments) = match self.apply(value.clone()) {
            InferType::Record {
                definition,
                arguments,
            } => (definition, arguments),
            InferType::Variable(variable) => {
                let candidates = self
                    .records
                    .iter()
                    .filter(|(_, record)| {
                        names.iter().all(|name| record.fields.contains_key(*name))
                    })
                    .map(|(definition, _)| definition.clone())
                    .collect::<Vec<_>>();
                if candidates.len() != 1 {
                    self.push_error(module, pattern.span, TypeErrorKind::AmbiguousRecord, None);
                    for field in fields {
                        let field_type = self.fresh();
                        self.bind_pattern(module, &field.pattern, field_type);
                    }
                    return;
                }
                let definition = candidates[0].clone();
                let arguments = (0..self.records[&definition].parameters.len())
                    .map(|_| self.fresh())
                    .collect::<Vec<_>>();
                self.bind_variable(
                    variable,
                    InferType::Record {
                        definition: definition.clone(),
                        arguments: arguments.clone(),
                    },
                    module,
                    pattern.span,
                    None,
                );
                (definition, arguments)
            }
            InferType::Error => {
                for field in fields {
                    let field_type = self.fresh();
                    self.bind_pattern(module, &field.pattern, field_type);
                }
                return;
            }
            actual => {
                self.push_error(
                    module,
                    pattern.span,
                    TypeErrorKind::Mismatch {
                        expected: "nominal record".to_owned(),
                        actual: display_infer(&actual, &self.resolved),
                    },
                    None,
                );
                for field in fields {
                    let field_type = self.fresh();
                    self.bind_pattern(module, &field.pattern, field_type);
                }
                return;
            }
        };

        let record = self.records[&definition].clone();
        let replacements = record
            .parameters
            .iter()
            .map(|(_, variable)| *variable)
            .zip(arguments)
            .collect::<BTreeMap<_, _>>();
        for field in fields {
            let Some((_, field_type)) = record.fields.get(&field.name.normalized) else {
                self.push_error(
                    module,
                    field.span,
                    TypeErrorKind::UnknownField {
                        field: field.name.normalized.clone(),
                    },
                    None,
                );
                let field_type = self.fresh();
                self.bind_pattern(module, &field.pattern, field_type);
                continue;
            };
            self.bind_pattern(
                module,
                &field.pattern,
                replace_variables(field_type, &replacements),
            );
        }
    }

    fn check_match_coverage(
        &mut self,
        module: ModuleId,
        scrutinee: InferType,
        cases: &[hir::MatchCase],
        match_span: Span,
    ) {
        let mut all_covered = false;
        let mut booleans = BTreeSet::new();
        let mut constructors = BTreeSet::new();

        for case in cases {
            let coverage = self.pattern_coverage(module, &case.pattern);
            let already_covered = match &coverage {
                PatternCoverage::CatchAll => all_covered,
                PatternCoverage::Boolean(value) => all_covered || booleans.contains(value),
                PatternCoverage::Constructor(definition) => {
                    all_covered || constructors.contains(definition)
                }
                PatternCoverage::Other => all_covered,
            };
            if already_covered {
                let source_name = self
                    .resolved
                    .module(module)
                    .map(|module| module.hir.source_name.clone())
                    .unwrap_or_default();
                self.warnings.push(
                    Diagnostic::new(
                        codes::UNREACHABLE_MATCH_CASE,
                        Severity::Warning,
                        "match 分支不可达",
                        "match case is unreachable",
                    )
                    .with_primary_span(DiagnosticSpan::new(&source_name, case.pattern.span))
                    .with_fact("reason", "covered_by_previous_unguarded_case"),
                );
            }
            if case.guard.is_some() || already_covered {
                continue;
            }
            match coverage {
                PatternCoverage::CatchAll => all_covered = true,
                PatternCoverage::Boolean(value) => {
                    booleans.insert(value);
                }
                PatternCoverage::Constructor(definition) => {
                    constructors.insert(definition);
                }
                PatternCoverage::Other => {}
            }
        }

        if all_covered {
            return;
        }
        let witness = match self.apply(scrutinee) {
            InferType::Bool => [false, true]
                .into_iter()
                .find(|value| !booleans.contains(value))
                .map(|value| value.to_string()),
            InferType::Variant { definition, .. } => {
                self.variants.get(&definition).and_then(|variant| {
                    variant.constructors.iter().find_map(|(name, constructor)| {
                        (!constructors.contains(constructor)).then(|| name.clone())
                    })
                })
            }
            _ => None,
        };
        if let Some(witness) = witness {
            self.push_error(
                module,
                match_span,
                TypeErrorKind::NonExhaustiveMatch { witness },
                None,
            );
        }
    }

    fn pattern_coverage(&self, module: ModuleId, pattern: &hir::Pattern) -> PatternCoverage {
        if let Some(constructor) = self
            .resolved
            .pattern_constructor(module, pattern.id)
            .cloned()
        {
            let covers_payload = match &pattern.kind {
                hir::PatternKind::Binding { .. } => true,
                hir::PatternKind::Constructor { arguments, .. } => arguments
                    .iter()
                    .all(|argument| self.pattern_is_irrefutable(module, argument)),
                _ => false,
            };
            return if covers_payload {
                PatternCoverage::Constructor(constructor)
            } else {
                PatternCoverage::Other
            };
        }
        match &pattern.kind {
            hir::PatternKind::Binding { .. } | hir::PatternKind::Wildcard => {
                PatternCoverage::CatchAll
            }
            hir::PatternKind::Literal(hir::Literal::Boolean(value)) => {
                PatternCoverage::Boolean(*value)
            }
            _ => PatternCoverage::Other,
        }
    }

    fn pattern_is_irrefutable(&self, module: ModuleId, pattern: &hir::Pattern) -> bool {
        if self
            .resolved
            .pattern_constructor(module, pattern.id)
            .is_some()
        {
            return false;
        }
        match &pattern.kind {
            hir::PatternKind::Binding { .. }
            | hir::PatternKind::Wildcard
            | hir::PatternKind::Unit => true,
            hir::PatternKind::Tuple(elements) => elements
                .iter()
                .all(|element| self.pattern_is_irrefutable(module, element)),
            hir::PatternKind::Record(fields) => fields
                .iter()
                .all(|field| self.pattern_is_irrefutable(module, &field.pattern)),
            hir::PatternKind::Literal(_) | hir::PatternKind::Constructor { .. } => false,
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
                    let (reason, mutability) = if info.parameter {
                        (
                            "function parameters are immutable values",
                            "immutable_parameter",
                        )
                    } else {
                        ("local binding is not declared mutable", "immutable_binding")
                    };
                    self.push_error(
                        module,
                        place.span,
                        TypeErrorKind::InvalidAssignment {
                            reason,
                            mutability,
                            field: place.fields.first().map(|field| field.normalized.clone()),
                        },
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
                        mutability: "non_local",
                        field: place.fields.first().map(|field| field.normalized.clone()),
                    },
                    None,
                );
                InferType::Error
            }
        };
        self.inferred_place_roots
            .insert(ExpressionKey::new(module, assignment.id), value.clone());
        for field in &place.fields {
            value = self.project_field(value, field, module, field.span, true);
        }
        self.inferred_places
            .insert(ExpressionKey::new(module, assignment.id), value.clone());
        value
    }

    fn infer_expression_with_expected(
        &mut self,
        module: ModuleId,
        expression: &hir::Expression,
        expected: InferType,
    ) -> InferType {
        if let hir::ExpressionKind::Record(fields) = &expression.kind {
            let key = ExpressionKey::new(module, expression.id);
            let value = self.infer_record(module, fields, expression.span, Some(expected));
            self.inferred_expressions.insert(key, value.clone());
            value
        } else {
            self.infer_expression(module, expression)
        }
    }

    fn infer_record(
        &mut self,
        module: ModuleId,
        fields: &[hir::RecordField],
        span: Span,
        expected: Option<InferType>,
    ) -> InferType {
        let mut field_names = BTreeSet::new();
        let mut duplicate = false;
        for field in fields {
            if !field_names.insert(field.name.normalized.as_str()) {
                duplicate = true;
                self.push_error(
                    module,
                    field.span,
                    TypeErrorKind::DuplicateRecordField {
                        field: field.name.normalized.clone(),
                    },
                    None,
                );
            }
        }
        if let Some(InferType::Record {
            definition,
            arguments,
        }) = expected.map(|expected| self.apply(expected))
        {
            let Some(record) = self.records.get(&definition).cloned() else {
                return InferType::Error;
            };
            let replacements = record
                .parameters
                .iter()
                .map(|(_, variable)| *variable)
                .zip(arguments.iter().cloned())
                .collect::<BTreeMap<_, _>>();
            for field in fields {
                let Some((_, field_type)) = record.fields.get(&field.name.normalized) else {
                    self.push_error(
                        module,
                        field.span,
                        TypeErrorKind::UnknownField {
                            field: field.name.normalized.clone(),
                        },
                        None,
                    );
                    self.infer_expression(module, &field.value);
                    continue;
                };
                let expected = replace_variables(field_type, &replacements);
                let actual = self.infer_expression(module, &field.value);
                self.unify(expected, actual, module, field.span, None);
            }
            let missing = record
                .fields
                .keys()
                .filter(|name| !field_names.contains(name.as_str()))
                .cloned()
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                self.push_error(
                    module,
                    span,
                    TypeErrorKind::MissingRecordFields { fields: missing },
                    None,
                );
            }
            return InferType::Record {
                definition,
                arguments,
            };
        }
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
            if !duplicate {
                let supplied_contains = self
                    .records
                    .iter()
                    .filter(|(_, record)| {
                        record
                            .fields
                            .keys()
                            .all(|field| field_names.contains(field.as_str()))
                    })
                    .map(|(id, _)| id.clone())
                    .collect::<Vec<_>>();
                if supplied_contains.len() == 1 {
                    let definition = supplied_contains[0].clone();
                    let record = self.records[&definition].clone();
                    let arguments = record
                        .parameters
                        .iter()
                        .map(|_| self.fresh())
                        .collect::<Vec<_>>();
                    let replacements = record
                        .parameters
                        .iter()
                        .map(|(_, variable)| *variable)
                        .zip(arguments.iter().cloned())
                        .collect::<BTreeMap<_, _>>();
                    for field in fields {
                        let actual = self.infer_expression(module, &field.value);
                        if let Some((_, field_type)) = record.fields.get(&field.name.normalized) {
                            let expected = replace_variables(field_type, &replacements);
                            self.unify(expected, actual, module, field.span, None);
                        } else {
                            self.push_error(
                                module,
                                field.span,
                                TypeErrorKind::UnknownField {
                                    field: field.name.normalized.clone(),
                                },
                                None,
                            );
                        }
                    }
                    return InferType::Record {
                        definition,
                        arguments,
                    };
                }
                let containing = self
                    .records
                    .iter()
                    .filter(|(_, record)| {
                        field_names
                            .iter()
                            .all(|field| record.fields.contains_key(*field))
                    })
                    .map(|(id, _)| id.clone())
                    .collect::<Vec<_>>();
                if containing.len() == 1 {
                    let missing = self.records[&containing[0]]
                        .fields
                        .keys()
                        .filter(|field| !field_names.contains(field.as_str()))
                        .cloned()
                        .collect::<Vec<_>>();
                    self.push_error(
                        module,
                        span,
                        TypeErrorKind::MissingRecordFields { fields: missing },
                        None,
                    );
                } else {
                    self.push_error(module, span, TypeErrorKind::AmbiguousRecord, None);
                }
            }
            for field in fields {
                self.infer_expression(module, &field.value);
            }
            return InferType::Error;
        }
        let definition = candidates[0].clone();
        let record = self.records[&definition].clone();
        let arguments = record
            .parameters
            .iter()
            .map(|_| self.fresh())
            .collect::<Vec<_>>();
        let replacements = record
            .parameters
            .iter()
            .map(|(_, variable)| *variable)
            .zip(arguments.iter().cloned())
            .collect::<BTreeMap<_, _>>();
        for field in fields {
            let expected =
                replace_variables(&record.fields[&field.name.normalized].1, &replacements);
            let actual = self.infer_expression(module, &field.value);
            self.unify(expected, actual, module, field.span, None);
        }
        InferType::Record {
            definition,
            arguments,
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
        let (definition, arguments) = match target {
            InferType::Record {
                definition,
                arguments,
            } => (definition, arguments),
            InferType::Variable(variable) => {
                let candidates = self
                    .records
                    .iter()
                    .filter(|(_, record)| record.fields.contains_key(&field.normalized))
                    .map(|(id, _)| id.clone())
                    .collect::<Vec<_>>();
                if candidates.len() != 1 {
                    self.push_error(
                        module,
                        span,
                        if candidates.is_empty() {
                            TypeErrorKind::UnknownField {
                                field: field.normalized.clone(),
                            }
                        } else {
                            TypeErrorKind::AmbiguousRecord
                        },
                        None,
                    );
                    return InferType::Error;
                }
                let definition = candidates[0].clone();
                let parameter_count = self.records[&definition].parameters.len();
                let arguments = (0..parameter_count)
                    .map(|_| self.fresh())
                    .collect::<Vec<_>>();
                self.bind_variable(
                    variable,
                    InferType::Record {
                        definition: definition.clone(),
                        arguments: arguments.clone(),
                    },
                    module,
                    span,
                    None,
                );
                (definition, arguments)
            }
            InferType::Error => return InferType::Error,
            _ => {
                self.push_error(
                    module,
                    span,
                    TypeErrorKind::UnknownField {
                        field: field.normalized.clone(),
                    },
                    None,
                );
                return InferType::Error;
            }
        };
        let Some(record) = self.records.get(&definition).cloned() else {
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
                    mutability: "immutable_field",
                    field: Some(field.normalized.clone()),
                },
                None,
            );
        }
        let replacements = record
            .parameters
            .iter()
            .map(|(_, variable)| *variable)
            .zip(arguments)
            .collect::<BTreeMap<_, _>>();
        replace_variables(&field_type, &replacements)
    }

    fn infer_application(
        &mut self,
        callable: InferType,
        arguments: Vec<InferType>,
        module: ModuleId,
        span: Span,
    ) -> InferType {
        let restriction_reason = self.restriction_reason(&callable);
        let callable = self.apply(callable);
        match callable {
            InferType::Variable(variable) => {
                let result = self.fresh();
                let expected = function(arguments, result.clone());
                self.bind_variable(variable, expected, module, span, restriction_reason);
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
                        restriction_reason,
                    );
                    return InferType::Error;
                }
                for (expected, actual) in parameters.iter().zip(&arguments) {
                    self.unify(
                        expected.clone(),
                        actual.clone(),
                        module,
                        span,
                        restriction_reason,
                    );
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
                        actual: display_infer(&self.apply(actual), &self.resolved),
                    },
                    restriction_reason,
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
        self.type_syntax_with_parameters(module, syntax, &BTreeMap::new())
    }

    fn type_syntax_with_parameters(
        &mut self,
        module: ModuleId,
        syntax: &hir::TypeSyntax,
        declaration_parameters: &BTreeMap<String, InferType>,
    ) -> InferType {
        if syntax.atoms.iter().any(|atom| {
            matches!(atom, hir::TypeAtom::Variable(name) if !declaration_parameters.contains_key(&name.normalized))
        }) {
            self.push_error(
                module,
                syntax.span,
                TypeErrorKind::UnsupportedTypeSyntax,
                None,
            );
            return InferType::Error;
        }
        let mut variables = declaration_parameters.clone();
        let mut position = 0;
        let parsed = self.parse_function_type(
            module,
            &syntax.atoms,
            &mut position,
            &mut variables,
            syntax.span,
        );
        if position != syntax.atoms.len() {
            self.push_error(
                module,
                syntax.span,
                TypeErrorKind::UnsupportedTypeSyntax,
                None,
            );
            InferType::Error
        } else {
            parsed
        }
    }

    fn parse_function_type(
        &mut self,
        module: ModuleId,
        atoms: &[hir::TypeAtom],
        position: &mut usize,
        variables: &mut BTreeMap<String, InferType>,
        span: Span,
    ) -> InferType {
        let left = self.parse_product_type(module, atoms, position, variables, span);
        if atoms
            .get(*position)
            .is_some_and(|atom| matches!(atom, hir::TypeAtom::Arrow))
        {
            *position += 1;
            let right = self.parse_function_type(module, atoms, position, variables, span);
            if let InferType::Function {
                mut parameters,
                result,
            } = right
            {
                parameters.insert(0, left);
                InferType::Function { parameters, result }
            } else {
                function(vec![left], right)
            }
        } else {
            left
        }
    }

    fn parse_product_type(
        &mut self,
        module: ModuleId,
        atoms: &[hir::TypeAtom],
        position: &mut usize,
        variables: &mut BTreeMap<String, InferType>,
        span: Span,
    ) -> InferType {
        let first = self.parse_primary_type(module, atoms, position, variables, span);
        let mut elements = vec![first];
        while atoms
            .get(*position)
            .is_some_and(|atom| matches!(atom, hir::TypeAtom::Product))
        {
            *position += 1;
            elements.push(self.parse_primary_type(module, atoms, position, variables, span));
        }
        if elements.len() == 1 {
            elements.pop().expect("one product element")
        } else {
            InferType::Tuple(elements)
        }
    }

    fn parse_primary_type(
        &mut self,
        module: ModuleId,
        atoms: &[hir::TypeAtom],
        position: &mut usize,
        variables: &mut BTreeMap<String, InferType>,
        span: Span,
    ) -> InferType {
        match atoms.get(*position) {
            Some(hir::TypeAtom::Variable(name)) => {
                *position += 1;
                if let Some(value) = variables.get(&name.normalized) {
                    value.clone()
                } else {
                    let value = self.fresh();
                    variables.insert(name.normalized.clone(), value.clone());
                    value
                }
            }
            Some(hir::TypeAtom::LeftParen) => {
                *position += 1;
                let value = self.parse_function_type(module, atoms, position, variables, span);
                if atoms
                    .get(*position)
                    .is_some_and(|atom| matches!(atom, hir::TypeAtom::RightParen))
                {
                    *position += 1;
                    value
                } else {
                    self.push_error(module, span, TypeErrorKind::UnsupportedTypeSyntax, None);
                    InferType::Error
                }
            }
            Some(hir::TypeAtom::Name(_)) => {
                let mut names = Vec::new();
                let hir::TypeAtom::Name(first) = &atoms[*position] else {
                    unreachable!("match checked above")
                };
                names.push(first.normalized.clone());
                *position += 1;
                while atoms
                    .get(*position)
                    .is_some_and(|atom| matches!(atom, hir::TypeAtom::Dot))
                {
                    *position += 1;
                    let Some(hir::TypeAtom::Name(segment)) = atoms.get(*position) else {
                        self.push_error(module, span, TypeErrorKind::UnsupportedTypeSyntax, None);
                        return InferType::Error;
                    };
                    names.push(segment.normalized.clone());
                    *position += 1;
                }
                let mut arguments = Vec::new();
                if atoms
                    .get(*position)
                    .is_some_and(|atom| matches!(atom, hir::TypeAtom::LeftAngle))
                {
                    *position += 1;
                    loop {
                        arguments.push(
                            self.parse_function_type(module, atoms, position, variables, span),
                        );
                        match atoms.get(*position) {
                            Some(hir::TypeAtom::Comma) => *position += 1,
                            Some(hir::TypeAtom::RightAngle) => {
                                *position += 1;
                                break;
                            }
                            _ => {
                                self.push_error(
                                    module,
                                    span,
                                    TypeErrorKind::UnsupportedTypeSyntax,
                                    None,
                                );
                                return InferType::Error;
                            }
                        }
                    }
                }
                self.named_type(module, &names, arguments, span)
            }
            _ => {
                self.push_error(module, span, TypeErrorKind::UnsupportedTypeSyntax, None);
                InferType::Error
            }
        }
    }

    fn named_type(
        &mut self,
        module: ModuleId,
        names: &[String],
        arguments: Vec<InferType>,
        span: Span,
    ) -> InferType {
        if names.len() == 1 {
            let primitive = match names[0].as_str() {
                "Unit" => Some(InferType::Unit),
                "Bool" => Some(InferType::Bool),
                "Int" => Some(InferType::Int),
                "f64" => Some(InferType::Float64),
                "Text" => Some(InferType::Text),
                _ => None,
            };
            if let Some(primitive) = primitive {
                if arguments.is_empty() {
                    return primitive;
                }
                self.push_error(module, span, TypeErrorKind::UnsupportedTypeSyntax, None);
                return InferType::Error;
            }
            if names[0] == "List" {
                if let [element] = arguments.as_slice() {
                    return InferType::List(Box::new(element.clone()));
                }
                self.push_error(module, span, TypeErrorKind::UnsupportedTypeSyntax, None);
                return InferType::Error;
            }
        }

        let Some(definition) = self.resolve_type_definition(module, names) else {
            self.push_error(module, span, TypeErrorKind::UnsupportedTypeSyntax, None);
            return InferType::Error;
        };
        let Some(scheme) = self.definitions.get(&definition).cloned() else {
            return InferType::Error;
        };
        let parameter_ids = self
            .records
            .get(&definition)
            .map(|record| {
                record
                    .parameters
                    .iter()
                    .map(|(_, variable)| *variable)
                    .collect::<Vec<_>>()
            })
            .or_else(|| {
                self.variants.get(&definition).map(|variant| {
                    variant
                        .parameters
                        .iter()
                        .map(|(_, variable)| *variable)
                        .collect::<Vec<_>>()
                })
            })
            .unwrap_or_else(|| scheme.quantified.iter().copied().collect());
        if parameter_ids.len() != arguments.len() {
            self.push_error(
                module,
                span,
                TypeErrorKind::Arity {
                    expected: parameter_ids.len(),
                    actual: arguments.len(),
                },
                None,
            );
            return InferType::Error;
        }
        let replacements = parameter_ids
            .into_iter()
            .zip(arguments)
            .collect::<BTreeMap<_, _>>();
        replace_variables(&scheme.value, &replacements)
    }

    fn resolve_type_definition(&self, module: ModuleId, names: &[String]) -> Option<DefinitionId> {
        let id = match names {
            [name] => self
                .resolved
                .definition_id(module, name)
                .or_else(|| self.resolved.prelude_definition(name)),
            [alias, name] => self
                .resolved
                .module(module)
                .and_then(|current| current.imports.get(alias))
                .and_then(|imported| self.resolved.definition_id(*imported, name)),
            _ => None,
        }?;
        self.resolved
            .definition(id)
            .filter(|definition| definition.kind == ling_resolve::DefinitionKind::Type)
            .map(|definition| definition.id.clone())
    }

    fn unify(
        &mut self,
        expected: InferType,
        actual: InferType,
        module: ModuleId,
        span: Span,
        restriction_reason: Option<&'static str>,
    ) {
        let restriction_reason = restriction_reason
            .or_else(|| self.restriction_reason(&expected))
            .or_else(|| self.restriction_reason(&actual));
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
                    definition: left,
                    arguments: left_arguments,
                },
                InferType::Record {
                    definition: right,
                    arguments: right_arguments,
                },
            ) if left == right && left_arguments.len() == right_arguments.len() => {
                for (left, right) in left_arguments.into_iter().zip(right_arguments) {
                    self.unify(left, right, module, span, restriction_reason);
                }
            }
            (
                InferType::Variant {
                    definition: left,
                    arguments: left_arguments,
                },
                InferType::Variant {
                    definition: right,
                    arguments: right_arguments,
                },
            ) if left == right && left_arguments.len() == right_arguments.len() => {
                for (left, right) in left_arguments.into_iter().zip(right_arguments) {
                    self.unify(left, right, module, span, restriction_reason);
                }
            }
            _ => self.push_error(
                module,
                span,
                TypeErrorKind::Mismatch {
                    expected: display_infer(&expected, &self.resolved),
                    actual: display_infer(&actual, &self.resolved),
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

    fn register_restriction(&mut self, value: &InferType, reason: &'static str) {
        for variable in free_variables(value) {
            self.restricted_variables.entry(variable).or_insert(reason);
        }
    }

    fn restriction_reason(&self, value: &InferType) -> Option<&'static str> {
        free_variables(value)
            .into_iter()
            .find_map(|variable| self.restricted_variables.get(&variable).copied())
    }

    fn value_restriction(
        &self,
        module: ModuleId,
        mutable: bool,
        parameters: &[hir::Pattern],
        value: &hir::Expression,
    ) -> Option<&'static str> {
        if mutable {
            Some("mutable_binding")
        } else if !parameters.is_empty() {
            None
        } else if self.contains_mutable_record_value(module, value) {
            Some("mutable_field")
        } else if self.is_non_expansive(module, value) {
            None
        } else {
            Some("expansive_rhs")
        }
    }

    fn is_non_expansive(&self, module: ModuleId, expression: &hir::Expression) -> bool {
        match &expression.kind {
            hir::ExpressionKind::Name { reference, .. }
            | hir::ExpressionKind::Projection { reference, .. } => self
                .resolved
                .reference(module, *reference)
                .is_some_and(|target| match target {
                    ReferenceTarget::Definition(definition) => self
                        .resolved
                        .definition(definition)
                        .is_some_and(|definition| !definition.mutable),
                    ReferenceTarget::Binding(binding) => self
                        .resolved
                        .bindings()
                        .get(binding)
                        .is_some_and(|binding| !binding.mutable),
                }),
            hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => true,
            hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => elements
                .iter()
                .all(|element| self.is_non_expansive(module, element)),
            hir::ExpressionKind::Record(fields) => fields
                .iter()
                .all(|field| self.is_non_expansive(module, &field.value)),
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                self.expression_definition(module, function)
                    .and_then(|definition| self.resolved.definition(definition))
                    .is_some_and(|definition| {
                        definition.kind == ling_resolve::DefinitionKind::Constructor
                    })
                    && arguments
                        .iter()
                        .all(|argument| self.is_non_expansive(module, argument))
            }
            _ => false,
        }
    }

    fn contains_mutable_record_value(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
    ) -> bool {
        let record_is_mutable = if matches!(expression.kind, hir::ExpressionKind::Record(_)) {
            self.inferred_expressions
                .get(&ExpressionKey::new(module, expression.id))
                .cloned()
                .map(|value| self.apply(value))
                .and_then(|value| match value {
                    InferType::Record { definition, .. } => Some(definition),
                    _ => None,
                })
                .and_then(|definition| self.records.get(&definition))
                .is_some_and(|record| record.fields.values().any(|(mutable, _)| *mutable))
        } else {
            false
        };
        if record_is_mutable {
            return true;
        }
        match &expression.kind {
            hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => elements
                .iter()
                .any(|element| self.contains_mutable_record_value(module, element)),
            hir::ExpressionKind::Record(fields) => fields
                .iter()
                .any(|field| self.contains_mutable_record_value(module, &field.value)),
            hir::ExpressionKind::Application { arguments, .. } => arguments
                .iter()
                .any(|argument| self.contains_mutable_record_value(module, argument)),
            _ => false,
        }
    }

    fn expression_definition(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
    ) -> Option<&DefinitionId> {
        let reference = match &expression.kind {
            hir::ExpressionKind::Name { reference, .. }
            | hir::ExpressionKind::Projection { reference, .. } => *reference,
            _ => return None,
        };
        match self.resolved.reference(module, reference)? {
            ReferenceTarget::Definition(definition) => Some(definition),
            ReferenceTarget::Binding(_) => None,
        }
    }

    fn check_equality_constraints(&mut self) {
        let constraints = std::mem::take(&mut self.equality_constraints);
        for (module, span, value) in constraints {
            let actual = self.apply(value);
            if !self.supports_equality(&actual, &mut BTreeSet::new()) {
                self.push_error(
                    module,
                    span,
                    TypeErrorKind::UnsupportedEquality {
                        actual: display_infer(&actual, &self.resolved),
                    },
                    None,
                );
            }
        }
    }

    fn supports_equality(&self, value: &InferType, visited: &mut BTreeSet<InferType>) -> bool {
        let value = self.apply(value.clone());
        if !visited.insert(value.clone()) {
            return true;
        }
        match value {
            InferType::Unit
            | InferType::Bool
            | InferType::Int
            | InferType::Float64
            | InferType::Text
            | InferType::Error => true,
            InferType::Tuple(elements) => elements
                .iter()
                .all(|element| self.supports_equality(element, visited)),
            InferType::List(element) => self.supports_equality(&element, visited),
            InferType::Record {
                definition,
                arguments,
            } => self.records.get(&definition).is_some_and(|record| {
                let replacements = record
                    .parameters
                    .iter()
                    .map(|(_, variable)| *variable)
                    .zip(arguments)
                    .collect::<BTreeMap<_, _>>();
                record.fields.values().all(|(_, field)| {
                    self.supports_equality(&replace_variables(field, &replacements), visited)
                })
            }),
            InferType::Variant {
                definition,
                arguments,
            } => self.variants.get(&definition).is_some_and(|variant| {
                let replacements = variant
                    .parameters
                    .iter()
                    .map(|(_, variable)| *variable)
                    .zip(arguments)
                    .collect::<BTreeMap<_, _>>();
                variant.cases.values().all(|payload| {
                    payload.as_ref().is_none_or(|payload| {
                        self.supports_equality(&replace_variables(payload, &replacements), visited)
                    })
                })
            }),
            InferType::Function { .. } | InferType::Variable(_) => false,
        }
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

fn display_nominal(arena: &TypeArena, definition: &DefinitionId, arguments: &[TypeId]) -> String {
    if arguments.is_empty() {
        definition.to_string()
    } else {
        format!(
            "{}<{}>",
            definition,
            arguments
                .iter()
                .map(|argument| arena.display(*argument))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn display_infer(value: &InferType, resolved: &ResolvedProgram) -> String {
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
                .map(|element| display_infer(element, resolved))
                .collect::<Vec<_>>()
                .join(" * ")
        ),
        InferType::List(element) => format!("List<{}>", display_infer(element, resolved)),
        InferType::Function { parameters, result } => {
            let mut parts = parameters
                .iter()
                .map(|parameter| match parameter {
                    InferType::Function { .. } => {
                        format!("({})", display_infer(parameter, resolved))
                    }
                    _ => display_infer(parameter, resolved),
                })
                .collect::<Vec<_>>();
            parts.push(display_infer(result, resolved));
            parts.join(" -> ")
        }
        InferType::Record {
            definition,
            arguments,
        }
        | InferType::Variant {
            definition,
            arguments,
        } => {
            let name = nominal_display_name(resolved, definition);
            if arguments.is_empty() {
                name
            } else {
                format!(
                    "{}<{}>",
                    name,
                    arguments
                        .iter()
                        .map(|argument| display_infer(argument, resolved))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        InferType::Variable(variable) => format!("'t{variable}"),
        InferType::Error => "<error>".to_owned(),
    }
}

fn display_resolved_type(arena: &TypeArena, resolved: &ResolvedProgram, id: TypeId) -> String {
    match arena.get(id) {
        Type::Unit => "Unit".to_owned(),
        Type::Bool => "Bool".to_owned(),
        Type::Int => "Int".to_owned(),
        Type::Float64 => "f64".to_owned(),
        Type::Text => "Text".to_owned(),
        Type::Tuple(elements) => format!(
            "({})",
            elements
                .iter()
                .map(|element| display_resolved_type(arena, resolved, *element))
                .collect::<Vec<_>>()
                .join(" * ")
        ),
        Type::List(element) => {
            format!("List<{}>", display_resolved_type(arena, resolved, *element))
        }
        Type::Function { parameters, result } => {
            let mut parts = parameters
                .iter()
                .map(|parameter| match arena.get(*parameter) {
                    Type::Function { .. } => {
                        format!("({})", display_resolved_type(arena, resolved, *parameter))
                    }
                    _ => display_resolved_type(arena, resolved, *parameter),
                })
                .collect::<Vec<_>>();
            parts.push(display_resolved_type(arena, resolved, *result));
            parts.join(" -> ")
        }
        Type::NominalRecord {
            definition,
            arguments,
        }
        | Type::NominalVariant {
            definition,
            arguments,
        } => {
            let name = nominal_display_name(resolved, definition);
            if arguments.is_empty() {
                name
            } else {
                format!(
                    "{}<{}>",
                    name,
                    arguments
                        .iter()
                        .map(|argument| display_resolved_type(arena, resolved, *argument))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        Type::Variable(variable) => format!("'t{variable}"),
        Type::Error => "<error>".to_owned(),
    }
}

fn nominal_display_name(resolved: &ResolvedProgram, definition: &DefinitionId) -> String {
    let info = resolved
        .definition(definition)
        .expect("a typed nominal type has a resolved definition");
    let ambiguous = resolved
        .definitions()
        .values()
        .filter(|candidate| candidate.kind == DefinitionKind::Type && candidate.name == info.name)
        .nth(1)
        .is_some();
    if ambiguous {
        format!("{}.{}", info.module_name, info.name)
    } else {
        info.name.clone()
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

    fn resolved_modules(sources: &[(&str, &str)]) -> ResolvedProgram {
        let programs = sources
            .iter()
            .enumerate()
            .map(|(index, (name, text))| {
                let source = SourceFile::from_bytes(
                    SourceId::new(u32::try_from(index).expect("test source index fits")),
                    *name,
                    text.as_bytes().to_vec(),
                )
                .expect("valid source");
                let parsed = parse(&source);
                assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
                let ast = lower_ast(&source, &parsed).expect("valid AST");
                hir::lower(source.name(), &ast).expect("valid HIR")
            })
            .collect();
        ling_resolve::resolve(programs, "Main").expect("resolves")
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
    fn boolean_operators_require_bool_operands_at_original_byte_spans() {
        check(resolved(concat!(
            "module Main\n\n",
            "let conjunction = true && false\n",
            "let disjunction = false || true\n",
        )))
        .expect("Bool operands type-check");

        for (text, marker) in [
            ("module Main\n\nlet 结果 = 1 && true\n", "1"),
            ("module Main\n\nlet 结果 = false || 2\n", "2"),
        ] {
            let errors = check(resolved(text)).expect_err("Int operand must fail");
            let error = errors
                .iter()
                .find(|error| matches!(error.kind, TypeErrorKind::Mismatch { .. }))
                .expect("type mismatch is reported");
            let start = u32::try_from(text.find(marker).expect("marker exists"))
                .expect("test source offset fits u32");
            assert_eq!(error.span.start().get(), start);
            assert_eq!(error.span.end().get(), start + 1);
            assert!(matches!(
                &error.kind,
                TypeErrorKind::Mismatch { expected, actual }
                    if expected == "Bool" && actual == "Int"
            ));
        }
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

    #[test]
    fn value_restriction_preserves_values_and_reports_stable_reasons() {
        check(resolved(concat!(
            "module Main\n\n",
            "type Box<'a> = { value: 'a }\n\n",
            "let test () =\n",
            "    let empty = []\n",
            "    let intValues: List<Int> = empty\n",
            "    let textValues: List<Text> = empty\n",
            "    let box = { value = [] }\n",
            "    let intBox: Box<List<Int>> = box\n",
            "    let textBox: Box<List<Text>> = box\n",
            "    let option = Some []\n",
            "    let intOption: Option<List<Int>> = option\n",
            "    let textOption: Option<List<Text>> = option\n",
            "    ()\n",
        )))
        .expect("non-expansive list, record, and constructor values generalize");

        let mutable_binding = check(resolved(concat!(
            "module Main\n\n",
            "let test () =\n",
            "    let mutable id value = value\n",
            "    id 1\n",
            "    id \"text\"\n",
            "    ()\n",
        )))
        .expect_err("mutable bindings remain monomorphic");
        let mutable_binding_error = mutable_binding
            .iter()
            .find(|error| {
                matches!(error.kind, TypeErrorKind::Mismatch { .. })
                    && error.restriction_reason == Some("mutable_binding")
            })
            .expect("mutable restriction is attached to the mismatch");
        let diagnostic: serde_json::Value = serde_json::from_str(
            &mutable_binding_error
                .to_diagnostic()
                .render_json()
                .expect("diagnostic serializes"),
        )
        .expect("diagnostic JSON parses");
        assert_eq!(diagnostic["facts"]["generalization"], "restricted");
        assert_eq!(diagnostic["facts"]["restriction_reason"], "mutable_binding");
        let expansive = check(resolved(concat!(
            "module Main\n\n",
            "type TextValues = { values: List<Text> }\n\n",
            "let test () =\n",
            "    let values = if true then [] else []\n",
            "    sum values\n",
            "    let texts: TextValues = { values = values }\n",
            "    ()\n",
        )))
        .expect_err("expansive bindings remain monomorphic");
        assert!(expansive.iter().any(|error| {
            matches!(error.kind, TypeErrorKind::Mismatch { .. })
                && error.restriction_reason == Some("expansive_rhs")
        }));

        let mutable_field = check(resolved(concat!(
            "module Main\n\n",
            "type MutableBox<'a> = { mutable value: 'a }\n\n",
            "let test () =\n",
            "    let box = { value = [] }\n",
            "    let ints: MutableBox<List<Int>> = box\n",
            "    let texts: MutableBox<List<Text>> = box\n",
            "    ()\n",
        )))
        .expect_err("records with mutable fields remain monomorphic");
        assert!(mutable_field.iter().any(|error| {
            matches!(error.kind, TypeErrorKind::Mismatch { .. })
                && error.restriction_reason == Some("mutable_field")
        }));
    }

    #[test]
    fn equality_accepts_seed_values_and_rejects_functions() {
        check(resolved(concat!(
            "module Main\n\n",
            "type Point = { x: Int; y: Int }\n",
            "type State =\n",
            "    | Healthy\n",
            "    | Hurt of Int\n\n",
            "let primitive = 1.5 == 1.5\n",
            "let tupleValue = (1, \"x\") == (1, \"x\")\n",
            "let listValue = [1; 2] == [1; 2]\n",
            "let recordValue = { x = 1; y = 2 } == { x = 1; y = 2 }\n",
            "let variantValue = Hurt 1 == Hurt 1\n",
        )))
        .expect("all implemented structural Seed values support equality");

        let function = check(resolved(concat!(
            "module Main\n\n",
            "let identity value = value\n",
            "let invalid = identity == identity\n",
        )))
        .expect_err("functions do not have structural equality");
        let error = function
            .iter()
            .find(|error| matches!(error.kind, TypeErrorKind::UnsupportedEquality { .. }))
            .expect("function equality has a dedicated type error");
        assert_eq!(error.to_diagnostic().code(), codes::UNSUPPORTED_EQUALITY);
    }

    #[test]
    fn instantiates_generic_records_and_prelude_variants_independently() {
        let typed = check(resolved(concat!(
            "module Main\n\n",
            "type Box<'a> =\n",
            "    { value: 'a }\n\n",
            "let intBox = { value = 1 }\n",
            "let textBox = { value = \"text\" }\n",
            "let intOption: Option<Int> = Some 1\n",
            "let textOption: Option<Text> = Some \"text\"\n",
            "let intResult: Result<Int, Text> = Ok 1\n",
            "let textError: Result<Int, Text> = Error \"failed\"\n",
            "let unwrapOrZero option =\n",
            "    match option with\n",
            "    | Some value -> value\n",
            "    | None -> 0\n",
        )))
        .expect("generic nominal declarations instantiate per use");
        for definition in [PreludeDefinition::Option, PreludeDefinition::Result] {
            assert!(
                typed
                    .variants()
                    .contains_key(typed.resolved().prelude_id(definition))
            );
        }
    }

    #[test]
    fn rejects_incomplete_and_duplicate_record_literals() {
        check(resolved(concat!(
            "module Main\n\n",
            "type Person =\n",
            "    { name: Text\n",
            "      age: Int }\n\n",
            "let person = { age = 1; name = \"Ling\" }\n",
        )))
        .expect("record field order is not semantically significant");

        let missing = check(resolved(concat!(
            "module Main\n\n",
            "type Person =\n",
            "    { name: Text\n",
            "      age: Int }\n\n",
            "let person = { name = \"Ling\" }\n",
        )))
        .expect_err("missing record fields must fail");
        assert!(missing.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::MissingRecordFields { fields } if fields == &["age".to_owned()]
        )));

        let duplicate = check(resolved(concat!(
            "module Main\n\n",
            "type Person = { name: Text }\n\n",
            "let person = { name = \"Ling\"; name = \"Zero\" }\n",
        )))
        .expect_err("duplicate record fields must fail");
        assert!(duplicate.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::DuplicateRecordField { field } if field == "name"
        )));

        let unknown = check(resolved(concat!(
            "module Main\n\n",
            "type Person = { name: Text; age: Int }\n\n",
            "let person = { name = \"Ling\"; age = 1; city = \"Shanghai\" }\n",
        )))
        .expect_err("unknown record fields must fail precisely");
        assert!(unknown.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::UnknownField { field } if field == "city"
        )));

        let wrong_type = check(resolved(concat!(
            "module Main\n\n",
            "type Person = { name: Text; age: Int }\n\n",
            "let person = { name = \"Ling\"; age = \"old\" }\n",
        )))
        .expect_err("record field values must match their declared types");
        assert!(
            wrong_type
                .iter()
                .any(|error| matches!(error.kind, TypeErrorKind::Mismatch { .. }))
        );
    }

    #[test]
    fn expected_nominal_type_disambiguates_identical_record_fields() {
        let typed = check(resolved(concat!(
            "module Main\n\n",
            "type WorldPoint = { x: Int; y: Int }\n",
            "type ScreenPoint = { x: Int; y: Int }\n\n",
            "let origin: WorldPoint = { x = 0; y = 0 }\n",
        )))
        .expect("an explicit nominal annotation disambiguates a record literal");
        let entry = typed.resolved().entry();
        let origin = typed
            .resolved()
            .definition_id(entry, "origin")
            .expect("origin definition");
        let origin_type = typed.definition_type(origin).expect("origin type");
        let world_point = typed
            .resolved()
            .definition_id(entry, "WorldPoint")
            .expect("WorldPoint definition");
        assert!(matches!(
            typed.arena().get(origin_type),
            Type::NominalRecord { definition, .. } if definition == world_point
        ));

        let errors = check(resolved(concat!(
            "module Main\n\n",
            "type WorldPoint = { x: Int; y: Int }\n",
            "type ScreenPoint = { x: Int; y: Int }\n\n",
            "let origin = { x = 0; y = 0 }\n",
        )))
        .expect_err("the same literal is ambiguous without an expected nominal type");
        assert!(
            errors
                .iter()
                .any(|error| matches!(error.kind, TypeErrorKind::AmbiguousRecord))
        );
    }

    #[test]
    fn validates_constructor_pattern_arity() {
        let errors = check(resolved(concat!(
            "module Main\n\n",
            "type State =\n",
            "    | Healthy\n",
            "    | Hurt of Int\n\n",
            "let invalid state =\n",
            "    match state with\n",
            "    | Hurt -> 1\n",
            "    | Healthy -> 0\n",
        )))
        .expect_err("payload constructors require a payload pattern");
        assert!(errors.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::InvalidConstructorPattern {
                constructor,
                expected: 1,
                actual: 0,
            } if constructor == "Hurt"
        )));
    }

    #[test]
    fn checks_boolean_and_variant_exhaustiveness_with_stable_witnesses() {
        check(resolved(concat!(
            "module Main\n\n",
            "let complete value =\n",
            "    match value with\n",
            "    | true -> 1\n",
            "    | false -> 0\n",
        )))
        .expect("both Boolean values are exhaustive");

        let boolean = check(resolved(concat!(
            "module Main\n\n",
            "let incomplete value =\n",
            "    match value with\n",
            "    | true -> 1\n",
        )))
        .expect_err("a single Boolean branch is incomplete");
        assert!(boolean.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::NonExhaustiveMatch { witness } if witness == "false"
        )));

        let variant = check(resolved(concat!(
            "module Main\n\n",
            "type State =\n",
            "    | Healthy\n",
            "    | Hurt of Int\n",
            "    | Dead\n\n",
            "let incomplete state =\n",
            "    match state with\n",
            "    | Healthy -> 1\n",
        )))
        .expect_err("missing variant constructors must fail");
        assert!(variant.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::NonExhaustiveMatch { witness } if witness == "Dead"
        )));
    }

    #[test]
    fn reports_unreachable_cases_and_ignores_guards_for_coverage() {
        let typed = check(resolved(concat!(
            "module Main\n\n",
            "let duplicate value =\n",
            "    match value with\n",
            "    | true -> 1\n",
            "    | true -> 2\n",
            "    | false -> 0\n",
        )))
        .expect("unreachable cases are warnings");
        assert_eq!(typed.warnings().len(), 1);
        assert_eq!(typed.warnings()[0].code(), codes::UNREACHABLE_MATCH_CASE);

        let errors = check(resolved(concat!(
            "module Main\n\n",
            "let guarded value =\n",
            "    match value with\n",
            "    | true when true -> 1\n",
            "    | false -> 0\n",
        )))
        .expect_err("a guarded branch does not prove coverage");
        assert!(errors.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::NonExhaustiveMatch { witness } if witness == "true"
        )));
    }

    #[test]
    fn types_wildcard_tuple_and_refutable_constructor_payload_patterns() {
        check(resolved(concat!(
            "module Main\n\n",
            "type PairBox =\n",
            "    | Pair of Int * Int\n",
            "    | Empty\n\n",
            "let sumPair value =\n",
            "    match value with\n",
            "    | Pair (left, right) -> left + right\n",
            "    | Empty -> 0\n\n",
            "let ignore value =\n",
            "    match value with\n",
            "    | _ -> 0\n",
        )))
        .expect("tuple payloads and wildcard cases type-check");

        let errors = check(resolved(concat!(
            "module Main\n\n",
            "type State =\n",
            "    | Healthy\n",
            "    | Hurt of Int\n\n",
            "let describe state =\n",
            "    match state with\n",
            "    | Healthy -> 0\n",
            "    | Hurt 0 -> 1\n",
        )))
        .expect_err("a literal payload pattern does not cover its constructor");
        assert!(errors.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::NonExhaustiveMatch { witness } if witness == "Hurt"
        )));
    }

    #[test]
    fn types_record_patterns_and_rejects_invalid_pattern_fields() {
        check(resolved(concat!(
            "module Main\n\n",
            "type Point = { x: Int; y: Int }\n\n",
            "let xCoordinate point =\n",
            "    match point with\n",
            "    | { x = value; y = _ } -> value\n",
        )))
        .expect("record fields constrain and bind a nominal record pattern");

        let unknown = check(resolved(concat!(
            "module Main\n\n",
            "type Point = { x: Int; y: Int }\n\n",
            "let invalid =\n",
            "    match { x = 1; y = 2 } with\n",
            "    | { z = _ } -> 0\n",
        )))
        .expect_err("unknown record-pattern fields must fail");
        assert!(unknown.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::UnknownField { field } if field == "z"
        )));

        let duplicate = check(resolved(concat!(
            "module Main\n\n",
            "type Point = { x: Int; y: Int }\n\n",
            "let invalid =\n",
            "    match { x = 1; y = 2 } with\n",
            "    | { x = left; x = right } -> left + right\n",
        )))
        .expect_err("duplicate record-pattern fields must fail");
        assert!(duplicate.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::DuplicateRecordField { field } if field == "x"
        )));
    }

    #[test]
    fn enforces_return_type_annotations() {
        check(resolved("module Main\n\nlet identity value: Int = value\n"))
            .expect("matching return annotation type-checks");
        let errors = check(resolved(
            "module Main\n\nlet invalid value: Text = value + 1\n",
        ))
        .expect_err("mismatched return annotation must fail");
        assert!(
            errors
                .iter()
                .any(|error| matches!(error.kind, TypeErrorKind::Mismatch { .. }))
        );

        let undeclared = check(resolved("module Main\n\ntype Box<'a> = { value: 'b }\n"))
            .expect_err("nominal declarations cannot use undeclared type parameters");
        assert!(
            undeclared
                .iter()
                .any(|error| matches!(error.kind, TypeErrorKind::UnsupportedTypeSyntax))
        );

        let cycle = check(resolved(concat!(
            "module Main\n\n",
            "type First<'a> = Second<'a>\n",
            "type Second<'a> = First<'a>\n",
        )))
        .expect_err("recursive aliases are rejected deterministically");
        assert_eq!(
            cycle
                .iter()
                .filter(|error| matches!(error.kind, TypeErrorKind::UnsupportedTypeSyntax))
                .count(),
            2
        );
    }

    #[test]
    fn enforces_mutable_roots_fields_and_parameter_value_semantics() {
        check(resolved(concat!(
            "module Main\n\n",
            "type Counter = { mutable value: Int }\n\n",
            "let update () =\n",
            "    let mutable counter = { value = 0 }\n",
            "    counter.value <- 1\n",
            "    counter\n",
        )))
        .expect("a mutable field on a mutable local is assignable");

        let immutable_root = check(resolved(concat!(
            "module Main\n\n",
            "type Counter = { mutable value: Int }\n\n",
            "let update () =\n",
            "    let counter = { value = 0 }\n",
            "    counter.value <- 1\n",
        )))
        .expect_err("an immutable local is not an assignment root");
        assert!(immutable_root.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::InvalidAssignment {
                mutability: "immutable_binding",
                field: Some(field),
                ..
            } if field == "value"
        )));

        let immutable_field = check(resolved(concat!(
            "module Main\n\n",
            "type Counter = { value: Int }\n\n",
            "let update () =\n",
            "    let mutable counter = { value = 0 }\n",
            "    counter.value <- 1\n",
        )))
        .expect_err("an immutable field is not assignable");
        assert!(immutable_field.iter().any(|error| matches!(
            &error.kind,
            TypeErrorKind::InvalidAssignment {
                mutability: "immutable_field",
                field: Some(field),
                ..
            } if field == "value"
        )));

        let parameter = check(resolved(concat!(
            "module Main\n\n",
            "type Counter = { mutable value: Int }\n\n",
            "let update counter = counter.value <- 1\n",
        )))
        .expect_err("function parameters use value semantics and are immutable roots");
        assert!(parameter.iter().any(|error| matches!(
            error.kind,
            TypeErrorKind::InvalidAssignment {
                mutability: "immutable_parameter",
                ..
            }
        )));
    }

    #[test]
    fn checks_cross_module_generic_aliases_and_constructor_patterns() {
        check(resolved_modules(&[
            (
                "Main.ling",
                concat!(
                    "module Main\n\n",
                    "import Domain.State as State\n\n",
                    "let describe value =\n",
                    "    match value with\n",
                    "    | State.Hurt amount -> amount\n",
                    "    | State.Healthy -> 0\n",
                    "\nlet result = describe (State.Hurt 30)\n",
                    "let foreign: State.Box<Int> = { value = 2 }\n",
                ),
            ),
            (
                "Domain/State.ling",
                concat!(
                    "module Domain.State\n\n",
                    "type Box<'a> = { value: 'a }\n",
                    "type Alias<'a> = Box<'a>\n",
                    "type Forward<'a> = Later<'a>\n",
                    "type Later<'a> = Box<'a>\n",
                    "let aliasValue: Alias<Int> = { value = 1 }\n",
                    "let forwardValue: Forward<Int> = { value = 2 }\n",
                    "type State =\n",
                    "    | Healthy\n",
                    "    | Hurt of Int\n",
                ),
            ),
        ]))
        .expect("qualified constructors and generic aliases type-check across modules");
    }
}
