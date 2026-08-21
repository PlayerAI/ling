use crate::{
    BYTECODE_MAGIC, Block, BlockIndex, BlockParameter, BytecodeError, BytecodePhase,
    BytecodeReason, Capability, CaptureOperand, CompareOperator, Constant, ConstantIndex,
    DecodeLimits, Effect, FORMAT_VERSION_1_0, FORMAT_VERSION_1_1, FormatVersion, Function,
    FunctionIndex, FunctionKind, HEADER_BYTES, Instruction, IntBinaryOperator, IntUnaryOperator,
    IntegerSign, Intrinsic, LANGUAGE_VERSION, Module, ModuleIndex, NO_INDEX, Package,
    PackageContentDigest, PackageIndex, PackageReference, ProgramParts, RegisterIndex, Source,
    SourceDigest, SourceIndex, SourceMapEntry, SourceOrigin, SourceSpan, StringIndex, Terminator,
    TypeIndex, UNICODE_VERSION, UnverifiedProgram, ValueType,
};

/// Untrusted result of decoding one syntactically framed version-1.0 artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedProgramV1 {
    model: UnverifiedProgram,
    version: FormatVersion,
    pub(crate) offsets: DecodedOffsets,
}

impl DecodedProgramV1 {
    /// Returns the decoded model without granting execution authority.
    #[must_use]
    pub const fn model(&self) -> &UnverifiedProgram {
        &self.model
    }

    /// Returns the exact format tuple selected before table decoding.
    #[must_use]
    pub const fn version(&self) -> FormatVersion {
        self.version
    }

