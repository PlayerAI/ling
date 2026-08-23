use ling_db::{QueryError, observe_rename_identifier};
use ling_source::{SourceError, SourceFile, VfsError};
use serde_json::{Map, Value, json};

use super::location_projection::{
    LocationProjectionError, identifier_text, range_value, target_document,
};
use super::publication::compiler_for_snapshot;
use super::{
    HandleOutcome, INVALID_PARAMS, LifecycleState, LspServer, MAX_FRAME_BYTES, RequestSnapshot,
    RequestSnapshotError, error_or_none, parse_text_document_position, success_response,
};

/// Current Preview prepare-rename writer marker.
pub const PREPARE_RENAME_PROTOCOL_VERSION: &str = "ling.lsp.prepare-rename/0.1";

const REQUEST_FAILED: i32 = -32_803;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PrepareRenameError {
    InvalidParams,
    Snapshot(RequestSnapshotError),
    CompilerInput(VfsError),
    Compiler(QueryError),
    Source(SourceError),
    Projection(LocationProjectionError),
    InvalidIdentifier,
    InconsistentIdentifier,
    Stale,
    ResponseTooLarge,
}

impl LspServer {
    pub(crate) fn prepare_rename(
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
        match self.prepare_rename_result(&params) {
            Ok(result) => {
                let response = success_response(id.clone(), result);
                if response.len() > MAX_FRAME_BYTES {
                    return prepare_rename_error(id, &PrepareRenameError::ResponseTooLarge);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => prepare_rename_error(id, &error),
        }
    }

    fn prepare_rename_result(&self, params: &Value) -> Result<Value, PrepareRenameError> {
        let (uri, position) =
            parse_text_document_position(params).ok_or(PrepareRenameError::InvalidParams)?;
        let snapshot = self
            .capture_request_snapshot()
            .map_err(PrepareRenameError::Snapshot)?;
        let document = snapshot
            .document(uri)
            .ok_or(PrepareRenameError::InvalidParams)?;
        let temporary = document.is_temporary().then_some(document.uri());
        let mut compiler = compiler_for_snapshot(&snapshot, temporary)
            .map_err(PrepareRenameError::CompilerInput)?;
        let file = compiler
            .vfs()
            .file_id(document.logical_name())
            .ok_or(PrepareRenameError::InvalidParams)?;
        let source = SourceFile::from_bytes(
            file,
            document.logical_name().to_owned(),
            document.bytes().to_vec(),
        )
        .map_err(PrepareRenameError::Source)?;
        let offset = source
            .original_offset(position, snapshot.position_encoding())
            .map_err(|_| PrepareRenameError::InvalidParams)?;
        let index = compiler
            .checked_reference_search_index(file)
            .map_err(PrepareRenameError::Compiler)?;
        let Some(selection) = index.selection_at(document.logical_name(), offset) else {
            return self.finish_prepare_rename(&snapshot, Value::Null);
        };
        let Some(declaration) = selection.declaration() else {
            return self.finish_prepare_rename(&snapshot, Value::Null);
        };
        let declaration_document = target_document(&snapshot, declaration.source_name())
            .map_err(PrepareRenameError::Projection)?;
        if !document.is_writable() || !declaration_document.is_writable() {
            return self.finish_prepare_rename(&snapshot, Value::Null);
        }

        let declaration_file = compiler.vfs().file_id(declaration.source_name()).ok_or(
            PrepareRenameError::Projection(LocationProjectionError::MissingTargetDocument),
        )?;
        let placeholder = identifier_text(document, selection.span(), file)
            .map_err(PrepareRenameError::Projection)?;
        let declaration_name =
            identifier_text(declaration_document, declaration.span(), declaration_file)
                .map_err(PrepareRenameError::Projection)?;
        let selected = observe_rename_identifier(placeholder)
            .map_err(|_| PrepareRenameError::InvalidIdentifier)?;
        let declared = observe_rename_identifier(declaration_name)
            .map_err(|_| PrepareRenameError::InvalidIdentifier)?;
        if selected.normalized() != declared.normalized() {
            return Err(PrepareRenameError::InconsistentIdentifier);
        }
        let range = range_value(
            selection.source_name(),
            selection.span(),
            &snapshot,
            &compiler,
        )
        .map_err(PrepareRenameError::Projection)?;
        self.finish_prepare_rename(
            &snapshot,
            json!({"placeholder": placeholder, "range": range}),
        )
    }

    fn finish_prepare_rename(
        &self,
        snapshot: &RequestSnapshot,
        result: Value,
    ) -> Result<Value, PrepareRenameError> {
        if self
            .capture_request_snapshot()
            .map_err(PrepareRenameError::Snapshot)?
            != *snapshot
        {
            return Err(PrepareRenameError::Stale);
        }
        Ok(result)
    }
}

pub(crate) fn parse_prepare_rename_capability(
    text_document: &Map<String, Value>,
) -> Result<(), ()> {
    let Some(capability) = text_document.get("rename") else {
        return Ok(());
    };
    let capability = capability.as_object().ok_or(())?;
    for field in ["dynamicRegistration", "prepareSupport"] {
        if let Some(value) = capability.get(field) {
            value.as_bool().ok_or(())?;
        }
    }
    if let Some(value) = capability.get("prepareSupportDefaultBehavior") {
        value.as_u64().filter(|value| *value == 1).ok_or(())?;
    }
    Ok(())
}

fn prepare_rename_error(id: Value, error: &PrepareRenameError) -> HandleOutcome {
    match error {
        PrepareRenameError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "重命名准备参数无效 / invalid prepare-rename parameters",
        ),
        PrepareRenameError::Snapshot(_)
        | PrepareRenameError::CompilerInput(_)
        | PrepareRenameError::Compiler(_)
        | PrepareRenameError::Source(_)
        | PrepareRenameError::Projection(_)
        | PrepareRenameError::InvalidIdentifier
        | PrepareRenameError::InconsistentIdentifier
        | PrepareRenameError::Stale
        | PrepareRenameError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "重命名准备不可用 / prepare rename unavailable",
        ),
    }
}
