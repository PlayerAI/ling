use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

use crate::protocols::{self, ProtocolRecords};

const REGISTRY_PATH: &str = "schemas/registry.toml";
const POLICY_PATH: &str = "docs/governance/SCHEMA-LIFECYCLE.md";
const PROTOCOL_INVENTORY_PATH: &str = "docs/governance/protocol-inventory.toml";
const JSON_SCHEMA_DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";
const SUPPORTED_SCHEMA_KEYWORDS: &[&str] = &[
    "$defs",
    "$id",
    "$ref",
    "$schema",
    "additionalProperties",
    "const",
    "enum",
    "items",
    "minimum",
    "minLength",
    "patternProperties",
    "properties",
    "required",
    "title",
    "type",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub schema_count: usize,
    pub valid_fixture_count: usize,
    pub invalid_fixture_count: usize,
    pub canonical_fixture_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilitySummary {
    pub verified_edge_count: usize,
    pub no_previous_version_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorruptSummary {
    pub mutation_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct SchemaRegistry {
    schema_version: u32,
    updated: String,
    policy: String,
    protocol_inventory: String,
    schema_root: String,
    version_policy: VersionPolicy,
    #[serde(default)]
    schema: Vec<SchemaRecord>,
    #[serde(default)]
    non_json_boundary: Vec<NonJsonBoundary>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct VersionPolicy {
    major_minor: String,
    integer: String,
    writer: String,
    n_minus_one: String,
    semantic_identity: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct SchemaRecord {
    id: String,
    name: String,
    protocol_id: String,
    version: String,
    marker: String,
    version_kind: String,
    stability: String,
    format: String,
    canonical: bool,
    writer: String,
    reader: String,
    #[serde(default)]
    reader_versions: Vec<String>,
    previous_version: String,
    previous_marker: String,
    compatibility: String,
    compatibility_dir: String,
    migration_adapter: String,
    unknown_fields: String,
    missing_fields: String,
    #[serde(default)]
    reader_defaults: Vec<String>,
    canonical_encoding: String,
    #[serde(default)]
    hash_scheme_ids: Vec<String>,
    schema_path: String,
    valid_dir: String,
    invalid_dir: String,
    canonical_dir: String,
    reader_adapter: String,
    #[serde(default)]
    writer_evidence: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct NonJsonBoundary {
    protocol_id: String,
    format: String,
    reason: String,
    #[serde(default)]
    evidence: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct InvalidExpectation {
    schema_version: u32,
    expected: String,
}

pub fn validate_all(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let registry = load_registry(root)?;
    let protocol_records = protocols::protocol_records(root)?;
    let mut errors = validate_registry(root, &registry, &protocol_records);
    let mut valid_fixture_count = 0;
    let mut invalid_fixture_count = 0;
    let mut canonical_fixture_count = 0;

    for record in &registry.schema {
        let schema = match load_schema_document(root, record) {
            Ok(schema) => schema,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        validate_schema_document(record, &schema, &mut errors);
        valid_fixture_count += validate_valid_fixtures(root, record, &schema, &mut errors);
        invalid_fixture_count += validate_invalid_fixtures(root, record, &schema, &mut errors);
        canonical_fixture_count += validate_canonical_fixtures(root, record, &schema, &mut errors);
    }

    finish(errors).map(|()| CheckSummary {
        schema_count: registry.schema.len(),
        valid_fixture_count,
        invalid_fixture_count,
        canonical_fixture_count,
    })
}

pub fn compatibility(
    root: &Path,
    from: &str,
    to: &str,
) -> Result<CompatibilitySummary, Vec<String>> {
    if from != "N-1" || to != "N" {
        return Err(vec![format!(
            "GOV-SCHEMA-0009: compatibility range must be --from N-1 --to N, got {from:?} to {to:?}"
        )]);
    }
    let registry = load_registry(root)?;
    let protocols = protocols::protocol_records(root)?;
    let errors = validate_registry(root, &registry, &protocols);
    let mut verified_edge_count = 0;
    let mut no_previous_version_count = 0;
    let mut compatibility_errors = errors;

    for record in &registry.schema {
        match record.compatibility.as_str() {
            "NoPreviousVersion" => no_previous_version_count += 1,
            other => {
                verified_edge_count += 1;
                compatibility_errors.push(format!(
                    "GOV-SCHEMA-0009: {} declares unsupported compatibility edge {other}; add executable N-1 reader or migration validation before claiming it",
                    record.id
                ));
            }
        }
    }

    finish(compatibility_errors).map(|()| CompatibilitySummary {
        verified_edge_count,
        no_previous_version_count,
    })
}

pub fn corrupt_inputs(root: &Path) -> Result<CorruptSummary, Vec<String>> {
    validate_all(root)?;
    let registry = load_registry(root)?;
    let mut errors = Vec::new();
    let mut mutation_count = 0;

    for record in &registry.schema {
        let schema = match load_schema_document(root, record) {
            Ok(schema) => schema,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        let valid_paths = match fixture_paths(root, &record.valid_dir, ".json") {
            Ok(paths) => paths,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        for path in valid_paths {
            let label = relative_path(root, &path);
            let text = match fs::read_to_string(&path) {
                Ok(text) => text,
                Err(error) => {
                    errors.push(format!(
                        "GOV-SCHEMA-0002: cannot read valid fixture {label}: {error}"
                    ));
                    continue;
                }
            };
            let value = match serde_json::from_str::<Value>(&text) {
                Ok(value) => value,
                Err(error) => {
                    errors.push(format!(
                        "GOV-SCHEMA-0010: cannot mutate invalid baseline {label}: {error}"
                    ));
                    continue;
                }
            };

            mutation_count += 1;
            let mut truncated = text.trim_end().as_bytes().to_vec();
            truncated.pop();
            if serde_json::from_slice::<Value>(&truncated).is_ok() {
                errors.push(format!(
                    "GOV-SCHEMA-0010: truncation mutation unexpectedly parses for {label}"
                ));
            }

            mutation_count += 1;
            let trailing = format!("{}{{}}", text.trim_end());
            if serde_json::from_str::<Value>(&trailing).is_ok() {
                errors.push(format!(
                    "GOV-SCHEMA-0010: trailing-data mutation unexpectedly parses for {label}"
                ));
            }

            mutation_count += 1;
            let mut wrong_marker = value.clone();
            set_object_field(
                &mut wrong_marker,
                "schema",
                Value::String(format!("{}.corrupt", record.marker)),
            );
            expect_schema_rejection(
                record,
                &schema,
                &wrong_marker,
                "wrong marker",
                &label,
                &mut errors,
            );

            mutation_count += 1;
            let mut missing = value.clone();
            remove_first_required(&schema, &mut missing);
            expect_schema_rejection(
                record,
                &schema,
                &missing,
                "missing required field",
                &label,
                &mut errors,
            );

            mutation_count += 1;
            let mut unknown = value.clone();
            set_object_field(&mut unknown, "unknown_core", Value::Bool(true));
            expect_schema_rejection(
                record,
                &schema,
                &unknown,
                "unknown core field",
                &label,
                &mut errors,
            );

            if record.unknown_fields == "NamespacedExtensions" {
                mutation_count += 1;
                let mut extended = value.clone();
                set_object_field(&mut extended, "x-gov-0106", Value::Bool(true));
                expect_schema_and_reader_acceptance(
                    record,
                    &schema,
                    &extended,
                    "namespaced extension",
                    &label,
                    &mut errors,
                );
            }

            if record.missing_fields == "ReaderDefault" {
                mutation_count += 1;
                let mut defaulted = value.clone();
                if !remove_reference_source_kind(&mut defaulted) {
                    errors.push(format!(
                        "GOV-SCHEMA-0010: {label} has no references[].source_kind field to prove the registered reader default"
                    ));
                } else {
                    expect_schema_and_reader_acceptance(
                        record,
                        &schema,
                        &defaulted,
                        "reader default",
                        &label,
                        &mut errors,
                    );
                }
            }
        }

        if record.canonical {
            let canonical_paths = match fixture_paths(root, &record.canonical_dir, ".bin") {
                Ok(paths) => paths,
                Err(error) => {
                    errors.push(error);
                    continue;
                }
            };
            for path in canonical_paths {
                mutation_count += 1;
                match fs::read(&path) {
                    Ok(mut bytes) => {
                        if bytes.last() == Some(&b'\n') {
                            bytes.pop();
                        }
                        if canonical_encoding_errors(record, &bytes).is_empty() {
                            errors.push(format!(
                                "GOV-SCHEMA-0010: newline mutation unexpectedly remains canonical for {}",
                                relative_path(root, &path)
                            ));
                        }
                    }
                    Err(error) => errors.push(format!(
                        "GOV-SCHEMA-0002: cannot read canonical fixture {}: {error}",
                        relative_path(root, &path)
                    )),
                }
            }
        }
    }

    finish(errors).map(|()| CorruptSummary { mutation_count })
}

fn load_registry(root: &Path) -> Result<SchemaRegistry, Vec<String>> {
    let text = fs::read_to_string(root.join(REGISTRY_PATH)).map_err(|error| {
        vec![format!(
            "GOV-SCHEMA-0002: cannot read {REGISTRY_PATH}: {error}"
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-SCHEMA-0011: invalid schema registry {REGISTRY_PATH}: {error}"
        )]
    })
}

fn validate_registry(
    root: &Path,
    registry: &SchemaRegistry,
    protocols: &ProtocolRecords,
) -> Vec<String> {
    let mut errors = Vec::new();
    if registry.schema_version != 1 {
        errors.push(format!(
            "GOV-SCHEMA-0011: unsupported schema_version {}; expected 1",
            registry.schema_version
        ));
    }
    if !is_date(&registry.updated) {
        errors.push("GOV-SCHEMA-0011: updated must be a YYYY-MM-DD date".to_owned());
    }
    if registry.policy != POLICY_PATH
        || registry.protocol_inventory != PROTOCOL_INVENTORY_PATH
        || registry.schema_root != "schemas"
    {
        errors.push(format!(
            "GOV-SCHEMA-0011: registry must reference {POLICY_PATH}, {PROTOCOL_INVENTORY_PATH}, and schemas"
        ));
    }
    for path in [
        &registry.policy,
        &registry.protocol_inventory,
        &registry.schema_root,
    ] {
        validate_repository_path(root, path, true, &mut errors);
    }
    validate_version_policy(&registry.version_policy, &mut errors);

    let expected_json = protocols
        .iter()
        .filter(|(_, protocol)| {
            protocol.visibility == "Public"
                && protocol.implemented
                && protocol.public_schema
                && protocol.category == "JSON"
        })
        .map(|(id, _)| id.as_str())
        .collect::<BTreeSet<_>>();
    let expected_non_json = protocols
        .iter()
        .filter(|(_, protocol)| {
            protocol.visibility == "Public"
                && protocol.implemented
                && protocol.public_schema
                && protocol.category != "JSON"
        })
        .map(|(id, _)| id.as_str())
        .collect::<BTreeSet<_>>();

    let mut ids = BTreeSet::new();
    let mut names_and_versions = BTreeSet::new();
    let mut protocol_ids = BTreeSet::new();
    let mut markers = BTreeSet::new();
    for record in &registry.schema {
        if !valid_id(&record.id, "SCHEMA-") || !ids.insert(record.id.as_str()) {
            errors.push(format!(
                "GOV-SCHEMA-0001: invalid or duplicate schema id {}",
                display_id(&record.id)
            ));
        }
        if !valid_name(&record.name)
            || !names_and_versions.insert((record.name.as_str(), record.version.as_str()))
        {
            errors.push(format!(
                "GOV-SCHEMA-0001: invalid or duplicate schema name/version {:?}/{}",
                record.name, record.version
            ));
        }
        if !protocol_ids.insert(record.protocol_id.as_str()) {
            errors.push(format!(
                "GOV-SCHEMA-0001: duplicate protocol schema {}",
                display_id(&record.protocol_id)
            ));
        }
        if record.marker.is_empty() || !markers.insert(record.marker.as_str()) {
            errors.push(format!(
                "GOV-SCHEMA-0001: empty or duplicate marker for {}",
                display_id(&record.id)
            ));
        }
        validate_schema_record(root, record, protocols, &mut errors);
    }

    let actual_json = registry
        .schema
        .iter()
        .map(|record| record.protocol_id.as_str())
        .collect::<BTreeSet<_>>();
    if actual_json != expected_json {
        errors.push(format!(
            "GOV-SCHEMA-0003: JSON schema protocols differ from the protocol inventory; expected {expected_json:?}, found {actual_json:?}"
        ));
    }

    let mut actual_non_json = BTreeSet::new();
    for boundary in &registry.non_json_boundary {
        if !actual_non_json.insert(boundary.protocol_id.as_str()) {
            errors.push(format!(
                "GOV-SCHEMA-0001: duplicate non-JSON boundary {}",
                display_id(&boundary.protocol_id)
            ));
        }
        match protocols.get(&boundary.protocol_id) {
            Some(protocol)
                if protocol.visibility == "Public"
                    && protocol.implemented
                    && protocol.public_schema
                    && protocol.category != "JSON" => {}
            Some(_) => errors.push(format!(
                "GOV-SCHEMA-0003: non-JSON boundary {} disagrees with the protocol inventory",
                display_id(&boundary.protocol_id)
            )),
            None => errors.push(format!(
                "GOV-SCHEMA-0003: non-JSON boundary {} is absent from the protocol inventory",
                display_id(&boundary.protocol_id)
            )),
        }
        if boundary.format != "CanonicalText"
            || boundary.reason.trim().is_empty()
            || boundary.evidence.is_empty()
        {
            errors.push(format!(
                "GOV-SCHEMA-0011: non-JSON boundary {} needs CanonicalText format, a reason, and evidence",
                display_id(&boundary.protocol_id)
            ));
        }
        validate_unique_paths(root, &boundary.evidence, "boundary evidence", &mut errors);
    }
    if actual_non_json != expected_non_json {
        errors.push(format!(
            "GOV-SCHEMA-0003: non-JSON schema boundaries differ from the protocol inventory; expected {expected_non_json:?}, found {actual_non_json:?}"
        ));
    }
    errors
}

fn validate_version_policy(policy: &VersionPolicy, errors: &mut Vec<String>) {
    if policy.writer != "CurrentOnly" {
        errors.push("GOV-SCHEMA-0011: version_policy.writer must be CurrentOnly".to_owned());
    }
    for (field, value) in [
        ("major_minor", &policy.major_minor),
        ("integer", &policy.integer),
        ("n_minus_one", &policy.n_minus_one),
        ("semantic_identity", &policy.semantic_identity),
    ] {
        if value.trim().is_empty() {
            errors.push(format!(
                "GOV-SCHEMA-0011: version_policy.{field} must not be empty"
            ));
        }
    }
}

fn validate_schema_record(
    root: &Path,
    record: &SchemaRecord,
    protocols: &ProtocolRecords,
    errors: &mut Vec<String>,
) {
    match protocols.get(&record.protocol_id) {
        Some(protocol)
            if protocol.category == "JSON"
                && protocol.visibility == "Public"
                && protocol.implemented
                && protocol.public_schema
                && protocol.current_version == record.marker
                && protocol.stability == record.stability
                && protocol.canonical == record.canonical => {}
        Some(_) => errors.push(format!(
            "GOV-SCHEMA-0003: {} disagrees with protocol {}",
            display_id(&record.id),
            display_id(&record.protocol_id)
        )),
        None => errors.push(format!(
            "GOV-SCHEMA-0003: {} references unknown protocol {}",
            display_id(&record.id),
            display_id(&record.protocol_id)
        )),
    }
    let valid_version = match record.version_kind.as_str() {
        "MajorMinor" => valid_major_minor(&record.version),
        "Integer" => valid_integer_version(&record.version),
        _ => false,
    };
    if !valid_version {
        errors.push(format!(
            "GOV-SCHEMA-0011: {} has an invalid declared version kind or value",
            display_id(&record.id),
        ));
    }
    if !record.marker.ends_with(&format!("/{}", record.version)) {
        errors.push(format!(
            "GOV-SCHEMA-0003: {} marker {} does not end in its version {}",
            display_id(&record.id),
            record.marker,
            record.version
        ));
    }
    if !matches!(
        record.stability.as_str(),
        "Experimental" | "Preview" | "Stable"
    ) || record.format != "JSON"
        || record.writer != "CurrentOnly"
    {
        errors.push(format!(
            "GOV-SCHEMA-0011: {} has an invalid stability, format, or writer policy",
            display_id(&record.id)
        ));
    }
    match record.reader.as_str() {
        "None" if record.reader_versions.is_empty() && record.reader_adapter == "None" => {}
        "CurrentOnly"
            if record.reader_versions == [record.marker.clone()]
                && matches!(
                    (record.marker.as_str(), record.reader_adapter.as_str()),
                    ("ling.semantic/0.1", "SemanticGraphV0_1")
                        | ("ling.semantic/0.2", "SemanticGraphV0_2")
                        | ("ling.lock/1", "LockFileV1")
                ) => {}
        _ => errors.push(format!(
            "GOV-SCHEMA-0011: {} has an unsupported or inconsistent reader declaration",
            display_id(&record.id)
        )),
    }
    if record.compatibility != "NoPreviousVersion"
        || !record.previous_version.is_empty()
        || !record.previous_marker.is_empty()
        || !record.compatibility_dir.is_empty()
        || record.migration_adapter != "None"
    {
        errors.push(format!(
            "GOV-SCHEMA-0009: {} may claim only NoPreviousVersion until an executable edge is implemented",
            display_id(&record.id)
        ));
    }
    if !matches!(
        record.unknown_fields.as_str(),
        "Reject" | "NamespacedExtensions" | "Unspecified"
    ) || !matches!(
        record.missing_fields.as_str(),
        "RejectRequired" | "ReaderDefault" | "Unspecified"
    ) {
        errors.push(format!(
            "GOV-SCHEMA-0011: {} has an invalid unknown- or missing-field policy",
            display_id(&record.id)
        ));
    }
    if record.missing_fields == "ReaderDefault" {
        if record.reader_defaults.is_empty() || record.reader == "None" {
            errors.push(format!(
                "GOV-SCHEMA-0008: {} declares ReaderDefault without defaults and a reader",
                display_id(&record.id)
            ));
        }
    } else if !record.reader_defaults.is_empty() {
        errors.push(format!(
            "GOV-SCHEMA-0011: {} lists reader defaults without ReaderDefault policy",
            display_id(&record.id)
        ));
    }
    if record.reader_adapter == "SemanticGraphV0_1"
        && record.reader_defaults != ["references[].source_kind=expression"]
    {
        errors.push(format!(
            "GOV-SCHEMA-0008: {} must register the Semantic Graph reader's executable source_kind default",
            display_id(&record.id)
        ));
    }
    validate_unique_values(
        &record.reader_defaults,
        &record.id,
        "reader_defaults",
        errors,
    );
    validate_unique_values(
        &record.hash_scheme_ids,
        &record.id,
        "hash_scheme_ids",
        errors,
    );

    let package_root = format!("schemas/{}/{}", record.name, record.version);
    if record.schema_path != format!("{package_root}/schema.json")
        || record.valid_dir != format!("{package_root}/valid")
        || record.invalid_dir != format!("{package_root}/invalid")
    {
        errors.push(format!(
            "GOV-SCHEMA-0002: {} paths do not match schemas/<name>/<version>",
            display_id(&record.id)
        ));
    }
    for path in [&record.schema_path, &record.valid_dir, &record.invalid_dir] {
        validate_repository_path(root, path, true, errors);
    }
    validate_path_kind(root, &record.schema_path, false, errors);
    validate_path_kind(root, &record.valid_dir, true, errors);
    validate_path_kind(root, &record.invalid_dir, true, errors);
    validate_fixture_directory(root, &record.valid_dir, &[".json"], errors);
    validate_fixture_directory(
        root,
        &record.invalid_dir,
        &[".json", ".expect.toml"],
        errors,
    );
    if record.canonical {
        if record.canonical_encoding != "CompactJsonLf"
            || record.canonical_dir != format!("{package_root}/canonical")
            || record.hash_scheme_ids.is_empty()
        {
            errors.push(format!(
                "GOV-SCHEMA-0007: canonical schema {} needs CompactJsonLf, a canonical directory, and hash scheme IDs",
                display_id(&record.id)
            ));
        }
        validate_repository_path(root, &record.canonical_dir, true, errors);
        validate_path_kind(root, &record.canonical_dir, true, errors);
        validate_fixture_directory(root, &record.canonical_dir, &[".bin"], errors);
    } else if record.canonical_encoding != "None"
        || !record.canonical_dir.is_empty()
        || !record.hash_scheme_ids.is_empty()
    {
        errors.push(format!(
            "GOV-SCHEMA-0007: non-canonical schema {} claims canonical encoding, fixtures, or hash schemes",
            display_id(&record.id)
        ));
    }
    if record.writer_evidence.is_empty() {
        errors.push(format!(
            "GOV-SCHEMA-0005: {} has no writer evidence",
            display_id(&record.id)
        ));
    }
    validate_unique_paths(root, &record.writer_evidence, "writer evidence", errors);
    for scheme in &record.hash_scheme_ids {
        let present = record.writer_evidence.iter().any(|path| {
            fs::read_to_string(root.join(path)).is_ok_and(|text| text.contains(scheme))
        });
        if !present {
            errors.push(format!(
                "GOV-SCHEMA-0007: {} hash scheme {scheme:?} is absent from writer evidence",
                display_id(&record.id)
            ));
        }
    }
}

fn load_schema_document(root: &Path, record: &SchemaRecord) -> Result<Value, String> {
    let text = fs::read_to_string(root.join(&record.schema_path)).map_err(|error| {
        format!(
            "GOV-SCHEMA-0002: cannot read {}: {error}",
            record.schema_path
        )
    })?;
    serde_json::from_str(&text).map_err(|error| {
        format!(
            "GOV-SCHEMA-0004: invalid JSON Schema {}: {error}",
            record.schema_path
        )
    })
}

fn validate_schema_document(record: &SchemaRecord, schema: &Value, errors: &mut Vec<String>) {
    let Some(object) = schema.as_object() else {
        errors.push(format!(
            "GOV-SCHEMA-0004: {} must contain an object JSON Schema",
            record.schema_path
        ));
        return;
    };
    if object.get("$schema").and_then(Value::as_str) != Some(JSON_SCHEMA_DIALECT) {
        errors.push(format!(
            "GOV-SCHEMA-0004: {} must declare Draft 2020-12",
            record.schema_path
        ));
    }
    let expected_id = format!("urn:ling:schema:{}:{}", record.name, record.version);
    if object.get("$id").and_then(Value::as_str) != Some(expected_id.as_str()) {
        errors.push(format!(
            "GOV-SCHEMA-0004: {} must use $id {expected_id}",
            record.schema_path
        ));
    }
    if object.get("type").and_then(Value::as_str) != Some("object") {
        errors.push(format!(
            "GOV-SCHEMA-0004: {} root type must be object",
            record.schema_path
        ));
    }
    let marker_field = if record.marker == "ling.lock/1" {
        "format"
    } else {
        "schema"
    };
    let marker = object
        .get("properties")
        .and_then(Value::as_object)
        .and_then(|properties| properties.get(marker_field))
        .and_then(Value::as_object)
        .and_then(|schema| schema.get("const"))
        .and_then(Value::as_str);
    if marker != Some(record.marker.as_str()) {
        errors.push(format!(
            "GOV-SCHEMA-0004: {} must constrain {marker_field} to {}",
            record.schema_path, record.marker,
        ));
    }
    lint_schema(schema, schema, "$", errors);
}

fn lint_schema(schema: &Value, root_schema: &Value, path: &str, errors: &mut Vec<String>) {
    if schema.is_boolean() {
        return;
    }
    let Some(object) = schema.as_object() else {
        errors.push(format!(
            "GOV-SCHEMA-0004: schema node {path} must be an object or boolean"
        ));
        return;
    };
    for key in object.keys() {
        if !SUPPORTED_SCHEMA_KEYWORDS.contains(&key.as_str()) {
            errors.push(format!(
                "GOV-SCHEMA-0004: unsupported JSON Schema keyword {key:?} at {path}"
            ));
        }
    }
    if let Some(reference) = object.get("$ref") {
        match reference.as_str() {
            Some(reference) if resolve_reference(root_schema, reference).is_some() => {}
            Some(reference) => errors.push(format!(
                "GOV-SCHEMA-0004: unresolved or non-local $ref {reference:?} at {path}"
            )),
            None => errors.push(format!("GOV-SCHEMA-0004: $ref at {path} must be a string")),
        }
    }
    if let Some(types) = object.get("type") {
        let valid = match types {
            Value::String(kind) => valid_json_type(kind),
            Value::Array(kinds) => {
                !kinds.is_empty()
                    && kinds
                        .iter()
                        .all(|kind| kind.as_str().is_some_and(valid_json_type))
            }
            _ => false,
        };
        if !valid {
            errors.push(format!(
                "GOV-SCHEMA-0004: invalid type declaration at {path}"
            ));
        }
    }
    if let Some(required) = object.get("required") {
        let values = required.as_array();
        if values.is_none_or(|values| {
            values.is_empty()
                || values.iter().any(|value| value.as_str().is_none())
                || values
                    .iter()
                    .filter_map(Value::as_str)
                    .collect::<BTreeSet<_>>()
                    .len()
                    != values.len()
        }) {
            errors.push(format!(
                "GOV-SCHEMA-0004: required at {path} must be a non-empty unique string array"
            ));
        }
    }
    if object
        .get("minimum")
        .is_some_and(|value| !value.is_number())
        || object
            .get("minLength")
            .is_some_and(|value| value.as_u64().is_none())
        || object
            .get("enum")
            .is_some_and(|value| value.as_array().is_none_or(Vec::is_empty))
    {
        errors.push(format!(
            "GOV-SCHEMA-0004: invalid numeric, length, or enum constraint at {path}"
        ));
    }
    for keyword in ["properties", "$defs"] {
        if let Some(children) = object.get(keyword) {
            match children.as_object() {
                Some(children) => {
                    for (name, child) in children {
                        lint_schema(
                            child,
                            root_schema,
                            &format!("{path}/{keyword}/{name}"),
                            errors,
                        );
                    }
                }
                None => errors.push(format!(
                    "GOV-SCHEMA-0004: {keyword} at {path} must be an object"
                )),
            }
        }
    }
    if let Some(patterns) = object.get("patternProperties") {
        match patterns.as_object() {
            Some(patterns) => {
                for (pattern, child) in patterns {
                    if pattern != "^x-" {
                        errors.push(format!(
                            "GOV-SCHEMA-0004: only the reviewed ^x- extension pattern is supported at {path}"
                        ));
                    }
                    lint_schema(
                        child,
                        root_schema,
                        &format!("{path}/patternProperties/{pattern}"),
                        errors,
                    );
                }
            }
            None => errors.push(format!(
                "GOV-SCHEMA-0004: patternProperties at {path} must be an object"
            )),
        }
    }
    for keyword in ["items", "additionalProperties"] {
        if let Some(child) = object.get(keyword) {
            if child.is_boolean() || child.is_object() {
                lint_schema(child, root_schema, &format!("{path}/{keyword}"), errors);
            } else {
                errors.push(format!(
                    "GOV-SCHEMA-0004: {keyword} at {path} must be a schema"
                ));
            }
        }
    }
}

fn validate_valid_fixtures(
    root: &Path,
    record: &SchemaRecord,
    schema: &Value,
    errors: &mut Vec<String>,
) -> usize {
    let paths = match fixture_paths(root, &record.valid_dir, ".json") {
        Ok(paths) => paths,
        Err(error) => {
            errors.push(error);
            return 0;
        }
    };
    if paths.is_empty() {
        errors.push(format!(
            "GOV-SCHEMA-0005: {} has no valid JSON fixtures",
            record.id
        ));
    }
    for path in &paths {
        let label = relative_path(root, path);
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!(
                    "GOV-SCHEMA-0002: cannot read valid fixture {label}: {error}"
                ));
                continue;
            }
        };
        let value = match serde_json::from_str::<Value>(&text) {
            Ok(value) => value,
            Err(error) => {
                errors.push(format!(
                    "GOV-SCHEMA-0005: valid fixture {label} is invalid JSON: {error}"
                ));
                continue;
            }
        };
        let shape_errors = instance_errors(&value, schema);
        if !shape_errors.is_empty() {
            errors.push(format!(
                "GOV-SCHEMA-0005: valid fixture {label} violates its schema: {}",
                shape_errors.join("; ")
            ));
        }
        if let Err(error) = reader_accepts(record, &text) {
            errors.push(format!(
                "GOV-SCHEMA-0008: reader rejected valid fixture {label}: {error}"
            ));
        }
    }
    paths.len()
}

fn validate_invalid_fixtures(
    root: &Path,
    record: &SchemaRecord,
    schema: &Value,
    errors: &mut Vec<String>,
) -> usize {
    let paths = match fixture_paths(root, &record.invalid_dir, ".json") {
        Ok(paths) => paths,
        Err(error) => {
            errors.push(error);
            return 0;
        }
    };
    if paths.is_empty() {
        errors.push(format!(
            "GOV-SCHEMA-0006: {} has no invalid JSON fixtures",
            record.id
        ));
    }
    let expected_sidecars = paths
        .iter()
        .map(|path| path.with_extension("expect.toml"))
        .collect::<BTreeSet<_>>();
    match fixture_paths(root, &record.invalid_dir, ".expect.toml") {
        Ok(actual_sidecars) => {
            for path in actual_sidecars {
                if !expected_sidecars.contains(&path) {
                    errors.push(format!(
                        "GOV-SCHEMA-0006: orphan invalid-fixture expectation {}",
                        relative_path(root, &path)
                    ));
                }
            }
        }
        Err(error) => errors.push(error),
    }

    for path in &paths {
        validate_invalid_fixture(root, record, schema, path, errors);
    }
    paths.len()
}

fn validate_invalid_fixture(
    root: &Path,
    record: &SchemaRecord,
    schema: &Value,
    path: &Path,
    errors: &mut Vec<String>,
) {
    let label = relative_path(root, path);
    let sidecar = path.with_extension("expect.toml");
    let expectation = match fs::read_to_string(&sidecar) {
        Ok(text) => match toml::from_str::<InvalidExpectation>(&text) {
            Ok(expectation) => expectation,
            Err(error) => {
                errors.push(format!(
                    "GOV-SCHEMA-0006: invalid expectation {}: {error}",
                    relative_path(root, &sidecar)
                ));
                return;
            }
        },
        Err(error) => {
            errors.push(format!(
                "GOV-SCHEMA-0006: cannot read expectation {}: {error}",
                relative_path(root, &sidecar)
            ));
            return;
        }
    };
    if expectation.schema_version != 1
        || !matches!(
            expectation.expected.as_str(),
            "InvalidJson" | "SchemaViolation" | "ReaderViolation"
        )
    {
        errors.push(format!(
            "GOV-SCHEMA-0006: expectation for {label} has an unsupported version or class"
        ));
        return;
    }
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            errors.push(format!(
                "GOV-SCHEMA-0002: cannot read invalid fixture {label}: {error}"
            ));
            return;
        }
    };
    let parsed = serde_json::from_str::<Value>(&text);
    match expectation.expected.as_str() {
        "InvalidJson" => {
            if parsed.is_ok() {
                errors.push(format!(
                    "GOV-SCHEMA-0006: {label} expected InvalidJson but parsed"
                ));
            }
        }
        "SchemaViolation" => match parsed {
            Ok(value) if instance_errors(&value, schema).is_empty() => errors.push(format!(
                "GOV-SCHEMA-0006: {label} expected SchemaViolation but passed"
            )),
            Ok(_) => {}
            Err(error) => errors.push(format!(
                "GOV-SCHEMA-0006: {label} expected SchemaViolation but is invalid JSON: {error}"
            )),
        },
        "ReaderViolation" => match parsed {
            Ok(value) => {
                let shape_errors = instance_errors(&value, schema);
                if !shape_errors.is_empty() {
                    errors.push(format!(
                        "GOV-SCHEMA-0006: {label} expected ReaderViolation but violates the schema: {}",
                        shape_errors.join("; ")
                    ));
                } else if reader_accepts(record, &text).is_ok() {
                    errors.push(format!(
                        "GOV-SCHEMA-0006: {label} expected ReaderViolation but the reader accepted it"
                    ));
                }
            }
            Err(error) => errors.push(format!(
                "GOV-SCHEMA-0006: {label} expected ReaderViolation but is invalid JSON: {error}"
            )),
        },
        _ => unreachable!("expectation class checked above"),
    }
}

fn validate_canonical_fixtures(
    root: &Path,
    record: &SchemaRecord,
    schema: &Value,
    errors: &mut Vec<String>,
) -> usize {
    if !record.canonical {
        return 0;
    }
    let paths = match fixture_paths(root, &record.canonical_dir, ".bin") {
        Ok(paths) => paths,
        Err(error) => {
            errors.push(error);
            return 0;
        }
    };
    if paths.is_empty() {
        errors.push(format!(
            "GOV-SCHEMA-0007: canonical schema {} has no byte golden",
            record.id
        ));
    }
    for path in &paths {
        let label = relative_path(root, path);
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) => {
                errors.push(format!(
                    "GOV-SCHEMA-0002: cannot read canonical fixture {label}: {error}"
                ));
                continue;
            }
        };
        for error in canonical_encoding_errors(record, &bytes) {
            errors.push(format!("GOV-SCHEMA-0007: {label}: {error}"));
        }
        let text = match std::str::from_utf8(&bytes) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!("GOV-SCHEMA-0007: {label} is not UTF-8: {error}"));
                continue;
            }
        };
        let value = match serde_json::from_str::<Value>(text) {
            Ok(value) => value,
            Err(error) => {
                errors.push(format!("GOV-SCHEMA-0007: {label} is not JSON: {error}"));
                continue;
            }
        };
        let shape_errors = instance_errors(&value, schema);
        if !shape_errors.is_empty() {
            errors.push(format!(
                "GOV-SCHEMA-0007: {label} violates its schema: {}",
                shape_errors.join("; ")
            ));
        }
        if let Err(error) = reader_accepts(record, text) {
            errors.push(format!(
                "GOV-SCHEMA-0008: reader rejected canonical fixture {label}: {error}"
            ));
        }
        let canonical_root = root.join(&record.canonical_dir);
        let relative = path.strip_prefix(&canonical_root).unwrap_or(path);
        let mut valid_relative = relative.to_path_buf();
        valid_relative.set_extension("json");
        let valid_path = root.join(&record.valid_dir).join(valid_relative);
        match fs::read(&valid_path) {
            Ok(valid_bytes) if valid_bytes == bytes => {}
            Ok(_) => errors.push(format!(
                "GOV-SCHEMA-0007: {label} differs from writer fixture {}",
                relative_path(root, &valid_path)
            )),
            Err(error) => errors.push(format!(
                "GOV-SCHEMA-0007: {label} has no matching valid fixture {}: {error}",
                relative_path(root, &valid_path)
            )),
        }
    }
    paths.len()
}

