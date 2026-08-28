use std::cell::Cell as ScalarCell;
use std::collections::VecDeque;
use std::rc::Rc;

use super::*;

#[derive(Clone, Debug)]
pub(super) struct ContinuationValue {
    handler_id: u64,
    operation: String,
    mode: HandlerResumeMode,
    frames: Vec<Frame>,
    uses: ScalarCell<usize>,
    active: Rc<ScalarCell<bool>>,
}

#[derive(Clone, Debug)]
struct ActiveHandler {
    id: u64,
    module: ModuleId,
    clauses: Vec<hir::HandlerClause>,
    environment: Environment,
}

impl ActiveHandler {
    fn handles(&self, operation: &str) -> bool {
        self.clauses
            .iter()
            .any(|clause| clause.operation.normalized() == operation)
    }

    fn clause(&self, operation: &str) -> Option<&hir::HandlerClause> {
        self.clauses
            .iter()
            .find(|clause| clause.operation.normalized() == operation)
    }
}

#[derive(Clone, Debug)]
enum Control {
    Expression {
        module: ModuleId,
        expression: hir::Expression,
        environment: Environment,
    },
    Definition(DefinitionId),
    Value(Value),
    Apply {
        module: ModuleId,
        callable: Value,
        arguments: Vec<Value>,
        span: Span,
    },
    Operation {
        module: ModuleId,
        operation: String,
        arguments: Vec<Value>,
        span: Span,
    },
    TaskScopeEnter {
        module: ModuleId,
        expression: ExpressionKey,
        span: Span,
        body: hir::Expression,
        environment: Environment,
    },
    TaskScopeExit {
        expression: ExpressionKey,
        span: Span,
        value: Value,
    },
    TaskSpawn {
        call: ExpressionKey,
        span: Span,
        arguments: Vec<Value>,
    },
    TaskAwait {
        continuation: ExpressionKey,
        span: Span,
        handle: Value,
    },
    TaskReturn {
        span: Span,
        value: Value,
    },
}

#[derive(Clone, Debug)]
enum Frame {
    Sequence {
        module: ModuleId,
        remaining: VecDeque<hir::SequenceElement>,
        environment: Environment,
    },
    LocalBinding {
        module: ModuleId,
        binding: hir::LocalBinding,
        cell: Cell,
        remaining: VecDeque<hir::SequenceElement>,
        environment: Environment,
    },
    TaskScope {
        expression: ExpressionKey,
        span: Span,
    },
    TaskSpawnArguments {
        module: ModuleId,
        call: ExpressionKey,
        span: Span,
        values: Vec<Value>,
        remaining: VecDeque<hir::Expression>,
        environment: Environment,
    },
    TaskAwait {
        continuation: ExpressionKey,
        span: Span,
    },
    TaskReturn {
        span: Span,
    },
    TaskLetAwaitHandle {
        module: ModuleId,
        binding: hir::TaskLetAwait,
        remaining: VecDeque<hir::SequenceElement>,
        environment: Environment,
    },
    TaskLetAwaitBinding {
        module: ModuleId,
        binding: hir::TaskLetAwait,
        remaining: VecDeque<hir::SequenceElement>,
        environment: Environment,
    },
    If {
        module: ModuleId,
        then_branch: hir::Expression,
        else_branch: hir::Expression,
        environment: Environment,
        span: Span,
    },
    MatchScrutinee {
        module: ModuleId,
        cases: VecDeque<hir::MatchCase>,
        environment: Environment,
        span: Span,
    },
    MatchGuard {
        module: ModuleId,
        scrutinee: Value,
        body: hir::Expression,
        remaining: VecDeque<hir::MatchCase>,
        original_environment: Environment,
        case_environment: Environment,
        span: Span,
    },
    Assignment {
        module: ModuleId,
        place: hir::Place,
        environment: Environment,
    },
    ApplicationFunction {
        module: ModuleId,
        arguments: VecDeque<hir::Expression>,
        environment: Environment,
        span: Span,
    },
    ApplicationArgument {
        module: ModuleId,
        callable: Value,
        values: Vec<Value>,
        remaining: VecDeque<hir::Expression>,
        environment: Environment,
        span: Span,
    },
    Projection {
        module: ModuleId,
        field: String,
        span: Span,
    },
    BooleanLeft {
        module: ModuleId,
        operator: hir::BinaryOperator,
        right: hir::Expression,
        environment: Environment,
    },
    BooleanRight {
        module: ModuleId,
        span: Span,
    },
    BinaryLeft {
        module: ModuleId,
        operator: hir::BinaryOperator,
        right: hir::Expression,
        environment: Environment,
        span: Span,
    },
    BinaryRight {
        module: ModuleId,
        operator: hir::BinaryOperator,
        left: Value,
        span: Span,
    },
    Unary {
        module: ModuleId,
        operator: hir::UnaryOperator,
        span: Span,
    },
    Aggregate {
        module: ModuleId,
        kind: AggregateKind,
        values: Vec<Value>,
        remaining: VecDeque<hir::Expression>,
        environment: Environment,
    },
    Record {
        module: ModuleId,
        definition: DefinitionId,
        names: VecDeque<String>,
        values: BTreeMap<String, Value>,
        remaining: VecDeque<hir::Expression>,
        environment: Environment,
    },
    RecordUpdateBase {
        module: ModuleId,
        fields: VecDeque<hir::RecordField>,
        environment: Environment,
        span: Span,
    },
    RecordUpdateField {
        module: ModuleId,
        definition: DefinitionId,
        fields: BTreeMap<String, Value>,
        current_name: String,
        remaining: VecDeque<hir::RecordField>,
        environment: Environment,
    },
    Map {
        module: ModuleId,
        callable: Value,
        values: VecDeque<Value>,
        output: Vec<Value>,
        span: Span,
    },
    HandlerBoundary(ActiveHandler),
    ClauseBoundary {
        handler_id: u64,
        active: Rc<ScalarCell<bool>>,
    },
    ResumeReturn {
        clause_boundary: Box<Frame>,
        clause_frames: Vec<Frame>,
    },
}

#[derive(Clone, Copy, Debug)]
enum AggregateKind {
    Tuple,
    List,
}

