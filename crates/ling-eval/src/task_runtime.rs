//! DEC-0266 scheduler-neutral Structured Task lifecycle kernel.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use ling_concurrency::{CleanupRegionId, ScopeId, StateId, TaskId};
use ling_effects::{
    CHECKED_TASK_MACHINE_VERSION, CheckedProgram, CheckedTaskCore, CheckedTaskMachine,
    CheckedTaskMachineEdgeKind, CheckedTaskMachineStateKind,
};
use ling_resolve::{DefinitionId, DefinitionKind, DefinitionOrigin, ExpressionKey, ModuleId};
use ling_source::Span;
use ling_types::{Type, TypeId};
use num_bigint::BigInt;

use super::machine::{Evaluation, EvaluationOutcome, TaskBoundary};
use super::{Console, Environment, Interpreter, RuntimeFault, RuntimeFaultKind, Value};

/// One scheduler-independent dynamic Task identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaskPath(Box<[u32]>);

impl TaskPath {
    #[must_use]
    pub fn root() -> Self {
        Self(Box::new([]))
    }

    #[must_use]
    pub fn segments(&self) -> &[u32] {
        &self.0
    }

    fn child(&self, task: TaskId) -> Option<Self> {
        if !task.is_valid() {
            return None;
        }
        let mut segments = self.0.to_vec();
        segments.push(task.get());
        Some(Self(segments.into_boxed_slice()))
    }

    fn lexical_task(&self) -> TaskId {
        self.0
            .last()
            .copied()
            .map_or_else(|| TaskId::new(1), TaskId::new)
    }
}

impl fmt::Display for TaskPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Root")?;
        for segment in &self.0 {
            write!(formatter, "/{segment}")?;
        }
        Ok(())
    }
}

/// Checked values accepted and returned by the internal Task embedding boundary.
#[derive(Clone, Debug, PartialEq)]
pub enum TaskValue {
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
}

impl From<TaskValue> for Value {
    fn from(value: TaskValue) -> Self {
        match value {
            TaskValue::Unit => Self::Unit,
            TaskValue::Bool(value) => Self::Bool(value),
            TaskValue::Int(value) => Self::Int(value),
            TaskValue::Float64(value) => Self::Float64(value),
            TaskValue::Text(value) => Self::Text(value),
            TaskValue::Tuple(values) => Self::Tuple(values.into_iter().map(Self::from).collect()),
            TaskValue::List(values) => Self::List(values.into_iter().map(Self::from).collect()),
            TaskValue::Record { definition, fields } => Self::Record {
                definition,
                fields: fields
                    .into_iter()
                    .map(|(name, value)| (name, Self::from(value)))
                    .collect(),
            },
            TaskValue::Variant {
                definition,
                case,
                payload,
            } => Self::Variant {
                definition,
                case,
                payload: payload.map(|value| Box::new(Self::from(*value))),
            },
        }
    }
}

