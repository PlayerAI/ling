use std::error::Error;
use std::fmt;

use crate::{
    BYTECODE_MAGIC, Block, CaptureOperand, Constant, DecodeLimits, FORMAT_VERSION_1_0,
    FORMAT_VERSION_1_1, FormatVersion, Function, FunctionKind, HEADER_BYTES, Instruction,
    LANGUAGE_VERSION, LoweredProgramV1, LoweredProgramV1_1, NO_INDEX, PackageReference,
    SourceMapEntry, Terminator, UNICODE_VERSION, UnverifiedProgram, ValueType, VerifiedProgramV1,
};

/// Failure categories for deterministic bytecode writing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EncodingErrorKind {
    ResourceLimit {
        resource: String,
        actual: u64,
        maximum: u64,
    },
    ModelInvariant {
        reason: String,
    },
}

/// A bounded bytecode writer failure; malformed input never panics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncodingError {
    kind: EncodingErrorKind,
}

impl EncodingError {
    #[must_use]
    pub const fn kind(&self) -> &EncodingErrorKind {
        &self.kind
    }

    fn resource(actual: u64, maximum: u64) -> Self {
        Self {
            kind: EncodingErrorKind::ResourceLimit {
                resource: "artifact_bytes".to_owned(),
                actual,
                maximum,
            },
        }
    }

    fn invariant(reason: impl Into<String>) -> Self {
        Self {
            kind: EncodingErrorKind::ModelInvariant {
                reason: reason.into(),
            },
        }
    }
}

impl fmt::Display for EncodingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            EncodingErrorKind::ResourceLimit {
                resource,
                actual,
                maximum,
            } => write!(
                formatter,
                "bytecode encoding resource {resource} is {actual}, maximum {maximum}"
            ),
            EncodingErrorKind::ModelInvariant { reason } => {
                write!(formatter, "invalid lowered bytecode model: {reason}")
            }
        }
    }
}

impl Error for EncodingError {}

/// Encodes canonical bytecode using the RFC-0014 hard artifact limit.
pub fn encode_v1(program: &LoweredProgramV1) -> Result<Vec<u8>, EncodingError> {
    encode_v1_with_limit(program, DecodeLimits::rfc_0014().artifact_bytes())
}

/// Encodes canonical bytecode under a caller-supplied limit no larger than the RFC maximum.
pub fn encode_v1_with_limit(
    program: &LoweredProgramV1,
    artifact_byte_limit: u64,
) -> Result<Vec<u8>, EncodingError> {
    let hard_limit = DecodeLimits::rfc_0014().artifact_bytes();
    let bytes = encode_model(program.model(), FORMAT_VERSION_1_0, hard_limit)?;
    let actual =
        u64::try_from(bytes.len()).map_err(|_| EncodingError::resource(u64::MAX, hard_limit))?;
    let effective_limit = artifact_byte_limit.min(hard_limit);
    if actual > effective_limit {
        return Err(EncodingError::resource(actual, effective_limit));
    }
    Ok(bytes)
}

/// Encodes canonical bytecode 1.1 using the RFC-0015 hard artifact limit.
pub fn encode_v1_1(program: &LoweredProgramV1_1) -> Result<Vec<u8>, EncodingError> {
    encode_v1_1_with_limit(program, DecodeLimits::rfc_0015().artifact_bytes())
}

/// Encodes canonical bytecode 1.1 under a caller-supplied limit no larger than RFC-0015.
pub fn encode_v1_1_with_limit(
    program: &LoweredProgramV1_1,
    artifact_byte_limit: u64,
) -> Result<Vec<u8>, EncodingError> {
    let hard_limit = DecodeLimits::rfc_0015().artifact_bytes();
    let bytes = encode_model(program.model(), FORMAT_VERSION_1_1, hard_limit)?;
    let actual =
        u64::try_from(bytes.len()).map_err(|_| EncodingError::resource(u64::MAX, hard_limit))?;
    let effective_limit = artifact_byte_limit.min(hard_limit);
    if actual > effective_limit {
        return Err(EncodingError::resource(actual, effective_limit));
    }
    Ok(bytes)
}

/// Re-encodes a verified program using the canonical version-1.0 writer.
pub fn encode_verified_v1(program: &VerifiedProgramV1) -> Result<Vec<u8>, EncodingError> {
    encode_verified_v1_with_limit(program, DecodeLimits::rfc_0014().artifact_bytes())
}

