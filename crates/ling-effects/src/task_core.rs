//! DEC-0264 checked-only Structured Task projection and static ownership checks.

use std::collections::{BTreeMap, BTreeSet};

use ling_concurrency::{
    CancellationTokenId, CleanupRegionId, ScopeId, SuspensionPoint, SuspensionPointId, TaskCore,
    TaskCoreNodeId, TaskId, TaskNode, TaskNodeSpec,
};
use ling_hir as hir;
use ling_resolve::{
    BindingKey, DefinitionId, DefinitionKind, ExpressionKey, ModuleId, ReferenceTarget,
};
use ling_source::Span;
use ling_types::{Type, TypeId, TypedProgram};

use crate::EffectRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedTaskSignature {
    parameters: Box<[TypeId]>,
    result: TypeId,
    effects: EffectRow,
}

impl CheckedTaskSignature {
    #[must_use]
    pub fn parameters(&self) -> &[TypeId] {
        &self.parameters
    }

    #[must_use]
    pub const fn result(&self) -> TypeId {
        self.result
    }

    #[must_use]
    pub const fn effects(&self) -> &EffectRow {
        &self.effects
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckedTaskScope {
    id: ScopeId,
    parent: Option<ScopeId>,
    expression: ExpressionKey,
    span: Span,
}

impl CheckedTaskScope {
    #[must_use]
    pub const fn id(self) -> ScopeId {
        self.id
    }

    #[must_use]
    pub const fn parent(self) -> Option<ScopeId> {
        self.parent
    }

    #[must_use]
    pub const fn expression(self) -> ExpressionKey {
        self.expression
    }

    #[must_use]
    pub const fn span(self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckedTaskSpawnSyntax {
    Explicit,
    FusedLetAwait,
}

impl CheckedTaskSpawnSyntax {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Explicit => "explicit",
            Self::FusedLetAwait => "let!",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedTaskSpawn {
    task: TaskId,
    scope: ScopeId,
    parent: TaskId,
    target: DefinitionId,
    call: ExpressionKey,
    cancellation: CancellationTokenId,
    cleanup: CleanupRegionId,
    span: Span,
    target_span: Span,
    call_span: Span,
    syntax: CheckedTaskSpawnSyntax,
    operator_span: Span,
    pattern_span: Option<Span>,
}

impl CheckedTaskSpawn {
    #[must_use]
    pub const fn task(&self) -> TaskId {
        self.task
    }

    #[must_use]
    pub const fn scope(&self) -> ScopeId {
        self.scope
    }

    #[must_use]
    pub const fn parent(&self) -> TaskId {
        self.parent
    }

    #[must_use]
    pub const fn target(&self) -> &DefinitionId {
        &self.target
    }

    #[must_use]
    pub const fn call(&self) -> ExpressionKey {
        self.call
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
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub const fn target_span(&self) -> Span {
        self.target_span
    }

    #[must_use]
    pub const fn call_span(&self) -> Span {
        self.call_span
    }

    #[must_use]
    pub const fn syntax(&self) -> &CheckedTaskSpawnSyntax {
        &self.syntax
    }

    #[must_use]
    pub const fn operator_span(&self) -> Span {
        self.operator_span
    }

    #[must_use]
    pub const fn pattern_span(&self) -> Option<Span> {
        self.pattern_span
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuspensionLiveBinding {
    binding: BindingKey,
    value_type: TypeId,
}

impl SuspensionLiveBinding {
    #[must_use]
    pub const fn binding(&self) -> BindingKey {
        self.binding
    }

    #[must_use]
    pub const fn value_type(&self) -> TypeId {
        self.value_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedTaskSuspension {
    id: SuspensionPointId,
    scope: ScopeId,
    awaited_task: TaskId,
    continuation: ExpressionKey,
    live: Box<[SuspensionLiveBinding]>,
    span: Span,
}

impl CheckedTaskSuspension {
    #[must_use]
    pub const fn id(&self) -> SuspensionPointId {
        self.id
    }

    #[must_use]
    pub const fn scope(&self) -> ScopeId {
        self.scope
    }

    #[must_use]
    pub const fn awaited_task(&self) -> TaskId {
        self.awaited_task
    }

    #[must_use]
    pub const fn continuation(&self) -> ExpressionKey {
        self.continuation
    }

    #[must_use]
    pub fn live(&self) -> &[SuspensionLiveBinding] {
        &self.live
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedTaskCore {
    definition: DefinitionId,
    signature: CheckedTaskSignature,
    root_scope: ScopeId,
    scopes: Box<[CheckedTaskScope]>,
    spawns: Box<[CheckedTaskSpawn]>,
    suspensions: Box<[CheckedTaskSuspension]>,
    result_body: ExpressionKey,
    source_span: Span,
    projection: TaskCore,
}

impl CheckedTaskCore {
    #[must_use]
    pub const fn definition(&self) -> &DefinitionId {
        &self.definition
    }

    #[must_use]
    pub const fn signature(&self) -> &CheckedTaskSignature {
        &self.signature
    }

    #[must_use]
    pub const fn root_scope(&self) -> ScopeId {
        self.root_scope
    }

    #[must_use]
    pub fn scopes(&self) -> &[CheckedTaskScope] {
        &self.scopes
    }

    #[must_use]
    pub fn spawns(&self) -> &[CheckedTaskSpawn] {
        &self.spawns
    }

    #[must_use]
    pub fn suspensions(&self) -> &[CheckedTaskSuspension] {
        &self.suspensions
    }

    #[must_use]
    pub const fn result_body(&self) -> ExpressionKey {
        self.result_body
    }

    #[must_use]
    pub const fn source_span(&self) -> Span {
        self.source_span
    }

    #[must_use]
    pub const fn projection(&self) -> &TaskCore {
        &self.projection
    }

    /// Path-free deterministic bytes for checked reconstruction tests.
    #[must_use]
    pub fn canonical_bytes(&self, typed: &TypedProgram) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_text(&mut bytes, "ling.checked-task-core/0");
        push_text(&mut bytes, self.definition.as_str());
        push_u32(&mut bytes, self.signature.parameters.len() as u32);
        for parameter in &self.signature.parameters {
            push_text(&mut bytes, &typed.display_type(*parameter));
        }
        push_text(&mut bytes, &typed.display_type(self.signature.result));
        let effects = self.signature.effects.canonical_names();
        push_u32(&mut bytes, effects.len() as u32);
        for effect in effects {
            push_text(&mut bytes, &effect);
        }
        push_u32(&mut bytes, self.scopes.len() as u32);
        for scope in &self.scopes {
            push_u32(&mut bytes, scope.id.get());
            push_u32(&mut bytes, scope.parent.map_or(0, ScopeId::get));
        }
        push_u32(&mut bytes, self.spawns.len() as u32);
        for spawn in &self.spawns {
            push_u32(&mut bytes, spawn.task.get());
            push_u32(&mut bytes, spawn.scope.get());
            push_u32(&mut bytes, spawn.parent.get());
            push_text(&mut bytes, spawn.target.as_str());
            push_text(&mut bytes, spawn.syntax.as_str());
            push_u32(&mut bytes, spawn.cancellation.get());
            push_u32(&mut bytes, spawn.cleanup.get());
        }
        push_u32(&mut bytes, self.suspensions.len() as u32);
        for suspension in &self.suspensions {
            push_u32(&mut bytes, suspension.id.get());
            push_u32(&mut bytes, suspension.scope.get());
            push_u32(&mut bytes, suspension.awaited_task.get());
            push_u32(&mut bytes, suspension.live.len() as u32);
            for live in &suspension.live {
                push_u32(&mut bytes, live.binding.module().get());
                push_u32(&mut bytes, live.binding.local().get());
                push_text(&mut bytes, &typed.display_type(live.value_type));
            }
        }
        push_u32(&mut bytes, self.result_body.local().get());
        bytes.extend_from_slice(&self.projection.canonical_bytes());
        bytes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TaskCheckFailure {
    pub source_name: String,
    pub span: Span,
    pub kind: TaskCheckFailureKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum TaskCheckFailureKind {
    Structure {
        reason: &'static str,
        target: Option<String>,
    },
    Handle {
        reason: &'static str,
        handle: Option<String>,
        scope: Option<u32>,
    },
    Suspension {
        reason: &'static str,
        binding: Option<String>,
        suspension: u32,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HandleUse {
    scope: ScopeId,
    task: TaskId,
    consumed: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct FlowState {
    bindings: BTreeSet<BindingKey>,
    handles: BTreeMap<BindingKey, HandleUse>,
}

pub(crate) fn build_checked_task_cores(
    typed: &TypedProgram,
    definition_effects: &BTreeMap<DefinitionId, EffectRow>,
) -> Result<BTreeMap<DefinitionId, CheckedTaskCore>, Vec<TaskCheckFailure>> {
    let mut output = BTreeMap::new();
    let mut failures = Vec::new();
    for module in typed.resolved().modules() {
        for declaration in &module.hir.tasks {
            let Some(definition) = typed
                .resolved()
                .definition_id(module.id, &declaration.name.normalized)
                .cloned()
            else {
                failures.push(TaskCheckFailure {
                    source_name: module.hir.source_name.clone(),
                    span: declaration.name.span,
                    kind: TaskCheckFailureKind::Structure {
                        reason: "unresolved_task_definition",
                        target: Some(declaration.name.normalized.clone()),
                    },
                });
                continue;
            };
            match Builder::new(
                typed,
                definition_effects,
                module.id,
                &module.hir.source_name,
            )
            .build(definition.clone(), declaration)
            {
                Ok(core) => {
                    output.insert(definition, core);
                }
                Err(mut errors) => failures.append(&mut errors),
            }
        }
    }
    detect_spawn_cycles(&output, typed, &mut failures);
    if failures.is_empty() {
        Ok(output)
    } else {
        failures.sort_by_key(|failure| {
            (
                failure.source_name.clone(),
                failure.span.start().get(),
                format!("{:?}", failure.kind),
            )
        });
        Err(failures)
    }
}

struct Builder<'a> {
    typed: &'a TypedProgram,
    definition_effects: &'a BTreeMap<DefinitionId, EffectRow>,
    module: ModuleId,
    source_name: &'a str,
    scopes_by_expression: BTreeMap<hir::ExpressionId, ScopeId>,
    expression_scope: BTreeMap<hir::ExpressionId, ScopeId>,
    spawn_by_expression: BTreeMap<hir::ExpressionId, (TaskId, ScopeId)>,
    suspension_by_expression: BTreeMap<hir::ExpressionId, SuspensionPointId>,
    scopes: Vec<CheckedTaskScope>,
    spawns: Vec<CheckedTaskSpawn>,
    suspensions: Vec<CheckedTaskSuspension>,
    resume_bindings: BTreeSet<BindingKey>,
    failures: Vec<TaskCheckFailure>,
    next_scope: u32,
    next_task: u32,
    next_suspension: u32,
}

impl<'a> Builder<'a> {
    fn new(
        typed: &'a TypedProgram,
        definition_effects: &'a BTreeMap<DefinitionId, EffectRow>,
        module: ModuleId,
        source_name: &'a str,
    ) -> Self {
        Self {
            typed,
            definition_effects,
            module,
            source_name,
            scopes_by_expression: BTreeMap::new(),
            expression_scope: BTreeMap::new(),
            spawn_by_expression: BTreeMap::new(),
            suspension_by_expression: BTreeMap::new(),
            scopes: Vec::new(),
            spawns: Vec::new(),
            suspensions: Vec::new(),
            resume_bindings: BTreeSet::new(),
            failures: Vec::new(),
            next_scope: 1,
            next_task: 2,
            next_suspension: 1,
        }
    }

    fn build(
        mut self,
        definition: DefinitionId,
        declaration: &hir::TaskDeclaration,
    ) -> Result<CheckedTaskCore, Vec<TaskCheckFailure>> {
        let Some(type_id) = self.typed.definition_type(&definition) else {
            self.structure(
                declaration.span,
                "missing_task_type",
                Some(definition.to_string()),
            );
            return Err(self.failures);
        };
        let Type::Task { parameters, result } = self.typed.arena().get(type_id) else {
            self.structure(
                declaration.span,
                "task_type_disagreement",
                Some(definition.to_string()),
            );
            return Err(self.failures);
        };
        let Some(effects) = self.definition_effects.get(&definition).cloned() else {
            self.structure(
                declaration.span,
                "missing_task_effect_row",
                Some(definition.to_string()),
            );
            return Err(self.failures);
        };
        let hir::ExpressionKind::TaskScope { .. } = &declaration.body.kind else {
            self.structure(declaration.body.span, "missing_outer_scope", None);
            return Err(self.failures);
        };
        self.index_expression(&declaration.body, None);
        self.collect_resume_bindings(&declaration.body);
        for (span, reason) in validate_task_returns(&declaration.body) {
            self.structure(span, reason, None);
        }
        let mut state = FlowState::default();
        for parameter in &declaration.parameters {
            collect_pattern_bindings(self.module, parameter, &mut state.bindings);
        }
        self.visit_expression(&declaration.body, &BTreeSet::new(), &mut state, false);
        let available = state
            .handles
            .iter()
            .filter(|(_, handle)| !handle.consumed)
            .map(|(binding, _)| *binding)
            .collect::<Vec<_>>();
        for binding in available {
            self.handle_error(
                declaration.body.span,
                "handle_not_observed_on_scope_exit",
                self.binding_name(binding),
                None,
            );
        }
        if !self.failures.is_empty() {
            return Err(self.failures);
        }

        let root_scope = self.scopes[0].id;
        let root_task = TaskId::new(1);
        let root_suspensions = self
            .suspensions
            .iter()
            .map(|suspension| {
                SuspensionPoint::new(
                    suspension.id,
                    TaskCoreNodeId::new(declaration.body.id.get().saturating_add(1)),
                    Some(suspension.span),
                )
            })
            .collect::<Vec<_>>();
        let mut nodes = vec![TaskNode::new(TaskNodeSpec {
            task: root_task,
            scope: root_scope,
            parent: None,
            body: TaskCoreNodeId::new(declaration.body.id.get().saturating_add(1)),
            cancellation: CancellationTokenId::new(1),
            cleanup: CleanupRegionId::new(1),
            suspension_points: root_suspensions.into_boxed_slice(),
            detach: None,
            source_span: Some(declaration.span),
        })];
        for spawn in &self.spawns {
            nodes.push(TaskNode::new(TaskNodeSpec {
                task: spawn.task,
                scope: spawn.scope,
                parent: Some(spawn.parent),
                body: TaskCoreNodeId::new(spawn.task.get()),
                cancellation: spawn.cancellation,
                cleanup: spawn.cleanup,
                suspension_points: Box::new([]),
                detach: None,
                source_span: Some(spawn.span),
            }));
        }
        let projection = TaskCore::new(root_scope, root_task, nodes, Some(declaration.span))
            .map_err(|error| {
                vec![TaskCheckFailure {
                    source_name: self.source_name.to_owned(),
                    span: declaration.span,
                    kind: TaskCheckFailureKind::Structure {
                        reason: "invalid_task_core_projection",
                        target: Some(error.to_string()),
                    },
                }]
            })?;
        Ok(CheckedTaskCore {
            definition,
            signature: CheckedTaskSignature {
                parameters: parameters.clone().into_boxed_slice(),
                result: *result,
                effects,
            },
            root_scope,
            scopes: self.scopes.into_boxed_slice(),
            spawns: self.spawns.into_boxed_slice(),
            suspensions: self.suspensions.into_boxed_slice(),
            result_body: ExpressionKey::new(self.module, declaration.body.id),
            source_span: declaration.span,
            projection,
        })
    }

    fn index_expression(&mut self, expression: &hir::Expression, parent_scope: Option<ScopeId>) {
        if let Some(scope) = parent_scope {
            self.expression_scope.insert(expression.id, scope);
        }
        match &expression.kind {
            hir::ExpressionKind::TaskScope { body, .. } => {
                let scope = ScopeId::new(self.next_scope);
                self.next_scope = self.next_scope.saturating_add(1);
                self.scopes_by_expression.insert(expression.id, scope);
                self.expression_scope.insert(expression.id, scope);
                self.scopes.push(CheckedTaskScope {
                    id: scope,
                    parent: parent_scope,
                    expression: ExpressionKey::new(self.module, expression.id),
                    span: expression.span,
                });
                self.index_expression(body, Some(scope));
            }
            hir::ExpressionKind::TaskSpawn { call, .. } => {
                self.index_spawn(call, expression.span, parent_scope);
                self.index_expression(call, parent_scope);
            }
            hir::ExpressionKind::TaskAwait { handle, .. } => {
                self.suspension_by_expression
                    .insert(expression.id, SuspensionPointId::new(self.next_suspension));
                self.next_suspension = self.next_suspension.saturating_add(1);
                self.index_expression(handle, parent_scope);
            }
            hir::ExpressionKind::Sequence(elements) => {
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            self.index_expression(&binding.value, parent_scope)
                        }
                        hir::SequenceElement::LetAwait(binding) => {
                            self.index_spawn(&binding.call, binding.span, parent_scope);
                            self.suspension_by_expression.insert(
                                binding.call.id,
                                SuspensionPointId::new(self.next_suspension),
                            );
                            self.next_suspension = self.next_suspension.saturating_add(1);
                            self.index_expression(&binding.call, parent_scope);
                        }
                        hir::SequenceElement::Expression(value) => {
                            self.index_expression(value, parent_scope)
                        }
                    }
                }
            }
            hir::ExpressionKind::Handle { body, clauses } => {
                self.index_expression(body, parent_scope);
                for clause in clauses {
                    self.index_expression(&clause.body, parent_scope);
                }
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.index_expression(condition, parent_scope);
                self.index_expression(then_branch, parent_scope);
                self.index_expression(else_branch, parent_scope);
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                self.index_expression(scrutinee, parent_scope);
                for case in cases {
                    if let Some(guard) = &case.guard {
                        self.index_expression(guard, parent_scope);
                    }
                    self.index_expression(&case.body, parent_scope);
                }
            }
            hir::ExpressionKind::Assignment { value, .. }
            | hir::ExpressionKind::TaskReturn { value, .. }
            | hir::ExpressionKind::Unary { operand: value, .. }
            | hir::ExpressionKind::Projection { target: value, .. } => {
                self.index_expression(value, parent_scope)
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                self.index_expression(function, parent_scope);
                for argument in arguments {
                    self.index_expression(argument, parent_scope);
                }
            }
            hir::ExpressionKind::Binary { left, right, .. } => {
                self.index_expression(left, parent_scope);
                self.index_expression(right, parent_scope);
            }
            hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
                for value in values {
                    self.index_expression(value, parent_scope);
                }
            }
            hir::ExpressionKind::Record(fields) => {
                for field in fields {
                    self.index_expression(&field.value, parent_scope);
                }
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                self.index_expression(base, parent_scope);
                for field in fields {
                    self.index_expression(&field.value, parent_scope);
                }
            }
            hir::ExpressionKind::Name { .. }
            | hir::ExpressionKind::Literal(_)
            | hir::ExpressionKind::Unit => {}
        }
    }

    fn index_spawn(&mut self, call: &hir::Expression, span: Span, scope: Option<ScopeId>) {
        let task = TaskId::new(self.next_task);
        self.next_task = self.next_task.saturating_add(1);
        self.spawn_by_expression
            .insert(call.id, (task, scope.unwrap_or(ScopeId::new(0))));
        let _ = span;
    }

    fn visit_expression(
        &mut self,
        expression: &hir::Expression,
        future: &BTreeSet<BindingKey>,
        state: &mut FlowState,
        ordinary: bool,
    ) {
        match &expression.kind {
            hir::ExpressionKind::TaskScope { body, .. } => {
                let scope = self.scopes_by_expression[&expression.id];
                let before_bindings = state.bindings.clone();
                let before_handles = state.handles.clone();
                self.visit_expression(body, future, state, false);
                let leaked = state
                    .handles
                    .iter()
                    .filter(|(_, handle)| handle.scope == scope && !handle.consumed)
                    .map(|(binding, _)| *binding)
                    .collect::<Vec<_>>();
                for binding in leaked {
                    self.handle_error(
                        expression.span,
                        "handle_not_observed_on_scope_exit",
                        self.binding_name(binding),
                        Some(scope.get()),
                    );
                }
                state.bindings = before_bindings;
                state
                    .handles
                    .retain(|binding, _| before_handles.contains_key(binding));
                for (binding, handle) in before_handles {
                    state.handles.insert(binding, handle);
                }
            }
            hir::ExpressionKind::Sequence(elements) => {
                for (index, element) in elements.iter().enumerate() {
                    let mut next = future.clone();
                    for suffix in &elements[index + 1..] {
                        collect_element_references(self.module, suffix, self.typed, &mut next);
                    }
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            if matches!(binding.value.kind, hir::ExpressionKind::TaskSpawn { .. }) {
                                self.bind_spawn(binding, &next, state);
                            } else {
                                self.visit_expression(&binding.value, &next, state, true);
                                state
                                    .bindings
                                    .insert(BindingKey::new(self.module, binding.id));
                            }
                        }
                        hir::SequenceElement::LetAwait(binding) => {
                            let (task, scope, target) = self.validate_task_call(&binding.call);
                            if let (Some(task), Some(scope)) = (task, scope) {
                                self.record_spawn(
                                    &binding.call,
                                    binding.span,
                                    binding.let_bang_span,
                                    Some(binding.pattern.span),
                                    CheckedTaskSpawnSyntax::FusedLetAwait,
                                    target,
                                );
                                self.record_suspension(
                                    binding.call.id,
                                    binding.span,
                                    scope,
                                    task,
                                    None,
                                    &next,
                                    state,
                                );
                            }
                            collect_pattern_bindings(
                                self.module,
                                &binding.pattern,
                                &mut state.bindings,
                            );
                            self.visit_task_call_arguments(&binding.call, &next, state);
                        }
                        hir::SequenceElement::Expression(value) => {
                            self.visit_expression(value, &next, state, ordinary)
                        }
                    }
                }
            }
            hir::ExpressionKind::TaskSpawn { call, .. } => {
                self.validate_task_call(call);
                self.visit_task_call_arguments(call, future, state);
                if ordinary {
                    self.handle_error(expression.span, "spawn_handle_not_bound", None, None);
                }
            }
            hir::ExpressionKind::TaskAwait { handle, .. } => {
                self.consume_handle(expression, handle, future, state);
            }
            hir::ExpressionKind::TaskReturn { value, .. } => {
                self.visit_expression(value, future, state, true);
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(condition, future, state, true);
                self.visit_branches([then_branch.as_ref(), else_branch.as_ref()], future, state);
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                self.visit_expression(scrutinee, future, state, true);
                for case in cases {
                    if let Some(guard) = &case.guard {
                        self.visit_expression(guard, future, state, true);
                    }
                }
                self.visit_branches(cases.iter().map(|case| &case.body), future, state);
            }
            hir::ExpressionKind::Handle { body, clauses } => {
                self.visit_expression(body, future, state, true);
                for clause in clauses {
                    self.visit_expression(&clause.body, future, state, true);
                }
            }
            hir::ExpressionKind::Name { reference, .. } => {
                if ordinary {
                    self.reject_handle_reference(*reference, expression.span, state);
                }
            }
            hir::ExpressionKind::Assignment { value, .. } => {
                self.visit_expression(value, future, state, true)
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                self.visit_expression(function, future, state, true);
                for argument in arguments {
                    self.visit_expression(argument, future, state, true);
                }
            }
            hir::ExpressionKind::Projection { target, .. }
            | hir::ExpressionKind::Unary {
                operand: target, ..
            } => self.visit_expression(target, future, state, true),
            hir::ExpressionKind::Binary { left, right, .. } => {
                self.visit_expression(left, future, state, true);
                self.visit_expression(right, future, state, true);
            }
            hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
                for value in values {
                    self.visit_expression(value, future, state, true);
                }
            }
            hir::ExpressionKind::Record(fields) => {
                for field in fields {
                    self.visit_expression(&field.value, future, state, true);
                }
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                self.visit_expression(base, future, state, true);
                for field in fields {
                    self.visit_expression(&field.value, future, state, true);
                }
            }
            hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
        }
    }

    fn bind_spawn(
        &mut self,
        binding: &hir::LocalBinding,
        future: &BTreeSet<BindingKey>,
        state: &mut FlowState,
    ) {
        let hir::ExpressionKind::TaskSpawn { keyword_span, call } = &binding.value.kind else {
            return;
        };
        let (task, scope, target) = self.validate_task_call(call);
        self.visit_task_call_arguments(call, future, state);
        let key = BindingKey::new(self.module, binding.id);
        state.bindings.insert(key);
        if let (Some(task), Some(scope)) = (task, scope) {
            self.record_spawn(
                call,
                binding.value.span,
                *keyword_span,
                None,
                CheckedTaskSpawnSyntax::Explicit,
                target,
            );
            state.handles.insert(
                key,
                HandleUse {
                    scope,
                    task,
                    consumed: false,
                },
            );
        }
    }

    fn validate_task_call(
        &mut self,
        call: &hir::Expression,
    ) -> (Option<TaskId>, Option<ScopeId>, Option<DefinitionId>) {
        let Some(&(task, scope)) = self.spawn_by_expression.get(&call.id) else {
            self.structure(call.span, "missing_spawn_identity", None);
            return (None, None, None);
        };
        if !scope.is_valid() {
            self.structure(call.span, "spawn_outside_scope", None);
            return (Some(task), None, None);
        }
        let hir::ExpressionKind::Application { function, .. } = &call.kind else {
            self.structure(
                call.span,
                "spawn_target_must_be_direct_task_application",
                None,
            );
            return (Some(task), Some(scope), None);
        };
        let reference = match &function.kind {
            hir::ExpressionKind::Name { reference, .. }
            | hir::ExpressionKind::Projection { reference, .. } => *reference,
            _ => {
                self.structure(function.span, "spawn_target_must_be_direct_task", None);
                return (Some(task), Some(scope), None);
            }
        };
        let target = match self.typed.resolved().reference(self.module, reference) {
            Some(ReferenceTarget::Definition(target)) => target.clone(),
            _ => {
                self.structure(function.span, "spawn_target_must_resolve_to_task", None);
                return (Some(task), Some(scope), None);
            }
        };
        if !self
            .typed
            .resolved()
            .definition(&target)
            .is_some_and(|info| info.kind == DefinitionKind::Task)
        {
            self.structure(
                function.span,
                "spawn_target_is_not_task",
                Some(target.to_string()),
            );
            return (Some(task), Some(scope), None);
        }
        (Some(task), Some(scope), Some(target))
    }

    fn visit_task_call_arguments(
        &mut self,
        call: &hir::Expression,
        future: &BTreeSet<BindingKey>,
        state: &mut FlowState,
    ) {
        if let hir::ExpressionKind::Application { arguments, .. } = &call.kind {
            for argument in arguments {
                self.visit_expression(argument, future, state, true);
            }
        }
    }

    fn record_spawn(
        &mut self,
        call: &hir::Expression,
        span: Span,
        operator_span: Span,
        pattern_span: Option<Span>,
        syntax: CheckedTaskSpawnSyntax,
        target: Option<DefinitionId>,
    ) {
        let Some(target) = target else { return };
        let (task, scope) = self.spawn_by_expression[&call.id];
        let target_span = match &call.kind {
            hir::ExpressionKind::Application { function, .. } => function.span,
            _ => call.span,
        };
        self.spawns.push(CheckedTaskSpawn {
            task,
            scope,
            parent: TaskId::new(1),
            target,
            call: ExpressionKey::new(self.module, call.id),
            cancellation: CancellationTokenId::new(task.get()),
            cleanup: CleanupRegionId::new(task.get()),
            span,
            target_span,
            call_span: call.span,
            syntax,
            operator_span,
            pattern_span,
        });
    }

    fn consume_handle(
        &mut self,
        expression: &hir::Expression,
        handle: &hir::Expression,
        future: &BTreeSet<BindingKey>,
        state: &mut FlowState,
    ) {
        let hir::ExpressionKind::Name { reference, name } = &handle.kind else {
            self.handle_error(handle.span, "await_requires_direct_handle", None, None);
            return;
        };
        let Some(ReferenceTarget::Binding(binding)) =
            self.typed.resolved().reference(self.module, *reference)
        else {
            self.handle_error(
                handle.span,
                "await_requires_task_handle",
                Some(name.normalized.clone()),
                None,
            );
            return;
        };
        let Some(existing) = state.handles.get(binding).copied() else {
            self.handle_error(
                handle.span,
                "await_requires_task_handle",
                Some(name.normalized.clone()),
                None,
            );
            return;
        };
        let current_scope = self
            .enclosing_scope(expression.id)
            .unwrap_or(ScopeId::new(0));
        if existing.scope != current_scope {
            self.handle_error(
                handle.span,
                "cross_scope_handle_observation",
                Some(name.normalized.clone()),
                Some(current_scope.get()),
            );
            return;
        }
        if existing.consumed {
            self.handle_error(
                handle.span,
                "handle_observed_more_than_once",
                Some(name.normalized.clone()),
                Some(current_scope.get()),
            );
            return;
        }
        if let Some(use_state) = state.handles.get_mut(binding) {
            use_state.consumed = true;
        }
        self.record_suspension(
            expression.id,
            expression.span,
            current_scope,
            existing.task,
            Some(*binding),
            future,
            state,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn record_suspension(
        &mut self,
        expression: hir::ExpressionId,
        span: Span,
        scope: ScopeId,
        task: TaskId,
        awaited: Option<BindingKey>,
        future: &BTreeSet<BindingKey>,
        state: &FlowState,
    ) {
        let suspension = self.suspension_by_expression[&expression];
        let mut live = Vec::new();
        for binding in state.bindings.intersection(future) {
            if Some(*binding) == awaited {
                continue;
            }
            let Some(info) = self.typed.resolved().bindings().get(binding) else {
                continue;
            };
            if info.mutable {
                self.suspension_error(
                    span,
                    "mutable_binding_crosses_suspension",
                    Some(info.name.clone()),
                    suspension,
                );
            }
            if self.resume_bindings.contains(binding) {
                self.suspension_error(
                    span,
                    "handler_continuation_crosses_suspension",
                    Some(info.name.clone()),
                    suspension,
                );
            }
            if let Some(handle) = state.handles.get(binding).filter(|handle| !handle.consumed) {
                if handle.scope == scope {
                    // DEC-0266 keeps this linear handle in the runtime scope
                    // registry. It is deliberately absent from the typed
                    // suspension frame below.
                    continue;
                }
                self.suspension_error(
                    span,
                    "cross_scope_task_handle_crosses_suspension",
                    Some(info.name.clone()),
                    suspension,
                );
            }
            if let Some(value_type) = self.typed.binding_type(*binding) {
                live.push(SuspensionLiveBinding {
                    binding: *binding,
                    value_type,
                });
            }
        }
        live.sort_by_key(|binding| {
            (
                binding.binding.module().get(),
                binding.binding.local().get(),
            )
        });
        self.suspensions.push(CheckedTaskSuspension {
            id: suspension,
            scope,
            awaited_task: task,
            continuation: ExpressionKey::new(self.module, expression),
            live: live.into_boxed_slice(),
            span,
        });
    }

    fn visit_branches<'b>(
        &mut self,
        branches: impl IntoIterator<Item = &'b hir::Expression>,
        future: &BTreeSet<BindingKey>,
        state: &mut FlowState,
    ) {
        let baseline = state.clone();
        let mut outcomes = Vec::new();
        for branch in branches {
            let mut branch_state = baseline.clone();
            self.visit_expression(branch, future, &mut branch_state, false);
            for (binding, handle) in &branch_state.handles {
                if !baseline.handles.contains_key(binding) && !handle.consumed {
                    self.handle_error(
                        branch.span,
                        "branch_local_handle_not_observed",
                        self.binding_name(*binding),
                        Some(handle.scope.get()),
                    );
                }
            }
            outcomes.push(branch_state);
        }
        if let Some(first) = outcomes.first() {
            for (binding, baseline_handle) in &baseline.handles {
                let consumed = first
                    .handles
                    .get(binding)
                    .map_or(baseline_handle.consumed, |handle| handle.consumed);
                if outcomes.iter().any(|outcome| {
                    outcome
                        .handles
                        .get(binding)
                        .map_or(baseline_handle.consumed, |handle| handle.consumed)
                        != consumed
                }) {
                    self.handle_error(
                        self.typed.resolved().bindings()[binding].span,
                        "handle_observation_differs_across_paths",
                        self.binding_name(*binding),
                        Some(baseline_handle.scope.get()),
                    );
                } else if let Some(handle) = state.handles.get_mut(binding) {
                    handle.consumed = consumed;
                }
            }
        }
    }

    fn reject_handle_reference(
        &mut self,
        reference: hir::ReferenceId,
        span: Span,
        state: &FlowState,
    ) {
        let Some(ReferenceTarget::Binding(binding)) =
            self.typed.resolved().reference(self.module, reference)
        else {
            return;
        };
        if state.handles.contains_key(binding) {
            self.handle_error(
                span,
                "task_handle_is_not_first_class",
                self.binding_name(*binding),
                state.handles.get(binding).map(|handle| handle.scope.get()),
            );
        }
    }

    fn collect_resume_bindings(&mut self, expression: &hir::Expression) {
        walk_expression(expression, &mut |value| {
            if let hir::ExpressionKind::Handle { clauses, .. } = &value.kind {
                for clause in clauses {
                    if let Some(resume) = &clause.resume {
                        self.resume_bindings
                            .insert(BindingKey::new(self.module, resume.id));
                    }
                }
            }
        });
    }

    fn enclosing_scope(&self, expression: hir::ExpressionId) -> Option<ScopeId> {
        self.expression_scope.get(&expression).copied()
    }

    fn binding_name(&self, binding: BindingKey) -> Option<String> {
        self.typed
            .resolved()
            .bindings()
            .get(&binding)
            .map(|info| info.name.clone())
    }

    fn structure(&mut self, span: Span, reason: &'static str, target: Option<String>) {
        self.failures.push(TaskCheckFailure {
            source_name: self.source_name.to_owned(),
            span,
            kind: TaskCheckFailureKind::Structure { reason, target },
        });
    }

    fn handle_error(
        &mut self,
        span: Span,
        reason: &'static str,
        handle: Option<String>,
        scope: Option<u32>,
    ) {
        self.failures.push(TaskCheckFailure {
            source_name: self.source_name.to_owned(),
            span,
            kind: TaskCheckFailureKind::Handle {
                reason,
                handle,
                scope,
            },
        });
    }

    fn suspension_error(
        &mut self,
        span: Span,
        reason: &'static str,
        binding: Option<String>,
        suspension: SuspensionPointId,
    ) {
        self.failures.push(TaskCheckFailure {
            source_name: self.source_name.to_owned(),
            span,
            kind: TaskCheckFailureKind::Suspension {
                reason,
                binding,
                suspension: suspension.get(),
            },
        });
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ReturnFlow {
    may_fall_through: bool,
    may_return: bool,
}

fn validate_task_returns(expression: &hir::Expression) -> Vec<(Span, &'static str)> {
    let mut failures = Vec::new();
    let hir::ExpressionKind::TaskScope { body, .. } = &expression.kind else {
        return vec![(expression.span, "missing_outer_scope")];
    };
    let flow = task_scope_return_flow(body, &mut failures);
    if flow.may_fall_through || !flow.may_return {
        failures.push((expression.span, "missing_final_return"));
    }
    failures
}

fn task_scope_return_flow(
    expression: &hir::Expression,
    failures: &mut Vec<(Span, &'static str)>,
) -> ReturnFlow {
    match &expression.kind {
        hir::ExpressionKind::TaskReturn { value, .. } => {
            reject_nested_returns(value, failures);
            ReturnFlow {
                may_fall_through: false,
                may_return: true,
            }
        }
        hir::ExpressionKind::TaskScope { body, .. } => {
            let nested = task_scope_return_flow(body, failures);
            if nested.may_fall_through || !nested.may_return {
                failures.push((expression.span, "missing_final_return"));
            }
            ReturnFlow {
                may_fall_through: true,
                may_return: false,
            }
        }
        hir::ExpressionKind::Sequence(elements) => {
            let mut flow = ReturnFlow {
                may_fall_through: true,
                may_return: false,
            };
            for element in elements {
                if !flow.may_fall_through {
                    failures.push((sequence_element_span(element), "non_final_return"));
                    continue;
                }
                let next = match element {
                    hir::SequenceElement::Let(binding) => {
                        reject_nested_returns(&binding.value, failures);
                        ReturnFlow {
                            may_fall_through: true,
                            may_return: false,
                        }
                    }
                    hir::SequenceElement::LetAwait(binding) => {
                        reject_nested_returns(&binding.call, failures);
                        ReturnFlow {
                            may_fall_through: true,
                            may_return: false,
                        }
                    }
                    hir::SequenceElement::Expression(value) => {
                        task_scope_return_flow(value, failures)
                    }
                };
                flow = ReturnFlow {
                    may_fall_through: flow.may_fall_through && next.may_fall_through,
                    may_return: flow.may_return || next.may_return,
                };
            }
            flow
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            reject_nested_returns(condition, failures);
            let then_flow = task_scope_return_flow(then_branch, failures);
            let else_flow = task_scope_return_flow(else_branch, failures);
            ReturnFlow {
                may_fall_through: then_flow.may_fall_through || else_flow.may_fall_through,
                may_return: then_flow.may_return || else_flow.may_return,
            }
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            reject_nested_returns(scrutinee, failures);
            let mut flow = ReturnFlow {
                may_fall_through: cases.is_empty(),
                may_return: false,
            };
            for case in cases {
                if let Some(guard) = &case.guard {
                    reject_nested_returns(guard, failures);
                }
                let case_flow = task_scope_return_flow(&case.body, failures);
                flow.may_fall_through |= case_flow.may_fall_through;
                flow.may_return |= case_flow.may_return;
            }
            flow
        }
        _ => {
            reject_nested_returns(expression, failures);
            ReturnFlow {
                may_fall_through: true,
                may_return: false,
            }
        }
    }
}

fn reject_nested_returns(expression: &hir::Expression, failures: &mut Vec<(Span, &'static str)>) {
    walk_expression(expression, &mut |value| {
        if matches!(value.kind, hir::ExpressionKind::TaskReturn { .. }) {
            failures.push((value.span, "return_in_value_position"));
        }
    });
}

fn sequence_element_span(element: &hir::SequenceElement) -> Span {
    match element {
        hir::SequenceElement::Let(binding) => binding.span,
        hir::SequenceElement::LetAwait(binding) => binding.span,
        hir::SequenceElement::Expression(value) => value.span,
    }
}

fn collect_pattern_bindings(
    module: ModuleId,
    pattern: &hir::Pattern,
    output: &mut BTreeSet<BindingKey>,
) {
    match &pattern.kind {
        hir::PatternKind::Binding { id, .. } => {
            output.insert(BindingKey::new(module, *id));
        }
        hir::PatternKind::Tuple(values) => {
            for value in values {
                collect_pattern_bindings(module, value, output);
            }
        }
        hir::PatternKind::Record(fields) => {
            for field in fields {
                collect_pattern_bindings(module, &field.pattern, output);
            }
        }
        hir::PatternKind::Constructor { arguments, .. } => {
            for argument in arguments {
                collect_pattern_bindings(module, argument, output);
            }
        }
        hir::PatternKind::Wildcard | hir::PatternKind::Unit | hir::PatternKind::Literal(_) => {}
    }
}

fn collect_element_references(
    module: ModuleId,
    element: &hir::SequenceElement,
    typed: &TypedProgram,
    output: &mut BTreeSet<BindingKey>,
) {
    match element {
        hir::SequenceElement::Let(binding) => {
            collect_expression_references(module, &binding.value, typed, output)
        }
        hir::SequenceElement::LetAwait(binding) => {
            collect_expression_references(module, &binding.call, typed, output)
        }
        hir::SequenceElement::Expression(value) => {
            collect_expression_references(module, value, typed, output)
        }
    }
}

fn collect_expression_references(
    module: ModuleId,
    expression: &hir::Expression,
    typed: &TypedProgram,
    output: &mut BTreeSet<BindingKey>,
) {
    walk_expression(expression, &mut |value| match &value.kind {
        hir::ExpressionKind::Name { reference, .. }
        | hir::ExpressionKind::Projection { reference, .. } => {
            if let Some(ReferenceTarget::Binding(binding)) =
                typed.resolved().reference(module, *reference)
            {
                output.insert(*binding);
            }
        }
        hir::ExpressionKind::Assignment { place, .. } => {
            if let Some(ReferenceTarget::Binding(binding)) =
                typed.resolved().reference(module, place.root_reference)
            {
                output.insert(*binding);
            }
        }
        _ => {}
    });
}

fn walk_expression(expression: &hir::Expression, visit: &mut impl FnMut(&hir::Expression)) {
    visit(expression);
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => walk_expression(&binding.value, visit),
                    hir::SequenceElement::LetAwait(binding) => {
                        walk_expression(&binding.call, visit)
                    }
                    hir::SequenceElement::Expression(value) => walk_expression(value, visit),
                }
            }
        }
        hir::ExpressionKind::TaskScope { body, .. } => walk_expression(body, visit),
        hir::ExpressionKind::TaskSpawn { call, .. } => walk_expression(call, visit),
        hir::ExpressionKind::TaskAwait { handle, .. } => walk_expression(handle, visit),
        hir::ExpressionKind::TaskReturn { value, .. }
        | hir::ExpressionKind::Assignment { value, .. } => walk_expression(value, visit),
        hir::ExpressionKind::Handle { body, clauses } => {
            walk_expression(body, visit);
            for clause in clauses {
                walk_expression(&clause.body, visit);
            }
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            walk_expression(condition, visit);
            walk_expression(then_branch, visit);
            walk_expression(else_branch, visit);
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            walk_expression(scrutinee, visit);
            for case in cases {
                if let Some(guard) = &case.guard {
                    walk_expression(guard, visit);
                }
                walk_expression(&case.body, visit);
            }
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            walk_expression(function, visit);
            for argument in arguments {
                walk_expression(argument, visit);
            }
        }
        hir::ExpressionKind::Projection { target, .. }
        | hir::ExpressionKind::Unary {
            operand: target, ..
        } => walk_expression(target, visit),
        hir::ExpressionKind::Binary { left, right, .. } => {
            walk_expression(left, visit);
            walk_expression(right, visit);
        }
        hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
            for value in values {
                walk_expression(value, visit);
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                walk_expression(&field.value, visit);
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            walk_expression(base, visit);
            for field in fields {
                walk_expression(&field.value, visit);
            }
        }
        hir::ExpressionKind::Name { .. }
        | hir::ExpressionKind::Literal(_)
        | hir::ExpressionKind::Unit => {}
    }
}

fn detect_spawn_cycles(
    cores: &BTreeMap<DefinitionId, CheckedTaskCore>,
    typed: &TypedProgram,
    failures: &mut Vec<TaskCheckFailure>,
) {
    fn visit(
        current: &DefinitionId,
        cores: &BTreeMap<DefinitionId, CheckedTaskCore>,
        active: &mut BTreeSet<DefinitionId>,
        done: &mut BTreeSet<DefinitionId>,
    ) -> Option<DefinitionId> {
        if done.contains(current) {
            return None;
        }
        if !active.insert(current.clone()) {
            return Some(current.clone());
        }
        if let Some(core) = cores.get(current) {
            for spawn in core.spawns() {
                if let Some(cycle) = visit(spawn.target(), cores, active, done) {
                    return Some(cycle);
                }
            }
        }
        active.remove(current);
        done.insert(current.clone());
        None
    }

    let mut active = BTreeSet::new();
    let mut done = BTreeSet::new();
    for definition in cores.keys() {
        if let Some(cycle) = visit(definition, cores, &mut active, &mut done) {
            let info = typed.resolved().definition(&cycle);
            failures.push(TaskCheckFailure {
                source_name: info
                    .and_then(|info| info.source_name.clone())
                    .unwrap_or_else(|| "<unknown>".to_owned()),
                span: info
                    .and_then(|info| info.span)
                    .unwrap_or(cores[&cycle].source_span()),
                kind: TaskCheckFailureKind::Structure {
                    reason: "recursive_spawn_chain",
                    target: Some(cycle.to_string()),
                },
            });
            break;
        }
    }
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_text(bytes: &mut Vec<u8>, value: &str) {
    push_u32(bytes, value.len() as u32);
    bytes.extend_from_slice(value.as_bytes());
}
