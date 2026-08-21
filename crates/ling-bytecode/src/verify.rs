use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};

use ling_unicode::{IdentifierStatus, inspect_identifier};

use crate::decode::{DecodedOffsets, DecodedProgramV1};
use crate::path::validate_logical_name;
use crate::{
    Block, BytecodeError, BytecodePhase, BytecodeReason, Capability, Constant, DecodeLimits,
    Effect, Function, Instruction, IntegerSign, Intrinsic, PackageReference, RegisterIndex,
    Terminator, UnverifiedProgram, ValueType, decode_v1, decode_v1_with_limit,
};

/// Immutable bytecode state that has passed every RFC-0014 verifier phase.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedProgramV1 {
    model: UnverifiedProgram,
}

impl VerifiedProgramV1 {
    /// Returns the fully verified program model.
    #[must_use]
    pub const fn model(&self) -> &UnverifiedProgram {
        &self.model
    }
}

/// Independently verifies a decoded, untrusted version-1.0 program.
pub fn verify_v1(decoded: DecodedProgramV1) -> Result<VerifiedProgramV1, BytecodeError> {
    Verifier::new(decoded.model(), &decoded.offsets).verify()?;
    let (model, _) = decoded.into_parts();
    Ok(VerifiedProgramV1 { model })
}

/// Decodes and independently verifies one version-1.0 artifact.
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

struct Verifier<'a> {
    model: &'a UnverifiedProgram,
    offsets: &'a DecodedOffsets,
}

impl<'a> Verifier<'a> {
    const fn new(model: &'a UnverifiedProgram, offsets: &'a DecodedOffsets) -> Self {
        Self { model, offsets }
    }

