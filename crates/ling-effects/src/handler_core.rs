//! Checked first-order handler Core projection for the EFF-2103 child slice.
//!
//! This module stores validated handler data only. It does not parse source,
//! interpret continuations, or create a runtime/bytecode protocol.

use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};

use crate::{
    EffectLabel, EffectRowModel, EffectSourceSpan, EffectTypeRef, HandlerClause, HandlerContract,
    HandlerContractError, ResumeMode,
};

/// Opaque checked-Core body identity. Zero is reserved for unresolved data.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HandlerCoreNodeId(u32);

impl HandlerCoreNodeId {
    /// Creates a body identity. `0` is rejected by [`HandlerCore::new`].
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// Declared resume use in one checked handler clause.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResumeUse {
    Never,
    Once,
    Many,
}

impl ResumeUse {
    const fn rank(self) -> u8 {
        match self {
            Self::Never => 0,
            Self::Once => 1,
            Self::Many => 2,
        }
    }

    const fn permitted_by(self, mode: ResumeMode) -> bool {
        self.rank()
            <= match mode {
                ResumeMode::Never => 0,
                ResumeMode::Once => 1,
                ResumeMode::Many => 2,
            }
    }
}

/// One canonical checked handler clause.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HandlerCoreClause {
    clause: HandlerClause,
    body: HandlerCoreNodeId,
    resume_use: ResumeUse,
}

impl HandlerCoreClause {
    /// Creates a clause projection; body identity validation happens at the
    /// enclosing Core boundary so all clauses share one error contract.
    #[must_use]
    pub const fn new(
        clause: HandlerClause,
        body: HandlerCoreNodeId,
        resume_use: ResumeUse,
    ) -> Self {
        Self {
            clause,
            body,
            resume_use,
        }
    }

    #[must_use]
    pub const fn clause(&self) -> &HandlerClause {
        &self.clause
    }

    #[must_use]
    pub const fn body(&self) -> HandlerCoreNodeId {
        self.body
    }

    #[must_use]
    pub const fn resume_use(&self) -> ResumeUse {
        self.resume_use
    }
}

/// A checked, lexical, first-order handler Core value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HandlerCore {
    input: EffectRowModel,
    contract: HandlerContract,
    clauses: Box<[HandlerCoreClause]>,
    body: HandlerCoreNodeId,
    return_type: EffectTypeRef,
    source_span: Option<EffectSourceSpan>,
}

impl HandlerCore {
    /// Builds a checked Core value and computes its residual row from the input.
    pub fn new(
        input: EffectRowModel,
        body: HandlerCoreNodeId,
        return_type: EffectTypeRef,
        clauses: impl IntoIterator<Item = HandlerCoreClause>,
        source_span: Option<EffectSourceSpan>,
    ) -> Result<Self, HandlerCoreError> {
        if !body.is_valid() {
            return Err(HandlerCoreError::UnresolvedBody { body });
        }
        let mut clauses = clauses.into_iter().collect::<Vec<_>>();
        for clause in &clauses {
            if !clause.body.is_valid() {
                return Err(HandlerCoreError::UnresolvedBody { body: clause.body });
            }
            if !clause
                .resume_use
                .permitted_by(clause.clause.operation().resume_mode())
            {
                return Err(HandlerCoreError::ResumeUseExceedsMode {
                    label: clause.clause.label().clone(),
                    mode: clause.clause.operation().resume_mode(),
                    usage: clause.resume_use,
                });
            }
        }
        clauses.sort();
        let contract =
            HandlerContract::for_input(&input, clauses.iter().map(|clause| clause.clause.clone()))
                .map_err(HandlerCoreError::Contract)?;
        Ok(Self {
            input,
            contract,
            clauses: clauses.into_boxed_slice(),
            body,
            return_type,
            source_span,
        })
    }

    #[must_use]
    pub const fn input(&self) -> &EffectRowModel {
        &self.input
    }

