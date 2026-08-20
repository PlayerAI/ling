//! Deterministic checked-program snapshots and `ling.semantic/0.1` JSON.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_effects::CheckedProgram;
use ling_hir as hir;
use ling_resolve::{
    DefinitionId, DefinitionKind, DefinitionOrigin, ExpressionKey, ModuleId, PRELUDE_MODULE,
    ReferenceTarget,
};
use serde::{Deserialize, Serialize};

pub const SEMANTIC_SCHEMA: &str = "ling.semantic/0.1";
pub const LANGUAGE_VERSION: &str = "0.0.1-dev";

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BodyId(String);

impl BodyId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BodyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProgramId(String);

impl ProgramId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProgramId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticGraph {
    pub schema: String,
    pub language_version: String,
    pub unicode_version: String,
    pub program_id: String,
    pub entry_module: String,
    pub modules: Vec<SemanticModule>,
    pub definitions: Vec<SemanticDefinition>,
    #[serde(default)]
    pub nodes: Vec<SemanticNode>,
    pub references: Vec<SemanticReference>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticModule {
    pub name: String,
    pub explicit: bool,
    pub requires: Vec<String>,
    pub imports: Vec<SemanticImport>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticImport {
    pub alias: String,
    pub module: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticDefinition {
    pub definition_id: String,
    pub body_id: String,
    pub module: String,
    pub name: String,
    pub kind: String,
    pub origin: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub effects: Vec<String>,
    pub capabilities: Vec<String>,
}

/// A deterministic non-definition node required by RFC-0001 §6.11.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticNode {
    pub node_id: String,
    pub module: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub owner: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<u32>,
    #[serde(default)]
    pub effects: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier_skeleton: Option<String>,
    #[serde(default)]
    pub identifier_scripts: Vec<String>,
    #[serde(default)]
    pub identifier_suspicious_mixed_script: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticReference {
    pub module: String,
    #[serde(default = "default_reference_source_kind")]
    pub source_kind: String,
    pub reference: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    pub target_kind: String,
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditModel {
    pub language_version: String,
    pub semantic_schema: String,
    pub unicode_version: String,
    pub program_id: String,
    pub entry_module: String,
    pub modules: Vec<AuditModule>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditModule {
    pub name: String,
    pub explicit: bool,
    pub capabilities: Vec<String>,
    pub imports: Vec<SemanticImport>,
    pub definitions: Vec<AuditDefinition>,
    pub nodes: Vec<AuditNode>,
    pub references: Vec<AuditReference>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditDefinition {
    pub definition_id: String,
    pub body_id: String,
    pub name: String,
    pub kind: String,
    pub origin: String,
    pub type_name: String,
    pub effects: Vec<String>,
    pub capabilities: Vec<String>,
    pub unicode_source: String,
    pub unicode_nfc: String,
    pub unicode_skeleton: String,
    pub unicode_scripts: Vec<String>,
    pub unicode_suspicious_mixed_script: bool,
    pub implementation: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditNode {
    pub node_id: String,
    pub kind: String,
    pub name: Option<String>,
    pub owner: String,
    pub type_name: Option<String>,
    pub mutable: Option<bool>,
    pub ordinal: Option<u32>,
    pub effects: Vec<String>,
    pub capabilities: Vec<String>,
    pub identifier_source: Option<String>,
    pub identifier_skeleton: Option<String>,
    pub identifier_scripts: Vec<String>,
    pub identifier_suspicious_mixed_script: bool,
    pub implementation: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditReference {
    pub source_kind: String,
    pub reference: u32,
    pub source_id: Option<String>,
    pub target_kind: String,
    pub target: String,
}

fn default_reference_source_kind() -> String {
    "expression".to_owned()
}

#[derive(Clone, Debug)]
pub struct ProgramSnapshot {
    checked: CheckedProgram,
    graph: SemanticGraph,
    body_ids: BTreeMap<DefinitionId, BodyId>,
    program_id: ProgramId,
    json: String,
}

impl ProgramSnapshot {
    #[must_use]
    pub const fn checked(&self) -> &CheckedProgram {
        &self.checked
    }

    #[must_use]
    pub const fn graph(&self) -> &SemanticGraph {
        &self.graph
    }

    #[must_use]
    pub fn body_id(&self, definition: &DefinitionId) -> Option<&BodyId> {
        self.body_ids.get(definition)
    }

    #[must_use]
    pub const fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    #[must_use]
    pub fn json(&self) -> &str {
        &self.json
    }

    #[must_use]
    pub fn audit_model(&self) -> AuditModel {
        let resolved = self.checked.typed().resolved();
        let mut modules = self
            .graph
            .modules
            .iter()
            .map(|module| {
                (
                    module.name.clone(),
                    AuditModule {
                        name: module.name.clone(),
                        explicit: module.explicit,
                        capabilities: module.requires.clone(),
                        imports: module.imports.clone(),
                        definitions: Vec::new(),
                        nodes: Vec::new(),
                        references: Vec::new(),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        for definition in &self.graph.definitions {
            let info = resolved
                .definitions()
                .values()
                .find(|info| info.id.as_str() == definition.definition_id)
                .expect("snapshot definitions originate from the resolved program");
            let module = modules
                .entry(definition.module.clone())
                .or_insert_with(|| AuditModule {
                    name: definition.module.clone(),
                    explicit: false,
                    capabilities: Vec::new(),
                    imports: Vec::new(),
                    definitions: Vec::new(),
                    nodes: Vec::new(),
                    references: Vec::new(),
                });
            module.definitions.push(AuditDefinition {
                definition_id: definition.definition_id.clone(),
                body_id: definition.body_id.clone(),
                name: definition.name.clone(),
                kind: definition.kind.clone(),
                origin: definition.origin.clone(),
                type_name: definition.type_name.clone(),
                effects: definition.effects.clone(),
                capabilities: definition.capabilities.clone(),
                unicode_source: info.name_source.clone(),
                unicode_nfc: info.name.clone(),
                unicode_skeleton: info.name_skeleton.clone(),
                unicode_scripts: info.name_scripts.clone(),
                unicode_suspicious_mixed_script: info.name_suspicious_mixed_script,
                implementation: "implemented".to_owned(),
            });
        }
        for node in &self.graph.nodes {
            modules
                .entry(node.module.clone())
                .or_insert_with(|| AuditModule {
                    name: node.module.clone(),
                    explicit: false,
                    capabilities: Vec::new(),
                    imports: Vec::new(),
                    definitions: Vec::new(),
                    nodes: Vec::new(),
                    references: Vec::new(),
                })
                .nodes
                .push(AuditNode {
                    node_id: node.node_id.clone(),
                    kind: node.kind.clone(),
                    name: node.name.clone(),
                    owner: node.owner.clone(),
                    type_name: node.type_name.clone(),
                    mutable: node.mutable,
                    ordinal: node.ordinal,
                    effects: node.effects.clone(),
                    capabilities: node.capabilities.clone(),
                    identifier_source: node.identifier_source.clone(),
                    identifier_skeleton: node.identifier_skeleton.clone(),
                    identifier_scripts: node.identifier_scripts.clone(),
                    identifier_suspicious_mixed_script: node.identifier_suspicious_mixed_script,
                    implementation: "implemented".to_owned(),
                });
        }
        for reference in &self.graph.references {
            modules
                .get_mut(&reference.module)
                .expect("reference modules originate from the resolved program")
                .references
                .push(AuditReference {
                    source_kind: reference.source_kind.clone(),
                    reference: reference.reference,
                    source_id: reference.source_id.clone(),
                    target_kind: reference.target_kind.clone(),
                    target: reference.target.clone(),
                });
        }
        for module in modules.values_mut() {
            module.capabilities.sort();
            module.imports.sort_by(|left, right| {
                (&left.alias, &left.module).cmp(&(&right.alias, &right.module))
            });
            module
                .definitions
                .sort_by(|left, right| left.definition_id.cmp(&right.definition_id));
            module
                .nodes
                .sort_by(|left, right| left.node_id.cmp(&right.node_id));
            module.references.sort_by(|left, right| {
                (
                    &left.source_kind,
                    left.reference,
                    &left.target_kind,
                    &left.target,
                )
                    .cmp(&(
                        &right.source_kind,
                        right.reference,
                        &right.target_kind,
                        &right.target,
                    ))
            });
        }
        AuditModel {
            language_version: self.graph.language_version.clone(),
            semantic_schema: self.graph.schema.clone(),
            unicode_version: self.graph.unicode_version.clone(),
            program_id: self.graph.program_id.clone(),
            entry_module: self.graph.entry_module.clone(),
            modules: modules.into_values().collect(),
        }
    }
}

#[derive(Debug)]
pub struct SnapshotError(serde_json::Error);

impl fmt::Display for SnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to serialize semantic snapshot: {}",
            self.0
        )
    }
}

impl Error for SnapshotError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticReadError {
    pub kind: SemanticReadErrorKind,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticReadErrorKind {
    InvalidJson {
        message: String,
    },
    ExpectedObject,
    UnknownField {
        field: String,
    },
    InvalidSchema {
        actual: String,
    },
    InvalidLanguageVersion {
        actual: String,
    },
    InvalidUnicodeVersion {
        actual: String,
    },
    InvalidId {
        value: String,
    },
    DuplicateId {
        value: String,
    },
    DuplicateModule {
        module: String,
    },
    MissingEntryModule {
        module: String,
    },
    InvalidDefinitionKind {
        kind: String,
    },
    InvalidNodeKind {
        kind: String,
    },
    InvalidOrigin {
        origin: String,
    },
    InvalidPreludeModule {
        module: String,
    },
    InvalidImplementation {
        value: String,
    },
    InvalidReferenceSourceKind {
        kind: String,
    },
    InvalidReferenceTargetKind {
        kind: String,
    },
    DanglingReference {
        target: String,
    },
    DanglingOwner {
        owner: String,
    },
    CyclicOwner {
        node_id: String,
    },
    UnknownModule {
        module: String,
    },
    DuplicateReference {
        module: String,
        source_kind: String,
        reference: u32,
    },
    DuplicateImportAlias {
        module: String,
        alias: String,
    },
}

impl fmt::Display for SemanticReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid semantic graph at {}: {:?}",
            self.path, self.kind
        )
    }
}

impl Error for SemanticReadError {}

/// Builds a checked snapshot using versioned canonical binary hash inputs.
pub fn build(checked: CheckedProgram) -> Result<ProgramSnapshot, SnapshotError> {
    SnapshotBuilder::new(checked).build()
}

/// Parses and structurally validates a `ling.semantic/0.1` JSON document.
///
/// Unknown fields are accepted only when their names begin with `x-`, keeping
/// extensions forward-compatible without silently accepting misspelled core
/// protocol fields. The returned graph is data only and cannot be converted
/// into the checked snapshot required by the evaluator.
pub fn read_json(input: &str) -> Result<SemanticGraph, SemanticReadError> {
    let value: serde_json::Value =
        serde_json::from_str(input).map_err(|error| SemanticReadError {
            kind: SemanticReadErrorKind::InvalidJson {
                message: error.to_string(),
            },
            path: "$".to_owned(),
        })?;
    validate_json_fields(&value)?;
    let graph: SemanticGraph =
        serde_json::from_value(value).map_err(|error| SemanticReadError {
            kind: SemanticReadErrorKind::InvalidJson {
                message: error.to_string(),
            },
            path: "$".to_owned(),
        })?;
    validate_graph(&graph)?;
    Ok(graph)
}

/// Validates an isolated Audit model without granting it executable status.
pub fn validate_audit_model(model: &AuditModel) -> Result<(), SemanticReadError> {
    let graph = SemanticGraph {
        schema: model.semantic_schema.clone(),
        language_version: model.language_version.clone(),
        unicode_version: model.unicode_version.clone(),
        program_id: model.program_id.clone(),
        entry_module: model.entry_module.clone(),
        modules: model
            .modules
            .iter()
            .map(|module| SemanticModule {
                name: module.name.clone(),
                explicit: module.explicit,
                requires: module.capabilities.clone(),
                imports: module.imports.clone(),
            })
            .collect(),
        definitions: model
            .modules
            .iter()
            .flat_map(|module| {
                module
                    .definitions
                    .iter()
                    .map(|definition| SemanticDefinition {
                        definition_id: definition.definition_id.clone(),
                        body_id: definition.body_id.clone(),
                        module: module.name.clone(),
                        name: definition.name.clone(),
                        kind: definition.kind.clone(),
                        origin: definition.origin.clone(),
                        type_name: definition.type_name.clone(),
                        effects: definition.effects.clone(),
                        capabilities: definition.capabilities.clone(),
                    })
            })
            .collect(),
        nodes: model
            .modules
            .iter()
            .flat_map(|module| {
                module.nodes.iter().map(|node| SemanticNode {
                    node_id: node.node_id.clone(),
                    module: module.name.clone(),
                    kind: node.kind.clone(),
                    name: node.name.clone(),
                    owner: node.owner.clone(),
                    type_name: node.type_name.clone(),
                    mutable: node.mutable,
                    ordinal: node.ordinal,
                    effects: node.effects.clone(),
                    capabilities: node.capabilities.clone(),
                    identifier_source: node.identifier_source.clone(),
                    identifier_skeleton: node.identifier_skeleton.clone(),
                    identifier_scripts: node.identifier_scripts.clone(),
                    identifier_suspicious_mixed_script: node.identifier_suspicious_mixed_script,
                })
            })
            .collect(),
        references: model
            .modules
            .iter()
            .flat_map(|module| {
                module.references.iter().map(|reference| SemanticReference {
                    module: module.name.clone(),
                    source_kind: reference.source_kind.clone(),
                    reference: reference.reference,
                    source_id: reference.source_id.clone(),
                    target_kind: reference.target_kind.clone(),
                    target: reference.target.clone(),
                })
            })
            .collect(),
    };
    validate_graph(&graph)?;
    for (module_index, module) in model.modules.iter().enumerate() {
        for (definition_index, definition) in module.definitions.iter().enumerate() {
            if definition.implementation != "implemented" {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::InvalidImplementation {
                        value: definition.implementation.clone(),
                    },
                    path: format!(
                        "$.modules[{module_index}].definitions[{definition_index}].implementation"
                    ),
                });
            }
        }
        for (node_index, node) in module.nodes.iter().enumerate() {
            if node.implementation != "implemented" {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::InvalidImplementation {
                        value: node.implementation.clone(),
                    },
                    path: format!("$.modules[{module_index}].nodes[{node_index}].implementation"),
                });
            }
        }
    }
    Ok(())
}

fn validate_json_fields(value: &serde_json::Value) -> Result<(), SemanticReadError> {
    validate_object_fields(
        value,
        "$",
        &[
            "schema",
            "language_version",
            "unicode_version",
            "program_id",
            "entry_module",
            "modules",
            "definitions",
            "nodes",
            "references",
        ],
    )?;
    validate_array_objects(
        value.get("modules"),
        "$.modules",
        &["name", "explicit", "requires", "imports"],
        |module, path| {
            validate_array_objects(
                module.get("imports"),
                &format!("{path}.imports"),
                &["alias", "module"],
                |_, _| Ok(()),
            )
        },
    )?;
    validate_array_objects(
        value.get("definitions"),
        "$.definitions",
        &[
            "definition_id",
            "body_id",
            "module",
            "name",
            "kind",
            "origin",
            "type",
            "effects",
            "capabilities",
        ],
        |_, _| Ok(()),
    )?;
    validate_array_objects(
        value.get("nodes"),
        "$.nodes",
        &[
            "node_id",
            "module",
            "kind",
            "name",
            "owner",
            "type",
            "mutable",
            "ordinal",
            "effects",
            "capabilities",
            "identifier_source",
            "identifier_skeleton",
            "identifier_scripts",
            "identifier_suspicious_mixed_script",
        ],
        |_, _| Ok(()),
    )?;
    validate_array_objects(
        value.get("references"),
        "$.references",
        &[
            "module",
            "source_kind",
            "reference",
            "source_id",
            "target_kind",
            "target",
        ],
        |_, _| Ok(()),
    )
}

fn validate_array_objects(
    value: Option<&serde_json::Value>,
    path: &str,
    fields: &[&str],
    mut nested: impl FnMut(&serde_json::Value, &str) -> Result<(), SemanticReadError>,
) -> Result<(), SemanticReadError> {
    let Some(serde_json::Value::Array(values)) = value else {
        return Ok(());
    };
    for (index, value) in values.iter().enumerate() {
        let item_path = format!("{path}[{index}]");
        validate_object_fields(value, &item_path, fields)?;
        nested(value, &item_path)?;
    }
    Ok(())
}

fn validate_object_fields(
    value: &serde_json::Value,
    path: &str,
    fields: &[&str],
) -> Result<(), SemanticReadError> {
    let serde_json::Value::Object(object) = value else {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::ExpectedObject,
            path: path.to_owned(),
        });
    };
    for field in object.keys() {
        if !fields.contains(&field.as_str()) && !field.starts_with("x-") {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::UnknownField {
                    field: field.clone(),
                },
                path: format!("{path}.{field}"),
            });
        }
    }
    Ok(())
}