fn instance_errors(instance: &Value, schema: &Value) -> Vec<String> {
    let mut errors = Vec::new();
    validate_instance(instance, schema, schema, "$", &mut errors);
    errors
}

fn validate_instance(
    instance: &Value,
    schema: &Value,
    root_schema: &Value,
    path: &str,
    errors: &mut Vec<String>,
) {
    match schema {
        Value::Bool(true) => return,
        Value::Bool(false) => {
            errors.push(format!("{path} is rejected by a false schema"));
            return;
        }
        Value::Object(_) => {}
        _ => {
            errors.push(format!("invalid schema node at {path}"));
            return;
        }
    }
    let object = schema.as_object().expect("object checked above");
    if let Some(reference) = object.get("$ref").and_then(Value::as_str) {
        match resolve_reference(root_schema, reference) {
            Some(target) => validate_instance(instance, target, root_schema, path, errors),
            None => errors.push(format!("unresolved $ref {reference} at {path}")),
        }
    }
    if let Some(types) = object.get("type") {
        let matches = match types {
            Value::String(kind) => instance_has_type(instance, kind),
            Value::Array(kinds) => kinds
                .iter()
                .filter_map(Value::as_str)
                .any(|kind| instance_has_type(instance, kind)),
            _ => false,
        };
        if !matches {
            errors.push(format!("{path} has the wrong JSON type"));
            return;
        }
    }
    if object
        .get("const")
        .is_some_and(|expected| instance != expected)
    {
        errors.push(format!("{path} does not equal const"));
    }
    if object
        .get("enum")
        .and_then(Value::as_array)
        .is_some_and(|values| !values.iter().any(|expected| expected == instance))
    {
        errors.push(format!("{path} is not an allowed enum value"));
    }
    if let (Some(value), Some(minimum)) = (
        instance.as_f64(),
        object.get("minimum").and_then(Value::as_f64),
    ) {
        if value < minimum {
            errors.push(format!("{path} is below minimum {minimum}"));
        }
    }
    if let (Some(value), Some(minimum)) = (
        instance.as_str(),
        object.get("minLength").and_then(Value::as_u64),
    ) {
        if u64::try_from(value.chars().count()).is_ok_and(|length| length < minimum) {
            errors.push(format!("{path} is shorter than minLength {minimum}"));
        }
    }
    if let Some(instance) = instance.as_object() {
        if let Some(required) = object.get("required").and_then(Value::as_array) {
            for field in required.iter().filter_map(Value::as_str) {
                if !instance.contains_key(field) {
                    errors.push(format!("{path} is missing required field {field}"));
                }
            }
        }
        let properties = object.get("properties").and_then(Value::as_object);
        let patterns = object.get("patternProperties").and_then(Value::as_object);
        for (field, value) in instance {
            let mut matched = false;
            if let Some(child) = properties.and_then(|properties| properties.get(field)) {
                matched = true;
                validate_instance(
                    value,
                    child,
                    root_schema,
                    &format!("{path}.{field}"),
                    errors,
                );
            }
            if field.starts_with("x-") {
                if let Some(child) = patterns.and_then(|patterns| patterns.get("^x-")) {
                    matched = true;
                    validate_instance(
                        value,
                        child,
                        root_schema,
                        &format!("{path}.{field}"),
                        errors,
                    );
                }
            }
            if !matched {
                match object.get("additionalProperties") {
                    Some(Value::Bool(false)) => {
                        errors.push(format!("{path} has unknown field {field}"));
                    }
                    Some(child @ Value::Object(_)) => validate_instance(
                        value,
                        child,
                        root_schema,
                        &format!("{path}.{field}"),
                        errors,
                    ),
                    _ => {}
                }
            }
        }
    }
    if let (Some(instance), Some(items)) = (instance.as_array(), object.get("items")) {
        for (index, value) in instance.iter().enumerate() {
            validate_instance(
                value,
                items,
                root_schema,
                &format!("{path}[{index}]"),
                errors,
            );
        }
    }
}