    fn verify(&self) -> Result<(), BytecodeError> {
        self.verify_tables()?;
        self.verify_function_shapes()?;
        self.verify_control_flow_and_types()?;
        self.verify_effects_capabilities_and_entry()?;
        self.verify_source_map()
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
        const REQUIRED: [ValueType; 4] = [
            ValueType::Unit,
            ValueType::Bool,
            ValueType::Int,
            ValueType::Text,
        ];
        if self.model.types() != REQUIRED {
            let mismatch = self
                .model
                .types()
                .iter()
                .zip(REQUIRED)
                .position(|(actual, expected)| *actual != expected)
                .unwrap_or(self.model.types().len().min(REQUIRED.len()));
            return Err(self.table_error(
                BytecodeReason::InvalidTableOrder,
                offset(&self.offsets.types, mismatch, self.offsets.type_count),
                [to_u32(mismatch)],
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
            if !is_valid_identifier(name) {
                return Err(error(
                    BytecodePhase::Instruction,
                    BytecodeReason::InvalidName,
                    function_offset,
                    [to_u32(function_index)],
                ));
            }
            let skeleton_key = (
                function.module.get(),
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
            if function.effects.windows(2).any(|pair| pair[0] >= pair[1]) {
                return Err(error(
                    BytecodePhase::Effect,
                    BytecodeReason::InvalidTableOrder,
                    function_offset,
                    [to_u32(function_index)],
                ));
            }
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
                if (previous.module.get(), previous_name)
                    >= (function.module.get(), name.as_bytes())
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
        for value_type in &function.parameter_types {
            self.ensure_index(
                value_type.get(),
                self.model.types().len(),
                BytecodePhase::Type,
                BytecodeReason::InvalidTypeIndex,
                function_offset,
                [to_u32(function_index), value_type.get()],
            )?;
        }
        self.ensure_index(
            function.result_type.get(),
            self.model.types().len(),
            BytecodePhase::Type,
            BytecodeReason::InvalidTypeIndex,
            function_offset,
            [to_u32(function_index), function.result_type.get()],
        )
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
            for parameter in &block.parameters {
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
            Instruction::Intrinsic { arguments, .. } => self.ensure_registers(
                function,
                arguments,
                instruction_offset,
                function_index,
                block_index,
            ),
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
                let value_type = self.value_type(parameter.value_type.get());
                self.define_register(
                    &mut definitions,
                    parameter.register,
                    DefinitionLocation::BlockParameter,
                    value_type,
                    function_index,
                    block_index,
                    self.block_offset(function_index, block_index),
                )?;
            }
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                let value_type = self.instruction_result_type(instruction);
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
        Ok(definitions.into_iter().flatten().collect())
    }

    #[allow(clippy::too_many_arguments)]
    fn define_register(
        &self,
        definitions: &mut [Option<Definition>],
        register: RegisterIndex,
        location: DefinitionLocation,
        value_type: ValueType,
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
        _function: &Function,
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
                ValueType::Int,
                BytecodeReason::InvalidRegisterType,
            ),
            Instruction::IntBinary { left, right, .. } => {
                check(*left, ValueType::Int, BytecodeReason::InvalidRegisterType)?;
                check(*right, ValueType::Int, BytecodeReason::InvalidRegisterType)
            }
            Instruction::Compare {
                operator,
                left,
                right,
                ..
            } => {
                check(
                    *left,
                    operator.operand_type(),
                    BytecodeReason::InvalidRegisterType,
                )?;
                check(
                    *right,
                    operator.operand_type(),
                    BytecodeReason::InvalidRegisterType,
                )
            }
            Instruction::Call {
                function: callee,
                arguments,
                ..
            } => {
                let callee = &self.model.functions()[to_usize(callee.get())];
                if arguments.len() != callee.parameter_types.len() {
                    return Err(error(
                        BytecodePhase::Type,
                        BytecodeReason::CallSignatureMismatch,
                        instruction_offset,
                        [to_u32(function_index), callee.name.get()],
                    ));
                }
                for (argument, expected) in arguments.iter().zip(&callee.parameter_types) {
                    check(
                        *argument,
                        self.value_type(expected.get()),
                        BytecodeReason::CallSignatureMismatch,
                    )?;
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
                    check(*argument, *expected, BytecodeReason::CallSignatureMismatch)?;
                }
                Ok(())
            }
            Instruction::ConsoleWrite { text, .. } => {
                check(*text, ValueType::Text, BytecodeReason::InvalidRegisterType)
            }
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
                    ValueType::Bool,
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
                self.value_type(function.result_type.get()),
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
                self.value_type(parameter.value_type.get()),
                block_index,
                use_ordinal,
                terminator_offset,
                BytecodeReason::BlockArgumentTypeMismatch,
                function_index,
            )?;
        }
        Ok(())
    }

    fn verify_effects_capabilities_and_entry(&self) -> Result<(), BytecodeError> {
        let mut required = vec![false; self.model.functions().len()];
        let mut reverse_calls = vec![Vec::new(); self.model.functions().len()];
        for (caller, function) in self.model.functions().iter().enumerate() {
            for block in &function.blocks {
                for instruction in &block.instructions {
                    match instruction {
                        Instruction::ConsoleWrite { .. } => required[caller] = true,
                        Instruction::Call {
                            function: callee, ..
                        } => push_fallible(
                            &mut reverse_calls[to_usize(callee.get())],
                            caller,
                            self.function_offset(caller),
                        )?,
                        Instruction::Const { .. }
                        | Instruction::IntUnary { .. }
                        | Instruction::IntBinary { .. }
                        | Instruction::Compare { .. }
                        | Instruction::Intrinsic { .. } => {}
                    }
                }
            }
        }
        let mut pending = VecDeque::new();
        for (index, value) in required.iter().enumerate() {
            if *value {
                pending.push_back(index);
            }
        }
        while let Some(callee) = pending.pop_front() {
            for caller in &reverse_calls[callee] {
                if !required[*caller] {
                    required[*caller] = true;
                    pending.push_back(*caller);
                }
            }
        }

        for (index, function) in self.model.functions().iter().enumerate() {
            let declared = function.effects.as_slice();
            let expected = if required[index] {
                &[Effect::ConsoleWrite][..]
            } else {
                &[][..]
            };
            if declared != expected {
                return Err(error(
                    BytecodePhase::Effect,
                    BytecodeReason::EffectMismatch,
                    self.function_offset(index),
                    [to_u32(index)],
                ));
            }
            if required[index] {
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
        self.verify_entry()
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
        if module_name != "Main" || function_name != "main" || !valid_signature {
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

    fn instruction_result_type(&self, instruction: &Instruction) -> ValueType {
        match instruction {
            Instruction::Const { constant, .. } => {
                constant_value_type(&self.model.constants()[to_usize(constant.get())])
            }
            Instruction::IntUnary { .. } | Instruction::IntBinary { .. } => ValueType::Int,
            Instruction::Compare { .. } => ValueType::Bool,
            Instruction::Call { function, .. } => {
                let function = &self.model.functions()[to_usize(function.get())];
                self.value_type(function.result_type.get())
            }
            Instruction::Intrinsic { intrinsic, .. } => intrinsic_result(*intrinsic),
            Instruction::ConsoleWrite { .. } => ValueType::Unit,
        }
    }

    fn value_type(&self, index: u32) -> ValueType {
        self.model
            .types()
            .get(to_usize(index))
            .copied()
            .unwrap_or(ValueType::Unit)
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
    value_type: ValueType,
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
    expected: ValueType,
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

fn constant_value_type(constant: &Constant) -> ValueType {
    match constant {
        Constant::Unit => ValueType::Unit,
        Constant::Bool(_) => ValueType::Bool,
        Constant::Int { .. } => ValueType::Int,
        Constant::Text(_) => ValueType::Text,
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
