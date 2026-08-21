use ling_bytecode::{
    CompareOperator, Constant, Effect, Instruction, IntBinaryOperator, IntUnaryOperator,
    IntegerSign, Intrinsic, RegisterIndex, Terminator, UnverifiedProgram, VerifiedProgramV1,
};
use num_bigint::{BigInt, Sign};

use crate::fault::{
    ExecutionError, InternalExecutionError, RuntimeFault, RuntimeFaultKind, RuntimeResource,
};
use crate::host::HostCapabilities;
use crate::value::{Heap, Value};

/// Explicit execution limits required by RFC-0014.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutionLimits {
    step_limit: u64,
    frame_limit: u64,
    heap_byte_limit: u64,
}

impl ExecutionLimits {
    #[must_use]
    pub const fn new(step_limit: u64, frame_limit: u64, heap_byte_limit: u64) -> Self {
        Self {
            step_limit,
            frame_limit,
            heap_byte_limit,
        }
    }

    #[must_use]
    pub const fn step_limit(self) -> u64 {
        self.step_limit
    }

    #[must_use]
    pub const fn frame_limit(self) -> u64 {
        self.frame_limit
    }

    #[must_use]
    pub const fn heap_byte_limit(self) -> u64 {
        self.heap_byte_limit
    }
}

/// Executes the verified `Main.main (Unit) -> Unit` entry point.
///
/// Capability preflight completes before the first instruction. Instructions
/// and terminators are charged exactly once immediately before execution.
pub fn execute_v1(
    program: &VerifiedProgramV1,
    limits: ExecutionLimits,
    host: &mut HostCapabilities<'_>,
) -> Result<(), ExecutionError> {
    Engine::new(program.model(), limits, host).execute()
}

struct Engine<'program, 'host_ref, 'capability> {
    model: &'program UnverifiedProgram,
    limits: ExecutionLimits,
    host: &'host_ref mut HostCapabilities<'capability>,
    heap: Heap,
    frames: Vec<Frame>,
    steps: u64,
    committed: bool,
}

impl<'program, 'host_ref, 'capability> Engine<'program, 'host_ref, 'capability> {
    fn new(
        model: &'program UnverifiedProgram,
        limits: ExecutionLimits,
        host: &'host_ref mut HostCapabilities<'capability>,
    ) -> Self {
        Self {
            model,
            limits,
            host,
            heap: Heap::new(limits.heap_byte_limit),
            frames: Vec::new(),
            steps: 0,
            committed: false,
        }
    }

    fn execute(mut self) -> Result<(), ExecutionError> {
        let entry = self.model.entry().get();
        let entry_location = Location::new(entry, 0, 0);
        let entry_function = self.function(entry)?;
        if entry_function.effects.contains(&Effect::ConsoleWrite) && self.host.console.is_none() {
            return Err(self.runtime_error(
                RuntimeFaultKind::CapabilityUnavailable {
                    capability: "Console.Write",
                },
                entry_location,
            ));
        }

        self.push_frame(entry, &[Value::Unit], None, entry_location)?;
        loop {
            let cursor = self.cursor()?;
            let model = self.model;
            let function = model
                .functions()
                .get(to_usize(cursor.function)?)
                .ok_or_else(|| internal("verified function index is absent"))?;
            let block = function
                .blocks
                .get(to_usize(cursor.block)?)
                .ok_or_else(|| internal("verified block index is absent"))?;
            if cursor.instruction < block.instructions.len() {
                let ordinal = to_u32(cursor.instruction)?;
                let location = Location::new(cursor.function, cursor.block, ordinal);
                self.charge_step(location)?;
                let instruction = &block.instructions[cursor.instruction];
                self.current_frame_mut()?.instruction = cursor.instruction.saturating_add(1);
                self.execute_instruction(instruction, location)?;
            } else {
                let ordinal = to_u32(block.instructions.len())?;
                let location = Location::new(cursor.function, cursor.block, ordinal);
                self.charge_step(location)?;
                if self.execute_terminator(&block.terminator, location)? {
                    return Ok(());
                }
            }
        }
    }