fn reader_accepts(record: &SchemaRecord, input: &str) -> Result<(), String> {
    match record.reader_adapter.as_str() {
        "None" => Ok(()),
        "SemanticGraphV0_1" => ling_semantic::read_json(input)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "SemanticGraphV0_2" => ling_semantic::read_project_json(input)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "LockFileV1" => {
            ling_project::parse_lock_file(ling_project::LOCK_FILE_NAME, input.as_bytes())
                .map(|_| ())
                .map_err(|error| error.to_string())
        }
        adapter => Err(format!("unsupported reader adapter {adapter}")),
    }
}

fn expect_schema_rejection(
    _record: &SchemaRecord,
    schema: &Value,
    value: &Value,
    mutation: &str,
    label: &str,
    errors: &mut Vec<String>,
) {
    if instance_errors(value, schema).is_empty() {
        errors.push(format!(
            "GOV-SCHEMA-0010: {mutation} mutation unexpectedly satisfies the schema for {label}"
        ));
    }
}

fn expect_schema_and_reader_acceptance(
    record: &SchemaRecord,
    schema: &Value,
    value: &Value,
    mutation: &str,
    label: &str,
    errors: &mut Vec<String>,
) {
    let shape_errors = instance_errors(value, schema);
    if !shape_errors.is_empty() {
        errors.push(format!(
            "GOV-SCHEMA-0010: {mutation} mutation violates the schema for {label}: {}",
            shape_errors.join("; ")
        ));
        return;
    }
    let text = match serde_json::to_string(value) {
        Ok(text) => text,
        Err(error) => {
            errors.push(format!(
                "GOV-SCHEMA-0010: cannot serialize {mutation} mutation for {label}: {error}"
            ));
            return;
        }
    };
    if let Err(error) = reader_accepts(record, &text) {
        errors.push(format!(
            "GOV-SCHEMA-0010: reader rejected {mutation} mutation for {label}: {error}"
        ));
    }
}

