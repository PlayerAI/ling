#[path = "support/effect_property.rs"]
mod effect_property;

use std::collections::BTreeMap;

use ling_bytecode::{
    Effect as BytecodeEffect, FunctionKind, LoweringSource, UnverifiedProgram, ValueType,
    VerifiedProgramV1, decode_and_verify_v1, decode_and_verify_v1_1, decode_and_verify_v1_2,
    decode_and_verify_v1_3, decode_and_verify_v1_4, encode_v1, encode_v1_1, encode_v1_2,
    encode_v1_3, encode_v1_4, lower_v1, lower_v1_1, lower_v1_2, lower_v1_3, lower_v1_4,
};
use ling_effects::locate_main;
use ling_eval::{
    MemoryConsole, RuntimeFault as EvalRuntimeFault, RuntimeFaultKind as EvalFaultKind,
    execute_main,
};
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceFile, SourceId};
use ling_vm::{
    ConsoleCapability, ExecutionError, ExecutionLimits, HostCapabilities, HostError,
    RuntimeFault as VmRuntimeFault, RuntimeFaultKind as VmFaultKind, execute_v1,
};

use effect_property::{
    FIXED_SEEDS, GeneratedCase, MAX_OUTPUT_BYTES as PROPERTY_MAX_OUTPUT_BYTES, MAX_SHRINK_ATTEMPTS,
    ORDINALS_PER_SEED, Scenario, generate, minimize_failure, shrink_candidates,
};

const HELLO: &str = include_str!("../../../tests/bytecode/v1/programs/hello.ling");
const DIRECT_CALL: &str = include_str!("../../../tests/bytecode/v1/programs/direct-call.ling");
const FORMAT_FAULT: &str = include_str!("../../../tests/bytecode/v1/programs/format-fault.ling");
const SOURCE_CLOSURE: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "let factory prefix =\n",
    "    let local value = Console.write prefix\n",
    "    local\n\n",
    "let main () =\n",
    "    let callback = factory \"hello\"\n",
    "    callback \"world\"\n",
);
const SOURCE_RECURSION: &str = concat!(
    "module Main\n\n",
    "let rec loop () : Unit =\n",
    "    loop ()\n\n",
    "let main () =\n",
    "    ()\n",
);
const SOURCE_SKIPPED_EFFECT: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "let main () =\n",
    "    if false then\n",
    "        Console.write \"skipped\"\n",
    "    else\n",
    "        ()\n",
);
const SOURCE_AGGREGATE: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "type Point = { x: Int; y: Int }\n\n",
    "let main () =\n",
    "    let point = { x = 1; y = 2 }\n",
    "    let changed = { point with x = point.x + 1 }\n",
    "    Console.write (Text.format \"{}\" (changed.x + changed.y))\n",
);
const SOURCE_MATCH: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "type State =\n",
    "    | Idle\n",
    "    | Ready of Int\n\n",
    "let classify state =\n",
    "    match state with\n",
    "    | Ready value -> value\n",
    "    | Idle -> 0\n\n",
    "let main () =\n",
    "    Console.write (Text.format \"{}\" (classify (Ready 7)))\n",
);
const SOURCE_MUTATION: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "type Counter = { mutable value: Int }\n\n",
    "let main () =\n",
    "    let mutable counter = { value = 0 }\n",
    "    counter.value <- 2\n",
    "    Console.write (Text.format \"{}\" counter.value)\n",
);
const SOURCE_HANDLER_RESUME: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "let main () =\n",
    "    handle Console.write \"body\" with\n",
    "        operation Console.Write.write(message, resume) ->\n",
    "            resume ()\n",
    "            Console.write \"clause\"\n",
);
const SOURCE_HANDLER_DEEP: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "let emitBoth () =\n",
    "    Console.write \"first\"\n",
    "    Console.write \"second\"\n\n",
    "let main () =\n",
    "    handle emitBoth () with\n",
    "        operation Console.Write.write(message, resume) ->\n",
    "            if message == \"first\" then\n",
    "                resume ()\n",
    "                Console.write \"after resume\"\n",
    "            else\n",
    "                ()\n",
);
const SOURCE_HANDLER_CARDINALITY: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "let invokeTwice callback =\n",
    "    let ignored = callback ()\n",
    "    callback ()\n\n",
    "let main () =\n",
    "    handle Console.write \"body\" with\n",
    "        operation Console.Write.write(message, resume) -> invokeTwice resume\n",
);
const SOURCE_HANDLER_FAULT: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "let main () =\n",
    "    handle Console.write \"trigger\" with\n",
    "        operation Console.Write.write(message, resume) ->\n",
    "            Console.write \"committed\"\n",
    "            let ignored = 1 / 0\n",
    "            ()\n",
);
const SOURCE_HANDLER_CELL: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "let main () =\n",
    "    let mutable cell = 0\n",
    "    let ignored =\n",
    "        handle Console.write \"body\" with\n",
    "            operation Console.Write.write(message, resume) ->\n",
    "                resume ()\n",
    "                cell <- 3\n",
    "    Console.write (Text.format \"{}\" cell)\n",
);
const SOURCE_HANDLER_CELL_ALIAS: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "let main () =\n",
    "    let mutable cell = 0\n",
    "    let set value = cell <- value\n",
    "    let ignored =\n",
    "        handle Console.write \"body\" with\n",
    "            operation Console.Write.write(message, resume) ->\n",
    "                set 7\n",
    "                resume ()\n",
    "    Console.write (Text.format \"{}\" cell)\n",
);
const SOURCE_HANDLER_CELL_DEEP: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "let emitBoth () =\n",
    "    Console.write \"first\"\n",
    "    Console.write \"second\"\n\n",
    "let main () =\n",
    "    let mutable cell = 0\n",
    "    let ignored =\n",
    "        handle emitBoth () with\n",
    "            operation Console.Write.write(message, resume) ->\n",
    "                if message == \"first\" then\n",
    "                    cell <- cell + 1\n",
    "                    resume ()\n",
    "                else\n",
    "                    cell <- cell + 10\n",
    "    Console.write (Text.format \"{}\" cell)\n",
);

