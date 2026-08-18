//! Strict checked-core interpreter with injected host capabilities.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_hir as hir;
use ling_resolve::{
    BindingKey, Builtin, DefinitionId, DefinitionKind, DefinitionOrigin, ExpressionKey, ModuleId,
    ReferenceTarget,
};
use ling_semantic::ProgramSnapshot;
use ling_source::Span;
use ling_types::Type;
use num_bigint::BigInt;

/// Host console capability. `text` already contains Ling's canonical LF.
pub trait Console {
    fn write(&mut self, text: &str) -> Result<(), HostError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostErrorCategory {
    BrokenPipe,
    PermissionDenied,
    Interrupted,
    Other,
}

impl HostErrorCategory {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::BrokenPipe => "broken_pipe",
            Self::PermissionDenied => "permission_denied",
            Self::Interrupted => "interrupted",
            Self::Other => "other",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostError {
    category: HostErrorCategory,
}

impl HostError {
    #[must_use]
    pub const fn new(category: HostErrorCategory) -> Self {
        Self { category }
    }

    #[must_use]
    pub const fn category(&self) -> HostErrorCategory {
        self.category
    }
}

impl fmt::Display for HostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.category.name())
    }
}

impl Error for HostError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeFault {
    pub kind: RuntimeFaultKind,
    pub source_name: String,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeFaultKind {
    HostCapability {
        operation: &'static str,
        category: HostErrorCategory,
    },
    InvalidFormatPlaceholderCount {
        count: usize,
    },
    DivisionByZero,
    InvalidCheckedCore {
        invariant: &'static str,
    },
}

impl RuntimeFault {
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        let (zh, en, category) = match &self.kind {
            RuntimeFaultKind::HostCapability {
                operation,
                category,
            } => (
                format!("宿主 Capability 操作“{operation}”失败"),
                format!("host capability operation `{operation}` failed"),
                category.name(),
            ),
            RuntimeFaultKind::InvalidFormatPlaceholderCount { count } => (
                format!("Text.format 要求恰好一个占位符，实际为 {count}"),
                format!("Text.format requires exactly one placeholder, found {count}"),
                "invalid_format",
            ),
            RuntimeFaultKind::DivisionByZero => (
                "整数除数不能为零".to_owned(),
                "integer divisor cannot be zero".to_owned(),
                "division_by_zero",
            ),
            RuntimeFaultKind::InvalidCheckedCore { invariant } => (
                format!("Checked Core 不变量失效：{invariant}"),
                format!("checked-core invariant failed: {invariant}"),
                "checked_core_invariant",
            ),
        };
        Diagnostic::new(codes::RUNTIME_FAULT, Severity::Error, zh, en)
            .with_primary_span(DiagnosticSpan::new(&self.source_name, self.span))
            .with_fact("category", category)
    }
}

impl fmt::Display for RuntimeFault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.kind)
    }
}

impl Error for RuntimeFault {}

#[derive(Default)]
pub struct MemoryConsole {
    output: String,
    failure: Option<HostErrorCategory>,
}

impl MemoryConsole {
    #[must_use]
    pub fn failing(category: HostErrorCategory) -> Self {
        Self {
            output: String::new(),
            failure: Some(category),
        }
    }

    #[must_use]
    pub fn output(&self) -> &str {
        &self.output
    }
}

impl Console for MemoryConsole {
    fn write(&mut self, text: &str) -> Result<(), HostError> {
        if let Some(category) = self.failure {
            Err(HostError::new(category))
        } else {
            self.output.push_str(text);
            Ok(())
        }
    }
}

/// Executes a previously validated main definition from a checked snapshot.
pub fn execute_main(
    snapshot: &ProgramSnapshot,
    main: &DefinitionId,
    console: &mut dyn Console,
) -> Result<(), RuntimeFault> {
    Interpreter::new(snapshot, console).execute_main(main)
}

type Cell = Rc<RefCell<Value>>;
type Environment = BTreeMap<BindingKey, Cell>;

