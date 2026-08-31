//! Internal checked-core data for the future structured-concurrency surface.
//!
//! This crate deliberately contains no source-language parser, runtime,
//! scheduler, bytecode instruction, CLI command, or wire protocol.  It is a
//! `publish = false` workspace component used to validate the identities that
//! a later accepted Task lowering may provide.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_source::Span;

mod actor;
mod budget;
mod lifecycle;
mod mailbox;
mod mailbox_contract;
mod message;
mod property;
mod runtime;
mod scheduler;
mod state_machine;
mod supervisor;
mod turn;

pub use actor::{
    ActorId, ActorIdentityError, ActorIdentityKind, ActorIdentityModel, ActorInstance,
    ActorInstanceSpec, ActorRefId, ActorReference, ActorReferenceKind, ActorReferenceSpec,
    ActorType, ActorTypeId, ActorTypeSpec,
};
pub use budget::{
    BudgetIdentityKind, BudgetObservation, BudgetObservationError, BudgetObservationId,
    BudgetObservationKind, BudgetObservationModel, BudgetObservationSpec,
};
pub use lifecycle::{
    FaultId, LifecycleEvent, LifecycleEventKind, LifecycleEventSpec, LifecycleIdentityKind,
    LifecycleTrace, LifecycleTraceError,
};
pub use mailbox::{
    MailboxId, MailboxIdentityKind, MailboxObservation, MailboxObservationError,
    MailboxObservationKind, MailboxObservationModel, MailboxObservationSpec,
};
pub use mailbox_contract::{
    CHECKED_LOCAL_MAILBOX_VERSION, LocalMailboxContract, MAX_LOCAL_MAILBOX_CAPACITY,
    MailboxAdmission, MailboxCapacity, MailboxContractError, MailboxOverflowPolicy,
};
pub use message::{
    MessageFieldId, MessageIdentityKind, MessageSchema, MessageSchemaError, MessageSchemaId,
    MessageSchemaIdentityModel, MessageSchemaSpec,
};
pub use property::{
    PropertyIdentityKind, PropertyObservation, PropertyObservationError, PropertyObservationId,
    PropertyObservationKind, PropertyObservationModel, PropertyObservationSpec,
};
pub use runtime::{
    RuntimeIdentityKind, RuntimeObservation, RuntimeObservationError, RuntimeObservationId,
    RuntimeObservationKind, RuntimeObservationModel, RuntimeObservationSpec,
};
pub use scheduler::{
    SchedulerEventId, SchedulerIdentityKind, SchedulerObservation, SchedulerObservationKind,
    SchedulerObservationSpec, SchedulerObservationTrace, SchedulerTraceError, SchedulerTraceId,
};
pub use state_machine::{
    LocalId, MachineIdentityKind, StateEdgeKind, StateId, StateMachineError, StateMachineModel,
    StateNode, StateNodeSpec, StateTransition, StateTransitionSpec, TransitionId,
};
pub use supervisor::{
    SupervisorIdentityKind, SupervisorObservation, SupervisorObservationError,
    SupervisorObservationId, SupervisorObservationKind, SupervisorObservationModel,
    SupervisorObservationSpec,
};
pub use turn::{
    TurnId, TurnIdentityKind, TurnObservation, TurnObservationError, TurnObservationKind,
    TurnObservationModel, TurnObservationSpec,
};

macro_rules! id_type {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        impl $name {
            /// Creates an identity.  Zero is reserved for unresolved data.
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
    };
}

id_type!(ScopeId, "One lexical Task scope identity.");
id_type!(TaskId, "One task identity within a checked Task core.");
id_type!(
    TaskCoreNodeId,
    "Opaque checked body identity for a Task node."
);
id_type!(SuspensionPointId, "One explicit suspension-point identity.");
id_type!(CancellationTokenId, "One cancellation-token identity.");
id_type!(CleanupRegionId, "One cleanup-region identity.");

/// A checked suspension point.  The body identity is opaque and is never
/// interpreted by this data-only boundary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SuspensionPoint {
    id: SuspensionPointId,
    body: TaskCoreNodeId,
    source_span: Option<Span>,
}

