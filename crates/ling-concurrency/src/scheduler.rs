//! Non-executable scheduler observations for the future Task test boundary.

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
    SchedulerTraceId,
    "One opaque scheduler-observation trace identity."
);
id_type!(
    SchedulerEventId,
    "One opaque scheduler-observation event identity."
);

/// Structural scheduler labels. They do not define scheduling behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SchedulerObservationKind {
    SeedObserved,
    ReadyObserved,
    WakeObserved,
    ClockObserved,
    InterleavingObserved,
    TraceClosed,
}

impl SchedulerObservationKind {
    const fn rank(self) -> u8 {
        match self {
            Self::SeedObserved => 0,
            Self::ReadyObserved => 1,
            Self::WakeObserved => 2,
            Self::ClockObserved => 3,
            Self::InterleavingObserved => 4,
            Self::TraceClosed => 5,
        }
    }
}

/// Inputs for one immutable scheduler observation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SchedulerObservationSpec {
    pub id: SchedulerEventId,
    pub trace: SchedulerTraceId,
    pub scope: Option<ScopeId>,
    pub task: Option<TaskId>,
    pub kind: SchedulerObservationKind,
    pub source_span: Option<Span>,
}

/// One immutable scheduler observation. It has no execution authority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SchedulerObservation {
    id: SchedulerEventId,
    trace: SchedulerTraceId,
    scope: Option<ScopeId>,
    task: Option<TaskId>,
    kind: SchedulerObservationKind,
    source_span: Option<Span>,
}

impl SchedulerObservation {
    #[must_use]
    pub const fn new(spec: SchedulerObservationSpec) -> Self {
        Self {
            id: spec.id,
            trace: spec.trace,
            scope: spec.scope,
            task: spec.task,
            kind: spec.kind,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> SchedulerEventId {
        self.id
    }

    #[must_use]
    pub const fn trace(self) -> SchedulerTraceId {
        self.trace
    }

    #[must_use]
    pub const fn scope(self) -> Option<ScopeId> {
        self.scope
    }

    #[must_use]
    pub const fn task(self) -> Option<TaskId> {
        self.task
    }

    #[must_use]
    pub const fn kind(self) -> SchedulerObservationKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable scheduler observation trace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchedulerObservationTrace {
    trace: SchedulerTraceId,
    observations: Box<[SchedulerObservation]>,
    source_span: Option<Span>,
}

impl SchedulerObservationTrace {
    pub fn new(
        trace: SchedulerTraceId,
        observations: impl IntoIterator<Item = SchedulerObservation>,
        source_span: Option<Span>,
    ) -> Result<Self, SchedulerTraceError> {
        if !trace.is_valid() {
            return Err(SchedulerTraceError::InvalidIdentity {
                kind: SchedulerIdentityKind::Trace,
                value: trace.get(),
            });
        }
        let mut observations = observations.into_iter().collect::<Vec<_>>();
        observations.sort_by_key(SchedulerObservation::id);
        let mut event_ids = BTreeSet::new();
        for observation in &observations {
            if !observation.id.is_valid() {
                return Err(SchedulerTraceError::InvalidIdentity {
                    kind: SchedulerIdentityKind::Event,
                    value: observation.id.get(),
                });
            }
            if !observation.trace.is_valid() {
                return Err(SchedulerTraceError::InvalidIdentity {
                    kind: SchedulerIdentityKind::Trace,
                    value: observation.trace.get(),
                });
            }
            if observation.scope.is_some_and(|scope| !scope.is_valid()) {
                return Err(SchedulerTraceError::InvalidIdentity {
                    kind: SchedulerIdentityKind::Scope,
                    value: observation.scope.expect("checked above").get(),
                });
            }
            if observation.task.is_some_and(|task| !task.is_valid()) {
                return Err(SchedulerTraceError::InvalidIdentity {
                    kind: SchedulerIdentityKind::Task,
                    value: observation.task.expect("checked above").get(),
                });
            }
            if !event_ids.insert(observation.id) {
                return Err(SchedulerTraceError::DuplicateEvent {
                    event: observation.id,
                });
            }
        }
        Ok(Self {
            trace,
            observations: observations.into_boxed_slice(),
            source_span,
        })
    }

    #[must_use]
    pub const fn trace(&self) -> SchedulerTraceId {
        self.trace
    }

    #[must_use]
    pub fn observations(&self) -> &[SchedulerObservation] {
        &self.observations
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }

    /// Returns path-free deterministic bytes. Source evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"ling.task-scheduler-observation/0");
        push_u32(&mut bytes, self.trace.get());
        push_u32(&mut bytes, self.observations.len() as u32);
        for observation in &self.observations {
            push_u32(&mut bytes, observation.id.get());
            push_u32(&mut bytes, observation.trace.get());
            push_optional_u32(&mut bytes, observation.scope.map(ScopeId::get));
            push_optional_u32(&mut bytes, observation.task.map(TaskId::get));
            bytes.push(observation.kind.rank());
        }
        bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SchedulerIdentityKind {
    Trace,
    Event,
    Scope,
    Task,
}

impl SchedulerIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Event => "event",
            Self::Scope => "scope",
            Self::Task => "task",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SchedulerTraceError {
    InvalidIdentity {
        kind: SchedulerIdentityKind,
        value: u32,
    },
    DuplicateEvent {
        event: SchedulerEventId,
    },
}

impl fmt::Display for SchedulerTraceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateEvent { event } => {
                write!(formatter, "duplicate scheduler event {}", event.get())
            }
        }
    }
}

impl Error for SchedulerTraceError {}

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