fn validate_graph(graph: &SemanticGraph) -> Result<(), SemanticReadError> {
    if graph.schema != SEMANTIC_SCHEMA {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::InvalidSchema {
                actual: graph.schema.clone(),
            },
            path: "$.schema".to_owned(),
        });
    }
    if graph.language_version != LANGUAGE_VERSION {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::InvalidLanguageVersion {
                actual: graph.language_version.clone(),
            },
            path: "$.language_version".to_owned(),
        });
    }
    if graph.unicode_version != ling_unicode::UNICODE_VERSION.to_string() {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::InvalidUnicodeVersion {
                actual: graph.unicode_version.clone(),
            },
            path: "$.unicode_version".to_owned(),
        });
    }
    validate_id(&graph.program_id, "$.program_id")?;

    let mut modules = BTreeSet::new();
    for (index, module) in graph.modules.iter().enumerate() {
        if !modules.insert(module.name.clone()) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DuplicateModule {
                    module: module.name.clone(),
                },
                path: format!("$.modules[{index}].name"),
            });
        }
    }
    if !modules.contains(&graph.entry_module) {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::MissingEntryModule {
                module: graph.entry_module.clone(),
            },
            path: "$.entry_module".to_owned(),
        });
    }
    for (module_index, module) in graph.modules.iter().enumerate() {
        let mut aliases = BTreeSet::new();
        for (import_index, import) in module.imports.iter().enumerate() {
            if !aliases.insert(import.alias.clone()) {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::DuplicateImportAlias {
                        module: module.name.clone(),
                        alias: import.alias.clone(),
                    },
                    path: format!("$.modules[{module_index}].imports[{import_index}].alias"),
                });
            }
            if !modules.contains(&import.module) {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::UnknownModule {
                        module: import.module.clone(),
                    },
                    path: format!("$.modules[{module_index}].imports[{import_index}].module"),
                });
            }
        }
    }

    let mut definitions = BTreeSet::new();
    let mut definition_modules = BTreeMap::new();
    let mut bodies = BTreeSet::new();
    for (index, definition) in graph.definitions.iter().enumerate() {
        validate_id(
            &definition.definition_id,
            &format!("$.definitions[{index}].definition_id"),
        )?;
        validate_id(
            &definition.body_id,
            &format!("$.definitions[{index}].body_id"),
        )?;
        if !definitions.insert(definition.definition_id.clone()) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DuplicateId {
                    value: definition.definition_id.clone(),
                },
                path: format!("$.definitions[{index}].definition_id"),
            });
        }
        definition_modules.insert(definition.definition_id.clone(), definition.module.clone());
        if !bodies.insert(definition.body_id.clone()) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DuplicateId {
                    value: definition.body_id.clone(),
                },
                path: format!("$.definitions[{index}].body_id"),
            });
        }
        if !matches!(
            definition.kind.as_str(),
            "value" | "type" | "constructor" | "builtin"
        ) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::InvalidDefinitionKind {
                    kind: definition.kind.clone(),
                },
                path: format!("$.definitions[{index}].kind"),
            });
        }
        if !matches!(definition.origin.as_str(), "user" | "builtin" | "prelude") {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::InvalidOrigin {
                    origin: definition.origin.clone(),
                },
                path: format!("$.definitions[{index}].origin"),
            });
        }
        if definition.origin == "user" && !modules.contains(&definition.module) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::UnknownModule {
                    module: definition.module.clone(),
                },
                path: format!("$.definitions[{index}].module"),
            });
        }
        if definition.origin == "prelude" && definition.module != PRELUDE_MODULE {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::InvalidPreludeModule {
                    module: definition.module.clone(),
                },
                path: format!("$.definitions[{index}].module"),
            });
        }
    }

    let allowed_node_kinds = [
        "field",
        "variant",
        "binding",
        "function",
        "parameter",
        "pattern",
        "expression",
        "effect",
        "capability",
    ];
    let mut nodes = BTreeMap::new();
    let mut node_owners = BTreeMap::new();
    for (index, node) in graph.nodes.iter().enumerate() {
        validate_id(&node.node_id, &format!("$.nodes[{index}].node_id"))?;
        if definitions.contains(&node.node_id)
            || nodes
                .insert(node.node_id.clone(), node.kind.clone())
                .is_some()
        {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DuplicateId {
                    value: node.node_id.clone(),
                },
                path: format!("$.nodes[{index}].node_id"),
            });
        }
        if !allowed_node_kinds.contains(&node.kind.as_str()) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::InvalidNodeKind {
                    kind: node.kind.clone(),
                },
                path: format!("$.nodes[{index}].kind"),
            });
        }
        node_owners.insert(node.node_id.clone(), node.owner.clone());
    }
    for (index, node) in graph.nodes.iter().enumerate() {
        let owner_exists = definitions.contains(&node.owner) || nodes.contains_key(&node.owner);
        if !owner_exists {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DanglingOwner {
                    owner: node.owner.clone(),
                },
                path: format!("$.nodes[{index}].owner"),
            });
        }
        let owner_module = definition_modules.get(&node.owner).or_else(|| {
            graph
                .nodes
                .iter()
                .find(|candidate| candidate.node_id == node.owner)
                .map(|candidate| &candidate.module)
        });
        if owner_module.is_some_and(|module| module != &node.module) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::UnknownModule {
                    module: node.module.clone(),
                },
                path: format!("$.nodes[{index}].module"),
            });
        }
        let mut seen = BTreeSet::new();
        let mut owner = node.owner.as_str();
        while let Some(next) = node_owners.get(owner) {
            if !seen.insert(owner) {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::CyclicOwner {
                        node_id: node.node_id.clone(),
                    },
                    path: format!("$.nodes[{index}].owner"),
                });
            }
            owner = next;
        }
    }

    let mut references = BTreeSet::new();
    for (index, reference) in graph.references.iter().enumerate() {
        if !modules.contains(&reference.module) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::UnknownModule {
                    module: reference.module.clone(),
                },
                path: format!("$.references[{index}].module"),
            });
        }
        if !matches!(reference.source_kind.as_str(), "expression" | "pattern") {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::InvalidReferenceSourceKind {
                    kind: reference.source_kind.clone(),
                },
                path: format!("$.references[{index}].source_kind"),
            });
        }
        if !references.insert((
            reference.module.clone(),
            reference.source_kind.clone(),
            reference.reference,
        )) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DuplicateReference {
                    module: reference.module.clone(),
                    source_kind: reference.source_kind.clone(),
                    reference: reference.reference,
                },
                path: format!("$.references[{index}].reference"),
            });
        }
        if let Some(source_id) = &reference.source_id {
            validate_id(source_id, &format!("$.references[{index}].source_id"))?;
            let expected_kind = reference.source_kind.as_str();
            if nodes.get(source_id).map(String::as_str) != Some(expected_kind) {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::DanglingReference {
                        target: source_id.clone(),
                    },
                    path: format!("$.references[{index}].source_id"),
                });
            }
        }
        match reference.target_kind.as_str() {
            "definition" => {
                validate_id(&reference.target, &format!("$.references[{index}].target"))?;
                if !definitions.contains(&reference.target) {
                    return Err(SemanticReadError {
                        kind: SemanticReadErrorKind::DanglingReference {
                            target: reference.target.clone(),
                        },
                        path: format!("$.references[{index}].target"),
                    });
                }
            }
            "binding" => {
                let legacy_local = reference
                    .target
                    .strip_prefix("local:")
                    .and_then(|value| value.parse::<u32>().ok())
                    .is_some();
                let resolved_node =
                    nodes.get(&reference.target).map(String::as_str) == Some("binding");
                if !legacy_local && !resolved_node {
                    return Err(SemanticReadError {
                        kind: SemanticReadErrorKind::DanglingReference {
                            target: reference.target.clone(),
                        },
                        path: format!("$.references[{index}].target"),
                    });
                }
            }
            kind => {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::InvalidReferenceTargetKind {
                        kind: kind.to_owned(),
                    },
                    path: format!("$.references[{index}].target_kind"),
                });
            }
        }
    }
    Ok(())
}