    fn execute_instruction(
        &mut self,
        instruction: &Instruction,
        location: Location,
    ) -> Result<(), ExecutionError> {
        match instruction {
            Instruction::Const {
                destination,
                constant,
            } => {
                let constant = self
                    .model
                    .constants()
                    .get(to_usize(constant.get())?)
                    .ok_or_else(|| internal("verified constant index is absent"))?;
                let value = self.constant_value(constant, location)?;
                self.write_current(*destination, value)
            }
            Instruction::IntUnary {
                destination,
                operator,
                operand,
            } => {
                let operand = self.read_current(*operand)?;
                let value = self.execute_int_unary(*operator, operand, location)?;
                self.write_current(*destination, value)
            }
            Instruction::IntBinary {
                destination,
                operator,
                left,
                right,
            } => {
                let left = self.read_current(*left)?;
                let right = self.read_current(*right)?;
                let value = self.execute_int_binary(*operator, left, right, location)?;
                self.write_current(*destination, value)
            }
            Instruction::Compare {
                destination,
                operator,
                left,
                right,
            } => {
                let left = self.read_current(*left)?;
                let right = self.read_current(*right)?;
                let value = self.execute_compare(*operator, &left, &right)?;
                self.write_current(*destination, Value::Bool(value))
            }
            Instruction::Call {
                destination,
                function,
                arguments,
            } => {
                self.ensure_frame_slot(location)?;
                let values = self.collect_registers(arguments, location, "call_arguments")?;
                self.push_frame(function.get(), &values, Some(*destination), location)
            }
            Instruction::Intrinsic {
                destination,
                intrinsic,
                arguments,
            } => {
                let values = self.collect_registers(arguments, location, "intrinsic_arguments")?;
                let value = self.execute_intrinsic(*intrinsic, &values, location)?;
                self.write_current(*destination, value)
            }
            Instruction::ConsoleWrite { destination, text } => {
                let text = self.read_current(*text)?;
                let text = text
                    .as_text()
                    .ok_or_else(|| internal("verified ConsoleWrite operand is not Text"))?;
                let result = self
                    .host
                    .console
                    .as_deref_mut()
                    .ok_or_else(|| internal("preflighted Console.Write capability disappeared"))?
                    .write_line(text);
                match result {
                    Ok(()) => {
                        self.committed = true;
                        self.write_current(*destination, Value::Unit)
                    }
                    Err(error) => {
                        self.committed |= error.committed();
                        Err(self.runtime_error(
                            RuntimeFaultKind::HostCapability {
                                operation: "Console.write",
                                category: error.category(),
                            },
                            location,
                        ))
                    }
                }
            }
        }
    }

    fn execute_terminator(
        &mut self,
        terminator: &Terminator,
        location: Location,
    ) -> Result<bool, ExecutionError> {
        match terminator {
            Terminator::Jump { target, arguments } => {
                let values = self.collect_registers(arguments, location, "jump_arguments")?;
                self.enter_block(target.get(), &values)?;
                Ok(false)
            }
            Terminator::Branch {
                condition,
                true_target,
                true_arguments,
                false_target,
                false_arguments,
            } => {
                let condition = self
                    .read_current(*condition)?
                    .as_bool()
                    .ok_or_else(|| internal("verified Branch condition is not Bool"))?;
                let (target, arguments) = if condition {
                    (true_target, true_arguments)
                } else {
                    (false_target, false_arguments)
                };
                let values = self.collect_registers(arguments, location, "branch_arguments")?;
                self.enter_block(target.get(), &values)?;
                Ok(false)
            }
            Terminator::Return { value } => {
                let value = self.read_current(*value)?;
                let frame = self
                    .frames
                    .pop()
                    .ok_or_else(|| internal("return executed without an active frame"))?;
                match (self.frames.last_mut(), frame.return_destination) {
                    (Some(caller), Some(destination)) => {
                        write_register(caller, destination, value)?;
                        Ok(false)
                    }
                    (None, None) if value.is_unit() => Ok(true),
                    (None, None) => {
                        Err(internal("verified entry returned a non-Unit value").into())
                    }
                    _ => Err(internal("verified call return destination is inconsistent").into()),
                }
            }
        }
    }

    fn execute_int_unary(
        &mut self,
        operator: IntUnaryOperator,
        operand: Value,
        location: Location,
    ) -> Result<Value, ExecutionError> {
        let integer = operand
            .as_int()
            .ok_or_else(|| internal("verified integer unary operand is not Int"))?;
        match operator {
            IntUnaryOperator::Positive => Ok(operand),
            IntUnaryOperator::Negative if integer == &BigInt::from(0_u8) => Ok(operand),
            IntUnaryOperator::Negative => {
                let estimate = integer.bits().saturating_add(7) / 8;
                self.ensure_heap(estimate, "Int.negate", location)?;
                self.heap
                    .int(-integer)
                    .map_err(|()| self.out_of_memory("Int.negate", location))
            }
        }
    }

