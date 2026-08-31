//! Checked-only contract for one non-suspending local Actor turn.
//!
//! This module classifies accepted turn semantics. It contains no mailbox,
//! state cell, scheduler, interpreter, VM, or execution entry point.

use std::error::Error;
use std::fmt;

use ling_source::Span;

use crate::ActorTypeId;

pub const CHECKED_ACTOR_TURN_VERSION: &str = "ling.checked-actor-turn/1";

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActorTurnDispatch {
    OneMessage,
}

impl ActorTurnDispatch {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneMessage => "OneMessage",
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::OneMessage => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActorTurnSuspension {
    Forbidden,
}

impl ActorTurnSuspension {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Forbidden => "Forbidden",
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::Forbidden => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActorTurnReentry {
    Forbidden,
}

impl ActorTurnReentry {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Forbidden => "Forbidden",
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::Forbidden => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActorTurnStateCommit {
    PublishOnNormalReturn,
}

impl ActorTurnStateCommit {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishOnNormalReturn => "PublishOnNormalReturn",
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::PublishOnNormalReturn => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActorTurnSelfSend {
    MailboxOnly,
}

impl ActorTurnSelfSend {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MailboxOnly => "MailboxOnly",
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::MailboxOnly => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActorTurnCompletion {
    NormalReturn,
    Unsuccessful,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActorTurnStatePublication {
    PublishCandidate,
    PreservePrevious,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorTurnContract {
    actor: Box<str>,
    actor_type: ActorTypeId,
    dispatch: ActorTurnDispatch,
    suspension: ActorTurnSuspension,
    reentry: ActorTurnReentry,
    state_commit: ActorTurnStateCommit,
    self_send: ActorTurnSelfSend,
    transition: u32,
    state_binding: u32,
    message_binding: u32,
    receive_span: Span,
    body_span: Span,
    canonical_bytes: Box<[u8]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorTurnSpec {
    pub actor: Box<str>,
    pub actor_type: ActorTypeId,
    pub transition: u32,
    pub state_binding: u32,
    pub message_binding: u32,
    pub receive_span: Span,
    pub body_span: Span,
}

impl ActorTurnContract {
    pub fn new_checked_profile(spec: ActorTurnSpec) -> Result<Self, ActorTurnContractError> {
        if spec.actor.is_empty() {
            return Err(ActorTurnContractError::MissingActor);
        }
        if !spec.actor_type.is_valid() {
            return Err(ActorTurnContractError::InvalidActorType);
        }
        if spec.state_binding == spec.message_binding {
            return Err(ActorTurnContractError::AliasedBindings {
                binding: spec.state_binding,
            });
        }
        if spec.receive_span.source() != spec.body_span.source()
            || spec.receive_span.start() >= spec.receive_span.end()
            || spec.body_span.start() >= spec.body_span.end()
            || spec.body_span.start() <= spec.receive_span.start()
            || spec.body_span.end() > spec.receive_span.end()
        {
            return Err(ActorTurnContractError::InvalidSpans);
        }
        let dispatch = ActorTurnDispatch::OneMessage;
        let suspension = ActorTurnSuspension::Forbidden;
        let reentry = ActorTurnReentry::Forbidden;
        let state_commit = ActorTurnStateCommit::PublishOnNormalReturn;
        let self_send = ActorTurnSelfSend::MailboxOnly;
        let mut canonical_bytes = Vec::new();
        push_text(&mut canonical_bytes, CHECKED_ACTOR_TURN_VERSION);
        push_text(&mut canonical_bytes, &spec.actor);
        canonical_bytes.extend_from_slice(&spec.actor_type.get().to_be_bytes());
        canonical_bytes.extend_from_slice(&[
            dispatch.tag(),
            suspension.tag(),
            reentry.tag(),
            state_commit.tag(),
            self_send.tag(),
        ]);
        canonical_bytes.extend_from_slice(&spec.transition.to_be_bytes());
        canonical_bytes.extend_from_slice(&spec.state_binding.to_be_bytes());
        canonical_bytes.extend_from_slice(&spec.message_binding.to_be_bytes());
        Ok(Self {
            actor: spec.actor,
            actor_type: spec.actor_type,
            dispatch,
            suspension,
            reentry,
            state_commit,
            self_send,
            transition: spec.transition,
            state_binding: spec.state_binding,
            message_binding: spec.message_binding,
            receive_span: spec.receive_span,
            body_span: spec.body_span,
            canonical_bytes: canonical_bytes.into_boxed_slice(),
        })
    }

    #[must_use]
    pub fn actor(&self) -> &str {
        &self.actor
    }

    #[must_use]
    pub const fn actor_type(&self) -> ActorTypeId {
        self.actor_type
    }

    #[must_use]
    pub const fn dispatch(&self) -> ActorTurnDispatch {
        self.dispatch
    }

    #[must_use]
    pub const fn suspension(&self) -> ActorTurnSuspension {
        self.suspension
    }

    #[must_use]
    pub const fn reentry(&self) -> ActorTurnReentry {
        self.reentry
    }

    #[must_use]
    pub const fn state_commit(&self) -> ActorTurnStateCommit {
        self.state_commit
    }

    #[must_use]
    pub const fn self_send(&self) -> ActorTurnSelfSend {
        self.self_send
    }

    #[must_use]
    pub const fn transition(&self) -> u32 {
        self.transition
    }

    #[must_use]
    pub const fn state_binding(&self) -> u32 {
        self.state_binding
    }

    #[must_use]
    pub const fn message_binding(&self) -> u32 {
        self.message_binding
    }

    #[must_use]
    pub const fn receive_span(&self) -> Span {
        self.receive_span
    }

    #[must_use]
    pub const fn body_span(&self) -> Span {
        self.body_span
    }

    #[must_use]
    pub const fn classify_completion(
        &self,
        completion: ActorTurnCompletion,
    ) -> ActorTurnStatePublication {
        match completion {
            ActorTurnCompletion::NormalReturn => ActorTurnStatePublication::PublishCandidate,
            ActorTurnCompletion::Unsuccessful => ActorTurnStatePublication::PreservePrevious,
        }
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorTurnContractError {
    MissingActor,
    InvalidActorType,
    AliasedBindings { binding: u32 },
    InvalidSpans,
}

impl fmt::Display for ActorTurnContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingActor => formatter.write_str("Actor turn owner is empty"),
            Self::InvalidActorType => formatter.write_str("Actor turn type identity is zero"),
            Self::AliasedBindings { binding } => write!(
                formatter,
                "Actor turn state and message bindings alias identity {binding}"
            ),
            Self::InvalidSpans => formatter.write_str("Actor turn source spans are inconsistent"),
        }
    }
}

impl Error for ActorTurnContractError {}

fn push_text(output: &mut Vec<u8>, value: &str) {
    let length = u32::try_from(value.len()).expect("Actor turn canonical domain is bounded");
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value.as_bytes());
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

    fn spec() -> ActorTurnSpec {
        ActorTurnSpec {
            actor: "actor-definition".into(),
            actor_type: ActorTypeId::new(7),
            transition: 11,
            state_binding: 12,
            message_binding: 13,
            receive_span: span(0, 10, 30),
            body_span: span(0, 20, 30),
        }
    }

    #[test]
    fn checked_profile_fixes_all_turn_modes() {
        let contract = ActorTurnContract::new_checked_profile(spec()).expect("valid contract");
        assert_eq!(contract.dispatch().as_str(), "OneMessage");
        assert_eq!(contract.suspension().as_str(), "Forbidden");
        assert_eq!(contract.reentry().as_str(), "Forbidden");
        assert_eq!(contract.state_commit().as_str(), "PublishOnNormalReturn");
        assert_eq!(contract.self_send().as_str(), "MailboxOnly");
    }

    #[test]
    fn completion_classification_is_failure_atomic() {
        let contract = ActorTurnContract::new_checked_profile(spec()).expect("valid contract");
        assert_eq!(
            contract.classify_completion(ActorTurnCompletion::NormalReturn),
            ActorTurnStatePublication::PublishCandidate
        );
        assert_eq!(
            contract.classify_completion(ActorTurnCompletion::Unsuccessful),
            ActorTurnStatePublication::PreservePrevious
        );
    }

    #[test]
    fn canonical_bytes_are_domain_separated_and_deterministic() {
        let first = ActorTurnContract::new_checked_profile(spec()).expect("first contract");
        let mut changed = spec();
        changed.transition = 14;
        let second = ActorTurnContract::new_checked_profile(spec()).expect("second contract");
        let changed = ActorTurnContract::new_checked_profile(changed).expect("changed contract");
        assert_eq!(first.canonical_bytes(), second.canonical_bytes());
        assert_ne!(first.canonical_bytes(), changed.canonical_bytes());
        assert!(
            first
                .canonical_bytes()
                .windows(CHECKED_ACTOR_TURN_VERSION.len())
                .any(|window| window == CHECKED_ACTOR_TURN_VERSION.as_bytes())
        );
    }

    #[test]
    fn rejects_invalid_owner_identities_bindings_and_spans() {
        let mut missing_owner = spec();
        missing_owner.actor = "".into();
        assert_eq!(
            ActorTurnContract::new_checked_profile(missing_owner),
            Err(ActorTurnContractError::MissingActor)
        );

        let mut aliased = spec();
        aliased.message_binding = aliased.state_binding;
        assert!(matches!(
            ActorTurnContract::new_checked_profile(aliased),
            Err(ActorTurnContractError::AliasedBindings { .. })
        ));

        let mut bad_span = spec();
        bad_span.body_span = span(1, 20, 30);
        assert_eq!(
            ActorTurnContract::new_checked_profile(bad_span),
            Err(ActorTurnContractError::InvalidSpans)
        );
    }
}
