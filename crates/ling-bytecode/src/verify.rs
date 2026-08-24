use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};

use ling_unicode::{IdentifierStatus, inspect_identifier};

use crate::decode::{DecodedOffsets, DecodedProgramV1};
use crate::path::validate_logical_name;
use crate::{
    Block, BytecodeError, BytecodePhase, BytecodeReason, Capability, CaptureOperand, Constant,
    DecodeLimits, Effect, FORMAT_VERSION_1_0, FORMAT_VERSION_1_1, FORMAT_VERSION_1_2,
    FORMAT_VERSION_1_3, FORMAT_VERSION_1_4, FormatVersion, Function, FunctionKind,
    HandlerOperation, Instruction, IntegerSign, Intrinsic, PackageReference, RegisterIndex,
    Terminator, TypeIndex, UnverifiedProgram, ValueType, decode_v1, decode_v1_1,
    decode_v1_1_with_limit, decode_v1_2, decode_v1_2_with_limit, decode_v1_3,
    decode_v1_3_with_limit, decode_v1_4, decode_v1_4_with_limit, decode_v1_with_limit,
};

/// Immutable bytecode state that has passed every RFC-0014 verifier phase.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedProgramV1 {
    model: UnverifiedProgram,
    version: FormatVersion,
    entry_console_capability_required: bool,
}

impl VerifiedProgramV1 {
    /// Returns the fully verified program model.
    #[must_use]
    pub const fn model(&self) -> &UnverifiedProgram {
        &self.model
    }

    /// Returns the exact verified wire-format revision.
    #[must_use]
    pub const fn version(&self) -> FormatVersion {
        self.version
    }

    /// Returns whether the entry's unmasked checked closure requires Console authority.
    #[must_use]
    pub const fn entry_console_capability_required(&self) -> bool {
        self.entry_console_capability_required
    }
}

/// Independently verifies a decoded, untrusted version-1.0 program.
pub fn verify_v1(decoded: DecodedProgramV1) -> Result<VerifiedProgramV1, BytecodeError> {
    let entry_console_capability_required =
        Verifier::new(decoded.model(), decoded.version(), &decoded.offsets).verify()?;
    let (model, version, _) = decoded.into_parts();
    Ok(VerifiedProgramV1 {
        model,
        version,
        entry_console_capability_required,
    })
}

/// Decodes and independently verifies one version-1.x artifact.
pub fn decode_and_verify_v1(bytes: &[u8]) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1(bytes)?)
}

/// Decodes and verifies under a caller artifact limit capped by RFC-0014.
pub fn decode_and_verify_v1_with_limit(
    bytes: &[u8],
    artifact_byte_limit: u64,
) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1_with_limit(bytes, artifact_byte_limit)?)
}

/// Decodes and independently verifies either bytecode 1.0 or 1.1.
pub fn decode_and_verify_v1_1(bytes: &[u8]) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1_1(bytes)?)
}

/// Decodes and verifies either supported 1.x revision under a caller limit.
pub fn decode_and_verify_v1_1_with_limit(
    bytes: &[u8],
    artifact_byte_limit: u64,
) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1_1_with_limit(bytes, artifact_byte_limit)?)
}

/// Decodes and independently verifies bytecode 1.0, 1.1, or 1.2.
pub fn decode_and_verify_v1_2(bytes: &[u8]) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1_2(bytes)?)
}

/// Decodes and verifies any supported 1.x revision under an RFC-0016 limit.
pub fn decode_and_verify_v1_2_with_limit(
    bytes: &[u8],
    artifact_byte_limit: u64,
) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1_2_with_limit(bytes, artifact_byte_limit)?)
}

/// Decodes and independently verifies bytecode 1.0 through 1.3.
pub fn decode_and_verify_v1_3(bytes: &[u8]) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1_3(bytes)?)
}

/// Decodes and verifies bytecode 1.0 through 1.3 under a bounded limit.
pub fn decode_and_verify_v1_3_with_limit(
    bytes: &[u8],
    artifact_byte_limit: u64,
) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1_3_with_limit(bytes, artifact_byte_limit)?)
}

/// Decodes and independently verifies bytecode 1.0 through 1.4.
pub fn decode_and_verify_v1_4(bytes: &[u8]) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1_4(bytes)?)
}

/// Decodes and verifies bytecode 1.0 through 1.4 under a bounded limit.
pub fn decode_and_verify_v1_4_with_limit(
    bytes: &[u8],
    artifact_byte_limit: u64,
) -> Result<VerifiedProgramV1, BytecodeError> {
    verify_v1(decode_v1_4_with_limit(bytes, artifact_byte_limit)?)
}

struct Verifier<'a> {
    model: &'a UnverifiedProgram,
    version: FormatVersion,
    offsets: &'a DecodedOffsets,
}

impl<'a> Verifier<'a> {
    const fn new(
        model: &'a UnverifiedProgram,
        version: FormatVersion,
        offsets: &'a DecodedOffsets,
    ) -> Self {
        Self {
            model,
            version,
            offsets,
        }
    }

    fn verify(&self) -> Result<bool, BytecodeError> {
        self.verify_tables()?;
        self.verify_function_shapes()?;
        self.verify_control_flow_and_types()?;
        let entry_console_capability_required = self.verify_effects_capabilities_and_entry()?;
        self.verify_source_map()?;
        Ok(entry_console_capability_required)
    }

    fn verify_tables(&self) -> Result<(), BytecodeError> {
        self.verify_strings()?;
        self.verify_packages()?;
        self.verify_modules()?;
        self.verify_types()?;
        self.verify_constants()?;
        self.verify_sources()
    }

    fn verify_strings(&self) -> Result<(), BytecodeError> {
        for index in 1..self.model.strings().len() {
            if self.model.strings()[index - 1].as_bytes() >= self.model.strings()[index].as_bytes()
            {
                return Err(self.table_error(
                    BytecodeReason::InvalidTableOrder,
                    offset(&self.offsets.strings, index, 40),
                    [to_u32(index - 1), to_u32(index)],
                ));
            }
        }
        Ok(())
    }

    fn verify_packages(&self) -> Result<(), BytecodeError> {
        for (index, package) in self.model.packages().iter().enumerate() {
            let record_offset = offset(&self.offsets.packages, index, 40);
            let name = self.string_at(package.name.get(), record_offset)?;
            let version = self.string_at(package.version.get(), record_offset)?;
            if !is_valid_package_name(name) || parse_package_version(version).is_none() {
                return Err(self.table_error(
                    BytecodeReason::InvalidName,
                    record_offset,
                    [to_u32(index)],
                ));
            }
            if index > 0
                && self.compare_packages(&self.model.packages()[index - 1], package)
                    != Ordering::Less
            {
                return Err(self.table_error(
                    BytecodeReason::InvalidTableOrder,
                    record_offset,
                    [to_u32(index - 1), to_u32(index)],
                ));
            }
        }
        Ok(())
    }

    fn compare_packages(&self, left: &crate::Package, right: &crate::Package) -> Ordering {
        let left_name = self
            .model
            .strings()
            .get(to_usize(left.name.get()))
            .map_or(&[][..], |value| value.as_bytes());
        let right_name = self
            .model
            .strings()
            .get(to_usize(right.name.get()))
            .map_or(&[][..], |value| value.as_bytes());
        let left_version = self
            .model
            .strings()
            .get(to_usize(left.version.get()))
            .and_then(|value| parse_package_version(value))
            .unwrap_or([0; 3]);
        let right_version = self
            .model
            .strings()
            .get(to_usize(right.version.get()))
            .and_then(|value| parse_package_version(value))
            .unwrap_or([0; 3]);
        (left_name, left_version, left.content_sha256.as_bytes()).cmp(&(
            right_name,
            right_version,
            right.content_sha256.as_bytes(),
        ))
    }

    fn verify_modules(&self) -> Result<(), BytecodeError> {
        let mut skeletons = BTreeSet::new();
        for (index, module) in self.model.modules().iter().enumerate() {
            let record_offset = offset(&self.offsets.modules, index, 40);
            if let PackageReference::Package(package) = module.package {
                self.ensure_index(
                    package.get(),
                    self.model.packages().len(),
                    BytecodePhase::Table,
                    BytecodeReason::InvalidPackageIndex,
                    record_offset,
                    [to_u32(index), package.get()],
                )?;
            }
            let name = self.string_at(module.name.get(), record_offset)?;
            if !is_valid_qualified_name(name) {
                return Err(self.table_error(
                    BytecodeReason::InvalidName,
                    record_offset,
                    [to_u32(index)],
                ));
            }
            if module
                .capabilities
                .windows(2)
                .any(|pair| pair[0] >= pair[1])
            {
                return Err(error(
                    BytecodePhase::Capability,
                    BytecodeReason::InvalidTableOrder,
                    record_offset,
                    [to_u32(index)],
                ));
            }
            let skeleton_key = (
                package_reference_key(module.package),
                ling_unicode::confusable_skeleton(name),
            );
            if !skeletons.insert(skeleton_key) {
                return Err(self.table_error(
                    BytecodeReason::InvalidName,
                    record_offset,
                    [to_u32(index)],
                ));
            }
            if index > 0 {
                let previous = &self.model.modules()[index - 1];
                let previous_name = self
                    .model
                    .strings()
                    .get(to_usize(previous.name.get()))
                    .map_or(&[][..], |value| value.as_bytes());
                if (package_reference_key(previous.package), previous_name)
                    >= (package_reference_key(module.package), name.as_bytes())
                {
                    return Err(self.table_error(
                        BytecodeReason::InvalidTableOrder,
                        record_offset,
                        [to_u32(index - 1), to_u32(index)],
                    ));
                }
            }
        }
        Ok(())
    }

    fn verify_types(&self) -> Result<(), BytecodeError> {
        let required = [
            ValueType::Unit,
            ValueType::Bool,
            ValueType::Int,
            ValueType::Text,
        ];
        let prefix_matches = self.model.types().get(..4) == Some(required.as_slice());
        let exact_1_0 = self.version == FORMAT_VERSION_1_0 && self.model.types().len() == 4;
        if !prefix_matches || (self.version == FORMAT_VERSION_1_0 && !exact_1_0) {
            let mismatch = self
                .model
                .types()
                .iter()
                .zip(&required)
                .position(|(actual, expected)| actual != expected)
                .unwrap_or(self.model.types().len().min(required.len()));
            return Err(self.table_error(
                BytecodeReason::InvalidTableOrder,
                offset(&self.offsets.types, mismatch, self.offsets.type_count),
                [to_u32(mismatch)],
            ));
        }
        if self.version >= FORMAT_VERSION_1_1 {
            for (index, value) in self.model.types().iter().enumerate().skip(4) {
                let record_offset = offset(&self.offsets.types, index, self.offsets.type_count);
                match value {
                    ValueType::Function {
                        parameters,
                        result,
                        effects,
                    } => {
                        self.verify_effect_row(effects, Some(index), record_offset)?;
                        if parameters.is_empty()
                            || parameters
                                .iter()
                                .chain(std::iter::once(result))
                                .any(|value| {
                                    to_usize(value.get()) >= index || self.is_cell_type(*value)
                                })
                        {
                            return Err(error(
                                BytecodePhase::Type,
                                BytecodeReason::InvalidTypeIndex,
                                record_offset,
                                [to_u32(index)],
                            ));
                        }
                    }
                    ValueType::Tuple { elements } if self.version >= FORMAT_VERSION_1_2 => {
                        if elements.iter().any(|value| {
                            to_usize(value.get()) >= index || self.is_cell_type(*value)
                        }) {
                            return Err(error(
                                BytecodePhase::Type,
                                BytecodeReason::InvalidTypeIndex,
                                record_offset,
                                [to_u32(index)],
                            ));
                        }
                    }
                    ValueType::Record {
                        module,
                        name,
                        arguments,
                        fields,
                    } if self.version >= FORMAT_VERSION_1_2 => {
                        self.ensure_index(
                            module.get(),
                            self.model.modules().len(),
                            BytecodePhase::Type,
                            BytecodeReason::InvalidModuleIndex,
                            record_offset,
                            [to_u32(index), module.get()],
                        )?;
                        self.ensure_type_name(*name, record_offset, index)?;
                        self.verify_type_references(arguments, index, record_offset)?;
                        if arguments.iter().any(|value| self.is_cell_type(*value)) {
                            return Err(error(
                                BytecodePhase::Type,
                                BytecodeReason::InvalidTypeIndex,
                                record_offset,
                                [to_u32(index)],
                            ));
                        }
                        let mut names = BTreeSet::new();
                        for field in fields {
                            self.ensure_type_name(field.name, record_offset, index)?;
                            if !names.insert(field.name) {
                                return Err(error(
                                    BytecodePhase::Type,
                                    BytecodeReason::InvalidTableOrder,
                                    record_offset,
                                    [to_u32(index), field.name.get()],
                                ));
                            }
                            self.verify_type_references(
                                std::slice::from_ref(&field.value_type),
                                index,
                                record_offset,
                            )?;
                            if self.is_cell_type(field.value_type) {
                                return Err(error(
                                    BytecodePhase::Type,
                                    BytecodeReason::InvalidTypeIndex,
                                    record_offset,
                                    [to_u32(index), field.value_type.get()],
                                ));
                            }
                        }
                    }
                    ValueType::Variant {
                        module,
                        name,
                        arguments,
                        cases,
                    } if self.version >= FORMAT_VERSION_1_2 => {
                        self.ensure_index(
                            module.get(),
                            self.model.modules().len(),
                            BytecodePhase::Type,
                            BytecodeReason::InvalidModuleIndex,
                            record_offset,
                            [to_u32(index), module.get()],
                        )?;
                        self.ensure_type_name(*name, record_offset, index)?;
                        self.verify_type_references(arguments, index, record_offset)?;
                        if arguments.iter().any(|value| self.is_cell_type(*value)) {
                            return Err(error(
                                BytecodePhase::Type,
                                BytecodeReason::InvalidTypeIndex,
                                record_offset,
                                [to_u32(index)],
                            ));
                        }
                        let mut names = BTreeSet::new();
                        for case in cases {
                            self.ensure_type_name(case.name, record_offset, index)?;
                            if !names.insert(case.name) {
                                return Err(error(
                                    BytecodePhase::Type,
                                    BytecodeReason::InvalidTableOrder,
                                    record_offset,
                                    [to_u32(index), case.name.get()],
                                ));
                            }
                            if let Some(payload) = case.payload {
                                self.verify_type_references(
                                    std::slice::from_ref(&payload),
                                    index,
                                    record_offset,
                                )?;
                                if self.is_cell_type(payload) {
                                    return Err(error(
                                        BytecodePhase::Type,
                                        BytecodeReason::InvalidTypeIndex,
                                        record_offset,
                                        [to_u32(index), payload.get()],
                                    ));
                                }
                            }
                        }
                    }
                    ValueType::Cell(value) if self.version >= FORMAT_VERSION_1_4 => {
                        if to_usize(value.get()) >= index || self.is_cell_type(*value) {
                            return Err(error(
                                BytecodePhase::Type,
                                BytecodeReason::InvalidTypeIndex,
                                record_offset,
                                [to_u32(index), value.get()],
                            ));
                        }
                    }
                    _ => {
                        return Err(self.table_error(
                            BytecodeReason::InvalidTableOrder,
                            record_offset,
                            [to_u32(index)],
                        ));
                    }
                }
                if self.version < FORMAT_VERSION_1_2
                    && index > 4
                    && self.model.types()[index - 1] >= *value
                {
                    return Err(self.table_error(
                        BytecodeReason::InvalidTableOrder,
                        record_offset,
                        [to_u32(index - 1), to_u32(index)],
                    ));
                }
            }
            if self.version >= FORMAT_VERSION_1_2 {
                self.verify_v1_2_type_order()?;
            }
        }
        Ok(())
    }