    fn execute_int_binary(
        &mut self,
        operator: IntBinaryOperator,
        left: Value,
        right: Value,
        location: Location,
    ) -> Result<Value, ExecutionError> {
        let left_integer = left
            .as_int()
            .ok_or_else(|| internal("verified left integer operand is not Int"))?;
        let right_integer = right
            .as_int()
            .ok_or_else(|| internal("verified right integer operand is not Int"))?;
        let zero = BigInt::from(0_u8);
        let (operation, estimate) = match operator {
            IntBinaryOperator::Add => (
                "Int.add",
                left_integer
                    .bits()
                    .max(right_integer.bits())
                    .saturating_add(8)
                    / 8,
            ),
            IntBinaryOperator::Subtract => (
                "Int.subtract",
                left_integer
                    .bits()
                    .max(right_integer.bits())
                    .saturating_add(8)
                    / 8,
            ),
            IntBinaryOperator::Multiply => (
                "Int.multiply",
                left_integer
                    .bits()
                    .saturating_add(right_integer.bits())
                    .saturating_add(7)
                    / 8,
            ),
            IntBinaryOperator::Divide => ("Int.divide", left_integer.bits().saturating_add(7) / 8),
            IntBinaryOperator::Remainder => {
                ("Int.remainder", left_integer.bits().saturating_add(7) / 8)
            }
        };
        if matches!(
            operator,
            IntBinaryOperator::Divide | IntBinaryOperator::Remainder
        ) && right_integer == &zero
        {
            return Err(
                self.runtime_error(RuntimeFaultKind::DivisionByZero { operation }, location)
            );
        }
        self.ensure_heap(estimate, operation, location)?;
        let result = match operator {
            IntBinaryOperator::Add => left_integer + right_integer,
            IntBinaryOperator::Subtract => left_integer - right_integer,
            IntBinaryOperator::Multiply => left_integer * right_integer,
            IntBinaryOperator::Divide => left_integer / right_integer,
            IntBinaryOperator::Remainder => left_integer % right_integer,
        };
        self.heap
            .int(result)
            .map_err(|()| self.out_of_memory(operation, location))
    }

    fn execute_compare(
        &self,
        operator: CompareOperator,
        left: &Value,
        right: &Value,
    ) -> Result<bool, ExecutionError> {
        match operator {
            CompareOperator::BoolEqual | CompareOperator::BoolNotEqual => {
                let left = left
                    .as_bool()
                    .ok_or_else(|| internal("verified Bool comparison operand is not Bool"))?;
                let right = right
                    .as_bool()
                    .ok_or_else(|| internal("verified Bool comparison operand is not Bool"))?;
                Ok(if operator == CompareOperator::BoolEqual {
                    left == right
                } else {
                    left != right
                })
            }
            CompareOperator::IntEqual
            | CompareOperator::IntNotEqual
            | CompareOperator::IntLess
            | CompareOperator::IntLessEqual
            | CompareOperator::IntGreater
            | CompareOperator::IntGreaterEqual => {
                let left = left
                    .as_int()
                    .ok_or_else(|| internal("verified Int comparison operand is not Int"))?;
                let right = right
                    .as_int()
                    .ok_or_else(|| internal("verified Int comparison operand is not Int"))?;
                Ok(match operator {
                    CompareOperator::IntEqual => left == right,
                    CompareOperator::IntNotEqual => left != right,
                    CompareOperator::IntLess => left < right,
                    CompareOperator::IntLessEqual => left <= right,
                    CompareOperator::IntGreater => left > right,
                    CompareOperator::IntGreaterEqual => left >= right,
                    _ => return Err(internal("integer comparison dispatch is incomplete").into()),
                })
            }
            CompareOperator::TextEqual | CompareOperator::TextNotEqual => {
                let left = left
                    .as_text()
                    .ok_or_else(|| internal("verified Text comparison operand is not Text"))?;
                let right = right
                    .as_text()
                    .ok_or_else(|| internal("verified Text comparison operand is not Text"))?;
                Ok(if operator == CompareOperator::TextEqual {
                    left == right
                } else {
                    left != right
                })
            }
        }
    }

