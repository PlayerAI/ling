use std::collections::BTreeMap;
use std::panic::{AssertUnwindSafe, catch_unwind};

use ling_bytecode::{
    CaptureOperand, CompareOperator, Constant, HandlerOperation, Instruction, IntBinaryOperator,
    IntUnaryOperator, IntegerSign, Intrinsic, RegisterIndex, Terminator, UnverifiedProgram,
    ValueType, VerifiedProgramV1,
};
use num_bigint::{BigInt, Sign};

use crate::cancel::CancellationToken;
use crate::fault::{
    ExecutionError, InternalExecutionError, RuntimeFault, RuntimeFaultKind, RuntimeResource,
};
use crate::host::{HostCapabilities, HostError, HostErrorCategory};
use crate::value::{Allocation, BoundValue, Closure, Heap, HeapCharge, Value};

/// Explicit execution limits required by RFC-0014.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutionLimits {
    step_limit: u64,
    frame_limit: u64,
    heap_byte_limit: u64,
    handler_depth_limit: u64,
    continuation_frame_limit: u64,
}

impl ExecutionLimits {
    #[must_use]
    pub const fn new(step_limit: u64, frame_limit: u64, heap_byte_limit: u64) -> Self {
        Self {
            step_limit,
            frame_limit,
            heap_byte_limit,
            handler_depth_limit: frame_limit,
            continuation_frame_limit: frame_limit,
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

    /// Overrides the handler-depth and captured-continuation-frame ceilings.
    #[must_use]
    pub const fn with_handler_limits(
        mut self,
        handler_depth_limit: u64,
        continuation_frame_limit: u64,
    ) -> Self {
        self.handler_depth_limit = handler_depth_limit;
        self.continuation_frame_limit = continuation_frame_limit;
        self
    }

    #[must_use]
    pub const fn handler_depth_limit(self) -> u64 {
        self.handler_depth_limit
    }

    #[must_use]
    pub const fn continuation_frame_limit(self) -> u64 {
        self.continuation_frame_limit
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
    execute_v1_inner(program, limits, host, None)
}

/// Executes the verified entry point with a host-owned cancellation request.
///
/// Cancellation is checked before capability preflight and immediately before
/// every instruction or terminator. A request is cooperative: the currently
/// executing host operation is allowed to finish, and effects committed before
/// the next checkpoint remain committed.
pub fn execute_v1_with_cancellation(
    program: &VerifiedProgramV1,
    limits: ExecutionLimits,
    host: &mut HostCapabilities<'_>,
    cancellation: &CancellationToken,
) -> Result<(), ExecutionError> {
    execute_v1_inner(program, limits, host, Some(cancellation))
}

fn execute_v1_inner(
    program: &VerifiedProgramV1,
    limits: ExecutionLimits,
    host: &mut HostCapabilities<'_>,
    cancellation: Option<&CancellationToken>,
) -> Result<(), ExecutionError> {
    Engine::new(
        program.model(),
        program.entry_console_capability_required(),
        limits,
        host,
        cancellation,
    )
    .execute()
}

struct Engine<'program, 'host_ref, 'capability, 'cancel> {
    model: &'program UnverifiedProgram,
    entry_console_capability_required: bool,
    limits: ExecutionLimits,
    host: &'host_ref mut HostCapabilities<'capability>,
    cancellation: Option<&'cancel CancellationToken>,
    heap: Heap,
    cells: Vec<CellEntry>,
    frames: Vec<Frame>,
    handlers: Vec<ActiveHandler>,
    continuations: BTreeMap<u64, ContinuationState>,
    resume_returns: BTreeMap<u64, ResumeReturn>,
    next_runtime_id: u64,
    steps: u64,
    committed: bool,
}

impl<'program, 'host_ref, 'capability, 'cancel> Engine<'program, 'host_ref, 'capability, 'cancel> {
    fn new(
        model: &'program UnverifiedProgram,
        entry_console_capability_required: bool,
        limits: ExecutionLimits,
        host: &'host_ref mut HostCapabilities<'capability>,
        cancellation: Option<&'cancel CancellationToken>,
    ) -> Self {
        Self {
            model,
            entry_console_capability_required,
            limits,
            host,
            cancellation,
            heap: Heap::new(limits.heap_byte_limit),
            cells: Vec::new(),
            frames: Vec::new(),
            handlers: Vec::new(),
            continuations: BTreeMap::new(),
            resume_returns: BTreeMap::new(),
            next_runtime_id: 0,
            steps: 0,
            committed: false,
        }
    }

    fn execute(mut self) -> Result<(), ExecutionError> {
        let entry = self.model.entry().get();
        let entry_location = Location::new(entry, 0, 0);
        self.check_cancellation(entry_location)?;
        if self.entry_console_capability_required && self.host.console.is_none() {
            return Err(self.runtime_error(
                RuntimeFaultKind::CapabilityUnavailable {
                    capability: "Console.Write",
                },
                entry_location,
            ));
        }

        self.push_frame(entry, &[Value::Unit], ReturnTarget::Entry, entry_location)?;
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
                self.check_cancellation(location)?;
                self.charge_step(location)?;
                let instruction = &block.instructions[cursor.instruction];
                self.current_frame_mut()?.instruction = cursor.instruction.saturating_add(1);
                self.execute_instruction(instruction, location)?;
            } else {
                let ordinal = to_u32(block.instructions.len())?;
                let location = Location::new(cursor.function, cursor.block, ordinal);
                self.check_cancellation(location)?;
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
                self.push_frame(
                    function.get(),
                    &values,
                    ReturnTarget::Register(*destination),
                    location,
                )
            }
            Instruction::MakeClosure {
                destination,
                function,
                captures,
            } => {
                let mut bound = Vec::new();
                bound
                    .try_reserve_exact(captures.len())
                    .map_err(|_| self.out_of_memory("make_closure", location))?;
                for capture in captures {
                    bound.push(match capture {
                        CaptureOperand::Register(register) => {
                            BoundValue::Value(self.read_current(*register)?)
                        }
                        CaptureOperand::SelfReference => BoundValue::SelfReference,
                    });
                }
                let closure = self
                    .heap
                    .closure(function.get(), bound)
                    .map_err(|()| self.out_of_memory("make_closure", location))?;
                self.write_current(*destination, closure)
            }
            Instruction::CallClosure {
                destination,
                callee,
                arguments,
            } => self.execute_call_closure(*destination, *callee, arguments, location),
            Instruction::Handle {
                destination,
                body_function,
                body_captures,
                clauses,
            } => {
                let handler_count = u64::try_from(self.handlers.len())
                    .map_err(|_| internal("handler depth does not fit u64"))?;
                if handler_count >= self.limits.handler_depth_limit {
                    return Err(self.runtime_error(
                        RuntimeFaultKind::ResourceLimit {
                            resource: RuntimeResource::HandlerDepth,
                        },
                        location,
                    ));
                }
                self.handlers
                    .try_reserve(1)
                    .map_err(|_| self.out_of_memory("handler_stack", location))?;

                let body = self.capture_closure(body_function.get(), body_captures, location)?;
                let mut runtime_clauses = Vec::new();
                runtime_clauses
                    .try_reserve_exact(clauses.len())
                    .map_err(|_| self.out_of_memory("handler_clauses", location))?;
                for clause in clauses {
                    runtime_clauses.push(RuntimeHandlerClause {
                        operation: clause.operation,
                        resume_present: clause.resume_present,
                        closure: self.capture_closure(
                            clause.function.get(),
                            &clause.captures,
                            location,
                        )?,
                    });
                }
                let handler_id = self.allocate_runtime_id()?;
                let boundary_depth = self.frames.len();
                self.handlers.push(ActiveHandler {
                    id: handler_id,
                    boundary_depth,
                    destination: *destination,
                    clauses: runtime_clauses,
                });
                let body_closure = body
                    .as_closure()
                    .cloned()
                    .ok_or_else(|| internal("constructed Handler body is not a closure"))?;
                let body_arguments = self.materialize_closure_bound(&body_closure, location)?;
                self.push_frame(
                    body_function.get(),
                    &body_arguments,
                    ReturnTarget::HandlerBody {
                        handler_id,
                        destination: *destination,
                    },
                    location,
                )
            }
            Instruction::MakeTuple {
                destination,
                elements,
                ..
            } => {
                let values = self.collect_registers(elements, location, "make_tuple")?;
                let count = u64::try_from(values.len())
                    .map_err(|_| internal("tuple element count does not fit u64"))?;
                let bytes = count
                    .checked_mul(16)
                    .and_then(|bytes| bytes.checked_add(16))
                    .ok_or_else(|| self.out_of_memory("make_tuple", location))?;
                self.ensure_heap(bytes, "make_tuple", location)?;
                let tuple = self
                    .heap
                    .tuple(values)
                    .map_err(|()| self.out_of_memory("make_tuple", location))?;
                self.write_current(*destination, tuple)
            }
            Instruction::GetTuple {
                destination,
                tuple,
                element,
            } => {
                let tuple_value = self.read_current(*tuple)?;
                let tuple = tuple_value
                    .as_tuple()
                    .ok_or_else(|| internal("verified GetTuple operand is not a tuple"))?;
                let value = tuple
                    .value()
                    .get(to_usize(*element)?)
                    .cloned()
                    .ok_or_else(|| internal("verified GetTuple index is absent"))?;
                self.write_current(*destination, value)
            }
            Instruction::MakeRecord {
                destination,
                record,
                fields,
            } => {
                let values = self.collect_registers(fields, location, "make_record")?;
                let count = u64::try_from(values.len())
                    .map_err(|_| internal("record field count does not fit u64"))?;
                let bytes = count
                    .checked_mul(16)
                    .and_then(|bytes| bytes.checked_add(24))
                    .ok_or_else(|| self.out_of_memory("make_record", location))?;
                self.ensure_heap(bytes, "make_record", location)?;
                let record = self
                    .heap
                    .record(record.get(), values)
                    .map_err(|()| self.out_of_memory("make_record", location))?;
                self.write_current(*destination, record)
            }
            Instruction::GetField {
                destination,
                record,
                field,
            } => {
                let record_value = self.read_current(*record)?;
                let record = record_value
                    .as_record()
                    .ok_or_else(|| internal("verified GetField operand is not a record"))?;
                let value = record
                    .value()
                    .fields()
                    .get(to_usize(*field)?)
                    .cloned()
                    .ok_or_else(|| internal("verified GetField index is absent"))?;
                self.write_current(*destination, value)
            }
            Instruction::UpdateRecord {
                destination,
                base,
                updates,
            } => {
                let base_value = self.read_current(*base)?;
                let base_record = base_value
                    .as_record()
                    .ok_or_else(|| internal("verified UpdateRecord base is not a record"))?;
                let type_index = base_record.value().type_index();
                let mut fields = base_record.value().fields().to_vec();
                for update in updates {
                    let field = fields
                        .get_mut(to_usize(update.field)?)
                        .ok_or_else(|| internal("verified UpdateRecord field is absent"))?;
                    *field = self.read_current(update.value)?;
                }
                let count = u64::try_from(fields.len())
                    .map_err(|_| internal("record field count does not fit u64"))?;
                let bytes = count
                    .checked_mul(16)
                    .and_then(|bytes| bytes.checked_add(24))
                    .ok_or_else(|| self.out_of_memory("update_record", location))?;
                self.ensure_heap(bytes, "update_record", location)?;
                let record = self
                    .heap
                    .record(type_index, fields)
                    .map_err(|()| self.out_of_memory("update_record", location))?;
                self.write_current(*destination, record)
            }
            Instruction::MakeVariant {
                destination,
                variant,
                case,
                payload,
            } => {
                let payload_value = payload
                    .map(|register| self.read_current(register))
                    .transpose()?;
                let bytes = 32_u64 + u64::from(payload_value.is_some()) * 16;
                self.ensure_heap(bytes, "make_variant", location)?;
                let variant = self
                    .heap
                    .variant(variant.get(), *case, payload_value)
                    .map_err(|()| self.out_of_memory("make_variant", location))?;
                self.write_current(*destination, variant)
            }
            Instruction::VariantIs {
                variant,
                case,
                destination,
            } => {
                let variant_value = self.read_current(*variant)?;
                let variant = variant_value
                    .as_variant()
                    .ok_or_else(|| internal("verified VariantIs operand is not a variant"))?;
                let cases = self.variant_cases(variant.value().type_index())?;
                if to_usize(*case)? >= cases.len() {
                    return Err(internal("verified VariantIs case is absent").into());
                }
                self.write_current(*destination, Value::Bool(variant.value().case() == *case))
            }
            Instruction::GetVariantPayload {
                destination,
                variant,
                case,
            } => {
                let variant_value = self.read_current(*variant)?;
                let variant = variant_value.as_variant().ok_or_else(|| {
                    internal("verified GetVariantPayload operand is not a variant")
                })?;
                if variant.value().case() != *case {
                    return Err(
                        internal("verified GetVariantPayload case does not match tag").into(),
                    );
                }
                let payload = variant
                    .value()
                    .payload()
                    .cloned()
                    .ok_or_else(|| internal("verified GetVariantPayload has no payload"))?;
                self.write_current(*destination, payload)
            }
            Instruction::CellNew {
                destination,
                initial,
            } => {
                const CELL_BYTES: u64 = 24;
                let value = self.read_current(*initial)?;
                let charge = self
                    .heap
                    .charge(CELL_BYTES)
                    .map_err(|()| self.out_of_memory("cell.new", location))?;
                self.cells
                    .try_reserve(1)
                    .map_err(|_| self.out_of_memory("cell.new", location))?;
                let id = u64::try_from(self.cells.len())
                    .map_err(|_| internal("Cell count does not fit private CellId"))?;
                self.cells.push(CellEntry {
                    value,
                    _charge: charge,
                });
                self.write_current(*destination, Value::Cell(id))
            }
            Instruction::CellGet { destination, cell } => {
                let id = self
                    .read_current(*cell)?
                    .as_cell_id()
                    .ok_or_else(|| internal("verified CellGet operand is not a Cell"))?;
                let value = self
                    .cells
                    .get(to_usize_u64(id)?)
                    .ok_or_else(|| internal("verified CellGet identity is absent"))?
                    .value
                    .clone();
                self.write_current(*destination, value)
            }
            Instruction::CellSet {
                destination,
                cell,
                value,
            } => {
                let id = self
                    .read_current(*cell)?
                    .as_cell_id()
                    .ok_or_else(|| internal("verified CellSet operand is not a Cell"))?;
                let replacement = self.read_current(*value)?;
                self.check_cancellation(location)?;
                self.cells
                    .get_mut(to_usize_u64(id)?)
                    .ok_or_else(|| internal("verified CellSet identity is absent"))?
                    .value = replacement;
                self.committed = true;
                self.write_current(*destination, Value::Unit)
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
                let text_value = self.read_current(*text)?;
                let text = text_value
                    .as_text()
                    .ok_or_else(|| internal("verified ConsoleWrite operand is not Text"))?;
                if self
                    .handlers
                    .iter()
                    .rposition(|handler| {
                        handler
                            .clauses
                            .iter()
                            .any(|clause| clause.operation == HandlerOperation::ConsoleWrite)
                    })
                    .is_some()
                {
                    return self.dispatch_console_handler(*destination, text_value, location);
                }
                let console =
                    self.host.console.as_deref_mut().ok_or_else(|| {
                        internal("preflighted Console.Write capability disappeared")
                    })?;
                let result = match catch_unwind(AssertUnwindSafe(|| console.write_line(text))) {
                    Ok(result) => result,
                    Err(_) => Err(HostError::after_commit(HostErrorCategory::Other)),
                };
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
                self.finish_return(frame.return_target, value)
            }
        }
    }

    fn finish_return(
        &mut self,
        target: ReturnTarget,
        value: Value,
    ) -> Result<bool, ExecutionError> {
        match target {
            ReturnTarget::Entry if self.frames.is_empty() && value.is_unit() => Ok(true),
            ReturnTarget::Entry if self.frames.is_empty() => {
                Err(internal("verified entry returned a non-Unit value").into())
            }
            ReturnTarget::Register(destination) => {
                write_register(self.current_frame_mut()?, destination, value)?;
                Ok(false)
            }
            ReturnTarget::HandlerBody {
                handler_id,
                destination,
            } => {
                self.pop_handler(handler_id)?;
                write_register(self.current_frame_mut()?, destination, value)?;
                Ok(false)
            }
            ReturnTarget::HandlerClause {
                continuation_id,
                completion,
            } => {
                if let Some(continuation_id) = continuation_id {
                    self.continuations.remove(&continuation_id);
                }
                self.finish_clause(completion, value)
            }
            ReturnTarget::ResumedBody {
                handler_id,
                resume_return_id,
            } => {
                self.pop_handler(handler_id)?;
                let suspended = self
                    .resume_returns
                    .remove(&resume_return_id)
                    .ok_or_else(|| internal("verified resume return boundary is absent"))?;
                self.frames.extend(suspended.frames);
                self.handlers.extend(suspended.handlers);
                let state = self
                    .continuations
                    .get(&suspended.continuation_id)
                    .ok_or_else(|| internal("active continuation state is absent"))?;
                if !state.active {
                    return Err(internal("resumed continuation became inactive").into());
                }
                write_register(self.current_frame_mut()?, suspended.destination, value)?;
                Ok(false)
            }
            _ => Err(internal("verified call return target is inconsistent").into()),
        }
    }

    fn finish_clause(
        &mut self,
        completion: ClauseCompletion,
        value: Value,
    ) -> Result<bool, ExecutionError> {
        match completion {
            ClauseCompletion::Handler { destination } => {
                write_register(self.current_frame_mut()?, destination, value)?;
                Ok(false)
            }
            ClauseCompletion::Resume { resume_return_id } => {
                let suspended = self
                    .resume_returns
                    .remove(&resume_return_id)
                    .ok_or_else(|| internal("verified resume return boundary is absent"))?;
                self.frames.extend(suspended.frames);
                self.handlers.extend(suspended.handlers);
                if !self.continuations.contains_key(&suspended.continuation_id) {
                    return Err(internal("active continuation state is absent").into());
                }
                write_register(self.current_frame_mut()?, suspended.destination, value)?;
                Ok(false)
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

    fn execute_call_closure(
        &mut self,
        destination: RegisterIndex,
        callee: RegisterIndex,
        argument_registers: &[RegisterIndex],
        location: Location,
    ) -> Result<(), ExecutionError> {
        let callee_value = self.read_current(callee)?;
        if let Some(continuation_id) = callee_value.as_continuation_id() {
            return self.execute_resume(continuation_id, destination, argument_registers, location);
        }
        let closure = callee_value
            .as_closure()
            .cloned()
            .ok_or_else(|| internal("verified CallClosure callee is not a closure"))?;
        let function_index = closure.value().function();
        let parameter_count = self.function(function_index)?.parameter_types.len();
        let bound_count = closure.value().bound().len();
        let remaining = parameter_count.checked_sub(bound_count).ok_or_else(|| {
            internal("verified closure has more bound values than target parameters")
        })?;
        if argument_registers.is_empty() || argument_registers.len() > remaining {
            return Err(
                internal("verified CallClosure argument count changed after verification").into(),
            );
        }
        let complete = argument_registers.len() == remaining;
        if complete {
            self.ensure_frame_slot(location)?;
        }

        let arguments =
            self.collect_registers(argument_registers, location, "call_closure_arguments")?;
        let mut bound = self.materialize_closure_bound(&closure, location)?;
        bound
            .try_reserve_exact(arguments.len())
            .map_err(|_| self.out_of_memory("call_closure_arguments", location))?;
        bound.extend(arguments);

        if complete {
            self.push_frame(
                function_index,
                &bound,
                ReturnTarget::Register(destination),
                location,
            )
        } else {
            let mut partial_bound = Vec::new();
            partial_bound
                .try_reserve_exact(bound.len())
                .map_err(|_| self.out_of_memory("partial_application", location))?;
            partial_bound.extend(bound.into_iter().map(BoundValue::Value));
            let partial = self
                .heap
                .closure(function_index, partial_bound)
                .map_err(|()| self.out_of_memory("partial_application", location))?;
            self.write_current(destination, partial)
        }
    }

    fn capture_closure(
        &mut self,
        function: u32,
        captures: &[CaptureOperand],
        location: Location,
    ) -> Result<Value, ExecutionError> {
        let mut bound = Vec::new();
        bound
            .try_reserve_exact(captures.len())
            .map_err(|_| self.out_of_memory("handler_closure", location))?;
        for capture in captures {
            bound.push(match capture {
                CaptureOperand::Register(register) => {
                    BoundValue::Value(self.read_current(*register)?)
                }
                CaptureOperand::SelfReference => BoundValue::SelfReference,
            });
        }
        self.heap
            .closure(function, bound)
            .map_err(|()| self.out_of_memory("handler_closure", location))
    }

    fn dispatch_console_handler(
        &mut self,
        operation_destination: RegisterIndex,
        input: Value,
        location: Location,
    ) -> Result<(), ExecutionError> {
        self.check_cancellation(location)?;
        let selected_index = self
            .handlers
            .iter()
            .rposition(|handler| {
                handler
                    .clauses
                    .iter()
                    .any(|clause| clause.operation == HandlerOperation::ConsoleWrite)
            })
            .ok_or_else(|| internal("handler dispatch has no selected clause"))?;
        let selected = self.handlers[selected_index].clone();
        let clause = selected
            .clauses
            .iter()
            .find(|clause| clause.operation == HandlerOperation::ConsoleWrite)
            .cloned()
            .ok_or_else(|| internal("selected Handler clause is absent"))?;
        let captured_frame_count = self
            .frames
            .len()
            .checked_sub(selected.boundary_depth)
            .ok_or_else(|| internal("Handler boundary exceeds frame stack"))?;
        if u64::try_from(captured_frame_count)
            .map_err(|_| internal("continuation frame count does not fit u64"))?
            > self.limits.continuation_frame_limit
        {
            return Err(self.runtime_error(
                RuntimeFaultKind::ResourceLimit {
                    resource: RuntimeResource::ContinuationFrame,
                },
                location,
            ));
        }

        let clause_closure = clause
            .closure
            .as_closure()
            .cloned()
            .ok_or_else(|| internal("verified Handler clause is not a closure"))?;
        let mut arguments = self.materialize_closure_bound(&clause_closure, location)?;
        arguments
            .try_reserve_exact(usize::from(clause.resume_present) + 1)
            .map_err(|_| self.out_of_memory("handler_clause_arguments", location))?;
        arguments.push(input);
        let continuation_id = if clause.resume_present {
            let id = self.allocate_runtime_id()?;
            let continuation = self
                .heap
                .continuation(id)
                .map_err(|()| self.out_of_memory("handler_continuation", location))?;
            arguments.push(continuation);
            Some(id)
        } else {
            None
        };

        let captured_frames = self.frames.split_off(selected.boundary_depth);
        let captured_handlers = self.handlers.split_off(selected_index);
        let completion = match captured_frames
            .first()
            .map(|frame| frame.return_target)
            .ok_or_else(|| internal("Handler continuation has no body frame"))?
        {
            ReturnTarget::HandlerBody { handler_id, .. } if handler_id == selected.id => {
                ClauseCompletion::Handler {
                    destination: selected.destination,
                }
            }
            ReturnTarget::ResumedBody {
                handler_id,
                resume_return_id,
            } if handler_id == selected.id => ClauseCompletion::Resume { resume_return_id },
            _ => return Err(internal("Handler body return boundary is inconsistent").into()),
        };
        if let Some(continuation_id) = continuation_id {
            self.continuations.insert(
                continuation_id,
                ContinuationState {
                    active: true,
                    uses: 0,
                    operation: "Console.Write.write",
                    operation_destination,
                    boundary_depth: selected.boundary_depth,
                    outer_handler_count: selected_index,
                    frames: captured_frames,
                    handlers: captured_handlers,
                },
            );
        }
        self.push_frame(
            clause_closure.value().function(),
            &arguments,
            ReturnTarget::HandlerClause {
                continuation_id,
                completion,
            },
            location,
        )
    }

    fn execute_resume(
        &mut self,
        continuation_id: u64,
        destination: RegisterIndex,
        argument_registers: &[RegisterIndex],
        location: Location,
    ) -> Result<(), ExecutionError> {
        self.check_cancellation(location)?;
        if argument_registers.len() != 1 {
            return Err(internal("verified continuation call argument count is not one").into());
        }
        let output = self.read_current(argument_registers[0])?;
        let (operation_destination, boundary_depth, outer_handler_count) = {
            let state = self
                .continuations
                .get(&continuation_id)
                .ok_or_else(|| internal("verified continuation state is absent"))?;
            if !state.active {
                return Err(internal("verified continuation is outside its lifetime").into());
            }
            if state.uses >= 1 {
                return Err(self.runtime_error(
                    RuntimeFaultKind::HandlerResumeCardinality {
                        operation: state.operation,
                    },
                    location,
                ));
            }
            (
                state.operation_destination,
                state.boundary_depth,
                state.outer_handler_count,
            )
        };
        let (mut resumed_frames, resumed_handlers) = {
            let state = self
                .continuations
                .get(&continuation_id)
                .ok_or_else(|| internal("verified continuation state is absent"))?;
            (
                clone_frames(&state.frames)
                    .map_err(|()| self.out_of_memory("continuation_frames", location))?,
                clone_handlers(&state.handlers)
                    .map_err(|()| self.out_of_memory("continuation_handlers", location))?,
            )
        };
        let handler_id = resumed_handlers
            .first()
            .map(|handler| handler.id)
            .ok_or_else(|| internal("continuation has no selected Handler"))?;
        let bottom_target = resumed_frames
            .first()
            .ok_or_else(|| internal("continuation has no body frame"))?;
        if !matches!(
            bottom_target.return_target,
            ReturnTarget::HandlerBody { handler_id: id, .. } if id == handler_id
        ) {
            return Err(internal("continuation body boundary is inconsistent").into());
        }
        write_register(
            resumed_frames
                .last_mut()
                .ok_or_else(|| internal("continuation has no performing frame"))?,
            operation_destination,
            output,
        )?;
        let resume_return_id = self.allocate_runtime_id()?;
        resumed_frames
            .first_mut()
            .ok_or_else(|| internal("continuation has no body frame"))?
            .return_target = ReturnTarget::ResumedBody {
            handler_id,
            resume_return_id,
        };
        if self.frames.len() < boundary_depth || self.handlers.len() < outer_handler_count {
            return Err(internal("continuation owner boundary is no longer active").into());
        }
        let suspended_frames = self.frames.split_off(boundary_depth);
        let suspended_handlers = self.handlers.split_off(outer_handler_count);
        self.resume_returns.insert(
            resume_return_id,
            ResumeReturn {
                continuation_id,
                destination,
                frames: suspended_frames,
                handlers: suspended_handlers,
            },
        );
        self.continuations
            .get_mut(&continuation_id)
            .ok_or_else(|| internal("verified continuation state is absent"))?
            .uses += 1;
        self.handlers.extend(resumed_handlers);
        self.frames.extend(resumed_frames);
        Ok(())
    }

    fn allocate_runtime_id(&mut self) -> Result<u64, ExecutionError> {
        let id = self.next_runtime_id;
        self.next_runtime_id = self
            .next_runtime_id
            .checked_add(1)
            .ok_or_else(|| internal("runtime identity space is exhausted"))?;
        Ok(id)
    }

    fn pop_handler(&mut self, handler_id: u64) -> Result<(), ExecutionError> {
        let handler = self
            .handlers
            .pop()
            .ok_or_else(|| internal("Handler return has no active boundary"))?;
        if handler.id != handler_id {
            return Err(internal("Handler return boundary is not innermost").into());
        }
        Ok(())
    }

    fn materialize_closure_bound(
        &self,
        closure: &std::rc::Rc<Allocation<Closure>>,
        location: Location,
    ) -> Result<Vec<Value>, ExecutionError> {
        let captures = closure.value().bound();
        let mut values = Vec::new();
        values
            .try_reserve_exact(captures.len())
            .map_err(|_| self.out_of_memory("call_closure_captures", location))?;
        for capture in captures {
            values.push(match capture {
                BoundValue::Value(value) => value.clone(),
                BoundValue::SelfReference => Value::Closure(std::rc::Rc::clone(closure)),
            });
        }
        Ok(values)
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

    fn variant_cases(
        &self,
        type_index: u32,
    ) -> Result<&[ling_bytecode::VariantCase], ExecutionError> {
        match self.model.types().get(to_usize(type_index)?) {
            Some(ValueType::Variant { cases, .. }) => Ok(cases),
            _ => Err(internal("verified variant type is absent").into()),
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

    fn check_cancellation(&self, location: Location) -> Result<(), ExecutionError> {
        if self
            .cancellation
            .is_some_and(CancellationToken::is_cancelled)
        {
            Err(self.runtime_error(RuntimeFaultKind::Cancelled, location))
        } else {
            Ok(())
        }
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
        return_target: ReturnTarget,
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
        let mut frame = Frame::new(function_index, register_count, return_target)
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

#[derive(Clone)]
struct Frame {
    function: u32,
    block: u32,
    instruction: usize,
    registers: Vec<Option<Value>>,
    return_target: ReturnTarget,
}

impl Frame {
    fn new(function: u32, register_count: usize, return_target: ReturnTarget) -> Result<Self, ()> {
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
            return_target,
        })
    }
}

#[derive(Clone, Copy)]
enum ReturnTarget {
    Entry,
    Register(RegisterIndex),
    HandlerBody {
        handler_id: u64,
        destination: RegisterIndex,
    },
    HandlerClause {
        continuation_id: Option<u64>,
        completion: ClauseCompletion,
    },
    ResumedBody {
        handler_id: u64,
        resume_return_id: u64,
    },
}

#[derive(Clone, Copy)]
enum ClauseCompletion {
    Handler { destination: RegisterIndex },
    Resume { resume_return_id: u64 },
}

#[derive(Clone)]
struct RuntimeHandlerClause {
    operation: HandlerOperation,
    resume_present: bool,
    closure: Value,
}

#[derive(Clone)]
struct ActiveHandler {
    id: u64,
    boundary_depth: usize,
    destination: RegisterIndex,
    clauses: Vec<RuntimeHandlerClause>,
}

struct ContinuationState {
    active: bool,
    uses: u64,
    operation: &'static str,
    operation_destination: RegisterIndex,
    boundary_depth: usize,
    outer_handler_count: usize,
    frames: Vec<Frame>,
    handlers: Vec<ActiveHandler>,
}

struct CellEntry {
    value: Value,
    _charge: HeapCharge,
}

struct ResumeReturn {
    continuation_id: u64,
    destination: RegisterIndex,
    frames: Vec<Frame>,
    handlers: Vec<ActiveHandler>,
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

fn clone_frames(frames: &[Frame]) -> Result<Vec<Frame>, ()> {
    let mut cloned = Vec::new();
    cloned.try_reserve_exact(frames.len()).map_err(|_| ())?;
    cloned.extend(frames.iter().cloned());
    Ok(cloned)
}

fn clone_handlers(handlers: &[ActiveHandler]) -> Result<Vec<ActiveHandler>, ()> {
    let mut cloned = Vec::new();
    cloned.try_reserve_exact(handlers.len()).map_err(|_| ())?;
    cloned.extend(handlers.iter().cloned());
    Ok(cloned)
}

fn internal(invariant: &'static str) -> InternalExecutionError {
    InternalExecutionError::new(invariant)
}

fn to_usize(value: u32) -> Result<usize, InternalExecutionError> {
    usize::try_from(value).map_err(|_| internal("verified u32 index does not fit host usize"))
}

fn to_usize_u64(value: u64) -> Result<usize, InternalExecutionError> {
    usize::try_from(value).map_err(|_| internal("verified u64 CellId does not fit host usize"))
}

fn to_u32(value: usize) -> Result<u32, InternalExecutionError> {
    u32::try_from(value).map_err(|_| internal("verified executable ordinal does not fit u32"))
}
