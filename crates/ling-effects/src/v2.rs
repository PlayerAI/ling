//! Experimental v0.2 Effect row and first-order handler model.
//!
//! This module is deliberately separate from the v0.0.1 Seed `EffectRow`.
//! Seed checking and bytecode keep their existing closed-set representation;
//! these values are the accepted, path-free core model that later inference
//! and lowering stages may consume.

use std::error::Error;
use std::fmt;

use ling_unicode::validate_identifier;
use unicode_normalization::UnicodeNormalization;

/// A canonical, dot-separated effect identity.
///
/// Every segment is a Unicode 17 XID identifier stored in NFC form. The
/// identity contains no source path, host address, allocation index, or
/// display-only alias.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectId {
    segments: Box<[String]>,
}

impl EffectId {
    /// Parses and NFC-normalizes a dot-separated effect identity.
    pub fn new(raw: &str) -> Result<Self, EffectIdError> {
        if raw.is_empty() {
            return Err(EffectIdError::Empty);
        }
        let mut segments = Vec::new();
        for (ordinal, segment) in raw.split('.').enumerate() {
            if segment.is_empty() {
                return Err(EffectIdError::EmptySegment { ordinal });
            }
            let identifier =
                validate_identifier(segment).map_err(|error| EffectIdError::InvalidSegment {
                    ordinal,
                    segment: segment.to_owned(),
                    reason: error.to_string(),
                })?;
            segments.push(identifier.normalized().to_owned());
        }
        Ok(Self {
            segments: segments.into_boxed_slice(),
        })
    }

    /// Returns the canonical dot-separated spelling.
    #[must_use]
    pub fn as_str(&self) -> String {
        self.segments.join(".")
    }

    /// Returns canonical NFC-normalized segments.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    fn reserved(raw: &str) -> Self {
        Self {
            segments: raw
                .split('.')
                .map(str::to_owned)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl fmt::Display for EffectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_str())
    }
}

/// Errors raised while constructing an [`EffectId`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectIdError {
    Empty,
    EmptySegment {
        ordinal: usize,
    },
    InvalidSegment {
        ordinal: usize,
        segment: String,
        reason: String,
    },
}

impl fmt::Display for EffectIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("effect identity is empty"),
            Self::EmptySegment { ordinal } => {
                write!(formatter, "effect identity segment {ordinal} is empty")
            }
            Self::InvalidSegment {
                ordinal,
                segment,
                reason,
            } => write!(
                formatter,
                "effect identity segment {ordinal} `{segment}` is invalid: {reason}"
            ),
        }
    }
}

impl Error for EffectIdError {}

/// A canonical Typed-Core type reference used as an Effect label parameter.
///
/// The value is not a source-level pretty-printed type. Callers must provide
/// the canonical Typed-Core identity; this wrapper normalizes NFC and rejects
/// source paths, host separators, whitespace, and control characters.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectTypeRef(Box<str>);

impl EffectTypeRef {
    /// Creates a path-free canonical type reference.
    pub fn new(raw: &str) -> Result<Self, EffectTypeRefError> {
        if raw.is_empty() {
            return Err(EffectTypeRefError::Empty);
        }
        if raw.chars().any(|character| {
            character.is_whitespace() || character.is_control() || matches!(character, '/' | '\\')
        }) {
            return Err(EffectTypeRefError::NotCanonical {
                reason: "reference contains whitespace, control, or path separator".to_owned(),
            });
        }
        let normalized: String = raw.nfc().collect();
        Ok(Self(normalized.into_boxed_str()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EffectTypeRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Errors raised while constructing an [`EffectTypeRef`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectTypeRefError {
    Empty,
    NotCanonical { reason: String },
}

impl fmt::Display for EffectTypeRefError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Typed-Core type reference is empty"),
            Self::NotCanonical { reason } => {
                write!(
                    formatter,
                    "Typed-Core type reference is not canonical: {reason}"
                )
            }
        }
    }
}

impl Error for EffectTypeRefError {}

/// A canonical effect identity and its ordered Typed-Core type parameters.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectLabel {
    effect: EffectId,
    parameters: Box<[EffectTypeRef]>,
}

impl EffectLabel {
    #[must_use]
    pub fn new(effect: EffectId, parameters: impl IntoIterator<Item = EffectTypeRef>) -> Self {
        Self {
            effect,
            parameters: parameters.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn effect_id(&self) -> &EffectId {
        &self.effect
    }

    #[must_use]
    pub fn parameters(&self) -> &[EffectTypeRef] {
        &self.parameters
    }

    /// Returns the canonical label spelling used for ordering and projection.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        if self.parameters.is_empty() {
            return self.effect.as_str();
        }
        let parameters = self
            .parameters
            .iter()
            .map(EffectTypeRef::as_str)
            .collect::<Vec<_>>()
            .join(",");
        format!("{}<{parameters}>", self.effect)
    }