    fn execute_intrinsic(
        &mut self,
        intrinsic: Intrinsic,
        arguments: &[Value],
        location: Location,
    ) -> Result<Value, ExecutionError> {
        match intrinsic {
            Intrinsic::TextFormat => {
                let template = arguments
                    .first()
                    .and_then(Value::as_text)
                    .ok_or_else(|| internal("verified Text.format template is not Text"))?;
                let integer = arguments
                    .get(1)
                    .and_then(Value::as_int)
                    .ok_or_else(|| internal("verified Text.format value is not Int"))?;
                let count = template.match_indices("{}").count();
                if count != 1 {
                    let count = u32::try_from(count).map_err(|_| {
                        internal("verified Text.format placeholder count does not fit u32")
                    })?;
                    return Err(self.runtime_error(
                        RuntimeFaultKind::InvalidFormatPlaceholderCount { count },
                        location,
                    ));
                }
                let integer_bytes = integer.bits().saturating_add(7) / 8;
                let sign_bytes = u64::from(integer.sign() == Sign::Minus);
                let digit_bound = integer_bytes
                    .saturating_mul(3)
                    .saturating_add(sign_bytes)
                    .max(1);
                let template_bytes = u64::try_from(template.len()).map_err(|_| {
                    internal("verified Text.format template length does not fit u64")
                })?;
                let estimate = template_bytes.saturating_sub(2).saturating_add(digit_bound);
                self.ensure_heap(estimate, "Text.format", location)?;

                let rendered_integer = integer.to_string();
                let output_length = template
                    .len()
                    .checked_sub(2)
                    .and_then(|value| value.checked_add(rendered_integer.len()))
                    .ok_or_else(|| self.out_of_memory("Text.format", location))?;
                let mut output = String::new();
                output
                    .try_reserve_exact(output_length)
                    .map_err(|_| self.out_of_memory("Text.format", location))?;
                let placeholder = template
                    .find("{}")
                    .ok_or_else(|| internal("counted Text.format placeholder is absent"))?;
                output.push_str(&template[..placeholder]);
                output.push_str(&rendered_integer);
                output.push_str(&template[placeholder + 2..]);
                self.heap
                    .text(output)
                    .map_err(|()| self.out_of_memory("Text.format", location))
            }
            Intrinsic::MaxInt | Intrinsic::MinInt => {
                let left = arguments
                    .first()
                    .ok_or_else(|| internal("verified max/min left argument is absent"))?;
                let right = arguments
                    .get(1)
                    .ok_or_else(|| internal("verified max/min right argument is absent"))?;
                let left_integer = left
                    .as_int()
                    .ok_or_else(|| internal("verified max/min left argument is not Int"))?;
                let right_integer = right
                    .as_int()
                    .ok_or_else(|| internal("verified max/min right argument is not Int"))?;
                let choose_left = match intrinsic {
                    Intrinsic::MaxInt => left_integer >= right_integer,
                    Intrinsic::MinInt => left_integer <= right_integer,
                    Intrinsic::TextFormat => {
                        return Err(internal("intrinsic dispatch is incomplete").into());
                    }
                };
                Ok(if choose_left {
                    left.clone()
                } else {
                    right.clone()
                })
            }
        }
    }

    fn constant_value(
        &mut self,
        constant: &Constant,
        location: Location,
    ) -> Result<Value, ExecutionError> {
        match constant {
            Constant::Unit => Ok(Value::Unit),
            Constant::Bool(value) => Ok(Value::Bool(*value)),
            Constant::Int { sign, magnitude } => {
                let bytes = u64::try_from(magnitude.len())
                    .map_err(|_| internal("verified integer magnitude length does not fit u64"))?;
                self.ensure_heap(bytes, "constant_int", location)?;
                let sign = match sign {
                    IntegerSign::Zero | IntegerSign::Positive => Sign::Plus,
                    IntegerSign::Negative => Sign::Minus,
                };
                let integer = BigInt::from_bytes_be(sign, magnitude);
                self.heap
                    .int(integer)
                    .map_err(|()| self.out_of_memory("constant_int", location))
            }
            Constant::Text(index) => {
                let text = self
                    .model
                    .strings()
                    .get(to_usize(index.get())?)
                    .ok_or_else(|| internal("verified Text constant string is absent"))?;
                let bytes = u64::try_from(text.len())
                    .map_err(|_| internal("verified Text constant length does not fit u64"))?;
                self.ensure_heap(bytes, "constant_text", location)?;
                let mut owned = String::new();
                owned
                    .try_reserve_exact(text.len())
                    .map_err(|_| self.out_of_memory("constant_text", location))?;
                owned.push_str(text);
                self.heap
                    .text(owned)
                    .map_err(|()| self.out_of_memory("constant_text", location))
            }
        }
    }

