mod support;

use ling_bytecode::{
    Instruction, LoweringSource, SourceMapEntry, VerifiedProgramV1, decode_and_verify_v1,
    decode_and_verify_v1_1, decode_and_verify_v1_2, decode_and_verify_v1_3, decode_and_verify_v1_4,
    encode_v1, encode_v1_1, encode_v1_2, encode_v1_3, encode_v1_4, lower_v1, lower_v1_1,
    lower_v1_2, lower_v1_3, lower_v1_4,
};
use ling_diagnostics::codes;
use ling_effects::locate_main;
use ling_eval::{MemoryConsole, execute_main};
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceFile, SourceId};
use ling_vm::{
    CancellationToken, ConsoleCapability, ExecutionError, ExecutionLimits, HostCapabilities,
    HostError, HostErrorCategory, RuntimeFault, RuntimeFaultKind, RuntimeResource, execute_v1,
    execute_v1_with_cancellation,
};

use support::wire;

const DIRECT_CALL: &str = include_str!("../../../tests/bytecode/v1/programs/direct-call.ling");
const HELLO: &str = include_str!("../../../tests/bytecode/v1/programs/hello.ling");
const CELL_STATE_HEX: &str =
    include_str!("../../../tests/bytecode/v1/golden/cell-state-1.4.lbc.hex");

fn decode_hex(value: &str) -> Vec<u8> {
    let value = value.trim();
    assert_eq!(value.len() % 2, 0, "hex fixture has complete bytes");
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).expect("hex fixture is ASCII");
            u8::from_str_radix(text, 16).expect("fixture is hexadecimal")
        })
        .collect()
}

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

fn verified_v1_2(fixture: &Fixture) -> VerifiedProgramV1 {
    let lowered = lower_v1_2(
        &fixture.snapshot,
        &[LoweringSource::new(&fixture.source, "src/Main.ling")],
    )
    .expect("fixture lowers to bytecode 1.2");
    let bytes = encode_v1_2(&lowered).expect("fixture encodes");
    decode_and_verify_v1_2(&bytes).expect("fixture independently verifies")
}

fn verified_v1_3(fixture: &Fixture) -> VerifiedProgramV1 {
    let lowered = lower_v1_3(
        &fixture.snapshot,
        &[LoweringSource::new(&fixture.source, "src/Main.ling")],
    )
    .expect("fixture lowers to bytecode 1.3");
    let bytes = encode_v1_3(&lowered).expect("fixture encodes");
    decode_and_verify_v1_3(&bytes).expect("fixture independently verifies")
}

fn verified_v1_4(fixture: &Fixture) -> VerifiedProgramV1 {
    let lowered = lower_v1_4(
        &fixture.snapshot,
        &[LoweringSource::new(&fixture.source, "src/Main.ling")],
    )
    .expect("fixture lowers to bytecode 1.4");
    let bytes = encode_v1_4(&lowered).expect("fixture encodes");
    decode_and_verify_v1_4(&bytes).expect("fixture independently verifies")
}

fn generous_limits() -> ExecutionLimits {
    ExecutionLimits::new(100_000, 1_024, 16 * 1024 * 1024)
}

