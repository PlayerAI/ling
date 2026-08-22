//! Non-executable Actor turn observations.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use ling_source::Span;

use crate::ActorTypeId;

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

id_type!(TurnId, "One opaque Actor turn observation identity.");

/// Turn vocabulary labels are observations only; they do not define reentry.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TurnObservationKind {
    NoAwait,
    FreezeAndRelease,
    ForbidReentry,
    GuardedReentry,
    SelfSend,
    Watchdog,
}

impl TurnObservationKind {
    const fn rank(self) -> u8 {
        match self {
            Self::NoAwait => 0,
            Self::FreezeAndRelease => 1,
            Self::ForbidReentry => 2,
            Self::GuardedReentry => 3,
            Self::SelfSend => 4,
            Self::Watchdog => 5,
        }
    }
}

/// Inputs for one immutable turn observation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TurnObservationSpec {
    pub id: TurnId,
    pub actor: Option<ActorTypeId>,
    pub kind: TurnObservationKind,
    pub source_span: Option<Span>,
}

/// One immutable turn observation. It has no await, reentry, or watchdog authority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TurnObservation {
    id: TurnId,
    actor: Option<ActorTypeId>,
    kind: TurnObservationKind,
    source_span: Option<Span>,
}

impl TurnObservation {
    #[must_use]
    pub const fn new(spec: TurnObservationSpec) -> Self {
        Self {
            id: spec.id,
            actor: spec.actor,
            kind: spec.kind,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> TurnId {
        self.id
    }

    #[must_use]
    pub const fn actor(self) -> Option<ActorTypeId> {
        self.actor
    }

    #[must_use]
    pub const fn kind(self) -> TurnObservationKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable turn-observation model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TurnObservationModel {
    turns: Box<[TurnObservation]>,
    source_span: Option<Span>,
}

impl TurnObservationModel {
    pub fn new(
        turns: impl IntoIterator<Item = TurnObservation>,
        source_span: Option<Span>,
    ) -> Result<Self, TurnObservationError> {
        let mut turns = turns.into_iter().collect::<Vec<_>>();
        turns.sort_by_key(TurnObservation::id);
        let mut turn_ids = BTreeSet::new();
        for turn in &turns {
            if !turn.id.is_valid() {
                return Err(TurnObservationError::InvalidIdentity {
                    kind: TurnIdentityKind::Turn,
                    value: turn.id.get(),
                });
            }
            if let Some(actor) = turn.actor {
                if !actor.is_valid() {
                    return Err(TurnObservationError::InvalidIdentity {
                        kind: TurnIdentityKind::Actor,
                        value: actor.get(),
                    });
                }
            }
            if !turn_ids.insert(turn.id) {
                return Err(TurnObservationError::DuplicateTurn { turn: turn.id });
            }
        }
        Ok(Self {
            turns: turns.into_boxed_slice(),
            source_span,
        })
    }

    #[must_use]
    pub fn turns(&self) -> &[TurnObservation] {
        &self.turns
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }

    /// Returns path-free deterministic bytes. Source evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"ling.actor-turn-observation/0");
        push_u32(&mut bytes, self.turns.len() as u32);
        for turn in &self.turns {
            push_u32(&mut bytes, turn.id.get());
            push_optional_u32(&mut bytes, turn.actor.map(ActorTypeId::get));
            bytes.push(turn.kind.rank());
        }
        bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TurnIdentityKind {
    Turn,
    Actor,
}

impl TurnIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Turn => "turn",
            Self::Actor => "turn actor",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TurnObservationError {
    InvalidIdentity { kind: TurnIdentityKind, value: u32 },
    DuplicateTurn { turn: TurnId },
}

impl fmt::Display for TurnObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateTurn { turn } => write!(formatter, "duplicate turn {}", turn.get()),
        }
    }
}

impl Error for TurnObservationError {}

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

    fn turn(id: u32, actor: Option<u32>, kind: TurnObservationKind) -> TurnObservation {
        TurnObservation::new(TurnObservationSpec {
            id: TurnId::new(id),
            actor: actor.map(ActorTypeId::new),
            kind,
            source_span: None,
        })
    }

    #[test]
    fn validates_and_orders_structural_turn_observations() {
        let model = TurnObservationModel::new(
            [
                turn(2, Some(7), TurnObservationKind::GuardedReentry),
                turn(1, None, TurnObservationKind::NoAwait),
            ],
            None,
        )
        .expect("turn observations are valid");
        assert_eq!(
            model
                .turns()
                .iter()
                .map(|turn| turn.id().get())
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_eq!(model.turns()[1].kind(), TurnObservationKind::GuardedReentry);
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = TurnObservationModel::new(
            [TurnObservation::new(TurnObservationSpec {
                id: TurnId::new(1),
                actor: Some(ActorTypeId::new(7)),
                kind: TurnObservationKind::FreezeAndRelease,
                source_span: Some(span(1, 1, 2)),
            })],
            Some(span(1, 0, 2)),
        )
        .expect("first model")
        .canonical_bytes();
        let second = TurnObservationModel::new(
            [TurnObservation::new(TurnObservationSpec {
                id: TurnId::new(1),
                actor: Some(ActorTypeId::new(7)),
                kind: TurnObservationKind::FreezeAndRelease,
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
        let invalid =
            TurnObservationModel::new([turn(0, None, TurnObservationKind::NoAwait)], None)
                .expect_err("zero turn identity must be rejected");
        assert!(matches!(
            invalid,
            TurnObservationError::InvalidIdentity { .. }
        ));

        let duplicate = TurnObservationModel::new(
            [
                turn(1, None, TurnObservationKind::NoAwait),
                turn(1, None, TurnObservationKind::Watchdog),
            ],
            None,
        )
        .expect_err("duplicate turn identity must be rejected");
        assert!(matches!(
            duplicate,
            TurnObservationError::DuplicateTurn { .. }
        ));

        let actor =
            TurnObservationModel::new([turn(1, Some(0), TurnObservationKind::NoAwait)], None)
                .expect_err("zero actor identity must be rejected");
        assert!(matches!(
            actor,
            TurnObservationError::InvalidIdentity { .. }
        ));
    }
}
