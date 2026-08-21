mod support;

use ling_bytecode::{
    Instruction, LoweringSource, SourceMapEntry, VerifiedProgramV1, decode_and_verify_v1,
    decode_and_verify_v1_1, encode_v1, encode_v1_1, lower_v1, lower_v1_1,
};
use ling_diagnostics::codes;
use ling_effects::locate_main;
use ling_eval::{MemoryConsole, execute_main};
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceFile, SourceId};
use ling_vm::{
    ConsoleCapability, ExecutionError, ExecutionLimits, HostCapabilities, HostError,
    HostErrorCategory, RuntimeFault, RuntimeFaultKind, RuntimeResource, execute_v1,
};

use support::wire;

const DIRECT_CALL: &str = include_str!("../../../tests/bytecode/v1/programs/direct-call.ling");
const HELLO: &str = include_str!("../../../tests/bytecode/v1/programs/hello.ling");

struct Fixture {
    source: SourceFile,
    snapshot: ProgramSnapshot,
}

fn fixture(name: &str, text: &str) -> Fixture {
    let source = SourceFile::from_bytes(SourceId::new(0), name, text.as_bytes().to_vec())
        .expect("fixture is valid UTF-8 source");
    let parsed = ling_syntax::parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = ling_ast::lower(&source, &parsed).expect("fixture has valid AST");
    let hir = ling_hir::lower(source.name(), &ast).expect("fixture has valid HIR");
    let resolved = ling_resolve::resolve(vec![hir], "Main").expect("fixture resolves");
    let typed = ling_types::check(resolved).expect("fixture type-checks");
    let checked = ling_effects::check(typed).expect("fixture passes Effect/Capability checks");
    let snapshot = ling_semantic::build(checked).expect("fixture produces a checked snapshot");
    Fixture { source, snapshot }
}

fn verified(fixture: &Fixture) -> VerifiedProgramV1 {
    let lowered = lower_v1(
        &fixture.snapshot,
        &[LoweringSource::new(&fixture.source, "src/Main.ling")],
    )
    .expect("fixture lowers to bytecode 1.0");
    let bytes = encode_v1(&lowered).expect("fixture encodes");
    decode_and_verify_v1(&bytes).expect("fixture independently verifies")
}

fn verified_v1_1(fixture: &Fixture) -> VerifiedProgramV1 {
    let lowered = lower_v1_1(
        &fixture.snapshot,
        &[LoweringSource::new(&fixture.source, "src/Main.ling")],
    )
    .expect("fixture lowers to bytecode 1.1");
    let bytes = encode_v1_1(&lowered).expect("fixture encodes");
    decode_and_verify_v1_1(&bytes).expect("fixture independently verifies")
}

fn generous_limits() -> ExecutionLimits {
    ExecutionLimits::new(100_000, 1_024, 16 * 1024 * 1024)
}

#[derive(Default)]
struct RecordingConsole {
    output: String,
    failure: Option<HostError>,
}

impl RecordingConsole {
    fn failing(error: HostError) -> Self {
        Self {
            output: String::new(),
            failure: Some(error),
        }
    }
}

impl ConsoleCapability for RecordingConsole {
    fn write_line(&mut self, text: &str) -> Result<(), HostError> {
        if let Some(error) = self.failure {
            if error.committed() {
                self.output.push_str(text);
                self.output.push('\n');
            }
            Err(error)
        } else {
            self.output.push_str(text);
            self.output.push('\n');
            Ok(())
        }
    }
}

fn runtime(error: ExecutionError) -> RuntimeFault {
    match error {
        ExecutionError::Runtime(fault) => fault,
        ExecutionError::Internal(error) => panic!("unexpected VM invariant: {error}"),
    }
}

fn source_map_entry(
    program: &VerifiedProgramV1,
    function: u32,
    block: u32,
    ordinal: u32,
) -> SourceMapEntry {
    *program
        .model()
        .source_map()
        .iter()
        .find(|entry| {
            (entry.function.get(), entry.block.get(), entry.ordinal) == (function, block, ordinal)
        })
        .expect("verified executable location has a source map")
}

