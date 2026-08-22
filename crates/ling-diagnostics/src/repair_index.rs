use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde_json::Value;

use crate::{Diagnostic, DiagnosticCode, DiagnosticSpan, Severity};

/// One structured repair observation copied from a diagnostic.
///
/// This is not a code action, edit, or `FixPlan`. It retains only the existing
/// diagnostic code/span and structured `Repair` payload so a future adapter
/// need not parse localized message text.
#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticRepairObservation {
    diagnostic_code: DiagnosticCode,
    severity: Severity,
    primary_span: Option<DiagnosticSpan>,
    repair_index: usize,
    kind: String,
    changes_semantics: bool,
    facts: BTreeMap<String, Value>,
}

impl DiagnosticRepairObservation {
    #[must_use]
    pub const fn diagnostic_code(&self) -> DiagnosticCode {
        self.diagnostic_code
    }

    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    #[must_use]
    pub const fn primary_span(&self) -> Option<&DiagnosticSpan> {
        self.primary_span.as_ref()
    }

    #[must_use]
    pub const fn repair_index(&self) -> usize {
        self.repair_index
    }

    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    #[must_use]
    pub const fn changes_semantics(&self) -> bool {
        self.changes_semantics
    }

    #[must_use]
    pub fn facts(&self) -> &BTreeMap<String, Value> {
        &self.facts
    }
}

/// Deterministic structured repair observations from validated diagnostics.
///
/// The index does not assign action IDs, applicability, preferred/suppressed
/// state, capabilities, versions, edits, rollback, or protocol semantics.
#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticRepairIndex {
    entries: Box<[DiagnosticRepairObservation]>,
}

impl DiagnosticRepairIndex {
    #[must_use]
    pub fn entries(&self) -> &[DiagnosticRepairObservation] {
        &self.entries
    }

    #[must_use]
    pub fn code_entries(&self, code: DiagnosticCode) -> Vec<&DiagnosticRepairObservation> {
        self.entries
            .iter()
            .filter(|entry| entry.diagnostic_code == code)
            .collect()
    }

    #[must_use]
    pub fn kind_entries(&self, kind: &str) -> Vec<&DiagnosticRepairObservation> {
        self.entries
            .iter()
            .filter(|entry| entry.kind == kind)
            .collect()
    }

    #[must_use]
    pub fn from_diagnostics(diagnostics: &[Diagnostic]) -> Self {
        let mut entries = diagnostics
            .iter()
            .flat_map(|diagnostic| {
                diagnostic
                    .repairs()
                    .iter()
                    .enumerate()
                    .map(|(repair_index, repair)| DiagnosticRepairObservation {
                        diagnostic_code: diagnostic.code(),
                        severity: diagnostic.severity(),
                        primary_span: diagnostic.primary_span().cloned(),
                        repair_index,
                        kind: repair.kind().to_owned(),
                        changes_semantics: repair.changes_semantics(),
                        facts: repair.facts().clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        entries.sort_by(observation_order);
        debug_assert!(
            entries
                .windows(2)
                .all(|pair| { observation_order(&pair[0], &pair[1]) != Ordering::Greater })
        );
        Self {
            entries: entries.into_boxed_slice(),
        }
    }
}

fn observation_order(
    left: &DiagnosticRepairObservation,
    right: &DiagnosticRepairObservation,
) -> Ordering {
    left.diagnostic_code
        .cmp(&right.diagnostic_code)
        .then_with(|| severity_order(left.severity).cmp(&severity_order(right.severity)))
        .then_with(|| span_order(left.primary_span.as_ref(), right.primary_span.as_ref()))
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.changes_semantics.cmp(&right.changes_semantics))
        .then_with(|| facts_key(&left.facts).cmp(&facts_key(&right.facts)))
        .then_with(|| left.repair_index.cmp(&right.repair_index))
}

const fn severity_order(severity: Severity) -> u8 {
    match severity {
        Severity::Error => 0,
        Severity::Warning => 1,
        Severity::Note => 2,
    }
}

fn span_order(left: Option<&DiagnosticSpan>, right: Option<&DiagnosticSpan>) -> Ordering {
    match (left, right) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(left), Some(right)) => left
            .file()
            .cmp(right.file())
            .then_with(|| left.start_byte().cmp(&right.start_byte()))
            .then_with(|| left.end_byte().cmp(&right.end_byte())),
    }
}

fn facts_key(facts: &BTreeMap<String, Value>) -> String {
    serde_json::to_string(facts).expect("serde_json::Value facts are serializable")
}

#[cfg(test)]
mod tests {
    use crate::{Diagnostic, DiagnosticSpan, Repair, Severity, codes};

    use super::*;

    fn diagnostic(message: &str, kind: &str, value: &str) -> Diagnostic {
        Diagnostic::new(codes::INVALID_NUMBER, Severity::Error, message, message)
            .with_primary_span(DiagnosticSpan::at("Main.ling", 4, 5))
            .with_repair(Repair::new(kind, false).with_fact("replacement", value))
    }

    #[test]
    fn indexes_structured_repairs_without_using_message_text() {
        let first = diagnostic("中文提示", "replace-token", "value");
        let second = diagnostic("different wording", "replace-token", "value");
        let index = DiagnosticRepairIndex::from_diagnostics(&[first, second]);

        assert_eq!(index.entries().len(), 2);
        assert_eq!(index.kind_entries("replace-token").len(), 2);
        assert_eq!(index.entries()[0].diagnostic_code(), codes::INVALID_NUMBER);
        assert_eq!(
            index.entries()[0].primary_span().unwrap().file(),
            "Main.ling"
        );
        assert_eq!(
            index.entries()[0].facts()["replacement"],
            Value::String("value".to_owned())
        );
    }

    #[test]
    fn construction_is_repeatable_and_empty_diagnostics_publish_no_repairs() {
        let diagnostics = [
            diagnostic("提示", "one", "a"),
            diagnostic("提示", "two", "b"),
        ];
        let first = DiagnosticRepairIndex::from_diagnostics(&diagnostics);
        let second = DiagnosticRepairIndex::from_diagnostics(&diagnostics);

        assert_eq!(first, second);
        assert!(
            DiagnosticRepairIndex::from_diagnostics(&[])
                .entries()
                .is_empty()
        );
    }
}
