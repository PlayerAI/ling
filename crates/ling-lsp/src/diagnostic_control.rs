use std::collections::{BTreeMap, BTreeSet};

use ling_diagnostics::codes;
use serde_json::{Map, Value, json};

use super::publication::{DiagnosticAnalysisError, DiagnosticAnalysisResult, grouped_diagnostics};

/// Version marker for deterministic LSP diagnostic storm control.
pub const DIAGNOSTIC_CONTROL_PROTOCOL_VERSION: &str = "ling.lsp.diagnostic-control/0.1";
/// Default maximum retained compiler diagnostics for one document.
pub const DEFAULT_MAX_DIAGNOSTICS_PER_DOCUMENT: usize = 100;
/// Default maximum retained compiler diagnostics for one workspace snapshot.
pub const DEFAULT_MAX_DIAGNOSTICS_PER_WORKSPACE: usize = 1_000;
/// Largest configurable per-document diagnostic limit.
pub const MAX_DIAGNOSTICS_PER_DOCUMENT: usize = 4_096;
/// Largest configurable per-workspace diagnostic limit.
pub const MAX_DIAGNOSTICS_PER_WORKSPACE: usize = 65_536;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DiagnosticLimits {
    per_document: usize,
    per_workspace: usize,
}

impl DiagnosticLimits {
    pub(crate) const DEFAULT: Self = Self {
        per_document: DEFAULT_MAX_DIAGNOSTICS_PER_DOCUMENT,
        per_workspace: DEFAULT_MAX_DIAGNOSTICS_PER_WORKSPACE,
    };

    pub(crate) const fn new(per_document: usize, per_workspace: usize) -> Self {
        Self {
            per_document,
            per_workspace,
        }
    }

    pub(crate) const fn per_document(self) -> usize {
        self.per_document
    }

    pub(crate) const fn per_workspace(self) -> usize {
        self.per_workspace
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiagnosticControlError {
    InvalidDiagnosticShape,
    Serialization,
    CountOverflow,
}

impl std::fmt::Display for DiagnosticControlError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDiagnosticShape => {
                formatter.write_str("diagnostic control received an invalid adapter value")
            }
            Self::Serialization => {
                formatter.write_str("diagnostic control identity serialization failed")
            }
            Self::CountOverflow => formatter.write_str("diagnostic control count overflow"),
        }
    }
}

impl std::error::Error for DiagnosticControlError {}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Omission {
    deduplicated: usize,
    capped: usize,
    first_range: Option<Value>,
}

impl Omission {
    fn omit_duplicate(&mut self, diagnostic: &Value) -> Result<(), DiagnosticControlError> {
        self.capture_range(diagnostic)?;
        self.deduplicated = self
            .deduplicated
            .checked_add(1)
            .ok_or(DiagnosticControlError::CountOverflow)?;
        Ok(())
    }

    fn omit_capped(&mut self, diagnostics: &[Value]) -> Result<(), DiagnosticControlError> {
        if let Some(first) = diagnostics.first() {
            self.capture_range(first)?;
        }
        self.capped = self
            .capped
            .checked_add(diagnostics.len())
            .ok_or(DiagnosticControlError::CountOverflow)?;
        Ok(())
    }

    fn capture_range(&mut self, diagnostic: &Value) -> Result<(), DiagnosticControlError> {
        let range = diagnostic_range(diagnostic)?;
        if self.first_range.is_none() {
            self.first_range = Some(range.clone());
        }
        Ok(())
    }

    fn omitted(&self) -> Result<usize, DiagnosticControlError> {
        self.deduplicated
            .checked_add(self.capped)
            .ok_or(DiagnosticControlError::CountOverflow)
    }
}

