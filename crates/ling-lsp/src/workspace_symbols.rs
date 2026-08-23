//! Snapshot-indexed Preview workspace symbols governed by RFC-0045.

use std::cmp::Ordering;
use std::collections::BTreeMap;

use ling_db::{QueryError, ResolvedDefinitionKind};
use ling_source::{PositionError, SourceFile, Span, VfsError};
use serde_json::{Value, json};

use crate::publication::compiler_for_snapshot;
use crate::{
    CancellationToken, HandleOutcome, INVALID_PARAMS, LifecycleState, LspServer, MAX_FRAME_BYTES,
    REQUEST_FAILED, RequestSnapshot, RequestSnapshotError, error_or_none, success_response,
};

/// Preview workspace-symbol writer marker.
pub const WORKSPACE_SYMBOL_PROTOCOL_VERSION: &str = "ling.lsp.workspace-symbol/0.1";
/// Maximum UTF-8 bytes accepted in one workspace-symbol query.
pub const MAX_WORKSPACE_SYMBOL_QUERY_BYTES: usize = 256;
/// Maximum number of workspace-symbol results returned by one request.
pub const MAX_WORKSPACE_SYMBOLS: usize = 256;

const REQUEST_CANCELLED: i32 = -32_800;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorkspaceSymbolState {
    cache: Option<WorkspaceSymbolCache>,
}

impl WorkspaceSymbolState {
    pub(crate) const fn new() -> Self {
        Self { cache: None }
    }

    fn records(&self, snapshot: &RequestSnapshot) -> Option<&[WorkspaceSymbolRecord]> {
        self.cache
            .as_ref()
            .filter(|cache| cache.snapshot == *snapshot)
            .map(|cache| cache.records.as_ref())
    }

