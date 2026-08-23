use std::collections::BTreeMap;

use blake3::Hasher;
use serde_json::{Value, json};

use super::publication::grouped_diagnostics;
use super::{
    DIAGNOSTIC_PROTOCOL_VERSION, HandleOutcome, INTERNAL_ERROR, INVALID_PARAMS, LifecycleState,
    LspServer, MAX_FRAME_BYTES, METHOD_NOT_FOUND, RequestDocument, document_identity,
    error_or_none, success_response,
};

/// Version marker for the capability-gated Preview pull-diagnostics protocol.
pub const PULL_DIAGNOSTICS_PROTOCOL_VERSION: &str = "ling.lsp.pull-diagnostics/0.1";
/// Maximum previous-result entries accepted by one workspace request.
pub const MAX_PULL_PREVIOUS_RESULTS: usize = 1_024;

const REQUEST_FAILED: i32 = -32_803;
const RESULT_ID_DOMAIN: &str = "ling.lsp.pull-result-id/0.1";
const RESULT_ID_PREFIX: &str = "ling.lsp.pull-result/0.1:blake3:";

#[derive(Clone, Debug, Eq, PartialEq)]
struct DocumentPullParams {
    uri: String,
    previous_result_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkspacePullParams {
    previous_result_ids: BTreeMap<String, String>,
}

impl LspServer {
    pub(super) fn document_diagnostic(
        &self,
        is_request: bool,
        id: Value,
        params: Value,
    ) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error_for(true, id);
        }
        if !self.pull_diagnostics_supported {
            return error_or_none(
                true,
                id,
                METHOD_NOT_FOUND,
                "未协商拉取诊断 / pull diagnostics were not negotiated",
            );
        }

        let Ok(params) = parse_document_params(&params) else {
            return invalid_params(id);
        };
        if self.document(&params.uri).is_none() {
            return invalid_params(id);
        }

        let Ok(analysis) = self.analyze_current_diagnostics() else {
            return internal_error(id);
        };
        let Ok(grouped) = grouped_diagnostics(&analysis) else {
            return internal_error(id);
        };
        let Some(items) = grouped.get(&params.uri) else {
            return internal_error(id);
        };
        let Ok(result_id) = diagnostic_result_id(&params.uri, items) else {
            return internal_error(id);
        };

        let result = if params.previous_result_id.as_deref() == Some(result_id.as_str()) {
            json!({
                "kind": "unchanged",
                "resultId": result_id,
            })
        } else {
            json!({
                "items": items,
                "kind": "full",
                "resultId": result_id,
            })
        };
        bounded_success(id, result)
    }

    pub(super) fn workspace_diagnostic(
        &self,
        is_request: bool,
        id: Value,
        params: Value,
    ) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error_for(true, id);
        }
        if !self.pull_diagnostics_supported {
            return error_or_none(
                true,
                id,
                METHOD_NOT_FOUND,
                "未协商拉取诊断 / pull diagnostics were not negotiated",
            );
        }

        let Ok(params) = parse_workspace_params(&params) else {
            return invalid_params(id);
        };
        let Ok(analysis) = self.analyze_current_diagnostics() else {
            return internal_error(id);
        };
        let Ok(mut grouped) = grouped_diagnostics(&analysis) else {
            return internal_error(id);
        };
        for uri in params.previous_result_ids.keys() {
            grouped.entry(uri.clone()).or_default();
        }

        let mut reports = Vec::with_capacity(grouped.len());
        for (uri, items) in grouped {
            let Ok(result_id) = diagnostic_result_id(&uri, &items) else {
                return internal_error(id);
            };
            let version = analysis
                .snapshot()
                .document(&uri)
                .and_then(RequestDocument::client_version);
            let report = if params.previous_result_ids.get(&uri) == Some(&result_id) {
                json!({
                    "kind": "unchanged",
                    "resultId": result_id,
                    "uri": uri,
                    "version": version,
                })
            } else {
                json!({
                    "items": items,
                    "kind": "full",
                    "resultId": result_id,
                    "uri": uri,
                    "version": version,
                })
            };
            reports.push(report);
        }
        bounded_success(id, json!({"items": reports}))
    }
}

fn parse_document_params(params: &Value) -> Result<DocumentPullParams, ()> {
    let object = params.as_object().ok_or(())?;
    validate_identifier(object.get("identifier"))?;
    let document = object
        .get("textDocument")
        .and_then(Value::as_object)
        .ok_or(())?;
    let uri = document.get("uri").and_then(Value::as_str).ok_or(())?;
    document_identity(uri).map_err(|_| ())?;
    let previous_result_id = object
        .get("previousResultId")
        .map(|value| value.as_str().map(str::to_owned).ok_or(()))
        .transpose()?;
    Ok(DocumentPullParams {
        uri: uri.to_owned(),
        previous_result_id,
    })
}