    #[must_use]
    pub const fn contract(&self) -> &HandlerContract {
        &self.contract
    }

    #[must_use]
    pub fn residual(&self) -> &EffectRowModel {
        self.contract.residual()
    }

    #[must_use]
    pub fn clauses(&self) -> &[HandlerCoreClause] {
        &self.clauses
    }

    #[must_use]
    pub const fn body(&self) -> HandlerCoreNodeId {
        self.body
    }

    #[must_use]
    pub const fn return_type(&self) -> &EffectTypeRef {
        &self.return_type
    }

    #[must_use]
    pub fn source_span(&self) -> Option<&EffectSourceSpan> {
        self.source_span.as_ref()
    }

    /// Rejects a residual row at an explicitly requested closed boundary.
    pub fn require_closed(&self) -> Result<(), HandlerCoreError> {
        if self.residual().is_pure() {
            return Ok(());
        }
        Err(HandlerCoreError::UnhandledResidual {
            residual: Box::new(self.residual().clone()),
            diagnostic: Box::new(unhandled_residual_diagnostic(
                self.residual(),
                self.source_span.as_ref(),
            )),
        })
    }

    /// Returns path-free, deterministic Core bytes. Source evidence is omitted.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_field(&mut bytes, b"handler-core-v1");
        push_field(&mut bytes, &self.input.canonical_bytes());
        push_field(&mut bytes, &self.residual().canonical_bytes());
        bytes.extend_from_slice(&self.body.get().to_be_bytes());
        push_field(&mut bytes, self.return_type.as_str().as_bytes());
        for clause in &self.clauses {
            push_field(
                &mut bytes,
                clause.clause.label().canonical_name().as_bytes(),
            );
            push_field(
                &mut bytes,
                clause.clause.operation().canonical_name().as_bytes(),
            );
            bytes.extend_from_slice(&clause.body.get().to_be_bytes());
            bytes.push(clause.resume_use.rank());
        }
        bytes
    }
}

/// Errors raised while constructing or closing a handler Core value.
#[derive(Clone, Debug)]
pub enum HandlerCoreError {
    UnresolvedBody {
        body: HandlerCoreNodeId,
    },
    ResumeUseExceedsMode {
        label: EffectLabel,
        mode: ResumeMode,
        usage: ResumeUse,
    },
    Contract(HandlerContractError),
    UnhandledResidual {
        residual: Box<EffectRowModel>,
        diagnostic: Box<Diagnostic>,
    },
}

impl HandlerCoreError {
    #[must_use]
    pub const fn diagnostic(&self) -> Option<&Diagnostic> {
        match self {
            Self::UnhandledResidual { diagnostic, .. } => Some(diagnostic),
            Self::UnresolvedBody { .. } | Self::ResumeUseExceedsMode { .. } | Self::Contract(_) => {
                None
            }
        }
    }
}

impl fmt::Display for HandlerCoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnresolvedBody { body } => {
                write!(formatter, "handler Core body {} is unresolved", body.get())
            }
            Self::ResumeUseExceedsMode { label, mode, usage } => write!(
                formatter,
                "handler clause `{label}` declares {usage:?} resume use beyond {mode:?}"
            ),
            Self::Contract(error) => error.fmt(formatter),
            Self::UnhandledResidual { residual, .. } => {
                write!(formatter, "handler leaves residual row `{residual}`")
            }
        }
    }
}

impl Error for HandlerCoreError {}