impl TryFrom<Value> for TaskValue {
    type Error = &'static str;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Unit => Ok(Self::Unit),
            Value::Bool(value) => Ok(Self::Bool(value)),
            Value::Int(value) => Ok(Self::Int(value)),
            Value::Float64(value) => Ok(Self::Float64(value)),
            Value::Text(value) => Ok(Self::Text(value)),
            Value::Tuple(values) => values
                .into_iter()
                .map(Self::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map(Self::Tuple),
            Value::List(values) => values
                .into_iter()
                .map(Self::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map(Self::List),
            Value::Record { definition, fields } => fields
                .into_iter()
                .map(|(name, value)| Ok((name, Self::try_from(value)?)))
                .collect::<Result<BTreeMap<_, _>, _>>()
                .map(|fields| Self::Record { definition, fields }),
            Value::Variant {
                definition,
                case,
                payload,
            } => Ok(Self::Variant {
                definition,
                case,
                payload: payload
                    .map(|value| Self::try_from(*value).map(Box::new))
                    .transpose()?,
            }),
            Value::Closure(_)
            | Value::Builtin { .. }
            | Value::Constructor { .. }
            | Value::Resume { .. }
            | Value::TaskHandle(_) => Err("Task result is not a publishable checked value"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskRuntimeLimits {
    max_tasks: usize,
    max_scopes: usize,
    max_steps: usize,
    max_faults: usize,
}

impl TaskRuntimeLimits {
    #[must_use]
    pub const fn new(
        max_tasks: usize,
        max_scopes: usize,
        max_steps: usize,
        max_faults: usize,
    ) -> Self {
        Self {
            max_tasks,
            max_scopes,
            max_steps,
            max_faults,
        }
    }

    #[must_use]
    pub const fn max_tasks(self) -> usize {
        self.max_tasks
    }

    #[must_use]
    pub const fn max_scopes(self) -> usize {
        self.max_scopes
    }

    #[must_use]
    pub const fn max_steps(self) -> usize {
        self.max_steps
    }

    #[must_use]
    pub const fn max_faults(self) -> usize {
        self.max_faults
    }

    fn is_valid(self) -> bool {
        self.max_tasks > 0 && self.max_scopes > 0 && self.max_steps > 0 && self.max_faults > 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskCancellationCause {
    Requested,
    Ancestor,
    Deadline,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TaskRuntimeState {
    Ready,
    Running,
    Suspended { awaited: TaskPath },
    Joining { scope: u32 },
    Cancelling { cause: TaskCancellationCause },
    Cleaning { reason: &'static str },
    Completed(TaskValue),
    Cancelled,
    Faulted { fault_count: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskStepKind {
    ScopeOpened { scope: u32 },
    ScopeClosed { scope: u32 },
    ChildRegistered { child: TaskPath },
    Suspended { child: TaskPath },
    AwaitReady { child: TaskPath },
    HostEffectCompleted,
    JoinPending { scope: u32 },
    CancellationPropagated,
    Completed,
    Cancelled,
    Faulted { fault_count: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskStep {
    task: TaskPath,
    kind: TaskStepKind,
}

impl TaskStep {
    #[must_use]
    pub fn task(&self) -> &TaskPath {
        &self.task
    }

    #[must_use]
    pub const fn kind(&self) -> &TaskStepKind {
        &self.kind
    }
}

#[derive(Clone, Debug)]
struct ScopeRuntime {
    state: ScopeState,
    parent: Option<ScopeId>,
    children: BTreeSet<TaskPath>,
    handles: BTreeMap<TaskPath, bool>,
    span: Span,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScopeState {
    Open,
    Closing,
    Closed,
}

#[derive(Clone, Debug)]
enum Phase {
    Ready,
    Running,
    Suspended {
        awaited: TaskPath,
    },
    Joining {
        scope: ScopeId,
        value: Value,
    },
    Returning(Value),
    Cancelling {
        cause: TaskCancellationCause,
        propagated: bool,
    },
    FaultPending {
        propagated: bool,
    },
    Completed(Value),
    Cancelled,
    Faulted,
}

#[derive(Clone, Debug)]
struct TaskInstance {
    path: TaskPath,
    definition: DefinitionId,
    owner: Option<(TaskPath, ScopeId)>,
    evaluation: Evaluation,
    phase: Phase,
    pending_resume: Option<Value>,
    scopes: BTreeMap<ScopeId, ScopeRuntime>,
    scope_stack: Vec<ScopeId>,
    checkpoint: StateId,
    cleanup: CleanupRegionId,
    cleanup_count: usize,
    faults: BTreeMap<TaskPath, RuntimeFault>,
    type_bindings: BTreeMap<u32, RuntimeTypeShape>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum RuntimeTypeShape {
    Unknown,
    Unit,
    Bool,
    Int,
    Float64,
    Text,
    Tuple(Vec<Self>),
    List(Box<Self>),
    Record(DefinitionId),
    Variant(DefinitionId),
}

/// Explicitly-driven Task lifecycle kernel. It never selects a ready Task.
pub struct TaskRuntime<'checked, 'console> {
    interpreter: Interpreter<'checked, 'console>,
    tasks: BTreeMap<TaskPath, TaskInstance>,
    limits: TaskRuntimeLimits,
    steps: usize,
    step_limit_faulted: bool,
}

impl<'checked, 'console> TaskRuntime<'checked, 'console> {
    /// Creates one caller-selected checked Task root without exposing it as a
    /// Ling source entry point.
    pub fn new(
        checked: &'checked CheckedProgram,
        root: &DefinitionId,
        arguments: Vec<TaskValue>,
        console: &'console mut dyn Console,
        limits: TaskRuntimeLimits,
    ) -> Result<Self, RuntimeFault> {
        let core = checked.task_core(root).ok_or_else(|| {
            entry_fault(
                checked,
                RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "TaskRuntime root has no Checked Task Core",
                },
            )
        })?;
        let machine = checked.task_machine(root).ok_or_else(|| {
            entry_fault(
                checked,
                RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "TaskRuntime root has no Checked Task machine",
                },
            )
        })?;
        validate_pair(core, machine).map_err(|invariant| {
            source_fault(
                checked,
                core,
                RuntimeFaultKind::InvalidCheckedCore { invariant },
            )
        })?;
        if !limits.is_valid() {
            return Err(source_fault(
                checked,
                core,
                RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "TaskRuntime limits must all be non-zero",
                },
            ));
        }
        if arguments.len() != core.signature().parameters().len() {
            return Err(source_fault(
                checked,
                core,
                RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "TaskRuntime argument arity disagrees with Checked Task signature",
                },
            ));
        }
        let mut type_bindings = BTreeMap::new();
        for (argument, expected) in arguments.iter().zip(core.signature().parameters()) {
            if !value_matches_type(argument, *expected, checked, &mut type_bindings) {
                return Err(source_fault(
                    checked,
                    core,
                    RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "TaskRuntime argument type disagrees with Checked Task signature",
                    },
                ));
            }
        }

        let (module, declaration) = task_declaration(checked, root).ok_or_else(|| {
            source_fault(
                checked,
                core,
                RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "TaskRuntime root declaration is absent",
                },
            )
        })?;
        let mut interpreter = Interpreter::new(checked, console);
        let mut environment = Environment::new();
        for (pattern, argument) in declaration.parameters.iter().zip(arguments) {
            let value = Value::from(argument);
            if !interpreter.match_pattern(module, pattern, &value, &mut environment)? {
                return Err(source_fault(
                    checked,
                    core,
                    RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "TaskRuntime checked root argument did not match",
                    },
                ));
            }
        }
        let root_path = TaskPath::root();
        let instance = TaskInstance {
            path: root_path.clone(),
            definition: root.clone(),
            owner: None,
            evaluation: Evaluation::new(module, declaration.body.clone(), environment),
            phase: Phase::Ready,
            pending_resume: None,
            scopes: BTreeMap::new(),
            scope_stack: Vec::new(),
            checkpoint: machine.entry(),
            cleanup: machine.cleanup(),
            cleanup_count: 0,
            faults: BTreeMap::new(),
            type_bindings,
        };
        Ok(Self {
            interpreter,
            tasks: BTreeMap::from([(root_path, instance)]),
            limits,
            steps: 0,
            step_limit_faulted: false,
        })
    }

    #[must_use]
    pub fn ready(&self) -> Vec<TaskPath> {
        self.tasks
            .iter()
            .filter(|(path, task)| self.is_selectable(path, task))
            .map(|(path, _)| path.clone())
            .collect()
    }

    #[must_use]
    pub fn state(&self, path: &TaskPath) -> Option<TaskRuntimeState> {
        self.tasks
            .get(path)
            .and_then(|task| public_state(task).ok())
    }

    #[must_use]
    pub fn root_state(&self) -> TaskRuntimeState {
        self.state(&TaskPath::root())
            .expect("TaskRuntime always retains its root")
    }

    #[must_use]
    pub fn cleanup_count(&self, path: &TaskPath) -> Option<usize> {
        self.tasks.get(path).map(|task| task.cleanup_count)
    }

    #[must_use]
    pub fn faults(&self, path: &TaskPath) -> Option<Vec<(TaskPath, RuntimeFault)>> {
        self.tasks.get(path).map(|task| {
            task.faults
                .iter()
                .map(|(path, fault)| (path.clone(), fault.clone()))
                .collect()
        })
    }

    #[must_use]
    pub fn root_fault(&self) -> Option<RuntimeFault> {
        let root = self.tasks.get(&TaskPath::root())?;
        if !matches!(root.phase, Phase::Faulted) {
            return None;
        }
        let (primary_path, primary) = root.faults.first_key_value()?;
        let related_tasks = root
            .faults
            .keys()
            .filter(|path| *path != primary_path)
            .map(ToString::to_string)
            .collect();
        Some(RuntimeFault {
            kind: RuntimeFaultKind::TaskFaultAggregate {
                primary_task: primary_path.to_string(),
                fault_count: root.faults.len(),
                related_tasks,
            },
            source_name: primary.source_name.clone(),
            span: primary.span,
        })
    }

    pub fn request_cancel(&mut self, path: &TaskPath) -> Result<(), RuntimeFault> {
        if !self.tasks.contains_key(path) {
            return Err(self.driver_fault(path, "unknown_task"));
        }
        self.mark_cancel(path, TaskCancellationCause::Requested);
        Ok(())
    }

    pub fn step(&mut self, path: &TaskPath) -> Result<TaskStep, RuntimeFault> {
        let Some(task) = self.tasks.get(path) else {
            return Err(self.driver_fault(path, "unknown_task"));
        };
        if !self.is_selectable(path, task) {
            return Err(self.driver_fault(path, "task_not_ready"));
        }

        if self.steps >= self.limits.max_steps && !self.step_limit_faulted {
            self.step_limit_faulted = true;
            let fault = self.resource_fault(path, "lifecycle_steps", self.limits.max_steps);
            self.begin_fault(path, fault);
        } else if !self.step_limit_faulted {
            self.steps += 1;
        }

        let mut task = self
            .tasks
            .remove(path)
            .expect("selectability checked before removal");
        let result = match task.phase.clone() {
            Phase::Ready => self.step_ready(&mut task),
            Phase::Joining { scope, value } => self.finish_join(&mut task, scope, value),
            Phase::Returning(value) => self.finish_return(&mut task, value),
            Phase::Cancelling { cause, propagated } => {
                self.step_cancellation(&mut task, cause, propagated)
            }
            Phase::FaultPending { propagated } => self.step_fault(&mut task, propagated),
            Phase::Running
            | Phase::Suspended { .. }
            | Phase::Completed(_)
            | Phase::Cancelled
            | Phase::Faulted => Err(self.driver_fault(path, "task_not_ready")),
        };
        let step = match result {
            Ok(step) => step,
            Err(fault) => {
                self.tasks.insert(task.path.clone(), task);
                return Err(fault);
            }
        };
        let terminal = matches!(
            task.phase,
            Phase::Completed(_) | Phase::Cancelled | Phase::Faulted
        );
        let fault_pending = matches!(task.phase, Phase::FaultPending { .. });
        let task_path = task.path.clone();
        self.tasks.insert(task_path.clone(), task);
        if fault_pending {
            self.notify_owner_fault(&task_path);
        }
        if terminal {
            self.notify_owner(&task_path);
        }
        Ok(step)
    }

    fn step_ready(&mut self, task: &mut TaskInstance) -> Result<TaskStep, RuntimeFault> {
        task.phase = Phase::Running;
        if let Some(value) = task.pending_resume.take() {
            task.evaluation
                .resume_value(value)
                .map_err(|invariant| self.invariant_fault(task, invariant))?;
        }
        match task.evaluation.run(&mut self.interpreter) {
            Ok(EvaluationOutcome::HostEffect) => {
                task.phase = Phase::Ready;
                Ok(step(task, TaskStepKind::HostEffectCompleted))
            }
            Ok(EvaluationOutcome::Complete(_)) => {
                let fault = self.invariant_fault(task, "checked Task completed without return");
                self.begin_fault_instance(task, fault);
                Ok(step(
                    task,
                    TaskStepKind::Faulted {
                        fault_count: task.faults.len(),
                    },
                ))
            }
            Ok(EvaluationOutcome::Task(boundary)) => self.handle_boundary(task, boundary),
            Err(fault) => {
                self.begin_fault_instance(task, fault);
                Ok(step(
                    task,
                    TaskStepKind::Faulted {
                        fault_count: task.faults.len(),
                    },
                ))
            }
        }
    }

    fn handle_boundary(
        &mut self,
        task: &mut TaskInstance,
        boundary: TaskBoundary,
    ) -> Result<TaskStep, RuntimeFault> {
        match boundary {
            TaskBoundary::ScopeEnter { expression, span } => {
                self.open_scope(task, expression, span)
            }
            TaskBoundary::ScopeExit {
                expression,
                span,
                value,
            } => self.close_scope(task, expression, span, value),
            TaskBoundary::Spawn {
                call,
                span,
                arguments,
            } => self.spawn(task, call, span, arguments),
            TaskBoundary::Await {
                continuation,
                span,
                handle,
            } => self.await_child(task, continuation, span, handle),
            TaskBoundary::Return { span, value } => self.return_task(task, span, value),
        }
    }

    fn open_scope(
        &mut self,
        task: &mut TaskInstance,
        expression: ExpressionKey,
        span: Span,
    ) -> Result<TaskStep, RuntimeFault> {
        let core = self.core(task)?;
        let scope = core
            .scopes()
            .iter()
            .find(|scope| scope.expression() == expression)
            .copied()
            .ok_or_else(|| self.invariant_fault(task, "Task scope lacks Checked identity"))?;
        if task.scopes.contains_key(&scope.id())
            || scope.parent() != task.scope_stack.last().copied()
        {
            return Err(
                self.invariant_fault(task, "Task scope ownership disagrees with Checked Core")
            );
        }
        if task.scope_stack.len() >= self.limits.max_scopes {
            let fault = self.resource_fault(&task.path, "runtime_scopes", self.limits.max_scopes);
            self.begin_fault_instance(task, fault);
            return Ok(step(
                task,
                TaskStepKind::Faulted {
                    fault_count: task.faults.len(),
                },
            ));
        }
        task.scopes.insert(
            scope.id(),
            ScopeRuntime {
                state: ScopeState::Open,
                parent: scope.parent(),
                children: BTreeSet::new(),
                handles: BTreeMap::new(),
                span,
            },
        );
        task.scope_stack.push(scope.id());
        task.evaluation
            .enter_scope()
            .map_err(|invariant| self.invariant_fault(task, invariant))?;
        task.phase = Phase::Ready;
        Ok(step(
            task,
            TaskStepKind::ScopeOpened {
                scope: scope.id().get(),
            },
        ))
    }

    fn close_scope(
        &mut self,
        task: &mut TaskInstance,
        expression: ExpressionKey,
        span: Span,
        value: Value,
    ) -> Result<TaskStep, RuntimeFault> {
        let core = self.core(task)?;
        let checked_scope = core
            .scopes()
            .iter()
            .find(|scope| scope.expression() == expression)
            .copied()
            .ok_or_else(|| self.invariant_fault(task, "Task scope exit lacks Checked identity"))?;
        let scope = checked_scope.id();
        if task.scope_stack.last().copied() != Some(scope) {
            return Err(self.invariant_fault(task, "Task scope exit is not nested"));
        }
        let runtime_scope = task.scopes.get(&scope).ok_or_else(|| {
            self.invariant_fault(task, "Task scope exit registry entry is absent")
        })?;
        if runtime_scope.parent != checked_scope.parent()
            || runtime_scope.span != span
            || checked_scope.span() != span
        {
            return Err(self.invariant_fault(task, "Task scope source identity disagrees"));
        }
        self.require_observed_handles(task, scope)?;
        if self.scope_has_live_children(task, scope) {
            task.scopes
                .get_mut(&scope)
                .expect("scope found above")
                .state = ScopeState::Closing;
            task.phase = Phase::Joining { scope, value };
            return Ok(step(task, TaskStepKind::JoinPending { scope: scope.get() }));
        }
        self.finish_scope_close(task, scope, value)
    }

    fn finish_join(
        &mut self,
        task: &mut TaskInstance,
        scope: ScopeId,
        value: Value,
    ) -> Result<TaskStep, RuntimeFault> {
        if self.scope_has_live_children(task, scope) {
            return Err(self.driver_fault(&task.path, "join_not_ready"));
        }
        self.finish_scope_close(task, scope, value)
    }

    fn finish_scope_close(
        &mut self,
        task: &mut TaskInstance,
        scope: ScopeId,
        value: Value,
    ) -> Result<TaskStep, RuntimeFault> {
        if !task.scopes.contains_key(&scope) {
            return Err(self.invariant_fault(task, "Task scope registry entry is absent"));
        }
        let runtime_scope = task.scopes.get_mut(&scope).expect("checked above");
        runtime_scope.state = ScopeState::Closed;
        task.scope_stack.pop();
        task.evaluation
            .resume_value(value)
            .map_err(|invariant| self.invariant_fault(task, invariant))?;
        task.phase = Phase::Ready;
        Ok(step(task, TaskStepKind::ScopeClosed { scope: scope.get() }))
    }

    fn spawn(
        &mut self,
        task: &mut TaskInstance,
        call: ExpressionKey,
        span: Span,
        arguments: Vec<Value>,
    ) -> Result<TaskStep, RuntimeFault> {
        let core = self.core(task)?;
        let spawn = core
            .spawns()
            .iter()
            .find(|spawn| spawn.call() == call)
            .cloned()
            .ok_or_else(|| self.invariant_fault(task, "Task spawn lacks Checked identity"))?;
        let Some(scope) = task.scopes.get(&spawn.scope()) else {
            return Err(self.invariant_fault(task, "Task spawn scope is not open"));
        };
        if scope.state != ScopeState::Open || spawn.parent() != TaskId::new(1) {
            return Err(
                self.invariant_fault(task, "Task spawn ownership disagrees with Checked Core")
            );
        }
        if self.tasks.len().saturating_add(1) >= self.limits.max_tasks {
            let fault = self.resource_fault(&task.path, "runtime_tasks", self.limits.max_tasks);
            self.begin_fault_instance(task, fault);
            return Ok(step(
                task,
                TaskStepKind::Faulted {
                    fault_count: task.faults.len(),
                },
            ));
        }
        let child_path = task
            .path
            .child(spawn.task())
            .ok_or_else(|| self.invariant_fault(task, "Task spawn identity is invalid"))?;
        if self.tasks.contains_key(&child_path) {
            return Err(self.invariant_fault(task, "Task runtime path is duplicated"));
        }
        let child = self.build_child(
            &child_path,
            spawn.target(),
            arguments,
            (task.path.clone(), spawn.scope()),
            spawn.cleanup(),
            span,
        )?;
        let scope = task
            .scopes
            .get_mut(&spawn.scope())
            .expect("scope checked above");
        scope.children.insert(child_path.clone());
        scope.handles.insert(child_path.clone(), false);
        task.evaluation
            .resume_value(Value::TaskHandle(child_path.clone()))
            .map_err(|invariant| self.invariant_fault(task, invariant))?;
        task.phase = Phase::Ready;
        self.tasks.insert(child_path.clone(), child);
        Ok(step(
            task,
            TaskStepKind::ChildRegistered { child: child_path },
        ))
    }

    fn await_child(
        &mut self,
        task: &mut TaskInstance,
        continuation: ExpressionKey,
        span: Span,
        handle: Value,
    ) -> Result<TaskStep, RuntimeFault> {
        let Value::TaskHandle(child_path) = handle else {
            return Err(self.invariant_fault(task, "Task await did not receive a runtime handle"));
        };
        let core = self.core(task)?;
        let suspension = core
            .suspensions()
            .iter()
            .find(|suspension| suspension.continuation() == continuation)
            .cloned()
            .ok_or_else(|| self.invariant_fault(task, "Task await lacks Checked suspension"))?;
        if child_path.lexical_task() != suspension.awaited_task() {
            return Err(
                self.invariant_fault(task, "Task await handle disagrees with Checked suspension")
            );
        }
        if !task.scopes.contains_key(&suspension.scope()) {
            return Err(self.invariant_fault(task, "Task await scope is absent"));
        }
        if !task.scopes[&suspension.scope()]
            .handles
            .contains_key(&child_path)
        {
            return Err(self.invariant_fault(task, "Task await handle is not scope-owned"));
        }
        let consumed = task
            .scopes
            .get_mut(&suspension.scope())
            .expect("checked above")
            .handles
            .get_mut(&child_path)
            .expect("checked above");
        if *consumed {
            return Err(self.invariant_fault(task, "Task await handle was consumed twice"));
        }
        *consumed = true;
        self.advance_checkpoint(task, continuation, span)?;
        let child = self
            .tasks
            .get(&child_path)
            .ok_or_else(|| self.invariant_fault(task, "Task await child is absent"))?;
        match &child.phase {
            Phase::Completed(value) => {
                task.pending_resume = Some(value.clone());
                task.phase = Phase::Ready;
                Ok(step(task, TaskStepKind::AwaitReady { child: child_path }))
            }
            Phase::Cancelled => {
                task.phase = Phase::Cancelling {
                    cause: TaskCancellationCause::Ancestor,
                    propagated: false,
                };
                Ok(step(task, TaskStepKind::CancellationPropagated))
            }
            Phase::Faulted => {
                self.merge_faults(task, &child.faults);
                task.phase = Phase::FaultPending { propagated: false };
                Ok(step(
                    task,
                    TaskStepKind::Faulted {
                        fault_count: task.faults.len(),
                    },
                ))
            }
            _ => {
                task.phase = Phase::Suspended {
                    awaited: child_path.clone(),
                };
                Ok(step(task, TaskStepKind::Suspended { child: child_path }))
            }
        }
    }

    fn return_task(
        &mut self,
        task: &mut TaskInstance,
        _span: Span,
        value: Value,
    ) -> Result<TaskStep, RuntimeFault> {
        self.advance_to_cleanup(task, CheckedTaskMachineStateKind::ReturnCleanup)?;
        for scope in task.scope_stack.iter().copied() {
            self.require_observed_handles(task, scope)?;
        }
        if self.has_live_descendants(&task.path) {
            task.phase = Phase::Returning(value);
            let scope = task
                .scope_stack
                .last()
                .copied()
                .unwrap_or_else(|| self.core(task).expect("core exists").root_scope());
            return Ok(step(task, TaskStepKind::JoinPending { scope: scope.get() }));
        }
        self.finish_return(task, value)
    }

    fn finish_return(
        &mut self,
        task: &mut TaskInstance,
        value: Value,
    ) -> Result<TaskStep, RuntimeFault> {
        if self.has_live_descendants(&task.path) {
            return Err(self.driver_fault(&task.path, "return_join_not_ready"));
        }
        self.close_all_scopes(task, true)?;
        self.cleanup(task);
        let task_value = TaskValue::try_from(value.clone())
            .map_err(|invariant| self.invariant_fault(task, invariant))?;
        let expected = self.core(task)?.signature().result();
        if !value_matches_type(
            &task_value,
            expected,
            self.interpreter.checked,
            &mut task.type_bindings,
        ) {
            return Err(
                self.invariant_fault(task, "Task return value disagrees with Checked signature")
            );
        }
        self.advance_cleanup_terminal(task, CheckedTaskMachineStateKind::Completed)?;
        task.phase = Phase::Completed(value);
        Ok(step(task, TaskStepKind::Completed))
    }

    fn step_cancellation(
        &mut self,
        task: &mut TaskInstance,
        cause: TaskCancellationCause,
        propagated: bool,
    ) -> Result<TaskStep, RuntimeFault> {
        if !propagated {
            self.cancel_descendants(&task.path);
            task.phase = Phase::Cancelling {
                cause,
                propagated: true,
            };
            if self.has_live_descendants(&task.path) {
                return Ok(step(task, TaskStepKind::CancellationPropagated));
            }
        }
        if self.has_live_descendants(&task.path) {
            return Err(self.driver_fault(&task.path, "cancellation_drain_not_ready"));
        }
        self.advance_to_cleanup(task, CheckedTaskMachineStateKind::CancelCleanup)?;
        self.close_all_scopes(task, false)?;
        self.cleanup(task);
        self.advance_cleanup_terminal(task, CheckedTaskMachineStateKind::Cancelled)?;
        task.phase = Phase::Cancelled;
        Ok(step(task, TaskStepKind::Cancelled))
    }

    fn step_fault(
        &mut self,
        task: &mut TaskInstance,
        propagated: bool,
    ) -> Result<TaskStep, RuntimeFault> {
        if !propagated {
            self.cancel_descendants(&task.path);
            task.phase = Phase::FaultPending { propagated: true };
            if self.has_live_descendants(&task.path) {
                return Ok(step(
                    task,
                    TaskStepKind::Faulted {
                        fault_count: task.faults.len(),
                    },
                ));
            }
        }
        if self.has_live_descendants(&task.path) {
            return Err(self.driver_fault(&task.path, "fault_drain_not_ready"));
        }
        self.advance_to_cleanup(task, CheckedTaskMachineStateKind::FaultCleanup)?;
        self.close_all_scopes(task, false)?;
        self.cleanup(task);
        self.advance_cleanup_terminal(task, CheckedTaskMachineStateKind::Faulted)?;
        task.phase = Phase::Faulted;
        Ok(step(
            task,
            TaskStepKind::Faulted {
                fault_count: task.faults.len(),
            },
        ))
    }

    fn begin_fault(&mut self, path: &TaskPath, fault: RuntimeFault) {
        if let Some(mut task) = self.tasks.remove(path) {
            self.begin_fault_instance(&mut task, fault);
            self.tasks.insert(path.clone(), task);
        }
    }

    fn begin_fault_instance(&self, task: &mut TaskInstance, fault: RuntimeFault) {
        if task.faults.len() < self.limits.max_faults || task.faults.contains_key(&task.path) {
            task.faults.insert(task.path.clone(), fault);
        } else {
            let limit = self.resource_fault(&task.path, "retained_faults", self.limits.max_faults);
            task.faults.insert(task.path.clone(), limit);
            while task.faults.len() > self.limits.max_faults {
                let Some(path) = task.faults.keys().next_back().cloned() else {
                    break;
                };
                if path == task.path && task.faults.len() > 1 {
                    let Some(other) = task.faults.keys().rev().nth(1).cloned() else {
                        break;
                    };
                    task.faults.remove(&other);
                } else {
                    task.faults.remove(&path);
                }
            }
        }
        task.phase = Phase::FaultPending { propagated: false };
    }

    fn notify_owner(&mut self, child_path: &TaskPath) {
        let Some(child) = self.tasks.get(child_path) else {
            return;
        };
        let Some((owner_path, owner_scope)) = child.owner.clone() else {
            return;
        };
        let child_phase = child.phase.clone();
        let child_faults = child.faults.clone();
        let child_value = match &child.phase {
            Phase::Completed(value) => Some(value.clone()),
            _ => None,
        };
        let Some(owner) = self.tasks.get_mut(&owner_path) else {
            return;
        };
        match child_phase {
            Phase::Completed(_) => {
                if matches!(
                    &owner.phase,
                    Phase::Suspended { awaited } if awaited == child_path
                ) {
                    owner.pending_resume = child_value;
                    owner.phase = Phase::Ready;
                }
            }
            Phase::Cancelled => {
                if matches!(owner.phase, Phase::Suspended { .. }) {
                    owner.phase = Phase::Cancelling {
                        cause: TaskCancellationCause::Ancestor,
                        propagated: false,
                    };
                }
            }
            Phase::Faulted => {
                let _ = owner;
                self.publish_child_fault(&owner_path, owner_scope, child_faults);
            }
            _ => {}
        }
    }

    fn notify_owner_fault(&mut self, child_path: &TaskPath) {
        let Some(child) = self.tasks.get(child_path) else {
            return;
        };
        let Some((owner_path, owner_scope)) = child.owner.clone() else {
            return;
        };
        self.publish_child_fault(&owner_path, owner_scope, child.faults.clone());
    }

    fn publish_child_fault(
        &mut self,
        owner_path: &TaskPath,
        owner_scope: ScopeId,
        child_faults: BTreeMap<TaskPath, RuntimeFault>,
    ) {
        let overflow = self.tasks.get(owner_path).is_some_and(|owner| {
            owner
                .faults
                .keys()
                .chain(child_faults.keys())
                .collect::<BTreeSet<_>>()
                .len()
                > self.limits.max_faults
        });
        let overflow_fault = overflow
            .then(|| self.resource_fault(owner_path, "retained_faults", self.limits.max_faults));
        let Some(owner) = self.tasks.get_mut(owner_path) else {
            return;
        };
        if let Some(fault) = overflow_fault {
            owner.faults.clear();
            owner.faults.insert(owner_path.clone(), fault);
        } else {
            owner.faults.extend(child_faults);
        }
        if let Some(scope) = owner.scopes.get_mut(&owner_scope) {
            scope.state = ScopeState::Closing;
        }
        if !matches!(owner.phase, Phase::Faulted | Phase::Completed(_)) {
            owner.phase = Phase::FaultPending { propagated: false };
        }
    }

    fn merge_faults(&self, task: &mut TaskInstance, incoming: &BTreeMap<TaskPath, RuntimeFault>) {
        let distinct = task
            .faults
            .keys()
            .chain(incoming.keys())
            .collect::<BTreeSet<_>>()
            .len();
        if distinct <= self.limits.max_faults {
            task.faults.extend(incoming.clone());
        } else {
            let fault = self.resource_fault(&task.path, "retained_faults", self.limits.max_faults);
            task.faults.clear();
            task.faults.insert(task.path.clone(), fault);
        }
    }

    fn mark_cancel(&mut self, path: &TaskPath, cause: TaskCancellationCause) {
        let descendants = self.descendants(path);
        if let Some(task) = self.tasks.get_mut(path) {
            if !matches!(
                task.phase,
                Phase::Completed(_)
                    | Phase::Cancelled
                    | Phase::Faulted
                    | Phase::FaultPending { .. }
            ) {
                task.phase = Phase::Cancelling {
                    cause,
                    propagated: false,
                };
            }
        }
        for descendant in descendants {
            if let Some(task) = self.tasks.get_mut(&descendant) {
                if !matches!(
                    task.phase,
                    Phase::Completed(_)
                        | Phase::Cancelled
                        | Phase::Faulted
                        | Phase::FaultPending { .. }
                ) {
                    task.phase = Phase::Cancelling {
                        cause: TaskCancellationCause::Ancestor,
                        propagated: false,
                    };
                }
            }
        }
    }

    fn cancel_descendants(&mut self, path: &TaskPath) {
        for descendant in self.descendants(path) {
            if let Some(task) = self.tasks.get_mut(&descendant) {
                if !matches!(
                    task.phase,
                    Phase::Completed(_)
                        | Phase::Cancelled
                        | Phase::Faulted
                        | Phase::FaultPending { .. }
                ) {
                    task.phase = Phase::Cancelling {
                        cause: TaskCancellationCause::Ancestor,
                        propagated: false,
                    };
                }
            }
        }
    }

    fn descendants(&self, path: &TaskPath) -> Vec<TaskPath> {
        self.tasks
            .keys()
            .filter(|candidate| {
                candidate.0.len() > path.0.len() && candidate.0[..path.0.len()] == *path.0
            })
            .cloned()
            .collect()
    }

    fn has_live_descendants(&self, path: &TaskPath) -> bool {
        self.descendants(path).iter().any(|path| {
            self.tasks.get(path).is_some_and(|task| {
                !matches!(
                    task.phase,
                    Phase::Completed(_) | Phase::Cancelled | Phase::Faulted
                )
            })
        })
    }

    fn scope_has_live_children(&self, task: &TaskInstance, scope: ScopeId) -> bool {
        task.scopes.get(&scope).is_some_and(|scope| {
            scope.children.iter().any(|path| {
                self.tasks.get(path).is_some_and(|child| {
                    !matches!(
                        child.phase,
                        Phase::Completed(_) | Phase::Cancelled | Phase::Faulted
                    )
                })
            })
        })
    }

    fn require_observed_handles(
        &self,
        task: &TaskInstance,
        scope: ScopeId,
    ) -> Result<(), RuntimeFault> {
        let scope = task
            .scopes
            .get(&scope)
            .ok_or_else(|| self.invariant_fault(task, "Task scope is absent"))?;
        if scope.handles.values().any(|consumed| !consumed) {
            return Err(
                self.invariant_fault(task, "normal scope close has an unobserved Task handle")
            );
        }
        Ok(())
    }

    fn close_all_scopes(
        &mut self,
        task: &mut TaskInstance,
        require_observed: bool,
    ) -> Result<(), RuntimeFault> {
        while let Some(scope_id) = task.scope_stack.pop() {
            if require_observed {
                self.require_observed_handles(task, scope_id)?;
            }
            if !task.scopes.contains_key(&scope_id) {
                return Err(self.invariant_fault(task, "Task cleanup scope is absent"));
            }
            let scope = task.scopes.get_mut(&scope_id).expect("checked above");
            scope
                .handles
                .values_mut()
                .for_each(|consumed| *consumed = true);
            scope.state = ScopeState::Closed;
        }
        Ok(())
    }

    fn cleanup(&self, task: &mut TaskInstance) {
        let _ = task.cleanup;
        task.cleanup_count += 1;
    }

    fn advance_checkpoint(
        &self,
        task: &mut TaskInstance,
        continuation: ExpressionKey,
        _span: Span,
    ) -> Result<(), RuntimeFault> {
        let machine = self.machine(task)?;
        let state = machine
            .states()
            .iter()
            .find(|state| {
                matches!(
                    state.kind(),
                    CheckedTaskMachineStateKind::Suspend { continuation: key, .. }
                        if key == continuation
                )
            })
            .map(|state| state.id())
            .ok_or_else(|| self.invariant_fault(task, "Task suspension state is absent"))?;
        require_normal_edge(machine, task.checkpoint, state)
            .map_err(|invariant| self.invariant_fault(task, invariant))?;
        task.checkpoint = state;
        Ok(())
    }

    fn advance_to_cleanup(
        &self,
        task: &mut TaskInstance,
        kind: CheckedTaskMachineStateKind,
    ) -> Result<(), RuntimeFault> {
        let machine = self.machine(task)?;
        let target = state_with_kind(machine, kind)
            .ok_or_else(|| self.invariant_fault(task, "Task cleanup state is absent"))?;
        let edge_kind = match kind {
            CheckedTaskMachineStateKind::ReturnCleanup => None,
            CheckedTaskMachineStateKind::CancelCleanup => Some(CheckedTaskMachineEdgeKind::Cancel),
            CheckedTaskMachineStateKind::FaultCleanup => Some(CheckedTaskMachineEdgeKind::Fault),
            _ => return Err(self.invariant_fault(task, "invalid Task cleanup transition")),
        };
        let valid = machine.edges().iter().any(|edge| {
            edge.from() == task.checkpoint
                && edge.to() == target
                && edge_kind.map_or_else(
                    || {
                        matches!(
                            edge.kind(),
                            CheckedTaskMachineEdgeKind::Continue
                                | CheckedTaskMachineEdgeKind::Resume
                        )
                    },
                    |kind| edge.kind() == kind,
                )
        });
        if !valid {
            return Err(self.invariant_fault(task, "Task cleanup edge is absent"));
        }
        task.checkpoint = target;
        Ok(())
    }

    fn advance_cleanup_terminal(
        &self,
        task: &mut TaskInstance,
        kind: CheckedTaskMachineStateKind,
    ) -> Result<(), RuntimeFault> {
        let machine = self.machine(task)?;
        let target = state_with_kind(machine, kind)
            .ok_or_else(|| self.invariant_fault(task, "Task terminal state is absent"))?;
        if !machine.edges().iter().any(|edge| {
            edge.from() == task.checkpoint
                && edge.to() == target
                && edge.kind() == CheckedTaskMachineEdgeKind::Cleanup
        }) {
            return Err(self.invariant_fault(task, "Task terminal cleanup edge is absent"));
        }
        task.checkpoint = target;
        Ok(())
    }

    fn build_child(
        &mut self,
        path: &TaskPath,
        definition: &DefinitionId,
        arguments: Vec<Value>,
        owner: (TaskPath, ScopeId),
        cleanup: CleanupRegionId,
        span: Span,
    ) -> Result<TaskInstance, RuntimeFault> {
        let checked = self.interpreter.checked;
        let core = checked.task_core(definition).ok_or_else(|| {
            self.invariant_fault_at(path, span, "spawn target has no Checked Task Core")
        })?;
        let machine = checked.task_machine(definition).ok_or_else(|| {
            self.invariant_fault_at(path, span, "spawn target has no Checked Task machine")
        })?;
        validate_pair(core, machine)
            .map_err(|invariant| self.invariant_fault_at(path, span, invariant))?;
        if arguments.len() != core.signature().parameters().len() {
            return Err(self.invariant_fault_at(path, span, "spawn argument arity mismatch"));
        }
        let (module, declaration) = task_declaration(checked, definition).ok_or_else(|| {
            self.invariant_fault_at(path, span, "spawn Task declaration is absent")
        })?;
        let mut environment = Environment::new();
        let mut type_bindings = BTreeMap::new();
        for ((pattern, value), expected) in declaration
            .parameters
            .iter()
            .zip(arguments)
            .zip(core.signature().parameters())
        {
            let public = TaskValue::try_from(value.clone())
                .map_err(|invariant| self.invariant_fault_at(path, span, invariant))?;
            if !value_matches_type(&public, *expected, checked, &mut type_bindings)
                || !self
                    .interpreter
                    .match_pattern(module, pattern, &value, &mut environment)?
            {
                return Err(self.invariant_fault_at(path, span, "spawn checked argument mismatch"));
            }
        }
        Ok(TaskInstance {
            path: path.clone(),
            definition: definition.clone(),
            owner: Some(owner),
            evaluation: Evaluation::new(module, declaration.body.clone(), environment),
            phase: Phase::Ready,
            pending_resume: None,
            scopes: BTreeMap::new(),
            scope_stack: Vec::new(),
            checkpoint: machine.entry(),
            cleanup,
            cleanup_count: 0,
            faults: BTreeMap::new(),
            type_bindings,
        })
    }

    fn core<'a>(&'a self, task: &TaskInstance) -> Result<&'a CheckedTaskCore, RuntimeFault> {
        self.interpreter
            .checked
            .task_core(&task.definition)
            .ok_or_else(|| self.invariant_fault(task, "runtime Task Core disappeared"))
    }

    fn machine<'a>(&'a self, task: &TaskInstance) -> Result<&'a CheckedTaskMachine, RuntimeFault> {
        self.interpreter
            .checked
            .task_machine(&task.definition)
            .ok_or_else(|| self.invariant_fault(task, "runtime Task machine disappeared"))
    }

    fn is_selectable(&self, path: &TaskPath, task: &TaskInstance) -> bool {
        match &task.phase {
            Phase::Ready => true,
            Phase::Joining { .. } | Phase::Returning(_) => !self.has_live_descendants(path),
            Phase::Cancelling { propagated, .. } | Phase::FaultPending { propagated } => {
                !*propagated || !self.has_live_descendants(path)
            }
            Phase::Running
            | Phase::Suspended { .. }
            | Phase::Completed(_)
            | Phase::Cancelled
            | Phase::Faulted => false,
        }
    }

    fn driver_fault(&self, path: &TaskPath, reason: &'static str) -> RuntimeFault {
        let root = self
            .interpreter
            .checked
            .task_cores()
            .values()
            .next()
            .expect("TaskRuntime has a checked root");
        source_fault(
            self.interpreter.checked,
            root,
            RuntimeFaultKind::TaskDriver {
                reason,
                task: path.to_string(),
            },
        )
    }

    fn invariant_fault(&self, task: &TaskInstance, invariant: &'static str) -> RuntimeFault {
        self.invariant_fault_at(&task.path, self.core_span(task), invariant)
    }

    fn invariant_fault_at(
        &self,
        path: &TaskPath,
        span: Span,
        invariant: &'static str,
    ) -> RuntimeFault {
        RuntimeFault {
            kind: RuntimeFaultKind::InvalidCheckedCore { invariant },
            source_name: self
                .tasks
                .get(path)
                .and_then(|task| task_source(self.interpreter.checked, &task.definition))
                .unwrap_or_else(|| {
                    self.interpreter
                        .module_source(self.interpreter.checked.typed().resolved().entry())
                }),
            span,
        }
    }

    fn resource_fault(
        &self,
        path: &TaskPath,
        resource: &'static str,
        limit: usize,
    ) -> RuntimeFault {
        let (source_name, span) = self.tasks.get(path).map_or_else(
            || {
                let module = self.interpreter.checked.typed().resolved().entry_module();
                (module.hir.source_name.clone(), module.hir.span)
            },
            |task| {
                (
                    task_source(self.interpreter.checked, &task.definition).unwrap_or_default(),
                    self.core_span(task),
                )
            },
        );
        RuntimeFault {
            kind: RuntimeFaultKind::TaskResourceLimit { resource, limit },
            source_name,
            span,
        }
    }

    fn core_span(&self, task: &TaskInstance) -> Span {
        self.interpreter
            .checked
            .task_core(&task.definition)
            .map_or_else(
                || {
                    self.interpreter
                        .checked
                        .typed()
                        .resolved()
                        .entry_module()
                        .hir
                        .span
                },
                CheckedTaskCore::source_span,
            )
    }
}

fn step(task: &TaskInstance, kind: TaskStepKind) -> TaskStep {
    TaskStep {
        task: task.path.clone(),
        kind,
    }
}

fn public_state(task: &TaskInstance) -> Result<TaskRuntimeState, &'static str> {
    match &task.phase {
        Phase::Ready | Phase::Returning(_) => Ok(TaskRuntimeState::Ready),
        Phase::Running => Ok(TaskRuntimeState::Running),
        Phase::Suspended { awaited } => Ok(TaskRuntimeState::Suspended {
            awaited: awaited.clone(),
        }),
        Phase::Joining { scope, .. } => Ok(TaskRuntimeState::Joining { scope: scope.get() }),
        Phase::Cancelling { cause, .. } => Ok(TaskRuntimeState::Cancelling { cause: *cause }),
        Phase::FaultPending { .. } => Ok(TaskRuntimeState::Cleaning { reason: "Fault" }),
        Phase::Completed(value) => Ok(TaskRuntimeState::Completed(TaskValue::try_from(
            value.clone(),
        )?)),
        Phase::Cancelled => Ok(TaskRuntimeState::Cancelled),
        Phase::Faulted => Ok(TaskRuntimeState::Faulted {
            fault_count: task.faults.len(),
        }),
    }
}

fn validate_pair(core: &CheckedTaskCore, machine: &CheckedTaskMachine) -> Result<(), &'static str> {
    if machine.version() != CHECKED_TASK_MACHINE_VERSION {
        return Err("Task machine version is unsupported");
    }
    if machine.definition() != core.definition()
        || machine.root_scope() != core.root_scope()
        || machine.source_span() != core.source_span()
        || machine.projection().entry() != machine.entry()
    {
        return Err("Checked Task Core and machine disagree");
    }
    Ok(())
}

fn task_declaration<'a>(
    checked: &'a CheckedProgram,
    definition: &DefinitionId,
) -> Option<(ModuleId, &'a ling_hir::TaskDeclaration)> {
    let info = checked.typed().resolved().definition(definition)?;
    if info.kind != DefinitionKind::Task {
        return None;
    }
    let DefinitionOrigin::User { module } = info.origin else {
        return None;
    };
    let resolved_module = checked.typed().resolved().module(module)?;
    resolved_module
        .hir
        .tasks
        .iter()
        .find(|task| task.name.normalized == info.name)
        .map(|task| (module, task))
}

