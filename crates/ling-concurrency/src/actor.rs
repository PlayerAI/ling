//! Non-executable Actor identity data for the future Actor boundary.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use ling_source::Span;

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

id_type!(ActorTypeId, "One opaque Actor type identity.");
id_type!(ActorId, "One opaque Actor instance identity.");
id_type!(ActorRefId, "One opaque Actor reference identity.");

/// Local/remote labels are structural observations, not serialization rules.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActorReferenceKind {
    Local,
    Remote,
}

impl ActorReferenceKind {
    const fn rank(self) -> u8 {
        match self {
            Self::Local => 0,
            Self::Remote => 1,
        }
    }
}

/// Inputs for one opaque Actor type identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorTypeSpec {
    pub id: ActorTypeId,
    pub source_span: Option<Span>,
}

/// One immutable opaque Actor type identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorType {
    id: ActorTypeId,
    source_span: Option<Span>,
}

impl ActorType {
    #[must_use]
    pub const fn new(spec: ActorTypeSpec) -> Self {
        Self {
            id: spec.id,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> ActorTypeId {
        self.id
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// Inputs for one opaque Actor instance identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorInstanceSpec {
    pub id: ActorId,
    pub actor_type: ActorTypeId,
    pub source_span: Option<Span>,
}

/// One immutable opaque Actor instance identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorInstance {
    id: ActorId,
    actor_type: ActorTypeId,
    source_span: Option<Span>,
}

impl ActorInstance {
    #[must_use]
    pub const fn new(spec: ActorInstanceSpec) -> Self {
        Self {
            id: spec.id,
            actor_type: spec.actor_type,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> ActorId {
        self.id
    }

    #[must_use]
    pub const fn actor_type(self) -> ActorTypeId {
        self.actor_type
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// Inputs for one opaque Actor reference identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorReferenceSpec {
    pub id: ActorRefId,
    pub actor: ActorId,
    pub kind: ActorReferenceKind,
    pub source_span: Option<Span>,
}

/// One immutable opaque Actor reference identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorReference {
    id: ActorRefId,
    actor: ActorId,
    kind: ActorReferenceKind,
    source_span: Option<Span>,
}

impl ActorReference {
    #[must_use]
    pub const fn new(spec: ActorReferenceSpec) -> Self {
        Self {
            id: spec.id,
            actor: spec.actor,
            kind: spec.kind,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> ActorRefId {
        self.id
    }

    #[must_use]
    pub const fn actor(self) -> ActorId {
        self.actor
    }

    #[must_use]
    pub const fn kind(self) -> ActorReferenceKind {
        self.kind
    }

    #[must_use]
    pub const fn source_span(self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable Actor identity model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorIdentityModel {
    types: Box<[ActorType]>,
    actors: Box<[ActorInstance]>,
    references: Box<[ActorReference]>,
    source_span: Option<Span>,
}

impl ActorIdentityModel {
    pub fn new(
        types: impl IntoIterator<Item = ActorType>,
        actors: impl IntoIterator<Item = ActorInstance>,
        references: impl IntoIterator<Item = ActorReference>,
        source_span: Option<Span>,
    ) -> Result<Self, ActorIdentityError> {
        let mut types = types.into_iter().collect::<Vec<_>>();
        let mut actors = actors.into_iter().collect::<Vec<_>>();
        let mut references = references.into_iter().collect::<Vec<_>>();
        types.sort_by_key(ActorType::id);
        actors.sort_by_key(ActorInstance::id);
        references.sort_by_key(ActorReference::id);

        let mut type_ids = BTreeSet::new();
        for actor_type in &types {
            if !actor_type.id.is_valid() {
                return Err(ActorIdentityError::InvalidIdentity {
                    kind: ActorIdentityKind::Type,
                    value: actor_type.id.get(),
                });
            }
            if !type_ids.insert(actor_type.id) {
                return Err(ActorIdentityError::DuplicateType {
                    actor_type: actor_type.id,
                });
            }
        }

        let mut actor_ids = BTreeSet::new();
        for actor in &actors {
            if !actor.id.is_valid() {
                return Err(ActorIdentityError::InvalidIdentity {
                    kind: ActorIdentityKind::Actor,
                    value: actor.id.get(),
                });
            }
            if !actor.actor_type.is_valid() {
                return Err(ActorIdentityError::InvalidIdentity {
                    kind: ActorIdentityKind::Type,
                    value: actor.actor_type.get(),
                });
            }
            if !type_ids.contains(&actor.actor_type) {
                return Err(ActorIdentityError::UnknownType {
                    actor: actor.id,
                    actor_type: actor.actor_type,
                });
            }
            if !actor_ids.insert(actor.id) {
                return Err(ActorIdentityError::DuplicateActor { actor: actor.id });
            }
        }

        let mut reference_ids = BTreeSet::new();
        for reference in &references {
            if !reference.id.is_valid() {
                return Err(ActorIdentityError::InvalidIdentity {
                    kind: ActorIdentityKind::Reference,
                    value: reference.id.get(),
                });
            }
            if !reference.actor.is_valid() {
                return Err(ActorIdentityError::InvalidIdentity {
                    kind: ActorIdentityKind::Actor,
                    value: reference.actor.get(),
                });
            }
            if !actor_ids.contains(&reference.actor) {
                return Err(ActorIdentityError::UnknownActor {
                    reference: reference.id,
                    actor: reference.actor,
                });
            }
            if !reference_ids.insert(reference.id) {
                return Err(ActorIdentityError::DuplicateReference {
                    reference: reference.id,
                });
            }
        }

        Ok(Self {
            types: types.into_boxed_slice(),
            actors: actors.into_boxed_slice(),
            references: references.into_boxed_slice(),
            source_span,
        })
    }

    #[must_use]
    pub fn types(&self) -> &[ActorType] {
        &self.types
    }

    #[must_use]
    pub fn actors(&self) -> &[ActorInstance] {
        &self.actors
    }

    #[must_use]
    pub fn references(&self) -> &[ActorReference] {
        &self.references
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }

    /// Returns path-free deterministic bytes. Source evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"ling.actor-identity/0");
        push_u32(&mut bytes, self.types.len() as u32);
        for actor_type in &self.types {
            push_u32(&mut bytes, actor_type.id.get());
        }
        push_u32(&mut bytes, self.actors.len() as u32);
        for actor in &self.actors {
            push_u32(&mut bytes, actor.id.get());
            push_u32(&mut bytes, actor.actor_type.get());
        }
        push_u32(&mut bytes, self.references.len() as u32);
        for reference in &self.references {
            push_u32(&mut bytes, reference.id.get());
            push_u32(&mut bytes, reference.actor.get());
            bytes.push(reference.kind.rank());
        }
        bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActorIdentityKind {
    Type,
    Actor,
    Reference,
}

impl ActorIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Type => "actor type",
            Self::Actor => "actor",
            Self::Reference => "actor reference",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActorIdentityError {
    InvalidIdentity {
        kind: ActorIdentityKind,
        value: u32,
    },
    DuplicateType {
        actor_type: ActorTypeId,
    },
    DuplicateActor {
        actor: ActorId,
    },
    DuplicateReference {
        reference: ActorRefId,
    },
    UnknownType {
        actor: ActorId,
        actor_type: ActorTypeId,
    },
    UnknownActor {
        reference: ActorRefId,
        actor: ActorId,
    },
}

impl fmt::Display for ActorIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateType { actor_type } => {
                write!(formatter, "duplicate actor type {}", actor_type.get())
            }
            Self::DuplicateActor { actor } => write!(formatter, "duplicate actor {}", actor.get()),
            Self::DuplicateReference { reference } => {
                write!(formatter, "duplicate actor reference {}", reference.get())
            }
            Self::UnknownType { actor, actor_type } => write!(
                formatter,
                "actor {} refers to unknown actor type {}",
                actor.get(),
                actor_type.get()
            ),
            Self::UnknownActor { reference, actor } => write!(
                formatter,
                "actor reference {} refers to unknown actor {}",
                reference.get(),
                actor.get()
            ),
        }
    }
}

impl Error for ActorIdentityError {}

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

    fn actor_type(id: u32, source_span: Option<Span>) -> ActorType {
        ActorType::new(ActorTypeSpec {
            id: ActorTypeId::new(id),
            source_span,
        })
    }

    fn actor(id: u32, actor_type: u32, source_span: Option<Span>) -> ActorInstance {
        ActorInstance::new(ActorInstanceSpec {
            id: ActorId::new(id),
            actor_type: ActorTypeId::new(actor_type),
            source_span,
        })
    }

    fn reference(id: u32, actor: u32, kind: ActorReferenceKind) -> ActorReference {
        ActorReference::new(ActorReferenceSpec {
            id: ActorRefId::new(id),
            actor: ActorId::new(actor),
            kind,
            source_span: None,
        })
    }

    #[test]
    fn validates_and_orders_actor_identity_facts() {
        let model = ActorIdentityModel::new(
            [actor_type(2, None), actor_type(1, None)],
            [actor(2, 1, None), actor(1, 2, None)],
            [
                reference(2, 1, ActorReferenceKind::Remote),
                reference(1, 1, ActorReferenceKind::Local),
            ],
            None,
        )
        .expect("actor identity model is valid");
        assert_eq!(
            model
                .types()
                .iter()
                .map(|actor_type| actor_type.id().get())
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_eq!(model.actors()[0].id(), ActorId::new(1));
        assert_eq!(model.references()[0].kind(), ActorReferenceKind::Local);
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = ActorIdentityModel::new(
            [actor_type(1, Some(span(1, 1, 2)))],
            [actor(1, 1, Some(span(1, 3, 4)))],
            [reference(1, 1, ActorReferenceKind::Local)],
            Some(span(1, 0, 4)),
        )
        .expect("first model")
        .canonical_bytes();
        let second = ActorIdentityModel::new(
            [actor_type(1, Some(span(9, 10, 11)))],
            [actor(1, 1, Some(span(9, 30, 31)))],
            [reference(1, 1, ActorReferenceKind::Local)],
            Some(span(9, 0, 31)),
        )
        .expect("second model")
        .canonical_bytes();
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_invalid_duplicate_and_unknown_actor_facts() {
        let invalid = ActorIdentityModel::new([actor_type(0, None)], [], [], None)
            .expect_err("zero type identity must be rejected");
        assert!(matches!(
            invalid,
            ActorIdentityError::InvalidIdentity { .. }
        ));

        let duplicate =
            ActorIdentityModel::new([actor_type(1, None), actor_type(1, None)], [], [], None)
                .expect_err("duplicate type identity must be rejected");
        assert!(matches!(
            duplicate,
            ActorIdentityError::DuplicateType { .. }
        ));

        let unknown = ActorIdentityModel::new([actor_type(1, None)], [actor(1, 9, None)], [], None)
            .expect_err("unknown actor type must be rejected");
        assert!(matches!(unknown, ActorIdentityError::UnknownType { .. }));
    }
}
