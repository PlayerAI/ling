//! Strict checked-core interpreter with injected host capabilities.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_effects::{CheckedProgram, ResumeMode, ResumeUse};
use ling_hir as hir;
use ling_resolve::{
    BindingKey, Builtin, DefinitionId, DefinitionKind, DefinitionOrigin, ExpressionKey,
    HandlerResumeMode, HandlerValueType, ModuleId, ReferenceTarget, resolve_handler_operation,
};
use ling_semantic::{ProgramSnapshot, ProjectProgramSnapshot};
use ling_source::Span;
use ling_types::Type;
use num_bigint::BigInt;

mod machine;
mod task_runtime;
mod task_scheduler;

pub use task_runtime::{
    TaskCancellationCause, TaskPath, TaskRuntime, TaskRuntimeLimits, TaskRuntimeState, TaskStep,
    TaskStepKind, TaskValue,
};
pub use task_scheduler::{
    TASK_SCHEDULE_TRACE_VERSION, TaskDeadline, TaskExplorationLimit, TaskExplorationResult,
    TaskFaultSummary, TaskHostOutcome, TaskHostResponse, TaskHostScript, TaskReplayError,
    TaskScheduleConfig, TaskScheduleEvent, TaskScheduleEventKind, TaskScheduleTerminal,
    TaskScheduleTrace, TaskSchedulerError, TaskSchedulerLimits, explore_task_schedules,
    replay_task_schedule, run_task_schedule, task_schedule_splitmix64,
};

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
    HandlerResumeCardinality {
        operation: String,
        mode: HandlerResumeMode,
    },
    TaskImplementationBoundary {
        definition: String,
    },
    TaskResourceLimit {
        resource: &'static str,
        limit: usize,
    },
    TaskDriver {
        reason: &'static str,
        task: String,
    },
    TaskFaultAggregate {
        primary_task: String,
        fault_count: usize,
        related_tasks: Vec<String>,
    },
}

impl RuntimeFault {
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        if let RuntimeFaultKind::TaskImplementationBoundary { definition } = &self.kind {
            return Diagnostic::new(
                codes::TASK_IMPLEMENTATION_BOUNDARY,
                Severity::Error,
                "已检查 Task 尚不能进入 interpreter.execute 阶段",
                "checked Task cannot enter the `interpreter.execute` stage yet",
            )
            .with_primary_span(DiagnosticSpan::new(&self.source_name, self.span))
            .with_fact("definition", definition.clone())
            .with_fact("stage", "interpreter.execute")
            .with_fact("required_tasks", "TASK-2202,TASK-2203");
        }
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
            RuntimeFaultKind::HandlerResumeCardinality { operation, mode } => (
                format!("Handler operation“{operation}”的 {mode:?} continuation 被重复调用"),
                format!(
                    "the {mode:?} continuation for handler operation `{operation}` was invoked more than permitted"
                ),
                "handler_resume_cardinality",
            ),
            RuntimeFaultKind::TaskResourceLimit { resource, limit } => (
                format!("Structured Task 资源上限已耗尽：{resource}={limit}"),
                format!("Structured Task resource limit exhausted: {resource}={limit}"),
                "resource_limit",
            ),
            RuntimeFaultKind::TaskDriver { reason, task } => (
                format!("Structured Task 驱动请求无效：{reason}（{task}）"),
                format!("invalid Structured Task driver request: {reason} ({task})"),
                "task_driver",
            ),
            RuntimeFaultKind::TaskFaultAggregate {
                primary_task,
                fault_count,
                ..
            } => (
                format!("Structured Task 失败：主 Task {primary_task}，共 {fault_count} 个 Fault"),
                format!(
                    "Structured Task failed: primary Task {primary_task}, {fault_count} Fault(s)"
                ),
                "task_fault_aggregate",
            ),
            RuntimeFaultKind::TaskImplementationBoundary { .. } => {
                unreachable!("Task boundary returned above")
            }
        };
        let mut diagnostic = Diagnostic::new(codes::RUNTIME_FAULT, Severity::Error, zh, en)
            .with_primary_span(DiagnosticSpan::new(&self.source_name, self.span))
            .with_fact("category", category);
        if let RuntimeFaultKind::HandlerResumeCardinality { operation, .. } = &self.kind {
            diagnostic = diagnostic.with_fact("operation", operation.clone());
        }
        match &self.kind {
            RuntimeFaultKind::TaskResourceLimit { resource, limit } => {
                diagnostic = diagnostic
                    .with_fact("resource", *resource)
                    .with_fact("limit", limit.to_string());
            }
            RuntimeFaultKind::TaskDriver { reason, task } => {
                diagnostic = diagnostic
                    .with_fact("reason", *reason)
                    .with_fact("task", task.clone());
            }
            RuntimeFaultKind::TaskFaultAggregate {
                primary_task,
                fault_count,
                related_tasks,
            } => {
                diagnostic = diagnostic
                    .with_fact("primary_task", primary_task.clone())
                    .with_fact("fault_count", fault_count.to_string())
                    .with_fact("related_tasks", related_tasks.join(","));
            }
            _ => {}
        }
        diagnostic
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
    reject_checked_tasks(snapshot.checked())?;
    Interpreter::new(snapshot.checked(), console).execute_main(main)
}