/// Re-encodes a verified program under a caller limit capped by RFC-0014.
pub fn encode_verified_v1_with_limit(
    program: &VerifiedProgramV1,
    artifact_byte_limit: u64,
) -> Result<Vec<u8>, EncodingError> {
    let limits = if program.version() == FORMAT_VERSION_1_1 {
        DecodeLimits::rfc_0015()
    } else {
        DecodeLimits::rfc_0014()
    };
    let hard_limit = limits.artifact_bytes();
    let bytes = encode_model(program.model(), program.version(), hard_limit)?;
    let actual =
        u64::try_from(bytes.len()).map_err(|_| EncodingError::resource(u64::MAX, hard_limit))?;
    let effective_limit = artifact_byte_limit.min(hard_limit);
    if actual > effective_limit {
        return Err(EncodingError::resource(actual, effective_limit));
    }
    Ok(bytes)
}

pub(crate) fn encode_model(
    program: &UnverifiedProgram,
    version: FormatVersion,
    hard_limit: u64,
) -> Result<Vec<u8>, EncodingError> {
    if version != FORMAT_VERSION_1_0 && version != FORMAT_VERSION_1_1 {
        return Err(EncodingError::invariant(
            "unsupported bytecode writer version",
        ));
    }
    let mut writer = Writer::new(hard_limit)?;
    writer.bytes(&BYTECODE_MAGIC)?;
    writer.u32(HEADER_BYTES)?;
    writer.u16(version.major())?;
    writer.u16(version.minor())?;
    writer.u16(LANGUAGE_VERSION.major())?;
    writer.u16(LANGUAGE_VERSION.minor())?;
    writer.u16(UNICODE_VERSION.major())?;
    writer.u16(UNICODE_VERSION.minor())?;
    writer.u16(UNICODE_VERSION.patch())?;
    writer.u16(0)?;
    writer.u32(0)?;
    writer.u64(0)?;

    writer.table(program.strings(), |payload, value| {
        payload.bytes(value.as_bytes())
    })?;
    writer.table(program.packages(), |payload, package| {
        payload.u32(package.name.get())?;
        payload.u32(package.version.get())?;
        payload.bytes(package.content_sha256.as_bytes())
    })?;
    writer.table(program.modules(), |payload, module| {
        payload.u32(match module.package {
            PackageReference::Standalone => NO_INDEX,
            PackageReference::Package(index) => index.get(),
        })?;
        payload.u32(module.name.get())?;
        payload.length(module.capabilities.len(), "capability count")?;
        for capability in &module.capabilities {
            payload.u8(capability.tag())?;
        }
        Ok(())
    })?;
    writer.table(program.types(), |payload, value| {
        encode_type(payload, value, version)
    })?;
    writer.table(program.constants(), encode_constant)?;
    writer.table(program.sources(), |payload, source| {
        payload.u32(source.module.get())?;
        payload.u32(source.logical_name.get())?;
        payload.u64(source.original_byte_length)?;
        payload.bytes(source.content_sha256.as_bytes())
    })?;
    writer.table(program.functions(), |payload, function| {
        encode_function(payload, function, version)
    })?;
    writer.u32(program.entry().get())?;
    writer.table(program.source_map(), encode_source_map)?;

    let mut bytes = writer.finish();
    let total =
        u64::try_from(bytes.len()).map_err(|_| EncodingError::resource(u64::MAX, hard_limit))?;
    let target = bytes
        .get_mut(32..40)
        .ok_or_else(|| EncodingError::invariant("encoded header is shorter than 40 bytes"))?;
    target.copy_from_slice(&total.to_le_bytes());
    Ok(bytes)
}

fn encode_type(
    writer: &mut Writer,
    value: &ValueType,
    version: FormatVersion,
) -> Result<(), EncodingError> {
    writer.u8(value.tag())?;
    match value {
        ValueType::Unit | ValueType::Bool | ValueType::Int | ValueType::Text => Ok(()),
        ValueType::Function {
            parameters,
            result,
            effects,
        } if version == FORMAT_VERSION_1_1 => {
            writer.bytes(&[0; 3])?;
            writer.length(parameters.len(), "function type parameter count")?;
            for parameter in parameters {
                writer.u32(parameter.get())?;
            }
            writer.u32(result.get())?;
            writer.length(effects.len(), "function type effect count")?;
            for effect in effects {
                writer.u8(effect.tag())?;
            }
            Ok(())
        }
        ValueType::Function { .. } => Err(EncodingError::invariant(
            "bytecode 1.0 cannot encode a function value type",
        )),
    }
}