#[test]
fn executes_bytecode_1_4_cells_with_exact_heap_accounting() {
    let program = decode_and_verify_v1_4(&decode_hex(CELL_STATE_HEX))
        .expect("canonical Cell/State artifact verifies");
    execute_v1(
        &program,
        ExecutionLimits::new(5, 1, 25),
        &mut HostCapabilities::none(),
    )
    .expect("one-byte Int plus one 24-byte Cell fits exactly");

    let error = execute_v1(
        &program,
        ExecutionLimits::new(5, 1, 24),
        &mut HostCapabilities::none(),
    )
    .expect_err("one-under Cell heap budget fails before allocation");
    let ExecutionError::Runtime(fault) = error else {
        panic!("Cell allocation limit is a source-mapped runtime Fault");
    };
    assert_eq!(
        fault.kind(),
        &RuntimeFaultKind::OutOfMemory {
            operation: "cell.new"
        }
    );
    assert!(!fault.committed());
    assert_eq!(fault.span().start_byte(), 0);
    assert_eq!(fault.span().end_byte(), 1);
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

struct PanickingConsole;

impl ConsoleCapability for PanickingConsole {
    fn write_line(&mut self, _text: &str) -> Result<(), HostError> {
        panic!("injected host adapter panic")
    }
}

struct CancellingConsole {
    output: String,
    token: CancellationToken,
}

impl ConsoleCapability for CancellingConsole {
    fn write_line(&mut self, text: &str) -> Result<(), HostError> {
        self.output.push_str(text);
        self.output.push('\n');
        self.token.cancel();
        Ok(())
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
fn vm_executes_checked_console_handlers_without_intercepted_host_output() {
    for (name, clause) in [
        (
            "handler-zero.ling",
            "        operation Console.Write.write(message, resume) -> ()\n",
        ),
        (
            "handler-resume.ling",
            "        operation Console.Write.write(message, resume) -> resume ()\n",
        ),
    ] {
        let text = format!(
            "module Main\n    requires Console.Write\n\nlet main () =\n    handle Console.write \"handled\" with\n{clause}"
        );
        let fixture = fixture(name, &text);
        let program = verified_v1_3(&fixture);
        assert!(program.entry_console_capability_required());
        let mut console = RecordingConsole::default();
        let mut host = HostCapabilities::with_console(&mut console);
        execute_v1(&program, generous_limits(), &mut host).expect("Handler executes");
        assert_eq!(console.output, "", "{name}");
    }
}

#[test]
fn vm_cells_match_interpreter_mutation_before_after_and_without_resume() {
    for (name, clause_body, expected) in [
        ("cell-zero-resume.ling", "cell <- 1", "1\n"),
        (
            "cell-before-resume.ling",
            "cell <- 2\n                resume ()",
            "2\n",
        ),
        (
            "cell-after-resume.ling",
            "resume ()\n                cell <- 3",
            "3\n",
        ),
    ] {
        let text = format!(
            concat!(
                "module Main\n",
                "    requires Console.Write\n\n",
                "let main () =\n",
                "    let mutable cell = 0\n",
                "    let ignored =\n",
                "        handle Console.write \"handled\" with\n",
                "            operation Console.Write.write(message, resume) ->\n",
                "                {clause_body}\n",
                "    Console.write (Text.format \"{{}}\" cell)\n",
            ),
            clause_body = clause_body,
        );
        let fixture = fixture(name, &text);
        let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
        let mut interpreter_console = MemoryConsole::default();
        execute_main(&fixture.snapshot, &main, &mut interpreter_console)
            .expect("checked interpreter executes shared mutation");

        let program = verified_v1_4(&fixture);
        let mut vm_console = RecordingConsole::default();
        let mut host = HostCapabilities::with_console(&mut vm_console);
        execute_v1(&program, generous_limits(), &mut host)
            .expect("VM executes shared Cell mutation");
        assert_eq!(interpreter_console.output(), expected, "{name}");
        assert_eq!(vm_console.output, expected, "{name}");
    }
}

#[test]
fn vm_closure_and_handler_capture_the_same_cell_identity() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    let mutable cell = 0\n",
        "    let set value = cell <- value\n",
        "    let ignored =\n",
        "        handle Console.write \"handled\" with\n",
        "            operation Console.Write.write(message, resume) ->\n",
        "                set 7\n",
        "                resume ()\n",
        "    Console.write (Text.format \"{}\" cell)\n",
    );
    let fixture = fixture("handler-cell-alias.ling", text);
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
    let mut interpreter_console = MemoryConsole::default();
    execute_main(&fixture.snapshot, &main, &mut interpreter_console)
        .expect("checked interpreter executes aliased mutation");

    let program = verified_v1_4(&fixture);
    let mut vm_console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut vm_console);
    execute_v1(&program, generous_limits(), &mut host).expect("VM executes aliased Cell mutation");
    assert_eq!(interpreter_console.output(), "7\n");
    assert_eq!(vm_console.output, "7\n");
}