#[derive(Clone, Debug)]
pub(super) enum TaskBoundary {
    ScopeEnter {
        expression: ExpressionKey,
        span: Span,
    },
    ScopeExit {
        expression: ExpressionKey,
        span: Span,
        value: Value,
    },
    Spawn {
        call: ExpressionKey,
        span: Span,
        arguments: Vec<Value>,
    },
    Await {
        continuation: ExpressionKey,
        span: Span,
        handle: Value,
    },
    Return {
        span: Span,
        value: Value,
    },
}

#[derive(Clone, Debug)]
pub(super) enum EvaluationOutcome {
    Complete(Value),
    HostEffect,
    Task(TaskBoundary),
}

#[derive(Clone, Debug)]
pub(super) struct Evaluation {
    control: Option<Control>,
    frames: Vec<Frame>,
}

impl Evaluation {
    pub(super) fn new(
        module: ModuleId,
        expression: hir::Expression,
        environment: Environment,
    ) -> Self {
        Self {
            control: Some(Control::Expression {
                module,
                expression,
                environment,
            }),
            frames: Vec::new(),
        }
    }

    pub(super) fn enter_scope(&mut self) -> Result<(), &'static str> {
        let Some(Control::TaskScopeEnter {
            module,
            body,
            environment,
            ..
        }) = self.control.take()
        else {
            return Err("task runtime resumed a non-scope-entry boundary");
        };
        self.control = Some(Control::Expression {
            module,
            expression: body,
            environment,
        });
        Ok(())
    }

    pub(super) fn resume_value(&mut self, value: Value) -> Result<(), &'static str> {
        if !matches!(
            self.control,
            Some(
                Control::TaskScopeExit { .. }
                    | Control::TaskSpawn { .. }
                    | Control::TaskAwait { .. }
            )
        ) {
            return Err("task runtime resumed a non-value boundary");
        }
        self.control = Some(Control::Value(value));
        Ok(())
    }

    pub(super) fn run(
        &mut self,
        interpreter: &mut Interpreter<'_, '_>,
    ) -> Result<EvaluationOutcome, RuntimeFault> {
        let effect_epoch = interpreter.host_effect_epoch;
        loop {
            if let Some(boundary) = self.boundary() {
                return Ok(EvaluationOutcome::Task(boundary));
            }
            let control = self.control.take().ok_or_else(|| {
                interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "task evaluation resumed after completion",
                })
            })?;
            self.control = Some(match control {
                Control::Expression {
                    module,
                    expression,
                    environment,
                } => step_expression(
                    interpreter,
                    module,
                    expression,
                    environment,
                    &mut self.frames,
                )?,
                Control::Definition(definition) => definition_control(interpreter, &definition)?,
                Control::Value(value) => {
                    let Some(frame) = self.frames.pop() else {
                        if matches!(value, Value::Resume { .. }) {
                            return Err(interpreter.fault_at_entry(
                                RuntimeFaultKind::InvalidCheckedCore {
                                    invariant: "handler continuation escaped its checked invocation",
                                },
                            ));
                        }
                        self.control = None;
                        return Ok(EvaluationOutcome::Complete(value));
                    };
                    continue_frame(interpreter, frame, value, &mut self.frames)?
                }
                Control::Apply {
                    module,
                    callable,
                    arguments,
                    span,
                } => apply(
                    interpreter,
                    module,
                    callable,
                    arguments,
                    span,
                    &mut self.frames,
                )?,
                Control::Operation {
                    module,
                    operation,
                    arguments,
                    span,
                } => dispatch_operation(
                    interpreter,
                    module,
                    operation,
                    arguments,
                    span,
                    &mut self.frames,
                )?,
                Control::TaskScopeEnter { .. }
                | Control::TaskScopeExit { .. }
                | Control::TaskSpawn { .. }
                | Control::TaskAwait { .. }
                | Control::TaskReturn { .. } => unreachable!("returned by boundary above"),
            });
            if interpreter.host_effect_epoch != effect_epoch {
                return Ok(EvaluationOutcome::HostEffect);
            }
        }
    }

    fn boundary(&self) -> Option<TaskBoundary> {
        match self.control.as_ref()? {
            Control::TaskScopeEnter {
                expression, span, ..
            } => Some(TaskBoundary::ScopeEnter {
                expression: *expression,
                span: *span,
            }),
            Control::TaskScopeExit {
                expression,
                span,
                value,
            } => Some(TaskBoundary::ScopeExit {
                expression: *expression,
                span: *span,
                value: value.clone(),
            }),
            Control::TaskSpawn {
                call,
                span,
                arguments,
            } => Some(TaskBoundary::Spawn {
                call: *call,
                span: *span,
                arguments: arguments.clone(),
            }),
            Control::TaskAwait {
                continuation,
                span,
                handle,
            } => Some(TaskBoundary::Await {
                continuation: *continuation,
                span: *span,
                handle: handle.clone(),
            }),
            Control::TaskReturn { span, value } => Some(TaskBoundary::Return {
                span: *span,
                value: value.clone(),
            }),
            _ => None,
        }
    }
}

pub(super) fn evaluate(
    interpreter: &mut Interpreter<'_, '_>,
    module: ModuleId,
    expression: hir::Expression,
    environment: Environment,
) -> Result<Value, RuntimeFault> {
    let mut evaluation = Evaluation::new(module, expression, environment);
    loop {
        match evaluation.run(interpreter)? {
            EvaluationOutcome::Complete(value) => return Ok(value),
            EvaluationOutcome::HostEffect => {}
            EvaluationOutcome::Task(boundary) => {
                let _ = boundary;
                return Err(
                    interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "checked task execution reached the ordinary evaluator",
                    }),
                );
            }
        }
    }
}

fn expression_control(
    module: ModuleId,
    expression: hir::Expression,
    environment: Environment,
) -> Control {
    Control::Expression {
        module,
        expression,
        environment,
    }
}

