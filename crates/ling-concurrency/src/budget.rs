//! Non-executable restart-budget and circuit observations.

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
    BudgetObservationId,
    "One opaque restart-budget observation identity."
);

/// Budget and circuit labels are observations only; they do not define recovery behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BudgetObservationKind {
    RestartCount,
    Window,
    Backoff,
    MaxRestarts,
    FaultProvenance,
    CircuitClosed,
    CircuitOpen,
    CircuitHalfOpen,
}

impl BudgetObservationKind {
    const fn rank(self) -> u8 {
        match self {
            Self::RestartCount => 0,
            Self::Window => 1,
            Self::Backoff => 2,
            Self::MaxRestarts => 3,
            Self::FaultProvenance => 4,
            Self::CircuitClosed => 5,
            Self::CircuitOpen => 6,
            Self::CircuitHalfOpen => 7,
        }
    }
}

/// Inputs for one immutable restart-budget observation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BudgetObservationSpec {
    pub id: BudgetObservationId,
    pub actor: Option<ActorId>,
    pub kind: BudgetObservationKind,
    pub source_span: Option<Span>,
}

/// One immutable restart-budget observation. It has no clock, counter, or circuit authority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BudgetObservation {
    id: BudgetObservationId,
    actor: Option<ActorId>,
    kind: BudgetObservationKind,
    source_span: Option<Span>,
}

impl BudgetObservation {
    #[must_use]
    pub const fn new(spec: BudgetObservationSpec) -> Self {
        Self {
            id: spec.id,
            actor: spec.actor,
            kind: spec.kind,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> BudgetObservationId {
        self.id
    }

    #[must_use]
    pub const fn actor(self) -> Option<ActorId> {
        self.actor
    }

    #[must_use]
    pub const fn kind(self) -> BudgetObservationKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable budget/circuit observation model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetObservationModel {
    observations: Box<[BudgetObservation]>,
    source_span: Option<Span>,
}

impl BudgetObservationModel {
    pub fn new(
        observations: impl IntoIterator<Item = BudgetObservation>,
        source_span: Option<Span>,
    ) -> Result<Self, BudgetObservationError> {
        let mut observations = observations.into_iter().collect::<Vec<_>>();
        observations.sort_by_key(BudgetObservation::id);
        let mut observation_ids = BTreeSet::new();
        for observation in &observations {
            if !observation.id.is_valid() {
                return Err(BudgetObservationError::InvalidIdentity {
                    kind: BudgetIdentityKind::Observation,
                    value: observation.id.get(),
                });
            }
            if let Some(actor) = observation.actor {
                if !actor.is_valid() {
                    return Err(BudgetObservationError::InvalidIdentity {
                        kind: BudgetIdentityKind::Actor,
                        value: actor.get(),
                    });
                }
            }
            if !observation_ids.insert(observation.id) {
                return Err(BudgetObservationError::DuplicateObservation {
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
    pub fn observations(&self) -> &[BudgetObservation] {
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
        push_field(&mut bytes, b"ling.actor-budget-observation/0");
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
pub enum BudgetIdentityKind {
    Observation,
    Actor,
}

impl BudgetIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Observation => "budget observation",
            Self::Actor => "budget actor",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BudgetObservationError {
    InvalidIdentity {
        kind: BudgetIdentityKind,
        value: u32,
    },
    DuplicateObservation {
        observation: BudgetObservationId,
    },
}

impl fmt::Display for BudgetObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateObservation { observation } => {
                write!(
                    formatter,
                    "duplicate budget observation {}",
                    observation.get()
                )
            }
        }
    }
}

impl Error for BudgetObservationError {}

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

    fn observation(id: u32, actor: Option<u32>, kind: BudgetObservationKind) -> BudgetObservation {
        BudgetObservation::new(BudgetObservationSpec {
            id: BudgetObservationId::new(id),
            actor: actor.map(ActorId::new),
            kind,
            source_span: None,
        })
    }

    #[test]
    fn validates_and_orders_structural_budget_observations() {
        let model = BudgetObservationModel::new(
            [
                observation(2, Some(7), BudgetObservationKind::CircuitOpen),
                observation(1, None, BudgetObservationKind::RestartCount),
            ],
            None,
        )
        .expect("budget observations are valid");
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
            BudgetObservationKind::CircuitOpen
        );
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = BudgetObservationModel::new(
            [BudgetObservation::new(BudgetObservationSpec {
                id: BudgetObservationId::new(1),
                actor: Some(ActorId::new(7)),
                kind: BudgetObservationKind::FaultProvenance,
                source_span: Some(span(1, 1, 2)),
            })],
            Some(span(1, 0, 2)),
        )
        .expect("first model")
        .canonical_bytes();
        let second = BudgetObservationModel::new(
            [BudgetObservation::new(BudgetObservationSpec {
                id: BudgetObservationId::new(1),
                actor: Some(ActorId::new(7)),
                kind: BudgetObservationKind::FaultProvenance,
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
        let invalid = BudgetObservationModel::new(
            [observation(0, None, BudgetObservationKind::RestartCount)],
            None,
        )
        .expect_err("zero observation identity must be rejected");
        assert!(matches!(
            invalid,
            BudgetObservationError::InvalidIdentity { .. }
        ));

        let duplicate = BudgetObservationModel::new(
            [
                observation(1, None, BudgetObservationKind::RestartCount),
                observation(1, None, BudgetObservationKind::CircuitClosed),
            ],
            None,
        )
        .expect_err("duplicate observation identity must be rejected");
        assert!(matches!(
            duplicate,
            BudgetObservationError::DuplicateObservation { .. }
        ));

        let actor = BudgetObservationModel::new(
            [observation(1, Some(0), BudgetObservationKind::RestartCount)],
            None,
        )
        .expect_err("zero actor identity must be rejected");
        assert!(matches!(
            actor,
            BudgetObservationError::InvalidIdentity { .. }
        ));
    }
}