    fn verify_v1_2_type_order(&self) -> Result<(), BytecodeError> {
        let mut emitted = BTreeSet::new();
        emitted.extend(0..4);
        for index in 4..self.model.types().len() {
            let expected = (index..self.model.types().len())
                .filter(|candidate| {
                    type_dependencies(&self.model.types()[*candidate])
                        .iter()
                        .all(|dependency| emitted.contains(dependency))
                })
                .min_by(|left, right| self.model.types()[*left].cmp(&self.model.types()[*right]));
            if expected != Some(index) {
                return Err(self.table_error(
                    BytecodeReason::InvalidTableOrder,
                    offset(&self.offsets.types, index, self.offsets.type_count),
                    [to_u32(expected.unwrap_or(index)), to_u32(index)],
                ));
            }
            emitted.insert(index);
        }
        Ok(())
    }

    fn verify_type_references(
        &self,
        references: &[TypeIndex],
        index: usize,
        record_offset: u64,
    ) -> Result<(), BytecodeError> {
        if references
            .iter()
            .any(|value| to_usize(value.get()) >= index)
        {
            return Err(error(
                BytecodePhase::Type,
                BytecodeReason::InvalidTypeIndex,
                record_offset,
                [to_u32(index)],
            ));
        }
        Ok(())
    }