    #[must_use]
    pub fn clock() -> Self {
        Self::new(EffectId::reserved("Clock"), [])
    }

    #[must_use]
    pub fn random() -> Self {
        Self::new(EffectId::reserved("Random"), [])
    }

    #[must_use]
    pub fn console_write() -> Self {
        Self::new(EffectId::reserved("Console.Write"), [])
    }

    #[must_use]
    pub fn state(value: EffectTypeRef) -> Self {
        Self::new(EffectId::reserved("State"), [value])
    }

    #[must_use]
    pub fn task_spawn() -> Self {
        Self::new(EffectId::reserved("Task.Spawn"), [])
    }

    #[must_use]
    pub fn task_await() -> Self {
        Self::new(EffectId::reserved("Task.Await"), [])
    }

    #[must_use]
    pub fn task() -> Self {
        Self::new(EffectId::reserved("Task"), [])
    }

    #[must_use]
    pub fn actor_send(value: EffectTypeRef) -> Self {
        Self::new(EffectId::reserved("ActorSend"), [value])
    }
}

impl fmt::Display for EffectLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical_name())
    }
}

/// A binder-local row variable identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RowVariableId(u32);

impl RowVariableId {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// The tail of a v0.2 Effect row.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EffectRowTail {
    Closed,
    Variable(RowVariableId),
}

/// A sorted, duplicate-free Effect label set plus a closed or open tail.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectRowModel {
    labels: Box<[EffectLabel]>,
    tail: EffectRowTail,
}

impl fmt::Display for EffectRowModel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical_name())
    }
}

impl EffectRowModel {
    /// Builds a row, sorting labels and eliminating duplicates deterministically.
    #[must_use]
    pub fn new(labels: impl IntoIterator<Item = EffectLabel>, tail: EffectRowTail) -> Self {
        let mut labels = labels.into_iter().collect::<Vec<_>>();
        labels.sort();
        labels.dedup();
        Self {
            labels: labels.into_boxed_slice(),
            tail,
        }
    }

    #[must_use]
    pub fn pure() -> Self {
        Self::new([], EffectRowTail::Closed)
    }

    #[must_use]
    pub fn closed(labels: impl IntoIterator<Item = EffectLabel>) -> Self {
        Self::new(labels, EffectRowTail::Closed)
    }

    #[must_use]
    pub fn open(labels: impl IntoIterator<Item = EffectLabel>, variable: RowVariableId) -> Self {
        Self::new(labels, EffectRowTail::Variable(variable))
    }

    #[must_use]
    pub fn labels(&self) -> &[EffectLabel] {
        &self.labels
    }

    #[must_use]
    pub const fn tail(&self) -> EffectRowTail {
        self.tail
    }

    #[must_use]
    pub fn is_pure(&self) -> bool {
        self.labels.is_empty() && matches!(self.tail, EffectRowTail::Closed)
    }

    #[must_use]
    pub fn contains(&self, label: &EffectLabel) -> bool {
        self.labels.binary_search(label).is_ok()
    }

    /// Unions rows without guessing when two distinct open tails meet.
    pub fn union(&self, other: &Self) -> Result<Self, EffectRowUnionError> {
        let tail = match (self.tail, other.tail) {
            (EffectRowTail::Closed, right) | (right, EffectRowTail::Closed) => right,
            (EffectRowTail::Variable(left), EffectRowTail::Variable(right)) if left == right => {
                EffectRowTail::Variable(left)
            }
            (EffectRowTail::Variable(left), EffectRowTail::Variable(right)) => {
                return Err(EffectRowUnionError::DistinctTails { left, right });
            }
        };
        let labels = self
            .labels
            .iter()
            .cloned()
            .chain(other.labels.iter().cloned());
        Ok(Self::new(labels, tail))
    }