/// Executes a previously validated root entry from a package-aware checked
/// project snapshot. The interpreter consumes only checked Typed Core.
pub fn execute_project_main(
    snapshot: &ProjectProgramSnapshot,
    main: &DefinitionId,
    console: &mut dyn Console,
) -> Result<(), RuntimeFault> {
    reject_checked_tasks(snapshot.checked())?;
    Interpreter::new(snapshot.checked(), console).execute_main(main)
}

/// Canonical, host-independent result of evaluating one checked definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluatedValue {
    rendered: String,
    unit: bool,
}

impl EvaluatedValue {
    #[must_use]
    pub fn rendered(&self) -> &str {
        &self.rendered
    }

    #[must_use]
    pub const fn is_unit(&self) -> bool {
        self.unit
    }
}

/// Evaluates a checked top-level definition without granting unchecked input
/// access to the interpreter.
pub fn evaluate_definition(
    snapshot: &ProgramSnapshot,
    definition: &DefinitionId,
    console: &mut dyn Console,
) -> Result<EvaluatedValue, RuntimeFault> {
    reject_checked_tasks(snapshot.checked())?;
    let value = Interpreter::new(snapshot.checked(), console).definition_value(definition)?;
    Ok(EvaluatedValue {
        rendered: render_value(&value),
        unit: matches!(value, Value::Unit),
    })
}

fn reject_checked_tasks(checked: &CheckedProgram) -> Result<(), RuntimeFault> {
    let Some(core) = checked.task_cores().values().next() else {
        return Ok(());
    };
    let source_name = checked
        .typed()
        .resolved()
        .definition(core.definition())
        .and_then(|definition| definition.source_name.clone())
        .unwrap_or_else(|| "<task>".to_owned());
    Err(RuntimeFault {
        kind: RuntimeFaultKind::TaskImplementationBoundary {
            definition: core.definition().to_string(),
        },
        source_name,
        span: core.source_span(),
    })
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
    Resume {
        continuation: Rc<machine::ContinuationValue>,
        source_span: Span,
    },
    TaskHandle(TaskPath),
}

#[derive(Clone, Debug)]
struct Closure {
    module: ModuleId,
    parameters: Vec<hir::Pattern>,
    body: hir::Expression,
    environment: Environment,
}

struct Interpreter<'snapshot, 'console> {
    checked: &'snapshot CheckedProgram,
    console: &'console mut dyn Console,
    next_handler_id: u64,
    host_effect_epoch: u64,
}

impl<'snapshot, 'console> Interpreter<'snapshot, 'console> {
    const fn new(checked: &'snapshot CheckedProgram, console: &'console mut dyn Console) -> Self {
        Self {
            checked,
            console,
            next_handler_id: 0,
            host_effect_epoch: 0,
        }
    }

