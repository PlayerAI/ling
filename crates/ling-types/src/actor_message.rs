use std::collections::BTreeMap;

use ling_resolve::DefinitionId;

use crate::{Type, TypeId, TypedProgram};

pub const ACTOR_MESSAGE_SCHEMA_DOMAIN: &str = "ling.actor-message-schema-id/v1";
const LANGUAGE_VERSION: &str = "0.0.1-dev";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SendableLocal {
    Value,
}

impl SendableLocal {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Value => "SendableLocal(Value)",
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActorMessageSchemaId(String);

impl ActorMessageSchemaId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorMessageSchema {
    id: ActorMessageSchemaId,
    root: u32,
    nodes: Vec<ActorMessageSchemaNode>,
    canonical_bytes: Box<[u8]>,
}

impl ActorMessageSchema {
    pub fn build(typed: &TypedProgram, root: TypeId) -> Result<Self, ActorMessageSchemaError> {
        let root_key = type_key(typed, root, &BTreeMap::new())?;
        let mut builder = SchemaBuilder::new(typed);
        let root = builder.intern(root_key)?;
        let nodes = builder.finish()?;
        let canonical_bytes = canonical_schema_bytes(root, &nodes);
        let id = ActorMessageSchemaId(format!(
            "experimental:blake3:{}",
            blake3::hash(&canonical_bytes).to_hex()
        ));
        Ok(Self {
            id,
            root,
            nodes,
            canonical_bytes: canonical_bytes.into_boxed_slice(),
        })
    }

    #[must_use]
    pub const fn id(&self) -> &ActorMessageSchemaId {
        &self.id
    }

    #[must_use]
    pub const fn root(&self) -> u32 {
        self.root
    }