    /// Removes one explicitly handled label and preserves the row tail.
    #[must_use]
    pub fn without_label(&self, label: &EffectLabel) -> Self {
        Self::new(
            self.labels
                .iter()
                .filter(|candidate| *candidate != label)
                .cloned(),
            self.tail,
        )
    }

    /// Returns canonical label names in sorted order, excluding the tail.
    #[must_use]
    pub fn canonical_names(&self) -> Vec<String> {
        self.labels
            .iter()
            .map(EffectLabel::canonical_name)
            .collect()
    }

    /// Returns a deterministic, path-free row spelling.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        let labels = self.canonical_names().join(",");
        match self.tail {
            EffectRowTail::Closed => format!("{{{labels}}}"),
            EffectRowTail::Variable(variable) => format!("{{{labels}|ρ{}}}", variable.get()),
        }
    }

    /// Returns length-delimited canonical bytes suitable for a graph key.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"effect-row-v2");
        for label in &self.labels {
            push_field(&mut bytes, label.canonical_name().as_bytes());
        }
        match self.tail {
            EffectRowTail::Closed => bytes.push(0),
            EffectRowTail::Variable(variable) => {
                bytes.push(1);
                bytes.extend_from_slice(&variable.get().to_be_bytes());
            }
        }
        bytes
    }
}

/// Error raised when row union would have to choose between two variables.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectRowUnionError {
    DistinctTails {
        left: RowVariableId,
        right: RowVariableId,
    },
}

impl fmt::Display for EffectRowUnionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DistinctTails { left, right } => write!(
                formatter,
                "Effect row union requires a constraint for distinct tails ρ{} and ρ{}",
                left.get(),
                right.get()
            ),
        }
    }
}

impl Error for EffectRowUnionError {}

/// Operation continuation cardinality declared by a handler clause.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResumeMode {
    Never,
    Once,
    Many,
}

/// A first-order operation signature owned by one Effect identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectOperation {
    owner: EffectId,
    name: String,
    inputs: Box<[EffectTypeRef]>,
    output: EffectTypeRef,
    resume_mode: ResumeMode,
}

impl EffectOperation {
    pub fn new(
        owner: EffectId,
        name: &str,
        inputs: impl IntoIterator<Item = EffectTypeRef>,
        output: EffectTypeRef,
        resume_mode: ResumeMode,
    ) -> Result<Self, EffectOperationError> {
        if name.is_empty() {
            return Err(EffectOperationError::EmptyName);
        }
        let identifier =
            validate_identifier(name).map_err(|error| EffectOperationError::InvalidName {
                name: name.to_owned(),
                reason: error.to_string(),
            })?;
        Ok(Self {
            owner,
            name: identifier.normalized().to_owned(),
            inputs: inputs.into_iter().collect(),
            output,
            resume_mode,
        })
    }

    #[must_use]
    pub fn owner(&self) -> &EffectId {
        &self.owner
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn inputs(&self) -> &[EffectTypeRef] {
        &self.inputs
    }

    #[must_use]
    pub fn output(&self) -> &EffectTypeRef {
        &self.output
    }

    #[must_use]
    pub const fn resume_mode(&self) -> ResumeMode {
        self.resume_mode
    }

    #[must_use]
    pub fn canonical_name(&self) -> String {
        let inputs = self
            .inputs
            .iter()
            .map(EffectTypeRef::as_str)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{}::{}({inputs})->{}::{:?}",
            self.owner, self.name, self.output, self.resume_mode
        )
    }
}

impl fmt::Display for EffectOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical_name())
    }
}

/// Errors raised while constructing an [`EffectOperation`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectOperationError {
    EmptyName,
    InvalidName { name: String, reason: String },
}

