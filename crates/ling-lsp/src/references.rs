use ling_db::{MAX_REFERENCE_SEARCH_ENTRIES, QueryError};
use ling_source::{SourceError, SourceFile, VfsError};
use serde_json::{Map, Value};

use super::location_projection::{LocationProjectionError, location_value};
use super::publication::compiler_for_snapshot;
use super::{
    HandleOutcome, INVALID_PARAMS, LifecycleState, LspServer, MAX_FRAME_BYTES,
    RequestSnapshotError, error_or_none, parse_text_document_position, success_response,
};

/// Current Preview checked-references writer marker.
pub const REFERENCES_PROTOCOL_VERSION: &str = "ling.lsp.references/0.1";
/// Maximum locations returned by one references request.
pub const MAX_REFERENCE_LOCATIONS: usize = MAX_REFERENCE_SEARCH_ENTRIES;

const REQUEST_FAILED: i32 = -32_803;

#[derive(Clone, Debug, Eq, PartialEq)]
enum ReferencesError {
    InvalidParams,
    Snapshot(RequestSnapshotError),
    CompilerInput(VfsError),
    Compiler(QueryError),
    Source(SourceError),
    Projection(LocationProjectionError),
    Stale,
    TooManyLocations,
    ResponseTooLarge,
}

impl LspServer {
    pub(crate) fn references(&self, is_request: bool, id: Value, params: Value) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        match self.references_result(&params) {
            Ok(result) => {
                let response = success_response(id.clone(), result);
                if response.len() > MAX_FRAME_BYTES {
                    return references_error(id, &ReferencesError::ResponseTooLarge);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => references_error(id, &error),
        }
    }

    fn references_result(&self, params: &Value) -> Result<Value, ReferencesError> {
        let (uri, position, include_declaration) =
            parse_references_params(params).ok_or(ReferencesError::InvalidParams)?;
        let snapshot = self
            .capture_request_snapshot()
            .map_err(ReferencesError::Snapshot)?;
        let document = snapshot
            .document(uri)
            .ok_or(ReferencesError::InvalidParams)?;
        let temporary = document.is_temporary().then_some(document.uri());
        let mut compiler =
            compiler_for_snapshot(&snapshot, temporary).map_err(ReferencesError::CompilerInput)?;
        let file = compiler
            .vfs()
            .file_id(document.logical_name())
            .ok_or(ReferencesError::InvalidParams)?;
        let source = SourceFile::from_bytes(
            file,
            document.logical_name().to_owned(),
            document.bytes().to_vec(),
        )
        .map_err(ReferencesError::Source)?;
        let offset = source
            .original_offset(position, snapshot.position_encoding())
            .map_err(|_| ReferencesError::InvalidParams)?;
        let index = compiler
            .checked_reference_search_index(file)
            .map_err(ReferencesError::Compiler)?;
        let locations = index
            .locations_at(document.logical_name(), offset, include_declaration)
            .unwrap_or_default();
        if locations.len() > MAX_REFERENCE_LOCATIONS {
            return Err(ReferencesError::TooManyLocations);
        }
        let result = locations
            .into_iter()
            .map(|location| {
                location_value(
                    location.source_name(),
                    location.span(),
                    &snapshot,
                    &compiler,
                )
                .map_err(ReferencesError::Projection)
            })
            .collect::<Result<Vec<_>, _>>()?;
        if self
            .capture_request_snapshot()
            .map_err(ReferencesError::Snapshot)?
            != snapshot
        {
            return Err(ReferencesError::Stale);
        }
        Ok(Value::Array(result))
    }
}

pub(crate) fn parse_references_capability(text_document: &Map<String, Value>) -> Result<(), ()> {
    let Some(capability) = text_document.get("references") else {
        return Ok(());
    };
    let capability = capability.as_object().ok_or(())?;
    if let Some(dynamic) = capability.get("dynamicRegistration") {
        dynamic.as_bool().ok_or(())?;
    }
    Ok(())
}

fn parse_references_params(value: &Value) -> Option<(&str, ling_source::LspPosition, bool)> {
    let (uri, position) = parse_text_document_position(value)?;
    let context = value.as_object()?.get("context")?.as_object()?;
    let include_declaration = context.get("includeDeclaration")?.as_bool()?;
    Some((uri, position, include_declaration))
}

fn references_error(id: Value, error: &ReferencesError) -> HandleOutcome {
    match error {
        ReferencesError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "引用参数无效 / invalid references parameters",
        ),
        ReferencesError::Snapshot(_)
        | ReferencesError::CompilerInput(_)
        | ReferencesError::Compiler(_)
        | ReferencesError::Source(_)
        | ReferencesError::Projection(_)
        | ReferencesError::Stale
        | ReferencesError::TooManyLocations
        | ReferencesError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "引用查询不可用 / references unavailable",
        ),
    }
}