#[test]
fn vm_matches_the_checked_interpreter_for_hello_and_direct_calls() {
    for (name, text) in [("hello.ling", HELLO), ("direct-call.ling", DIRECT_CALL)] {
        let fixture = fixture(name, text);
        let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
        let mut interpreter_console = MemoryConsole::default();
        execute_main(&fixture.snapshot, &main, &mut interpreter_console)
            .expect("interpreter executes");

        let program = verified(&fixture);
        let mut vm_console = RecordingConsole::default();
        let mut host = HostCapabilities::with_console(&mut vm_console);
        execute_v1(&program, generous_limits(), &mut host).expect("VM executes verified input");
        assert_eq!(vm_console.output, interpreter_console.output(), "{name}");
    }
}

#[test]
fn vm_executes_every_scalar_operator_and_both_branch_directions() {
    for take_false_branch in [false, true] {
        let program = decode_and_verify_v1(&wire::scalar_artifact(false, take_false_branch, false))
            .expect("independent scalar artifact verifies");
        let mut console = RecordingConsole::default();
        let mut host = HostCapabilities::with_console(&mut console);
        execute_v1(&program, generous_limits(), &mut host).expect("scalar artifact executes");
        assert_eq!(console.output, "value=10\n");
    }
}

#[test]
fn vm_executes_captured_and_partially_applied_closures() {
    let program = decode_and_verify_v1_1(&wire::closure_artifact())
        .expect("independent closure artifact verifies");
    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    execute_v1(&program, generous_limits(), &mut host).expect("closure artifact executes");
    assert_eq!(console.output, "hello\n");
}

#[test]
fn vm_matches_the_checked_interpreter_for_source_level_closure_lowering() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let select prefix value = prefix\n\n",
        "let main () =\n",
        "    let prefix = \"hello\"\n",
        "    let local value = Console.write prefix\n",
        "    let partial = select prefix\n",
        "    local (partial \"world\")\n",
    );
    let fixture = fixture("source-closures.ling", text);
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
    let mut interpreter_console = MemoryConsole::default();
    execute_main(&fixture.snapshot, &main, &mut interpreter_console)
        .expect("interpreter executes closure source");

    let program = verified_v1_1(&fixture);
    let mut vm_console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut vm_console);
    execute_v1(&program, generous_limits(), &mut host).expect("VM executes closure source");
    assert_eq!(vm_console.output, interpreter_console.output());
    assert_eq!(vm_console.output, "hello\n");
}

#[test]
fn vm_bounds_source_level_top_and_local_recursion_with_explicit_frames() {
    let text = concat!(
        "module Main\n\n",
        "let rec top () : Unit = top ()\n\n",
        "let main () =\n",
        "    let rec local () : Unit = local ()\n",
        "    local ()\n",
    );
    let fixture = fixture("source-recursion.ling", text);
    let program = verified_v1_1(&fixture);
    let mut host = HostCapabilities::none();
    let fault = runtime(
        execute_v1(&program, ExecutionLimits::new(10_000, 8, 1_024), &mut host)
            .expect_err("source recursion must reach the configured frame limit"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::ResourceLimit {
            resource: RuntimeResource::Frame
        }
    ));
    assert!(!fault.committed());
}

#[test]
fn vm_matches_the_checked_interpreter_for_a_returned_closure() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let factory prefix =\n",
        "    let local value = Console.write prefix\n",
        "    local\n\n",
        "let main () =\n",
        "    let callback = factory \"hello\"\n",
        "    callback \"world\"\n",
    );
    let fixture = fixture("returned-closure.ling", text);
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
    let mut interpreter_console = MemoryConsole::default();
    execute_main(&fixture.snapshot, &main, &mut interpreter_console)
        .expect("interpreter executes returned closure");

    let program = verified_v1_1(&fixture);
    let mut vm_console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut vm_console);
    execute_v1(&program, generous_limits(), &mut host).expect("VM executes returned closure");
    assert_eq!(vm_console.output, interpreter_console.output());
    assert_eq!(vm_console.output, "hello\n");
}

