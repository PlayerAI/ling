//! Deterministic Experimental v0.2 Effect-row constraint solving.
//!
//! The solver consumes only the canonical RFC-0006 row model. It is deliberately
//! separate from the v0.0.1 Seed checker and does not parse source or execute a
//! handler.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};

use crate::{
    EffectLabel, EffectRowModel, EffectRowTail, HandlerContract, HandlerContractError,
    RowVariableId,
};

/// A source span retained as diagnostic evidence without entering row identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectSourceSpan {
    file: Box<str>,
    start_byte: u64,
    end_byte: u64,
}

impl EffectSourceSpan {
    /// Creates a span in the original UTF-8 byte-offset domain.
    #[must_use]
    pub fn new(file: impl Into<String>, start_byte: u64, end_byte: u64) -> Self {
        Self {
            file: file.into().into_boxed_str(),
            start_byte,
            end_byte,
        }
    }

    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    #[must_use]
    pub const fn start_byte(&self) -> u64 {
        self.start_byte
    }

    #[must_use]
    pub const fn end_byte(&self) -> u64 {
        self.end_byte
    }
}

/// Stable provenance for one collected constraint.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectConstraintOrigin {
    ordinal: u32,
    span: Option<EffectSourceSpan>,
}

impl EffectConstraintOrigin {
    /// Creates an origin with no source span.
    #[must_use]
    pub const fn new(ordinal: u32) -> Self {
        Self {
            ordinal,
            span: None,
        }
    }

    /// Attaches the original UTF-8 byte span used by diagnostics only.
    #[must_use]
    pub fn with_span(mut self, span: EffectSourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    #[must_use]
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    #[must_use]
    pub fn span(&self) -> Option<&EffectSourceSpan> {
        self.span.as_ref()
    }
}

/// A bounded equality or required-label constraint.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EffectConstraint {
    /// Requires two rows to denote the same set and tail.
    Equal {
        left: EffectRowModel,
        right: EffectRowModel,
        origin: EffectConstraintOrigin,
    },
    /// Requires a label to be present while preserving an open tail.
    Requires {
        row: EffectRowModel,
        label: EffectLabel,
        origin: EffectConstraintOrigin,
    },
}

impl EffectConstraint {
    #[must_use]
    pub const fn origin(&self) -> &EffectConstraintOrigin {
        match self {
            Self::Equal { origin, .. } | Self::Requires { origin, .. } => origin,
        }
    }

    fn sort_key(
        &self,
    ) -> (
        &EffectConstraintOrigin,
        u8,
        &EffectRowModel,
        Option<&EffectRowModel>,
        Option<&EffectLabel>,
    ) {
        match self {
            Self::Equal {
                left,
                right,
                origin,
            } => (origin, 0, left, Some(right), None),
            Self::Requires { row, label, origin } => (origin, 1, row, None, Some(label)),
        }
    }
}

/// A deterministic substitution from row variables to normalized rows.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectSubstitution {
    bindings: BTreeMap<RowVariableId, EffectRowModel>,
}

impl EffectSubstitution {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    #[must_use]
    pub fn bindings(&self) -> &BTreeMap<RowVariableId, EffectRowModel> {
        &self.bindings
    }

    #[must_use]
    pub fn get(&self, variable: RowVariableId) -> Option<&EffectRowModel> {
        self.bindings.get(&variable)
    }

    /// Applies the substitution to a row until its tail is normalized.
    #[must_use]
    pub fn apply(&self, row: &EffectRowModel) -> EffectRowModel {
        normalize_with_bindings(row, &self.bindings, &mut BTreeSet::new())
    }

    /// Returns canonical bytes for deterministic cache or test comparisons.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"effect-substitution-v1");
        for (variable, row) in &self.bindings {
            bytes.extend_from_slice(&variable.get().to_be_bytes());
            push_field(&mut bytes, &row.canonical_bytes());
        }
        bytes
    }
}

/// Successful output of the row constraint solver.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectInference {
    substitution: EffectSubstitution,
}

impl EffectInference {
    #[must_use]
    pub const fn substitution(&self) -> &EffectSubstitution {
        &self.substitution
    }