#[derive(Clone, Debug)]
enum Value {
    Unit,
    Bool(bool),
    Int(BigInt),
    Float64(f64),
    Text(String),
    Tuple(Vec<Self>),
    List(Vec<Self>),
    Record {
        definition: DefinitionId,
        fields: BTreeMap<String, Self>,
    },
    Variant {
        definition: DefinitionId,
        case: String,
        payload: Option<Box<Self>>,
    },
    Closure(Box<Closure>),
    Builtin {
        builtin: Builtin,
        arguments: Vec<Self>,
    },
    Constructor {
        definition: DefinitionId,
        case: String,
    },
}

#[derive(Clone, Debug)]
struct Closure {
    module: ModuleId,
    parameters: Vec<hir::Pattern>,
    body: hir::Expression,
    environment: Environment,
}

struct Interpreter<'snapshot, 'console> {
    snapshot: &'snapshot ProgramSnapshot,
    console: &'console mut dyn Console,
}

impl<'snapshot, 'console> Interpreter<'snapshot, 'console> {
    const fn new(snapshot: &'snapshot ProgramSnapshot, console: &'console mut dyn Console) -> Self {
        Self { snapshot, console }
    }

    fn execute_main(&mut self, main: &DefinitionId) -> Result<(), RuntimeFault> {
        let checked = self.snapshot.checked();
        let Some(info) = checked.typed().resolved().definition(main) else {
            return Err(self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "main DefinitionId is absent",
            }));
        };
        if !matches!(info.origin, DefinitionOrigin::User { .. }) || info.name != "main" {
            return Err(self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "main DefinitionId was not validated",
            }));
        }
        let callable = self.definition_value(main)?;
        let result = self.apply(
            callable,
            vec![Value::Unit],
            info.span
                .unwrap_or_else(|| checked.typed().resolved().entry_module().hir.module.span),
        )?;
        if matches!(result, Value::Unit) {
            Ok(())
        } else {
            Err(self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "validated main returned a non-Unit value",
            }))
        }
    }

    fn eval_expression(
        &mut self,
        module: ModuleId,
        expression: &hir::Expression,
        environment: &mut Environment,
    ) -> Result<Value, RuntimeFault> {
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                let mut result = Value::Unit;
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            self.eval_local_binding(module, binding, environment)?;
                            result = Value::Unit;
                        }
                        hir::SequenceElement::Expression(expression) => {
                            result = self.eval_expression(module, expression, environment)?;
                        }
                    }
                }
                Ok(result)
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => match self.eval_expression(module, condition, environment)? {
                Value::Bool(true) => self.eval_expression(module, then_branch, environment),
                Value::Bool(false) => self.eval_expression(module, else_branch, environment),
                _ => Err(self.fault(module, expression.span, "if condition is not Bool")),
            },
            hir::ExpressionKind::Match { scrutinee, cases } => {
                let value = self.eval_expression(module, scrutinee, environment)?;
                for case in cases {
                    let mut case_environment = environment.clone();
                    if self.match_pattern(module, &case.pattern, &value, &mut case_environment)? {
                        if let Some(guard) = &case.guard {
                            if !matches!(
                                self.eval_expression(module, guard, &mut case_environment)?,
                                Value::Bool(true)
                            ) {
                                continue;
                            }
                        }
                        return self.eval_expression(module, &case.body, &mut case_environment);
                    }
                }
                Err(self.fault(
                    module,
                    expression.span,
                    "checked match had no matching case",
                ))
            }
            hir::ExpressionKind::Assignment { place, value } => {
                let value = self.eval_expression(module, value, environment)?;
                let target = self.reference_target(module, place.root_reference)?;
                let ReferenceTarget::Binding(binding) = target else {
                    return Err(self.fault(
                        module,
                        place.span,
                        "assignment root is not a local binding",
                    ));
                };
                let Some(cell) = environment.get(&binding) else {
                    return Err(self.fault(module, place.span, "assignment cell is absent"));
                };
                if place.fields.is_empty() {
                    *cell.borrow_mut() = value;
                } else {
                    let mut root = cell.borrow_mut();
                    assign_fields(&mut root, &place.fields, value)
                        .map_err(|invariant| self.fault(module, place.span, invariant))?;
                }
                Ok(Value::Unit)
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                let callable = self.eval_expression(module, function, environment)?;
                let mut values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    values.push(self.eval_expression(module, argument, environment)?);
                }
                self.apply(callable, values, expression.span)
            }
            hir::ExpressionKind::Projection {
                reference,
                target,
                field,
            } => {
                if self
                    .snapshot
                    .checked()
                    .typed()
                    .resolved()
                    .reference(module, *reference)
                    .is_some()
                {
                    return self.reference_value(module, *reference, environment);
                }
                match self.eval_expression(module, target, environment)? {
                    Value::Record { fields, .. } => {
                        fields.get(&field.normalized).cloned().ok_or_else(|| {
                            self.fault(module, expression.span, "record field is absent")
                        })
                    }
                    _ => Err(self.fault(
                        module,
                        expression.span,
                        "projection target is not a record",
                    )),
                }
            }
            hir::ExpressionKind::Name { reference, .. } => {
                self.reference_value(module, *reference, environment)
            }
            hir::ExpressionKind::Binary {
                operator,
                left,
                right,
            } => {
                let left = self.eval_expression(module, left, environment)?;
                let right = self.eval_expression(module, right, environment)?;
                self.eval_binary(module, expression.span, *operator, left, right)
            }
            hir::ExpressionKind::Unary { operator, operand } => {
                let value = self.eval_expression(module, operand, environment)?;
                match (operator, value) {
                    (hir::UnaryOperator::Positive, Value::Int(value)) => Ok(Value::Int(value)),
                    (hir::UnaryOperator::Negative, Value::Int(value)) => Ok(Value::Int(-value)),
                    _ => Err(self.fault(module, expression.span, "unary operand is not Int")),
                }
            }
            hir::ExpressionKind::Literal(literal) => match literal {
                hir::Literal::Integer { .. } => self
                    .snapshot
                    .checked()
                    .typed()
                    .integer(ExpressionKey::new(module, expression.id))
                    .cloned()
                    .map(Value::Int)
                    .ok_or_else(|| self.fault(module, expression.span, "typed integer is absent")),
                hir::Literal::Float(value) => value
                    .parse::<f64>()
                    .map(Value::Float64)
                    .map_err(|_| self.fault(module, expression.span, "typed float is invalid")),
                hir::Literal::Text(value) => Ok(Value::Text(value.clone())),
                hir::Literal::Boolean(value) => Ok(Value::Bool(*value)),
            },
            hir::ExpressionKind::Unit => Ok(Value::Unit),
            hir::ExpressionKind::Tuple(elements) => {
                let mut values = Vec::with_capacity(elements.len());
                for element in elements {
                    values.push(self.eval_expression(module, element, environment)?);
                }
                Ok(Value::Tuple(values))
            }
            hir::ExpressionKind::List(elements) => {
                let mut values = Vec::with_capacity(elements.len());
                for element in elements {
                    values.push(self.eval_expression(module, element, environment)?);
                }
                Ok(Value::List(values))
            }
            hir::ExpressionKind::Record(fields) => {
                let type_id = self
                    .snapshot
                    .checked()
                    .typed()
                    .expression_type(ExpressionKey::new(module, expression.id))
                    .ok_or_else(|| self.fault(module, expression.span, "record type is absent"))?;
                let Type::NominalRecord { definition, .. } =
                    self.snapshot.checked().typed().arena().get(type_id)
                else {
                    return Err(self.fault(module, expression.span, "record has non-record type"));
                };
                let mut values = BTreeMap::new();
                for field in fields {
                    values.insert(
                        field.name.normalized.clone(),
                        self.eval_expression(module, &field.value, environment)?,
                    );
                }
                Ok(Value::Record {
                    definition: definition.clone(),
                    fields: values,
                })
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                let Value::Record {
                    definition,
                    fields: mut values,
                } = self.eval_expression(module, base, environment)?
                else {
                    return Err(self.fault(
                        module,
                        expression.span,
                        "record update base is not a record",
                    ));
                };
                for field in fields {
                    values.insert(
                        field.name.normalized.clone(),
                        self.eval_expression(module, &field.value, environment)?,
                    );
                }
                Ok(Value::Record {
                    definition,
                    fields: values,
                })
            }
        }
    }

    fn eval_local_binding(
        &mut self,
        module: ModuleId,
        binding: &hir::LocalBinding,
        environment: &mut Environment,
    ) -> Result<(), RuntimeFault> {
        let key = BindingKey::new(module, binding.id);
        let cell = Rc::new(RefCell::new(Value::Unit));
        if binding.recursive {
            environment.insert(key, Rc::clone(&cell));
        }
        let value = if binding.parameters.is_empty() {
            self.eval_expression(module, &binding.value, environment)?
        } else {
            Value::Closure(Box::new(Closure {
                module,
                parameters: binding.parameters.clone(),
                body: binding.value.clone(),
                environment: environment.clone(),
            }))
        };
        *cell.borrow_mut() = value;
        environment.insert(key, cell);
        Ok(())
    }

    fn reference_value(
        &mut self,
        module: ModuleId,
        reference: hir::ReferenceId,
        environment: &Environment,
    ) -> Result<Value, RuntimeFault> {
        match self.reference_target(module, reference)? {
            ReferenceTarget::Definition(definition) => self.definition_value(&definition),
            ReferenceTarget::Binding(binding) => environment
                .get(&binding)
                .map(|cell| cell.borrow().clone())
                .ok_or_else(|| {
                    self.fault(module, self.module_span(module), "binding cell is absent")
                }),
        }
    }

    fn reference_target(
        &self,
        module: ModuleId,
        reference: hir::ReferenceId,
    ) -> Result<ReferenceTarget, RuntimeFault> {
        self.snapshot
            .checked()
            .typed()
            .resolved()
            .reference(module, reference)
            .cloned()
            .ok_or_else(|| {
                self.fault(
                    module,
                    self.module_span(module),
                    "resolved reference is absent",
                )
            })
    }

    fn definition_value(&mut self, definition: &DefinitionId) -> Result<Value, RuntimeFault> {
        let resolved = self.snapshot.checked().typed().resolved();
        let info = resolved.definition(definition).ok_or_else(|| {
            self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "definition is absent",
            })
        })?;
        match info.origin {
            DefinitionOrigin::Builtin(builtin) => Ok(Value::Builtin {
                builtin,
                arguments: Vec::new(),
            }),
            DefinitionOrigin::User { module } => {
                let resolved_module = resolved
                    .module(module)
                    .expect("resolved definition module exists");
                if info.kind == DefinitionKind::Constructor {
                    if let Some((variant, case, has_payload)) =
                        self.find_constructor(module, &info.name)
                    {
                        return if has_payload {
                            Ok(Value::Constructor {
                                definition: variant,
                                case,
                            })
                        } else {
                            Ok(Value::Variant {
                                definition: variant,
                                case,
                                payload: None,
                            })
                        };
                    }
                }
                let value = resolved_module
                    .hir
                    .definitions
                    .iter()
                    .find(|value| value.name.normalized == info.name)
                    .ok_or_else(|| {
                        self.fault(
                            module,
                            resolved_module.hir.span,
                            "definition body is absent",
                        )
                    })?;
                if value.parameters.is_empty() {
                    self.eval_expression(module, &value.value, &mut Environment::new())
                } else {
                    Ok(Value::Closure(Box::new(Closure {
                        module,
                        parameters: value.parameters.clone(),
                        body: value.value.clone(),
                        environment: Environment::new(),
                    })))
                }
            }
        }
    }

    fn find_constructor(
        &self,
        module: ModuleId,
        name: &str,
    ) -> Option<(DefinitionId, String, bool)> {
        let typed = self.snapshot.checked().typed();
        typed.variants().iter().find_map(|(definition, variant)| {
            let belongs_to_module = typed
                .resolved()
                .definition(definition)
                .is_some_and(|info| matches!(info.origin, DefinitionOrigin::User { module: owner } if owner == module));
            if !belongs_to_module {
                return None;
            }
            variant
                .cases
                .iter()
                .find(|case| case.name == name)
                .map(|case| (definition.clone(), case.name.clone(), case.payload.is_some()))
        })
    }

    fn apply(
        &mut self,
        callable: Value,
        arguments: Vec<Value>,
        span: Span,
    ) -> Result<Value, RuntimeFault> {
        match callable {
            Value::Closure(closure) => {
                if arguments.len() > closure.parameters.len() {
                    return Err(self.fault(
                        closure.module,
                        span,
                        "checked call supplied too many arguments",
                    ));
                }
                let mut environment = closure.environment.clone();
                for (pattern, value) in closure.parameters.iter().zip(&arguments) {
                    if !self.match_pattern(closure.module, pattern, value, &mut environment)? {
                        return Err(self.fault(
                            closure.module,
                            span,
                            "checked function argument did not match",
                        ));
                    }
                }
                if arguments.len() == closure.parameters.len() {
                    self.eval_expression(closure.module, &closure.body, &mut environment)
                } else {
                    Ok(Value::Closure(Box::new(Closure {
                        module: closure.module,
                        parameters: closure.parameters[arguments.len()..].to_vec(),
                        body: closure.body,
                        environment,
                    })))
                }
            }
            Value::Builtin {
                builtin,
                arguments: mut captured,
            } => {
                captured.extend(arguments);
                self.apply_builtin(builtin, captured, span)
            }
            Value::Constructor { definition, case } => {
                if arguments.len() == 1 {
                    Ok(Value::Variant {
                        definition,
                        case,
                        payload: Some(Box::new(
                            arguments.into_iter().next().expect("length checked"),
                        )),
                    })
                } else {
                    Err(self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "variant constructor arity is not one",
                    }))
                }
            }
            _ => Err(self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "checked application target is not callable",
            })),
        }
    }

    fn apply_builtin(
        &mut self,
        builtin: Builtin,
        arguments: Vec<Value>,
        span: Span,
    ) -> Result<Value, RuntimeFault> {
        let arity = match builtin {
            Builtin::ConsoleWrite | Builtin::Sum => 1,
            Builtin::TextFormat | Builtin::Max | Builtin::Min | Builtin::Map => 2,
        };
        if arguments.len() < arity {
            return Ok(Value::Builtin { builtin, arguments });
        }
        if arguments.len() > arity {
            return Err(self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "checked builtin call supplied too many arguments",
            }));
        }
        let module = self.snapshot.checked().typed().resolved().entry();
        match builtin {
            Builtin::ConsoleWrite => {
                let [Value::Text(text)] = arguments.as_slice() else {
                    return Err(self.fault(module, span, "Console.write argument is not Text"));
                };
                let mut line = text.clone();
                line.push('\n');
                self.console.write(&line).map_err(|error| RuntimeFault {
                    kind: RuntimeFaultKind::HostCapability {
                        operation: "Console.write",
                        category: error.category(),
                    },
                    source_name: self.module_source(module),
                    span,
                })?;
                Ok(Value::Unit)
            }
            Builtin::TextFormat => {
                let [Value::Text(template), Value::Int(value)] = arguments.as_slice() else {
                    return Err(self.fault(
                        module,
                        span,
                        "Text.format arguments have invalid values",
                    ));
                };
                let count = template.match_indices("{}").count();
                if count != 1 {
                    return Err(RuntimeFault {
                        kind: RuntimeFaultKind::InvalidFormatPlaceholderCount { count },
                        source_name: self.module_source(module),
                        span,
                    });
                }
                Ok(Value::Text(template.replacen("{}", &value.to_string(), 1)))
            }
            Builtin::Max | Builtin::Min => {
                let [Value::Int(left), Value::Int(right)] = arguments.as_slice() else {
                    return Err(self.fault(module, span, "max/min arguments are not Int"));
                };
                let value = if builtin == Builtin::Max {
                    std::cmp::max(left, right)
                } else {
                    std::cmp::min(left, right)
                };
                Ok(Value::Int(value.clone()))
            }
            Builtin::Sum => {
                let [Value::List(values)] = arguments.as_slice() else {
                    return Err(self.fault(module, span, "sum argument is not a List"));
                };
                let mut result = BigInt::from(0_u8);
                for value in values {
                    let Value::Int(value) = value else {
                        return Err(self.fault(module, span, "sum element is not Int"));
                    };
                    result += value;
                }
                Ok(Value::Int(result))
            }
            Builtin::Map => {
                let [callable, Value::List(values)] = arguments.as_slice() else {
                    return Err(self.fault(module, span, "map arguments have invalid values"));
                };
                let mut result = Vec::with_capacity(values.len());
                for value in values {
                    result.push(self.apply(callable.clone(), vec![value.clone()], span)?);
                }
                Ok(Value::List(result))
            }
        }
    }

    fn match_pattern(
        &mut self,
        module: ModuleId,
        pattern: &hir::Pattern,
        value: &Value,
        environment: &mut Environment,
    ) -> Result<bool, RuntimeFault> {
        match &pattern.kind {
            hir::PatternKind::Binding { id, .. } => {
                environment.insert(
                    BindingKey::new(module, *id),
                    Rc::new(RefCell::new(value.clone())),
                );
                Ok(true)
            }
            hir::PatternKind::Unit => Ok(matches!(value, Value::Unit)),
            hir::PatternKind::Literal(pattern) => Ok(literal_matches(pattern, value)),
            hir::PatternKind::Tuple(patterns) => {
                let Value::Tuple(values) = value else {
                    return Ok(false);
                };
                if patterns.len() != values.len() {
                    return Ok(false);
                }
                for (pattern, value) in patterns.iter().zip(values) {
                    if !self.match_pattern(module, pattern, value, environment)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            hir::PatternKind::Constructor { name, arguments } => {
                let Value::Variant { case, payload, .. } = value else {
                    return Ok(false);
                };
                if &name.normalized != case {
                    return Ok(false);
                }
                match (arguments.as_slice(), payload.as_deref()) {
                    ([], None) => Ok(true),
                    ([argument], Some(payload)) => {
                        self.match_pattern(module, argument, payload, environment)
                    }
                    _ => Ok(false),
                }
            }
        }
    }

    fn eval_binary(
        &self,
        module: ModuleId,
        span: Span,
        operator: hir::BinaryOperator,
        left: Value,
        right: Value,
    ) -> Result<Value, RuntimeFault> {
        use hir::BinaryOperator as Operator;
        match operator {
            Operator::Equal | Operator::NotEqual => {
                let equal = values_equal(&left, &right);
                Ok(Value::Bool(if operator == Operator::Equal {
                    equal
                } else {
                    !equal
                }))
            }
            Operator::Less
            | Operator::LessEqual
            | Operator::Greater
            | Operator::GreaterEqual
            | Operator::Add
            | Operator::Subtract
            | Operator::Multiply
            | Operator::Divide
            | Operator::Remainder => {
                let (Value::Int(left), Value::Int(right)) = (left, right) else {
                    return Err(self.fault(
                        module,
                        span,
                        "integer operator received non-Int values",
                    ));
                };
                match operator {
                    Operator::Less => Ok(Value::Bool(left < right)),
                    Operator::LessEqual => Ok(Value::Bool(left <= right)),
                    Operator::Greater => Ok(Value::Bool(left > right)),
                    Operator::GreaterEqual => Ok(Value::Bool(left >= right)),
                    Operator::Add => Ok(Value::Int(left + right)),
                    Operator::Subtract => Ok(Value::Int(left - right)),
                    Operator::Multiply => Ok(Value::Int(left * right)),
                    Operator::Divide | Operator::Remainder if right == BigInt::from(0_u8) => {
                        Err(RuntimeFault {
                            kind: RuntimeFaultKind::DivisionByZero,
                            source_name: self.module_source(module),
                            span,
                        })
                    }
                    Operator::Divide => Ok(Value::Int(left / right)),
                    Operator::Remainder => Ok(Value::Int(left % right)),
                    Operator::Equal | Operator::NotEqual => unreachable!("handled above"),
                }
            }
        }
    }

    fn fault(&self, module: ModuleId, span: Span, invariant: &'static str) -> RuntimeFault {
        RuntimeFault {
            kind: RuntimeFaultKind::InvalidCheckedCore { invariant },
            source_name: self.module_source(module),
            span,
        }
    }

    fn fault_at_entry(&self, kind: RuntimeFaultKind) -> RuntimeFault {
        let module = self.snapshot.checked().typed().resolved().entry_module();
        RuntimeFault {
            kind,
            source_name: module.hir.source_name.clone(),
            span: module.hir.module.span,
        }
    }

    fn module_source(&self, module: ModuleId) -> String {
        self.snapshot
            .checked()
            .typed()
            .resolved()
            .module(module)
            .map(|module| module.hir.source_name.clone())
            .unwrap_or_default()
    }

    fn module_span(&self, module: ModuleId) -> Span {
        self.snapshot
            .checked()
            .typed()
            .resolved()
            .module(module)
            .map_or_else(
                || {
                    self.snapshot
                        .checked()
                        .typed()
                        .resolved()
                        .entry_module()
                        .hir
                        .span
                },
                |module| module.hir.span,
            )
    }
}

fn assign_fields(
    target: &mut Value,
    fields: &[hir::Name],
    value: Value,
) -> Result<(), &'static str> {
    let Some((field, remaining)) = fields.split_first() else {
        *target = value;
        return Ok(());
    };
    let Value::Record { fields, .. } = target else {
        return Err("assignment projection root is not a record");
    };
    let Some(target) = fields.get_mut(&field.normalized) else {
        return Err("assignment projection field is absent");
    };
    assign_fields(target, remaining, value)
}

