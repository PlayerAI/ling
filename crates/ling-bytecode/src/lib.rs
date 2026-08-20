//! Data model for the untrusted `ling.bytecode/1.0` boundary.
//!
//! VM-1202 adds deterministic checked lowering, writing, and debug
//! disassembly. Decoding, independent verification, and execution remain
//! unavailable until their owning VM tasks are implemented.

mod disassemble;
mod encode;
mod format;
mod lower;
mod model;

pub use disassemble::disassemble_v1;
pub use encode::{EncodingError, EncodingErrorKind, encode_v1, encode_v1_with_limit};
pub use format::{
    BYTECODE_MAGIC, BYTECODE_PROTOCOL, DecodeLimits, FORMAT_VERSION, FormatVersion, HEADER_BYTES,
    LANGUAGE_VERSION, NO_INDEX, UNICODE_VERSION, UnicodeVersion,
};
pub use lower::{LoweredProgramV1, LoweringError, LoweringErrorKind, LoweringSource, lower_v1};
pub use model::{
    Block, BlockIndex, BlockParameter, Capability, CompareOperator, Constant, ConstantIndex,
    Effect, Function, FunctionIndex, Instruction, IntBinaryOperator, IntUnaryOperator, IntegerSign,
    Intrinsic, Module, ModuleIndex, Package, PackageContentDigest, PackageIndex, PackageReference,
    ProgramParts, RegisterIndex, Source, SourceDigest, SourceIndex, SourceMapEntry, SourceOrigin,
    SourceSpan, StringIndex, Terminator, TypeIndex, UnverifiedProgram, ValueType,
};