const MAX_FIXTURES: usize = 16;
const MAX_SOURCE_NAME_BYTES: usize = 256;
const MAX_SOURCE_BYTES: usize = 64 * 1024;
const MAX_EVENT_BYTES: usize = 16 * 1024;

#[derive(Clone, Copy)]
enum Revision {
    V1,
    V1_1,
    V1_2,
    V1_3,
    V1_4,
}

struct Case {
    logical_name: &'static str,
    text: &'static str,
    revision: Revision,
}

struct Fixture {
    source: SourceFile,
    snapshot: ProgramSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FaultProjection {
    category: String,
    operation: String,
    source_name: String,
    start_byte: u64,
    end_byte: u64,
    committed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Outcome {
    events: Vec<String>,
    returned_unit: bool,
    fault: Option<FaultProjection>,
}

#[derive(Default)]
struct EventConsole {
    output: String,
}

impl ConsoleCapability for EventConsole {
    fn write_line(&mut self, text: &str) -> Result<(), HostError> {
        self.output.push_str(text);
        self.output.push('\n');
        Ok(())
    }
}

fn fixture(logical_name: &str, text: &str) -> Fixture {
    checked_fixture(logical_name, text).expect("fixture passes the complete checked pipeline")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CheckStage {
    Source,
    Syntax,
    Ast,
    Hir,
    Resolution,
    Types,
    Effects,
    Snapshot,
}

fn checked_fixture(_logical_name: &str, text: &str) -> Result<Fixture, CheckStage> {
    let source =
        SourceFile::from_bytes(SourceId::new(0), "src/Main.ling", text.as_bytes().to_vec())
            .map_err(|_| CheckStage::Source)?;
    let parsed = ling_syntax::parse(&source);
    if !parsed.is_valid() {
        return Err(CheckStage::Syntax);
    }
    let ast = ling_ast::lower(&source, &parsed).map_err(|_| CheckStage::Ast)?;
    let hir = ling_hir::lower(source.name(), &ast).map_err(|_| CheckStage::Hir)?;
    let resolved = ling_resolve::resolve(vec![hir], "Main").map_err(|_| CheckStage::Resolution)?;
    let typed = ling_types::check(resolved).map_err(|_| CheckStage::Types)?;
    let checked = ling_effects::check(typed).map_err(|_| CheckStage::Effects)?;
    let snapshot = ling_semantic::build(checked).map_err(|_| CheckStage::Snapshot)?;
    Ok(Fixture { source, snapshot })
}

fn verified(fixture: &Fixture, revision: Revision) -> VerifiedProgramV1 {
    let source = [LoweringSource::new(&fixture.source, "src/Main.ling")];
    match revision {
        Revision::V1 => {
            let lowered = lower_v1(&fixture.snapshot, &source).expect("fixture lowers to v1");
            let bytes = encode_v1(&lowered).expect("fixture encodes as v1");
            decode_and_verify_v1(&bytes).expect("v1 artifact verifies")
        }
        Revision::V1_1 => {
            let lowered = lower_v1_1(&fixture.snapshot, &source).expect("fixture lowers to v1.1");
            let bytes = encode_v1_1(&lowered).expect("fixture encodes as v1.1");
            decode_and_verify_v1_1(&bytes).expect("v1.1 artifact verifies")
        }
        Revision::V1_2 => {
            let lowered = lower_v1_2(&fixture.snapshot, &source).expect("fixture lowers to v1.2");
            let bytes = encode_v1_2(&lowered).expect("fixture encodes as v1.2");
            decode_and_verify_v1_2(&bytes).expect("v1.2 artifact verifies")
        }
        Revision::V1_3 => {
            let lowered = lower_v1_3(&fixture.snapshot, &source).expect("fixture lowers to v1.3");
            let bytes = encode_v1_3(&lowered).expect("fixture encodes as v1.3");
            decode_and_verify_v1_3(&bytes).expect("v1.3 artifact verifies")
        }
        Revision::V1_4 => {
            let lowered = lower_v1_4(&fixture.snapshot, &source).expect("fixture lowers to v1.4");
            let bytes = encode_v1_4(&lowered).expect("fixture encodes as v1.4");
            decode_and_verify_v1_4(&bytes).expect("v1.4 artifact verifies")
        }
    }
}

fn encoded_v1_4(fixture: &Fixture, logical_name: &str) -> (Vec<u8>, VerifiedProgramV1) {
    let source = [LoweringSource::new(&fixture.source, "src/Main.ling")];
    let lowered = lower_v1_4(&fixture.snapshot, &source)
        .unwrap_or_else(|error| panic!("{logical_name}: property case lowers to v1.4: {error}"));
    let bytes = encode_v1_4(&lowered).expect("property case encodes as v1.4");
    let verified = decode_and_verify_v1_4(&bytes).expect("property artifact verifies");
    (bytes, verified)
}

fn checked_named_rows(fixture: &Fixture) -> BTreeMap<String, Vec<String>> {
    let checked = fixture.snapshot.checked();
    checked
        .typed()
        .resolved()
        .definitions()
        .iter()
        .filter_map(|(id, definition)| {
            matches!(
                definition.origin,
                ling_resolve::DefinitionOrigin::User { .. }
            )
            .then(|| {
                checked
                    .definition_effect(id)
                    .map(|row| (definition.name.clone(), row.canonical_names()))
            })
            .flatten()
        })
        .collect()
}

fn bytecode_named_rows(program: &VerifiedProgramV1) -> BTreeMap<String, Vec<String>> {
    let model = program.model();
    model
        .functions()
        .iter()
        .filter(|function| function.kind == FunctionKind::Named)
        .map(|function| {
            let name = model.strings()[function.name.get() as usize].clone();
            let mut row = function
                .effects
                .iter()
                .map(|effect| bytecode_effect_name(model, effect))
                .collect::<Vec<_>>();
            row.sort();
            (name, row)
        })
        .collect()
}

fn bytecode_effect_name(model: &UnverifiedProgram, effect: &BytecodeEffect) -> String {
    match effect {
        BytecodeEffect::ConsoleWrite => "Console.Write".to_owned(),
        BytecodeEffect::State(payload) => {
            format!(
                "State<{}>",
                bytecode_type_name(model, payload.get() as usize)
            )
        }
    }
}

fn bytecode_type_name(model: &UnverifiedProgram, index: usize) -> String {
    match &model.types()[index] {
        ValueType::Unit => "Unit".to_owned(),
        ValueType::Bool => "Bool".to_owned(),
        ValueType::Int => "Int".to_owned(),
        ValueType::Text => "Text".to_owned(),
        ValueType::Cell(payload) => {
            format!(
                "Cell<{}>",
                bytecode_type_name(model, payload.get() as usize)
            )
        }
        ValueType::Record { name, .. } | ValueType::Variant { name, .. } => {
            model.strings()[name.get() as usize].clone()
        }
        ValueType::Tuple { elements } => format!(
            "({})",
            elements
                .iter()
                .map(|element| bytecode_type_name(model, element.get() as usize))
                .collect::<Vec<_>>()
                .join(",")
        ),
        ValueType::Function {
            parameters,
            result,
            effects,
        } => {
            let parameters = parameters
                .iter()
                .map(|parameter| bytecode_type_name(model, parameter.get() as usize))
                .collect::<Vec<_>>()
                .join(",");
            let mut effects = effects
                .iter()
                .map(|effect| bytecode_effect_name(model, effect))
                .collect::<Vec<_>>();
            effects.sort();
            format!(
                "({parameters})->{}!{{{}}}",
                bytecode_type_name(model, result.get() as usize),
                effects.join(",")
            )
        }
    }
}

fn logical_events(output: &str) -> Vec<String> {
    assert!(
        output.len() <= MAX_EVENT_BYTES,
        "logical host-event stream exceeds the differential bound"
    );
    output
        .lines()
        .map(|event| {
            assert!(
                event.len() <= MAX_EVENT_BYTES,
                "logical host event exceeds the differential bound"
            );
            event.to_owned()
        })
        .collect()
}

fn eval_fault(error: &EvalRuntimeFault, committed: bool) -> FaultProjection {
    assert!(
        error.source_name.len() <= MAX_SOURCE_NAME_BYTES,
        "interpreter Fault source name exceeds the differential bound"
    );
    let (category, operation) = match &error.kind {
        EvalFaultKind::HostCapability {
            operation,
            category,
        } => (category.name(), *operation),
        EvalFaultKind::InvalidFormatPlaceholderCount { .. } => ("invalid_format", "Text.format"),
        EvalFaultKind::DivisionByZero => ("division_by_zero", "Int.divide"),
        EvalFaultKind::InvalidCheckedCore { .. } => ("checked_core_invariant", "checked_core"),
        EvalFaultKind::HandlerResumeCardinality { operation, .. } => {
            ("handler_resume_cardinality", operation.as_str())
        }
    };
    FaultProjection {
        category: category.to_owned(),
        operation: operation.to_owned(),
        source_name: error.source_name.clone(),
        start_byte: u64::from(error.span.start().get()),
        end_byte: u64::from(error.span.end().get()),
        committed,
    }
}

fn vm_fault(error: &VmRuntimeFault) -> FaultProjection {
    assert!(
        error.source_name().len() <= MAX_SOURCE_NAME_BYTES,
        "VM Fault source name exceeds the differential bound"
    );
    let (category, operation) = match error.kind() {
        VmFaultKind::DivisionByZero { operation } => ("division_by_zero", *operation),
        VmFaultKind::InvalidFormatPlaceholderCount { .. } => ("invalid_format", "Text.format"),
        VmFaultKind::HostCapability {
            operation,
            category,
        } => (category.name(), *operation),
        VmFaultKind::CapabilityUnavailable { capability } => {
            ("capability_unavailable", *capability)
        }
        VmFaultKind::ResourceLimit { resource } => (
            "resource_limit",
            match resource {
                ling_vm::RuntimeResource::Step => "step_limit",
                ling_vm::RuntimeResource::Frame => "frame_limit",
                ling_vm::RuntimeResource::HandlerDepth => "handler_depth_limit",
                ling_vm::RuntimeResource::ContinuationFrame => "continuation_frame_limit",
            },
        ),
        VmFaultKind::OutOfMemory { operation } => ("out_of_memory", *operation),
        VmFaultKind::Cancelled => ("cancelled", "execution.cancelled"),
        VmFaultKind::HandlerResumeCardinality { operation } => {
            ("handler_resume_cardinality", *operation)
        }
    };
    FaultProjection {
        category: category.to_owned(),
        operation: operation.to_owned(),
        source_name: error.source_name().to_owned(),
        start_byte: error.span().start_byte(),
        end_byte: error.span().end_byte(),
        committed: error.committed(),
    }
}

fn run_interpreter(fixture: &Fixture) -> Outcome {
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has Main.main");
    let mut console = MemoryConsole::default();
    let result = execute_main(&fixture.snapshot, &main, &mut console);
    let events = logical_events(console.output());
    let fault = result
        .as_ref()
        .err()
        .map(|error| eval_fault(error, !events.is_empty()));
    Outcome {
        events,
        returned_unit: result.is_ok(),
        fault,
    }
}

fn run_vm(program: &VerifiedProgramV1) -> Outcome {
    let mut console = EventConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let result = execute_v1(
        program,
        ExecutionLimits::new(100_000, 1_024, 16 * 1024 * 1024),
        &mut host,
    );
    let events = logical_events(&console.output);
    let fault = match result {
        Ok(()) => None,
        Err(ExecutionError::Runtime(error)) => Some(vm_fault(&error)),
        Err(ExecutionError::Internal(error)) => {
            panic!("differential harness hit a VM invariant: {error}")
        }
    };
    Outcome {
        events,
        returned_unit: fault.is_none(),
        fault,
    }
}

fn assert_differential(logical_name: &str, text: &str, revision: Revision) {
    assert!(
        logical_name.len() <= MAX_SOURCE_NAME_BYTES,
        "fixture logical name exceeds the differential bound"
    );
    assert!(
        text.len() <= MAX_SOURCE_BYTES,
        "fixture source exceeds the differential bound"
    );
    let compiled = fixture(logical_name, text);
    let rebuilt = fixture(logical_name, text);
    assert_eq!(
        compiled.snapshot.program_id(),
        rebuilt.snapshot.program_id(),
        "{logical_name}: checked snapshot identity is not deterministic"
    );
    let reference = run_interpreter(&compiled);
    let program = verified(&compiled, revision);
    let vm = run_vm(&program);
    assert_eq!(
        reference.events, vm.events,
        "{logical_name}: logical events"
    );
    assert_eq!(
        reference.returned_unit, vm.returned_unit,
        "{logical_name}: Unit entry result"
    );
    assert_eq!(
        reference.fault, vm.fault,
        "{logical_name}: Runtime Fault projection"
    );
}

#[test]
fn supported_bytecode_slices_match_the_checked_interpreter() {
    let cases = [
        Case {
            logical_name: "tests/bytecode/v1/programs/hello.ling",
            text: HELLO,
            revision: Revision::V1,
        },
        Case {
            logical_name: "tests/bytecode/v1/programs/direct-call.ling",
            text: DIRECT_CALL,
            revision: Revision::V1,
        },
        Case {
            logical_name: "tests/bytecode/v1/programs/format-fault.ling",
            text: FORMAT_FAULT,
            revision: Revision::V1_1,
        },
        Case {
            logical_name: "tests/bytecode/v1/source-closure.ling",
            text: SOURCE_CLOSURE,
            revision: Revision::V1_1,
        },
        Case {
            logical_name: "tests/bytecode/v1/source-recursion.ling",
            text: SOURCE_RECURSION,
            revision: Revision::V1_1,
        },
        Case {
            logical_name: "tests/bytecode/v1/source-skipped-effect.ling",
            text: SOURCE_SKIPPED_EFFECT,
            revision: Revision::V1_2,
        },
        Case {
            logical_name: "tests/bytecode/v1/source-aggregate.ling",
            text: SOURCE_AGGREGATE,
            revision: Revision::V1_2,
        },
        Case {
            logical_name: "tests/bytecode/v1/source-match.ling",
            text: SOURCE_MATCH,
            revision: Revision::V1_2,
        },
        Case {
            logical_name: "tests/bytecode/v1/source-mutation.ling",
            text: SOURCE_MUTATION,
            revision: Revision::V1_2,
        },
        Case {
            logical_name: "tests/bytecode/v1/handler-resume.ling",
            text: SOURCE_HANDLER_RESUME,
            revision: Revision::V1_3,
        },
        Case {
            logical_name: "tests/bytecode/v1/handler-deep.ling",
            text: SOURCE_HANDLER_DEEP,
            revision: Revision::V1_3,
        },
        Case {
            logical_name: "tests/bytecode/v1/handler-cardinality.ling",
            text: SOURCE_HANDLER_CARDINALITY,
            revision: Revision::V1_3,
        },
        Case {
            logical_name: "tests/bytecode/v1/handler-fault.ling",
            text: SOURCE_HANDLER_FAULT,
            revision: Revision::V1_3,
        },
        Case {
            logical_name: "tests/bytecode/v1/handler-cell.ling",
            text: SOURCE_HANDLER_CELL,
            revision: Revision::V1_4,
        },
        Case {
            logical_name: "tests/bytecode/v1/handler-cell-alias.ling",
            text: SOURCE_HANDLER_CELL_ALIAS,
            revision: Revision::V1_4,
        },
        Case {
            logical_name: "tests/bytecode/v1/handler-cell-deep.ling",
            text: SOURCE_HANDLER_CELL_DEEP,
            revision: Revision::V1_4,
        },
    ];
    assert!(cases.len() <= MAX_FIXTURES);
    for case in cases {
        assert_differential(case.logical_name, case.text, case.revision);
    }
}

fn assert_property_case(case: &GeneratedCase) {
    case.validate_bounds()
        .expect("generated case stays bounded");
    let rebuilt_case = generate(case.seed, case.ordinal);
    assert_eq!(*case, rebuilt_case, "generated source must be reproducible");

    let first = checked_fixture(&case.logical_name, &case.source).unwrap_or_else(|stage| {
        panic!(
            "{}: seed={:#018x}, ordinal={}, rejected at {stage:?}",
            case.logical_name, case.seed, case.ordinal
        )
    });
    let rebuilt = checked_fixture(&case.logical_name, &case.source)
        .expect("the repeated generated case remains checked");
    assert_eq!(
        first.snapshot.program_id(),
        rebuilt.snapshot.program_id(),
        "{}: checked Program ID for seed={:#018x}, ordinal={}",
        case.logical_name,
        case.seed,
        case.ordinal
    );

    let (first_bytes, first_program) = encoded_v1_4(&first, &case.logical_name);
    let (rebuilt_bytes, rebuilt_program) = encoded_v1_4(&rebuilt, &case.logical_name);
    assert_eq!(
        first_bytes, rebuilt_bytes,
        "{}: bytecode for seed={:#018x}, ordinal={}",
        case.logical_name, case.seed, case.ordinal
    );
    assert_eq!(first_program, rebuilt_program);

    let checked_rows = checked_named_rows(&first);
    let bytecode_rows = bytecode_named_rows(&first_program);
    for (name, expected) in checked_rows {
        assert_eq!(
            bytecode_rows.get(&name),
            Some(&expected),
            "{}: residual row for named definition {name}",
            case.logical_name
        );
    }

    let interpreter = run_interpreter(&first);
    let vm = run_vm(&first_program);
    let output_bytes = interpreter.events.iter().map(String::len).sum::<usize>();
    assert!(
        output_bytes <= PROPERTY_MAX_OUTPUT_BYTES,
        "{}: generated output exceeds DEC-0263",
        case.logical_name
    );
    assert_eq!(
        interpreter, vm,
        "{}: seed={:#018x}, ordinal={} differential observation",
        case.logical_name, case.seed, case.ordinal
    );
}

#[test]
fn fixed_effect_property_seeds_replay_through_checked_source_and_bytecode() {
    let mut covered = std::collections::BTreeSet::new();
    for seed in FIXED_SEEDS {
        for ordinal in 0..ORDINALS_PER_SEED {
            let case = generate(seed, ordinal);
            covered.insert(case.scenario);
            assert_property_case(&case);
        }
    }
    assert_eq!(covered.len(), ORDINALS_PER_SEED as usize);
}

fn injected_difference(left: &Outcome, right: &Outcome) -> Option<&'static str> {
    if left.events != right.events {
        Some("ordered_host_events")
    } else if left.returned_unit != right.returned_unit {
        Some("result_value")
    } else if left.fault != right.fault {
        Some("runtime_fault")
    } else {
        None
    }
}

#[test]
fn deterministic_shrinking_rechecks_candidates_and_preserves_failure_projection() {
    let case = generate(FIXED_SEEDS[0], Scenario::UnicodeSource as u32);
    let before = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("crate manifest is readable");
    let candidates = shrink_candidates(&case);
    assert!(candidates.len() <= MAX_SHRINK_ATTEMPTS);
    assert_eq!(candidates, shrink_candidates(&case));
    assert!(
        checked_fixture("generated/unchecked.ling", "module Main\nlet main () =").is_err(),
        "unchecked candidates must be rejected before evaluation"
    );

    let reference = Outcome {
        events: vec!["reference".to_owned()],
        returned_unit: true,
        fault: None,
    };
    let divergent = Outcome {
        events: vec!["divergent".to_owned()],
        returned_unit: true,
        fault: None,
    };
    let projection = injected_difference(&reference, &divergent);
    assert_eq!(projection, Some("ordered_host_events"));
    let preserve = |source: &str| {
        checked_fixture(&case.logical_name, source).is_ok()
            && injected_difference(&reference, &divergent) == projection
    };
    let first = minimize_failure(&case, preserve);
    let second = minimize_failure(&case, preserve);
    assert_eq!(first, second, "shrinking must replay deterministically");
    assert!(first.1 <= MAX_SHRINK_ATTEMPTS);
    assert!(first.0.len() <= case.source.len());

    let after = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("crate manifest remains readable");
    assert_eq!(
        before, after,
        "generation and shrinking must not write the worktree"
    );
}

#[test]
fn generated_shapes_enforce_every_dec_0263_bound() {
    for ordinal in 0..ORDINALS_PER_SEED {
        generate(FIXED_SEEDS[1], ordinal)
            .validate_bounds()
            .expect("repository generator templates stay within every fixed bound");
    }
    let mut oversized = generate(FIXED_SEEDS[1], 0);
    oversized.shape.definitions = effect_property::MAX_DEFINITIONS + 1;
    assert_eq!(oversized.validate_bounds(), Err("definition bound"));
    oversized = generate(FIXED_SEEDS[1], 0);
    oversized.source = "x".repeat(effect_property::MAX_SOURCE_BYTES + 1);
    assert_eq!(oversized.validate_bounds(), Err("source-byte bound"));
}