fn step_expression(
    interpreter: &mut Interpreter<'_, '_>,
    module: ModuleId,
    expression: hir::Expression,
    environment: Environment,
    frames: &mut Vec<Frame>,
) -> Result<Control, RuntimeFault> {
    let span = expression.span;
    match expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            start_sequence(interpreter, module, elements.into(), environment, frames)
        }
        hir::ExpressionKind::TaskScope { body, .. } => {
            let expression = ExpressionKey::new(module, expression.id);
            frames.push(Frame::TaskScope { expression, span });
            Ok(Control::TaskScopeEnter {
                module,
                expression,
                span,
                body: *body,
                environment,
            })
        }
        hir::ExpressionKind::TaskSpawn { call, .. } => {
            start_task_spawn(interpreter, module, *call, span, environment, frames)
        }
        hir::ExpressionKind::TaskAwait { handle, .. } => {
            frames.push(Frame::TaskAwait {
                continuation: ExpressionKey::new(module, expression.id),
                span,
            });
            Ok(expression_control(module, *handle, environment))
        }
        hir::ExpressionKind::TaskReturn { value, .. } => {
            frames.push(Frame::TaskReturn { span });
            Ok(expression_control(module, *value, environment))
        }
        hir::ExpressionKind::Handle { body, clauses } => {
            validate_handler(interpreter, module, expression.id, &body, &clauses, span)?;
            let id = interpreter.next_handler_id;
            interpreter.next_handler_id =
                interpreter.next_handler_id.checked_add(1).ok_or_else(|| {
                    interpreter.fault(module, span, "handler invocation identity overflow")
                })?;
            frames.push(Frame::HandlerBoundary(ActiveHandler {
                id,
                module,
                clauses,
                environment: environment.clone(),
            }));
            Ok(expression_control(module, *body, environment))
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            frames.push(Frame::If {
                module,
                then_branch: *then_branch,
                else_branch: *else_branch,
                environment: environment.clone(),
                span,
            });
            Ok(expression_control(module, *condition, environment))
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            frames.push(Frame::MatchScrutinee {
                module,
                cases: cases.into(),
                environment: environment.clone(),
                span,
            });
            Ok(expression_control(module, *scrutinee, environment))
        }
        hir::ExpressionKind::Assignment { place, value } => {
            frames.push(Frame::Assignment {
                module,
                place,
                environment: environment.clone(),
            });
            Ok(expression_control(module, *value, environment))
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            let key = ExpressionKey::new(module, expression.id);
            frames.push(Frame::ApplicationFunction {
                module,
                arguments: arguments.into(),
                environment: environment.clone(),
                span,
            });
            if let Some(call) = interpreter.checked.typed().trait_member_call(key) {
                Ok(Control::Definition(call.implementation().clone()))
            } else {
                Ok(expression_control(module, *function, environment))
            }
        }
        hir::ExpressionKind::Projection {
            reference,
            target,
            field,
        } => {
            if interpreter
                .checked
                .typed()
                .resolved()
                .reference(module, reference)
                .is_some()
            {
                reference_control(interpreter, module, reference, span, &environment)
            } else {
                frames.push(Frame::Projection {
                    module,
                    field: field.normalized,
                    span,
                });
                Ok(expression_control(module, *target, environment))
            }
        }
        hir::ExpressionKind::Name { reference, .. } => {
            reference_control(interpreter, module, reference, span, &environment)
        }
        hir::ExpressionKind::Binary {
            operator,
            left,
            right,
        } => {
            let frame = if matches!(
                operator,
                hir::BinaryOperator::BooleanAnd | hir::BinaryOperator::BooleanOr
            ) {
                Frame::BooleanLeft {
                    module,
                    operator,
                    right: *right,
                    environment: environment.clone(),
                }
            } else {
                Frame::BinaryLeft {
                    module,
                    operator,
                    right: *right,
                    environment: environment.clone(),
                    span,
                }
            };
            frames.push(frame);
            Ok(expression_control(module, *left, environment))
        }
        hir::ExpressionKind::Unary { operator, operand } => {
            frames.push(Frame::Unary {
                module,
                operator,
                span,
            });
            Ok(expression_control(module, *operand, environment))
        }
        hir::ExpressionKind::Literal(literal) => match literal {
            hir::Literal::Integer { .. } => interpreter
                .checked
                .typed()
                .integer(ExpressionKey::new(module, expression.id))
                .cloned()
                .map(|value| Control::Value(Value::Int(value)))
                .ok_or_else(|| interpreter.fault(module, span, "typed integer is absent")),
            hir::Literal::Float(value) => value
                .parse::<f64>()
                .map(|value| Control::Value(Value::Float64(value)))
                .map_err(|_| interpreter.fault(module, span, "typed float is invalid")),
            hir::Literal::Text(value) => Ok(Control::Value(Value::Text(value))),
            hir::Literal::Boolean(value) => Ok(Control::Value(Value::Bool(value))),
        },
        hir::ExpressionKind::Unit => Ok(Control::Value(Value::Unit)),
        hir::ExpressionKind::Tuple(elements) => start_aggregate(
            module,
            AggregateKind::Tuple,
            elements.into(),
            environment,
            frames,
        ),
        hir::ExpressionKind::List(elements) => start_aggregate(
            module,
            AggregateKind::List,
            elements.into(),
            environment,
            frames,
        ),
        hir::ExpressionKind::Record(fields) => {
            let type_id = interpreter
                .checked
                .typed()
                .expression_type(ExpressionKey::new(module, expression.id))
                .ok_or_else(|| interpreter.fault(module, span, "record type is absent"))?;
            let Type::NominalRecord { definition, .. } =
                interpreter.checked.typed().arena().get(type_id)
            else {
                return Err(interpreter.fault(module, span, "record has non-record type"));
            };
            start_record(
                module,
                definition.clone(),
                fields.into(),
                environment,
                frames,
            )
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            frames.push(Frame::RecordUpdateBase {
                module,
                fields: fields.into(),
                environment: environment.clone(),
                span,
            });
            Ok(expression_control(module, *base, environment))
        }
    }
}

