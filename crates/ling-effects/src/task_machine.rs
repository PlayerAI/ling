//! Deterministic, non-executable lowering from Checked Task Core.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use ling_concurrency::{
    CancellationTokenId, CleanupRegionId, LocalId, ScopeId, StateEdgeKind, StateId,
    StateMachineModel, StateNode, StateNodeSpec, StateTransition, StateTransitionSpec,
    SuspensionPointId, TaskCoreNodeId, TaskId, TransitionId,
};
use ling_hir as hir;
use ling_resolve::{BindingKey, DefinitionId, ExpressionKey, ModuleId};
use ling_source::Span;
use ling_types::{TypeId, TypedProgram};

use crate::task_core::{CheckedTaskCore, CheckedTaskSuspension};

pub const CHECKED_TASK_MACHINE_VERSION: &str = "ling.task-machine/0.1";

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CheckedTaskMachineEdgeKind {
    Continue,
    Resume,
    Cancel,
    Fault,
    Cleanup,
}

impl CheckedTaskMachineEdgeKind {
    const fn rank(self) -> u8 {
        match self {
            Self::Continue => 0,
            Self::Resume => 1,
            Self::Cancel => 2,
            Self::Fault => 3,
            Self::Cleanup => 4,
        }
    }

    const fn projection(self) -> StateEdgeKind {
        match self {
            Self::Continue | Self::Resume => StateEdgeKind::Resume,
            Self::Cancel => StateEdgeKind::Cancel,
            Self::Fault => StateEdgeKind::Fault,
            Self::Cleanup => StateEdgeKind::Cleanup,
        }
    }

