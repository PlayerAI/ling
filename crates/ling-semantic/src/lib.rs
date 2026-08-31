//! Deterministic checked-program snapshots and `ling.semantic/0.1` JSON.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_effects::{
    ActorTurnContract, ActorTurnSpec, ActorTypeId, CheckedProgram, LocalMailboxContract,
    MailboxCapacity, MailboxOverflowPolicy,
};
use ling_hir as hir;
use ling_project::PackageIdentity;
use ling_resolve::{
    DefinitionId, DefinitionKind, DefinitionOrigin, ExpressionKey, ModuleId, PRELUDE_MODULE,
    ReferenceTarget,
};
use ling_source::{ByteOffset, SourceId, Span};
use serde::{Deserialize, Serialize};

pub const SEMANTIC_SCHEMA: &str = "ling.semantic/0.1";
pub const PROJECT_SEMANTIC_SCHEMA: &str = "ling.semantic/0.2";
pub const LANGUAGE_VERSION: &str = "0.0.1-dev";
pub const TRAIT_IDE_EXTENSION_VERSION: &str = "0.1";
pub const TASK_GRAPH_EXTENSION_VERSION: &str = "0.1";
pub const ACTOR_GRAPH_EXTENSION_VERSION: &str = "0.3";

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_graph_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_package: Option<SemanticPackageIdentity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<SemanticPackage>,
    pub entry_module: String,
    pub modules: Vec<SemanticModule>,
    pub definitions: Vec<SemanticDefinition>,
    #[serde(default)]
    pub nodes: Vec<SemanticNode>,
    pub references: Vec<SemanticReference>,
    #[serde(
        rename = "x-ling-trait-ide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trait_ide: Option<SemanticTraitIdeProjection>,
    #[serde(
        rename = "x-ling-task",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task: Option<SemanticTaskProjection>,
    #[serde(
        rename = "x-ling-actor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actor: Option<SemanticActorProjection>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticActorProjection {
    pub version: String,
    pub actors: Vec<SemanticActor>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticActor {
    pub definition_id: String,
    pub message_type: String,
    pub sendability: String,
    pub schema_id: String,
    pub root: u32,
    pub span: SemanticByteSpan,
    pub mailbox: SemanticActorMailbox,
    pub turn: SemanticActorTurn,
    pub nodes: Vec<SemanticActorSchemaNode>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticActorMailbox {
    pub capacity: u32,
    pub overflow: String,
    pub canonical_bytes: Vec<u8>,
    pub span: SemanticByteSpan,
    pub capacity_span: SemanticByteSpan,
    pub policy_span: SemanticByteSpan,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticActorTurn {
    pub actor_type: u32,
    pub dispatch: String,
    pub suspension: String,
    pub reentry: String,
    pub state_commit: String,
    pub self_send: String,
    pub transition: u32,
    pub state_binding: u32,
    pub message_binding: u32,
    pub canonical_bytes: Vec<u8>,
    pub span: SemanticByteSpan,
    pub body_span: SemanticByteSpan,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticActorSchemaNode {
    pub id: u32,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<SemanticActorField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cases: Vec<SemanticActorCase>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticActorField {
    pub name: String,
    pub value: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticActorCase {
    pub definition: String,
    pub name: String,
    pub payload: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticTaskProjection {
    pub version: String,
    pub tasks: Vec<SemanticTask>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticTask {
    pub definition_id: String,
    pub signature: String,
    pub effects: Vec<String>,
    pub root_scope: u32,
    pub result_body: u32,
    pub span: SemanticByteSpan,
    pub scopes: Vec<SemanticTaskScope>,
    pub spawns: Vec<SemanticTaskSpawn>,
    pub suspensions: Vec<SemanticTaskSuspension>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticTaskScope {
    pub id: u32,
    pub parent: Option<u32>,
    pub span: SemanticByteSpan,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticTaskSpawn {
    pub task: u32,
    pub scope: u32,
    pub parent: u32,
    pub target: String,
    pub cancellation: u32,
    pub cleanup: u32,
    pub syntax: String,
    pub span: SemanticByteSpan,
    pub operator_span: SemanticByteSpan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern_span: Option<SemanticByteSpan>,
    pub target_span: SemanticByteSpan,
    pub call_span: SemanticByteSpan,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticTaskSuspension {
    pub id: u32,
    pub scope: u32,
    pub awaited_task: u32,
    pub live: Vec<SemanticTaskLiveBinding>,
    pub span: SemanticByteSpan,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticTaskLiveBinding {
    pub binding: u32,
    #[serde(rename = "type")]
    pub type_name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SemanticPackageIdentity {
    pub name: String,
    pub version: String,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticPackage {
    pub identity: SemanticPackageIdentity,
    pub entry_module: String,
    pub exports: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticModule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<SemanticPackageIdentity>,
    pub name: String,
    pub explicit: bool,
    pub requires: Vec<String>,
    pub imports: Vec<SemanticImport>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticImport {
    pub alias: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<SemanticPackageIdentity>,
    pub module: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticDefinition {
    pub definition_id: String,
    pub body_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<SemanticPackageIdentity>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<SemanticPackageIdentity>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<SemanticPackageIdentity>,
    pub module: String,
    #[serde(default = "default_reference_source_kind")]
    pub source_kind: String,
    pub reference: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    pub target_kind: String,
    pub target: String,
}

/// Experimental, data-only projection of checked Trait dictionary witnesses.
///
/// This is carried as an `x-*` extension so graphs without Trait witnesses
/// retain their existing wire shape. The projection never enters evaluation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticTraitIdeProjection {
    pub version: String,
    pub witnesses: Vec<SemanticTraitWitness>,
}

impl SemanticTraitIdeProjection {
    /// Returns witnesses for a Trait identity in canonical projection order.
    ///
    /// The projection is already selected and immutable; this helper only
    /// filters its records and never reruns Trait resolution.
    pub fn witnesses_by_trait_id<'a>(
        &'a self,
        trait_id: &'a str,
    ) -> impl Iterator<Item = &'a SemanticTraitWitness> + 'a {
        self.witnesses
            .iter()
            .filter(move |witness| witness.trait_id == trait_id)
    }

    /// Returns the first witness for an implementation identity.
    ///
    /// Reader validation rejects malformed graph data before consumers use
    /// it. For directly constructed values, the first projection-order match
    /// keeps this read-only helper deterministic without inventing a new
    /// uniqueness or selection rule.
    #[must_use]
    pub fn witness_by_implementation_id(
        &self,
        implementation_id: &str,
    ) -> Option<&SemanticTraitWitness> {
        self.witnesses
            .iter()
            .find(|witness| witness.implementation_id == implementation_id)
    }

    /// Returns members for a Trait definition identity in projection order.
    pub fn members_by_trait_definition_id<'a>(
        &'a self,
        trait_definition_id: &'a str,
    ) -> impl Iterator<Item = &'a SemanticTraitMember> + 'a {
        self.witnesses
            .iter()
            .flat_map(|witness| witness.members.iter())
            .filter(move |member| member.trait_definition_id == trait_definition_id)
    }

    /// Returns the first member for an implementation definition identity.
    ///
    /// As with [`Self::witness_by_implementation_id`], this is a deterministic
    /// read over the existing projection and does not perform validation or
    /// selection.
    #[must_use]
    pub fn member_by_implementation_definition_id(
        &self,
        implementation_definition_id: &str,
    ) -> Option<&SemanticTraitMember> {
        self.witnesses
            .iter()
            .flat_map(|witness| witness.members.iter())
            .find(|member| member.implementation_definition_id == implementation_definition_id)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticTraitWitness {
    pub trait_id: String,
    pub trait_name: String,
    pub trait_module: String,
    pub receiver: String,
    pub implementation_id: String,
    pub implementation_module: String,
    pub obligation_order: u32,
    pub members: Vec<SemanticTraitMember>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticTraitMember {
    pub ordinal: u32,
    pub name: String,
    pub trait_definition_id: String,
    pub implementation_definition_id: String,
    pub trait_source: String,
    pub trait_span: SemanticByteSpan,
    pub implementation_source: String,
    pub implementation_span: SemanticByteSpan,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticByteSpan {
    pub source: u32,
    pub start: u32,
    pub end: u32,
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
    pub handlers: Vec<AuditHandler>,
    pub tasks: Vec<AuditTask>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditTask {
    pub definition_id: String,
    pub signature: String,
    pub effects: Vec<String>,
    pub root_scope: u32,
    pub result_body: u32,
    pub source_start: u32,
    pub source_end: u32,
    pub scopes: Vec<AuditTaskScope>,
    pub spawns: Vec<AuditTaskSpawn>,
    pub suspensions: Vec<AuditTaskSuspension>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditTaskScope {
    pub id: u32,
    pub parent: Option<u32>,
    pub source_start: u32,
    pub source_end: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditTaskSpawn {
    pub task: u32,
    pub scope: u32,
    pub parent: u32,
    pub target: String,
    pub cancellation: u32,
    pub cleanup: u32,
    pub syntax: String,
    pub operator_start: u32,
    pub operator_end: u32,
    pub pattern_start: Option<u32>,
    pub pattern_end: Option<u32>,
    pub target_start: u32,
    pub target_end: u32,
    pub call_start: u32,
    pub call_end: u32,
    pub source_start: u32,
    pub source_end: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditTaskSuspension {
    pub id: u32,
    pub scope: u32,
    pub awaited_task: u32,
    pub live: Vec<AuditTaskLiveBinding>,
    pub source_start: u32,
    pub source_end: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditTaskLiveBinding {
    pub binding: u32,
    pub type_name: String,
}

/// Canonical Audit Source projection of one checked first-order handler.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditHandler {
    pub handler_id: String,
    pub expression: u32,
    pub source_start: u32,
    pub source_end: u32,
    pub body: u32,
    pub return_type: String,
    pub input_row: String,
    pub eliminated_effects: Vec<String>,
    pub residual_row: String,
    pub clauses: Vec<AuditHandlerClause>,
}

/// Canonical operation evidence nested under an [`AuditHandler`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditHandlerClause {
    pub operation: String,
    pub label: String,
    pub body: u32,
    pub resume_mode: String,
    pub resume_use: String,
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

const fn resume_mode_name(mode: ling_effects::ResumeMode) -> &'static str {
    match mode {
        ling_effects::ResumeMode::Never => "never",
        ling_effects::ResumeMode::Once => "once",
        ling_effects::ResumeMode::Many => "many",
    }
}

const fn resume_use_name(usage: ling_effects::ResumeUse) -> &'static str {
    match usage {
        ling_effects::ResumeUse::Never => "never",
        ling_effects::ResumeUse::Once => "once",
        ling_effects::ResumeUse::Many => "many",
    }
}

fn semantic_package_identity(package: &PackageIdentity) -> SemanticPackageIdentity {
    SemanticPackageIdentity {
        name: package.name().as_str().to_owned(),
        version: package.version().to_string(),
        source: package.source().as_str().to_owned(),
    }
}

fn encode_package_identity_to_encoder(package: &PackageIdentity, encoder: &mut Encoder) {
    encoder.string(package.name().as_str());
    encoder.string(&package.version().to_string());
    encoder.string(package.source().as_str());
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IdentityMode {
    Seed,
    Project,
}

impl IdentityMode {
    const fn schema(self) -> &'static str {
        match self {
            Self::Seed => SEMANTIC_SCHEMA,
            Self::Project => PROJECT_SEMANTIC_SCHEMA,
        }
    }

    const fn body_domain(self) -> &'static str {
        match self {
            Self::Seed => "ling.body-id/v1",
            Self::Project => "ling.body-id/v2",
        }
    }

    const fn program_domain(self) -> &'static str {
        match self {
            Self::Seed => "ling.program-id/v1",
            Self::Project => "ling.program-id/v2",
        }
    }

    const fn node_domain(self) -> &'static str {
        match self {
            Self::Seed => "ling.semantic-node-id/v1",
            Self::Project => "ling.semantic-node-id/v2",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProgramSnapshot {
    checked: CheckedProgram,
    graph: SemanticGraph,
    body_ids: BTreeMap<DefinitionId, BodyId>,
    program_id: ProgramId,
    json: String,
}

/// Package-aware checked snapshot using `ling.semantic/0.2` identities.
#[derive(Clone, Debug)]
pub struct ProjectProgramSnapshot {
    checked: CheckedProgram,
    graph: SemanticGraph,
    body_ids: BTreeMap<DefinitionId, BodyId>,
    program_id: ProgramId,
    json: String,
}

impl ProjectProgramSnapshot {
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

    /// Returns the optional experimental Trait IDE projection.
    #[must_use]
    pub fn trait_ide(&self) -> Option<&SemanticTraitIdeProjection> {
        self.graph.trait_ide.as_ref()
    }
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

    /// Returns the optional experimental Trait IDE projection.
    #[must_use]
    pub fn trait_ide(&self) -> Option<&SemanticTraitIdeProjection> {
        self.graph.trait_ide.as_ref()
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
                        handlers: Vec::new(),
                        tasks: Vec::new(),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        for definition in &self.graph.definitions {
            if definition.kind == "actor" {
                continue;
            }
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
                    handlers: Vec::new(),
                    tasks: Vec::new(),
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
                    handlers: Vec::new(),
                    tasks: Vec::new(),
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
        for (key, core) in self.checked.handler_cores() {
            let resolved_module = resolved
                .module(key.module())
                .expect("checked Handler Core references an existing module");
            let module_name = resolved_module.hir.module.name.normalized();
            let handler_id = self
                .graph
                .nodes
                .iter()
                .find(|node| {
                    node.module == module_name
                        && node.kind == "expression"
                        && node.name.as_deref() == Some("handler")
                        && node.ordinal == Some(key.local().get())
                })
                .expect("checked Handler Core has a Semantic Graph expression node")
                .node_id
                .clone();
            let span = core
                .source_span()
                .expect("checked source Handler Core retains its original byte span");
            let input_names = core.input().canonical_names();
            let residual_names = core.residual().canonical_names();
            let eliminated_effects = input_names
                .iter()
                .filter(|name| !residual_names.contains(name))
                .cloned()
                .collect();
            let clauses = core
                .clauses()
                .iter()
                .map(|clause| AuditHandlerClause {
                    operation: clause.clause().operation().canonical_name(),
                    label: clause.clause().label().canonical_name(),
                    body: clause.body().get(),
                    resume_mode: resume_mode_name(clause.clause().operation().resume_mode())
                        .to_owned(),
                    resume_use: resume_use_name(clause.resume_use()).to_owned(),
                })
                .collect();
            modules
                .get_mut(&module_name)
                .expect("checked Handler Core module exists in Audit model")
                .handlers
                .push(AuditHandler {
                    handler_id,
                    expression: key.local().get(),
                    source_start: u32::try_from(span.start_byte()).unwrap_or(u32::MAX),
                    source_end: u32::try_from(span.end_byte()).unwrap_or(u32::MAX),
                    body: core.body().get(),
                    return_type: core.return_type().as_str().to_owned(),
                    input_row: core.input().canonical_name(),
                    eliminated_effects,
                    residual_row: core.residual().canonical_name(),
                    clauses,
                });
        }
        for core in self.checked.task_cores().values() {
            let info = resolved
                .definition(core.definition())
                .expect("checked Task Core references an existing definition");
            let module = modules
                .get_mut(&info.module_name)
                .expect("checked Task Core module exists in Audit model");
            module.tasks.push(AuditTask {
                definition_id: core.definition().to_string(),
                signature: format!(
                    "{} ! {{{}}}",
                    self.checked.typed().display_type(
                        self.checked
                            .typed()
                            .definition_type(core.definition())
                            .expect("checked Task type exists")
                    ),
                    core.signature().effects().canonical_names().join(", ")
                ),
                effects: core.signature().effects().canonical_names(),
                root_scope: core.root_scope().get(),
                result_body: core.result_body().local().get(),
                source_start: core.source_span().start().get(),
                source_end: core.source_span().end().get(),
                scopes: core
                    .scopes()
                    .iter()
                    .map(|scope| AuditTaskScope {
                        id: scope.id().get(),
                        parent: scope.parent().map(|parent| parent.get()),
                        source_start: scope.span().start().get(),
                        source_end: scope.span().end().get(),
                    })
                    .collect(),
                spawns: core
                    .spawns()
                    .iter()
                    .map(|spawn| AuditTaskSpawn {
                        task: spawn.task().get(),
                        scope: spawn.scope().get(),
                        parent: spawn.parent().get(),
                        target: spawn.target().to_string(),
                        cancellation: spawn.cancellation().get(),
                        cleanup: spawn.cleanup().get(),
                        syntax: spawn.syntax().as_str().to_owned(),
                        operator_start: spawn.operator_span().start().get(),
                        operator_end: spawn.operator_span().end().get(),
                        pattern_start: spawn.pattern_span().map(|span| span.start().get()),
                        pattern_end: spawn.pattern_span().map(|span| span.end().get()),
                        target_start: spawn.target_span().start().get(),
                        target_end: spawn.target_span().end().get(),
                        call_start: spawn.call_span().start().get(),
                        call_end: spawn.call_span().end().get(),
                        source_start: spawn.span().start().get(),
                        source_end: spawn.span().end().get(),
                    })
                    .collect(),
                suspensions: core
                    .suspensions()
                    .iter()
                    .map(|suspension| AuditTaskSuspension {
                        id: suspension.id().get(),
                        scope: suspension.scope().get(),
                        awaited_task: suspension.awaited_task().get(),
                        live: suspension
                            .live()
                            .iter()
                            .map(|live| AuditTaskLiveBinding {
                                binding: live.binding().local().get(),
                                type_name: self.checked.typed().display_type(live.value_type()),
                            })
                            .collect(),
                        source_start: suspension.span().start().get(),
                        source_end: suspension.span().end().get(),
                    })
                    .collect(),
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
            module
                .handlers
                .sort_by(|left, right| left.handler_id.cmp(&right.handler_id));
            module
                .tasks
                .sort_by(|left, right| left.definition_id.cmp(&right.definition_id));
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
pub enum SnapshotError {
    Serialization(serde_json::Error),
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialization(error) => {
                write!(formatter, "failed to serialize semantic snapshot: {error}")
            }
        }
    }
}

impl Error for SnapshotError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
        }
    }
}

#[derive(Debug)]
pub enum ProjectSnapshotError {
    MissingProjectContext,
    UnsupportedCheckedHandler,
    Serialization(serde_json::Error),
}

impl fmt::Display for ProjectSnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProjectContext => formatter.write_str(
                "cannot build a package-aware semantic snapshot from a file-mode program",
            ),
            Self::UnsupportedCheckedHandler => formatter.write_str(
                "checked handlers require an accepted package-aware graph and Audit projection",
            ),
            Self::Serialization(error) => {
                write!(
                    formatter,
                    "failed to serialize project semantic snapshot: {error}"
                )
            }
        }
    }
}

impl Error for ProjectSnapshotError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::MissingProjectContext | Self::UnsupportedCheckedHandler => None,
            Self::Serialization(error) => Some(error),
        }
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
    MissingProjectField {
        field: String,
    },
    InvalidPackageIdentity {
        value: String,
    },
    DuplicatePackage {
        package: String,
    },
    UnknownPackage {
        package: String,
    },
    UnknownPackageModule {
        package: String,
        module: String,
    },
    PrivatePackageModule {
        package: String,
        module: String,
    },
    UnimportedReference {
        target: String,
    },
    PackageCoordinateMismatch {
        entity: String,
    },
    InvalidTraitIdeVersion {
        version: String,
    },
    InvalidTraitIdeIdentity {
        value: String,
    },
    InvalidTraitIdeSpan,
    DuplicateTraitIdeOrdinal {
        ordinal: u32,
    },
    UnsortedTraitIdeOrdinal {
        ordinal: u32,
    },
    TraitIdeMismatch {
        value: String,
    },
    InvalidAuditHandler {
        reason: String,
    },
    InvalidTaskProjection {
        reason: String,
    },
    InvalidActorProjection {
        reason: String,
    },
    InvalidAuditTask {
        reason: String,
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
    SnapshotBuilder::new(checked, IdentityMode::Seed).build()
}

/// Builds a package-aware checked snapshot using `ling.semantic/0.2` identities.
///
/// The checked program must originate from [`ling_resolve::resolve_project`].
/// File-mode programs continue to use [`build`] and retain their frozen 0.1
/// bytes and identifiers.
pub fn build_project(
    checked: CheckedProgram,
) -> Result<ProjectProgramSnapshot, ProjectSnapshotError> {
    if checked.typed().resolved().project().is_none() {
        return Err(ProjectSnapshotError::MissingProjectContext);
    }
    if !checked.handler_cores().is_empty() {
        return Err(ProjectSnapshotError::UnsupportedCheckedHandler);
    }
    SnapshotBuilder::new(checked, IdentityMode::Project).build_project()
}

/// Parses and structurally validates a `ling.semantic/0.1` JSON document.
///
/// Unknown fields are accepted only when their names begin with `x-`, keeping
/// extensions forward-compatible without silently accepting misspelled core
/// protocol fields. The returned graph is data only and cannot be converted
/// into the checked snapshot required by the evaluator.
pub fn read_json(input: &str) -> Result<SemanticGraph, SemanticReadError> {
    read_graph(input, IdentityMode::Seed)
}

/// Parses and structurally validates a package-aware `ling.semantic/0.2` JSON document.
///
/// This reader is intentionally separate from [`read_json`]: neither reader
/// guesses a protocol version from the presence of package fields.
pub fn read_project_json(input: &str) -> Result<SemanticGraph, SemanticReadError> {
    read_graph(input, IdentityMode::Project)
}

fn read_graph(input: &str, mode: IdentityMode) -> Result<SemanticGraph, SemanticReadError> {
    let value: serde_json::Value =
        serde_json::from_str(input).map_err(|error| SemanticReadError {
            kind: SemanticReadErrorKind::InvalidJson {
                message: error.to_string(),
            },
            path: "$".to_owned(),
        })?;
    validate_json_fields(&value, mode)?;
    let graph: SemanticGraph =
        serde_json::from_value(value).map_err(|error| SemanticReadError {
            kind: SemanticReadErrorKind::InvalidJson {
                message: error.to_string(),
            },
            path: "$".to_owned(),
        })?;
    validate_graph(&graph, mode)?;
    Ok(graph)
}

/// Validates an isolated Audit model without granting it executable status.
pub fn validate_audit_model(model: &AuditModel) -> Result<(), SemanticReadError> {
    let graph = SemanticGraph {
        schema: model.semantic_schema.clone(),
        language_version: model.language_version.clone(),
        unicode_version: model.unicode_version.clone(),
        program_id: model.program_id.clone(),
        package_graph_id: None,
        root_package: None,
        packages: Vec::new(),
        entry_module: model.entry_module.clone(),
        modules: model
            .modules
            .iter()
            .map(|module| SemanticModule {
                package: None,
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
                        package: None,
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
                    package: None,
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
                    package: None,
                    module: module.name.clone(),
                    source_kind: reference.source_kind.clone(),
                    reference: reference.reference,
                    source_id: reference.source_id.clone(),
                    target_kind: reference.target_kind.clone(),
                    target: reference.target.clone(),
                })
            })
            .collect(),
        trait_ide: None,
        task: None,
        actor: None,
    };
    validate_graph(&graph, IdentityMode::Seed)?;
    let task_definitions = model
        .modules
        .iter()
        .flat_map(|module| module.definitions.iter())
        .filter(|definition| definition.kind == "task")
        .map(|definition| definition.definition_id.as_str())
        .collect::<BTreeSet<_>>();
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
        let expression_nodes = module
            .nodes
            .iter()
            .filter(|node| node.kind == "expression")
            .map(|node| (node.node_id.as_str(), node))
            .collect::<BTreeMap<_, _>>();
        let mut previous_handler_id: Option<&str> = None;
        for (handler_index, handler) in module.handlers.iter().enumerate() {
            let path = format!("$.modules[{module_index}].handlers[{handler_index}]");
            if previous_handler_id.is_some_and(|previous| previous >= handler.handler_id.as_str()) {
                return Err(invalid_audit_handler(
                    &path,
                    "handler identities must be unique and strictly sorted",
                ));
            }
            previous_handler_id = Some(&handler.handler_id);
            let Some(node) = expression_nodes.get(handler.handler_id.as_str()) else {
                return Err(invalid_audit_handler(
                    &format!("{path}.handler_id"),
                    "handler identity must reference an expression node in the same module",
                ));
            };
            if node.name.as_deref() != Some("handler")
                || node.ordinal != Some(handler.expression)
                || handler.source_start > handler.source_end
                || handler.body == 0
                || !expression_nodes
                    .values()
                    .any(|candidate| candidate.ordinal == Some(handler.body - 1))
                || handler.return_type.is_empty()
                || handler.input_row.is_empty()
                || handler.residual_row.is_empty()
                || handler.clauses.is_empty()
            {
                return Err(invalid_audit_handler(
                    &path,
                    "handler node, span, body, type, rows, or clauses are invalid",
                ));
            }
            if !strictly_sorted_unique(&handler.eliminated_effects) {
                return Err(invalid_audit_handler(
                    &format!("{path}.eliminated_effects"),
                    "eliminated effects must be unique and sorted",
                ));
            }
            let mut previous_label: Option<&str> = None;
            for (clause_index, clause) in handler.clauses.iter().enumerate() {
                let clause_path = format!("{path}.clauses[{clause_index}]");
                if previous_label.is_some_and(|previous| previous >= clause.label.as_str()) {
                    return Err(invalid_audit_handler(
                        &clause_path,
                        "handler clause labels must be unique and strictly sorted",
                    ));
                }
                previous_label = Some(&clause.label);
                if clause.body == 0
                    || !expression_nodes
                        .values()
                        .any(|candidate| candidate.ordinal == Some(clause.body - 1))
                    || !valid_audit_handler_clause(clause)
                {
                    return Err(invalid_audit_handler(
                        &clause_path,
                        "handler clause operation, label, body, resume mode, or use is invalid",
                    ));
                }
            }
            let expected_eliminated = handler
                .clauses
                .iter()
                .map(|clause| clause.label.clone())
                .filter(|label| {
                    row_contains_label(&handler.input_row, label)
                        && !row_contains_label(&handler.residual_row, label)
                })
                .collect::<Vec<_>>();
            if handler.eliminated_effects != expected_eliminated {
                return Err(invalid_audit_handler(
                    &format!("{path}.eliminated_effects"),
                    "eliminated effects do not match the input, clauses, and residual row",
                ));
            }
        }
        let mut previous_task: Option<&str> = None;
        for (task_index, task) in module.tasks.iter().enumerate() {
            let path = format!("$.modules[{module_index}].tasks[{task_index}]");
            if previous_task.is_some_and(|previous| previous >= task.definition_id.as_str()) {
                return Err(invalid_audit_task(
                    &path,
                    "Task definitions must be unique and strictly sorted",
                ));
            }
            previous_task = Some(&task.definition_id);
            validate_audit_task(task, &task_definitions, &path)?;
        }
    }
    Ok(())
}

fn validate_audit_task(
    task: &AuditTask,
    task_definitions: &BTreeSet<&str>,
    path: &str,
) -> Result<(), SemanticReadError> {
    if !task_definitions.contains(task.definition_id.as_str())
        || task.signature.is_empty()
        || task.root_scope == 0
        || task.source_start > task.source_end
        || !strictly_sorted_unique(&task.effects)
    {
        return Err(invalid_audit_task(
            path,
            "Task definition, signature, identity, span, or Effect row is invalid",
        ));
    }
    let mut scope_ids = BTreeSet::new();
    for (scope_index, scope) in task.scopes.iter().enumerate() {
        if scope.id == 0
            || !scope_ids.insert(scope.id)
            || scope.source_start > scope.source_end
            || scope.source_start < task.source_start
            || scope.source_end > task.source_end
            || scope
                .parent
                .is_some_and(|parent| !scope_ids.contains(&parent))
        {
            return Err(invalid_audit_task(
                &format!("{path}.scopes[{scope_index}]"),
                "Task scope identity, parent, order, or span is invalid",
            ));
        }
    }
    if task
        .scopes
        .first()
        .is_none_or(|scope| scope.id != task.root_scope || scope.parent.is_some())
    {
        return Err(invalid_audit_task(
            &format!("{path}.root_scope"),
            "root scope must be the first parentless Task scope",
        ));
    }
    let mut spawned = BTreeMap::new();
    for (spawn_index, spawn) in task.spawns.iter().enumerate() {
        let pattern_valid = match spawn.syntax.as_str() {
            "explicit" => spawn.pattern_start.is_none() && spawn.pattern_end.is_none(),
            "let!" => {
                matches!((spawn.pattern_start, spawn.pattern_end), (Some(start), Some(end)) if start <= end)
            }
            _ => false,
        };
        if spawn.task <= 1
            || spawned.insert(spawn.task, spawn.scope).is_some()
            || !scope_ids.contains(&spawn.scope)
            || spawn.parent != 1
            || !task_definitions.contains(spawn.target.as_str())
            || spawn.cancellation != spawn.task
            || spawn.cleanup != spawn.task
            || !pattern_valid
            || !valid_nested_span(
                task.source_start,
                task.source_end,
                spawn.source_start,
                spawn.source_end,
            )
            || !valid_nested_span(
                task.source_start,
                task.source_end,
                spawn.operator_start,
                spawn.operator_end,
            )
            || !valid_nested_span(
                task.source_start,
                task.source_end,
                spawn.target_start,
                spawn.target_end,
            )
            || !valid_nested_span(
                task.source_start,
                task.source_end,
                spawn.call_start,
                spawn.call_end,
            )
        {
            return Err(invalid_audit_task(
                &format!("{path}.spawns[{spawn_index}]"),
                "Task spawn identity, ownership, syntax, target, or span is invalid",
            ));
        }
    }
    let mut suspension_ids = BTreeSet::new();
    let mut awaited = BTreeSet::new();
    for (suspension_index, suspension) in task.suspensions.iter().enumerate() {
        let mut previous_binding = None;
        let live_valid = suspension.live.iter().all(|live| {
            let ordered = previous_binding.is_none_or(|previous| previous < live.binding);
            previous_binding = Some(live.binding);
            ordered && live.binding != 0 && !live.type_name.is_empty()
        });
        if suspension.id == 0
            || !suspension_ids.insert(suspension.id)
            || spawned.get(&suspension.awaited_task) != Some(&suspension.scope)
            || !valid_nested_span(
                task.source_start,
                task.source_end,
                suspension.source_start,
                suspension.source_end,
            )
            || !live_valid
        {
            return Err(invalid_audit_task(
                &format!("{path}.suspensions[{suspension_index}]"),
                "Task suspension identity, ownership, live set, or span is invalid",
            ));
        }
        awaited.insert(suspension.awaited_task);
    }
    if awaited != spawned.keys().copied().collect() {
        return Err(invalid_audit_task(
            &format!("{path}.suspensions"),
            "every spawned Task must have at least one statically checked suspension observation",
        ));
    }
    Ok(())
}

fn valid_nested_span(outer_start: u32, outer_end: u32, start: u32, end: u32) -> bool {
    start <= end && outer_start <= start && end <= outer_end
}

fn invalid_audit_task(path: &str, reason: &str) -> SemanticReadError {
    SemanticReadError {
        kind: SemanticReadErrorKind::InvalidAuditTask {
            reason: reason.to_owned(),
        },
        path: path.to_owned(),
    }
}

fn invalid_audit_handler(path: &str, reason: &str) -> SemanticReadError {
    SemanticReadError {
        kind: SemanticReadErrorKind::InvalidAuditHandler {
            reason: reason.to_owned(),
        },
        path: path.to_owned(),
    }
}

fn strictly_sorted_unique(values: &[String]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn valid_audit_handler_clause(clause: &AuditHandlerClause) -> bool {
    let expected = match clause.operation.as_str() {
        "Clock::now()->Int::Once" => ("Clock", "once"),
        "Console.Write::write(Text)->Unit::Once" => ("Console.Write", "once"),
        "Random::next(Int)->Int::Many" => ("Random", "many"),
        _ => return false,
    };
    if clause.label != expected.0 || clause.resume_mode != expected.1 {
        return false;
    }
    matches!(
        (clause.resume_mode.as_str(), clause.resume_use.as_str()),
        ("never", "never") | ("once", "never" | "once") | ("many", "never" | "once" | "many")
    )
}

fn row_contains_label(row: &str, label: &str) -> bool {
    row.strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .is_some_and(|contents| {
            contents
                .split('|')
                .next()
                .unwrap_or_default()
                .split(',')
                .any(|candidate| candidate == label)
        })
}

fn validate_json_fields(
    value: &serde_json::Value,
    mode: IdentityMode,
) -> Result<(), SemanticReadError> {
    let root_fields: &[&str] = match mode {
        IdentityMode::Seed => &[
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
        IdentityMode::Project => &[
            "schema",
            "language_version",
            "unicode_version",
            "program_id",
            "package_graph_id",
            "root_package",
            "packages",
            "entry_module",
            "modules",
            "definitions",
            "nodes",
            "references",
        ],
    };
    validate_object_fields(value, "$", root_fields)?;

    if mode == IdentityMode::Project {
        validate_required_fields(value, "$", root_fields)?;
        validate_package_object(value.get("root_package"), "$.root_package")?;
        validate_array_objects(
            value.get("packages"),
            "$.packages",
            &["identity", "entry_module", "exports"],
            |package, path| {
                validate_required_fields(package, path, &["identity", "entry_module", "exports"])?;
                validate_package_object(package.get("identity"), &format!("{path}.identity"))
            },
        )?;
    }

    let module_fields: &[&str] = match mode {
        IdentityMode::Seed => &["name", "explicit", "requires", "imports"],
        IdentityMode::Project => &["package", "name", "explicit", "requires", "imports"],
    };
    let import_fields: &[&str] = match mode {
        IdentityMode::Seed => &["alias", "module"],
        IdentityMode::Project => &["alias", "package", "module"],
    };
    validate_array_objects(
        value.get("modules"),
        "$.modules",
        module_fields,
        |module, path| {
            if mode == IdentityMode::Project {
                validate_required_fields(module, path, module_fields)?;
                validate_package_object(module.get("package"), &format!("{path}.package"))?;
            }
            validate_array_objects(
                module.get("imports"),
                &format!("{path}.imports"),
                import_fields,
                |import, import_path| {
                    if mode == IdentityMode::Project {
                        validate_required_fields(import, import_path, import_fields)?;
                        validate_package_object(
                            import.get("package"),
                            &format!("{import_path}.package"),
                        )?;
                    }
                    Ok(())
                },
            )
        },
    )?;

    let definition_fields: &[&str] = match mode {
        IdentityMode::Seed => &[
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
        IdentityMode::Project => &[
            "definition_id",
            "body_id",
            "package",
            "module",
            "name",
            "kind",
            "origin",
            "type",
            "effects",
            "capabilities",
        ],
    };
    validate_array_objects(
        value.get("definitions"),
        "$.definitions",
        definition_fields,
        |definition, path| {
            if mode == IdentityMode::Project {
                validate_required_fields(
                    definition,
                    path,
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
                )?;
                if definition.get("package").is_some() {
                    validate_package_object(definition.get("package"), &format!("{path}.package"))?;
                }
            }
            Ok(())
        },
    )?;

    let node_fields: &[&str] = match mode {
        IdentityMode::Seed => &[
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
        IdentityMode::Project => &[
            "node_id",
            "package",
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
    };
    validate_array_objects(value.get("nodes"), "$.nodes", node_fields, |node, path| {
        if mode == IdentityMode::Project {
            validate_required_fields(
                node,
                path,
                &[
                    "node_id",
                    "module",
                    "kind",
                    "owner",
                    "effects",
                    "capabilities",
                    "identifier_scripts",
                    "identifier_suspicious_mixed_script",
                ],
            )?;
            if node.get("package").is_some() {
                validate_package_object(node.get("package"), &format!("{path}.package"))?;
            }
        }
        Ok(())
    })?;

    let reference_fields: &[&str] = match mode {
        IdentityMode::Seed => &[
            "module",
            "source_kind",
            "reference",
            "source_id",
            "target_kind",
            "target",
        ],
        IdentityMode::Project => &[
            "package",
            "module",
            "source_kind",
            "reference",
            "source_id",
            "target_kind",
            "target",
        ],
    };
    validate_array_objects(
        value.get("references"),
        "$.references",
        reference_fields,
        |reference, path| {
            if mode == IdentityMode::Project {
                validate_required_fields(reference, path, reference_fields)?;
                validate_package_object(reference.get("package"), &format!("{path}.package"))?;
            }
            Ok(())
        },
    )
}

fn validate_package_object(
    value: Option<&serde_json::Value>,
    path: &str,
) -> Result<(), SemanticReadError> {
    let Some(value) = value else {
        return Ok(());
    };
    let fields = &["name", "version", "source"];
    validate_object_fields(value, path, fields)?;
    validate_required_fields(value, path, fields)
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

fn validate_required_fields(
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
    for field in fields {
        if !object.contains_key(*field) {
            return Err(missing_project_field(field, &format!("{path}.{field}")));
        }
    }
    Ok(())
}

fn validate_graph(graph: &SemanticGraph, mode: IdentityMode) -> Result<(), SemanticReadError> {
    if graph.schema != mode.schema() {
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

    let package_identities = validate_project_context(graph, mode)?;
    let mut modules = BTreeSet::new();
    for (index, module) in graph.modules.iter().enumerate() {
        let coordinate = required_module_coordinate(
            module.package.as_ref(),
            &module.name,
            &format!("$.modules[{index}].package"),
            mode,
            &package_identities,
        )?;
        if !modules.insert(coordinate) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DuplicateModule {
                    module: module.name.clone(),
                },
                path: format!("$.modules[{index}].name"),
            });
        }
    }
    let entry_package = match mode {
        IdentityMode::Seed => None,
        IdentityMode::Project => graph.root_package.clone(),
    };
    if !modules.contains(&(entry_package, graph.entry_module.clone())) {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::MissingEntryModule {
                module: graph.entry_module.clone(),
            },
            path: "$.entry_module".to_owned(),
        });
    }
    if mode == IdentityMode::Project {
        for (package_index, package) in graph.packages.iter().enumerate() {
            validate_package_module(
                &modules,
                &package.identity,
                &package.entry_module,
                &format!("$.packages[{package_index}].entry_module"),
            )?;
            let mut exports = BTreeSet::new();
            for (export_index, export) in package.exports.iter().enumerate() {
                if !exports.insert(export) {
                    return Err(SemanticReadError {
                        kind: SemanticReadErrorKind::DuplicateModule {
                            module: export.clone(),
                        },
                        path: format!("$.packages[{package_index}].exports[{export_index}]"),
                    });
                }
                validate_package_module(
                    &modules,
                    &package.identity,
                    export,
                    &format!("$.packages[{package_index}].exports[{export_index}]"),
                )?;
            }
        }
    }
    let package_exports = graph
        .packages
        .iter()
        .map(|package| {
            (
                package.identity.clone(),
                package.exports.iter().cloned().collect::<BTreeSet<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut module_imports = BTreeMap::<ModuleCoordinate, BTreeSet<ModuleCoordinate>>::new();
    for (module_index, module) in graph.modules.iter().enumerate() {
        let module_coordinate = required_module_coordinate(
            module.package.as_ref(),
            &module.name,
            &format!("$.modules[{module_index}].package"),
            mode,
            &package_identities,
        )?;
        let mut aliases = BTreeSet::new();
        let mut imported_coordinates = BTreeSet::new();
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
            let coordinate = required_module_coordinate(
                import.package.as_ref(),
                &import.module,
                &format!("$.modules[{module_index}].imports[{import_index}].package"),
                mode,
                &package_identities,
            )?;
            if !modules.contains(&coordinate) {
                return Err(SemanticReadError {
                    kind: unknown_module_kind(&coordinate),
                    path: format!("$.modules[{module_index}].imports[{import_index}].module"),
                });
            }
            if mode == IdentityMode::Project && coordinate.0 != module_coordinate.0 {
                let target_package = coordinate
                    .0
                    .as_ref()
                    .expect("project imports always have package coordinates");
                let exported = package_exports
                    .get(target_package)
                    .is_some_and(|exports| exports.contains(&coordinate.1));
                if !exported {
                    return Err(SemanticReadError {
                        kind: SemanticReadErrorKind::PrivatePackageModule {
                            package: package_label(target_package),
                            module: coordinate.1.clone(),
                        },
                        path: format!("$.modules[{module_index}].imports[{import_index}].module"),
                    });
                }
            }
            imported_coordinates.insert(coordinate);
        }
        module_imports.insert(module_coordinate, imported_coordinates);
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
        let definition_coordinate = match definition.origin.as_str() {
            "user" => required_module_coordinate(
                definition.package.as_ref(),
                &definition.module,
                &format!("$.definitions[{index}].package"),
                mode,
                &package_identities,
            )?,
            _ => system_coordinate(
                definition.package.as_ref(),
                &definition.module,
                &format!("$.definitions[{index}].package"),
            )?,
        };
        definition_modules.insert(
            definition.definition_id.clone(),
            definition_coordinate.clone(),
        );
        if !bodies.insert(definition.body_id.clone()) && mode == IdentityMode::Seed {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DuplicateId {
                    value: definition.body_id.clone(),
                },
                path: format!("$.definitions[{index}].body_id"),
            });
        }
        if !matches!(
            definition.kind.as_str(),
            "value" | "task" | "actor" | "type" | "constructor" | "builtin" | "trait-member"
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
        if definition.origin == "user" && !modules.contains(&definition_coordinate) {
            return Err(SemanticReadError {
                kind: unknown_module_kind(&definition_coordinate),
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
        "task",
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
        let coordinate = optional_module_coordinate(
            node.package.as_ref(),
            &node.module,
            &format!("$.nodes[{index}].package"),
            mode,
            &package_identities,
        )?;
        if coordinate.0.is_some() && !modules.contains(&coordinate) {
            return Err(SemanticReadError {
                kind: unknown_module_kind(&coordinate),
                path: format!("$.nodes[{index}].module"),
            });
        }
        if definitions.contains(&node.node_id)
            || nodes
                .insert(node.node_id.clone(), (node.kind.clone(), coordinate))
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
        let owner_module = definition_modules
            .get(&node.owner)
            .or_else(|| nodes.get(&node.owner).map(|(_, coordinate)| coordinate));
        let node_coordinate = nodes
            .get(&node.node_id)
            .map(|(_, coordinate)| coordinate)
            .expect("validated node was inserted");
        if owner_module.is_some_and(|coordinate| coordinate != node_coordinate) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::PackageCoordinateMismatch {
                    entity: node.node_id.clone(),
                },
                path: format!("$.nodes[{index}].package"),
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
        let coordinate = required_module_coordinate(
            reference.package.as_ref(),
            &reference.module,
            &format!("$.references[{index}].package"),
            mode,
            &package_identities,
        )?;
        if !modules.contains(&coordinate) {
            return Err(SemanticReadError {
                kind: unknown_module_kind(&coordinate),
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
            coordinate.clone(),
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
            if nodes
                .get(source_id)
                .is_none_or(|(kind, source_coordinate)| {
                    kind != expected_kind || source_coordinate != &coordinate
                })
            {
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
                let target_coordinate = &definition_modules[&reference.target];
                let target_is_system = target_coordinate.0.is_none();
                let target_is_local = target_coordinate == &coordinate;
                let target_is_imported = module_imports
                    .get(&coordinate)
                    .is_some_and(|imports| imports.contains(target_coordinate));
                if !target_is_system && !target_is_local && !target_is_imported {
                    return Err(SemanticReadError {
                        kind: SemanticReadErrorKind::UnimportedReference {
                            target: reference.target.clone(),
                        },
                        path: format!("$.references[{index}].target"),
                    });
                }
            }
            "binding" => {
                let legacy_local = mode == IdentityMode::Seed
                    && reference
                        .target
                        .strip_prefix("local:")
                        .and_then(|value| value.parse::<u32>().ok())
                        .is_some();
                let resolved_node = nodes
                    .get(&reference.target)
                    .is_some_and(|(kind, _)| kind == "binding");
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
    validate_trait_ide(graph)?;
    validate_task_projection(graph)?;
    validate_actor_projection(graph)?;
    Ok(())
}

fn validate_task_projection(graph: &SemanticGraph) -> Result<(), SemanticReadError> {
    let Some(projection) = &graph.task else {
        return Ok(());
    };
    if projection.version != TASK_GRAPH_EXTENSION_VERSION {
        return Err(invalid_task_projection(
            "$.x-ling-task.version",
            "unsupported Task projection version",
        ));
    }
    let task_definitions = graph
        .definitions
        .iter()
        .filter(|definition| definition.kind == "task")
        .map(|definition| definition.definition_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut previous_definition: Option<&str> = None;
    for (task_index, task) in projection.tasks.iter().enumerate() {
        let path = format!("$.x-ling-task.tasks[{task_index}]");
        if previous_definition.is_some_and(|previous| previous >= task.definition_id.as_str()) {
            return Err(invalid_task_projection(
                &path,
                "Task definitions must be unique and strictly sorted",
            ));
        }
        previous_definition = Some(&task.definition_id);
        if !task_definitions.contains(task.definition_id.as_str())
            || task.signature.is_empty()
            || task.root_scope == 0
            || task.span.start > task.span.end
            || !strictly_sorted_unique(&task.effects)
        {
            return Err(invalid_task_projection(
                &path,
                "Task definition, signature, identity, span, or Effect row is invalid",
            ));
        }
        let mut scope_ids = BTreeSet::new();
        for (scope_index, scope) in task.scopes.iter().enumerate() {
            if scope.id == 0
                || !scope_ids.insert(scope.id)
                || scope.span.start > scope.span.end
                || scope
                    .parent
                    .is_some_and(|parent| !scope_ids.contains(&parent))
            {
                return Err(invalid_task_projection(
                    &format!("{path}.scopes[{scope_index}]"),
                    "Task scope identity, parent, order, or span is invalid",
                ));
            }
        }
        if task
            .scopes
            .first()
            .is_none_or(|scope| scope.id != task.root_scope || scope.parent.is_some())
        {
            return Err(invalid_task_projection(
                &format!("{path}.root_scope"),
                "root scope must be the first parentless Task scope",
            ));
        }
        let mut spawned = BTreeMap::new();
        for (spawn_index, spawn) in task.spawns.iter().enumerate() {
            if spawn.task <= 1
                || spawned.insert(spawn.task, spawn.scope).is_some()
                || !scope_ids.contains(&spawn.scope)
                || spawn.parent != 1
                || !task_definitions.contains(spawn.target.as_str())
                || spawn.cancellation != spawn.task
                || spawn.cleanup != spawn.task
                || !matches!(spawn.syntax.as_str(), "explicit" | "let!")
                || spawn.span.start > spawn.span.end
                || spawn.operator_span.start > spawn.operator_span.end
                || spawn.target_span.start > spawn.target_span.end
                || spawn.call_span.start > spawn.call_span.end
                || spawn
                    .pattern_span
                    .as_ref()
                    .is_some_and(|span| span.start > span.end)
                || (spawn.syntax == "let!") != spawn.pattern_span.is_some()
            {
                return Err(invalid_task_projection(
                    &format!("{path}.spawns[{spawn_index}]"),
                    "Task spawn identity, ownership, syntax, target, or span is invalid",
                ));
            }
        }
        let mut suspension_ids = BTreeSet::new();
        for (suspension_index, suspension) in task.suspensions.iter().enumerate() {
            let mut previous_binding = None;
            let live_valid = suspension.live.iter().all(|live| {
                let ordered = previous_binding.is_none_or(|previous| previous < live.binding);
                previous_binding = Some(live.binding);
                ordered && live.binding != 0 && !live.type_name.is_empty()
            });
            if suspension.id == 0
                || !suspension_ids.insert(suspension.id)
                || spawned.get(&suspension.awaited_task) != Some(&suspension.scope)
                || suspension.span.start > suspension.span.end
                || !live_valid
            {
                return Err(invalid_task_projection(
                    &format!("{path}.suspensions[{suspension_index}]"),
                    "Task suspension identity, ownership, live set, or span is invalid",
                ));
            }
        }
    }
    Ok(())
}

fn invalid_task_projection(path: &str, reason: &str) -> SemanticReadError {
    SemanticReadError {
        kind: SemanticReadErrorKind::InvalidTaskProjection {
            reason: reason.to_owned(),
        },
        path: path.to_owned(),
    }
}

fn validate_actor_projection(graph: &SemanticGraph) -> Result<(), SemanticReadError> {
    let actor_definitions = graph
        .definitions
        .iter()
        .filter(|definition| definition.kind == "actor")
        .map(|definition| (definition.definition_id.as_str(), definition))
        .collect::<BTreeMap<_, _>>();
    let Some(projection) = &graph.actor else {
        return if actor_definitions.is_empty() {
            Ok(())
        } else {
            Err(invalid_actor_projection(
                "$.x-ling-actor",
                "checked Actor definitions require the Actor projection",
            ))
        };
    };
    if projection.version != ACTOR_GRAPH_EXTENSION_VERSION {
        return Err(invalid_actor_projection(
            "$.x-ling-actor.version",
            "unsupported Actor projection version",
        ));
    }
    if projection.actors.len() != actor_definitions.len() {
        return Err(invalid_actor_projection(
            "$.x-ling-actor.actors",
            "Actor projection must cover every checked Actor definition exactly once",
        ));
    }
    let mut previous_definition: Option<&str> = None;
    for (actor_index, actor) in projection.actors.iter().enumerate() {
        let path = format!("$.x-ling-actor.actors[{actor_index}]");
        if previous_definition.is_some_and(|previous| previous >= actor.definition_id.as_str()) {
            return Err(invalid_actor_projection(
                &path,
                "Actor definitions must be unique and strictly sorted",
            ));
        }
        previous_definition = Some(&actor.definition_id);
        let Some(definition) = actor_definitions.get(actor.definition_id.as_str()) else {
            return Err(invalid_actor_projection(
                &path,
                "Actor projection owner is not a checked Actor definition",
            ));
        };
        validate_id(&actor.definition_id, &format!("{path}.definition_id"))?;
        validate_id(&actor.schema_id, &format!("{path}.schema_id"))?;
        if actor.message_type.is_empty()
            || !definition.type_name.contains(&actor.message_type)
            || actor.sendability != "SendableLocal(Value)"
            || actor.span.start > actor.span.end
            || actor.nodes.is_empty()
            || usize::try_from(actor.root)
                .ok()
                .is_none_or(|root| root >= actor.nodes.len())
        {
            return Err(invalid_actor_projection(
                &path,
                "Actor message type, SendableLocal class, root, owner, or span is invalid",
            ));
        }
        if actor.mailbox.span.source != actor.span.source {
            return Err(invalid_actor_projection(
                &format!("{path}.mailbox.span.source"),
                "Actor message and mailbox evidence must use the same source",
            ));
        }
        validate_actor_mailbox(&actor.mailbox, &format!("{path}.mailbox"))?;
        if actor.turn.span.source != actor.span.source
            || actor.turn.span.source != actor.mailbox.span.source
        {
            return Err(invalid_actor_projection(
                &format!("{path}.turn.span.source"),
                "Actor message, mailbox, and turn evidence must use the same source",
            ));
        }
        validate_actor_turn(actor, &format!("{path}.turn"))?;
        validate_actor_nodes(actor, &path)?;
        let expected = format!(
            "experimental:blake3:{}",
            blake3::hash(&actor_schema_bytes(actor.root, &actor.nodes)).to_hex()
        );
        if actor.schema_id != expected {
            return Err(invalid_actor_projection(
                &format!("{path}.schema_id"),
                "Actor message schema digest does not match its canonical graph",
            ));
        }
    }
    Ok(())
}

fn validate_actor_turn(actor: &SemanticActor, path: &str) -> Result<(), SemanticReadError> {
    let turn = &actor.turn;
    let receive_span = actor_turn_span(&turn.span)
        .ok_or_else(|| invalid_actor_projection(path, "Actor turn receive span is invalid"))?;
    let body_span = actor_turn_span(&turn.body_span)
        .ok_or_else(|| invalid_actor_projection(path, "Actor turn body span is invalid"))?;
    let expected = ActorTurnContract::new_checked_profile(ActorTurnSpec {
        actor: actor.definition_id.as_str().into(),
        actor_type: ActorTypeId::new(turn.actor_type),
        transition: turn.transition,
        state_binding: turn.state_binding,
        message_binding: turn.message_binding,
        receive_span,
        body_span,
    })
    .map_err(|error| invalid_actor_projection(path, &error.to_string()))?;
    if turn.dispatch != expected.dispatch().as_str()
        || turn.suspension != expected.suspension().as_str()
        || turn.reentry != expected.reentry().as_str()
        || turn.state_commit != expected.state_commit().as_str()
        || turn.self_send != expected.self_send().as_str()
    {
        return Err(invalid_actor_projection(
            path,
            "Actor turn modes do not match the accepted checked profile",
        ));
    }
    if turn.canonical_bytes != expected.canonical_bytes() {
        return Err(invalid_actor_projection(
            &format!("{path}.canonical_bytes"),
            "Actor turn canonical bytes do not match its owner, modes, or checked identities",
        ));
    }
    Ok(())
}

fn actor_turn_span(span: &SemanticByteSpan) -> Option<Span> {
    Span::new(
        SourceId::new(span.source),
        ByteOffset::new(span.start),
        ByteOffset::new(span.end),
    )
    .ok()
}

fn validate_actor_mailbox(
    mailbox: &SemanticActorMailbox,
    path: &str,
) -> Result<(), SemanticReadError> {
    let capacity = MailboxCapacity::new(mailbox.capacity).map_err(|_| {
        invalid_actor_projection(
            &format!("{path}.capacity"),
            "Actor mailbox capacity is outside the accepted range",
        )
    })?;
    if mailbox.overflow != MailboxOverflowPolicy::Reject.as_str() {
        return Err(invalid_actor_projection(
            &format!("{path}.overflow"),
            "Actor mailbox overflow policy is unsupported",
        ));
    }
    if mailbox.span.start >= mailbox.span.end
        || mailbox.capacity_span.start >= mailbox.capacity_span.end
        || mailbox.policy_span.start >= mailbox.policy_span.end
        || mailbox.span.source != mailbox.capacity_span.source
        || mailbox.span.source != mailbox.policy_span.source
        || mailbox.capacity_span.start <= mailbox.span.start
        || mailbox.capacity_span.end > mailbox.span.end
        || mailbox.policy_span.start <= mailbox.span.start
        || mailbox.policy_span.end > mailbox.span.end
        || mailbox.capacity_span.end >= mailbox.policy_span.start
    {
        return Err(invalid_actor_projection(
            path,
            "Actor mailbox source spans are invalid or inconsistent",
        ));
    }
    let expected = LocalMailboxContract::new(capacity, MailboxOverflowPolicy::Reject);
    if mailbox.canonical_bytes != expected.canonical_bytes() {
        return Err(invalid_actor_projection(
            &format!("{path}.canonical_bytes"),
            "Actor mailbox canonical bytes do not match capacity and policy",
        ));
    }
    Ok(())
}

fn validate_actor_nodes(actor: &SemanticActor, path: &str) -> Result<(), SemanticReadError> {
    let node_count = u32::try_from(actor.nodes.len()).unwrap_or(u32::MAX);
    for (node_index, node) in actor.nodes.iter().enumerate() {
        let node_path = format!("{path}.nodes[{node_index}]");
        if node.id != u32::try_from(node_index).unwrap_or(u32::MAX) {
            return Err(invalid_actor_projection(
                &node_path,
                "Actor message schema nodes must use contiguous canonical IDs",
            ));
        }
        let targets_valid = |targets: &[u32]| targets.iter().all(|target| *target < node_count);
        let shape_valid = match node.kind.as_str() {
            "primitive" => {
                matches!(
                    node.name.as_deref(),
                    Some("Unit" | "Bool" | "Int" | "Float64" | "Text")
                ) && node.definition.is_none()
                    && node.arguments.is_empty()
                    && node.elements.is_empty()
                    && node.element.is_none()
                    && node.fields.is_empty()
                    && node.cases.is_empty()
            }
            "tuple" => {
                node.name.is_none()
                    && node.definition.is_none()
                    && node.arguments.is_empty()
                    && !node.elements.is_empty()
                    && targets_valid(&node.elements)
                    && node.element.is_none()
                    && node.fields.is_empty()
                    && node.cases.is_empty()
            }
            "list" => {
                node.name.is_none()
                    && node.definition.is_none()
                    && node.arguments.is_empty()
                    && node.elements.is_empty()
                    && node.element.is_some_and(|element| element < node_count)
                    && node.fields.is_empty()
                    && node.cases.is_empty()
            }
            "record" => {
                node.name.is_none()
                    && node.definition.is_some()
                    && targets_valid(&node.arguments)
                    && node.elements.is_empty()
                    && node.element.is_none()
                    && node.cases.is_empty()
                    && strictly_sorted_actor_fields(&node.fields, node_count)
            }
            "variant" => {
                node.name.is_none()
                    && node.definition.is_some()
                    && targets_valid(&node.arguments)
                    && node.elements.is_empty()
                    && node.element.is_none()
                    && node.fields.is_empty()
                    && !node.cases.is_empty()
                    && strictly_sorted_actor_cases(&node.cases, node_count)
            }
            _ => false,
        };
        if !shape_valid {
            return Err(invalid_actor_projection(
                &node_path,
                "Actor message schema node kind, shape, order, or edge target is invalid",
            ));
        }
        if let Some(definition) = &node.definition {
            validate_id(definition, &format!("{node_path}.definition"))?;
        }
        for (case_index, case) in node.cases.iter().enumerate() {
            validate_id(
                &case.definition,
                &format!("{node_path}.cases[{case_index}].definition"),
            )?;
        }
    }
    Ok(())
}

fn strictly_sorted_actor_fields(fields: &[SemanticActorField], node_count: u32) -> bool {
    let mut previous: Option<&str> = None;
    fields.iter().all(|field| {
        let ordered = previous.is_none_or(|value| value < field.name.as_str());
        previous = Some(&field.name);
        ordered && !field.name.is_empty() && field.value < node_count
    })
}

fn strictly_sorted_actor_cases(cases: &[SemanticActorCase], node_count: u32) -> bool {
    let mut previous: Option<&str> = None;
    cases.iter().all(|case| {
        let ordered = previous.is_none_or(|value| value < case.name.as_str());
        previous = Some(&case.name);
        ordered && !case.name.is_empty() && case.payload.is_none_or(|payload| payload < node_count)
    })
}

fn invalid_actor_projection(path: &str, reason: &str) -> SemanticReadError {
    SemanticReadError {
        kind: SemanticReadErrorKind::InvalidActorProjection {
            reason: reason.to_owned(),
        },
        path: path.to_owned(),
    }
}

fn actor_schema_bytes(root: u32, nodes: &[SemanticActorSchemaNode]) -> Vec<u8> {
    let mut output = Vec::new();
    actor_push_text(&mut output, ling_types::ACTOR_MESSAGE_SCHEMA_DOMAIN);
    actor_push_text(&mut output, LANGUAGE_VERSION);
    actor_push_u32(&mut output, root);
    actor_push_u32(&mut output, u32::try_from(nodes.len()).unwrap_or(u32::MAX));
    for node in nodes {
        actor_push_u32(&mut output, node.id);
        match node.kind.as_str() {
            "primitive" => {
                output.push(0);
                actor_push_text(&mut output, node.name.as_deref().unwrap_or_default());
            }
            "tuple" => {
                output.push(1);
                actor_push_ids(&mut output, &node.elements);
            }
            "list" => {
                output.push(2);
                actor_push_u32(&mut output, node.element.unwrap_or(u32::MAX));
            }
            "record" => {
                output.push(3);
                actor_push_text(&mut output, node.definition.as_deref().unwrap_or_default());
                actor_push_ids(&mut output, &node.arguments);
                actor_push_u32(
                    &mut output,
                    u32::try_from(node.fields.len()).unwrap_or(u32::MAX),
                );
                for field in &node.fields {
                    actor_push_text(&mut output, &field.name);
                    actor_push_u32(&mut output, field.value);
                }
            }
            "variant" => {
                output.push(4);
                actor_push_text(&mut output, node.definition.as_deref().unwrap_or_default());
                actor_push_ids(&mut output, &node.arguments);
                actor_push_u32(
                    &mut output,
                    u32::try_from(node.cases.len()).unwrap_or(u32::MAX),
                );
                for case in &node.cases {
                    actor_push_text(&mut output, &case.definition);
                    actor_push_text(&mut output, &case.name);
                    match case.payload {
                        Some(payload) => {
                            output.push(1);
                            actor_push_u32(&mut output, payload);
                        }
                        None => output.push(0),
                    }
                }
            }
            _ => output.push(u8::MAX),
        }
    }
    output
}

fn actor_push_ids(output: &mut Vec<u8>, values: &[u32]) {
    actor_push_u32(output, u32::try_from(values.len()).unwrap_or(u32::MAX));
    for value in values {
        actor_push_u32(output, *value);
    }
}

fn actor_push_text(output: &mut Vec<u8>, value: &str) {
    actor_push_u32(output, u32::try_from(value.len()).unwrap_or(u32::MAX));
    output.extend_from_slice(value.as_bytes());
}

fn actor_push_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_le_bytes());
}

fn validate_trait_ide(graph: &SemanticGraph) -> Result<(), SemanticReadError> {
    let projection = graph.trait_ide.as_ref();
    let Some(projection) = projection else {
        return Ok(());
    };
    if projection.version != TRAIT_IDE_EXTENSION_VERSION {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::InvalidTraitIdeVersion {
                version: projection.version.clone(),
            },
            path: "$.x-ling-trait-ide.version".to_owned(),
        });
    }
    let mut witness_ordinals = BTreeSet::new();
    let mut previous_witness = None;
    for (witness_index, witness) in projection.witnesses.iter().enumerate() {
        validate_id(
            &witness.trait_id,
            &format!("$.x-ling-trait-ide.witnesses[{witness_index}].trait_id"),
        )?;
        validate_id(
            &witness.implementation_id,
            &format!("$.x-ling-trait-ide.witnesses[{witness_index}].implementation_id"),
        )?;
        if witness.trait_name.is_empty()
            || witness.trait_module.is_empty()
            || witness.receiver.is_empty()
            || witness.implementation_module.is_empty()
        {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::InvalidTraitIdeIdentity {
                    value: witness.trait_name.clone(),
                },
                path: format!("$.x-ling-trait-ide.witnesses[{witness_index}]"),
            });
        }
        if !witness_ordinals.insert(witness.obligation_order) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DuplicateTraitIdeOrdinal {
                    ordinal: witness.obligation_order,
                },
                path: format!("$.x-ling-trait-ide.witnesses[{witness_index}].obligation_order"),
            });
        }
        if previous_witness.is_some_and(|previous| previous > witness.obligation_order) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::UnsortedTraitIdeOrdinal {
                    ordinal: witness.obligation_order,
                },
                path: format!("$.x-ling-trait-ide.witnesses[{witness_index}].obligation_order"),
            });
        }
        previous_witness = Some(witness.obligation_order);

        let mut member_ordinals = BTreeSet::new();
        let mut previous_member = None;
        for (member_index, member) in witness.members.iter().enumerate() {
            validate_id(
                &member.trait_definition_id,
                &format!(
                    "$.x-ling-trait-ide.witnesses[{witness_index}].members[{member_index}].trait_definition_id"
                ),
            )?;
            validate_id(
                &member.implementation_definition_id,
                &format!(
                    "$.x-ling-trait-ide.witnesses[{witness_index}].members[{member_index}].implementation_definition_id"
                ),
            )?;
            if !graph
                .definitions
                .iter()
                .any(|definition| definition.definition_id == member.trait_definition_id)
                || !graph.definitions.iter().any(|definition| {
                    definition.definition_id == member.implementation_definition_id
                })
            {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::TraitIdeMismatch {
                        value: member.name.clone(),
                    },
                    path: format!(
                        "$.x-ling-trait-ide.witnesses[{witness_index}].members[{member_index}]"
                    ),
                });
            }
            if member.name.is_empty()
                || member.trait_source.is_empty()
                || member.implementation_source.is_empty()
                || member.trait_span.start > member.trait_span.end
                || member.implementation_span.start > member.implementation_span.end
            {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::InvalidTraitIdeSpan,
                    path: format!(
                        "$.x-ling-trait-ide.witnesses[{witness_index}].members[{member_index}]"
                    ),
                });
            }
            if !member_ordinals.insert(member.ordinal) {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::DuplicateTraitIdeOrdinal {
                        ordinal: member.ordinal,
                    },
                    path: format!(
                        "$.x-ling-trait-ide.witnesses[{witness_index}].members[{member_index}].ordinal"
                    ),
                });
            }
            if previous_member.is_some_and(|previous| previous > member.ordinal) {
                return Err(SemanticReadError {
                    kind: SemanticReadErrorKind::UnsortedTraitIdeOrdinal {
                        ordinal: member.ordinal,
                    },
                    path: format!(
                        "$.x-ling-trait-ide.witnesses[{witness_index}].members[{member_index}].ordinal"
                    ),
                });
            }
            previous_member = Some(member.ordinal);
        }
        if witness.members.is_empty() {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::TraitIdeMismatch {
                    value: witness.trait_id.clone(),
                },
                path: format!("$.x-ling-trait-ide.witnesses[{witness_index}].members"),
            });
        }
    }
    Ok(())
}

type ModuleCoordinate = (Option<SemanticPackageIdentity>, String);

fn validate_project_context(
    graph: &SemanticGraph,
    mode: IdentityMode,
) -> Result<BTreeSet<SemanticPackageIdentity>, SemanticReadError> {
    if mode == IdentityMode::Seed {
        return Ok(BTreeSet::new());
    }

    let graph_id = graph
        .package_graph_id
        .as_deref()
        .ok_or_else(|| missing_project_field("package_graph_id", "$.package_graph_id"))?;
    validate_sha256_id(graph_id, "$.package_graph_id")?;
    let root = graph
        .root_package
        .as_ref()
        .ok_or_else(|| missing_project_field("root_package", "$.root_package"))?;
    validate_package_identity(root, "$.root_package")?;
    if graph.packages.is_empty() {
        return Err(missing_project_field("packages", "$.packages"));
    }

    let mut identities = BTreeSet::new();
    let mut names = BTreeSet::new();
    for (index, package) in graph.packages.iter().enumerate() {
        validate_package_identity(&package.identity, &format!("$.packages[{index}].identity"))?;
        if !names.insert(package.identity.name.clone())
            || !identities.insert(package.identity.clone())
        {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::DuplicatePackage {
                    package: package.identity.name.clone(),
                },
                path: format!("$.packages[{index}].identity"),
            });
        }
    }
    if !identities.contains(root) {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::UnknownPackage {
                package: package_label(root),
            },
            path: "$.root_package".to_owned(),
        });
    }
    Ok(identities)
}