    fn observation(
        id: u32,
        kind: SchedulerObservationKind,
        source_span: Option<Span>,
    ) -> SchedulerObservation {
        SchedulerObservation::new(SchedulerObservationSpec {
            id: SchedulerEventId::new(id),
            trace: SchedulerTraceId::new(1),
            scope: Some(ScopeId::new(2)),
            task: Some(TaskId::new(3)),
            kind,
            source_span,
        })
    }

    #[test]
    fn validates_and_orders_structural_scheduler_observations() {
        let trace = SchedulerObservationTrace::new(
            SchedulerTraceId::new(1),
            [
                observation(3, SchedulerObservationKind::WakeObserved, None),
                observation(1, SchedulerObservationKind::SeedObserved, None),
                observation(2, SchedulerObservationKind::ReadyObserved, None),
            ],
            None,
        )
        .expect("trace is valid");
        assert_eq!(
            trace
                .observations()
                .iter()
                .map(|observation| observation.id().get())
                .collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert_eq!(
            trace.observations()[0].kind(),
            SchedulerObservationKind::SeedObserved
        );
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = SchedulerObservationTrace::new(
            SchedulerTraceId::new(1),
            [
                observation(
                    2,
                    SchedulerObservationKind::WakeObserved,
                    Some(span(1, 4, 5)),
                ),
                observation(
                    1,
                    SchedulerObservationKind::SeedObserved,
                    Some(span(1, 1, 2)),
                ),
            ],
            Some(span(1, 0, 5)),
        )
        .expect("first trace")
        .canonical_bytes();
        let second = SchedulerObservationTrace::new(
            SchedulerTraceId::new(1),
            [
                observation(
                    1,
                    SchedulerObservationKind::SeedObserved,
                    Some(span(9, 10, 11)),
                ),
                observation(
                    2,
                    SchedulerObservationKind::WakeObserved,
                    Some(span(9, 40, 41)),
                ),
            ],
            Some(span(9, 0, 41)),
        )
        .expect("second trace")
        .canonical_bytes();
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_invalid_and_duplicate_observation_identities() {
        let invalid = SchedulerObservationTrace::new(
            SchedulerTraceId::new(1),
            [SchedulerObservation::new(SchedulerObservationSpec {
                id: SchedulerEventId::new(0),
                trace: SchedulerTraceId::new(1),
                scope: None,
                task: None,
                kind: SchedulerObservationKind::SeedObserved,
                source_span: None,
            })],
            None,
        )
        .expect_err("zero event identity must be rejected");
        assert!(matches!(
            invalid,
            SchedulerTraceError::InvalidIdentity { .. }
        ));

        let duplicate = SchedulerObservationTrace::new(
            SchedulerTraceId::new(1),
            [
                observation(1, SchedulerObservationKind::SeedObserved, None),
                observation(1, SchedulerObservationKind::TraceClosed, None),
            ],
            None,
        )
        .expect_err("duplicate event identity must be rejected");
        assert!(matches!(
            duplicate,
            SchedulerTraceError::DuplicateEvent { .. }
        ));
    }
}