    pub(crate) fn into_parts(self) -> (UnverifiedProgram, FormatVersion, DecodedOffsets) {
        (self.model, self.version, self.offsets)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DecodedOffsets {
    pub type_count: u64,
    pub strings: Vec<u64>,
    pub packages: Vec<u64>,
    pub modules: Vec<u64>,
    pub types: Vec<u64>,
    pub constants: Vec<ConstantOffset>,
    pub sources: Vec<u64>,
    pub functions: Vec<FunctionOffset>,
    pub entry: u64,
    pub source_map_count: u64,
    pub source_map: Vec<u64>,
    pub executable_locations: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ConstantOffset {
    pub record: u64,
    pub declared_type: TypeIndex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FunctionOffset {
    pub record: u64,
    pub blocks: Vec<BlockOffset>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BlockOffset {
    pub record: u64,
    pub instructions: Vec<u64>,
    pub terminator: u64,
}

/// Decodes under the immutable RFC-0014 hard limits.
pub fn decode_v1(bytes: &[u8]) -> Result<DecodedProgramV1, BytecodeError> {
    decode_v1_with_limit(bytes, DecodeLimits::rfc_0014().artifact_bytes())
}

/// Decodes under a caller limit capped by the RFC-0014 artifact maximum.
pub fn decode_v1_with_limit(
    bytes: &[u8],
    artifact_byte_limit: u64,
) -> Result<DecodedProgramV1, BytecodeError> {
    decode_with_limit(bytes, artifact_byte_limit, false)
}

/// Decodes either bytecode 1.0 or 1.1 under RFC-0015 hard limits.
pub fn decode_v1_1(bytes: &[u8]) -> Result<DecodedProgramV1, BytecodeError> {
    decode_v1_1_with_limit(bytes, DecodeLimits::rfc_0015().artifact_bytes())
}

/// Decodes either supported 1.x revision under a caller-capped limit.
pub fn decode_v1_1_with_limit(
    bytes: &[u8],
    artifact_byte_limit: u64,
) -> Result<DecodedProgramV1, BytecodeError> {
    decode_with_limit(bytes, artifact_byte_limit, true)
}

fn decode_with_limit(
    bytes: &[u8],
    artifact_byte_limit: u64,
    accept_1_1: bool,
) -> Result<DecodedProgramV1, BytecodeError> {
    let maximum_limits = if accept_1_1 {
        DecodeLimits::rfc_0015()
    } else {
        DecodeLimits::rfc_0014()
    };
    let limits = maximum_limits;
    let effective_limit = artifact_byte_limit.min(limits.artifact_bytes());
    let actual = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if actual > effective_limit {
        return Err(BytecodeError::resource(
            BytecodePhase::Envelope,
            32,
            "artifact_bytes",
            actual,
            effective_limit,
        ));
    }

    let mut reader = Reader::root(bytes);
    let version = decode_header(&mut reader, actual, accept_1_1)?;
    let limits = if version == FORMAT_VERSION_1_0 {
        DecodeLimits::rfc_0014()
    } else {
        DecodeLimits::rfc_0015()
    };

    let (strings, string_offsets) = decode_strings(&mut reader, limits)?;
    let (packages, package_offsets) = decode_packages(&mut reader, limits)?;
    let (modules, module_offsets) = decode_modules(&mut reader, limits)?;
    let type_count_offset = reader.offset();
    let (types, type_offsets) = decode_types(&mut reader, limits, version)?;
    let (constants, constant_offsets) = decode_constants(&mut reader, limits)?;
    let (sources, source_offsets) = decode_sources(&mut reader, limits)?;

    let mut executable_locations = 0_u32;
    let (functions, function_offsets) =
        decode_functions(&mut reader, limits, version, &mut executable_locations)?;
    let entry_offset = reader.offset();
    let entry = FunctionIndex::new(reader.u32(BytecodePhase::Entry)?);
    let source_map_count_offset = reader.offset();
    let (source_map, source_map_offsets) = decode_source_map(&mut reader, limits)?;
    reader.ensure_finished(BytecodePhase::Envelope, BytecodeReason::TrailingBytes)?;

    Ok(DecodedProgramV1 {
        model: UnverifiedProgram::from_parts(ProgramParts {
            strings,
            packages,
            modules,
            types,
            constants,
            sources,
            functions,
            entry,
            source_map,
        }),
        version,
        offsets: DecodedOffsets {
            type_count: type_count_offset,
            strings: string_offsets,
            packages: package_offsets,
            modules: module_offsets,
            types: type_offsets,
            constants: constant_offsets,
            sources: source_offsets,
            functions: function_offsets,
            entry: entry_offset,
            source_map_count: source_map_count_offset,
            source_map: source_map_offsets,
            executable_locations,
        },
    })
}

fn decode_header(
    reader: &mut Reader<'_>,
    actual_length: u64,
    accept_1_1: bool,
) -> Result<FormatVersion, BytecodeError> {
    let magic = reader.fixed::<8>(BytecodePhase::Envelope)?;
    if magic != BYTECODE_MAGIC {
        return Err(BytecodeError::new(
            BytecodePhase::Envelope,
            BytecodeReason::InvalidMagic,
            0,
            8,
        ));
    }

    let header_length = reader.u32(BytecodePhase::Envelope)?;
    if header_length != HEADER_BYTES {
        return Err(BytecodeError::new(
            BytecodePhase::Envelope,
            BytecodeReason::InvalidHeaderLength,
            8,
            4,
        ));
    }

    let format = FormatVersion::new(
        reader.u16(BytecodePhase::Envelope)?,
        reader.u16(BytecodePhase::Envelope)?,
    );
    let supported_format =
        format == FORMAT_VERSION_1_0 || (accept_1_1 && format == FORMAT_VERSION_1_1);
    if !supported_format {
        let offset = if format.major() == 1 { 14 } else { 12 };
        return Err(BytecodeError::new(
            BytecodePhase::Envelope,
            BytecodeReason::UnsupportedVersion,
            offset,
            2,
        ));
    }

    let version_fields = [
        (
            16,
            reader.u16(BytecodePhase::Envelope)?,
            LANGUAGE_VERSION.major(),
        ),
        (
            18,
            reader.u16(BytecodePhase::Envelope)?,
            LANGUAGE_VERSION.minor(),
        ),
        (
            20,
            reader.u16(BytecodePhase::Envelope)?,
            UNICODE_VERSION.major(),
        ),
        (
            22,
            reader.u16(BytecodePhase::Envelope)?,
            UNICODE_VERSION.minor(),
        ),
        (
            24,
            reader.u16(BytecodePhase::Envelope)?,
            UNICODE_VERSION.patch(),
        ),
    ];
    if let Some((offset, _, _)) = version_fields
        .into_iter()
        .find(|(_, actual, expected)| actual != expected)
    {
        return Err(BytecodeError::new(
            BytecodePhase::Envelope,
            BytecodeReason::UnsupportedVersion,
            offset,
            2,
        ));
    }

    let reserved = reader.u16(BytecodePhase::Envelope)?;
    if reserved != 0 {
        return Err(BytecodeError::new(
            BytecodePhase::Envelope,
            BytecodeReason::ReservedNonzero,
            26,
            2,
        ));
    }
    let flags = reader.u32(BytecodePhase::Envelope)?;
    if flags != 0 {
        return Err(BytecodeError::new(
            BytecodePhase::Envelope,
            BytecodeReason::ReservedNonzero,
            28,
            4,
        ));
    }

    let declared_length = reader.u64(BytecodePhase::Envelope)?;
    if declared_length > actual_length {
        return Err(BytecodeError::new(
            BytecodePhase::Envelope,
            BytecodeReason::TruncatedArtifact,
            32,
            8,
        ));
    }
    if declared_length < actual_length {
        return Err(BytecodeError::new(
            BytecodePhase::Envelope,
            BytecodeReason::TrailingBytes,
            declared_length,
            1,
        ));
    }
    Ok(format)
}

fn decode_strings(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
) -> Result<(Vec<String>, Vec<u64>), BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Table,
        "string_entries",
        limits.string_entries(),
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(
        count,
        "string_entries",
        limits.string_entries(),
        count_offset,
    )?;
    let mut offsets = allocate_vec(
        count,
        "string_entries",
        limits.string_entries(),
        count_offset,
    )?;
    for _ in 0..count {
        let (mut record, offset, length) =
            reader.record(BytecodePhase::Table, BytecodeReason::InvalidRecordLength)?;
        if u64::from(length) > u64::from(limits.bytes_per_string_or_integer()) {
            return Err(BytecodeError::resource(
                BytecodePhase::Table,
                offset.saturating_sub(4),
                "string_bytes",
                u64::from(length),
                u64::from(limits.bytes_per_string_or_integer()),
            ));
        }
        let raw = record.remaining_bytes(BytecodePhase::Table)?;
        let text = std::str::from_utf8(raw).map_err(|error| {
            BytecodeError::new(
                BytecodePhase::Table,
                BytecodeReason::InvalidUtf8,
                offset.saturating_add(u64::try_from(error.valid_up_to()).unwrap_or(u64::MAX)),
                1,
            )
        })?;
        let mut owned = String::new();
        owned.try_reserve_exact(text.len()).map_err(|_| {
            BytecodeError::resource(
                BytecodePhase::Table,
                offset,
                "string_bytes",
                u64::try_from(text.len()).unwrap_or(u64::MAX),
                u64::from(limits.bytes_per_string_or_integer()),
            )
        })?;
        owned.push_str(text);
        values.push(owned);
        offsets.push(offset);
    }
    Ok((values, offsets))
}

fn decode_packages(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
) -> Result<(Vec<Package>, Vec<u64>), BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Table,
        "packages",
        limits.packages(),
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(count, "packages", limits.packages(), count_offset)?;
    let mut offsets = allocate_vec(count, "packages", limits.packages(), count_offset)?;
    for _ in 0..count {
        let (mut record, offset, _) =
            reader.record(BytecodePhase::Table, BytecodeReason::InvalidRecordLength)?;
        let name = StringIndex::new(record.u32(BytecodePhase::Table)?);
        let version = StringIndex::new(record.u32(BytecodePhase::Table)?);
        let digest = record.fixed::<32>(BytecodePhase::Table)?;
        record.ensure_finished(BytecodePhase::Table, BytecodeReason::InvalidRecordLength)?;
        values.push(Package {
            name,
            version,
            content_sha256: PackageContentDigest::new(digest),
        });
        offsets.push(offset);
    }
    Ok((values, offsets))
}

fn decode_modules(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
) -> Result<(Vec<Module>, Vec<u64>), BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Table,
        "modules",
        limits.modules(),
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(count, "modules", limits.modules(), count_offset)?;
    let mut offsets = allocate_vec(count, "modules", limits.modules(), count_offset)?;
    for _ in 0..count {
        let (mut record, offset, _) =
            reader.record(BytecodePhase::Table, BytecodeReason::InvalidRecordLength)?;
        let package = match record.u32(BytecodePhase::Table)? {
            NO_INDEX => PackageReference::Standalone,
            value => PackageReference::Package(PackageIndex::new(value)),
        };
        let name = StringIndex::new(record.u32(BytecodePhase::Table)?);
        let capability_count_offset = record.offset();
        let capability_count = record.bounded_count(
            BytecodePhase::Capability,
            "module_capabilities",
            limits.arguments_per_operation(),
            1,
            BytecodeReason::InvalidRecordLength,
        )?;
        let mut capabilities = allocate_vec(
            capability_count,
            "module_capabilities",
            limits.arguments_per_operation(),
            capability_count_offset,
        )?;
        for _ in 0..capability_count {
            let tag_offset = record.offset();
            let capability = match record.u8(BytecodePhase::Capability)? {
                1 => Capability::ConsoleWrite,
                _ => {
                    return Err(BytecodeError::new(
                        BytecodePhase::Capability,
                        BytecodeReason::InvalidTag,
                        tag_offset,
                        1,
                    ));
                }
            };
            capabilities.push(capability);
        }
        record.ensure_finished(BytecodePhase::Table, BytecodeReason::InvalidRecordLength)?;
        values.push(Module {
            package,
            name,
            capabilities,
        });
        offsets.push(offset);
    }
    Ok((values, offsets))
}

fn decode_types(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
    version: FormatVersion,
) -> Result<(Vec<ValueType>, Vec<u64>), BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Table,
        "types",
        limits.types(),
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(count, "types", limits.types(), count_offset)?;
    let mut offsets = allocate_vec(count, "types", limits.types(), count_offset)?;
    for _ in 0..count {
        let (mut record, offset, _) =
            reader.record(BytecodePhase::Table, BytecodeReason::InvalidRecordLength)?;
        let tag_offset = record.offset();
        let value = match record.u8(BytecodePhase::Table)? {
            0 => ValueType::Unit,
            1 => ValueType::Bool,
            2 => ValueType::Int,
            3 => ValueType::Text,
            0x10 if version == FORMAT_VERSION_1_1 => {
                record.zeroes(3, BytecodePhase::Type)?;
                let parameters = decode_type_indexes(
                    &mut record,
                    limits,
                    "function_type_parameters",
                    limits.arguments_per_operation(),
                )?;
                let result = TypeIndex::new(record.u32(BytecodePhase::Type)?);
                let effects = decode_effects(&mut record, limits)?;
                ValueType::Function {
                    parameters,
                    result,
                    effects,
                }
            }
            _ => {
                return Err(BytecodeError::new(
                    BytecodePhase::Table,
                    BytecodeReason::InvalidTag,
                    tag_offset,
                    1,
                ));
            }
        };
        record.ensure_finished(BytecodePhase::Table, BytecodeReason::InvalidRecordLength)?;
        values.push(value);
        offsets.push(offset);
    }
    Ok((values, offsets))
}

fn decode_constants(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
) -> Result<(Vec<Constant>, Vec<ConstantOffset>), BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Constant,
        "constants",
        limits.constants(),
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(count, "constants", limits.constants(), count_offset)?;
    let mut offsets = allocate_vec(count, "constants", limits.constants(), count_offset)?;
    for _ in 0..count {
        let (mut record, offset, _) =
            reader.record(BytecodePhase::Constant, BytecodeReason::InvalidRecordLength)?;
        let tag_offset = record.offset();
        let tag = record.u8(BytecodePhase::Constant)?;
        record.zeroes(3, BytecodePhase::Constant)?;
        let declared_type = TypeIndex::new(record.u32(BytecodePhase::Constant)?);
        let constant = match tag {
            0 => Constant::Unit,
            1 => {
                let value_offset = record.offset();
                let value = match record.u8(BytecodePhase::Constant)? {
                    0 => false,
                    1 => true,
                    _ => {
                        return Err(BytecodeError::new(
                            BytecodePhase::Constant,
                            BytecodeReason::InvalidBoolean,
                            value_offset,
                            1,
                        ));
                    }
                };
                Constant::Bool(value)
            }
            2 => {
                let sign_offset = record.offset();
                let sign = match record.u8(BytecodePhase::Constant)? {
                    0 => IntegerSign::Zero,
                    1 => IntegerSign::Positive,
                    2 => IntegerSign::Negative,
                    _ => {
                        return Err(BytecodeError::new(
                            BytecodePhase::Constant,
                            BytecodeReason::InvalidTag,
                            sign_offset,
                            1,
                        ));
                    }
                };
                record.zeroes(3, BytecodePhase::Constant)?;
                let magnitude_offset = record.offset();
                let length = record.bounded_count(
                    BytecodePhase::Constant,
                    "integer_magnitude_bytes",
                    limits.bytes_per_string_or_integer(),
                    1,
                    BytecodeReason::InvalidRecordLength,
                )?;
                let raw = record.take(
                    usize::try_from(length).unwrap_or(usize::MAX),
                    BytecodePhase::Constant,
                )?;
                let mut magnitude = Vec::new();
                magnitude.try_reserve_exact(raw.len()).map_err(|_| {
                    BytecodeError::resource(
                        BytecodePhase::Constant,
                        magnitude_offset,
                        "integer_magnitude_bytes",
                        u64::from(length),
                        u64::from(limits.bytes_per_string_or_integer()),
                    )
                })?;
                magnitude.extend_from_slice(raw);
                Constant::Int { sign, magnitude }
            }
            3 => Constant::Text(StringIndex::new(record.u32(BytecodePhase::Constant)?)),
            _ => {
                return Err(BytecodeError::new(
                    BytecodePhase::Constant,
                    BytecodeReason::InvalidTag,
                    tag_offset,
                    1,
                ));
            }
        };
        record.ensure_finished(BytecodePhase::Constant, BytecodeReason::InvalidRecordLength)?;
        values.push(constant);
        offsets.push(ConstantOffset {
            record: offset,
            declared_type,
        });
    }
    Ok((values, offsets))
}

