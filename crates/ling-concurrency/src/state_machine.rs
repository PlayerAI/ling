//! Data-only state-machine projection for the future Task lowering boundary.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_source::Span;

use crate::{TaskCoreNodeId, TaskId};

macro_rules! id_type {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        impl $name {
            /// Creates an identity. Zero is reserved for unresolved data.
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

id_type!(
    StateId,
    "One state identity in an internal Task state machine."
);
id_type!(
    LocalId,
    "One opaque live-local identity in a state snapshot."
);
id_type!(TransitionId, "One structural transition identity.");

/// The edge labels are structural observations only. They do not define
/// cancellation, cleanup, Fault, or scheduling semantics.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StateEdgeKind {
    Resume,
    Cancel,
    Cleanup,
    Fault,
}

impl StateEdgeKind {
    const fn rank(self) -> u8 {
        match self {
            Self::Resume => 0,
            Self::Cancel => 1,
            Self::Cleanup => 2,
            Self::Fault => 3,
        }
    }
}

/// Inputs for one state node. Continuation and local identities are opaque.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNodeSpec {
    pub id: StateId,
    pub continuation: TaskCoreNodeId,
    pub live_locals: Box<[LocalId]>,
    pub source_span: Option<Span>,
}

/// One immutable state node with deterministic local ordering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateNode {
    id: StateId,
    continuation: TaskCoreNodeId,
    live_locals: Box<[LocalId]>,
    source_span: Option<Span>,
}

impl StateNode {
    #[must_use]
    pub fn new(spec: StateNodeSpec) -> Self {
        let mut live_locals = spec.live_locals.into_vec();
        live_locals.sort();
        Self {
            id: spec.id,
            continuation: spec.continuation,
            live_locals: live_locals.into_boxed_slice(),
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> StateId {
        self.id
    }

    #[must_use]
    pub const fn continuation(&self) -> TaskCoreNodeId {
        self.continuation
    }

    #[must_use]
    pub fn live_locals(&self) -> &[LocalId] {
        &self.live_locals
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }
}

/// Inputs for one structural state edge.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StateTransitionSpec {
    pub id: TransitionId,
    pub from: StateId,
    pub to: StateId,
    pub kind: StateEdgeKind,
    pub source_span: Option<Span>,
}

/// One immutable structural state edge.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StateTransition {
    id: TransitionId,
    from: StateId,
    to: StateId,
    kind: StateEdgeKind,
    source_span: Option<Span>,
}

