//! Data model for the untrusted `ling.bytecode/1.x` boundary.
//!
//! VM-1202 adds deterministic checked lowering, writing, and debug
//! disassembly. VM-1203 adds bounded decoding and an independent verifier as
//! the sole constructor of executable-authority state. VM-1204 execution is
//! implemented separately in `ling-vm`, which accepts only that verified
//! state. VM-1205 extends that boundary with the backward-compatible 1.1
//! function, closure, and recursion records accepted by RFC-0015.

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
    DecodedProgramV1, decode_v1, decode_v1_1, decode_v1_1_with_limit, decode_v1_with_limit,
};
pub use disassemble::{disassemble_v1, disassemble_v1_1};
pub use encode::{
    EncodingError, EncodingErrorKind, encode_v1, encode_v1_1, encode_v1_1_with_limit,
    encode_v1_with_limit, encode_verified_v1, encode_verified_v1_with_limit,
};
pub use error::{BytecodeError, BytecodePhase, BytecodeReason};
pub use format::{
    BYTECODE_MAGIC, BYTECODE_PROTOCOL, BYTECODE_PROTOCOL_1_0, BYTECODE_PROTOCOL_1_1, DecodeLimits,
    FORMAT_VERSION, FORMAT_VERSION_1_0, FORMAT_VERSION_1_1, FormatVersion, HEADER_BYTES,
    LANGUAGE_VERSION, NO_INDEX, UNICODE_VERSION, UnicodeVersion,
};
pub use lower::{
    LoweredProgramV1, LoweredProgramV1_1, LoweringError, LoweringErrorKind, LoweringSource,
    lower_v1, lower_v1_1,
};
pub use model::{
    Block, BlockIndex, BlockParameter, Capability, CaptureOperand, CompareOperator, Constant,
    ConstantIndex, Effect, Function, FunctionIndex, FunctionKind, Instruction, IntBinaryOperator,
    IntUnaryOperator, IntegerSign, Intrinsic, Module, ModuleIndex, Package, PackageContentDigest,
    PackageIndex, PackageReference, ProgramParts, RegisterIndex, Source, SourceDigest, SourceIndex,
    SourceMapEntry, SourceOrigin, SourceSpan, StringIndex, Terminator, TypeIndex,
    UnverifiedProgram, ValueType,
};
pub use verify::{
    VerifiedProgramV1, decode_and_verify_v1, decode_and_verify_v1_1,
    decode_and_verify_v1_1_with_limit, decode_and_verify_v1_with_limit, verify_v1,
};
