use ling_db::{CheckedHoverEntry, CheckedHoverKind, MAX_CHECKED_HOVER_ENTRIES, QueryError};
use ling_source::{LspPosition, PositionError, SourceError, SourceFile, Span, VfsError};
use serde_json::{Map, Value, json};

use super::publication::compiler_for_snapshot;
use super::{
    HandleOutcome, INVALID_PARAMS, LifecycleState, LspServer, MAX_FRAME_BYTES,
    RequestSnapshotError, error_or_none, success_response,
};

/// Current Preview checked-hover writer marker.
pub const HOVER_PROTOCOL_VERSION: &str = "ling.lsp.hover/0.1";
/// Maximum checked entries examined by one hover request.
pub const MAX_HOVER_ENTRIES: usize = MAX_CHECKED_HOVER_ENTRIES;
/// Maximum UTF-8 bytes in one rendered hover content value.
pub const MAX_HOVER_CONTENT_BYTES: usize = 65_536;

const REQUEST_FAILED: i32 = -32_803;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum HoverMarkup {
    #[default]
    Plaintext,
    Markdown,
}

impl HoverMarkup {
    pub(crate) const fn wire_name(self) -> &'static str {
        match self {
            Self::Plaintext => "plaintext",
            Self::Markdown => "markdown",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum HoverError {
    InvalidParams,
    Snapshot(RequestSnapshotError),
    CompilerInput(VfsError),
    Compiler(QueryError),
    Source(SourceError),
    Position(PositionError),
    InvalidSpan,
    UnsafeMarkup,
    ContentTooLarge,
    Stale,
    ResponseTooLarge,
}

impl LspServer {
    pub(crate) fn hover(&self, is_request: bool, id: Value, params: Value) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        match self.hover_result(&params) {
            Ok(result) => {
                let response = success_response(id.clone(), result);
                if response.len() > MAX_FRAME_BYTES {
                    return hover_error(id, &HoverError::ResponseTooLarge);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => hover_error(id, &error),
        }
    }

    fn hover_result(&self, params: &Value) -> Result<Value, HoverError> {
        let (uri, position) = parse_hover_params(params)?;
        let snapshot = self
            .capture_request_snapshot()
            .map_err(HoverError::Snapshot)?;
        let document = snapshot.document(uri).ok_or(HoverError::InvalidParams)?;
        let temporary = document.is_temporary().then_some(document.uri());
        let mut compiler =
            compiler_for_snapshot(&snapshot, temporary).map_err(HoverError::CompilerInput)?;
        let file = compiler
            .vfs()
            .file_id(document.logical_name())
            .ok_or(HoverError::InvalidParams)?;
        let source = SourceFile::from_bytes(
            file,
            document.logical_name().to_owned(),
            document.bytes().to_vec(),
        )
        .map_err(HoverError::Source)?;
        let offset = source
            .original_offset(position, snapshot.position_encoding())
            .map_err(|_| HoverError::InvalidParams)?;
        let index = compiler
            .checked_hover_index(file)
            .map_err(HoverError::Compiler)?;
        let result = index
            .source_entry_at(document.logical_name(), offset)
            .map(|entry| {
                hover_value(
                    entry,
                    &source,
                    snapshot.position_encoding(),
                    self.hover_markup,
                )
            })
            .transpose()?
            .unwrap_or(Value::Null);
        if self
            .capture_request_snapshot()
            .map_err(HoverError::Snapshot)?
            != snapshot
        {
            return Err(HoverError::Stale);
        }
        Ok(result)
    }
}

pub(crate) fn parse_hover_capability(
    text_document: &Map<String, Value>,
) -> Result<HoverMarkup, ()> {
    let Some(hover) = text_document.get("hover") else {
        return Ok(HoverMarkup::Plaintext);
    };
    let hover = hover.as_object().ok_or(())?;
    let Some(formats) = hover.get("contentFormat") else {
        return Ok(HoverMarkup::Plaintext);
    };
    let formats = formats
        .as_array()
        .filter(|values| !values.is_empty())
        .ok_or(())?;
    if formats.iter().any(|value| !value.is_string()) {
        return Err(());
    }
    formats
        .iter()
        .filter_map(Value::as_str)
        .find_map(|format| match format {
            "plaintext" => Some(HoverMarkup::Plaintext),
            "markdown" => Some(HoverMarkup::Markdown),
            _ => None,
        })
        .ok_or(())
}

fn parse_hover_params(params: &Value) -> Result<(&str, LspPosition), HoverError> {
    let object = params.as_object().ok_or(HoverError::InvalidParams)?;
    let uri = object
        .get("textDocument")
        .and_then(Value::as_object)
        .and_then(|document| document.get("uri"))
        .and_then(Value::as_str)
        .ok_or(HoverError::InvalidParams)?;
    let position = object
        .get("position")
        .and_then(Value::as_object)
        .ok_or(HoverError::InvalidParams)?;
    let line = position
        .get("line")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or(HoverError::InvalidParams)?;
    let character = position
        .get("character")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or(HoverError::InvalidParams)?;
    Ok((uri, LspPosition::new(line, character)))
}

fn hover_value(
    entry: &CheckedHoverEntry,
    source: &SourceFile,
    encoding: ling_source::PositionEncoding,
    markup: HoverMarkup,
) -> Result<Value, HoverError> {
    let content = render_hover(entry, markup)?;
    if content.len() > MAX_HOVER_CONTENT_BYTES {
        return Err(HoverError::ContentTooLarge);
    }
    Ok(json!({
        "contents": {
            "kind": markup.wire_name(),
            "value": content,
        },
        "range": project_range(source, entry.span(), encoding)?,
    }))
}

fn render_hover(entry: &CheckedHoverEntry, markup: HoverMarkup) -> Result<String, HoverError> {
    let signature = entry.type_display().map_or_else(
        || entry.name().to_owned(),
        |display| format!("{}: {display}", entry.name()),
    );
    let mut facts = vec![("种类 / kind", kind_label(entry.kind()).to_owned())];
    if entry.is_mutable() {
        facts.push(("可变 / mutable", "true".to_owned()));
    }
    if !entry.effects().is_empty() {
        facts.push(("效果 / effects", entry.effects().join(", ")));
    }
    if !entry.capabilities().is_empty() {
        facts.push(("能力 / capabilities", entry.capabilities().join(", ")));
    }
    if let Some(selection) = entry.trait_selection() {
        facts.push((
            "Trait 选择 / Trait selection",
            format!(
                "{}<{}>.{}",
                selection.trait_name(),
                selection.receiver(),
                selection.member()
            ),
        ));
    }
    Ok(match markup {
        HoverMarkup::Plaintext => {
            let mut content = signature;
            for (label, value) in facts {
                content.push('\n');
                content.push_str(label);
                content.push_str(": ");
                content.push_str(&value);
            }
            content
        }
        HoverMarkup::Markdown => {
            if signature.contains('`') || facts.iter().any(|(_, value)| value.contains('`')) {
                return Err(HoverError::UnsafeMarkup);
            }
            let mut content = format!("```ling\n{signature}\n```");
            for (label, value) in facts {
                content.push_str("\n- ");
                content.push_str(label);
                content.push_str(": `");
                content.push_str(&value);
                content.push('`');
            }
            content
        }
    })
}

fn project_range(
    source: &SourceFile,
    span: Span,
    encoding: ling_source::PositionEncoding,
) -> Result<Value, HoverError> {
    if span.source() != source.id() || span.start() >= span.end() {
        return Err(HoverError::InvalidSpan);
    }
    let start = source
        .lsp_position(span.start(), encoding)
        .map_err(HoverError::Position)?;
    let end = source
        .lsp_position(span.end(), encoding)
        .map_err(HoverError::Position)?;
    Ok(json!({
        "end": {"character": end.character(), "line": end.line()},
        "start": {"character": start.character(), "line": start.line()},
    }))
}

const fn kind_label(kind: CheckedHoverKind) -> &'static str {
    match kind {
        CheckedHoverKind::Value => "value",
        CheckedHoverKind::Type => "type",
        CheckedHoverKind::Constructor => "constructor",
        CheckedHoverKind::Builtin => "builtin",
        CheckedHoverKind::TraitMember => "trait-member",
        CheckedHoverKind::ImplementationMember => "implementation-member",
        CheckedHoverKind::Binding => "binding",
        CheckedHoverKind::Parameter => "parameter",
    }
}

fn hover_error(id: Value, error: &HoverError) -> HandleOutcome {
    match error {
        HoverError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "悬停参数无效 / invalid hover parameters",
        ),
        HoverError::Snapshot(_)
        | HoverError::CompilerInput(_)
        | HoverError::Compiler(_)
        | HoverError::Source(_)
        | HoverError::Position(_)
        | HoverError::InvalidSpan
        | HoverError::UnsafeMarkup
        | HoverError::ContentTooLarge
        | HoverError::Stale
        | HoverError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "悬停信息不可用 / hover information unavailable",
        ),
    }
}