fn decode_sources(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
) -> Result<(Vec<Source>, Vec<u64>), BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Table,
        "sources",
        limits.sources(),
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(count, "sources", limits.sources(), count_offset)?;
    let mut offsets = allocate_vec(count, "sources", limits.sources(), count_offset)?;
    for _ in 0..count {
        let (mut record, offset, _) =
            reader.record(BytecodePhase::Table, BytecodeReason::InvalidRecordLength)?;
        let module = ModuleIndex::new(record.u32(BytecodePhase::Table)?);
        let logical_name = StringIndex::new(record.u32(BytecodePhase::Table)?);
        let original_byte_length = record.u64(BytecodePhase::Table)?;
        let content_sha256 = SourceDigest::new(record.fixed::<32>(BytecodePhase::Table)?);
        record.ensure_finished(BytecodePhase::Table, BytecodeReason::InvalidRecordLength)?;
        values.push(Source {
            module,
            logical_name,
            original_byte_length,
            content_sha256,
        });
        offsets.push(offset);
    }
    Ok((values, offsets))
}

fn decode_functions(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
    version: FormatVersion,
    executable_locations: &mut u32,
) -> Result<(Vec<Function>, Vec<FunctionOffset>), BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Instruction,
        "functions",
        limits.functions(),
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(count, "functions", limits.functions(), count_offset)?;
    let mut offsets = allocate_vec(count, "functions", limits.functions(), count_offset)?;
    for _ in 0..count {
        let (mut record, offset, _) = reader.record(
            BytecodePhase::Instruction,
            BytecodeReason::InvalidRecordLength,
        )?;
        let (kind, capture_count) = if version == FORMAT_VERSION_1_1 {
            let kind_offset = record.offset();
            let kind = match record.u8(BytecodePhase::Instruction)? {
                0 => FunctionKind::Named,
                1 => FunctionKind::ClosureBody,
                _ => return Err(invalid_tag(BytecodePhase::Instruction, kind_offset)),
            };
            record.zeroes(3, BytecodePhase::Instruction)?;
            (kind, None)
        } else {
            (FunctionKind::Named, Some(0))
        };
        let module = ModuleIndex::new(record.u32(BytecodePhase::Instruction)?);
        let name = StringIndex::new(record.u32(BytecodePhase::Instruction)?);
        let capture_count = match capture_count {
            Some(value) => value,
            None => record.u32(BytecodePhase::Instruction)?,
        };
        let parameter_types = decode_type_indexes(
            &mut record,
            limits,
            "function_parameters",
            limits.registers_per_function(),
        )?;
        let result_type = TypeIndex::new(record.u32(BytecodePhase::Type)?);
        let effects = decode_effects(&mut record, limits)?;
        let register_offset = record.offset();
        let register_count = record.u32(BytecodePhase::Register)?;
        if register_count > limits.registers_per_function() {
            return Err(BytecodeError::resource(
                BytecodePhase::Register,
                register_offset,
                "registers_per_function",
                u64::from(register_count),
                u64::from(limits.registers_per_function()),
            ));
        }
        let (blocks, block_offsets) =
            decode_blocks(&mut record, limits, version, executable_locations)?;
        record.ensure_finished(
            BytecodePhase::Instruction,
            BytecodeReason::InvalidRecordLength,
        )?;
        values.push(Function {
            kind,
            module,
            name,
            capture_count,
            parameter_types,
            result_type,
            effects,
            register_count,
            blocks,
        });
        offsets.push(FunctionOffset {
            record: offset,
            blocks: block_offsets,
        });
    }
    Ok((values, offsets))
}