pub(crate) fn parse_diagnostic_limits(params: &Map<String, Value>) -> Result<DiagnosticLimits, ()> {
    let Some(options) = params.get("initializationOptions") else {
        return Ok(DiagnosticLimits::DEFAULT);
    };
    let Some(options) = options.as_object() else {
        return Ok(DiagnosticLimits::DEFAULT);
    };
    let Some(control) = options.get("lingDiagnosticControl") else {
        return Ok(DiagnosticLimits::DEFAULT);
    };
    let control = control.as_object().ok_or(())?;
    let per_document = parse_limit(
        control.get("maxPerDocument"),
        DEFAULT_MAX_DIAGNOSTICS_PER_DOCUMENT,
        MAX_DIAGNOSTICS_PER_DOCUMENT,
    )?;
    let per_workspace = parse_limit(
        control.get("maxPerWorkspace"),
        DEFAULT_MAX_DIAGNOSTICS_PER_WORKSPACE,
        MAX_DIAGNOSTICS_PER_WORKSPACE,
    )?;
    Ok(DiagnosticLimits::new(per_document, per_workspace))
}

fn parse_limit(value: Option<&Value>, default: usize, maximum: usize) -> Result<usize, ()> {
    let Some(value) = value else {
        return Ok(default);
    };
    value
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .filter(|value| (1..=maximum).contains(value))
        .ok_or(())
}

pub(crate) fn controlled_diagnostics(
    result: &DiagnosticAnalysisResult,
    limits: DiagnosticLimits,
) -> Result<BTreeMap<String, Vec<Value>>, DiagnosticAnalysisError> {
    let grouped = grouped_diagnostics(result)?;
    apply_control(grouped, limits).map_err(DiagnosticAnalysisError::Control)
}

fn apply_control(
    mut grouped: BTreeMap<String, Vec<Value>>,
    limits: DiagnosticLimits,
) -> Result<BTreeMap<String, Vec<Value>>, DiagnosticControlError> {
    let mut document_omissions = BTreeMap::<String, Omission>::new();
    for (uri, diagnostics) in &mut grouped {
        let omission = document_omissions.entry(uri.clone()).or_default();
        let mut roots = BTreeSet::new();
        let mut retained = Vec::with_capacity(diagnostics.len());
        for diagnostic in diagnostics.drain(..) {
            let identity = root_identity(&diagnostic)?;
            if roots.insert(identity) {
                retained.push(diagnostic);
            } else {
                omission.omit_duplicate(&diagnostic)?;
            }
        }
        if retained.len() > limits.per_document() {
            let capped = retained.split_off(limits.per_document());
            omission.omit_capped(&capped)?;
        }
        *diagnostics = retained;
    }

    let mut remaining = limits.per_workspace();
    let mut workspace_omission = Omission::default();
    let mut workspace_summary_uri = None;
    for (uri, diagnostics) in &mut grouped {
        let retained = diagnostics.len().min(remaining);
        remaining -= retained;
        if retained < diagnostics.len() {
            let capped = diagnostics.split_off(retained);
            if workspace_summary_uri.is_none() {
                workspace_summary_uri = Some(uri.clone());
            }
            workspace_omission.omit_capped(&capped)?;
        }
    }

    for (uri, omission) in document_omissions {
        if omission.omitted()? > 0 {
            let summary = omission_summary("document", limits.per_document(), &omission)?;
            grouped
                .get_mut(&uri)
                .ok_or(DiagnosticControlError::InvalidDiagnosticShape)?
                .push(summary);
        }
    }
    if workspace_omission.omitted()? > 0 {
        let uri = workspace_summary_uri.ok_or(DiagnosticControlError::InvalidDiagnosticShape)?;
        let summary = omission_summary("workspace", limits.per_workspace(), &workspace_omission)?;
        grouped
            .get_mut(&uri)
            .ok_or(DiagnosticControlError::InvalidDiagnosticShape)?
            .push(summary);
    }
    Ok(grouped)
}

