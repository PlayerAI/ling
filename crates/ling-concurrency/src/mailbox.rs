//! Non-executable Actor mailbox observations.

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

id_type!(MailboxId, "One opaque Actor mailbox identity.");

/// Policy labels are observations only; they do not define send behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MailboxObservationKind {
    Wait,
    Reject,
    DropNewest,
    DropOldest,
    Coalesce,
}

impl MailboxObservationKind {
    const fn rank(self) -> u8 {
        match self {
            Self::Wait => 0,
            Self::Reject => 1,
            Self::DropNewest => 2,
            Self::DropOldest => 3,
            Self::Coalesce => 4,
        }
    }
}

/// Inputs for one immutable mailbox observation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MailboxObservationSpec {
    pub id: MailboxId,
    pub owner: Option<ActorTypeId>,
    pub kind: MailboxObservationKind,
    pub source_span: Option<Span>,
}

/// One immutable mailbox observation. It has no queue or send authority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MailboxObservation {
    id: MailboxId,
    owner: Option<ActorTypeId>,
    kind: MailboxObservationKind,
    source_span: Option<Span>,
}

impl MailboxObservation {
    #[must_use]
    pub const fn new(spec: MailboxObservationSpec) -> Self {
        Self {
            id: spec.id,
            owner: spec.owner,
            kind: spec.kind,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> MailboxId {
        self.id
    }

    #[must_use]
    pub const fn owner(self) -> Option<ActorTypeId> {
        self.owner
    }

    #[must_use]
    pub const fn kind(self) -> MailboxObservationKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable mailbox-observation model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MailboxObservationModel {
    mailboxes: Box<[MailboxObservation]>,
    source_span: Option<Span>,
}

impl MailboxObservationModel {
    pub fn new(
        mailboxes: impl IntoIterator<Item = MailboxObservation>,
        source_span: Option<Span>,
    ) -> Result<Self, MailboxObservationError> {
        let mut mailboxes = mailboxes.into_iter().collect::<Vec<_>>();
        mailboxes.sort_by_key(MailboxObservation::id);
        let mut mailbox_ids = BTreeSet::new();
        for mailbox in &mailboxes {
            if !mailbox.id.is_valid() {
                return Err(MailboxObservationError::InvalidIdentity {
                    kind: MailboxIdentityKind::Mailbox,
                    value: mailbox.id.get(),
                });
            }
            if let Some(owner) = mailbox.owner {
                if !owner.is_valid() {
                    return Err(MailboxObservationError::InvalidIdentity {
                        kind: MailboxIdentityKind::Owner,
                        value: owner.get(),
                    });
                }
            }
            if !mailbox_ids.insert(mailbox.id) {
                return Err(MailboxObservationError::DuplicateMailbox {
                    mailbox: mailbox.id,
                });
            }
        }
        Ok(Self {
            mailboxes: mailboxes.into_boxed_slice(),
            source_span,
        })
    }

    #[must_use]
    pub fn mailboxes(&self) -> &[MailboxObservation] {
        &self.mailboxes
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }

    /// Returns path-free deterministic bytes. Source evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"ling.actor-mailbox-observation/0");
        push_u32(&mut bytes, self.mailboxes.len() as u32);
        for mailbox in &self.mailboxes {
            push_u32(&mut bytes, mailbox.id.get());
            push_optional_u32(&mut bytes, mailbox.owner.map(ActorTypeId::get));
            bytes.push(mailbox.kind.rank());
        }
        bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MailboxIdentityKind {
    Mailbox,
    Owner,
}

impl MailboxIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Mailbox => "mailbox",
            Self::Owner => "mailbox owner",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MailboxObservationError {
    InvalidIdentity {
        kind: MailboxIdentityKind,
        value: u32,
    },
    DuplicateMailbox {
        mailbox: MailboxId,
    },
}

impl fmt::Display for MailboxObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateMailbox { mailbox } => {
                write!(formatter, "duplicate mailbox {}", mailbox.get())
            }
        }
    }
}

impl Error for MailboxObservationError {}

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

    fn mailbox(id: u32, owner: Option<u32>, kind: MailboxObservationKind) -> MailboxObservation {
        MailboxObservation::new(MailboxObservationSpec {
            id: MailboxId::new(id),
            owner: owner.map(ActorTypeId::new),
            kind,
            source_span: None,
        })
    }

    #[test]
    fn validates_and_orders_structural_mailbox_observations() {
        let model = MailboxObservationModel::new(
            [
                mailbox(2, Some(7), MailboxObservationKind::Reject),
                mailbox(1, None, MailboxObservationKind::Wait),
            ],
            None,
        )
        .expect("mailbox observations are valid");
        assert_eq!(
            model
                .mailboxes()
                .iter()
                .map(|mailbox| mailbox.id().get())
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_eq!(model.mailboxes()[1].kind(), MailboxObservationKind::Reject);
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = MailboxObservationModel::new(
            [MailboxObservation::new(MailboxObservationSpec {
                id: MailboxId::new(1),
                owner: Some(ActorTypeId::new(7)),
                kind: MailboxObservationKind::DropOldest,
                source_span: Some(span(1, 1, 2)),
            })],
            Some(span(1, 0, 2)),
        )
        .expect("first model")
        .canonical_bytes();
        let second = MailboxObservationModel::new(
            [MailboxObservation::new(MailboxObservationSpec {
                id: MailboxId::new(1),
                owner: Some(ActorTypeId::new(7)),
                kind: MailboxObservationKind::DropOldest,
                source_span: Some(span(9, 10, 11)),
            })],
            Some(span(9, 0, 11)),
        )
        .expect("second model")
        .canonical_bytes();
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_invalid_duplicate_and_owner_identities() {
        let invalid =
            MailboxObservationModel::new([mailbox(0, None, MailboxObservationKind::Wait)], None)
                .expect_err("zero mailbox identity must be rejected");
        assert!(matches!(
            invalid,
            MailboxObservationError::InvalidIdentity { .. }
        ));

        let duplicate = MailboxObservationModel::new(
            [
                mailbox(1, None, MailboxObservationKind::Wait),
                mailbox(1, None, MailboxObservationKind::Reject),
            ],
            None,
        )
        .expect_err("duplicate mailbox identity must be rejected");
        assert!(matches!(
            duplicate,
            MailboxObservationError::DuplicateMailbox { .. }
        ));

        let owner =
            MailboxObservationModel::new([mailbox(1, Some(0), MailboxObservationKind::Wait)], None)
                .expect_err("zero owner identity must be rejected");
        assert!(matches!(
            owner,
            MailboxObservationError::InvalidIdentity { .. }
        ));
    }
}