fn missing_project_field(field: &str, path: &str) -> SemanticReadError {
    SemanticReadError {
        kind: SemanticReadErrorKind::MissingProjectField {
            field: field.to_owned(),
        },
        path: path.to_owned(),
    }
}

fn required_module_coordinate(
    package: Option<&SemanticPackageIdentity>,
    module: &str,
    path: &str,
    mode: IdentityMode,
    packages: &BTreeSet<SemanticPackageIdentity>,
) -> Result<ModuleCoordinate, SemanticReadError> {
    if mode == IdentityMode::Project && package.is_none() {
        return Err(missing_project_field("package", path));
    }
    optional_module_coordinate(package, module, path, mode, packages)
}

fn optional_module_coordinate(
    package: Option<&SemanticPackageIdentity>,
    module: &str,
    path: &str,
    mode: IdentityMode,
    packages: &BTreeSet<SemanticPackageIdentity>,
) -> Result<ModuleCoordinate, SemanticReadError> {
    if mode == IdentityMode::Seed && package.is_some() {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::PackageCoordinateMismatch {
                entity: module.to_owned(),
            },
            path: path.to_owned(),
        });
    }
    if let Some(package) = package {
        validate_package_identity(package, path)?;
        if !packages.contains(package) {
            return Err(SemanticReadError {
                kind: SemanticReadErrorKind::UnknownPackage {
                    package: package_label(package),
                },
                path: path.to_owned(),
            });
        }
    }
    Ok((package.cloned(), module.to_owned()))
}

