use std::collections::{BTreeSet, VecDeque};

use blake3::Hasher;
use ling_db::{SEMANTIC_TOKEN_GENERATION_VERSION, SemanticTokenKind, SemanticTokenModifier};
use ling_source::{PositionError, SourceError, SourceFile};
use serde_json::{Map, Value, json};

use super::publication::compiler_for_snapshot;
use super::{
    CancellationToken, HandleOutcome, INVALID_PARAMS, LifecycleState, MAX_FRAME_BYTES,
    MAX_WORKSPACE_URI_BYTES, METHOD_NOT_FOUND, REQUEST_FAILED, RequestSnapshotError, error_or_none,
    success_response,
};

/// Accepted Preview semantic-token transport marker.
pub const SEMANTIC_TOKEN_PROTOCOL_VERSION: &str = "ling.lsp.semantic-tokens/0.1";
/// Accepted RFC-0046 taxonomy marker.
pub const SEMANTIC_TOKEN_TAXONOMY_VERSION: &str = "ling.semantic-token-taxonomy/0.1";
/// Maximum number of retained full-result arrays in one server session.
pub const MAX_SEMANTIC_TOKEN_RESULTS: usize = 32;
/// Maximum number of semantic tokens encoded in one result.
pub const MAX_SEMANTIC_TOKENS: usize = 32_768;
/// Maximum number of unsigned integers encoded in one result.
pub const MAX_SEMANTIC_TOKEN_DATA_ELEMENTS: usize = MAX_SEMANTIC_TOKENS * 5;
/// Maximum accepted UTF-8 byte length of a client-provided result ID.
pub const MAX_SEMANTIC_TOKEN_RESULT_ID_BYTES: usize = 80;

const REQUEST_CANCELLED: i32 = -32_800;

const TOKEN_KINDS: [SemanticTokenKind; 17] = [
    SemanticTokenKind::Namespace,
    SemanticTokenKind::Type,
    SemanticTokenKind::Enum,
    SemanticTokenKind::Interface,
    SemanticTokenKind::Struct,
    SemanticTokenKind::TypeParameter,
    SemanticTokenKind::Parameter,
    SemanticTokenKind::Variable,
    SemanticTokenKind::Property,
    SemanticTokenKind::EnumMember,
    SemanticTokenKind::Function,
    SemanticTokenKind::Method,
    SemanticTokenKind::Keyword,
    SemanticTokenKind::Comment,
    SemanticTokenKind::String,
    SemanticTokenKind::Number,
    SemanticTokenKind::Operator,
];

