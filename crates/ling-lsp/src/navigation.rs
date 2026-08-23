use ling_db::QueryError;
use ling_source::{SourceError, SourceFile, VfsError};
use serde_json::{Map, Value};

use super::location_projection::{LocationProjectionError, location_value};
use super::publication::compiler_for_snapshot;
use super::{
    HandleOutcome, INVALID_PARAMS, LifecycleState, LspServer, MAX_FRAME_BYTES,
    RequestSnapshotError, error_or_none, parse_text_document_position, success_response,
};

/// Current Preview resolver-navigation writer marker.
pub const NAVIGATION_PROTOCOL_VERSION: &str = "ling.lsp.navigation/0.1";
/// Maximum locations returned by any version 0.1 navigation request.
pub const MAX_NAVIGATION_TARGETS: usize = 1;

const REQUEST_FAILED: i32 = -32_803;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NavigationMethod {
    Definition,
    Declaration,
    TypeDefinition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum NavigationError {
    InvalidParams,
    Snapshot(RequestSnapshotError),
    CompilerInput(VfsError),
    Compiler(QueryError),
    Source(SourceError),
    Projection(LocationProjectionError),
    Stale,
    ResponseTooLarge,
}

impl LspServer {
    pub(crate) fn navigation(
        &self,
        is_request: bool,
        id: Value,
        params: Value,
        method: NavigationMethod,
    ) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        match self.navigation_result(&params, method) {
            Ok(result) => {
                let response = success_response(id.clone(), result);
                if response.len() > MAX_FRAME_BYTES {
                    return navigation_error(id, &NavigationError::ResponseTooLarge);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => navigation_error(id, &error),
        }
    }

    fn navigation_result(
        &self,
        params: &Value,
        method: NavigationMethod,
    ) -> Result<Value, NavigationError> {
        let (uri, position) =
            parse_text_document_position(params).ok_or(NavigationError::InvalidParams)?;
        let snapshot = self
            .capture_request_snapshot()
            .map_err(NavigationError::Snapshot)?;
        let document = snapshot
            .document(uri)
            .ok_or(NavigationError::InvalidParams)?;
        let temporary = document.is_temporary().then_some(document.uri());
        let mut compiler =
            compiler_for_snapshot(&snapshot, temporary).map_err(NavigationError::CompilerInput)?;
        let file = compiler
            .vfs()
            .file_id(document.logical_name())
            .ok_or(NavigationError::InvalidParams)?;
        let source = SourceFile::from_bytes(
            file,
            document.logical_name().to_owned(),
            document.bytes().to_vec(),
        )
        .map_err(NavigationError::Source)?;
        let offset = source
            .original_offset(position, snapshot.position_encoding())
            .map_err(|_| NavigationError::InvalidParams)?;
        let index = match method {
            NavigationMethod::Definition | NavigationMethod::Declaration => compiler
                .resolved_navigation_index(file)
                .map_err(NavigationError::Compiler)?,
            NavigationMethod::TypeDefinition => compiler
                .checked_navigation_index(file)
                .map_err(NavigationError::Compiler)?,
        };
        let location = index
            .source_entry_at(document.logical_name(), offset)
            .and_then(|entry| match method {
                NavigationMethod::Definition | NavigationMethod::Declaration => entry.definition(),
                NavigationMethod::TypeDefinition => entry.type_definition(),
            });
        let result = location
            .map(|location| {
                location_value(
                    location.source_name(),
                    location.span(),
                    &snapshot,
                    &compiler,
                )
                .map_err(NavigationError::Projection)
            })
            .transpose()?
            .unwrap_or(Value::Null);
        if self
            .capture_request_snapshot()
            .map_err(NavigationError::Snapshot)?
            != snapshot
        {
            return Err(NavigationError::Stale);
        }
        Ok(result)
    }
}

pub(crate) fn parse_navigation_capabilities(text_document: &Map<String, Value>) -> Result<(), ()> {
    for name in ["definition", "declaration", "typeDefinition"] {
        let Some(capability) = text_document.get(name) else {
            continue;
        };
        let capability = capability.as_object().ok_or(())?;
        if let Some(dynamic) = capability.get("dynamicRegistration") {
            dynamic.as_bool().ok_or(())?;
        }
    }
    Ok(())
}

fn navigation_error(id: Value, error: &NavigationError) -> HandleOutcome {
    match error {
        NavigationError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "导航参数无效 / invalid navigation parameters",
        ),
        NavigationError::Snapshot(_)
        | NavigationError::CompilerInput(_)
        | NavigationError::Compiler(_)
        | NavigationError::Source(_)
        | NavigationError::Projection(_)
        | NavigationError::Stale
        | NavigationError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "导航不可用 / navigation unavailable",
        ),
    }
}
