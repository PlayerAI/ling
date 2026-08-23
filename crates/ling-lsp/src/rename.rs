use std::collections::BTreeMap;

use ling_db::{
    CompilerDb, QueryError, RenameIdentifierStatus, ResolvedReferenceRelation,
    ResolvedReferenceTargetKey, observe_rename_identifier,
};
use ling_source::{ByteOffset, SourceError, SourceFile, Span, VfsError};
use serde_json::{Map, Value, json};

use super::location_projection::{
    LocationProjectionError, identifier_text, range_value, target_document,
};
use super::publication::{compiler_for_snapshot, compiler_for_snapshot_with_overrides};
use super::{
    CancellationToken, HandleOutcome, INVALID_PARAMS, LifecycleState, LspServer,
    MAX_DOCUMENT_BYTES, MAX_FRAME_BYTES, REQUEST_CANCELLED, RequestSnapshot, RequestSnapshotError,
    error_or_none, parse_text_document_position, success_response,
};

/// Current Preview checked-rename writer marker.
pub const RENAME_PROTOCOL_VERSION: &str = "ling.lsp.rename/0.1";

const REQUEST_FAILED: i32 = -32_803;

#[derive(Clone, Debug, Eq, PartialEq)]
enum RenameError {
    InvalidParams,
    UnsupportedTransaction,
    Snapshot(RequestSnapshotError),
    CompilerInput(VfsError),
    Compiler(QueryError),
    Source(SourceError),
    Projection(LocationProjectionError),
    InvalidNewName,
    InvalidOccurrence,
    OverlappingOccurrence,
    SimulationMismatch,
    Stale,
    ResponseTooLarge,
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenameOccurrence {
    source_name: String,
    span: Span,
    relation: Option<ResolvedReferenceRelation>,
    declaration: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum RenameTarget {
    Symbol {
        key: ResolvedReferenceTargetKey,
        normalized: String,
        selected_source_name: String,
        selected_span: Span,
        occurrences: Vec<RenameOccurrence>,
    },
    Alias {
        module_id: u32,
        target_module_id: u32,
        normalized: String,
        selected_source_name: String,
        selected_span: Span,
        occurrences: Vec<RenameOccurrence>,
    },
}

impl RenameTarget {
    fn selected(&self) -> (&str, Span) {
        match self {
            Self::Symbol {
                selected_source_name,
                selected_span,
                ..
            }
            | Self::Alias {
                selected_source_name,
                selected_span,
                ..
            } => (selected_source_name, *selected_span),
        }
    }

    fn occurrences(&self) -> &[RenameOccurrence] {
        match self {
            Self::Symbol { occurrences, .. } | Self::Alias { occurrences, .. } => occurrences,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ExpectedOccurrence {
    source_name: String,
    start: u32,
    end: u32,
    relation: Option<ResolvedReferenceRelation>,
    declaration: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DocumentRenamePlan {
    source_name: String,
    uri: String,
    client_version: Option<i64>,
    bytes: Vec<u8>,
    original_occurrences: Vec<RenameOccurrence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenamePlan {
    documents: Vec<DocumentRenamePlan>,
    expected_occurrences: Vec<ExpectedOccurrence>,
    selected_source_name: String,
    selected_start: u32,
}

impl LspServer {
    pub(crate) fn rename(
        &self,
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
        match self.rename_result(&params, cancellation) {
            Ok(result) => {
                if cancellation.check().is_err() {
                    return rename_error(id, &RenameError::Cancelled);
                }
                let response = success_response(id.clone(), result);
                if response.len() > MAX_FRAME_BYTES {
                    return rename_error(id, &RenameError::ResponseTooLarge);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => rename_error(id, &error),
        }
    }

    fn rename_result(
        &self,
        params: &Value,
        cancellation: &CancellationToken,
    ) -> Result<Value, RenameError> {
        if !self.transactional_rename_supported {
            return Err(RenameError::UnsupportedTransaction);
        }
        let (uri, position, new_name) = parse_rename_params(params)?;
        check_cancelled(cancellation)?;
        let snapshot = self
            .capture_request_snapshot()
            .map_err(RenameError::Snapshot)?;
        check_cancelled(cancellation)?;
        let document = snapshot.document(uri).ok_or(RenameError::InvalidParams)?;
        let temporary = document.is_temporary().then_some(document.uri());
        let mut compiler =
            compiler_for_snapshot(&snapshot, temporary).map_err(RenameError::CompilerInput)?;
        let file = compiler
            .vfs()
            .file_id(document.logical_name())
            .ok_or(RenameError::InvalidParams)?;
        let source = SourceFile::from_bytes(
            file,
            document.logical_name().to_owned(),
            document.bytes().to_vec(),
        )
        .map_err(RenameError::Source)?;
        let offset = source
            .original_offset(position, snapshot.position_encoding())
            .map_err(|_| RenameError::InvalidParams)?;

        let Some(target) = select_target(
            &snapshot,
            &mut compiler,
            file,
            document.logical_name(),
            offset,
            cancellation,
        )?
        else {
            return self.finish_rename(&snapshot, Value::Null, cancellation);
        };
        let candidate =
            observe_rename_identifier(new_name).map_err(|_| RenameError::InvalidNewName)?;
        if candidate.was_normalized()
            || candidate.status() != RenameIdentifierStatus::Allowed
            || candidate.has_suspicious_mixed_script()
        {
            return Err(RenameError::InvalidNewName);
        }
        if target.occurrences().iter().all(|occurrence| {
            occurrence_text(&snapshot, &compiler, occurrence).ok() == Some(new_name)
        }) {
            return self.finish_rename(&snapshot, Value::Null, cancellation);
        }

        let plan = build_plan(&snapshot, &compiler, &target, new_name, cancellation)?;
        simulate_plan(
            &snapshot,
            temporary,
            &target,
            &plan,
            candidate.normalized(),
            cancellation,
        )?;
        let result = workspace_edit(&snapshot, &compiler, &plan, new_name, cancellation)?;
        self.finish_rename(&snapshot, result, cancellation)
    }

    fn finish_rename(
        &self,
        snapshot: &RequestSnapshot,
        result: Value,
        cancellation: &CancellationToken,
    ) -> Result<Value, RenameError> {
        check_cancelled(cancellation)?;
        if self
            .capture_request_snapshot()
            .map_err(RenameError::Snapshot)?
            != *snapshot
        {
            return Err(RenameError::Stale);
        }
        check_cancelled(cancellation)?;
        Ok(result)
    }
}

fn select_target(
    snapshot: &RequestSnapshot,
    compiler: &mut CompilerDb,
    file: ling_source::SourceId,
    source_name: &str,
    offset: ByteOffset,
    cancellation: &CancellationToken,
) -> Result<Option<RenameTarget>, RenameError> {
    check_cancelled(cancellation)?;
    let references = compiler
        .checked_reference_search_index_with_cancellation(file, &|| cancellation.is_cancelled())
        .map_err(RenameError::Compiler)?;
    if let Some(selection) = references.selection_at(source_name, offset) {
        let Some(declaration) = selection.declaration() else {
            return Ok(None);
        };
        let locations = references
            .locations_at(source_name, offset, true)
            .ok_or(RenameError::InvalidOccurrence)?;
        let declaration_occurrence = RenameOccurrence {
            source_name: declaration.source_name().to_owned(),
            span: declaration.span(),
            relation: None,
            declaration: true,
        };
        let normalized = observe_occurrence(snapshot, compiler, &declaration_occurrence)?;
        let occurrences = locations
            .into_iter()
            .map(|location| RenameOccurrence {
                source_name: location.source_name().to_owned(),
                span: location.span(),
                relation: location.relation(),
                declaration: location.is_declaration(),
            })
            .collect::<Vec<_>>();
        validate_occurrences(snapshot, compiler, &occurrences, &normalized, cancellation)?;
        return Ok(Some(RenameTarget::Symbol {
            key: selection.target().clone(),
            normalized,
            selected_source_name: selection.source_name().to_owned(),
            selected_span: selection.span(),
            occurrences,
        }));
    }

    let aliases = compiler
        .checked_rename_alias_index_with_cancellation(file, &|| cancellation.is_cancelled())
        .map_err(RenameError::Compiler)?;
    check_cancelled(cancellation)?;
    let Some(selection) = aliases.selection_at(source_name, offset) else {
        return Ok(None);
    };
    let occurrences = selection
        .locations()
        .iter()
        .map(|location| RenameOccurrence {
            source_name: location.source_name().to_owned(),
            span: location.span(),
            relation: None,
            declaration: location.is_declaration(),
        })
        .collect::<Vec<_>>();
    validate_occurrences(
        snapshot,
        compiler,
        &occurrences,
        selection.normalized(),
        cancellation,
    )?;
    Ok(Some(RenameTarget::Alias {
        module_id: selection.module_id(),
        target_module_id: selection.target_module_id(),
        normalized: selection.normalized().to_owned(),
        selected_source_name: selection.selected_source_name().to_owned(),
        selected_span: selection.selected_span(),
        occurrences,
    }))
}

fn validate_occurrences(
    snapshot: &RequestSnapshot,
    compiler: &CompilerDb,
    occurrences: &[RenameOccurrence],
    expected_normalized: &str,
    cancellation: &CancellationToken,
) -> Result<(), RenameError> {
    if occurrences.is_empty() {
        return Err(RenameError::InvalidOccurrence);
    }
    for occurrence in occurrences {
        check_cancelled(cancellation)?;
        let normalized = observe_occurrence(snapshot, compiler, occurrence)?;
        if normalized != expected_normalized {
            return Err(RenameError::InvalidOccurrence);
        }
        let document =
            target_document(snapshot, &occurrence.source_name).map_err(RenameError::Projection)?;
        if !document.is_writable() {
            return Err(RenameError::InvalidOccurrence);
        }
    }
    Ok(())
}

fn observe_occurrence(
    snapshot: &RequestSnapshot,
    compiler: &CompilerDb,
    occurrence: &RenameOccurrence,
) -> Result<String, RenameError> {
    let text = occurrence_text(snapshot, compiler, occurrence)?;
    observe_rename_identifier(text)
        .map(|observation| observation.normalized().to_owned())
        .map_err(|_| RenameError::InvalidOccurrence)
}

fn occurrence_text<'snapshot>(
    snapshot: &'snapshot RequestSnapshot,
    compiler: &CompilerDb,
    occurrence: &RenameOccurrence,
) -> Result<&'snapshot str, RenameError> {
    let document =
        target_document(snapshot, &occurrence.source_name).map_err(RenameError::Projection)?;
    let file = compiler
        .vfs()
        .file_id(&occurrence.source_name)
        .ok_or(RenameError::InvalidOccurrence)?;
    identifier_text(document, occurrence.span, file).map_err(RenameError::Projection)
}

fn build_plan(
    snapshot: &RequestSnapshot,
    compiler: &CompilerDb,
    target: &RenameTarget,
    new_name: &str,
    cancellation: &CancellationToken,
) -> Result<RenamePlan, RenameError> {
    let mut grouped = BTreeMap::<String, Vec<RenameOccurrence>>::new();
    for occurrence in target.occurrences() {
        grouped
            .entry(occurrence.source_name.clone())
            .or_default()
            .push(occurrence.clone());
    }

    let (selected_source_name, selected_span) = target.selected();
    let mut selected_start = None;
    let mut expected_occurrences = Vec::with_capacity(target.occurrences().len());
    let mut documents = Vec::with_capacity(grouped.len());
    for (source_name, mut occurrences) in grouped {
        check_cancelled(cancellation)?;
        occurrences.sort_by_key(|occurrence| {
            (
                occurrence.span.start(),
                occurrence.span.end(),
                occurrence.declaration,
                occurrence.relation,
            )
        });
        if occurrences
            .windows(2)
            .any(|pair| pair[0].span.end() > pair[1].span.start())
        {
            return Err(RenameError::OverlappingOccurrence);
        }
        let document = target_document(snapshot, &source_name).map_err(RenameError::Projection)?;
        if !document.is_writable() {
            return Err(RenameError::InvalidOccurrence);
        }
        let file = compiler
            .vfs()
            .file_id(&source_name)
            .ok_or(RenameError::InvalidOccurrence)?;
        let mut delta = 0_i64;
        for occurrence in &occurrences {
            check_cancelled(cancellation)?;
            let old_text = identifier_text(document, occurrence.span, file)
                .map_err(RenameError::Projection)?;
            let start = i64::from(occurrence.span.start().get()) + delta;
            let end = start
                .checked_add(
                    i64::try_from(new_name.len()).map_err(|_| RenameError::InvalidOccurrence)?,
                )
                .ok_or(RenameError::InvalidOccurrence)?;
            let start = u32::try_from(start).map_err(|_| RenameError::InvalidOccurrence)?;
            let end = u32::try_from(end).map_err(|_| RenameError::InvalidOccurrence)?;
            let expected = ExpectedOccurrence {
                source_name: source_name.clone(),
                start,
                end,
                relation: occurrence.relation,
                declaration: occurrence.declaration,
            };
            if source_name == selected_source_name
                && occurrence.span == selected_span
                && selected_start.replace(start).is_some()
            {
                return Err(RenameError::InvalidOccurrence);
            }
            expected_occurrences.push(expected);
            delta = delta
                .checked_add(
                    i64::try_from(new_name.len()).map_err(|_| RenameError::InvalidOccurrence)?
                        - i64::try_from(old_text.len())
                            .map_err(|_| RenameError::InvalidOccurrence)?,
                )
                .ok_or(RenameError::InvalidOccurrence)?;
        }

        let mut bytes = document.bytes().to_vec();
        for occurrence in occurrences.iter().rev() {
            check_cancelled(cancellation)?;
            let start = usize::try_from(occurrence.span.start().get())
                .map_err(|_| RenameError::InvalidOccurrence)?;
            let end = usize::try_from(occurrence.span.end().get())
                .map_err(|_| RenameError::InvalidOccurrence)?;
            if bytes.get(start..end).is_none() {
                return Err(RenameError::InvalidOccurrence);
            }
            bytes.splice(start..end, new_name.bytes());
        }
        if bytes.len() > MAX_DOCUMENT_BYTES {
            return Err(RenameError::ResponseTooLarge);
        }
        documents.push(DocumentRenamePlan {
            source_name,
            uri: document.uri().to_owned(),
            client_version: document.client_version(),
            bytes,
            original_occurrences: occurrences,
        });
    }
    documents.sort_by(|left, right| left.uri.cmp(&right.uri));
    expected_occurrences.sort();
    Ok(RenamePlan {
        documents,
        expected_occurrences,
        selected_source_name: selected_source_name.to_owned(),
        selected_start: selected_start.ok_or(RenameError::InvalidOccurrence)?,
    })
}

fn simulate_plan(
    snapshot: &RequestSnapshot,
    temporary: Option<&str>,
    target: &RenameTarget,
    plan: &RenamePlan,
    new_normalized: &str,
    cancellation: &CancellationToken,
) -> Result<(), RenameError> {
    check_cancelled(cancellation)?;
    let overrides = plan
        .documents
        .iter()
        .map(|document| (document.source_name.clone(), document.bytes.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut compiler = compiler_for_snapshot_with_overrides(snapshot, temporary, &overrides)
        .map_err(RenameError::CompilerInput)?;
    let file = compiler
        .vfs()
        .file_id(&plan.selected_source_name)
        .ok_or(RenameError::SimulationMismatch)?;
    let offset = ByteOffset::new(plan.selected_start);
    check_cancelled(cancellation)?;

    match target {
        RenameTarget::Symbol {
            key, normalized, ..
        } => {
            let index = compiler
                .checked_reference_search_index_with_cancellation(file, &|| {
                    cancellation.is_cancelled()
                })
                .map_err(RenameError::Compiler)?;
            let selection = index
                .selection_at(&plan.selected_source_name, offset)
                .ok_or(RenameError::SimulationMismatch)?;
            validate_symbol_identity(key, selection.target(), normalized, new_normalized)?;
            let locations = index
                .locations_at(&plan.selected_source_name, offset, true)
                .ok_or(RenameError::SimulationMismatch)?;
            let mut actual = locations
                .into_iter()
                .map(|location| ExpectedOccurrence {
                    source_name: location.source_name().to_owned(),
                    start: location.span().start().get(),
                    end: location.span().end().get(),
                    relation: location.relation(),
                    declaration: location.is_declaration(),
                })
                .collect::<Vec<_>>();
            check_cancelled(cancellation)?;
            actual.sort();
            if actual != plan.expected_occurrences {
                return Err(RenameError::SimulationMismatch);
            }
        }
        RenameTarget::Alias {
            module_id,
            target_module_id,
            ..
        } => {
            let index = compiler
                .checked_rename_alias_index_with_cancellation(file, &|| cancellation.is_cancelled())
                .map_err(RenameError::Compiler)?;
            let selection = index
                .selection_at(&plan.selected_source_name, offset)
                .ok_or(RenameError::SimulationMismatch)?;
            if selection.module_id() != *module_id
                || selection.target_module_id() != *target_module_id
                || selection.normalized() != new_normalized
            {
                return Err(RenameError::SimulationMismatch);
            }
            let mut actual = selection
                .locations()
                .iter()
                .map(|location| ExpectedOccurrence {
                    source_name: location.source_name().to_owned(),
                    start: location.span().start().get(),
                    end: location.span().end().get(),
                    relation: None,
                    declaration: location.is_declaration(),
                })
                .collect::<Vec<_>>();
            check_cancelled(cancellation)?;
            actual.sort();
            if actual != plan.expected_occurrences {
                return Err(RenameError::SimulationMismatch);
            }
        }
    }
    Ok(())
}

fn validate_symbol_identity(
    old: &ResolvedReferenceTargetKey,
    new: &ResolvedReferenceTargetKey,
    old_normalized: &str,
    new_normalized: &str,
) -> Result<(), RenameError> {
    match (old, new) {
        (
            ResolvedReferenceTargetKey::Definition(old),
            ResolvedReferenceTargetKey::Definition(new),
        ) => {
            let changed_name = old_normalized != new_normalized;
            if changed_name == (old == new) {
                return Err(RenameError::SimulationMismatch);
            }
        }
        (
            ResolvedReferenceTargetKey::Binding {
                module_id: old_module,
                binding_id: old_binding,
            },
            ResolvedReferenceTargetKey::Binding {
                module_id: new_module,
                binding_id: new_binding,
            },
        ) if old_module == new_module && old_binding == new_binding => {}
        _ => return Err(RenameError::SimulationMismatch),
    }
    Ok(())
}

fn workspace_edit(
    snapshot: &RequestSnapshot,
    compiler: &CompilerDb,
    plan: &RenamePlan,
    new_name: &str,
    cancellation: &CancellationToken,
) -> Result<Value, RenameError> {
    let mut document_changes = Vec::with_capacity(plan.documents.len());
    for document in &plan.documents {
        check_cancelled(cancellation)?;
        let edits = document
            .original_occurrences
            .iter()
            .map(|occurrence| {
                check_cancelled(cancellation)?;
                range_value(&document.source_name, occurrence.span, snapshot, compiler)
                    .map(|range| json!({"newText": new_name, "range": range}))
                    .map_err(RenameError::Projection)
            })
            .collect::<Result<Vec<_>, _>>()?;
        document_changes.push(json!({
            "edits": edits,
            "textDocument": {
                "uri": document.uri,
                "version": document.client_version,
            },
        }));
    }
    Ok(json!({"documentChanges": document_changes}))
}

fn parse_rename_params(
    value: &Value,
) -> Result<(&str, ling_source::LspPosition, &str), RenameError> {
    let (uri, position) = parse_text_document_position(value).ok_or(RenameError::InvalidParams)?;
    let new_name = value
        .as_object()
        .and_then(|object| object.get("newName"))
        .and_then(Value::as_str)
        .ok_or(RenameError::InvalidParams)?;
    Ok((uri, position, new_name))
}

pub(crate) fn parse_workspace_edit_capability(
    capabilities: &Map<String, Value>,
) -> Result<bool, ()> {
    let Some(workspace) = capabilities.get("workspace") else {
        return Ok(false);
    };
    let workspace = workspace.as_object().ok_or(())?;
    let Some(workspace_edit) = workspace.get("workspaceEdit") else {
        return Ok(false);
    };
    let workspace_edit = workspace_edit.as_object().ok_or(())?;
    let document_changes = match workspace_edit.get("documentChanges") {
        Some(value) => value.as_bool().ok_or(())?,
        None => false,
    };
    let failure_handling = match workspace_edit.get("failureHandling") {
        Some(value) => {
            let value = value.as_str().ok_or(())?;
            if !matches!(
                value,
                "abort" | "transactional" | "textOnlyTransactional" | "undo"
            ) {
                return Err(());
            }
            Some(value)
        }
        None => None,
    };
    Ok(document_changes && failure_handling == Some("transactional"))
}

fn rename_error(id: Value, error: &RenameError) -> HandleOutcome {
    match error {
        RenameError::Cancelled | RenameError::Compiler(QueryError::Cancelled) => error_or_none(
            true,
            id,
            REQUEST_CANCELLED,
            "重命名已取消 / rename cancelled",
        ),
        RenameError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "重命名参数无效 / invalid rename parameters",
        ),
        RenameError::UnsupportedTransaction
        | RenameError::Snapshot(_)
        | RenameError::CompilerInput(_)
        | RenameError::Compiler(_)
        | RenameError::Source(_)
        | RenameError::Projection(_)
        | RenameError::InvalidNewName
        | RenameError::InvalidOccurrence
        | RenameError::OverlappingOccurrence
        | RenameError::SimulationMismatch
        | RenameError::Stale
        | RenameError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "重命名不可用 / rename unavailable",
        ),
    }
}

fn check_cancelled(cancellation: &CancellationToken) -> Result<(), RenameError> {
    cancellation.check().map_err(|_| RenameError::Cancelled)
}