    const fn is_normal(self) -> bool {
        matches!(self, Self::Continue | Self::Resume)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckedTaskMachineStateKind {
    Entry,
    Suspend {
        suspension: SuspensionPointId,
        scope: ScopeId,
        awaited_task: TaskId,
        continuation: ExpressionKey,
    },
    ReturnCleanup,
    CancelCleanup,
    FaultCleanup,
    Completed,
    Cancelled,
    Faulted,
}

impl CheckedTaskMachineStateKind {
    const fn rank(self) -> u8 {
        match self {
            Self::Entry => 0,
            Self::Suspend { .. } => 1,
            Self::ReturnCleanup => 2,
            Self::CancelCleanup => 3,
            Self::FaultCleanup => 4,
            Self::Completed => 5,
            Self::Cancelled => 6,
            Self::Faulted => 7,
        }
    }

    const fn is_active(self) -> bool {
        matches!(self, Self::Entry | Self::Suspend { .. })
    }

    const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled | Self::Faulted)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckedTaskFrameSlot {
    binding: BindingKey,
    value_type: TypeId,
}

impl CheckedTaskFrameSlot {
    #[must_use]
    pub const fn binding(self) -> BindingKey {
        self.binding
    }

    #[must_use]
    pub const fn value_type(self) -> TypeId {
        self.value_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedTaskMachineState {
    id: StateId,
    kind: CheckedTaskMachineStateKind,
    frame: Box<[CheckedTaskFrameSlot]>,
    source_span: Option<Span>,
}

impl CheckedTaskMachineState {
    #[must_use]
    pub const fn id(&self) -> StateId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> CheckedTaskMachineStateKind {
        self.kind
    }

    #[must_use]
    pub fn frame(&self) -> &[CheckedTaskFrameSlot] {
        &self.frame
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckedTaskMachineEdge {
    id: TransitionId,
    from: StateId,
    to: StateId,
    kind: CheckedTaskMachineEdgeKind,
    source_span: Option<Span>,
}

impl CheckedTaskMachineEdge {
    #[must_use]
    pub const fn id(self) -> TransitionId {
        self.id
    }

    #[must_use]
    pub const fn from(self) -> StateId {
        self.from
    }

    #[must_use]
    pub const fn to(self) -> StateId {
        self.to
    }

    #[must_use]
    pub const fn kind(self) -> CheckedTaskMachineEdgeKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedTaskMachine {
    version: &'static str,
    definition: DefinitionId,
    entry: StateId,
    root_scope: ScopeId,
    cancellation: CancellationTokenId,
    cleanup: CleanupRegionId,
    states: Box<[CheckedTaskMachineState]>,
    edges: Box<[CheckedTaskMachineEdge]>,
    source_span: Span,
    projection: Option<StateMachineModel>,
}

impl CheckedTaskMachine {
    #[must_use]
    pub const fn version(&self) -> &'static str {
        self.version
    }

    #[must_use]
    pub const fn definition(&self) -> &DefinitionId {
        &self.definition
    }

    #[must_use]
    pub const fn entry(&self) -> StateId {
        self.entry
    }

    #[must_use]
    pub const fn root_scope(&self) -> ScopeId {
        self.root_scope
    }

    #[must_use]
    pub const fn cancellation(&self) -> CancellationTokenId {
        self.cancellation
    }

    #[must_use]
    pub const fn cleanup(&self) -> CleanupRegionId {
        self.cleanup
    }

    #[must_use]
    pub fn states(&self) -> &[CheckedTaskMachineState] {
        &self.states
    }

    #[must_use]
    pub fn edges(&self) -> &[CheckedTaskMachineEdge] {
        &self.edges
    }

    #[must_use]
    pub const fn source_span(&self) -> Span {
        self.source_span
    }

    #[must_use]
    pub fn projection(&self) -> &StateMachineModel {
        self.projection
            .as_ref()
            .expect("validated Checked Task machine has a DEC-0092 projection")
    }

    #[must_use]
    pub fn state(&self, id: StateId) -> Option<&CheckedTaskMachineState> {
        self.states.iter().find(|state| state.id == id)
    }

    /// Returns path-free deterministic bytes. Source-map evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self, typed: &TypedProgram) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_text(&mut bytes, self.version);
        push_text(&mut bytes, self.definition.as_str());
        push_u32(&mut bytes, self.entry.get());
        push_u32(&mut bytes, self.root_scope.get());
        push_u32(&mut bytes, self.cancellation.get());
        push_u32(&mut bytes, self.cleanup.get());
        push_u32(&mut bytes, self.states.len() as u32);
        for state in &self.states {
            push_u32(&mut bytes, state.id.get());
            bytes.push(state.kind.rank());
            if let CheckedTaskMachineStateKind::Suspend {
                suspension,
                scope,
                awaited_task,
                continuation,
            } = state.kind
            {
                push_u32(&mut bytes, suspension.get());
                push_u32(&mut bytes, scope.get());
                push_u32(&mut bytes, awaited_task.get());
                push_u32(&mut bytes, continuation.module().get());
                push_u32(&mut bytes, continuation.local().get());
            }
            push_u32(&mut bytes, state.frame.len() as u32);
            for slot in &state.frame {
                push_u32(&mut bytes, slot.binding.module().get());
                push_u32(&mut bytes, slot.binding.local().get());
                push_text(&mut bytes, &typed.display_type(slot.value_type));
            }
        }
        push_u32(&mut bytes, self.edges.len() as u32);
        for edge in &self.edges {
            push_u32(&mut bytes, edge.id.get());
            push_u32(&mut bytes, edge.from.get());
            push_u32(&mut bytes, edge.to.get());
            bytes.push(edge.kind.rank());
        }
        bytes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TaskMachineFailure {
    pub source_name: String,
    pub span: Span,
    pub reason: &'static str,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MachineError {
    reason: &'static str,
    detail: String,
}

impl MachineError {
    fn new(reason: &'static str, detail: impl Into<String>) -> Self {
        Self {
            reason,
            detail: detail.into(),
        }
    }
}

impl fmt::Display for MachineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.reason, self.detail)
    }
}

pub(crate) fn build_checked_task_machines(
    typed: &TypedProgram,
    cores: &BTreeMap<DefinitionId, CheckedTaskCore>,
) -> Result<BTreeMap<DefinitionId, CheckedTaskMachine>, Vec<TaskMachineFailure>> {
    let mut output = BTreeMap::new();
    let mut failures = Vec::new();
    for module in typed.resolved().modules() {
        for declaration in &module.hir.tasks {
            let Some(definition) = typed
                .resolved()
                .definition_id(module.id, &declaration.name.normalized)
                .cloned()
            else {
                failures.push(TaskMachineFailure {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.span,
                    reason: "missing_task_machine_definition",
                    detail: declaration.name.normalized.clone(),
                });
                continue;
            };
            let Some(core) = cores.get(&definition) else {
                failures.push(TaskMachineFailure {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.span,
                    reason: "missing_checked_task_core_for_machine",
                    detail: definition.to_string(),
                });
                continue;
            };
            match lower_machine(module.id, declaration, core) {
                Ok(machine) => {
                    output.insert(definition, machine);
                }
                Err(error) => failures.push(TaskMachineFailure {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.span,
                    reason: error.reason,
                    detail: error.detail,
                }),
            }
        }
    }
    if output.len() != cores.len() {
        for definition in cores.keys() {
            if !output.contains_key(definition)
                && !failures
                    .iter()
                    .any(|failure| failure.detail.contains(definition.as_str()))
            {
                failures.push(TaskMachineFailure {
                    source_name: "<checked-task-machine>".to_owned(),
                    span: cores[definition].source_span(),
                    reason: "unmatched_checked_task_core_for_machine",
                    detail: definition.to_string(),
                });
            }
        }
    }
    if failures.is_empty() {
        Ok(output)
    } else {
        failures.sort_by(|left, right| {
            (
                &left.source_name,
                left.span.start(),
                left.reason,
                &left.detail,
            )
                .cmp(&(
                    &right.source_name,
                    right.span.start(),
                    right.reason,
                    &right.detail,
                ))
        });
        Err(failures)
    }
}

fn lower_machine(
    module: ModuleId,
    declaration: &hir::TaskDeclaration,
    core: &CheckedTaskCore,
) -> Result<CheckedTaskMachine, MachineError> {
    let root = core
        .projection()
        .node(core.projection().root_task())
        .ok_or_else(|| {
            MachineError::new(
                "missing_machine_root_task",
                format!("{:?}", core.definition()),
            )
        })?;
    let entry = StateId::new(1);
    let mut states = vec![CheckedTaskMachineState {
        id: entry,
        kind: CheckedTaskMachineStateKind::Entry,
        frame: Box::new([]),
        source_span: Some(core.source_span()),
    }];
    let mut suspension_states = BTreeMap::new();
    for (index, suspension) in core.suspensions().iter().enumerate() {
        let state_id = indexed_state(index, 2)?;
        if suspension.continuation().module() != module {
            return Err(MachineError::new(
                "task_machine_continuation_module_mismatch",
                format!("suspension {}", suspension.id().get()),
            ));
        }
        suspension_states.insert(suspension.continuation(), state_id);
        states.push(suspension_state(state_id, suspension));
    }
    let special_start = u32::try_from(core.suspensions().len())
        .map_err(|_| MachineError::new("task_machine_identity_overflow", "suspension count"))?
        .checked_add(2)
        .ok_or_else(|| MachineError::new("task_machine_identity_overflow", "special states"))?;
    let return_cleanup = StateId::new(special_start);
    let cancel_cleanup = StateId::new(special_start.saturating_add(1));
    let fault_cleanup = StateId::new(special_start.saturating_add(2));
    let completed = StateId::new(special_start.saturating_add(3));
    let cancelled = StateId::new(special_start.saturating_add(4));
    let faulted = StateId::new(special_start.saturating_add(5));
    for (id, kind, span) in [
        (
            return_cleanup,
            CheckedTaskMachineStateKind::ReturnCleanup,
            Some(core.source_span()),
        ),
        (
            cancel_cleanup,
            CheckedTaskMachineStateKind::CancelCleanup,
            Some(core.source_span()),
        ),
        (
            fault_cleanup,
            CheckedTaskMachineStateKind::FaultCleanup,
            Some(core.source_span()),
        ),
        (completed, CheckedTaskMachineStateKind::Completed, None),
        (cancelled, CheckedTaskMachineStateKind::Cancelled, None),
        (faulted, CheckedTaskMachineStateKind::Faulted, None),
    ] {
        states.push(CheckedTaskMachineState {
            id,
            kind,
            frame: Box::new([]),
            source_span: span,
        });
    }

    let mut flow = FlowBuilder::new(module, suspension_states);
    let successors = BTreeSet::from([return_cleanup]);
    let body_entries = flow.lower_expression(&declaration.body, &successors, &successors)?;
    for target in body_entries {
        flow.edge(entry, target, CheckedTaskMachineEdgeKind::Continue);
    }
    for state in states.iter().filter(|state| state.kind.is_active()) {
        flow.edge(state.id, cancel_cleanup, CheckedTaskMachineEdgeKind::Cancel);
        flow.edge(state.id, fault_cleanup, CheckedTaskMachineEdgeKind::Fault);
    }
    flow.edge(
        return_cleanup,
        completed,
        CheckedTaskMachineEdgeKind::Cleanup,
    );
    flow.edge(
        cancel_cleanup,
        cancelled,
        CheckedTaskMachineEdgeKind::Cleanup,
    );
    flow.edge(fault_cleanup, faulted, CheckedTaskMachineEdgeKind::Cleanup);
    flow.ensure_all_suspensions_used()?;
    let edges = flow.finish()?;
    states.sort_by_key(CheckedTaskMachineState::id);
    let mut machine = CheckedTaskMachine {
        version: CHECKED_TASK_MACHINE_VERSION,
        definition: core.definition().clone(),
        entry,
        root_scope: core.root_scope(),
        cancellation: root.cancellation(),
        cleanup: root.cleanup(),
        states: states.into_boxed_slice(),
        edges,
        source_span: core.source_span(),
        projection: None,
    };
    validate_machine(&machine, core)?;
    machine.projection = Some(build_projection(&machine)?);
    Ok(machine)
}

fn indexed_state(index: usize, offset: u32) -> Result<StateId, MachineError> {
    let index = u32::try_from(index)
        .map_err(|_| MachineError::new("task_machine_identity_overflow", "state index"))?;
    let value = index
        .checked_add(offset)
        .ok_or_else(|| MachineError::new("task_machine_identity_overflow", "state identity"))?;
    Ok(StateId::new(value))
}

fn suspension_state(id: StateId, suspension: &CheckedTaskSuspension) -> CheckedTaskMachineState {
    let mut frame = suspension
        .live()
        .iter()
        .map(|live| CheckedTaskFrameSlot {
            binding: live.binding(),
            value_type: live.value_type(),
        })
        .collect::<Vec<_>>();
    frame.sort_by_key(|slot| (slot.binding.module().get(), slot.binding.local().get()));
    CheckedTaskMachineState {
        id,
        kind: CheckedTaskMachineStateKind::Suspend {
            suspension: suspension.id(),
            scope: suspension.scope(),
            awaited_task: suspension.awaited_task(),
            continuation: suspension.continuation(),
        },
        frame: frame.into_boxed_slice(),
        source_span: Some(suspension.span()),
    }
}

struct FlowBuilder {
    module: ModuleId,
    suspension_states: BTreeMap<ExpressionKey, StateId>,
    suspension_uses: BTreeMap<ExpressionKey, u32>,
    edges: BTreeSet<(StateId, StateId, CheckedTaskMachineEdgeKind)>,
}

impl FlowBuilder {
    fn new(module: ModuleId, suspension_states: BTreeMap<ExpressionKey, StateId>) -> Self {
        let suspension_uses = suspension_states
            .keys()
            .copied()
            .map(|key| (key, 0))
            .collect();
        Self {
            module,
            suspension_states,
            suspension_uses,
            edges: BTreeSet::new(),
        }
    }

    fn lower_expression(
        &mut self,
        expression: &hir::Expression,
        successors: &BTreeSet<StateId>,
        return_targets: &BTreeSet<StateId>,
    ) -> Result<BTreeSet<StateId>, MachineError> {
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                let mut current = successors.clone();
                for element in elements.iter().rev() {
                    current = match element {
                        hir::SequenceElement::Let(binding) => {
                            self.lower_expression(&binding.value, &current, return_targets)?
                        }
                        hir::SequenceElement::LetAwait(binding) => self.lower_suspension(
                            binding.call.id,
                            &binding.call,
                            &current,
                            return_targets,
                        )?,
                        hir::SequenceElement::Expression(value) => {
                            self.lower_expression(value, &current, return_targets)?
                        }
                    };
                }
                Ok(current)
            }
            hir::ExpressionKind::TaskScope { body, .. } => {
                self.lower_expression(body, successors, successors)
            }
            hir::ExpressionKind::TaskSpawn { call, .. } => {
                self.lower_expression(call, successors, return_targets)
            }
            hir::ExpressionKind::TaskAwait { handle, .. } => {
                self.lower_suspension(expression.id, handle, successors, return_targets)
            }
            hir::ExpressionKind::TaskReturn { value, .. } => {
                self.lower_expression(value, return_targets, return_targets)
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let mut branches =
                    self.lower_expression(then_branch, successors, return_targets)?;
                branches.extend(self.lower_expression(else_branch, successors, return_targets)?);
                self.lower_expression(condition, &branches, return_targets)
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                let mut branches = BTreeSet::new();
                for case in cases {
                    let body = self.lower_expression(&case.body, successors, return_targets)?;
                    let entry = if let Some(guard) = &case.guard {
                        self.lower_expression(guard, &body, return_targets)?
                    } else {
                        body
                    };
                    branches.extend(entry);
                }
                self.lower_expression(scrutinee, &branches, return_targets)
            }
            hir::ExpressionKind::Handle { body, clauses } => {
                let mut entries = self.lower_expression(body, successors, return_targets)?;
                for clause in clauses {
                    entries.extend(self.lower_expression(
                        &clause.body,
                        successors,
                        return_targets,
                    )?);
                }
                Ok(entries)
            }
            hir::ExpressionKind::Assignment { value, .. }
            | hir::ExpressionKind::Projection { target: value, .. }
            | hir::ExpressionKind::Unary { operand: value, .. } => {
                self.lower_expression(value, successors, return_targets)
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                let mut current = successors.clone();
                for argument in arguments.iter().rev() {
                    current = self.lower_expression(argument, &current, return_targets)?;
                }
                self.lower_expression(function, &current, return_targets)
            }
            hir::ExpressionKind::Binary { left, right, .. } => {
                let right = self.lower_expression(right, successors, return_targets)?;
                self.lower_expression(left, &right, return_targets)
            }
            hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
                self.lower_ordered(values.iter(), successors, return_targets)
            }
            hir::ExpressionKind::Record(fields) => {
                let mut current = successors.clone();
                for field in fields.iter().rev() {
                    current = self.lower_expression(&field.value, &current, return_targets)?;
                }
                Ok(current)
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                let mut current = successors.clone();
                for field in fields.iter().rev() {
                    current = self.lower_expression(&field.value, &current, return_targets)?;
                }
                self.lower_expression(base, &current, return_targets)
            }
            hir::ExpressionKind::Name { .. }
            | hir::ExpressionKind::Literal(_)
            | hir::ExpressionKind::Unit => Ok(successors.clone()),
        }
    }

    fn lower_ordered<'a>(
        &mut self,
        values: impl DoubleEndedIterator<Item = &'a hir::Expression>,
        successors: &BTreeSet<StateId>,
        return_targets: &BTreeSet<StateId>,
    ) -> Result<BTreeSet<StateId>, MachineError> {
        let mut current = successors.clone();
        for value in values.rev() {
            current = self.lower_expression(value, &current, return_targets)?;
        }
        Ok(current)
    }