fn continue_frame(
    interpreter: &mut Interpreter<'_, '_>,
    frame: Frame,
    value: Value,
    frames: &mut Vec<Frame>,
) -> Result<Control, RuntimeFault> {
    match frame {
        Frame::Sequence {
            module,
            remaining,
            environment,
        } => {
            if remaining.is_empty() {
                Ok(Control::Value(value))
            } else {
                start_sequence(interpreter, module, remaining, environment, frames)
            }
        }
        Frame::LocalBinding {
            module,
            binding,
            cell,
            remaining,
            mut environment,
        } => {
            *cell.borrow_mut() = value;
            environment.insert(BindingKey::new(module, binding.id), cell);
            start_sequence(interpreter, module, remaining, environment, frames)
        }
        Frame::TaskScope { expression, span } => Ok(Control::TaskScopeExit {
            expression,
            span,
            value,
        }),
        Frame::TaskSpawnArguments {
            module,
            call,
            span,
            mut values,
            mut remaining,
            environment,
        } => {
            values.push(value);
            if let Some(argument) = remaining.pop_front() {
                frames.push(Frame::TaskSpawnArguments {
                    module,
                    call,
                    span,
                    values,
                    remaining,
                    environment: environment.clone(),
                });
                Ok(expression_control(module, argument, environment))
            } else {
                Ok(Control::TaskSpawn {
                    call,
                    span,
                    arguments: values,
                })
            }
        }
        Frame::TaskAwait { continuation, span } => Ok(Control::TaskAwait {
            continuation,
            span,
            handle: value,
        }),
        Frame::TaskReturn { span } => Ok(Control::TaskReturn { span, value }),
        Frame::TaskLetAwaitHandle {
            module,
            binding,
            remaining,
            environment,
        } => {
            frames.push(Frame::TaskLetAwaitBinding {
                module,
                binding: binding.clone(),
                remaining,
                environment,
            });
            Ok(Control::TaskAwait {
                continuation: ExpressionKey::new(module, binding.call.id),
                span: binding.span,
                handle: value,
            })
        }
        Frame::TaskLetAwaitBinding {
            module,
            binding,
            remaining,
            mut environment,
        } => {
            if !interpreter.match_pattern(module, &binding.pattern, &value, &mut environment)? {
                return Err(interpreter.fault(
                    module,
                    binding.pattern.span,
                    "checked let! result did not match",
                ));
            }
            start_sequence(interpreter, module, remaining, environment, frames)
        }
        Frame::If {
            module,
            then_branch,
            else_branch,
            environment,
            span,
        } => match value {
            Value::Bool(true) => Ok(expression_control(module, then_branch, environment)),
            Value::Bool(false) => Ok(expression_control(module, else_branch, environment)),
            _ => Err(interpreter.fault(module, span, "if condition is not Bool")),
        },
        Frame::MatchScrutinee {
            module,
            cases,
            environment,
            span,
        } => select_match_case(interpreter, module, value, cases, environment, span, frames),
        Frame::MatchGuard {
            module,
            scrutinee,
            body,
            remaining,
            original_environment,
            case_environment,
            span,
        } => match value {
            Value::Bool(true) => Ok(expression_control(module, body, case_environment)),
            Value::Bool(false) => select_match_case(
                interpreter,
                module,
                scrutinee,
                remaining,
                original_environment,
                span,
                frames,
            ),
            _ => Err(interpreter.fault(module, span, "match guard is not Bool")),
        },
        Frame::Assignment {
            module,
            place,
            environment,
        } => {
            let target = interpreter.reference_target(module, place.root_reference)?;
            let ReferenceTarget::Binding(binding) = target else {
                return Err(interpreter.fault(
                    module,
                    place.span,
                    "assignment root is not a local binding",
                ));
            };
            let Some(cell) = environment.get(&binding) else {
                return Err(interpreter.fault(module, place.span, "assignment cell is absent"));
            };
            if place.fields.is_empty() {
                *cell.borrow_mut() = value;
            } else {
                let mut root = cell.borrow_mut();
                assign_fields(&mut root, &place.fields, value)
                    .map_err(|invariant| interpreter.fault(module, place.span, invariant))?;
            }
            Ok(Control::Value(Value::Unit))
        }
        Frame::ApplicationFunction {
            module,
            mut arguments,
            environment,
            span,
        } => {
            if let Some(argument) = arguments.pop_front() {
                frames.push(Frame::ApplicationArgument {
                    module,
                    callable: value,
                    values: Vec::new(),
                    remaining: arguments,
                    environment: environment.clone(),
                    span,
                });
                Ok(expression_control(module, argument, environment))
            } else {
                Ok(Control::Apply {
                    module,
                    callable: value,
                    arguments: Vec::new(),
                    span,
                })
            }
        }
        Frame::ApplicationArgument {
            module,
            callable,
            mut values,
            mut remaining,
            environment,
            span,
        } => {
            values.push(value);
            if let Some(argument) = remaining.pop_front() {
                frames.push(Frame::ApplicationArgument {
                    module,
                    callable,
                    values,
                    remaining,
                    environment: environment.clone(),
                    span,
                });
                Ok(expression_control(module, argument, environment))
            } else {
                Ok(Control::Apply {
                    module,
                    callable,
                    arguments: values,
                    span,
                })
            }
        }
        Frame::Projection {
            module,
            field,
            span,
        } => match value {
            Value::Record { fields, .. } => fields
                .get(&field)
                .cloned()
                .map(Control::Value)
                .ok_or_else(|| interpreter.fault(module, span, "record field is absent")),
            _ => Err(interpreter.fault(module, span, "projection target is not a record")),
        },
        Frame::BooleanLeft {
            module,
            operator,
            right,
            environment,
        } => {
            let Value::Bool(left) = value else {
                return Err(interpreter.fault(module, right.span, "boolean operand is not Bool"));
            };
            if (operator == hir::BinaryOperator::BooleanAnd && !left)
                || (operator == hir::BinaryOperator::BooleanOr && left)
            {
                Ok(Control::Value(Value::Bool(left)))
            } else {
                frames.push(Frame::BooleanRight {
                    module,
                    span: right.span,
                });
                Ok(expression_control(module, right, environment))
            }
        }
        Frame::BooleanRight { module, span } => match value {
            Value::Bool(value) => Ok(Control::Value(Value::Bool(value))),
            _ => Err(interpreter.fault(module, span, "boolean operand is not Bool")),
        },
        Frame::BinaryLeft {
            module,
            operator,
            right,
            environment,
            span,
        } => {
            frames.push(Frame::BinaryRight {
                module,
                operator,
                left: value,
                span,
            });
            Ok(expression_control(module, right, environment))
        }
        Frame::BinaryRight {
            module,
            operator,
            left,
            span,
        } => interpreter
            .eval_binary(module, span, operator, left, value)
            .map(Control::Value),
        Frame::Unary {
            module,
            operator,
            span,
        } => match (operator, value) {
            (hir::UnaryOperator::Positive, Value::Int(value)) => {
                Ok(Control::Value(Value::Int(value)))
            }
            (hir::UnaryOperator::Negative, Value::Int(value)) => {
                Ok(Control::Value(Value::Int(-value)))
            }
            _ => Err(interpreter.fault(module, span, "unary operand is not Int")),
        },
        Frame::Aggregate {
            module,
            kind,
            mut values,
            mut remaining,
            environment,
        } => {
            values.push(value);
            if let Some(next) = remaining.pop_front() {
                frames.push(Frame::Aggregate {
                    module,
                    kind,
                    values,
                    remaining,
                    environment: environment.clone(),
                });
                Ok(expression_control(module, next, environment))
            } else {
                Ok(Control::Value(match kind {
                    AggregateKind::Tuple => Value::Tuple(values),
                    AggregateKind::List => Value::List(values),
                }))
            }
        }
        Frame::Record {
            module,
            definition,
            mut names,
            mut values,
            mut remaining,
            environment,
        } => {
            let name = names
                .pop_front()
                .expect("record field name accompanies expression");
            values.insert(name, value);
            if let Some(next) = remaining.pop_front() {
                frames.push(Frame::Record {
                    module,
                    definition,
                    names,
                    values,
                    remaining,
                    environment: environment.clone(),
                });
                Ok(expression_control(module, next, environment))
            } else {
                Ok(Control::Value(Value::Record {
                    definition,
                    fields: values,
                }))
            }
        }
        Frame::RecordUpdateBase {
            module,
            mut fields,
            environment,
            span,
        } => {
            let Value::Record {
                definition,
                fields: values,
            } = value
            else {
                return Err(interpreter.fault(module, span, "record update base is not a record"));
            };
            if let Some(field) = fields.pop_front() {
                frames.push(Frame::RecordUpdateField {
                    module,
                    definition,
                    fields: values,
                    current_name: field.name.normalized,
                    remaining: fields,
                    environment: environment.clone(),
                });
                Ok(expression_control(module, field.value, environment))
            } else {
                Ok(Control::Value(Value::Record {
                    definition,
                    fields: values,
                }))
            }
        }
        Frame::RecordUpdateField {
            module,
            definition,
            mut fields,
            current_name,
            mut remaining,
            environment,
        } => {
            fields.insert(current_name, value);
            if let Some(field) = remaining.pop_front() {
                frames.push(Frame::RecordUpdateField {
                    module,
                    definition,
                    fields,
                    current_name: field.name.normalized,
                    remaining,
                    environment: environment.clone(),
                });
                Ok(expression_control(module, field.value, environment))
            } else {
                Ok(Control::Value(Value::Record { definition, fields }))
            }
        }
        Frame::Map {
            module,
            callable,
            mut values,
            mut output,
            span,
        } => {
            output.push(value);
            if let Some(next) = values.pop_front() {
                frames.push(Frame::Map {
                    module,
                    callable: callable.clone(),
                    values,
                    output,
                    span,
                });
                Ok(Control::Apply {
                    module,
                    callable,
                    arguments: vec![next],
                    span,
                })
            } else {
                Ok(Control::Value(Value::List(output)))
            }
        }
        Frame::HandlerBoundary(_) => Ok(Control::Value(value)),
        Frame::ClauseBoundary { active, .. } => {
            active.set(false);
            Ok(Control::Value(value))
        }
        Frame::ResumeReturn {
            clause_boundary,
            clause_frames,
        } => {
            frames.push(*clause_boundary);
            frames.extend(clause_frames);
            Ok(Control::Value(value))
        }
    }
}