    #[must_use]
    pub fn normalize(&self, row: &EffectRowModel) -> EffectRowModel {
        self.substitution.apply(row)
    }
}

/// A deterministic, collected EFF-2102 constraint solver.
#[derive(Clone, Debug, Default)]
pub struct EffectConstraintSolver {
    constraints: Vec<EffectConstraint>,
}

impl EffectConstraintSolver {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    #[must_use]
    pub fn from_constraints(constraints: impl IntoIterator<Item = EffectConstraint>) -> Self {
        Self {
            constraints: constraints.into_iter().collect(),
        }
    }

    pub fn push(&mut self, constraint: EffectConstraint) {
        self.constraints.push(constraint);
    }

    #[must_use]
    pub fn constraints(&self) -> &[EffectConstraint] {
        &self.constraints
    }

    /// Solves constraints in canonical order, independent of insertion order.
    pub fn solve(&self) -> Result<EffectInference, EffectConstraintError> {
        let mut constraints = self.constraints.clone();
        constraints.sort_by(compare_constraints);
        constraints.dedup();

        let mut state = SolverState {
            substitutions: BTreeMap::new(),
            variable_origins: BTreeMap::new(),
            next_fresh: next_fresh_variable(&constraints),
        };

        for constraint in &constraints {
            match constraint {
                EffectConstraint::Equal {
                    left,
                    right,
                    origin,
                } => {
                    state.unify(left, right, origin)?;
                }
                EffectConstraint::Requires { row, label, origin } => {
                    state.require(row, label, origin)?;
                }
            }
        }

        let bindings = state.substitutions.clone();
        let substitutions = bindings
            .into_iter()
            .map(|(variable, row)| {
                let normalized =
                    normalize_with_bindings(&row, &state.substitutions, &mut BTreeSet::new());
                (variable, normalized)
            })
            .collect();
        Ok(EffectInference {
            substitution: EffectSubstitution {
                bindings: substitutions,
            },
        })
    }
}

/// The reason a row constraint could not be satisfied.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EffectConflictKind {
    RowConstraint,
    OccursCheck,
}

/// Stable conflict evidence and its bilingual diagnostic.
#[derive(Clone, Debug)]
pub struct EffectConstraintConflict {
    kind: EffectConflictKind,
    origins: Box<[EffectConstraintOrigin]>,
    diagnostic: Diagnostic,
}

impl EffectConstraintConflict {
    #[must_use]
    pub const fn kind(&self) -> EffectConflictKind {
        self.kind
    }

    #[must_use]
    pub fn origins(&self) -> &[EffectConstraintOrigin] {
        &self.origins
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostic
    }
}

/// Errors produced by EFF-2102 solving.
#[derive(Clone, Debug)]
pub enum EffectConstraintError {
    Conflict(Box<EffectConstraintConflict>),
    FreshVariableExhausted { origin: EffectConstraintOrigin },
}

impl EffectConstraintError {
    #[must_use]
    pub const fn conflict(&self) -> Option<&EffectConstraintConflict> {
        match self {
            Self::Conflict(conflict) => Some(conflict),
            Self::FreshVariableExhausted { .. } => None,
        }
    }
}

impl fmt::Display for EffectConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conflict(conflict) => write!(
                formatter,
                "{}",
                conflict
                    .diagnostic()
                    .render_human(ling_diagnostics::MessageLanguage::English)
            ),
            Self::FreshVariableExhausted { origin } => write!(
                formatter,
                "cannot allocate a fresh row variable for constraint {}",
                origin.ordinal()
            ),
        }
    }
}

impl Error for EffectConstraintError {}

/// Explicit boundary supplied by the checked value restriction.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeneralizationBoundary {
    is_value: bool,
    allows_effects: bool,
}

impl GeneralizationBoundary {
    #[must_use]
    pub const fn value(allows_effects: bool) -> Self {
        Self {
            is_value: true,
            allows_effects,
        }
    }

    #[must_use]
    pub const fn monomorphic() -> Self {
        Self {
            is_value: false,
            allows_effects: false,
        }
    }