fn set_object_field(value: &mut Value, field: &str, replacement: Value) {
    if let Some(object) = value.as_object_mut() {
        object.insert(field.to_owned(), replacement);
    }
}

fn remove_first_required(schema: &Value, value: &mut Value) {
    let field = schema
        .get("required")
        .and_then(Value::as_array)
        .and_then(|required| required.first())
        .and_then(Value::as_str);
    if let (Some(field), Some(object)) = (field, value.as_object_mut()) {
        object.remove(field);
    }
}

fn remove_reference_source_kind(value: &mut Value) -> bool {
    value
        .get_mut("references")
        .and_then(Value::as_array_mut)
        .and_then(|references| references.first_mut())
        .and_then(Value::as_object_mut)
        .and_then(|reference| reference.remove("source_kind"))
        .is_some()
}

fn canonical_encoding_errors(record: &SchemaRecord, bytes: &[u8]) -> Vec<String> {
    let mut errors = Vec::new();
    if record.canonical_encoding != "CompactJsonLf" {
        errors.push(format!(
            "unsupported canonical encoding {}",
            record.canonical_encoding
        ));
        return errors;
    }
    if bytes.last() != Some(&b'\n')
        || bytes.len() >= 2 && bytes.get(bytes.len() - 2) == Some(&b'\n')
    {
        errors.push("canonical bytes must end in exactly one LF".to_owned());
    }
    if bytes.contains(&b'\r') {
        errors.push("canonical bytes must not contain CR".to_owned());
    }
    let content = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    if contains_json_whitespace_outside_strings(content) {
        errors.push("canonical JSON must contain no insignificant whitespace".to_owned());
    }
    errors
}