fn validate_id(value: &str, path: &str) -> Result<(), SemanticReadError> {
    const PREFIX: &str = "experimental:blake3:";
    let digest = value.strip_prefix(PREFIX);
    if !digest.is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    }) {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::InvalidId {
                value: value.to_owned(),
            },
            path: path.to_owned(),
        });
    }
    Ok(())
}

struct SnapshotBuilder {
    checked: CheckedProgram,
}

impl SnapshotBuilder {
    const fn new(checked: CheckedProgram) -> Self {
        Self { checked }
    }

    fn build(self) -> Result<ProgramSnapshot, SnapshotError> {
        let body_ids = self.body_ids();
        let program_id = self.program_id(&body_ids);
        let modules = self.modules();
        let definitions = self.definitions(&body_ids);
        let nodes = self.nodes();
        let references = self.references();
        let graph = SemanticGraph {
            schema: SEMANTIC_SCHEMA.to_owned(),
            language_version: LANGUAGE_VERSION.to_owned(),
            unicode_version: ling_unicode::UNICODE_VERSION.to_string(),
            program_id: program_id.to_string(),
            entry_module: self
                .checked
                .typed()
                .resolved()
                .entry_module()
                .hir
                .module
                .name
                .normalized(),
            modules,
            definitions,
            nodes,
            references,
        };
        let json = serde_json::to_string(&graph).map_err(SnapshotError)?;
        Ok(ProgramSnapshot {
            checked: self.checked,
            graph,
            body_ids,
            program_id,
            json,
        })
    }

    fn body_ids(&self) -> BTreeMap<DefinitionId, BodyId> {
        self.checked
            .typed()
            .resolved()
            .definitions()
            .keys()
            .map(|definition| {
                let mut encoder = Encoder::new("ling.body-id/v1");
                encoder.string(LANGUAGE_VERSION);
                encoder.string(SEMANTIC_SCHEMA);
                self.encode_definition(definition, &mut encoder);
                (definition.clone(), BodyId(hash(encoder.finish())))
            })
            .collect()
    }

    fn program_id(&self, body_ids: &BTreeMap<DefinitionId, BodyId>) -> ProgramId {
        let mut encoder = Encoder::new("ling.program-id/v1");
        encoder.string(LANGUAGE_VERSION);
        encoder.string(SEMANTIC_SCHEMA);
        encoder.string(&ling_unicode::UNICODE_VERSION.to_string());
        encoder.u32(u32::try_from(body_ids.len()).unwrap_or(u32::MAX));
        for (definition, body) in body_ids {
            encoder.string(definition.as_str());
            encoder.string(body.as_str());
        }
        ProgramId(hash(encoder.finish()))
    }

    fn modules(&self) -> Vec<SemanticModule> {
        let resolved = self.checked.typed().resolved();
        let mut modules = resolved
            .modules()
            .iter()
            .map(|module| {
                let mut requires = module
                    .hir
                    .module
                    .requires
                    .iter()
                    .map(hir::QualifiedName::normalized)
                    .collect::<Vec<_>>();
                requires.sort();
                let mut imports = module
                    .imports
                    .iter()
                    .filter_map(|(alias, target)| {
                        resolved.module(*target).map(|target| SemanticImport {
                            alias: alias.clone(),
                            module: target.hir.module.name.normalized(),
                        })
                    })
                    .collect::<Vec<_>>();
                imports.sort_by(|left, right| {
                    (&left.alias, &left.module).cmp(&(&right.alias, &right.module))
                });
                SemanticModule {
                    name: module.hir.module.name.normalized(),
                    explicit: module.hir.module.explicit,
                    requires,
                    imports,
                }
            })
            .collect::<Vec<_>>();
        modules.sort_by(|left, right| left.name.cmp(&right.name));
        modules
    }

    fn definitions(&self, body_ids: &BTreeMap<DefinitionId, BodyId>) -> Vec<SemanticDefinition> {
        let typed = self.checked.typed();
        typed
            .resolved()
            .definitions()
            .iter()
            .map(|(id, definition)| {
                let effects = self
                    .checked
                    .definition_effect(id)
                    .map_or_else(Vec::new, ling_effects::EffectRow::names);
                let capabilities = effects
                    .iter()
                    .filter(|effect| effect.as_str() == "Console.Write")
                    .map(|_| "Console.Write".to_owned())
                    .collect();
                let type_name = typed
                    .definition_type(id)
                    .map_or_else(|| "<untyped>".to_owned(), |value| typed.display_type(value));
                SemanticDefinition {
                    definition_id: id.to_string(),
                    body_id: body_ids[id].to_string(),
                    module: definition.module_name.clone(),
                    name: definition.name.clone(),
                    kind: definition_kind(definition.kind).to_owned(),
                    origin: match definition.origin {
                        DefinitionOrigin::User { .. } => "user".to_owned(),
                        DefinitionOrigin::Builtin(_) => "builtin".to_owned(),
                        DefinitionOrigin::Prelude(_) => "prelude".to_owned(),
                    },
                    type_name,
                    effects,
                    capabilities,
                }
            })
            .collect()
    }

    fn nodes(&self) -> Vec<SemanticNode> {
        let typed = self.checked.typed();
        let resolved = typed.resolved();
        let mut nodes = Vec::new();

        for (definition_id, definition) in resolved.definitions() {
            let type_name = typed
                .definition_type(definition_id)
                .map(|type_id| typed.display_type(type_id));
            if definition.kind == DefinitionKind::Value {
                nodes.push(semantic_node(
                    "binding",
                    &definition.module_name,
                    &format!("definition:{definition_id}"),
                    Some(&definition.name),
                    definition_id.as_str(),
                    type_name.clone(),
                    Some(definition.mutable),
                    None,
                    Vec::new(),
                    Vec::new(),
                    identifier_metadata_from_definition(definition),
                ));
            }
            if let Some(function_type) = self.checked.definition_function_type(definition_id) {
                let function_id = semantic_node_id(
                    "function",
                    &definition.module_name,
                    &format!("definition:{definition_id}"),
                );
                let effects = function_type.effects().names();
                let capabilities = capabilities_for_effects(&effects);
                nodes.push(SemanticNode {
                    node_id: function_id.clone(),
                    module: definition.module_name.clone(),
                    kind: "function".to_owned(),
                    name: Some(definition.name.clone()),
                    owner: definition_id.to_string(),
                    type_name: type_name.clone(),
                    mutable: None,
                    ordinal: None,
                    effects: effects.clone(),
                    capabilities: capabilities.clone(),
                    identifier_source: Some(definition.name_source.clone()),
                    identifier_skeleton: Some(definition.name_skeleton.clone()),
                    identifier_scripts: definition.name_scripts.clone(),
                    identifier_suspicious_mixed_script: definition.name_suspicious_mixed_script,
                });
                if !matches!(definition.origin, DefinitionOrigin::User { .. }) {
                    for (ordinal, parameter) in function_type.parameters().iter().enumerate() {
                        nodes.push(semantic_node(
                            "parameter",
                            &definition.module_name,
                            &format!("definition:{definition_id}:parameter:{ordinal}"),
                            None,
                            &function_id,
                            Some(typed.display_type(*parameter)),
                            None,
                            Some(u32::try_from(ordinal).unwrap_or(u32::MAX)),
                            Vec::new(),
                            Vec::new(),
                            IdentifierMetadata::default(),
                        ));
                    }
                }
            }
            if let Some(effects) = self.checked.definition_effect(definition_id) {
                for effect in effects.names() {
                    nodes.push(semantic_node(
                        "effect",
                        &definition.module_name,
                        &format!("definition:{definition_id}:effect:{effect}"),
                        Some(&effect),
                        definition_id.as_str(),
                        None,
                        None,
                        None,
                        Vec::new(),
                        Vec::new(),
                        IdentifierMetadata::default(),
                    ));
                }
                for capability in capabilities_for_effects(&effects.names()) {
                    nodes.push(semantic_node(
                        "capability",
                        &definition.module_name,
                        &format!("definition:{definition_id}:capability:{capability}"),
                        Some(&capability),
                        definition_id.as_str(),
                        None,
                        None,
                        None,
                        Vec::new(),
                        Vec::new(),
                        IdentifierMetadata::default(),
                    ));
                }
            }
        }

        for module in resolved.modules() {
            let module_name = module.hir.module.name.normalized();
            for declaration in &module.hir.types {
                let Some(definition_id) =
                    resolved.definition_id(module.id, &declaration.name.normalized)
                else {
                    continue;
                };
                match &declaration.definition {
                    hir::TypeDefinition::Record(fields) => {
                        let record = typed.records().get(definition_id);
                        for (ordinal, field) in fields.iter().enumerate() {
                            let type_name = record.and_then(|record| {
                                record
                                    .fields
                                    .iter()
                                    .find(|info| info.name == field.name.normalized)
                                    .map(|info| typed.display_type(info.field_type))
                            });
                            nodes.push(semantic_node(
                                "field",
                                &module_name,
                                &format!("type:{definition_id}:field:{}", field.name.normalized),
                                Some(&field.name.normalized),
                                definition_id.as_str(),
                                type_name,
                                Some(field.mutable),
                                Some(u32::try_from(ordinal).unwrap_or(u32::MAX)),
                                Vec::new(),
                                Vec::new(),
                                identifier_metadata(&field.name),
                            ));
                        }
                    }
                    hir::TypeDefinition::Variant(cases) => {
                        let variant = typed.variants().get(definition_id);
                        for (ordinal, case) in cases.iter().enumerate() {
                            let type_name = variant.and_then(|variant| {
                                variant
                                    .cases
                                    .iter()
                                    .find(|info| info.name == case.name.normalized)
                                    .and_then(|info| info.payload.map(|id| typed.display_type(id)))
                            });
                            nodes.push(semantic_node(
                                "variant",
                                &module_name,
                                &format!("type:{definition_id}:variant:{}", case.name.normalized),
                                Some(&case.name.normalized),
                                definition_id.as_str(),
                                type_name,
                                None,
                                Some(u32::try_from(ordinal).unwrap_or(u32::MAX)),
                                Vec::new(),
                                Vec::new(),
                                identifier_metadata(&case.name),
                            ));
                        }
                    }
                    hir::TypeDefinition::Alias(_) => {}
                }
            }
            for definition in &module.hir.definitions {
                let Some(definition_id) =
                    resolved.definition_id(module.id, &definition.name.normalized)
                else {
                    continue;
                };
                let function_id = semantic_node_id(
                    "function",
                    &module_name,
                    &format!("definition:{definition_id}"),
                );
                for (ordinal, parameter) in definition.parameters.iter().enumerate() {
                    add_parameter_and_pattern_nodes(
                        &mut nodes,
                        typed,
                        module.id,
                        &module_name,
                        &function_id,
                        ordinal,
                        parameter,
                    );
                }
                add_expression_nodes(
                    &mut nodes,
                    &self.checked,
                    module.id,
                    &module_name,
                    definition_id.as_str(),
                    &definition.value,
                );
            }
        }
        nodes.sort_by(|left, right| left.node_id.cmp(&right.node_id));
        nodes
    }