    #[must_use]
    pub const fn permits(self) -> bool {
        self.is_value && self.allows_effects
    }
}

/// A row scheme with sorted binder-local quantified variables.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectRowScheme {
    body: EffectRowModel,
    quantified: Box<[RowVariableId]>,
}

impl EffectRowScheme {
    /// Generalizes only when the explicit value restriction permits it.
    #[must_use]
    pub fn generalize(
        row: &EffectRowModel,
        environment: &[EffectRowModel],
        boundary: GeneralizationBoundary,
    ) -> Self {
        let mut quantified = Vec::new();
        if boundary.permits() {
            if let EffectRowTail::Variable(variable) = row.tail() {
                let environment_contains = environment
                    .iter()
                    .any(|candidate| candidate.tail() == EffectRowTail::Variable(variable));
                if !environment_contains {
                    quantified.push(variable);
                }
            }
        }
        Self {
            body: row.clone(),
            quantified: quantified.into_boxed_slice(),
        }
    }

    #[must_use]
    pub const fn body(&self) -> &EffectRowModel {
        &self.body
    }

    #[must_use]
    pub fn quantified(&self) -> &[RowVariableId] {
        &self.quantified
    }

    /// Instantiates with a caller-provided deterministic fresh-variable seed.
    pub fn instantiate(&self, seed: u32) -> Result<EffectRowModel, EffectInstantiationError> {
        let Some(EffectRowTail::Variable(variable)) = Some(self.body.tail()) else {
            return Ok(self.body.clone());
        };
        let Some(index) = self
            .quantified
            .iter()
            .position(|candidate| *candidate == variable)
        else {
            return Ok(self.body.clone());
        };
        let index = u32::try_from(index).map_err(|_| EffectInstantiationError::Exhausted)?;
        let fresh = seed
            .checked_add(index)
            .map(RowVariableId::new)
            .ok_or(EffectInstantiationError::Exhausted)?;
        Ok(EffectRowModel::new(
            self.body.labels().iter().cloned(),
            EffectRowTail::Variable(fresh),
        ))
    }
}

/// Failure to produce a deterministic fresh binder-local variable.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EffectInstantiationError {
    Exhausted,
}

impl fmt::Display for EffectInstantiationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exhausted => {
                formatter.write_str("effect row instantiation exhausted row-variable IDs")
            }
        }
    }
}

impl Error for EffectInstantiationError {}

/// Applies a lexical handler contract to a row while preserving its tail.
pub fn subtract_handler(
    input: &EffectRowModel,
    handler: &HandlerContract,
) -> Result<EffectRowModel, HandlerContractError> {
    handler.apply(input)
}

struct SolverState {
    substitutions: BTreeMap<RowVariableId, EffectRowModel>,
    variable_origins: BTreeMap<RowVariableId, BTreeSet<EffectConstraintOrigin>>,
    next_fresh: Option<u32>,
}

impl SolverState {
    fn unify(
        &mut self,
        left: &EffectRowModel,
        right: &EffectRowModel,
        origin: &EffectConstraintOrigin,
    ) -> Result<(), EffectConstraintError> {
        let left = self.normalize(left, origin)?;
        let right = self.normalize(right, origin)?;
        if left == right {
            return Ok(());
        }

        let left_only = labels_only(left.labels(), right.labels());
        let right_only = labels_only(right.labels(), left.labels());
        match (left.tail(), right.tail()) {
            (EffectRowTail::Closed, EffectRowTail::Closed) => {
                Err(self.row_conflict(origin, &left, &right, None, []))
            }
            (EffectRowTail::Variable(variable), EffectRowTail::Closed) => {
                if !left_only.is_empty() {
                    return Err(self.row_conflict(origin, &left, &right, None, [variable]));
                }
                self.bind(variable, EffectRowModel::closed(right_only), origin)
            }
            (EffectRowTail::Closed, EffectRowTail::Variable(variable)) => {
                if !right_only.is_empty() {
                    return Err(self.row_conflict(origin, &left, &right, None, [variable]));
                }
                self.bind(variable, EffectRowModel::closed(left_only), origin)
            }
            (EffectRowTail::Variable(left_variable), EffectRowTail::Variable(right_variable))
                if left_variable == right_variable =>
            {
                if left_only.is_empty() && right_only.is_empty() {
                    Ok(())
                } else {
                    Err(self.row_conflict(origin, &left, &right, None, [left_variable]))
                }
            }
            (EffectRowTail::Variable(left_variable), EffectRowTail::Variable(right_variable)) => {
                let fresh = self.fresh(origin)?;
                let left_binding = EffectRowModel::open(right_only, fresh);
                let right_binding = EffectRowModel::open(left_only, fresh);
                if left_variable > right_variable {
                    self.bind(left_variable, left_binding, origin)?;
                    self.bind(right_variable, right_binding, origin)
                } else {
                    self.bind(right_variable, right_binding, origin)?;
                    self.bind(left_variable, left_binding, origin)
                }
            }
        }
    }