impl fmt::Display for EffectOperationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("Effect operation name is empty"),
            Self::InvalidName { name, reason } => {
                write!(
                    formatter,
                    "Effect operation name `{name}` is invalid: {reason}"
                )
            }
        }
    }
}

impl Error for EffectOperationError {}

/// One lexical handler clause, tied to an explicitly handled Effect label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HandlerClause {
    label: EffectLabel,
    operation: EffectOperation,
}

impl HandlerClause {
    pub fn new(label: EffectLabel, operation: EffectOperation) -> Result<Self, HandlerClauseError> {
        if label.effect_id() != operation.owner() {
            return Err(HandlerClauseError::OwnerMismatch {
                label: label.effect_id().clone(),
                operation: operation.owner().clone(),
            });
        }
        Ok(Self { label, operation })
    }

    #[must_use]
    pub fn label(&self) -> &EffectLabel {
        &self.label
    }

    #[must_use]
    pub fn operation(&self) -> &EffectOperation {
        &self.operation
    }
}

/// Errors raised while tying an operation to a handler label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HandlerClauseError {
    OwnerMismatch {
        label: EffectId,
        operation: EffectId,
    },
}

impl fmt::Display for HandlerClauseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OwnerMismatch { label, operation } => write!(
                formatter,
                "handler label `{label}` does not match operation owner `{operation}`"
            ),
        }
    }
}

impl Error for HandlerClauseError {}

/// A lexical first-order handler contract and its declared residual row.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HandlerContract {
    clauses: Box<[HandlerClause]>,
    residual: EffectRowModel,
}

impl HandlerContract {
    /// Creates a canonical contract and rejects duplicate handled labels.
    pub fn new(
        clauses: impl IntoIterator<Item = HandlerClause>,
        residual: EffectRowModel,
    ) -> Result<Self, HandlerContractError> {
        let mut clauses = clauses.into_iter().collect::<Vec<_>>();
        clauses.sort();
        if let Some(window) = clauses
            .windows(2)
            .find(|pair| pair[0].label == pair[1].label)
        {
            return Err(HandlerContractError::DuplicateLabel(
                window[0].label.clone(),
            ));
        }
        Ok(Self {
            clauses: clauses.into_boxed_slice(),
            residual,
        })
    }

    /// Builds a contract whose residual is computed from an input row.
    pub fn for_input(
        input: &EffectRowModel,
        clauses: impl IntoIterator<Item = HandlerClause>,
    ) -> Result<Self, HandlerContractError> {
        let clauses = clauses.into_iter().collect::<Vec<_>>();
        let residual = clauses.iter().fold(input.clone(), |row, clause| {
            row.without_label(clause.label())
        });
        Self::new(clauses, residual)
    }

    #[must_use]
    pub fn clauses(&self) -> &[HandlerClause] {
        &self.clauses
    }

    #[must_use]
    pub fn residual(&self) -> &EffectRowModel {
        &self.residual
    }

    /// Applies the lexical handler and checks its declared residual row.
    pub fn apply(&self, input: &EffectRowModel) -> Result<EffectRowModel, HandlerContractError> {
        let actual = self.clauses.iter().fold(input.clone(), |row, clause| {
            row.without_label(clause.label())
        });
        if actual != self.residual {
            return Err(HandlerContractError::ResidualMismatch {
                declared: self.residual.clone(),
                actual,
            });
        }
        Ok(self.residual.clone())
    }
}

/// Versioned in-process projection shape for a future Semantic Graph
/// extension. This is not a wire protocol and does not alter Seed JSON.
pub const EFFECT_GRAPH_EXTENSION_VERSION: &str = "ling.effect/0.1";

/// Canonical Effect model values consumed by a later graph adapter.
///
/// The projection owns no source paths, spans, host capabilities, allocation
/// identities, or runtime state. Its canonical bytes are suitable as a stable
/// graph-input boundary once a separate public schema authority is accepted.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectGraphProjection {
    rows: Box<[EffectRowModel]>,
    operations: Box<[EffectOperation]>,
    handlers: Box<[HandlerContract]>,
}