    fn verify_effect_row(
        &self,
        effects: &[Effect],
        containing_type: Option<usize>,
        record_offset: u64,
    ) -> Result<(), BytecodeError> {
        if effects.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(error(
                BytecodePhase::Effect,
                BytecodeReason::InvalidTableOrder,
                record_offset,
                [containing_type.map_or(u32::MAX, to_u32)],
            ));
        }
        for effect in effects {
            let Effect::State(value) = effect else {
                continue;
            };
            let index = to_usize(value.get());
            if self.version < FORMAT_VERSION_1_4
                || index >= self.model.types().len()
                || containing_type.is_some_and(|containing| index >= containing)
                || self.is_cell_type(*value)
            {
                return Err(error(
                    BytecodePhase::Effect,
                    BytecodeReason::InvalidTypeIndex,
                    record_offset,
                    [value.get()],
                ));
            }
        }
        Ok(())
    }

    fn is_cell_type(&self, value: TypeIndex) -> bool {
        matches!(
            self.model.types().get(to_usize(value.get())),
            Some(ValueType::Cell(_))
        )
    }

    fn ensure_type_name(
        &self,
        name: crate::StringIndex,
        record_offset: u64,
        index: usize,
    ) -> Result<(), BytecodeError> {
        let value = self.string_at_with_phase(name.get(), record_offset, BytecodePhase::Type)?;
        if !is_valid_identifier(value) {
            return Err(error(
                BytecodePhase::Type,
                BytecodeReason::InvalidName,
                record_offset,
                [to_u32(index), name.get()],
            ));
        }
        Ok(())
    }

    fn verify_constants(&self) -> Result<(), BytecodeError> {
        for (index, constant) in self.model.constants().iter().enumerate() {
            let metadata = self.offsets.constants.get(index);
            let record_offset = metadata.map_or(40, |value| value.record);
            let declared_type = metadata.map_or(u32::MAX, |value| value.declared_type.get());
            let expected_type = constant_type_index(constant);
            if declared_type != expected_type {
                return Err(error(
                    BytecodePhase::Constant,
                    BytecodeReason::InvalidTypeIndex,
                    record_offset,
                    [to_u32(index), declared_type],
                ));
            }
            match constant {
                Constant::Int { sign, magnitude } => {
                    let canonical = if magnitude.is_empty() {
                        *sign == IntegerSign::Zero
                    } else {
                        matches!(sign, IntegerSign::Positive | IntegerSign::Negative)
                            && magnitude.first().is_some_and(|byte| *byte != 0)
                    };
                    if !canonical {
                        return Err(error(
                            BytecodePhase::Constant,
                            BytecodeReason::NoncanonicalInteger,
                            record_offset,
                            [to_u32(index)],
                        ));
                    }
                }
                Constant::Text(string) => {
                    self.ensure_index(
                        string.get(),
                        self.model.strings().len(),
                        BytecodePhase::Constant,
                        BytecodeReason::InvalidStringIndex,
                        record_offset,
                        [to_u32(index), string.get()],
                    )?;
                }
                Constant::Unit | Constant::Bool(_) => {}
            }
            if index > 0
                && compare_constants(
                    &self.model.constants()[index - 1],
                    self.offsets.constants[index - 1].declared_type.get(),
                    constant,
                    declared_type,
                ) != Ordering::Less
            {
                return Err(error(
                    BytecodePhase::Constant,
                    BytecodeReason::InvalidTableOrder,
                    record_offset,
                    [to_u32(index - 1), to_u32(index)],
                ));
            }
        }
        Ok(())
    }

    fn verify_sources(&self) -> Result<(), BytecodeError> {
        for (index, source) in self.model.sources().iter().enumerate() {
            let record_offset = offset(&self.offsets.sources, index, 40);
            self.ensure_index(
                source.module.get(),
                self.model.modules().len(),
                BytecodePhase::Table,
                BytecodeReason::InvalidModuleIndex,
                record_offset,
                [to_u32(index), source.module.get()],
            )?;
            let logical_name = self.string_at(source.logical_name.get(), record_offset)?;
            if validate_logical_name(logical_name).is_err() {
                return Err(self.table_error(
                    BytecodeReason::InvalidLogicalPath,
                    record_offset,
                    [to_u32(index)],
                ));
            }
            if index > 0 {
                let previous = &self.model.sources()[index - 1];
                let previous_name = self
                    .model
                    .strings()
                    .get(to_usize(previous.logical_name.get()))
                    .map_or(&[][..], |value| value.as_bytes());
                if (previous.module.get(), previous_name)
                    >= (source.module.get(), logical_name.as_bytes())
                {
                    return Err(self.table_error(
                        BytecodeReason::InvalidTableOrder,
                        record_offset,
                        [to_u32(index - 1), to_u32(index)],
                    ));
                }
            }
        }
        Ok(())
    }

    fn verify_function_shapes(&self) -> Result<(), BytecodeError> {
        let mut skeletons = BTreeSet::new();
        for (function_index, function) in self.model.functions().iter().enumerate() {
            let function_offset = self.function_offset(function_index);
            self.ensure_index(
                function.module.get(),
                self.model.modules().len(),
                BytecodePhase::Instruction,
                BytecodeReason::InvalidModuleIndex,
                function_offset,
                [to_u32(function_index), function.module.get()],
            )?;
            let name = self.string_at_with_phase(
                function.name.get(),
                function_offset,
                BytecodePhase::Instruction,
            )?;
            let valid_name = match function.kind {
                FunctionKind::Named => is_valid_identifier(name),
                FunctionKind::ClosureBody => is_valid_closure_label(name),
            };
            if !valid_name {
                return Err(error(
                    BytecodePhase::Instruction,
                    BytecodeReason::InvalidName,
                    function_offset,
                    [to_u32(function_index)],
                ));
            }
            let skeleton_key = (
                function.module.get(),
                function.kind,
                ling_unicode::confusable_skeleton(name),
            );
            if !skeletons.insert(skeleton_key) {
                return Err(error(
                    BytecodePhase::Instruction,
                    BytecodeReason::InvalidName,
                    function_offset,
                    [to_u32(function_index)],
                ));
            }
            self.verify_type_indexes(function, function_index, function_offset)?;
            if function.capture_count > to_u32(function.parameter_types.len())
                || (function.kind == FunctionKind::Named && function.capture_count != 0)
                || (function.kind == FunctionKind::ClosureBody
                    && ((self.version < FORMAT_VERSION_1_3
                        && to_usize(function.capture_count) >= function.parameter_types.len())
                        || to_usize(function.capture_count) > function.parameter_types.len()))
                || (self.version == FORMAT_VERSION_1_0 && function.kind != FunctionKind::Named)
            {
                return Err(error(
                    BytecodePhase::Instruction,
                    BytecodeReason::InvalidBlockShape,
                    function_offset,
                    [to_u32(function_index), function.capture_count],
                ));
            }
            if function.kind == FunctionKind::ClosureBody
                && self.complete_function_type(function).is_none()
                && !(self.version >= FORMAT_VERSION_1_3
                    && to_usize(function.capture_count) == function.parameter_types.len())
            {
                return Err(error(
                    BytecodePhase::Type,
                    BytecodeReason::InvalidTypeIndex,
                    function_offset,
                    [to_u32(function_index)],
                ));
            }
            self.verify_effect_row(&function.effects, None, function_offset)?;
            if function.blocks.is_empty() {
                return Err(error(
                    BytecodePhase::ControlFlow,
                    BytecodeReason::InvalidBlockShape,
                    function_offset,
                    [to_u32(function_index)],
                ));
            }
            self.verify_entry_block_shape(function, function_index, function_offset)?;
            self.verify_block_references(function, function_index)?;

            if function_index > 0 {
                let previous = &self.model.functions()[function_index - 1];
                let previous_name = self
                    .model
                    .strings()
                    .get(to_usize(previous.name.get()))
                    .map_or(&[][..], |value| value.as_bytes());
                if (previous.module.get(), previous.kind, previous_name)
                    >= (function.module.get(), function.kind, name.as_bytes())
                {
                    return Err(error(
                        BytecodePhase::Instruction,
                        BytecodeReason::InvalidTableOrder,
                        function_offset,
                        [to_u32(function_index - 1), to_u32(function_index)],
                    ));
                }
            }
        }

        self.ensure_index(
            self.model.entry().get(),
            self.model.functions().len(),
            BytecodePhase::Entry,
            BytecodeReason::InvalidEntry,
            self.offsets.entry,
            [self.model.entry().get()],
        )
    }

    fn verify_type_indexes(
        &self,
        function: &Function,
        function_index: usize,
        function_offset: u64,
    ) -> Result<(), BytecodeError> {
        for (parameter_index, value_type) in function.parameter_types.iter().enumerate() {
            self.ensure_index(
                value_type.get(),
                self.model.types().len(),
                BytecodePhase::Type,
                BytecodeReason::InvalidTypeIndex,
                function_offset,
                [to_u32(function_index), value_type.get()],
            )?;
            if self.is_cell_type(*value_type)
                && !(self.version >= FORMAT_VERSION_1_4
                    && function.kind == FunctionKind::ClosureBody
                    && parameter_index < to_usize(function.capture_count))
            {
                return Err(error(
                    BytecodePhase::Type,
                    BytecodeReason::InvalidTypeIndex,
                    function_offset,
                    [to_u32(function_index), value_type.get()],
                ));
            }
        }
        self.ensure_index(
            function.result_type.get(),
            self.model.types().len(),
            BytecodePhase::Type,
            BytecodeReason::InvalidTypeIndex,
            function_offset,
            [to_u32(function_index), function.result_type.get()],
        )?;
        if self.is_cell_type(function.result_type) {
            return Err(error(
                BytecodePhase::Type,
                BytecodeReason::InvalidTypeIndex,
                function_offset,
                [to_u32(function_index), function.result_type.get()],
            ));
        }
        Ok(())
    }

    fn verify_entry_block_shape(
        &self,
        function: &Function,
        function_index: usize,
        function_offset: u64,
    ) -> Result<(), BytecodeError> {
        let entry = &function.blocks[0];
        if entry.parameters.len() != function.parameter_types.len()
            || entry
                .parameters
                .iter()
                .zip(&function.parameter_types)
                .any(|(parameter, expected)| parameter.value_type != *expected)
        {
            return Err(error(
                BytecodePhase::ControlFlow,
                BytecodeReason::InvalidBlockShape,
                function_offset,
                [to_u32(function_index), 0],
            ));
        }
        Ok(())
    }

    fn verify_block_references(
        &self,
        function: &Function,
        function_index: usize,
    ) -> Result<(), BytecodeError> {
        for (block_index, block) in function.blocks.iter().enumerate() {
            let block_offset = self.block_offset(function_index, block_index);
            for (parameter_index, parameter) in block.parameters.iter().enumerate() {
                self.ensure_register(
                    function,
                    parameter.register,
                    block_offset,
                    function_index,
                    block_index,
                )?;
                self.ensure_index(
                    parameter.value_type.get(),
                    self.model.types().len(),
                    BytecodePhase::Type,
                    BytecodeReason::InvalidTypeIndex,
                    block_offset,
                    [
                        to_u32(function_index),
                        to_u32(block_index),
                        parameter.value_type.get(),
                    ],
                )?;
                if self.is_cell_type(parameter.value_type)
                    && !(self.version >= FORMAT_VERSION_1_4
                        && block_index == 0
                        && function.kind == FunctionKind::ClosureBody
                        && parameter_index < to_usize(function.capture_count))
                {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidTypeIndex,
                        block_offset,
                        [
                            to_u32(function_index),
                            to_u32(block_index),
                            parameter.value_type.get(),
                        ],
                    ));
                }
            }
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                let instruction_offset =
                    self.instruction_offset(function_index, block_index, instruction_index);
                self.verify_instruction_references(
                    function,
                    instruction,
                    function_index,
                    block_index,
                    instruction_offset,
                )?;
            }
            self.verify_terminator_references(
                function,
                &block.terminator,
                function_index,
                block_index,
                self.terminator_offset(function_index, block_index),
            )?;
        }
        Ok(())
    }

    fn verify_instruction_references(
        &self,
        function: &Function,
        instruction: &Instruction,
        function_index: usize,
        block_index: usize,
        instruction_offset: u64,
    ) -> Result<(), BytecodeError> {
        self.ensure_register(
            function,
            instruction_destination(instruction),
            instruction_offset,
            function_index,
            block_index,
        )?;
        match instruction {
            Instruction::Const { constant, .. } => self.ensure_index(
                constant.get(),
                self.model.constants().len(),
                BytecodePhase::Constant,
                BytecodeReason::InvalidConstantIndex,
                instruction_offset,
                [to_u32(function_index), constant.get()],
            ),
            Instruction::IntUnary { operand, .. } => self.ensure_register(
                function,
                *operand,
                instruction_offset,
                function_index,
                block_index,
            ),
            Instruction::IntBinary { left, right, .. }
            | Instruction::Compare { left, right, .. } => {
                self.ensure_register(
                    function,
                    *left,
                    instruction_offset,
                    function_index,
                    block_index,
                )?;
                self.ensure_register(
                    function,
                    *right,
                    instruction_offset,
                    function_index,
                    block_index,
                )
            }
            Instruction::Call {
                function: callee,
                arguments,
                ..
            } => {
                self.ensure_index(
                    callee.get(),
                    self.model.functions().len(),
                    BytecodePhase::Instruction,
                    BytecodeReason::InvalidFunctionIndex,
                    instruction_offset,
                    [to_u32(function_index), callee.get()],
                )?;
                self.ensure_registers(
                    function,
                    arguments,
                    instruction_offset,
                    function_index,
                    block_index,
                )
            }
            Instruction::MakeClosure {
                function: callee,
                captures,
                ..
            } => {
                self.ensure_index(
                    callee.get(),
                    self.model.functions().len(),
                    BytecodePhase::Instruction,
                    BytecodeReason::InvalidFunctionIndex,
                    instruction_offset,
                    [to_u32(function_index), callee.get()],
                )?;
                for capture in captures {
                    if let CaptureOperand::Register(register) = capture {
                        self.ensure_register(
                            function,
                            *register,
                            instruction_offset,
                            function_index,
                            block_index,
                        )?;
                    }
                }
                Ok(())
            }
            Instruction::CallClosure {
                callee, arguments, ..
            } => {
                self.ensure_register(
                    function,
                    *callee,
                    instruction_offset,
                    function_index,
                    block_index,
                )?;
                self.ensure_registers(
                    function,
                    arguments,
                    instruction_offset,
                    function_index,
                    block_index,
                )
            }
            Instruction::Handle {
                body_function,
                body_captures,
                clauses,
                ..
            } => {
                self.ensure_index(
                    body_function.get(),
                    self.model.functions().len(),
                    BytecodePhase::Instruction,
                    BytecodeReason::InvalidFunctionIndex,
                    instruction_offset,
                    [to_u32(function_index), body_function.get()],
                )?;
                for capture in body_captures
                    .iter()
                    .chain(clauses.iter().flat_map(|clause| clause.captures.iter()))
                {
                    if let CaptureOperand::Register(register) = capture {
                        self.ensure_register(
                            function,
                            *register,
                            instruction_offset,
                            function_index,
                            block_index,
                        )?;
                    }
                }
                let mut previous = None;
                for clause in clauses {
                    self.ensure_index(
                        clause.function.get(),
                        self.model.functions().len(),
                        BytecodePhase::Instruction,
                        BytecodeReason::InvalidFunctionIndex,
                        instruction_offset,
                        [to_u32(function_index), clause.function.get()],
                    )?;
                    if previous.is_some_and(|tag| tag >= clause.operation.tag()) {
                        return Err(error(
                            BytecodePhase::Instruction,
                            BytecodeReason::InvalidTableOrder,
                            instruction_offset,
                            [to_u32(function_index), clause.operation.tag().into()],
                        ));
                    }
                    previous = Some(clause.operation.tag());
                }
                Ok(())
            }
            Instruction::Intrinsic { arguments, .. } => self.ensure_registers(
                function,
                arguments,
                instruction_offset,
                function_index,
                block_index,
            ),
            Instruction::MakeTuple {
                tuple, elements, ..
            } => {
                self.ensure_index(
                    tuple.get(),
                    self.model.types().len(),
                    BytecodePhase::Type,
                    BytecodeReason::InvalidTypeIndex,
                    instruction_offset,
                    [to_u32(function_index), tuple.get()],
                )?;
                self.ensure_registers(
                    function,
                    elements,
                    instruction_offset,
                    function_index,
                    block_index,
                )
            }
            Instruction::MakeRecord { record, fields, .. } => {
                self.ensure_index(
                    record.get(),
                    self.model.types().len(),
                    BytecodePhase::Type,
                    BytecodeReason::InvalidTypeIndex,
                    instruction_offset,
                    [to_u32(function_index), record.get()],
                )?;
                self.ensure_registers(
                    function,
                    fields,
                    instruction_offset,
                    function_index,
                    block_index,
                )
            }
            Instruction::GetTuple { tuple, .. } | Instruction::GetField { record: tuple, .. } => {
                self.ensure_register(
                    function,
                    *tuple,
                    instruction_offset,
                    function_index,
                    block_index,
                )
            }
            Instruction::UpdateRecord { base, updates, .. } => {
                self.ensure_register(
                    function,
                    *base,
                    instruction_offset,
                    function_index,
                    block_index,
                )?;
                for update in updates {
                    self.ensure_register(
                        function,
                        update.value,
                        instruction_offset,
                        function_index,
                        block_index,
                    )?;
                }
                Ok(())
            }
            Instruction::MakeVariant {
                variant, payload, ..
            } => {
                self.ensure_index(
                    variant.get(),
                    self.model.types().len(),
                    BytecodePhase::Type,
                    BytecodeReason::InvalidTypeIndex,
                    instruction_offset,
                    [to_u32(function_index), variant.get()],
                )?;
                payload.map_or(Ok(()), |payload| {
                    self.ensure_register(
                        function,
                        payload,
                        instruction_offset,
                        function_index,
                        block_index,
                    )
                })
            }
            Instruction::VariantIs { variant, .. }
            | Instruction::GetVariantPayload { variant, .. } => self.ensure_register(
                function,
                *variant,
                instruction_offset,
                function_index,
                block_index,
            ),
            Instruction::CellNew { initial, .. } => self.ensure_register(
                function,
                *initial,
                instruction_offset,
                function_index,
                block_index,
            ),
            Instruction::CellGet { cell, .. } => self.ensure_register(
                function,
                *cell,
                instruction_offset,
                function_index,
                block_index,
            ),
            Instruction::CellSet { cell, value, .. } => {
                self.ensure_register(
                    function,
                    *cell,
                    instruction_offset,
                    function_index,
                    block_index,
                )?;
                self.ensure_register(
                    function,
                    *value,
                    instruction_offset,
                    function_index,
                    block_index,
                )
            }
            Instruction::ConsoleWrite { text, .. } => self.ensure_register(
                function,
                *text,
                instruction_offset,
                function_index,
                block_index,
            ),
        }
    }

    fn verify_terminator_references(
        &self,
        function: &Function,
        terminator: &Terminator,
        function_index: usize,
        block_index: usize,
        terminator_offset: u64,
    ) -> Result<(), BytecodeError> {
        match terminator {
            Terminator::Jump { target, arguments } => {
                self.ensure_block(
                    function,
                    target.get(),
                    terminator_offset,
                    function_index,
                    block_index,
                )?;
                self.ensure_registers(
                    function,
                    arguments,
                    terminator_offset,
                    function_index,
                    block_index,
                )
            }
            Terminator::Branch {
                condition,
                true_target,
                true_arguments,
                false_target,
                false_arguments,
            } => {
                self.ensure_register(
                    function,
                    *condition,
                    terminator_offset,
                    function_index,
                    block_index,
                )?;
                self.ensure_block(
                    function,
                    true_target.get(),
                    terminator_offset,
                    function_index,
                    block_index,
                )?;
                self.ensure_registers(
                    function,
                    true_arguments,
                    terminator_offset,
                    function_index,
                    block_index,
                )?;
                self.ensure_block(
                    function,
                    false_target.get(),
                    terminator_offset,
                    function_index,
                    block_index,
                )?;
                self.ensure_registers(
                    function,
                    false_arguments,
                    terminator_offset,
                    function_index,
                    block_index,
                )
            }
            Terminator::Return { value } => self.ensure_register(
                function,
                *value,
                terminator_offset,
                function_index,
                block_index,
            ),
        }
    }

    fn verify_control_flow_and_types(&self) -> Result<(), BytecodeError> {
        for (function_index, function) in self.model.functions().iter().enumerate() {
            self.verify_one_function(function, function_index)?;
        }
        Ok(())
    }

    fn verify_one_function(
        &self,
        function: &Function,
        function_index: usize,
    ) -> Result<(), BytecodeError> {
        let graph = self.build_cfg(function, function_index)?;
        self.ensure_reachable(&graph.successors, function_index)?;
        let dominators = Dominators::compute(
            &graph.successors,
            &graph.predecessors,
            self.function_offset(function_index),
            function_index,
        )?;
        let definitions = self.collect_definitions(function, function_index)?;
        self.verify_uses(function, function_index, &definitions, &dominators)
    }

    fn build_cfg(
        &self,
        function: &Function,
        function_index: usize,
    ) -> Result<ControlFlowGraph, BytecodeError> {
        let mut successors = vec![Vec::new(); function.blocks.len()];
        let mut predecessors = vec![Vec::new(); function.blocks.len()];
        for (block_index, block) in function.blocks.iter().enumerate() {
            let targets = terminator_targets(&block.terminator);
            for target in targets {
                let target = to_usize(target);
                if target >= function.blocks.len() {
                    return Err(error(
                        BytecodePhase::ControlFlow,
                        BytecodeReason::InvalidBlockIndex,
                        self.terminator_offset(function_index, block_index),
                        [to_u32(function_index), to_u32(block_index), to_u32(target)],
                    ));
                }
                push_fallible(
                    &mut successors[block_index],
                    target,
                    self.terminator_offset(function_index, block_index),
                )?;
                push_fallible(
                    &mut predecessors[target],
                    block_index,
                    self.terminator_offset(function_index, block_index),
                )?;
            }
        }
        Ok(ControlFlowGraph {
            successors,
            predecessors,
        })
    }

    fn ensure_reachable(
        &self,
        successors: &[Vec<usize>],
        function_index: usize,
    ) -> Result<(), BytecodeError> {
        let mut reached = vec![false; successors.len()];
        let mut pending = Vec::new();
        pending.push(0);
        reached[0] = true;
        while let Some(block) = pending.pop() {
            for successor in successors[block].iter().rev() {
                if !reached[*successor] {
                    reached[*successor] = true;
                    pending.push(*successor);
                }
            }
        }
        if let Some(block) = reached.iter().position(|value| !*value) {
            return Err(error(
                BytecodePhase::ControlFlow,
                BytecodeReason::UnreachableBlock,
                self.block_offset(function_index, block),
                [to_u32(function_index), to_u32(block)],
            ));
        }
        Ok(())
    }

    fn collect_definitions(
        &self,
        function: &Function,
        function_index: usize,
    ) -> Result<Vec<Definition>, BytecodeError> {
        let mut definitions = vec![None; to_usize(function.register_count)];
        for (block_index, block) in function.blocks.iter().enumerate() {
            for parameter in &block.parameters {
                self.define_register(
                    &mut definitions,
                    parameter.register,
                    DefinitionLocation::BlockParameter,
                    parameter.value_type,
                    function_index,
                    block_index,
                    self.block_offset(function_index, block_index),
                )?;
            }
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                let value_type = self
                    .instruction_result_type(instruction, &definitions)
                    .unwrap_or(TypeIndex::new(u32::MAX));
                self.define_register(
                    &mut definitions,
                    instruction_destination(instruction),
                    DefinitionLocation::Instruction(instruction_index),
                    value_type,
                    function_index,
                    block_index,
                    self.instruction_offset(function_index, block_index, instruction_index),
                )?;
            }
        }
        if let Some(register) = definitions.iter().position(Option::is_none) {
            return Err(error(
                BytecodePhase::Register,
                BytecodeReason::InvalidRegisterType,
                self.function_offset(function_index),
                [to_u32(function_index), to_u32(register)],
            ));
        }
        let mut changed = true;
        while changed {
            changed = false;
            for block in &function.blocks {
                for instruction in &block.instructions {
                    let Instruction::CallClosure { destination, .. } = instruction else {
                        continue;
                    };
                    let index = to_usize(destination.get());
                    if definitions[index]
                        .as_ref()
                        .is_some_and(|definition| definition.value_type.get() != u32::MAX)
                    {
                        continue;
                    }
                    if let Some(value_type) =
                        self.instruction_result_type(instruction, &definitions)
                    {
                        if let Some(definition) = definitions[index].as_mut() {
                            definition.value_type = value_type;
                        }
                        changed = true;
                    }
                }
            }
        }
        if let Some(register) = definitions.iter().position(|definition| {
            definition
                .as_ref()
                .is_some_and(|definition| definition.value_type.get() == u32::MAX)
        }) {
            return Err(error(
                BytecodePhase::Type,
                BytecodeReason::InvalidRegisterType,
                self.function_offset(function_index),
                [to_u32(function_index), to_u32(register)],
            ));
        }
        Ok(definitions.into_iter().flatten().collect())
    }

    #[allow(clippy::too_many_arguments)]
    fn define_register(
        &self,
        definitions: &mut [Option<Definition>],
        register: RegisterIndex,
        location: DefinitionLocation,
        value_type: TypeIndex,
        function_index: usize,
        block_index: usize,
        definition_offset: u64,
    ) -> Result<(), BytecodeError> {
        let Some(slot) = definitions.get_mut(to_usize(register.get())) else {
            return Err(error(
                BytecodePhase::Register,
                BytecodeReason::InvalidRegisterIndex,
                definition_offset,
                [to_u32(function_index), register.get()],
            ));
        };
        if slot.is_some() {
            return Err(error(
                BytecodePhase::Register,
                BytecodeReason::DuplicateRegisterDefinition,
                definition_offset,
                [to_u32(function_index), to_u32(block_index), register.get()],
            ));
        }
        *slot = Some(Definition {
            block: block_index,
            location,
            value_type,
        });
        Ok(())
    }

    fn verify_uses(
        &self,
        function: &Function,
        function_index: usize,
        definitions: &[Definition],
        dominators: &Dominators,
    ) -> Result<(), BytecodeError> {
        for (block_index, block) in function.blocks.iter().enumerate() {
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                self.verify_instruction_uses(
                    function,
                    instruction,
                    function_index,
                    block_index,
                    instruction_index,
                    definitions,
                    dominators,
                )?;
            }
            self.verify_terminator_uses(
                function,
                block,
                function_index,
                block_index,
                definitions,
                dominators,
            )?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn verify_instruction_uses(
        &self,
        function: &Function,
        instruction: &Instruction,
        function_index: usize,
        block_index: usize,
        instruction_index: usize,
        definitions: &[Definition],
        dominators: &Dominators,
    ) -> Result<(), BytecodeError> {
        let instruction_offset =
            self.instruction_offset(function_index, block_index, instruction_index);
        let check = |register, expected, reason| {
            check_use(
                definitions,
                dominators,
                register,
                expected,
                block_index,
                instruction_index,
                instruction_offset,
                reason,
                function_index,
            )
        };
        match instruction {
            Instruction::Const { .. } => Ok(()),
            Instruction::IntUnary { operand, .. } => check(
                *operand,
                TypeIndex::new(2),
                BytecodeReason::InvalidRegisterType,
            ),
            Instruction::IntBinary { left, right, .. } => {
                check(
                    *left,
                    TypeIndex::new(2),
                    BytecodeReason::InvalidRegisterType,
                )?;
                check(
                    *right,
                    TypeIndex::new(2),
                    BytecodeReason::InvalidRegisterType,
                )
            }
            Instruction::Compare {
                operator,
                left,
                right,
                ..
            } => {
                check(
                    *left,
                    scalar_type_index(operator.operand_type()),
                    BytecodeReason::InvalidRegisterType,
                )?;
                check(
                    *right,
                    scalar_type_index(operator.operand_type()),
                    BytecodeReason::InvalidRegisterType,
                )
            }
            Instruction::Call {
                function: callee,
                arguments,
                ..
            } => {
                let callee = &self.model.functions()[to_usize(callee.get())];
                if callee.kind != FunctionKind::Named
                    || arguments.len() != callee.parameter_types.len()
                {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::CallSignatureMismatch,
                        instruction_offset,
                        [to_u32(function_index), callee.name.get()],
                    ));
                }
                for (argument, expected) in arguments.iter().zip(&callee.parameter_types) {
                    check(*argument, *expected, BytecodeReason::CallSignatureMismatch)?;
                }
                Ok(())
            }
            Instruction::MakeClosure {
                function: target,
                captures,
                ..
            } => {
                let target = &self.model.functions()[to_usize(target.get())];
                let Some(closure_type) = self.complete_function_type(target) else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidTypeIndex,
                        instruction_offset,
                        [to_u32(function_index)],
                    ));
                };
                if captures.len() != to_usize(target.capture_count)
                    || (target.kind == FunctionKind::Named && !captures.is_empty())
                {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::CallSignatureMismatch,
                        instruction_offset,
                        [to_u32(function_index), target.capture_count],
                    ));
                }
                let mut self_seen = false;
                for (capture, expected) in captures.iter().zip(&target.parameter_types) {
                    match capture {
                        CaptureOperand::Register(register) => {
                            check(*register, *expected, BytecodeReason::CallSignatureMismatch)?
                        }
                        CaptureOperand::SelfReference
                            if !self_seen
                                && target.kind == FunctionKind::ClosureBody
                                && *expected == closure_type =>
                        {
                            self_seen = true;
                        }
                        CaptureOperand::SelfReference => {
                            return Err(error(
                                BytecodePhase::Type,
                                BytecodeReason::CallSignatureMismatch,
                                instruction_offset,
                                [to_u32(function_index), expected.get()],
                            ));
                        }
                    }
                }
                Ok(())
            }
            Instruction::CallClosure {
                callee, arguments, ..
            } => {
                let callee_type = definitions[to_usize(callee.get())].value_type;
                let Some((parameters, _, _)) = self.function_type(callee_type) else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::CallSignatureMismatch,
                        instruction_offset,
                        [to_u32(function_index), callee_type.get()],
                    ));
                };
                if arguments.is_empty() || arguments.len() > parameters.len() {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::CallSignatureMismatch,
                        instruction_offset,
                        [to_u32(function_index), to_u32(arguments.len())],
                    ));
                }
                check(*callee, callee_type, BytecodeReason::CallSignatureMismatch)?;
                for (argument, expected) in arguments.iter().zip(parameters) {
                    check(*argument, *expected, BytecodeReason::CallSignatureMismatch)?;
                }
                Ok(())
            }
            Instruction::Intrinsic {
                intrinsic,
                arguments,
                ..
            } => {
                let expected = intrinsic_parameters(*intrinsic);
                if arguments.len() != expected.len() {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::CallSignatureMismatch,
                        instruction_offset,
                        [to_u32(function_index), intrinsic.tag().into()],
                    ));
                }
                for (argument, expected) in arguments.iter().zip(expected) {
                    check(
                        *argument,
                        scalar_type_index(expected.clone()),
                        BytecodeReason::CallSignatureMismatch,
                    )?;
                }
                Ok(())
            }
            Instruction::MakeTuple {
                destination,
                tuple,
                elements,
            } => {
                if definitions[to_usize(destination.get())].value_type != *tuple {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), tuple.get()],
                    ));
                }
                let ValueType::Tuple {
                    elements: ref expected,
                } = self.model.types()[to_usize(tuple.get())]
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), tuple.get()],
                    ));
                };
                if expected.len() != elements.len() {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::CallSignatureMismatch,
                        instruction_offset,
                        [to_u32(function_index), to_u32(elements.len())],
                    ));
                }
                for (register, expected) in elements.iter().zip(expected) {
                    check(*register, *expected, BytecodeReason::CallSignatureMismatch)?;
                }
                Ok(())
            }
            Instruction::GetTuple {
                destination,
                tuple,
                element,
            } => {
                let tuple_type = definitions[to_usize(tuple.get())].value_type;
                let ValueType::Tuple { ref elements } =
                    self.model.types()[to_usize(tuple_type.get())]
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), tuple_type.get()],
                    ));
                };
                let Some(expected) = elements.get(to_usize(*element)).copied() else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidTypeIndex,
                        instruction_offset,
                        [to_u32(function_index), *element],
                    ));
                };
                if definitions[to_usize(destination.get())].value_type != expected {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), destination.get()],
                    ));
                }
                check(*tuple, tuple_type, BytecodeReason::InvalidRegisterType)
            }
            Instruction::MakeRecord {
                destination,
                record,
                fields,
            } => {
                if definitions[to_usize(destination.get())].value_type != *record {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), record.get()],
                    ));
                }
                let ValueType::Record {
                    fields: ref expected,
                    ..
                } = self.model.types()[to_usize(record.get())]
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), record.get()],
                    ));
                };
                if expected.len() != fields.len() {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::CallSignatureMismatch,
                        instruction_offset,
                        [to_u32(function_index), to_u32(fields.len())],
                    ));
                }
                for (register, expected) in fields.iter().zip(expected) {
                    check(
                        *register,
                        expected.value_type,
                        BytecodeReason::CallSignatureMismatch,
                    )?;
                }
                Ok(())
            }
            Instruction::GetField {
                destination,
                record,
                field,
            } => {
                let record_type = definitions[to_usize(record.get())].value_type;
                let ValueType::Record { ref fields, .. } =
                    self.model.types()[to_usize(record_type.get())]
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), record_type.get()],
                    ));
                };
                let Some(expected) = fields.get(to_usize(*field)).map(|field| field.value_type)
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidTypeIndex,
                        instruction_offset,
                        [to_u32(function_index), *field],
                    ));
                };
                if definitions[to_usize(destination.get())].value_type != expected {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), destination.get()],
                    ));
                }
                check(*record, record_type, BytecodeReason::InvalidRegisterType)
            }
            Instruction::UpdateRecord {
                destination,
                base,
                updates,
            } => {
                let record_type = definitions[to_usize(base.get())].value_type;
                let ValueType::Record { ref fields, .. } =
                    self.model.types()[to_usize(record_type.get())]
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), record_type.get()],
                    ));
                };
                let mut seen = BTreeSet::new();
                for update in updates {
                    let Some(field) = fields.get(to_usize(update.field)) else {
                        return Err(error(
                            BytecodePhase::Type,
                            BytecodeReason::InvalidTypeIndex,
                            instruction_offset,
                            [to_u32(function_index), update.field],
                        ));
                    };
                    if !seen.insert(update.field) {
                        return Err(error(
                            BytecodePhase::Type,
                            BytecodeReason::CallSignatureMismatch,
                            instruction_offset,
                            [to_u32(function_index), update.field],
                        ));
                    }
                    check(
                        update.value,
                        field.value_type,
                        BytecodeReason::CallSignatureMismatch,
                    )?;
                }
                if definitions[to_usize(destination.get())].value_type != record_type {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), destination.get()],
                    ));
                }
                check(*base, record_type, BytecodeReason::InvalidRegisterType)
            }
            Instruction::MakeVariant {
                destination,
                variant,
                case,
                payload,
            } => {
                if definitions[to_usize(destination.get())].value_type != *variant {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), variant.get()],
                    ));
                }
                let ValueType::Variant { ref cases, .. } =
                    self.model.types()[to_usize(variant.get())]
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), variant.get()],
                    ));
                };
                let Some(case_info) = cases.get(to_usize(*case)) else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidTypeIndex,
                        instruction_offset,
                        [to_u32(function_index), *case],
                    ));
                };
                match (case_info.payload, payload) {
                    (Some(expected), Some(register)) => {
                        check(*register, expected, BytecodeReason::CallSignatureMismatch)?;
                    }
                    (None, None) => {}
                    _ => {
                        return Err(error(
                            BytecodePhase::Type,
                            BytecodeReason::CallSignatureMismatch,
                            instruction_offset,
                            [to_u32(function_index), *case],
                        ));
                    }
                }
                Ok(())
            }
            Instruction::VariantIs {
                destination,
                variant,
                case,
            } => {
                let variant_type = definitions[to_usize(variant.get())].value_type;
                let ValueType::Variant { ref cases, .. } =
                    self.model.types()[to_usize(variant_type.get())]
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), variant_type.get()],
                    ));
                };
                if to_usize(*case) >= cases.len()
                    || definitions[to_usize(destination.get())].value_type != TypeIndex::new(1)
                {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidTypeIndex,
                        instruction_offset,
                        [to_u32(function_index), *case],
                    ));
                }
                check(*variant, variant_type, BytecodeReason::InvalidRegisterType)
            }
            Instruction::GetVariantPayload {
                destination,
                variant,
                case,
            } => {
                let variant_type = definitions[to_usize(variant.get())].value_type;
                let ValueType::Variant { ref cases, .. } =
                    self.model.types()[to_usize(variant_type.get())]
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), variant_type.get()],
                    ));
                };
                let Some(expected) = cases.get(to_usize(*case)).and_then(|case| case.payload)
                else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidTypeIndex,
                        instruction_offset,
                        [to_u32(function_index), *case],
                    ));
                };
                if definitions[to_usize(destination.get())].value_type != expected {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), destination.get()],
                    ));
                }
                check(*variant, variant_type, BytecodeReason::InvalidRegisterType)
            }
            Instruction::Handle {
                destination,
                body_function,
                body_captures,
                clauses,
            } => {
                let body = &self.model.functions()[to_usize(body_function.get())];
                let result_type = definitions[to_usize(destination.get())].value_type;
                if self.version < FORMAT_VERSION_1_3
                    || body.kind != FunctionKind::ClosureBody
                    || body.module != function.module
                    || body.capture_count != to_u32(body_captures.len())
                    || to_usize(body.capture_count) != body.parameter_types.len()
                    || body.result_type != result_type
                    || clauses.is_empty()
                {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::CallSignatureMismatch,
                        instruction_offset,
                        [to_u32(function_index), body_function.get()],
                    ));
                }
                for (capture, expected) in body_captures.iter().zip(&body.parameter_types) {
                    match capture {
                        CaptureOperand::Register(register) => {
                            check(*register, *expected, BytecodeReason::CallSignatureMismatch)?;
                        }
                        CaptureOperand::SelfReference => {
                            return Err(error(
                                BytecodePhase::Type,
                                BytecodeReason::CallSignatureMismatch,
                                instruction_offset,
                                [to_u32(function_index), expected.get()],
                            ));
                        }
                    }
                }
                let handles_console = clauses
                    .iter()
                    .any(|clause| clause.operation == HandlerOperation::ConsoleWrite);
                let resume_effects = body
                    .effects
                    .iter()
                    .copied()
                    .filter(|effect| !(handles_console && *effect == Effect::ConsoleWrite))
                    .collect::<Vec<_>>();
                for clause in clauses {
                    let target = &self.model.functions()[to_usize(clause.function.get())];
                    let (inputs, output) = match clause.operation {
                        HandlerOperation::ConsoleWrite => (&[3_u32][..], 0_u32),
                        HandlerOperation::ClockNow => (&[][..], 2_u32),
                        HandlerOperation::RandomNext => (&[2_u32][..], 2_u32),
                    };
                    let capture_count = to_usize(target.capture_count);
                    let source_parameters = target.parameter_types.get(capture_count..);
                    let expected_parameter_count =
                        inputs.len() + usize::from(clause.resume_present);
                    if target.kind != FunctionKind::ClosureBody
                        || target.module != function.module
                        || target.capture_count != to_u32(clause.captures.len())
                        || target.result_type != result_type
                        || source_parameters.is_none_or(|parameters| {
                            parameters.len() != expected_parameter_count
                                || parameters
                                    .iter()
                                    .zip(inputs)
                                    .any(|(actual, expected)| actual.get() != *expected)
                        })
                    {
                        return Err(error(
                            BytecodePhase::Type,
                            BytecodeReason::CallSignatureMismatch,
                            instruction_offset,
                            [to_u32(function_index), clause.function.get()],
                        ));
                    }
                    if clause.resume_present {
                        let resume_type = source_parameters
                            .and_then(|parameters| parameters.last())
                            .copied();
                        let expected_resume = self.find_function_type(
                            &[TypeIndex::new(output)],
                            result_type,
                            &resume_effects,
                        );
                        if resume_type != expected_resume {
                            return Err(error(
                                BytecodePhase::Type,
                                BytecodeReason::CallSignatureMismatch,
                                instruction_offset,
                                [to_u32(function_index), clause.function.get()],
                            ));
                        }
                    }
                    for (capture, expected) in clause.captures.iter().zip(&target.parameter_types) {
                        match capture {
                            CaptureOperand::Register(register) => {
                                check(*register, *expected, BytecodeReason::CallSignatureMismatch)?;
                            }
                            CaptureOperand::SelfReference => {
                                return Err(error(
                                    BytecodePhase::Type,
                                    BytecodeReason::CallSignatureMismatch,
                                    instruction_offset,
                                    [to_u32(function_index), expected.get()],
                                ));
                            }
                        }
                    }
                }
                Ok(())
            }
            Instruction::CellNew {
                destination,
                initial,
            } => {
                let initial_type = definitions[to_usize(initial.get())].value_type;
                let destination_type = definitions[to_usize(destination.get())].value_type;
                if self.version < FORMAT_VERSION_1_4
                    || self.find_cell_type(initial_type) != Some(destination_type)
                {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), destination.get(), initial.get()],
                    ));
                }
                check(*initial, initial_type, BytecodeReason::InvalidRegisterType)
            }
            Instruction::CellGet { destination, cell } => {
                let cell_type = definitions[to_usize(cell.get())].value_type;
                let Some(value_type) = self.cell_value_type(cell_type) else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), cell.get()],
                    ));
                };
                if self.version < FORMAT_VERSION_1_4
                    || definitions[to_usize(destination.get())].value_type != value_type
                {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), destination.get()],
                    ));
                }
                check(*cell, cell_type, BytecodeReason::InvalidRegisterType)
            }
            Instruction::CellSet {
                destination,
                cell,
                value,
            } => {
                let cell_type = definitions[to_usize(cell.get())].value_type;
                let Some(value_type) = self.cell_value_type(cell_type) else {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), cell.get()],
                    ));
                };
                if self.version < FORMAT_VERSION_1_4
                    || definitions[to_usize(destination.get())].value_type != TypeIndex::new(0)
                {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::InvalidRegisterType,
                        instruction_offset,
                        [to_u32(function_index), destination.get()],
                    ));
                }
                check(*cell, cell_type, BytecodeReason::InvalidRegisterType)?;
                check(*value, value_type, BytecodeReason::InvalidRegisterType)
            }
            Instruction::ConsoleWrite { text, .. } => check(
                *text,
                TypeIndex::new(3),
                BytecodeReason::InvalidRegisterType,
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn verify_terminator_uses(
        &self,
        function: &Function,
        block: &Block,
        function_index: usize,
        block_index: usize,
        definitions: &[Definition],
        dominators: &Dominators,
    ) -> Result<(), BytecodeError> {
        let use_ordinal = block.instructions.len();
        let terminator_offset = self.terminator_offset(function_index, block_index);
        let check = |register, expected, reason| {
            check_use(
                definitions,
                dominators,
                register,
                expected,
                block_index,
                use_ordinal,
                terminator_offset,
                reason,
                function_index,
            )
        };
        match &block.terminator {
            Terminator::Jump { target, arguments } => self.verify_edge_arguments(
                function,
                target.get(),
                arguments,
                function_index,
                block_index,
                use_ordinal,
                terminator_offset,
                definitions,
                dominators,
            ),
            Terminator::Branch {
                condition,
                true_target,
                true_arguments,
                false_target,
                false_arguments,
            } => {
                check(
                    *condition,
                    TypeIndex::new(1),
                    BytecodeReason::InvalidRegisterType,
                )?;
                self.verify_edge_arguments(
                    function,
                    true_target.get(),
                    true_arguments,
                    function_index,
                    block_index,
                    use_ordinal,
                    terminator_offset,
                    definitions,
                    dominators,
                )?;
                self.verify_edge_arguments(
                    function,
                    false_target.get(),
                    false_arguments,
                    function_index,
                    block_index,
                    use_ordinal,
                    terminator_offset,
                    definitions,
                    dominators,
                )
            }
            Terminator::Return { value } => check(
                *value,
                function.result_type,
                BytecodeReason::InvalidReturnType,
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn verify_edge_arguments(
        &self,
        function: &Function,
        target: u32,
        arguments: &[RegisterIndex],
        function_index: usize,
        block_index: usize,
        use_ordinal: usize,
        terminator_offset: u64,
        definitions: &[Definition],
        dominators: &Dominators,
    ) -> Result<(), BytecodeError> {
        let parameters = &function.blocks[to_usize(target)].parameters;
        if arguments.len() != parameters.len() {
            return Err(error(
                BytecodePhase::Type,
                BytecodeReason::BlockArgumentTypeMismatch,
                terminator_offset,
                [to_u32(function_index), to_u32(block_index), target],
            ));
        }
        for (argument, parameter) in arguments.iter().zip(parameters) {
            check_use(
                definitions,
                dominators,
                *argument,
                parameter.value_type,
                block_index,
                use_ordinal,
                terminator_offset,
                BytecodeReason::BlockArgumentTypeMismatch,
                function_index,
            )?;
        }
        Ok(())
    }

    fn verify_effects_capabilities_and_entry(&self) -> Result<bool, BytecodeError> {
        let mut required = vec![BTreeSet::new(); self.model.functions().len()];
        let mut capability_required = vec![false; self.model.functions().len()];
        let mut reverse_calls = vec![Vec::new(); self.model.functions().len()];
        for (caller, function) in self.model.functions().iter().enumerate() {
            let definitions = self.collect_definitions(function, caller)?;
            for block in &function.blocks {
                for instruction in &block.instructions {
                    match instruction {
                        Instruction::ConsoleWrite { .. } => {
                            required[caller].insert(Effect::ConsoleWrite);
                            capability_required[caller] = true;
                        }
                        Instruction::Call {
                            function: callee, ..
                        } => push_fallible(
                            &mut reverse_calls[to_usize(callee.get())],
                            caller,
                            self.function_offset(caller),
                        )?,
                        Instruction::CallClosure {
                            callee, arguments, ..
                        } => {
                            let callee_type = definitions[to_usize(callee.get())].value_type;
                            let Some((parameters, _, effects)) = self.function_type(callee_type)
                            else {
                                return Err(error(
                                    BytecodePhase::Type,
                                    BytecodeReason::CallSignatureMismatch,
                                    self.function_offset(caller),
                                    [to_u32(caller), callee_type.get()],
                                ));
                            };
                            if arguments.len() == parameters.len() {
                                required[caller].extend(effects.iter().copied());
                                capability_required[caller] |=
                                    effects.contains(&Effect::ConsoleWrite);
                            }
                        }
                        Instruction::Handle {
                            body_function,
                            clauses,
                            ..
                        } => {
                            let body = &self.model.functions()[to_usize(body_function.get())];
                            let body_console = body.effects.contains(&Effect::ConsoleWrite);
                            let handles_console = clauses
                                .iter()
                                .any(|clause| clause.operation == HandlerOperation::ConsoleWrite);
                            for effect in &body.effects {
                                if !(handles_console && *effect == Effect::ConsoleWrite) {
                                    required[caller].insert(*effect);
                                }
                            }
                            let mut clause_console = false;
                            for clause in clauses {
                                let effects = &self.model.functions()
                                    [to_usize(clause.function.get())]
                                .effects;
                                required[caller].extend(effects.iter().copied());
                                clause_console |= effects.contains(&Effect::ConsoleWrite);
                            }
                            capability_required[caller] |= body_console || clause_console;
                        }
                        Instruction::CellNew { initial, .. } => {
                            let value_type = definitions[to_usize(initial.get())].value_type;
                            required[caller].insert(Effect::State(value_type));
                        }
                        Instruction::CellGet { cell, .. } | Instruction::CellSet { cell, .. } => {
                            let cell_type = definitions[to_usize(cell.get())].value_type;
                            let Some(value_type) = self.cell_value_type(cell_type) else {
                                return Err(error(
                                    BytecodePhase::Type,
                                    BytecodeReason::InvalidRegisterType,
                                    self.function_offset(caller),
                                    [to_u32(caller), cell.get()],
                                ));
                            };
                            required[caller].insert(Effect::State(value_type));
                        }
                        Instruction::Const { .. }
                        | Instruction::IntUnary { .. }
                        | Instruction::IntBinary { .. }
                        | Instruction::Compare { .. }
                        | Instruction::MakeClosure { .. }
                        | Instruction::Intrinsic { .. }
                        | Instruction::MakeTuple { .. }
                        | Instruction::GetTuple { .. }
                        | Instruction::MakeRecord { .. }
                        | Instruction::GetField { .. }
                        | Instruction::UpdateRecord { .. }
                        | Instruction::MakeVariant { .. }
                        | Instruction::VariantIs { .. }
                        | Instruction::GetVariantPayload { .. } => {}
                    }
                }
            }
        }
        let mut pending = VecDeque::new();
        for (index, value) in required.iter().enumerate() {
            if !value.is_empty() {
                pending.push_back(index);
            }
        }
        while let Some(callee) = pending.pop_front() {
            let callee_effects = required[callee].clone();
            for caller in &reverse_calls[callee] {
                let previous = required[*caller].len();
                required[*caller].extend(callee_effects.iter().copied());
                if required[*caller].len() != previous {
                    pending.push_back(*caller);
                }
            }
        }
        let mut pending = capability_required
            .iter()
            .enumerate()
            .filter_map(|(index, value)| value.then_some(index))
            .collect::<VecDeque<_>>();
        while let Some(callee) = pending.pop_front() {
            for caller in &reverse_calls[callee] {
                if !capability_required[*caller] {
                    capability_required[*caller] = true;
                    pending.push_back(*caller);
                }
            }
        }

        for (index, function) in self.model.functions().iter().enumerate() {
            let declared = function.effects.as_slice();
            let expected = required[index].iter().copied().collect::<Vec<_>>();
            if declared != expected {
                return Err(error(
                    BytecodePhase::Effect,
                    BytecodeReason::EffectMismatch,
                    self.function_offset(index),
                    [to_u32(index)],
                ));
            }
            if capability_required[index] {
                let module = &self.model.modules()[to_usize(function.module.get())];
                if !module.capabilities.contains(&Capability::ConsoleWrite) {
                    return Err(error(
                        BytecodePhase::Capability,
                        BytecodeReason::CapabilityMismatch,
                        self.function_offset(index),
                        [to_u32(index), function.module.get()],
                    ));
                }
            }
        }
        self.verify_entry()?;
        Ok(capability_required[to_usize(self.model.entry().get())])
    }

    fn verify_entry(&self) -> Result<(), BytecodeError> {
        let entry_index = to_usize(self.model.entry().get());
        let entry = &self.model.functions()[entry_index];
        let module = &self.model.modules()[to_usize(entry.module.get())];
        let module_name = &self.model.strings()[to_usize(module.name.get())];
        let function_name = &self.model.strings()[to_usize(entry.name.get())];
        let valid_signature = entry.parameter_types.as_slice() == [crate::TypeIndex::new(0)]
            && entry.result_type == crate::TypeIndex::new(0)
            && entry.blocks[0].parameters.len() == 1
            && entry.blocks[0].parameters[0].value_type == crate::TypeIndex::new(0);
        if module_name != "Main"
            || function_name != "main"
            || entry.kind != FunctionKind::Named
            || !valid_signature
        {
            return Err(error(
                BytecodePhase::Entry,
                BytecodeReason::InvalidEntry,
                self.offsets.entry,
                [self.model.entry().get()],
            ));
        }
        Ok(())
    }

    fn verify_source_map(&self) -> Result<(), BytecodeError> {
        if self.model.source_map().len() != to_usize(self.offsets.executable_locations) {
            return Err(error(
                BytecodePhase::SourceMap,
                BytecodeReason::IncompleteSourceMap,
                self.offsets.source_map_count,
                [
                    to_u32(self.model.source_map().len()),
                    self.offsets.executable_locations,
                ],
            ));
        }

        for (index, entry) in self.model.source_map().iter().enumerate() {
            self.verify_source_map_entry(entry, index)?;
        }

        for index in 1..self.model.source_map().len() {
            let left = source_map_key(&self.model.source_map()[index - 1]);
            let right = source_map_key(&self.model.source_map()[index]);
            if left >= right {
                let reason = if left == right {
                    BytecodeReason::DuplicateSourceMap
                } else {
                    BytecodeReason::InvalidSourceMapOrder
                };
                return Err(error(
                    BytecodePhase::SourceMap,
                    reason,
                    offset(
                        &self.offsets.source_map,
                        index,
                        self.offsets.source_map_count,
                    ),
                    [to_u32(index - 1), to_u32(index)],
                ));
            }
        }

        let mut cursor = 0_usize;
        for (function_index, function) in self.model.functions().iter().enumerate() {
            for (block_index, block) in function.blocks.iter().enumerate() {
                for ordinal in 0..=block.instructions.len() {
                    let entry = &self.model.source_map()[cursor];
                    let expected = (to_u32(function_index), to_u32(block_index), to_u32(ordinal));
                    if source_map_key(entry) != expected {
                        return Err(error(
                            BytecodePhase::SourceMap,
                            BytecodeReason::IncompleteSourceMap,
                            offset(
                                &self.offsets.source_map,
                                cursor,
                                self.offsets.source_map_count,
                            ),
                            [expected.0, expected.1, expected.2],
                        ));
                    }
                    cursor += 1;
                }
            }
        }
        Ok(())
    }

    fn verify_source_map_entry(
        &self,
        entry: &crate::SourceMapEntry,
        index: usize,
    ) -> Result<(), BytecodeError> {
        let entry_offset = offset(
            &self.offsets.source_map,
            index,
            self.offsets.source_map_count,
        );
        self.ensure_index(
            entry.function.get(),
            self.model.functions().len(),
            BytecodePhase::SourceMap,
            BytecodeReason::InvalidFunctionIndex,
            entry_offset,
            [to_u32(index), entry.function.get()],
        )?;
        let function = &self.model.functions()[to_usize(entry.function.get())];
        self.ensure_index(
            entry.block.get(),
            function.blocks.len(),
            BytecodePhase::SourceMap,
            BytecodeReason::InvalidBlockIndex,
            entry_offset,
            [to_u32(index), entry.function.get(), entry.block.get()],
        )?;
        let block = &function.blocks[to_usize(entry.block.get())];
        if to_usize(entry.ordinal) > block.instructions.len() {
            return Err(error(
                BytecodePhase::SourceMap,
                BytecodeReason::IncompleteSourceMap,
                entry_offset,
                [
                    to_u32(index),
                    entry.function.get(),
                    entry.block.get(),
                    entry.ordinal,
                ],
            ));
        }
        self.ensure_index(
            entry.source.get(),
            self.model.sources().len(),
            BytecodePhase::SourceMap,
            BytecodeReason::InvalidSourceIndex,
            entry_offset,
            [to_u32(index), entry.source.get()],
        )?;
        let source = &self.model.sources()[to_usize(entry.source.get())];
        if entry.span.start_byte() > entry.span.end_byte()
            || entry.span.end_byte() > source.original_byte_length
        {
            return Err(error(
                BytecodePhase::SourceMap,
                BytecodeReason::InvalidSourceSpan,
                entry_offset,
                [to_u32(index), entry.source.get()],
            ));
        }
        if source.module != function.module {
            return Err(error(
                BytecodePhase::SourceMap,
                BytecodeReason::InvalidSourceOwner,
                entry_offset,
                [to_u32(index), entry.source.get(), function.module.get()],
            ));
        }
        Ok(())
    }

    fn instruction_result_type(
        &self,
        instruction: &Instruction,
        definitions: &[Option<Definition>],
    ) -> Option<TypeIndex> {
        match instruction {
            Instruction::Const { constant, .. } => Some(TypeIndex::new(constant_type_index(
                &self.model.constants()[to_usize(constant.get())],
            ))),
            Instruction::IntUnary { .. } | Instruction::IntBinary { .. } => Some(TypeIndex::new(2)),
            Instruction::Compare { .. } => Some(TypeIndex::new(1)),
            Instruction::Call { function, .. } => {
                let function = &self.model.functions()[to_usize(function.get())];
                Some(function.result_type)
            }
            Instruction::MakeClosure { function, .. } => {
                self.complete_function_type(&self.model.functions()[to_usize(function.get())])
            }
            Instruction::Handle { body_function, .. } => {
                Some(self.model.functions()[to_usize(body_function.get())].result_type)
            }
            Instruction::CallClosure {
                callee, arguments, ..
            } => {
                let callee_type = definitions
                    .get(to_usize(callee.get()))?
                    .as_ref()?
                    .value_type;
                if callee_type.get() == u32::MAX {
                    return None;
                }
                let (parameters, result, effects) = self.function_type(callee_type)?;
                if arguments.is_empty() || arguments.len() > parameters.len() {
                    return None;
                }
                if arguments.len() == parameters.len() {
                    Some(result)
                } else {
                    self.find_function_type(&parameters[arguments.len()..], result, effects)
                }
            }
            Instruction::Intrinsic { intrinsic, .. } => {
                Some(scalar_type_index(intrinsic_result(*intrinsic)))
            }
            Instruction::MakeTuple { tuple, .. } => Some(*tuple),
            Instruction::GetTuple { tuple, element, .. } => {
                let tuple_type = definitions.get(to_usize(tuple.get()))?.as_ref()?.value_type;
                match self.model.types().get(to_usize(tuple_type.get()))? {
                    ValueType::Tuple { elements } => elements.get(to_usize(*element)).copied(),
                    _ => None,
                }
            }
            Instruction::MakeRecord { record, .. } => Some(*record),
            Instruction::GetField { record, field, .. } => {
                let record_type = definitions
                    .get(to_usize(record.get()))?
                    .as_ref()?
                    .value_type;
                match self.model.types().get(to_usize(record_type.get()))? {
                    ValueType::Record { fields, .. } => {
                        fields.get(to_usize(*field)).map(|f| f.value_type)
                    }
                    _ => None,
                }
            }
            Instruction::UpdateRecord { base, .. } => {
                Some(definitions.get(to_usize(base.get()))?.as_ref()?.value_type)
            }
            Instruction::MakeVariant { variant, .. } => Some(*variant),
            Instruction::VariantIs { .. } => Some(TypeIndex::new(1)),
            Instruction::GetVariantPayload { variant, case, .. } => {
                let variant_type = definitions
                    .get(to_usize(variant.get()))?
                    .as_ref()?
                    .value_type;
                match self.model.types().get(to_usize(variant_type.get()))? {
                    ValueType::Variant { cases, .. } => {
                        cases.get(to_usize(*case)).and_then(|case| case.payload)
                    }
                    _ => None,
                }
            }
            Instruction::CellNew { initial, .. } => {
                let initial_type = definitions
                    .get(to_usize(initial.get()))?
                    .as_ref()?
                    .value_type;
                self.find_cell_type(initial_type)
            }
            Instruction::CellGet { cell, .. } => {
                let cell_type = definitions.get(to_usize(cell.get()))?.as_ref()?.value_type;
                self.cell_value_type(cell_type)
            }
            Instruction::CellSet { .. } => Some(TypeIndex::new(0)),
            Instruction::ConsoleWrite { .. } => Some(TypeIndex::new(0)),
        }
    }

    fn function_type(&self, index: TypeIndex) -> Option<(&[TypeIndex], TypeIndex, &[Effect])> {
        match self.model.types().get(to_usize(index.get()))? {
            ValueType::Function {
                parameters,
                result,
                effects,
            } => Some((parameters, *result, effects)),
            ValueType::Unit
            | ValueType::Bool
            | ValueType::Int
            | ValueType::Text
            | ValueType::Tuple { .. }
            | ValueType::Record { .. }
            | ValueType::Variant { .. }
            | ValueType::Cell(_) => None,
        }
    }

    fn find_cell_type(&self, value: TypeIndex) -> Option<TypeIndex> {
        self.model
            .types()
            .iter()
            .position(|candidate| *candidate == ValueType::Cell(value))
            .map(|index| TypeIndex::new(to_u32(index)))
    }

    fn cell_value_type(&self, cell: TypeIndex) -> Option<TypeIndex> {
        match self.model.types().get(to_usize(cell.get()))? {
            ValueType::Cell(value) => Some(*value),
            _ => None,
        }
    }

    fn find_function_type(
        &self,
        parameters: &[TypeIndex],
        result: TypeIndex,
        effects: &[Effect],
    ) -> Option<TypeIndex> {
        self.model
            .types()
            .iter()
            .position(|value| {
                matches!(
                    value,
                    ValueType::Function {
                        parameters: candidate_parameters,
                        result: candidate_result,
                        effects: candidate_effects,
                    } if candidate_parameters == parameters
                        && *candidate_result == result
                        && candidate_effects == effects
                )
            })
            .map(|index| TypeIndex::new(to_u32(index)))
    }

    fn complete_function_type(&self, function: &Function) -> Option<TypeIndex> {
        self.find_function_type(
            function
                .parameter_types
                .get(to_usize(function.capture_count)..)?,
            function.result_type,
            &function.effects,
        )
    }

    fn string_at(&self, index: u32, record_offset: u64) -> Result<&'a str, BytecodeError> {
        self.string_at_with_phase(index, record_offset, BytecodePhase::Table)
    }

    fn string_at_with_phase(
        &self,
        index: u32,
        record_offset: u64,
        phase: BytecodePhase,
    ) -> Result<&'a str, BytecodeError> {
        self.model.strings().get(to_usize(index)).map_or_else(
            || {
                Err(error(
                    phase,
                    BytecodeReason::InvalidStringIndex,
                    record_offset,
                    [index],
                ))
            },
            |value| Ok(value.as_str()),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn ensure_index<const N: usize>(
        &self,
        index: u32,
        length: usize,
        phase: BytecodePhase,
        reason: BytecodeReason,
        record_offset: u64,
        referenced: [u32; N],
    ) -> Result<(), BytecodeError> {
        if to_usize(index) < length {
            Ok(())
        } else {
            Err(error(phase, reason, record_offset, referenced))
        }
    }

    fn ensure_register(
        &self,
        function: &Function,
        register: RegisterIndex,
        record_offset: u64,
        function_index: usize,
        block_index: usize,
    ) -> Result<(), BytecodeError> {
        self.ensure_index(
            register.get(),
            to_usize(function.register_count),
            BytecodePhase::Register,
            BytecodeReason::InvalidRegisterIndex,
            record_offset,
            [to_u32(function_index), to_u32(block_index), register.get()],
        )
    }

    fn ensure_registers(
        &self,
        function: &Function,
        registers: &[RegisterIndex],
        record_offset: u64,
        function_index: usize,
        block_index: usize,
    ) -> Result<(), BytecodeError> {
        for register in registers {
            self.ensure_register(
                function,
                *register,
                record_offset,
                function_index,
                block_index,
            )?;
        }
        Ok(())
    }

    fn ensure_block(
        &self,
        function: &Function,
        block: u32,
        record_offset: u64,
        function_index: usize,
        block_index: usize,
    ) -> Result<(), BytecodeError> {
        self.ensure_index(
            block,
            function.blocks.len(),
            BytecodePhase::ControlFlow,
            BytecodeReason::InvalidBlockIndex,
            record_offset,
            [to_u32(function_index), to_u32(block_index), block],
        )
    }

    fn table_error<const N: usize>(
        &self,
        reason: BytecodeReason,
        record_offset: u64,
        referenced: [u32; N],
    ) -> BytecodeError {
        error(BytecodePhase::Table, reason, record_offset, referenced)
    }

    fn function_offset(&self, function: usize) -> u64 {
        self.offsets
            .functions
            .get(function)
            .map_or(40, |value| value.record)
    }

    fn block_offset(&self, function: usize, block: usize) -> u64 {
        self.offsets
            .functions
            .get(function)
            .and_then(|value| value.blocks.get(block))
            .map_or_else(|| self.function_offset(function), |value| value.record)
    }

    fn instruction_offset(&self, function: usize, block: usize, instruction: usize) -> u64 {
        self.offsets
            .functions
            .get(function)
            .and_then(|value| value.blocks.get(block))
            .and_then(|value| value.instructions.get(instruction))
            .copied()
            .unwrap_or_else(|| self.block_offset(function, block))
    }

    fn terminator_offset(&self, function: usize, block: usize) -> u64 {
        self.offsets
            .functions
            .get(function)
            .and_then(|value| value.blocks.get(block))
            .map_or_else(
                || self.block_offset(function, block),
                |value| value.terminator,
            )
    }
}

#[derive(Clone, Copy, Debug)]
struct Definition {
    block: usize,
    location: DefinitionLocation,
    value_type: TypeIndex,
}

#[derive(Clone, Copy, Debug)]
enum DefinitionLocation {
    BlockParameter,
    Instruction(usize),
}

struct Dominators {
    enter: Vec<usize>,
    exit: Vec<usize>,
}

struct ControlFlowGraph {
    successors: Vec<Vec<usize>>,
    predecessors: Vec<Vec<usize>>,
}

impl Dominators {
    fn compute(
        successors: &[Vec<usize>],
        predecessors: &[Vec<usize>],
        error_offset: u64,
        function_index: usize,
    ) -> Result<Self, BytecodeError> {
        let count = successors.len();
        let mut visited = vec![false; count];
        let mut postorder = Vec::with_capacity(count);
        let mut stack = Vec::with_capacity(count);
        visited[0] = true;
        stack.push((0_usize, 0_usize));
        while let Some((node, child_index)) = stack.last_mut() {
            if *child_index < successors[*node].len() {
                let child = successors[*node][*child_index];
                *child_index += 1;
                if !visited[child] {
                    visited[child] = true;
                    stack.push((child, 0));
                }
            } else {
                postorder.push(*node);
                stack.pop();
            }
        }
        let mut reverse_postorder = postorder;
        reverse_postorder.reverse();
        let mut order = vec![0_usize; count];
        for (index, block) in reverse_postorder.iter().enumerate() {
            order[*block] = index;
        }

        let mut immediate = vec![None; count];
        immediate[0] = Some(0);
        let mut passes = 0_usize;
        loop {
            let mut changed = false;
            for block in reverse_postorder.iter().copied().skip(1) {
                let mut defined_predecessors = predecessors[block]
                    .iter()
                    .copied()
                    .filter(|predecessor| immediate[*predecessor].is_some());
                let Some(mut candidate) = defined_predecessors.next() else {
                    continue;
                };
                for predecessor in defined_predecessors {
                    candidate = intersect(predecessor, candidate, &immediate, &order);
                }
                if immediate[block] != Some(candidate) {
                    immediate[block] = Some(candidate);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
            passes = passes.saturating_add(1);
            if passes > count {
                return Err(BytecodeError::resource(
                    BytecodePhase::ControlFlow,
                    error_offset,
                    "dominator_iterations",
                    u64::try_from(passes).unwrap_or(u64::MAX),
                    u64::try_from(count).unwrap_or(u64::MAX),
                )
                .with_indices([to_u32(function_index)]));
            }
        }

        let mut children = vec![Vec::new(); count];
        for (block, parent) in immediate.iter().copied().enumerate().skip(1) {
            let Some(parent) = parent else {
                return Err(error(
                    BytecodePhase::ControlFlow,
                    BytecodeReason::UnreachableBlock,
                    error_offset,
                    [to_u32(function_index), to_u32(block)],
                ));
            };
            children[parent].push(block);
        }
        let mut enter = vec![0; count];
        let mut exit = vec![0; count];
        let mut timer = 0_usize;
        let mut traversal = vec![(0_usize, false)];
        while let Some((node, exiting)) = traversal.pop() {
            if exiting {
                exit[node] = timer;
                timer = timer.saturating_add(1);
            } else {
                enter[node] = timer;
                timer = timer.saturating_add(1);
                traversal.push((node, true));
                for child in children[node].iter().rev() {
                    traversal.push((*child, false));
                }
            }
        }
        Ok(Self { enter, exit })
    }

    fn dominates(&self, dominator: usize, block: usize) -> bool {
        self.enter[dominator] <= self.enter[block] && self.exit[block] <= self.exit[dominator]
    }
}

fn intersect(
    mut left: usize,
    mut right: usize,
    immediate: &[Option<usize>],
    order: &[usize],
) -> usize {
    while left != right {
        while order[left] > order[right] {
            left = immediate[left].unwrap_or(0);
        }
        while order[right] > order[left] {
            right = immediate[right].unwrap_or(0);
        }
    }
    left
}

#[allow(clippy::too_many_arguments)]
fn check_use(
    definitions: &[Definition],
    dominators: &Dominators,
    register: RegisterIndex,
    expected: TypeIndex,
    use_block: usize,
    use_ordinal: usize,
    use_offset: u64,
    mismatch_reason: BytecodeReason,
    function_index: usize,
) -> Result<(), BytecodeError> {
    let Some(definition) = definitions.get(to_usize(register.get())) else {
        return Err(error(
            BytecodePhase::Register,
            BytecodeReason::InvalidRegisterIndex,
            use_offset,
            [to_u32(function_index), register.get()],
        ));
    };
    let dominates = if definition.block == use_block {
        match definition.location {
            DefinitionLocation::BlockParameter => true,
            DefinitionLocation::Instruction(definition_ordinal) => definition_ordinal < use_ordinal,
        }
    } else {
        dominators.dominates(definition.block, use_block)
    };
    if !dominates {
        return Err(error(
            BytecodePhase::Register,
            BytecodeReason::RegisterNotDominated,
            use_offset,
            [to_u32(function_index), register.get()],
        ));
    }
    if definition.value_type != expected {
        return Err(error(
            BytecodePhase::Type,
            mismatch_reason,
            use_offset,
            [to_u32(function_index), register.get()],
        ));
    }
    Ok(())
}

fn error<const N: usize>(
    phase: BytecodePhase,
    reason: BytecodeReason,
    offset: u64,
    indices: [u32; N],
) -> BytecodeError {
    BytecodeError::new(phase, reason, offset, 1).with_indices(indices)
}

fn offset(offsets: &[u64], index: usize, fallback: u64) -> u64 {
    offsets.get(index).copied().unwrap_or(fallback)
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

fn type_dependencies(value: &ValueType) -> Vec<usize> {
    match value {
        ValueType::Unit | ValueType::Bool | ValueType::Int | ValueType::Text => Vec::new(),
        ValueType::Function {
            parameters, result, ..
        } => parameters
            .iter()
            .chain(std::iter::once(result))
            .map(|index| to_usize(index.get()))
            .collect(),
        ValueType::Tuple { elements } => {
            elements.iter().map(|index| to_usize(index.get())).collect()
        }
        ValueType::Record {
            arguments, fields, ..
        } => arguments
            .iter()
            .chain(fields.iter().map(|field| &field.value_type))
            .map(|index| to_usize(index.get()))
            .collect(),
        ValueType::Variant {
            arguments, cases, ..
        } => arguments
            .iter()
            .chain(cases.iter().filter_map(|case| case.payload.as_ref()))
            .map(|index| to_usize(index.get()))
            .collect(),
        ValueType::Cell(value) => vec![to_usize(value.get())],
    }
}

fn to_usize(value: u32) -> usize {
    usize::try_from(value).unwrap_or(usize::MAX)
}

fn is_valid_package_name(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes.len() > 63 || !bytes[0].is_ascii_lowercase() {
        return false;
    }
    let mut previous_hyphen = false;
    for byte in bytes {
        match byte {
            b'a'..=b'z' | b'0'..=b'9' => previous_hyphen = false,
            b'-' if !previous_hyphen => previous_hyphen = true,
            _ => return false,
        }
    }
    !previous_hyphen
}

fn parse_package_version(value: &str) -> Option<[u32; 3]> {
    let mut values = [0_u32; 3];
    let mut components = value.split('.');
    for output in &mut values {
        let component = components.next()?;
        if component.is_empty()
            || !component.bytes().all(|byte| byte.is_ascii_digit())
            || (component.len() > 1 && component.starts_with('0'))
        {
            return None;
        }
        *output = component.parse().ok()?;
    }
    components.next().is_none().then_some(values)
}

fn is_valid_qualified_name(value: &str) -> bool {
    !value.is_empty() && value.split('.').all(is_valid_identifier)
}

fn is_valid_identifier(value: &str) -> bool {
    let Ok(security) = inspect_identifier(value) else {
        return false;
    };
    security.identifier().normalized() == value
        && security.status() == IdentifierStatus::Allowed
        && !security.has_suspicious_mixed_script()
}

fn is_valid_closure_label(value: &str) -> bool {
    let Some(suffix) = value.strip_prefix("closure_") else {
        return false;
    };
    let mut components = suffix.split('_');
    (0..3).all(|_| components.next().is_some_and(is_canonical_unsigned_decimal))
        && components.next().is_none()
}

fn is_canonical_unsigned_decimal(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'))
        && value.parse::<u64>().is_ok()
}

fn package_reference_key(reference: PackageReference) -> Option<u32> {
    match reference {
        PackageReference::Standalone => None,
        PackageReference::Package(index) => Some(index.get()),
    }
}

fn constant_type_index(constant: &Constant) -> u32 {
    match constant {
        Constant::Unit => 0,
        Constant::Bool(_) => 1,
        Constant::Int { .. } => 2,
        Constant::Text(_) => 3,
    }
}

fn scalar_type_index(value: ValueType) -> TypeIndex {
    match value {
        ValueType::Unit => TypeIndex::new(0),
        ValueType::Bool => TypeIndex::new(1),
        ValueType::Int => TypeIndex::new(2),
        ValueType::Text => TypeIndex::new(3),
        ValueType::Function { .. }
        | ValueType::Tuple { .. }
        | ValueType::Record { .. }
        | ValueType::Variant { .. }
        | ValueType::Cell(_) => unreachable!("aggregate/function/Cell types are not scalar"),
    }
}

fn compare_constants(
    left: &Constant,
    left_type: u32,
    right: &Constant,
    right_type: u32,
) -> Ordering {
    left_type
        .cmp(&right_type)
        .then_with(|| left.tag().cmp(&right.tag()))
        .then_with(|| compare_constant_payload(left, right))
}

fn compare_constant_payload(left: &Constant, right: &Constant) -> Ordering {
    match (left, right) {
        (Constant::Unit, Constant::Unit) => Ordering::Equal,
        (Constant::Bool(left), Constant::Bool(right)) => u8::from(*left).cmp(&u8::from(*right)),
        (
            Constant::Int {
                sign: left_sign,
                magnitude: left_magnitude,
            },
            Constant::Int {
                sign: right_sign,
                magnitude: right_magnitude,
            },
        ) => left_sign
            .tag()
            .cmp(&right_sign.tag())
            .then_with(|| {
                to_u32(left_magnitude.len())
                    .to_le_bytes()
                    .cmp(&to_u32(right_magnitude.len()).to_le_bytes())
            })
            .then_with(|| left_magnitude.cmp(right_magnitude)),
        (Constant::Text(left), Constant::Text(right)) => {
            left.get().to_le_bytes().cmp(&right.get().to_le_bytes())
        }
        _ => Ordering::Equal,
    }
}

fn instruction_destination(instruction: &Instruction) -> RegisterIndex {
    match instruction {
        Instruction::Const { destination, .. }
        | Instruction::IntUnary { destination, .. }
        | Instruction::IntBinary { destination, .. }
        | Instruction::Compare { destination, .. }
        | Instruction::Call { destination, .. }
        | Instruction::MakeClosure { destination, .. }
        | Instruction::CallClosure { destination, .. }
        | Instruction::Handle { destination, .. }
        | Instruction::MakeTuple { destination, .. }
        | Instruction::GetTuple { destination, .. }
        | Instruction::MakeRecord { destination, .. }
        | Instruction::GetField { destination, .. }
        | Instruction::UpdateRecord { destination, .. }
        | Instruction::MakeVariant { destination, .. }
        | Instruction::VariantIs { destination, .. }
        | Instruction::GetVariantPayload { destination, .. }
        | Instruction::CellNew { destination, .. }
        | Instruction::CellGet { destination, .. }
        | Instruction::CellSet { destination, .. }
        | Instruction::Intrinsic { destination, .. }
        | Instruction::ConsoleWrite { destination, .. } => *destination,
    }
}

fn terminator_targets(terminator: &Terminator) -> Vec<u32> {
    match terminator {
        Terminator::Jump { target, .. } => vec![target.get()],
        Terminator::Branch {
            true_target,
            false_target,
            ..
        } => vec![true_target.get(), false_target.get()],
        Terminator::Return { .. } => Vec::new(),
    }
}

fn intrinsic_parameters(intrinsic: Intrinsic) -> &'static [ValueType] {
    match intrinsic {
        Intrinsic::TextFormat => &[ValueType::Text, ValueType::Int],
        Intrinsic::MaxInt | Intrinsic::MinInt => &[ValueType::Int, ValueType::Int],
    }
}

fn intrinsic_result(intrinsic: Intrinsic) -> ValueType {
    match intrinsic {
        Intrinsic::TextFormat => ValueType::Text,
        Intrinsic::MaxInt | Intrinsic::MinInt => ValueType::Int,
    }
}

fn source_map_key(entry: &crate::SourceMapEntry) -> (u32, u32, u32) {
    (entry.function.get(), entry.block.get(), entry.ordinal)
}

fn push_fallible<T: Copy>(
    values: &mut Vec<T>,
    value: T,
    error_offset: u64,
) -> Result<(), BytecodeError> {
    values.try_reserve(1).map_err(|_| {
        BytecodeError::resource(
            BytecodePhase::ControlFlow,
            error_offset,
            "verification_edges",
            u64::try_from(values.len())
                .unwrap_or(u64::MAX)
                .saturating_add(1),
            u64::from(DecodeLimits::rfc_0014().executable_locations()),
        )
    })?;
    values.push(value);
    Ok(())
}

#[cfg(test)]
mod bytecode_1_4_tests {
    use super::*;
    use crate::{
        Block, BlockIndex, BlockParameter, Constant, ConstantIndex, FunctionIndex,
        LoweredProgramV1_4, Module, ModuleIndex, PackageReference, ProgramParts, Source,
        SourceDigest, SourceIndex, SourceMapEntry, SourceOrigin, SourceSpan, StringIndex,
        encode_v1_4, encode_verified_v1,
    };
    use sha2::Digest;

    fn cell_parts() -> ProgramParts {
        let instructions = vec![
            Instruction::Const {
                destination: RegisterIndex::new(1),
                constant: ConstantIndex::new(0),
            },
            Instruction::CellNew {
                destination: RegisterIndex::new(2),
                initial: RegisterIndex::new(1),
            },
            Instruction::CellGet {
                destination: RegisterIndex::new(3),
                cell: RegisterIndex::new(2),
            },
            Instruction::CellSet {
                destination: RegisterIndex::new(4),
                cell: RegisterIndex::new(2),
                value: RegisterIndex::new(3),
            },
        ];
        ProgramParts {
            strings: vec![
                "Main".to_owned(),
                "main".to_owned(),
                "src/Main.ling".to_owned(),
            ],
            packages: Vec::new(),
            modules: vec![Module {
                package: PackageReference::Standalone,
                name: StringIndex::new(0),
                capabilities: Vec::new(),
            }],
            types: vec![
                ValueType::Unit,
                ValueType::Bool,
                ValueType::Int,
                ValueType::Text,
                ValueType::Cell(TypeIndex::new(2)),
            ],
            constants: vec![Constant::Int {
                sign: IntegerSign::Positive,
                magnitude: vec![1],
            }],
            sources: vec![Source {
                module: ModuleIndex::new(0),
                logical_name: StringIndex::new(2),
                original_byte_length: 1,
                content_sha256: SourceDigest::new([0x14; 32]),
            }],
            functions: vec![Function {
                kind: FunctionKind::Named,
                module: ModuleIndex::new(0),
                name: StringIndex::new(1),
                capture_count: 0,
                parameter_types: vec![TypeIndex::new(0)],
                result_type: TypeIndex::new(0),
                effects: vec![Effect::State(TypeIndex::new(2))],
                register_count: 5,
                blocks: vec![Block {
                    parameters: vec![BlockParameter {
                        register: RegisterIndex::new(0),
                        value_type: TypeIndex::new(0),
                    }],
                    instructions,
                    terminator: Terminator::Return {
                        value: RegisterIndex::new(4),
                    },
                }],
            }],
            entry: FunctionIndex::new(0),
            source_map: (0..=4)
                .map(|ordinal| SourceMapEntry {
                    function: FunctionIndex::new(0),
                    block: BlockIndex::new(0),
                    ordinal,
                    source: SourceIndex::new(0),
                    span: SourceSpan::new(0, 1),
                    origin: SourceOrigin::LoweringDerived,
                })
                .collect(),
        }
    }

    fn encode_parts(parts: ProgramParts) -> Vec<u8> {
        crate::encode::encode_model(
            &UnverifiedProgram::from_parts(parts),
            FORMAT_VERSION_1_4,
            DecodeLimits::rfc_0016().artifact_bytes(),
        )
        .expect("test model is encodable")
    }

    fn verify_parts(parts: ProgramParts) -> Result<VerifiedProgramV1, BytecodeError> {
        decode_and_verify_v1_4(&encode_parts(parts))
    }

    fn unique_position(bytes: &[u8], needle: &[u8]) -> usize {
        let positions = bytes
            .windows(needle.len())
            .enumerate()
            .filter_map(|(index, window)| (window == needle).then_some(index))
            .collect::<Vec<_>>();
        assert_eq!(positions.len(), 1, "fixture marker is unique");
        positions[0]
    }

    fn closure_cell_parts() -> ProgramParts {
        let state = vec![Effect::State(TypeIndex::new(2))];
        let functions = vec![
            Function {
                kind: FunctionKind::Named,
                module: ModuleIndex::new(0),
                name: StringIndex::new(2),
                capture_count: 0,
                parameter_types: vec![TypeIndex::new(0)],
                result_type: TypeIndex::new(0),
                effects: state.clone(),
                register_count: 5,
                blocks: vec![Block {
                    parameters: vec![BlockParameter {
                        register: RegisterIndex::new(0),
                        value_type: TypeIndex::new(0),
                    }],
                    instructions: vec![
                        Instruction::Const {
                            destination: RegisterIndex::new(1),
                            constant: ConstantIndex::new(0),
                        },
                        Instruction::CellNew {
                            destination: RegisterIndex::new(2),
                            initial: RegisterIndex::new(1),
                        },
                        Instruction::MakeClosure {
                            destination: RegisterIndex::new(3),
                            function: FunctionIndex::new(1),
                            captures: vec![CaptureOperand::Register(RegisterIndex::new(2))],
                        },
                        Instruction::CallClosure {
                            destination: RegisterIndex::new(4),
                            callee: RegisterIndex::new(3),
                            arguments: vec![RegisterIndex::new(0)],
                        },
                    ],
                    terminator: Terminator::Return {
                        value: RegisterIndex::new(4),
                    },
                }],
            },
            Function {
                kind: FunctionKind::ClosureBody,
                module: ModuleIndex::new(0),
                name: StringIndex::new(1),
                capture_count: 1,
                parameter_types: vec![TypeIndex::new(5), TypeIndex::new(0)],
                result_type: TypeIndex::new(0),
                effects: state,
                register_count: 4,
                blocks: vec![Block {
                    parameters: vec![
                        BlockParameter {
                            register: RegisterIndex::new(0),
                            value_type: TypeIndex::new(5),
                        },
                        BlockParameter {
                            register: RegisterIndex::new(1),
                            value_type: TypeIndex::new(0),
                        },
                    ],
                    instructions: vec![
                        Instruction::CellGet {
                            destination: RegisterIndex::new(2),
                            cell: RegisterIndex::new(0),
                        },
                        Instruction::CellSet {
                            destination: RegisterIndex::new(3),
                            cell: RegisterIndex::new(0),
                            value: RegisterIndex::new(2),
                        },
                    ],
                    terminator: Terminator::Return {
                        value: RegisterIndex::new(3),
                    },
                }],
            },
        ];
        let mut source_map = Vec::new();
        for (function, count) in [(0, 5), (1, 3)] {
            source_map.extend((0..count).map(|ordinal| SourceMapEntry {
                function: FunctionIndex::new(function),
                block: BlockIndex::new(0),
                ordinal,
                source: SourceIndex::new(0),
                span: SourceSpan::new(0, 1),
                origin: SourceOrigin::LoweringDerived,
            }));
        }
        ProgramParts {
            strings: vec![
                "Main".to_owned(),
                "closure_0_0_0".to_owned(),
                "main".to_owned(),
                "src/Main.ling".to_owned(),
            ],
            packages: Vec::new(),
            modules: vec![Module {
                package: PackageReference::Standalone,
                name: StringIndex::new(0),
                capabilities: Vec::new(),
            }],
            types: vec![
                ValueType::Unit,
                ValueType::Bool,
                ValueType::Int,
                ValueType::Text,
                ValueType::Function {
                    parameters: vec![TypeIndex::new(0)],
                    result: TypeIndex::new(0),
                    effects: vec![Effect::State(TypeIndex::new(2))],
                },
                ValueType::Cell(TypeIndex::new(2)),
            ],
            constants: vec![Constant::Int {
                sign: IntegerSign::Positive,
                magnitude: vec![1],
            }],
            sources: vec![Source {
                module: ModuleIndex::new(0),
                logical_name: StringIndex::new(3),
                original_byte_length: 1,
                content_sha256: SourceDigest::new([0x24; 32]),
            }],
            functions,
            entry: FunctionIndex::new(0),
            source_map,
        }
    }

    #[test]
    fn cell_state_wire_round_trips_and_older_readers_reject_it() {
        let lowered = LoweredProgramV1_4::new(UnverifiedProgram::from_parts(cell_parts()));
        let bytes = encode_v1_4(&lowered).expect("canonical 1.4 model encodes");
        let digest = sha2::Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let hexadecimal = bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        assert_eq!(bytes.len(), 599);
        assert_eq!(
            digest,
            "18b74871f6800165f47966ebc9b870180d887d31ab00cd76a5a0e20c7868dfab"
        );
        assert_eq!(
            hexadecimal,
            include_str!("../../../tests/bytecode/v1/golden/cell-state-1.4.lbc.hex").trim_end()
        );

        assert_eq!(&bytes[12..16], &[1, 0, 4, 0]);
        for record in [
            &[0x14, 0, 0, 0, 2, 0, 0, 0][..],
            &[0x1d, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0][..],
            &[0x1e, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 0][..],
            &[0x1f, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0][..],
        ] {
            assert!(bytes.windows(record.len()).any(|window| window == record));
        }

        let verified = decode_and_verify_v1_4(&bytes).expect("valid Cell/State program verifies");
        assert_eq!(verified.version(), FORMAT_VERSION_1_4);
        assert_eq!(
            encode_verified_v1(&verified).expect("verified 1.4 canonical re-encoding"),
            bytes
        );
        let disassembly = crate::disassemble_v1_4(&lowered);
        assert_eq!(
            disassembly,
            include_str!("../../../tests/bytecode/v1/golden/cell-state-1.4.dis")
        );

        for reader in [
            crate::decode_v1,
            crate::decode_v1_1,
            crate::decode_v1_2,
            crate::decode_v1_3,
        ] {
            let error = reader(&bytes).expect_err("older reader rejects bytecode 1.4");
            assert_eq!(error.reason(), BytecodeReason::UnsupportedVersion);
        }
    }

    #[test]
    fn cell_and_state_invariants_fail_before_verified_publication() {
        let mut forward = cell_parts();
        forward.types[4] = ValueType::Cell(TypeIndex::new(4));
        assert_eq!(
            verify_parts(forward)
                .expect_err("Cell type cannot reference itself")
                .reason(),
            BytecodeReason::InvalidTypeIndex
        );

        let mut escaped = cell_parts();
        escaped.types.push(ValueType::Tuple {
            elements: vec![TypeIndex::new(4)],
        });
        assert_eq!(
            verify_parts(escaped)
                .expect_err("Cell cannot enter aggregate fields")
                .reason(),
            BytecodeReason::InvalidTypeIndex
        );

        let mut state_of_cell = cell_parts();
        state_of_cell.functions[0].effects = vec![Effect::State(TypeIndex::new(4))];
        assert_eq!(
            verify_parts(state_of_cell)
                .expect_err("State cannot carry a Cell type")
                .reason(),
            BytecodeReason::InvalidTypeIndex
        );

        let mut duplicate = cell_parts();
        duplicate.functions[0]
            .effects
            .push(Effect::State(TypeIndex::new(2)));
        assert_eq!(
            verify_parts(duplicate)
                .expect_err("State rows are duplicate-free")
                .reason(),
            BytecodeReason::InvalidTableOrder
        );

        let mut missing = cell_parts();
        missing.functions[0].effects.clear();
        assert_eq!(
            verify_parts(missing)
                .expect_err("Cell instructions require State")
                .reason(),
            BytecodeReason::EffectMismatch
        );

        let mut excess = cell_parts();
        excess.functions[0].effects.insert(0, Effect::ConsoleWrite);
        assert_eq!(
            verify_parts(excess)
                .expect_err("excess effects are rejected")
                .reason(),
            BytecodeReason::EffectMismatch
        );

        let mut unordered = cell_parts();
        unordered.functions[0].effects =
            vec![Effect::State(TypeIndex::new(2)), Effect::ConsoleWrite];
        assert_eq!(
            verify_parts(unordered)
                .expect_err("State rows follow tag order")
                .reason(),
            BytecodeReason::InvalidTableOrder
        );

        let mut result_escape = cell_parts();
        result_escape.functions[0].result_type = TypeIndex::new(4);
        assert_eq!(
            verify_parts(result_escape)
                .expect_err("Cell cannot escape through a function result")
                .reason(),
            BytecodeReason::InvalidTypeIndex
        );

        let mut incomplete_map = cell_parts();
        incomplete_map.source_map.pop();
        assert_eq!(
            verify_parts(incomplete_map)
                .expect_err("every Cell instruction is source mapped")
                .reason(),
            BytecodeReason::IncompleteSourceMap
        );

        let mut mistyped = cell_parts();
        let Instruction::CellSet { value, .. } =
            &mut mistyped.functions[0].blocks[0].instructions[3]
        else {
            panic!("fixture has CellSet");
        };
        *value = RegisterIndex::new(0);
        assert_eq!(
            verify_parts(mistyped)
                .expect_err("CellSet value must match Cell payload")
                .reason(),
            BytecodeReason::InvalidRegisterType
        );
    }

    #[test]
    fn malformed_cell_state_records_are_bounded_and_structured() {
        let bytes = encode_parts(cell_parts());

        let mut reserved = bytes.clone();
        let cell_type = unique_position(&reserved, &[8, 0, 0, 0, 0x14, 0, 0, 0, 2, 0, 0, 0]);
        reserved[cell_type + 5] = 1;
        assert_eq!(
            decode_v1_4(&reserved)
                .expect_err("Cell reserved bytes are zero")
                .reason(),
            BytecodeReason::ReservedNonzero
        );

        let mut invalid_state_tag = bytes.clone();
        let state = unique_position(&invalid_state_tag, &[1, 0, 0, 0, 2, 2, 0, 0, 0]);
        invalid_state_tag[state + 4] = 3;
        assert_eq!(
            decode_v1_4(&invalid_state_tag)
                .expect_err("unknown State tag is rejected")
                .reason(),
            BytecodeReason::InvalidTag
        );

        let mut short_instruction = bytes;
        let cell_new = unique_position(
            &short_instruction,
            &[12, 0, 0, 0, 0x1d, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0],
        );
        short_instruction[cell_new] = 8;
        assert_eq!(
            decode_v1_4(&short_instruction)
                .expect_err("short CellNew is failure-atomic")
                .reason(),
            BytecodeReason::InvalidInstructionLength
        );
    }

    #[test]
    fn cell_capture_prefix_and_closure_state_propagation_are_verified() {
        verify_parts(closure_cell_parts())
            .expect("Cell capture prefix and full closure call retain State<Int>");

        let mut source_parameter = closure_cell_parts();
        source_parameter.functions[1].capture_count = 0;
        assert_eq!(
            verify_parts(source_parameter)
                .expect_err("Cell cannot escape into a closure source parameter")
                .reason(),
            BytecodeReason::InvalidTypeIndex
        );

        let mut missing_caller_state = closure_cell_parts();
        missing_caller_state.functions[0].effects.clear();
        assert_eq!(
            verify_parts(missing_caller_state)
                .expect_err("full closure calls propagate State")
                .reason(),
            BytecodeReason::EffectMismatch
        );

        let mut missing_callee_state = closure_cell_parts();
        missing_callee_state.functions[1].effects.clear();
        let ValueType::Function { effects, .. } = &mut missing_callee_state.types[4] else {
            panic!("fixture has closure function type");
        };
        effects.clear();
        assert_eq!(
            verify_parts(missing_callee_state)
                .expect_err("Cell instructions require the closure State row")
                .reason(),
            BytecodeReason::EffectMismatch
        );
    }
}