#[test]
fn vm_matches_the_checked_interpreter_for_a_higher_order_parameter() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let apply callback value: Unit =\n",
        "    let ignored: Unit = callback value\n",
        "    Console.write value\n\n",
        "let main () =\n",
        "    let local value = Console.write value\n",
        "    apply local \"hello\"\n",
    );
    let fixture = fixture("higher-order.ling", text);
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
    let mut interpreter_console = MemoryConsole::default();
    execute_main(&fixture.snapshot, &main, &mut interpreter_console)
        .expect("interpreter executes higher-order parameter");

    let program = verified_v1_1(&fixture);
    let mut vm_console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut vm_console);
    execute_v1(&program, generous_limits(), &mut host).expect("VM executes higher-order parameter");
    assert_eq!(vm_console.output, interpreter_console.output());
    assert_eq!(vm_console.output, "hello\nhello\n");
}

#[test]
fn vm_reports_atomic_heap_failure_for_closure_and_partial_allocation() {
    let returned = fixture(
        "returned-closure-oom.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let factory prefix =\n",
            "    let local value = Console.write prefix\n",
            "    local\n\n",
            "let main () =\n",
            "    let callback = factory \"hello\"\n",
            "    callback \"world\"\n",
        ),
    );
    let returned_program = verified_v1_1(&returned);
    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1(
            &returned_program,
            ExecutionLimits::new(1_000, 16, 36),
            &mut host,
        )
        .expect_err("captured closure allocation must exceed the heap limit"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::OutOfMemory {
            operation: "make_closure"
        }
    ));
    assert!(!fault.committed());
    assert_eq!(console.output, "");

    let partial = fixture(
        "partial-oom.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let select prefix value = prefix\n\n",
            "let main () =\n",
            "    let partial = select \"hello\"\n",
            "    Console.write (partial \"world\")\n",
        ),
    );
    let partial_program = verified_v1_1(&partial);
    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1(
            &partial_program,
            ExecutionLimits::new(1_000, 16, 52),
            &mut host,
        )
        .expect_err("partial application allocation must exceed the heap limit"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::OutOfMemory {
            operation: "partial_application"
        }
    ));
    assert!(!fault.committed());
    assert_eq!(console.output, "");
}

#[test]
fn division_by_zero_is_source_mapped_before_later_effects() {
    let program = decode_and_verify_v1(&wire::scalar_artifact(true, false, false))
        .expect("zero-divisor artifact verifies");
    let expected = source_map_entry(&program, 0, 0, 9);
    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1(&program, generous_limits(), &mut host).expect_err("division must fault"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::DivisionByZero {
            operation: "Int.divide"
        }
    ));
    assert_eq!(fault.span(), expected.span);
    assert!(!fault.committed());
    assert_eq!(console.output, "");
}

#[test]
fn invalid_format_is_source_mapped_before_the_console_effect() {
    let program = decode_and_verify_v1(&wire::scalar_artifact(false, false, true))
        .expect("invalid-format artifact verifies");
    let expected = source_map_entry(&program, 0, 0, 21);
    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let fault =
        runtime(execute_v1(&program, generous_limits(), &mut host).expect_err("format must fault"));
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::InvalidFormatPlaceholderCount { count: 2 }
    ));
    assert_eq!(fault.span(), expected.span);
    assert!(!fault.committed());
    assert_eq!(console.output, "");
}

#[test]
fn missing_capability_wins_preflight_before_limits_or_effects() {
    let fixture = fixture("direct-call.ling", DIRECT_CALL);
    let program = verified(&fixture);
    let expected = source_map_entry(&program, program.model().entry().get(), 0, 0);
    let mut host = HostCapabilities::none();
    let fault = runtime(
        execute_v1(&program, ExecutionLimits::new(0, 0, 0), &mut host)
            .expect_err("missing capability must fail preflight"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::CapabilityUnavailable {
            capability: "Console.Write"
        }
    ));
    assert_eq!(fault.span(), expected.span);
    assert!(!fault.committed());
}