fn decode_blocks(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
    version: FormatVersion,
    executable_locations: &mut u32,
) -> Result<(Vec<Block>, Vec<BlockOffset>), BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::ControlFlow,
        "blocks_per_function",
        limits.blocks_per_function(),
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(
        count,
        "blocks_per_function",
        limits.blocks_per_function(),
        count_offset,
    )?;
    let mut offsets = allocate_vec(
        count,
        "blocks_per_function",
        limits.blocks_per_function(),
        count_offset,
    )?;
    for _ in 0..count {
        let (mut record, offset, _) = reader.record(
            BytecodePhase::ControlFlow,
            BytecodeReason::InvalidRecordLength,
        )?;
        let parameter_count_offset = record.offset();
        let parameter_count = record.bounded_count(
            BytecodePhase::Register,
            "block_parameters",
            limits.registers_per_function(),
            8,
            BytecodeReason::InvalidRecordLength,
        )?;
        let mut parameters = allocate_vec(
            parameter_count,
            "block_parameters",
            limits.registers_per_function(),
            parameter_count_offset,
        )?;
        for _ in 0..parameter_count {
            parameters.push(BlockParameter {
                register: RegisterIndex::new(record.u32(BytecodePhase::Register)?),
                value_type: TypeIndex::new(record.u32(BytecodePhase::Type)?),
            });
        }

        let instruction_count_offset = record.offset();
        let instruction_count = record.bounded_count(
            BytecodePhase::Instruction,
            "executable_locations",
            limits.executable_locations(),
            4,
            BytecodeReason::InvalidInstructionLength,
        )?;
        charge_executable_locations(
            executable_locations,
            instruction_count.saturating_add(1),
            instruction_count_offset,
            limits,
        )?;
        let mut instructions = allocate_vec(
            instruction_count,
            "executable_locations",
            limits.executable_locations(),
            instruction_count_offset,
        )?;
        let mut instruction_offsets = allocate_vec(
            instruction_count,
            "executable_locations",
            limits.executable_locations(),
            instruction_count_offset,
        )?;
        for _ in 0..instruction_count {
            let (mut instruction, instruction_offset, _) = record.record(
                BytecodePhase::Instruction,
                BytecodeReason::InvalidInstructionLength,
            )?;
            let value = decode_instruction(&mut instruction, limits, version)?;
            instruction.ensure_finished(
                BytecodePhase::Instruction,
                BytecodeReason::InvalidInstructionLength,
            )?;
            instructions.push(value);
            instruction_offsets.push(instruction_offset);
        }
        let (mut terminator, terminator_offset, _) = record.record(
            BytecodePhase::ControlFlow,
            BytecodeReason::InvalidInstructionLength,
        )?;
        let terminator_value = decode_terminator(&mut terminator, limits)?;
        terminator.ensure_finished(
            BytecodePhase::ControlFlow,
            BytecodeReason::InvalidInstructionLength,
        )?;
        record.ensure_finished(
            BytecodePhase::ControlFlow,
            BytecodeReason::InvalidRecordLength,
        )?;
        values.push(Block {
            parameters,
            instructions,
            terminator: terminator_value,
        });
        offsets.push(BlockOffset {
            record: offset,
            instructions: instruction_offsets,
            terminator: terminator_offset,
        });
    }
    Ok((values, offsets))
}

