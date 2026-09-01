//! Publish-disabled local Supervisor containment authorized by DEC-0276.
//!
//! This module owns one fixed child set over the real checked-Core Actor
//! runtime. It deliberately has no public re-export, source operation,
//! serialized protocol, or backend execution surface.

use std::collections::{BTreeMap, VecDeque};
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
    Backoff,
    Restarting,
    CircuitOpen,
    Contained,
    Stopped,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SupervisorCircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RestartBudget {
    max_restarts: usize,
    window_ticks: u64,
    backoff_ticks: u64,
}

impl RestartBudget {
    #[must_use]
    pub(super) const fn new(max_restarts: usize, window_ticks: u64, backoff_ticks: u64) -> Self {
        Self {
            max_restarts,
            window_ticks,
            backoff_ticks,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SupervisorPolicy {
    ContainOne,
    RestartOneBudgeted(RestartBudget),
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
    RestartScheduled {
        actor: ActorId,
        report: ChildFaultReport,
        eligible_tick: u64,
    },
    CircuitOpened {
        actor: ActorId,
        report: ChildFaultReport,
        open_until: u64,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SupervisorAdvanceResult {
    tick: u64,
    attempts: usize,
    restarted: usize,
    initializer_faults: usize,
    half_open_probes: usize,
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
    InvalidRestartBudget {
        reason: &'static str,
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
    ClockRegression { current: u64, requested: u64 },
    Runtime(ActorRuntimeError),
    InvalidChildFaultReport { reason: &'static str },
    InvalidRestartTransition { reason: &'static str },
}

impl fmt::Display for SupervisorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRunning { state } => {
                write!(formatter, "local Supervisor is not running: {state:?}")
            }
            Self::ClockRegression { current, requested } => write!(
                formatter,
                "local Supervisor logical clock regressed: {requested} < {current}"
            ),
            Self::Runtime(error) => error.fmt(formatter),
            Self::InvalidChildFaultReport { reason } => {
                write!(formatter, "invalid child Fault report: {reason}")
            }
            Self::InvalidRestartTransition { reason } => {
                write!(formatter, "invalid Supervisor restart transition: {reason}")
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
    restart: Option<RestartSlot>,
}

#[derive(Clone, Debug)]
struct RestartSlot {
    attempts: VecDeque<u64>,
    circuit: SupervisorCircuitState,
    eligible_tick: Option<u64>,
    open_until: Option<u64>,
    last_fault: Option<ChildFaultReport>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SupervisorChildSnapshot {
    actor_type: ActorTypeId,
    definition: DefinitionId,
    actor: Option<ActorId>,
    state: SupervisorChildState,
    actor_state: Option<ActorInstanceState>,
    queued_messages: usize,
    cleanup_count: usize,
    circuit: Option<SupervisorCircuitState>,
    restart_attempts: Vec<u64>,
    eligible_tick: Option<u64>,
    open_until: Option<u64>,
    last_fault: Option<ChildFaultReport>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SupervisorSnapshot {
    state: SupervisorState,
    tick: u64,
    budget: Option<RestartBudget>,
    children: Vec<SupervisorChildSnapshot>,
}

/// One run-owned, fixed-child, non-nested `ContainOne` Supervisor.
pub(super) struct LocalActorSupervisor<'checked> {
    state: SupervisorState,
    runtime: ActorRuntime<'checked>,
    owner_control: LocalTaskControl,
    slots: BTreeMap<ActorTypeId, SupervisorChildSlot>,
    policy: SupervisorPolicy,
    tick: u64,
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
        Self::start_with_policy(
            checked,
            runtime_id,
            limits,
            owner_control,
            definitions,
            SupervisorPolicy::ContainOne,
            console,
        )
    }

    pub(super) fn start_restarting(
        checked: &'checked CheckedProgram,
        runtime_id: ActorRuntimeId,
        limits: ActorRuntimeLimits,
        owner_control: &LocalTaskControl,
        definitions: &[DefinitionId],
        budget: RestartBudget,
        console: &mut dyn Console,
    ) -> Result<Self, SupervisorStartError> {
        Self::start_with_policy(
            checked,
            runtime_id,
            limits,
            owner_control,
            definitions,
            SupervisorPolicy::RestartOneBudgeted(budget),
            console,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn start_with_policy(
        checked: &'checked CheckedProgram,
        runtime_id: ActorRuntimeId,
        limits: ActorRuntimeLimits,
        owner_control: &LocalTaskControl,
        definitions: &[DefinitionId],
        policy: SupervisorPolicy,
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

        if let Some(kind) = construction_limit_error(limits, ordered.len(), policy) {
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
                        restart: match policy {
                            SupervisorPolicy::ContainOne => None,
                            SupervisorPolicy::RestartOneBudgeted(_) => Some(RestartSlot {
                                attempts: VecDeque::new(),
                                circuit: SupervisorCircuitState::Closed,
                                eligible_tick: None,
                                open_until: None,
                                last_fault: None,
                            }),
                        },
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
            policy,
            tick: 0,
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
            .filter(|slot| slot.state == SupervisorChildState::Running)
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
                match self.policy {
                    SupervisorPolicy::ContainOne => {
                        self.acknowledge_fault(&report)?;
                        Ok(SupervisorTurnResult::Contained { actor, report })
                    }
                    SupervisorPolicy::RestartOneBudgeted(budget) => {
                        self.acknowledge_restart_fault(report, budget)
                    }
                }
            }
        }
    }

    pub(super) fn advance_to(
        &mut self,
        tick: u64,
        console: &mut dyn Console,
    ) -> Result<SupervisorAdvanceResult, SupervisorError> {
        self.observe_owner_cancellation()?;
        self.ensure_running()?;
        let SupervisorPolicy::RestartOneBudgeted(budget) = self.policy else {
            return Err(SupervisorError::InvalidRestartTransition {
                reason: "ContainOne Supervisor has no logical restart clock",
            });
        };
        if tick < self.tick {
            return Err(SupervisorError::ClockRegression {
                current: self.tick,
                requested: tick,
            });
        }

        let mut due = Vec::new();
        let mut invalid = None;
        for (actor_type, slot) in &self.slots {
            let restart = slot
                .restart
                .as_ref()
                .expect("restarting policy preallocates restart state");
            match slot.state {
                SupervisorChildState::Backoff => match restart.eligible_tick {
                    Some(eligible_tick) if eligible_tick <= tick => {
                        let mut attempts = restart.attempts.clone();
                        if let Err(reason) =
                            prune_attempts(&mut attempts, tick, budget.window_ticks)
                        {
                            invalid = Some(reason);
                        } else if attempts.len() >= budget.max_restarts {
                            invalid = Some("Backoff restart has no budget capacity");
                        } else {
                            due.push((*actor_type, false));
                        }
                    }
                    Some(_) => {}
                    None => invalid = Some("Backoff slot has no eligible tick"),
                },
                SupervisorChildState::CircuitOpen => match restart.open_until {
                    Some(open_until) if open_until <= tick => {
                        let mut attempts = restart.attempts.clone();
                        if let Err(reason) =
                            prune_attempts(&mut attempts, tick, budget.window_ticks)
                        {
                            invalid = Some(reason);
                        } else if attempts.len() >= budget.max_restarts {
                            invalid = Some("Open circuit reached expiry without budget capacity");
                        } else {
                            due.push((*actor_type, true));
                        }
                    }
                    Some(_) => {}
                    None => invalid = Some("Open circuit has no expiry tick"),
                },
                SupervisorChildState::Starting
                | SupervisorChildState::Running
                | SupervisorChildState::Contained
                | SupervisorChildState::Stopped => {}
                SupervisorChildState::Restarting => {
                    invalid = Some("Restarting slot escaped a coordinator boundary");
                }
            }
            if invalid.is_some() {
                break;
            }
        }
        if let Some(reason) = invalid {
            return Err(self.reject_restart_transition(reason));
        }
        if !due.is_empty() {
            if tick.checked_add(budget.window_ticks).is_none()
                || tick.checked_add(budget.backoff_ticks).is_none()
            {
                return Err(
                    self.reject_restart_transition("restart deadline arithmetic overflowed")
                );
            }
            if let Err(error) = self.runtime.preflight_supervised_restarts(due.len()) {
                return Err(self.reject_restart_runtime(error));
            }
        }

        self.tick = tick;
        let mut result = SupervisorAdvanceResult {
            tick,
            attempts: 0,
            restarted: 0,
            initializer_faults: 0,
            half_open_probes: 0,
        };
        for (actor_type, half_open) in due {
            let definition = self
                .slots
                .get(&actor_type)
                .expect("due child slot remains registered")
                .definition
                .clone();
            let transition_error = {
                let slot = self
                    .slots
                    .get_mut(&actor_type)
                    .expect("due child slot remains registered");
                let restart = slot
                    .restart
                    .as_mut()
                    .expect("restarting policy preallocates restart state");
                if let Err(reason) =
                    prune_attempts(&mut restart.attempts, tick, budget.window_ticks)
                {
                    Some(reason)
                } else if restart.attempts.len() >= budget.max_restarts {
                    Some("due restart has no budget capacity")
                } else {
                    restart.attempts.push_back(tick);
                    restart.eligible_tick = None;
                    restart.open_until = None;
                    restart.circuit = if half_open {
                        SupervisorCircuitState::HalfOpen
                    } else {
                        SupervisorCircuitState::Closed
                    };
                    slot.state = SupervisorChildState::Restarting;
                    None
                }
            };
            if let Some(reason) = transition_error {
                return Err(self.reject_restart_transition(reason));
            }
            result.attempts += 1;
            if half_open {
                result.half_open_probes += 1;
            }

            match self.runtime.spawn_for_restart(&definition, console) {
                Ok(reference) => {
                    let slot = self
                        .slots
                        .get_mut(&actor_type)
                        .expect("restarted child slot remains registered");
                    let restart = slot
                        .restart
                        .as_mut()
                        .expect("restarting policy preallocates restart state");
                    slot.reference = Some(reference);
                    slot.state = SupervisorChildState::Running;
                    restart.circuit = SupervisorCircuitState::Closed;
                    restart.eligible_tick = None;
                    restart.open_until = None;
                    result.restarted += 1;
                }
                Err(ActorSpawnError::Fault(fault)) => {
                    let report = match self.report_from_initializer_fault(&fault) {
                        Ok(report) => report,
                        Err(reason) => return Err(self.reject_restart_transition(reason)),
                    };
                    if let Err(reason) =
                        self.finish_initializer_fault(actor_type, report, budget, half_open)
                    {
                        return Err(self.reject_restart_transition(reason));
                    }
                    result.initializer_faults += 1;
                }
                Err(ActorSpawnError::Runtime(error)) => {
                    return Err(self.reject_restart_runtime(error));
                }
            }
        }
        Ok(result)
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
                let actor = slot
                    .reference
                    .as_ref()
                    .and_then(|reference| self.runtime.snapshot(reference.actor()));
                SupervisorChildSnapshot {
                    actor_type: *actor_type,
                    definition: slot.definition.clone(),
                    actor: actor.as_ref().map(|actor| actor.reference().actor()),
                    state: slot.state,
                    actor_state: actor.as_ref().map(ActorInstanceSnapshot::lifecycle),
                    queued_messages: actor
                        .as_ref()
                        .map_or(0, ActorInstanceSnapshot::queued_messages),
                    cleanup_count: actor
                        .as_ref()
                        .map_or(0, ActorInstanceSnapshot::cleanup_count),
                    circuit: slot.restart.as_ref().map(|restart| restart.circuit),
                    restart_attempts: slot.restart.as_ref().map_or_else(Vec::new, |restart| {
                        restart.attempts.iter().copied().collect()
                    }),
                    eligible_tick: slot
                        .restart
                        .as_ref()
                        .and_then(|restart| restart.eligible_tick),
                    open_until: slot.restart.as_ref().and_then(|restart| restart.open_until),
                    last_fault: slot
                        .restart
                        .as_ref()
                        .and_then(|restart| restart.last_fault.clone()),
                }
            })
            .collect();
        SupervisorSnapshot {
            state: self.state,
            tick: self.tick,
            budget: match self.policy {
                SupervisorPolicy::ContainOne => None,
                SupervisorPolicy::RestartOneBudgeted(budget) => Some(budget),
            },
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

    fn acknowledge_restart_fault(
        &mut self,
        report: ChildFaultReport,
        budget: RestartBudget,
    ) -> Result<SupervisorTurnResult, SupervisorError> {
        if let Err(reason) = self.validate_fault_report(&report) {
            return Err(self.reject_fault_report(reason));
        }
        let mut attempts = self
            .slots
            .get(&report.actor_type)
            .and_then(|slot| slot.restart.as_ref())
            .expect("restarting policy preallocates restart state")
            .attempts
            .clone();
        if let Err(reason) = prune_attempts(&mut attempts, self.tick, budget.window_ticks) {
            return Err(self.reject_restart_transition(reason));
        }
        let actor = report.actor;
        if attempts.len() < budget.max_restarts {
            let Some(eligible_tick) = self.tick.checked_add(budget.backoff_ticks) else {
                return Err(self.reject_restart_transition("restart backoff tick overflowed"));
            };
            let slot = self
                .slots
                .get_mut(&report.actor_type)
                .expect("validated child slot remains registered");
            let restart = slot
                .restart
                .as_mut()
                .expect("restarting policy preallocates restart state");
            restart.attempts = attempts;
            restart.circuit = SupervisorCircuitState::Closed;
            restart.eligible_tick = Some(eligible_tick);
            restart.open_until = None;
            restart.last_fault = Some(report.clone());
            slot.state = SupervisorChildState::Backoff;
            Ok(SupervisorTurnResult::RestartScheduled {
                actor,
                report,
                eligible_tick,
            })
        } else {
            let Some(open_until) = attempts
                .front()
                .copied()
                .and_then(|attempt| attempt.checked_add(budget.window_ticks))
            else {
                return Err(self
                    .reject_restart_transition("restart circuit expiry could not be represented"));
            };
            let slot = self
                .slots
                .get_mut(&report.actor_type)
                .expect("validated child slot remains registered");
            let restart = slot
                .restart
                .as_mut()
                .expect("restarting policy preallocates restart state");
            restart.attempts = attempts;
            restart.circuit = SupervisorCircuitState::Open;
            restart.eligible_tick = None;
            restart.open_until = Some(open_until);
            restart.last_fault = Some(report.clone());
            slot.state = SupervisorChildState::CircuitOpen;
            Ok(SupervisorTurnResult::CircuitOpened {
                actor,
                report,
                open_until,
            })
        }
    }

    fn report_from_initializer_fault(
        &self,
        fault: &ActorFault,
    ) -> Result<ChildFaultReport, &'static str> {
        if fault.runtime() != self.runtime.id() {
            return Err("restart initializer Fault has the wrong runtime identity");
        }
        if fault.phase() != ActorFaultPhase::Initializer {
            return Err("restart spawn returned a non-initializer Fault");
        }
        let slot = self
            .slots
            .get(&fault.actor_type())
            .ok_or("restart initializer Fault has no child slot")?;
        if slot.state != SupervisorChildState::Restarting || &slot.definition != fault.definition()
        {
            return Err("restart initializer Fault disagrees with its child slot");
        }
        if self.runtime.reference(fault.actor()).is_some()
            || self.runtime.snapshot(fault.actor()).is_some()
        {
            return Err("failed restart initializer published an Actor record");
        }
        let mut events = self.runtime.events().into_iter().filter(|event| {
            matches!(
                event.kind(),
                ActorRuntimeEventKind::ActorSpawnFaulted { actor }
                    if *actor == fault.actor()
            )
        });
        if events.next().is_none() || events.next().is_some() {
            return Err("restart initializer Fault event count is inconsistent");
        }
        Ok(ChildFaultReport {
            runtime: fault.runtime(),
            actor: fault.actor(),
            actor_type: fault.actor_type(),
            definition: fault.definition().clone(),
            expression: fault.expression(),
            phase: fault.phase(),
            span: fault.cause().span,
            category: runtime_fault_category(&fault.cause().kind),
            discarded_messages: 0,
            cleanup_count: 0,
        })
    }

    fn finish_initializer_fault(
        &mut self,
        actor_type: ActorTypeId,
        report: ChildFaultReport,
        budget: RestartBudget,
        half_open: bool,
    ) -> Result<(), &'static str> {
        if report.actor_type != actor_type || report.phase != ActorFaultPhase::Initializer {
            return Err("initializer Fault report targets the wrong restart slot");
        }
        let attempts = &self
            .slots
            .get(&actor_type)
            .and_then(|slot| slot.restart.as_ref())
            .ok_or("restart slot state is absent")?
            .attempts;
        let opens = half_open || attempts.len() >= budget.max_restarts;
        let deadline = if opens {
            attempts
                .front()
                .copied()
                .and_then(|attempt| attempt.checked_add(budget.window_ticks))
                .ok_or("initializer Fault circuit expiry overflowed")?
        } else {
            self.tick
                .checked_add(budget.backoff_ticks)
                .ok_or("initializer Fault backoff tick overflowed")?
        };
        let slot = self
            .slots
            .get_mut(&actor_type)
            .ok_or("restart child slot disappeared")?;
        let restart = slot
            .restart
            .as_mut()
            .ok_or("restart slot state is absent")?;
        slot.reference = None;
        restart.last_fault = Some(report);
        if opens {
            slot.state = SupervisorChildState::CircuitOpen;
            restart.circuit = SupervisorCircuitState::Open;
            restart.eligible_tick = None;
            restart.open_until = Some(deadline);
        } else {
            slot.state = SupervisorChildState::Backoff;
            restart.circuit = SupervisorCircuitState::Closed;
            restart.eligible_tick = Some(deadline);
            restart.open_until = None;
        }
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

    fn reject_restart_transition(&mut self, reason: &'static str) -> SupervisorError {
        self.state = SupervisorState::Failed;
        self.owner_control.cancel();
        if self.runtime.state() == ActorRuntimeState::Running {
            self.runtime.fail_unacknowledged_supervised_fault();
        }
        self.mark_running_slots_stopped();
        SupervisorError::InvalidRestartTransition { reason }
    }

    fn reject_restart_runtime(&mut self, error: ActorRuntimeError) -> SupervisorError {
        self.state = SupervisorState::Failed;
        self.owner_control.cancel();
        if self.runtime.state() == ActorRuntimeState::Running {
            self.runtime.fail_unacknowledged_supervised_fault();
        }
        self.mark_running_slots_stopped();
        SupervisorError::Runtime(error)
    }

    fn mark_running_slots_stopped(&mut self) {
        for slot in self.slots.values_mut() {
            if matches!(
                slot.state,
                SupervisorChildState::Running
                    | SupervisorChildState::Backoff
                    | SupervisorChildState::Restarting
                    | SupervisorChildState::CircuitOpen
            ) {
                slot.state = SupervisorChildState::Stopped;
                if let Some(restart) = &mut slot.restart {
                    restart.eligible_tick = None;
                    restart.open_until = None;
                }
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
    policy: SupervisorPolicy,
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
    let Some(required_commands) = children.checked_add(1) else {
        return Some(SupervisorStartErrorKind::ResourceExhausted {
            resource: "commands",
            required: usize::MAX,
            limit: limits.max_commands(),
        });
    };
    if required_commands > limits.max_commands() {
        return Some(SupervisorStartErrorKind::ResourceExhausted {
            resource: "commands",
            required: required_commands,
            limit: limits.max_commands(),
        });
    }
    let Some(required_events) = children
        .checked_mul(2)
        .and_then(|events| events.checked_add(1))
    else {
        return Some(SupervisorStartErrorKind::ResourceExhausted {
            resource: "events",
            required: usize::MAX,
            limit: limits.max_events(),
        });
    };
    if required_events > limits.max_events() {
        return Some(SupervisorStartErrorKind::ResourceExhausted {
            resource: "events",
            required: required_events,
            limit: limits.max_events(),
        });
    }
    if let SupervisorPolicy::RestartOneBudgeted(budget) = policy {
        if budget.max_restarts == 0 {
            return Some(SupervisorStartErrorKind::InvalidRestartBudget {
                reason: "max_restarts must be nonzero",
            });
        }
        if budget.window_ticks == 0 {
            return Some(SupervisorStartErrorKind::InvalidRestartBudget {
                reason: "window_ticks must be nonzero",
            });
        }
        if budget.backoff_ticks == 0 {
            return Some(SupervisorStartErrorKind::InvalidRestartBudget {
                reason: "backoff_ticks must be nonzero",
            });
        }
        let Some(history_entries) = children.checked_mul(budget.max_restarts) else {
            return Some(SupervisorStartErrorKind::ResourceExhausted {
                resource: "restart_history",
                required: usize::MAX,
                limit: limits.max_faults(),
            });
        };
        if history_entries > limits.max_faults() {
            return Some(SupervisorStartErrorKind::ResourceExhausted {
                resource: "restart_history",
                required: history_entries,
                limit: limits.max_faults(),
            });
        }
        let Some(created_with_one_full_window) = children.checked_add(history_entries) else {
            return Some(SupervisorStartErrorKind::ResourceExhausted {
                resource: "created_actors",
                required: usize::MAX,
                limit: limits.max_created_actors(),
            });
        };
        if created_with_one_full_window > limits.max_created_actors() {
            return Some(SupervisorStartErrorKind::ResourceExhausted {
                resource: "created_actors",
                required: created_with_one_full_window,
                limit: limits.max_created_actors(),
            });
        }
    }
    None
}

fn prune_attempts(
    attempts: &mut VecDeque<u64>,
    tick: u64,
    window_ticks: u64,
) -> Result<(), &'static str> {
    while let Some(attempt) = attempts.front().copied() {
        let expiry = attempt
            .checked_add(window_ticks)
            .ok_or("restart window expiry overflowed")?;
        if expiry > tick {
            break;
        }
        attempts.pop_front();
    }
    Ok(())
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
#[path = "actor_supervisor_evidence.rs"]
mod evidence_tests;

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

    const TWO_FAULTING_ACTORS: &str = concat!(
        "module Main\n\n",
        "actor First : Int =\n",
        "    mailbox capacity 2 overflow Reject\n",
        "    state Int = 12\n",
        "    receive state message =\n",
        "        state / message\n\n",
        "actor Second : Int =\n",
        "    mailbox capacity 2 overflow Reject\n",
        "    state Int = 18\n",
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

    fn start_two_restarting<'checked>(
        checked: &'checked CheckedProgram,
        control: &LocalTaskControl,
        console: &mut MemoryConsole,
        budget: RestartBudget,
    ) -> (LocalActorSupervisor<'checked>, ActorTypeId, ActorTypeId) {
        let counter = actor(checked, "Counter");
        let divider = actor(checked, "Divider");
        let counter_type = actor_type(checked, &counter);
        let divider_type = actor_type(checked, &divider);
        let supervisor = LocalActorSupervisor::start_restarting(
            checked,
            ActorRuntimeId::new(72),
            limits(),
            control,
            &[divider, counter],
            budget,
            console,
        )
        .expect("fixed child restarting Supervisor starts");
        (supervisor, counter_type, divider_type)
    }

    #[test]
    pub(super) fn budgeted_restart_waits_for_backoff_and_publishes_fresh_initializer_state() {
        let checked = checked("restart.ling", TWO_ACTORS.as_bytes().to_vec());
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let (mut supervisor, counter_type, divider_type) = start_two_restarting(
            &checked,
            &control,
            &mut console,
            RestartBudget::new(2, 10, 2),
        );
        let counter = supervisor
            .child_reference(counter_type)
            .expect("Counter child")
            .clone();
        let divider = supervisor
            .child_reference(divider_type)
            .expect("Divider child")
            .clone();
        supervisor
            .send(&counter, ActorSenderId::new(1), integer(4))
            .expect("sibling message");
        supervisor
            .send(&divider, ActorSenderId::new(2), integer(0))
            .expect("faulting message");
        let SupervisorTurnResult::RestartScheduled {
            actor,
            report,
            eligible_tick,
        } = supervisor
            .step(divider.actor(), &mut console)
            .expect("Fault schedules a restart")
        else {
            panic!("budgeted policy must schedule the first restart");
        };
        assert_eq!(actor, divider.actor());
        assert_eq!(report.phase, ActorFaultPhase::Turn);
        assert_eq!(eligible_tick, 2);
        assert!(!control.is_cancelled());
        assert!(supervisor.child_reference(divider_type).is_none());
        let closed = supervisor
            .send(&divider, ActorSenderId::new(2), integer(9))
            .expect_err("old incarnation remains closed");
        assert_eq!(closed.kind(), &ActorSendErrorKind::Closed);

        assert_eq!(
            supervisor
                .advance_to(1, &mut console)
                .expect("clock advances before eligibility"),
            SupervisorAdvanceResult {
                tick: 1,
                attempts: 0,
                restarted: 0,
                initializer_faults: 0,
                half_open_probes: 0,
            }
        );
        let advanced = supervisor
            .advance_to(2, &mut console)
            .expect("eligible restart succeeds");
        assert_eq!(advanced.attempts, 1);
        assert_eq!(advanced.restarted, 1);
        let replacement = supervisor
            .child_reference(divider_type)
            .expect("replacement child")
            .clone();
        assert!(replacement.actor() > divider.actor());
        let replacement_state = supervisor
            .runtime
            .snapshot(replacement.actor())
            .expect("replacement snapshot");
        assert_eq!(replacement_state.state(), Some(&integer(20)));
        assert_eq!(replacement_state.queued_messages(), 0);
        assert_eq!(supervisor.ready(), [counter.actor()]);
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
    }

    #[test]
    pub(super) fn exact_window_boundary_opens_and_half_open_probe_closes_the_circuit() {
        let checked = checked("circuit.ling", TWO_ACTORS.as_bytes().to_vec());
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let (mut supervisor, _, divider_type) = start_two_restarting(
            &checked,
            &control,
            &mut console,
            RestartBudget::new(2, 5, 1),
        );

        for tick in [0_u64, 1] {
            let divider = supervisor
                .child_reference(divider_type)
                .expect("running Divider")
                .clone();
            supervisor
                .send(&divider, ActorSenderId::new(2), integer(0))
                .expect("faulting message");
            let SupervisorTurnResult::RestartScheduled { eligible_tick, .. } = supervisor
                .step(divider.actor(), &mut console)
                .expect("restart remains inside budget")
            else {
                panic!("first two Faults must schedule restart attempts");
            };
            assert_eq!(eligible_tick, tick + 1);
            let advanced = supervisor
                .advance_to(tick + 1, &mut console)
                .expect("budgeted replacement succeeds");
            assert_eq!(advanced.restarted, 1);
        }

        let divider = supervisor
            .child_reference(divider_type)
            .expect("second replacement")
            .clone();
        supervisor
            .send(&divider, ActorSenderId::new(2), integer(0))
            .expect("third Fault message");
        let SupervisorTurnResult::CircuitOpened { open_until, .. } = supervisor
            .step(divider.actor(), &mut console)
            .expect("exhausted budget opens the circuit")
        else {
            panic!("third Fault must open the circuit");
        };
        assert_eq!(open_until, 6);
        assert!(supervisor.child_reference(divider_type).is_none());
        assert_eq!(
            supervisor
                .advance_to(5, &mut console)
                .expect("Open circuit does not probe early")
                .attempts,
            0
        );
        let open = supervisor.snapshot();
        let divider_slot = open
            .children
            .iter()
            .find(|child| child.actor_type == divider_type)
            .expect("Divider slot");
        assert_eq!(divider_slot.state, SupervisorChildState::CircuitOpen);
        assert_eq!(divider_slot.circuit, Some(SupervisorCircuitState::Open));
        assert_eq!(divider_slot.restart_attempts, [1, 2]);
        assert_eq!(divider_slot.open_until, Some(6));

        let probe = supervisor
            .advance_to(6, &mut console)
            .expect("exact expiry permits one probe");
        assert_eq!(probe.attempts, 1);
        assert_eq!(probe.restarted, 1);
        assert_eq!(probe.half_open_probes, 1);
        let closed = supervisor.snapshot();
        let divider_slot = closed
            .children
            .iter()
            .find(|child| child.actor_type == divider_type)
            .expect("Divider slot");
        assert_eq!(divider_slot.state, SupervisorChildState::Running);
        assert_eq!(divider_slot.circuit, Some(SupervisorCircuitState::Closed));
        assert_eq!(divider_slot.restart_attempts, [2, 6]);
        assert_eq!(divider_slot.open_until, None);
        assert!(!control.is_cancelled());
    }

    #[test]
    pub(super) fn initializer_fault_consumes_attempt_and_half_open_failure_reopens_once() {
        let checked = checked("initializer-retry.ling", TWO_ACTORS.as_bytes().to_vec());
        let divider = actor(&checked, "Divider");
        let divider_type = actor_type(&checked, &divider);
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let mut supervisor = LocalActorSupervisor::start_restarting(
            &checked,
            ActorRuntimeId::new(73),
            limits(),
            &control,
            &[divider],
            RestartBudget::new(1, 5, 1),
            &mut console,
        )
        .expect("single-child restarting Supervisor");
        let original = supervisor
            .child_reference(divider_type)
            .expect("initial Divider")
            .clone();
        supervisor
            .send(&original, ActorSenderId::new(1), integer(0))
            .expect("faulting message");
        assert!(matches!(
            supervisor.step(original.actor(), &mut console),
            Ok(SupervisorTurnResult::RestartScheduled {
                eligible_tick: 1,
                ..
            })
        ));

        ActorRuntime::request_initializer_panic();
        let first = supervisor
            .advance_to(1, &mut console)
            .expect("initializer Fault is contained by the restart policy");
        assert_eq!(first.initializer_faults, 1);
        assert_eq!(first.restarted, 0);
        let snapshot = supervisor.snapshot();
        let slot = &snapshot.children[0];
        assert_eq!(slot.state, SupervisorChildState::CircuitOpen);
        assert_eq!(slot.restart_attempts, [1]);
        assert_eq!(slot.open_until, Some(6));
        assert_eq!(
            slot.last_fault.as_ref().map(|fault| fault.phase),
            Some(ActorFaultPhase::Initializer)
        );
        assert_eq!(
            slot.last_fault.as_ref().map(|fault| fault.cleanup_count),
            Some(0)
        );
        assert!(supervisor.child_reference(divider_type).is_none());
        assert_eq!(supervisor.runtime.state(), ActorRuntimeState::Running);
        assert!(!control.is_cancelled());

        ActorRuntime::request_initializer_panic();
        let probe = supervisor
            .advance_to(6, &mut console)
            .expect("HalfOpen initializer Fault reopens the circuit");
        assert_eq!(probe.half_open_probes, 1);
        assert_eq!(probe.initializer_faults, 1);
        let reopened = supervisor.snapshot();
        assert_eq!(reopened.children[0].restart_attempts, [6]);
        assert_eq!(reopened.children[0].open_until, Some(11));
        assert_eq!(
            supervisor
                .advance_to(6, &mut console)
                .expect("same tick cannot run a second probe")
                .attempts,
            0
        );
        let recovered = supervisor
            .advance_to(11, &mut console)
            .expect("next exact expiry permits one successful probe");
        assert_eq!(recovered.restarted, 1);
        assert_eq!(recovered.half_open_probes, 1);
        let replacement = supervisor
            .child_reference(divider_type)
            .expect("eventual replacement");
        assert_eq!(replacement.actor().get(), 4);
        assert!(!control.is_cancelled());
    }

    #[test]
    pub(super) fn restart_configuration_clock_and_overflow_fail_at_the_defined_boundaries() {
        let checked = checked("restart-boundaries.ling", TWO_ACTORS.as_bytes().to_vec());
        let divider = actor(&checked, "Divider");
        let mut console = MemoryConsole::default();

        for (budget, reason) in [
            (RestartBudget::new(0, 1, 1), "max_restarts"),
            (RestartBudget::new(1, 0, 1), "window_ticks"),
            (RestartBudget::new(1, 1, 0), "backoff_ticks"),
        ] {
            let control = LocalTaskControl::new();
            let error = match LocalActorSupervisor::start_restarting(
                &checked,
                ActorRuntimeId::new(74),
                limits(),
                &control,
                std::slice::from_ref(&divider),
                budget,
                &mut console,
            ) {
                Ok(_) => panic!("zero restart configuration must be rejected"),
                Err(error) => error,
            };
            assert!(matches!(
                error.kind(),
                SupervisorStartErrorKind::InvalidRestartBudget { reason: actual }
                    if actual.contains(reason)
            ));
            assert!(control.is_cancelled());
        }

        let counter = actor(&checked, "Counter");
        let bounded_control = LocalTaskControl::new();
        let bounded = match LocalActorSupervisor::start_restarting(
            &checked,
            ActorRuntimeId::new(74),
            limits(),
            &bounded_control,
            &[divider.clone(), counter],
            RestartBudget::new(4, 5, 1),
            &mut console,
        ) {
            Ok(_) => panic!("one full restart-history window must fit created Actor bounds"),
            Err(error) => error,
        };
        assert!(matches!(
            bounded.kind(),
            SupervisorStartErrorKind::ResourceExhausted {
                resource: "created_actors",
                required: 10,
                limit: 8,
            }
        ));
        assert!(bounded_control.is_cancelled());

        let control = LocalTaskControl::new();
        let mut supervisor = LocalActorSupervisor::start_restarting(
            &checked,
            ActorRuntimeId::new(75),
            limits(),
            &control,
            std::slice::from_ref(&divider),
            RestartBudget::new(1, 2, 2),
            &mut console,
        )
        .expect("valid restart configuration");
        assert_eq!(
            supervisor
                .advance_to(5, &mut console)
                .expect("forward clock advance")
                .tick,
            5
        );
        assert_eq!(
            supervisor.advance_to(4, &mut console),
            Err(SupervisorError::ClockRegression {
                current: 5,
                requested: 4,
            })
        );
        assert_eq!(supervisor.snapshot().tick, 5);
        assert!(!control.is_cancelled());

        supervisor
            .advance_to(u64::MAX, &mut console)
            .expect("clock can reach the maximum while no restart is pending");
        let reference = supervisor
            .child_reference(actor_type(&checked, &divider))
            .expect("running child")
            .clone();
        supervisor
            .send(&reference, ActorSenderId::new(1), integer(0))
            .expect("faulting message");
        assert!(matches!(
            supervisor.step(reference.actor(), &mut console),
            Err(SupervisorError::InvalidRestartTransition { .. })
        ));
        assert_eq!(supervisor.state(), SupervisorState::Failed);
        assert!(control.is_cancelled());
    }

    #[test]
    pub(super) fn simultaneous_due_slots_restart_in_canonical_actor_type_order() {
        let checked = checked(
            "canonical-restarts.ling",
            TWO_FAULTING_ACTORS.as_bytes().to_vec(),
        );
        let first = actor(&checked, "First");
        let second = actor(&checked, "Second");
        let first_type = actor_type(&checked, &first);
        let second_type = actor_type(&checked, &second);
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let mut supervisor = LocalActorSupervisor::start_restarting(
            &checked,
            ActorRuntimeId::new(76),
            limits(),
            &control,
            &[second, first],
            RestartBudget::new(1, 5, 1),
            &mut console,
        )
        .expect("two restarting children start in canonical order");
        for actor_type in [second_type, first_type] {
            let reference = supervisor
                .child_reference(actor_type)
                .expect("faulting child")
                .clone();
            supervisor
                .send(&reference, ActorSenderId::new(1), integer(0))
                .expect("faulting message");
            assert!(matches!(
                supervisor.step(reference.actor(), &mut console),
                Ok(SupervisorTurnResult::RestartScheduled {
                    eligible_tick: 1,
                    ..
                })
            ));
        }
        let advanced = supervisor
            .advance_to(1, &mut console)
            .expect("both due slots restart atomically");
        assert_eq!(advanced.attempts, 2);
        assert_eq!(advanced.restarted, 2);
        let mut replacements = [first_type, second_type]
            .into_iter()
            .map(|actor_type| {
                (
                    actor_type,
                    supervisor
                        .child_reference(actor_type)
                        .expect("replacement")
                        .actor(),
                )
            })
            .collect::<Vec<_>>();
        replacements.sort_by_key(|(actor_type, _)| *actor_type);
        assert_eq!(
            replacements
                .iter()
                .map(|(_, actor)| actor.get())
                .collect::<Vec<_>>(),
            [3, 4]
        );
        assert!(!control.is_cancelled());
    }

    #[test]
    pub(super) fn pending_restart_is_cancelled_without_new_actor_or_double_cleanup() {
        let checked = checked("cancel-restart.ling", TWO_ACTORS.as_bytes().to_vec());
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let (mut supervisor, counter_type, divider_type) = start_two_restarting(
            &checked,
            &control,
            &mut console,
            RestartBudget::new(2, 10, 2),
        );
        let counter = supervisor
            .child_reference(counter_type)
            .expect("Counter child")
            .clone();
        let divider = supervisor
            .child_reference(divider_type)
            .expect("Divider child")
            .clone();
        supervisor
            .send(&counter, ActorSenderId::new(1), integer(4))
            .expect("queued sibling message");
        supervisor
            .send(&divider, ActorSenderId::new(2), integer(0))
            .expect("faulting message");
        assert!(matches!(
            supervisor.step(divider.actor(), &mut console),
            Ok(SupervisorTurnResult::RestartScheduled {
                eligible_tick: 2,
                ..
            })
        ));
        let created_before_cancel = supervisor.runtime.metrics().created_actors();
        control.cancel();
        assert!(
            supervisor
                .observe_owner_cancellation()
                .expect("owner cancellation is observed")
        );
        assert_eq!(supervisor.state(), SupervisorState::Stopped);
        assert_eq!(
            supervisor.runtime.metrics().created_actors(),
            created_before_cancel
        );
        assert_eq!(supervisor.runtime.metrics().cleanups(), 2);
        assert_eq!(
            supervisor
                .runtime
                .snapshot(divider.actor())
                .expect("failed Divider record")
                .cleanup_count(),
            1
        );
        assert_eq!(
            supervisor
                .runtime
                .snapshot(counter.actor())
                .expect("stopped Counter record")
                .cleanup_count(),
            1
        );
        assert!(supervisor.snapshot().children.iter().all(|child| {
            child.state == SupervisorChildState::Stopped
                && child.eligible_tick.is_none()
                && child.open_until.is_none()
        }));
        assert!(matches!(
            supervisor.advance_to(2, &mut console),
            Err(SupervisorError::NotRunning {
                state: SupervisorState::Stopped
            })
        ));
    }

    #[test]
    pub(super) fn restart_preflight_exhaustion_is_terminal_and_attempt_free() {
        let checked = checked("restart-resource.ling", TWO_ACTORS.as_bytes().to_vec());
        let divider = actor(&checked, "Divider");
        let divider_type = actor_type(&checked, &divider);
        let control = LocalTaskControl::new();
        let mut console = MemoryConsole::default();
        let limits = ActorRuntimeLimits::new(2, 1, 4, 3, 4, 16, 2, 1);
        let mut supervisor = LocalActorSupervisor::start_restarting(
            &checked,
            ActorRuntimeId::new(77),
            limits,
            &control,
            &[divider],
            RestartBudget::new(1, 5, 1),
            &mut console,
        )
        .expect("construction fits the bounded runtime");
        let reference = supervisor
            .child_reference(divider_type)
            .expect("Divider child")
            .clone();
        supervisor
            .send(&reference, ActorSenderId::new(1), integer(0))
            .expect("faulting message uses the second command");
        assert!(matches!(
            supervisor.step(reference.actor(), &mut console),
            Ok(SupervisorTurnResult::RestartScheduled {
                eligible_tick: 1,
                ..
            })
        ));
        assert_eq!(supervisor.runtime.metrics().commands(), 3);
        assert!(matches!(
            supervisor.advance_to(1, &mut console),
            Err(SupervisorError::Runtime(
                ActorRuntimeError::ResourceExhausted {
                    resource: "commands",
                    limit: 3,
                }
            ))
        ));
        assert_eq!(supervisor.runtime.metrics().created_actors(), 1);
        assert_eq!(supervisor.runtime.metrics().faults(), 1);
        assert_eq!(supervisor.state(), SupervisorState::Failed);
        assert!(control.is_cancelled());
    }

    #[test]
    pub(super) fn contain_one_fault_seals_only_the_child_and_preserves_the_sibling() {
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
    pub(super) fn every_stale_duplicate_or_inconsistent_report_uses_root_fallback() {
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
    pub(super) fn sequential_faults_can_contain_all_children_without_restart_or_double_cleanup() {
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
                    && child.actor_state == Some(ActorInstanceState::Failed)
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
    pub(super) fn stop_and_owner_cancellation_cleanup_each_live_child_once() {
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
    pub(super) fn unicode_bom_crlf_reconstruction_preserves_containment_projection_and_span() {
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

    #[test]
    pub(super) fn unicode_bom_crlf_reconstruction_preserves_restart_projection_and_span() {
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
            ("restart-unicode-lf.ling", source.as_bytes().to_vec()),
            (
                "restart-unicode-crlf.ling",
                source.replace('\n', "\r\n").into_bytes(),
            ),
            (
                "restart-unicode-bom.ling",
                format!("\u{feff}{source}").into_bytes(),
            ),
        ];
        let projections = variants
            .into_iter()
            .map(|(source_name, bytes)| {
                let checked = checked(source_name, bytes);
                let definition = actor(&checked, "计数器");
                let actor_type = actor_type(&checked, &definition);
                let control = LocalTaskControl::new();
                let mut console = MemoryConsole::default();
                let mut supervisor = LocalActorSupervisor::start_restarting(
                    &checked,
                    ActorRuntimeId::new(102),
                    limits(),
                    &control,
                    &[definition],
                    RestartBudget::new(1, 5, 1),
                    &mut console,
                )
                .expect("Unicode restarting Supervisor starts");
                let reference = supervisor
                    .child_reference(actor_type)
                    .expect("Unicode child")
                    .clone();
                supervisor
                    .send(&reference, ActorSenderId::new(1), integer(0))
                    .expect("faulting message");
                let SupervisorTurnResult::RestartScheduled { report, .. } = supervisor
                    .step(reference.actor(), &mut console)
                    .expect("Unicode Fault schedules restart")
                else {
                    panic!("division by zero must schedule a restart");
                };
                let advanced = supervisor
                    .advance_to(1, &mut console)
                    .expect("Unicode child restarts");
                let replacement = supervisor
                    .child_reference(actor_type)
                    .expect("Unicode replacement");
                let snapshot = supervisor.snapshot();
                let slot = &snapshot.children[0];
                (
                    actor_type.get(),
                    report.category,
                    report.cleanup_count,
                    report.span.end().get() - report.span.start().get(),
                    advanced,
                    replacement.actor().get(),
                    slot.restart_attempts.clone(),
                    slot.circuit,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(projections[0], projections[1]);
        assert_eq!(projections[0], projections[2]);
        assert!(projections[0].3 > 0);
        assert_eq!(projections[0].5, 2);
        assert_eq!(projections[0].6, [1]);
        assert_eq!(projections[0].7, Some(SupervisorCircuitState::Closed));
    }
}