#[test]
fn step_limit_faults_before_the_operation_and_preserves_prior_effects() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    Console.write \"first\"\n",
        "    Console.write \"second\"\n",
    );
    let fixture = fixture("two-effects.ling", text);
    let program = verified(&fixture);
    let entry = program.model().entry().get();
    let function = &program.model().functions()[entry as usize];
    let ordinals = function.blocks[0]
        .instructions
        .iter()
        .enumerate()
        .filter_map(|(ordinal, instruction)| {
            matches!(instruction, Instruction::ConsoleWrite { .. }).then_some(ordinal)
        })
        .collect::<Vec<_>>();
    assert_eq!(ordinals.len(), 2);
    let second = u32::try_from(ordinals[1]).expect("ordinal fits u32");
    let expected = source_map_entry(&program, entry, 0, second);

    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let limits = ExecutionLimits::new(u64::from(second), 16, 1_024);
    let fault =
        runtime(execute_v1(&program, limits, &mut host).expect_err("step limit must fault"));
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::ResourceLimit {
            resource: RuntimeResource::Step
        }
    ));
    assert_eq!(fault.span(), expected.span);
    assert!(fault.committed());
    assert_eq!(console.output, "first\n");
}

#[test]
fn frame_and_heap_limits_fail_without_partial_operations() {
    let direct = fixture("direct-call.ling", DIRECT_CALL);
    let direct_program = verified(&direct);
    let entry = direct_program.model().entry().get();
    let call_ordinal = direct_program.model().functions()[entry as usize].blocks[0]
        .instructions
        .iter()
        .position(|instruction| matches!(instruction, Instruction::Call { .. }))
        .expect("direct fixture contains a call");
    let expected = source_map_entry(
        &direct_program,
        entry,
        0,
        u32::try_from(call_ordinal).expect("ordinal fits u32"),
    );
    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1(
            &direct_program,
            ExecutionLimits::new(100, 1, 1_024),
            &mut host,
        )
        .expect_err("callee frame must exceed limit"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::ResourceLimit {
            resource: RuntimeResource::Frame
        }
    ));
    assert_eq!(fault.span(), expected.span);
    assert!(!fault.committed());
    assert_eq!(console.output, "");

    let hello = fixture("hello.ling", HELLO);
    let hello_program = verified(&hello);
    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1(&hello_program, ExecutionLimits::new(100, 16, 0), &mut host)
            .expect_err("Text constant must exceed the heap ceiling"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::OutOfMemory {
            operation: "constant_text"
        }
    ));
    assert!(!fault.committed());
    assert_eq!(console.output, "");
}

#[test]
fn recursive_calls_use_explicit_frames_and_stop_at_the_frame_limit() {
    let program = decode_and_verify_v1(&wire::recursive_artifact())
        .expect("independent recursive artifact verifies");
    let expected = source_map_entry(&program, 0, 0, 0);
    let mut host = HostCapabilities::none();
    let fault = runtime(
        execute_v1(&program, ExecutionLimits::new(1_000, 64, 0), &mut host)
            .expect_err("recursive frames must reach the configured limit"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::ResourceLimit {
            resource: RuntimeResource::Frame
        }
    ));
    assert_eq!(fault.span(), expected.span);
    assert!(!fault.committed());
}

#[test]
fn host_failure_preserves_category_commit_state_and_bilingual_diagnostic() {
    let fixture = fixture("hello.ling", HELLO);
    let program = verified(&fixture);

    for (error, expected_output, committed) in [
        (
            HostError::before_commit(HostErrorCategory::BrokenPipe),
            "",
            false,
        ),
        (
            HostError::after_commit(HostErrorCategory::Interrupted),
            "你好，零\n",
            true,
        ),
    ] {
        let mut console = RecordingConsole::failing(error);
        let mut host = HostCapabilities::with_console(&mut console);
        let fault = runtime(
            execute_v1(&program, generous_limits(), &mut host)
                .expect_err("host adapter must propagate failure"),
        );
        assert!(matches!(
            fault.kind(),
            RuntimeFaultKind::HostCapability {
                operation: "Console.write",
                category
            } if *category == error.category()
        ));
        assert_eq!(fault.committed(), committed);
        assert_eq!(console.output, expected_output);
        let diagnostic = fault.to_diagnostic();
        assert_eq!(diagnostic.code(), codes::RUNTIME_FAULT);
        let json = diagnostic.render_json().expect("diagnostic renders");
        assert_eq!(
            fault.to_string(),
            format!("{}: Console.write", error.category().name())
        );
        assert!(json.contains(error.category().name()));
        assert!(json.contains("\"operation\":\"Console.write\""));
        assert!(json.contains(&format!("\"committed\":{committed}")));
    }
}