fn decode_instruction(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
    version: FormatVersion,
) -> Result<Instruction, BytecodeError> {
    let opcode_offset = reader.offset();
    let opcode = reader.u8(BytecodePhase::Instruction)?;
    reader.zeroes(3, BytecodePhase::Instruction)?;
    match opcode {
        0x01 => Ok(Instruction::Const {
            destination: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
            constant: ConstantIndex::new(reader.u32(BytecodePhase::Constant)?),
        }),
        0x02 => {
            let destination = RegisterIndex::new(reader.u32(BytecodePhase::Register)?);
            let tag_offset = reader.offset();
            let operator = match reader.u8(BytecodePhase::Instruction)? {
                0 => IntUnaryOperator::Positive,
                1 => IntUnaryOperator::Negative,
                _ => return Err(invalid_tag(BytecodePhase::Instruction, tag_offset)),
            };
            reader.zeroes(3, BytecodePhase::Instruction)?;
            Ok(Instruction::IntUnary {
                destination,
                operator,
                operand: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
            })
        }
        0x03 => {
            let destination = RegisterIndex::new(reader.u32(BytecodePhase::Register)?);
            let tag_offset = reader.offset();
            let operator = match reader.u8(BytecodePhase::Instruction)? {
                0 => IntBinaryOperator::Add,
                1 => IntBinaryOperator::Subtract,
                2 => IntBinaryOperator::Multiply,
                3 => IntBinaryOperator::Divide,
                4 => IntBinaryOperator::Remainder,
                _ => return Err(invalid_tag(BytecodePhase::Instruction, tag_offset)),
            };
            reader.zeroes(3, BytecodePhase::Instruction)?;
            Ok(Instruction::IntBinary {
                destination,
                operator,
                left: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
                right: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
            })
        }
        0x04 => {
            let destination = RegisterIndex::new(reader.u32(BytecodePhase::Register)?);
            let tag_offset = reader.offset();
            let operator = match reader.u8(BytecodePhase::Instruction)? {
                0 => CompareOperator::BoolEqual,
                1 => CompareOperator::BoolNotEqual,
                2 => CompareOperator::IntEqual,
                3 => CompareOperator::IntNotEqual,
                4 => CompareOperator::IntLess,
                5 => CompareOperator::IntLessEqual,
                6 => CompareOperator::IntGreater,
                7 => CompareOperator::IntGreaterEqual,
                8 => CompareOperator::TextEqual,
                9 => CompareOperator::TextNotEqual,
                _ => return Err(invalid_tag(BytecodePhase::Instruction, tag_offset)),
            };
            reader.zeroes(3, BytecodePhase::Instruction)?;
            Ok(Instruction::Compare {
                destination,
                operator,
                left: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
                right: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
            })
        }
        0x10 => Ok(Instruction::Call {
            destination: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
            function: FunctionIndex::new(reader.u32(BytecodePhase::Instruction)?),
            arguments: decode_registers(reader, limits)?,
        }),
        0x11 => {
            let destination = RegisterIndex::new(reader.u32(BytecodePhase::Register)?);
            let tag_offset = reader.offset();
            let intrinsic = match reader.u8(BytecodePhase::Instruction)? {
                0 => Intrinsic::TextFormat,
                1 => Intrinsic::MaxInt,
                2 => Intrinsic::MinInt,
                _ => return Err(invalid_tag(BytecodePhase::Instruction, tag_offset)),
            };
            reader.zeroes(3, BytecodePhase::Instruction)?;
            Ok(Instruction::Intrinsic {
                destination,
                intrinsic,
                arguments: decode_registers(reader, limits)?,
            })
        }
        0x12 if version == FORMAT_VERSION_1_1 => {
            let destination = RegisterIndex::new(reader.u32(BytecodePhase::Register)?);
            let function = FunctionIndex::new(reader.u32(BytecodePhase::Instruction)?);
            let count_offset = reader.offset();
            let count = reader.bounded_count(
                BytecodePhase::Register,
                "closure_captures",
                limits.arguments_per_operation(),
                8,
                BytecodeReason::InvalidInstructionLength,
            )?;
            let mut captures = allocate_vec(
                count,
                "closure_captures",
                limits.arguments_per_operation(),
                count_offset,
            )?;
            for _ in 0..count {
                let tag_offset = reader.offset();
                let tag = reader.u8(BytecodePhase::Register)?;
                reader.zeroes(3, BytecodePhase::Register)?;
                let value = reader.u32(BytecodePhase::Register)?;
                let capture = match tag {
                    0 => CaptureOperand::Register(RegisterIndex::new(value)),
                    1 if value == NO_INDEX => CaptureOperand::SelfReference,
                    1 => {
                        return Err(BytecodeError::new(
                            BytecodePhase::Register,
                            BytecodeReason::InvalidTag,
                            tag_offset,
                            8,
                        ));
                    }
                    _ => return Err(invalid_tag(BytecodePhase::Register, tag_offset)),
                };
                captures.push(capture);
            }
            Ok(Instruction::MakeClosure {
                destination,
                function,
                captures,
            })
        }
        0x13 if version == FORMAT_VERSION_1_1 => Ok(Instruction::CallClosure {
            destination: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
            callee: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
            arguments: decode_registers(reader, limits)?,
        }),
        0x20 => Ok(Instruction::ConsoleWrite {
            destination: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
            text: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
        }),
        _ => Err(BytecodeError::new(
            BytecodePhase::Instruction,
            BytecodeReason::UnknownOpcode,
            opcode_offset,
            1,
        )),
    }
}