fn contains_json_whitespace_outside_strings(bytes: &[u8]) -> bool {
    let mut in_string = false;
    let mut escaped = false;
    for &byte in bytes {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
        } else if byte == b'"' {
            in_string = true;
        } else if byte.is_ascii_whitespace() {
            return true;
        }
    }
    false
}

fn resolve_reference<'a>(root: &'a Value, reference: &str) -> Option<&'a Value> {
    let name = reference.strip_prefix("#/$defs/")?;
    if name.is_empty() || name.contains('/') || name.contains('~') {
        return None;
    }
    root.get("$defs")?.get(name)
}

fn valid_json_type(kind: &str) -> bool {
    matches!(
        kind,
        "array" | "boolean" | "integer" | "null" | "number" | "object" | "string"
    )
}

fn instance_has_type(value: &Value, kind: &str) -> bool {
    match kind {
        "array" => value.is_array(),
        "boolean" => value.is_boolean(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "null" => value.is_null(),
        "number" => value.is_number(),
        "object" => value.is_object(),
        "string" => value.is_string(),
        _ => false,
    }
}

fn fixture_paths(root: &Path, directory: &str, suffix: &str) -> Result<Vec<PathBuf>, String> {
    let directory_path = root.join(directory);
    if !directory_path.is_dir() {
        return Err(format!(
            "GOV-SCHEMA-0002: fixture directory {directory} is missing"
        ));
    }
    let mut files = Vec::new();
    collect_files(&directory_path, &mut files)
        .map_err(|error| format!("GOV-SCHEMA-0002: cannot enumerate {directory}: {error}"))?;
    files.retain(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(suffix))
    });
    files.sort();
    Ok(files)
}

