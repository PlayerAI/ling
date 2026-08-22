//! Non-executable lifecycle observations for the future Task runtime boundary.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use ling_source::Span;

use crate::{ScopeId, TaskId};

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
    LifecycleEventId,
    "One opaque lifecycle-observation event identity."
);
id_type!(FaultId, "One opaque observed Fault identity.");

/// Structural event labels. They do not define runtime ordering or effects.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LifecycleEventKind {
    ScopeCreated,
    ChildRegistered,
    JoinObserved,
    CancellationObserved,
    FaultObserved,
    CleanupObserved,
    ScopeClosed,
}

impl LifecycleEventKind {
    const fn rank(self) -> u8 {
        match self {
            Self::ScopeCreated => 0,
            Self::ChildRegistered => 1,
            Self::JoinObserved => 2,
            Self::CancellationObserved => 3,
            Self::FaultObserved => 4,
            Self::CleanupObserved => 5,
            Self::ScopeClosed => 6,
        }
    }
}

/// Inputs for one immutable lifecycle observation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LifecycleEventSpec {
    pub id: LifecycleEventId,
    pub scope: ScopeId,
    pub task: Option<TaskId>,
    pub kind: LifecycleEventKind,
    pub related_task: Option<TaskId>,
    pub fault: Option<FaultId>,
    pub source_span: Option<Span>,
}

/// One immutable lifecycle observation. It has no execution authority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LifecycleEvent {
    id: LifecycleEventId,
    scope: ScopeId,
    task: Option<TaskId>,
    kind: LifecycleEventKind,
    related_task: Option<TaskId>,
    fault: Option<FaultId>,
    source_span: Option<Span>,
}