impl SuspensionPoint {
    #[must_use]
    pub const fn new(
        id: SuspensionPointId,
        body: TaskCoreNodeId,
        source_span: Option<Span>,
    ) -> Self {
        Self {
            id,
            body,
            source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> SuspensionPointId {
        self.id
    }

    #[must_use]
    pub const fn body(self) -> TaskCoreNodeId {
        self.body
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// Capability evidence required to detach a task.  This type records the
/// identities only; it does not grant authority or perform detachment.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DetachCapability {
    owner_scope: ScopeId,
    error_channel: TaskCoreNodeId,
    cancellation: CancellationTokenId,
}

impl DetachCapability {
    #[must_use]
    pub const fn new(
        owner_scope: ScopeId,
        error_channel: TaskCoreNodeId,
        cancellation: CancellationTokenId,
    ) -> Self {
        Self {
            owner_scope,
            error_channel,
            cancellation,
        }
    }

    #[must_use]
    pub const fn owner_scope(self) -> ScopeId {
        self.owner_scope
    }

    #[must_use]
    pub const fn error_channel(self) -> TaskCoreNodeId {
        self.error_channel
    }

    #[must_use]
    pub const fn cancellation(self) -> CancellationTokenId {
        self.cancellation
    }
}

/// Inputs for one immutable checked Task node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskNodeSpec {
    pub task: TaskId,
    pub scope: ScopeId,
    pub parent: Option<TaskId>,
    pub body: TaskCoreNodeId,
    pub cancellation: CancellationTokenId,
    pub cleanup: CleanupRegionId,
    pub suspension_points: Box<[SuspensionPoint]>,
    pub detach: Option<DetachCapability>,
    pub source_span: Option<Span>,
}

/// One immutable checked Task node before it is inserted into a [`TaskCore`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskNode {
    task: TaskId,
    scope: ScopeId,
    parent: Option<TaskId>,
    body: TaskCoreNodeId,
    cancellation: CancellationTokenId,
    cleanup: CleanupRegionId,
    suspension_points: Box<[SuspensionPoint]>,
    detach: Option<DetachCapability>,
    source_span: Option<Span>,
}

impl TaskNode {
    #[must_use]
    pub fn new(spec: TaskNodeSpec) -> Self {
        let mut suspension_points = spec.suspension_points.into_vec();
        suspension_points.sort_by_key(SuspensionPoint::id);
        Self {
            task: spec.task,
            scope: spec.scope,
            parent: spec.parent,
            body: spec.body,
            cancellation: spec.cancellation,
            cleanup: spec.cleanup,
            suspension_points: suspension_points.into_boxed_slice(),
            detach: spec.detach,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn task(&self) -> TaskId {
        self.task
    }

    #[must_use]
    pub const fn scope(&self) -> ScopeId {
        self.scope
    }

    #[must_use]
    pub const fn parent(&self) -> Option<TaskId> {
        self.parent
    }

    #[must_use]
    pub const fn body(&self) -> TaskCoreNodeId {
        self.body
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
    pub fn suspension_points(&self) -> &[SuspensionPoint] {
        &self.suspension_points
    }

    #[must_use]
    pub const fn detach(&self) -> Option<DetachCapability> {
        self.detach
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }
}

/// A checked, immutable Task identity graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskCore {
    root_scope: ScopeId,
    root_task: TaskId,
    nodes: Box<[TaskNode]>,
    source_span: Option<Span>,
}

impl TaskCore {
    /// Validates and canonically orders a checked Task graph.
    pub fn new(
        root_scope: ScopeId,
        root_task: TaskId,
        nodes: impl IntoIterator<Item = TaskNode>,
        source_span: Option<Span>,
    ) -> Result<Self, TaskCoreError> {
        if !root_scope.is_valid() {
            return Err(TaskCoreError::InvalidIdentity {
                kind: IdentityKind::Scope,
                value: root_scope.get(),
            });
        }
        if !root_task.is_valid() {
            return Err(TaskCoreError::InvalidIdentity {
                kind: IdentityKind::Task,
                value: root_task.get(),
            });
        }

        let mut nodes = nodes.into_iter().collect::<Vec<_>>();
        nodes.sort_by_key(TaskNode::task);
        if nodes.is_empty() {
            return Err(TaskCoreError::MissingRoot { root: root_task });
        }

        let mut by_task = BTreeMap::new();
        for (index, node) in nodes.iter().enumerate() {
            if !node.task.is_valid() {
                return Err(TaskCoreError::InvalidIdentity {
                    kind: IdentityKind::Task,
                    value: node.task.get(),
                });
            }
            if !node.scope.is_valid() {
                return Err(TaskCoreError::InvalidIdentity {
                    kind: IdentityKind::Scope,
                    value: node.scope.get(),
                });
            }
            if !node.body.is_valid() {
                return Err(TaskCoreError::InvalidIdentity {
                    kind: IdentityKind::Body,
                    value: node.body.get(),
                });
            }
            if !node.cancellation.is_valid() {
                return Err(TaskCoreError::InvalidIdentity {
                    kind: IdentityKind::Cancellation,
                    value: node.cancellation.get(),
                });
            }
            if !node.cleanup.is_valid() {
                return Err(TaskCoreError::InvalidIdentity {
                    kind: IdentityKind::Cleanup,
                    value: node.cleanup.get(),
                });
            }
            if by_task.insert(node.task, index).is_some() {
                return Err(TaskCoreError::DuplicateTask { task: node.task });
            }
        }

        let Some(&root_index) = by_task.get(&root_task) else {
            return Err(TaskCoreError::MissingRoot { root: root_task });
        };
        if nodes[root_index].parent.is_some() {
            return Err(TaskCoreError::RootHasParent { root: root_task });
        }

        let mut suspension_ids = BTreeSet::new();
        for node in &nodes {
            for point in &node.suspension_points {
                if !point.id.is_valid() {
                    return Err(TaskCoreError::InvalidIdentity {
                        kind: IdentityKind::Suspension,
                        value: point.id.get(),
                    });
                }
                if !point.body.is_valid() {
                    return Err(TaskCoreError::InvalidIdentity {
                        kind: IdentityKind::Body,
                        value: point.body.get(),
                    });
                }
                if !suspension_ids.insert(point.id) {
                    return Err(TaskCoreError::DuplicateSuspension { point: point.id });
                }
            }
            if let Some(detach) = node.detach {
                if !detach.owner_scope.is_valid()
                    || !detach.error_channel.is_valid()
                    || !detach.cancellation.is_valid()
                {
                    return Err(TaskCoreError::InvalidDetach { task: node.task });
                }
            }
        }

        for node in &nodes {
            let Some(parent) = node.parent else {
                continue;
            };
            if !by_task.contains_key(&parent) {
                return Err(TaskCoreError::UnknownParent {
                    task: node.task,
                    parent,
                });
            }
            if parent == node.task {
                return Err(TaskCoreError::Cycle { task: node.task });
            }
            let mut current = parent;
            let mut visited = BTreeSet::new();
            while let Some(&index) = by_task.get(&current) {
                if !visited.insert(current) {
                    return Err(TaskCoreError::Cycle { task: node.task });
                }
                let Some(next) = nodes[index].parent else {
                    break;
                };
                if next == node.task {
                    return Err(TaskCoreError::Cycle { task: node.task });
                }
                current = next;
            }
        }

        Ok(Self {
            root_scope,
            root_task,
            nodes: nodes.into_boxed_slice(),
            source_span,
        })
    }

    #[must_use]
    pub const fn root_scope(&self) -> ScopeId {
        self.root_scope
    }

    #[must_use]
    pub const fn root_task(&self) -> TaskId {
        self.root_task
    }

    #[must_use]
    pub fn nodes(&self) -> &[TaskNode] {
        &self.nodes
    }

    #[must_use]
    pub fn node(&self, task: TaskId) -> Option<&TaskNode> {
        self.nodes.iter().find(|node| node.task == task)
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }

    /// Returns path-free deterministic bytes. Source evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"ling.task-core/0");
        push_u32(&mut bytes, self.root_scope.get());
        push_u32(&mut bytes, self.root_task.get());
        push_u32(&mut bytes, self.nodes.len() as u32);
        for node in &self.nodes {
            push_u32(&mut bytes, node.task.get());
            push_u32(&mut bytes, node.scope.get());
            push_u32(&mut bytes, node.parent.map_or(0, TaskId::get));
            push_u32(&mut bytes, node.body.get());
            push_u32(&mut bytes, node.cancellation.get());
            push_u32(&mut bytes, node.cleanup.get());
            match node.detach {
                Some(detach) => {
                    bytes.push(1);
                    push_u32(&mut bytes, detach.owner_scope.get());
                    push_u32(&mut bytes, detach.error_channel.get());
                    push_u32(&mut bytes, detach.cancellation.get());
                }
                None => bytes.push(0),
            }
            push_u32(&mut bytes, node.suspension_points.len() as u32);
            for point in &node.suspension_points {
                push_u32(&mut bytes, point.id.get());
                push_u32(&mut bytes, point.body.get());
            }
        }
        bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IdentityKind {
    Scope,
    Task,
    Body,
    Suspension,
    Cancellation,
    Cleanup,
}

impl IdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Scope => "scope",
            Self::Task => "task",
            Self::Body => "body",
            Self::Suspension => "suspension",
            Self::Cancellation => "cancellation",
            Self::Cleanup => "cleanup",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskCoreError {
    InvalidIdentity { kind: IdentityKind, value: u32 },
    MissingRoot { root: TaskId },
    RootHasParent { root: TaskId },
    DuplicateTask { task: TaskId },
    DuplicateSuspension { point: SuspensionPointId },
    UnknownParent { task: TaskId, parent: TaskId },
    Cycle { task: TaskId },
    InvalidDetach { task: TaskId },
}

impl fmt::Display for TaskCoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::MissingRoot { root } => write!(formatter, "root task {} is absent", root.get()),
            Self::RootHasParent { root } => {
                write!(formatter, "root task {} has a parent", root.get())
            }
            Self::DuplicateTask { task } => write!(formatter, "duplicate task {}", task.get()),
            Self::DuplicateSuspension { point } => {
                write!(formatter, "duplicate suspension point {}", point.get())
            }
            Self::UnknownParent { task, parent } => write!(
                formatter,
                "task {} references unknown parent {}",
                task.get(),
                parent.get()
            ),
            Self::Cycle { task } => write!(formatter, "task parent cycle reaches {}", task.get()),
            Self::InvalidDetach { task } => {
                write!(
                    formatter,
                    "task {} has incomplete detach capability",
                    task.get()
                )
            }
        }
    }
}

impl Error for TaskCoreError {}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_field(bytes: &mut Vec<u8>, value: &[u8]) {
    push_u32(bytes, value.len() as u32);
    bytes.extend_from_slice(value);
}

#[cfg(test)]
mod tests {
    use ling_source::{ByteOffset, SourceId};