fn system_coordinate(
    package: Option<&SemanticPackageIdentity>,
    module: &str,
    path: &str,
) -> Result<ModuleCoordinate, SemanticReadError> {
    if package.is_some() {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::PackageCoordinateMismatch {
                entity: module.to_owned(),
            },
            path: path.to_owned(),
        });
    }
    Ok((None, module.to_owned()))
}

fn validate_package_module(
    modules: &BTreeSet<ModuleCoordinate>,
    package: &SemanticPackageIdentity,
    module: &str,
    path: &str,
) -> Result<(), SemanticReadError> {
    if modules.contains(&(Some(package.clone()), module.to_owned())) {
        return Ok(());
    }
    Err(SemanticReadError {
        kind: SemanticReadErrorKind::UnknownPackageModule {
            package: package_label(package),
            module: module.to_owned(),
        },
        path: path.to_owned(),
    })
}

fn unknown_module_kind(coordinate: &ModuleCoordinate) -> SemanticReadErrorKind {
    match &coordinate.0 {
        Some(package) => SemanticReadErrorKind::UnknownPackageModule {
            package: package_label(package),
            module: coordinate.1.clone(),
        },
        None => SemanticReadErrorKind::UnknownModule {
            module: coordinate.1.clone(),
        },
    }
}

fn package_label(package: &SemanticPackageIdentity) -> String {
    format!("{}@{}#{}", package.name, package.version, package.source)
}