fn root_identity(diagnostic: &Value) -> Result<Vec<u8>, DiagnosticControlError> {
    let diagnostic = diagnostic
        .as_object()
        .ok_or(DiagnosticControlError::InvalidDiagnosticShape)?;
    let code = diagnostic
        .get("code")
        .and_then(Value::as_str)
        .ok_or(DiagnosticControlError::InvalidDiagnosticShape)?;
    let range = diagnostic
        .get("range")
        .filter(|range| valid_range(range))
        .ok_or(DiagnosticControlError::InvalidDiagnosticShape)?;
    let data = diagnostic
        .get("data")
        .and_then(Value::as_object)
        .ok_or(DiagnosticControlError::InvalidDiagnosticShape)?;
    let semantic_id = data
        .get("semanticId")
        .filter(|value| value.is_null() || value.is_string())
        .ok_or(DiagnosticControlError::InvalidDiagnosticShape)?;
    let facts = data
        .get("facts")
        .and_then(Value::as_object)
        .ok_or(DiagnosticControlError::InvalidDiagnosticShape)?;
    serde_json::to_vec(&json!([code, range, semantic_id, facts]))
        .map_err(|_| DiagnosticControlError::Serialization)
}

fn diagnostic_range(diagnostic: &Value) -> Result<&Value, DiagnosticControlError> {
    diagnostic
        .as_object()
        .and_then(|diagnostic| diagnostic.get("range"))
        .filter(|range| valid_range(range))
        .ok_or(DiagnosticControlError::InvalidDiagnosticShape)
}

fn valid_range(range: &Value) -> bool {
    let Some(range) = range.as_object() else {
        return false;
    };
    ["start", "end"].into_iter().all(|endpoint| {
        range
            .get(endpoint)
            .and_then(Value::as_object)
            .is_some_and(|position| {
                position.get("line").and_then(Value::as_u64).is_some()
                    && position.get("character").and_then(Value::as_u64).is_some()
            })
    })
}