const TOKEN_MODIFIERS: [SemanticTokenModifier; 7] = [
    SemanticTokenModifier::Declaration,
    SemanticTokenModifier::Definition,
    SemanticTokenModifier::Readonly,
    SemanticTokenModifier::Modification,
    SemanticTokenModifier::Documentation,
    SemanticTokenModifier::DefaultLibrary,
    SemanticTokenModifier::Mutable,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SemanticTokenOptions {
    enabled: bool,
    delta: bool,
    token_types: Vec<String>,
    token_modifiers: Vec<String>,
}

impl SemanticTokenOptions {
    pub(crate) const fn disabled() -> Self {
        Self {
            enabled: false,
            delta: false,
            token_types: Vec::new(),
            token_modifiers: Vec::new(),
        }
    }

    pub(crate) const fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) const fn delta(&self) -> bool {
        self.delta
    }

    pub(crate) fn provider_value(&self) -> Option<Value> {
        self.enabled.then(|| {
            json!({
                "full": {"delta": self.delta},
                "legend": {
                    "tokenModifiers": self.token_modifiers,
                    "tokenTypes": self.token_types,
                },
                "range": false,
                "workDoneProgress": false,
            })
        })
    }

    pub(crate) fn discovery_value(&self, encoding: ling_source::PositionEncoding) -> Option<Value> {
        self.enabled.then(|| {
            json!({
                "delta": self.delta,
                "generation": SEMANTIC_TOKEN_GENERATION_VERSION,
                "maxDataElements": MAX_SEMANTIC_TOKEN_DATA_ELEMENTS,
                "maxRetainedResults": MAX_SEMANTIC_TOKEN_RESULTS,
                "positionEncoding": encoding.wire_name(),
                "taxonomy": SEMANTIC_TOKEN_TAXONOMY_VERSION,
                "version": SEMANTIC_TOKEN_PROTOCOL_VERSION,
            })
        })
    }

    fn type_index(&self, kind: SemanticTokenKind) -> Option<u32> {
        let selected = self
            .token_types
            .iter()
            .position(|selected| selected == kind.as_str());
        selected
            .or_else(|| {
                kind_fallbacks(kind).iter().find_map(|fallback| {
                    self.token_types
                        .iter()
                        .position(|selected| selected == fallback)
                })
            })
            .and_then(|index| u32::try_from(index).ok())
    }

    fn modifier_mask(&self, modifiers: &[SemanticTokenModifier]) -> Result<u32, ()> {
        let mut mask = 0_u32;
        for modifier in modifiers {
            if let Some(index) = self
                .token_modifiers
                .iter()
                .position(|selected| selected == modifier.as_str())
            {
                let shift = u32::try_from(index).map_err(|_| ())?;
                mask |= 1_u32.checked_shl(shift).ok_or(())?;
            }
        }
        Ok(mask)
    }

    fn projection_identity(&self, encoding: ling_source::PositionEncoding) -> String {
        let mut hasher = Hasher::new();
        hash_text(&mut hasher, SEMANTIC_TOKEN_PROTOCOL_VERSION);
        hash_text(&mut hasher, encoding.wire_name());
        hash_strings(&mut hasher, &self.token_types);
        hash_strings(&mut hasher, &self.token_modifiers);
        hasher.finalize().to_hex().to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SemanticTokenState {
    options: SemanticTokenOptions,
    history: VecDeque<SemanticTokenRecord>,
}

impl SemanticTokenState {
    pub(crate) const fn new() -> Self {
        Self {
            options: SemanticTokenOptions::disabled(),
            history: VecDeque::new(),
        }
    }

    pub(crate) fn configure(&mut self, options: SemanticTokenOptions) {
        self.options = options;
        self.history.clear();
    }

    fn base(&self, result_id: &str, uri: &str, projection: &str) -> Option<Box<[u32]>> {
        self.history
            .iter()
            .find(|record| {
                record.result_id == result_id
                    && record.uri == uri
                    && record.projection == projection
            })
            .map(|record| record.data.clone())
    }

    fn publish(&mut self, record: SemanticTokenRecord) -> Result<(), ()> {
        if let Some(existing) = self
            .history
            .iter()
            .find(|existing| existing.result_id == record.result_id)
        {
            return (existing == &record).then_some(()).ok_or(());
        }
        if self.history.len() == MAX_SEMANTIC_TOKEN_RESULTS {
            self.history.pop_front();
        }
        self.history.push_back(record);
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SemanticTokenRecord {
    result_id: String,
    uri: String,
    projection: String,
    data: Box<[u32]>,
}

struct ComputedTokens {
    record: SemanticTokenRecord,
    snapshot: super::RequestSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SemanticTokenError {
    InvalidParams,
    Disabled,
    Snapshot(RequestSnapshotError),
    MissingDocument,
    CompilerInput(ling_source::VfsError),
    Compiler(ling_db::QueryError),
    Source(SourceError),
    Position(PositionError),
    InvalidProjection,
    TooManyTokens,
    Cancelled,
    Stale,
    Collision,
    ResponseTooLarge,
}

pub(crate) fn parse_capability(
    text_document: &Map<String, Value>,
) -> Result<SemanticTokenOptions, ()> {
    let Some(capability) = text_document.get("semanticTokens") else {
        return Ok(SemanticTokenOptions::disabled());
    };
    let capability = capability.as_object().ok_or(())?;
    validate_optional_booleans(
        capability,
        &[
            "dynamicRegistration",
            "overlappingTokenSupport",
            "multilineTokenSupport",
            "serverCancelSupport",
            "augmentsSyntaxTokens",
        ],
    )?;

    let requests = capability
        .get("requests")
        .and_then(Value::as_object)
        .ok_or(())?;
    if let Some(range) = requests.get("range")
        && !range.is_boolean()
        && !range.is_object()
    {
        return Err(());
    }
    let (full, delta) = match requests.get("full") {
        Some(Value::Bool(value)) => (*value, false),
        Some(Value::Object(options)) => {
            let delta = match options.get("delta") {
                Some(value) => value.as_bool().ok_or(())?,
                None => false,
            };
            (true, delta)
        }
        None => (false, false),
        Some(_) => return Err(()),
    };

    let supported_types = string_set(capability.get("tokenTypes").ok_or(())?)?;
    let supported_modifiers = string_set(capability.get("tokenModifiers").ok_or(())?)?;
    let formats = string_set(capability.get("formats").ok_or(())?)?;
    let token_types = TOKEN_KINDS
        .iter()
        .map(|kind| kind.as_str())
        .filter(|kind| supported_types.contains(*kind))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let token_modifiers = TOKEN_MODIFIERS
        .iter()
        .map(|modifier| modifier.as_str())
        .filter(|modifier| supported_modifiers.contains(*modifier))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let enabled = full && formats.contains("relative") && !token_types.is_empty();
    Ok(SemanticTokenOptions {
        enabled,
        delta: enabled && delta,
        token_types,
        token_modifiers,
    })
}

fn validate_optional_booleans(object: &Map<String, Value>, names: &[&str]) -> Result<(), ()> {
    if names
        .iter()
        .any(|name| object.get(*name).is_some_and(|value| !value.is_boolean()))
    {
        return Err(());
    }
    Ok(())
}

fn string_set(value: &Value) -> Result<BTreeSet<&str>, ()> {
    value
        .as_array()
        .ok_or(())?
        .iter()
        .map(|value| value.as_str().ok_or(()))
        .collect()
}

fn kind_fallbacks(kind: SemanticTokenKind) -> &'static [&'static str] {
    match kind {
        SemanticTokenKind::Namespace => &["variable"],
        SemanticTokenKind::Enum
        | SemanticTokenKind::Interface
        | SemanticTokenKind::Struct
        | SemanticTokenKind::TypeParameter => &["type", "variable"],
        SemanticTokenKind::Parameter
        | SemanticTokenKind::Property
        | SemanticTokenKind::EnumMember => &["variable"],
        SemanticTokenKind::Method => &["function", "variable"],
        SemanticTokenKind::Function | SemanticTokenKind::Type => &["variable"],
        SemanticTokenKind::Variable
        | SemanticTokenKind::Keyword
        | SemanticTokenKind::Comment
        | SemanticTokenKind::String
        | SemanticTokenKind::Number
        | SemanticTokenKind::Operator => &[],
    }
}

impl super::LspServer {
    pub(crate) fn semantic_tokens_full(
        &mut self,
        is_request: bool,
        id: Value,
        params: Value,
        cancellation: &CancellationToken,
    ) -> HandleOutcome {
        self.semantic_tokens(is_request, id, params, None, cancellation)
    }

    pub(crate) fn semantic_tokens_delta(
        &mut self,
        is_request: bool,
        id: Value,
        params: Value,
        cancellation: &CancellationToken,
    ) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        if !self.semantic_token_state.options.delta() {
            return semantic_token_error(id, &SemanticTokenError::Disabled);
        }
        let (uri, previous_result_id) = match parse_delta_params(&params) {
            Ok((uri, previous_result_id)) => (uri.to_owned(), previous_result_id.to_owned()),
            Err(error) => return semantic_token_error(id, &error),
        };
        self.semantic_tokens(
            true,
            id,
            params,
            Some((&uri, &previous_result_id)),
            cancellation,
        )
    }

    fn semantic_tokens(
        &mut self,
        is_request: bool,
        id: Value,
        params: Value,
        delta: Option<(&str, &str)>,
        cancellation: &CancellationToken,
    ) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        if !self.semantic_token_state.options.enabled() {
            return semantic_token_error(id, &SemanticTokenError::Disabled);
        }
        let uri = match delta {
            Some((uri, _)) => uri,
            None => match parse_full_params(&params) {
                Ok(uri) => uri,
                Err(error) => return semantic_token_error(id, &error),
            },
        };
        match self.compute_semantic_tokens(uri, cancellation) {
            Ok(computed) => {
                let projection = computed.record.projection.clone();
                let base = delta.and_then(|(_, previous)| {
                    self.semantic_token_state.base(previous, uri, &projection)
                });
                let full = full_result(&computed.record);
                let selected = match base {
                    Some(base) => match delta_result(&computed.record, &base, cancellation) {
                        Ok(delta_value) => {
                            let delta_response = success_response(id.clone(), delta_value);
                            if delta_response.len() <= MAX_FRAME_BYTES {
                                Ok(delta_response)
                            } else {
                                bounded_response(id.clone(), full)
                            }
                        }
                        Err(error) => Err(error),
                    },
                    None => bounded_response(id.clone(), full),
                };
                let response = match selected {
                    Ok(response) => response,
                    Err(error) => return semantic_token_error(id, &error),
                };
                if cancellation.check().is_err() {
                    return semantic_token_error(id, &SemanticTokenError::Cancelled);
                }
                match self.capture_request_snapshot() {
                    Ok(current) if current == computed.snapshot => {}
                    Ok(_) => return semantic_token_error(id, &SemanticTokenError::Stale),
                    Err(error) => {
                        return semantic_token_error(id, &SemanticTokenError::Snapshot(error));
                    }
                }
                if self.semantic_token_state.publish(computed.record).is_err() {
                    return semantic_token_error(id, &SemanticTokenError::Collision);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => semantic_token_error(id, &error),
        }
    }

    fn compute_semantic_tokens(
        &self,
        uri: &str,
        cancellation: &CancellationToken,
    ) -> Result<ComputedTokens, SemanticTokenError> {
        cancellation
            .check()
            .map_err(|_| SemanticTokenError::Cancelled)?;
        let snapshot = self
            .capture_request_snapshot()
            .map_err(SemanticTokenError::Snapshot)?;
        let document = snapshot
            .document(uri)
            .ok_or(SemanticTokenError::MissingDocument)?;
        let temporary = document.is_temporary().then_some(document.uri());
        let mut compiler = compiler_for_snapshot(&snapshot, temporary)
            .map_err(SemanticTokenError::CompilerInput)?;
        let file = compiler
            .vfs()
            .file_id(document.logical_name())
            .ok_or(SemanticTokenError::MissingDocument)?;
        cancellation
            .check()
            .map_err(|_| SemanticTokenError::Cancelled)?;
        let index = compiler
            .semantic_token_index(file)
            .map_err(SemanticTokenError::Compiler)?;
        if index.source() != file || index.source_name() != document.logical_name() {
            return Err(SemanticTokenError::InvalidProjection);
        }
        let source = SourceFile::from_bytes(
            file,
            document.logical_name().to_owned(),
            document.bytes().to_vec(),
        )
        .map_err(SemanticTokenError::Source)?;
        let data = project_data(
            &source,
            index.entries(),
            &self.semantic_token_state.options,
            snapshot.position_encoding(),
            cancellation,
        )?;
        let projection = self
            .semantic_token_state
            .options
            .projection_identity(snapshot.position_encoding());
        let result_id = result_id(
            uri,
            document.client_version(),
            snapshot.position_encoding(),
            &self.semantic_token_state.options,
            &data,
        );
        Ok(ComputedTokens {
            record: SemanticTokenRecord {
                result_id,
                uri: uri.to_owned(),
                projection,
                data: data.into_boxed_slice(),
            },
            snapshot,
        })
    }
}

fn parse_full_params(params: &Value) -> Result<&str, SemanticTokenError> {
    let uri = params
        .as_object()
        .and_then(|params| params.get("textDocument"))
        .and_then(Value::as_object)
        .and_then(|document| document.get("uri"))
        .and_then(Value::as_str)
        .ok_or(SemanticTokenError::InvalidParams)?;
    validate_text(uri, MAX_WORKSPACE_URI_BYTES)?;
    Ok(uri)
}

fn parse_delta_params(params: &Value) -> Result<(&str, &str), SemanticTokenError> {
    let uri = parse_full_params(params)?;
    let previous = params
        .as_object()
        .and_then(|params| params.get("previousResultId"))
        .and_then(Value::as_str)
        .ok_or(SemanticTokenError::InvalidParams)?;
    validate_text(previous, MAX_SEMANTIC_TOKEN_RESULT_ID_BYTES)?;
    Ok((uri, previous))
}

fn validate_text(value: &str, max_bytes: usize) -> Result<(), SemanticTokenError> {
    if value.is_empty() || value.len() > max_bytes || value.contains('\0') {
        return Err(SemanticTokenError::InvalidParams);
    }
    Ok(())
}

fn project_data(
    source: &SourceFile,
    entries: &[ling_db::SemanticTokenEntry],
    options: &SemanticTokenOptions,
    encoding: ling_source::PositionEncoding,
    cancellation: &CancellationToken,
) -> Result<Vec<u32>, SemanticTokenError> {
    let mut data = Vec::with_capacity(entries.len().min(MAX_SEMANTIC_TOKENS).saturating_mul(5));
    let mut previous_start = None::<(u32, u32)>;
    let mut previous_end = None::<(u32, u32)>;
    for entry in entries {
        cancellation
            .check()
            .map_err(|_| SemanticTokenError::Cancelled)?;
        let Some(token_type) = options.type_index(entry.kind()) else {
            continue;
        };
        let span = entry.span();
        if span.source() != source.id() || span.start() >= span.end() {
            return Err(SemanticTokenError::InvalidProjection);
        }
        let start = source
            .lsp_position(span.start(), encoding)
            .map_err(SemanticTokenError::Position)?;
        let end = source
            .lsp_position(span.end(), encoding)
            .map_err(SemanticTokenError::Position)?;
        if start.line() != end.line() || start.character() >= end.character() {
            return Err(SemanticTokenError::InvalidProjection);
        }
        let absolute_start = (start.line(), start.character());
        let absolute_end = (end.line(), end.character());
        if previous_end.is_some_and(|previous| absolute_start < previous) {
            return Err(SemanticTokenError::InvalidProjection);
        }
        let (delta_line, delta_start) = match previous_start {
            None => absolute_start,
            Some((line, character)) if line == start.line() => (
                0,
                start
                    .character()
                    .checked_sub(character)
                    .ok_or(SemanticTokenError::InvalidProjection)?,
            ),
            Some((line, _)) => (
                start
                    .line()
                    .checked_sub(line)
                    .ok_or(SemanticTokenError::InvalidProjection)?,
                start.character(),
            ),
        };
        if data.len() == MAX_SEMANTIC_TOKEN_DATA_ELEMENTS {
            return Err(SemanticTokenError::TooManyTokens);
        }
        data.extend_from_slice(&[
            delta_line,
            delta_start,
            end.character() - start.character(),
            token_type,
            options
                .modifier_mask(entry.modifiers())
                .map_err(|_| SemanticTokenError::InvalidProjection)?,
        ]);
        previous_start = Some(absolute_start);
        previous_end = Some(absolute_end);
    }
    Ok(data)
}

fn result_id(
    uri: &str,
    client_version: Option<i64>,
    encoding: ling_source::PositionEncoding,
    options: &SemanticTokenOptions,
    data: &[u32],
) -> String {
    let mut hasher = Hasher::new();
    hash_text(&mut hasher, SEMANTIC_TOKEN_PROTOCOL_VERSION);
    hash_text(&mut hasher, uri);
    match client_version {
        Some(version) => {
            hasher.update(&[1]);
            hasher.update(&version.to_le_bytes());
        }
        None => {
            hasher.update(&[0]);
        }
    }
    hash_text(&mut hasher, encoding.wire_name());
    hash_strings(&mut hasher, &options.token_types);
    hash_strings(&mut hasher, &options.token_modifiers);
    hasher.update(&(data.len() as u64).to_le_bytes());
    for value in data {
        hasher.update(&value.to_le_bytes());
    }
    format!("st1-{}", hasher.finalize().to_hex())
}

fn hash_strings(hasher: &mut Hasher, values: &[String]) {
    hasher.update(&(values.len() as u64).to_le_bytes());
    for value in values {
        hash_text(hasher, value);
    }
}

fn hash_text(hasher: &mut Hasher, value: &str) {
    hasher.update(&(value.len() as u64).to_le_bytes());
    hasher.update(value.as_bytes());
}

fn full_result(record: &SemanticTokenRecord) -> Value {
    json!({
        "data": record.data,
        "resultId": record.result_id,
    })
}

fn delta_result(
    record: &SemanticTokenRecord,
    base: &[u32],
    cancellation: &CancellationToken,
) -> Result<Value, SemanticTokenError> {
    let current = record.data.as_ref();
    let mut prefix = 0;
    while prefix < base.len() && prefix < current.len() && base[prefix] == current[prefix] {
        cancellation
            .check()
            .map_err(|_| SemanticTokenError::Cancelled)?;
        prefix += 1;
    }
    let mut suffix = 0;
    while suffix < base.len() - prefix
        && suffix < current.len() - prefix
        && base[base.len() - suffix - 1] == current[current.len() - suffix - 1]
    {
        cancellation
            .check()
            .map_err(|_| SemanticTokenError::Cancelled)?;
        suffix += 1;
    }
    let edits = if prefix == base.len() && prefix == current.len() {
        Vec::new()
    } else {
        let start = u32::try_from(prefix).map_err(|_| SemanticTokenError::TooManyTokens)?;
        let delete_count = u32::try_from(base.len() - prefix - suffix)
            .map_err(|_| SemanticTokenError::TooManyTokens)?;
        let inserted = &current[prefix..current.len() - suffix];
        let mut edit = Map::new();
        edit.insert("deleteCount".to_owned(), json!(delete_count));
        edit.insert("start".to_owned(), json!(start));
        if !inserted.is_empty() {
            edit.insert("data".to_owned(), json!(inserted));
        }
        vec![Value::Object(edit)]
    };
    Ok(json!({
        "edits": edits,
        "resultId": record.result_id,
    }))
}

fn bounded_response(id: Value, result: Value) -> Result<Vec<u8>, SemanticTokenError> {
    let response = success_response(id, result);
    if response.len() > MAX_FRAME_BYTES {
        return Err(SemanticTokenError::ResponseTooLarge);
    }
    Ok(response)
}

fn semantic_token_error(id: Value, error: &SemanticTokenError) -> HandleOutcome {
    match error {
        SemanticTokenError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "语义 Token 参数无效 / invalid semantic token parameters",
        ),
        SemanticTokenError::Disabled => error_or_none(
            true,
            id,
            METHOD_NOT_FOUND,
            "语义 Token 方法不可用 / semantic token method unavailable",
        ),
        SemanticTokenError::Cancelled => error_or_none(
            true,
            id,
            REQUEST_CANCELLED,
            "语义 Token 查询已取消 / semantic token query cancelled",
        ),
        SemanticTokenError::Snapshot(_)
        | SemanticTokenError::MissingDocument
        | SemanticTokenError::CompilerInput(_)
        | SemanticTokenError::Compiler(_)
        | SemanticTokenError::Source(_)
        | SemanticTokenError::Position(_)
        | SemanticTokenError::InvalidProjection
        | SemanticTokenError::TooManyTokens
        | SemanticTokenError::Stale
        | SemanticTokenError::Collision
        | SemanticTokenError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "语义 Token 不可用 / semantic tokens unavailable",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_delta_replaces_only_the_unequal_middle() {
        let record = SemanticTokenRecord {
            result_id: "st1-current".to_owned(),
            uri: "ling://workspace/Main.ling".to_owned(),
            projection: "projection".to_owned(),
            data: vec![1, 2, 8, 9, 5].into_boxed_slice(),
        };
        assert_eq!(
            delta_result(&record, &[1, 2, 3, 4, 5], &CancellationToken::new()).unwrap(),
            json!({
                "edits": [{"data": [8, 9], "deleteCount": 2, "start": 2}],
                "resultId": "st1-current",
            })
        );
    }

    #[test]
    fn equal_delta_has_no_edits() {
        let record = SemanticTokenRecord {
            result_id: "st1-equal".to_owned(),
            uri: "ling://workspace/Main.ling".to_owned(),
            projection: "projection".to_owned(),
            data: vec![1, 2].into_boxed_slice(),
        };
        assert_eq!(
            delta_result(&record, &[1, 2], &CancellationToken::new()).unwrap(),
            json!({"edits": [], "resultId": "st1-equal"})
        );
    }
}