    fn references(&self) -> Vec<SemanticReference> {
        let resolved = self.checked.typed().resolved();
        let source_ids = reference_source_ids(resolved);
        let mut references = resolved
            .references()
            .iter()
            .filter_map(|(key, target)| {
                resolved.module(key.module()).map(|module| {
                    let (target_kind, target) = match target {
                        ReferenceTarget::Definition(definition) => {
                            ("definition".to_owned(), definition.to_string())
                        }
                        ReferenceTarget::Binding(binding) => {
                            let binding_module = resolved
                                .module(binding.module())
                                .expect("resolved binding module exists")
                                .hir
                                .module
                                .name
                                .normalized();
                            (
                                "binding".to_owned(),
                                semantic_node_id(
                                    "binding",
                                    &binding_module,
                                    &format!("local:{}", binding.local().get()),
                                ),
                            )
                        }
                    };
                    SemanticReference {
                        module: module.hir.module.name.normalized(),
                        source_kind: "expression".to_owned(),
                        reference: key.local().get(),
                        source_id: source_ids.get(&(key.module(), key.local())).cloned(),
                        target_kind,
                        target,
                    }
                })
            })
            .collect::<Vec<_>>();
        references.extend(
            resolved
                .pattern_constructors()
                .iter()
                .filter_map(|(key, target)| {
                    resolved
                        .module(key.module())
                        .map(|module| SemanticReference {
                            module: module.hir.module.name.normalized(),
                            source_kind: "pattern".to_owned(),
                            reference: key.local().get(),
                            source_id: Some(semantic_node_id(
                                "pattern",
                                &module.hir.module.name.normalized(),
                                &format!("local:{}", key.local().get()),
                            )),
                            target_kind: "definition".to_owned(),
                            target: target.to_string(),
                        })
                }),
        );
        references.sort_by(|left, right| {
            (
                &left.module,
                &left.source_kind,
                left.reference,
                &left.target_kind,
                &left.target,
            )
                .cmp(&(
                    &right.module,
                    &right.source_kind,
                    right.reference,
                    &right.target_kind,
                    &right.target,
                ))
        });
        references
    }

    fn encode_definition(&self, definition: &DefinitionId, encoder: &mut Encoder) {
        let typed = self.checked.typed();
        let info = &typed.resolved().definitions()[definition];
        encoder.string(definition_kind(info.kind));
        if let Some(type_id) = typed.definition_type(definition) {
            encoder.string(&typed.arena().display(type_id));
        } else {
            encoder.string("<untyped>");
        }
        let effects = self
            .checked
            .definition_effect(definition)
            .map_or_else(Vec::new, ling_effects::EffectRow::canonical_names);
        encoder.strings(&effects);
        let capabilities = effects
            .iter()
            .filter(|effect| effect.as_str() == "Console.Write")
            .cloned()
            .collect::<Vec<_>>();
        encoder.strings(&capabilities);
        match info.origin {
            DefinitionOrigin::Builtin(builtin) => {
                encoder.u8(0);
                encoder.string(builtin.qualified_name());
            }
            DefinitionOrigin::User { module } => {
                encoder.u8(1);
                let resolved_module = typed
                    .resolved()
                    .module(module)
                    .expect("definition module exists");
                if let Some(value) = resolved_module
                    .hir
                    .definitions
                    .iter()
                    .find(|value| value.name.normalized == info.name)
                {
                    encoder.u8(0);
                    encoder.bool(value.recursive);
                    encoder.bool(value.mutable);
                    encoder.u32(u32::try_from(value.parameters.len()).unwrap_or(u32::MAX));
                    for pattern in &value.parameters {
                        self.encode_pattern(module, pattern, encoder);
                    }
                    self.encode_expression(module, &value.value, encoder);
                } else if let Some(value) = resolved_module
                    .hir
                    .types
                    .iter()
                    .find(|value| value.name.normalized == info.name)
                {
                    encoder.u8(1);
                    encode_type_declaration(value, encoder);
                } else {
                    encoder.u8(2);
                    encoder.string(&info.name);
                }
            }
            DefinitionOrigin::Prelude(definition) => {
                encoder.u8(2);
                encoder.string(PRELUDE_MODULE);
                encoder.string(definition.name());
            }
        }
    }

    fn encode_expression(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
        encoder: &mut Encoder,
    ) {
        let typed = self.checked.typed();
        if let Some(type_id) = typed.expression_type(ExpressionKey::new(module, expression.id)) {
            encoder.string(&typed.arena().display(type_id));
        } else {
            encoder.string("<untyped>");
        }
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                encoder.u8(0);
                encoder.u32(u32::try_from(elements.len()).unwrap_or(u32::MAX));
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            encoder.u8(0);
                            encoder.u32(binding.id.get());
                            encoder.bool(binding.recursive);
                            encoder.bool(binding.mutable);
                            encoder
                                .u32(u32::try_from(binding.parameters.len()).unwrap_or(u32::MAX));
                            for pattern in &binding.parameters {
                                self.encode_pattern(module, pattern, encoder);
                            }
                            self.encode_expression(module, &binding.value, encoder);
                        }
                        hir::SequenceElement::Expression(expression) => {
                            encoder.u8(1);
                            self.encode_expression(module, expression, encoder);
                        }
                    }
                }
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                encoder.u8(1);
                self.encode_expression(module, condition, encoder);
                self.encode_expression(module, then_branch, encoder);
                self.encode_expression(module, else_branch, encoder);
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                encoder.u8(2);
                self.encode_expression(module, scrutinee, encoder);
                encoder.u32(u32::try_from(cases.len()).unwrap_or(u32::MAX));
                for case in cases {
                    self.encode_pattern(module, &case.pattern, encoder);
                    encoder.bool(case.guard.is_some());
                    if let Some(guard) = &case.guard {
                        self.encode_expression(module, guard, encoder);
                    }
                    self.encode_expression(module, &case.body, encoder);
                }
            }
            hir::ExpressionKind::Assignment { place, value } => {
                encoder.u8(3);
                self.encode_reference(module, place.root_reference, encoder);
                encoder.u32(u32::try_from(place.fields.len()).unwrap_or(u32::MAX));
                for field in &place.fields {
                    encoder.string(&field.normalized);
                }
                self.encode_expression(module, value, encoder);
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                encoder.u8(4);
                self.encode_expression(module, function, encoder);
                encoder.u32(u32::try_from(arguments.len()).unwrap_or(u32::MAX));
                for argument in arguments {
                    self.encode_expression(module, argument, encoder);
                }
            }
            hir::ExpressionKind::Projection {
                reference,
                target,
                field,
            } => {
                encoder.u8(5);
                if typed.resolved().reference(module, *reference).is_some() {
                    encoder.u8(0);
                    self.encode_reference(module, *reference, encoder);
                } else {
                    encoder.u8(1);
                    self.encode_expression(module, target, encoder);
                    encoder.string(&field.normalized);
                }
            }
            hir::ExpressionKind::Name { reference, .. } => {
                encoder.u8(6);
                self.encode_reference(module, *reference, encoder);
            }
            hir::ExpressionKind::Binary {
                operator,
                left,
                right,
            } => {
                encoder.u8(7);
                encoder.u8(binary_tag(*operator));
                self.encode_expression(module, left, encoder);
                self.encode_expression(module, right, encoder);
            }
            hir::ExpressionKind::Unary { operator, operand } => {
                encoder.u8(8);
                encoder.u8(match operator {
                    hir::UnaryOperator::Positive => 0,
                    hir::UnaryOperator::Negative => 1,
                });
                self.encode_expression(module, operand, encoder);
            }
            hir::ExpressionKind::Literal(literal) => {
                encoder.u8(9);
                match literal {
                    hir::Literal::Integer { .. } => {
                        encoder.u8(0);
                        let bytes = typed
                            .integer(ExpressionKey::new(module, expression.id))
                            .map(num_bigint_bytes)
                            .unwrap_or_default();
                        encoder.bytes(&bytes);
                    }
                    hir::Literal::Float(value) => {
                        encoder.u8(1);
                        encoder.bytes(
                            &value
                                .parse::<f64>()
                                .unwrap_or(f64::NAN)
                                .to_bits()
                                .to_be_bytes(),
                        );
                    }
                    hir::Literal::Text(value) => {
                        encoder.u8(2);
                        encoder.string(value);
                    }
                    hir::Literal::Boolean(value) => {
                        encoder.u8(3);
                        encoder.bool(*value);
                    }
                }
            }
            hir::ExpressionKind::Unit => encoder.u8(10),
            hir::ExpressionKind::Tuple(elements) => {
                encoder.u8(11);
                encoder.u32(u32::try_from(elements.len()).unwrap_or(u32::MAX));
                for element in elements {
                    self.encode_expression(module, element, encoder);
                }
            }
            hir::ExpressionKind::Record(fields) => {
                encoder.u8(12);
                self.encode_record_fields(module, fields, encoder);
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                encoder.u8(13);
                self.encode_expression(module, base, encoder);
                self.encode_record_fields(module, fields, encoder);
            }
            hir::ExpressionKind::List(elements) => {
                encoder.u8(14);
                encoder.u32(u32::try_from(elements.len()).unwrap_or(u32::MAX));
                for element in elements {
                    self.encode_expression(module, element, encoder);
                }
            }
        }
    }

    fn encode_record_fields(
        &self,
        module: ModuleId,
        fields: &[hir::RecordField],
        encoder: &mut Encoder,
    ) {
        encoder.u32(u32::try_from(fields.len()).unwrap_or(u32::MAX));
        for field in fields {
            encoder.string(&field.name.normalized);
            self.encode_expression(module, &field.value, encoder);
        }
    }

    fn encode_reference(
        &self,
        module: ModuleId,
        reference: hir::ReferenceId,
        encoder: &mut Encoder,
    ) {
        match self.checked.typed().resolved().reference(module, reference) {
            Some(ReferenceTarget::Definition(definition)) => {
                encoder.u8(0);
                encoder.string(definition.as_str());
            }
            Some(ReferenceTarget::Binding(binding)) => {
                encoder.u8(1);
                encoder.u32(binding.local().get());
            }
            None => encoder.u8(u8::MAX),
        }
    }

    fn encode_pattern(&self, module: ModuleId, pattern: &hir::Pattern, encoder: &mut Encoder) {
        let resolved = self.checked.typed().resolved();
        match &pattern.kind {
            hir::PatternKind::Binding { id, .. } => {
                if let Some(definition) = resolved.pattern_constructor(module, pattern.id) {
                    encoder.u8(4);
                    encoder.string(definition.as_str());
                    encoder.u32(0);
                } else {
                    encoder.u8(0);
                    encoder.u32(id.get());
                }
            }
            hir::PatternKind::Wildcard => encoder.u8(5),
            hir::PatternKind::Unit => encoder.u8(1),
            hir::PatternKind::Literal(literal) => {
                encoder.u8(2);
                encode_literal_without_context(literal, encoder);
            }
            hir::PatternKind::Tuple(elements) => {
                encoder.u8(3);
                encoder.u32(u32::try_from(elements.len()).unwrap_or(u32::MAX));
                for element in elements {
                    self.encode_pattern(module, element, encoder);
                }
            }
            hir::PatternKind::Record(fields) => {
                encoder.u8(6);
                encoder.u32(u32::try_from(fields.len()).unwrap_or(u32::MAX));
                for field in fields {
                    encoder.string(&field.name.normalized);
                    self.encode_pattern(module, &field.pattern, encoder);
                }
            }
            hir::PatternKind::Constructor {
                qualifier,
                name,
                arguments,
            } => {
                encoder.u8(4);
                if let Some(definition) = resolved.pattern_constructor(module, pattern.id) {
                    encoder.string(definition.as_str());
                } else {
                    encoder.string(&qualifier.as_ref().map_or_else(
                        || name.normalized.clone(),
                        |qualifier| format!("{}.{}", qualifier.normalized, name.normalized),
                    ));
                }
                encoder.u32(u32::try_from(arguments.len()).unwrap_or(u32::MAX));
                for argument in arguments {
                    self.encode_pattern(module, argument, encoder);
                }
            }
        }
    }
}