fn task_source(checked: &CheckedProgram, definition: &DefinitionId) -> Option<String> {
    checked
        .typed()
        .resolved()
        .definition(definition)
        .and_then(|info| info.source_name.clone())
}

fn entry_fault(checked: &CheckedProgram, kind: RuntimeFaultKind) -> RuntimeFault {
    let module = checked.typed().resolved().entry_module();
    RuntimeFault {
        kind,
        source_name: module.hir.source_name.clone(),
        span: module.hir.span,
    }
}

fn source_fault(
    checked: &CheckedProgram,
    core: &CheckedTaskCore,
    kind: RuntimeFaultKind,
) -> RuntimeFault {
    RuntimeFault {
        kind,
        source_name: task_source(checked, core.definition()).unwrap_or_else(|| "<task>".to_owned()),
        span: core.source_span(),
    }
}

fn value_matches_type(
    value: &TaskValue,
    expected: TypeId,
    checked: &CheckedProgram,
    bindings: &mut BTreeMap<u32, RuntimeTypeShape>,
) -> bool {
    if !task_value_is_well_formed(value, checked) {
        return false;
    }
    value_matches_type_with_substitution(value, expected, checked, bindings, &BTreeMap::new())
}

fn value_matches_type_with_substitution(
    value: &TaskValue,
    expected: TypeId,
    checked: &CheckedProgram,
    bindings: &mut BTreeMap<u32, RuntimeTypeShape>,
    substitutions: &BTreeMap<u32, TypeId>,
) -> bool {
    match (value, checked.typed().arena().get(expected)) {
        (TaskValue::Unit, Type::Unit)
        | (TaskValue::Bool(_), Type::Bool)
        | (TaskValue::Int(_), Type::Int)
        | (TaskValue::Float64(_), Type::Float64)
        | (TaskValue::Text(_), Type::Text) => true,
        (TaskValue::Tuple(values), Type::Tuple(types)) => {
            values.len() == types.len()
                && values.iter().zip(types).all(|(value, expected)| {
                    value_matches_type_with_substitution(
                        value,
                        *expected,
                        checked,
                        bindings,
                        substitutions,
                    )
                })
        }
        (TaskValue::List(values), Type::List(expected)) => values.iter().all(|value| {
            value_matches_type_with_substitution(value, *expected, checked, bindings, substitutions)
        }),
        (
            TaskValue::Record { definition, fields },
            Type::NominalRecord {
                definition: expected,
                arguments,
            },
        ) => {
            definition == expected
                && record_matches_type(
                    definition,
                    fields,
                    arguments,
                    checked,
                    bindings,
                    substitutions,
                )
        }
        (
            TaskValue::Variant {
                definition,
                case,
                payload,
            },
            Type::NominalVariant {
                definition: expected,
                arguments,
            },
        ) => {
            definition == expected
                && variant_matches_type(
                    definition,
                    case,
                    payload.as_deref(),
                    arguments,
                    checked,
                    bindings,
                    substitutions,
                )
        }
        (_, Type::Variable(variable)) => {
            if let Some(substituted) = substitutions.get(variable).filter(|substituted| {
                !matches!(
                    checked.typed().arena().get(**substituted),
                    Type::Variable(substituted_variable) if substituted_variable == variable
                )
            }) {
                return value_matches_type_with_substitution(
                    value,
                    *substituted,
                    checked,
                    bindings,
                    substitutions,
                );
            }
            let Some(actual) = runtime_type_shape(value) else {
                return false;
            };
            match bindings.get_mut(variable) {
                Some(bound) => unify_runtime_type_shape(bound, actual),
                None => {
                    bindings.insert(*variable, actual);
                    true
                }
            }
        }
        _ => false,
    }
}