    fn require(
        &mut self,
        row: &EffectRowModel,
        label: &EffectLabel,
        origin: &EffectConstraintOrigin,
    ) -> Result<(), EffectConstraintError> {
        let row = self.normalize(row, origin)?;
        if row.contains(label) {
            return Ok(());
        }
        let EffectRowTail::Variable(variable) = row.tail() else {
            return Err(self.row_conflict(
                origin,
                &row,
                &EffectRowModel::closed([]),
                Some(label),
                [],
            ));
        };
        let fresh = self.fresh(origin)?;
        self.bind(
            variable,
            EffectRowModel::open([label.clone()], fresh),
            origin,
        )
    }

    fn bind(
        &mut self,
        variable: RowVariableId,
        term: EffectRowModel,
        origin: &EffectConstraintOrigin,
    ) -> Result<(), EffectConstraintError> {
        let term = self.normalize(&term, origin)?;
        if term.labels().is_empty() && term.tail() == EffectRowTail::Variable(variable) {
            return Ok(());
        }
        if self.occurs(variable, &term, &mut BTreeSet::new()) {
            return Err(self.occurs_conflict(origin, variable, &term));
        }
        if let Some(existing) = self.substitutions.remove(&variable) {
            self.unify(&existing, &term, origin)?;
        } else {
            self.substitutions.insert(variable, term.clone());
            self.variable_origins
                .entry(variable)
                .or_default()
                .insert(origin.clone());
        }
        Ok(())
    }

    fn normalize(
        &self,
        row: &EffectRowModel,
        origin: &EffectConstraintOrigin,
    ) -> Result<EffectRowModel, EffectConstraintError> {
        let mut visiting = BTreeSet::new();
        let normalized = normalize_with_bindings_checked(row, &self.substitutions, &mut visiting);
        if normalized.is_none() {
            return Err(self.occurs_conflict(
                origin,
                row.tail_variable().unwrap_or(RowVariableId::new(0)),
                row,
            ));
        }
        Ok(normalized.expect("checked normalization is present"))
    }

    fn occurs(
        &self,
        variable: RowVariableId,
        row: &EffectRowModel,
        visiting: &mut BTreeSet<RowVariableId>,
    ) -> bool {
        let EffectRowTail::Variable(tail) = row.tail() else {
            return false;
        };
        if tail == variable {
            return true;
        }
        if !visiting.insert(tail) {
            return false;
        }
        let result = self
            .substitutions
            .get(&tail)
            .is_some_and(|bound| self.occurs(variable, bound, visiting));
        visiting.remove(&tail);
        result
    }

    fn fresh(
        &mut self,
        origin: &EffectConstraintOrigin,
    ) -> Result<RowVariableId, EffectConstraintError> {
        let Some(value) = self.next_fresh else {
            return Err(EffectConstraintError::FreshVariableExhausted {
                origin: origin.clone(),
            });
        };
        self.next_fresh = value.checked_add(1);
        Ok(RowVariableId::new(value))
    }

    fn origins_for(
        &self,
        current: &EffectConstraintOrigin,
        variables: impl IntoIterator<Item = RowVariableId>,
    ) -> Box<[EffectConstraintOrigin]> {
        let mut origins = BTreeSet::from([current.clone()]);
        for variable in variables {
            if let Some(variable_origins) = self.variable_origins.get(&variable) {
                origins.extend(variable_origins.iter().cloned());
            }
        }
        origins.into_iter().collect()
    }