#[derive(Default)]
struct IdentifierMetadata {
    source: Option<String>,
    skeleton: Option<String>,
    scripts: Vec<String>,
    suspicious_mixed_script: bool,
}

fn identifier_metadata(name: &hir::Name) -> IdentifierMetadata {
    IdentifierMetadata {
        source: Some(name.source.clone()),
        skeleton: Some(name.skeleton.clone()),
        scripts: name.scripts.clone(),
        suspicious_mixed_script: name.suspicious_mixed_script,
    }
}

fn identifier_metadata_from_definition(
    definition: &ling_resolve::DefinitionInfo,
) -> IdentifierMetadata {
    IdentifierMetadata {
        source: Some(definition.name_source.clone()),
        skeleton: Some(definition.name_skeleton.clone()),
        scripts: definition.name_scripts.clone(),
        suspicious_mixed_script: definition.name_suspicious_mixed_script,
    }
}

#[allow(clippy::too_many_arguments)]
fn semantic_node(
    kind: &str,
    module: &str,
    identity: &str,
    name: Option<&str>,
    owner: &str,
    type_name: Option<String>,
    mutable: Option<bool>,
    ordinal: Option<u32>,
    effects: Vec<String>,
    capabilities: Vec<String>,
    metadata: IdentifierMetadata,
) -> SemanticNode {
    SemanticNode {
        node_id: semantic_node_id(kind, module, identity),
        module: module.to_owned(),
        kind: kind.to_owned(),
        name: name.map(str::to_owned),
        owner: owner.to_owned(),
        type_name,
        mutable,
        ordinal,
        effects,
        capabilities,
        identifier_source: metadata.source,
        identifier_skeleton: metadata.skeleton,
        identifier_scripts: metadata.scripts,
        identifier_suspicious_mixed_script: metadata.suspicious_mixed_script,
    }
}

fn semantic_node_id(kind: &str, module: &str, identity: &str) -> String {
    let mut encoder = Encoder::new("ling.semantic-node-id/v1");
    encoder.string(LANGUAGE_VERSION);
    encoder.string(SEMANTIC_SCHEMA);
    encoder.string(kind);
    encoder.string(module);
    encoder.string(identity);
    hash(encoder.finish())
}

fn capabilities_for_effects(effects: &[String]) -> Vec<String> {
    effects
        .iter()
        .filter(|effect| effect.as_str() == "Console.Write")
        .map(|_| "Console.Write".to_owned())
        .collect()
}

fn add_parameter_and_pattern_nodes(
    nodes: &mut Vec<SemanticNode>,
    typed: &ling_types::TypedProgram,
    module: ModuleId,
    module_name: &str,
    function_id: &str,
    ordinal: usize,
    pattern: &hir::Pattern,
) {
    let name = pattern_binding_name(typed.resolved(), module, pattern);
    let type_name = pattern_binding_type(typed, module, pattern);
    let parameter_id = semantic_node_id(
        "parameter",
        module_name,
        &format!("local:{}", pattern.id.get()),
    );
    nodes.push(semantic_node(
        "parameter",
        module_name,
        &format!("local:{}", pattern.id.get()),
        name.map(|name| name.normalized.as_str()),
        function_id,
        type_name,
        None,
        Some(u32::try_from(ordinal).unwrap_or(u32::MAX)),
        Vec::new(),
        Vec::new(),
        name.map_or_else(IdentifierMetadata::default, identifier_metadata),
    ));
    add_pattern_nodes(nodes, typed, module, module_name, &parameter_id, pattern);
}

fn add_pattern_nodes(
    nodes: &mut Vec<SemanticNode>,
    typed: &ling_types::TypedProgram,
    module: ModuleId,
    module_name: &str,
    owner: &str,
    pattern: &hir::Pattern,
) {
    let resolved = typed.resolved();
    let name = pattern_name(pattern);
    let pattern_id = semantic_node_id(
        "pattern",
        module_name,
        &format!("local:{}", pattern.id.get()),
    );
    nodes.push(semantic_node(
        "pattern",
        module_name,
        &format!("local:{}", pattern.id.get()),
        name.map(|name| name.normalized.as_str()),
        owner,
        pattern_binding_type(typed, module, pattern),
        None,
        Some(pattern.id.get()),
        Vec::new(),
        Vec::new(),
        name.map_or_else(IdentifierMetadata::default, identifier_metadata),
    ));
    if let hir::PatternKind::Binding { id, name } = &pattern.kind {
        if resolved.pattern_constructor(module, pattern.id).is_none() {
            let key = ling_resolve::BindingKey::new(module, *id);
            let info = resolved.bindings().get(&key);
            nodes.push(semantic_node(
                "binding",
                module_name,
                &format!("local:{}", id.get()),
                Some(&name.normalized),
                &pattern_id,
                typed.binding_type(key).map(|id| typed.display_type(id)),
                info.map(|info| info.mutable),
                Some(id.get()),
                Vec::new(),
                Vec::new(),
                identifier_metadata(name),
            ));
        }
    }
    match &pattern.kind {
        hir::PatternKind::Tuple(patterns) => {
            for pattern in patterns {
                add_pattern_nodes(nodes, typed, module, module_name, &pattern_id, pattern);
            }
        }
        hir::PatternKind::Record(fields) => {
            for field in fields {
                add_pattern_nodes(
                    nodes,
                    typed,
                    module,
                    module_name,
                    &pattern_id,
                    &field.pattern,
                );
            }
        }
        hir::PatternKind::Constructor { arguments, .. } => {
            for argument in arguments {
                add_pattern_nodes(nodes, typed, module, module_name, &pattern_id, argument);
            }
        }
        hir::PatternKind::Binding { .. }
        | hir::PatternKind::Wildcard
        | hir::PatternKind::Unit
        | hir::PatternKind::Literal(_) => {}
    }
}

fn pattern_name(pattern: &hir::Pattern) -> Option<&hir::Name> {
    match &pattern.kind {
        hir::PatternKind::Binding { name, .. } | hir::PatternKind::Constructor { name, .. } => {
            Some(name)
        }
        _ => None,
    }
}

fn pattern_binding_name<'pattern>(
    resolved: &ling_resolve::ResolvedProgram,
    module: ModuleId,
    pattern: &'pattern hir::Pattern,
) -> Option<&'pattern hir::Name> {
    match &pattern.kind {
        hir::PatternKind::Binding { name, .. }
            if resolved.pattern_constructor(module, pattern.id).is_none() =>
        {
            Some(name)
        }
        _ => None,
    }
}

fn pattern_binding_type(
    typed: &ling_types::TypedProgram,
    module: ModuleId,
    pattern: &hir::Pattern,
) -> Option<String> {
    let hir::PatternKind::Binding { id, .. } = pattern.kind else {
        return None;
    };
    typed
        .binding_type(ling_resolve::BindingKey::new(module, id))
        .map(|id| typed.display_type(id))
}