    use super::*;

    fn span(source: u32, start: u32, end: u32) -> Span {
        Span::new(
            SourceId::new(source),
            ByteOffset::new(start),
            ByteOffset::new(end),
        )
        .expect("valid span")
    }

    fn root(source_span: Option<Span>) -> TaskNode {
        TaskNode::new(TaskNodeSpec {
            task: TaskId::new(1),
            scope: ScopeId::new(1),
            parent: None,
            body: TaskCoreNodeId::new(10),
            cancellation: CancellationTokenId::new(20),
            cleanup: CleanupRegionId::new(30),
            suspension_points: [SuspensionPoint::new(
                SuspensionPointId::new(2),
                TaskCoreNodeId::new(11),
                source_span,
            )]
            .into(),
            detach: None,
            source_span,
        })
    }

    fn child(source_span: Option<Span>) -> TaskNode {
        TaskNode::new(TaskNodeSpec {
            task: TaskId::new(2),
            scope: ScopeId::new(1),
            parent: Some(TaskId::new(1)),
            body: TaskCoreNodeId::new(12),
            cancellation: CancellationTokenId::new(21),
            cleanup: CleanupRegionId::new(31),
            suspension_points: Box::new([]),
            detach: Some(DetachCapability::new(
                ScopeId::new(1),
                TaskCoreNodeId::new(13),
                CancellationTokenId::new(22),
            )),
            source_span,
        })
    }