fn encode_constant(writer: &mut Writer, constant: &Constant) -> Result<(), EncodingError> {
    writer.u8(constant.tag())?;
    writer.bytes(&[0; 3])?;
    let type_index = match constant {
        Constant::Unit => 0,
        Constant::Bool(_) => 1,
        Constant::Int { .. } => 2,
        Constant::Text(_) => 3,
    };
    writer.u32(type_index)?;
    match constant {
        Constant::Unit => Ok(()),
        Constant::Bool(value) => writer.u8(u8::from(*value)),
        Constant::Int { sign, magnitude } => {
            writer.u8(sign.tag())?;
            writer.bytes(&[0; 3])?;
            writer.length(magnitude.len(), "integer magnitude length")?;
            writer.bytes(magnitude)
        }
        Constant::Text(value) => writer.u32(value.get()),
    }
}

fn encode_function(
    writer: &mut Writer,
    function: &Function,
    version: FormatVersion,
) -> Result<(), EncodingError> {
    if version == FORMAT_VERSION_1_1 {
        writer.u8(function.kind.tag())?;
        writer.bytes(&[0; 3])?;
    } else if function.kind != FunctionKind::Named || function.capture_count != 0 {
        return Err(EncodingError::invariant(
            "bytecode 1.0 functions must be named and captureless",
        ));
    }
    writer.u32(function.module.get())?;
    writer.u32(function.name.get())?;
    if version == FORMAT_VERSION_1_1 {
        writer.u32(function.capture_count)?;
    }
    writer.length(function.parameter_types.len(), "parameter type count")?;
    for parameter in &function.parameter_types {
        writer.u32(parameter.get())?;
    }
    writer.u32(function.result_type.get())?;
    writer.length(function.effects.len(), "effect count")?;
    for effect in &function.effects {
        writer.u8(effect.tag())?;
    }
    writer.u32(function.register_count)?;
    writer.length(function.blocks.len(), "block count")?;
    for block in &function.blocks {
        writer.record(|payload| encode_block(payload, block, version))?;
    }
    Ok(())
}

fn encode_block(
    writer: &mut Writer,
    block: &Block,
    version: FormatVersion,
) -> Result<(), EncodingError> {
    writer.length(block.parameters.len(), "block parameter count")?;
    for parameter in &block.parameters {
        writer.u32(parameter.register.get())?;
        writer.u32(parameter.value_type.get())?;
    }
    writer.length(block.instructions.len(), "instruction count")?;
    for instruction in &block.instructions {
        writer.record(|payload| encode_instruction(payload, instruction, version))?;
    }
    writer.record(|payload| encode_terminator(payload, &block.terminator))
}

fn encode_instruction(
    writer: &mut Writer,
    instruction: &Instruction,
    version: FormatVersion,
) -> Result<(), EncodingError> {
    writer.u8(instruction.opcode())?;
    writer.bytes(&[0; 3])?;
    match instruction {
        Instruction::Const {
            destination,
            constant,
        } => {
            writer.u32(destination.get())?;
            writer.u32(constant.get())
        }
        Instruction::IntUnary {
            destination,
            operator,
            operand,
        } => {
            writer.u32(destination.get())?;
            writer.u8(operator.tag())?;
            writer.bytes(&[0; 3])?;
            writer.u32(operand.get())
        }
        Instruction::IntBinary {
            destination,
            operator,
            left,
            right,
        } => {
            writer.u32(destination.get())?;
            writer.u8(operator.tag())?;
            writer.bytes(&[0; 3])?;
            writer.u32(left.get())?;
            writer.u32(right.get())
        }
        Instruction::Compare {
            destination,
            operator,
            left,
            right,
        } => {
            writer.u32(destination.get())?;
            writer.u8(operator.tag())?;
            writer.bytes(&[0; 3])?;
            writer.u32(left.get())?;
            writer.u32(right.get())
        }
        Instruction::Call {
            destination,
            function,
            arguments,
        } => {
            writer.u32(destination.get())?;
            writer.u32(function.get())?;
            writer.registers(arguments)
        }
        Instruction::MakeClosure {
            destination,
            function,
            captures,
        } if version == FORMAT_VERSION_1_1 => {
            writer.u32(destination.get())?;
            writer.u32(function.get())?;
            writer.length(captures.len(), "closure capture count")?;
            for capture in captures {
                writer.u8(capture.tag())?;
                writer.bytes(&[0; 3])?;
                match capture {
                    CaptureOperand::Register(register) => writer.u32(register.get())?,
                    CaptureOperand::SelfReference => writer.u32(NO_INDEX)?,
                }
            }
            Ok(())
        }
        Instruction::CallClosure {
            destination,
            callee,
            arguments,
        } if version == FORMAT_VERSION_1_1 => {
            writer.u32(destination.get())?;
            writer.u32(callee.get())?;
            writer.registers(arguments)
        }
        Instruction::MakeClosure { .. } | Instruction::CallClosure { .. } => Err(
            EncodingError::invariant("bytecode 1.0 cannot encode closure instructions"),
        ),
        Instruction::Intrinsic {
            destination,
            intrinsic,
            arguments,
        } => {
            writer.u32(destination.get())?;
            writer.u8(intrinsic.tag())?;
            writer.bytes(&[0; 3])?;
            writer.registers(arguments)
        }
        Instruction::ConsoleWrite { destination, text } => {
            writer.u32(destination.get())?;
            writer.u32(text.get())
        }
    }
}