    fn lower_suspension(
        &mut self,
        expression: hir::ExpressionId,
        evaluated_before_suspend: &hir::Expression,
        successors: &BTreeSet<StateId>,
        return_targets: &BTreeSet<StateId>,
    ) -> Result<BTreeSet<StateId>, MachineError> {
        let key = ExpressionKey::new(self.module, expression);
        let state = self.suspension_states.get(&key).copied().ok_or_else(|| {
            MachineError::new(
                "missing_checked_suspension_for_machine",
                format!("expression {}", expression.get()),
            )
        })?;
        let uses = self
            .suspension_uses
            .get_mut(&key)
            .expect("suspension state and use table agree");
        *uses = uses.saturating_add(1);
        for successor in successors {
            self.edge(state, *successor, CheckedTaskMachineEdgeKind::Resume);
        }
        self.lower_expression(
            evaluated_before_suspend,
            &BTreeSet::from([state]),
            return_targets,
        )
    }

    fn edge(&mut self, from: StateId, to: StateId, kind: CheckedTaskMachineEdgeKind) {
        self.edges.insert((from, to, kind));
    }

    fn ensure_all_suspensions_used(&self) -> Result<(), MachineError> {
        for (continuation, uses) in &self.suspension_uses {
            if *uses != 1 {
                return Err(MachineError::new(
                    "checked_suspension_machine_use_disagreement",
                    format!(
                        "expression {} used {uses} times",
                        continuation.local().get()
                    ),
                ));
            }
        }
        Ok(())
    }