fn add_expression_nodes(
    nodes: &mut Vec<SemanticNode>,
    checked: &ling_effects::CheckedProgram,
    module: ModuleId,
    module_name: &str,
    owner: &str,
    expression: &hir::Expression,
) {
    let typed = checked.typed();
    let expression_id = semantic_node_id(
        "expression",
        module_name,
        &format!("local:{}", expression.id.get()),
    );
    nodes.push(semantic_node(
        "expression",
        module_name,
        &format!("local:{}", expression.id.get()),
        Some(expression_kind(&expression.kind)),
        owner,
        typed
            .expression_type(ExpressionKey::new(module, expression.id))
            .map(|id| typed.display_type(id)),
        None,
        Some(expression.id.get()),
        Vec::new(),
        Vec::new(),
        IdentifierMetadata::default(),
    ));
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => {
                        let key = ling_resolve::BindingKey::new(module, binding.id);
                        let binding_id = semantic_node_id(
                            "binding",
                            module_name,
                            &format!("local:{}", binding.id.get()),
                        );
                        nodes.push(semantic_node(
                            "binding",
                            module_name,
                            &format!("local:{}", binding.id.get()),
                            Some(&binding.name.normalized),
                            owner,
                            typed.binding_type(key).map(|id| typed.display_type(id)),
                            Some(binding.mutable),
                            Some(binding.id.get()),
                            Vec::new(),
                            Vec::new(),
                            identifier_metadata(&binding.name),
                        ));
                        let parameter_owner = if let Some(function_type) =
                            checked.binding_function_type(key)
                        {
                            let function_id = semantic_node_id(
                                "function",
                                module_name,
                                &format!("local:{}", binding.id.get()),
                            );
                            let effects = function_type.effects().names();
                            let capabilities = capabilities_for_effects(&effects);
                            nodes.push(semantic_node(
                                "function",
                                module_name,
                                &format!("local:{}", binding.id.get()),
                                Some(&binding.name.normalized),
                                &binding_id,
                                typed.binding_type(key).map(|id| typed.display_type(id)),
                                None,
                                Some(binding.id.get()),
                                effects.clone(),
                                capabilities.clone(),
                                identifier_metadata(&binding.name),
                            ));
                            for effect in effects {
                                nodes.push(semantic_node(
                                    "effect",
                                    module_name,
                                    &format!("local:{}:effect:{effect}", binding.id.get()),
                                    Some(&effect),
                                    &function_id,
                                    None,
                                    None,
                                    None,
                                    Vec::new(),
                                    Vec::new(),
                                    IdentifierMetadata::default(),
                                ));
                            }
                            for capability in capabilities {
                                nodes.push(semantic_node(
                                    "capability",
                                    module_name,
                                    &format!("local:{}:capability:{capability}", binding.id.get()),
                                    Some(&capability),
                                    &function_id,
                                    None,
                                    None,
                                    None,
                                    Vec::new(),
                                    Vec::new(),
                                    IdentifierMetadata::default(),
                                ));
                            }
                            function_id
                        } else {
                            binding_id
                        };
                        for (ordinal, parameter) in binding.parameters.iter().enumerate() {
                            add_parameter_and_pattern_nodes(
                                nodes,
                                typed,
                                module,
                                module_name,
                                &parameter_owner,
                                ordinal,
                                parameter,
                            );
                        }
                        add_expression_nodes(
                            nodes,
                            checked,
                            module,
                            module_name,
                            &parameter_owner,
                            &binding.value,
                        );
                    }
                    hir::SequenceElement::Expression(expression) => {
                        add_expression_nodes(nodes, checked, module, module_name, owner, expression)
                    }
                }
            }
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            for expression in [
                condition.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ] {
                add_expression_nodes(nodes, checked, module, module_name, owner, expression);
            }
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            add_expression_nodes(nodes, checked, module, module_name, owner, scrutinee);
            for case in cases {
                add_pattern_nodes(
                    nodes,
                    typed,
                    module,
                    module_name,
                    &expression_id,
                    &case.pattern,
                );
                if let Some(guard) = &case.guard {
                    add_expression_nodes(nodes, checked, module, module_name, owner, guard);
                }
                add_expression_nodes(nodes, checked, module, module_name, owner, &case.body);
            }
        }
        hir::ExpressionKind::Assignment { value, .. } => {
            add_expression_nodes(nodes, checked, module, module_name, owner, value);
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            add_expression_nodes(nodes, checked, module, module_name, owner, function);
            for argument in arguments {
                add_expression_nodes(nodes, checked, module, module_name, owner, argument);
            }
        }
        hir::ExpressionKind::Projection { target, .. } => {
            add_expression_nodes(nodes, checked, module, module_name, owner, target);
        }
        hir::ExpressionKind::Binary { left, right, .. } => {
            add_expression_nodes(nodes, checked, module, module_name, owner, left);
            add_expression_nodes(nodes, checked, module, module_name, owner, right);
        }
        hir::ExpressionKind::Unary { operand, .. } => {
            add_expression_nodes(nodes, checked, module, module_name, owner, operand);
        }
        hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
            for element in elements {
                add_expression_nodes(nodes, checked, module, module_name, owner, element);
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                add_expression_nodes(nodes, checked, module, module_name, owner, &field.value);
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            add_expression_nodes(nodes, checked, module, module_name, owner, base);
            for field in fields {
                add_expression_nodes(nodes, checked, module, module_name, owner, &field.value);
            }
        }
        hir::ExpressionKind::Name { .. }
        | hir::ExpressionKind::Literal(_)
        | hir::ExpressionKind::Unit => {}
    }
}

fn expression_kind(expression: &hir::ExpressionKind) -> &'static str {
    match expression {
        hir::ExpressionKind::Sequence(_) => "sequence",
        hir::ExpressionKind::If { .. } => "if",
        hir::ExpressionKind::Match { .. } => "match",
        hir::ExpressionKind::Assignment { .. } => "assignment",
        hir::ExpressionKind::Application { .. } => "application",
        hir::ExpressionKind::Projection { .. } => "projection",
        hir::ExpressionKind::Name { .. } => "name",
        hir::ExpressionKind::Binary { .. } => "binary",
        hir::ExpressionKind::Unary { .. } => "unary",
        hir::ExpressionKind::Literal(_) => "literal",
        hir::ExpressionKind::Unit => "unit",
        hir::ExpressionKind::Tuple(_) => "tuple",
        hir::ExpressionKind::Record(_) => "record",
        hir::ExpressionKind::RecordUpdate { .. } => "record_update",
        hir::ExpressionKind::List(_) => "list",
    }
}

fn reference_source_ids(
    resolved: &ling_resolve::ResolvedProgram,
) -> BTreeMap<(ModuleId, hir::ReferenceId), String> {
    let mut sources = BTreeMap::new();
    for module in resolved.modules() {
        let module_name = module.hir.module.name.normalized();
        for definition in &module.hir.definitions {
            collect_expression_reference_sources(
                &module_name,
                module.id,
                &definition.value,
                &mut sources,
            );
        }
    }
    sources
}

fn collect_expression_reference_sources(
    module_name: &str,
    module: ModuleId,
    expression: &hir::Expression,
    sources: &mut BTreeMap<(ModuleId, hir::ReferenceId), String>,
) {
    let source_id = semantic_node_id(
        "expression",
        module_name,
        &format!("local:{}", expression.id.get()),
    );
    match &expression.kind {
        hir::ExpressionKind::Assignment { place, value } => {
            sources.insert((module, place.root_reference), source_id);
            collect_expression_reference_sources(module_name, module, value, sources);
        }
        hir::ExpressionKind::Projection {
            reference, target, ..
        } => {
            sources.insert((module, *reference), source_id);
            collect_expression_reference_sources(module_name, module, target, sources);
        }
        hir::ExpressionKind::Name { reference, .. } => {
            sources.insert((module, *reference), source_id);
        }
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                let expression = match element {
                    hir::SequenceElement::Let(binding) => &binding.value,
                    hir::SequenceElement::Expression(expression) => expression,
                };
                collect_expression_reference_sources(module_name, module, expression, sources);
            }
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            for expression in [
                condition.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ] {
                collect_expression_reference_sources(module_name, module, expression, sources);
            }
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            collect_expression_reference_sources(module_name, module, scrutinee, sources);
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_expression_reference_sources(module_name, module, guard, sources);
                }
                collect_expression_reference_sources(module_name, module, &case.body, sources);
            }
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            collect_expression_reference_sources(module_name, module, function, sources);
            for argument in arguments {
                collect_expression_reference_sources(module_name, module, argument, sources);
            }
        }
        hir::ExpressionKind::Binary { left, right, .. } => {
            collect_expression_reference_sources(module_name, module, left, sources);
            collect_expression_reference_sources(module_name, module, right, sources);
        }
        hir::ExpressionKind::Unary { operand, .. } => {
            collect_expression_reference_sources(module_name, module, operand, sources);
        }
        hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
            for element in elements {
                collect_expression_reference_sources(module_name, module, element, sources);
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                collect_expression_reference_sources(module_name, module, &field.value, sources);
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            collect_expression_reference_sources(module_name, module, base, sources);
            for field in fields {
                collect_expression_reference_sources(module_name, module, &field.value, sources);
            }
        }
        hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
    }
}

fn definition_kind(kind: DefinitionKind) -> &'static str {
    match kind {
        DefinitionKind::Value => "value",
        DefinitionKind::Type => "type",
        DefinitionKind::Constructor => "constructor",
        DefinitionKind::Builtin => "builtin",
    }
}

fn encode_literal_without_context(literal: &hir::Literal, encoder: &mut Encoder) {
    match literal {
        hir::Literal::Integer { radix, digits } => {
            encoder.u8(0);
            encoder.u32(*radix);
            encoder.string(&digits.replace('_', ""));
        }
        hir::Literal::Float(value) => {
            encoder.u8(1);
            encoder.bytes(
                &value
                    .parse::<f64>()
                    .unwrap_or(f64::NAN)
                    .to_bits()
                    .to_be_bytes(),
            );
        }
        hir::Literal::Text(value) => {
            encoder.u8(2);
            encoder.string(value);
        }
        hir::Literal::Boolean(value) => {
            encoder.u8(3);
            encoder.bool(*value);
        }
    }
}

fn encode_type_declaration(declaration: &hir::TypeDeclaration, encoder: &mut Encoder) {
    encoder.u32(u32::try_from(declaration.parameters.len()).unwrap_or(u32::MAX));
    let mut variables = declaration
        .parameters
        .iter()
        .enumerate()
        .map(|(index, parameter)| {
            (
                parameter.normalized.clone(),
                u32::try_from(index).unwrap_or(u32::MAX),
            )
        })
        .collect::<BTreeMap<_, _>>();
    match &declaration.definition {
        hir::TypeDefinition::Record(fields) => {
            encoder.u8(0);
            encoder.u32(u32::try_from(fields.len()).unwrap_or(u32::MAX));
            for field in fields {
                encoder.string(&field.name.normalized);
                encoder.bool(field.mutable);
                encode_type_syntax(&field.field_type, &mut variables, encoder);
            }
        }
        hir::TypeDefinition::Variant(cases) => {
            encoder.u8(1);
            encoder.u32(u32::try_from(cases.len()).unwrap_or(u32::MAX));
            for case in cases {
                encoder.string(&case.name.normalized);
                encoder.bool(case.payload.is_some());
                if let Some(payload) = &case.payload {
                    encode_type_syntax(payload, &mut variables, encoder);
                }
            }
        }
        hir::TypeDefinition::Alias(alias) => {
            encoder.u8(2);
            encode_type_syntax(alias, &mut variables, encoder);
        }
    }
}

fn encode_type_syntax(
    syntax: &hir::TypeSyntax,
    variables: &mut BTreeMap<String, u32>,
    encoder: &mut Encoder,
) {
    encoder.u32(u32::try_from(syntax.atoms.len()).unwrap_or(u32::MAX));
    for atom in &syntax.atoms {
        match atom {
            hir::TypeAtom::Name(name) => {
                encoder.u8(0);
                encoder.string(&name.normalized);
            }
            hir::TypeAtom::Variable(name) => {
                encoder.u8(1);
                let next = u32::try_from(variables.len()).unwrap_or(u32::MAX);
                let variable = *variables.entry(name.normalized.clone()).or_insert(next);
                encoder.u32(variable);
            }
            hir::TypeAtom::Arrow => encoder.u8(2),
            hir::TypeAtom::Product => encoder.u8(3),
            hir::TypeAtom::LeftParen => encoder.u8(4),
            hir::TypeAtom::RightParen => encoder.u8(5),
            hir::TypeAtom::LeftAngle => encoder.u8(6),
            hir::TypeAtom::RightAngle => encoder.u8(7),
            hir::TypeAtom::Comma => encoder.u8(8),
            hir::TypeAtom::Dot => encoder.u8(9),
        }
    }
}

const fn binary_tag(operator: hir::BinaryOperator) -> u8 {
    match operator {
        hir::BinaryOperator::Equal => 0,
        hir::BinaryOperator::NotEqual => 1,
        hir::BinaryOperator::Less => 2,
        hir::BinaryOperator::LessEqual => 3,
        hir::BinaryOperator::Greater => 4,
        hir::BinaryOperator::GreaterEqual => 5,
        hir::BinaryOperator::Add => 6,
        hir::BinaryOperator::Subtract => 7,
        hir::BinaryOperator::Multiply => 8,
        hir::BinaryOperator::Divide => 9,
        hir::BinaryOperator::Remainder => 10,
    }
}

fn num_bigint_bytes(value: &num_bigint::BigInt) -> Vec<u8> {
    let (sign, bytes) = value.to_bytes_be();
    let mut output = vec![match sign {
        num_bigint::Sign::Minus => 0,
        num_bigint::Sign::NoSign => 1,
        num_bigint::Sign::Plus => 2,
    }];
    output.extend_from_slice(&bytes);
    output
}

fn hash(bytes: Vec<u8>) -> String {
    format!("experimental:blake3:{}", blake3::hash(&bytes).to_hex())
}

struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    fn new(domain: &str) -> Self {
        let mut encoder = Self { bytes: Vec::new() };
        encoder.string(domain);
        encoder
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }

    fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn bool(&mut self, value: bool) {
        self.u8(u8::from(value));
    }

    fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn bytes(&mut self, value: &[u8]) {
        self.u32(u32::try_from(value.len()).unwrap_or(u32::MAX));
        self.bytes.extend_from_slice(value);
    }

    fn string(&mut self, value: &str) {
        self.bytes(value.as_bytes());
    }

    fn strings(&mut self, values: &[String]) {
        self.u32(u32::try_from(values.len()).unwrap_or(u32::MAX));
        for value in values {
            self.string(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;

    fn snapshot(text: &str) -> ProgramSnapshot {
        snapshot_named("test.ling", text)
    }

    fn snapshot_named(source_name: &str, text: &str) -> ProgramSnapshot {
        let source =
            SourceFile::from_bytes(SourceId::new(0), source_name, text.as_bytes().to_vec())
                .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = hir::lower(source.name(), &ast).expect("valid HIR");
        let resolved = ling_resolve::resolve(vec![hir], "Main").expect("resolves");
        let typed = ling_types::check(resolved).expect("type-checks");
        let checked = ling_effects::check(typed).expect("effects check");
        build(checked).expect("snapshot builds")
    }

    #[test]
    fn hello_snapshot_is_byte_deterministic() {
        let source =
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"你好，零\"\n";
        let first = snapshot(source);
        let second = snapshot(source);
        assert_eq!(first.json(), second.json());
        assert_eq!(first.program_id(), second.program_id());
        assert!(first.json().contains("\"schema\":\"ling.semantic/0.1\""));
    }

    #[test]
    fn constructor_pattern_references_are_resolved_graph_edges() {
        let snapshot = snapshot(concat!(
            "module Main\n\n",
            "type State =\n",
            "    | Healthy\n",
            "    | Hurt of Int\n\n",
            "let describe state =\n",
            "    match state with\n",
            "    | Healthy -> 0\n",
            "    | Hurt amount -> amount\n",
        ));
        let constructor_ids = snapshot
            .graph()
            .definitions
            .iter()
            .filter(|definition| definition.kind == "constructor")
            .map(|definition| definition.definition_id.as_str())
            .collect::<BTreeSet<_>>();
        let pattern_references = snapshot
            .graph()
            .references
            .iter()
            .filter(|reference| reference.source_kind == "pattern")
            .collect::<Vec<_>>();

        assert_eq!(pattern_references.len(), 2);
        assert!(pattern_references.iter().all(|reference| {
            reference.target_kind == "definition"
                && constructor_ids.contains(reference.target.as_str())
        }));
    }

    #[test]
    fn local_function_effects_and_ownership_are_explicit() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    let write value = Console.write value\n",
            "    write \"hello\"\n",
        ));
        let graph = snapshot.graph();
        let function = graph
            .nodes
            .iter()
            .find(|node| node.kind == "function" && node.name.as_deref() == Some("write"))
            .expect("local function node");
        assert_eq!(function.effects, ["Console.Write"]);
        assert_eq!(function.capabilities, ["Console.Write"]);
        assert!(graph.nodes.iter().any(|node| {
            node.kind == "binding"
                && node.name.as_deref() == Some("write")
                && node.node_id == function.owner
        }));
        assert!(graph.nodes.iter().any(|node| {
            node.kind == "effect"
                && node.name.as_deref() == Some("Console.Write")
                && node.owner == function.node_id
        }));
        assert!(
            graph
                .nodes
                .iter()
                .any(|node| node.kind == "expression" && node.owner == function.node_id)
        );
    }

    #[test]
    fn semantic_graph_covers_every_required_node_kind_with_resolved_edges() {
        let snapshot = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "type 人物 =\n",
            "    { 姓名: Text\n",
            "      mutable 血量: Int\n",
            "      最大血量: Int }\n\n",
            "type 生存状态 =\n",
            "    | 健康\n",
            "    | 受伤 of Int\n",
            "    | 死亡\n\n",
            "let 描述 状态 人物 =\n",
            "    match 状态 with\n",
            "    | 健康 -> 人物.姓名\n",
            "    | 受伤 血量 -> Text.format \"受伤 {}\" 血量\n",
            "    | 死亡 -> \"死亡\"\n\n",
            "let main () =\n",
            "    let mutable 关羽 =\n",
            "        { 姓名 = \"关羽\"\n",
            "          血量 = 30\n",
            "          最大血量 = 100 }\n",
            "    关羽.血量 <- 20\n",
            "    Console.write (描述 (受伤 关羽.血量) 关羽)\n",
        ));
        let graph = snapshot.graph();

        let mut kinds = graph
            .nodes
            .iter()
            .map(|node| node.kind.as_str())
            .collect::<BTreeSet<_>>();
        kinds.extend(
            graph
                .definitions
                .iter()
                .map(|definition| definition.kind.as_str()),
        );
        if !graph.modules.is_empty() {
            kinds.insert("module");
        }
        for required in [
            "module",
            "type",
            "field",
            "variant",
            "binding",
            "function",
            "parameter",
            "pattern",
            "expression",
            "effect",
            "capability",
        ] {
            assert!(
                kinds.contains(required),
                "missing Semantic Graph kind `{required}`"
            );
        }

        let definitions = graph
            .definitions
            .iter()
            .map(|definition| definition.definition_id.as_str())
            .collect::<BTreeSet<_>>();
        let nodes = graph
            .nodes
            .iter()
            .map(|node| (node.node_id.as_str(), node.kind.as_str()))
            .collect::<BTreeMap<_, _>>();
        for node in &graph.nodes {
            assert!(
                definitions.contains(node.owner.as_str())
                    || nodes.contains_key(node.owner.as_str()),
                "node owner must resolve: {}",
                node.owner
            );
        }
        for reference in &graph.references {
            let source_id = reference
                .source_id
                .as_deref()
                .expect("writer emits source IDs");
            assert_eq!(nodes.get(source_id), Some(&reference.source_kind.as_str()));
            match reference.target_kind.as_str() {
                "definition" => assert!(definitions.contains(reference.target.as_str())),
                "binding" => {
                    assert_eq!(nodes.get(reference.target.as_str()), Some(&"binding"));
                    assert!(!reference.target.starts_with("local:"));
                }
                kind => panic!("unexpected reference target kind `{kind}`"),
            }
        }

        assert!(
            graph
                .definitions
                .iter()
                .any(|definition| definition.name == "人物")
        );
        assert!(graph.nodes.iter().any(|node| {
            node.kind == "field"
                && node.name.as_deref() == Some("血量")
                && node.identifier_source.as_deref() == Some("血量")
                && node.mutable == Some(true)
        }));
        assert!(graph.nodes.iter().any(|node| {
            node.kind == "variant"
                && node.name.as_deref() == Some("受伤")
                && node.identifier_source.as_deref() == Some("受伤")
        }));
        let main = graph
            .definitions
            .iter()
            .find(|definition| definition.name == "main")
            .expect("main definition");
        assert_eq!(
            main.effects,
            ["Console.Write".to_owned(), "State<人物>".to_owned()]
        );
        assert!(graph.nodes.iter().any(|node| {
            node.kind == "effect"
                && node.name.as_deref() == Some("State<人物>")
                && node.owner == main.definition_id
        }));
        assert!(
            graph
                .definitions
                .iter()
                .all(|definition| !definition.type_name.contains("experimental:blake3:"))
        );
        assert!(graph.nodes.iter().all(|node| {
            node.type_name
                .as_deref()
                .is_none_or(|type_name| !type_name.contains("experimental:blake3:"))
        }));
        assert!(
            snapshot
                .audit_model()
                .modules
                .iter()
                .flat_map(|module| &module.definitions)
                .any(|definition| {
                    definition.name == "main"
                        && definition.effects.contains(&"State<人物>".to_owned())
                })
        );
        assert_eq!(
            read_json(snapshot.json()).expect("complete graph validates"),
            *graph
        );
    }

    #[test]
    fn prelude_definitions_have_stable_provenance_and_references() {
        let first = snapshot(concat!(
            "module Main\n\n",
            "let value: Option<Int> = Some 1\n",
            "let unwrap option =\n",
            "    match option with\n",
            "    | Some payload -> payload\n",
            "    | None -> 0\n",
        ));
        let second = snapshot(concat!(
            "module Main\n\n",
            "let value: Option<Int> = Some 1\n",
            "let unwrap renamed =\n",
            "    match renamed with\n",
            "    | Some item -> item\n",
            "    | None -> 0\n",
        ));
        let prelude = first
            .graph()
            .definitions
            .iter()
            .filter(|definition| definition.origin == "prelude")
            .collect::<Vec<_>>();

        assert_eq!(prelude.len(), 6);
        assert!(prelude.iter().all(|definition| {
            definition.module == PRELUDE_MODULE
                && definition.definition_id.starts_with("experimental:blake3:")
        }));
        let prelude_ids = prelude
            .iter()
            .map(|definition| definition.definition_id.as_str())
            .collect::<BTreeSet<_>>();
        assert!(first.graph().references.iter().any(|reference| {
            reference.target_kind == "definition" && prelude_ids.contains(reference.target.as_str())
        }));
        let first_ids = prelude
            .iter()
            .map(|definition| (&definition.name, &definition.definition_id))
            .collect::<BTreeMap<_, _>>();
        let second_ids = second
            .graph()
            .definitions
            .iter()
            .filter(|definition| definition.origin == "prelude")
            .map(|definition| (&definition.name, &definition.definition_id))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(first_ids, second_ids);
    }

    #[test]
    fn whitespace_does_not_change_body_or_program_ids() {
        let compact = snapshot(
            "module Main\n    requires Console.Write\nlet main () = Console.write \"x\"\n",
        );
        let spaced = snapshot(
            "module Main\n    requires Console.Write\n\n\nlet main () =\n    Console.write \"x\"\n",
        );
        assert_eq!(compact.program_id(), spaced.program_id());

        let changed = snapshot(
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"y\"\n",
        );
        assert_ne!(compact.program_id(), changed.program_id());
        let compact_main = compact
            .graph()
            .definitions
            .iter()
            .find(|definition| definition.name == "main")
            .expect("compact main definition");
        let changed_main = changed
            .graph()
            .definitions
            .iter()
            .find(|definition| definition.name == "main")
            .expect("changed main definition");
        assert_ne!(compact_main.body_id, changed_main.body_id);
    }

    #[test]
    fn local_alpha_renames_and_type_parameter_renames_preserve_ids() {
        let local_left = snapshot("module Main\n\nlet identity value = value\n");
        let local_right = snapshot("module Main\n\nlet identity renamed = renamed\n");
        assert_eq!(local_left.program_id(), local_right.program_id());

        let type_left = snapshot(concat!(
            "module Main\n\n",
            "type Box<'a> = { value: 'a }\n",
            "let boxed: Box<Int> = { value = 1 }\n",
        ));
        let type_right = snapshot(concat!(
            "module Main\n\n",
            "type Box<'renamed> = { value: 'renamed }\n",
            "let boxed: Box<Int> = { value = 1 }\n",
        ));
        assert_eq!(type_left.program_id(), type_right.program_id());

        let repeated = snapshot("module Main\n\ntype Pair<'a, 'b> = 'a * 'a\n");
        let distinct = snapshot("module Main\n\ntype Pair<'a, 'b> = 'a * 'b\n");
        assert_ne!(repeated.program_id(), distinct.program_id());
    }

    #[test]
    fn semantic_ids_cover_canonical_inputs_and_dependency_invalidation() {
        let canonical = snapshot_named(
            "first/location/Main.ling",
            "module Main\n\nlet callee value = value + 1\nlet caller value = callee value\n",
        );
        let presentation_only = snapshot_named(
            "other/location/RenamedFile.ling",
            concat!(
                "module Main\r\n",
                "\r\n",
                "// display-only comment\r\n",
                "let callee value =\r\n",
                "    value + 1\r\n",
                "let caller value = callee value\r\n",
            ),
        );
        assert_eq!(canonical.program_id(), presentation_only.program_id());

        let changed_dependency = snapshot(concat!(
            "module Main\n\n",
            "let callee value = value + 2\n",
            "let caller value = callee value\n",
        ));
        let body = |snapshot: &ProgramSnapshot, name: &str| {
            snapshot
                .graph()
                .definitions
                .iter()
                .find(|definition| definition.name == name)
                .map(|definition| definition.body_id.clone())
                .expect("definition exists")
        };
        assert_ne!(
            body(&canonical, "callee"),
            body(&changed_dependency, "callee")
        );
        assert_eq!(
            body(&canonical, "caller"),
            body(&changed_dependency, "caller")
        );
        assert_ne!(canonical.program_id(), changed_dependency.program_id());

        let changed_operator = snapshot(concat!(
            "module Main\n\n",
            "let callee value = value - 1\n",
            "let caller value = callee value\n",
        ));
        assert_ne!(
            body(&canonical, "callee"),
            body(&changed_operator, "callee")
        );

        let pure = snapshot("module Main\n\nlet action () = ()\n");
        let effectful = snapshot(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let action () = Console.write \"x\"\n",
        ));
        assert_ne!(body(&pure, "action"), body(&effectful, "action"));

        let renamed = snapshot(concat!(
            "module Main\n\n",
            "let renamed value = value + 1\n",
            "let caller value = renamed value\n",
        ));
        let canonical_definition = canonical
            .graph()
            .definitions
            .iter()
            .find(|definition| definition.name == "callee")
            .expect("callee definition");
        let renamed_definition = renamed
            .graph()
            .definitions
            .iter()
            .find(|definition| definition.name == "renamed")
            .expect("renamed definition");
        assert_ne!(
            canonical_definition.definition_id,
            renamed_definition.definition_id
        );
    }

    #[test]
    fn semantic_reader_round_trips_and_accepts_namespaced_extensions() {
        let snapshot = snapshot(
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"你好，零\"\n",
        );
        let parsed = read_json(snapshot.json()).expect("writer output validates");
        assert_eq!(&parsed, snapshot.graph());

        let mut legacy: serde_json::Value =
            serde_json::from_str(snapshot.json()).expect("writer output is JSON");
        for reference in legacy["references"]
            .as_array_mut()
            .expect("references array")
        {
            reference
                .as_object_mut()
                .expect("reference object")
                .remove("source_kind");
        }
        let legacy = read_json(&serde_json::to_string(&legacy).expect("legacy graph serializes"))
            .expect("missing source_kind defaults to expression");
        assert!(
            legacy
                .references
                .iter()
                .all(|reference| reference.source_kind == "expression")
        );

        let mut extended: serde_json::Value =
            serde_json::from_str(snapshot.json()).expect("writer output is JSON");
        extended["x-vendor"] = serde_json::json!({ "future": true });
        extended["definitions"][0]["x-display"] = serde_json::json!("optional");
        read_json(&serde_json::to_string(&extended).expect("extension serializes"))
            .expect("x- fields are compatible extensions");

        extended["misspelled_schema"] = serde_json::json!(true);
        let error = read_json(&serde_json::to_string(&extended).expect("mutation serializes"))
            .expect_err("unknown core fields are rejected");
        assert!(matches!(
            error.kind,
            SemanticReadErrorKind::UnknownField { .. }
        ));
    }

    #[test]
    fn semantic_reader_accepts_and_rejects_schema_corpus() {
        let valid = include_str!("../../../schemas/semantic/0.1/valid/hello.json");
        let graph = read_json(valid).expect("valid schema corpus graph");
        assert_eq!(graph.schema, SEMANTIC_SCHEMA);
        assert_eq!(graph.entry_module, "Main");

        let invalid = include_str!("../../../schemas/semantic/0.1/invalid/invalid-program-id.json");
        assert!(matches!(
            read_json(invalid)
                .expect_err("invalid corpus ID must fail")
                .kind,
            SemanticReadErrorKind::InvalidId { .. }
        ));
    }

    #[test]
    fn semantic_reader_rejects_bad_versions_ids_kinds_and_references() {
        let snapshot = snapshot(
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"x\"\n",
        );
        let original: serde_json::Value =
            serde_json::from_str(snapshot.json()).expect("writer output is JSON");

        let mut bad_schema = original.clone();
        bad_schema["schema"] = serde_json::json!("ling.semantic/9.9");
        assert!(matches!(
            read_json(&serde_json::to_string(&bad_schema).expect("mutation serializes"))
                .expect_err("bad schema must fail")
                .kind,
            SemanticReadErrorKind::InvalidSchema { .. }
        ));

        let mut bad_language = original.clone();
        bad_language["language_version"] = serde_json::json!("9.9.9");
        assert!(matches!(
            read_json(&serde_json::to_string(&bad_language).expect("mutation serializes"))
                .expect_err("bad language version must fail")
                .kind,
            SemanticReadErrorKind::InvalidLanguageVersion { .. }
        ));

        let mut bad_unicode = original.clone();
        bad_unicode["unicode_version"] = serde_json::json!("16.0.0");
        assert!(matches!(
            read_json(&serde_json::to_string(&bad_unicode).expect("mutation serializes"))
                .expect_err("bad Unicode version must fail")
                .kind,
            SemanticReadErrorKind::InvalidUnicodeVersion { .. }
        ));

        let mut bad_id = original.clone();
        bad_id["program_id"] = serde_json::json!("not-an-id");
        assert!(matches!(
            read_json(&serde_json::to_string(&bad_id).expect("mutation serializes"))
                .expect_err("bad ID must fail")
                .kind,
            SemanticReadErrorKind::InvalidId { .. }
        ));

        let mut duplicate = original.clone();
        let duplicate_definition = duplicate["definitions"][0].clone();
        duplicate["definitions"]
            .as_array_mut()
            .expect("definitions array")
            .push(duplicate_definition);
        assert!(matches!(
            read_json(&serde_json::to_string(&duplicate).expect("mutation serializes"))
                .expect_err("duplicate IDs must fail")
                .kind,
            SemanticReadErrorKind::DuplicateId { .. }
        ));

        let mut bad_kind = original.clone();
        bad_kind["definitions"][0]["kind"] = serde_json::json!("mystery");
        assert!(matches!(
            read_json(&serde_json::to_string(&bad_kind).expect("mutation serializes"))
                .expect_err("unknown definition kind must fail")
                .kind,
            SemanticReadErrorKind::InvalidDefinitionKind { .. }
        ));

        let mut bad_node_kind = original.clone();
        bad_node_kind["nodes"][0]["kind"] = serde_json::json!("mystery");
        assert!(matches!(
            read_json(&serde_json::to_string(&bad_node_kind).expect("mutation serializes"))
                .expect_err("unknown node kind must fail")
                .kind,
            SemanticReadErrorKind::InvalidNodeKind { .. }
        ));

        let mut duplicate_node = original.clone();
        let node = duplicate_node["nodes"][0].clone();
        duplicate_node["nodes"]
            .as_array_mut()
            .expect("nodes array")
            .push(node);
        assert!(matches!(
            read_json(&serde_json::to_string(&duplicate_node).expect("mutation serializes"))
                .expect_err("duplicate node IDs must fail")
                .kind,
            SemanticReadErrorKind::DuplicateId { .. }
        ));

        let missing_id = format!("experimental:blake3:{}", "0".repeat(64));
        let mut dangling_owner = original.clone();
        dangling_owner["nodes"][0]["owner"] = serde_json::json!(&missing_id);
        assert!(matches!(
            read_json(&serde_json::to_string(&dangling_owner).expect("mutation serializes"))
                .expect_err("dangling node owners must fail")
                .kind,
            SemanticReadErrorKind::DanglingOwner { .. }
        ));

        let mut dangling_source = original.clone();
        dangling_source["references"][0]["source_id"] = serde_json::json!(&missing_id);
        assert!(matches!(
            read_json(&serde_json::to_string(&dangling_source).expect("mutation serializes"))
                .expect_err("dangling reference sources must fail")
                .kind,
            SemanticReadErrorKind::DanglingReference { .. }
        ));

        let mut cyclic_owner = original.clone();
        let node_values = cyclic_owner["nodes"].as_array_mut().expect("nodes array");
        let mut pair = None;
        'outer: for left in 0..node_values.len() {
            for right in (left + 1)..node_values.len() {
                if node_values[left]["module"] == node_values[right]["module"] {
                    pair = Some((left, right));
                    break 'outer;
                }
            }
        }
        let (left, right) = pair.expect("two nodes in one module");
        let left_id = node_values[left]["node_id"].clone();
        let right_id = node_values[right]["node_id"].clone();
        node_values[left]["owner"] = right_id;
        node_values[right]["owner"] = left_id;
        assert!(matches!(
            read_json(&serde_json::to_string(&cyclic_owner).expect("mutation serializes"))
                .expect_err("cyclic node ownership must fail")
                .kind,
            SemanticReadErrorKind::CyclicOwner { .. }
        ));

        let mut dangling = original;
        let reference = dangling["references"]
            .as_array_mut()
            .expect("references array")
            .iter_mut()
            .find(|reference| reference["target_kind"] == "definition")
            .expect("definition reference");
        reference["target"] = serde_json::json!(format!("experimental:blake3:{}", "0".repeat(64)));
        assert!(matches!(
            read_json(&serde_json::to_string(&dangling).expect("mutation serializes"))
                .expect_err("dangling references must fail")
                .kind,
            SemanticReadErrorKind::DanglingReference { .. }
        ));

        let mut malformed_reference = serde_json::from_str::<serde_json::Value>(snapshot.json())
            .expect("writer output is JSON");
        let reference = malformed_reference["references"]
            .as_array_mut()
            .expect("references array")
            .iter_mut()
            .find(|reference| reference["target_kind"] == "definition")
            .expect("definition reference");
        reference["target"] = serde_json::json!("bad-id");
        assert!(matches!(
            read_json(&serde_json::to_string(&malformed_reference).expect("mutation serializes"))
                .expect_err("malformed reference IDs must fail")
                .kind,
            SemanticReadErrorKind::InvalidId { .. }
        ));

        let mut bad_source_kind = serde_json::from_str::<serde_json::Value>(snapshot.json())
            .expect("writer output is JSON");
        bad_source_kind["references"][0]["source_kind"] = serde_json::json!("mystery");
        assert!(matches!(
            read_json(&serde_json::to_string(&bad_source_kind).expect("mutation serializes"))
                .expect_err("unknown reference source kinds must fail")
                .kind,
            SemanticReadErrorKind::InvalidReferenceSourceKind { .. }
        ));

        let mut bad_prelude_module = serde_json::from_str::<serde_json::Value>(snapshot.json())
            .expect("writer output is JSON");
        let prelude = bad_prelude_module["definitions"]
            .as_array_mut()
            .expect("definitions array")
            .iter_mut()
            .find(|definition| definition["origin"] == "prelude")
            .expect("Prelude definition");
        prelude["module"] = serde_json::json!("User.Prelude");
        assert!(matches!(
            read_json(&serde_json::to_string(&bad_prelude_module).expect("mutation serializes"))
                .expect_err("Prelude definitions must use the canonical logical module")
                .kind,
            SemanticReadErrorKind::InvalidPreludeModule { .. }
        ));
    }
}
