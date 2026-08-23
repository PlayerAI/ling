use std::collections::{BTreeMap, BTreeSet};

use ling_semantic::{ProgramSnapshot, SemanticDefinition};
use serde::{Deserialize, Serialize};

pub const QUERY_SCHEMA: &str = "ling.semantic-query/0.1";
pub const TRANSACTION_SCHEMA: &str = "ling.semantic-transaction/0.1";
pub const TRANSACTION_RESULT_SCHEMA: &str = "ling.semantic-transaction-result/0.1";
pub const MAX_TRANSACTION_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QueryReport {
    schema: &'static str,
    semantic_schema: String,
    language_version: String,
    unicode_version: String,
    program_id: String,
    symbol: String,
    matches: Vec<QueryMatch>,
}

impl QueryReport {
    pub fn build(snapshot: &ProgramSnapshot, symbol: &str) -> Result<Self, QueryError> {
        require_single_file_scope(snapshot).map_err(QueryError::Scope)?;
        let identifier = ling_unicode::inspect_identifier(symbol)
            .map_err(|error| QueryError::InvalidSymbol(error.to_string()))?;
        let symbol = identifier.identifier().normalized().to_owned();
        let graph = snapshot.graph();
        let matches = graph
            .definitions
            .iter()
            .filter(|definition| definition.origin == "user" && definition.name == symbol)
            .map(QueryMatch::from)
            .collect();
        Ok(Self {
            schema: QUERY_SCHEMA,
            semantic_schema: graph.schema.clone(),
            language_version: graph.language_version.clone(),
            unicode_version: graph.unicode_version.clone(),
            program_id: graph.program_id.clone(),
            symbol,
            matches,
        })
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn matches(&self) -> &[QueryMatch] {
        &self.matches
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QueryMatch {
    definition_id: String,
    body_id: String,
    module: String,
    name: String,
    kind: String,
    #[serde(rename = "type")]
    type_name: String,
    effects: Vec<String>,
    capabilities: Vec<String>,
}

impl QueryMatch {
    pub fn summary(&self) -> String {
        format!(
            "{} {} {} : {} [definition_id={}; body_id={}; effects={}; capabilities={}]",
            self.kind,
            self.module,
            self.name,
            self.type_name,
            self.definition_id,
            self.body_id,
            joined_or_dash(&self.effects),
            joined_or_dash(&self.capabilities),
        )
    }
}

impl From<&SemanticDefinition> for QueryMatch {
    fn from(definition: &SemanticDefinition) -> Self {
        Self {
            definition_id: definition.definition_id.clone(),
            body_id: definition.body_id.clone(),
            module: definition.module.clone(),
            name: definition.name.clone(),
            kind: definition.kind.clone(),
            type_name: definition.type_name.clone(),
            effects: definition.effects.clone(),
            capabilities: definition.capabilities.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryError {
    InvalidSymbol(String),
    Scope(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransactionRequest {
    schema: String,
    base_program_id: String,
    target_ids: Vec<String>,
    operation: ReplaceOperation,
    preserve: Vec<PreserveConstraint>,
    provenance: Provenance,
}

impl TransactionRequest {
    pub fn parse(bytes: &[u8]) -> Result<Self, TransactionError> {
        if bytes.len() > MAX_TRANSACTION_BYTES {
            return Err(TransactionError::InvalidInput(
                "transaction document exceeds 1048576 bytes".to_owned(),
            ));
        }
        let request: Self = serde_json::from_slice(bytes)
            .map_err(|error| TransactionError::InvalidInput(error.to_string()))?;
        request.validate_shape()?;
        Ok(request)
    }

    pub fn replacement(&self) -> &str {
        &self.operation.content
    }

    pub fn validate_current(&self, current: &ProgramSnapshot) -> Result<(), TransactionError> {
        require_single_file_scope(current).map_err(TransactionError::InvalidInput)?;
        if self.base_program_id != current.program_id().as_str() {
            return Err(TransactionError::StaleBase {
                expected: current.program_id().to_string(),
                found: self.base_program_id.clone(),
            });
        }
        let current_definitions = definitions_by_id(current);
        for target in &self.target_ids {
            if current_definitions
                .get(target.as_str())
                .is_none_or(|definition| definition.origin != "user")
            {
                return Err(TransactionError::InvalidInput(format!(
                    "target `{target}` is not a current user definition"
                )));
            }
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<(), TransactionError> {
        if self.schema != TRANSACTION_SCHEMA {
            return Err(TransactionError::InvalidInput(format!(
                "expected schema `{TRANSACTION_SCHEMA}`"
            )));
        }
        if self.base_program_id.is_empty() {
            return Err(TransactionError::InvalidInput(
                "base_program_id must not be empty".to_owned(),
            ));
        }
        if self.target_ids.is_empty() || !strictly_sorted(&self.target_ids) {
            return Err(TransactionError::InvalidInput(
                "target_ids must be non-empty, unique, and byte-sorted".to_owned(),
            ));
        }
        if self.operation.kind != "replace_source" || self.operation.content.is_empty() {
            return Err(TransactionError::InvalidInput(
                "operation must be one non-empty replace_source".to_owned(),
            ));
        }
        if self.preserve != PreserveConstraint::all() {
            return Err(TransactionError::InvalidInput(
                "preserve must contain definition_set, types, effects, and capabilities in canonical order"
                    .to_owned(),
            ));
        }
        if self.provenance.actor.is_empty()
            || self.provenance.actor.len() > 128
            || self.provenance.reason.is_empty()
            || self.provenance.reason.len() > 512
        {
            return Err(TransactionError::InvalidInput(
                "provenance actor/reason must be non-empty and within byte limits".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ReplaceOperation {
    kind: String,
    content: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum PreserveConstraint {
    DefinitionSet,
    Types,
    Effects,
    Capabilities,
}

impl PreserveConstraint {
    fn all() -> Vec<Self> {
        vec![
            Self::DefinitionSet,
            Self::Types,
            Self::Effects,
            Self::Capabilities,
        ]
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    actor: String,
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TransactionReport {
    schema: &'static str,
    transaction_schema: &'static str,
    status: &'static str,
    committed: bool,
    base_program_id: String,
    candidate_program_id: String,
    target_ids: Vec<String>,
    changed_body_ids: Vec<String>,
    preserved: Vec<PreserveConstraint>,
    provenance: Provenance,
}

impl TransactionReport {
    pub fn validate(
        current: &ProgramSnapshot,
        candidate: &ProgramSnapshot,
        request: &TransactionRequest,
    ) -> Result<Self, TransactionError> {
        request.validate_current(current)?;
        require_single_file_scope(candidate).map_err(TransactionError::InvalidInput)?;

        let current_definitions = definitions_by_id(current);
        let candidate_definitions = definitions_by_id(candidate);
        if current_definitions.keys().ne(candidate_definitions.keys()) {
            return Err(TransactionError::PreserveViolation(
                "definition_set".to_owned(),
            ));
        }

        let targets = request
            .target_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let mut changed_body_ids = Vec::new();
        for (definition_id, current_definition) in &current_definitions {
            let candidate_definition = candidate_definitions
                .get(definition_id)
                .expect("equal key sets contain the current definition");
            if identity_projection(current_definition) != identity_projection(candidate_definition)
            {
                return Err(TransactionError::PreserveViolation(
                    "definition_set".to_owned(),
                ));
            }
            if current_definition.type_name != candidate_definition.type_name {
                return Err(TransactionError::PreserveViolation("types".to_owned()));
            }
            if current_definition.effects != candidate_definition.effects {
                return Err(TransactionError::PreserveViolation("effects".to_owned()));
            }
            if current_definition.capabilities != candidate_definition.capabilities {
                return Err(TransactionError::PreserveViolation(
                    "capabilities".to_owned(),
                ));
            }
            if current_definition.body_id != candidate_definition.body_id {
                if !targets.contains(definition_id) {
                    return Err(TransactionError::PreserveViolation(
                        "unauthorized_body_change".to_owned(),
                    ));
                }
                changed_body_ids.push((*definition_id).to_owned());
            }
        }
        if changed_body_ids.is_empty() {
            return Err(TransactionError::PreserveViolation(
                "no_semantic_change".to_owned(),
            ));
        }

        Ok(Self {
            schema: TRANSACTION_RESULT_SCHEMA,
            transaction_schema: TRANSACTION_SCHEMA,
            status: "validated",
            committed: false,
            base_program_id: current.program_id().to_string(),
            candidate_program_id: candidate.program_id().to_string(),
            target_ids: request.target_ids.clone(),
            changed_body_ids,
            preserved: PreserveConstraint::all(),
            provenance: request.provenance.clone(),
        })
    }

    pub fn changed_body_ids(&self) -> &[String] {
        &self.changed_body_ids
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionError {
    InvalidInput(String),
    StaleBase { expected: String, found: String },
    PreserveViolation(String),
}

fn definitions_by_id(snapshot: &ProgramSnapshot) -> BTreeMap<&str, &SemanticDefinition> {
    snapshot
        .graph()
        .definitions
        .iter()
        .map(|definition| (definition.definition_id.as_str(), definition))
        .collect()
}

fn identity_projection(
    definition: &SemanticDefinition,
) -> (
    &str,
    &str,
    &str,
    &str,
    &Option<ling_semantic::SemanticPackageIdentity>,
) {
    (
        &definition.module,
        &definition.name,
        &definition.kind,
        &definition.origin,
        &definition.package,
    )
}

fn require_single_file_scope(snapshot: &ProgramSnapshot) -> Result<(), String> {
    let graph = snapshot.graph();
    if graph.modules.len() != 1 || !graph.modules[0].imports.is_empty() {
        return Err("command requires exactly one source module with no imports".to_owned());
    }
    Ok(())
}

fn strictly_sorted(values: &[String]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn joined_or_dash(values: &[String]) -> String {
    if values.is_empty() {
        "-".to_owned()
    } else {
        values.join(",")
    }
}

#[cfg(test)]
mod tests {
    use crate::compile_source;

    use super::*;

    fn compile(text: &str) -> crate::Compiled {
        compile_source("Main.ling", text.as_bytes().to_vec()).expect("checked source")
    }

    fn request_for(current: &ProgramSnapshot, target: String, content: &str) -> TransactionRequest {
        TransactionRequest {
            schema: TRANSACTION_SCHEMA.to_owned(),
            base_program_id: current.program_id().to_string(),
            target_ids: vec![target],
            operation: ReplaceOperation {
                kind: "replace_source".to_owned(),
                content: content.to_owned(),
            },
            preserve: PreserveConstraint::all(),
            provenance: Provenance {
                actor: "test".to_owned(),
                reason: "body update".to_owned(),
            },
        }
    }

    #[test]
    fn query_normalizes_and_returns_only_user_definitions() {
        let compiled = compile("module Main\n\nlet é = 1\nlet main () = é\n");
        let report = QueryReport::build(&compiled.snapshot, "e\u{301}").unwrap();
        assert_eq!(report.symbol(), "é");
        assert_eq!(report.matches().len(), 1);
        assert!(report.to_json().unwrap().contains(QUERY_SCHEMA));
    }

    #[test]
    fn patch_accepts_only_authorized_body_changes() {
        let current = compile("module Main\n\nlet value = 1\nlet main () = value\n");
        let candidate = compile("module Main\n\nlet value = 2\nlet main () = value\n");
        let target = current
            .snapshot
            .graph()
            .definitions
            .iter()
            .find(|definition| definition.name == "value")
            .unwrap()
            .definition_id
            .clone();
        let request = request_for(&current.snapshot, target.clone(), "unused by validator");
        let report =
            TransactionReport::validate(&current.snapshot, &candidate.snapshot, &request).unwrap();
        assert_eq!(report.changed_body_ids(), [target]);
        assert!(report.to_json().unwrap().contains("\"committed\":false"));
    }

    #[test]
    fn patch_rejects_stale_base_and_constraint_drift() {
        let current = compile("module Main\n\nlet value = 1\n");
        let candidate = compile("module Main\n\nlet value = \"changed\"\n");
        let target = current
            .snapshot
            .graph()
            .definitions
            .iter()
            .find(|definition| definition.name == "value")
            .unwrap()
            .definition_id
            .clone();
        let mut request: TransactionRequest = serde_json::from_value(serde_json::json!({
            "schema": TRANSACTION_SCHEMA,
            "base_program_id": current.snapshot.program_id().as_str(),
            "target_ids": [target],
            "operation": {"kind": "replace_source", "content": "candidate"},
            "preserve": ["definition_set", "types", "effects", "capabilities"],
            "provenance": {"actor": "test", "reason": "negative"}
        }))
        .unwrap();
        request.base_program_id = "experimental:blake3:stale".to_owned();
        assert!(matches!(
            TransactionReport::validate(&current.snapshot, &candidate.snapshot, &request),
            Err(TransactionError::StaleBase { .. })
        ));
        request.base_program_id = current.snapshot.program_id().to_string();
        assert_eq!(
            TransactionReport::validate(&current.snapshot, &candidate.snapshot, &request),
            Err(TransactionError::PreserveViolation("types".to_owned()))
        );
    }

    #[test]
    fn transaction_parser_rejects_unknown_fields_and_noncanonical_targets() {
        let unknown = br#"{"schema":"ling.semantic-transaction/0.1","base_program_id":"id","target_ids":["b","a"],"operation":{"kind":"replace_source","content":"x"},"preserve":["definition_set","types","effects","capabilities"],"provenance":{"actor":"a","reason":"r"},"extra":true}"#;
        assert!(matches!(
            TransactionRequest::parse(unknown),
            Err(TransactionError::InvalidInput(_))
        ));
        assert!(matches!(
            TransactionRequest::parse(&vec![b' '; MAX_TRANSACTION_BYTES + 1]),
            Err(TransactionError::InvalidInput(_))
        ));
    }

    #[test]
    fn patch_rejects_definition_effect_and_authorization_drift() {
        let current = compile("module Main\n\nlet first = 1\nlet second = 2\n");
        let definition = |name: &str| {
            current
                .snapshot
                .graph()
                .definitions
                .iter()
                .find(|definition| definition.name == name)
                .unwrap()
                .definition_id
                .clone()
        };

        let added = compile("module Main\n\nlet first = 1\nlet second = 2\nlet third = 3\n");
        let request = request_for(&current.snapshot, definition("first"), "added");
        assert_eq!(
            TransactionReport::validate(&current.snapshot, &added.snapshot, &request),
            Err(TransactionError::PreserveViolation(
                "definition_set".to_owned()
            ))
        );

        let unauthorized = compile("module Main\n\nlet first = 10\nlet second = 20\n");
        assert_eq!(
            TransactionReport::validate(&current.snapshot, &unauthorized.snapshot, &request),
            Err(TransactionError::PreserveViolation(
                "unauthorized_body_change".to_owned()
            ))
        );

        let unchanged = compile("module Main\n\n\nlet first = 1\nlet second = 2\n");
        assert_eq!(
            TransactionReport::validate(&current.snapshot, &unchanged.snapshot, &request),
            Err(TransactionError::PreserveViolation(
                "no_semantic_change".to_owned()
            ))
        );

        let effect_current = compile("module Main\n\nlet main () = ()\n");
        let effect_target = effect_current
            .snapshot
            .graph()
            .definitions
            .iter()
            .find(|definition| definition.name == "main")
            .unwrap()
            .definition_id
            .clone();
        let effect_candidate = compile(
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"x\"\n",
        );
        let effect_request = request_for(&effect_current.snapshot, effect_target, "effect");
        assert_eq!(
            TransactionReport::validate(
                &effect_current.snapshot,
                &effect_candidate.snapshot,
                &effect_request,
            ),
            Err(TransactionError::PreserveViolation("effects".to_owned()))
        );
    }
}