    fn execute_main(&mut self, main: &DefinitionId) -> Result<(), RuntimeFault> {
        let checked = self.checked;
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
        if self.checked.handler_cores().is_empty() {
            self.eval_expression_direct(module, expression, environment)
        } else {
            machine::evaluate(self, module, expression.clone(), environment.clone())
        }
    }

    fn eval_expression_direct(
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
                        hir::SequenceElement::LetAwait(binding) => {
                            return Err(self.fault(
                                module,
                                binding.span,
                                "checked task execution is not implemented",
                            ));
                        }
                        hir::SequenceElement::Expression(expression) => {
                            result = self.eval_expression(module, expression, environment)?;
                        }
                    }
                }
                Ok(result)
            }
            hir::ExpressionKind::TaskScope { .. }
            | hir::ExpressionKind::TaskSpawn { .. }
            | hir::ExpressionKind::TaskAwait { .. }
            | hir::ExpressionKind::TaskReturn { .. } => Err(self.fault(
                module,
                expression.span,
                "checked task execution is not implemented",
            )),
            hir::ExpressionKind::Handle { .. } => Err(self.fault(
                module,
                expression.span,
                "checked handler execution requires accepted EFF-2104 authority",
            )),
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
                let mut values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    values.push(self.eval_expression(module, argument, environment)?);
                }
                let key = ExpressionKey::new(module, expression.id);
                if let Some(call) = self.checked.typed().trait_member_call(key).cloned() {
                    let callable = self.definition_value(call.implementation())?;
                    return self.apply(callable, values, expression.span);
                }
                let callable = self.eval_expression(module, function, environment)?;
                self.apply(callable, values, expression.span)
            }
            hir::ExpressionKind::Projection {
                reference,
                target,
                field,
            } => {
                if self
                    .checked
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
            } => match operator {
                hir::BinaryOperator::BooleanAnd | hir::BinaryOperator::BooleanOr => {
                    let left_value = self.eval_expression(module, left, environment)?;
                    let Value::Bool(left_value) = left_value else {
                        return Err(self.fault(module, left.span, "boolean operand is not Bool"));
                    };
                    if (*operator == hir::BinaryOperator::BooleanAnd && !left_value)
                        || (*operator == hir::BinaryOperator::BooleanOr && left_value)
                    {
                        return Ok(Value::Bool(left_value));
                    }
                    match self.eval_expression(module, right, environment)? {
                        Value::Bool(value) => Ok(Value::Bool(value)),
                        _ => Err(self.fault(module, right.span, "boolean operand is not Bool")),
                    }
                }
                _ => {
                    let left = self.eval_expression(module, left, environment)?;
                    let right = self.eval_expression(module, right, environment)?;
                    self.eval_binary(module, expression.span, *operator, left, right)
                }
            },
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
                    .checked
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
                    .checked
                    .typed()
                    .expression_type(ExpressionKey::new(module, expression.id))
                    .ok_or_else(|| self.fault(module, expression.span, "record type is absent"))?;
                let Type::NominalRecord { definition, .. } =
                    self.checked.typed().arena().get(type_id)
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
        self.checked
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
        let resolved = self.checked.typed().resolved();
        let info = resolved.definition(definition).cloned().ok_or_else(|| {
            self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "definition is absent",
            })
        })?;
        if resolved.trait_member(definition).is_some() {
            return Err(self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "bare Trait member reached evaluation",
            }));
        }
        let implementation_member = resolved.impl_member(definition).cloned();
        match info.origin {
            DefinitionOrigin::Builtin(builtin) => Ok(Value::Builtin {
                builtin,
                arguments: Vec::new(),
            }),
            DefinitionOrigin::Prelude(_) => self.constructor_value(definition).ok_or_else(|| {
                self.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "Prelude definition is not a constructor value",
                })
            }),
            DefinitionOrigin::User { module } => {
                let resolved_module = resolved
                    .module(module)
                    .expect("resolved definition module exists");
                if let Some(member) = implementation_member {
                    let implementation = resolved_module
                        .hir
                        .impls
                        .get(member.impl_ordinal)
                        .and_then(|implementation| {
                            implementation.members.get(member.member_ordinal)
                        })
                        .ok_or_else(|| {
                            self.fault(
                                module,
                                resolved_module.hir.span,
                                "implementation member body is absent",
                            )
                        })?;
                    return if implementation.parameters.is_empty() {
                        self.eval_expression(module, &implementation.value, &mut Environment::new())
                    } else {
                        Ok(Value::Closure(Box::new(Closure {
                            module,
                            parameters: implementation.parameters.clone(),
                            body: implementation.value.clone(),
                            environment: Environment::new(),
                        })))
                    };
                }
                if info.kind == DefinitionKind::Constructor {
                    if let Some(value) = self.constructor_value(definition) {
                        return Ok(value);
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

    fn constructor_value(&self, constructor: &DefinitionId) -> Option<Value> {
        self.find_constructor(constructor)
            .map(|(definition, case, has_payload)| {
                if has_payload {
                    Value::Constructor { definition, case }
                } else {
                    Value::Variant {
                        definition,
                        case,
                        payload: None,
                    }
                }
            })
    }

    fn find_constructor(&self, constructor: &DefinitionId) -> Option<(DefinitionId, String, bool)> {
        let typed = self.checked.typed();
        typed.variants().iter().find_map(|(definition, variant)| {
            variant
                .cases
                .iter()
                .find(|case| &case.definition == constructor)
                .map(|case| {
                    (
                        definition.clone(),
                        case.name.clone(),
                        case.payload.is_some(),
                    )
                })
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
        let module = self.checked.typed().resolved().entry();
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
                self.record_host_effect(module, span)?;
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
                if let Some((definition, case)) = self.pattern_constructor(module, pattern) {
                    return Ok(matches!(
                        value,
                        Value::Variant {
                            definition: actual_definition,
                            case: actual_case,
                            payload: None,
                        } if actual_definition == &definition && actual_case == &case
                    ));
                }
                environment.insert(
                    BindingKey::new(module, *id),
                    Rc::new(RefCell::new(value.clone())),
                );
                Ok(true)
            }
            hir::PatternKind::Wildcard => Ok(true),
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
            hir::PatternKind::Record(patterns) => {
                let Value::Record { fields, .. } = value else {
                    return Ok(false);
                };
                for pattern in patterns {
                    let Some(value) = fields.get(&pattern.name.normalized) else {
                        return Ok(false);
                    };
                    if !self.match_pattern(module, &pattern.pattern, value, environment)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            hir::PatternKind::Constructor { arguments, .. } => {
                let Some((expected_definition, expected_case)) =
                    self.pattern_constructor(module, pattern)
                else {
                    return Err(self.fault(
                        module,
                        pattern.span,
                        "constructor pattern is unresolved",
                    ));
                };
                let Value::Variant {
                    definition,
                    case,
                    payload,
                } = value
                else {
                    return Ok(false);
                };
                if definition != &expected_definition || case != &expected_case {
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

    fn pattern_constructor(
        &self,
        module: ModuleId,
        pattern: &hir::Pattern,
    ) -> Option<(DefinitionId, String)> {
        let resolved = self.checked.typed().resolved();
        let constructor = resolved.pattern_constructor(module, pattern.id)?;
        self.find_constructor(constructor)
            .map(|(definition, case, _)| (definition, case))
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
            Operator::BooleanAnd | Operator::BooleanOr => Err(self.fault(
                module,
                span,
                "boolean operator bypassed short-circuit evaluation",
            )),
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
                    Operator::BooleanAnd
                    | Operator::BooleanOr
                    | Operator::Equal
                    | Operator::NotEqual => unreachable!("handled above"),
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

    fn record_host_effect(&mut self, module: ModuleId, span: Span) -> Result<(), RuntimeFault> {
        self.host_effect_epoch = self
            .host_effect_epoch
            .checked_add(1)
            .ok_or_else(|| self.fault(module, span, "host Effect observation identity overflow"))?;
        Ok(())
    }

    fn fault_at_entry(&self, kind: RuntimeFaultKind) -> RuntimeFault {
        let module = self.checked.typed().resolved().entry_module();
        RuntimeFault {
            kind,
            source_name: module.hir.source_name.clone(),
            span: module.hir.module.span,
        }
    }

    fn module_source(&self, module: ModuleId) -> String {
        self.checked
            .typed()
            .resolved()
            .module(module)
            .map(|module| module.hir.source_name.clone())
            .unwrap_or_default()
    }

    fn module_span(&self, module: ModuleId) -> Span {
        self.checked.typed().resolved().module(module).map_or_else(
            || self.checked.typed().resolved().entry_module().hir.span,
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

fn render_value(value: &Value) -> String {
    match value {
        Value::Unit => "()".to_owned(),
        Value::Bool(value) => value.to_string(),
        Value::Int(value) => value.to_string(),
        Value::Float64(value) => value.to_string(),
        Value::Text(value) => render_text(value),
        Value::Tuple(values) => format!(
            "({})",
            values
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::List(values) => format!(
            "[{}]",
            values
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join("; ")
        ),
        Value::Record { fields, .. } => format!(
            "{{ {} }}",
            fields
                .iter()
                .map(|(name, value)| format!("{name} = {}", render_value(value)))
                .collect::<Vec<_>>()
                .join("; ")
        ),
        Value::Variant { case, payload, .. } => payload.as_ref().map_or_else(
            || case.clone(),
            |payload| format!("{case} {}", render_value(payload)),
        ),
        Value::Closure(_) => "<function>".to_owned(),
        Value::Builtin { builtin, arguments } => {
            format!("<builtin:{}:{}>", builtin.qualified_name(), arguments.len())
        }
        Value::Constructor { case, .. } => format!("<constructor:{case}>"),
        Value::Resume { .. } => "<continuation>".to_owned(),
        Value::TaskHandle(path) => format!("<task:{}>", path),
    }
}

fn render_text(value: &str) -> String {
    let mut rendered = String::with_capacity(value.len().saturating_add(2));
    rendered.push('"');
    for character in value.chars() {
        match character {
            '"' => rendered.push_str("\\\""),
            '\\' => rendered.push_str("\\\\"),
            '\n' => rendered.push_str("\\n"),
            '\r' => rendered.push_str("\\r"),
            '\t' => rendered.push_str("\\t"),
            character if character.is_control() => {
                use std::fmt::Write as _;
                write!(rendered, "\\u{{{:x}}}", u32::from(character))
                    .expect("writing to a String cannot fail");
            }
            character => rendered.push(character),
        }
    }
    rendered.push('"');
    rendered
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

    #[test]
    fn checked_handler_intercepts_console_write_without_host_output() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"handled\" with\n",
            "        operation Console.Write.write(message, resume) -> ()\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("handler executes");
        assert_eq!(console.output(), "");
    }

    #[test]
    fn checked_handler_resume_restores_the_deep_handler_before_returning_to_the_clause() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"body\" with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            resume ()\n",
            "            Console.write \"clause\"\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("resumed handler executes");
        assert_eq!(console.output(), "clause\n");
    }

    #[test]
    fn nested_handlers_select_the_nearest_clause_and_expose_clause_effects_outward() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let inner () =\n",
            "    handle Console.write \"body\" with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            Console.write \"inner clause\"\n\n",
            "let main () =\n",
            "    handle inner () with\n",
            "        operation Console.Write.write(message, resume) -> ()\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("nested handlers execute");
        assert_eq!(console.output(), "");
    }

    #[test]
    fn handler_intercepts_operations_reached_through_a_function_call() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let emit () = Console.write \"transitive\"\n\n",
            "let main () =\n",
            "    handle emit () with\n",
            "        operation Console.Write.write(message, resume) -> ()\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("transitive handler executes");
        assert_eq!(console.output(), "");
    }

    #[test]
    fn once_continuation_rejects_repeated_dynamic_invocation() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"body\" with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            let ignored = map resume [(); ()]\n",
            "            ()\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        let fault = execute_main(&snapshot, &main, &mut console)
            .expect_err("Once continuation must reject its second invocation");
        assert!(matches!(
            fault.kind,
            RuntimeFaultKind::HandlerResumeCardinality {
                ref operation,
                mode: HandlerResumeMode::Once,
            } if operation == "Console.Write.write"
        ));
        let diagnostic = fault.to_diagnostic();
        assert_eq!(diagnostic.code(), codes::RUNTIME_FAULT);
        assert_eq!(
            diagnostic
                .facts()
                .get("operation")
                .and_then(|value| value.as_str()),
            Some("Console.Write.write")
        );
    }

    #[test]
    fn resumed_sequence_reinstalls_the_handler_for_later_operations() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let emitBoth () =\n",
            "    Console.write \"first\"\n",
            "    Console.write \"second\"\n\n",
            "let main () =\n",
            "    handle emitBoth () with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            if message == \"first\" then resume () else ()\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("deep sequence executes");
        assert_eq!(console.output(), "");
    }

    #[test]
    fn resumed_continuation_observes_the_same_mutable_cells() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    let mutable cell = 0\n",
            "    let body () =\n",
            "        Console.write \"trigger\"\n",
            "        cell <- 1\n",
            "    handle body () with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            resume ()\n",
            "            if cell == 1 then Console.write \"shared\" else Console.write \"stale\"\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("stateful resume executes");
        assert_eq!(console.output(), "shared\n");
    }

    #[test]
    fn clause_fault_preserves_already_committed_host_output() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"trigger\" with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            Console.write \"committed\"\n",
            "            let ignored = 1 / 0\n",
            "            ()\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        let fault = execute_main(&snapshot, &main, &mut console).expect_err("fault propagates");
        assert!(matches!(fault.kind, RuntimeFaultKind::DivisionByZero));
        assert_eq!(console.output(), "committed\n");
    }

    #[test]
    fn handler_fault_preserves_original_bom_crlf_unicode_byte_span() {
        let source = concat!(
            "\u{feff}module Main\r\n",
            "    requires Console.Write\r\n\r\n",
            "let main () =\r\n",
            "    handle Console.write \"你好\" with\r\n",
            "        operation Console.Write.write(message, resume) ->\r\n",
            "            let ignored = map resume [(); ()]\r\n",
            "            ()\r\n",
        );
        let snapshot = snapshot(source);
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        let fault = execute_main(&snapshot, &main, &mut console).expect_err("second resume faults");
        let resume_start = source.find("resume [").expect("resume call exists") as u32;
        assert_eq!(fault.source_name, "test.ling");
        assert_eq!(fault.span.start().get(), resume_start);
        assert!(fault.span.end().get() > resume_start);
    }

    #[test]
    fn map_and_application_evaluate_strictly_left_to_right() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let writeAndReturn value =\n",
            "    Console.write (Text.format \"{}\" value)\n",
            "    value\n\n",
            "let main () =\n",
            "    map writeAndReturn [3; 1; 2]\n",
            "    max (writeAndReturn 4) (writeAndReturn 5)\n",
            "    ()\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("ordered program executes");
        assert_eq!(console.output(), "3\n1\n2\n4\n5\n");
    }

    #[test]
    fn boolean_operators_evaluate_left_once_and_short_circuit_right() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let mark value =\n",
            "    Console.write \"rhs\"\n",
            "    value\n\n",
            "let main () =\n",
            "    let leftAnd = mark false && mark true\n",
            "    let leftOr = mark true || mark false\n",
            "    let requiredAnd = true && mark true\n",
            "    let requiredOr = false || mark true\n",
            "    let skippedFaultAnd = false && (1 / 0 == 0)\n",
            "    let skippedFaultOr = true || (1 / 0 == 0)\n",
            "    if leftAnd || leftOr && requiredAnd && requiredOr && skippedFaultAnd && skippedFaultOr then\n",
            "        Console.write \"unexpected\"\n",
            "    else\n",
            "        Console.write \"ok\"\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("boolean program executes");
        assert_eq!(console.output(), "rhs\nrhs\nrhs\nrhs\nok\n");
    }

    #[test]
    fn integer_builtins_preserve_arbitrary_precision_negatives_and_empty_sum() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let huge = 340282366920938463463374607431768211456\n\n",
            "let main () =\n",
            "    let total = sum [max huge (-1); min (-5) (-2); sum []]\n",
            "    Console.write (Text.format \"{}\" total)\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("integer builtins execute");
        assert_eq!(
            console.output(),
            "340282366920938463463374607431768211451\n"
        );
    }

    #[test]
    fn finite_float_literals_and_equality_execute_with_ieee_semantics() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    if 1.5e-2 == 0.015 then\n",
            "        Console.write \"equal\"\n",
            "    else\n",
            "        Console.write \"different\"\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("float equality executes");
        assert_eq!(console.output(), "equal\n");
    }

    #[test]
    fn record_copy_update_and_mutable_fields_preserve_value_semantics() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "type Counter = { mutable value: Int }\n\n",
            "let main () =\n",
            "    let mutable original = { value = 1 }\n",
            "    let mutablyCopied = original\n",
            "    let updated = { original with value = 3 }\n",
            "    let mutable changed = mutablyCopied\n",
            "    changed.value <- 2\n",
            "    Console.write (Text.format \"{}\" original.value)\n",
            "    Console.write (Text.format \"{}\" changed.value)\n",
            "    Console.write (Text.format \"{}\" updated.value)\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("record copies execute");
        assert_eq!(console.output(), "1\n2\n3\n");
    }

    #[test]
    fn text_format_rejects_non_singleton_placeholder_counts() {
        for (template, expected_count) in [("no placeholder", 0), ("{} and {}", 2)] {
            let snapshot = snapshot(&format!(
                concat!(
                    "module Main\n\n",
                    "let main () =\n",
                    "    Text.format \"{template}\" 1\n",
                    "    ()\n",
                ),
                template = template
            ));
            let main = locate_main(snapshot.checked()).expect("valid main");
            let mut console = MemoryConsole::default();
            let fault = execute_main(&snapshot, &main, &mut console)
                .expect_err("invalid placeholder count faults");
            assert!(matches!(
                fault.kind,
                RuntimeFaultKind::InvalidFormatPlaceholderCount { count }
                    if count == expected_count
            ));
            assert_eq!(fault.to_diagnostic().code(), codes::RUNTIME_FAULT);
        }
    }

    #[test]
    fn wildcard_and_tuple_constructor_payload_patterns_execute() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "type PairBox =\n",
            "    | Pair of Int * Int\n",
            "    | Empty\n\n",
            "let describe value =\n",
            "    match value with\n",
            "    | Pair (left, right) -> Text.format \"{}\" (left + right)\n",
            "    | _ -> \"empty\"\n\n",
            "let main () = Console.write (describe (Pair (2, 3)))\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("pattern program executes");
        assert_eq!(console.output(), "5\n");
    }

    #[test]
    fn prelude_option_and_result_constructors_execute() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let unwrap option =\n",
            "    match option with\n",
            "    | Some value -> value\n",
            "    | None -> 0\n\n",
            "let resultValue result =\n",
            "    match result with\n",
            "    | Ok value -> value\n",
            "    | Error _ -> 0\n\n",
            "let main () =\n",
            "    Console.write (Text.format \"{}\" (unwrap (Some (resultValue (Ok 7)))))\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();

        execute_main(&snapshot, &main, &mut console).expect("Prelude constructors execute");

        assert_eq!(console.output(), "7\n");
    }

    #[test]
    fn record_patterns_execute_without_copying_unbound_fields() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "type Point = { x: Int; y: Int }\n\n",
            "let describe point =\n",
            "    match point with\n",
            "    | { x = value; y = _ } -> Text.format \"{}\" value\n\n",
            "let main () = Console.write (describe { x = 7; y = 9 })\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("record pattern executes");
        assert_eq!(console.output(), "7\n");
    }

    #[test]
    fn concrete_trait_member_dispatch_executes_selected_impl_body() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "let main () = Console.write (Renderable.render { name = \"Ling\" })\n",
        ));
        let main = locate_main(snapshot.checked()).expect("valid main");
        let mut console = MemoryConsole::default();
        execute_main(&snapshot, &main, &mut console).expect("Trait member dispatch executes");
        assert_eq!(console.output(), "Ling\n");
    }
}