    fn finish(self) -> Result<Box<[CheckedTaskMachineEdge]>, MachineError> {
        let mut output = Vec::with_capacity(self.edges.len());
        for (index, (from, to, kind)) in self.edges.into_iter().enumerate() {
            let id = u32::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .map(TransitionId::new)
                .ok_or_else(|| {
                    MachineError::new("task_machine_identity_overflow", "edge identity")
                })?;
            output.push(CheckedTaskMachineEdge {
                id,
                from,
                to,
                kind,
                source_span: None,
            });
        }
        Ok(output.into_boxed_slice())
    }
}

fn validate_machine(
    machine: &CheckedTaskMachine,
    core: &CheckedTaskCore,
) -> Result<(), MachineError> {
    if machine.version != CHECKED_TASK_MACHINE_VERSION {
        return Err(MachineError::new(
            "unsupported_task_machine_version",
            machine.version,
        ));
    }
    if machine.definition != *core.definition()
        || machine.root_scope != core.root_scope()
        || !machine.entry.is_valid()
        || !machine.cancellation.is_valid()
        || !machine.cleanup.is_valid()
    {
        return Err(MachineError::new(
            "task_machine_header_disagreement",
            machine.definition.to_string(),
        ));
    }
    let mut states = BTreeMap::new();
    let mut roles = BTreeMap::<u8, Vec<StateId>>::new();
    let mut suspension_states = BTreeMap::new();
    for state in &machine.states {
        if !state.id.is_valid() || states.insert(state.id, state).is_some() {
            return Err(MachineError::new(
                "invalid_or_duplicate_task_machine_state",
                state.id.get().to_string(),
            ));
        }
        roles.entry(state.kind.rank()).or_default().push(state.id);
        match state.kind {
            CheckedTaskMachineStateKind::Suspend { suspension, .. } => {
                if suspension_states.insert(suspension, state).is_some() {
                    return Err(MachineError::new(
                        "duplicate_task_machine_suspension",
                        suspension.get().to_string(),
                    ));
                }
            }
            _ if !state.frame.is_empty() => {
                return Err(MachineError::new(
                    "non_suspension_task_machine_frame",
                    state.id.get().to_string(),
                ));
            }
            _ => {}
        }
    }
    for rank in [0_u8, 2, 3, 4, 5, 6, 7] {
        if roles.get(&rank).map_or(0, Vec::len) != 1 {
            return Err(MachineError::new(
                "task_machine_state_role_cardinality",
                rank.to_string(),
            ));
        }
    }
    if states.get(&machine.entry).map(|state| state.kind)
        != Some(CheckedTaskMachineStateKind::Entry)
    {
        return Err(MachineError::new(
            "task_machine_entry_role_disagreement",
            machine.entry.get().to_string(),
        ));
    }
    if suspension_states.len() != core.suspensions().len() {
        return Err(MachineError::new(
            "task_machine_suspension_count_disagreement",
            suspension_states.len().to_string(),
        ));
    }
    for suspension in core.suspensions() {
        let Some(state) = suspension_states.get(&suspension.id()) else {
            return Err(MachineError::new(
                "missing_task_machine_suspension_state",
                suspension.id().get().to_string(),
            ));
        };
        let CheckedTaskMachineStateKind::Suspend {
            scope,
            awaited_task,
            continuation,
            ..
        } = state.kind
        else {
            unreachable!("suspension map contains suspension states")
        };
        let expected_frame = suspension
            .live()
            .iter()
            .map(|live| (live.binding(), live.value_type()))
            .collect::<Vec<_>>();
        let actual_frame = state
            .frame
            .iter()
            .map(|slot| (slot.binding, slot.value_type))
            .collect::<Vec<_>>();
        if scope != suspension.scope()
            || awaited_task != suspension.awaited_task()
            || continuation != suspension.continuation()
            || actual_frame != expected_frame
            || state.source_span != Some(suspension.span())
        {
            return Err(MachineError::new(
                "task_machine_suspension_core_disagreement",
                suspension.id().get().to_string(),
            ));
        }
    }

    let mut edge_ids = BTreeSet::new();
    let mut edge_keys = BTreeSet::new();
    let mut outgoing = BTreeMap::<StateId, Vec<&CheckedTaskMachineEdge>>::new();
    let mut adjacency = BTreeMap::<StateId, Vec<StateId>>::new();
    let mut normal_reverse = BTreeMap::<StateId, Vec<StateId>>::new();
    for edge in &machine.edges {
        if !edge.id.is_valid() || !edge_ids.insert(edge.id) {
            return Err(MachineError::new(
                "invalid_or_duplicate_task_machine_edge",
                edge.id.get().to_string(),
            ));
        }
        if !states.contains_key(&edge.from) || !states.contains_key(&edge.to) {
            return Err(MachineError::new(
                "unknown_task_machine_edge_endpoint",
                edge.id.get().to_string(),
            ));
        }
        if !edge_keys.insert((edge.from, edge.to, edge.kind)) {
            return Err(MachineError::new(
                "duplicate_task_machine_edge",
                edge.id.get().to_string(),
            ));
        }
        outgoing.entry(edge.from).or_default().push(edge);
        adjacency.entry(edge.from).or_default().push(edge.to);
        if edge.kind.is_normal() {
            normal_reverse.entry(edge.to).or_default().push(edge.from);
        }
    }
    for state in &machine.states {
        let state_outgoing = outgoing.get(&state.id).map_or(&[][..], Vec::as_slice);
        if state.kind.is_terminal() && !state_outgoing.is_empty() {
            return Err(MachineError::new(
                "terminal_task_machine_state_has_outgoing_edge",
                state.id.get().to_string(),
            ));
        }
        if state.kind.is_active() {
            let expected_normal = if matches!(state.kind, CheckedTaskMachineStateKind::Entry) {
                CheckedTaskMachineEdgeKind::Continue
            } else {
                CheckedTaskMachineEdgeKind::Resume
            };
            if !state_outgoing
                .iter()
                .any(|edge| edge.kind == expected_normal)
                || state_outgoing
                    .iter()
                    .any(|edge| edge.kind.is_normal() && edge.kind != expected_normal)
            {
                return Err(MachineError::new(
                    "task_machine_normal_edge_kind_disagreement",
                    state.id.get().to_string(),
                ));
            }
        }
    }
    let role = |rank: u8| roles[&rank][0];
    let return_cleanup = role(2);
    let cancel_cleanup = role(3);
    let fault_cleanup = role(4);
    let completed = role(5);
    let cancelled = role(6);
    let faulted = role(7);
    for state in machine.states.iter().filter(|state| state.kind.is_active()) {
        require_edge(
            &edge_keys,
            state.id,
            cancel_cleanup,
            CheckedTaskMachineEdgeKind::Cancel,
        )?;
        require_edge(
            &edge_keys,
            state.id,
            fault_cleanup,
            CheckedTaskMachineEdgeKind::Fault,
        )?;
    }
    for (from, to) in [
        (return_cleanup, completed),
        (cancel_cleanup, cancelled),
        (fault_cleanup, faulted),
    ] {
        require_edge(&edge_keys, from, to, CheckedTaskMachineEdgeKind::Cleanup)?;
        if outgoing.get(&from).map_or(0, Vec::len) != 1 {
            return Err(MachineError::new(
                "task_machine_cleanup_exit_disagreement",
                from.get().to_string(),
            ));
        }
    }

    let reachable = reachable_from(machine.entry, &adjacency);
    if reachable.len() != machine.states.len() {
        return Err(MachineError::new(
            "unreachable_task_machine_state",
            format!("{} of {}", reachable.len(), machine.states.len()),
        ));
    }
    let can_return = reachable_from(return_cleanup, &normal_reverse);
    for state in machine.states.iter().filter(|state| state.kind.is_active()) {
        if !can_return.contains(&state.id) {
            return Err(MachineError::new(
                "task_machine_normal_path_lacks_return_cleanup",
                state.id.get().to_string(),
            ));
        }
    }
    Ok(())
}