fn start_task_spawn(
    interpreter: &Interpreter<'_, '_>,
    module: ModuleId,
    call: hir::Expression,
    span: Span,
    environment: Environment,
    frames: &mut Vec<Frame>,
) -> Result<Control, RuntimeFault> {
    let call_key = ExpressionKey::new(module, call.id);
    let hir::ExpressionKind::Application { arguments, .. } = call.kind else {
        return Err(interpreter.fault(
            module,
            call.span,
            "checked Task spawn is not a direct application",
        ));
    };
    let mut remaining = VecDeque::from(arguments);
    let Some(argument) = remaining.pop_front() else {
        return Ok(Control::TaskSpawn {
            call: call_key,
            span,
            arguments: Vec::new(),
        });
    };
    frames.push(Frame::TaskSpawnArguments {
        module,
        call: call_key,
        span,
        values: Vec::new(),
        remaining,
        environment: environment.clone(),
    });
    Ok(expression_control(module, argument, environment))
}

fn start_sequence(
    interpreter: &mut Interpreter<'_, '_>,
    module: ModuleId,
    mut elements: VecDeque<hir::SequenceElement>,
    mut environment: Environment,
    frames: &mut Vec<Frame>,
) -> Result<Control, RuntimeFault> {
    loop {
        let Some(element) = elements.pop_front() else {
            return Ok(Control::Value(Value::Unit));
        };
        match element {
            hir::SequenceElement::LetAwait(binding) => {
                frames.push(Frame::TaskLetAwaitHandle {
                    module,
                    binding: binding.clone(),
                    remaining: elements,
                    environment: environment.clone(),
                });
                return start_task_spawn(
                    interpreter,
                    module,
                    binding.call,
                    binding.span,
                    environment,
                    frames,
                );
            }
            hir::SequenceElement::Expression(expression) => {
                frames.push(Frame::Sequence {
                    module,
                    remaining: elements,
                    environment: environment.clone(),
                });
                return Ok(expression_control(module, expression, environment));
            }
            hir::SequenceElement::Let(binding) if binding.parameters.is_empty() => {
                let key = BindingKey::new(module, binding.id);
                let cell = Rc::new(RefCell::new(Value::Unit));
                if binding.recursive {
                    environment.insert(key, Rc::clone(&cell));
                }
                frames.push(Frame::LocalBinding {
                    module,
                    binding: binding.clone(),
                    cell,
                    remaining: elements,
                    environment: environment.clone(),
                });
                return Ok(expression_control(module, binding.value, environment));
            }
            hir::SequenceElement::Let(binding) => {
                let key = BindingKey::new(module, binding.id);
                let cell = Rc::new(RefCell::new(Value::Unit));
                if binding.recursive {
                    environment.insert(key, Rc::clone(&cell));
                }
                let value = Value::Closure(Box::new(Closure {
                    module,
                    parameters: binding.parameters,
                    body: binding.value,
                    environment: environment.clone(),
                }));
                *cell.borrow_mut() = value;
                environment.insert(key, cell);
            }
        }
    }
}