fn collect_files(directory: &Path, output: &mut Vec<PathBuf>) -> std::io::Result<()> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_files(&entry.path(), output)?;
        } else if file_type.is_file() {
            output.push(entry.path());
        }
    }
    Ok(())
}

fn validate_repository_path(
    root: &Path,
    value: &str,
    require_exists: bool,
    errors: &mut Vec<String>,
) {
    if !is_relative_path(value) {
        errors.push(format!(
            "GOV-SCHEMA-0002: invalid repository path {value:?}"
        ));
        return;
    }
    if require_exists && !root.join(value).exists() {
        errors.push(format!("GOV-SCHEMA-0002: missing repository path {value}"));
    }
}

fn validate_path_kind(root: &Path, value: &str, directory: bool, errors: &mut Vec<String>) {
    let path = root.join(value);
    let valid = if directory {
        path.is_dir()
    } else {
        path.is_file()
    };
    if !valid {
        errors.push(format!(
            "GOV-SCHEMA-0002: repository path {value} must be a {}",
            if directory { "directory" } else { "file" }
        ));
    }
}

fn validate_fixture_directory(
    root: &Path,
    directory: &str,
    allowed_suffixes: &[&str],
    errors: &mut Vec<String>,
) {
    let directory_path = root.join(directory);
    if !directory_path.is_dir() {
        return;
    }
    let mut files = Vec::new();
    if let Err(error) = collect_files(&directory_path, &mut files) {
        errors.push(format!(
            "GOV-SCHEMA-0002: cannot enumerate {directory}: {error}"
        ));
        return;
    }
    for path in files {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if !allowed_suffixes.iter().any(|suffix| name.ends_with(suffix)) {
            errors.push(format!(
                "GOV-SCHEMA-0002: unexpected fixture file {}",
                relative_path(root, &path)
            ));
        }
    }
}

