use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticCode, DiagnosticSpan, Severity, codes};

/// Deterministic validation phase for one bytecode failure.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BytecodePhase {
    Envelope,
    Table,
    Constant,
    Instruction,
    ControlFlow,
    Register,
    Type,
    Effect,
    Capability,
    Entry,
    SourceMap,
}

impl BytecodePhase {
    /// Returns the stable machine-readable phase tag.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Envelope => "envelope",
            Self::Table => "table",
            Self::Constant => "constant",
            Self::Instruction => "instruction",
            Self::ControlFlow => "control_flow",
            Self::Register => "register",
            Self::Type => "type",
            Self::Effect => "effect",
            Self::Capability => "capability",
            Self::Entry => "entry",
            Self::SourceMap => "source_map",
        }
    }
}

/// Stable reason selected independently of Rust error/debug text.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BytecodeReason {
    InvalidMagic,
    InvalidHeaderLength,
    UnsupportedVersion,
    ReservedNonzero,
    TruncatedArtifact,
    TrailingBytes,
    ResourceLimit,
    InvalidUtf8,
    InvalidTableOrder,
    InvalidName,
    InvalidLogicalPath,
    InvalidTag,
    InvalidBoolean,
    InvalidRecordLength,
    NoncanonicalInteger,
    UnknownOpcode,
    InvalidInstructionLength,
    InvalidStringIndex,
    InvalidPackageIndex,
    InvalidModuleIndex,
    InvalidTypeIndex,
    InvalidConstantIndex,
    InvalidSourceIndex,
    InvalidFunctionIndex,
    InvalidBlockIndex,
    InvalidRegisterIndex,
    DuplicateRegisterDefinition,
    RegisterNotDominated,
    UnreachableBlock,
    InvalidBlockShape,
    InvalidRegisterType,
    BlockArgumentTypeMismatch,
    CallSignatureMismatch,
    InvalidReturnType,
    EffectMismatch,
    CapabilityMismatch,
    InvalidEntry,
    IncompleteSourceMap,
    DuplicateSourceMap,
    InvalidSourceMapOrder,
    InvalidSourceSpan,
    InvalidSourceOwner,
}

impl BytecodeReason {
    /// Returns the stable machine-readable reason tag.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidMagic => "invalid_magic",
            Self::InvalidHeaderLength => "invalid_header_length",
            Self::UnsupportedVersion => "unsupported_version",
            Self::ReservedNonzero => "reserved_nonzero",
            Self::TruncatedArtifact => "truncated_artifact",
            Self::TrailingBytes => "trailing_bytes",
            Self::ResourceLimit => "resource_limit",
            Self::InvalidUtf8 => "invalid_utf8",
            Self::InvalidTableOrder => "invalid_table_order",
            Self::InvalidName => "invalid_name",
            Self::InvalidLogicalPath => "invalid_logical_path",
            Self::InvalidTag => "invalid_tag",
            Self::InvalidBoolean => "invalid_boolean",
            Self::InvalidRecordLength => "invalid_record_length",
            Self::NoncanonicalInteger => "noncanonical_integer",
            Self::UnknownOpcode => "unknown_opcode",
            Self::InvalidInstructionLength => "invalid_instruction_length",
            Self::InvalidStringIndex => "invalid_string_index",
            Self::InvalidPackageIndex => "invalid_package_index",
            Self::InvalidModuleIndex => "invalid_module_index",
            Self::InvalidTypeIndex => "invalid_type_index",
            Self::InvalidConstantIndex => "invalid_constant_index",
            Self::InvalidSourceIndex => "invalid_source_index",
            Self::InvalidFunctionIndex => "invalid_function_index",
            Self::InvalidBlockIndex => "invalid_block_index",
            Self::InvalidRegisterIndex => "invalid_register_index",
            Self::DuplicateRegisterDefinition => "duplicate_register_definition",
            Self::RegisterNotDominated => "register_not_dominated",
            Self::UnreachableBlock => "unreachable_block",
            Self::InvalidBlockShape => "invalid_block_shape",
            Self::InvalidRegisterType => "invalid_register_type",
            Self::BlockArgumentTypeMismatch => "block_argument_type_mismatch",
            Self::CallSignatureMismatch => "call_signature_mismatch",
            Self::InvalidReturnType => "invalid_return_type",
            Self::EffectMismatch => "effect_mismatch",
            Self::CapabilityMismatch => "capability_mismatch",
            Self::InvalidEntry => "invalid_entry",
            Self::IncompleteSourceMap => "incomplete_source_map",
            Self::DuplicateSourceMap => "duplicate_source_map",
            Self::InvalidSourceMapOrder => "invalid_source_map_order",
            Self::InvalidSourceSpan => "invalid_source_span",
            Self::InvalidSourceOwner => "invalid_source_owner",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResourceFacts {
    resource: String,
    actual: u64,
    maximum: u64,
}

/// One bounded decoder/verifier failure at an artifact byte offset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BytecodeError {
    phase: BytecodePhase,
    reason: BytecodeReason,
    offset: u64,
    width: u32,
    referenced_indices: Vec<u32>,
    resource: Option<ResourceFacts>,
}