fn task_value_is_well_formed(value: &TaskValue, checked: &CheckedProgram) -> bool {
    let mut bindings = BTreeMap::new();
    match value {
        TaskValue::Unit
        | TaskValue::Bool(_)
        | TaskValue::Int(_)
        | TaskValue::Float64(_)
        | TaskValue::Text(_) => true,
        TaskValue::Tuple(values) | TaskValue::List(values) => {
            runtime_type_shape(value).is_some()
                && values
                    .iter()
                    .all(|value| task_value_is_well_formed(value, checked))
        }
        TaskValue::Record { definition, fields } => record_matches_type(
            definition,
            fields,
            &[],
            checked,
            &mut bindings,
            &BTreeMap::new(),
        ),
        TaskValue::Variant {
            definition,
            case,
            payload,
        } => variant_matches_type(
            definition,
            case,
            payload.as_deref(),
            &[],
            checked,
            &mut bindings,
            &BTreeMap::new(),
        ),
    }
}

fn record_matches_type(
    definition: &DefinitionId,
    fields: &BTreeMap<String, TaskValue>,
    arguments: &[TypeId],
    checked: &CheckedProgram,
    bindings: &mut BTreeMap<u32, RuntimeTypeShape>,
    outer_substitutions: &BTreeMap<u32, TypeId>,
) -> bool {
    let Some(info) = checked.typed().records().get(definition) else {
        return false;
    };
    if fields.len() != info.fields.len() {
        return false;
    }
    let substitutions = nominal_substitutions(definition, arguments, checked, outer_substitutions);
    info.fields.iter().all(|field| {
        fields.get(&field.name).is_some_and(|value| {
            value_matches_type_with_substitution(
                value,
                field.field_type,
                checked,
                bindings,
                &substitutions,
            )
        })
    })
}