impl StateTransition {
    #[must_use]
    pub const fn new(spec: StateTransitionSpec) -> Self {
        Self {
            id: spec.id,
            from: spec.from,
            to: spec.to,
            kind: spec.kind,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> TransitionId {
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
    pub const fn kind(self) -> StateEdgeKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// An immutable state-machine identity graph. It is not executable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateMachineModel {
    task: TaskId,
    entry: StateId,
    states: Box<[StateNode]>,
    transitions: Box<[StateTransition]>,
    source_span: Option<Span>,
}

impl StateMachineModel {
    pub fn new(
        task: TaskId,
        entry: StateId,
        states: impl IntoIterator<Item = StateNode>,
        transitions: impl IntoIterator<Item = StateTransition>,
        source_span: Option<Span>,
    ) -> Result<Self, StateMachineError> {
        if !task.is_valid() {
            return Err(StateMachineError::InvalidIdentity {
                kind: MachineIdentityKind::Task,
                value: task.get(),
            });
        }
        if !entry.is_valid() {
            return Err(StateMachineError::InvalidIdentity {
                kind: MachineIdentityKind::State,
                value: entry.get(),
            });
        }

        let mut states = states.into_iter().collect::<Vec<_>>();
        states.sort_by_key(StateNode::id);
        if states.is_empty() {
            return Err(StateMachineError::MissingEntry { entry });
        }

        let mut state_indices = BTreeMap::new();
        for (index, state) in states.iter().enumerate() {
            if !state.id.is_valid() {
                return Err(StateMachineError::InvalidIdentity {
                    kind: MachineIdentityKind::State,
                    value: state.id.get(),
                });
            }
            if !state.continuation.is_valid() {
                return Err(StateMachineError::InvalidIdentity {
                    kind: MachineIdentityKind::Continuation,
                    value: state.continuation.get(),
                });
            }
            let mut locals = BTreeSet::new();
            for local in &state.live_locals {
                if !local.is_valid() {
                    return Err(StateMachineError::InvalidIdentity {
                        kind: MachineIdentityKind::Local,
                        value: local.get(),
                    });
                }
                if !locals.insert(*local) {
                    return Err(StateMachineError::DuplicateLocal {
                        state: state.id,
                        local: *local,
                    });
                }
            }
            if state_indices.insert(state.id, index).is_some() {
                return Err(StateMachineError::DuplicateState { state: state.id });
            }
        }
        if !state_indices.contains_key(&entry) {
            return Err(StateMachineError::MissingEntry { entry });
        }

        let mut transitions = transitions.into_iter().collect::<Vec<_>>();
        transitions.sort_by_key(StateTransition::id);
        let mut transition_ids = BTreeSet::new();
        let mut edges = BTreeSet::new();
        for transition in &transitions {
            if !transition.id.is_valid() {
                return Err(StateMachineError::InvalidIdentity {
                    kind: MachineIdentityKind::Transition,
                    value: transition.id.get(),
                });
            }
            if !transition.from.is_valid() || !transition.to.is_valid() {
                return Err(StateMachineError::InvalidTransition {
                    transition: transition.id,
                });
            }
            if !state_indices.contains_key(&transition.from)
                || !state_indices.contains_key(&transition.to)
            {
                return Err(StateMachineError::UnknownState {
                    transition: transition.id,
                });
            }
            if !transition_ids.insert(transition.id) {
                return Err(StateMachineError::DuplicateTransition {
                    transition: transition.id,
                });
            }
            if !edges.insert((transition.from, transition.to, transition.kind)) {
                return Err(StateMachineError::DuplicateEdge {
                    from: transition.from,
                    to: transition.to,
                    kind: transition.kind,
                });
            }
        }

        Ok(Self {
            task,
            entry,
            states: states.into_boxed_slice(),
            transitions: transitions.into_boxed_slice(),
            source_span,
        })
    }

    #[must_use]
    pub const fn task(&self) -> TaskId {
        self.task
    }

    #[must_use]
    pub const fn entry(&self) -> StateId {
        self.entry
    }

    #[must_use]
    pub fn states(&self) -> &[StateNode] {
        &self.states
    }

    #[must_use]
    pub fn transitions(&self) -> &[StateTransition] {
        &self.transitions
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }

    /// Returns path-free deterministic bytes. Source evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"ling.task-state-machine/0");
        push_u32(&mut bytes, self.task.get());
        push_u32(&mut bytes, self.entry.get());
        push_u32(&mut bytes, self.states.len() as u32);
        for state in &self.states {
            push_u32(&mut bytes, state.id.get());
            push_u32(&mut bytes, state.continuation.get());
            push_u32(&mut bytes, state.live_locals.len() as u32);
            for local in &state.live_locals {
                push_u32(&mut bytes, local.get());
            }
        }
        push_u32(&mut bytes, self.transitions.len() as u32);
        for transition in &self.transitions {
            push_u32(&mut bytes, transition.id.get());
            push_u32(&mut bytes, transition.from.get());
            push_u32(&mut bytes, transition.to.get());
            bytes.push(transition.kind.rank());
        }
        bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MachineIdentityKind {
    Task,
    State,
    Continuation,
    Local,
    Transition,
}

impl MachineIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::State => "state",
            Self::Continuation => "continuation",
            Self::Local => "local",
            Self::Transition => "transition",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateMachineError {
    InvalidIdentity {
        kind: MachineIdentityKind,
        value: u32,
    },
    MissingEntry {
        entry: StateId,
    },
    DuplicateState {
        state: StateId,
    },
    DuplicateLocal {
        state: StateId,
        local: LocalId,
    },
    InvalidTransition {
        transition: TransitionId,
    },
    UnknownState {
        transition: TransitionId,
    },
    DuplicateTransition {
        transition: TransitionId,
    },
    DuplicateEdge {
        from: StateId,
        to: StateId,
        kind: StateEdgeKind,
    },
}

impl fmt::Display for StateMachineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::MissingEntry { entry } => {
                write!(formatter, "entry state {} is absent", entry.get())
            }
            Self::DuplicateState { state } => write!(formatter, "duplicate state {}", state.get()),
            Self::DuplicateLocal { state, local } => write!(
                formatter,
                "state {} repeats live local {}",
                state.get(),
                local.get()
            ),
            Self::InvalidTransition { transition } => {
                write!(
                    formatter,
                    "transition {} has an invalid endpoint",
                    transition.get()
                )
            }
            Self::UnknownState { transition } => {
                write!(
                    formatter,
                    "transition {} references an unknown state",
                    transition.get()
                )
            }
            Self::DuplicateTransition { transition } => {
                write!(formatter, "duplicate transition {}", transition.get())
            }
            Self::DuplicateEdge { from, to, kind } => write!(
                formatter,
                "duplicate {:?} edge {} -> {}",
                kind,
                from.get(),
                to.get()
            ),
        }
    }
}

impl Error for StateMachineError {}

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