fn require_edge(
    edges: &BTreeSet<(StateId, StateId, CheckedTaskMachineEdgeKind)>,
    from: StateId,
    to: StateId,
    kind: CheckedTaskMachineEdgeKind,
) -> Result<(), MachineError> {
    if edges.contains(&(from, to, kind)) {
        Ok(())
    } else {
        Err(MachineError::new(
            "missing_task_machine_exit_edge",
            format!("{} -> {} ({kind:?})", from.get(), to.get()),
        ))
    }
}

fn reachable_from(
    start: StateId,
    adjacency: &BTreeMap<StateId, Vec<StateId>>,
) -> BTreeSet<StateId> {
    let mut reachable = BTreeSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(state) = queue.pop_front() {
        if !reachable.insert(state) {
            continue;
        }
        if let Some(successors) = adjacency.get(&state) {
            queue.extend(successors.iter().copied());
        }
    }
    reachable
}

fn build_projection(machine: &CheckedTaskMachine) -> Result<StateMachineModel, MachineError> {
    let states = machine.states.iter().map(|state| {
        let continuation = match state.kind {
            CheckedTaskMachineStateKind::Entry => machine.entry.get(),
            CheckedTaskMachineStateKind::Suspend { continuation, .. } => {
                continuation.local().get().saturating_add(1)
            }
            _ => state.id.get(),
        };
        StateNode::new(StateNodeSpec {
            id: state.id,
            continuation: TaskCoreNodeId::new(continuation),
            live_locals: state
                .frame
                .iter()
                .map(|slot| LocalId::new(slot.binding.local().get().saturating_add(1)))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            source_span: state.source_span,
        })
    });
    let transitions = machine.edges.iter().map(|edge| {
        StateTransition::new(StateTransitionSpec {
            id: edge.id,
            from: edge.from,
            to: edge.to,
            kind: edge.kind.projection(),
            source_span: edge.source_span,
        })
    });
    StateMachineModel::new(
        TaskId::new(1),
        machine.entry,
        states,
        transitions,
        Some(machine.source_span),
    )
    .map_err(|error| MachineError::new("invalid_task_machine_projection", error.to_string()))
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_text(bytes: &mut Vec<u8>, value: &str) {
    push_u32(bytes, value.len() as u32);
    bytes.extend_from_slice(value.as_bytes());
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_hir::lower as lower_hir;
    use ling_resolve::resolve;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;
    use ling_types::check as check_types;

    use super::*;

    fn machine_and_core() -> (CheckedTaskMachine, CheckedTaskCore) {
        let text = concat!(
            "module Main\n\n",
            "task child value =\n",
            "    scope\n",
            "        return value\n\n",
            "task parent value =\n",
            "    scope\n",
            "        let seed = value\n",
            "        let! result = child value\n",
            "        return seed\n",
        );
        let source = SourceFile::from_bytes(
            SourceId::new(0),
            "machine-validation.ling",
            text.as_bytes().to_vec(),
        )
        .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = lower_hir(source.name(), &ast).expect("valid HIR");
        let resolved = resolve(vec![hir], "Main").expect("resolved program");
        let typed = check_types(resolved).expect("typed program");
        let checked = crate::check(typed).expect("checked program");
        let definition = checked
            .task_machines()
            .iter()
            .find_map(|(definition, machine)| {
                machine
                    .states()
                    .iter()
                    .any(|state| {
                        matches!(state.kind(), CheckedTaskMachineStateKind::Suspend { .. })
                    })
                    .then_some(definition.clone())
            })
            .expect("parent Task definition");
        (
            checked
                .task_machine(&definition)
                .expect("Task machine")
                .clone(),
            checked.task_core(&definition).expect("Task Core").clone(),
        )
    }

    fn validation_reason(machine: &CheckedTaskMachine, core: &CheckedTaskCore) -> &'static str {
        validate_machine(machine, core)
            .expect_err("malformed machine must fail validation")
            .reason
    }

    #[test]
    fn validation_rejects_malformed_roles_endpoints_exits_and_frames() {
        let (machine, core) = machine_and_core();

        let mut invalid = machine.clone();
        invalid.version = "ling.task-machine/invalid";
        assert_eq!(
            validation_reason(&invalid, &core),
            "unsupported_task_machine_version"
        );

        let mut invalid = machine.clone();
        invalid.states[0].id = invalid.states[1].id;
        assert_eq!(
            validation_reason(&invalid, &core),
            "invalid_or_duplicate_task_machine_state"
        );

        let mut invalid = machine.clone();
        invalid.edges[0].to = StateId::new(u32::MAX);
        assert_eq!(
            validation_reason(&invalid, &core),
            "unknown_task_machine_edge_endpoint"
        );

        let suspend = machine
            .states
            .iter()
            .find(|state| matches!(state.kind, CheckedTaskMachineStateKind::Suspend { .. }))
            .expect("suspension");
        let mut invalid = machine.clone();
        invalid
            .states
            .iter_mut()
            .find(|state| state.id == suspend.id)
            .expect("suspension")
            .frame = Box::new([]);
        assert_eq!(
            validation_reason(&invalid, &core),
            "task_machine_suspension_core_disagreement"
        );

        let mut invalid = machine.clone();
        let cancel_edge = invalid
            .edges
            .iter()
            .position(|edge| {
                edge.from == suspend.id && edge.kind == CheckedTaskMachineEdgeKind::Cancel
            })
            .expect("cancel edge");
        let mut edges = invalid.edges.to_vec();
        edges.remove(cancel_edge);
        invalid.edges = edges.into_boxed_slice();
        assert_eq!(
            validation_reason(&invalid, &core),
            "missing_task_machine_exit_edge"
        );

        let mut invalid = machine.clone();
        let terminal = invalid
            .states
            .iter()
            .find(|state| matches!(state.kind, CheckedTaskMachineStateKind::Completed))
            .expect("terminal")
            .id;
        let mut edges = invalid.edges.to_vec();
        edges.push(CheckedTaskMachineEdge {
            id: TransitionId::new(edges.len() as u32 + 1),
            from: terminal,
            to: terminal,
            kind: CheckedTaskMachineEdgeKind::Cleanup,
            source_span: None,
        });
        invalid.edges = edges.into_boxed_slice();
        assert_eq!(
            validation_reason(&invalid, &core),
            "terminal_task_machine_state_has_outgoing_edge"
        );
    }

    #[test]
    fn validation_accepts_a_synthetic_checked_loop_boundary() {
        let (mut machine, core) = machine_and_core();
        let suspend = machine
            .states
            .iter()
            .find(|state| matches!(state.kind, CheckedTaskMachineStateKind::Suspend { .. }))
            .expect("suspension")
            .id;
        let mut edges = machine.edges.to_vec();
        edges.push(CheckedTaskMachineEdge {
            id: TransitionId::new(edges.len() as u32 + 1),
            from: suspend,
            to: suspend,
            kind: CheckedTaskMachineEdgeKind::Resume,
            source_span: None,
        });
        edges.sort_by_key(|edge| edge.id);
        machine.edges = edges.into_boxed_slice();

        validate_machine(&machine, &core).expect("future checked back edge remains representable");
    }
}