fn start_aggregate(
    module: ModuleId,
    kind: AggregateKind,
    mut elements: VecDeque<hir::Expression>,
    environment: Environment,
    frames: &mut Vec<Frame>,
) -> Result<Control, RuntimeFault> {
    let Some(first) = elements.pop_front() else {
        return Ok(Control::Value(match kind {
            AggregateKind::Tuple => Value::Tuple(Vec::new()),
            AggregateKind::List => Value::List(Vec::new()),
        }));
    };
    frames.push(Frame::Aggregate {
        module,
        kind,
        values: Vec::new(),
        remaining: elements,
        environment: environment.clone(),
    });
    Ok(expression_control(module, first, environment))
}

fn start_record(
    module: ModuleId,
    definition: DefinitionId,
    fields: VecDeque<hir::RecordField>,
    environment: Environment,
    frames: &mut Vec<Frame>,
) -> Result<Control, RuntimeFault> {
    let mut names = fields
        .iter()
        .map(|field| field.name.normalized.clone())
        .collect::<VecDeque<_>>();
    let mut expressions = fields
        .into_iter()
        .map(|field| field.value)
        .collect::<VecDeque<_>>();
    let Some(first) = expressions.pop_front() else {
        return Ok(Control::Value(Value::Record {
            definition,
            fields: BTreeMap::new(),
        }));
    };
    debug_assert!(!names.is_empty());
    frames.push(Frame::Record {
        module,
        definition,
        names: std::mem::take(&mut names),
        values: BTreeMap::new(),
        remaining: expressions,
        environment: environment.clone(),
    });
    Ok(expression_control(module, first, environment))
}

fn select_match_case(
    interpreter: &mut Interpreter<'_, '_>,
    module: ModuleId,
    scrutinee: Value,
    mut cases: VecDeque<hir::MatchCase>,
    environment: Environment,
    span: Span,
    frames: &mut Vec<Frame>,
) -> Result<Control, RuntimeFault> {
    while let Some(case) = cases.pop_front() {
        let mut case_environment = environment.clone();
        if interpreter.match_pattern(module, &case.pattern, &scrutinee, &mut case_environment)? {
            if let Some(guard) = case.guard {
                frames.push(Frame::MatchGuard {
                    module,
                    scrutinee: scrutinee.clone(),
                    body: case.body,
                    remaining: cases,
                    original_environment: environment.clone(),
                    case_environment: case_environment.clone(),
                    span,
                });
                return Ok(expression_control(module, guard, case_environment));
            }
            return Ok(expression_control(module, case.body, case_environment));
        }
    }
    Err(interpreter.fault(module, span, "checked match had no matching case"))
}

fn reference_control(
    interpreter: &Interpreter<'_, '_>,
    module: ModuleId,
    reference: hir::ReferenceId,
    source_span: Span,
    environment: &Environment,
) -> Result<Control, RuntimeFault> {
    match interpreter.reference_target(module, reference)? {
        ReferenceTarget::Definition(definition) => Ok(Control::Definition(definition)),
        ReferenceTarget::Binding(binding) => environment.get(&binding).map_or_else(
            || {
                Err(interpreter.fault(
                    module,
                    interpreter.module_span(module),
                    "binding cell is absent",
                ))
            },
            |cell| {
                let value = match cell.borrow().clone() {
                    Value::Resume { continuation, .. } => Value::Resume {
                        continuation,
                        source_span,
                    },
                    value => value,
                };
                Ok(Control::Value(value))
            },
        ),
    }
}

fn definition_control(
    interpreter: &mut Interpreter<'_, '_>,
    definition: &DefinitionId,
) -> Result<Control, RuntimeFault> {
    let resolved = interpreter.checked.typed().resolved();
    let info = resolved.definition(definition).cloned().ok_or_else(|| {
        interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
            invariant: "definition is absent",
        })
    })?;
    if resolved.trait_member(definition).is_some() {
        return Err(
            interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "bare Trait member reached evaluation",
            }),
        );
    }
    let implementation_member = resolved.impl_member(definition).cloned();
    match info.origin {
        DefinitionOrigin::Builtin(builtin) => Ok(Control::Value(Value::Builtin {
            builtin,
            arguments: Vec::new(),
        })),
        DefinitionOrigin::Prelude(_) => interpreter
            .constructor_value(definition)
            .map(Control::Value)
            .ok_or_else(|| {
                interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
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
                    .and_then(|implementation| implementation.members.get(member.member_ordinal))
                    .ok_or_else(|| {
                        interpreter.fault(
                            module,
                            resolved_module.hir.span,
                            "implementation member body is absent",
                        )
                    })?;
                return if implementation.parameters.is_empty() {
                    Ok(expression_control(
                        module,
                        implementation.value.clone(),
                        Environment::new(),
                    ))
                } else {
                    Ok(Control::Value(Value::Closure(Box::new(Closure {
                        module,
                        parameters: implementation.parameters.clone(),
                        body: implementation.value.clone(),
                        environment: Environment::new(),
                    }))))
                };
            }
            if info.kind == DefinitionKind::Constructor {
                if let Some(value) = interpreter.constructor_value(definition) {
                    return Ok(Control::Value(value));
                }
            }
            let value = resolved_module
                .hir
                .definitions
                .iter()
                .find(|value| value.name.normalized == info.name)
                .ok_or_else(|| {
                    interpreter.fault(
                        module,
                        resolved_module.hir.span,
                        "definition body is absent",
                    )
                })?;
            if value.parameters.is_empty() {
                Ok(expression_control(
                    module,
                    value.value.clone(),
                    Environment::new(),
                ))
            } else {
                Ok(Control::Value(Value::Closure(Box::new(Closure {
                    module,
                    parameters: value.parameters.clone(),
                    body: value.value.clone(),
                    environment: Environment::new(),
                }))))
            }
        }
    }
}