    fn state(id: u32, locals: &[u32], source_span: Option<Span>) -> StateNode {
        StateNode::new(StateNodeSpec {
            id: StateId::new(id),
            continuation: TaskCoreNodeId::new(100 + id),
            live_locals: locals
                .iter()
                .copied()
                .map(LocalId::new)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            source_span,
        })
    }

    fn transition(
        id: u32,
        from: u32,
        to: u32,
        kind: StateEdgeKind,
        source_span: Option<Span>,
    ) -> StateTransition {
        StateTransition::new(StateTransitionSpec {
            id: TransitionId::new(id),
            from: StateId::new(from),
            to: StateId::new(to),
            kind,
            source_span,
        })
    }

    #[test]
    fn validates_state_machine_edges_and_live_local_order() {
        let model = StateMachineModel::new(
            TaskId::new(7),
            StateId::new(1),
            [state(2, &[9, 8], None), state(1, &[3, 2], None)],
            [
                transition(4, 1, 2, StateEdgeKind::Resume, None),
                transition(5, 2, 1, StateEdgeKind::Cleanup, None),
            ],
            None,
        )
        .expect("state machine is valid");
        assert_eq!(
            model
                .states()
                .iter()
                .map(|state| state.id().get())
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_eq!(
            model.states()[1].live_locals(),
            [LocalId::new(8), LocalId::new(9)]
        );
        assert_eq!(model.transitions()[0].kind(), StateEdgeKind::Resume);
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = StateMachineModel::new(
            TaskId::new(7),
            StateId::new(1),
            [
                state(1, &[2, 3], Some(span(1, 1, 2))),
                state(2, &[8, 9], Some(span(1, 3, 4))),
            ],
            [transition(
                4,
                1,
                2,
                StateEdgeKind::Resume,
                Some(span(1, 5, 6)),
            )],
            Some(span(1, 0, 6)),
        )
        .expect("first model")
        .canonical_bytes();
        let second = StateMachineModel::new(
            TaskId::new(7),
            StateId::new(1),
            [
                state(2, &[9, 8], Some(span(9, 30, 31))),
                state(1, &[3, 2], Some(span(9, 10, 11))),
            ],
            [transition(
                4,
                1,
                2,
                StateEdgeKind::Resume,
                Some(span(9, 40, 41)),
            )],
            Some(span(9, 0, 41)),
        )
        .expect("second model")
        .canonical_bytes();
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_duplicate_states_locals_edges_and_unknown_endpoints() {
        let duplicate_state = StateMachineModel::new(
            TaskId::new(7),
            StateId::new(1),
            [state(1, &[], None), state(1, &[], None)],
            [],
            None,
        )
        .expect_err("duplicate state must be rejected");
        assert!(matches!(
            duplicate_state,
            StateMachineError::DuplicateState { .. }
        ));

        let duplicate_local = StateMachineModel::new(
            TaskId::new(7),
            StateId::new(1),
            [state(1, &[4, 4], None)],
            [],
            None,
        )
        .expect_err("duplicate local must be rejected");
        assert!(matches!(
            duplicate_local,
            StateMachineError::DuplicateLocal { .. }
        ));

        let unknown = StateMachineModel::new(
            TaskId::new(7),
            StateId::new(1),
            [state(1, &[], None)],
            [transition(2, 1, 9, StateEdgeKind::Fault, None)],
            None,
        )
        .expect_err("unknown endpoint must be rejected");
        assert!(matches!(unknown, StateMachineError::UnknownState { .. }));

        let duplicate_edge = StateMachineModel::new(
            TaskId::new(7),
            StateId::new(1),
            [state(1, &[], None), state(2, &[], None)],
            [
                transition(2, 1, 2, StateEdgeKind::Resume, None),
                transition(3, 1, 2, StateEdgeKind::Resume, None),
            ],
            None,
        )
        .expect_err("duplicate edge must be rejected");
        assert!(matches!(
            duplicate_edge,
            StateMachineError::DuplicateEdge { .. }
        ));
    }
}