#[test]
fn vm_cell_mutation_commits_before_a_later_clause_fault() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    let mutable cell = 0\n",
        "    handle Console.write \"handled\" with\n",
        "        operation Console.Write.write(message, resume) ->\n",
        "            cell <- 5\n",
        "            let ignored = 1 / 0\n",
        "            ()\n",
    );
    let fixture = fixture("handler-cell-fault.ling", text);
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
    let mut interpreter_console = MemoryConsole::default();
    let interpreter_fault = execute_main(&fixture.snapshot, &main, &mut interpreter_console)
        .expect_err("checked interpreter preserves the clause Fault");

    let program = verified_v1_4(&fixture);
    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1(&program, generous_limits(), &mut host)
            .expect_err("VM preserves the clause Fault after CellSet"),
    );
    assert!(matches!(
        interpreter_fault.kind,
        ling_eval::RuntimeFaultKind::DivisionByZero
    ));
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::DivisionByZero {
            operation: "Int.divide"
        }
    ));
    assert_eq!(
        u64::from(interpreter_fault.span.start().get()),
        fault.span().start_byte()
    );
    assert_eq!(
        u64::from(interpreter_fault.span.end().get()),
        fault.span().end_byte()
    );
    assert!(fault.committed(), "CellSet commits before its Unit result");
    assert_eq!(interpreter_console.output(), "");
    assert_eq!(console.output, "");
}

#[test]
fn cancellation_precedes_pending_cell_set() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    let mutable cell = 0\n",
        "    let value = 5\n",
        "    handle Console.write \"handled\" with\n",
        "        operation Console.Write.write(message, resume) ->\n",
        "            Console.write \"cancel\"\n",
        "            cell <- value\n",
        "            resume ()\n",
    );
    let fixture = fixture("handler-cell-cancel.ling", text);
    let program = verified_v1_4(&fixture);
    let token = CancellationToken::new();
    let mut console = CancellingConsole {
        output: String::new(),
        token: token.clone(),
    };
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1_with_cancellation(&program, generous_limits(), &mut host, &token)
            .expect_err("cancellation wins before the pending CellSet"),
    );
    let assignment_start = u64::try_from(text.find("cell <- value").expect("assignment span"))
        .expect("source offset fits");
    assert_eq!(fault.kind(), &RuntimeFaultKind::Cancelled);
    assert_eq!(fault.span().start_byte(), assignment_start);
    assert!(
        fault.committed(),
        "the preceding host write remains committed"
    );
    assert_eq!(console.output, "cancel\n");
}

#[test]
fn cancellation_precedes_restoration_and_preserves_prior_cell_mutation() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    let mutable cell = 0\n",
        "    handle Console.write \"handled\" with\n",
        "        operation Console.Write.write(message, resume) ->\n",
        "            cell <- 5\n",
        "            Console.write \"cancel\"\n",
        "            resume ()\n",
    );
    let fixture = fixture("handler-cell-restore-cancel.ling", text);
    let program = verified_v1_4(&fixture);
    let token = CancellationToken::new();
    let mut console = CancellingConsole {
        output: String::new(),
        token: token.clone(),
    };
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1_with_cancellation(&program, generous_limits(), &mut host, &token)
            .expect_err("cancellation wins before continuation restoration"),
    );
    let resume_argument_start =
        u64::try_from(text.rfind("()").expect("resume argument span")).expect("source offset fits");
    assert_eq!(fault.kind(), &RuntimeFaultKind::Cancelled);
    assert_eq!(fault.span().start_byte(), resume_argument_start);
    assert!(
        fault.committed(),
        "CellSet and the host write remain committed"
    );
    assert_eq!(console.output, "cancel\n");
}

#[test]
fn masked_console_handler_still_requires_injected_capability() {
    let fixture = fixture(
        "handler-capability.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"handled\" with\n",
            "        operation Console.Write.write(message, resume) -> ()\n",
        ),
    );
    let program = verified_v1_3(&fixture);
    let mut host = HostCapabilities::none();
    let fault = runtime(
        execute_v1(&program, generous_limits(), &mut host)
            .expect_err("unmasked capability closure requires injection"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::CapabilityUnavailable {
            capability: "Console.Write"
        }
    ));
}