fn variant_matches_type(
    definition: &DefinitionId,
    case: &str,
    payload: Option<&TaskValue>,
    arguments: &[TypeId],
    checked: &CheckedProgram,
    bindings: &mut BTreeMap<u32, RuntimeTypeShape>,
    outer_substitutions: &BTreeMap<u32, TypeId>,
) -> bool {
    let Some(info) = checked.typed().variants().get(definition) else {
        return false;
    };
    let Some(case) = info.cases.iter().find(|candidate| candidate.name == case) else {
        return false;
    };
    let substitutions = nominal_substitutions(definition, arguments, checked, outer_substitutions);
    match (payload, case.payload) {
        (None, None) => true,
        (Some(value), Some(expected)) => {
            value_matches_type_with_substitution(value, expected, checked, bindings, &substitutions)
        }
        _ => false,
    }
}

fn nominal_substitutions(
    definition: &DefinitionId,
    arguments: &[TypeId],
    checked: &CheckedProgram,
    outer: &BTreeMap<u32, TypeId>,
) -> BTreeMap<u32, TypeId> {
    let mut output = outer.clone();
    let typed = checked.typed();
    if let Some(info) = typed.records().get(definition) {
        let mut variables = Vec::new();
        for field in &info.fields {
            collect_type_variables(field.field_type, checked, &mut variables);
        }
        for (variable, actual) in variables.into_iter().zip(arguments) {
            output.entry(variable).or_insert(*actual);
        }
    } else if let Some(info) = typed.variants().get(definition) {
        if let Some(case) = info.cases.first() {
            if let Some(constructor) = typed.definition_type(&case.definition) {
                let result = match typed.arena().get(constructor) {
                    Type::Function { result, .. } => *result,
                    _ => constructor,
                };
                if let Type::NominalVariant {
                    arguments: generic, ..
                } = typed.arena().get(result)
                {
                    for (generic, actual) in generic.iter().zip(arguments) {
                        collect_type_substitutions(*generic, *actual, checked, &mut output);
                    }
                }
            }
        }
    }
    output
}

