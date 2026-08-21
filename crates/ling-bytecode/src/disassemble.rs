use std::fmt::Write;

use crate::{
    CaptureOperand, Constant, FunctionKind, Instruction, IntegerSign, LoweredProgramV1,
    LoweredProgramV1_1, LoweredProgramV1_2, PackageReference, SourceOrigin, Terminator,
    UnverifiedProgram, ValueType,
};

/// Renders a deterministic human-readable debug view; this text is not a wire protocol.
#[must_use]
pub fn disassemble_v1(program: &LoweredProgramV1) -> String {
    disassemble_model(program.model(), "1.0", false)
}

/// Renders a deterministic version-1.1 debug view; this text is not a wire protocol.
#[must_use]
pub fn disassemble_v1_1(program: &LoweredProgramV1_1) -> String {
    disassemble_model(program.model(), "1.1", true)
}

/// Renders a deterministic version-1.2 debug view; this text is not a wire protocol.
#[must_use]
pub fn disassemble_v1_2(program: &LoweredProgramV1_2) -> String {
    disassemble_model(program.model(), "1.2", true)
}

fn disassemble_model(
    model: &UnverifiedProgram,
    revision: &str,
    show_closure_metadata: bool,
) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "; ling.bytecode/{revision} debug disassembly (non-contract)"
    )
    .expect("writing to String cannot fail");

    writeln!(output, "strings {}", model.strings().len()).expect("writing to String cannot fail");
    for (index, value) in model.strings().iter().enumerate() {
        writeln!(output, "  @s{index} {}", quoted(value)).expect("writing to String cannot fail");
    }

    writeln!(output, "packages {}", model.packages().len()).expect("writing to String cannot fail");
    for (index, package) in model.packages().iter().enumerate() {
        writeln!(
            output,
            "  @p{index} name=@s{} version=@s{} sha256={}",
            package.name.get(),
            package.version.get(),
            digest(package.content_sha256.as_bytes())
        )
        .expect("writing to String cannot fail");
    }

    writeln!(output, "modules {}", model.modules().len()).expect("writing to String cannot fail");
    for (index, module) in model.modules().iter().enumerate() {
        let package = match module.package {
            PackageReference::Standalone => "standalone".to_owned(),
            PackageReference::Package(package) => format!("@p{}", package.get()),
        };
        writeln!(
            output,
            "  @m{index} {package} @s{} {} capabilities={}",
            module.name.get(),
            quoted(string(model, module.name.get())),
            capability_list(&module.capabilities)
        )
        .expect("writing to String cannot fail");
    }

    writeln!(output, "types {}", model.types().len()).expect("writing to String cannot fail");
    for (index, value) in model.types().iter().enumerate() {
        writeln!(
            output,
            "  @t{index} {} tag=0x{:02x}",
            type_text(value),
            value.tag()
        )
        .expect("writing to String cannot fail");
    }

    writeln!(output, "constants {}", model.constants().len())
        .expect("writing to String cannot fail");
    for (index, value) in model.constants().iter().enumerate() {
        writeln!(output, "  @c{index} {}", constant(model, value))
            .expect("writing to String cannot fail");
    }

    writeln!(output, "sources {}", model.sources().len()).expect("writing to String cannot fail");
    for (index, source) in model.sources().iter().enumerate() {
        writeln!(
            output,
            "  @src{index} @m{} @s{} {} bytes={} sha256={}",
            source.module.get(),
            source.logical_name.get(),
            quoted(string(model, source.logical_name.get())),
            source.original_byte_length,
            digest(source.content_sha256.as_bytes())
        )
        .expect("writing to String cannot fail");
    }

    writeln!(output, "functions {}", model.functions().len())
        .expect("writing to String cannot fail");
    for (function_index, function) in model.functions().iter().enumerate() {
        let parameters = function
            .parameter_types
            .iter()
            .map(|value| format!("@t{}", value.get()))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            output,
            "  fn @f{function_index} @m{} @s{} {} ({parameters}) -> @t{} effects={} registers={}{}",
            function.module.get(),
            function.name.get(),
            quoted(string(model, function.name.get())),
            function.result_type.get(),
            effect_list(&function.effects),
            function.register_count,
            if show_closure_metadata {
                format!(
                    " kind={} captures={}",
                    match function.kind {
                        FunctionKind::Named => "named",
                        FunctionKind::ClosureBody => "closure-body",
                    },
                    function.capture_count
                )
            } else {
                String::new()
            }
        )
        .expect("writing to String cannot fail");
        for (block_index, block) in function.blocks.iter().enumerate() {
            let parameters = block
                .parameters
                .iter()
                .map(|parameter| {
                    format!(
                        "%r{}:@t{}",
                        parameter.register.get(),
                        parameter.value_type.get()
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(output, "    block @b{block_index} ({parameters})")
                .expect("writing to String cannot fail");
            for (ordinal, instruction) in block.instructions.iter().enumerate() {
                writeln!(output, "      {ordinal}: {}", instruction_text(instruction))
                    .expect("writing to String cannot fail");
            }
            writeln!(
                output,
                "      {}: {}",
                block.instructions.len(),
                terminator_text(&block.terminator)
            )
            .expect("writing to String cannot fail");
        }
    }

    writeln!(output, "entry @f{}", model.entry().get()).expect("writing to String cannot fail");
    writeln!(output, "source-map {}", model.source_map().len())
        .expect("writing to String cannot fail");
    for entry in model.source_map() {
        writeln!(
            output,
            "  @f{}:@b{}:{} -> @src{} {}..{} {}",
            entry.function.get(),
            entry.block.get(),
            entry.ordinal,
            entry.source.get(),
            entry.span.start_byte(),
            entry.span.end_byte(),
            match entry.origin {
                SourceOrigin::Direct => "direct",
                SourceOrigin::LoweringDerived => "lowering-derived",
            }
        )
        .expect("writing to String cannot fail");
    }
    output
}

fn string(model: &UnverifiedProgram, index: u32) -> &str {
    model
        .strings()
        .get(index as usize)
        .map_or("<invalid-string-index>", String::as_str)
}

fn type_text(value: &ValueType) -> String {
    match value {
        ValueType::Unit => "Unit".to_owned(),
        ValueType::Bool => "Bool".to_owned(),
        ValueType::Int => "Int".to_owned(),
        ValueType::Text => "Text".to_owned(),
        ValueType::Function {
            parameters,
            result,
            effects,
        } => format!(
            "Function ({}) -> @t{} effects={}",
            parameters
                .iter()
                .map(|parameter| format!("@t{}", parameter.get()))
                .collect::<Vec<_>>()
                .join(", "),
            result.get(),
            effect_list(effects)
        ),
        ValueType::Tuple { elements } => format!(
            "Tuple ({})",
            elements
                .iter()
                .map(|element| format!("@t{}", element.get()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        ValueType::Record {
            module,
            name,
            arguments,
            fields,
        } => format!(
            "Record @m{} @s{} args=({}) fields=({})",
            module.get(),
            name.get(),
            arguments
                .iter()
                .map(|argument| format!("@t{}", argument.get()))
                .collect::<Vec<_>>()
                .join(", "),
            fields
                .iter()
                .map(|field| format!(
                    "@s{}:@t{}{}",
                    field.name.get(),
                    field.value_type.get(),
                    if field.mutable { " mut" } else { "" }
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        ValueType::Variant {
            module,
            name,
            arguments,
            cases,
        } => format!(
            "Variant @m{} @s{} args=({}) cases=({})",
            module.get(),
            name.get(),
            arguments
                .iter()
                .map(|argument| format!("@t{}", argument.get()))
                .collect::<Vec<_>>()
                .join(", "),
            cases
                .iter()
                .map(|case| format!(
                    "@s{}{}",
                    case.name.get(),
                    case.payload
                        .map_or(String::new(), |payload| format!(":@t{}", payload.get()))
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn capability_list(values: &[crate::Capability]) -> String {
    let values = values
        .iter()
        .map(|value| match value {
            crate::Capability::ConsoleWrite => "Console.Write",
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{values}]")
}

fn effect_list(values: &[crate::Effect]) -> String {
    let values = values
        .iter()
        .map(|value| match value {
            crate::Effect::ConsoleWrite => "Console.Write",
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{values}]")
}

fn constant(model: &UnverifiedProgram, value: &Constant) -> String {
    match value {
        Constant::Unit => "Unit ()".to_owned(),
        Constant::Bool(value) => format!("Bool {value}"),
        Constant::Int { sign, magnitude } => {
            let sign = match sign {
                IntegerSign::Zero => "zero",
                IntegerSign::Positive => "positive",
                IntegerSign::Negative => "negative",
            };
            format!("Int {sign} 0x{}", hexadecimal(magnitude))
        }
        Constant::Text(index) => format!(
            "Text @s{} {}",
            index.get(),
            quoted(string(model, index.get()))
        ),
    }
}

fn instruction_text(value: &Instruction) -> String {
    match value {
        Instruction::Const {
            destination,
            constant,
        } => format!("%r{} = const @c{}", destination.get(), constant.get()),
        Instruction::IntUnary {
            destination,
            operator,
            operand,
        } => format!(
            "%r{} = int.{} %r{}",
            destination.get(),
            match operator {
                crate::IntUnaryOperator::Positive => "positive",
                crate::IntUnaryOperator::Negative => "negative",
            },
            operand.get()
        ),
        Instruction::IntBinary {
            destination,
            operator,
            left,
            right,
        } => format!(
            "%r{} = int.{} %r{}, %r{}",
            destination.get(),
            match operator {
                crate::IntBinaryOperator::Add => "add",
                crate::IntBinaryOperator::Subtract => "subtract",
                crate::IntBinaryOperator::Multiply => "multiply",
                crate::IntBinaryOperator::Divide => "divide",
                crate::IntBinaryOperator::Remainder => "remainder",
            },
            left.get(),
            right.get()
        ),
        Instruction::Compare {
            destination,
            operator,
            left,
            right,
        } => format!(
            "%r{} = compare.0x{:02x} %r{}, %r{}",
            destination.get(),
            operator.tag(),
            left.get(),
            right.get()
        ),
        Instruction::Call {
            destination,
            function,
            arguments,
        } => format!(
            "%r{} = call @f{} ({})",
            destination.get(),
            function.get(),
            register_list(arguments)
        ),
        Instruction::MakeClosure {
            destination,
            function,
            captures,
        } => format!(
            "%r{} = make-closure @f{} ({})",
            destination.get(),
            function.get(),
            captures
                .iter()
                .map(|capture| match capture {
                    CaptureOperand::Register(register) => format!("%r{}", register.get()),
                    CaptureOperand::SelfReference => "self".to_owned(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Instruction::CallClosure {
            destination,
            callee,
            arguments,
        } => format!(
            "%r{} = call-closure %r{} ({})",
            destination.get(),
            callee.get(),
            register_list(arguments)
        ),
        Instruction::MakeTuple {
            destination,
            tuple,
            elements,
        } => format!(
            "%r{} = make-tuple @t{} ({})",
            destination.get(),
            tuple.get(),
            register_list(elements)
        ),
        Instruction::GetTuple {
            destination,
            tuple,
            element,
        } => format!(
            "%r{} = get-tuple %r{}[{}]",
            destination.get(),
            tuple.get(),
            element
        ),
        Instruction::MakeRecord {
            destination,
            record,
            fields,
        } => format!(
            "%r{} = make-record @t{} ({})",
            destination.get(),
            record.get(),
            register_list(fields)
        ),
        Instruction::GetField {
            destination,
            record,
            field,
        } => format!(
            "%r{} = get-field %r{}[{}]",
            destination.get(),
            record.get(),
            field
        ),
        Instruction::UpdateRecord {
            destination,
            base,
            updates,
        } => format!(
            "%r{} = update-record %r{} ({})",
            destination.get(),
            base.get(),
            updates
                .iter()
                .map(|update| format!("{}:%r{}", update.field, update.value.get()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Instruction::MakeVariant {
            destination,
            variant,
            case,
            payload,
        } => format!(
            "%r{} = make-variant @t{} {} {}",
            destination.get(),
            variant.get(),
            case,
            payload.map_or_else(|| "()".to_owned(), |value| format!("%r{}", value.get()))
        ),
        Instruction::VariantIs {
            destination,
            variant,
            case,
        } => format!(
            "%r{} = variant-is %r{} {}",
            destination.get(),
            variant.get(),
            case
        ),
        Instruction::GetVariantPayload {
            destination,
            variant,
            case,
        } => format!(
            "%r{} = get-variant-payload %r{} {}",
            destination.get(),
            variant.get(),
            case
        ),
        Instruction::Intrinsic {
            destination,
            intrinsic,
            arguments,
        } => format!(
            "%r{} = intrinsic.0x{:02x} ({})",
            destination.get(),
            intrinsic.tag(),
            register_list(arguments)
        ),
        Instruction::ConsoleWrite { destination, text } => {
            format!("%r{} = console.write %r{}", destination.get(), text.get())
        }
    }
}

fn terminator_text(value: &Terminator) -> String {
    match value {
        Terminator::Jump { target, arguments } => {
            format!("jump @b{} ({})", target.get(), register_list(arguments))
        }
        Terminator::Branch {
            condition,
            true_target,
            true_arguments,
            false_target,
            false_arguments,
        } => format!(
            "branch %r{} @b{} ({}) @b{} ({})",
            condition.get(),
            true_target.get(),
            register_list(true_arguments),
            false_target.get(),
            register_list(false_arguments)
        ),
        Terminator::Return { value } => format!("return %r{}", value.get()),
    }
}

fn register_list(values: &[crate::RegisterIndex]) -> String {
    values
        .iter()
        .map(|value| format!("%r{}", value.get()))
        .collect::<Vec<_>>()
        .join(", ")
}

fn quoted(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '\0' => output.push_str("\\0"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            value if value.is_control() => {
                write!(output, "\\u{{{:x}}}", u32::from(value))
                    .expect("writing to String cannot fail");
            }
            value => output.push(value),
        }
    }
    output.push('"');
    output
}

fn digest(value: &[u8; 32]) -> String {
    hexadecimal(value)
}

fn hexadecimal(value: &[u8]) -> String {
    let mut output = String::with_capacity(value.len() * 2);
    for byte in value {
        write!(output, "{byte:02x}").expect("writing to String cannot fail");
    }
    output
}