#[test]
fn vm_reinstalls_deep_handlers_and_exposes_clause_effects_outward() {
    for (name, text) in [
        (
            "handler-deep.ling",
            concat!(
                "module Main\n",
                "    requires Console.Write\n\n",
                "let emitBoth () =\n",
                "    Console.write \"first\"\n",
                "    Console.write \"second\"\n\n",
                "let main () =\n",
                "    handle emitBoth () with\n",
                "        operation Console.Write.write(message, resume) ->\n",
                "            if message == \"first\" then resume () else ()\n",
            ),
        ),
        (
            "handler-nested.ling",
            concat!(
                "module Main\n",
                "    requires Console.Write\n\n",
                "let inner () =\n",
                "    handle Console.write \"body\" with\n",
                "        operation Console.Write.write(message, resume) ->\n",
                "            Console.write \"inner clause\"\n\n",
                "let main () =\n",
                "    handle inner () with\n",
                "        operation Console.Write.write(message, resume) -> ()\n",
            ),
        ),
    ] {
        let fixture = fixture(name, text);
        let program = verified_v1_3(&fixture);
        let mut console = RecordingConsole::default();
        let mut host = HostCapabilities::with_console(&mut console);
        execute_v1(&program, generous_limits(), &mut host).expect("deep Handler executes");
        assert_eq!(console.output, "", "{name}");
    }
}

#[test]
fn vm_rejects_a_second_once_resume_with_the_registered_fault_projection() {
    let fixture = fixture(
        "handler-cardinality.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let invokeTwice callback =\n",
            "    let ignored = callback ()\n",
            "    callback ()\n\n",
            "let main () =\n",
            "    handle Console.write \"body\" with\n",
            "        operation Console.Write.write(message, resume) -> invokeTwice resume\n",
        ),
    );
    let program = verified_v1_3(&fixture);
    let mut console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1(&program, generous_limits(), &mut host)
            .expect_err("Once continuation rejects its second restoration"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::HandlerResumeCardinality {
            operation: "Console.Write.write"
        }
    ));
    let diagnostic = fault.to_diagnostic();
    assert_eq!(diagnostic.code(), codes::RUNTIME_FAULT);
    assert_eq!(
        diagnostic
            .facts()
            .get("category")
            .and_then(|value| value.as_str()),
        Some("handler_resume_cardinality")
    );
}

#[test]
fn handler_and_continuation_limits_fail_before_the_bounded_action() {
    let fixture = fixture(
        "handler-limits.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"body\" with\n",
            "        operation Console.Write.write(message, resume) -> resume ()\n",
        ),
    );
    let program = verified_v1_3(&fixture);
    for (limits, expected) in [
        (
            generous_limits().with_handler_limits(0, 1_024),
            RuntimeResource::HandlerDepth,
        ),
        (
            generous_limits().with_handler_limits(1_024, 0),
            RuntimeResource::ContinuationFrame,
        ),
    ] {
        let mut console = RecordingConsole::default();
        let mut host = HostCapabilities::with_console(&mut console);
        let fault = runtime(
            execute_v1(&program, limits, &mut host).expect_err("bounded Handler action faults"),
        );
        assert_eq!(
            fault.kind(),
            &RuntimeFaultKind::ResourceLimit { resource: expected }
        );
        assert_eq!(console.output, "");
    }
}

#[test]
fn cancellation_wins_before_continuation_restoration_and_preserves_committed_output() {
    let fixture = fixture(
        "handler-cancel.ling",
        concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let main () =\n",
            "    handle Console.write \"trigger\" with\n",
            "        operation Console.Write.write(message, resume) ->\n",
            "            Console.write \"committed\"\n",
            "            resume ()\n",
        ),
    );
    let program = verified_v1_3(&fixture);
    let token = CancellationToken::new();
    let mut console = CancellingConsole {
        output: String::new(),
        token: token.clone(),
    };
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1_with_cancellation(&program, generous_limits(), &mut host, &token)
            .expect_err("cancellation precedes resume restoration"),
    );
    assert_eq!(fault.kind(), &RuntimeFaultKind::Cancelled);
    assert!(fault.committed());
    assert_eq!(console.output, "committed\n");
}