fn parse_workspace_params(params: &Value) -> Result<WorkspacePullParams, ()> {
    let object = params.as_object().ok_or(())?;
    validate_identifier(object.get("identifier"))?;
    let entries = object
        .get("previousResultIds")
        .and_then(Value::as_array)
        .ok_or(())?;
    if entries.len() > MAX_PULL_PREVIOUS_RESULTS {
        return Err(());
    }

    let mut previous_result_ids = BTreeMap::new();
    for entry in entries {
        let entry = entry.as_object().ok_or(())?;
        let uri = entry.get("uri").and_then(Value::as_str).ok_or(())?;
        document_identity(uri).map_err(|_| ())?;
        let value = entry.get("value").and_then(Value::as_str).ok_or(())?;
        if previous_result_ids
            .insert(uri.to_owned(), value.to_owned())
            .is_some()
        {
            return Err(());
        }
    }
    Ok(WorkspacePullParams {
        previous_result_ids,
    })
}

fn validate_identifier(identifier: Option<&Value>) -> Result<(), ()> {
    match identifier {
        None => Ok(()),
        Some(Value::String(identifier)) if identifier == PULL_DIAGNOSTICS_PROTOCOL_VERSION => {
            Ok(())
        }
        Some(_) => Err(()),
    }
}

fn diagnostic_result_id(uri: &str, diagnostics: &[Value]) -> Result<String, ()> {
    let diagnostics = serde_json::to_vec(diagnostics).map_err(|_| ())?;
    let mut hasher = Hasher::new();
    for bytes in [
        RESULT_ID_DOMAIN.as_bytes(),
        DIAGNOSTIC_PROTOCOL_VERSION.as_bytes(),
        uri.as_bytes(),
        diagnostics.as_slice(),
    ] {
        hash_part(&mut hasher, bytes)?;
    }
    Ok(format!("{RESULT_ID_PREFIX}{}", hasher.finalize().to_hex()))
}

fn hash_part(hasher: &mut Hasher, bytes: &[u8]) -> Result<(), ()> {
    let length = u64::try_from(bytes.len()).map_err(|_| ())?;
    hasher.update(&length.to_be_bytes());
    hasher.update(bytes);
    Ok(())
}

fn bounded_success(id: Value, result: Value) -> HandleOutcome {
    let response = success_response(id.clone(), result);
    if response.len() > MAX_FRAME_BYTES {
        return error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "拉取诊断结果超过传输上限 / pull diagnostic result exceeds transport limit",
        );
    }
    HandleOutcome::Response(response)
}

fn invalid_params(id: Value) -> HandleOutcome {
    error_or_none(
        true,
        id,
        INVALID_PARAMS,
        "拉取诊断参数无效 / invalid pull diagnostic parameters",
    )
}

fn internal_error(id: Value) -> HandleOutcome {
    error_or_none(
        true,
        id,
        INTERNAL_ERROR,
        "拉取诊断失败 / pull diagnostics failed",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_id_uses_exact_domain_separated_length_prefixes() {
        let uri = "ling://workspace/src/Main.ling";
        let diagnostics = vec![json!({"code": "L-TYPE-0001", "message": "x"})];
        let actual = diagnostic_result_id(uri, &diagnostics).unwrap();

        let mut hasher = Hasher::new();
        for bytes in [
            RESULT_ID_DOMAIN.as_bytes(),
            DIAGNOSTIC_PROTOCOL_VERSION.as_bytes(),
            uri.as_bytes(),
            serde_json::to_vec(&diagnostics).unwrap().as_slice(),
        ] {
            let length = u64::try_from(bytes.len()).unwrap();
            hasher.update(&length.to_be_bytes());
            hasher.update(bytes);
        }
        assert_eq!(
            actual,
            format!("{RESULT_ID_PREFIX}{}", hasher.finalize().to_hex())
        );
    }

    #[test]
    fn result_id_separates_uri_and_diagnostic_bytes() {
        let empty = diagnostic_result_id("ling://workspace/a.ling", &[]).unwrap();
        let other_uri = diagnostic_result_id("ling://workspace/b.ling", &[]).unwrap();
        let other_items =
            diagnostic_result_id("ling://workspace/a.ling", &[json!({"code": "L-TYPE-0001"})])
                .unwrap();
        assert_ne!(empty, other_uri);
        assert_ne!(empty, other_items);
    }
}
