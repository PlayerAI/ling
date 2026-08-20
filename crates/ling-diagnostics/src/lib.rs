//! Versioned, bilingual diagnostics shared by all compiler stages.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use ling_source::Span;
use serde::Serialize;
use serde::ser::Serializer;
use serde_json::Value;

pub const DIAGNOSTIC_SCHEMA: &str = "ling.diagnostic/0.1";

pub mod codes {
    use super::DiagnosticCode;

    pub const SOURCE_READ_FAILED: DiagnosticCode = DiagnosticCode::new("L-IO-0001");
    pub const PROJECT_LOCK_IO_FAILED: DiagnosticCode = DiagnosticCode::new("L-IO-0002");
    pub const INVALID_UTF8: DiagnosticCode = DiagnosticCode::new("L-LEX-0001");
    pub const MISPLACED_BOM: DiagnosticCode = DiagnosticCode::new("L-LEX-0002");
    pub const SOURCE_TOO_LARGE: DiagnosticCode = DiagnosticCode::new("L-LEX-0003");
    pub const INVALID_IDENTIFIER: DiagnosticCode = DiagnosticCode::new("L-LEX-0004");
    pub const UNEXPECTED_CHARACTER: DiagnosticCode = DiagnosticCode::new("L-LEX-0005");
    pub const UNTERMINATED_BLOCK_COMMENT: DiagnosticCode = DiagnosticCode::new("L-LEX-0006");
    pub const COMMENT_NESTING_TOO_DEEP: DiagnosticCode = DiagnosticCode::new("L-LEX-0007");
    pub const UNTERMINATED_TEXT: DiagnosticCode = DiagnosticCode::new("L-LEX-0008");
    pub const INVALID_TEXT_ESCAPE: DiagnosticCode = DiagnosticCode::new("L-LEX-0009");
    pub const INVALID_UNICODE_ESCAPE: DiagnosticCode = DiagnosticCode::new("L-LEX-0010");
    pub const INVALID_NUMBER: DiagnosticCode = DiagnosticCode::new("L-LEX-0011");
    pub const UNSUPPORTED_CHARACTER_LITERAL: DiagnosticCode = DiagnosticCode::new("L-LEX-0012");
    pub const TAB_IN_INDENTATION: DiagnosticCode = DiagnosticCode::new("L-SYNTAX-0001");
    pub const INCONSISTENT_DEDENT: DiagnosticCode = DiagnosticCode::new("L-SYNTAX-0002");
    pub const LAYOUT_NESTING_TOO_DEEP: DiagnosticCode = DiagnosticCode::new("L-SYNTAX-0003");
    pub const UNMATCHED_CLOSING_DELIMITER: DiagnosticCode = DiagnosticCode::new("L-SYNTAX-0004");
    pub const MISMATCHED_CLOSING_DELIMITER: DiagnosticCode = DiagnosticCode::new("L-SYNTAX-0005");
    pub const UNCLOSED_DELIMITER: DiagnosticCode = DiagnosticCode::new("L-SYNTAX-0006");
    pub const UNEXPECTED_TOKEN: DiagnosticCode = DiagnosticCode::new("L-SYNTAX-0010");
    pub const PARSE_RECURSION_LIMIT: DiagnosticCode = DiagnosticCode::new("L-SYNTAX-0011");
    pub const UNDEFINED_NAME: DiagnosticCode = DiagnosticCode::new("L-NAME-0001");
    pub const DUPLICATE_DEFINITION: DiagnosticCode = DiagnosticCode::new("L-NAME-0002");
    pub const INVALID_MODULE: DiagnosticCode = DiagnosticCode::new("L-NAME-0003");
    pub const DUPLICATE_IMPORT_ALIAS: DiagnosticCode = DiagnosticCode::new("L-NAME-0004");
    pub const IMPORT_CYCLE: DiagnosticCode = DiagnosticCode::new("L-NAME-0005");
    pub const CONFUSABLE_COLLISION: DiagnosticCode = DiagnosticCode::new("L-NAME-0006");
    pub const RESERVED_NAME: DiagnosticCode = DiagnosticCode::new("L-NAME-0007");
    pub const MODULE_NOT_FOUND: DiagnosticCode = DiagnosticCode::new("L-NAME-0008");
    pub const SUSPICIOUS_MIXED_SCRIPT: DiagnosticCode = DiagnosticCode::new("L-NAME-0009");
    pub const TYPE_MISMATCH: DiagnosticCode = DiagnosticCode::new("L-TYPE-0001");
    pub const INFINITE_TYPE: DiagnosticCode = DiagnosticCode::new("L-TYPE-0002");
    pub const CALL_ARITY: DiagnosticCode = DiagnosticCode::new("L-TYPE-0003");
    pub const UNKNOWN_FIELD: DiagnosticCode = DiagnosticCode::new("L-TYPE-0004");
    pub const AMBIGUOUS_RECORD: DiagnosticCode = DiagnosticCode::new("L-TYPE-0005");
    pub const NON_EXHAUSTIVE_MATCH: DiagnosticCode = DiagnosticCode::new("L-TYPE-0006");
    pub const UNREACHABLE_MATCH_CASE: DiagnosticCode = DiagnosticCode::new("L-TYPE-0007");
    pub const DUPLICATE_RECORD_FIELD: DiagnosticCode = DiagnosticCode::new("L-TYPE-0008");
    pub const MISSING_RECORD_FIELDS: DiagnosticCode = DiagnosticCode::new("L-TYPE-0009");
    pub const INVALID_CONSTRUCTOR_PATTERN: DiagnosticCode = DiagnosticCode::new("L-TYPE-0010");
    pub const UNSUPPORTED_EQUALITY: DiagnosticCode = DiagnosticCode::new("L-TYPE-0011");
    pub const INVALID_ASSIGNMENT: DiagnosticCode = DiagnosticCode::new("L-MUT-0001");
    pub const MISSING_CAPABILITY: DiagnosticCode = DiagnosticCode::new("L-CAP-0001");
    pub const UNKNOWN_CAPABILITY: DiagnosticCode = DiagnosticCode::new("L-CAP-0002");
    pub const UNUSED_CAPABILITY: DiagnosticCode = DiagnosticCode::new("L-CAP-0003");
    pub const INVALID_ENTRY_MODULE: DiagnosticCode = DiagnosticCode::new("L-ENTRY-0001");
    pub const MISSING_MAIN: DiagnosticCode = DiagnosticCode::new("L-ENTRY-0002");
    pub const INVALID_MAIN_SIGNATURE: DiagnosticCode = DiagnosticCode::new("L-ENTRY-0003");
    pub const INVALID_PROJECT_MANIFEST_BYTES: DiagnosticCode =
        DiagnosticCode::new("L-PROJECT-0001");
    pub const INVALID_PROJECT_MANIFEST_STRUCTURE: DiagnosticCode =
        DiagnosticCode::new("L-PROJECT-0002");
    pub const UNSUPPORTED_PROJECT_MANIFEST_VERSION: DiagnosticCode =
        DiagnosticCode::new("L-PROJECT-0003");
    pub const INVALID_PROJECT_PACKAGE_METADATA: DiagnosticCode =
        DiagnosticCode::new("L-PROJECT-0004");
    pub const INVALID_PROJECT_SOURCE_LAYOUT: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0005");
    pub const INVALID_PROJECT_EXPORT: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0006");
    pub const INVALID_PROJECT_DEPENDENCY: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0007");
    pub const INVALID_PROJECT_SOURCE_ROOT: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0008");
    pub const INVALID_PROJECT_SOURCE_PATH: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0009");
    pub const INVALID_PROJECT_MODULE_DECLARATION: DiagnosticCode =
        DiagnosticCode::new("L-PROJECT-0010");
    pub const DUPLICATE_PROJECT_MODULE: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0011");
    pub const PROJECT_MODULE_NOT_FOUND: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0012");
    pub const INVALID_PROJECT_IMPORT_GRAPH: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0013");
    pub const INVALID_PROJECT_DEPENDENCY_GRAPH: DiagnosticCode =
        DiagnosticCode::new("L-PROJECT-0014");
    pub const PROJECT_RESOURCE_LIMIT_EXCEEDED: DiagnosticCode =
        DiagnosticCode::new("L-PROJECT-0015");
    pub const PROJECT_DEPENDENCY_MODULE_NOT_FOUND: DiagnosticCode =
        DiagnosticCode::new("L-PROJECT-0016");
    pub const PRIVATE_PROJECT_MODULE_ACCESS: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0017");
    pub const INVALID_PROJECT_LOCK: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0018");
    pub const PROJECT_LOCK_MISMATCH: DiagnosticCode = DiagnosticCode::new("L-PROJECT-0019");
    pub const RUNTIME_FAULT: DiagnosticCode = DiagnosticCode::new("L-RUNTIME-0001");
    pub const AUDIT_SYNTAX: DiagnosticCode = DiagnosticCode::new("L-AUDIT-0001");
    pub const AUDIT_VERSION: DiagnosticCode = DiagnosticCode::new("L-AUDIT-0002");
    pub const AUDIT_STRUCTURE: DiagnosticCode = DiagnosticCode::new("L-AUDIT-0003");
    pub const AUDIT_MODEL: DiagnosticCode = DiagnosticCode::new("L-AUDIT-0004");
    pub const INTERNAL_COMPILER_ERROR: DiagnosticCode = DiagnosticCode::new("L-INTERNAL-0001");
    pub const SEMANTIC_SNAPSHOT_MISMATCH: DiagnosticCode = DiagnosticCode::new("L-SNAPSHOT-0001");
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticCode(&'static str);

impl DiagnosticCode {
    #[must_use]
    pub const fn new(code: &'static str) -> Self {
        Self(code)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl Serialize for DiagnosticCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Note,
}

impl fmt::Display for Severity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
        };
        formatter.write_str(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DiagnosticSpan {
    file: String,
    start_byte: u32,
    end_byte: u32,
}

impl DiagnosticSpan {
    #[must_use]
    pub fn new(file: impl Into<String>, span: Span) -> Self {
        Self {
            file: file.into(),
            start_byte: span.start().get(),
            end_byte: span.end().get(),
        }
    }

    #[must_use]
    pub fn at(file: impl Into<String>, start_byte: u32, end_byte: u32) -> Self {
        Self {
            file: file.into(),
            start_byte,
            end_byte,
        }
    }

    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    #[must_use]
    pub const fn start_byte(&self) -> u32 {
        self.start_byte
    }

    #[must_use]
    pub const fn end_byte(&self) -> u32 {
        self.end_byte
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Repair {
    kind: String,
    changes_semantics: bool,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    facts: BTreeMap<String, Value>,
}

impl Repair {
    #[must_use]
    pub fn new(kind: impl Into<String>, changes_semantics: bool) -> Self {
        Self {
            kind: kind.into(),
            changes_semantics,
            facts: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_fact(mut self, name: impl Into<String>, value: impl Into<Value>) -> Self {
        self.facts.insert(name.into(), value.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Diagnostic {
    schema: &'static str,
    code: DiagnosticCode,
    severity: Severity,
    message_zh: String,
    message_en: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_span: Option<DiagnosticSpan>,
    semantic_id: Option<String>,
    facts: BTreeMap<String, Value>,
    repairs: Vec<Repair>,
}

impl Diagnostic {
    #[must_use]
    pub fn new(
        code: DiagnosticCode,
        severity: Severity,
        message_zh: impl Into<String>,
        message_en: impl Into<String>,
    ) -> Self {
        Self {
            schema: DIAGNOSTIC_SCHEMA,
            code,
            severity,
            message_zh: message_zh.into(),
            message_en: message_en.into(),
            primary_span: None,
            semantic_id: None,
            facts: BTreeMap::new(),
            repairs: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_primary_span(mut self, span: DiagnosticSpan) -> Self {
        self.primary_span = Some(span);
        self
    }

    #[must_use]
    pub fn with_semantic_id(mut self, semantic_id: impl Into<String>) -> Self {
        self.semantic_id = Some(semantic_id.into());
        self
    }

    #[must_use]
    pub fn with_fact(mut self, name: impl Into<String>, value: impl Into<Value>) -> Self {
        self.facts.insert(name.into(), value.into());
        self
    }

    #[must_use]
    pub fn with_repair(mut self, repair: Repair) -> Self {
        self.repairs.push(repair);
        self
    }

    #[must_use]
    pub const fn code(&self) -> DiagnosticCode {
        self.code
    }

    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    #[must_use]
    pub const fn primary_span(&self) -> Option<&DiagnosticSpan> {
        self.primary_span.as_ref()
    }

    pub fn render_json(&self) -> Result<String, RenderError> {
        serde_json::to_string(self).map_err(RenderError)
    }

    #[must_use]
    pub fn render_human(&self, language: MessageLanguage) -> String {
        let message = match language {
            MessageLanguage::Chinese => &self.message_zh,
            MessageLanguage::English => &self.message_en,
        };
        let mut rendered = format!("{}[{}]: {message}", self.severity, self.code);
        if let Some(span) = &self.primary_span {
            rendered.push_str(&format!(
                "\n --> {}:{}..{}",
                span.file, span.start_byte, span.end_byte
            ));
        }
        rendered
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageLanguage {
    Chinese,
    English,
}

#[derive(Debug)]
pub struct RenderError(serde_json::Error);

impl fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "failed to render diagnostic JSON: {}", self.0)
    }
}

impl Error for RenderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_writer_matches_schema_corpus() {
        let diagnostic = Diagnostic::new(
            codes::INVALID_NUMBER,
            Severity::Error,
            "数字字面量格式无效",
            "invalid numeric literal",
        )
        .with_primary_span(DiagnosticSpan::at(
            "tests/conformance/m2-invalid-number/case.ling",
            12,
            17,
        ));
        let expected =
            include_str!("../../../schemas/diagnostic/0.1/valid/invalid-number.json").trim_end();

        assert_eq!(diagnostic.render_json().unwrap(), expected);
    }

    #[test]
    fn json_exposes_protocol_fields_without_freezing_message_wording() {
        let diagnostic = Diagnostic::new(
            codes::INVALID_UTF8,
            Severity::Error,
            "源码不是有效的 UTF-8",
            "source is not valid UTF-8",
        )
        .with_primary_span(DiagnosticSpan::at("main.ling", 2, 3))
        .with_fact("valid_up_to", 2_u64);

        let rendered = diagnostic.render_json().unwrap();
        let value: Value = serde_json::from_str(&rendered).unwrap();
        assert_eq!(value["schema"], DIAGNOSTIC_SCHEMA);
        assert_eq!(value["code"], "L-LEX-0001");
        assert_eq!(value["severity"], "error");
        assert!(
            value["message_zh"]
                .as_str()
                .is_some_and(|text| !text.is_empty())
        );
        assert!(
            value["message_en"]
                .as_str()
                .is_some_and(|text| !text.is_empty())
        );
        assert_eq!(value["primary_span"]["file"], "main.ling");
        assert_eq!(value["primary_span"]["start_byte"], 2);
        assert_eq!(value["primary_span"]["end_byte"], 3);
        assert_eq!(value["facts"]["valid_up_to"], 2);
        assert_eq!(value["repairs"], serde_json::json!([]));
    }

    #[test]
    fn json_orders_fact_maps_and_keeps_repairs_structured() {
        let build = |reverse: bool| {
            let diagnostic = Diagnostic::new(
                codes::TYPE_MISMATCH,
                Severity::Error,
                "类型不匹配",
                "type mismatch",
            );
            let repair = Repair::new("replace_type_annotation", false);
            if reverse {
                diagnostic
                    .with_fact("zeta", 2_u64)
                    .with_fact("alpha", 1_u64)
                    .with_repair(repair.with_fact("zeta", "Int").with_fact("alpha", "Text"))
            } else {
                diagnostic
                    .with_fact("alpha", 1_u64)
                    .with_fact("zeta", 2_u64)
                    .with_repair(repair.with_fact("alpha", "Text").with_fact("zeta", "Int"))
            }
        };

        let forward = build(false).render_json().unwrap();
        let reverse = build(true).render_json().unwrap();
        assert_eq!(forward, reverse);

        let value: Value = serde_json::from_str(&forward).unwrap();
        assert_eq!(value["repairs"][0]["kind"], "replace_type_annotation");
        assert_eq!(value["repairs"][0]["changes_semantics"], false);
        assert_eq!(value["repairs"][0]["facts"]["alpha"], "Text");
        assert_eq!(value["repairs"][0]["facts"]["zeta"], "Int");
    }

    #[test]
    fn json_span_offsets_remain_original_utf8_bytes() {
        let source = "零a";
        let first_scalar_end = u32::try_from(source.find('a').unwrap()).unwrap();
        assert_eq!(first_scalar_end, 3);

        let diagnostic = Diagnostic::new(
            codes::UNEXPECTED_CHARACTER,
            Severity::Error,
            "无法识别字符",
            "unrecognized character",
        )
        .with_primary_span(DiagnosticSpan::at("main.ling", 0, first_scalar_end));
        let value: Value = serde_json::from_str(&diagnostic.render_json().unwrap()).unwrap();
        assert_eq!(value["primary_span"]["start_byte"], 0);
        assert_eq!(value["primary_span"]["end_byte"], 3);
    }

    #[test]
    fn human_output_can_select_either_language() {
        let diagnostic = Diagnostic::new(
            codes::TYPE_MISMATCH,
            Severity::Error,
            "类型不匹配",
            "type mismatch",
        );

        assert_eq!(
            diagnostic.render_human(MessageLanguage::Chinese),
            "error[L-TYPE-0001]: 类型不匹配"
        );
        assert_eq!(
            diagnostic.render_human(MessageLanguage::English),
            "error[L-TYPE-0001]: type mismatch"
        );
    }
}