impl BytecodeError {
    #[must_use]
    pub const fn phase(&self) -> BytecodePhase {
        self.phase
    }

    #[must_use]
    pub const fn reason(&self) -> BytecodeReason {
        self.reason
    }

    #[must_use]
    pub const fn offset(&self) -> u64 {
        self.offset
    }

    #[must_use]
    pub fn referenced_indices(&self) -> &[u32] {
        &self.referenced_indices
    }

    #[must_use]
    pub const fn code(&self) -> DiagnosticCode {
        if matches!(self.reason, BytecodeReason::ResourceLimit) {
            return codes::BYTECODE_RESOURCE_LIMIT_EXCEEDED;
        }
        match self.phase {
            BytecodePhase::Envelope => codes::INVALID_BYTECODE_ENVELOPE,
            BytecodePhase::Table | BytecodePhase::Constant => codes::INVALID_BYTECODE_TABLE,
            BytecodePhase::Instruction
            | BytecodePhase::ControlFlow
            | BytecodePhase::Register
            | BytecodePhase::Type => codes::INVALID_BYTECODE_PROGRAM,
            BytecodePhase::Effect | BytecodePhase::Capability | BytecodePhase::Entry => {
                codes::INVALID_BYTECODE_AUTHORITY
            }
            BytecodePhase::SourceMap => codes::INVALID_BYTECODE_SOURCE_MAP,
        }
    }

    /// Builds the registered bilingual diagnostic without exposing host errors.
    #[must_use]
    pub fn to_diagnostic(&self, artifact_name: &str) -> Diagnostic {
        let reason = self.reason.as_str();
        let code = self.code();
        let (message_zh, message_en) = if code == codes::INVALID_BYTECODE_ENVELOPE {
            (
                format!("字节码封装无效：{reason}"),
                format!("invalid bytecode envelope: {reason}"),
            )
        } else if code == codes::BYTECODE_RESOURCE_LIMIT_EXCEEDED {
            let resource = self
                .resource
                .as_ref()
                .map_or("unknown", |facts| facts.resource.as_str());
            (
                format!("字节码资源超过上限：{resource}"),
                format!("bytecode resource limit exceeded: {resource}"),
            )
        } else if code == codes::INVALID_BYTECODE_TABLE {
            (
                format!("字节码表或规范值无效：{reason}"),
                format!("invalid bytecode table or canonical value: {reason}"),
            )
        } else if code == codes::INVALID_BYTECODE_PROGRAM {
            (
                format!("字节码程序结构无效：{reason}"),
                format!("invalid bytecode program structure: {reason}"),
            )
        } else if code == codes::INVALID_BYTECODE_AUTHORITY {
            (
                format!("字节码权限或入口无效：{reason}"),
                format!("invalid bytecode authority or entry: {reason}"),
            )
        } else {
            (
                format!("字节码源码映射无效：{reason}"),
                format!("invalid bytecode source map: {reason}"),
            )
        };

        let start = u32::try_from(self.offset).unwrap_or(u32::MAX);
        let end = self
            .offset
            .saturating_add(u64::from(self.width.max(1)))
            .min(u64::from(u32::MAX));
        let mut diagnostic = Diagnostic::new(code, Severity::Error, message_zh, message_en)
            .with_primary_span(DiagnosticSpan::at(
                artifact_name,
                start,
                u32::try_from(end).unwrap_or(u32::MAX),
            ))
            .with_fact("offset", self.offset)
            .with_fact("phase", self.phase.as_str())
            .with_fact("reason", reason);
        if !self.referenced_indices.is_empty() {
            diagnostic = diagnostic.with_fact(
                "referenced_indices",
                self.referenced_indices
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>(),
            );
        }
        if let Some(resource) = &self.resource {
            diagnostic = diagnostic
                .with_fact("actual", resource.actual)
                .with_fact("maximum", resource.maximum)
                .with_fact("resource", resource.resource.clone());
        }
        diagnostic
    }

    pub(crate) fn new(
        phase: BytecodePhase,
        reason: BytecodeReason,
        offset: u64,
        width: u32,
    ) -> Self {
        Self {
            phase,
            reason,
            offset,
            width,
            referenced_indices: Vec::new(),
            resource: None,
        }
    }

    pub(crate) fn with_indices(mut self, values: impl IntoIterator<Item = u32>) -> Self {
        self.referenced_indices.extend(values);
        self
    }

    pub(crate) fn resource(
        phase: BytecodePhase,
        offset: u64,
        resource: &str,
        actual: u64,
        maximum: u64,
    ) -> Self {
        Self {
            phase,
            reason: BytecodeReason::ResourceLimit,
            offset,
            width: 4,
            referenced_indices: Vec::new(),
            resource: Some(ResourceFacts {
                resource: resource.to_owned(),
                actual,
                maximum,
            }),
        }
    }
}

impl fmt::Display for BytecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} at byte {} ({})",
            self.reason.as_str(),
            self.offset,
            self.phase.as_str()
        )
    }
}

impl Error for BytecodeError {}