    fn charge_step(&mut self, location: Location) -> Result<(), ExecutionError> {
        if self.steps >= self.limits.step_limit {
            return Err(self.runtime_error(
                RuntimeFaultKind::ResourceLimit {
                    resource: RuntimeResource::Step,
                },
                location,
            ));
        }
        self.steps = self.steps.saturating_add(1);
        Ok(())
    }

    fn ensure_frame_slot(&self, location: Location) -> Result<(), ExecutionError> {
        let active = u64::try_from(self.frames.len())
            .map_err(|_| internal("active frame count does not fit u64"))?;
        if active >= self.limits.frame_limit {
            return Err(self.runtime_error(
                RuntimeFaultKind::ResourceLimit {
                    resource: RuntimeResource::Frame,
                },
                location,
            ));
        }
        Ok(())
    }

    fn ensure_heap(
        &self,
        bytes: u64,
        operation: &'static str,
        location: Location,
    ) -> Result<(), ExecutionError> {
        if self.heap.can_allocate(bytes) {
            Ok(())
        } else {
            Err(self.out_of_memory(operation, location))
        }
    }

    fn push_frame(
        &mut self,
        function_index: u32,
        arguments: &[Value],
        return_destination: Option<RegisterIndex>,
        location: Location,
    ) -> Result<(), ExecutionError> {
        self.ensure_frame_slot(location)?;
        self.frames
            .try_reserve(1)
            .map_err(|_| self.out_of_memory("frame_stack", location))?;
        let function = self.function(function_index)?;
        let entry = function
            .blocks
            .first()
            .ok_or_else(|| internal("verified function has no entry block"))?;
        if entry.parameters.len() != arguments.len() {
            return Err(internal("verified call argument count changed after verification").into());
        }
        let register_count = to_usize(function.register_count)?;
        let mut frame = Frame::new(function_index, register_count, return_destination)
            .map_err(|()| self.out_of_memory("frame_registers", location))?;
        for (parameter, value) in entry.parameters.iter().zip(arguments) {
            write_register(&mut frame, parameter.register, value.clone())?;
        }
        self.frames.push(frame);
        Ok(())
    }

    fn enter_block(&mut self, block_index: u32, arguments: &[Value]) -> Result<(), ExecutionError> {
        let function_index = self.current_frame()?.function;
        let model = self.model;
        let function = model
            .functions()
            .get(to_usize(function_index)?)
            .ok_or_else(|| internal("verified function index is absent"))?;
        let block = function
            .blocks
            .get(to_usize(block_index)?)
            .ok_or_else(|| internal("verified block index is absent"))?;
        if block.parameters.len() != arguments.len() {
            return Err(
                internal("verified block argument count changed after verification").into(),
            );
        }
        let frame = self.current_frame_mut()?;
        frame.block = block_index;
        frame.instruction = 0;
        for (parameter, value) in block.parameters.iter().zip(arguments) {
            write_register(frame, parameter.register, value.clone())?;
        }
        Ok(())
    }

    fn collect_registers(
        &self,
        registers: &[RegisterIndex],
        location: Location,
        operation: &'static str,
    ) -> Result<Vec<Value>, ExecutionError> {
        let mut values = Vec::new();
        values
            .try_reserve_exact(registers.len())
            .map_err(|_| self.out_of_memory(operation, location))?;
        for register in registers {
            values.push(self.read_current(*register)?);
        }
        Ok(values)
    }

    fn read_current(&self, register: RegisterIndex) -> Result<Value, ExecutionError> {
        let frame = self.current_frame()?;
        frame
            .registers
            .get(to_usize(register.get())?)
            .ok_or_else(|| internal("verified register index is absent"))?
            .clone()
            .ok_or_else(|| internal("verified register was read before runtime assignment").into())
    }