fn unhandled_residual_diagnostic(
    residual: &EffectRowModel,
    source_span: Option<&EffectSourceSpan>,
) -> Diagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::EFFECT_UNHANDLED_RESIDUAL,
        Severity::Error,
        format!("Handler 作用域仍有未处理 Effect：{residual}"),
        format!("handler scope leaves an unhandled residual Effect: `{residual}`"),
    )
    .with_fact("boundary", "closed")
    .with_fact("residual_row", residual.canonical_name());
    if let Some(span) = source_span {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EffectOperation;

    fn type_ref(value: &str) -> EffectTypeRef {
        EffectTypeRef::new(value).expect("canonical type reference")
    }

    fn operation(owner: &crate::EffectId, name: &str, mode: ResumeMode) -> EffectOperation {
        EffectOperation::new(owner.clone(), name, [], type_ref("Unit"), mode)
            .expect("canonical operation")
    }

    fn clause(
        label: EffectLabel,
        operation: EffectOperation,
        body: u32,
        usage: ResumeUse,
    ) -> HandlerCoreClause {
        HandlerCoreClause::new(
            HandlerClause::new(label, operation).expect("matching handler clause"),
            HandlerCoreNodeId::new(body),
            usage,
        )
    }

    #[test]
    fn handler_core_computes_nested_residuals_and_canonical_bytes() {
        let clock = EffectLabel::clock();
        let random = EffectLabel::random();
        let input = EffectRowModel::open(
            [clock.clone(), random.clone()],
            crate::RowVariableId::new(4),
        );
        let inner = HandlerCore::new(
            input,
            HandlerCoreNodeId::new(10),
            type_ref("Unit"),
            [clause(
                clock.clone(),
                operation(
                    &crate::EffectId::new("Clock").unwrap(),
                    "now",
                    ResumeMode::Once,
                ),
                11,
                ResumeUse::Once,
            )],
            Some(EffectSourceSpan::new("main.ling", 2, 9)),
        )
        .expect("inner Core");
        assert_eq!(inner.residual().canonical_name(), "{Random|ρ4}");
        let outer = HandlerCore::new(
            inner.residual().clone(),
            HandlerCoreNodeId::new(20),
            type_ref("Unit"),
            [clause(
                random,
                operation(
                    &crate::EffectId::new("Random").unwrap(),
                    "next",
                    ResumeMode::Many,
                ),
                21,
                ResumeUse::Many,
            )],
            None,
        )
        .expect("outer Core");
        assert_eq!(outer.residual().canonical_name(), "{|ρ4}");
        assert_ne!(inner.canonical_bytes(), outer.canonical_bytes());
    }

    #[test]
    fn resume_and_body_boundaries_are_checked() {
        let error = HandlerCore::new(
            EffectRowModel::closed([EffectLabel::clock()]),
            HandlerCoreNodeId::new(1),
            type_ref("Unit"),
            [clause(
                EffectLabel::clock(),
                operation(
                    &crate::EffectId::new("Clock").unwrap(),
                    "now",
                    ResumeMode::Once,
                ),
                2,
                ResumeUse::Many,
            )],
            None,
        )
        .expect_err("Many exceeds Once");
        assert!(matches!(
            error,
            HandlerCoreError::ResumeUseExceedsMode { .. }
        ));
        let unresolved = HandlerCore::new(
            EffectRowModel::pure(),
            HandlerCoreNodeId::new(0),
            type_ref("Unit"),
            [],
            None,
        )
        .expect_err("zero body is unresolved");
        assert!(matches!(
            unresolved,
            HandlerCoreError::UnresolvedBody { .. }
        ));
    }

    #[test]
    fn closed_boundary_reports_bilingual_residual_with_original_span() {
        let core = HandlerCore::new(
            EffectRowModel::closed([EffectLabel::random()]),
            HandlerCoreNodeId::new(1),
            type_ref("Unit"),
            [],
            Some(EffectSourceSpan::new("零.ling", 4, 10)),
        )
        .expect("handler with residual");
        let error = core.require_closed().expect_err("residual is unhandled");
        let diagnostic = error.diagnostic().expect("residual diagnostic");
        assert_eq!(diagnostic.code().as_str(), "L-EFFECT-0003");
        assert!(
            diagnostic
                .render_human(ling_diagnostics::MessageLanguage::Chinese)
                .contains("未处理 Effect")
        );
        assert_eq!(diagnostic.primary_span().unwrap().start_byte(), 4);
        assert!(diagnostic.render_json().unwrap().contains("residual_row"));
    }
}
