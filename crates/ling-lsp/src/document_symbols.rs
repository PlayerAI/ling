use ling_db::{MAX_RESOLVED_OUTLINE_NODES, QueryError, ResolvedOutlineKind, ResolvedOutlineNode};
use ling_source::{PositionError, SourceError, SourceFile, Span, VfsError};
use serde_json::{Value, json};

use super::publication::compiler_for_snapshot;
use super::{
    HandleOutcome, INVALID_PARAMS, LifecycleState, LspServer, MAX_FRAME_BYTES,
    RequestSnapshotError, error_or_none, success_response,
};

/// Current Preview Document Symbol writer marker.
pub const DOCUMENT_SYMBOL_PROTOCOL_VERSION: &str = "ling.lsp.document-symbol/0.1";
/// Maximum module root plus descendant symbols in one response.
pub const MAX_DOCUMENT_SYMBOLS: usize = MAX_RESOLVED_OUTLINE_NODES;

const REQUEST_FAILED: i32 = -32_803;

#[derive(Clone, Debug, Eq, PartialEq)]
enum DocumentSymbolError {
    InvalidParams,
    Snapshot(RequestSnapshotError),
    CompilerInput(VfsError),
    Compiler(QueryError),
    Source(SourceError),
    Position(PositionError),
    InvalidSpan,
    Stale,
    ResponseTooLarge,
}

impl LspServer {
    pub(crate) fn document_symbols(
        &self,
        is_request: bool,
        id: Value,
        params: Value,
    ) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        match self.document_symbol_result(&params) {
            Ok(result) => {
                let response = success_response(id.clone(), result);
                if response.len() > MAX_FRAME_BYTES {
                    return document_symbol_error(id, &DocumentSymbolError::ResponseTooLarge);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => document_symbol_error(id, &error),
        }
    }

    fn document_symbol_result(&self, params: &Value) -> Result<Value, DocumentSymbolError> {
        let uri = parse_document_symbol_params(params)?;
        let snapshot = self
            .capture_request_snapshot()
            .map_err(DocumentSymbolError::Snapshot)?;
        let document = snapshot
            .document(uri)
            .ok_or(DocumentSymbolError::InvalidParams)?;
        let temporary = document.is_temporary().then_some(document.uri());
        let mut compiler = compiler_for_snapshot(&snapshot, temporary)
            .map_err(DocumentSymbolError::CompilerInput)?;
        let file = compiler
            .vfs()
            .file_id(document.logical_name())
            .ok_or(DocumentSymbolError::InvalidParams)?;
        let outline = compiler
            .resolved_outline(file)
            .map_err(DocumentSymbolError::Compiler)?;
        if outline.node_count() > MAX_DOCUMENT_SYMBOLS {
            return Err(DocumentSymbolError::InvalidSpan);
        }
        let source = SourceFile::from_bytes(
            file,
            document.logical_name().to_owned(),
            document.bytes().to_vec(),
        )
        .map_err(DocumentSymbolError::Source)?;
        let result = if self.hierarchical_document_symbols {
            Value::Array(vec![hierarchical_symbol(
                outline.root(),
                &source,
                snapshot.position_encoding(),
            )?])
        } else {
            let mut symbols = Vec::with_capacity(outline.node_count());
            flatten_symbols(
                outline.root(),
                None,
                uri,
                &source,
                snapshot.position_encoding(),
                &mut symbols,
            )?;
            Value::Array(symbols)
        };
        if self
            .capture_request_snapshot()
            .map_err(DocumentSymbolError::Snapshot)?
            != snapshot
        {
            return Err(DocumentSymbolError::Stale);
        }
        Ok(result)
    }
}

fn parse_document_symbol_params(params: &Value) -> Result<&str, DocumentSymbolError> {
    params
        .as_object()
        .and_then(|object| object.get("textDocument"))
        .and_then(Value::as_object)
        .and_then(|document| document.get("uri"))
        .and_then(Value::as_str)
        .ok_or(DocumentSymbolError::InvalidParams)
}

fn hierarchical_symbol(
    node: &ResolvedOutlineNode,
    source: &SourceFile,
    encoding: ling_source::PositionEncoding,
) -> Result<Value, DocumentSymbolError> {
    let children = node
        .children()
        .iter()
        .map(|child| hierarchical_symbol(child, source, encoding))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(json!({
        "children": children,
        "kind": symbol_kind(node.kind()),
        "name": node.name(),
        "range": project_range(source, node.span(), encoding)?,
        "selectionRange": project_range(source, node.selection_span(), encoding)?,
    }))
}

fn flatten_symbols(
    node: &ResolvedOutlineNode,
    container: Option<&str>,
    uri: &str,
    source: &SourceFile,
    encoding: ling_source::PositionEncoding,
    output: &mut Vec<Value>,
) -> Result<(), DocumentSymbolError> {
    let mut symbol = json!({
        "kind": symbol_kind(node.kind()),
        "location": {
            "range": project_range(source, node.span(), encoding)?,
            "uri": uri,
        },
        "name": node.name(),
    });
    if let Some(container) = container {
        symbol["containerName"] = Value::String(container.to_owned());
    }
    output.push(symbol);
    for child in node.children() {
        flatten_symbols(child, Some(node.name()), uri, source, encoding, output)?;
    }
    Ok(())
}

fn project_range(
    source: &SourceFile,
    span: Span,
    encoding: ling_source::PositionEncoding,
) -> Result<Value, DocumentSymbolError> {
    if span.source() != source.id() || span.start() > span.end() {
        return Err(DocumentSymbolError::InvalidSpan);
    }
    let start = source
        .lsp_position(span.start(), encoding)
        .map_err(DocumentSymbolError::Position)?;
    let end = source
        .lsp_position(span.end(), encoding)
        .map_err(DocumentSymbolError::Position)?;
    Ok(json!({
        "end": {"character": end.character(), "line": end.line()},
        "start": {"character": start.character(), "line": start.line()},
    }))
}

const fn symbol_kind(kind: ResolvedOutlineKind) -> u8 {
    match kind {
        ResolvedOutlineKind::Module => 2,
        ResolvedOutlineKind::Alias => 5,
        ResolvedOutlineKind::TraitMember | ResolvedOutlineKind::ImplementationMember => 6,
        ResolvedOutlineKind::Field => 8,
        ResolvedOutlineKind::Variant => 10,
        ResolvedOutlineKind::Function => 12,
        ResolvedOutlineKind::Constant => 14,
        ResolvedOutlineKind::Implementation => 19,
        ResolvedOutlineKind::VariantCase => 22,
        ResolvedOutlineKind::Record => 23,
        ResolvedOutlineKind::Trait => 11,
    }
}

fn document_symbol_error(id: Value, error: &DocumentSymbolError) -> HandleOutcome {
    match error {
        DocumentSymbolError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "文档符号参数无效 / invalid document symbol parameters",
        ),
        DocumentSymbolError::Snapshot(_)
        | DocumentSymbolError::CompilerInput(_)
        | DocumentSymbolError::Compiler(_)
        | DocumentSymbolError::Source(_)
        | DocumentSymbolError::Position(_)
        | DocumentSymbolError::InvalidSpan
        | DocumentSymbolError::Stale
        | DocumentSymbolError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "文档符号不可用 / document symbols unavailable",
        ),
    }
}