impl LifecycleEvent {
    #[must_use]
    pub const fn new(spec: LifecycleEventSpec) -> Self {
        Self {
            id: spec.id,
            scope: spec.scope,
            task: spec.task,
            kind: spec.kind,
            related_task: spec.related_task,
            fault: spec.fault,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> LifecycleEventId {
        self.id
    }

    #[must_use]
    pub const fn scope(self) -> ScopeId {
        self.scope
    }

    #[must_use]
    pub const fn task(self) -> Option<TaskId> {
        self.task
    }

    #[must_use]
    pub const fn kind(self) -> LifecycleEventKind {
        self.kind
    }

    #[must_use]
    pub const fn related_task(self) -> Option<TaskId> {
        self.related_task
    }

    #[must_use]
    pub const fn fault(self) -> Option<FaultId> {
        self.fault
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable lifecycle observation trace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleTrace {
    scope: ScopeId,
    events: Box<[LifecycleEvent]>,
    source_span: Option<Span>,
}

impl LifecycleTrace {
    pub fn new(
        scope: ScopeId,
        events: impl IntoIterator<Item = LifecycleEvent>,
        source_span: Option<Span>,
    ) -> Result<Self, LifecycleTraceError> {
        if !scope.is_valid() {
            return Err(LifecycleTraceError::InvalidIdentity {
                kind: LifecycleIdentityKind::Scope,
                value: scope.get(),
            });
        }
        let mut events = events.into_iter().collect::<Vec<_>>();
        events.sort_by_key(LifecycleEvent::id);
        let mut event_ids = BTreeSet::new();
        for event in &events {
            if !event.id.is_valid() {
                return Err(LifecycleTraceError::InvalidIdentity {
                    kind: LifecycleIdentityKind::Event,
                    value: event.id.get(),
                });
            }
            if !event.scope.is_valid() {
                return Err(LifecycleTraceError::InvalidIdentity {
                    kind: LifecycleIdentityKind::Scope,
                    value: event.scope.get(),
                });
            }
            if event.task.is_some_and(|task| !task.is_valid()) {
                return Err(LifecycleTraceError::InvalidIdentity {
                    kind: LifecycleIdentityKind::Task,
                    value: event.task.expect("checked above").get(),
                });
            }
            if event.related_task.is_some_and(|task| !task.is_valid()) {
                return Err(LifecycleTraceError::InvalidIdentity {
                    kind: LifecycleIdentityKind::RelatedTask,
                    value: event.related_task.expect("checked above").get(),
                });
            }
            if event.fault.is_some_and(|fault| !fault.is_valid()) {
                return Err(LifecycleTraceError::InvalidIdentity {
                    kind: LifecycleIdentityKind::Fault,
                    value: event.fault.expect("checked above").get(),
                });
            }
            if !event_ids.insert(event.id) {
                return Err(LifecycleTraceError::DuplicateEvent { event: event.id });
            }
        }
        Ok(Self {
            scope,
            events: events.into_boxed_slice(),
            source_span,
        })
    }

    #[must_use]
    pub const fn scope(&self) -> ScopeId {
        self.scope
    }

    #[must_use]
    pub fn events(&self) -> &[LifecycleEvent] {
        &self.events
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }

    /// Returns path-free deterministic bytes. Source evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"ling.task-lifecycle-observation/0");
        push_u32(&mut bytes, self.scope.get());
        push_u32(&mut bytes, self.events.len() as u32);
        for event in &self.events {
            push_u32(&mut bytes, event.id.get());
            push_u32(&mut bytes, event.scope.get());
            push_optional_u32(&mut bytes, event.task.map(TaskId::get));
            bytes.push(event.kind.rank());
            push_optional_u32(&mut bytes, event.related_task.map(TaskId::get));
            push_optional_u32(&mut bytes, event.fault.map(FaultId::get));
        }
        bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LifecycleIdentityKind {
    Scope,
    Event,
    Task,
    RelatedTask,
    Fault,
}

impl LifecycleIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Scope => "scope",
            Self::Event => "event",
            Self::Task => "task",
            Self::RelatedTask => "related task",
            Self::Fault => "fault",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LifecycleTraceError {
    InvalidIdentity {
        kind: LifecycleIdentityKind,
        value: u32,
    },
    DuplicateEvent {
        event: LifecycleEventId,
    },
}

impl fmt::Display for LifecycleTraceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateEvent { event } => {
                write!(formatter, "duplicate lifecycle event {}", event.get())
            }
        }
    }
}

impl Error for LifecycleTraceError {}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_optional_u32(bytes: &mut Vec<u8>, value: Option<u32>) {
    match value {
        Some(value) => {
            bytes.push(1);
            push_u32(bytes, value);
        }
        None => bytes.push(0),
    }
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

    fn event(id: u32, kind: LifecycleEventKind, source_span: Option<Span>) -> LifecycleEvent {
        LifecycleEvent::new(LifecycleEventSpec {
            id: LifecycleEventId::new(id),
            scope: ScopeId::new(1),
            task: Some(TaskId::new(10)),
            kind,
            related_task: Some(TaskId::new(11)),
            fault: (kind == LifecycleEventKind::FaultObserved).then(|| FaultId::new(20)),
            source_span,
        })
    }

    #[test]
    fn validates_and_orders_structural_lifecycle_observations() {
        let trace = LifecycleTrace::new(
            ScopeId::new(1),
            [
                event(3, LifecycleEventKind::CleanupObserved, None),
                event(1, LifecycleEventKind::ScopeCreated, None),
                event(2, LifecycleEventKind::ChildRegistered, None),
            ],
            None,
        )
        .expect("trace is valid");
        assert_eq!(
            trace
                .events()
                .iter()
                .map(|event| event.id().get())
                .collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert_eq!(trace.events()[0].kind(), LifecycleEventKind::ScopeCreated);
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = LifecycleTrace::new(
            ScopeId::new(1),
            [
                event(2, LifecycleEventKind::FaultObserved, Some(span(1, 4, 5))),
                event(1, LifecycleEventKind::ScopeCreated, Some(span(1, 1, 2))),
            ],
            Some(span(1, 0, 5)),
        )
        .expect("first trace")
        .canonical_bytes();
        let second = LifecycleTrace::new(
            ScopeId::new(1),
            [
                event(1, LifecycleEventKind::ScopeCreated, Some(span(9, 10, 11))),
                event(2, LifecycleEventKind::FaultObserved, Some(span(9, 40, 41))),
            ],
            Some(span(9, 0, 41)),
        )
        .expect("second trace")
        .canonical_bytes();
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_invalid_and_duplicate_observation_identities() {
        let invalid = LifecycleTrace::new(
            ScopeId::new(1),
            [LifecycleEvent::new(LifecycleEventSpec {
                id: LifecycleEventId::new(0),
                scope: ScopeId::new(1),
                task: None,
                kind: LifecycleEventKind::ScopeCreated,
                related_task: None,
                fault: None,
                source_span: None,
            })],
            None,
        )
        .expect_err("zero event identity must be rejected");
        assert!(matches!(
            invalid,
            LifecycleTraceError::InvalidIdentity { .. }
        ));

        let duplicate = LifecycleTrace::new(
            ScopeId::new(1),
            [
                event(1, LifecycleEventKind::ScopeCreated, None),
                event(1, LifecycleEventKind::ScopeClosed, None),
            ],
            None,
        )
        .expect_err("duplicate event identity must be rejected");
        assert!(matches!(
            duplicate,
            LifecycleTraceError::DuplicateEvent { .. }
        ));
    }
}