    fn write_current(
        &mut self,
        register: RegisterIndex,
        value: Value,
    ) -> Result<(), ExecutionError> {
        write_register(self.current_frame_mut()?, register, value)
    }

    fn function(&self, index: u32) -> Result<&ling_bytecode::Function, ExecutionError> {
        self.model
            .functions()
            .get(to_usize(index)?)
            .ok_or_else(|| internal("verified function index is absent").into())
    }

    fn cursor(&self) -> Result<Cursor, ExecutionError> {
        let frame = self.current_frame()?;
        Ok(Cursor {
            function: frame.function,
            block: frame.block,
            instruction: frame.instruction,
        })
    }

    fn current_frame(&self) -> Result<&Frame, ExecutionError> {
        self.frames
            .last()
            .ok_or_else(|| internal("execution has no active frame").into())
    }

    fn current_frame_mut(&mut self) -> Result<&mut Frame, ExecutionError> {
        self.frames
            .last_mut()
            .ok_or_else(|| internal("execution has no active frame").into())
    }

    fn runtime_error(&self, kind: RuntimeFaultKind, location: Location) -> ExecutionError {
        self.mapped_runtime_fault(kind, location)
            .map_or_else(ExecutionError::Internal, ExecutionError::Runtime)
    }

    fn mapped_runtime_fault(
        &self,
        kind: RuntimeFaultKind,
        location: Location,
    ) -> Result<RuntimeFault, InternalExecutionError> {
        let source_map = self.model.source_map();
        let key = (location.function, location.block, location.ordinal);
        let index = source_map
            .binary_search_by_key(&key, |entry| {
                (entry.function.get(), entry.block.get(), entry.ordinal)
            })
            .map_err(|_| internal("verified executable location has no source map"))?;
        let entry = source_map
            .get(index)
            .ok_or_else(|| internal("verified source-map index is absent"))?;
        let source = self
            .model
            .sources()
            .get(to_usize(entry.source.get())?)
            .ok_or_else(|| internal("verified source index is absent"))?;
        let source_name = self
            .model
            .strings()
            .get(to_usize(source.logical_name.get())?)
            .ok_or_else(|| internal("verified source logical-name index is absent"))?
            .clone();
        Ok(RuntimeFault::new(
            kind,
            source_name,
            entry.span,
            self.committed,
        ))
    }

    fn out_of_memory(&self, operation: &'static str, location: Location) -> ExecutionError {
        self.runtime_error(RuntimeFaultKind::OutOfMemory { operation }, location)
    }
}

struct Frame {
    function: u32,
    block: u32,
    instruction: usize,
    registers: Vec<Option<Value>>,
    return_destination: Option<RegisterIndex>,
}

impl Frame {
    fn new(
        function: u32,
        register_count: usize,
        return_destination: Option<RegisterIndex>,
    ) -> Result<Self, ()> {
        let mut registers = Vec::new();
        registers
            .try_reserve_exact(register_count)
            .map_err(|_| ())?;
        registers.resize_with(register_count, || None);
        Ok(Self {
            function,
            block: 0,
            instruction: 0,
            registers,
            return_destination,
        })
    }
}

#[derive(Clone, Copy)]
struct Cursor {
    function: u32,
    block: u32,
    instruction: usize,
}

#[derive(Clone, Copy)]
struct Location {
    function: u32,
    block: u32,
    ordinal: u32,
}

impl Location {
    const fn new(function: u32, block: u32, ordinal: u32) -> Self {
        Self {
            function,
            block,
            ordinal,
        }
    }
}

fn write_register(
    frame: &mut Frame,
    register: RegisterIndex,
    value: Value,
) -> Result<(), ExecutionError> {
    let slot = frame
        .registers
        .get_mut(to_usize(register.get())?)
        .ok_or_else(|| internal("verified destination register index is absent"))?;
    *slot = Some(value);
    Ok(())
}

fn internal(invariant: &'static str) -> InternalExecutionError {
    InternalExecutionError::new(invariant)
}

fn to_usize(value: u32) -> Result<usize, InternalExecutionError> {
    usize::try_from(value).map_err(|_| internal("verified u32 index does not fit host usize"))
}

fn to_u32(value: usize) -> Result<u32, InternalExecutionError> {
    u32::try_from(value).map_err(|_| internal("verified executable ordinal does not fit u32"))
}
