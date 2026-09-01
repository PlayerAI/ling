//! Publish-disabled local Actor runtime authorized by DEC-0274.
//!
//! This module is an internal checked-Core embedding boundary. It deliberately
//! does not add Actor expressions, CLI execution, bytecode, VM instructions,
//! schemas, or diagnostics.

use std::collections::{BTreeMap, VecDeque};
use std::error::Error;
use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Mutex};

use ling_concurrency::{
    ActorId, ActorTurnContract, ActorTurnDispatch, ActorTurnReentry, ActorTurnSelfSend,
    ActorTurnSpec, ActorTurnStateCommit, ActorTurnSuspension, ActorTypeId, LocalMailboxContract,
    MailboxAdmission, MailboxOverflowPolicy,
};
use ling_effects::{CheckedActorCore, CheckedProgram};
use ling_hir as hir;
use ling_resolve::{DefinitionId, DefinitionKind, ExpressionKey, ModuleId};
use ling_types::{ActorMessageSchema, SendableLocal};

use super::task_local_scheduler::LocalTaskControl;
use super::task_runtime::{TaskPath, TaskValue, closed_value_matches_type};
use super::{Console, Environment, Interpreter, RuntimeFault, RuntimeFaultKind, Value};

/// Closed values accepted by the experimental Actor embedding boundary.
pub type ActorValue = TaskValue;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActorRuntimeId(u64);

impl ActorRuntimeId {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActorSenderId(u32);

impl ActorSenderId {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalActorRef {
    runtime: ActorRuntimeId,
    actor: ActorId,
    actor_type: ActorTypeId,
    message_schema: Box<str>,
}

impl LocalActorRef {
    #[must_use]
    pub const fn runtime(&self) -> ActorRuntimeId {
        self.runtime
    }

    #[must_use]
    pub const fn actor(&self) -> ActorId {
        self.actor
    }

    #[must_use]
    pub const fn actor_type(&self) -> ActorTypeId {
        self.actor_type
    }

    #[must_use]
    pub fn message_schema(&self) -> &str {
        &self.message_schema
    }
}

/// Explicit bounds for one local Actor runtime instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActorRuntimeLimits {
    max_created_actors: usize,
    max_live_actors: usize,
    max_queued_messages: usize,
    max_commands: usize,
    max_turns: usize,
    max_events: usize,
    max_faults: usize,
    max_shutdown_work: usize,
}

impl ActorRuntimeLimits {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        max_created_actors: usize,
        max_live_actors: usize,
        max_queued_messages: usize,
        max_commands: usize,
        max_turns: usize,
        max_events: usize,
        max_faults: usize,
        max_shutdown_work: usize,
    ) -> Self {
        Self {
            max_created_actors,
            max_live_actors,
            max_queued_messages,
            max_commands,
            max_turns,
            max_events,
            max_faults,
            max_shutdown_work,
        }
    }

    /// Fixed internal configuration. This is not a public language default.
    #[must_use]
    pub const fn internal() -> Self {
        Self::new(1_024, 256, 4_096, 1_000_000, 1_000_000, 4_096, 1_024, 256)
    }

    #[must_use]
    pub const fn max_created_actors(self) -> usize {
        self.max_created_actors
    }

    #[must_use]
    pub const fn max_live_actors(self) -> usize {
        self.max_live_actors
    }

    #[must_use]
    pub const fn max_queued_messages(self) -> usize {
        self.max_queued_messages
    }

    #[must_use]
    pub const fn max_commands(self) -> usize {
        self.max_commands
    }

    #[must_use]
    pub const fn max_turns(self) -> usize {
        self.max_turns
    }

    #[must_use]
    pub const fn max_events(self) -> usize {
        self.max_events
    }

    #[must_use]
    pub const fn max_faults(self) -> usize {
        self.max_faults
    }

    #[must_use]
    pub const fn max_shutdown_work(self) -> usize {
        self.max_shutdown_work
    }