    fn publish(&mut self, cache: WorkspaceSymbolCache) {
        self.cache = Some(cache);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkspaceSymbolCache {
    snapshot: RequestSnapshot,
    records: Box<[WorkspaceSymbolRecord]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkspaceSymbolRecord {
    name: String,
    module: String,
    uri: String,
    range: Value,
    kind: u8,
    span: Span,
    definition_id: String,
}

struct WorkspaceSymbolResult {
    value: Value,
    pending_cache: Option<WorkspaceSymbolCache>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorkspaceSymbolError {
    InvalidParams,
    Snapshot(RequestSnapshotError),
    CompilerInput(VfsError),
    Compiler(QueryError),
    Source(ling_source::SourceError),
    Position(PositionError),
    InvalidSpan,
    Cancelled,
    Stale,
    ResponseTooLarge,
}

impl LspServer {
    pub(crate) fn workspace_symbols(
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

        match self.workspace_symbol_result(&params, cancellation) {
            Ok(result) => {
                if cancellation.check().is_err() {
                    return workspace_symbol_error(id, &WorkspaceSymbolError::Cancelled);
                }
                let response = success_response(id.clone(), result.value);
                if response.len() > MAX_FRAME_BYTES {
                    return workspace_symbol_error(id, &WorkspaceSymbolError::ResponseTooLarge);
                }
                if cancellation.check().is_err() {
                    return workspace_symbol_error(id, &WorkspaceSymbolError::Cancelled);
                }
                if let Some(cache) = result.pending_cache {
                    self.workspace_symbol_state.publish(cache);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => workspace_symbol_error(id, &error),
        }
    }

    fn workspace_symbol_result(
        &self,
        params: &Value,
        cancellation: &CancellationToken,
    ) -> Result<WorkspaceSymbolResult, WorkspaceSymbolError> {
        let query = parse_workspace_symbol_params(params)?;
        cancellation
            .check()
            .map_err(|_| WorkspaceSymbolError::Cancelled)?;
        let snapshot = self
            .capture_request_snapshot()
            .map_err(WorkspaceSymbolError::Snapshot)?;

        let (records, pending_cache) =
            if let Some(records) = self.workspace_symbol_state.records(&snapshot) {
                (records.to_vec(), None)
            } else {
                let records = build_workspace_symbol_records(&snapshot, cancellation)?;
                let cache = WorkspaceSymbolCache {
                    snapshot: snapshot.clone(),
                    records: records.clone().into_boxed_slice(),
                };
                (records, Some(cache))
            };

        let value = select_workspace_symbols(&records, query, cancellation)?;
        cancellation
            .check()
            .map_err(|_| WorkspaceSymbolError::Cancelled)?;
        if self
            .capture_request_snapshot()
            .map_err(WorkspaceSymbolError::Snapshot)?
            != snapshot
        {
            return Err(WorkspaceSymbolError::Stale);
        }
        Ok(WorkspaceSymbolResult {
            value,
            pending_cache,
        })
    }
}

fn parse_workspace_symbol_params(params: &Value) -> Result<&str, WorkspaceSymbolError> {
    let query = params
        .as_object()
        .and_then(|object| object.get("query"))
        .and_then(Value::as_str)
        .ok_or(WorkspaceSymbolError::InvalidParams)?;
    if query.len() > MAX_WORKSPACE_SYMBOL_QUERY_BYTES || query.contains('\0') {
        return Err(WorkspaceSymbolError::InvalidParams);
    }
    Ok(query)
}

fn build_workspace_symbol_records(
    snapshot: &RequestSnapshot,
    cancellation: &CancellationToken,
) -> Result<Vec<WorkspaceSymbolRecord>, WorkspaceSymbolError> {
    cancellation
        .check()
        .map_err(|_| WorkspaceSymbolError::Cancelled)?;
    let workspace_documents = snapshot
        .documents()
        .iter()
        .filter(|document| document.is_writable() && !document.is_temporary())
        .collect::<Vec<_>>();
    let Some(root) = workspace_documents.first() else {
        return Ok(Vec::new());
    };

    let mut compiler =
        compiler_for_snapshot(snapshot, None).map_err(WorkspaceSymbolError::CompilerInput)?;
    let root_file = compiler
        .vfs()
        .file_id(root.logical_name())
        .ok_or(WorkspaceSymbolError::InvalidSpan)?;
    let index = compiler
        .resolved_definition_index(root_file)
        .map_err(WorkspaceSymbolError::Compiler)?;

    let mut sources = BTreeMap::new();
    for document in workspace_documents {
        cancellation
            .check()
            .map_err(|_| WorkspaceSymbolError::Cancelled)?;
        let file = compiler
            .vfs()
            .file_id(document.logical_name())
            .ok_or(WorkspaceSymbolError::InvalidSpan)?;
        let source = SourceFile::from_bytes(
            file,
            document.logical_name().to_owned(),
            document.bytes().to_vec(),
        )
        .map_err(WorkspaceSymbolError::Source)?;
        sources.insert(
            document.logical_name().to_owned(),
            (document.uri().to_owned(), source),
        );
    }

    let mut records = Vec::new();
    for symbol in index.symbols() {
        cancellation
            .check()
            .map_err(|_| WorkspaceSymbolError::Cancelled)?;
        let Some((uri, source)) = sources.get(symbol.source_name()) else {
            continue;
        };
        if symbol.span().source() != source.id() || symbol.span().start() > symbol.span().end() {
            return Err(WorkspaceSymbolError::InvalidSpan);
        }
        records.push(WorkspaceSymbolRecord {
            name: symbol.name_source().to_owned(),
            module: symbol.module_name().to_owned(),
            uri: uri.clone(),
            range: project_range(source, symbol.span(), snapshot.position_encoding())?,
            kind: symbol_kind(symbol.kind()),
            span: symbol.span(),
            definition_id: symbol.definition_id().to_owned(),
        });
    }
    records.sort_by(record_order);
    Ok(records)
}

fn select_workspace_symbols(
    records: &[WorkspaceSymbolRecord],
    query: &str,
    cancellation: &CancellationToken,
) -> Result<Value, WorkspaceSymbolError> {
    let mut matches = Vec::new();
    for record in records {
        cancellation
            .check()
            .map_err(|_| WorkspaceSymbolError::Cancelled)?;
        if query.is_empty() || record.name.starts_with(query) {
            matches.push(record);
        }
    }
    matches.sort_by(|left, right| match_order(left, right, query));

    let mut result = Vec::with_capacity(matches.len().min(MAX_WORKSPACE_SYMBOLS));
    for record in matches.into_iter().take(MAX_WORKSPACE_SYMBOLS) {
        cancellation
            .check()
            .map_err(|_| WorkspaceSymbolError::Cancelled)?;
        result.push(json!({
            "containerName": record.module,
            "kind": record.kind,
            "location": {
                "range": record.range,
                "uri": record.uri,
            },
            "name": record.name,
        }));
    }
    Ok(Value::Array(result))
}

fn project_range(
    source: &SourceFile,
    span: Span,
    encoding: ling_source::PositionEncoding,
) -> Result<Value, WorkspaceSymbolError> {
    let start = source
        .lsp_position(span.start(), encoding)
        .map_err(WorkspaceSymbolError::Position)?;
    let end = source
        .lsp_position(span.end(), encoding)
        .map_err(WorkspaceSymbolError::Position)?;
    Ok(json!({
        "end": {"character": end.character(), "line": end.line()},
        "start": {"character": start.character(), "line": start.line()},
    }))
}

const fn symbol_kind(kind: ResolvedDefinitionKind) -> u8 {
    match kind {
        ResolvedDefinitionKind::Value => 13,
        ResolvedDefinitionKind::Type => 5,
        ResolvedDefinitionKind::Constructor => 22,
        ResolvedDefinitionKind::TraitMember | ResolvedDefinitionKind::ImplementationMember => 6,
    }
}

fn record_order(left: &WorkspaceSymbolRecord, right: &WorkspaceSymbolRecord) -> Ordering {
    left.name
        .cmp(&right.name)
        .then_with(|| left.module.cmp(&right.module))
        .then_with(|| left.uri.cmp(&right.uri))
        .then_with(|| left.span.start().cmp(&right.span.start()))
        .then_with(|| left.span.end().cmp(&right.span.end()))
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.definition_id.cmp(&right.definition_id))
}

fn match_order(
    left: &WorkspaceSymbolRecord,
    right: &WorkspaceSymbolRecord,
    query: &str,
) -> Ordering {
    let left_exact = left.name == query;
    let right_exact = right.name == query;
    right_exact
        .cmp(&left_exact)
        .then_with(|| record_order(left, right))
}

fn workspace_symbol_error(id: Value, error: &WorkspaceSymbolError) -> HandleOutcome {
    match error {
        WorkspaceSymbolError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "工作区符号参数无效 / invalid workspace symbol parameters",
        ),
        WorkspaceSymbolError::Cancelled => error_or_none(
            true,
            id,
            REQUEST_CANCELLED,
            "工作区符号查询已取消 / workspace symbol query cancelled",
        ),
        WorkspaceSymbolError::Snapshot(_)
        | WorkspaceSymbolError::CompilerInput(_)
        | WorkspaceSymbolError::Compiler(_)
        | WorkspaceSymbolError::Source(_)
        | WorkspaceSymbolError::Position(_)
        | WorkspaceSymbolError::InvalidSpan
        | WorkspaceSymbolError::Stale
        | WorkspaceSymbolError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "工作区符号不可用 / workspace symbols unavailable",
        ),
    }
}

#[cfg(test)]
mod tests {
    use ling_source::{ByteOffset, SourceId};

    use super::*;

    fn record(name: &str, module: &str, offset: u32) -> WorkspaceSymbolRecord {
        WorkspaceSymbolRecord {
            name: name.to_owned(),
            module: module.to_owned(),
            uri: format!("ling://workspace/src/{module}.ling"),
            range: json!({}),
            kind: 13,
            span: Span::new(
                SourceId::new(1),
                ByteOffset::new(offset),
                ByteOffset::new(offset + 1),
            )
            .unwrap(),
            definition_id: format!("definition-{offset}"),
        }
    }

    #[test]
    fn exact_matches_precede_prefixes_and_truncation_is_deterministic() {
        let token = CancellationToken::new();
        let mut records = (0..=MAX_WORKSPACE_SYMBOLS)
            .map(|index| record(&format!("item{index:03}"), "Main", index as u32))
            .collect::<Vec<_>>();
        records.push(record("item", "Exact", 500));
        let value = select_workspace_symbols(&records, "item", &token).unwrap();
        let items = value.as_array().unwrap();
        assert_eq!(items.len(), MAX_WORKSPACE_SYMBOLS);
        assert_eq!(items[0]["name"], "item");
        assert_eq!(items[1]["name"], "item000");
        assert_eq!(
            select_workspace_symbols(&records, "item", &token).unwrap(),
            value
        );
    }

    #[test]
    fn cancelled_filter_discards_all_partial_results() {
        let token = CancellationToken::new();
        token.cancel();
        assert_eq!(
            select_workspace_symbols(&[record("value", "Main", 0)], "", &token),
            Err(WorkspaceSymbolError::Cancelled)
        );
    }
}