    fn row_conflict(
        &self,
        current: &EffectConstraintOrigin,
        left: &EffectRowModel,
        right: &EffectRowModel,
        required: Option<&EffectLabel>,
        variables: impl IntoIterator<Item = RowVariableId>,
    ) -> EffectConstraintError {
        let origins = self.origins_for(current, variables);
        EffectConstraintError::Conflict(Box::new(EffectConstraintConflict {
            kind: EffectConflictKind::RowConstraint,
            diagnostic: row_diagnostic(&origins, left, right, required),
            origins,
        }))
    }

    fn occurs_conflict(
        &self,
        current: &EffectConstraintOrigin,
        variable: RowVariableId,
        row: &EffectRowModel,
    ) -> EffectConstraintError {
        let origins = self.origins_for(current, [variable]);
        EffectConstraintError::Conflict(Box::new(EffectConstraintConflict {
            kind: EffectConflictKind::OccursCheck,
            diagnostic: occurs_diagnostic(&origins, variable, row),
            origins,
        }))
    }
}

fn compare_constraints(left: &EffectConstraint, right: &EffectConstraint) -> Ordering {
    let left_key = left.sort_key();
    let right_key = right.sort_key();
    left_key
        .0
        .cmp(right_key.0)
        .then_with(|| left_key.1.cmp(&right_key.1))
        .then_with(|| left_key.2.cmp(right_key.2))
        .then_with(|| left_key.3.cmp(&right_key.3))
        .then_with(|| left_key.4.cmp(&right_key.4))
}

fn labels_only(left: &[EffectLabel], right: &[EffectLabel]) -> Vec<EffectLabel> {
    left.iter()
        .filter(|label| right.binary_search(label).is_err())
        .cloned()
        .collect()
}

fn next_fresh_variable(constraints: &[EffectConstraint]) -> Option<u32> {
    let max = constraints
        .iter()
        .flat_map(|constraint| match constraint {
            EffectConstraint::Equal { left, right, .. } => [left.tail(), right.tail()],
            EffectConstraint::Requires { row, .. } => [row.tail(), EffectRowTail::Closed],
        })
        .filter_map(|tail| match tail {
            EffectRowTail::Closed => None,
            EffectRowTail::Variable(variable) => Some(variable.get()),
        })
        .max();
    max.map_or(Some(0), |value| value.checked_add(1))
}

fn normalize_with_bindings(
    row: &EffectRowModel,
    bindings: &BTreeMap<RowVariableId, EffectRowModel>,
    visiting: &mut BTreeSet<RowVariableId>,
) -> EffectRowModel {
    normalize_with_bindings_checked(row, bindings, visiting).unwrap_or_else(|| row.clone())
}

fn normalize_with_bindings_checked(
    row: &EffectRowModel,
    bindings: &BTreeMap<RowVariableId, EffectRowModel>,
    visiting: &mut BTreeSet<RowVariableId>,
) -> Option<EffectRowModel> {
    let EffectRowTail::Variable(variable) = row.tail() else {
        return Some(row.clone());
    };
    let Some(bound) = bindings.get(&variable) else {
        return Some(row.clone());
    };
    if !visiting.insert(variable) {
        return None;
    }
    let normalized = normalize_with_bindings_checked(bound, bindings, visiting)?;
    visiting.remove(&variable);
    Some(EffectRowModel::new(
        row.labels()
            .iter()
            .cloned()
            .chain(normalized.labels().iter().cloned()),
        normalized.tail(),
    ))
}

fn row_diagnostic(
    origins: &[EffectConstraintOrigin],
    left: &EffectRowModel,
    right: &EffectRowModel,
    required: Option<&EffectLabel>,
) -> Diagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::EFFECT_ROW_CONFLICT,
        Severity::Error,
        "Effect Row 约束冲突",
        "effect row constraint conflict",
    )
    .with_fact("conflict_set", origin_facts(origins))
    .with_fact("left_row", left.canonical_name())
    .with_fact("right_row", right.canonical_name());
    if let Some(label) = required {
        diagnostic = diagnostic.with_fact("required_label", label.canonical_name());
    }
    with_primary_span(diagnostic, origins)
}