fn collect_type_variables(expected: TypeId, checked: &CheckedProgram, output: &mut Vec<u32>) {
    match checked.typed().arena().get(expected) {
        Type::Variable(variable) if !output.contains(variable) => output.push(*variable),
        Type::Tuple(elements) => {
            for element in elements {
                collect_type_variables(*element, checked, output);
            }
        }
        Type::List(element) => collect_type_variables(*element, checked, output),
        Type::NominalRecord { arguments, .. } | Type::NominalVariant { arguments, .. } => {
            for argument in arguments {
                collect_type_variables(*argument, checked, output);
            }
        }
        _ => {}
    }
}

fn collect_type_substitutions(
    generic: TypeId,
    actual: TypeId,
    checked: &CheckedProgram,
    output: &mut BTreeMap<u32, TypeId>,
) {
    match (
        checked.typed().arena().get(generic),
        checked.typed().arena().get(actual),
    ) {
        (Type::Variable(variable), _) => {
            output.insert(*variable, actual);
        }
        (Type::Tuple(generic), Type::Tuple(actual)) => {
            for (generic, actual) in generic.iter().zip(actual) {
                collect_type_substitutions(*generic, *actual, checked, output);
            }
        }
        (Type::List(generic), Type::List(actual)) => {
            collect_type_substitutions(*generic, *actual, checked, output);
        }
        (
            Type::NominalRecord {
                arguments: generic, ..
            },
            Type::NominalRecord {
                arguments: actual, ..
            },
        )
        | (
            Type::NominalVariant {
                arguments: generic, ..
            },
            Type::NominalVariant {
                arguments: actual, ..
            },
        ) => {
            for (generic, actual) in generic.iter().zip(actual) {
                collect_type_substitutions(*generic, *actual, checked, output);
            }
        }
        _ => {}
    }
}