fn validate_package_identity(
    package: &SemanticPackageIdentity,
    path: &str,
) -> Result<(), SemanticReadError> {
    if !valid_package_name(&package.name)
        || !valid_package_version(&package.version)
        || !valid_sha256_id(&package.source)
    {
        return Err(SemanticReadError {
            kind: SemanticReadErrorKind::InvalidPackageIdentity {
                value: package_label(package),
            },
            path: path.to_owned(),
        });
    }
    Ok(())
}

fn valid_package_name(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !(1..=63).contains(&bytes.len()) || !bytes[0].is_ascii_lowercase() {
        return false;
    }
    let mut previous_hyphen = false;
    for byte in &bytes[1..] {
        if *byte == b'-' {
            if previous_hyphen {
                return false;
            }
            previous_hyphen = true;
        } else if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
            previous_hyphen = false;
        } else {
            return false;
        }
    }
    !previous_hyphen
}

fn valid_package_version(value: &str) -> bool {
    let components = value.split('.').collect::<Vec<_>>();
    components.len() == 3
        && components.iter().all(|component| {
            !component.is_empty()
                && (component == &"0" || !component.starts_with('0'))
                && component.bytes().all(|byte| byte.is_ascii_digit())
                && component.parse::<u32>().is_ok()
        })
}