impl EffectGraphProjection {
    /// Builds a sorted, duplicate-free in-process projection.
    #[must_use]
    pub fn new(
        rows: impl IntoIterator<Item = EffectRowModel>,
        operations: impl IntoIterator<Item = EffectOperation>,
        handlers: impl IntoIterator<Item = HandlerContract>,
    ) -> Self {
        let mut rows = rows.into_iter().collect::<Vec<_>>();
        rows.sort();
        rows.dedup();
        let mut operations = operations.into_iter().collect::<Vec<_>>();
        operations.sort();
        operations.dedup();
        let mut handlers = handlers.into_iter().collect::<Vec<_>>();
        handlers.sort();
        handlers.dedup();
        Self {
            rows: rows.into_boxed_slice(),
            operations: operations.into_boxed_slice(),
            handlers: handlers.into_boxed_slice(),
        }
    }

    #[must_use]
    pub fn schema(&self) -> &'static str {
        EFFECT_GRAPH_EXTENSION_VERSION
    }

    #[must_use]
    pub fn rows(&self) -> &[EffectRowModel] {
        &self.rows
    }

    #[must_use]
    pub fn operations(&self) -> &[EffectOperation] {
        &self.operations
    }

    #[must_use]
    pub fn handlers(&self) -> &[HandlerContract] {
        &self.handlers
    }

    /// Serializes the model into deterministic length-delimited graph bytes.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, EFFECT_GRAPH_EXTENSION_VERSION.as_bytes());
        for row in &self.rows {
            push_field(&mut bytes, &row.canonical_bytes());
        }
        for operation in &self.operations {
            push_field(&mut bytes, operation.canonical_name().as_bytes());
        }
        for handler in &self.handlers {
            for clause in handler.clauses() {
                push_field(&mut bytes, clause.label().canonical_name().as_bytes());
                push_field(&mut bytes, clause.operation().canonical_name().as_bytes());
            }
            push_field(&mut bytes, &handler.residual().canonical_bytes());
        }
        bytes
    }
}

/// Errors raised while creating or applying a handler contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HandlerContractError {
    DuplicateLabel(EffectLabel),
    ResidualMismatch {
        declared: EffectRowModel,
        actual: EffectRowModel,
    },
}

impl fmt::Display for HandlerContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateLabel(label) => write!(formatter, "handler handles `{label}` twice"),
            Self::ResidualMismatch { declared, actual } => write!(
                formatter,
                "handler residual row `{declared}` does not match input result `{actual}`"
            ),
        }
    }
}

impl Error for HandlerContractError {}

