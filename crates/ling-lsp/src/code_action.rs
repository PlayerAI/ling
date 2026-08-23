use ling_source::{LspPosition, SourceFile, SourceId};
use serde_json::{Map, Value, json};

use crate::RequestSnapshot;

pub const CODE_ACTION_PROTOCOL_VERSION: &str = "ling.lsp.code-action/0.1";
pub const FORMAT_ACTION_KIND: &str = "source.fixAll.ling.format";

const FIX_PLAN_SOURCE_ID: SourceId = SourceId::new(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CodeActionOptions {
    enabled: bool,
}

impl CodeActionOptions {
    pub(crate) const fn disabled() -> Self {
        Self { enabled: false }
    }

    pub(crate) const fn enabled(self) -> bool {
        self.enabled
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CodeActionParams {
    uri: String,
    start: LspPosition,
    end: LspPosition,
    format_requested: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CodeActionError {
    InvalidParams,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FormatterFixPlan {
    uri: String,
    version: i64,
    replacement: String,
    end: LspPosition,
}

pub(crate) fn parse_code_action_capability(
    text_document: &Map<String, Value>,
    transactional_workspace_edit: bool,
) -> Result<CodeActionOptions, ()> {
    let Some(code_action) = text_document.get("codeAction") else {
        return Ok(CodeActionOptions::disabled());
    };
    let code_action = code_action.as_object().ok_or(())?;
    if let Some(dynamic_registration) = code_action.get("dynamicRegistration") {
        dynamic_registration.as_bool().ok_or(())?;
    }
    let Some(literal_support) = code_action.get("codeActionLiteralSupport") else {
        return Ok(CodeActionOptions::disabled());
    };
    let literal_support = literal_support.as_object().ok_or(())?;
    let code_action_kind = literal_support
        .get("codeActionKind")
        .and_then(Value::as_object)
        .ok_or(())?;
    let value_set = code_action_kind
        .get("valueSet")
        .and_then(Value::as_array)
        .filter(|values| !values.is_empty())
        .ok_or(())?;
    let mut supports_format = false;
    for value in value_set {
        let value = value.as_str().ok_or(())?;
        supports_format |= value == FORMAT_ACTION_KIND;
    }
    Ok(CodeActionOptions {
        enabled: supports_format && transactional_workspace_edit,
    })
}

pub(crate) fn parse_code_action_params(params: &Value) -> Result<CodeActionParams, ()> {
    let object = params.as_object().ok_or(())?;
    let uri = object
        .get("textDocument")
        .and_then(Value::as_object)
        .and_then(|document| document.get("uri"))
        .and_then(Value::as_str)
        .ok_or(())?
        .to_owned();
    let range = object.get("range").and_then(Value::as_object).ok_or(())?;
    let start = parse_position(range.get("start")).ok_or(())?;
    let end = parse_position(range.get("end")).ok_or(())?;
    let context = object.get("context").and_then(Value::as_object).ok_or(())?;
    context
        .get("diagnostics")
        .and_then(Value::as_array)
        .ok_or(())?;
    let format_requested = match context.get("only") {
        None => true,
        Some(only) => {
            let only = only
                .as_array()
                .filter(|values| !values.is_empty())
                .ok_or(())?;
            let mut requested = false;
            for value in only {
                let value = value.as_str().ok_or(())?;
                requested |=
                    matches!(value, "source" | "source.fixAll") || value == FORMAT_ACTION_KIND;
            }
            requested
        }
    };
    if let Some(trigger_kind) = context.get("triggerKind") {
        if !matches!(trigger_kind.as_u64(), Some(1 | 2)) {
            return Err(());
        }
    }
    Ok(CodeActionParams {
        uri,
        start,
        end,
        format_requested,
    })
}

pub(crate) fn build_code_actions(
    snapshot: &RequestSnapshot,
    params: &CodeActionParams,
) -> Result<Value, CodeActionError> {
    let document = snapshot
        .document(&params.uri)
        .ok_or(CodeActionError::InvalidParams)?;
    let source = SourceFile::from_bytes(
        FIX_PLAN_SOURCE_ID,
        document.logical_name(),
        document.bytes().to_vec(),
    )
    .map_err(|_| CodeActionError::Unavailable)?;
    let start = source
        .original_offset(params.start, snapshot.position_encoding())
        .map_err(|_| CodeActionError::InvalidParams)?;
    let end = source
        .original_offset(params.end, snapshot.position_encoding())
        .map_err(|_| CodeActionError::InvalidParams)?;
    if start > end {
        return Err(CodeActionError::InvalidParams);
    }
    if !document.is_open() || !document.is_writable() {
        return Err(CodeActionError::Unavailable);
    }
    let version = document
        .client_version()
        .ok_or(CodeActionError::Unavailable)?;
    if !params.format_requested {
        return Ok(json!([]));
    }

    let parsed = ling_syntax::parse(&source);
    let format_document =
        ling_format::build_format_ir(&source, &parsed).map_err(|_| CodeActionError::Unavailable)?;
    let Some(edit) = ling_format::format_core_edit(&format_document)
        .map_err(|_| CodeActionError::Unavailable)?
    else {
        return Ok(json!([]));
    };
    let expected_end =
        u32::try_from(document.bytes().len()).map_err(|_| CodeActionError::Unavailable)?;
    if edit.source_id() != FIX_PLAN_SOURCE_ID || edit.range() != (0..expected_end) {
        return Err(CodeActionError::Unavailable);
    }
    let projected_end = source
        .lsp_position(
            source.source_map().original_len(),
            snapshot.position_encoding(),
        )
        .map_err(|_| CodeActionError::Unavailable)?;
    let replacement = if source.had_bom() {
        edit.replacement()
            .strip_prefix('\u{feff}')
            .ok_or(CodeActionError::Unavailable)?
    } else {
        edit.replacement()
    };
    let plan = FormatterFixPlan {
        uri: params.uri.clone(),
        version,
        replacement: replacement.to_owned(),
        end: projected_end,
    };
    Ok(render_plan(&plan))
}

fn parse_position(value: Option<&Value>) -> Option<LspPosition> {
    let position = value?.as_object()?;
    let line = u32::try_from(position.get("line")?.as_u64()?).ok()?;
    let character = u32::try_from(position.get("character")?.as_u64()?).ok()?;
    Some(LspPosition::new(line, character))
}

fn render_plan(plan: &FormatterFixPlan) -> Value {
    json!([{
        "edit": {
            "documentChanges": [{
                "edits": [{
                    "newText": plan.replacement,
                    "range": {
                        "end": {
                            "character": plan.end.character(),
                            "line": plan.end.line(),
                        },
                        "start": {"character": 0, "line": 0},
                    },
                }],
                "textDocument": {
                    "uri": plan.uri,
                    "version": plan.version,
                },
            }],
        },
        "isPreferred": true,
        "kind": FORMAT_ACTION_KIND,
        "title": "格式化文档 / Format document",
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_requires_exact_kind_and_transactional_workspace_edit() {
        let text_document = json!({
            "codeAction": {
                "codeActionLiteralSupport": {
                    "codeActionKind": {"valueSet": [FORMAT_ACTION_KIND]}
                }
            }
        });
        let text_document = text_document.as_object().unwrap();
        assert!(
            parse_code_action_capability(text_document, true)
                .unwrap()
                .enabled()
        );
        assert!(
            !parse_code_action_capability(text_document, false)
                .unwrap()
                .enabled()
        );
    }

    #[test]
    fn params_ignore_diagnostic_contents_and_apply_kind_hierarchy() {
        let params = json!({
            "textDocument": {"uri": "ling://workspace/Main.ling"},
            "range": {
                "start": {"line": 0, "character": 0},
                "end": {"line": 0, "character": 1}
            },
            "context": {
                "diagnostics": [{"message": 7, "data": {"repairs": [false]}}],
                "only": ["source.fixAll"]
            }
        });
        let parsed = parse_code_action_params(&params).unwrap();
        assert!(parsed.format_requested);
    }
}
