//! Data model for the untrusted `ling.bytecode/1.0` boundary.
//!
//! VM-1201 intentionally provides no encoder, decoder, verifier, lowering, or
//! execution entry point. Those capabilities belong to later VM tasks.

mod format;
mod model;

pub use format::{
    BYTECODE_MAGIC, BYTECODE_PROTOCOL, DecodeLimits, FORMAT_VERSION, FormatVersion, HEADER_BYTES,
    LANGUAGE_VERSION, NO_INDEX, UNICODE_VERSION, UnicodeVersion,
};
pub use model::{
    Block, BlockIndex, BlockParameter, Capability, CompareOperator, Constant, ConstantIndex,
    Effect, Function, FunctionIndex, Instruction, IntBinaryOperator, IntUnaryOperator, IntegerSign,
    Intrinsic, Module, ModuleIndex, Package, PackageContentDigest, PackageIndex, PackageReference,
    ProgramParts, RegisterIndex, Source, SourceDigest, SourceIndex, SourceMapEntry, SourceOrigin,
    SourceSpan, StringIndex, Terminator, TypeIndex, UnverifiedProgram, ValueType,
};