#[test]
fn vm_matches_the_checked_interpreter_for_static_trait_member_dispatch() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "trait Renderable<'a> =\n",
        "    render: 'a -> Text -> Text\n\n",
        "type Item = { name: Text }\n\n",
        "impl Renderable Item =\n",
        "    let render item suffix = suffix\n\n",
        "let main () =\n",
        "    let render = Renderable.render { name = \"Ling\" }\n",
        "    Console.write (render \"Ling\")\n",
    );
    let fixture = fixture("trait-dispatch.ling", text);
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
    let mut interpreter_console = MemoryConsole::default();
    execute_main(&fixture.snapshot, &main, &mut interpreter_console)
        .expect("interpreter executes Trait dispatch");

    let program = verified_v1_2(&fixture);
    let mut vm_console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut vm_console);
    execute_v1(&program, generous_limits(), &mut host).expect("v1.2 VM executes Trait dispatch");
    assert_eq!(vm_console.output, interpreter_console.output(), "v1.2");
    assert_eq!(interpreter_console.output(), "Ling\n");
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
fn vm_matches_the_checked_interpreter_for_v1_2_variant_match_execution() {
    let text = concat!(
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
    let fixture = fixture("v1_2-variant-match.ling", text);
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
    let mut interpreter_console = MemoryConsole::default();
    execute_main(&fixture.snapshot, &main, &mut interpreter_console)
        .expect("interpreter executes aggregate match");

    let program = verified_v1_2(&fixture);
    let mut vm_console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut vm_console);
    execute_v1(&program, generous_limits(), &mut host).expect("VM executes aggregate match");
    assert_eq!(vm_console.output, interpreter_console.output());
    assert_eq!(vm_console.output, "7\n");
}

#[test]
fn vm_matches_the_checked_interpreter_for_v1_2_record_tuple_execution() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "type Point = { x: Int; y: Int }\n\n",
        "let total point =\n",
        "    let changed = { point with x = point.x + 1 }\n",
        "    match (changed.x, changed.y) with\n",
        "    | (x, y) -> x + y\n\n",
        "let main () =\n",
        "    Console.write (Text.format \"{}\" (total { x = 1; y = 2 }))\n",
    );
    let fixture = fixture("v1_2-record-tuple.ling", text);
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
    let mut interpreter_console = MemoryConsole::default();
    execute_main(&fixture.snapshot, &main, &mut interpreter_console)
        .expect("interpreter executes record and tuple aggregate");

    let program = verified_v1_2(&fixture);
    let mut vm_console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut vm_console);
    execute_v1(&program, generous_limits(), &mut host)
        .expect("VM executes record and tuple aggregate");
    assert_eq!(vm_console.output, interpreter_console.output());
    assert_eq!(vm_console.output, "4\n");
}