fn apply(
    interpreter: &mut Interpreter<'_, '_>,
    module: ModuleId,
    callable: Value,
    arguments: Vec<Value>,
    span: Span,
    frames: &mut Vec<Frame>,
) -> Result<Control, RuntimeFault> {
    match callable {
        Value::Closure(closure) => {
            if arguments.len() > closure.parameters.len() {
                return Err(interpreter.fault(
                    closure.module,
                    span,
                    "checked call supplied too many arguments",
                ));
            }
            let mut environment = closure.environment.clone();
            for (pattern, value) in closure.parameters.iter().zip(&arguments) {
                if !interpreter.match_pattern(closure.module, pattern, value, &mut environment)? {
                    return Err(interpreter.fault(
                        closure.module,
                        span,
                        "checked function argument did not match",
                    ));
                }
            }
            if arguments.len() == closure.parameters.len() {
                Ok(expression_control(
                    closure.module,
                    closure.body,
                    environment,
                ))
            } else {
                Ok(Control::Value(Value::Closure(Box::new(Closure {
                    module: closure.module,
                    parameters: closure.parameters[arguments.len()..].to_vec(),
                    body: closure.body,
                    environment,
                }))))
            }
        }
        Value::Builtin {
            builtin,
            arguments: mut captured,
        } => {
            captured.extend(arguments);
            let arity = match builtin {
                Builtin::ConsoleWrite | Builtin::Sum => 1,
                Builtin::TextFormat | Builtin::Max | Builtin::Min | Builtin::Map => 2,
            };
            if captured.len() < arity {
                return Ok(Control::Value(Value::Builtin {
                    builtin,
                    arguments: captured,
                }));
            }
            if captured.len() > arity {
                return Err(
                    interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "checked builtin call supplied too many arguments",
                    }),
                );
            }
            match builtin {
                Builtin::ConsoleWrite => {
                    let [Value::Text(text)] = captured.as_slice() else {
                        return Err(interpreter.fault(
                            module,
                            span,
                            "Console.write argument is not Text",
                        ));
                    };
                    Ok(Control::Operation {
                        module,
                        operation: "Console.Write.write".to_owned(),
                        arguments: vec![Value::Text(text.clone())],
                        span,
                    })
                }
                Builtin::Map => {
                    let [callable, Value::List(values)] = captured.as_slice() else {
                        return Err(interpreter.fault(
                            module,
                            span,
                            "map arguments have invalid values",
                        ));
                    };
                    let mut values = VecDeque::from(values.clone());
                    let Some(first) = values.pop_front() else {
                        return Ok(Control::Value(Value::List(Vec::new())));
                    };
                    frames.push(Frame::Map {
                        module,
                        callable: callable.clone(),
                        values,
                        output: Vec::new(),
                        span,
                    });
                    Ok(Control::Apply {
                        module,
                        callable: callable.clone(),
                        arguments: vec![first],
                        span,
                    })
                }
                _ => interpreter
                    .apply_builtin(builtin, captured, span)
                    .map(Control::Value),
            }
        }
        Value::Constructor { definition, case } => {
            if arguments.len() == 1 {
                Ok(Control::Value(Value::Variant {
                    definition,
                    case,
                    payload: Some(Box::new(
                        arguments.into_iter().next().expect("length checked"),
                    )),
                }))
            } else {
                Err(
                    interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "variant constructor arity is not one",
                    }),
                )
            }
        }
        Value::Resume {
            continuation,
            source_span,
        } => {
            if arguments.len() != 1 {
                return Err(
                    interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "handler resume arity is not one",
                    }),
                );
            }
            if !continuation.active.get() {
                return Err(
                    interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "handler continuation escaped its owning invocation",
                    }),
                );
            }
            let uses = continuation.uses.get();
            if continuation.mode == HandlerResumeMode::Once && uses >= 1 {
                return Err(RuntimeFault {
                    kind: RuntimeFaultKind::HandlerResumeCardinality {
                        operation: continuation.operation.clone(),
                        mode: continuation.mode,
                    },
                    source_name: interpreter.module_source(module),
                    span: source_span,
                });
            }
            continuation.uses.set(uses.saturating_add(1));
            let boundary_index = frames
                .iter()
                .rposition(|frame| {
                    matches!(
                        frame,
                        Frame::ClauseBoundary { handler_id, .. }
                            if *handler_id == continuation.handler_id
                    )
                })
                .ok_or_else(|| {
                    interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "handler clause boundary is absent during resume",
                    })
                })?;
            let clause_frames = frames.split_off(boundary_index + 1);
            let clause_boundary = frames.pop().expect("located clause boundary");
            frames.push(Frame::ResumeReturn {
                clause_boundary: Box::new(clause_boundary),
                clause_frames,
            });
            frames.extend(continuation.frames.clone());
            Ok(Control::Value(
                arguments.into_iter().next().expect("length checked"),
            ))
        }
        _ => Err(
            interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "checked application target is not callable",
            }),
        ),
    }
}

