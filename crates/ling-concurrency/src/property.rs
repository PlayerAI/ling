//! Non-executable Actor property observations.

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
    PropertyObservationId,
    "One opaque Actor property observation identity."
);

/// Property labels are evidence vocabulary only; they do not assert runtime behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PropertyObservationKind {
    SerialState,
    ParallelActors,
    BoundedMailbox,
    SlowConsumer,
    PostStopSend,
    FaultCleanup,
    DeclaredOrdering,
    ShutdownCleanup,
}

impl PropertyObservationKind {
    const fn rank(self) -> u8 {
        match self {
            Self::SerialState => 0,
            Self::ParallelActors => 1,
            Self::BoundedMailbox => 2,
            Self::SlowConsumer => 3,
            Self::PostStopSend => 4,
            Self::FaultCleanup => 5,
            Self::DeclaredOrdering => 6,
            Self::ShutdownCleanup => 7,
        }
    }
}

/// Inputs for one immutable property observation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PropertyObservationSpec {
    pub id: PropertyObservationId,
    pub actor: Option<ActorId>,
    pub kind: PropertyObservationKind,
    pub source_span: Option<Span>,
}

/// One immutable property observation. It has no stress or runtime authority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PropertyObservation {
    id: PropertyObservationId,
    actor: Option<ActorId>,
    kind: PropertyObservationKind,
    source_span: Option<Span>,
}

impl PropertyObservation {
    #[must_use]
    pub const fn new(spec: PropertyObservationSpec) -> Self {
        Self {
            id: spec.id,
            actor: spec.actor,
            kind: spec.kind,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> PropertyObservationId {
        self.id
    }

    #[must_use]
    pub const fn actor(self) -> Option<ActorId> {
        self.actor
    }

    #[must_use]
    pub const fn kind(self) -> PropertyObservationKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable Actor-property observation model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertyObservationModel {
    observations: Box<[PropertyObservation]>,
    source_span: Option<Span>,
}

impl PropertyObservationModel {
    pub fn new(
        observations: impl IntoIterator<Item = PropertyObservation>,
        source_span: Option<Span>,
    ) -> Result<Self, PropertyObservationError> {
        let mut observations = observations.into_iter().collect::<Vec<_>>();
        observations.sort_by_key(PropertyObservation::id);
        let mut observation_ids = BTreeSet::new();
        for observation in &observations {
            if !observation.id.is_valid() {
                return Err(PropertyObservationError::InvalidIdentity {
                    kind: PropertyIdentityKind::Observation,
                    value: observation.id.get(),
                });
            }
            if let Some(actor) = observation.actor {
                if !actor.is_valid() {
                    return Err(PropertyObservationError::InvalidIdentity {
                        kind: PropertyIdentityKind::Actor,
                        value: actor.get(),
                    });
                }
            }
            if !observation_ids.insert(observation.id) {
                return Err(PropertyObservationError::DuplicateObservation {
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
    pub fn observations(&self) -> &[PropertyObservation] {
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
        push_field(&mut bytes, b"ling.actor-property-observation/0");
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
pub enum PropertyIdentityKind {
    Observation,
    Actor,
}

impl PropertyIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Observation => "property observation",
            Self::Actor => "property actor",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropertyObservationError {
    InvalidIdentity {
        kind: PropertyIdentityKind,
        value: u32,
    },
    DuplicateObservation {
        observation: PropertyObservationId,
    },
}

impl fmt::Display for PropertyObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateObservation { observation } => {
                write!(
                    formatter,
                    "duplicate property observation {}",
                    observation.get()
                )
            }
        }
    }
}

impl Error for PropertyObservationError {}

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
        kind: PropertyObservationKind,
    ) -> PropertyObservation {
        PropertyObservation::new(PropertyObservationSpec {
            id: PropertyObservationId::new(id),
            actor: actor.map(ActorId::new),
            kind,
            source_span: None,
        })
    }

    #[test]
    fn validates_and_orders_structural_property_observations() {
        let model = PropertyObservationModel::new(
            [
                observation(2, Some(7), PropertyObservationKind::FaultCleanup),
                observation(1, None, PropertyObservationKind::SerialState),
            ],
            None,
        )
        .expect("property observations are valid");
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
            PropertyObservationKind::FaultCleanup
        );
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = PropertyObservationModel::new(
            [PropertyObservation::new(PropertyObservationSpec {
                id: PropertyObservationId::new(1),
                actor: Some(ActorId::new(7)),
                kind: PropertyObservationKind::DeclaredOrdering,
                source_span: Some(span(1, 1, 2)),
            })],
            Some(span(1, 0, 2)),
        )
        .expect("first model")
        .canonical_bytes();
        let second = PropertyObservationModel::new(
            [PropertyObservation::new(PropertyObservationSpec {
                id: PropertyObservationId::new(1),
                actor: Some(ActorId::new(7)),
                kind: PropertyObservationKind::DeclaredOrdering,
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
        let invalid = PropertyObservationModel::new(
            [observation(0, None, PropertyObservationKind::SerialState)],
            None,
        )
        .expect_err("zero observation identity must be rejected");
        assert!(matches!(
            invalid,
            PropertyObservationError::InvalidIdentity { .. }
        ));

        let duplicate = PropertyObservationModel::new(
            [
                observation(1, None, PropertyObservationKind::SerialState),
                observation(1, None, PropertyObservationKind::ShutdownCleanup),
            ],
            None,
        )
        .expect_err("duplicate observation identity must be rejected");
        assert!(matches!(
            duplicate,
            PropertyObservationError::DuplicateObservation { .. }
        ));

        let actor = PropertyObservationModel::new(
            [observation(
                1,
                Some(0),
                PropertyObservationKind::SerialState,
            )],
            None,
        )
        .expect_err("zero actor identity must be rejected");
        assert!(matches!(
            actor,
            PropertyObservationError::InvalidIdentity { .. }
        ));
    }
}
