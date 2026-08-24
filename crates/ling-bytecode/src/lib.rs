//! Data model for the untrusted `ling.bytecode/1.x` boundary.
//!
//! VM-1202 adds deterministic checked lowering, writing, and debug
//! disassembly. VM-1203 adds bounded decoding and an independent verifier as
//! the sole constructor of executable-authority state. VM-1204 execution is
//! implemented separately in `ling-vm`, which accepts only that verified
//! state. VM-1205 extends that boundary with the backward-compatible 1.1
//! function, closure, and recursion records accepted by RFC-0015. RFC-0016
//! adds 1.2 aggregates and checked matches; DEC-0261 governs the bounded
//! implementation-active 1.3 Handler slice; DEC-0262 adds the backward-reading
//! 1.4 Cell/State representation.

mod decode;
mod disassemble;
mod encode;
mod error;
mod format;
mod lower;
mod model;
mod path;
mod verify;

pub use decode::{
    DecodedProgramV1, decode_v1, decode_v1_1, decode_v1_1_with_limit, decode_v1_2,
    decode_v1_2_with_limit, decode_v1_3, decode_v1_3_with_limit, decode_v1_4,
    decode_v1_4_with_limit, decode_v1_with_limit,
};
pub use disassemble::{
    disassemble_v1, disassemble_v1_1, disassemble_v1_2, disassemble_v1_3, disassemble_v1_4,
};
pub use encode::{
    EncodingError, EncodingErrorKind, encode_v1, encode_v1_1, encode_v1_1_with_limit, encode_v1_2,
    encode_v1_2_with_limit, encode_v1_3, encode_v1_3_with_limit, encode_v1_4,
    encode_v1_4_with_limit, encode_v1_with_limit, encode_verified_v1,
    encode_verified_v1_with_limit,
};
pub use error::{BytecodeError, BytecodePhase, BytecodeReason};
pub use format::{
    BYTECODE_MAGIC, BYTECODE_PROTOCOL, BYTECODE_PROTOCOL_1_0, BYTECODE_PROTOCOL_1_1,
    BYTECODE_PROTOCOL_1_2, BYTECODE_PROTOCOL_1_3, BYTECODE_PROTOCOL_1_4, DecodeLimits,
    FORMAT_VERSION, FORMAT_VERSION_1_0, FORMAT_VERSION_1_1, FORMAT_VERSION_1_2, FORMAT_VERSION_1_3,
    FORMAT_VERSION_1_4, FormatVersion, HEADER_BYTES, LANGUAGE_VERSION, NO_INDEX, UNICODE_VERSION,
    UnicodeVersion,
};
pub use lower::{
    LoweredProgramV1, LoweredProgramV1_1, LoweredProgramV1_2, LoweredProgramV1_3,
    LoweredProgramV1_4, LoweringError, LoweringErrorKind, LoweringSource, lower_v1, lower_v1_1,
    lower_v1_2, lower_v1_3, lower_v1_4,
};
pub use model::{
    Block, BlockIndex, BlockParameter, Capability, CaptureOperand, CompareOperator, Constant,
    ConstantIndex, Effect, Function, FunctionIndex, FunctionKind, HandlerClause, HandlerOperation,
    Instruction, IntBinaryOperator, IntUnaryOperator, IntegerSign, Intrinsic, Module, ModuleIndex,
    Package, PackageContentDigest, PackageIndex, PackageReference, ProgramParts, RecordField,
    RecordUpdate, RegisterIndex, Source, SourceDigest, SourceIndex, SourceMapEntry, SourceOrigin,
    SourceSpan, StringIndex, Terminator, TypeIndex, UnverifiedProgram, ValueType, VariantCase,
};
pub use verify::{
    VerifiedProgramV1, decode_and_verify_v1, decode_and_verify_v1_1,
    decode_and_verify_v1_1_with_limit, decode_and_verify_v1_2, decode_and_verify_v1_2_with_limit,
    decode_and_verify_v1_3, decode_and_verify_v1_3_with_limit, decode_and_verify_v1_4,
    decode_and_verify_v1_4_with_limit, decode_and_verify_v1_with_limit, verify_v1,
};