fn dispatch_operation(
    interpreter: &mut Interpreter<'_, '_>,
    module: ModuleId,
    operation: String,
    arguments: Vec<Value>,
    span: Span,
    frames: &mut Vec<Frame>,
) -> Result<Control, RuntimeFault> {
    let handler_index = frames.iter().rposition(
        |frame| matches!(frame, Frame::HandlerBoundary(handler) if handler.handles(&operation)),
    );
    let Some(handler_index) = handler_index else {
        if operation == "Console.Write.write" {
            let [Value::Text(text)] = arguments.as_slice() else {
                return Err(interpreter.fault(module, span, "Console.write argument is not Text"));
            };
            let mut line = text.clone();
            line.push('\n');
            interpreter
                .console
                .write(&line)
                .map_err(|error| RuntimeFault {
                    kind: RuntimeFaultKind::HostCapability {
                        operation: "Console.write",
                        category: error.category(),
                    },
                    source_name: interpreter.module_source(module),
                    span,
                })?;
            interpreter.record_host_effect(module, span)?;
            return Ok(Control::Value(Value::Unit));
        }
        return Err(interpreter.fault(module, span, "unhandled checked Effect operation"));
    };

    let captured = frames.split_off(handler_index + 1);
    let Frame::HandlerBoundary(handler) = frames.pop().expect("located handler boundary") else {
        unreachable!("located frame is a handler boundary")
    };
    let clause = handler
        .clause(&operation)
        .cloned()
        .expect("matching handler clause exists");
    let registered = resolve_handler_operation(&operation).ok_or_else(|| {
        interpreter.fault(
            handler.module,
            clause.operation.span,
            "handler operation is unregistered",
        )
    })?;
    if arguments.len() != clause.parameters.len() {
        return Err(
            interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "handler operation argument count disagrees with checked clause",
            }),
        );
    }

    let mut environment = handler.environment.clone();
    for (pattern, value) in clause.parameters.iter().zip(&arguments) {
        if !interpreter.match_pattern(handler.module, pattern, value, &mut environment)? {
            return Err(
                interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "checked handler operation input did not match",
                }),
            );
        }
    }

    let active = Rc::new(ScalarCell::new(true));
    if let Some(resume) = &clause.resume {
        let continuation = ContinuationValue {
            handler_id: handler.id,
            operation: operation.clone(),
            mode: registered.resume_mode(),
            frames: std::iter::once(Frame::HandlerBoundary(handler.clone()))
                .chain(captured)
                .collect(),
            uses: ScalarCell::new(0),
            active: Rc::clone(&active),
        };
        environment.insert(
            BindingKey::new(handler.module, resume.id),
            Rc::new(RefCell::new(Value::Resume {
                continuation: Rc::new(continuation),
                source_span: resume.name.span,
            })),
        );
    }
    frames.push(Frame::ClauseBoundary {
        handler_id: handler.id,
        active,
    });
    Ok(expression_control(handler.module, clause.body, environment))
}

fn validate_handler(
    interpreter: &Interpreter<'_, '_>,
    module: ModuleId,
    expression: hir::ExpressionId,
    body: &hir::Expression,
    clauses: &[hir::HandlerClause],
    span: Span,
) -> Result<(), RuntimeFault> {
    let Some(core) = interpreter
        .checked
        .handler_core(ExpressionKey::new(module, expression))
    else {
        return Err(
            interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "checked handler Core is absent",
            }),
        );
    };
    if core.body().get() != body.id.get().saturating_add(1) || core.clauses().len() != clauses.len()
    {
        return Err(
            interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "checked handler Core identity disagrees with HIR",
            }),
        );
    }
    let key = ExpressionKey::new(module, expression);
    let Some(return_type) = interpreter.checked.typed().expression_type(key) else {
        return Err(
            interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "checked handler result type is absent",
            }),
        );
    };
    let canonical_return_type = interpreter
        .checked
        .typed()
        .display_type(return_type)
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    let source = interpreter.module_source(module);
    if core.return_type().as_str() != canonical_return_type
        || !matches!(
            core.source_span(),
            Some(core_span)
                if core_span.file() == source
                    && core_span.start_byte() == u64::from(span.start().get())
                    && core_span.end_byte() == u64::from(span.end().get())
        )
    {
        return Err(
            interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "checked handler result type or source span disagrees with HIR",
            }),
        );
    }
    for clause in clauses {
        let operation = clause.operation.normalized();
        let Some(registered) = resolve_handler_operation(&operation) else {
            return Err(interpreter.fault(
                module,
                clause.operation.span,
                "handler operation is unregistered",
            ));
        };
        let expected_resume_use = if let Some(resume) = &clause.resume {
            match interpreter
                .checked
                .typed()
                .resolved()
                .handler_resume_uses(BindingKey::new(module, resume.id))
            {
                Some(0) => ResumeUse::Never,
                Some(1) => ResumeUse::Once,
                Some(_) => ResumeUse::Many,
                None => {
                    return Err(
                        interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                            invariant: "checked handler resume-use metadata is absent",
                        }),
                    );
                }
            }
        } else {
            ResumeUse::Never
        };
        let matches = core.clauses().iter().any(|core_clause| {
            core_clause.clause().operation().owner().as_str() == registered.owner()
                && core_clause.clause().operation().name() == registered.operation()
                && core_clause.clause().operation().inputs().len() == registered.inputs().len()
                && core_clause
                    .clause()
                    .operation()
                    .inputs()
                    .iter()
                    .zip(registered.inputs())
                    .all(|(actual, expected)| actual.as_str() == handler_value_type_name(*expected))
                && core_clause.clause().operation().output().as_str()
                    == handler_value_type_name(registered.output())
                && core_clause.clause().operation().resume_mode()
                    == handler_resume_mode(registered.resume_mode())
                && core_clause.body().get() == clause.body.id.get().saturating_add(1)
                && core_clause.resume_use() == expected_resume_use
        });
        if !matches {
            return Err(
                interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "checked handler clause identity disagrees with HIR",
                }),
            );
        }
    }
    if span.start() > span.end() {
        return Err(
            interpreter.fault_at_entry(RuntimeFaultKind::InvalidCheckedCore {
                invariant: "checked handler source span is invalid",
            }),
        );
    }
    Ok(())
}

const fn handler_value_type_name(value: HandlerValueType) -> &'static str {
    match value {
        HandlerValueType::Unit => "Unit",
        HandlerValueType::Int => "Int",
        HandlerValueType::Text => "Text",
    }
}

const fn handler_resume_mode(value: HandlerResumeMode) -> ResumeMode {
    match value {
        HandlerResumeMode::Never => ResumeMode::Never,
        HandlerResumeMode::Once => ResumeMode::Once,
        HandlerResumeMode::Many => ResumeMode::Many,
    }
}