fn runtime_type_shape(value: &TaskValue) -> Option<RuntimeTypeShape> {
    Some(match value {
        TaskValue::Unit => RuntimeTypeShape::Unit,
        TaskValue::Bool(_) => RuntimeTypeShape::Bool,
        TaskValue::Int(_) => RuntimeTypeShape::Int,
        TaskValue::Float64(_) => RuntimeTypeShape::Float64,
        TaskValue::Text(_) => RuntimeTypeShape::Text,
        TaskValue::Tuple(values) => RuntimeTypeShape::Tuple(
            values
                .iter()
                .map(runtime_type_shape)
                .collect::<Option<Vec<_>>>()?,
        ),
        TaskValue::List(values) => {
            let mut element = RuntimeTypeShape::Unknown;
            for value in values {
                if !unify_runtime_type_shape(&mut element, runtime_type_shape(value)?) {
                    return None;
                }
            }
            RuntimeTypeShape::List(Box::new(element))
        }
        TaskValue::Record { definition, .. } => RuntimeTypeShape::Record(definition.clone()),
        TaskValue::Variant { definition, .. } => RuntimeTypeShape::Variant(definition.clone()),
    })
}

fn unify_runtime_type_shape(bound: &mut RuntimeTypeShape, actual: RuntimeTypeShape) -> bool {
    match (&mut *bound, actual) {
        (RuntimeTypeShape::Unknown, actual) => {
            *bound = actual;
            true
        }
        (_, RuntimeTypeShape::Unknown) => true,
        (RuntimeTypeShape::Tuple(bound), RuntimeTypeShape::Tuple(actual))
            if bound.len() == actual.len() =>
        {
            bound
                .iter_mut()
                .zip(actual)
                .all(|(bound, actual)| unify_runtime_type_shape(bound, actual))
        }
        (RuntimeTypeShape::List(bound), RuntimeTypeShape::List(actual)) => {
            unify_runtime_type_shape(bound, *actual)
        }
        (bound, actual) => *bound == actual,
    }
}

fn state_with_kind(
    machine: &CheckedTaskMachine,
    expected: CheckedTaskMachineStateKind,
) -> Option<StateId> {
    machine
        .states()
        .iter()
        .find(|state| std::mem::discriminant(&state.kind()) == std::mem::discriminant(&expected))
        .map(|state| state.id())
}

fn require_normal_edge(
    machine: &CheckedTaskMachine,
    from: StateId,
    to: StateId,
) -> Result<(), &'static str> {
    if machine.edges().iter().any(|edge| {
        edge.from() == from
            && edge.to() == to
            && matches!(
                edge.kind(),
                CheckedTaskMachineEdgeKind::Continue | CheckedTaskMachineEdgeKind::Resume
            )
    }) {
        Ok(())
    } else {
        Err("executed Task path lacks a Checked machine edge")
    }
}
