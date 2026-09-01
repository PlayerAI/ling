//! Publish-disabled local Supervisor containment authorized by DEC-0276.
//!
//! This module owns one fixed child set over the real checked-Core Actor
//! runtime. It deliberately has no public re-export, source operation,
//! restart path, serialized protocol, or backend execution surface.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use ling_concurrency::{ActorId, ActorTypeId};
use ling_effects::CheckedProgram;
use ling_resolve::{DefinitionId, ExpressionKey};
use ling_source::Span;

use super::actor_runtime::{
    ActorFault, ActorFaultPhase, ActorInstanceSnapshot, ActorInstanceState, ActorRuntime,
    ActorRuntimeError, ActorRuntimeEvent, ActorRuntimeEventKind, ActorRuntimeId,
    ActorRuntimeLimits, ActorRuntimeMetrics, ActorRuntimeState, ActorSendError,
    ActorShutdownReason, ActorShutdownResult, ActorSpawnError, ActorTurnResult, ActorValue,
    LocalActorRef,
};
use super::task_local_scheduler::LocalTaskControl;
use super::{Console, RuntimeFaultKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SupervisorState {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SupervisorChildState {
    Starting,
    Running,
    Contained,
    Stopped,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ChildFaultReport {
    runtime: ActorRuntimeId,
    actor: ActorId,
    actor_type: ActorTypeId,
    definition: DefinitionId,
    expression: ExpressionKey,
    phase: ActorFaultPhase,
    span: Span,
    category: &'static str,
    discarded_messages: usize,
    cleanup_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum SupervisorTurnResult {
    Completed {
        actor: ActorId,
        state: ActorValue,
        remaining_messages: usize,
    },
    Contained {
        actor: ActorId,
        report: ChildFaultReport,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SupervisorStopResult {
    Stopped {
        actors: usize,
        discarded_messages: usize,
    },
    AlreadyStopped,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum SupervisorStartErrorKind {
    InvalidChildSet {
        reason: &'static str,
    },
    UnknownActorDefinition {
        definition: DefinitionId,
    },
    DuplicateActorType {
        actor_type: ActorTypeId,
    },
    ResourceExhausted {
        resource: &'static str,
        required: usize,
        limit: usize,
    },
    Runtime(ActorRuntimeError),
    Spawn(ActorSpawnError),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SupervisorConstructionEvidence {
    supervisor_state: SupervisorState,
    runtime_state: Option<ActorRuntimeState>,
    metrics: Option<ActorRuntimeMetrics>,
    children: Vec<ActorInstanceSnapshot>,
    events: Vec<ActorRuntimeEvent>,
    owner_cancelled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SupervisorStartError {
    kind: Box<SupervisorStartErrorKind>,
    evidence: Box<SupervisorConstructionEvidence>,
}

impl SupervisorStartError {
    fn without_runtime(kind: SupervisorStartErrorKind, owner_control: &LocalTaskControl) -> Self {
        owner_control.cancel();
        Self {
            kind: Box::new(kind),
            evidence: Box::new(SupervisorConstructionEvidence {
                supervisor_state: SupervisorState::Failed,
                runtime_state: None,
                metrics: None,
                children: Vec::new(),
                events: Vec::new(),
                owner_cancelled: owner_control.is_cancelled(),
            }),
        }
    }

    #[must_use]
    pub(super) const fn kind(&self) -> &SupervisorStartErrorKind {
        &self.kind
    }

    #[must_use]
    pub(super) const fn evidence(&self) -> &SupervisorConstructionEvidence {
        &self.evidence
    }
}

impl fmt::Display for SupervisorStartError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "local Supervisor construction failed: {:?}",
            self.kind
        )
    }
}

impl Error for SupervisorStartError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum SupervisorError {
    NotRunning { state: SupervisorState },
    Runtime(ActorRuntimeError),
    InvalidChildFaultReport { reason: &'static str },
}

impl fmt::Display for SupervisorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRunning { state } => {
                write!(formatter, "local Supervisor is not running: {state:?}")
            }
            Self::Runtime(error) => error.fmt(formatter),
            Self::InvalidChildFaultReport { reason } => {
                write!(formatter, "invalid child Fault report: {reason}")
            }
        }
    }
}

impl Error for SupervisorError {}

impl From<ActorRuntimeError> for SupervisorError {
    fn from(error: ActorRuntimeError) -> Self {
        Self::Runtime(error)
    }
}

#[derive(Clone, Debug)]
struct SupervisorChildSlot {
    definition: DefinitionId,
    reference: Option<LocalActorRef>,
    state: SupervisorChildState,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SupervisorChildSnapshot {
    actor_type: ActorTypeId,
    definition: DefinitionId,
    actor: ActorId,
    state: SupervisorChildState,
    actor_state: ActorInstanceState,
    queued_messages: usize,
    cleanup_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SupervisorSnapshot {
    state: SupervisorState,
    children: Vec<SupervisorChildSnapshot>,
}

/// One run-owned, fixed-child, non-nested `ContainOne` Supervisor.
pub(super) struct LocalActorSupervisor<'checked> {
    state: SupervisorState,
    runtime: ActorRuntime<'checked>,
    owner_control: LocalTaskControl,
    slots: BTreeMap<ActorTypeId, SupervisorChildSlot>,
}

impl<'checked> LocalActorSupervisor<'checked> {
    pub(super) fn start(
        checked: &'checked CheckedProgram,
        runtime_id: ActorRuntimeId,
        limits: ActorRuntimeLimits,
        owner_control: &LocalTaskControl,
        definitions: &[DefinitionId],
        console: &mut dyn Console,
    ) -> Result<Self, SupervisorStartError> {
        if definitions.is_empty() {
            return Err(SupervisorStartError::without_runtime(
                SupervisorStartErrorKind::InvalidChildSet {
                    reason: "Supervisor child set must be non-empty",
                },
                owner_control,
            ));
        }

        let mut ordered = BTreeMap::<ActorTypeId, DefinitionId>::new();
        for definition in definitions {
            let Some(core) = checked.actor_core(definition) else {
                return Err(SupervisorStartError::without_runtime(
                    SupervisorStartErrorKind::UnknownActorDefinition {
                        definition: definition.clone(),
                    },
                    owner_control,
                ));
            };
            if ordered
                .insert(core.actor_type(), definition.clone())
                .is_some()
            {
                return Err(SupervisorStartError::without_runtime(
                    SupervisorStartErrorKind::DuplicateActorType {
                        actor_type: core.actor_type(),
                    },
                    owner_control,
                ));
            }
        }

        let mut runtime = ActorRuntime::new_supervised(checked, runtime_id, limits, owner_control)
            .map_err(|error| {
                SupervisorStartError::without_runtime(
                    SupervisorStartErrorKind::Runtime(error),
                    owner_control,
                )
            })?;

        if let Some(kind) = construction_limit_error(limits, ordered.len()) {
            return Err(abort_construction(
                kind,
                &mut runtime,
                owner_control,
                &BTreeMap::new(),
            ));
        }

        let mut slots = ordered
            .iter()
            .map(|(actor_type, definition)| {
                (
                    *actor_type,
                    SupervisorChildSlot {
                        definition: definition.clone(),
                        reference: None,
                        state: SupervisorChildState::Starting,
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();

        for (actor_type, definition) in ordered {
            match runtime.spawn(&definition, console) {
                Ok(reference) => {
                    let slot = slots
                        .get_mut(&actor_type)
                        .expect("canonical child slot was preallocated");
                    slot.reference = Some(reference);
                    slot.state = SupervisorChildState::Running;
                }
                Err(error) => {
                    return Err(abort_construction(
                        SupervisorStartErrorKind::Spawn(error),
                        &mut runtime,
                        owner_control,
                        &slots,
                    ));
                }
            }
        }

        let mut supervisor = Self {
            state: SupervisorState::Starting,
            runtime,
            owner_control: owner_control.clone(),
            slots,
        };
        supervisor.state = SupervisorState::Running;
        Ok(supervisor)
    }

    #[must_use]
    pub(super) const fn state(&self) -> SupervisorState {
        self.state
    }

    #[must_use]
    pub(super) fn ready(&self) -> Vec<ActorId> {
        if self.state == SupervisorState::Running {
            self.runtime.ready()
        } else {
            Vec::new()
        }
    }

    #[must_use]
    pub(super) fn child_reference(&self, actor_type: ActorTypeId) -> Option<&LocalActorRef> {
        self.slots
            .get(&actor_type)
            .and_then(|slot| slot.reference.as_ref())
    }

    pub(super) fn send(
        &mut self,
        reference: &LocalActorRef,
        sender: super::actor_runtime::ActorSenderId,
        payload: ActorValue,
    ) -> Result<(), ActorSendError> {
        let result = self.runtime.send(reference, sender, payload);
        self.synchronize_runtime_terminal_state();
        result
    }

    pub(super) fn step(
        &mut self,
        actor: ActorId,
        console: &mut dyn Console,
    ) -> Result<SupervisorTurnResult, SupervisorError> {
        self.observe_owner_cancellation()?;
        self.ensure_running()?;
        match self.runtime.step(actor, console)? {
            ActorTurnResult::Completed {
                actor,
                state,
                remaining_messages,
            } => Ok(SupervisorTurnResult::Completed {
                actor,
                state,
                remaining_messages,
            }),
            ActorTurnResult::Faulted {
                actor,
                fault,
                discarded_messages,
                ..
            } => {
                let report = match self.report_from_fault(&fault, discarded_messages) {
                    Ok(report) => report,
                    Err(reason) => return Err(self.reject_fault_report(reason)),
                };
                self.acknowledge_fault(&report)?;
                Ok(SupervisorTurnResult::Contained { actor, report })
            }
        }
    }

    pub(super) fn stop(&mut self) -> Result<SupervisorStopResult, SupervisorError> {
        if matches!(
            self.state,
            SupervisorState::Stopped | SupervisorState::Failed
        ) {
            return Ok(SupervisorStopResult::AlreadyStopped);
        }
        self.ensure_running()?;
        let result = self.runtime.shutdown(ActorShutdownReason::Explicit)?;
        self.state = SupervisorState::Stopping;
        self.mark_running_slots_stopped();
        self.state = SupervisorState::Stopped;
        match result {
            ActorShutdownResult::Stopped {
                actors,
                discarded_messages,
            } => Ok(SupervisorStopResult::Stopped {
                actors,
                discarded_messages,
            }),
            ActorShutdownResult::AlreadyStopped => {
                Err(self.reject_fault_report("running Supervisor owned a terminal Actor runtime"))
            }
        }
    }

    pub(super) fn observe_owner_cancellation(&mut self) -> Result<bool, SupervisorError> {
        if self.state != SupervisorState::Running {
            return Ok(false);
        }
        let observed = self.runtime.observe_owner_cancellation()?;
        if observed {
            self.state = SupervisorState::Stopping;
            self.mark_running_slots_stopped();
            self.state = SupervisorState::Stopped;
        }
        Ok(observed)
    }

    #[must_use]
    pub(super) fn snapshot(&self) -> SupervisorSnapshot {
        let children = self
            .slots
            .iter()
            .map(|(actor_type, slot)| {
                let reference = slot
                    .reference
                    .as_ref()
                    .expect("published Supervisor child has a local reference");
                let actor = self
                    .runtime
                    .snapshot(reference.actor())
                    .expect("published Supervisor child has a terminal or live Actor record");
                SupervisorChildSnapshot {
                    actor_type: *actor_type,
                    definition: slot.definition.clone(),
                    actor: reference.actor(),
                    state: slot.state,
                    actor_state: actor.lifecycle(),
                    queued_messages: actor.queued_messages(),
                    cleanup_count: actor.cleanup_count(),
                }
            })
            .collect();
        SupervisorSnapshot {
            state: self.state,
            children,
        }
    }

    fn ensure_running(&self) -> Result<(), SupervisorError> {
        if self.state == SupervisorState::Running {
            Ok(())
        } else {
            Err(SupervisorError::NotRunning { state: self.state })
        }
    }

    fn report_from_fault(
        &self,
        fault: &ActorFault,
        discarded_messages: usize,
    ) -> Result<ChildFaultReport, &'static str> {
        let snapshot = self
            .runtime
            .snapshot(fault.actor())
            .ok_or("faulting child has no retained Actor record")?;
        Ok(ChildFaultReport {
            runtime: fault.runtime(),
            actor: fault.actor(),
            actor_type: fault.actor_type(),
            definition: fault.definition().clone(),
            expression: fault.expression(),
            phase: fault.phase(),
            span: fault.cause().span,
            category: runtime_fault_category(&fault.cause().kind),
            discarded_messages,
            cleanup_count: snapshot.cleanup_count(),
        })
    }

    fn acknowledge_fault(&mut self, report: &ChildFaultReport) -> Result<(), SupervisorError> {
        if let Err(reason) = self.validate_fault_report(report) {
            return Err(self.reject_fault_report(reason));
        }
        self.slots
            .get_mut(&report.actor_type)
            .expect("validated child slot remains registered")
            .state = SupervisorChildState::Contained;
        Ok(())
    }

    fn validate_fault_report(&self, report: &ChildFaultReport) -> Result<(), &'static str> {
        if self.state != SupervisorState::Running {
            return Err("Supervisor is not running");
        }
        if self.runtime.id() != report.runtime {
            return Err("runtime identity mismatch");
        }
        if report.phase != ActorFaultPhase::Turn {
            return Err("initializer Fault cannot be contained after publication");
        }
        let slot = self
            .slots
            .get(&report.actor_type)
            .ok_or("unknown child slot")?;
        if slot.state != SupervisorChildState::Running {
            return Err("child slot is stale or already acknowledged");
        }
        let reference = slot
            .reference
            .as_ref()
            .ok_or("child slot has no Actor incarnation")?;
        if reference.actor() != report.actor || reference.actor_type() != report.actor_type {
            return Err("child incarnation or type mismatch");
        }
        if slot.definition != report.definition {
            return Err("child definition mismatch");
        }
        let snapshot = self
            .runtime
            .snapshot(report.actor)
            .ok_or("faulting child record is absent")?;
        if snapshot.lifecycle() != ActorInstanceState::Failed
            || snapshot.state().is_some()
            || snapshot.queued_messages() != 0
            || snapshot.cleanup_count() != report.cleanup_count
            || report.cleanup_count != 1
        {
            return Err("faulting child terminal cleanup evidence is inconsistent");
        }
        let fault = snapshot
            .fault()
            .ok_or("faulting child did not retain one Fault")?;
        if fault.runtime() != report.runtime
            || fault.actor() != report.actor
            || fault.actor_type() != report.actor_type
            || fault.definition() != &report.definition
            || fault.expression() != report.expression
            || fault.phase() != report.phase
            || fault.cause().span != report.span
            || runtime_fault_category(&fault.cause().kind) != report.category
        {
            return Err("retained child Fault disagrees with its report");
        }
        let mut fault_events =
            self.runtime
                .events()
                .into_iter()
                .filter_map(|event| match event.kind() {
                    ActorRuntimeEventKind::ActorFaulted {
                        actor,
                        discarded_messages,
                    } if *actor == report.actor => Some(*discarded_messages),
                    _ => None,
                });
        if fault_events.next() != Some(report.discarded_messages) || fault_events.next().is_some() {
            return Err("child Fault event count or discard evidence is inconsistent");
        }
        Ok(())
    }

    fn reject_fault_report(&mut self, reason: &'static str) -> SupervisorError {
        self.state = SupervisorState::Failed;
        self.owner_control.cancel();
        if self.runtime.state() == ActorRuntimeState::Running {
            self.runtime.fail_unacknowledged_supervised_fault();
        }
        self.mark_running_slots_stopped();
        SupervisorError::InvalidChildFaultReport { reason }
    }

    fn mark_running_slots_stopped(&mut self) {
        for slot in self.slots.values_mut() {
            if slot.state == SupervisorChildState::Running {
                slot.state = SupervisorChildState::Stopped;
            }
        }
    }

    fn synchronize_runtime_terminal_state(&mut self) {
        match self.runtime.state() {
            ActorRuntimeState::Stopped if self.state == SupervisorState::Running => {
                self.state = SupervisorState::Stopping;
                self.mark_running_slots_stopped();
                self.state = SupervisorState::Stopped;
            }
            ActorRuntimeState::Failed if self.state == SupervisorState::Running => {
                self.state = SupervisorState::Failed;
                self.mark_running_slots_stopped();
            }
            _ => {}
        }
    }
}

fn construction_limit_error(
    limits: ActorRuntimeLimits,
    children: usize,
) -> Option<SupervisorStartErrorKind> {
    let checks = [
        ("created_actors", children, limits.max_created_actors()),
        ("live_actors", children, limits.max_live_actors()),
        ("faults", children, limits.max_faults()),
        ("shutdown_work", children, limits.max_shutdown_work()),
    ];
    for (resource, required, limit) in checks {
        if required > limit {
            return Some(SupervisorStartErrorKind::ResourceExhausted {
                resource,
                required,
                limit,
            });
        }
    }
    let required_commands = children.checked_add(1)?;
    if required_commands > limits.max_commands() {
        return Some(SupervisorStartErrorKind::ResourceExhausted {
            resource: "commands",
            required: required_commands,
            limit: limits.max_commands(),
        });
    }
    let required_events = children.checked_mul(2)?.checked_add(1)?;
    if required_events > limits.max_events() {
        return Some(SupervisorStartErrorKind::ResourceExhausted {
            resource: "events",
            required: required_events,
            limit: limits.max_events(),
        });
    }
    None
}

fn abort_construction(
    kind: SupervisorStartErrorKind,
    runtime: &mut ActorRuntime<'_>,
    owner_control: &LocalTaskControl,
    slots: &BTreeMap<ActorTypeId, SupervisorChildSlot>,
) -> SupervisorStartError {
    owner_control.cancel();
    if runtime.state() == ActorRuntimeState::Running {
        runtime
            .shutdown(ActorShutdownReason::OwnerFaulted)
            .expect("Supervisor construction preflight reserved shutdown evidence");
    }
    let children = slots
        .values()
        .filter_map(|slot| slot.reference.as_ref())
        .filter_map(|reference| runtime.snapshot(reference.actor()))
        .collect();
    SupervisorStartError {
        kind: Box::new(kind),
        evidence: Box::new(SupervisorConstructionEvidence {
            supervisor_state: SupervisorState::Failed,
            runtime_state: Some(runtime.state()),
            metrics: Some(runtime.metrics()),
            children,
            events: runtime.events(),
            owner_cancelled: owner_control.is_cancelled(),
        }),
    }
}

const fn runtime_fault_category(kind: &RuntimeFaultKind) -> &'static str {
    match kind {
        RuntimeFaultKind::HostCapability { .. } => "host_capability",
        RuntimeFaultKind::InvalidFormatPlaceholderCount { .. } => {
            "invalid_format_placeholder_count"
        }
        RuntimeFaultKind::DivisionByZero => "division_by_zero",
        RuntimeFaultKind::InvalidCheckedCore { .. } => "invalid_checked_core",
        RuntimeFaultKind::HandlerResumeCardinality { .. } => "handler_resume_cardinality",
        RuntimeFaultKind::TaskImplementationBoundary { .. } => "task_implementation_boundary",
        RuntimeFaultKind::TaskResourceLimit { .. } => "task_resource_limit",
        RuntimeFaultKind::TaskDriver { .. } => "task_driver",
        RuntimeFaultKind::TaskFaultAggregate { .. } => "task_fault_aggregate",
    }
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_effects::{CheckedProgram, check};
    use ling_hir::lower as lower_hir;
    use ling_resolve::{DefinitionId, resolve};
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;
    use ling_types::check as check_types;
    use num_bigint::BigInt;

    use super::*;
    use crate::{ActorSendErrorKind, ActorSenderId, MemoryConsole};

    const TWO_ACTORS: &str = concat!(
        "module Main\n\n",
        "actor Counter : Int =\n",
        "    mailbox capacity 4 overflow Reject\n",
        "    state Int = 1\n",
        "    receive state message =\n",
        "        state + message\n\n",
        "actor Divider : Int =\n",
        "    mailbox capacity 4 overflow Reject\n",
        "    state Int = 20\n",
        "    receive state message =\n",
        "        state / message\n\n",
        "let main () = ()\n",
    );

    fn checked(source_name: &str, bytes: Vec<u8>) -> CheckedProgram {
        let source = SourceFile::from_bytes(SourceId::new(0), source_name, bytes)
            .expect("valid test source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = lower_hir(source.name(), &ast).expect("valid HIR");
        let resolved = resolve(vec![hir], "Main").expect("resolved program");
        let typed = check_types(resolved).expect("typed program");
        check(typed).expect("checked program")
    }

    fn actor(checked: &CheckedProgram, name: &str) -> DefinitionId {
        checked
            .actor_cores()
            .keys()
            .find(|definition| {
                checked
                    .typed()
                    .resolved()
                    .definition(definition)
                    .is_some_and(|info| info.name == name)
            })
            .cloned()
            .expect("Actor definition")
    }

    fn actor_type(checked: &CheckedProgram, definition: &DefinitionId) -> ActorTypeId {
        checked
            .actor_core(definition)
            .expect("checked Actor Core")
            .actor_type()
    }

    fn limits() -> ActorRuntimeLimits {
        ActorRuntimeLimits::new(8, 8, 32, 128, 128, 256, 8, 8)
    }

    fn integer(value: i64) -> ActorValue {
        ActorValue::Int(BigInt::from(value))
    }

    fn start_two<'checked>(
        checked: &'checked CheckedProgram,
        control: &LocalTaskControl,
        console: &mut MemoryConsole,
    ) -> (LocalActorSupervisor<'checked>, ActorTypeId, ActorTypeId) {
        let counter = actor(checked, "Counter");
        let divider = actor(checked, "Divider");
        let counter_type = actor_type(checked, &counter);
        let divider_type = actor_type(checked, &divider);
        let supervisor = LocalActorSupervisor::start(
            checked,
            ActorRuntimeId::new(71),
            limits(),
            control,
            &[divider, counter],
            console,
        )
        .expect("fixed child Supervisor starts");
        (supervisor, counter_type, divider_type)
    }

    #[test]
    fn contain_one_fault_seals_only_the_child_and_preserves_the_sibling() {
        let checked = checked("supervisor.ling", TWO_ACTORS.as_bytes().to_vec());
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let (mut supervisor, counter_type, divider_type) =
            start_two(&checked, &control, &mut console);
        let counter = supervisor
            .child_reference(counter_type)
            .expect("Counter child")
            .clone();
        let divider = supervisor
            .child_reference(divider_type)
            .expect("Divider child")
            .clone();
        assert!(counter.actor() < divider.actor());

        supervisor
            .send(&counter, ActorSenderId::new(1), integer(4))
            .expect("sibling message");
        supervisor
            .send(&divider, ActorSenderId::new(2), integer(0))
            .expect("faulting message");
        let SupervisorTurnResult::Contained { actor, report } = supervisor
            .step(divider.actor(), &mut console)
            .expect("Fault is synchronously contained")
        else {
            panic!("division by zero must be contained");
        };
        assert_eq!(actor, divider.actor());
        assert_eq!(report.category, "division_by_zero");
        assert_eq!(report.cleanup_count, 1);
        assert!(!control.is_cancelled());
        assert_eq!(supervisor.state(), SupervisorState::Running);
        assert_eq!(supervisor.runtime.state(), ActorRuntimeState::Running);
        assert_eq!(supervisor.ready(), [counter.actor()]);
        let rejected = supervisor
            .send(&divider, ActorSenderId::new(2), integer(9))
            .expect_err("contained child admission is closed");
        assert_eq!(rejected.kind(), &ActorSendErrorKind::Closed);
        assert_eq!(rejected.into_payload(), integer(9));

        assert_eq!(
            supervisor
                .step(counter.actor(), &mut console)
                .expect("unaffected sibling continues"),
            SupervisorTurnResult::Completed {
                actor: counter.actor(),
                state: integer(5),
                remaining_messages: 0,
            }
        );
        let snapshot = supervisor.snapshot();
        assert_eq!(snapshot.state, SupervisorState::Running);
        assert_eq!(snapshot.children.len(), 2);
        assert_eq!(
            snapshot
                .children
                .iter()
                .find(|child| child.actor_type == divider_type)
                .expect("Divider slot")
                .state,
            SupervisorChildState::Contained
        );
        assert_eq!(
            snapshot
                .children
                .iter()
                .find(|child| child.actor_type == counter_type)
                .expect("Counter slot")
                .state,
            SupervisorChildState::Running
        );
    }

    #[test]
    fn malformed_fault_report_fails_the_supervisor_and_cancels_the_root() {
        let checked = checked("invalid-report.ling", TWO_ACTORS.as_bytes().to_vec());
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let (mut supervisor, counter_type, divider_type) =
            start_two(&checked, &control, &mut console);
        let counter = supervisor
            .child_reference(counter_type)
            .expect("Counter child")
            .clone();
        let divider = supervisor
            .child_reference(divider_type)
            .expect("Divider child")
            .clone();
        supervisor
            .send(&counter, ActorSenderId::new(1), integer(7))
            .expect("sibling message");
        supervisor
            .send(&divider, ActorSenderId::new(2), integer(0))
            .expect("faulting message");
        let ActorTurnResult::Faulted {
            fault,
            discarded_messages,
            ..
        } = supervisor
            .runtime
            .step(divider.actor(), &mut console)
            .expect("raw supervised turn Fault")
        else {
            panic!("division by zero must fault");
        };
        let mut report = supervisor
            .report_from_fault(&fault, discarded_messages)
            .expect("canonical report");
        report.runtime = ActorRuntimeId::new(72);

        assert!(matches!(
            supervisor.acknowledge_fault(&report),
            Err(SupervisorError::InvalidChildFaultReport {
                reason: "runtime identity mismatch"
            })
        ));
        assert!(control.is_cancelled());
        assert_eq!(supervisor.state(), SupervisorState::Failed);
        assert_eq!(supervisor.runtime.state(), ActorRuntimeState::Failed);
        let sibling = supervisor
            .runtime
            .snapshot(counter.actor())
            .expect("sibling terminal record");
        assert_eq!(sibling.lifecycle(), ActorInstanceState::Stopped);
        assert_eq!(sibling.cleanup_count(), 1);
        assert_eq!(sibling.queued_messages(), 0);
    }

    #[test]
    fn every_stale_duplicate_or_inconsistent_report_uses_root_fallback() {
        for mutation in 0..10 {
            let checked = checked(
                &format!("invalid-report-{mutation}.ling"),
                TWO_ACTORS.as_bytes().to_vec(),
            );
            let counter_definition = actor(&checked, "Counter");
            let divider_definition = actor(&checked, "Divider");
            let counter_expression = checked
                .actor_core(&counter_definition)
                .expect("Counter Core")
                .transition_body();
            let control = LocalTaskControl::new();
            let mut console = MemoryConsole::default();
            let (mut supervisor, counter_type, divider_type) =
                start_two(&checked, &control, &mut console);
            let counter = supervisor
                .child_reference(counter_type)
                .expect("Counter child")
                .clone();
            let divider = supervisor
                .child_reference(divider_type)
                .expect("Divider child")
                .clone();
            supervisor
                .send(&divider, ActorSenderId::new(1), integer(0))
                .expect("faulting message");
            let ActorTurnResult::Faulted {
                fault,
                discarded_messages,
                ..
            } = supervisor
                .runtime
                .step(divider.actor(), &mut console)
                .expect("raw supervised turn Fault")
            else {
                panic!("division by zero must fault");
            };
            let mut report = supervisor
                .report_from_fault(&fault, discarded_messages)
                .expect("canonical report");
            match mutation {
                0 => report.runtime = ActorRuntimeId::new(999),
                1 => report.actor = ActorId::new(999),
                2 => report.actor_type = ActorTypeId::new(999),
                3 => report.definition = counter_definition.clone(),
                4 => report.expression = counter_expression,
                5 => report.phase = ActorFaultPhase::Initializer,
                6 => report.category = "invalid_checked_core",
                7 => report.discarded_messages += 1,
                8 => report.cleanup_count = 0,
                9 => supervisor
                    .acknowledge_fault(&report)
                    .expect("first exact report is accepted"),
                _ => unreachable!(),
            }

            assert!(matches!(
                supervisor.acknowledge_fault(&report),
                Err(SupervisorError::InvalidChildFaultReport { .. })
            ));
            assert!(control.is_cancelled(), "mutation {mutation}");
            assert_eq!(supervisor.state(), SupervisorState::Failed);
            assert_eq!(supervisor.runtime.state(), ActorRuntimeState::Failed);
            assert_eq!(
                supervisor
                    .runtime
                    .snapshot(counter.actor())
                    .expect("sibling terminal record")
                    .cleanup_count(),
                1
            );
            assert_eq!(
                supervisor
                    .runtime
                    .snapshot(divider.actor())
                    .expect("faulting terminal record")
                    .cleanup_count(),
                1
            );
            assert_eq!(&divider_definition, fault.definition());
        }
    }

    #[test]
    fn sequential_faults_can_contain_all_children_without_restart_or_double_cleanup() {
        let source = concat!(
            "module Main\n\n",
            "actor First : Int =\n",
            "    mailbox capacity 1 overflow Reject\n",
            "    state Int = 12\n",
            "    receive state message =\n",
            "        state / message\n\n",
            "actor Second : Int =\n",
            "    mailbox capacity 1 overflow Reject\n",
            "    state Int = 18\n",
            "    receive state message =\n",
            "        state / message\n\n",
            "let main () = ()\n",
        );
        let checked = checked("all-contained.ling", source.as_bytes().to_vec());
        let first = actor(&checked, "First");
        let second = actor(&checked, "Second");
        let first_type = actor_type(&checked, &first);
        let second_type = actor_type(&checked, &second);
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let mut supervisor = LocalActorSupervisor::start(
            &checked,
            ActorRuntimeId::new(73),
            limits(),
            &control,
            &[second, first],
            &mut console,
        )
        .expect("two faulting children start");
        let first = supervisor
            .child_reference(first_type)
            .expect("First child")
            .clone();
        let second = supervisor
            .child_reference(second_type)
            .expect("Second child")
            .clone();
        for reference in [&second, &first] {
            supervisor
                .send(reference, ActorSenderId::new(1), integer(0))
                .expect("faulting message");
            assert!(matches!(
                supervisor
                    .step(reference.actor(), &mut console)
                    .expect("one child is contained"),
                SupervisorTurnResult::Contained { .. }
            ));
            assert_eq!(supervisor.state(), SupervisorState::Running);
            assert!(!control.is_cancelled());
        }
        assert!(supervisor.ready().is_empty());
        assert_eq!(supervisor.runtime.metrics().live_actors(), 0);
        assert_eq!(supervisor.runtime.metrics().cleanups(), 2);
        assert!(
            supervisor
                .snapshot()
                .children
                .iter()
                .all(|child| child.state == SupervisorChildState::Contained
                    && child.actor_state == ActorInstanceState::Failed
                    && child.cleanup_count == 1)
        );

        assert_eq!(
            supervisor.stop().expect("all-contained stop"),
            SupervisorStopResult::Stopped {
                actors: 0,
                discarded_messages: 0,
            }
        );
        assert_eq!(supervisor.runtime.metrics().cleanups(), 2);
        assert!(
            supervisor
                .snapshot()
                .children
                .iter()
                .all(|child| child.state == SupervisorChildState::Contained
                    && child.cleanup_count == 1)
        );
    }

    #[test]
    fn partial_child_construction_is_failure_atomic() {
        let source = concat!(
            "module Main\n\n",
            "actor Good : Int =\n",
            "    mailbox capacity 1 overflow Reject\n",
            "    state Int = 1\n",
            "    receive state message =\n",
            "        state\n\n",
            "actor Broken : Int =\n",
            "    mailbox capacity 1 overflow Reject\n",
            "    state Int = 1 / 0\n",
            "    receive state message =\n",
            "        state\n\n",
            "let main () = ()\n",
        );
        let checked = checked("construction-fault.ling", source.as_bytes().to_vec());
        let good = actor(&checked, "Good");
        let broken = actor(&checked, "Broken");
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let error = match LocalActorSupervisor::start(
            &checked,
            ActorRuntimeId::new(81),
            limits(),
            &control,
            &[broken, good],
            &mut console,
        ) {
            Ok(_) => panic!("initializer Fault must prevent Supervisor publication"),
            Err(error) => error,
        };

        assert!(matches!(error.kind(), SupervisorStartErrorKind::Spawn(_)));
        assert_eq!(
            error.evidence().runtime_state,
            Some(ActorRuntimeState::Failed)
        );
        assert_eq!(
            error
                .evidence()
                .metrics
                .expect("runtime metrics")
                .live_actors(),
            0
        );
        assert_eq!(error.evidence().children.len(), 1);
        assert!(error.evidence().owner_cancelled);
        assert!(
            error
                .evidence()
                .children
                .iter()
                .all(|child| child.lifecycle() != ActorInstanceState::Running
                    && child.cleanup_count() == 1)
        );
    }

    #[test]
    fn invalid_child_sets_and_construction_limits_fail_before_publication() {
        let checked = checked("child-set.ling", TWO_ACTORS.as_bytes().to_vec());
        let counter = actor(&checked, "Counter");
        let divider = actor(&checked, "Divider");
        let mut console = MemoryConsole::default();

        let empty_control = LocalTaskControl::new();
        let empty = match LocalActorSupervisor::start(
            &checked,
            ActorRuntimeId::new(91),
            limits(),
            &empty_control,
            &[],
            &mut console,
        ) {
            Ok(_) => panic!("empty child set must be invalid"),
            Err(error) => error,
        };
        assert!(matches!(
            empty.kind(),
            SupervisorStartErrorKind::InvalidChildSet { .. }
        ));
        assert_eq!(empty.evidence().runtime_state, None);
        assert!(empty_control.is_cancelled());

        let duplicate_control = LocalTaskControl::new();
        let duplicate = match LocalActorSupervisor::start(
            &checked,
            ActorRuntimeId::new(92),
            limits(),
            &duplicate_control,
            &[counter.clone(), counter],
            &mut console,
        ) {
            Ok(_) => panic!("duplicate Actor type must be invalid"),
            Err(error) => error,
        };
        assert!(matches!(
            duplicate.kind(),
            SupervisorStartErrorKind::DuplicateActorType { .. }
        ));
        assert!(duplicate_control.is_cancelled());

        let bounded_control = LocalTaskControl::new();
        let bounded = ActorRuntimeLimits::new(2, 2, 4, 2, 8, 16, 2, 2);
        let exhausted = match LocalActorSupervisor::start(
            &checked,
            ActorRuntimeId::new(93),
            bounded,
            &bounded_control,
            &[divider, actor(&checked, "Counter")],
            &mut console,
        ) {
            Ok(_) => panic!("construction and cleanup command budget must be preflighted"),
            Err(error) => error,
        };
        assert!(matches!(
            exhausted.kind(),
            SupervisorStartErrorKind::ResourceExhausted {
                resource: "commands",
                required: 3,
                limit: 2
            }
        ));
        assert_eq!(
            exhausted.evidence().runtime_state,
            Some(ActorRuntimeState::Stopped)
        );
        assert_eq!(
            exhausted
                .evidence()
                .metrics
                .expect("runtime metrics")
                .created_actors(),
            0
        );
    }

    #[test]
    fn stop_and_owner_cancellation_cleanup_each_live_child_once() {
        let checked = checked("stop.ling", TWO_ACTORS.as_bytes().to_vec());
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let (mut supervisor, counter_type, divider_type) =
            start_two(&checked, &control, &mut console);
        let counter = supervisor
            .child_reference(counter_type)
            .expect("Counter child")
            .clone();
        let divider = supervisor
            .child_reference(divider_type)
            .expect("Divider child")
            .clone();
        supervisor
            .send(&counter, ActorSenderId::new(1), integer(2))
            .expect("first mailbox");
        supervisor
            .send(&divider, ActorSenderId::new(2), integer(4))
            .expect("second mailbox");

        assert_eq!(
            supervisor.stop().expect("bounded stop"),
            SupervisorStopResult::Stopped {
                actors: 2,
                discarded_messages: 2,
            }
        );
        assert_eq!(
            supervisor.stop().expect("idempotent stop"),
            SupervisorStopResult::AlreadyStopped
        );
        assert_eq!(supervisor.runtime.metrics().cleanups(), 2);
        assert!(
            supervisor
                .snapshot()
                .children
                .iter()
                .all(|child| child.state == SupervisorChildState::Stopped
                    && child.cleanup_count == 1)
        );

        let cancellation_control = LocalTaskControl::new();
        let (mut cancelled, _, _) = start_two(&checked, &cancellation_control, &mut console);
        cancellation_control.cancel();
        assert!(
            cancelled
                .observe_owner_cancellation()
                .expect("owner cancellation")
        );
        assert!(
            !cancelled
                .observe_owner_cancellation()
                .expect("terminal observation is inert")
        );
        assert_eq!(cancelled.state(), SupervisorState::Stopped);
        assert_eq!(cancelled.runtime.metrics().cleanups(), 2);
    }

    #[test]
    fn successful_commands_preserve_bounded_shutdown_event_capacity() {
        let checked = checked("event-reserve.ling", TWO_ACTORS.as_bytes().to_vec());
        let counter = actor(&checked, "Counter");
        let counter_type = actor_type(&checked, &counter);
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let bounded = ActorRuntimeLimits::new(1, 1, 4, 16, 16, 4, 1, 1);
        let mut supervisor = LocalActorSupervisor::start(
            &checked,
            ActorRuntimeId::new(94),
            bounded,
            &control,
            &[counter],
            &mut console,
        )
        .expect("minimum runtime includes construction and shutdown evidence");
        let reference = supervisor
            .child_reference(counter_type)
            .expect("Counter child")
            .clone();
        supervisor
            .send(&reference, ActorSenderId::new(1), integer(2))
            .expect("send leaves two shutdown events");

        assert!(matches!(
            supervisor.step(reference.actor(), &mut console),
            Err(SupervisorError::Runtime(
                ActorRuntimeError::ResourceExhausted {
                    resource: "events",
                    limit: 4
                }
            ))
        ));
        assert_eq!(supervisor.runtime.metrics().queued_messages(), 1);
        assert_eq!(
            supervisor.stop().expect("reserved shutdown succeeds"),
            SupervisorStopResult::Stopped {
                actors: 1,
                discarded_messages: 1,
            }
        );
        assert_eq!(supervisor.runtime.metrics().cleanups(), 1);
    }

    #[test]
    fn unicode_bom_crlf_reconstruction_preserves_containment_projection_and_span() {
        let source = concat!(
            "module Main\n\n",
            "actor 计数器 : Int =\n",
            "    mailbox capacity 2 overflow Reject\n",
            "    state Int = 10\n",
            "    receive 状态 消息 =\n",
            "        状态 / 消息\n\n",
            "let main () = ()\n",
        );
        let variants = [
            ("unicode-lf.ling", source.as_bytes().to_vec()),
            (
                "unicode-crlf.ling",
                source.replace('\n', "\r\n").into_bytes(),
            ),
            ("unicode-bom.ling", format!("\u{feff}{source}").into_bytes()),
        ];
        let projections = variants
            .into_iter()
            .map(|(source_name, bytes)| {
                let checked = checked(source_name, bytes);
                let definition = actor(&checked, "计数器");
                let actor_type = actor_type(&checked, &definition);
                let control = LocalTaskControl::new();
                let mut console = MemoryConsole::default();
                let mut supervisor = LocalActorSupervisor::start(
                    &checked,
                    ActorRuntimeId::new(101),
                    limits(),
                    &control,
                    &[definition],
                    &mut console,
                )
                .expect("Unicode Supervisor starts");
                let reference = supervisor
                    .child_reference(actor_type)
                    .expect("Unicode child")
                    .clone();
                supervisor
                    .send(&reference, ActorSenderId::new(1), integer(0))
                    .expect("faulting message");
                let SupervisorTurnResult::Contained { report, .. } = supervisor
                    .step(reference.actor(), &mut console)
                    .expect("Unicode Fault contained")
                else {
                    panic!("division by zero must be contained");
                };
                (
                    report.actor.get(),
                    report.actor_type.get(),
                    report.category,
                    report.cleanup_count,
                    supervisor.state(),
                    supervisor.runtime.metrics().cleanups(),
                    report.span.end().get() - report.span.start().get(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(projections[0], projections[1]);
        assert_eq!(projections[0], projections[2]);
        assert!(projections[0].6 > 0);
    }
}
