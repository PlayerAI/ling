use std::collections::{BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex, MutexGuard};
use std::thread;

use ling_effects::CheckedProgram;
use ling_resolve::DefinitionId;

use crate::{Console, RuntimeFault, RuntimeFaultKind};

use super::task_runtime::{TaskPath, TaskRuntime, TaskRuntimeLimits, TaskRuntimeState, TaskValue};

const MAX_WORKERS: usize = 64;

/// Explicit, host-independent bounds for one production local Task run.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalTaskSchedulerConfig {
    worker_count: usize,
    queue_capacity: usize,
    max_direct_children_per_scope: usize,
    max_transitions: usize,
    max_park_wake_cycles: usize,
    max_shutdown_transitions: usize,
    runtime_limits: TaskRuntimeLimits,
}

impl LocalTaskSchedulerConfig {
    #[must_use]
    pub const fn new(
        worker_count: usize,
        queue_capacity: usize,
        max_direct_children_per_scope: usize,
        max_transitions: usize,
        max_park_wake_cycles: usize,
        max_shutdown_transitions: usize,
        runtime_limits: TaskRuntimeLimits,
    ) -> Self {
        Self {
            worker_count,
            queue_capacity,
            max_direct_children_per_scope,
            max_transitions,
            max_park_wake_cycles,
            max_shutdown_transitions,
            runtime_limits,
        }
    }

    /// Fixed CLI configuration. It deliberately does not inspect CPU count.
    #[must_use]
    pub const fn cli() -> Self {
        Self::new(
            4,
            1_024,
            256,
            1_000_000,
            100_000,
            64,
            TaskRuntimeLimits::new(1_024, 2_048, 1_000_000, 1_024),
        )
    }

    #[must_use]
    pub const fn worker_count(self) -> usize {
        self.worker_count
    }

    #[must_use]
    pub const fn queue_capacity(self) -> usize {
        self.queue_capacity
    }

    #[must_use]
    pub const fn max_direct_children_per_scope(self) -> usize {
        self.max_direct_children_per_scope
    }

    #[must_use]
    pub const fn max_transitions(self) -> usize {
        self.max_transitions
    }

    #[must_use]
    pub const fn max_park_wake_cycles(self) -> usize {
        self.max_park_wake_cycles
    }

    #[must_use]
    pub const fn max_shutdown_transitions(self) -> usize {
        self.max_shutdown_transitions
    }

    #[must_use]
    pub const fn runtime_limits(self) -> TaskRuntimeLimits {
        self.runtime_limits
    }
}

/// Clonable host cancellation signal for one scheduler invocation.
#[derive(Clone)]
pub struct LocalTaskControl {
    cancelled: Arc<AtomicBool>,
    wake: Arc<Condvar>,
}

impl LocalTaskControl {
    #[must_use]
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            wake: Arc::new(Condvar::new()),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
        self.wake.notify_all();
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

impl Default for LocalTaskControl {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LocalTaskMetrics {
    completed_steps: u64,
    enqueues: u64,
    dequeues: u64,
    parks: u64,
    wakes: u64,
    cancellation_observations: u64,
    worker_exits: u64,
    maximum_queue_width: usize,
}

impl LocalTaskMetrics {
    #[must_use]
    pub const fn completed_steps(&self) -> u64 {
        self.completed_steps
    }

    #[must_use]
    pub const fn enqueues(&self) -> u64 {
        self.enqueues
    }

    #[must_use]
    pub const fn dequeues(&self) -> u64 {
        self.dequeues
    }

    #[must_use]
    pub const fn parks(&self) -> u64 {
        self.parks
    }

    #[must_use]
    pub const fn wakes(&self) -> u64 {
        self.wakes
    }

    #[must_use]
    pub const fn cancellation_observations(&self) -> u64 {
        self.cancellation_observations
    }

    #[must_use]
    pub const fn worker_exits(&self) -> u64 {
        self.worker_exits
    }