fn valid_sha256_id(value: &str) -> bool {
    let Some(digest) = value.strip_prefix("sha256:") else {
        return false;
    };
    digest.len() == 64
        && digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn validate_sha256_id(value: &str, path: &str) -> Result<(), SemanticReadError> {
    if valid_sha256_id(value) {
        return Ok(());
    }
    Err(SemanticReadError {
        kind: SemanticReadErrorKind::InvalidPackageIdentity {
            value: value.to_owned(),
        },
        path: path.to_owned(),
    })
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
    mode: IdentityMode,
}

impl SnapshotBuilder {
    const fn new(checked: CheckedProgram, mode: IdentityMode) -> Self {
        Self { checked, mode }
    }

    fn build(self) -> Result<ProgramSnapshot, SnapshotError> {
        let body_ids = self.body_ids();
        let program_id = self.program_id(&body_ids);
        let modules = self.modules();
        let definitions = self.definitions(&body_ids);
        let nodes = self.nodes();
        let references = self.references();
        let trait_ide = self.trait_ide();
        let task = self.task_projection();
        let actor = self.actor_projection();
        let graph = SemanticGraph {
            schema: SEMANTIC_SCHEMA.to_owned(),
            language_version: LANGUAGE_VERSION.to_owned(),
            unicode_version: ling_unicode::UNICODE_VERSION.to_string(),
            program_id: program_id.to_string(),
            package_graph_id: None,
            root_package: None,
            packages: Vec::new(),
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
            trait_ide,
            task,
            actor,
        };
        let json = serde_json::to_string(&graph).map_err(SnapshotError::Serialization)?;
        Ok(ProgramSnapshot {
            checked: self.checked,
            graph,
            body_ids,
            program_id,
            json,
        })
    }

    fn build_project(self) -> Result<ProjectProgramSnapshot, ProjectSnapshotError> {
        let body_ids = self.body_ids();
        let program_id = self.program_id(&body_ids);
        let modules = self.modules();
        let definitions = self.definitions(&body_ids);
        let nodes = self.nodes();
        let references = self.references();
        let trait_ide = self.trait_ide();
        let task = self.task_projection();
        let actor = self.actor_projection();
        let project = self
            .checked
            .typed()
            .resolved()
            .project()
            .expect("build_project validates package context");
        let packages = project
            .packages()
            .iter()
            .map(|package| SemanticPackage {
                identity: semantic_package_identity(package.identity()),
                entry_module: package.entry().as_str().to_owned(),
                exports: package
                    .exports()
                    .iter()
                    .map(|module| module.as_str().to_owned())
                    .collect(),
            })
            .collect();
        let graph = SemanticGraph {
            schema: PROJECT_SEMANTIC_SCHEMA.to_owned(),
            language_version: LANGUAGE_VERSION.to_owned(),
            unicode_version: ling_unicode::UNICODE_VERSION.to_string(),
            program_id: program_id.to_string(),
            package_graph_id: Some(project.graph_id().as_str().to_owned()),
            root_package: Some(semantic_package_identity(project.root())),
            packages,
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
            trait_ide,
            task,
            actor,
        };
        let json = serde_json::to_string(&graph).map_err(ProjectSnapshotError::Serialization)?;
        Ok(ProjectProgramSnapshot {
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
            .iter()
            .filter(|(_, info)| {
                self.mode == IdentityMode::Seed || info.kind != DefinitionKind::Actor
            })
            .map(|(definition, _)| {
                let mut encoder = Encoder::new(self.mode.body_domain());
                encoder.string(LANGUAGE_VERSION);
                encoder.string(self.mode.schema());
                self.encode_definition(definition, &mut encoder);
                (definition.clone(), BodyId(hash(encoder.finish())))
            })
            .collect()
    }

    fn program_id(&self, body_ids: &BTreeMap<DefinitionId, BodyId>) -> ProgramId {
        let mut encoder = Encoder::new(self.mode.program_domain());
        encoder.string(LANGUAGE_VERSION);
        encoder.string(self.mode.schema());
        encoder.string(&ling_unicode::UNICODE_VERSION.to_string());
        let dictionary = self.checked.typed().dictionary();
        if !dictionary.witnesses().is_empty() {
            encoder.bytes(&dictionary.canonical_bytes());
        }
        if self.mode == IdentityMode::Project {
            let project = self
                .checked
                .typed()
                .resolved()
                .project()
                .expect("project identity mode requires project metadata");
            encoder.string(project.graph_id().as_str());
            encode_package_identity_to_encoder(project.root(), &mut encoder);
            encoder.u32(u32::try_from(project.packages().len()).unwrap_or(u32::MAX));
            for package in project.packages() {
                encode_package_identity_to_encoder(package.identity(), &mut encoder);
                encoder.string(package.entry().as_str());
                encoder.u32(u32::try_from(package.exports().len()).unwrap_or(u32::MAX));
                for export in package.exports() {
                    encoder.string(export.as_str());
                }
            }
        }
        encoder.u32(u32::try_from(body_ids.len()).unwrap_or(u32::MAX));
        for (definition, body) in body_ids {
            encoder.string(definition.as_str());
            encoder.string(body.as_str());
        }
        ProgramId(hash(encoder.finish()))
    }

    fn trait_ide(&self) -> Option<SemanticTraitIdeProjection> {
        let typed = self.checked.typed();
        let dictionary = typed.dictionary();
        if dictionary.witnesses().is_empty() {
            return None;
        }

        let resolved = typed.resolved();
        let mut witnesses = Vec::with_capacity(dictionary.witnesses().len());
        for witness in dictionary.witnesses() {
            let trait_module = resolved
                .module(witness.trait_module())
                .expect("checked Trait witness references an existing Trait module");
            let trait_module_name = trait_module.hir.module.name.normalized();
            let trait_name = witness.trait_name().to_owned();
            let package = trait_module.package.as_ref();
            let receiver = witness.receiver();
            let mut members = Vec::with_capacity(witness.members().len());
            let mut trait_definition_ids = Vec::with_capacity(witness.members().len());
            let mut implementation_definition_ids = Vec::with_capacity(witness.members().len());

            for member in witness.members() {
                let trait_member = resolved
                    .trait_members()
                    .values()
                    .find(|candidate| {
                        candidate.module == witness.trait_module()
                            && candidate.trait_name == trait_name
                            && candidate.member_name == member.name()
                            && candidate.ordinal == member.ordinal()
                    })
                    .expect("checked Trait witness references an existing Trait member");
                let implementation_member = resolved
                    .impl_member(member.definition())
                    .expect("checked Trait witness references an existing implementation member");
                let ordinal = u32::try_from(member.ordinal()).unwrap_or(u32::MAX);
                trait_definition_ids.push(trait_member.definition.as_str().to_owned());
                implementation_definition_ids.push(member.definition().as_str().to_owned());
                members.push(SemanticTraitMember {
                    ordinal,
                    name: member.name().to_owned(),
                    trait_definition_id: trait_member.definition.as_str().to_owned(),
                    implementation_definition_id: member.definition().as_str().to_owned(),
                    trait_source: trait_member.source_name.clone(),
                    trait_span: semantic_byte_span(trait_member.span),
                    implementation_source: implementation_member.source_name.clone(),
                    implementation_span: semantic_byte_span(implementation_member.span),
                });
            }

            let trait_id = trait_ide_id(
                self.mode,
                package,
                &trait_module_name,
                &trait_name,
                &trait_definition_ids,
            );
            let first_implementation_definition = witness
                .members()
                .first()
                .expect("checked Trait witness contains at least one member")
                .definition();
            let implementation_member = resolved
                .impl_member(first_implementation_definition)
                .expect("checked Trait witness references an existing implementation member");
            let implementation_module = resolved
                .module(implementation_member.module)
                .expect("checked implementation member references an existing module");
            let implementation_module_name = implementation_module.hir.module.name.normalized();
            let implementation_id = implementation_ide_id(
                self.mode,
                resolved
                    .module(implementation_member.module)
                    .and_then(|module| module.package.as_ref()),
                &implementation_module_name,
                &trait_id,
                &receiver,
                &implementation_definition_ids,
            );
            let obligation_order = u32::try_from(witness.obligation_order()).unwrap_or(u32::MAX);
            witnesses.push(SemanticTraitWitness {
                trait_id,
                trait_name,
                trait_module: trait_module_name,
                receiver,
                implementation_id,
                implementation_module: implementation_module_name,
                obligation_order,
                members,
            });
        }

        witnesses.sort_by_key(|witness| witness.obligation_order);
        Some(SemanticTraitIdeProjection {
            version: TRAIT_IDE_EXTENSION_VERSION.to_owned(),
            witnesses,
        })
    }

    fn task_projection(&self) -> Option<SemanticTaskProjection> {
        if self.checked.task_cores().is_empty() {
            return None;
        }
        let typed = self.checked.typed();
        let tasks = self
            .checked
            .task_cores()
            .values()
            .map(|core| {
                let type_id = typed
                    .definition_type(core.definition())
                    .expect("checked Task Core has a checked signature");
                SemanticTask {
                    definition_id: core.definition().to_string(),
                    signature: format!(
                        "{} ! {{{}}}",
                        typed.display_type(type_id),
                        core.signature().effects().canonical_names().join(", ")
                    ),
                    effects: core.signature().effects().canonical_names(),
                    root_scope: core.root_scope().get(),
                    result_body: core.result_body().local().get(),
                    span: semantic_byte_span(core.source_span()),
                    scopes: core
                        .scopes()
                        .iter()
                        .map(|scope| SemanticTaskScope {
                            id: scope.id().get(),
                            parent: scope.parent().map(|parent| parent.get()),
                            span: semantic_byte_span(scope.span()),
                        })
                        .collect(),
                    spawns: core
                        .spawns()
                        .iter()
                        .map(|spawn| SemanticTaskSpawn {
                            task: spawn.task().get(),
                            scope: spawn.scope().get(),
                            parent: spawn.parent().get(),
                            target: spawn.target().to_string(),
                            cancellation: spawn.cancellation().get(),
                            cleanup: spawn.cleanup().get(),
                            syntax: spawn.syntax().as_str().to_owned(),
                            span: semantic_byte_span(spawn.span()),
                            operator_span: semantic_byte_span(spawn.operator_span()),
                            pattern_span: spawn.pattern_span().map(semantic_byte_span),
                            target_span: semantic_byte_span(spawn.target_span()),
                            call_span: semantic_byte_span(spawn.call_span()),
                        })
                        .collect(),
                    suspensions: core
                        .suspensions()
                        .iter()
                        .map(|suspension| SemanticTaskSuspension {
                            id: suspension.id().get(),
                            scope: suspension.scope().get(),
                            awaited_task: suspension.awaited_task().get(),
                            live: suspension
                                .live()
                                .iter()
                                .map(|live| SemanticTaskLiveBinding {
                                    binding: live.binding().local().get(),
                                    type_name: typed.display_type(live.value_type()),
                                })
                                .collect(),
                            span: semantic_byte_span(suspension.span()),
                        })
                        .collect(),
                }
            })
            .collect();
        Some(SemanticTaskProjection {
            version: TASK_GRAPH_EXTENSION_VERSION.to_owned(),
            tasks,
        })
    }

    fn actor_projection(&self) -> Option<SemanticActorProjection> {
        if self.mode != IdentityMode::Seed || self.checked.actor_cores().is_empty() {
            return None;
        }
        let typed = self.checked.typed();
        let actors = self
            .checked
            .actor_cores()
            .values()
            .map(|core| {
                let contract = core.message_contract();
                let nodes = contract
                    .schema()
                    .nodes()
                    .iter()
                    .map(|node| {
                        let mut semantic = SemanticActorSchemaNode {
                            id: node.id,
                            kind: String::new(),
                            name: None,
                            definition: None,
                            arguments: Vec::new(),
                            elements: Vec::new(),
                            element: None,
                            fields: Vec::new(),
                            cases: Vec::new(),
                        };
                        match &node.kind {
                            ling_types::ActorMessageSchemaNodeKind::Primitive { name } => {
                                semantic.kind = "primitive".to_owned();
                                semantic.name = Some((*name).to_owned());
                            }
                            ling_types::ActorMessageSchemaNodeKind::Tuple { elements } => {
                                semantic.kind = "tuple".to_owned();
                                semantic.elements.clone_from(elements);
                            }
                            ling_types::ActorMessageSchemaNodeKind::List { element } => {
                                semantic.kind = "list".to_owned();
                                semantic.element = Some(*element);
                            }
                            ling_types::ActorMessageSchemaNodeKind::Record {
                                definition,
                                arguments,
                                fields,
                            } => {
                                semantic.kind = "record".to_owned();
                                semantic.definition = Some(definition.to_string());
                                semantic.arguments.clone_from(arguments);
                                semantic.fields = fields
                                    .iter()
                                    .map(|field| SemanticActorField {
                                        name: field.name.clone(),
                                        value: field.value,
                                    })
                                    .collect();
                            }
                            ling_types::ActorMessageSchemaNodeKind::Variant {
                                definition,
                                arguments,
                                cases,
                            } => {
                                semantic.kind = "variant".to_owned();
                                semantic.definition = Some(definition.to_string());
                                semantic.arguments.clone_from(arguments);
                                semantic.cases = cases
                                    .iter()
                                    .map(|case| SemanticActorCase {
                                        definition: case.definition.to_string(),
                                        name: case.name.clone(),
                                        payload: case.payload,
                                    })
                                    .collect();
                            }
                        }
                        semantic
                    })
                    .collect();
                SemanticActor {
                    definition_id: core.definition().to_string(),
                    message_type: typed.display_type(core.message_type()),
                    sendability: contract.sendability().as_str().to_owned(),
                    schema_id: contract.schema().id().as_str().to_owned(),
                    root: contract.schema().root(),
                    span: semantic_byte_span(contract.source_span()),
                    mailbox: {
                        let mailbox = core.mailbox_contract();
                        SemanticActorMailbox {
                            capacity: mailbox.mailbox().capacity().get(),
                            overflow: mailbox.mailbox().overflow().as_str().to_owned(),
                            canonical_bytes: mailbox.mailbox().canonical_bytes().to_vec(),
                            span: semantic_byte_span(mailbox.source_span()),
                            capacity_span: semantic_byte_span(mailbox.capacity_span()),
                            policy_span: semantic_byte_span(mailbox.policy_span()),
                        }
                    },
                    turn: {
                        let turn = core.turn_contract();
                        SemanticActorTurn {
                            actor_type: turn.actor_type().get(),
                            dispatch: turn.dispatch().as_str().to_owned(),
                            suspension: turn.suspension().as_str().to_owned(),
                            reentry: turn.reentry().as_str().to_owned(),
                            state_commit: turn.state_commit().as_str().to_owned(),
                            self_send: turn.self_send().as_str().to_owned(),
                            transition: turn.transition(),
                            state_binding: turn.state_binding(),
                            message_binding: turn.message_binding(),
                            canonical_bytes: turn.canonical_bytes().to_vec(),
                            span: semantic_byte_span(turn.receive_span()),
                            body_span: semantic_byte_span(turn.body_span()),
                        }
                    },
                    nodes,
                }
            })
            .collect();
        Some(SemanticActorProjection {
            version: ACTOR_GRAPH_EXTENSION_VERSION.to_owned(),
            actors,
        })
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
                            package: target.package.as_ref().map(semantic_package_identity),
                            module: target.hir.module.name.normalized(),
                        })
                    })
                    .collect::<Vec<_>>();
                imports.sort_by(|left, right| {
                    (&left.alias, &left.package, &left.module).cmp(&(
                        &right.alias,
                        &right.package,
                        &right.module,
                    ))
                });
                SemanticModule {
                    package: module.package.as_ref().map(semantic_package_identity),
                    name: module.hir.module.name.normalized(),
                    explicit: module.hir.module.explicit,
                    requires,
                    imports,
                }
            })
            .collect::<Vec<_>>();
        modules
            .sort_by(|left, right| (&left.package, &left.name).cmp(&(&right.package, &right.name)));
        modules
    }

    fn definitions(&self, body_ids: &BTreeMap<DefinitionId, BodyId>) -> Vec<SemanticDefinition> {
        let typed = self.checked.typed();
        typed
            .resolved()
            .definitions()
            .iter()
            .filter(|(_, definition)| {
                self.mode == IdentityMode::Seed || definition.kind != DefinitionKind::Actor
            })
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
                    package: definition.package.as_ref().map(semantic_package_identity),
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
                    self.mode,
                    definition.package.as_ref(),
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
                    self.mode,
                    definition.package.as_ref(),
                    "function",
                    &definition.module_name,
                    &format!("definition:{definition_id}"),
                );
                let effects = function_type.effects().names();
                let capabilities = capabilities_for_effects(&effects);
                nodes.push(SemanticNode {
                    node_id: function_id.clone(),
                    package: definition.package.as_ref().map(semantic_package_identity),
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
                            self.mode,
                            definition.package.as_ref(),
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
                        self.mode,
                        definition.package.as_ref(),
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
                        self.mode,
                        definition.package.as_ref(),
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
            let context = NodeContext {
                mode: self.mode,
                package: module.package.as_ref(),
                module: module.id,
                module_name: &module_name,
            };
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
                                self.mode,
                                module.package.as_ref(),
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
                                self.mode,
                                module.package.as_ref(),
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
                    self.mode,
                    module.package.as_ref(),
                    "function",
                    &module_name,
                    &format!("definition:{definition_id}"),
                );
                for (ordinal, parameter) in definition.parameters.iter().enumerate() {
                    add_parameter_and_pattern_nodes(
                        &mut nodes,
                        typed,
                        context,
                        &function_id,
                        ordinal,
                        parameter,
                    );
                }
                add_expression_nodes(
                    &mut nodes,
                    &self.checked,
                    context,
                    definition_id.as_str(),
                    &definition.value,
                );
            }
            for task in &module.hir.tasks {
                let Some(definition_id) = resolved.definition_id(module.id, &task.name.normalized)
                else {
                    continue;
                };
                let task_owner = semantic_node_id(
                    self.mode,
                    module.package.as_ref(),
                    "task",
                    &module_name,
                    &format!("definition:{definition_id}"),
                );
                nodes.push(semantic_node(
                    self.mode,
                    module.package.as_ref(),
                    "task",
                    &module_name,
                    &format!("definition:{definition_id}"),
                    Some(&task.name.normalized),
                    definition_id.as_str(),
                    typed
                        .definition_type(definition_id)
                        .map(|type_id| typed.display_type(type_id)),
                    None,
                    None,
                    self.checked
                        .definition_effect(definition_id)
                        .map_or_else(Vec::new, ling_effects::EffectRow::names),
                    Vec::new(),
                    identifier_metadata(&task.name),
                ));
                for (ordinal, parameter) in task.parameters.iter().enumerate() {
                    add_parameter_and_pattern_nodes(
                        &mut nodes,
                        typed,
                        context,
                        &task_owner,
                        ordinal,
                        parameter,
                    );
                }
                add_expression_nodes(&mut nodes, &self.checked, context, &task_owner, &task.body);
            }
            for (impl_ordinal, implementation) in module.hir.impls.iter().enumerate() {
                for (member_ordinal, member) in implementation.members.iter().enumerate() {
                    let Some(member_id) = resolved
                        .impl_members()
                        .values()
                        .find(|candidate| {
                            candidate.module == module.id
                                && candidate.impl_ordinal == impl_ordinal
                                && candidate.member_ordinal == member_ordinal
                                && candidate.member_name == member.name.normalized
                        })
                        .map(|candidate| candidate.definition.clone())
                    else {
                        continue;
                    };
                    let function_id = semantic_node_id(
                        self.mode,
                        module.package.as_ref(),
                        "function",
                        &module_name,
                        &format!("definition:{member_id}"),
                    );
                    for (ordinal, parameter) in member.parameters.iter().enumerate() {
                        add_parameter_and_pattern_nodes(
                            &mut nodes,
                            typed,
                            context,
                            &function_id,
                            ordinal,
                            parameter,
                        );
                    }
                    add_expression_nodes(
                        &mut nodes,
                        &self.checked,
                        context,
                        member_id.as_str(),
                        &member.value,
                    );
                }
            }
        }
        nodes.sort_by(|left, right| left.node_id.cmp(&right.node_id));
        nodes
    }

    fn references(&self) -> Vec<SemanticReference> {
        let resolved = self.checked.typed().resolved();
        let source_ids = reference_source_ids(resolved, self.mode);
        let mut references = resolved
            .references()
            .iter()
            .filter(|(key, _)| source_ids.contains_key(&(key.module(), key.local())))
            .filter_map(|(key, target)| {
                resolved.module(key.module()).map(|module| {
                    let (target_kind, target) = match target {
                        ReferenceTarget::Definition(definition) => {
                            ("definition".to_owned(), definition.to_string())
                        }
                        ReferenceTarget::Binding(binding) => {
                            let binding_module = resolved
                                .module(binding.module())
                                .expect("resolved binding module exists");
                            (
                                "binding".to_owned(),
                                semantic_node_id(
                                    self.mode,
                                    binding_module.package.as_ref(),
                                    "binding",
                                    &binding_module.hir.module.name.normalized(),
                                    &format!("local:{}", binding.local().get()),
                                ),
                            )
                        }
                    };
                    SemanticReference {
                        package: module.package.as_ref().map(semantic_package_identity),
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
                            package: module.package.as_ref().map(semantic_package_identity),
                            module: module.hir.module.name.normalized(),
                            source_kind: "pattern".to_owned(),
                            reference: key.local().get(),
                            source_id: Some(semantic_node_id(
                                self.mode,
                                module.package.as_ref(),
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
                &left.package,
                &left.module,
                &left.source_kind,
                left.reference,
                &left.target_kind,
                &left.target,
            )
                .cmp(&(
                    &right.package,
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
                } else if let Some(task) = resolved_module
                    .hir
                    .tasks
                    .iter()
                    .find(|task| task.name.normalized == info.name)
                {
                    encoder.u8(4);
                    encoder.u32(u32::try_from(task.parameters.len()).unwrap_or(u32::MAX));
                    for pattern in &task.parameters {
                        self.encode_pattern(module, pattern, encoder);
                    }
                    self.encode_expression(module, &task.body, encoder);
                } else if let Some(actor) = self.checked.actor_core(definition) {
                    encoder.u8(5);
                    encoder.bytes(actor.canonical_bytes());
                } else if let Some(member) = typed.resolved().impl_member(definition) {
                    let value = resolved_module
                        .hir
                        .impls
                        .get(member.impl_ordinal)
                        .and_then(|implementation| {
                            implementation.members.get(member.member_ordinal)
                        })
                        .expect("implementation member body exists");
                    encoder.u8(3);
                    encoder.u32(u32::try_from(member.impl_ordinal).unwrap_or(u32::MAX));
                    encoder.u32(u32::try_from(member.member_ordinal).unwrap_or(u32::MAX));
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
                        hir::SequenceElement::LetAwait(binding) => {
                            encoder.u8(2);
                            self.encode_pattern(module, &binding.pattern, encoder);
                            self.encode_expression(module, &binding.call, encoder);
                        }
                    }
                }
            }
            hir::ExpressionKind::TaskScope { body, .. } => {
                encoder.u8(16);
                self.encode_expression(module, body, encoder);
            }
            hir::ExpressionKind::TaskSpawn { call, .. } => {
                encoder.u8(17);
                self.encode_expression(module, call, encoder);
            }
            hir::ExpressionKind::TaskAwait { handle, .. } => {
                encoder.u8(18);
                self.encode_expression(module, handle, encoder);
            }
            hir::ExpressionKind::TaskReturn { value, .. } => {
                encoder.u8(19);
                self.encode_expression(module, value, encoder);
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
            hir::ExpressionKind::Handle { body, clauses } => {
                encoder.u8(15);
                self.encode_expression(module, body, encoder);
                encoder.u32(u32::try_from(clauses.len()).unwrap_or(u32::MAX));
                for clause in clauses {
                    encoder.string(&clause.operation.normalized());
                    encoder.u32(u32::try_from(clause.parameters.len()).unwrap_or(u32::MAX));
                    for parameter in &clause.parameters {
                        self.encode_pattern(module, parameter, encoder);
                    }
                    encoder.bool(clause.resume.is_some());
                    if let Some(resume) = &clause.resume {
                        encoder.u32(resume.id.get());
                    }
                    self.encode_expression(module, &clause.body, encoder);
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
    mode: IdentityMode,
    package: Option<&PackageIdentity>,
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
        node_id: semantic_node_id(mode, package, kind, module, identity),
        package: package.map(semantic_package_identity),
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

fn semantic_node_id(
    mode: IdentityMode,
    package: Option<&PackageIdentity>,
    kind: &str,
    module: &str,
    identity: &str,
) -> String {
    let mut encoder = Encoder::new(mode.node_domain());
    encoder.string(LANGUAGE_VERSION);
    encoder.string(mode.schema());
    if mode == IdentityMode::Project {
        match package {
            Some(package) => {
                encoder.string("package");
                encode_package_identity_to_encoder(package, &mut encoder);
            }
            None => encoder.string("system"),
        }
    }
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

#[derive(Clone, Copy)]
struct NodeContext<'a> {
    mode: IdentityMode,
    package: Option<&'a PackageIdentity>,
    module: ModuleId,
    module_name: &'a str,
}

fn add_parameter_and_pattern_nodes(
    nodes: &mut Vec<SemanticNode>,
    typed: &ling_types::TypedProgram,
    context: NodeContext<'_>,
    function_id: &str,
    ordinal: usize,
    pattern: &hir::Pattern,
) {
    let name = pattern_binding_name(typed.resolved(), context.module, pattern);
    let type_name = pattern_binding_type(typed, context.module, pattern);
    let parameter_id = semantic_node_id(
        context.mode,
        context.package,
        "parameter",
        context.module_name,
        &format!("local:{}", pattern.id.get()),
    );
    nodes.push(semantic_node(
        context.mode,
        context.package,
        "parameter",
        context.module_name,
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
    add_pattern_nodes(nodes, typed, context, &parameter_id, pattern);
}

fn add_pattern_nodes(
    nodes: &mut Vec<SemanticNode>,
    typed: &ling_types::TypedProgram,
    context: NodeContext<'_>,
    owner: &str,
    pattern: &hir::Pattern,
) {
    let resolved = typed.resolved();
    let name = pattern_name(pattern);
    let pattern_id = semantic_node_id(
        context.mode,
        context.package,
        "pattern",
        context.module_name,
        &format!("local:{}", pattern.id.get()),
    );
    nodes.push(semantic_node(
        context.mode,
        context.package,
        "pattern",
        context.module_name,
        &format!("local:{}", pattern.id.get()),
        name.map(|name| name.normalized.as_str()),
        owner,
        pattern_binding_type(typed, context.module, pattern),
        None,
        Some(pattern.id.get()),
        Vec::new(),
        Vec::new(),
        name.map_or_else(IdentifierMetadata::default, identifier_metadata),
    ));
    if let hir::PatternKind::Binding { id, name } = &pattern.kind {
        if resolved
            .pattern_constructor(context.module, pattern.id)
            .is_none()
        {
            let key = ling_resolve::BindingKey::new(context.module, *id);
            let info = resolved.bindings().get(&key);
            nodes.push(semantic_node(
                context.mode,
                context.package,
                "binding",
                context.module_name,
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
                add_pattern_nodes(nodes, typed, context, &pattern_id, pattern);
            }
        }
        hir::PatternKind::Record(fields) => {
            for field in fields {
                add_pattern_nodes(nodes, typed, context, &pattern_id, &field.pattern);
            }
        }
        hir::PatternKind::Constructor { arguments, .. } => {
            for argument in arguments {
                add_pattern_nodes(nodes, typed, context, &pattern_id, argument);
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
    context: NodeContext<'_>,
    owner: &str,
    expression: &hir::Expression,
) {
    let typed = checked.typed();
    let expression_id = semantic_node_id(
        context.mode,
        context.package,
        "expression",
        context.module_name,
        &format!("local:{}", expression.id.get()),
    );
    nodes.push(semantic_node(
        context.mode,
        context.package,
        "expression",
        context.module_name,
        &format!("local:{}", expression.id.get()),
        Some(expression_kind(&expression.kind)),
        owner,
        typed
            .expression_type(ExpressionKey::new(context.module, expression.id))
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
                        let key = ling_resolve::BindingKey::new(context.module, binding.id);
                        let binding_id = semantic_node_id(
                            context.mode,
                            context.package,
                            "binding",
                            context.module_name,
                            &format!("local:{}", binding.id.get()),
                        );
                        nodes.push(semantic_node(
                            context.mode,
                            context.package,
                            "binding",
                            context.module_name,
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
                                context.mode,
                                context.package,
                                "function",
                                context.module_name,
                                &format!("local:{}", binding.id.get()),
                            );
                            let effects = function_type.effects().names();
                            let capabilities = capabilities_for_effects(&effects);
                            nodes.push(semantic_node(
                                context.mode,
                                context.package,
                                "function",
                                context.module_name,
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
                                    context.mode,
                                    context.package,
                                    "effect",
                                    context.module_name,
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
                                    context.mode,
                                    context.package,
                                    "capability",
                                    context.module_name,
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
                                context,
                                &parameter_owner,
                                ordinal,
                                parameter,
                            );
                        }
                        add_expression_nodes(
                            nodes,
                            checked,
                            context,
                            &parameter_owner,
                            &binding.value,
                        );
                    }
                    hir::SequenceElement::Expression(expression) => {
                        add_expression_nodes(nodes, checked, context, owner, expression)
                    }
                    hir::SequenceElement::LetAwait(binding) => {
                        add_pattern_nodes(nodes, typed, context, &expression_id, &binding.pattern);
                        add_expression_nodes(
                            nodes,
                            checked,
                            context,
                            &expression_id,
                            &binding.call,
                        );
                    }
                }
            }
        }
        hir::ExpressionKind::TaskScope { body, .. } => {
            add_expression_nodes(nodes, checked, context, &expression_id, body)
        }
        hir::ExpressionKind::TaskSpawn { call, .. } => {
            add_expression_nodes(nodes, checked, context, &expression_id, call)
        }
        hir::ExpressionKind::TaskAwait { handle, .. } => {
            add_expression_nodes(nodes, checked, context, &expression_id, handle)
        }
        hir::ExpressionKind::TaskReturn { value, .. } => {
            add_expression_nodes(nodes, checked, context, &expression_id, value)
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
                add_expression_nodes(nodes, checked, context, owner, expression);
            }
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            add_expression_nodes(nodes, checked, context, owner, scrutinee);
            for case in cases {
                add_pattern_nodes(nodes, typed, context, &expression_id, &case.pattern);
                if let Some(guard) = &case.guard {
                    add_expression_nodes(nodes, checked, context, owner, guard);
                }
                add_expression_nodes(nodes, checked, context, owner, &case.body);
            }
        }
        hir::ExpressionKind::Assignment { value, .. } => {
            add_expression_nodes(nodes, checked, context, owner, value);
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            add_expression_nodes(nodes, checked, context, owner, function);
            for argument in arguments {
                add_expression_nodes(nodes, checked, context, owner, argument);
            }
        }
        hir::ExpressionKind::Projection { target, .. } => {
            add_expression_nodes(nodes, checked, context, owner, target);
        }
        hir::ExpressionKind::Binary { left, right, .. } => {
            for expression in [left.as_ref(), right.as_ref()] {
                add_expression_nodes(nodes, checked, context, owner, expression);
            }
        }
        hir::ExpressionKind::Unary { operand, .. } => {
            add_expression_nodes(nodes, checked, context, owner, operand);
        }
        hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
            for element in elements {
                add_expression_nodes(nodes, checked, context, owner, element);
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                add_expression_nodes(nodes, checked, context, owner, &field.value);
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            add_expression_nodes(nodes, checked, context, owner, base);
            for field in fields {
                add_expression_nodes(nodes, checked, context, owner, &field.value);
            }
        }
        hir::ExpressionKind::Handle { body, clauses } => {
            add_expression_nodes(nodes, checked, context, &expression_id, body);
            for clause in clauses {
                for parameter in &clause.parameters {
                    add_pattern_nodes(nodes, typed, context, &expression_id, parameter);
                }
                if let Some(resume) = &clause.resume {
                    let key = ling_resolve::BindingKey::new(context.module, resume.id);
                    nodes.push(semantic_node(
                        context.mode,
                        context.package,
                        "binding",
                        context.module_name,
                        &format!("local:{}", resume.id.get()),
                        Some(&resume.name.normalized),
                        &expression_id,
                        typed.binding_type(key).map(|id| typed.display_type(id)),
                        Some(false),
                        Some(resume.id.get()),
                        Vec::new(),
                        Vec::new(),
                        identifier_metadata(&resume.name),
                    ));
                }
                add_expression_nodes(nodes, checked, context, &expression_id, &clause.body);
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
        hir::ExpressionKind::TaskScope { .. } => "task_scope",
        hir::ExpressionKind::TaskSpawn { .. } => "task_spawn",
        hir::ExpressionKind::TaskAwait { .. } => "task_await",
        hir::ExpressionKind::TaskReturn { .. } => "task_return",
        hir::ExpressionKind::Handle { .. } => "handler",
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
    mode: IdentityMode,
) -> BTreeMap<(ModuleId, hir::ReferenceId), String> {
    let mut sources = BTreeMap::new();
    for module in resolved.modules() {
        let module_name = module.hir.module.name.normalized();
        for definition in &module.hir.definitions {
            collect_expression_reference_sources(
                mode,
                module.package.as_ref(),
                &module_name,
                module.id,
                &definition.value,
                &mut sources,
            );
        }
        for task in &module.hir.tasks {
            collect_expression_reference_sources(
                mode,
                module.package.as_ref(),
                &module_name,
                module.id,
                &task.body,
                &mut sources,
            );
        }
    }
    sources
}

fn collect_expression_reference_sources(
    mode: IdentityMode,
    package: Option<&PackageIdentity>,
    module_name: &str,
    module: ModuleId,
    expression: &hir::Expression,
    sources: &mut BTreeMap<(ModuleId, hir::ReferenceId), String>,
) {
    let source_id = semantic_node_id(
        mode,
        package,
        "expression",
        module_name,
        &format!("local:{}", expression.id.get()),
    );
    match &expression.kind {
        hir::ExpressionKind::Assignment { place, value } => {
            sources.insert((module, place.root_reference), source_id);
            collect_expression_reference_sources(
                mode,
                package,
                module_name,
                module,
                value,
                sources,
            );
        }
        hir::ExpressionKind::Projection {
            reference, target, ..
        } => {
            sources.insert((module, *reference), source_id);
            collect_expression_reference_sources(
                mode,
                package,
                module_name,
                module,
                target,
                sources,
            );
        }
        hir::ExpressionKind::Name { reference, .. } => {
            sources.insert((module, *reference), source_id);
        }
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                let expression = match element {
                    hir::SequenceElement::Let(binding) => &binding.value,
                    hir::SequenceElement::LetAwait(binding) => &binding.call,
                    hir::SequenceElement::Expression(expression) => expression,
                };
                collect_expression_reference_sources(
                    mode,
                    package,
                    module_name,
                    module,
                    expression,
                    sources,
                );
            }
        }
        hir::ExpressionKind::TaskScope { body, .. } => {
            collect_expression_reference_sources(mode, package, module_name, module, body, sources)
        }
        hir::ExpressionKind::TaskSpawn { call, .. } => {
            collect_expression_reference_sources(mode, package, module_name, module, call, sources)
        }
        hir::ExpressionKind::TaskAwait { handle, .. } => collect_expression_reference_sources(
            mode,
            package,
            module_name,
            module,
            handle,
            sources,
        ),
        hir::ExpressionKind::TaskReturn { value, .. } => {
            collect_expression_reference_sources(mode, package, module_name, module, value, sources)
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
                collect_expression_reference_sources(
                    mode,
                    package,
                    module_name,
                    module,
                    expression,
                    sources,
                );
            }
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            collect_expression_reference_sources(
                mode,
                package,
                module_name,
                module,
                scrutinee,
                sources,
            );
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_expression_reference_sources(
                        mode,
                        package,
                        module_name,
                        module,
                        guard,
                        sources,
                    );
                }
                collect_expression_reference_sources(
                    mode,
                    package,
                    module_name,
                    module,
                    &case.body,
                    sources,
                );
            }
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            collect_expression_reference_sources(
                mode,
                package,
                module_name,
                module,
                function,
                sources,
            );
            for argument in arguments {
                collect_expression_reference_sources(
                    mode,
                    package,
                    module_name,
                    module,
                    argument,
                    sources,
                );
            }
        }
        hir::ExpressionKind::Binary { left, right, .. } => {
            for expression in [left.as_ref(), right.as_ref()] {
                collect_expression_reference_sources(
                    mode,
                    package,
                    module_name,
                    module,
                    expression,
                    sources,
                );
            }
        }
        hir::ExpressionKind::Unary { operand, .. } => {
            collect_expression_reference_sources(
                mode,
                package,
                module_name,
                module,
                operand,
                sources,
            );
        }
        hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
            for element in elements {
                collect_expression_reference_sources(
                    mode,
                    package,
                    module_name,
                    module,
                    element,
                    sources,
                );
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                collect_expression_reference_sources(
                    mode,
                    package,
                    module_name,
                    module,
                    &field.value,
                    sources,
                );
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            collect_expression_reference_sources(mode, package, module_name, module, base, sources);
            for field in fields {
                collect_expression_reference_sources(
                    mode,
                    package,
                    module_name,
                    module,
                    &field.value,
                    sources,
                );
            }
        }
        hir::ExpressionKind::Handle { body, clauses } => {
            collect_expression_reference_sources(mode, package, module_name, module, body, sources);
            for clause in clauses {
                collect_expression_reference_sources(
                    mode,
                    package,
                    module_name,
                    module,
                    &clause.body,
                    sources,
                );
            }
        }
        hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
    }
}

fn semantic_byte_span(span: Span) -> SemanticByteSpan {
    SemanticByteSpan {
        source: span.source().get(),
        start: span.start().get(),
        end: span.end().get(),
    }
}

fn trait_ide_id(
    mode: IdentityMode,
    package: Option<&PackageIdentity>,
    module: &str,
    name: &str,
    member_definition_ids: &[String],
) -> String {
    let mut encoder = Encoder::new("ling.trait-ide-id/v1");
    encoder.string(LANGUAGE_VERSION);
    encoder.string(mode.schema());
    encode_optional_package(package, &mut encoder);
    encoder.string(module);
    encoder.string(name);
    encoder.strings(member_definition_ids);
    hash(encoder.finish())
}

fn implementation_ide_id(
    mode: IdentityMode,
    package: Option<&PackageIdentity>,
    module: &str,
    trait_id: &str,
    receiver: &str,
    member_definition_ids: &[String],
) -> String {
    let mut encoder = Encoder::new("ling.impl-ide-id/v1");
    encoder.string(LANGUAGE_VERSION);
    encoder.string(mode.schema());
    encode_optional_package(package, &mut encoder);
    encoder.string(module);
    encoder.string(trait_id);
    encoder.string(receiver);
    encoder.strings(member_definition_ids);
    hash(encoder.finish())
}

fn encode_optional_package(package: Option<&PackageIdentity>, encoder: &mut Encoder) {
    match package {
        Some(package) => {
            encoder.bool(true);
            encode_package_identity_to_encoder(package, encoder);
        }
        None => encoder.bool(false),
    }
}

fn definition_kind(kind: DefinitionKind) -> &'static str {
    match kind {
        DefinitionKind::Value => "value",
        DefinitionKind::Task => "task",
        DefinitionKind::Actor => "actor",
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
        hir::BinaryOperator::BooleanAnd => 11,
        hir::BinaryOperator::BooleanOr => 12,
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
        assert!(first.graph().actor.is_none());
        assert!(!first.json().contains("x-ling-actor"));
    }

    #[test]
    fn checked_actor_publishes_canonical_message_projection() {
        let source = concat!(
            "module Main\n\n",
            "type Envelope<'a> = { payload: 'a }\n",
            "type Tree<'a> =\n",
            "    | Branch of List<Tree<'a>>\n",
            "    | Leaf of 'a\n\n",
            "actor 收件箱 : Envelope<Tree<Int>> =\n",
            "    mailbox capacity 64 overflow Reject\n",
            "    state Int = 0\n",
            "    receive state message =\n",
            "        state\n",
        );
        let first = snapshot_named("actor-a.ling", source);
        let second = snapshot_named("actor-b.ling", &source.replace('\n', "\r\n"));
        let with_bom = snapshot_named("actor-bom.ling", &format!("\u{feff}{source}"));
        let projection = first.graph().actor.as_ref().expect("Actor extension");
        assert_eq!(projection.version, ACTOR_GRAPH_EXTENSION_VERSION);
        assert_eq!(projection.actors.len(), 1);
        let actor = &projection.actors[0];
        assert_eq!(actor.sendability, "SendableLocal(Value)");
        assert_eq!(actor.mailbox.capacity, 64);
        assert_eq!(actor.mailbox.overflow, "Reject");
        assert!(!actor.mailbox.canonical_bytes.is_empty());
        assert!(actor.mailbox.span.start < actor.mailbox.capacity_span.start);
        assert!(actor.mailbox.capacity_span.end < actor.mailbox.policy_span.start);
        assert_eq!(actor.turn.dispatch, "OneMessage");
        assert_eq!(actor.turn.suspension, "Forbidden");
        assert_eq!(actor.turn.reentry, "Forbidden");
        assert_eq!(actor.turn.state_commit, "PublishOnNormalReturn");
        assert_eq!(actor.turn.self_send, "MailboxOnly");
        assert_ne!(actor.turn.actor_type, 0);
        assert_ne!(actor.turn.state_binding, actor.turn.message_binding);
        assert!(!actor.turn.canonical_bytes.is_empty());
        assert_eq!(actor.turn.span.source, actor.span.source);
        assert!(actor.turn.span.start < actor.turn.body_span.start);
        assert!(actor.turn.body_span.end <= actor.turn.span.end);
        assert!(actor.schema_id.starts_with("experimental:blake3:"));
        assert!(actor.nodes.iter().any(|node| node.kind == "record"));
        assert!(actor.nodes.iter().any(|node| node.kind == "variant"));
        assert!(actor.nodes.iter().any(|node| node.kind == "list"));
        assert_eq!(first.program_id(), second.program_id());
        assert_eq!(first.program_id(), with_bom.program_id());
        assert_eq!(
            actor.turn.canonical_bytes,
            second
                .graph()
                .actor
                .as_ref()
                .expect("Actor extension")
                .actors[0]
                .turn
                .canonical_bytes
        );
        assert_eq!(
            actor.schema_id,
            second
                .graph()
                .actor
                .as_ref()
                .expect("Actor extension")
                .actors[0]
                .schema_id
        );
        read_json(first.json()).expect("isolated reader validates the emitted extension");
        validate_audit_model(&first.audit_model())
            .expect("Actor extension does not imply an Audit Source Actor projection");

        let changed = snapshot(&source.replace("Envelope<Tree<Int>>", "Envelope<Tree<Text>>"));
        assert_ne!(first.program_id(), changed.program_id());
        assert_ne!(
            actor.schema_id,
            changed
                .graph()
                .actor
                .as_ref()
                .expect("Actor extension")
                .actors[0]
                .schema_id
        );

        let changed_capacity = snapshot(&source.replace("capacity 64", "capacity 65"));
        assert_ne!(first.program_id(), changed_capacity.program_id());
        assert_ne!(
            actor.mailbox.canonical_bytes,
            changed_capacity
                .graph()
                .actor
                .as_ref()
                .expect("Actor extension")
                .actors[0]
                .mailbox
                .canonical_bytes
        );
    }

    #[test]
    fn actor_projection_reader_rejects_independent_corruptions() {
        let snapshot = snapshot(concat!(
            "module Main\n\n",
            "type Message = { values: List<Int> }\n\n",
            "actor Inbox : Message =\n",
            "    mailbox capacity 16 overflow Reject\n",
            "    state Int = 0\n",
            "    receive state message =\n",
            "        state\n",
        ));
        let value = serde_json::from_str::<serde_json::Value>(snapshot.json())
            .expect("valid semantic JSON");

        let mut bad_version = value.clone();
        bad_version["x-ling-actor"]["version"] = serde_json::json!("9.9");
        assert!(read_json(&bad_version.to_string()).is_err());

        let mut bad_digest = value.clone();
        bad_digest["x-ling-actor"]["actors"][0]["schema_id"] =
            serde_json::json!(format!("experimental:blake3:{}", "0".repeat(64)));
        assert!(read_json(&bad_digest.to_string()).is_err());

        let mut bad_span = value.clone();
        bad_span["x-ling-actor"]["actors"][0]["span"]["start"] = serde_json::json!(u32::MAX);
        assert!(read_json(&bad_span.to_string()).is_err());

        let mut bad_capacity = value.clone();
        bad_capacity["x-ling-actor"]["actors"][0]["mailbox"]["capacity"] = serde_json::json!(0);
        assert!(read_json(&bad_capacity.to_string()).is_err());

        let mut bad_policy = value.clone();
        bad_policy["x-ling-actor"]["actors"][0]["mailbox"]["overflow"] = serde_json::json!("Wait");
        assert!(read_json(&bad_policy.to_string()).is_err());

        let mut bad_mailbox_bytes = value.clone();
        bad_mailbox_bytes["x-ling-actor"]["actors"][0]["mailbox"]["canonical_bytes"] =
            serde_json::json!([0]);
        assert!(read_json(&bad_mailbox_bytes.to_string()).is_err());

        let mut bad_mailbox_span = value.clone();
        bad_mailbox_span["x-ling-actor"]["actors"][0]["mailbox"]["capacity_span"]["start"] =
            serde_json::json!(u32::MAX);
        assert!(read_json(&bad_mailbox_span.to_string()).is_err());

        for (field, invalid) in [
            ("dispatch", serde_json::json!("Batch")),
            ("suspension", serde_json::json!("Allowed")),
            ("reentry", serde_json::json!("Allowed")),
            ("state_commit", serde_json::json!("Incremental")),
            ("self_send", serde_json::json!("Recursive")),
        ] {
            let mut bad_mode = value.clone();
            bad_mode["x-ling-actor"]["actors"][0]["turn"][field] = invalid;
            assert!(read_json(&bad_mode.to_string()).is_err());
        }

        let mut bad_actor_type = value.clone();
        bad_actor_type["x-ling-actor"]["actors"][0]["turn"]["actor_type"] = serde_json::json!(0);
        assert!(read_json(&bad_actor_type.to_string()).is_err());

        let mut bad_transition = value.clone();
        let transition = bad_transition["x-ling-actor"]["actors"][0]["turn"]["transition"]
            .as_u64()
            .expect("transition identity");
        bad_transition["x-ling-actor"]["actors"][0]["turn"]["transition"] =
            serde_json::json!(transition + 1);
        assert!(read_json(&bad_transition.to_string()).is_err());

        let mut bad_bindings = value.clone();
        bad_bindings["x-ling-actor"]["actors"][0]["turn"]["message_binding"] =
            bad_bindings["x-ling-actor"]["actors"][0]["turn"]["state_binding"].clone();
        assert!(read_json(&bad_bindings.to_string()).is_err());

        let mut bad_turn_bytes = value.clone();
        bad_turn_bytes["x-ling-actor"]["actors"][0]["turn"]["canonical_bytes"] =
            serde_json::json!([0]);
        assert!(read_json(&bad_turn_bytes.to_string()).is_err());

        let mut bad_turn_source = value.clone();
        bad_turn_source["x-ling-actor"]["actors"][0]["turn"]["body_span"]["source"] =
            serde_json::json!(1);
        assert!(read_json(&bad_turn_source.to_string()).is_err());

        let mut bad_turn_order = value.clone();
        bad_turn_order["x-ling-actor"]["actors"][0]["turn"]["body_span"]["start"] =
            serde_json::json!(0);
        assert!(read_json(&bad_turn_order.to_string()).is_err());

        let mut bad_owner = value.clone();
        bad_owner["x-ling-actor"]["actors"][0]["definition_id"] =
            serde_json::json!(format!("experimental:blake3:{}", "0".repeat(64)));
        assert!(read_json(&bad_owner.to_string()).is_err());

        let mut bad_edge = value.clone();
        let nodes = bad_edge["x-ling-actor"]["actors"][0]["nodes"]
            .as_array_mut()
            .expect("schema nodes");
        let list = nodes
            .iter_mut()
            .find(|node| node["kind"] == "list")
            .expect("List node");
        list["element"] = serde_json::json!(u32::MAX);
        assert!(read_json(&bad_edge.to_string()).is_err());

        let mut unknown_field = value;
        unknown_field["x-ling-actor"]["actors"][0]["runtime_mailbox"] = serde_json::json!(true);
        assert!(read_json(&unknown_field.to_string()).is_err());
    }

    #[test]
    fn actor_projection_reader_rejects_noncanonical_actor_order() {
        let snapshot = snapshot(concat!(
            "module Main\n\n",
            "actor First : Int =\n",
            "    mailbox capacity 1 overflow Reject\n",
            "    state Int = 0\n",
            "    receive state message =\n",
            "        state\n\n",
            "actor Second : Int =\n",
            "    mailbox capacity 2 overflow Reject\n",
            "    state Int = 1\n",
            "    receive state message =\n",
            "        state + message\n",
        ));
        let mut value = serde_json::from_str::<serde_json::Value>(snapshot.json())
            .expect("valid semantic JSON");
        value["x-ling-actor"]["actors"]
            .as_array_mut()
            .expect("Actor list")
            .reverse();

        assert!(read_json(&value.to_string()).is_err());
    }

    #[test]
    fn checked_handler_publishes_graph_identity_and_audit_evidence() {
        let snapshot = snapshot_named(
            "handler.ling",
            "let value =\n    handle 1 with\n        operation Clock.now() -> 1\n",
        );
        let handler_node = snapshot
            .graph()
            .nodes
            .iter()
            .find(|node| node.kind == "expression" && node.name.as_deref() == Some("handler"))
            .expect("handler expression node");
        let audit = snapshot.audit_model();
        let handler = audit
            .modules
            .iter()
            .find(|module| module.name == "Main")
            .expect("Main Audit module")
            .handlers
            .first()
            .expect("checked handler Audit projection");
        assert_eq!(handler.handler_id, handler_node.node_id);
        assert_eq!(handler.input_row, "{}");
        assert_eq!(handler.residual_row, "{}");
        assert_eq!(handler.clauses[0].label, "Clock");
        assert!(handler.source_start < handler.source_end);
    }

    #[test]
    fn checked_task_publishes_valid_graph_and_audit_extensions() {
        let source = concat!(
            "module Main\n\n",
            "task child value =\n",
            "    scope\n",
            "        return value\n\n",
            "task parent value =\n",
            "    scope\n",
            "        let! result = child value\n",
            "        return result\n",
        );
        let snapshot = snapshot_named("任务.ling", source);
        let projection = snapshot
            .graph()
            .task
            .as_ref()
            .expect("Task graph extension");
        assert_eq!(projection.version, TASK_GRAPH_EXTENSION_VERSION);
        assert_eq!(projection.tasks.len(), 2);
        let parent = projection
            .tasks
            .iter()
            .find(|task| task.spawns.len() == 1)
            .expect("parent Task projection");
        assert_eq!(parent.spawns[0].syntax, "let!");
        assert!(parent.spawns[0].pattern_span.is_some());
        assert_eq!(parent.suspensions.len(), 1);
        assert_eq!(
            read_json(snapshot.json()).expect("Task graph reads"),
            *snapshot.graph()
        );

        let audit = snapshot.audit_model();
        validate_audit_model(&audit).expect("Task Audit validates");
        let audit_parent = audit
            .modules
            .iter()
            .flat_map(|module| &module.tasks)
            .find(|task| task.spawns.len() == 1)
            .expect("parent Task Audit");
        assert_eq!(audit_parent.spawns[0].syntax, "let!");
        assert!(audit_parent.spawns[0].pattern_start.is_some());
    }

    #[test]
    fn task_audit_accepts_path_exclusive_suspensions_for_one_spawn() {
        let snapshot = snapshot(concat!(
            "module Main\n\n",
            "task child value =\n",
            "    scope\n",
            "        return value\n\n",
            "task parent value =\n",
            "    scope\n",
            "        let handle = spawn child value\n",
            "        let result = if true then await handle else await handle\n",
            "        return result\n",
        ));
        let audit = snapshot.audit_model();
        let parent = audit
            .modules
            .iter()
            .flat_map(|module| &module.tasks)
            .find(|task| task.spawns.len() == 1)
            .expect("parent Task Audit projection");
        assert_eq!(parent.suspensions.len(), 2);
        assert_eq!(
            parent.suspensions[0].awaited_task,
            parent.suspensions[1].awaited_task
        );
        validate_audit_model(&audit).expect("path-exclusive suspensions validate");
    }

    #[test]
    fn trait_witness_identity_is_part_of_semantic_program_identity() {
        let first_source = concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "let main () = Renderable.render { name = \"Ling\" }\n",
        );
        let second_source = first_source.replace("item.name", "\"Other\"");
        let first = snapshot(first_source);
        let repeat = snapshot(first_source);
        let second = snapshot(&second_source);
        assert_eq!(
            first.checked().dictionary().canonical_bytes(),
            repeat.checked().dictionary().canonical_bytes()
        );
        assert_eq!(first.program_id(), repeat.program_id());
        assert_ne!(first.program_id(), second.program_id());
    }

    #[test]
    fn trait_ide_projection_preserves_selected_ids_and_original_spans() {
        let source = concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "let main () = Renderable.render { name = \"Ling\" }\n",
        );
        let snapshot = snapshot(source);
        let projection = snapshot
            .trait_ide()
            .expect("Trait witness projection is present");
        assert_eq!(projection.version, TRAIT_IDE_EXTENSION_VERSION);
        assert_eq!(projection.witnesses.len(), 1);
        let witness = &projection.witnesses[0];
        assert_eq!(witness.trait_name, "Renderable");
        assert_eq!(witness.trait_module, "Main");
        assert_eq!(witness.implementation_module, "Main");
        assert_eq!(witness.obligation_order, 0);
        assert_eq!(witness.members.len(), 1);
        let member = &witness.members[0];
        assert_eq!(member.ordinal, 0);
        assert_eq!(member.name, "render");
        assert!(member.trait_span.start < member.trait_span.end);
        assert!(member.implementation_span.start < member.implementation_span.end);
        assert!(member.trait_source.ends_with("test.ling"));
        assert!(member.implementation_source.ends_with("test.ling"));
        assert!(
            member
                .trait_definition_id
                .starts_with("experimental:blake3:")
        );
        assert!(
            member
                .implementation_definition_id
                .starts_with("experimental:blake3:")
        );
        assert!(witness.trait_id.starts_with("experimental:blake3:"));
        assert!(
            witness
                .implementation_id
                .starts_with("experimental:blake3:")
        );

        let decoded = read_json(snapshot.json()).expect("projection round-trips");
        assert_eq!(decoded, snapshot.graph().clone());
        let alternate_name = snapshot_named("different-host-name.ling", source);
        assert_eq!(snapshot.program_id(), alternate_name.program_id());
        assert_eq!(
            snapshot.trait_ide().unwrap().witnesses[0].trait_id,
            alternate_name.trait_ide().unwrap().witnesses[0].trait_id
        );
        assert_ne!(
            snapshot.trait_ide().unwrap().witnesses[0].members[0].trait_source,
            alternate_name.trait_ide().unwrap().witnesses[0].members[0].trait_source
        );
    }

    #[test]
    fn trait_ide_projection_lookups_are_read_only_and_projection_ordered() {
        let snapshot = snapshot(concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "let main () = Renderable.render { name = \"Ling\" }\n",
        ));
        let projection = snapshot.trait_ide().expect("Trait projection");
        let witness = &projection.witnesses[0];
        let member = &witness.members[0];

        assert_eq!(
            projection
                .witnesses_by_trait_id(&witness.trait_id)
                .map(|candidate| candidate.implementation_id.as_str())
                .collect::<Vec<_>>(),
            vec![witness.implementation_id.as_str()]
        );
        assert_eq!(
            projection
                .witness_by_implementation_id(&witness.implementation_id)
                .map(|candidate| candidate.trait_id.as_str()),
            Some(witness.trait_id.as_str())
        );
        assert!(projection.witness_by_implementation_id("missing").is_none());
        assert_eq!(
            projection
                .members_by_trait_definition_id(&member.trait_definition_id)
                .map(|candidate| candidate.implementation_definition_id.as_str())
                .collect::<Vec<_>>(),
            vec![member.implementation_definition_id.as_str()]
        );
        assert_eq!(
            projection
                .member_by_implementation_definition_id(&member.implementation_definition_id)
                .map(|candidate| candidate.name.as_str()),
            Some(member.name.as_str())
        );
        assert!(
            projection
                .member_by_implementation_definition_id("missing")
                .is_none()
        );

        let mut duplicate = witness.clone();
        duplicate.obligation_order = witness.obligation_order + 1;
        duplicate.implementation_id.push_str("-second");
        duplicate.members[0]
            .implementation_definition_id
            .push_str("-second");
        let mut ordered = projection.clone();
        ordered.witnesses.push(duplicate);
        assert_eq!(
            ordered
                .witnesses_by_trait_id(&witness.trait_id)
                .map(|candidate| candidate.implementation_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                witness.implementation_id.as_str(),
                ordered.witnesses[1].implementation_id.as_str()
            ]
        );
        assert_eq!(
            ordered
                .member_by_implementation_definition_id(&member.implementation_definition_id)
                .unwrap()
                .implementation_definition_id,
            member.implementation_definition_id
        );
    }

    #[test]
    fn trait_ide_projection_rejects_bad_extension_version_and_spans() {
        let snapshot = snapshot(concat!(
            "module Main\n\n",
            "trait Renderable<'a> =\n",
            "    render: 'a -> Text\n\n",
            "type Item = { name: Text }\n\n",
            "impl Renderable Item =\n",
            "    let render item = item.name\n\n",
            "let main () = Renderable.render { name = \"Ling\" }\n",
        ));
        let mut version: serde_json::Value =
            serde_json::from_str(snapshot.json()).expect("snapshot JSON");
        version["x-ling-trait-ide"]["version"] = serde_json::json!("0.2");
        let error = read_json(&serde_json::to_string(&version).unwrap()).expect_err("version");
        assert!(matches!(
            error.kind,
            SemanticReadErrorKind::InvalidTraitIdeVersion { .. }
        ));

        let mut span: serde_json::Value =
            serde_json::from_str(snapshot.json()).expect("snapshot JSON");
        span["x-ling-trait-ide"]["witnesses"][0]["members"][0]["trait_span"]["start"] =
            serde_json::json!(99);
        span["x-ling-trait-ide"]["witnesses"][0]["members"][0]["trait_span"]["end"] =
            serde_json::json!(1);
        let error = read_json(&serde_json::to_string(&span).unwrap()).expect_err("span");
        assert!(matches!(
            error.kind,
            SemanticReadErrorKind::InvalidTraitIdeSpan
        ));
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
    fn boolean_operator_tags_append_and_produce_distinct_stable_body_ids() {
        use hir::BinaryOperator as Operator;

        for (operator, expected) in [
            (Operator::Equal, 0),
            (Operator::NotEqual, 1),
            (Operator::Less, 2),
            (Operator::LessEqual, 3),
            (Operator::Greater, 4),
            (Operator::GreaterEqual, 5),
            (Operator::Add, 6),
            (Operator::Subtract, 7),
            (Operator::Multiply, 8),
            (Operator::Divide, 9),
            (Operator::Remainder, 10),
            (Operator::BooleanAnd, 11),
            (Operator::BooleanOr, 12),
        ] {
            assert_eq!(binary_tag(operator), expected);
        }

        let source = concat!(
            "module Main\n\n",
            "let conjunction left right = left && right\n",
            "let disjunction left right = left || right\n",
        );
        let first = snapshot(source);
        let second = snapshot(source);
        let body = |snapshot: &ProgramSnapshot, name: &str| {
            snapshot
                .graph()
                .definitions
                .iter()
                .find(|definition| definition.name == name)
                .map(|definition| definition.body_id.clone())
                .expect("definition exists")
        };
        assert_eq!(first.program_id(), second.program_id());
        assert_ne!(body(&first, "conjunction"), body(&first, "disjunction"));
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