#[test]
fn vm_matches_the_checked_interpreter_for_v1_2_mutable_place_execution() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "type Inner = { mutable value: Int }\n",
        "type Counter = { mutable inner: Inner }\n\n",
        "let mutate flag =\n",
        "    let mutable counter = { inner = { value = 0 } }\n",
        "    counter <- { inner = { value = 9 } }\n",
        "    if flag then\n",
        "        counter.inner.value <- 1\n",
        "    else\n",
        "        counter.inner.value <- 2\n",
        "    counter.inner.value\n\n",
        "let main () =\n",
        "    Console.write (Text.format \"{}\" (mutate true))\n",
    );
    let fixture = fixture("v1_2-mutable-place.ling", text);
    let main = locate_main(fixture.snapshot.checked()).expect("fixture has main");
    let mut interpreter_console = MemoryConsole::default();
    execute_main(&fixture.snapshot, &main, &mut interpreter_console)
        .expect("interpreter executes mutable place assignment");

    let program = verified_v1_2(&fixture);
    let mut vm_console = RecordingConsole::default();
    let mut host = HostCapabilities::with_console(&mut vm_console);
    execute_v1(&program, generous_limits(), &mut host)
        .expect("VM executes mutable place assignment");
    assert_eq!(vm_console.output, interpreter_console.output());
    assert_eq!(vm_console.output, "1\n");
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
fn cancellation_is_checked_before_preflight_and_preserves_no_commit() {
    let fixture = fixture("hello.ling", HELLO);
    let program = verified(&fixture);
    let expected = source_map_entry(&program, program.model().entry().get(), 0, 0);
    let token = CancellationToken::new();
    token.cancel();
    let mut host = HostCapabilities::none();
    let fault = runtime(
        execute_v1_with_cancellation(&program, ExecutionLimits::new(0, 0, 0), &mut host, &token)
            .expect_err("pre-cancelled execution must stop before preflight"),
    );
    assert!(matches!(fault.kind(), RuntimeFaultKind::Cancelled));
    assert_eq!(fault.span(), expected.span);
    assert!(!fault.committed());
    assert_eq!(fault.to_string(), "cancelled: execution.cancelled");
    let diagnostic = fault.to_diagnostic();
    assert_eq!(diagnostic.code(), codes::RUNTIME_FAULT);
    let json = diagnostic.render_json().expect("diagnostic renders");
    assert!(json.contains("execution was cancelled"));
    assert!(json.contains("\"category\":\"cancelled\""));
}

#[test]
fn cancellation_after_console_effect_reports_committed_source_mapped_fault() {
    let fixture = fixture("hello.ling", HELLO);
    let program = verified(&fixture);
    let entry = program.model().entry().get();
    let console_ordinal = program.model().functions()[entry as usize].blocks[0]
        .instructions
        .iter()
        .position(|instruction| matches!(instruction, Instruction::ConsoleWrite { .. }))
        .expect("hello fixture contains a Console.write instruction");
    let expected = source_map_entry(
        &program,
        entry,
        0,
        u32::try_from(console_ordinal + 1).expect("next executable ordinal fits u32"),
    );
    let token = CancellationToken::new();
    let mut console = CancellingConsole {
        output: String::new(),
        token: token.clone(),
    };
    let mut host = HostCapabilities::with_console(&mut console);
    let fault = runtime(
        execute_v1_with_cancellation(&program, generous_limits(), &mut host, &token)
            .expect_err("host-requested cancellation must stop at the next checkpoint"),
    );
    assert!(matches!(fault.kind(), RuntimeFaultKind::Cancelled));
    assert_eq!(fault.span(), expected.span);
    assert!(fault.committed());
    assert_eq!(console.output, "你好，零\n");
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

#[test]
fn host_panic_becomes_stable_other_fault_without_escaping_the_vm() {
    let fixture = fixture("hello.ling", HELLO);
    let program = verified(&fixture);
    let entry = program.model().entry().get();
    let console_ordinal = program.model().functions()[entry as usize].blocks[0]
        .instructions
        .iter()
        .position(|instruction| matches!(instruction, Instruction::ConsoleWrite { .. }))
        .expect("hello fixture contains a Console.write instruction");
    let expected = source_map_entry(
        &program,
        entry,
        0,
        u32::try_from(console_ordinal).expect("ordinal fits u32"),
    );
    let mut console = PanickingConsole;
    let mut host = HostCapabilities::with_console(&mut console);

    let fault = runtime(
        execute_v1(&program, generous_limits(), &mut host)
            .expect_err("host panic must become a Runtime Fault"),
    );
    assert!(matches!(
        fault.kind(),
        RuntimeFaultKind::HostCapability {
            operation: "Console.write",
            category: HostErrorCategory::Other,
        }
    ));
    assert!(fault.committed());
    assert_eq!(fault.span(), expected.span);
    assert_eq!(fault.to_diagnostic().code(), codes::RUNTIME_FAULT);
}
