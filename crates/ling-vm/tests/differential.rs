use ling_bytecode::{
    LoweringSource, VerifiedProgramV1, decode_and_verify_v1, decode_and_verify_v1_1,
    decode_and_verify_v1_2, decode_and_verify_v1_3, decode_and_verify_v1_4, encode_v1, encode_v1_1,
    encode_v1_2, encode_v1_3, encode_v1_4, lower_v1, lower_v1_1, lower_v1_2, lower_v1_3,
    lower_v1_4,
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

#[derive(Debug, Eq, PartialEq)]
struct FaultProjection {
    category: String,
    operation: String,
    source_name: String,
    start_byte: u64,
    end_byte: u64,
    committed: bool,
}

#[derive(Debug, Eq, PartialEq)]
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
    let source =
        SourceFile::from_bytes(SourceId::new(0), "src/Main.ling", text.as_bytes().to_vec())
            .expect("fixture is valid UTF-8 source");
    let parsed = ling_syntax::parse(&source);
    assert!(
        parsed.is_valid(),
        "{logical_name}: {:?}",
        parsed.parse_errors()
    );
    let ast = ling_ast::lower(&source, &parsed).expect("fixture has valid AST");
    let hir = ling_hir::lower(source.name(), &ast).expect("fixture has valid HIR");
    let resolved = ling_resolve::resolve(vec![hir], "Main").expect("fixture resolves");
    let typed = ling_types::check(resolved).expect("fixture type-checks");
    let checked = ling_effects::check(typed).expect("fixture passes Effect/Capability checks");
    let snapshot = ling_semantic::build(checked).expect("fixture produces a checked snapshot");
    Fixture { source, snapshot }
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