fn occurs_diagnostic(
    origins: &[EffectConstraintOrigin],
    variable: RowVariableId,
    row: &EffectRowModel,
) -> Diagnostic {
    with_primary_span(
        Diagnostic::new(
            codes::EFFECT_ROW_OCCURS_CHECK,
            Severity::Error,
            "Effect Row 推导产生无限 Row",
            "effect row inference produced an infinite row",
        )
        .with_fact("conflict_set", origin_facts(origins))
        .with_fact("row", row.canonical_name())
        .with_fact("variable", format!("ρ{}", variable.get())),
        origins,
    )
}

fn origin_facts(origins: &[EffectConstraintOrigin]) -> Vec<String> {
    origins
        .iter()
        .map(|origin| origin.ordinal().to_string())
        .collect()
}

fn with_primary_span(mut diagnostic: Diagnostic, origins: &[EffectConstraintOrigin]) -> Diagnostic {
    if let Some(span) = origins.iter().find_map(EffectConstraintOrigin::span) {
        diagnostic = diagnostic.with_primary_span(DiagnosticSpan::at_u64(
            span.file().to_owned(),
            span.start_byte(),
            span.end_byte(),
        ));
    }
    diagnostic
}

fn push_field(output: &mut Vec<u8>, field: &[u8]) {
    let length = u64::try_from(field.len()).unwrap_or(u64::MAX);
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(field);
}

trait EffectRowTailExt {
    fn tail_variable(&self) -> Option<RowVariableId>;
}