fn decode_terminator(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
) -> Result<Terminator, BytecodeError> {
    let opcode_offset = reader.offset();
    let opcode = reader.u8(BytecodePhase::ControlFlow)?;
    reader.zeroes(3, BytecodePhase::ControlFlow)?;
    match opcode {
        0x80 => Ok(Terminator::Jump {
            target: BlockIndex::new(reader.u32(BytecodePhase::ControlFlow)?),
            arguments: decode_registers(reader, limits)?,
        }),
        0x81 => Ok(Terminator::Branch {
            condition: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
            true_target: BlockIndex::new(reader.u32(BytecodePhase::ControlFlow)?),
            true_arguments: decode_registers(reader, limits)?,
            false_target: BlockIndex::new(reader.u32(BytecodePhase::ControlFlow)?),
            false_arguments: decode_registers(reader, limits)?,
        }),
        0x82 => Ok(Terminator::Return {
            value: RegisterIndex::new(reader.u32(BytecodePhase::Register)?),
        }),
        _ => Err(BytecodeError::new(
            BytecodePhase::ControlFlow,
            BytecodeReason::UnknownOpcode,
            opcode_offset,
            1,
        )),
    }
}

fn decode_type_indexes(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
    resource: &str,
    maximum: u32,
) -> Result<Vec<TypeIndex>, BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Type,
        resource,
        maximum,
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(count, resource, maximum, count_offset)?;
    for _ in 0..count {
        values.push(TypeIndex::new(reader.u32(BytecodePhase::Type)?));
    }
    let _ = limits;
    Ok(values)
}