fn validate_unique_paths(root: &Path, paths: &[String], field: &str, errors: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    for path in paths {
        if !seen.insert(path.as_str()) {
            errors.push(format!("GOV-SCHEMA-0001: duplicate {field} path {path}"));
        }
        validate_repository_path(root, path, true, errors);
    }
}

fn validate_unique_values(values: &[String], id: &str, field: &str, errors: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    for value in values {
        if value.trim().is_empty() || !seen.insert(value.as_str()) {
            errors.push(format!(
                "GOV-SCHEMA-0001: {id} has an empty or duplicate {field} entry"
            ));
        }
    }
}

fn valid_id(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        !suffix.is_empty()
            && !suffix.starts_with('-')
            && !suffix.ends_with('-')
            && suffix
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-')
    })
}

fn valid_name(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn valid_major_minor(value: &str) -> bool {
    value.split_once('.').is_some_and(|(major, minor)| {
        !major.is_empty()
            && !minor.is_empty()
            && !minor.contains('.')
            && major.bytes().all(|byte| byte.is_ascii_digit())
            && minor.bytes().all(|byte| byte.is_ascii_digit())
    })
}

fn valid_integer_version(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'))
}

fn is_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit())
}

fn is_relative_path(value: &str) -> bool {
    if value.is_empty() || value.contains('\\') {
        return false;
    }
    let path = Path::new(value);
    !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn display_id(value: &str) -> &str {
    if value.is_empty() {
        "<missing-id>"
    } else {
        value
    }
}

fn finish(mut errors: Vec<String>) -> Result<(), Vec<String>> {
    errors.sort();
    errors.dedup();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask")
    }

    #[test]
    fn repository_schema_corpus_is_valid_and_current() {
        let summary = validate_all(repository_root()).expect("schema corpus is valid");
        assert_eq!(summary.schema_count, 5);
        assert_eq!(summary.valid_fixture_count, 6);
        assert_eq!(summary.invalid_fixture_count, 19);
        assert_eq!(summary.canonical_fixture_count, 3);
    }

    #[test]
    fn repository_has_no_false_n_minus_one_claim() {
        let summary = compatibility(repository_root(), "N-1", "N")
            .expect("first-version compatibility state is valid");
        assert_eq!(summary.verified_edge_count, 0);
        assert_eq!(summary.no_previous_version_count, 5);
    }

    #[test]
    fn deterministic_corruptions_have_the_declared_outcomes() {
        let summary =
            corrupt_inputs(repository_root()).expect("corruptions are rejected correctly");
        assert!(summary.mutation_count >= 20);
    }

    #[test]
    fn validator_enforces_types_required_fields_and_extensions() {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["name"],
            "properties": { "name": { "type": "string", "minLength": 1 } },
            "patternProperties": { "^x-": true },
            "additionalProperties": false
        });
        assert!(instance_errors(&serde_json::json!({"name": "ling"}), &schema).is_empty());
        assert!(
            instance_errors(&serde_json::json!({"name": "ling", "x-test": 1}), &schema).is_empty()
        );
        assert!(!instance_errors(&serde_json::json!({"name": ""}), &schema).is_empty());
        assert!(
            !instance_errors(&serde_json::json!({"name": "ling", "typo": true}), &schema)
                .is_empty()
        );
    }

    #[test]
    fn linter_rejects_unreviewed_schema_keywords() {
        let schema = serde_json::json!({ "type": "array", "minItems": 1 });
        let mut errors = Vec::new();
        lint_schema(&schema, &schema, "$", &mut errors);
        assert!(errors.iter().any(|error| error.contains("minItems")));
    }

    #[test]
    fn registry_rejects_a_false_previous_version_claim() {
        let root = repository_root();
        let mut registry = load_registry(root).expect("registry parses");
        registry.schema[0].previous_version = "0.0".to_owned();
        let protocols = protocols::protocol_records(root).expect("protocol inventory is valid");
        let errors = validate_registry(root, &registry, &protocols);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("NoPreviousVersion"))
        );
    }

    #[test]
    fn compact_json_lf_rejects_whitespace_and_missing_newline() {
        let record = load_registry(repository_root())
            .expect("registry parses")
            .schema
            .into_iter()
            .find(|record| record.canonical)
            .expect("canonical schema exists");
        assert!(canonical_encoding_errors(&record, b"{\"a\":1}\n").is_empty());
        assert!(!canonical_encoding_errors(&record, b"{ \"a\":1}\n").is_empty());
        assert!(!canonical_encoding_errors(&record, b"{\"a\":1}").is_empty());
    }
}
