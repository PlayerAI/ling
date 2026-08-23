use ling_source::{PositionError, SourceError, SourceFile, SourceId, Span};
use serde_json::{Value, json};

use super::{RequestDocument, RequestSnapshot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LocationProjectionError {
    MissingTargetDocument,
    DuplicateTargetDocument,
    Source(SourceError),
    Position(PositionError),
    InvalidSpan,
}

pub(crate) fn location_value(
    source_name: &str,
    span: Span,
    snapshot: &RequestSnapshot,
    compiler: &ling_db::CompilerDb,
) -> Result<Value, LocationProjectionError> {
    let document = target_document(snapshot, source_name)?;
    let file = compiler
        .vfs()
        .file_id(source_name)
        .ok_or(LocationProjectionError::MissingTargetDocument)?;
    let source = SourceFile::from_bytes(file, source_name.to_owned(), document.bytes().to_vec())
        .map_err(LocationProjectionError::Source)?;
    Ok(json!({
        "range": project_range(&source, span, snapshot.position_encoding())?,
        "uri": document.uri(),
    }))
}

pub(crate) fn range_value(
    source_name: &str,
    span: Span,
    snapshot: &RequestSnapshot,
    compiler: &ling_db::CompilerDb,
) -> Result<Value, LocationProjectionError> {
    let document = target_document(snapshot, source_name)?;
    let file = compiler
        .vfs()
        .file_id(source_name)
        .ok_or(LocationProjectionError::MissingTargetDocument)?;
    let source = SourceFile::from_bytes(file, source_name.to_owned(), document.bytes().to_vec())
        .map_err(LocationProjectionError::Source)?;
    project_range(&source, span, snapshot.position_encoding())
}

pub(crate) fn target_document<'snapshot>(
    snapshot: &'snapshot RequestSnapshot,
    source_name: &str,
) -> Result<&'snapshot RequestDocument, LocationProjectionError> {
    let mut matching = snapshot
        .documents()
        .iter()
        .filter(|document| document.logical_name() == source_name);
    let document = matching
        .next()
        .ok_or(LocationProjectionError::MissingTargetDocument)?;
    if matching.next().is_some() {
        return Err(LocationProjectionError::DuplicateTargetDocument);
    }
    Ok(document)
}

pub(crate) fn identifier_text(
    document: &RequestDocument,
    span: Span,
    source: SourceId,
) -> Result<&str, LocationProjectionError> {
    if span.source() != source || span.start() >= span.end() {
        return Err(LocationProjectionError::InvalidSpan);
    }
    let start =
        usize::try_from(span.start().get()).map_err(|_| LocationProjectionError::InvalidSpan)?;
    let end =
        usize::try_from(span.end().get()).map_err(|_| LocationProjectionError::InvalidSpan)?;
    document
        .bytes()
        .get(start..end)
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .ok_or(LocationProjectionError::InvalidSpan)
}

fn project_range(
    source: &SourceFile,
    span: Span,
    encoding: ling_source::PositionEncoding,
) -> Result<Value, LocationProjectionError> {
    if span.source() != source.id() || span.start() >= span.end() {
        return Err(LocationProjectionError::InvalidSpan);
    }
    let start = source
        .lsp_position(span.start(), encoding)
        .map_err(LocationProjectionError::Position)?;
    let end = source
        .lsp_position(span.end(), encoding)
        .map_err(LocationProjectionError::Position)?;
    Ok(json!({
        "end": {"character": end.character(), "line": end.line()},
        "start": {"character": start.character(), "line": start.line()},
    }))
}