    #[must_use]
    pub fn nodes(&self) -> &[ActorMessageSchemaNode] {
        &self.nodes
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorMessageSchemaNode {
    pub id: u32,
    pub kind: ActorMessageSchemaNodeKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActorMessageSchemaNodeKind {
    Primitive {
        name: &'static str,
    },
    Tuple {
        elements: Vec<u32>,
    },
    List {
        element: u32,
    },
    Record {
        definition: DefinitionId,
        arguments: Vec<u32>,
        fields: Vec<ActorMessageField>,
    },
    Variant {
        definition: DefinitionId,
        arguments: Vec<u32>,
        cases: Vec<ActorMessageCase>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorMessageField {
    pub name: String,
    pub value: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorMessageCase {
    pub definition: DefinitionId,
    pub name: String,
    pub payload: Option<u32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActorMessageSchemaError {
    Unsupported(&'static str),
    MissingRecord,
    MissingVariant,
    TypeArgumentArity,
    OpenVariable,
    NonCanonicalGraph,
}

impl ActorMessageSchemaError {
    #[must_use]
    pub const fn reason(self) -> &'static str {
        match self {
            Self::Unsupported(reason) => reason,
            Self::MissingRecord => "message_schema_missing_record",
            Self::MissingVariant => "message_schema_missing_variant",
            Self::TypeArgumentArity => "message_schema_type_argument_arity",
            Self::OpenVariable => "message_schema_open_variable",
            Self::NonCanonicalGraph => "message_schema_non_canonical_graph",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum TypeKey {
    Unit,
    Bool,
    Int,
    Float64,
    Text,
    Tuple(Vec<Self>),
    List(Box<Self>),
    Record {
        definition: DefinitionId,
        arguments: Vec<Self>,
    },
    Variant {
        definition: DefinitionId,
        arguments: Vec<Self>,
    },
}

fn type_key(
    typed: &TypedProgram,
    type_id: TypeId,
    substitutions: &BTreeMap<u32, TypeKey>,
) -> Result<TypeKey, ActorMessageSchemaError> {
    match typed.arena.get(type_id) {
        Type::Unit => Ok(TypeKey::Unit),
        Type::Bool => Ok(TypeKey::Bool),
        Type::Int => Ok(TypeKey::Int),
        Type::Float64 => Ok(TypeKey::Float64),
        Type::Text => Ok(TypeKey::Text),
        Type::Tuple(elements) => Ok(TypeKey::Tuple(
            elements
                .iter()
                .map(|element| type_key(typed, *element, substitutions))
                .collect::<Result<_, _>>()?,
        )),
        Type::List(element) => Ok(TypeKey::List(Box::new(type_key(
            typed,
            *element,
            substitutions,
        )?))),
        Type::NominalRecord {
            definition,
            arguments,
        } => Ok(TypeKey::Record {
            definition: definition.clone(),
            arguments: arguments
                .iter()
                .map(|argument| type_key(typed, *argument, substitutions))
                .collect::<Result<_, _>>()?,
        }),
        Type::NominalVariant {
            definition,
            arguments,
        } => Ok(TypeKey::Variant {
            definition: definition.clone(),
            arguments: arguments
                .iter()
                .map(|argument| type_key(typed, *argument, substitutions))
                .collect::<Result<_, _>>()?,
        }),
        Type::Variable(variable) => substitutions
            .get(variable)
            .cloned()
            .ok_or(ActorMessageSchemaError::OpenVariable),
        Type::Function { .. } => Err(ActorMessageSchemaError::Unsupported(
            "message_type_function_not_sendable_local",
        )),
        Type::Task { .. } => Err(ActorMessageSchemaError::Unsupported(
            "message_type_task_not_sendable_local",
        )),
        Type::TaskHandle { .. } => Err(ActorMessageSchemaError::Unsupported(
            "message_type_task_handle_not_sendable_local",
        )),
        Type::Actor { .. } => Err(ActorMessageSchemaError::Unsupported(
            "message_type_actor_not_sendable_local",
        )),
        Type::Error => Err(ActorMessageSchemaError::Unsupported(
            "message_type_error_not_sendable_local",
        )),
    }
}

struct SchemaBuilder<'a> {
    typed: &'a TypedProgram,
    index: BTreeMap<TypeKey, u32>,
    nodes: Vec<Option<ActorMessageSchemaNode>>,
}

impl<'a> SchemaBuilder<'a> {
    fn new(typed: &'a TypedProgram) -> Self {
        Self {
            typed,
            index: BTreeMap::new(),
            nodes: Vec::new(),
        }
    }

    fn intern(&mut self, key: TypeKey) -> Result<u32, ActorMessageSchemaError> {
        if let Some(id) = self.index.get(&key) {
            return Ok(*id);
        }
        let id = u32::try_from(self.nodes.len())
            .map_err(|_| ActorMessageSchemaError::NonCanonicalGraph)?;
        self.index.insert(key.clone(), id);
        self.nodes.push(None);
        let kind = match key {
            TypeKey::Unit => primitive("Unit"),
            TypeKey::Bool => primitive("Bool"),
            TypeKey::Int => primitive("Int"),
            TypeKey::Float64 => primitive("Float64"),
            TypeKey::Text => primitive("Text"),
            TypeKey::Tuple(elements) => ActorMessageSchemaNodeKind::Tuple {
                elements: elements
                    .into_iter()
                    .map(|element| self.intern(element))
                    .collect::<Result<_, _>>()?,
            },
            TypeKey::List(element) => ActorMessageSchemaNodeKind::List {
                element: self.intern(*element)?,
            },
            TypeKey::Record {
                definition,
                arguments,
            } => self.record_kind(definition, arguments)?,
            TypeKey::Variant {
                definition,
                arguments,
            } => self.variant_kind(definition, arguments)?,
        };
        self.nodes[id as usize] = Some(ActorMessageSchemaNode { id, kind });
        Ok(id)
    }

    fn record_kind(
        &mut self,
        definition: DefinitionId,
        arguments: Vec<TypeKey>,
    ) -> Result<ActorMessageSchemaNodeKind, ActorMessageSchemaError> {
        let record = self
            .typed
            .records
            .get(&definition)
            .ok_or(ActorMessageSchemaError::MissingRecord)?;
        if record.parameter_variables.len() != arguments.len() {
            return Err(ActorMessageSchemaError::TypeArgumentArity);
        }
        let substitutions = record
            .parameter_variables
            .iter()
            .copied()
            .zip(arguments.iter().cloned())
            .collect::<BTreeMap<_, _>>();
        let argument_ids = arguments
            .into_iter()
            .map(|argument| self.intern(argument))
            .collect::<Result<_, _>>()?;
        let fields = record.fields.clone();
        let fields = fields
            .into_iter()
            .map(|field| {
                Ok(ActorMessageField {
                    name: field.name,
                    value: self.intern(type_key(self.typed, field.field_type, &substitutions)?)?,
                })
            })
            .collect::<Result<_, ActorMessageSchemaError>>()?;
        Ok(ActorMessageSchemaNodeKind::Record {
            definition,
            arguments: argument_ids,
            fields,
        })
    }

    fn variant_kind(
        &mut self,
        definition: DefinitionId,
        arguments: Vec<TypeKey>,
    ) -> Result<ActorMessageSchemaNodeKind, ActorMessageSchemaError> {
        let variant = self
            .typed
            .variants
            .get(&definition)
            .ok_or(ActorMessageSchemaError::MissingVariant)?;
        if variant.parameter_variables.len() != arguments.len() {
            return Err(ActorMessageSchemaError::TypeArgumentArity);
        }
        let substitutions = variant
            .parameter_variables
            .iter()
            .copied()
            .zip(arguments.iter().cloned())
            .collect::<BTreeMap<_, _>>();
        let argument_ids = arguments
            .into_iter()
            .map(|argument| self.intern(argument))
            .collect::<Result<_, _>>()?;
        let cases = variant.cases.clone();
        let cases = cases
            .into_iter()
            .map(|case| {
                let payload = case
                    .payload
                    .map(|payload| self.intern(type_key(self.typed, payload, &substitutions)?))
                    .transpose()?;
                Ok(ActorMessageCase {
                    definition: case.definition,
                    name: case.name,
                    payload,
                })
            })
            .collect::<Result<_, ActorMessageSchemaError>>()?;
        Ok(ActorMessageSchemaNodeKind::Variant {
            definition,
            arguments: argument_ids,
            cases,
        })
    }

    fn finish(self) -> Result<Vec<ActorMessageSchemaNode>, ActorMessageSchemaError> {
        self.nodes
            .into_iter()
            .map(|node| node.ok_or(ActorMessageSchemaError::NonCanonicalGraph))
            .collect()
    }
}

const fn primitive(name: &'static str) -> ActorMessageSchemaNodeKind {
    ActorMessageSchemaNodeKind::Primitive { name }
}

fn canonical_schema_bytes(root: u32, nodes: &[ActorMessageSchemaNode]) -> Vec<u8> {
    let mut output = Vec::new();
    push_text(&mut output, ACTOR_MESSAGE_SCHEMA_DOMAIN);
    push_text(&mut output, LANGUAGE_VERSION);
    push_u32(&mut output, root);
    push_u32(&mut output, u32::try_from(nodes.len()).unwrap_or(u32::MAX));
    for node in nodes {
        push_u32(&mut output, node.id);
        match &node.kind {
            ActorMessageSchemaNodeKind::Primitive { name } => {
                output.push(0);
                push_text(&mut output, name);
            }
            ActorMessageSchemaNodeKind::Tuple { elements } => {
                output.push(1);
                push_ids(&mut output, elements);
            }
            ActorMessageSchemaNodeKind::List { element } => {
                output.push(2);
                push_u32(&mut output, *element);
            }
            ActorMessageSchemaNodeKind::Record {
                definition,
                arguments,
                fields,
            } => {
                output.push(3);
                push_text(&mut output, definition.as_str());
                push_ids(&mut output, arguments);
                push_u32(&mut output, u32::try_from(fields.len()).unwrap_or(u32::MAX));
                for field in fields {
                    push_text(&mut output, &field.name);
                    push_u32(&mut output, field.value);
                }
            }
            ActorMessageSchemaNodeKind::Variant {
                definition,
                arguments,
                cases,
            } => {
                output.push(4);
                push_text(&mut output, definition.as_str());
                push_ids(&mut output, arguments);
                push_u32(&mut output, u32::try_from(cases.len()).unwrap_or(u32::MAX));
                for case in cases {
                    push_text(&mut output, case.definition.as_str());
                    push_text(&mut output, &case.name);
                    match case.payload {
                        Some(payload) => {
                            output.push(1);
                            push_u32(&mut output, payload);
                        }
                        None => output.push(0),
                    }
                }
            }
        }
    }
    output
}

fn push_ids(output: &mut Vec<u8>, values: &[u32]) {
    push_u32(output, u32::try_from(values.len()).unwrap_or(u32::MAX));
    for value in values {
        push_u32(output, *value);
    }
}

fn push_text(output: &mut Vec<u8>, value: &str) {
    push_u32(output, u32::try_from(value.len()).unwrap_or(u32::MAX));
    output.extend_from_slice(value.as_bytes());
}

fn push_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_le_bytes());
}