fn decode_effects(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
) -> Result<Vec<Effect>, BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Effect,
        "function_effects",
        limits.arguments_per_operation(),
        1,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut effects = allocate_vec(
        count,
        "function_effects",
        limits.arguments_per_operation(),
        count_offset,
    )?;
    for _ in 0..count {
        let offset = reader.offset();
        match reader.u8(BytecodePhase::Effect)? {
            1 => effects.push(Effect::ConsoleWrite),
            _ => return Err(invalid_tag(BytecodePhase::Effect, offset)),
        }
    }
    Ok(effects)
}

fn decode_registers(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
) -> Result<Vec<RegisterIndex>, BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::Register,
        "arguments_per_operation",
        limits.arguments_per_operation(),
        4,
        BytecodeReason::InvalidInstructionLength,
    )?;
    let mut values = allocate_vec(
        count,
        "arguments_per_operation",
        limits.arguments_per_operation(),
        count_offset,
    )?;
    for _ in 0..count {
        values.push(RegisterIndex::new(reader.u32(BytecodePhase::Register)?));
    }
    Ok(values)
}

fn decode_source_map(
    reader: &mut Reader<'_>,
    limits: DecodeLimits,
) -> Result<(Vec<SourceMapEntry>, Vec<u64>), BytecodeError> {
    let count_offset = reader.offset();
    let count = reader.bounded_count(
        BytecodePhase::SourceMap,
        "source_map_records",
        limits.executable_locations(),
        4,
        BytecodeReason::InvalidRecordLength,
    )?;
    let mut values = allocate_vec(
        count,
        "source_map_records",
        limits.executable_locations(),
        count_offset,
    )?;
    let mut offsets = allocate_vec(
        count,
        "source_map_records",
        limits.executable_locations(),
        count_offset,
    )?;
    for _ in 0..count {
        let (mut record, offset, _) = reader.record(
            BytecodePhase::SourceMap,
            BytecodeReason::InvalidRecordLength,
        )?;
        let function = FunctionIndex::new(record.u32(BytecodePhase::SourceMap)?);
        let block = BlockIndex::new(record.u32(BytecodePhase::SourceMap)?);
        let ordinal = record.u32(BytecodePhase::SourceMap)?;
        let source = SourceIndex::new(record.u32(BytecodePhase::SourceMap)?);
        let start_byte = record.u64(BytecodePhase::SourceMap)?;
        let end_byte = record.u64(BytecodePhase::SourceMap)?;
        let origin_offset = record.offset();
        let origin = match record.u8(BytecodePhase::SourceMap)? {
            0 => SourceOrigin::Direct,
            1 => SourceOrigin::LoweringDerived,
            _ => return Err(invalid_tag(BytecodePhase::SourceMap, origin_offset)),
        };
        record.zeroes(7, BytecodePhase::SourceMap)?;
        record.ensure_finished(
            BytecodePhase::SourceMap,
            BytecodeReason::InvalidRecordLength,
        )?;
        values.push(SourceMapEntry {
            function,
            block,
            ordinal,
            source,
            span: SourceSpan::new(start_byte, end_byte),
            origin,
        });
        offsets.push(offset);
    }
    Ok((values, offsets))
}

fn charge_executable_locations(
    total: &mut u32,
    additional: u32,
    offset: u64,
    limits: DecodeLimits,
) -> Result<(), BytecodeError> {
    let actual = total.checked_add(additional).ok_or_else(|| {
        BytecodeError::resource(
            BytecodePhase::Instruction,
            offset,
            "executable_locations",
            u64::MAX,
            u64::from(limits.executable_locations()),
        )
    })?;
    if actual > limits.executable_locations() {
        return Err(BytecodeError::resource(
            BytecodePhase::Instruction,
            offset,
            "executable_locations",
            u64::from(actual),
            u64::from(limits.executable_locations()),
        ));
    }
    *total = actual;
    Ok(())
}

fn allocate_vec<T>(
    count: u32,
    resource: &str,
    maximum: u32,
    offset: u64,
) -> Result<Vec<T>, BytecodeError> {
    let capacity = usize::try_from(count).map_err(|_| {
        BytecodeError::resource(
            BytecodePhase::Table,
            offset,
            resource,
            u64::from(count),
            u64::from(maximum),
        )
    })?;
    let mut values = Vec::new();
    values.try_reserve_exact(capacity).map_err(|_| {
        BytecodeError::resource(
            BytecodePhase::Table,
            offset,
            resource,
            u64::from(count),
            u64::from(maximum),
        )
    })?;
    Ok(values)
}