fn encode_terminator(writer: &mut Writer, terminator: &Terminator) -> Result<(), EncodingError> {
    writer.u8(terminator.opcode())?;
    writer.bytes(&[0; 3])?;
    match terminator {
        Terminator::Jump { target, arguments } => {
            writer.u32(target.get())?;
            writer.registers(arguments)
        }
        Terminator::Branch {
            condition,
            true_target,
            true_arguments,
            false_target,
            false_arguments,
        } => {
            writer.u32(condition.get())?;
            writer.u32(true_target.get())?;
            writer.registers(true_arguments)?;
            writer.u32(false_target.get())?;
            writer.registers(false_arguments)
        }
        Terminator::Return { value } => writer.u32(value.get()),
    }
}

fn encode_source_map(writer: &mut Writer, entry: &SourceMapEntry) -> Result<(), EncodingError> {
    writer.u32(entry.function.get())?;
    writer.u32(entry.block.get())?;
    writer.u32(entry.ordinal)?;
    writer.u32(entry.source.get())?;
    writer.u64(entry.span.start_byte())?;
    writer.u64(entry.span.end_byte())?;
    writer.u8(entry.origin.tag())?;
    writer.bytes(&[0; 7])
}

struct Writer {
    bytes: Vec<u8>,
    limit: usize,
}

impl Writer {
    fn new(limit: u64) -> Result<Self, EncodingError> {
        let limit = usize::try_from(limit).map_err(|_| {
            EncodingError::invariant("configured artifact limit does not fit host usize")
        })?;
        Ok(Self {
            bytes: Vec::new(),
            limit,
        })
    }

    fn u8(&mut self, value: u8) -> Result<(), EncodingError> {
        self.bytes(&[value])
    }

    fn u16(&mut self, value: u16) -> Result<(), EncodingError> {
        self.bytes(&value.to_le_bytes())
    }

    fn u32(&mut self, value: u32) -> Result<(), EncodingError> {
        self.bytes(&value.to_le_bytes())
    }

    fn u64(&mut self, value: u64) -> Result<(), EncodingError> {
        self.bytes(&value.to_le_bytes())
    }

    fn length(&mut self, value: usize, label: &str) -> Result<(), EncodingError> {
        let value = u32::try_from(value)
            .map_err(|_| EncodingError::invariant(format!("{label} does not fit u32")))?;
        self.u32(value)
    }

    fn bytes(&mut self, value: &[u8]) -> Result<(), EncodingError> {
        let next = self
            .bytes
            .len()
            .checked_add(value.len())
            .ok_or_else(|| EncodingError::resource(u64::MAX, self.limit as u64))?;
        if next > self.limit {
            return Err(EncodingError::resource(next as u64, self.limit as u64));
        }
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    fn record(
        &mut self,
        encode: impl FnOnce(&mut Self) -> Result<(), EncodingError>,
    ) -> Result<(), EncodingError> {
        let mut payload = Self {
            bytes: Vec::new(),
            limit: self.limit,
        };
        encode(&mut payload)?;
        self.length(payload.bytes.len(), "record payload length")?;
        self.bytes(&payload.bytes)
    }

    fn table<T>(
        &mut self,
        values: &[T],
        mut encode: impl FnMut(&mut Self, &T) -> Result<(), EncodingError>,
    ) -> Result<(), EncodingError> {
        self.length(values.len(), "table entry count")?;
        for value in values {
            self.record(|payload| encode(payload, value))?;
        }
        Ok(())
    }

    fn registers(&mut self, values: &[crate::RegisterIndex]) -> Result<(), EncodingError> {
        self.length(values.len(), "register argument count")?;
        for value in values {
            self.u32(value.get())?;
        }
        Ok(())
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}