    #[test]
    fn validates_nested_parent_child_core_and_orders_nodes() {
        let core = TaskCore::new(
            ScopeId::new(1),
            TaskId::new(1),
            [child(Some(span(0, 20, 22))), root(Some(span(0, 1, 3)))],
            Some(span(0, 0, 24)),
        )
        .expect("nested core is valid");
        assert_eq!(core.root_scope(), ScopeId::new(1));
        assert_eq!(core.root_task(), TaskId::new(1));
        assert_eq!(
            core.nodes()
                .iter()
                .map(|node| node.task().get())
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_eq!(
            core.node(TaskId::new(2)).expect("child").parent(),
            Some(TaskId::new(1))
        );
        assert_eq!(
            core.node(TaskId::new(1))
                .expect("root")
                .suspension_points()
                .len(),
            1
        );
    }

    #[test]
    fn canonical_bytes_ignore_source_evidence_and_input_order() {
        let first = TaskCore::new(
            ScopeId::new(1),
            TaskId::new(1),
            [root(Some(span(1, 1, 2))), child(Some(span(1, 3, 4)))],
            Some(span(1, 0, 4)),
        )
        .expect("first core")
        .canonical_bytes();
        let second = TaskCore::new(
            ScopeId::new(1),
            TaskId::new(1),
            [child(Some(span(9, 30, 31))), root(Some(span(9, 10, 11)))],
            Some(span(9, 0, 31)),
        )
        .expect("second core")
        .canonical_bytes();
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_duplicate_tasks_unknown_parents_and_cycles() {
        let duplicate = TaskCore::new(
            ScopeId::new(1),
            TaskId::new(1),
            [root(None), root(None)],
            None,
        )
        .expect_err("duplicate task must be rejected");
        assert!(matches!(duplicate, TaskCoreError::DuplicateTask { .. }));

        let unknown = TaskNode::new(TaskNodeSpec {
            task: TaskId::new(2),
            scope: ScopeId::new(1),
            parent: Some(TaskId::new(9)),
            body: TaskCoreNodeId::new(12),
            cancellation: CancellationTokenId::new(21),
            cleanup: CleanupRegionId::new(31),
            suspension_points: Box::new([]),
            detach: None,
            source_span: None,
        });
        let unknown = TaskCore::new(ScopeId::new(1), TaskId::new(1), [root(None), unknown], None)
            .expect_err("unknown parent must be rejected");
        assert!(matches!(unknown, TaskCoreError::UnknownParent { .. }));

        let first = TaskNode::new(TaskNodeSpec {
            task: TaskId::new(2),
            scope: ScopeId::new(1),
            parent: Some(TaskId::new(3)),
            body: TaskCoreNodeId::new(12),
            cancellation: CancellationTokenId::new(21),
            cleanup: CleanupRegionId::new(31),
            suspension_points: Box::new([]),
            detach: None,
            source_span: None,
        });
        let second = TaskNode::new(TaskNodeSpec {
            task: TaskId::new(3),
            scope: ScopeId::new(1),
            parent: Some(TaskId::new(2)),
            body: TaskCoreNodeId::new(13),
            cancellation: CancellationTokenId::new(22),
            cleanup: CleanupRegionId::new(32),
            suspension_points: Box::new([]),
            detach: None,
            source_span: None,
        });
        let cycle = TaskCore::new(
            ScopeId::new(1),
            TaskId::new(1),
            [root(None), first, second],
            None,
        )
        .expect_err("cycle must be rejected");
        assert!(matches!(cycle, TaskCoreError::Cycle { .. }));
    }

    #[test]
    fn rejects_invalid_and_duplicate_suspension_identities() {
        let invalid = TaskNode::new(TaskNodeSpec {
            task: TaskId::new(1),
            scope: ScopeId::new(1),
            parent: None,
            body: TaskCoreNodeId::new(10),
            cancellation: CancellationTokenId::new(20),
            cleanup: CleanupRegionId::new(30),
            suspension_points: [SuspensionPoint::new(
                SuspensionPointId::new(0),
                TaskCoreNodeId::new(11),
                None,
            )]
            .into(),
            detach: None,
            source_span: None,
        });
        let invalid = TaskCore::new(ScopeId::new(1), TaskId::new(1), [invalid], None)
            .expect_err("zero suspension identity must be rejected");
        assert!(matches!(
            invalid,
            TaskCoreError::InvalidIdentity {
                kind: IdentityKind::Suspension,
                ..
            }
        ));

        let duplicate = TaskNode::new(TaskNodeSpec {
            task: TaskId::new(1),
            scope: ScopeId::new(1),
            parent: None,
            body: TaskCoreNodeId::new(10),
            cancellation: CancellationTokenId::new(20),
            cleanup: CleanupRegionId::new(30),
            suspension_points: [
                SuspensionPoint::new(SuspensionPointId::new(2), TaskCoreNodeId::new(11), None),
                SuspensionPoint::new(SuspensionPointId::new(2), TaskCoreNodeId::new(12), None),
            ]
            .into(),
            detach: None,
            source_span: None,
        });
        let duplicate = TaskCore::new(ScopeId::new(1), TaskId::new(1), [duplicate], None)
            .expect_err("duplicate suspension identity must be rejected");
        assert!(matches!(
            duplicate,
            TaskCoreError::DuplicateSuspension { .. }
        ));
    }
}
