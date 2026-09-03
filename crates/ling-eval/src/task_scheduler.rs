use std::collections::{BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Mutex};

use ling_effects::CheckedProgram;
use ling_resolve::{DefinitionId, HandlerResumeMode};
use ling_source::Span;

use crate::{Console, HostError, HostErrorCategory, RuntimeFault, RuntimeFaultKind};

use super::task_runtime::{
    TaskCancellationCause, TaskPath, TaskRuntime, TaskRuntimeLimits, TaskRuntimeState,
    TaskStepKind, TaskValue,
};

pub const TASK_SCHEDULE_TRACE_VERSION: &str = "ling.task-schedule-trace/0";

/// Explicit bounds for the publish-disabled deterministic Task test driver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskSchedulerLimits {
    max_decisions: usize,
    max_tick: u64,
    max_deadlines: usize,
    max_trace_events: usize,
    max_exploration_runs: usize,
    max_exploration_depth: usize,
    max_ready_width: usize,
}

impl TaskSchedulerLimits {
    #[must_use]
    pub const fn new(
        max_decisions: usize,
        max_tick: u64,
        max_deadlines: usize,
        max_trace_events: usize,
        max_exploration_runs: usize,
        max_exploration_depth: usize,
        max_ready_width: usize,
    ) -> Self {
        Self {
            max_decisions,
            max_tick,
            max_deadlines,
            max_trace_events,
            max_exploration_runs,
            max_exploration_depth,
            max_ready_width,
        }
    }

    #[must_use]
    pub const fn max_decisions(self) -> usize {
        self.max_decisions
    }

    #[must_use]
    pub const fn max_tick(self) -> u64 {
        self.max_tick
    }

    #[must_use]
    pub const fn max_deadlines(self) -> usize {
        self.max_deadlines
    }

    #[must_use]
    pub const fn max_trace_events(self) -> usize {
        self.max_trace_events
    }

    #[must_use]
    pub const fn max_exploration_runs(self) -> usize {
        self.max_exploration_runs
    }

    #[must_use]
    pub const fn max_exploration_depth(self) -> usize {
        self.max_exploration_depth
    }

    #[must_use]
    pub const fn max_ready_width(self) -> usize {
        self.max_ready_width
    }