impl EffectRowTailExt for EffectRowModel {
    fn tail_variable(&self) -> Option<RowVariableId> {
        match self.tail() {
            EffectRowTail::Closed => None,
            EffectRowTail::Variable(variable) => Some(variable),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EffectTypeRef;

    fn label(name: &str) -> EffectLabel {
        EffectLabel::new(crate::EffectId::new(name).expect("valid test EffectId"), [])
    }

    fn origin(ordinal: u32) -> EffectConstraintOrigin {
        EffectConstraintOrigin::new(ordinal)
    }

    #[test]
    fn equal_constraints_are_order_independent_and_bind_closed_residuals() {
        let variable = RowVariableId::new(7);
        let left = EffectRowModel::open([label("Clock")], variable);
        let right = EffectRowModel::closed([label("Clock"), label("Random")]);
        let forward = EffectConstraintSolver::from_constraints([EffectConstraint::Equal {
            left: left.clone(),
            right: right.clone(),
            origin: origin(1),
        }])
        .solve()
        .expect("rows unify");
        let reverse = EffectConstraintSolver::from_constraints([EffectConstraint::Equal {
            left: right,
            right: left,
            origin: origin(1),
        }])
        .solve()
        .expect("reversed rows unify");
        assert_eq!(
            forward.substitution().canonical_bytes(),
            reverse.substitution().canonical_bytes()
        );
        assert_eq!(
            forward
                .normalize(&EffectRowModel::open([], variable))
                .canonical_name(),
            "{Random}"
        );
    }

    #[test]
    fn distinct_open_tails_get_a_deterministic_shared_residual() {
        let left_variable = RowVariableId::new(2);
        let right_variable = RowVariableId::new(5);
        let solver = EffectConstraintSolver::from_constraints([EffectConstraint::Equal {
            left: EffectRowModel::open([label("Clock")], left_variable),
            right: EffectRowModel::open([label("Random")], right_variable),
            origin: origin(4),
        }]);
        let result = solver.solve().expect("distinct tails unify");
        assert_eq!(
            result
                .substitution()
                .get(right_variable)
                .unwrap()
                .canonical_name(),
            "{Clock|ρ6}"
        );
        assert_eq!(
            result
                .substitution()
                .get(left_variable)
                .unwrap()
                .canonical_name(),
            "{Random|ρ6}"
        );
    }

    #[test]
    fn requires_constraint_preserves_unknown_tail() {
        let variable = RowVariableId::new(3);
        let result = EffectConstraintSolver::from_constraints([EffectConstraint::Requires {
            row: EffectRowModel::open([], variable),
            label: label("Clock"),
            origin: origin(9),
        }])
        .solve()
        .expect("required label is inserted");
        assert_eq!(
            result
                .normalize(&EffectRowModel::open([], variable))
                .canonical_name(),
            "{Clock|ρ4}"
        );
    }

    #[test]
    fn occurs_check_and_row_conflicts_have_stable_bilingual_diagnostics() {
        let variable = RowVariableId::new(1);
        let occurs = EffectConstraintSolver::from_constraints([EffectConstraint::Equal {
            left: EffectRowModel::open([], variable),
            right: EffectRowModel::open([label("Clock")], variable),
            origin: origin(2).with_span(EffectSourceSpan::new("main.ling", 3, 8)),
        }])
        .solve()
        .expect_err("same tail with extra label is a conflict");
        let diagnostic = occurs.conflict().expect("conflict evidence").diagnostic();
        assert_eq!(diagnostic.code().as_str(), "L-EFFECT-0001");
        assert!(
            diagnostic
                .render_human(ling_diagnostics::MessageLanguage::Chinese)
                .contains("约束冲突")
        );
        let json = diagnostic.render_json().expect("diagnostic JSON");
        assert!(json.contains("L-EFFECT-0001"));
        assert!(json.contains("conflict_set"));
        assert!(json.contains("main.ling"));

        let mut state = SolverState {
            substitutions: BTreeMap::new(),
            variable_origins: BTreeMap::new(),
            next_fresh: Some(3),
        };
        let cycle = state
            .bind(
                variable,
                EffectRowModel::open([label("Random")], variable),
                &origin(3),
            )
            .expect_err("a substitution cycle is rejected");
        let diagnostic = cycle
            .conflict()
            .expect("occurs conflict evidence")
            .diagnostic();
        assert_eq!(diagnostic.code().as_str(), "L-EFFECT-0002");
        assert!(
            diagnostic
                .render_human(ling_diagnostics::MessageLanguage::English)
                .contains("infinite row")
        );
        assert!(
            diagnostic
                .render_json()
                .expect("occurs diagnostic JSON")
                .contains("L-EFFECT-0002")
        );
    }

    #[test]
    fn generalization_respects_value_restriction_and_instantiation_seed() {
        let variable = RowVariableId::new(4);
        let row = EffectRowModel::open([label("Clock")], variable);
        let generalized =
            EffectRowScheme::generalize(&row, &[], GeneralizationBoundary::value(true));
        assert_eq!(generalized.quantified(), [variable]);
        assert_eq!(
            generalized.instantiate(20).unwrap().canonical_name(),
            "{Clock|ρ20}"
        );
        let monomorphic =
            EffectRowScheme::generalize(&row, &[], GeneralizationBoundary::monomorphic());
        assert!(monomorphic.quantified().is_empty());
        assert_eq!(monomorphic.instantiate(20).unwrap(), row);
    }

    #[test]
    fn source_spans_are_evidence_only() {
        let with_span =
            EffectConstraintOrigin::new(1).with_span(EffectSourceSpan::new("a.ling", 0, 2));
        let without_span =
            EffectConstraintOrigin::new(1).with_span(EffectSourceSpan::new("b.ling", 4, 6));
        let row = EffectRowModel::closed([EffectLabel::new(
            crate::EffectId::new("State").unwrap(),
            [EffectTypeRef::new("Int").unwrap()],
        )]);
        let left = EffectConstraintSolver::from_constraints([EffectConstraint::Equal {
            left: row.clone(),
            right: row.clone(),
            origin: with_span,
        }])
        .solve()
        .unwrap();
        let right = EffectConstraintSolver::from_constraints([EffectConstraint::Equal {
            left: row.clone(),
            right: row,
            origin: without_span,
        }])
        .solve()
        .unwrap();
        assert_eq!(
            left.substitution().canonical_bytes(),
            right.substitution().canonical_bytes()
        );
    }
}