fn omission_summary(
    scope: &str,
    maximum: usize,
    omission: &Omission,
) -> Result<Value, DiagnosticControlError> {
    let omitted = omission.omitted()?;
    let range = omission
        .first_range
        .as_ref()
        .ok_or(DiagnosticControlError::InvalidDiagnosticShape)?;
    Ok(json!({
        "code": codes::LSP_DIAGNOSTICS_OMITTED.as_str(),
        "data": {
            "facts": {
                "capped": omission.capped,
                "deduplicated": omission.deduplicated,
                "maximum": maximum,
                "omitted": omitted,
                "scope": scope,
            },
            "repairs": [],
            "semanticId": null,
            "version": DIAGNOSTIC_CONTROL_PROTOCOL_VERSION,
        },
        "message": format!("已省略 {omitted} 条诊断 / {omitted} diagnostics omitted"),
        "range": range,
        "relatedInformation": [],
        "severity": 2,
        "source": "ling",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diagnostic(code: &str, line: u64, semantic_id: Value, facts: Value, tag: &str) -> Value {
        json!({
            "code": code,
            "data": {
                "facts": facts,
                "repairs": [{"changes_semantics": false, "kind": tag}],
                "semanticId": semantic_id,
                "version": "ling.lsp.diagnostic/0.2",
            },
            "message": tag,
            "range": {
                "end": {"character": 1, "line": line},
                "start": {"character": 0, "line": line},
            },
            "relatedInformation": [],
            "severity": 1,
            "source": "ling",
        })
    }

    #[test]
    fn first_root_wins_and_independent_root_members_remain() {
        let first = diagnostic("L-TYPE-0001", 1, Value::Null, json!({}), "first");
        let duplicate = diagnostic("L-TYPE-0001", 1, Value::Null, json!({}), "later");
        let other_range = diagnostic("L-TYPE-0001", 2, Value::Null, json!({}), "range");
        let other_facts = diagnostic("L-TYPE-0001", 1, Value::Null, json!({"x": 1}), "facts");
        let other_semantic =
            diagnostic("L-TYPE-0001", 1, json!("sid:def:1"), json!({}), "semantic");
        let mut grouped = BTreeMap::new();
        grouped.insert(
            "ling://workspace/a.ling".to_owned(),
            vec![
                first.clone(),
                duplicate,
                other_facts.clone(),
                other_semantic.clone(),
                other_range.clone(),
            ],
        );
        let controlled = apply_control(grouped, DiagnosticLimits::new(10, 10)).unwrap();
        let items = &controlled["ling://workspace/a.ling"];
        assert_eq!(
            &items[..4],
            &[first, other_facts, other_semantic, other_range]
        );
        assert_eq!(items[4]["code"], "L-LSP-0001");
        assert_eq!(items[4]["data"]["facts"]["deduplicated"], 1);
        assert_eq!(items[4]["data"]["facts"]["capped"], 0);
    }

    #[test]
    fn document_and_workspace_caps_report_each_omission_exactly() {
        let mut grouped = BTreeMap::new();
        grouped.insert(
            "ling://workspace/a.ling".to_owned(),
            vec![
                diagnostic("L-SYNTAX-0010", 0, Value::Null, json!({}), "a0"),
                diagnostic("L-SYNTAX-0010", 1, Value::Null, json!({}), "a1"),
                diagnostic("L-SYNTAX-0010", 2, Value::Null, json!({}), "a2"),
            ],
        );
        grouped.insert(
            "ling://workspace/b.ling".to_owned(),
            vec![
                diagnostic("L-TYPE-0001", 0, Value::Null, json!({}), "b0"),
                diagnostic("L-TYPE-0001", 1, Value::Null, json!({}), "b1"),
            ],
        );
        let controlled = apply_control(grouped, DiagnosticLimits::new(2, 3)).unwrap();
        let a = &controlled["ling://workspace/a.ling"];
        let b = &controlled["ling://workspace/b.ling"];
        assert_eq!(a.len(), 3);
        assert_eq!(a[2]["data"]["facts"]["scope"], "document");
        assert_eq!(a[2]["data"]["facts"]["capped"], 1);
        assert_eq!(b.len(), 2);
        assert_eq!(b[1]["data"]["facts"]["scope"], "workspace");
        assert_eq!(b[1]["data"]["facts"]["capped"], 1);
    }

    #[test]
    fn malformed_adapter_value_fails_the_complete_transformation() {
        let mut grouped = BTreeMap::new();
        grouped.insert("ling://workspace/a.ling".to_owned(), vec![json!({})]);
        assert_eq!(
            apply_control(grouped, DiagnosticLimits::DEFAULT),
            Err(DiagnosticControlError::InvalidDiagnosticShape)
        );
    }

    #[test]
    fn one_uri_can_report_independent_document_and_workspace_omissions() {
        let mut grouped = BTreeMap::new();
        grouped.insert(
            "ling://workspace/a.ling".to_owned(),
            vec![
                diagnostic("L-SYNTAX-0010", 0, Value::Null, json!({}), "zero"),
                diagnostic("L-SYNTAX-0010", 1, Value::Null, json!({}), "one"),
                diagnostic("L-SYNTAX-0010", 2, Value::Null, json!({}), "two"),
            ],
        );
        let controlled = apply_control(grouped, DiagnosticLimits::new(2, 1)).unwrap();
        let items = &controlled["ling://workspace/a.ling"];
        assert_eq!(items.len(), 3);
        assert_eq!(items[0]["message"], "zero");
        assert_eq!(items[1]["data"]["facts"]["scope"], "document");
        assert_eq!(items[1]["data"]["facts"]["omitted"], 1);
        assert_eq!(items[2]["data"]["facts"]["scope"], "workspace");
        assert_eq!(items[2]["data"]["facts"]["omitted"], 1);
    }

    #[test]
    fn repeated_registered_resource_limit_is_one_root_with_explicit_omission() {
        let resource = diagnostic(
            "L-PROJECT-0015",
            4,
            Value::Null,
            json!({
                "actual": "65",
                "maximum": "64",
                "package": "app",
                "resource": "trait_solver_depth",
            }),
            "resource",
        );
        let mut grouped = BTreeMap::new();
        grouped.insert(
            "ling://workspace/a.ling".to_owned(),
            vec![resource.clone(), resource.clone()],
        );
        let controlled = apply_control(grouped, DiagnosticLimits::DEFAULT).unwrap();
        let items = &controlled["ling://workspace/a.ling"];
        assert_eq!(items[0], resource);
        assert_eq!(items[1]["code"], "L-LSP-0001");
        assert_eq!(items[1]["data"]["facts"]["deduplicated"], 1);
        assert_eq!(items[1]["data"]["facts"]["capped"], 0);
    }
}