    const fn is_valid(self) -> bool {
        self.max_decisions > 0
            && self.max_tick > 0
            && self.max_deadlines > 0
            && self.max_trace_events > 0
            && self.max_exploration_runs > 0
            && self.max_exploration_depth > 0
            && self.max_ready_width > 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskScheduleConfig {
    seed: u64,
    runtime_limits: TaskRuntimeLimits,
    scheduler_limits: TaskSchedulerLimits,
}

impl TaskScheduleConfig {
    #[must_use]
    pub const fn new(
        seed: u64,
        runtime_limits: TaskRuntimeLimits,
        scheduler_limits: TaskSchedulerLimits,
    ) -> Self {
        Self {
            seed,
            runtime_limits,
            scheduler_limits,
        }
    }

    #[must_use]
    pub const fn seed(self) -> u64 {
        self.seed
    }

    #[must_use]
    pub const fn runtime_limits(self) -> TaskRuntimeLimits {
        self.runtime_limits
    }

    #[must_use]
    pub const fn scheduler_limits(self) -> TaskSchedulerLimits {
        self.scheduler_limits
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TaskDeadline {
    tick: u64,
    task: TaskPath,
}

impl TaskDeadline {
    #[must_use]
    pub const fn new(tick: u64, task: TaskPath) -> Self {
        Self { tick, task }
    }

    #[must_use]
    pub const fn tick(&self) -> u64 {
        self.tick
    }

    #[must_use]
    pub const fn task(&self) -> &TaskPath {
        &self.task
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskHostResponse {
    Complete,
    Fail(HostErrorCategory),
    Panic,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaskHostScript {
    responses: Box<[TaskHostResponse]>,
}

impl TaskHostScript {
    #[must_use]
    pub fn new(responses: impl IntoIterator<Item = TaskHostResponse>) -> Self {
        Self {
            responses: responses.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn responses(&self) -> &[TaskHostResponse] {
        &self.responses
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskHostOutcome {
    Completed,
    Failed(HostErrorCategory),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskFaultSummary {
    task: TaskPath,
    category: String,
    operation: String,
    detail: String,
    source_name: String,
    source_span: Span,
}

impl TaskFaultSummary {
    #[must_use]
    pub const fn task(&self) -> &TaskPath {
        &self.task
    }

    #[must_use]
    pub fn category(&self) -> &str {
        &self.category
    }

    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }

    /// Source evidence is retained beside the logical trace but deliberately
    /// excluded from canonical bytes and replay equivalence.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn source_span(&self) -> Span {
        self.source_span
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TaskScheduleTerminal {
    Completed(TaskValue),
    Cancelled,
    Faulted { fault_count: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub enum TaskScheduleEventKind {
    Selection {
        ready: Box<[TaskPath]>,
        selected: TaskPath,
        step: TaskStepKind,
    },
    Deadline {
        task: TaskPath,
        applied: bool,
    },
    Host {
        text: String,
        outcome: TaskHostOutcome,
    },
    Closure {
        terminal: TaskScheduleTerminal,
        cleanup: Box<[(TaskPath, usize)]>,
        faults: Box<[TaskFaultSummary]>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaskScheduleEvent {
    id: u64,
    tick: u64,
    kind: TaskScheduleEventKind,
}

impl TaskScheduleEvent {
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    #[must_use]
    pub const fn tick(&self) -> u64 {
        self.tick
    }

    #[must_use]
    pub const fn kind(&self) -> &TaskScheduleEventKind {
        &self.kind
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaskScheduleTrace {
    version: String,
    config: TaskScheduleConfig,
    runtime_identity: Box<[u8]>,
    deadlines: Box<[TaskDeadline]>,
    host_script: TaskHostScript,
    events: Box<[TaskScheduleEvent]>,
}

impl TaskScheduleTrace {
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    #[must_use]
    pub const fn config(&self) -> TaskScheduleConfig {
        self.config
    }

    #[must_use]
    pub fn runtime_identity(&self) -> &[u8] {
        &self.runtime_identity
    }

    #[must_use]
    pub fn deadlines(&self) -> &[TaskDeadline] {
        &self.deadlines
    }

    #[must_use]
    pub const fn host_script(&self) -> &TaskHostScript {
        &self.host_script
    }

    #[must_use]
    pub fn events(&self) -> &[TaskScheduleEvent] {
        &self.events
    }

    pub fn validate(&self) -> Result<(), TaskSchedulerError> {
        validate_config(self.config)?;
        if self.version != TASK_SCHEDULE_TRACE_VERSION {
            return Err(TaskSchedulerError::InvalidTrace {
                event_id: None,
                reason: "unsupported_version",
            });
        }
        if self.runtime_identity.is_empty() {
            return Err(TaskSchedulerError::InvalidTrace {
                event_id: None,
                reason: "empty_runtime_identity",
            });
        }
        validate_inputs(
            self.config.scheduler_limits,
            &self.deadlines,
            &self.host_script,
        )?;
        if self.events.is_empty()
            || self.events.len() > self.config.scheduler_limits.max_trace_events
        {
            return Err(TaskSchedulerError::InvalidTrace {
                event_id: None,
                reason: "invalid_event_count",
            });
        }

        let mut last_tick = 0;
        let mut closures = 0;
        let mut last_deadline: Option<(u64, &TaskPath)> = None;
        let mut deadline_events = 0usize;
        let mut decisions = 0usize;
        for (index, event) in self.events.iter().enumerate() {
            let expected_id = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(TaskSchedulerError::InvalidTrace {
                    event_id: None,
                    reason: "event_identity_overflow",
                })?;
            if event.id != expected_id {
                return Err(TaskSchedulerError::InvalidTrace {
                    event_id: Some(event.id),
                    reason: "nonconsecutive_event_identity",
                });
            }
            if event.tick < last_tick || event.tick > self.config.scheduler_limits.max_tick {
                return Err(TaskSchedulerError::InvalidTrace {
                    event_id: Some(event.id),
                    reason: "invalid_event_tick",
                });
            }
            last_tick = event.tick;
            match &event.kind {
                TaskScheduleEventKind::Selection {
                    ready, selected, ..
                } => {
                    decisions += 1;
                    if ready.is_empty()
                        || ready.len() > self.config.scheduler_limits.max_ready_width
                        || decisions > self.config.scheduler_limits.max_decisions
                        || !is_strictly_ordered(ready)
                        || ready.binary_search(selected).is_err()
                    {
                        return Err(TaskSchedulerError::InvalidTrace {
                            event_id: Some(event.id),
                            reason: "invalid_ready_selection",
                        });
                    }
                }
                TaskScheduleEventKind::Deadline { task, .. } => {
                    let Some(input) = self.deadlines.get(deadline_events) else {
                        return Err(TaskSchedulerError::InvalidTrace {
                            event_id: Some(event.id),
                            reason: "deadline_event_has_no_input",
                        });
                    };
                    if input.tick != event.tick || input.task != *task {
                        return Err(TaskSchedulerError::InvalidTrace {
                            event_id: Some(event.id),
                            reason: "deadline_event_input_mismatch",
                        });
                    }
                    if last_deadline
                        .as_ref()
                        .is_some_and(|(tick, path)| (*tick, *path) >= (event.tick, task))
                    {
                        return Err(TaskSchedulerError::InvalidTrace {
                            event_id: Some(event.id),
                            reason: "noncanonical_deadline_event_order",
                        });
                    }
                    last_deadline = Some((event.tick, task));
                    deadline_events += 1;
                }
                TaskScheduleEventKind::Host { .. } => {}
                TaskScheduleEventKind::Closure {
                    terminal,
                    cleanup,
                    faults,
                } => {
                    closures += 1;
                    if index + 1 != self.events.len()
                        || cleanup.first().map(|item| &item.0) != Some(&TaskPath::root())
                        || !is_strictly_ordered_by(cleanup, |item| &item.0)
                        || !is_strictly_ordered_by(faults, TaskFaultSummary::task)
                        || matches!(terminal, TaskScheduleTerminal::Faulted { fault_count } if *fault_count != faults.len())
                        || matches!(
                            terminal,
                            TaskScheduleTerminal::Completed(_) | TaskScheduleTerminal::Cancelled
                        ) && !faults.is_empty()
                    {
                        return Err(TaskSchedulerError::InvalidTrace {
                            event_id: Some(event.id),
                            reason: "invalid_trace_closure",
                        });
                    }
                }
            }
        }
        if closures != 1 {
            return Err(TaskSchedulerError::InvalidTrace {
                event_id: None,
                reason: "trace_requires_one_closure",
            });
        }
        let due_deadlines = self
            .deadlines
            .iter()
            .take_while(|deadline| deadline.tick <= last_tick)
            .count();
        if deadline_events != due_deadlines {
            return Err(TaskSchedulerError::InvalidTrace {
                event_id: None,
                reason: "missing_deadline_event",
            });
        }
        Ok(())
    }

    /// Internal fixture bytes. This is not a public file or compatibility
    /// protocol and deliberately excludes source paths, spans, and host time.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();
        push_field(&mut output, self.version.as_bytes());
        encode_config(&mut output, self.config);
        push_field(&mut output, &self.runtime_identity);
        push_u64(&mut output, self.deadlines.len() as u64);
        for deadline in &self.deadlines {
            push_u64(&mut output, deadline.tick);
            encode_path(&mut output, &deadline.task);
        }
        push_u64(&mut output, self.host_script.responses.len() as u64);
        for response in &self.host_script.responses {
            encode_host_response(&mut output, *response);
        }
        push_u64(&mut output, self.events.len() as u64);
        for event in &self.events {
            encode_event(&mut output, event);
        }
        output
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskSchedulerError {
    InvalidLimit {
        limit: &'static str,
    },
    DeadlineLimit {
        limit: usize,
    },
    DuplicateDeadline {
        tick: u64,
        task: TaskPath,
    },
    DeadlineBeyondTick {
        tick: u64,
        limit: u64,
    },
    HostScriptLimit {
        limit: usize,
    },
    NoncanonicalReadySet,
    ReadyWidthLimit {
        width: usize,
        limit: usize,
    },
    DecisionLimit {
        limit: usize,
    },
    TickLimit {
        tick: u64,
        limit: u64,
    },
    TraceEventLimit {
        limit: usize,
    },
    UnknownDeadlineTask {
        tick: u64,
        task: TaskPath,
    },
    Quiescent {
        tick: u64,
    },
    InvalidSelection {
        task: TaskPath,
    },
    Runtime {
        fault: Box<TaskFaultSummary>,
    },
    ReplayDivergence {
        event_id: u64,
        reason: &'static str,
    },
    InvalidTrace {
        event_id: Option<u64>,
        reason: &'static str,
    },
}

impl fmt::Display for TaskSchedulerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimit { limit } => write!(formatter, "invalid scheduler limit `{limit}`"),
            Self::DeadlineLimit { limit } => write!(formatter, "deadline limit exceeded: {limit}"),
            Self::DuplicateDeadline { tick, task } => {
                write!(formatter, "duplicate deadline at tick {tick} for {task}")
            }
            Self::DeadlineBeyondTick { tick, limit } => {
                write!(formatter, "deadline tick {tick} exceeds {limit}")
            }
            Self::HostScriptLimit { limit } => write!(formatter, "host script exceeds {limit}"),
            Self::NoncanonicalReadySet => formatter.write_str("runtime ready set is noncanonical"),
            Self::ReadyWidthLimit { width, limit } => {
                write!(formatter, "ready width {width} exceeds {limit}")
            }
            Self::DecisionLimit { limit } => write!(formatter, "decision limit exhausted: {limit}"),
            Self::TickLimit { tick, limit } => write!(formatter, "tick {tick} exceeds {limit}"),
            Self::TraceEventLimit { limit } => write!(formatter, "trace limit exhausted: {limit}"),
            Self::UnknownDeadlineTask { tick, task } => {
                write!(formatter, "unknown deadline Task {task} at tick {tick}")
            }
            Self::Quiescent { tick } => {
                write!(formatter, "Task runtime is quiescent at tick {tick}")
            }
            Self::InvalidSelection { task } => write!(formatter, "Task {task} is not ready"),
            Self::Runtime { fault } => write!(
                formatter,
                "Task runtime fault {} for {}",
                fault.category, fault.task
            ),
            Self::ReplayDivergence { event_id, reason } => {
                write!(formatter, "replay diverged at event {event_id}: {reason}")
            }
            Self::InvalidTrace { event_id, reason } => {
                write!(formatter, "invalid trace at {event_id:?}: {reason}")
            }
        }
    }
}

impl Error for TaskSchedulerError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskReplayError {
    event_id: u64,
    reason: String,
}

impl TaskReplayError {
    #[must_use]
    pub const fn event_id(&self) -> u64 {
        self.event_id
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for TaskReplayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Task schedule replay mismatch at event {}: {}",
            self.event_id, self.reason
        )
    }
}

impl Error for TaskReplayError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskExplorationLimit {
    Runs,
    Depth,
    ReadyWidth,
    Decisions,
    Tick,
    TraceEvents,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaskExplorationResult {
    complete: bool,
    limit: Option<TaskExplorationLimit>,
    runs: usize,
    traces: Box<[TaskScheduleTrace]>,
    first_failure: Option<Box<TaskScheduleTrace>>,
}

impl TaskExplorationResult {
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    #[must_use]
    pub const fn limit(&self) -> Option<TaskExplorationLimit> {
        self.limit
    }

    #[must_use]
    pub const fn runs(&self) -> usize {
        self.runs
    }

    #[must_use]
    pub fn traces(&self) -> &[TaskScheduleTrace] {
        &self.traces
    }

    #[must_use]
    pub fn first_failure(&self) -> Option<&TaskScheduleTrace> {
        self.first_failure.as_deref()
    }
}

/// Runs one deterministic, seeded test schedule over checked Task Core.
pub fn run_task_schedule(
    checked: &CheckedProgram,
    root: &DefinitionId,
    arguments: Vec<TaskValue>,
    config: TaskScheduleConfig,
    deadlines: Vec<TaskDeadline>,
    host_script: TaskHostScript,
) -> Result<TaskScheduleTrace, TaskSchedulerError> {
    let mut selector = SeededSelector::new(config.seed);
    match drive(
        Recipe {
            checked,
            root,
            arguments: &arguments,
        },
        config,
        &deadlines,
        &host_script,
        &mut selector,
    )? {
        DriveResult::Complete(trace) => Ok(trace),
        DriveResult::Frontier { .. } => unreachable!("seeded selection never yields a frontier"),
    }
}

/// Replays every recorded choice against a freshly reconstructed runtime.
pub fn replay_task_schedule(
    checked: &CheckedProgram,
    root: &DefinitionId,
    arguments: Vec<TaskValue>,
    expected: &TaskScheduleTrace,
) -> Result<TaskScheduleTrace, TaskReplayError> {
    expected.validate().map_err(replay_error)?;
    let actual_identity = runtime_identity(checked, root, &arguments).map_err(replay_error)?;
    if actual_identity.as_ref() != expected.runtime_identity() {
        return Err(TaskReplayError {
            event_id: 0,
            reason: "runtime_identity_mismatch".to_owned(),
        });
    }
    let selections = expected
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            TaskScheduleEventKind::Selection {
                ready, selected, ..
            } => Some(RecordedSelection {
                event_id: event.id,
                tick: event.tick,
                ready: ready.clone(),
                selected: selected.clone(),
            }),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut selector = ReplaySelector::new(selections, expected.events.clone());
    let actual = match drive(
        Recipe {
            checked,
            root,
            arguments: &arguments,
        },
        expected.config,
        &expected.deadlines,
        &expected.host_script,
        &mut selector,
    )
    .map_err(replay_error)?
    {
        DriveResult::Complete(trace) => trace,
        DriveResult::Frontier { .. } => unreachable!("replay selection never yields a frontier"),
    };
    if !selector.is_exhausted() {
        return Err(TaskReplayError {
            event_id: selector.next_event_id(),
            reason: "recorded_event_not_consumed".to_owned(),
        });
    }
    compare_traces(expected, &actual)?;
    Ok(actual)
}

/// Explores finite choice prefixes in canonical breadth-first order.
pub fn explore_task_schedules(
    checked: &CheckedProgram,
    root: &DefinitionId,
    arguments: Vec<TaskValue>,
    config: TaskScheduleConfig,
    deadlines: Vec<TaskDeadline>,
    host_script: TaskHostScript,
) -> Result<TaskExplorationResult, TaskSchedulerError> {
    validate_config(config)?;
    validate_inputs(config.scheduler_limits, &deadlines, &host_script)?;
    let limits = config.scheduler_limits;
    let mut queue = VecDeque::from([Vec::<TaskPath>::new()]);
    let mut seen = BTreeSet::from([Vec::<TaskPath>::new()]);
    let mut traces = Vec::new();
    let mut first_failure = None;
    let mut runs = 0usize;

    while let Some(prefix) = queue.pop_front() {
        if runs >= limits.max_exploration_runs {
            return Ok(incomplete(
                TaskExplorationLimit::Runs,
                runs,
                traces,
                first_failure,
            ));
        }
        runs += 1;
        let mut selector = PrefixSelector::new(&prefix);
        let outcome = drive(
            Recipe {
                checked,
                root,
                arguments: &arguments,
            },
            config,
            &deadlines,
            &host_script,
            &mut selector,
        );
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(error) => {
                if let Some(limit) = exploration_limit(&error) {
                    return Ok(incomplete(limit, runs, traces, first_failure));
                }
                return Err(error);
            }
        };
        match outcome {
            DriveResult::Complete(trace) => {
                if first_failure.is_none() && trace_is_faulted(&trace) {
                    first_failure = Some(Box::new(trace.clone()));
                }
                traces.push(trace);
            }
            DriveResult::Frontier { ready } => {
                if prefix.len() >= limits.max_exploration_depth {
                    return Ok(incomplete(
                        TaskExplorationLimit::Depth,
                        runs,
                        traces,
                        first_failure,
                    ));
                }
                if ready.len() > limits.max_ready_width {
                    return Ok(incomplete(
                        TaskExplorationLimit::ReadyWidth,
                        runs,
                        traces,
                        first_failure,
                    ));
                }
                for task in ready.iter() {
                    let mut child = prefix.clone();
                    child.push(task.clone());
                    if seen.contains(&child) {
                        continue;
                    }
                    if seen.len() >= limits.max_exploration_runs {
                        return Ok(incomplete(
                            TaskExplorationLimit::Runs,
                            runs,
                            traces,
                            first_failure,
                        ));
                    }
                    seen.insert(child.clone());
                    queue.push_back(child);
                }
            }
        }
    }

    Ok(TaskExplorationResult {
        complete: true,
        limit: None,
        runs,
        traces: traces.into_boxed_slice(),
        first_failure,
    })
}

/// Exact DEC-0267 SplitMix64 transition used for one ready-set choice.
#[must_use]
pub fn task_schedule_splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

struct Recipe<'a> {
    checked: &'a CheckedProgram,
    root: &'a DefinitionId,
    arguments: &'a [TaskValue],
}

enum DriveResult {
    Complete(TaskScheduleTrace),
    Frontier { ready: Box<[TaskPath]> },
}

enum Choice {
    Select(TaskPath),
    Frontier,
}

trait Selector {
    fn choose(
        &mut self,
        tick: u64,
        ready: &[TaskPath],
        next_event_id: u64,
    ) -> Result<Choice, TaskSchedulerError>;

    fn observe(&mut self, _event: &TaskScheduleEvent) -> Result<(), TaskSchedulerError> {
        Ok(())
    }
}

struct SeededSelector {
    state: u64,
}

impl SeededSelector {
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Selector for SeededSelector {
    fn choose(
        &mut self,
        _tick: u64,
        ready: &[TaskPath],
        _next_event_id: u64,
    ) -> Result<Choice, TaskSchedulerError> {
        let output = task_schedule_splitmix64(&mut self.state);
        let width =
            u64::try_from(ready.len()).map_err(|_| TaskSchedulerError::ReadyWidthLimit {
                width: ready.len(),
                limit: usize::MAX,
            })?;
        let index = usize::try_from(output % width).expect("ready index fits usize");
        Ok(Choice::Select(ready[index].clone()))
    }
}

struct RecordedSelection {
    event_id: u64,
    tick: u64,
    ready: Box<[TaskPath]>,
    selected: TaskPath,
}

struct ReplaySelector {
    selections: Vec<RecordedSelection>,
    index: usize,
    expected_events: Box<[TaskScheduleEvent]>,
    observed: usize,
}

impl ReplaySelector {
    const fn new(
        selections: Vec<RecordedSelection>,
        expected_events: Box<[TaskScheduleEvent]>,
    ) -> Self {
        Self {
            selections,
            index: 0,
            expected_events,
            observed: 0,
        }
    }

    fn is_exhausted(&self) -> bool {
        self.index == self.selections.len() && self.observed == self.expected_events.len()
    }

    fn next_event_id(&self) -> u64 {
        self.expected_events
            .get(self.observed)
            .map_or(0, TaskScheduleEvent::id)
    }
}

impl Selector for ReplaySelector {
    fn choose(
        &mut self,
        tick: u64,
        ready: &[TaskPath],
        next_event_id: u64,
    ) -> Result<Choice, TaskSchedulerError> {
        let Some(recorded) = self.selections.get(self.index) else {
            return Err(TaskSchedulerError::ReplayDivergence {
                event_id: next_event_id,
                reason: "recorded_selection_missing",
            });
        };
        if recorded.event_id != next_event_id {
            return Err(TaskSchedulerError::ReplayDivergence {
                event_id: recorded.event_id,
                reason: "event_identity_mismatch",
            });
        }
        if recorded.tick != tick {
            return Err(TaskSchedulerError::ReplayDivergence {
                event_id: recorded.event_id,
                reason: "tick_mismatch",
            });
        }
        if recorded.ready.as_ref() != ready {
            return Err(TaskSchedulerError::ReplayDivergence {
                event_id: recorded.event_id,
                reason: "ready_set_mismatch",
            });
        }
        self.index += 1;
        Ok(Choice::Select(recorded.selected.clone()))
    }

    fn observe(&mut self, event: &TaskScheduleEvent) -> Result<(), TaskSchedulerError> {
        let Some(expected) = self.expected_events.get(self.observed) else {
            return Err(TaskSchedulerError::ReplayDivergence {
                event_id: event.id,
                reason: "unexpected_event",
            });
        };
        let mut expected_bytes = Vec::new();
        let mut actual_bytes = Vec::new();
        encode_event(&mut expected_bytes, expected);
        encode_event(&mut actual_bytes, event);
        if expected_bytes != actual_bytes {
            return Err(TaskSchedulerError::ReplayDivergence {
                event_id: expected.id.min(event.id),
                reason: "event_mismatch",
            });
        }
        self.observed += 1;
        Ok(())
    }
}

struct PrefixSelector<'a> {
    prefix: &'a [TaskPath],
    index: usize,
}

impl<'a> PrefixSelector<'a> {
    const fn new(prefix: &'a [TaskPath]) -> Self {
        Self { prefix, index: 0 }
    }
}

impl Selector for PrefixSelector<'_> {
    fn choose(
        &mut self,
        _tick: u64,
        _ready: &[TaskPath],
        _next_event_id: u64,
    ) -> Result<Choice, TaskSchedulerError> {
        let Some(task) = self.prefix.get(self.index) else {
            return Ok(Choice::Frontier);
        };
        self.index += 1;
        Ok(Choice::Select(task.clone()))
    }
}

fn drive(
    recipe: Recipe<'_>,
    config: TaskScheduleConfig,
    deadlines: &[TaskDeadline],
    host_script: &TaskHostScript,
    selector: &mut impl Selector,
) -> Result<DriveResult, TaskSchedulerError> {
    validate_config(config)?;
    let deadlines = canonical_deadlines(config.scheduler_limits, deadlines)?;
    validate_inputs(config.scheduler_limits, &deadlines, host_script)?;
    let identity = runtime_identity(recipe.checked, recipe.root, recipe.arguments)?;
    let (mut console, host_events) = DeterministicConsole::new(host_script.clone());
    let mut runtime = TaskRuntime::new(
        recipe.checked,
        recipe.root,
        recipe.arguments.to_vec(),
        &mut console,
        config.runtime_limits,
    )
    .map_err(|fault| runtime_error(TaskPath::root(), fault))?;
    let mut events = Vec::new();
    let mut deadline_index = 0usize;
    let mut host_index = 0usize;
    let mut decisions = 0usize;
    let mut tick = 0u64;

    loop {
        while let Some(deadline) = deadlines.get(deadline_index) {
            if deadline.tick > tick {
                break;
            }
            if deadline.tick < tick {
                return Err(TaskSchedulerError::InvalidTrace {
                    event_id: None,
                    reason: "missed_deadline_tick",
                });
            }
            reserve_events(&events, config.scheduler_limits, 2)?;
            let state = runtime.state(&deadline.task).ok_or_else(|| {
                TaskSchedulerError::UnknownDeadlineTask {
                    tick,
                    task: deadline.task.clone(),
                }
            })?;
            let applied = !is_terminal_state(&state);
            if applied {
                runtime
                    .request_cancel_with_cause(&deadline.task, TaskCancellationCause::Deadline)
                    .map_err(|fault| runtime_error(deadline.task.clone(), fault))?;
            }
            push_event(
                &mut events,
                config.scheduler_limits,
                tick,
                TaskScheduleEventKind::Deadline {
                    task: deadline.task.clone(),
                    applied,
                },
            )?;
            selector.observe(events.last().expect("deadline event was appended"))?;
            deadline_index += 1;
        }

        if is_terminal_state(&runtime.root_state()) {
            push_closure(&mut events, config.scheduler_limits, tick, &runtime)?;
            selector.observe(events.last().expect("closure event was appended"))?;
            let trace = TaskScheduleTrace {
                version: TASK_SCHEDULE_TRACE_VERSION.to_owned(),
                config,
                runtime_identity: identity,
                deadlines: deadlines.clone(),
                host_script: host_script.clone(),
                events: events.into_boxed_slice(),
            };
            trace.validate()?;
            return Ok(DriveResult::Complete(trace));
        }

        let ready = runtime.ready();
        if !is_strictly_ordered(&ready) {
            return Err(TaskSchedulerError::NoncanonicalReadySet);
        }
        if ready.len() > config.scheduler_limits.max_ready_width {
            return Err(TaskSchedulerError::ReadyWidthLimit {
                width: ready.len(),
                limit: config.scheduler_limits.max_ready_width,
            });
        }
        if ready.is_empty() {
            let Some(next) = deadlines.get(deadline_index) else {
                return Err(TaskSchedulerError::Quiescent { tick });
            };
            if next.tick > config.scheduler_limits.max_tick {
                return Err(TaskSchedulerError::TickLimit {
                    tick: next.tick,
                    limit: config.scheduler_limits.max_tick,
                });
            }
            tick = next.tick;
            continue;
        }
        if decisions >= config.scheduler_limits.max_decisions {
            return Err(TaskSchedulerError::DecisionLimit {
                limit: config.scheduler_limits.max_decisions,
            });
        }
        let next_event_id = next_event_id(&events)?;
        let choice = selector.choose(tick, &ready, next_event_id)?;
        let Choice::Select(selected) = choice else {
            return Ok(DriveResult::Frontier {
                ready: ready.into_boxed_slice(),
            });
        };
        if ready.binary_search(&selected).is_err() {
            return Err(TaskSchedulerError::InvalidSelection { task: selected });
        }
        let next_tick = tick.checked_add(1).ok_or(TaskSchedulerError::TickLimit {
            tick,
            limit: config.scheduler_limits.max_tick,
        })?;
        if next_tick > config.scheduler_limits.max_tick {
            return Err(TaskSchedulerError::TickLimit {
                tick: next_tick,
                limit: config.scheduler_limits.max_tick,
            });
        }
        // A runtime step emits at most one Console boundary. Reserving the
        // selection, possible host event, and final closure makes trace-limit
        // failure atomic with respect to the runtime mutation.
        reserve_events(&events, config.scheduler_limits, 3)?;
        let step = runtime
            .step(&selected)
            .map_err(|fault| runtime_error(selected.clone(), fault))?;
        decisions += 1;
        push_event(
            &mut events,
            config.scheduler_limits,
            tick,
            TaskScheduleEventKind::Selection {
                ready: ready.into_boxed_slice(),
                selected,
                step: step.kind().clone(),
            },
        )?;
        selector.observe(events.last().expect("selection event was appended"))?;
        tick = next_tick;
        let emitted = host_events.since(host_index);
        if emitted.len() > 1 {
            return Err(TaskSchedulerError::InvalidTrace {
                event_id: None,
                reason: "multiple_host_events_in_one_runtime_step",
            });
        }
        host_index += emitted.len();
        for event in emitted {
            push_event(
                &mut events,
                config.scheduler_limits,
                tick,
                TaskScheduleEventKind::Host {
                    text: event.text,
                    outcome: event.outcome,
                },
            )?;
            selector.observe(events.last().expect("host event was appended"))?;
        }
    }
}

#[derive(Clone)]
struct HostEventHandle(Arc<Mutex<Vec<HostRecord>>>);

impl HostEventHandle {
    fn since(&self, index: usize) -> Vec<HostRecord> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)[index..]
            .to_vec()
    }
}

#[derive(Clone)]
struct HostRecord {
    text: String,
    outcome: TaskHostOutcome,
}

struct DeterministicConsole {
    script: TaskHostScript,
    next: usize,
    events: HostEventHandle,
}

impl DeterministicConsole {
    fn new(script: TaskHostScript) -> (Self, HostEventHandle) {
        let events = HostEventHandle(Arc::new(Mutex::new(Vec::new())));
        (
            Self {
                script,
                next: 0,
                events: events.clone(),
            },
            events,
        )
    }
}

impl Console for DeterministicConsole {
    fn write(&mut self, text: &str) -> Result<(), HostError> {
        let response = self
            .script
            .responses
            .get(self.next)
            .copied()
            .unwrap_or(TaskHostResponse::Complete);
        self.next += 1;
        let outcome = catch_unwind(AssertUnwindSafe(|| match response {
            TaskHostResponse::Complete => Ok(TaskHostOutcome::Completed),
            TaskHostResponse::Fail(category) => {
                Err((TaskHostOutcome::Failed(category), HostError::new(category)))
            }
            TaskHostResponse::Panic => panic!("deterministic Task test-host panic"),
        }));
        match outcome {
            Ok(Ok(outcome)) => {
                self.events
                    .0
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .push(HostRecord {
                        text: text.to_owned(),
                        outcome,
                    });
                Ok(())
            }
            Ok(Err((outcome, error))) => {
                self.events
                    .0
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .push(HostRecord {
                        text: text.to_owned(),
                        outcome,
                    });
                Err(error)
            }
            Err(_) => Err(HostError::new(HostErrorCategory::Other)),
        }
    }
}

fn validate_config(config: TaskScheduleConfig) -> Result<(), TaskSchedulerError> {
    let limits = config.scheduler_limits;
    if !limits.is_valid() {
        let limit = if limits.max_decisions == 0 {
            "max_decisions"
        } else if limits.max_tick == 0 {
            "max_tick"
        } else if limits.max_deadlines == 0 {
            "max_deadlines"
        } else if limits.max_trace_events == 0 {
            "max_trace_events"
        } else if limits.max_exploration_runs == 0 {
            "max_exploration_runs"
        } else if limits.max_exploration_depth == 0 {
            "max_exploration_depth"
        } else {
            "max_ready_width"
        };
        return Err(TaskSchedulerError::InvalidLimit { limit });
    }
    if config.runtime_limits.max_tasks() == 0
        || config.runtime_limits.max_scopes() == 0
        || config.runtime_limits.max_steps() == 0
        || config.runtime_limits.max_faults() == 0
    {
        return Err(TaskSchedulerError::InvalidLimit {
            limit: "TaskRuntimeLimits",
        });
    }
    Ok(())
}

fn validate_inputs(
    limits: TaskSchedulerLimits,
    deadlines: &[TaskDeadline],
    host_script: &TaskHostScript,
) -> Result<(), TaskSchedulerError> {
    if deadlines.len() > limits.max_deadlines {
        return Err(TaskSchedulerError::DeadlineLimit {
            limit: limits.max_deadlines,
        });
    }
    if host_script.responses.len() > limits.max_trace_events {
        return Err(TaskSchedulerError::HostScriptLimit {
            limit: limits.max_trace_events,
        });
    }
    for deadline in deadlines {
        if deadline.tick > limits.max_tick {
            return Err(TaskSchedulerError::DeadlineBeyondTick {
                tick: deadline.tick,
                limit: limits.max_tick,
            });
        }
    }
    if !deadlines.windows(2).all(|pair| pair[0] < pair[1]) {
        let duplicate = deadlines
            .windows(2)
            .find(|pair| pair[0] == pair[1])
            .map(|pair| pair[0].clone());
        if let Some(deadline) = duplicate {
            return Err(TaskSchedulerError::DuplicateDeadline {
                tick: deadline.tick,
                task: deadline.task,
            });
        }
        return Err(TaskSchedulerError::InvalidTrace {
            event_id: None,
            reason: "noncanonical_deadline_input_order",
        });
    }
    Ok(())
}

fn canonical_deadlines(
    limits: TaskSchedulerLimits,
    deadlines: &[TaskDeadline],
) -> Result<Box<[TaskDeadline]>, TaskSchedulerError> {
    if deadlines.len() > limits.max_deadlines {
        return Err(TaskSchedulerError::DeadlineLimit {
            limit: limits.max_deadlines,
        });
    }
    let mut deadlines = deadlines.to_vec();
    deadlines.sort();
    if let Some(pair) = deadlines.windows(2).find(|pair| pair[0] == pair[1]) {
        return Err(TaskSchedulerError::DuplicateDeadline {
            tick: pair[0].tick,
            task: pair[0].task.clone(),
        });
    }
    for deadline in &deadlines {
        if deadline.tick > limits.max_tick {
            return Err(TaskSchedulerError::DeadlineBeyondTick {
                tick: deadline.tick,
                limit: limits.max_tick,
            });
        }
    }
    Ok(deadlines.into_boxed_slice())
}

fn runtime_identity(
    checked: &CheckedProgram,
    root: &DefinitionId,
    arguments: &[TaskValue],
) -> Result<Box<[u8]>, TaskSchedulerError> {
    let mut pending = VecDeque::from([root.clone()]);
    let mut definitions = BTreeSet::new();
    while let Some(definition) = pending.pop_front() {
        if !definitions.insert(definition.clone()) {
            continue;
        }
        let core = checked.task_core(&definition).ok_or_else(|| {
            runtime_error(
                TaskPath::root(),
                RuntimeFault {
                    kind: RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "Task scheduler recipe has no Checked Task Core",
                    },
                    source_name: String::new(),
                    span: checked.typed().resolved().entry_module().hir.span,
                },
            )
        })?;
        for spawn in core.spawns() {
            pending.push_back(spawn.target().clone());
        }
    }
    let snapshot = ling_semantic::build(checked.clone()).map_err(|_| {
        runtime_error(
            TaskPath::root(),
            RuntimeFault {
                kind: RuntimeFaultKind::InvalidCheckedCore {
                    invariant: "Task scheduler recipe has no semantic snapshot",
                },
                source_name: String::new(),
                span: checked.typed().resolved().entry_module().hir.span,
            },
        )
    })?;
    let mut output = Vec::new();
    push_field(&mut output, b"ling.task-runtime-recipe/1");
    push_u64(&mut output, definitions.len() as u64);
    for definition in definitions {
        let core = checked
            .task_core(&definition)
            .expect("collected Checked Task Core");
        let machine = checked.task_machine(&definition).ok_or_else(|| {
            runtime_error(
                TaskPath::root(),
                RuntimeFault {
                    kind: RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "Task scheduler recipe has no Checked Task machine",
                    },
                    source_name: String::new(),
                    span: core.source_span(),
                },
            )
        })?;
        let body_id = snapshot.body_id(&definition).ok_or_else(|| {
            runtime_error(
                TaskPath::root(),
                RuntimeFault {
                    kind: RuntimeFaultKind::InvalidCheckedCore {
                        invariant: "Task scheduler recipe has no semantic body identity",
                    },
                    source_name: String::new(),
                    span: core.source_span(),
                },
            )
        })?;
        push_field(&mut output, body_id.as_str().as_bytes());
        push_field(&mut output, &core.canonical_bytes(checked.typed()));
        push_field(&mut output, &machine.canonical_bytes(checked.typed()));
    }
    push_u64(&mut output, arguments.len() as u64);
    for argument in arguments {
        encode_value(&mut output, argument);
    }
    Ok(output.into_boxed_slice())
}

fn push_closure(
    events: &mut Vec<TaskScheduleEvent>,
    limits: TaskSchedulerLimits,
    tick: u64,
    runtime: &TaskRuntime<'_, '_>,
) -> Result<(), TaskSchedulerError> {
    let terminal = match runtime.root_state() {
        TaskRuntimeState::Completed(value) => TaskScheduleTerminal::Completed(value),
        TaskRuntimeState::Cancelled => TaskScheduleTerminal::Cancelled,
        TaskRuntimeState::Faulted { fault_count } => TaskScheduleTerminal::Faulted { fault_count },
        _ => {
            return Err(TaskSchedulerError::InvalidTrace {
                event_id: None,
                reason: "closure_before_terminal_state",
            });
        }
    };
    let cleanup = runtime
        .task_paths()
        .into_iter()
        .map(|path| {
            let count = runtime.cleanup_count(&path).unwrap_or_default();
            (path, count)
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let faults = runtime
        .faults(&TaskPath::root())
        .unwrap_or_default()
        .into_iter()
        .map(|(path, fault)| fault_summary(path, &fault))
        .collect::<Vec<_>>()
        .into_boxed_slice();
    push_event(
        events,
        limits,
        tick,
        TaskScheduleEventKind::Closure {
            terminal,
            cleanup,
            faults,
        },
    )
}

fn push_event(
    events: &mut Vec<TaskScheduleEvent>,
    limits: TaskSchedulerLimits,
    tick: u64,
    kind: TaskScheduleEventKind,
) -> Result<(), TaskSchedulerError> {
    reserve_events(events, limits, 1)?;
    let id = next_event_id(events)?;
    events.push(TaskScheduleEvent { id, tick, kind });
    Ok(())
}

fn next_event_id(events: &[TaskScheduleEvent]) -> Result<u64, TaskSchedulerError> {
    u64::try_from(events.len())
        .ok()
        .and_then(|value| value.checked_add(1))
        .ok_or(TaskSchedulerError::InvalidTrace {
            event_id: None,
            reason: "event_identity_overflow",
        })
}

fn reserve_events(
    events: &[TaskScheduleEvent],
    limits: TaskSchedulerLimits,
    additional: usize,
) -> Result<(), TaskSchedulerError> {
    if events.len().saturating_add(additional) > limits.max_trace_events {
        Err(TaskSchedulerError::TraceEventLimit {
            limit: limits.max_trace_events,
        })
    } else {
        Ok(())
    }
}

fn is_terminal_state(state: &TaskRuntimeState) -> bool {
    matches!(
        state,
        TaskRuntimeState::Completed(_)
            | TaskRuntimeState::Cancelled
            | TaskRuntimeState::Faulted { .. }
    )
}

fn is_strictly_ordered(values: &[TaskPath]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn is_strictly_ordered_by<T, K: Ord>(values: &[T], key: impl Fn(&T) -> &K) -> bool {
    values.windows(2).all(|pair| key(&pair[0]) < key(&pair[1]))
}

fn runtime_error(task: TaskPath, fault: RuntimeFault) -> TaskSchedulerError {
    TaskSchedulerError::Runtime {
        fault: Box::new(fault_summary(task, &fault)),
    }
}

fn fault_summary(task: TaskPath, fault: &RuntimeFault) -> TaskFaultSummary {
    let kind = &fault.kind;
    let (category, operation, detail) = match kind {
        RuntimeFaultKind::HostCapability {
            operation,
            category,
        } => ("host_capability", *operation, category.name().to_owned()),
        RuntimeFaultKind::InvalidFormatPlaceholderCount { count } => {
            ("invalid_format", "Text.format", count.to_string())
        }
        RuntimeFaultKind::DivisionByZero => ("division_by_zero", "integer.divide", String::new()),
        RuntimeFaultKind::InvalidCheckedCore { invariant } => (
            "checked_core_invariant",
            "checked_core",
            (*invariant).to_owned(),
        ),
        RuntimeFaultKind::HandlerResumeCardinality { operation, mode } => (
            "handler_resume_cardinality",
            operation.as_str(),
            resume_mode_name(*mode).to_owned(),
        ),
        RuntimeFaultKind::TaskImplementationBoundary { definition } => (
            "task_implementation_boundary",
            "interpreter.execute",
            definition.clone(),
        ),
        RuntimeFaultKind::TaskResourceLimit { resource, limit } => {
            ("resource_limit", *resource, limit.to_string())
        }
        RuntimeFaultKind::TaskDriver { reason, task } => ("task_driver", *reason, task.clone()),
        RuntimeFaultKind::TaskFaultAggregate {
            primary_task,
            fault_count,
            related_tasks,
        } => (
            "task_fault_aggregate",
            primary_task.as_str(),
            format!("{}:{}", fault_count, related_tasks.join(",")),
        ),
    };
    TaskFaultSummary {
        task,
        category: category.to_owned(),
        operation: operation.to_owned(),
        detail,
        source_name: fault.source_name.clone(),
        source_span: fault.span,
    }
}

const fn resume_mode_name(mode: HandlerResumeMode) -> &'static str {
    match mode {
        HandlerResumeMode::Never => "never",
        HandlerResumeMode::Once => "once",
        HandlerResumeMode::Many => "many",
    }
}

fn replay_error(error: TaskSchedulerError) -> TaskReplayError {
    match error {
        TaskSchedulerError::ReplayDivergence { event_id, reason } => TaskReplayError {
            event_id,
            reason: reason.to_owned(),
        },
        TaskSchedulerError::InvalidTrace { event_id, reason } => TaskReplayError {
            event_id: event_id.unwrap_or(0),
            reason: reason.to_owned(),
        },
        other => TaskReplayError {
            event_id: 0,
            reason: other.to_string(),
        },
    }
}

fn compare_traces(
    expected: &TaskScheduleTrace,
    actual: &TaskScheduleTrace,
) -> Result<(), TaskReplayError> {
    for (expected, actual) in expected.events.iter().zip(actual.events.iter()) {
        let mut expected_bytes = Vec::new();
        let mut actual_bytes = Vec::new();
        encode_event(&mut expected_bytes, expected);
        encode_event(&mut actual_bytes, actual);
        if expected_bytes != actual_bytes {
            return Err(TaskReplayError {
                event_id: expected.id.min(actual.id),
                reason: "event_mismatch".to_owned(),
            });
        }
    }
    if expected.events.len() != actual.events.len() {
        return Err(TaskReplayError {
            event_id: expected.events.len().min(actual.events.len()) as u64 + 1,
            reason: "event_count_mismatch".to_owned(),
        });
    }
    Ok(())
}

fn trace_is_faulted(trace: &TaskScheduleTrace) -> bool {
    trace.events.last().is_some_and(|event| {
        matches!(
            event.kind,
            TaskScheduleEventKind::Closure {
                terminal: TaskScheduleTerminal::Faulted { .. },
                ..
            }
        )
    })
}

fn exploration_limit(error: &TaskSchedulerError) -> Option<TaskExplorationLimit> {
    match error {
        TaskSchedulerError::ReadyWidthLimit { .. } => Some(TaskExplorationLimit::ReadyWidth),
        TaskSchedulerError::DecisionLimit { .. } => Some(TaskExplorationLimit::Decisions),
        TaskSchedulerError::TickLimit { .. } => Some(TaskExplorationLimit::Tick),
        TaskSchedulerError::TraceEventLimit { .. } => Some(TaskExplorationLimit::TraceEvents),
        _ => None,
    }
}

fn incomplete(
    limit: TaskExplorationLimit,
    runs: usize,
    traces: Vec<TaskScheduleTrace>,
    first_failure: Option<Box<TaskScheduleTrace>>,
) -> TaskExplorationResult {
    TaskExplorationResult {
        complete: false,
        limit: Some(limit),
        runs,
        traces: traces.into_boxed_slice(),
        first_failure,
    }
}

fn encode_config(output: &mut Vec<u8>, config: TaskScheduleConfig) {
    push_u64(output, config.seed);
    push_u64(output, config.runtime_limits.max_tasks() as u64);
    push_u64(output, config.runtime_limits.max_scopes() as u64);
    push_u64(output, config.runtime_limits.max_steps() as u64);
    push_u64(output, config.runtime_limits.max_faults() as u64);
    let limits = config.scheduler_limits;
    push_u64(output, limits.max_decisions as u64);
    push_u64(output, limits.max_tick);
    push_u64(output, limits.max_deadlines as u64);
    push_u64(output, limits.max_trace_events as u64);
    push_u64(output, limits.max_exploration_runs as u64);
    push_u64(output, limits.max_exploration_depth as u64);
    push_u64(output, limits.max_ready_width as u64);
}

fn encode_event(output: &mut Vec<u8>, event: &TaskScheduleEvent) {
    push_u64(output, event.id);
    push_u64(output, event.tick);
    match &event.kind {
        TaskScheduleEventKind::Selection {
            ready,
            selected,
            step,
        } => {
            output.push(0);
            push_u64(output, ready.len() as u64);
            for task in ready.iter() {
                encode_path(output, task);
            }
            encode_path(output, selected);
            encode_step(output, step);
        }
        TaskScheduleEventKind::Deadline { task, applied } => {
            output.push(1);
            encode_path(output, task);
            output.push(u8::from(*applied));
        }
        TaskScheduleEventKind::Host { text, outcome } => {
            output.push(2);
            push_field(output, text.as_bytes());
            encode_host_outcome(output, *outcome);
        }
        TaskScheduleEventKind::Closure {
            terminal,
            cleanup,
            faults,
        } => {
            output.push(3);
            encode_terminal(output, terminal);
            push_u64(output, cleanup.len() as u64);
            for (task, count) in cleanup.iter() {
                encode_path(output, task);
                push_u64(output, *count as u64);
            }
            push_u64(output, faults.len() as u64);
            for fault in faults.iter() {
                encode_path(output, &fault.task);
                push_field(output, fault.category.as_bytes());
                push_field(output, fault.operation.as_bytes());
                push_field(output, fault.detail.as_bytes());
            }
        }
    }
}

fn encode_terminal(output: &mut Vec<u8>, terminal: &TaskScheduleTerminal) {
    match terminal {
        TaskScheduleTerminal::Completed(value) => {
            output.push(0);
            encode_value(output, value);
        }
        TaskScheduleTerminal::Cancelled => output.push(1),
        TaskScheduleTerminal::Faulted { fault_count } => {
            output.push(2);
            push_u64(output, *fault_count as u64);
        }
    }
}

fn encode_step(output: &mut Vec<u8>, step: &TaskStepKind) {
    match step {
        TaskStepKind::ScopeOpened { scope } => {
            output.push(0);
            push_u64(output, u64::from(*scope));
        }
        TaskStepKind::ScopeClosed { scope } => {
            output.push(1);
            push_u64(output, u64::from(*scope));
        }
        TaskStepKind::ChildRegistered { child } => {
            output.push(2);
            encode_path(output, child);
        }
        TaskStepKind::Suspended { child } => {
            output.push(3);
            encode_path(output, child);
        }
        TaskStepKind::AwaitReady { child } => {
            output.push(4);
            encode_path(output, child);
        }
        TaskStepKind::HostEffectCompleted => output.push(5),
        TaskStepKind::JoinPending { scope } => {
            output.push(6);
            push_u64(output, u64::from(*scope));
        }
        TaskStepKind::CancellationPropagated => output.push(7),
        TaskStepKind::Completed => output.push(8),
        TaskStepKind::Cancelled => output.push(9),
        TaskStepKind::Faulted { fault_count } => {
            output.push(10);
            push_u64(output, *fault_count as u64);
        }
    }
}

fn encode_value(output: &mut Vec<u8>, value: &TaskValue) {
    match value {
        TaskValue::Unit => output.push(0),
        TaskValue::Bool(value) => {
            output.push(1);
            output.push(u8::from(*value));
        }
        TaskValue::Int(value) => {
            output.push(2);
            push_field(output, &value.to_signed_bytes_be());
        }
        TaskValue::Float64(value) => {
            output.push(3);
            push_u64(output, value.to_bits());
        }
        TaskValue::Text(value) => {
            output.push(4);
            push_field(output, value.as_bytes());
        }
        TaskValue::Tuple(values) => {
            output.push(5);
            push_u64(output, values.len() as u64);
            for value in values {
                encode_value(output, value);
            }
        }
        TaskValue::List(values) => {
            output.push(6);
            push_u64(output, values.len() as u64);
            for value in values {
                encode_value(output, value);
            }
        }
        TaskValue::Record { definition, fields } => {
            output.push(7);
            push_field(output, definition.as_str().as_bytes());
            push_u64(output, fields.len() as u64);
            for (name, value) in fields {
                push_field(output, name.as_bytes());
                encode_value(output, value);
            }
        }
        TaskValue::Variant {
            definition,
            case,
            payload,
        } => {
            output.push(8);
            push_field(output, definition.as_str().as_bytes());
            push_field(output, case.as_bytes());
            output.push(u8::from(payload.is_some()));
            if let Some(payload) = payload {
                encode_value(output, payload);
            }
        }
    }
}

fn encode_path(output: &mut Vec<u8>, path: &TaskPath) {
    push_u64(output, path.segments().len() as u64);
    for segment in path.segments() {
        push_u64(output, u64::from(*segment));
    }
}

fn encode_host_response(output: &mut Vec<u8>, response: TaskHostResponse) {
    match response {
        TaskHostResponse::Complete => output.push(0),
        TaskHostResponse::Fail(category) => {
            output.push(1);
            output.push(host_category_rank(category));
        }
        TaskHostResponse::Panic => output.push(2),
    }
}

fn encode_host_outcome(output: &mut Vec<u8>, outcome: TaskHostOutcome) {
    match outcome {
        TaskHostOutcome::Completed => output.push(0),
        TaskHostOutcome::Failed(category) => {
            output.push(1);
            output.push(host_category_rank(category));
        }
    }
}

const fn host_category_rank(category: HostErrorCategory) -> u8 {
    match category {
        HostErrorCategory::BrokenPipe => 0,
        HostErrorCategory::PermissionDenied => 1,
        HostErrorCategory::Interrupted => 2,
        HostErrorCategory::Other => 3,
    }
}

fn push_field(output: &mut Vec<u8>, value: &[u8]) {
    push_u64(output, value.len() as u64);
    output.extend_from_slice(value);
}

fn push_u64(output: &mut Vec<u8>, value: u64) {
    output.extend_from_slice(&value.to_be_bytes());
}

#[cfg(test)]
pub(crate) mod tests {
    use ling_ast::lower as lower_ast;
    use ling_effects::check;
    use ling_hir::lower as lower_hir;
    use ling_resolve::resolve;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;
    use ling_types::check as check_types;
    use num_bigint::BigInt;

    use super::*;

    fn limits() -> TaskSchedulerLimits {
        TaskSchedulerLimits::new(16, 16, 4, 64, 32, 16, 8)
    }

    fn config() -> TaskScheduleConfig {
        TaskScheduleConfig::new(0, TaskRuntimeLimits::new(8, 8, 32, 8), limits())
    }

    fn checked(text: &str) -> CheckedProgram {
        let source = SourceFile::from_bytes(
            SourceId::new(0),
            "scheduler-unit.ling",
            text.as_bytes().to_vec(),
        )
        .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = lower_hir(source.name(), &ast).expect("valid HIR");
        let resolved = resolve(vec![hir], "Main").expect("resolved");
        let typed = check_types(resolved).expect("typed");
        check(typed).expect("checked")
    }

    fn task(checked: &CheckedProgram, name: &str) -> DefinitionId {
        checked
            .task_cores()
            .keys()
            .find(|definition| {
                checked
                    .typed()
                    .resolved()
                    .definition(definition)
                    .is_some_and(|info| info.name == name)
            })
            .cloned()
            .expect("Task definition")
    }

    fn integer(value: i64) -> TaskValue {
        TaskValue::Int(BigInt::from(value))
    }

    const PARENT: &str = concat!(
        "module Main\n\n",
        "task child value =\n",
        "    scope\n",
        "        return value + 1\n\n",
        "task parent value =\n",
        "    scope\n",
        "        let handle = spawn child value\n",
        "        let result = await handle\n",
        "        return result\n",
    );

    const WRITER: &str = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "task writer text =\n",
        "    scope\n",
        "        Console.write text\n",
        "        return text\n",
    );

    #[test]
    fn splitmix64_vectors_are_frozen() {
        let mut zero = 0;
        assert_eq!(task_schedule_splitmix64(&mut zero), 0xe220_a839_7b1d_cdaf);
        assert_eq!(task_schedule_splitmix64(&mut zero), 0x6e78_9e6a_a1b9_65f4);
        let mut maximum = u64::MAX;
        assert_eq!(
            task_schedule_splitmix64(&mut maximum),
            0xe4d9_7177_1b65_2c20
        );
    }

    #[test]
    pub(crate) fn trace_validation_rejects_version_event_and_closure_corruption() {
        let closure = TaskScheduleEvent {
            id: 1,
            tick: 0,
            kind: TaskScheduleEventKind::Closure {
                terminal: TaskScheduleTerminal::Cancelled,
                cleanup: Box::new([(TaskPath::root(), 1)]),
                faults: Box::new([]),
            },
        };
        let mut trace = TaskScheduleTrace {
            version: TASK_SCHEDULE_TRACE_VERSION.to_owned(),
            config: config(),
            runtime_identity: Box::new([1]),
            deadlines: Box::new([]),
            host_script: TaskHostScript::default(),
            events: Box::new([closure]),
        };
        trace.validate().expect("minimal typed trace");
        trace.version = "bad".to_owned();
        assert!(matches!(
            trace.validate(),
            Err(TaskSchedulerError::InvalidTrace {
                reason: "unsupported_version",
                ..
            })
        ));
        trace.version = TASK_SCHEDULE_TRACE_VERSION.to_owned();
        trace.events[0].id = 2;
        assert!(matches!(
            trace.validate(),
            Err(TaskSchedulerError::InvalidTrace {
                reason: "nonconsecutive_event_identity",
                ..
            })
        ));
        trace.events[0].id = 1;
        trace.deadlines = Box::new([TaskDeadline::new(0, TaskPath::root())]);
        assert!(matches!(
            trace.validate(),
            Err(TaskSchedulerError::InvalidTrace {
                reason: "missing_deadline_event",
                ..
            })
        ));
        trace.deadlines = Box::new([]);
        trace.events = Box::new([trace.events[0].clone(), trace.events[0].clone()]);
        assert!(trace.validate().is_err());
        trace.events = Box::new([]);
        assert!(trace.validate().is_err());
    }

    #[test]
    pub(crate) fn trace_validation_rejects_truncated_and_gapped_event_sequences() {
        let checked = checked(PARENT);
        let root = task(&checked, "parent");
        let original = run_task_schedule(
            &checked,
            &root,
            vec![integer(4)],
            config(),
            vec![],
            TaskHostScript::default(),
        )
        .expect("recorded trace");
        original.validate().expect("complete trace");
        assert!(original.events.len() > 2);

        for retained in 0..original.events.len() {
            let mut prefix = original.clone();
            prefix.events = original.events[..retained].to_vec().into_boxed_slice();
            let error = prefix
                .validate()
                .expect_err("every strict prefix must be rejected");
            let expected_reason = if retained == 0 {
                "invalid_event_count"
            } else {
                "trace_requires_one_closure"
            };
            assert!(matches!(
                error,
                TaskSchedulerError::InvalidTrace { reason, .. } if reason == expected_reason
            ));
        }

        for removed in 0..original.events.len() - 1 {
            let mut gapped = original.clone();
            let mut events = gapped.events.into_vec();
            events.remove(removed);
            gapped.events = events.into_boxed_slice();
            assert!(matches!(
                gapped.validate(),
                Err(TaskSchedulerError::InvalidTrace {
                    reason: "nonconsecutive_event_identity",
                    ..
                })
            ));
        }
    }

    #[test]
    fn duplicate_and_noncanonical_deadlines_are_rejected() {
        let root = TaskPath::root();
        let duplicate = [
            TaskDeadline::new(1, root.clone()),
            TaskDeadline::new(1, root),
        ];
        assert!(matches!(
            canonical_deadlines(limits(), &duplicate),
            Err(TaskSchedulerError::DuplicateDeadline { .. })
        ));
    }

    #[test]
    pub(crate) fn replay_reports_the_first_mutated_selection_field() {
        let checked = checked(PARENT);
        let root = task(&checked, "parent");
        let original = run_task_schedule(
            &checked,
            &root,
            vec![integer(4)],
            config(),
            vec![],
            TaskHostScript::default(),
        )
        .expect("recorded trace");
        let selection_index = original
            .events
            .iter()
            .position(|event| {
                matches!(
                    &event.kind,
                    TaskScheduleEventKind::Selection { ready, .. } if ready.len() > 1
                )
            })
            .expect("multiple-ready selection");
        let selection_id = original.events[selection_index].id;

        let mut changed_choice = original.clone();
        if let TaskScheduleEventKind::Selection {
            ready, selected, ..
        } = &mut changed_choice.events[selection_index].kind
        {
            *selected = ready
                .iter()
                .find(|candidate| *candidate != selected)
                .expect("alternative ready Task")
                .clone();
        }
        changed_choice
            .validate()
            .expect("choice remains well-formed");
        let error = replay_task_schedule(&checked, &root, vec![integer(4)], &changed_choice)
            .expect_err("changed choice diverges");
        assert_eq!(error.event_id(), selection_id);

        let mut changed_step = original.clone();
        if let TaskScheduleEventKind::Selection { step, .. } =
            &mut changed_step.events[selection_index].kind
        {
            *step = TaskStepKind::Completed;
        }
        changed_step.validate().expect("step variant is typed");
        let error = replay_task_schedule(&checked, &root, vec![integer(4)], &changed_step)
            .expect_err("changed step diverges");
        assert_eq!(error.event_id(), selection_id);

        let mut changed_tick = original.clone();
        let prior_tick = changed_tick.events[selection_index - 1].tick;
        changed_tick.events[selection_index].tick = prior_tick;
        changed_tick.validate().expect("monotonic changed tick");
        let error = replay_task_schedule(&checked, &root, vec![integer(4)], &changed_tick)
            .expect_err("changed tick diverges");
        assert_eq!(error.event_id(), selection_id);

        let mut reordered_ready = original;
        if let TaskScheduleEventKind::Selection { ready, .. } =
            &mut reordered_ready.events[selection_index].kind
        {
            ready.reverse();
        }
        assert!(matches!(
            reordered_ready.validate(),
            Err(TaskSchedulerError::InvalidTrace {
                event_id: Some(id),
                reason: "invalid_ready_selection"
            }) if id == selection_id
        ));
    }

    #[test]
    pub(crate) fn replay_reports_deadline_host_and_terminal_mutations_at_their_event() {
        let echo = checked(concat!(
            "module Main\n\n",
            "task echo value =\n",
            "    scope\n",
            "        return value\n",
        ));
        let echo_root = task(&echo, "echo");
        let deadline_trace = run_task_schedule(
            &echo,
            &echo_root,
            vec![integer(1)],
            config(),
            vec![TaskDeadline::new(0, TaskPath::root())],
            TaskHostScript::default(),
        )
        .expect("deadline trace");
        let mut changed_deadline = deadline_trace.clone();
        let deadline_id = changed_deadline.events[0].id;
        if let TaskScheduleEventKind::Deadline { applied, .. } =
            &mut changed_deadline.events[0].kind
        {
            *applied = false;
        }
        let error = replay_task_schedule(&echo, &echo_root, vec![integer(1)], &changed_deadline)
            .expect_err("deadline event differs");
        assert_eq!(error.event_id(), deadline_id);

        let writer = checked(WRITER);
        let writer_root = task(&writer, "writer");
        let writer_trace = run_task_schedule(
            &writer,
            &writer_root,
            vec![TaskValue::Text("value".to_owned())],
            config(),
            vec![],
            TaskHostScript::default(),
        )
        .expect("writer trace");
        let host_index = writer_trace
            .events
            .iter()
            .position(|event| matches!(event.kind, TaskScheduleEventKind::Host { .. }))
            .expect("host event");
        let host_id = writer_trace.events[host_index].id;
        let mut changed_host = writer_trace.clone();
        if let TaskScheduleEventKind::Host { text, outcome } =
            &mut changed_host.events[host_index].kind
        {
            *text = "different\n".to_owned();
            *outcome = TaskHostOutcome::Failed(HostErrorCategory::Other);
        }
        let error = replay_task_schedule(
            &writer,
            &writer_root,
            vec![TaskValue::Text("value".to_owned())],
            &changed_host,
        )
        .expect_err("host event differs");
        assert_eq!(error.event_id(), host_id);

        let mut changed_terminal = writer_trace;
        let closure = changed_terminal.events.last_mut().expect("closure");
        let closure_id = closure.id;
        if let TaskScheduleEventKind::Closure { terminal, .. } = &mut closure.kind {
            *terminal = TaskScheduleTerminal::Cancelled;
        }
        let error = replay_task_schedule(
            &writer,
            &writer_root,
            vec![TaskValue::Text("value".to_owned())],
            &changed_terminal,
        )
        .expect_err("terminal event differs");
        assert_eq!(error.event_id(), closure_id);
    }
}
