use std::error::Error;
use std::fmt;

use ling_bytecode::SourceSpan;
use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};

use crate::HostErrorCategory;

/// Logical runtime resource whose deterministic limit was exhausted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeResource {
    Step,
    Frame,
    HandlerDepth,
    ContinuationFrame,
}

impl RuntimeResource {
    pub(crate) const fn operation(self) -> &'static str {
        match self {
            Self::Step => "step_limit",
            Self::Frame => "frame_limit",
            Self::HandlerDepth => "handler_depth_limit",
            Self::ContinuationFrame => "continuation_frame_limit",
        }
    }
}

/// Runtime failures defined for the verified bytecode 1.x execution boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeFaultKind {
    DivisionByZero {
        operation: &'static str,
    },
    InvalidFormatPlaceholderCount {
        count: u32,
    },
    HostCapability {
        operation: &'static str,
        category: HostErrorCategory,
    },
    CapabilityUnavailable {
        capability: &'static str,
    },
    ResourceLimit {
        resource: RuntimeResource,
    },
    OutOfMemory {
        operation: &'static str,
    },
    Cancelled,
    HandlerResumeCardinality {
        operation: &'static str,
    },
}

impl RuntimeFaultKind {
    const fn category(&self) -> &'static str {
        match self {
            Self::DivisionByZero { .. } => "division_by_zero",
            Self::InvalidFormatPlaceholderCount { .. } => "invalid_format",
            Self::HostCapability { category, .. } => category.name(),
            Self::CapabilityUnavailable { .. } => "capability_unavailable",
            Self::ResourceLimit { .. } => "resource_limit",
            Self::OutOfMemory { .. } => "out_of_memory",
            Self::Cancelled => "cancelled",
            Self::HandlerResumeCardinality { .. } => "handler_resume_cardinality",
        }
    }

    const fn operation(&self) -> &'static str {
        match self {
            Self::DivisionByZero { operation }
            | Self::HostCapability { operation, .. }
            | Self::HandlerResumeCardinality { operation }
            | Self::OutOfMemory { operation } => operation,
            Self::InvalidFormatPlaceholderCount { .. } => "Text.format",
            Self::CapabilityUnavailable { capability } => capability,
            Self::ResourceLimit { resource } => resource.operation(),
            Self::Cancelled => "execution.cancelled",
        }
    }
}

/// One source-mapped Ling Runtime Fault.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeFault {
    kind: RuntimeFaultKind,
    source_name: String,
    span: SourceSpan,
    committed: bool,
}

impl RuntimeFault {
    pub(crate) fn new(
        kind: RuntimeFaultKind,
        source_name: String,
        span: SourceSpan,
        committed: bool,
    ) -> Self {
        Self {
            kind,
            source_name,
            span,
            committed,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &RuntimeFaultKind {
        &self.kind
    }

    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }

    #[must_use]
    pub const fn committed(&self) -> bool {
        self.committed
    }

    /// Renders the existing bilingual `L-RUNTIME-0001` contract.
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        let (message_zh, message_en) = match &self.kind {
            RuntimeFaultKind::DivisionByZero { .. } => (
                "整数除数不能为零".to_owned(),
                "integer divisor cannot be zero".to_owned(),
            ),
            RuntimeFaultKind::InvalidFormatPlaceholderCount { count } => (
                format!("Text.format 要求恰好一个占位符，实际为 {count}"),
                format!("Text.format requires exactly one placeholder, found {count}"),
            ),
            RuntimeFaultKind::HostCapability {
                operation,
                category: _,
            } => (
                format!("宿主 Capability 操作“{operation}”失败"),
                format!("host capability operation `{operation}` failed"),
            ),
            RuntimeFaultKind::CapabilityUnavailable { capability } => (
                format!("宿主 Capability“{capability}”不可用"),
                format!("host capability `{capability}` is unavailable"),
            ),
            RuntimeFaultKind::ResourceLimit { resource } => (
                format!("运行时资源上限已耗尽：{}", resource.operation()),
                format!("runtime resource limit exhausted: {}", resource.operation()),
            ),
            RuntimeFaultKind::OutOfMemory { operation } => (
                format!("运行时内存上限阻止了操作“{operation}”"),
                format!("runtime memory ceiling prevented operation `{operation}`"),
            ),
            RuntimeFaultKind::Cancelled => (
                "运行时执行已取消".to_owned(),
                "runtime execution was cancelled".to_owned(),
            ),
            RuntimeFaultKind::HandlerResumeCardinality { operation } => (
                format!("Handler operation“{operation}”的 continuation 只能恢复一次"),
                format!("handler operation `{operation}` continuation may be resumed only once"),
            ),
        };
        Diagnostic::new(
            codes::RUNTIME_FAULT,
            Severity::Error,
            message_zh,
            message_en,
        )
        .with_primary_span(DiagnosticSpan::at_u64(
            &self.source_name,
            self.span.start_byte(),
            self.span.end_byte(),
        ))
        .with_fact("category", self.kind.category())
        .with_fact("committed", self.committed)
        .with_fact("operation", self.kind.operation())
    }
}

impl fmt::Display for RuntimeFault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.kind.category(),
            self.kind.operation()
        )
    }
}

impl Error for RuntimeFault {}

/// A verifier invariant failed after executable authority was created.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InternalExecutionError {
    invariant: &'static str,
}

impl InternalExecutionError {
    pub(crate) const fn new(invariant: &'static str) -> Self {
        Self { invariant }
    }

    #[must_use]
    pub const fn invariant(self) -> &'static str {
        self.invariant
    }
}

impl fmt::Display for InternalExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "verified bytecode invariant failed: {}",
            self.invariant
        )
    }
}

impl Error for InternalExecutionError {}

/// Complete VM failure boundary. Runtime Faults are user-observable;
/// verifier-invariant failures must be routed to `L-INTERNAL-0001` by an
/// owning compiler/CLI incident boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionError {
    Runtime(RuntimeFault),
    Internal(InternalExecutionError),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Runtime(fault) => fault.fmt(formatter),
            Self::Internal(error) => error.fmt(formatter),
        }
    }
}

impl Error for ExecutionError {}

impl From<RuntimeFault> for ExecutionError {
    fn from(fault: RuntimeFault) -> Self {
        Self::Runtime(fault)
    }
}

impl From<InternalExecutionError> for ExecutionError {
    fn from(error: InternalExecutionError) -> Self {
        Self::Internal(error)
    }
}
