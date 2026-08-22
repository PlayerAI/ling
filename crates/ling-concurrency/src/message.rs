//! Non-executable Actor message-schema identity data.

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

id_type!(MessageSchemaId, "One opaque Actor message-schema identity.");
id_type!(
    MessageFieldId,
    "One opaque field identity within a message schema."
);

/// Inputs for one immutable message-schema identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageSchemaSpec {
    pub id: MessageSchemaId,
    pub owner: Option<ActorTypeId>,
    pub fields: Box<[MessageFieldId]>,
    pub source_span: Option<Span>,
}

/// One immutable message-schema identity. It carries no payload type or wire rule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageSchema {
    id: MessageSchemaId,
    owner: Option<ActorTypeId>,
    fields: Box<[MessageFieldId]>,
    source_span: Option<Span>,
}

impl MessageSchema {
    #[must_use]
    pub fn new(spec: MessageSchemaSpec) -> Self {
        Self {
            id: spec.id,
            owner: spec.owner,
            fields: spec.fields,
            source_span: spec.source_span,
        }
    }

    #[must_use]
    pub const fn id(&self) -> MessageSchemaId {
        self.id
    }

    #[must_use]
    pub const fn owner(&self) -> Option<ActorTypeId> {
        self.owner
    }

    #[must_use]
    pub fn fields(&self) -> &[MessageFieldId] {
        &self.fields
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }
}

/// A deterministic, non-executable message-schema identity model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageSchemaIdentityModel {
    schemas: Box<[MessageSchema]>,
    source_span: Option<Span>,
}

impl MessageSchemaIdentityModel {
    pub fn new(
        schemas: impl IntoIterator<Item = MessageSchema>,
        source_span: Option<Span>,
    ) -> Result<Self, MessageSchemaError> {
        let mut schemas = schemas.into_iter().collect::<Vec<_>>();
        schemas.sort_by_key(MessageSchema::id);
        let mut schema_ids = BTreeSet::new();
        for schema in &mut schemas {
            if !schema.id.is_valid() {
                return Err(MessageSchemaError::InvalidIdentity {
                    kind: MessageIdentityKind::Schema,
                    value: schema.id.get(),
                });
            }
            if let Some(owner) = schema.owner {
                if !owner.is_valid() {
                    return Err(MessageSchemaError::InvalidIdentity {
                        kind: MessageIdentityKind::Owner,
                        value: owner.get(),
                    });
                }
            }
            if !schema_ids.insert(schema.id) {
                return Err(MessageSchemaError::DuplicateSchema { schema: schema.id });
            }

            schema.fields.sort_unstable();
            let mut field_ids = BTreeSet::new();
            for field in &schema.fields {
                if !field.is_valid() {
                    return Err(MessageSchemaError::InvalidIdentity {
                        kind: MessageIdentityKind::Field,
                        value: field.get(),
                    });
                }
                if !field_ids.insert(*field) {
                    return Err(MessageSchemaError::DuplicateField {
                        schema: schema.id,
                        field: *field,
                    });
                }
            }
        }

        Ok(Self {
            schemas: schemas.into_boxed_slice(),
            source_span,
        })
    }

    #[must_use]
    pub fn schemas(&self) -> &[MessageSchema] {
        &self.schemas
    }

    #[must_use]
    pub const fn source_span(&self) -> Option<Span> {
        self.source_span
    }

    /// Returns path-free deterministic bytes. Source evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"ling.actor-message-schema/0");
        push_u32(&mut bytes, self.schemas.len() as u32);
        for schema in &self.schemas {
            push_u32(&mut bytes, schema.id.get());
            push_optional_u32(&mut bytes, schema.owner.map(ActorTypeId::get));
            push_u32(&mut bytes, schema.fields.len() as u32);
            for field in &schema.fields {
                push_u32(&mut bytes, field.get());
            }
        }
        bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MessageIdentityKind {
    Schema,
    Owner,
    Field,
}

impl MessageIdentityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Schema => "message schema",
            Self::Owner => "message owner",
            Self::Field => "message field",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageSchemaError {
    InvalidIdentity {
        kind: MessageIdentityKind,
        value: u32,
    },
    DuplicateSchema {
        schema: MessageSchemaId,
    },
    DuplicateField {
        schema: MessageSchemaId,
        field: MessageFieldId,
    },
}

impl fmt::Display for MessageSchemaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { kind, value } => {
                write!(formatter, "invalid {} identity {value}", kind.as_str())
            }
            Self::DuplicateSchema { schema } => {
                write!(formatter, "duplicate message schema {}", schema.get())
            }
            Self::DuplicateField { schema, field } => write!(
                formatter,
                "duplicate message field {} in schema {}",
                field.get(),
                schema.get()
            ),
        }
    }
}

impl Error for MessageSchemaError {}

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

    fn schema(
        id: u32,
        owner: Option<u32>,
        fields: &[u32],
        source_span: Option<Span>,
    ) -> MessageSchema {
        MessageSchema::new(MessageSchemaSpec {
            id: MessageSchemaId::new(id),
            owner: owner.map(ActorTypeId::new),
            fields: fields
                .iter()
                .copied()
                .map(MessageFieldId::new)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            source_span,
        })
    }

    #[test]
    fn validates_and_orders_message_schema_identities() {
        let model = MessageSchemaIdentityModel::new(
            [
                schema(2, Some(7), &[9, 8], None),
                schema(1, None, &[3, 2], None),
            ],
            None,
        )
        .expect("message schema model is valid");
        assert_eq!(
            model
                .schemas()
                .iter()
                .map(|schema| schema.id().get())
                .collect::<Vec<_>>(),
            [1, 2]
        );
        assert_eq!(
            model.schemas()[1].fields(),
            [MessageFieldId::new(8), MessageFieldId::new(9)]
        );
    }

    #[test]
    fn canonical_bytes_ignore_input_order_and_source_evidence() {
        let first = MessageSchemaIdentityModel::new(
            [schema(1, Some(7), &[2, 1], Some(span(1, 1, 2)))],
            Some(span(1, 0, 2)),
        )
        .expect("first model")
        .canonical_bytes();
        let second = MessageSchemaIdentityModel::new(
            [schema(1, Some(7), &[1, 2], Some(span(9, 10, 11)))],
            Some(span(9, 0, 11)),
        )
        .expect("second model")
        .canonical_bytes();
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_invalid_duplicate_and_repeated_field_identities() {
        let invalid = MessageSchemaIdentityModel::new([schema(0, None, &[], None)], None)
            .expect_err("zero schema identity must be rejected");
        assert!(matches!(
            invalid,
            MessageSchemaError::InvalidIdentity { .. }
        ));

        let duplicate = MessageSchemaIdentityModel::new(
            [schema(1, None, &[], None), schema(1, None, &[], None)],
            None,
        )
        .expect_err("duplicate schema identity must be rejected");
        assert!(matches!(
            duplicate,
            MessageSchemaError::DuplicateSchema { .. }
        ));

        let repeated_field =
            MessageSchemaIdentityModel::new([schema(1, None, &[4, 4], None)], None)
                .expect_err("repeated field identity must be rejected");
        assert!(matches!(
            repeated_field,
            MessageSchemaError::DuplicateField { .. }
        ));
    }
}