fn literal_matches(pattern: &hir::Literal, value: &Value) -> bool {
    match (pattern, value) {
        (hir::Literal::Integer { radix, digits }, Value::Int(value)) => {
            BigInt::parse_bytes(digits.replace('_', "").as_bytes(), *radix).as_ref() == Some(value)
        }
        (hir::Literal::Float(pattern), Value::Float64(value)) => pattern
            .parse::<f64>()
            .is_ok_and(|pattern| pattern == *value),
        (hir::Literal::Text(pattern), Value::Text(value)) => pattern == value,
        (hir::Literal::Boolean(pattern), Value::Bool(value)) => pattern == value,
        _ => false,
    }
}

fn values_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Unit, Value::Unit) => true,
        (Value::Bool(left), Value::Bool(right)) => left == right,
        (Value::Int(left), Value::Int(right)) => left == right,
        (Value::Float64(left), Value::Float64(right)) => left == right,
        (Value::Text(left), Value::Text(right)) => left == right,
        (Value::Tuple(left), Value::Tuple(right)) | (Value::List(left), Value::List(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| values_equal(left, right))
        }
        (
            Value::Record {
                definition: left_definition,
                fields: left_fields,
            },
            Value::Record {
                definition: right_definition,
                fields: right_fields,
            },
        ) => {
            left_definition == right_definition
                && left_fields.len() == right_fields.len()
                && left_fields.iter().all(|(name, left)| {
                    right_fields
                        .get(name)
                        .is_some_and(|right| values_equal(left, right))
                })
        }
        (
            Value::Variant {
                definition: left_definition,
                case: left_case,
                payload: left_payload,
            },
            Value::Variant {
                definition: right_definition,
                case: right_case,
                payload: right_payload,
            },
        ) => {
            left_definition == right_definition
                && left_case == right_case
                && match (left_payload, right_payload) {
                    (None, None) => true,
                    (Some(left), Some(right)) => values_equal(left, right),
                    _ => false,
                }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_effects::locate_main;
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
        ling_semantic::build(checked).expect("snapshot builds")
    }

    #[test]
    fn hello_writes_exact_utf8_text_and_lf() {
        let snapshot = snapshot(
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"你好，零\"\n",
        );
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("hello executes");
        assert_eq!(console.output(), "你好，零\n");
    }

    #[test]
    fn console_failure_is_a_structured_runtime_fault() {
        let snapshot = snapshot(
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"x\"\n",
        );
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::failing(HostErrorCategory::BrokenPipe);
        let fault = execute_main(&snapshot, &main, &mut console).expect_err("host failure");
        assert!(matches!(
            fault.kind,
            RuntimeFaultKind::HostCapability {
                category: HostErrorCategory::BrokenPipe,
                ..
            }
        ));
        assert_eq!(fault.to_diagnostic().code(), codes::RUNTIME_FAULT);
    }
}