    #[must_use]
    pub const fn maximum_queue_width(&self) -> usize {
        self.maximum_queue_width
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalTaskSnapshotRecord {
    path: TaskPath,
    state: TaskRuntimeState,
    cleanup_count: usize,
}

impl LocalTaskSnapshotRecord {
    #[must_use]
    pub const fn path(&self) -> &TaskPath {
        &self.path
    }

    #[must_use]
    pub const fn state(&self) -> &TaskRuntimeState {
        &self.state
    }

    #[must_use]
    pub const fn cleanup_count(&self) -> usize {
        self.cleanup_count
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalTaskSnapshot {
    epoch: u64,
    root: TaskRuntimeState,
    records: Box<[LocalTaskSnapshotRecord]>,
}

impl LocalTaskSnapshot {
    #[must_use]
    pub const fn epoch(&self) -> u64 {
        self.epoch
    }

    #[must_use]
    pub const fn root(&self) -> &TaskRuntimeState {
        &self.root
    }

    #[must_use]
    pub const fn records(&self) -> &[LocalTaskSnapshotRecord] {
        &self.records
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LocalTaskTerminal {
    Completed(TaskValue),
    Cancelled,
    Faulted(RuntimeFault),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalTaskRun {
    terminal: LocalTaskTerminal,
    snapshot: LocalTaskSnapshot,
    metrics: LocalTaskMetrics,
}

impl LocalTaskRun {
    #[must_use]
    pub const fn terminal(&self) -> &LocalTaskTerminal {
        &self.terminal
    }

    #[must_use]
    pub const fn snapshot(&self) -> &LocalTaskSnapshot {
        &self.snapshot
    }

    #[must_use]
    pub const fn metrics(&self) -> &LocalTaskMetrics {
        &self.metrics
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LocalTaskSchedulerError {
    InvalidConfiguration { reason: &'static str },
    Runtime { fault: RuntimeFault },
    Internal { reason: &'static str },
}

impl fmt::Display for LocalTaskSchedulerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { reason } => {
                write!(
                    formatter,
                    "invalid local Task scheduler configuration: {reason}"
                )
            }
            Self::Runtime { .. } => formatter.write_str("checked Task runtime failed"),
            Self::Internal { reason } => {
                write!(formatter, "internal local Task scheduler failure: {reason}")
            }
        }
    }
}

impl Error for LocalTaskSchedulerError {}

impl LocalTaskSchedulerError {
    #[must_use]
    pub fn into_runtime_fault(self, checked: &CheckedProgram, root: &DefinitionId) -> RuntimeFault {
        match self {
            Self::Runtime { fault } => fault,
            Self::InvalidConfiguration { reason } | Self::Internal { reason } => {
                scheduler_fault(checked, root, reason)
            }
        }
    }
}

struct Coordinator<'checked, 'console> {
    runtime: Option<TaskRuntime<'checked, 'console>>,
    queue: VecDeque<TaskPath>,
    queued: BTreeSet<TaskPath>,
    config: LocalTaskSchedulerConfig,
    cancellation_observed: bool,
    startup_waiters: usize,
    startup_complete: bool,
    transitions: usize,
    shutdown_transitions: usize,
    shutdown: bool,
    terminal: Option<LocalTaskTerminal>,
    failure: Option<LocalTaskSchedulerError>,
    epoch: u64,
    snapshot: LocalTaskSnapshot,
    metrics: LocalTaskMetrics,
}

struct Shared<'checked, 'console> {
    coordinator: Mutex<Coordinator<'checked, 'console>>,
    wake: Arc<Condvar>,
}

/// Runs one checked Task root through the correctness-first fixed local pool.
pub fn run_local_task(
    checked: &CheckedProgram,
    root: &DefinitionId,
    arguments: Vec<TaskValue>,
    console: &mut dyn Console,
    config: LocalTaskSchedulerConfig,
    control: &LocalTaskControl,
) -> Result<LocalTaskRun, LocalTaskSchedulerError> {
    validate_config(config)?;
    preflight_direct_children(checked, root, config)?;
    let runtime = TaskRuntime::new(checked, root, arguments, console, config.runtime_limits)
        .map_err(|fault| LocalTaskSchedulerError::Runtime { fault })?;
    let snapshot = capture_snapshot(&runtime, 0)?;
    let mut coordinator = Coordinator {
        runtime: Some(runtime),
        queue: VecDeque::new(),
        queued: BTreeSet::new(),
        config,
        cancellation_observed: false,
        startup_waiters: 0,
        startup_complete: config.worker_count == 1,
        transitions: 0,
        shutdown_transitions: 0,
        shutdown: false,
        terminal: None,
        failure: None,
        epoch: 0,
        snapshot,
        metrics: LocalTaskMetrics::default(),
    };
    coordinator.refresh_queue()?;
    let shared = Arc::new(Shared {
        coordinator: Mutex::new(coordinator),
        wake: Arc::clone(&control.wake),
    });

    thread::scope(|scope| {
        let mut workers = Vec::with_capacity(config.worker_count);
        let startup = Arc::new(Barrier::new(config.worker_count));
        for worker_id in 0..config.worker_count {
            let shared = Arc::clone(&shared);
            let control = control.clone();
            let startup = Arc::clone(&startup);
            workers.push(scope.spawn(move || {
                let first_executor = if worker_id == 0 {
                    Some(startup)
                } else {
                    startup.wait();
                    None
                };
                let result = catch_unwind(AssertUnwindSafe(|| {
                    worker_loop(&shared, &control, first_executor.as_deref())
                }));
                if result.is_err() {
                    record_worker_panic(&shared);
                }
            }));
        }
        for worker in workers {
            if worker.join().is_err() {
                record_worker_panic(&shared);
            }
        }
    });

    let coordinator = lock_after_workers(&shared);
    if let Some(error) = coordinator.failure.clone() {
        return Err(error);
    }
    let terminal = coordinator
        .terminal
        .clone()
        .ok_or(LocalTaskSchedulerError::Internal {
            reason: "workers_joined_without_terminal_state",
        })?;
    Ok(LocalTaskRun {
        terminal,
        snapshot: coordinator.snapshot.clone(),
        metrics: coordinator.metrics.clone(),
    })
}

impl<'checked, 'console> Coordinator<'checked, 'console> {
    fn runtime(&self) -> &TaskRuntime<'checked, 'console> {
        self.runtime
            .as_ref()
            .expect("runtime is present outside an exclusive worker step")
    }

    fn runtime_mut(&mut self) -> &mut TaskRuntime<'checked, 'console> {
        self.runtime
            .as_mut()
            .expect("runtime is present outside an exclusive worker step")
    }

    fn refresh_queue(&mut self) -> Result<bool, LocalTaskSchedulerError> {
        let ready = self.runtime().ready();
        if ready.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(LocalTaskSchedulerError::Internal {
                reason: "noncanonical_ready_set",
            });
        }
        let mut inserted = false;
        for path in ready {
            if self.queued.contains(&path) {
                continue;
            }
            if self.queue.len() >= self.config.queue_capacity {
                return Err(LocalTaskSchedulerError::Runtime {
                    fault: self
                        .runtime()
                        .scheduler_resource_fault("local_queue", self.config.queue_capacity),
                });
            }
            self.queued.insert(path.clone());
            self.queue.push_back(path);
            self.metrics.enqueues = self.metrics.enqueues.saturating_add(1);
            self.metrics.maximum_queue_width =
                self.metrics.maximum_queue_width.max(self.queue.len());
            inserted = true;
        }
        Ok(inserted)
    }

    fn observe_cancellation(
        &mut self,
        control: &LocalTaskControl,
    ) -> Result<bool, LocalTaskSchedulerError> {
        if self.cancellation_observed || !control.is_cancelled() {
            return Ok(false);
        }
        self.runtime_mut()
            .request_cancel(&TaskPath::root())
            .map_err(|fault| LocalTaskSchedulerError::Runtime { fault })?;
        self.cancellation_observed = true;
        self.metrics.cancellation_observations =
            self.metrics.cancellation_observations.saturating_add(1);
        self.refresh_queue()?;
        Ok(true)
    }

    fn observe_terminal(&mut self) -> Result<bool, LocalTaskSchedulerError> {
        let terminal = match self.runtime().root_state() {
            TaskRuntimeState::Completed(value) => Some(LocalTaskTerminal::Completed(value)),
            TaskRuntimeState::Cancelled => Some(LocalTaskTerminal::Cancelled),
            TaskRuntimeState::Faulted { .. } => Some(LocalTaskTerminal::Faulted(
                self.runtime()
                    .root_fault()
                    .ok_or(LocalTaskSchedulerError::Internal {
                        reason: "faulted_root_has_no_fault",
                    })?,
            )),
            _ => None,
        };
        if let Some(terminal) = terminal {
            self.terminal = Some(terminal);
            self.shutdown = true;
            return Ok(true);
        }
        Ok(false)
    }

    fn update_snapshot(&mut self) -> Result<(), LocalTaskSchedulerError> {
        self.epoch = self
            .epoch
            .checked_add(1)
            .ok_or(LocalTaskSchedulerError::Internal {
                reason: "snapshot_epoch_overflow",
            })?;
        self.snapshot = capture_snapshot(self.runtime(), self.epoch)?;
        Ok(())
    }

    fn fail(&mut self, error: LocalTaskSchedulerError) {
        if self.failure.is_none() {
            self.failure = Some(error);
        }
        self.shutdown = true;
    }
}

fn worker_loop(
    shared: &Arc<Shared<'_, '_>>,
    control: &LocalTaskControl,
    first_executor: Option<&Barrier>,
) {
    let mut startup_waiter_registered = false;
    let mut first_step = first_executor.is_some();
    loop {
        let mut coordinator = match shared.coordinator.lock() {
            Ok(coordinator) => coordinator,
            Err(poisoned) => {
                let mut coordinator = poisoned.into_inner();
                coordinator.fail(LocalTaskSchedulerError::Internal {
                    reason: "coordinator_mutex_poisoned",
                });
                shared.wake.notify_all();
                return;
            }
        };

        if coordinator.runtime.is_some() {
            if let Err(error) = coordinator.observe_cancellation(control) {
                coordinator.fail(error);
            }
        }
        if !coordinator.shutdown && coordinator.runtime.is_some() {
            match coordinator.observe_terminal() {
                Ok(_) => {}
                Err(error) => coordinator.fail(error),
            }
        }
        if coordinator.shutdown {
            record_worker_exit(&mut coordinator);
            shared.wake.notify_all();
            return;
        }

        if coordinator.runtime.is_none() || coordinator.queue.is_empty() {
            if coordinator.metrics.parks as usize >= coordinator.config.max_park_wake_cycles {
                coordinator.fail(LocalTaskSchedulerError::Internal {
                    reason: "scheduler_park_wake_limit_exhausted",
                });
                shared.wake.notify_all();
                continue;
            }
            coordinator.metrics.parks = coordinator.metrics.parks.saturating_add(1);
            if !coordinator.startup_complete && !startup_waiter_registered {
                coordinator.startup_waiters = coordinator.startup_waiters.saturating_add(1);
                startup_waiter_registered = true;
                shared.wake.notify_all();
            }
            coordinator = match shared.wake.wait(coordinator) {
                Ok(coordinator) => coordinator,
                Err(poisoned) => {
                    let mut coordinator = poisoned.into_inner();
                    coordinator.fail(LocalTaskSchedulerError::Internal {
                        reason: "coordinator_mutex_poisoned",
                    });
                    shared.wake.notify_all();
                    return;
                }
            };
            coordinator.metrics.wakes = coordinator.metrics.wakes.saturating_add(1);
            drop(coordinator);
            continue;
        }

        let path = coordinator
            .queue
            .pop_front()
            .expect("non-empty queue has a front");
        if !coordinator.queued.remove(&path) {
            coordinator.fail(LocalTaskSchedulerError::Internal {
                reason: "queue_membership_missing",
            });
            shared.wake.notify_all();
            continue;
        }
        coordinator.metrics.dequeues = coordinator.metrics.dequeues.saturating_add(1);
        if coordinator.transitions >= coordinator.config.max_transitions {
            let fault = coordinator.runtime().scheduler_resource_fault(
                "scheduler_transitions",
                coordinator.config.max_transitions,
            );
            coordinator.fail(LocalTaskSchedulerError::Runtime { fault });
            shared.wake.notify_all();
            continue;
        }

        let mut runtime = coordinator
            .runtime
            .take()
            .expect("runtime availability checked before dequeue");
        let queued = std::mem::take(&mut coordinator.queued);
        let queue = std::mem::take(&mut coordinator.queue);
        drop(coordinator);
        if first_step {
            first_executor
                .expect("first-step executor carries the startup barrier")
                .wait();
            wait_for_startup_workers(shared);
            first_step = false;
        }
        thread::yield_now();
        let step = catch_unwind(AssertUnwindSafe(|| runtime.step(&path)));
        if step.is_err() {
            let _ = catch_unwind(AssertUnwindSafe(|| {
                runtime.request_cancel(&TaskPath::root())
            }));
        }
        let mut coordinator = match shared.coordinator.lock() {
            Ok(coordinator) => coordinator,
            Err(poisoned) => poisoned.into_inner(),
        };
        if coordinator.runtime.is_some() {
            coordinator.fail(LocalTaskSchedulerError::Internal {
                reason: "runtime_lease_collision",
            });
            shared.wake.notify_all();
            return;
        }
        coordinator.runtime = Some(runtime);
        coordinator.queued = queued;
        coordinator.queue = queue;
        match step {
            Ok(Ok(_)) => {
                coordinator.transitions += 1;
                coordinator.metrics.completed_steps =
                    coordinator.metrics.completed_steps.saturating_add(1);
                if let Err(error) = coordinator.update_snapshot() {
                    coordinator.fail(error);
                }
                if !coordinator.shutdown {
                    if let Err(error) = coordinator.refresh_queue() {
                        coordinator.fail(error);
                    }
                }
                if !coordinator.shutdown {
                    if let Err(error) = coordinator.observe_terminal() {
                        coordinator.fail(error);
                    }
                }
            }
            Ok(Err(fault)) => coordinator.fail(LocalTaskSchedulerError::Runtime { fault }),
            Err(_) => {
                coordinator.fail(LocalTaskSchedulerError::Internal {
                    reason: "worker_or_host_panic",
                });
            }
        }
        shared.wake.notify_all();
        drop(coordinator);
        thread::yield_now();
    }
}

fn wait_for_startup_workers(shared: &Arc<Shared<'_, '_>>) {
    let mut coordinator = shared
        .coordinator
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let target = coordinator.config.worker_count.saturating_sub(1);
    while coordinator.startup_waiters < target && !coordinator.shutdown {
        coordinator = shared
            .wake
            .wait(coordinator)
            .unwrap_or_else(std::sync::PoisonError::into_inner);
    }
    coordinator.startup_complete = true;
}

fn record_worker_exit(coordinator: &mut Coordinator<'_, '_>) {
    coordinator.shutdown_transitions = coordinator.shutdown_transitions.saturating_add(1);
    coordinator.metrics.worker_exits = coordinator.metrics.worker_exits.saturating_add(1);
    if coordinator.shutdown_transitions > coordinator.config.max_shutdown_transitions {
        coordinator.fail(LocalTaskSchedulerError::Internal {
            reason: "shutdown_transition_limit_exhausted",
        });
    }
}

fn record_worker_panic(shared: &Arc<Shared<'_, '_>>) {
    let mut coordinator = shared
        .coordinator
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    coordinator.fail(LocalTaskSchedulerError::Internal {
        reason: "worker_join_failure",
    });
    shared.wake.notify_all();
}

fn lock_after_workers<'a, 'checked, 'console>(
    shared: &'a Arc<Shared<'checked, 'console>>,
) -> MutexGuard<'a, Coordinator<'checked, 'console>> {
    shared
        .coordinator
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn validate_config(config: LocalTaskSchedulerConfig) -> Result<(), LocalTaskSchedulerError> {
    let reason = if config.worker_count == 0 {
        Some("worker_count_zero")
    } else if config.worker_count > MAX_WORKERS {
        Some("worker_count_exceeds_64")
    } else if config.queue_capacity == 0 {
        Some("queue_capacity_zero")
    } else if config.queue_capacity > config.runtime_limits.max_tasks() {
        Some("queue_capacity_exceeds_runtime_task_limit")
    } else if config.max_direct_children_per_scope == 0 {
        Some("direct_child_limit_zero")
    } else if config.max_transitions == 0 {
        Some("transition_limit_zero")
    } else if config.max_park_wake_cycles == 0 {
        Some("park_wake_limit_zero")
    } else if config.max_shutdown_transitions < config.worker_count {
        Some("shutdown_limit_below_worker_count")
    } else if config.runtime_limits.max_tasks() == 0
        || config.runtime_limits.max_scopes() == 0
        || config.runtime_limits.max_steps() == 0
        || config.runtime_limits.max_faults() == 0
    {
        Some("runtime_limit_zero")
    } else {
        None
    };
    reason.map_or(Ok(()), |reason| {
        Err(LocalTaskSchedulerError::InvalidConfiguration { reason })
    })
}

fn preflight_direct_children(
    checked: &CheckedProgram,
    root: &DefinitionId,
    config: LocalTaskSchedulerConfig,
) -> Result<(), LocalTaskSchedulerError> {
    for core in checked.task_cores().values() {
        for scope in core.scopes() {
            let count = core
                .spawns()
                .iter()
                .filter(|spawn| spawn.scope() == scope.id())
                .count();
            if count > config.max_direct_children_per_scope {
                return Err(LocalTaskSchedulerError::Runtime {
                    fault: RuntimeFault {
                        kind: RuntimeFaultKind::TaskResourceLimit {
                            resource: "scope_direct_children",
                            limit: config.max_direct_children_per_scope,
                        },
                        source_name: definition_source(checked, core.definition()),
                        span: scope.span(),
                    },
                });
            }
        }
    }
    if checked.task_core(root).is_none() {
        return Err(LocalTaskSchedulerError::Internal {
            reason: "root_checked_task_core_absent",
        });
    }
    Ok(())
}

fn capture_snapshot(
    runtime: &TaskRuntime<'_, '_>,
    epoch: u64,
) -> Result<LocalTaskSnapshot, LocalTaskSchedulerError> {
    let records = runtime
        .task_paths()
        .into_iter()
        .map(|path| {
            let state = runtime
                .state(&path)
                .ok_or(LocalTaskSchedulerError::Internal {
                    reason: "snapshot_task_state_absent",
                })?;
            let cleanup_count =
                runtime
                    .cleanup_count(&path)
                    .ok_or(LocalTaskSchedulerError::Internal {
                        reason: "snapshot_cleanup_count_absent",
                    })?;
            Ok(LocalTaskSnapshotRecord {
                path,
                state,
                cleanup_count,
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    Ok(LocalTaskSnapshot {
        epoch,
        root: runtime.root_state(),
        records,
    })
}

fn scheduler_fault(
    checked: &CheckedProgram,
    root: &DefinitionId,
    reason: &'static str,
) -> RuntimeFault {
    let span = checked.task_core(root).map_or_else(
        || checked.typed().resolved().entry_module().hir.span,
        |core| core.source_span(),
    );
    RuntimeFault {
        kind: RuntimeFaultKind::InvalidCheckedCore { invariant: reason },
        source_name: definition_source(checked, root),
        span,
    }
}

fn definition_source(checked: &CheckedProgram, definition: &DefinitionId) -> String {
    checked
        .typed()
        .resolved()
        .definition(definition)
        .and_then(|definition| definition.source_name.clone())
        .unwrap_or_else(|| "<task>".to_owned())
}