fn invalid_tag(phase: BytecodePhase, offset: u64) -> BytecodeError {
    BytecodeError::new(phase, BytecodeReason::InvalidTag, offset, 1)
}

#[derive(Clone, Copy)]
struct Reader<'a> {
    bytes: &'a [u8],
    base: u64,
    position: usize,
    boundary_reason: BytecodeReason,
}

impl<'a> Reader<'a> {
    fn root(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            base: 0,
            position: 0,
            boundary_reason: BytecodeReason::TruncatedArtifact,
        }
    }

    fn offset(&self) -> u64 {
        self.base
            .saturating_add(u64::try_from(self.position).unwrap_or(u64::MAX))
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    fn take(&mut self, length: usize, phase: BytecodePhase) -> Result<&'a [u8], BytecodeError> {
        let start = self.position;
        let end = start
            .checked_add(length)
            .ok_or_else(|| BytecodeError::new(phase, self.boundary_reason, self.offset(), 1))?;
        let value = self
            .bytes
            .get(start..end)
            .ok_or_else(|| BytecodeError::new(phase, self.boundary_reason, self.offset(), 1))?;
        self.position = end;
        Ok(value)
    }

    fn fixed<const N: usize>(&mut self, phase: BytecodePhase) -> Result<[u8; N], BytecodeError> {
        let bytes = self.take(N, phase)?;
        let mut value = [0; N];
        value.copy_from_slice(bytes);
        Ok(value)
    }

    fn u8(&mut self, phase: BytecodePhase) -> Result<u8, BytecodeError> {
        Ok(self.fixed::<1>(phase)?[0])
    }

    fn u16(&mut self, phase: BytecodePhase) -> Result<u16, BytecodeError> {
        Ok(u16::from_le_bytes(self.fixed::<2>(phase)?))
    }

    fn u32(&mut self, phase: BytecodePhase) -> Result<u32, BytecodeError> {
        Ok(u32::from_le_bytes(self.fixed::<4>(phase)?))
    }

    fn u64(&mut self, phase: BytecodePhase) -> Result<u64, BytecodeError> {
        Ok(u64::from_le_bytes(self.fixed::<8>(phase)?))
    }

    fn zeroes(&mut self, length: usize, phase: BytecodePhase) -> Result<(), BytecodeError> {
        let offset = self.offset();
        let bytes = self.take(length, phase)?;
        if let Some(index) = bytes.iter().position(|value| *value != 0) {
            return Err(BytecodeError::new(
                phase,
                BytecodeReason::ReservedNonzero,
                offset.saturating_add(u64::try_from(index).unwrap_or(u64::MAX)),
                1,
            ));
        }
        Ok(())
    }

    fn bounded_count(
        &mut self,
        phase: BytecodePhase,
        resource: &str,
        maximum: u32,
        minimum_item_bytes: usize,
        impossible_reason: BytecodeReason,
    ) -> Result<u32, BytecodeError> {
        let offset = self.offset();
        let count = self.u32(phase)?;
        if count > maximum {
            return Err(BytecodeError::resource(
                phase,
                offset,
                resource,
                u64::from(count),
                u64::from(maximum),
            ));
        }
        let count_usize = usize::try_from(count).map_err(|_| {
            BytecodeError::resource(
                phase,
                offset,
                resource,
                u64::from(count),
                u64::from(maximum),
            )
        })?;
        let minimum = count_usize.checked_mul(minimum_item_bytes).ok_or_else(|| {
            BytecodeError::resource(
                phase,
                offset,
                resource,
                u64::from(count),
                u64::from(maximum),
            )
        })?;
        if minimum > self.remaining() {
            return Err(BytecodeError::new(phase, impossible_reason, offset, 4));
        }
        Ok(count)
    }

    fn record(
        &mut self,
        phase: BytecodePhase,
        reason: BytecodeReason,
    ) -> Result<(Reader<'a>, u64, u32), BytecodeError> {
        let length_offset = self.offset();
        let length = self.u32(phase)?;
        let payload_offset = self.offset();
        let length_usize = usize::try_from(length)
            .map_err(|_| BytecodeError::new(phase, reason, length_offset, 4))?;
        if length_usize > self.remaining() {
            return Err(BytecodeError::new(phase, reason, length_offset, 4));
        }
        let payload = self.take(length_usize, phase)?;
        Ok((
            Reader {
                bytes: payload,
                base: payload_offset,
                position: 0,
                boundary_reason: reason,
            },
            payload_offset,
            length,
        ))
    }

    fn remaining_bytes(&mut self, phase: BytecodePhase) -> Result<&'a [u8], BytecodeError> {
        self.take(self.remaining(), phase)
    }

    fn ensure_finished(
        &self,
        phase: BytecodePhase,
        reason: BytecodeReason,
    ) -> Result<(), BytecodeError> {
        if self.remaining() == 0 {
            Ok(())
        } else {
            Err(BytecodeError::new(phase, reason, self.offset(), 1))
        }
    }
}
