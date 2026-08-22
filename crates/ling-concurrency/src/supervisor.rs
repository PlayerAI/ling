//! Non-executable Supervisor observations.

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
    SupervisorObservationId,
    "One opaque Supervisor observation identity."
);

/// Supervisor vocabulary labels are observations only; they do not define recovery behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SupervisorObservationKind {
    ChildSpec,
    Restart,
    Stop,
    Escalate,
    OneForOne,
    RestForOne,
    Transient,
    Permanent,
    Temporary,
    StateRestore,
    FaultChannel,
}

impl SupervisorObservationKind {
    const fn rank(self) -> u8 {
        match self {
            Self::ChildSpec => 0,
            Self::Restart => 1,
            Self::Stop => 2,
            Self::Escalate => 3,
            Self::OneForOne => 4,
            Self::RestForOne => 5,
            Self::Transient => 6,
            Self::Permanent => 7,
            Self::Temporary => 8,
            Self::StateRestore => 9,
            Self::FaultChannel => 10,
        }
    }
}

/// Inputs for one immutable Supervisor observation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SupervisorObservationSpec {
    pub id: SupervisorObservationId,
    pub actor: Option<ActorId>,
    pub kind: SupervisorObservationKind,
    pub source_span: Option<Span>,
}

/// One immutable Supervisor observation. It has no restart or escalation authority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SupervisorObservation {
    id: SupervisorObservationId,
    actor: Option<ActorId>,
    kind: SupervisorObservationKind,
    source_span: Option<Span>,
}

impl SupervisorObservation {
    #[must_use]
    pub const fn new(spec: SupervisorObservationSpec) -> Self {
        Self {
            id: spec.id,
            actor: spec.actor,
            kind: spec.kind,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> SupervisorObservationId {
        self.id
    }

    #[must_use]
    pub const fn actor(self) -> Option<ActorId> {
        self.actor
    }

    #[must_use]
    pub const fn kind(self) -> SupervisorObservationKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable Supervisor-observation model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisorObservationModel {
    observations: Box<[SupervisorObservation]>,
    source_span: Option<Span>,
}

impl SupervisorObservationModel {
    pub fn new(
        observations: impl IntoIterator<Item = SupervisorObservation>,
        source_span: Option<Span>,
    ) -> Result<Self, SupervisorObservationError> {
        let mut observations = observations.into_iter().collect::<Vec<_>>();
        observations.sort_by_key(SupervisorObservation::id);
        let mut observation_ids = BTreeSet::new();
        for observation in &observations {
            if !observation.id.is_valid() {
                return Err(SupervisorObservationError::InvalidIdentity {
                    kind: SupervisorIdentityKind::Observation,
                    value: observation.id.get(),
                });
            }
            if let Some(actor) = observation.actor {
                if !actor.is_valid() {
                    return Err(SupervisorObservationError::InvalidIdentity {
                        kind: SupervisorIdentityKind::Actor,
                        value: actor.get(),
                    });
                }
            }
            if !observation_ids.insert(observation.id) {
                return Err(SupervisorObservationError::DuplicateObservation {
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
    pub fn observations(&self) -> &[SupervisorObservation] {
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
        push_field(&mut bytes, b"ling.actor-supervisor-observation/0");
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
pub enum SupervisorIdentityKind {
    Observation,
    Actor,
}

impl SupervisorIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Observation => "supervisor observation",
            Self::Actor => "supervisor actor",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SupervisorObservationError {
    InvalidIdentity {
        kind: SupervisorIdentityKind,
        value: u32,
    },
    DuplicateObservation {
        observation: SupervisorObservationId,
    },
}

impl fmt::Display for SupervisorObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateObservation { observation } => {
                write!(
                    formatter,
                    "duplicate supervisor observation {}",
                    observation.get()
                )
            }
        }
    }
}

impl Error for SupervisorObservationError {}

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
        kind: SupervisorObservationKind,
    ) -> SupervisorObservation {
        SupervisorObservation::new(SupervisorObservationSpec {
            id: SupervisorObservationId::new(id),
            actor: actor.map(ActorId::new),
            kind,
            source_span: None,
        })
    }

    #[test]
    fn validates_and_orders_structural_supervisor_observations() {
        let model = SupervisorObservationModel::new(
            [
                observation(2, Some(7), SupervisorObservationKind::Restart),
                observation(1, None, SupervisorObservationKind::ChildSpec),
            ],
            None,
        )
        .expect("supervisor observations are valid");
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
            SupervisorObservationKind::Restart
        );
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = SupervisorObservationModel::new(
            [SupervisorObservation::new(SupervisorObservationSpec {
                id: SupervisorObservationId::new(1),
                actor: Some(ActorId::new(7)),
                kind: SupervisorObservationKind::FaultChannel,
                source_span: Some(span(1, 1, 2)),
            })],
            Some(span(1, 0, 2)),
        )
        .expect("first model")
        .canonical_bytes();
        let second = SupervisorObservationModel::new(
            [SupervisorObservation::new(SupervisorObservationSpec {
                id: SupervisorObservationId::new(1),
                actor: Some(ActorId::new(7)),
                kind: SupervisorObservationKind::FaultChannel,
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
        let invalid = SupervisorObservationModel::new(
            [observation(0, None, SupervisorObservationKind::ChildSpec)],
            None,
        )
        .expect_err("zero observation identity must be rejected");
        assert!(matches!(
            invalid,
            SupervisorObservationError::InvalidIdentity { .. }
        ));

        let duplicate = SupervisorObservationModel::new(
            [
                observation(1, None, SupervisorObservationKind::ChildSpec),
                observation(1, None, SupervisorObservationKind::Stop),
            ],
            None,
        )
        .expect_err("duplicate observation identity must be rejected");
        assert!(matches!(
            duplicate,
            SupervisorObservationError::DuplicateObservation { .. }
        ));

        let actor = SupervisorObservationModel::new(
            [observation(
                1,
                Some(0),
                SupervisorObservationKind::ChildSpec,
            )],
            None,
        )
        .expect_err("zero actor identity must be rejected");
        assert!(matches!(
            actor,
            SupervisorObservationError::InvalidIdentity { .. }
        ));
    }
}