    const fn invalid_resource(self) -> Option<&'static str> {
        if self.max_created_actors == 0 {
            Some("created_actors")
        } else if self.max_live_actors == 0 || self.max_live_actors > self.max_created_actors {
            Some("live_actors")
        } else if self.max_queued_messages == 0 {
            Some("queued_messages")
        } else if self.max_commands == 0 {
            Some("commands")
        } else if self.max_turns == 0 {
            Some("turns")
        } else if self.max_events == 0 {
            Some("events")
        } else if self.max_faults < self.max_created_actors {
            Some("faults")
        } else if self.max_shutdown_work < self.max_live_actors {
            Some("shutdown_work")
        } else if self.max_created_actors > u32::MAX as usize {
            Some("created_actors")
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorRuntimeState {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorInstanceState {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorFaultPhase {
    Initializer,
    Turn,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorFault {
    runtime: ActorRuntimeId,
    actor: ActorId,
    actor_type: ActorTypeId,
    definition: DefinitionId,
    expression: ExpressionKey,
    phase: ActorFaultPhase,
    cause: RuntimeFault,
}

impl ActorFault {
    #[must_use]
    pub const fn runtime(&self) -> ActorRuntimeId {
        self.runtime
    }

    #[must_use]
    pub const fn actor(&self) -> ActorId {
        self.actor
    }

    #[must_use]
    pub const fn actor_type(&self) -> ActorTypeId {
        self.actor_type
    }

    #[must_use]
    pub fn definition(&self) -> &DefinitionId {
        &self.definition
    }

    #[must_use]
    pub const fn expression(&self) -> ExpressionKey {
        self.expression
    }

    #[must_use]
    pub const fn phase(&self) -> ActorFaultPhase {
        self.phase
    }

    #[must_use]
    pub const fn cause(&self) -> &RuntimeFault {
        &self.cause
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorShutdownReason {
    Explicit,
    OwnerCompleted,
    OwnerCancelled,
    OwnerFaulted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActorRuntimeEventKind {
    ActorStarted {
        actor: ActorId,
    },
    ActorSpawnFaulted {
        actor: ActorId,
    },
    MessageAccepted {
        actor: ActorId,
        sender: ActorSenderId,
        sender_sequence: u64,
        admission_sequence: u64,
    },
    TurnStarted {
        actor: ActorId,
        sender: ActorSenderId,
        sender_sequence: u64,
        admission_sequence: u64,
    },
    TurnCompleted {
        actor: ActorId,
    },
    ActorFaulted {
        actor: ActorId,
        discarded_messages: usize,
    },
    ActorStopped {
        actor: ActorId,
        reason: ActorShutdownReason,
        discarded_messages: usize,
    },
    RuntimeStopped {
        reason: ActorShutdownReason,
    },
    RuntimeFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorRuntimeEvent {
    sequence: u128,
    kind: ActorRuntimeEventKind,
}

impl ActorRuntimeEvent {
    #[must_use]
    pub const fn sequence(&self) -> u128 {
        self.sequence
    }

    #[must_use]
    pub const fn kind(&self) -> &ActorRuntimeEventKind {
        &self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActorRuntimeMetrics {
    created_actors: usize,
    live_actors: usize,
    queued_messages: usize,
    commands: usize,
    turns: usize,
    faults: usize,
    cleanups: usize,
    discarded_messages: usize,
}

impl ActorRuntimeMetrics {
    #[must_use]
    pub const fn created_actors(self) -> usize {
        self.created_actors
    }

    #[must_use]
    pub const fn live_actors(self) -> usize {
        self.live_actors
    }

    #[must_use]
    pub const fn queued_messages(self) -> usize {
        self.queued_messages
    }

    #[must_use]
    pub const fn commands(self) -> usize {
        self.commands
    }

    #[must_use]
    pub const fn turns(self) -> usize {
        self.turns
    }

    #[must_use]
    pub const fn faults(self) -> usize {
        self.faults
    }

    #[must_use]
    pub const fn cleanups(self) -> usize {
        self.cleanups
    }

    #[must_use]
    pub const fn discarded_messages(self) -> usize {
        self.discarded_messages
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActorInstanceSnapshot {
    reference: LocalActorRef,
    definition: DefinitionId,
    lifecycle: ActorInstanceState,
    state: Option<ActorValue>,
    queued_messages: usize,
    cleanup_count: usize,
    terminal_reason: Option<ActorShutdownReason>,
    fault: Option<ActorFault>,
}

impl ActorInstanceSnapshot {
    #[must_use]
    pub const fn reference(&self) -> &LocalActorRef {
        &self.reference
    }

    #[must_use]
    pub fn definition(&self) -> &DefinitionId {
        &self.definition
    }

    #[must_use]
    pub const fn lifecycle(&self) -> ActorInstanceState {
        self.lifecycle
    }

    #[must_use]
    pub const fn state(&self) -> Option<&ActorValue> {
        self.state.as_ref()
    }

    #[must_use]
    pub const fn queued_messages(&self) -> usize {
        self.queued_messages
    }

    #[must_use]
    pub const fn cleanup_count(&self) -> usize {
        self.cleanup_count
    }

    #[must_use]
    pub const fn terminal_reason(&self) -> Option<ActorShutdownReason> {
        self.terminal_reason
    }

    #[must_use]
    pub const fn fault(&self) -> Option<&ActorFault> {
        self.fault.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActorRuntimeError {
    InvalidRuntimeId,
    InvalidLimit {
        resource: &'static str,
    },
    RuntimeNotRunning {
        state: ActorRuntimeState,
    },
    UnknownActorDefinition {
        definition: DefinitionId,
    },
    UnknownActor {
        actor: ActorId,
    },
    InvalidReference {
        actor: ActorId,
        reason: &'static str,
    },
    ActorNotReady {
        actor: ActorId,
    },
    ResourceExhausted {
        resource: &'static str,
        limit: usize,
    },
    InvalidCheckedCore {
        invariant: &'static str,
    },
}

impl fmt::Display for ActorRuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRuntimeId => formatter.write_str("Actor runtime identity must be nonzero"),
            Self::InvalidLimit { resource } => {
                write!(formatter, "invalid Actor runtime limit: {resource}")
            }
            Self::RuntimeNotRunning { state } => {
                write!(formatter, "Actor runtime is not running: {state:?}")
            }
            Self::UnknownActorDefinition { definition } => {
                write!(formatter, "unknown checked Actor definition: {definition}")
            }
            Self::UnknownActor { actor } => {
                write!(formatter, "unknown local Actor identity: {}", actor.get())
            }
            Self::InvalidReference { actor, reason } => write!(
                formatter,
                "invalid local Actor reference {}: {reason}",
                actor.get()
            ),
            Self::ActorNotReady { actor } => {
                write!(formatter, "Actor {} is not ready", actor.get())
            }
            Self::ResourceExhausted { resource, limit } => {
                write!(
                    formatter,
                    "Actor runtime resource exhausted: {resource}={limit}"
                )
            }
            Self::InvalidCheckedCore { invariant } => {
                write!(
                    formatter,
                    "Actor checked-Core invariant failed: {invariant}"
                )
            }
        }
    }
}

impl Error for ActorRuntimeError {}

#[derive(Clone, Debug, PartialEq)]
pub enum ActorSpawnError {
    Runtime(ActorRuntimeError),
    Fault(Box<ActorFault>),
}

impl fmt::Display for ActorSpawnError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Runtime(error) => error.fmt(formatter),
            Self::Fault(fault) => write!(
                formatter,
                "Actor {} {:?} fault: {}",
                fault.actor.get(),
                fault.phase,
                fault.cause
            ),
        }
    }
}

impl Error for ActorSpawnError {}

impl From<ActorRuntimeError> for ActorSpawnError {
    fn from(error: ActorRuntimeError) -> Self {
        Self::Runtime(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActorSendErrorKind {
    InvalidSender,
    WrongRuntime,
    UnknownActor,
    ActorTypeMismatch,
    MessageSchemaMismatch,
    PayloadTypeMismatch,
    Full {
        resource: &'static str,
        limit: usize,
    },
    Closed,
    ResourceExhausted {
        resource: &'static str,
        limit: usize,
    },
    InvalidCheckedCore {
        invariant: &'static str,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActorSendError {
    kind: ActorSendErrorKind,
    payload: ActorValue,
}

impl ActorSendError {
    #[must_use]
    pub const fn kind(&self) -> &ActorSendErrorKind {
        &self.kind
    }

    #[must_use]
    pub const fn payload(&self) -> &ActorValue {
        &self.payload
    }

    #[must_use]
    pub fn into_payload(self) -> ActorValue {
        self.payload
    }
}

impl fmt::Display for ActorSendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Actor send rejected: {:?}", self.kind)
    }
}

impl Error for ActorSendError {}

#[derive(Clone, Debug, PartialEq)]
pub enum ActorTurnResult {
    Completed {
        actor: ActorId,
        state: ActorValue,
        remaining_messages: usize,
    },
    Faulted {
        actor: ActorId,
        previous_state: ActorValue,
        fault: ActorFault,
        discarded_messages: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorStopResult {
    Stopped { discarded_messages: usize },
    AlreadyStopped,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorShutdownResult {
    Stopped {
        actors: usize,
        discarded_messages: usize,
    },
    AlreadyStopped,
}

#[derive(Clone, Debug, PartialEq)]
struct Envelope {
    sender: ActorSenderId,
    sender_sequence: u64,
    admission_sequence: u64,
    payload: ActorValue,
}

#[derive(Clone, Debug)]
struct ActorInstance {
    reference: LocalActorRef,
    core: CheckedActorCore,
    lifecycle: ActorInstanceState,
    state: Option<ActorValue>,
    mailbox: VecDeque<Envelope>,
    cleanup_count: usize,
    terminal_reason: Option<ActorShutdownReason>,
    fault: Option<ActorFault>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ActorFaultPolicy {
    CancelRoot,
    SupervisorContainment,
}

/// Deterministic, run-scoped local Actor runtime over `CheckedActorCore` only.
pub struct ActorRuntime<'checked> {
    checked: &'checked CheckedProgram,
    id: ActorRuntimeId,
    state: ActorRuntimeState,
    limits: ActorRuntimeLimits,
    owner_control: LocalTaskControl,
    fault_policy: ActorFaultPolicy,
    actors: BTreeMap<ActorId, ActorInstance>,
    sender_sequences: BTreeMap<ActorSenderId, u64>,
    events: VecDeque<ActorRuntimeEvent>,
    next_event_sequence: u128,
    next_admission_sequence: u64,
    created_actors: usize,
    live_actors: usize,
    queued_messages: usize,
    commands: usize,
    turns: usize,
    faults: usize,
    cleanups: usize,
    discarded_messages: usize,
}

impl<'checked> ActorRuntime<'checked> {
    pub fn new(
        checked: &'checked CheckedProgram,
        id: ActorRuntimeId,
        limits: ActorRuntimeLimits,
        owner_control: &LocalTaskControl,
    ) -> Result<Self, ActorRuntimeError> {
        Self::new_with_fault_policy(
            checked,
            id,
            limits,
            owner_control,
            ActorFaultPolicy::CancelRoot,
        )
    }

    pub(super) fn new_supervised(
        checked: &'checked CheckedProgram,
        id: ActorRuntimeId,
        limits: ActorRuntimeLimits,
        owner_control: &LocalTaskControl,
    ) -> Result<Self, ActorRuntimeError> {
        Self::new_with_fault_policy(
            checked,
            id,
            limits,
            owner_control,
            ActorFaultPolicy::SupervisorContainment,
        )
    }

    fn new_with_fault_policy(
        checked: &'checked CheckedProgram,
        id: ActorRuntimeId,
        limits: ActorRuntimeLimits,
        owner_control: &LocalTaskControl,
        fault_policy: ActorFaultPolicy,
    ) -> Result<Self, ActorRuntimeError> {
        if !id.is_valid() {
            return Err(ActorRuntimeError::InvalidRuntimeId);
        }
        if let Some(resource) = limits.invalid_resource() {
            return Err(ActorRuntimeError::InvalidLimit { resource });
        }
        for core in checked.actor_cores().values() {
            validate_actor_core(checked, core)?;
        }
        Ok(Self {
            checked,
            id,
            state: ActorRuntimeState::Running,
            limits,
            owner_control: owner_control.clone(),
            fault_policy,
            actors: BTreeMap::new(),
            sender_sequences: BTreeMap::new(),
            events: VecDeque::new(),
            next_event_sequence: 1,
            next_admission_sequence: 1,
            created_actors: 0,
            live_actors: 0,
            queued_messages: 0,
            commands: 0,
            turns: 0,
            faults: 0,
            cleanups: 0,
            discarded_messages: 0,
        })
    }

    #[must_use]
    pub const fn id(&self) -> ActorRuntimeId {
        self.id
    }

    #[must_use]
    pub fn owner(&self) -> TaskPath {
        TaskPath::root()
    }

    #[must_use]
    pub const fn state(&self) -> ActorRuntimeState {
        self.state
    }

    #[must_use]
    pub const fn limits(&self) -> ActorRuntimeLimits {
        self.limits
    }

    #[must_use]
    pub const fn metrics(&self) -> ActorRuntimeMetrics {
        ActorRuntimeMetrics {
            created_actors: self.created_actors,
            live_actors: self.live_actors,
            queued_messages: self.queued_messages,
            commands: self.commands,
            turns: self.turns,
            faults: self.faults,
            cleanups: self.cleanups,
            discarded_messages: self.discarded_messages,
        }
    }

    #[must_use]
    pub fn ready(&self) -> Vec<ActorId> {
        if self.state != ActorRuntimeState::Running {
            return Vec::new();
        }
        self.actors
            .iter()
            .filter(|(_, actor)| {
                actor.lifecycle == ActorInstanceState::Running && !actor.mailbox.is_empty()
            })
            .map(|(id, _)| *id)
            .collect()
    }

    #[must_use]
    pub fn reference(&self, actor: ActorId) -> Option<&LocalActorRef> {
        self.actors.get(&actor).map(|instance| &instance.reference)
    }

    #[must_use]
    pub fn snapshot(&self, actor: ActorId) -> Option<ActorInstanceSnapshot> {
        self.actors
            .get(&actor)
            .map(|instance| ActorInstanceSnapshot {
                reference: instance.reference.clone(),
                definition: instance.core.definition().clone(),
                lifecycle: instance.lifecycle,
                state: instance.state.clone(),
                queued_messages: instance.mailbox.len(),
                cleanup_count: instance.cleanup_count,
                terminal_reason: instance.terminal_reason,
                fault: instance.fault.clone(),
            })
    }

    #[must_use]
    pub fn events(&self) -> Vec<ActorRuntimeEvent> {
        self.events.iter().cloned().collect()
    }

    pub fn spawn(
        &mut self,
        definition: &DefinitionId,
        console: &mut dyn Console,
    ) -> Result<LocalActorRef, ActorSpawnError> {
        self.observe_owner_cancellation()?;
        self.ensure_running()?;
        let core = self
            .checked
            .actor_core(definition)
            .cloned()
            .ok_or_else(|| ActorRuntimeError::UnknownActorDefinition {
                definition: definition.clone(),
            })?;
        let (module, initializer) =
            actor_expression(self.checked, &core, ActorFaultPhase::Initializer)
                .map(|(module, expression)| (module, expression.clone()))
                .ok_or(ActorRuntimeError::InvalidCheckedCore {
                    invariant: "Actor initializer expression is absent",
                })?;
        self.require_capacity(
            self.created_actors,
            self.limits.max_created_actors,
            "created_actors",
        )?;
        self.require_capacity(self.live_actors, self.limits.max_live_actors, "live_actors")?;
        self.require_command_capacity()?;
        self.require_event_capacity(self.live_actors + 2)?;

        self.commands += 1;
        self.created_actors += 1;
        let actor_id = ActorId::new(
            u32::try_from(self.created_actors)
                .expect("validated Actor creation bound fits a runtime Actor identity"),
        );

        let value = catch_unwind(AssertUnwindSafe(|| {
            let mut interpreter = Interpreter::new(self.checked, console);
            interpreter.eval_expression(module, &initializer, &mut Environment::new())
        }))
        .unwrap_or_else(|_| {
            Err(actor_invariant_fault(
                self.checked,
                &core,
                ActorFaultPhase::Initializer,
                "Actor initializer evaluation panicked",
            ))
        });
        let state = match value {
            Ok(value) => match ActorValue::try_from(value) {
                Ok(value) if closed_value_matches_type(&value, core.state_type(), self.checked) => {
                    value
                }
                Ok(_) => {
                    let fault = actor_invariant_fault(
                        self.checked,
                        &core,
                        ActorFaultPhase::Initializer,
                        "Actor initializer value disagrees with Checked Actor state type",
                    );
                    return Err(self.fail_spawn(actor_id, &core, fault));
                }
                Err(_) => {
                    let fault = actor_invariant_fault(
                        self.checked,
                        &core,
                        ActorFaultPhase::Initializer,
                        "Actor initializer returned a non-closed runtime value",
                    );
                    return Err(self.fail_spawn(actor_id, &core, fault));
                }
            },
            Err(fault) => return Err(self.fail_spawn(actor_id, &core, fault)),
        };

        let reference = LocalActorRef {
            runtime: self.id,
            actor: actor_id,
            actor_type: core.actor_type(),
            message_schema: core.message_contract().schema().id().as_str().into(),
        };
        let instance = ActorInstance {
            reference: reference.clone(),
            core,
            lifecycle: ActorInstanceState::Running,
            state: Some(state),
            mailbox: VecDeque::new(),
            cleanup_count: 0,
            terminal_reason: None,
            fault: None,
        };
        self.actors.insert(actor_id, instance);
        self.live_actors += 1;
        self.push_event(ActorRuntimeEventKind::ActorStarted { actor: actor_id });
        Ok(reference)
    }

    pub fn send(
        &mut self,
        reference: &LocalActorRef,
        sender: ActorSenderId,
        payload: ActorValue,
    ) -> Result<(), ActorSendError> {
        if let Err(error) = self.observe_owner_cancellation() {
            return Err(ActorSendError {
                kind: send_runtime_error_kind(error),
                payload,
            });
        }
        let reject = |kind, payload| ActorSendError { kind, payload };
        if !sender.is_valid() {
            return Err(reject(ActorSendErrorKind::InvalidSender, payload));
        }
        if reference.runtime != self.id {
            return Err(reject(ActorSendErrorKind::WrongRuntime, payload));
        }
        let Some(instance) = self.actors.get(&reference.actor) else {
            return Err(reject(ActorSendErrorKind::UnknownActor, payload));
        };
        if instance.reference.actor_type != reference.actor_type {
            return Err(reject(ActorSendErrorKind::ActorTypeMismatch, payload));
        }
        if instance.reference.message_schema != reference.message_schema {
            return Err(reject(ActorSendErrorKind::MessageSchemaMismatch, payload));
        }
        if self.state != ActorRuntimeState::Running
            || instance.lifecycle != ActorInstanceState::Running
        {
            return Err(reject(ActorSendErrorKind::Closed, payload));
        }
        if !closed_value_matches_type(&payload, instance.core.message_type(), self.checked) {
            return Err(reject(ActorSendErrorKind::PayloadTypeMismatch, payload));
        }
        if self.queued_messages >= self.limits.max_queued_messages {
            return Err(reject(
                ActorSendErrorKind::Full {
                    resource: "queued_messages",
                    limit: self.limits.max_queued_messages,
                },
                payload,
            ));
        }
        let queued = u32::try_from(instance.mailbox.len())
            .expect("checked Actor mailbox length fits its u32 capacity");
        match instance
            .core
            .mailbox_contract()
            .mailbox()
            .classify_admission(queued)
        {
            Ok(MailboxAdmission::Accepted) => {}
            Ok(MailboxAdmission::Full) => {
                return Err(reject(
                    ActorSendErrorKind::Full {
                        resource: "actor_mailbox",
                        limit: instance.core.mailbox_contract().mailbox().capacity().get() as usize,
                    },
                    payload,
                ));
            }
            Err(_) => {
                return Err(reject(
                    ActorSendErrorKind::InvalidCheckedCore {
                        invariant: "Actor mailbox length exceeds its Checked contract",
                    },
                    payload,
                ));
            }
        }
        if self.commands >= self.limits.max_commands {
            return Err(reject(
                ActorSendErrorKind::ResourceExhausted {
                    resource: "commands",
                    limit: self.limits.max_commands,
                },
                payload,
            ));
        }
        let required_events = match self.fault_policy {
            ActorFaultPolicy::CancelRoot => 1,
            ActorFaultPolicy::SupervisorContainment => self.live_actors + 2,
        };
        if let Err(error) = self.require_event_capacity(required_events) {
            return Err(reject(send_runtime_error_kind(error), payload));
        }
        let sender_sequence = self
            .sender_sequences
            .get(&sender)
            .copied()
            .unwrap_or(0)
            .checked_add(1)
            .expect("accepted Actor sends are bounded by max_commands");
        let admission_sequence = self.next_admission_sequence;
        let next_admission_sequence = admission_sequence
            .checked_add(1)
            .expect("accepted Actor sends are bounded by max_commands");

        self.commands += 1;
        self.next_admission_sequence = next_admission_sequence;
        self.sender_sequences.insert(sender, sender_sequence);
        self.actors
            .get_mut(&reference.actor)
            .expect("validated Actor remains registered")
            .mailbox
            .push_back(Envelope {
                sender,
                sender_sequence,
                admission_sequence,
                payload,
            });
        self.queued_messages += 1;
        self.push_event(ActorRuntimeEventKind::MessageAccepted {
            actor: reference.actor,
            sender,
            sender_sequence,
            admission_sequence,
        });
        Ok(())
    }

    pub fn step(
        &mut self,
        actor: ActorId,
        console: &mut dyn Console,
    ) -> Result<ActorTurnResult, ActorRuntimeError> {
        self.observe_owner_cancellation()?;
        self.ensure_running()?;
        let instance = self
            .actors
            .get(&actor)
            .ok_or(ActorRuntimeError::UnknownActor { actor })?;
        if instance.lifecycle != ActorInstanceState::Running || instance.mailbox.is_empty() {
            return Err(ActorRuntimeError::ActorNotReady { actor });
        }
        let (module, transition) =
            actor_expression(self.checked, &instance.core, ActorFaultPhase::Turn)
                .map(|(module, expression)| (module, expression.clone()))
                .ok_or(ActorRuntimeError::InvalidCheckedCore {
                    invariant: "Actor transition expression is absent",
                })?;
        self.require_capacity(self.turns, self.limits.max_turns, "turns")?;
        self.require_command_capacity()?;
        let required_events = match self.fault_policy {
            ActorFaultPolicy::CancelRoot => self.live_actors + 2,
            ActorFaultPolicy::SupervisorContainment => self.live_actors + 3,
        };
        self.require_event_capacity(required_events)?;

        self.commands += 1;
        self.turns += 1;
        let (core, previous_state, envelope) = {
            let instance = self
                .actors
                .get_mut(&actor)
                .expect("validated Actor remains registered");
            let previous_state =
                instance
                    .state
                    .clone()
                    .ok_or(ActorRuntimeError::InvalidCheckedCore {
                        invariant: "running Actor has no committed state",
                    })?;
            let envelope = instance
                .mailbox
                .pop_front()
                .expect("ready Actor has one queued message");
            (instance.core.clone(), previous_state, envelope)
        };
        self.queued_messages -= 1;
        self.push_event(ActorRuntimeEventKind::TurnStarted {
            actor,
            sender: envelope.sender,
            sender_sequence: envelope.sender_sequence,
            admission_sequence: envelope.admission_sequence,
        });

        let value = catch_unwind(AssertUnwindSafe(|| {
            #[cfg(test)]
            property_tests::panic_turn_evaluation_if_requested();
            let mut environment = Environment::new();
            environment.insert(
                core.state_binding(),
                Arc::new(Mutex::new(Value::from(previous_state.clone()))),
            );
            environment.insert(
                core.message_binding(),
                Arc::new(Mutex::new(Value::from(envelope.payload))),
            );
            let mut interpreter = Interpreter::new(self.checked, console);
            interpreter.eval_expression(module, &transition, &mut environment)
        }))
        .unwrap_or_else(|_| {
            Err(actor_invariant_fault(
                self.checked,
                &core,
                ActorFaultPhase::Turn,
                "Actor transition evaluation panicked",
            ))
        });
        let next_state = match value {
            Ok(value) => match ActorValue::try_from(value) {
                Ok(value) if closed_value_matches_type(&value, core.state_type(), self.checked) => {
                    value
                }
                Ok(_) => {
                    let cause = actor_invariant_fault(
                        self.checked,
                        &core,
                        ActorFaultPhase::Turn,
                        "Actor transition value disagrees with Checked Actor state type",
                    );
                    return Ok(self.finish_turn_fault(actor, previous_state, cause));
                }
                Err(_) => {
                    let cause = actor_invariant_fault(
                        self.checked,
                        &core,
                        ActorFaultPhase::Turn,
                        "Actor transition returned a non-closed runtime value",
                    );
                    return Ok(self.finish_turn_fault(actor, previous_state, cause));
                }
            },
            Err(cause) => return Ok(self.finish_turn_fault(actor, previous_state, cause)),
        };

        let remaining_messages = {
            let instance = self
                .actors
                .get_mut(&actor)
                .expect("turn Actor remains registered");
            instance.state = Some(next_state.clone());
            instance.mailbox.len()
        };
        self.push_event(ActorRuntimeEventKind::TurnCompleted { actor });
        Ok(ActorTurnResult::Completed {
            actor,
            state: next_state,
            remaining_messages,
        })
    }

    pub fn stop(
        &mut self,
        reference: &LocalActorRef,
    ) -> Result<ActorStopResult, ActorRuntimeError> {
        self.observe_owner_cancellation()?;
        self.validate_reference(reference)?;
        if self
            .actors
            .get(&reference.actor)
            .is_some_and(|actor| actor.lifecycle != ActorInstanceState::Running)
        {
            return Ok(ActorStopResult::AlreadyStopped);
        }
        self.require_command_capacity()?;
        self.require_event_capacity(1)?;
        self.commands += 1;
        let discarded_messages = self.stop_actor(reference.actor, ActorShutdownReason::Explicit);
        Ok(ActorStopResult::Stopped { discarded_messages })
    }

    pub fn shutdown(
        &mut self,
        reason: ActorShutdownReason,
    ) -> Result<ActorShutdownResult, ActorRuntimeError> {
        if matches!(
            self.state,
            ActorRuntimeState::Stopped | ActorRuntimeState::Failed
        ) {
            return Ok(ActorShutdownResult::AlreadyStopped);
        }
        self.require_command_capacity()?;
        self.require_event_capacity(self.live_actors + 1)?;
        self.commands += 1;
        Ok(self.shutdown_internal(reason))
    }

    /// Observes the root Task cancellation token and synchronously drains all
    /// owned Actors. Returns whether this call performed the transition.
    pub fn observe_owner_cancellation(&mut self) -> Result<bool, ActorRuntimeError> {
        if self.state == ActorRuntimeState::Running && self.owner_control.is_cancelled() {
            self.require_event_capacity(self.live_actors + 1)?;
            self.shutdown_internal(ActorShutdownReason::OwnerCancelled);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn ensure_running(&self) -> Result<(), ActorRuntimeError> {
        if self.state == ActorRuntimeState::Running {
            Ok(())
        } else {
            Err(ActorRuntimeError::RuntimeNotRunning { state: self.state })
        }
    }

    fn require_command_capacity(&self) -> Result<(), ActorRuntimeError> {
        self.require_capacity(self.commands, self.limits.max_commands, "commands")
    }

    fn require_event_capacity(&self, additional: usize) -> Result<(), ActorRuntimeError> {
        if additional <= self.limits.max_events.saturating_sub(self.events.len()) {
            Ok(())
        } else {
            Err(ActorRuntimeError::ResourceExhausted {
                resource: "events",
                limit: self.limits.max_events,
            })
        }
    }

    fn require_capacity(
        &self,
        current: usize,
        limit: usize,
        resource: &'static str,
    ) -> Result<(), ActorRuntimeError> {
        if current < limit {
            Ok(())
        } else {
            Err(ActorRuntimeError::ResourceExhausted { resource, limit })
        }
    }

    fn validate_reference(&self, reference: &LocalActorRef) -> Result<(), ActorRuntimeError> {
        if reference.runtime != self.id {
            return Err(ActorRuntimeError::InvalidReference {
                actor: reference.actor,
                reason: "runtime_identity_mismatch",
            });
        }
        let instance =
            self.actors
                .get(&reference.actor)
                .ok_or(ActorRuntimeError::UnknownActor {
                    actor: reference.actor,
                })?;
        if instance.reference.actor_type != reference.actor_type {
            return Err(ActorRuntimeError::InvalidReference {
                actor: reference.actor,
                reason: "actor_type_mismatch",
            });
        }
        if instance.reference.message_schema != reference.message_schema {
            return Err(ActorRuntimeError::InvalidReference {
                actor: reference.actor,
                reason: "message_schema_mismatch",
            });
        }
        Ok(())
    }

    fn fail_spawn(
        &mut self,
        actor: ActorId,
        core: &CheckedActorCore,
        cause: RuntimeFault,
    ) -> ActorSpawnError {
        let fault = ActorFault {
            runtime: self.id,
            actor,
            actor_type: core.actor_type(),
            definition: core.definition().clone(),
            expression: core.initializer(),
            phase: ActorFaultPhase::Initializer,
            cause,
        };
        self.faults += 1;
        self.push_event(ActorRuntimeEventKind::ActorSpawnFaulted { actor });
        self.owner_control.cancel();
        self.fail_runtime();
        ActorSpawnError::Fault(Box::new(fault))
    }

    fn finish_turn_fault(
        &mut self,
        actor: ActorId,
        previous_state: ActorValue,
        cause: RuntimeFault,
    ) -> ActorTurnResult {
        let definition = self
            .actors
            .get(&actor)
            .expect("faulting Actor remains registered")
            .core
            .definition()
            .clone();
        let fault = ActorFault {
            runtime: self.id,
            actor,
            actor_type: self
                .actors
                .get(&actor)
                .expect("faulting Actor remains registered")
                .core
                .actor_type(),
            definition,
            expression: self
                .actors
                .get(&actor)
                .expect("faulting Actor remains registered")
                .core
                .transition_body(),
            phase: ActorFaultPhase::Turn,
            cause,
        };
        let discarded_messages = {
            let instance = self
                .actors
                .get_mut(&actor)
                .expect("faulting Actor remains registered");
            let discarded = instance.mailbox.len();
            instance.mailbox.clear();
            instance.state = None;
            instance.lifecycle = ActorInstanceState::Failed;
            instance.cleanup_count += 1;
            instance.terminal_reason = Some(ActorShutdownReason::OwnerFaulted);
            instance.fault = Some(fault.clone());
            discarded
        };
        self.live_actors -= 1;
        self.queued_messages -= discarded_messages;
        self.faults += 1;
        self.cleanups += 1;
        self.discarded_messages += discarded_messages;
        self.push_event(ActorRuntimeEventKind::ActorFaulted {
            actor,
            discarded_messages,
        });
        if self.fault_policy == ActorFaultPolicy::CancelRoot {
            self.owner_control.cancel();
            self.fail_runtime();
        }
        ActorTurnResult::Faulted {
            actor,
            previous_state,
            fault,
            discarded_messages,
        }
    }

    fn fail_runtime(&mut self) {
        self.state = ActorRuntimeState::Stopping;
        let actors = self
            .actors
            .iter()
            .filter(|(_, actor)| actor.lifecycle == ActorInstanceState::Running)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        debug_assert!(actors.len() <= self.limits.max_shutdown_work());
        for actor in actors {
            self.stop_actor(actor, ActorShutdownReason::OwnerFaulted);
        }
        self.state = ActorRuntimeState::Failed;
        self.push_event(ActorRuntimeEventKind::RuntimeFailed);
    }

    pub(super) fn fail_unacknowledged_supervised_fault(&mut self) {
        debug_assert_eq!(self.fault_policy, ActorFaultPolicy::SupervisorContainment);
        debug_assert_eq!(self.state, ActorRuntimeState::Running);
        self.owner_control.cancel();
        self.fail_runtime();
    }

    fn shutdown_internal(&mut self, reason: ActorShutdownReason) -> ActorShutdownResult {
        self.state = ActorRuntimeState::Stopping;
        let actors = self
            .actors
            .iter()
            .filter(|(_, actor)| actor.lifecycle == ActorInstanceState::Running)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        debug_assert!(actors.len() <= self.limits.max_shutdown_work());
        let mut discarded_messages = 0;
        for actor in &actors {
            discarded_messages += self.stop_actor(*actor, reason);
        }
        self.state = ActorRuntimeState::Stopped;
        self.push_event(ActorRuntimeEventKind::RuntimeStopped { reason });
        ActorShutdownResult::Stopped {
            actors: actors.len(),
            discarded_messages,
        }
    }

    fn stop_actor(&mut self, actor: ActorId, reason: ActorShutdownReason) -> usize {
        let discarded_messages = {
            let instance = self
                .actors
                .get_mut(&actor)
                .expect("shutdown Actor remains registered");
            if instance.lifecycle != ActorInstanceState::Running {
                return 0;
            }
            instance.lifecycle = ActorInstanceState::Stopping;
            let discarded = instance.mailbox.len();
            instance.mailbox.clear();
            instance.state = None;
            instance.cleanup_count += 1;
            instance.terminal_reason = Some(reason);
            instance.lifecycle = ActorInstanceState::Stopped;
            discarded
        };
        self.live_actors -= 1;
        self.queued_messages -= discarded_messages;
        self.cleanups += 1;
        self.discarded_messages += discarded_messages;
        self.push_event(ActorRuntimeEventKind::ActorStopped {
            actor,
            reason,
            discarded_messages,
        });
        discarded_messages
    }

    fn push_event(&mut self, kind: ActorRuntimeEventKind) {
        debug_assert!(self.events.len() < self.limits.max_events);
        let sequence = self.next_event_sequence;
        self.next_event_sequence = self
            .next_event_sequence
            .checked_add(1)
            .expect("bounded Actor commands cannot exhaust u128 event identities");
        self.events.push_back(ActorRuntimeEvent { sequence, kind });
    }
}

fn send_runtime_error_kind(error: ActorRuntimeError) -> ActorSendErrorKind {
    match error {
        ActorRuntimeError::ResourceExhausted { resource, limit } => {
            ActorSendErrorKind::ResourceExhausted { resource, limit }
        }
        ActorRuntimeError::InvalidCheckedCore { invariant } => {
            ActorSendErrorKind::InvalidCheckedCore { invariant }
        }
        _ => ActorSendErrorKind::InvalidCheckedCore {
            invariant: "Actor owner cancellation observation failed",
        },
    }
}

fn validate_actor_core(
    checked: &CheckedProgram,
    core: &CheckedActorCore,
) -> Result<(), ActorRuntimeError> {
    let invalid = |invariant| ActorRuntimeError::InvalidCheckedCore { invariant };
    let typed = checked.typed();
    let resolved = typed.resolved();
    let definition = resolved
        .definition(core.definition())
        .ok_or_else(|| invalid("Actor definition is absent from the checked program"))?;
    if definition.kind != DefinitionKind::Actor {
        return Err(invalid("Checked Actor owner is not an Actor definition"));
    }
    if !core.actor_type().is_valid()
        || core.message_contract().actor() != core.definition()
        || core.mailbox_contract().actor() != core.definition()
        || core.mailbox_contract().actor_type() != core.actor_type()
        || core.turn_contract().actor_type() != core.actor_type()
        || core.reference_type().actor_type() != core.actor_type()
    {
        return Err(invalid(
            "Checked Actor ownership or type identity is inconsistent",
        ));
    }
    if core.message_contract().message_type() != core.message_type()
        || core.reference_type().message() != core.message_type()
        || !core.reference_type().is_local_and_invariant()
        || core.message_contract().sendability() != SendableLocal::Value
    {
        return Err(invalid(
            "Checked Actor message or reference contract is inconsistent",
        ));
    }
    let actor_ids = core.actor_id_contract();
    if !actor_ids.is_runtime_scoped()
        || !actor_ids.requires_nonzero_unique_nonreusable_ids()
        || actor_ids.allocates_instances()
    {
        return Err(invalid("Checked Actor identity contract is inconsistent"));
    }
    if !core.effects().is_pure() {
        return Err(invalid("Checked Actor transition has a residual Effect"));
    }

    let module = core.initializer().module();
    if core.transition_body().module() != module
        || core.state_binding().module() != module
        || core.message_binding().module() != module
        || core.state_binding() == core.message_binding()
    {
        return Err(invalid(
            "Checked Actor expressions or bindings have inconsistent ownership",
        ));
    }
    if typed.expression_type(core.initializer()) != Some(core.state_type())
        || typed.expression_type(core.transition_body()) != Some(core.state_type())
        || typed.binding_type(core.state_binding()) != Some(core.state_type())
        || typed.binding_type(core.message_binding()) != Some(core.message_type())
    {
        return Err(invalid(
            "Checked Actor expression or binding type is inconsistent",
        ));
    }
    if actor_expression(checked, core, ActorFaultPhase::Initializer).is_none()
        || actor_expression(checked, core, ActorFaultPhase::Turn).is_none()
    {
        return Err(invalid(
            "Checked Actor expression is absent from its owning declaration",
        ));
    }

    let schema = ActorMessageSchema::build(typed, core.message_type())
        .map_err(|_| invalid("Checked Actor message schema cannot be reconstructed"))?;
    if &schema != core.message_contract().schema() {
        return Err(invalid("Checked Actor message schema is inconsistent"));
    }
    let mailbox = core.mailbox_contract().mailbox();
    if mailbox.overflow() != MailboxOverflowPolicy::Reject
        || LocalMailboxContract::new(mailbox.capacity(), mailbox.overflow()) != *mailbox
    {
        return Err(invalid("Checked Actor mailbox contract is inconsistent"));
    }

    let turn = core.turn_contract();
    if turn.actor() != core.definition().as_str()
        || turn.dispatch() != ActorTurnDispatch::OneMessage
        || turn.suspension() != ActorTurnSuspension::Forbidden
        || turn.reentry() != ActorTurnReentry::Forbidden
        || turn.state_commit() != ActorTurnStateCommit::PublishOnNormalReturn
        || turn.self_send() != ActorTurnSelfSend::MailboxOnly
        || turn.transition() != core.transition_body().local().get()
        || turn.state_binding() != core.state_binding().local().get()
        || turn.message_binding() != core.message_binding().local().get()
    {
        return Err(invalid("Checked Actor turn contract is inconsistent"));
    }
    let reconstructed_turn = ActorTurnContract::new_checked_profile(ActorTurnSpec {
        actor: core.definition().as_str().into(),
        actor_type: core.actor_type(),
        transition: core.transition_body().local().get(),
        state_binding: core.state_binding().local().get(),
        message_binding: core.message_binding().local().get(),
        receive_span: core.source_spans().receive_clause,
        body_span: core.source_spans().transition_body,
    })
    .map_err(|_| invalid("Checked Actor turn contract cannot be reconstructed"))?;
    if reconstructed_turn != *turn {
        return Err(invalid(
            "Checked Actor turn canonical evidence is inconsistent",
        ));
    }
    Ok(())
}

fn actor_expression<'checked>(
    checked: &'checked CheckedProgram,
    core: &CheckedActorCore,
    phase: ActorFaultPhase,
) -> Option<(ModuleId, &'checked hir::Expression)> {
    let key = match phase {
        ActorFaultPhase::Initializer => core.initializer(),
        ActorFaultPhase::Turn => core.transition_body(),
    };
    let module = checked.typed().resolved().module(key.module())?;
    let actor = module.hir.actors.iter().find(|actor| {
        actor.state.initializer.id == core.initializer().local()
            && actor.receive.body.id == core.transition_body().local()
    })?;
    let expression = match phase {
        ActorFaultPhase::Initializer => &actor.state.initializer,
        ActorFaultPhase::Turn => &actor.receive.body,
    };
    (expression.id == key.local()).then_some((key.module(), expression))
}

fn actor_invariant_fault(
    checked: &CheckedProgram,
    core: &CheckedActorCore,
    phase: ActorFaultPhase,
    invariant: &'static str,
) -> RuntimeFault {
    let key: ExpressionKey = match phase {
        ActorFaultPhase::Initializer => core.initializer(),
        ActorFaultPhase::Turn => core.transition_body(),
    };
    let source_name = checked
        .typed()
        .resolved()
        .module(key.module())
        .map(|module| module.hir.source_name.clone())
        .unwrap_or_else(|| "<actor>".to_owned());
    let span = match phase {
        ActorFaultPhase::Initializer => core.source_spans().initializer,
        ActorFaultPhase::Turn => core.source_spans().transition_body,
    };
    RuntimeFault {
        kind: RuntimeFaultKind::InvalidCheckedCore { invariant },
        source_name,
        span,
    }
}

#[cfg(test)]
#[path = "actor_runtime_properties.rs"]
mod property_tests;