fn push_field(output: &mut Vec<u8>, field: &[u8]) {
    let length = u64::try_from(field.len()).unwrap_or(u64::MAX);
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(field);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_ref(value: &str) -> EffectTypeRef {
        EffectTypeRef::new(value).expect("test type reference is canonical")
    }

    fn operation(owner: &EffectId, name: &str) -> EffectOperation {
        EffectOperation::new(
            owner.clone(),
            name,
            [type_ref("Int")],
            type_ref("Unit"),
            ResumeMode::Once,
        )
        .expect("test operation is valid")
    }

    #[test]
    fn effect_id_normalizes_nfc_and_rejects_paths() {
        let composed = EffectId::new("Café.Read").expect("composed id");
        let decomposed = EffectId::new("Cafe\u{301}.Read").expect("decomposed id");
        assert_eq!(composed, decomposed);
        assert_eq!(composed.as_str(), "Café.Read");
        assert!(EffectId::new("Clock/Now").is_err());
        assert!(EffectId::new("Clock..Now").is_err());
    }

    #[test]
    fn rows_are_sorted_duplicate_free_and_tail_safe() {
        let state = EffectLabel::state(type_ref("Text"));
        let clock = EffectLabel::clock();
        let left = EffectRowModel::open(
            [state.clone(), clock.clone(), state.clone()],
            RowVariableId::new(1),
        );
        let right = EffectRowModel::closed([clock.clone()]);
        let union = left.union(&right).expect("closed tail combines safely");
        assert_eq!(union.canonical_names(), ["Clock", "State<Text>"]);
        assert_eq!(union.tail(), EffectRowTail::Variable(RowVariableId::new(1)));
        assert!(!union.is_pure());
        assert!(
            left.union(&EffectRowModel::open([], RowVariableId::new(2)))
                .is_err()
        );
        assert_eq!(EffectRowModel::pure().canonical_name(), "{}");

        let caller_row = EffectRowModel::open([], RowVariableId::new(7));
        let callback_row = EffectRowModel::closed([EffectLabel::clock()]);
        assert_eq!(
            caller_row
                .union(&callback_row)
                .expect("polymorphic caller row is preserved")
                .canonical_name(),
            "{Clock|ρ7}"
        );
    }

    #[test]
    fn labels_and_operations_preserve_parameters_and_resume_mode() {
        let state_int = EffectLabel::state(type_ref("Int"));
        let state_text = EffectLabel::state(type_ref("Text"));
        assert_ne!(state_int, state_text);
        let owner = EffectId::new("Clock").expect("clock id");
        let operation = operation(&owner, "now");
        assert_eq!(operation.resume_mode(), ResumeMode::Once);
        assert_eq!(operation.inputs()[0].as_str(), "Int");
        assert!(
            EffectOperation::new(owner, "now.bad", [], type_ref("Int"), ResumeMode::Never,)
                .is_err()
        );
    }

    #[test]
    fn nested_handlers_remove_only_declared_labels_and_preserve_open_tail() {
        let clock = EffectLabel::clock();
        let random = EffectLabel::random();
        let input = EffectRowModel::open([clock.clone(), random.clone()], RowVariableId::new(3));
        let clock_clause = HandlerClause::new(
            clock.clone(),
            operation(&EffectId::new("Clock").expect("clock id"), "now"),
        )
        .expect("clock clause");
        let random_clause = HandlerClause::new(
            random.clone(),
            operation(&EffectId::new("Random").expect("random id"), "next"),
        )
        .expect("random clause");
        let inner = HandlerContract::for_input(&input, [clock_clause]).expect("inner handler");
        let after_inner = inner.apply(&input).expect("inner residual");
        assert_eq!(after_inner.canonical_name(), "{Random|ρ3}");
        let outer =
            HandlerContract::for_input(&after_inner, [random_clause]).expect("outer handler");
        assert_eq!(
            outer
                .apply(&after_inner)
                .expect("outer residual")
                .canonical_name(),
            "{|ρ3}"
        );
    }

    #[test]
    fn canonical_bytes_ignore_presentation_order() {
        let left = EffectRowModel::closed([
            EffectLabel::random(),
            EffectLabel::clock(),
            EffectLabel::random(),
        ]);
        let right = EffectRowModel::closed([EffectLabel::clock(), EffectLabel::random()]);
        assert_eq!(left.canonical_bytes(), right.canonical_bytes());
    }

    #[test]
    fn reserved_labels_and_graph_projection_are_versioned_and_deterministic() {
        let labels = [
            EffectLabel::clock(),
            EffectLabel::random(),
            EffectLabel::console_write(),
            EffectLabel::state(type_ref("Int")),
            EffectLabel::task(),
            EffectLabel::actor_send(type_ref("Message")),
        ];
        let row = EffectRowModel::closed(labels.clone());
        assert_eq!(
            row.canonical_names(),
            [
                "ActorSend<Message>",
                "Clock",
                "Console.Write",
                "Random",
                "State<Int>",
                "Task",
            ]
        );

        let clock = EffectLabel::clock();
        let clock_operation = operation(&EffectId::new("Clock").expect("clock id"), "now");
        let clause = HandlerClause::new(clock, clock_operation.clone()).expect("clock clause");
        let handler = HandlerContract::for_input(&row, [clause]).expect("handler contract");
        let first = EffectGraphProjection::new(
            [row.clone(), EffectRowModel::pure()],
            [clock_operation.clone()],
            [handler.clone()],
        );
        let second =
            EffectGraphProjection::new([EffectRowModel::pure(), row], [clock_operation], [handler]);
        assert_eq!(first.schema(), EFFECT_GRAPH_EXTENSION_VERSION);
        assert_eq!(first.canonical_bytes(), second.canonical_bytes());
        assert_eq!(first.rows().len(), 2);
        assert_eq!(first.operations().len(), 1);
        assert_eq!(first.handlers().len(), 1);
    }
}
