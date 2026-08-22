//! Non-executable Actor runtime observations.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use ling_source::Span;

use crate::ActorId;

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
    RuntimeObservationId,
    "One opaque Actor runtime observation identity."
);

/// Runtime lifecycle labels are observations only; they do not define a state machine.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RuntimeObservationKind {
    Spawn,
    Start,
    Dispatch,
    Suspend,
    Stop,
    Stopped,
    Failed,
    Restart,
}

impl RuntimeObservationKind {
    const fn rank(self) -> u8 {
        match self {
            Self::Spawn => 0,
            Self::Start => 1,
            Self::Dispatch => 2,
            Self::Suspend => 3,
            Self::Stop => 4,
            Self::Stopped => 5,
            Self::Failed => 6,
            Self::Restart => 7,
        }
    }
}

/// Inputs for one immutable runtime observation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RuntimeObservationSpec {
    pub id: RuntimeObservationId,
    pub actor: Option<ActorId>,
    pub kind: RuntimeObservationKind,
    pub source_span: Option<Span>,
}

/// One immutable runtime observation. It has no spawn, stop, or dispatch authority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RuntimeObservation {
    id: RuntimeObservationId,
    actor: Option<ActorId>,
    kind: RuntimeObservationKind,
    source_span: Option<Span>,
}

impl RuntimeObservation {
    #[must_use]
    pub const fn new(spec: RuntimeObservationSpec) -> Self {
        Self {
            id: spec.id,
            actor: spec.actor,
            kind: spec.kind,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> RuntimeObservationId {
        self.id
    }

    #[must_use]
    pub const fn actor(self) -> Option<ActorId> {
        self.actor
    }

    #[must_use]
    pub const fn kind(self) -> RuntimeObservationKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable Actor-runtime observation model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeObservationModel {
    observations: Box<[RuntimeObservation]>,
    source_span: Option<Span>,
}

impl RuntimeObservationModel {
    pub fn new(
        observations: impl IntoIterator<Item = RuntimeObservation>,
        source_span: Option<Span>,
    ) -> Result<Self, RuntimeObservationError> {
        let mut observations = observations.into_iter().collect::<Vec<_>>();
        observations.sort_by_key(RuntimeObservation::id);
        let mut observation_ids = BTreeSet::new();
        for observation in &observations {
            if !observation.id.is_valid() {
                return Err(RuntimeObservationError::InvalidIdentity {
                    kind: RuntimeIdentityKind::Observation,
                    value: observation.id.get(),
                });
            }
            if let Some(actor) = observation.actor {
                if !actor.is_valid() {
                    return Err(RuntimeObservationError::InvalidIdentity {
                        kind: RuntimeIdentityKind::Actor,
                        value: actor.get(),
                    });
                }
            }
            if !observation_ids.insert(observation.id) {
                return Err(RuntimeObservationError::DuplicateObservation {
                    observation: observation.id,
                });
            }
        }
        Ok(Self {
            observations: observations.into_boxed_slice(),
            source_span,
        })
    }

    #[must_use]
    pub fn observations(&self) -> &[RuntimeObservation] {
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
        push_field(&mut bytes, b"ling.actor-runtime-observation/0");
        push_u32(&mut bytes, self.observations.len() as u32);
        for observation in &self.observations {
            push_u32(&mut bytes, observation.id.get());
            push_optional_u32(&mut bytes, observation.actor.map(ActorId::get));
            bytes.push(observation.kind.rank());
        }
        bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RuntimeIdentityKind {
    Observation,
    Actor,
}

impl RuntimeIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Observation => "runtime observation",
            Self::Actor => "runtime actor",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeObservationError {
    InvalidIdentity {
        kind: RuntimeIdentityKind,
        value: u32,
    },
    DuplicateObservation {
        observation: RuntimeObservationId,
    },
}

impl fmt::Display for RuntimeObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateObservation { observation } => {
                write!(
                    formatter,
                    "duplicate runtime observation {}",
                    observation.get()
                )
            }
        }
    }
}

impl Error for RuntimeObservationError {}

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
        actor: Option<u32>,
        kind: RuntimeObservationKind,
    ) -> RuntimeObservation {
        RuntimeObservation::new(RuntimeObservationSpec {
            id: RuntimeObservationId::new(id),
            actor: actor.map(ActorId::new),
            kind,
            source_span: None,
        })
    }

    #[test]
    fn validates_and_orders_structural_runtime_observations() {
        let model = RuntimeObservationModel::new(
            [
                observation(2, Some(7), RuntimeObservationKind::Dispatch),
                observation(1, None, RuntimeObservationKind::Spawn),
            ],
            None,
        )
        .expect("runtime observations are valid");
        assert_eq!(
            model
                .observations()
                .iter()
                .map(|observation| observation.id().get())
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_eq!(
            model.observations()[1].kind(),
            RuntimeObservationKind::Dispatch
        );
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = RuntimeObservationModel::new(
            [RuntimeObservation::new(RuntimeObservationSpec {
                id: RuntimeObservationId::new(1),
                actor: Some(ActorId::new(7)),
                kind: RuntimeObservationKind::Failed,
                source_span: Some(span(1, 1, 2)),
            })],
            Some(span(1, 0, 2)),
        )
        .expect("first model")
        .canonical_bytes();
        let second = RuntimeObservationModel::new(
            [RuntimeObservation::new(RuntimeObservationSpec {
                id: RuntimeObservationId::new(1),
                actor: Some(ActorId::new(7)),
                kind: RuntimeObservationKind::Failed,
                source_span: Some(span(9, 10, 11)),
            })],
            Some(span(9, 0, 11)),
        )
        .expect("second model")
        .canonical_bytes();
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_invalid_duplicate_and_actor_identities() {
        let invalid = RuntimeObservationModel::new(
            [observation(0, None, RuntimeObservationKind::Spawn)],
            None,
        )
        .expect_err("zero observation identity must be rejected");
        assert!(matches!(
            invalid,
            RuntimeObservationError::InvalidIdentity { .. }
        ));

        let duplicate = RuntimeObservationModel::new(
            [
                observation(1, None, RuntimeObservationKind::Spawn),
                observation(1, None, RuntimeObservationKind::Stop),
            ],
            None,
        )
        .expect_err("duplicate observation identity must be rejected");
        assert!(matches!(
            duplicate,
            RuntimeObservationError::DuplicateObservation { .. }
        ));

        let actor = RuntimeObservationModel::new(
            [observation(1, Some(0), RuntimeObservationKind::Spawn)],
            None,
        )
        .expect_err("zero actor identity must be rejected");
        assert!(matches!(
            actor,
            RuntimeObservationError::InvalidIdentity { .. }
        ));
    }
}
