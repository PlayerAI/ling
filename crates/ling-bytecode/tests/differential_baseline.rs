use std::collections::BTreeSet;

use ling_diagnostics::codes;
use ling_effects::locate_main;
use ling_eval::{MemoryConsole, RuntimeFaultKind, evaluate_definition, execute_main};
use ling_resolve::DefinitionId;
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceFile, SourceId};

const CONSTANTS: &str = include_str!("../../../tests/bytecode/v1/programs/constants.ling");
const DIRECT_CALL: &str = include_str!("../../../tests/bytecode/v1/programs/direct-call.ling");
const FORMAT_FAULT: &str = include_str!("../../../tests/bytecode/v1/programs/format-fault.ling");
const HELLO: &str = include_str!("../../../tests/bytecode/v1/programs/hello.ling");
const MALFORMED_CASES: &str = include_str!("../../../tests/bytecode/v1/malformed-cases.tsv");

fn snapshot(logical_name: &str, text: &str) -> ProgramSnapshot {
    let source = SourceFile::from_bytes(SourceId::new(0), logical_name, text.as_bytes().to_vec())
        .expect("fixture is valid UTF-8 source");
    let parsed = ling_syntax::parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = ling_ast::lower(&source, &parsed).expect("fixture has valid AST");
    let hir = ling_hir::lower(source.name(), &ast).expect("fixture has valid HIR");
    let resolved = ling_resolve::resolve(vec![hir], "Main").expect("fixture resolves");
    let typed = ling_types::check(resolved).expect("fixture type-checks");
    let checked = ling_effects::check(typed).expect("fixture passes Effect/Capability checks");
    ling_semantic::build(checked).expect("fixture produces a checked snapshot")
}

fn definition(snapshot: &ProgramSnapshot, name: &str) -> DefinitionId {
    let resolved = snapshot.checked().typed().resolved();
    resolved
        .definition_id(resolved.entry(), name)
        .cloned()
        .unwrap_or_else(|| panic!("fixture definition `{name}` exists"))
}

#[test]
fn checked_interpreter_freezes_the_initial_vm_observables() {
    let hello = snapshot("tests/bytecode/v1/programs/hello.ling", HELLO);
    let hello_main = locate_main(hello.checked()).expect("hello has a valid main");
    let mut console = MemoryConsole::default();
    execute_main(&hello, &hello_main, &mut console).expect("hello executes");
    assert_eq!(console.output(), "你好，零\n");

    let direct_call = snapshot("tests/bytecode/v1/programs/direct-call.ling", DIRECT_CALL);
    let direct_main = locate_main(direct_call.checked()).expect("direct call has a valid main");
    let mut console = MemoryConsole::default();
    execute_main(&direct_call, &direct_main, &mut console).expect("direct call executes");
    assert_eq!(console.output(), "direct call\n");

    let constants = snapshot("tests/bytecode/v1/programs/constants.ling", CONSTANTS);
    let mut console = MemoryConsole::default();
    let integer = evaluate_definition(&constants, &definition(&constants, "integer"), &mut console)
        .expect("integer evaluates");
    let boolean = evaluate_definition(&constants, &definition(&constants, "boolean"), &mut console)
        .expect("boolean evaluates");
    let text = evaluate_definition(&constants, &definition(&constants, "text"), &mut console)
        .expect("text evaluates");
    assert_eq!(
        integer.rendered(),
        "340282366920938463463374607431768211456"
    );
    assert_eq!(boolean.rendered(), "true");
    assert_eq!(text.rendered(), "\"你好，零\"");
    assert_eq!(console.output(), "");
}

#[test]
fn checked_interpreter_freezes_fault_code_category_and_source_span() {
    let logical_name = "tests/bytecode/v1/programs/format-fault.ling";
    let snapshot = snapshot(logical_name, FORMAT_FAULT);
    let main = locate_main(snapshot.checked()).expect("fault fixture has a valid main");
    let mut console = MemoryConsole::default();
    let fault = execute_main(&snapshot, &main, &mut console).expect_err("format must fault");

    assert!(matches!(
        fault.kind,
        RuntimeFaultKind::InvalidFormatPlaceholderCount { count: 2 }
    ));
    assert_eq!(fault.source_name, logical_name);
    assert_eq!(fault.to_diagnostic().code(), codes::RUNTIME_FAULT);
    assert!(
        fault
            .to_diagnostic()
            .render_json()
            .expect("runtime diagnostic renders")
            .contains("\"category\":\"invalid_format\"")
    );
    let expression = "Text.format \"left {} right {}\" 7";
    let start = FORMAT_FAULT
        .find(expression)
        .expect("fault expression exists");
    assert_eq!(fault.span.start().get(), u32::try_from(start).unwrap());
    assert_eq!(
        fault.span.end().get(),
        u32::try_from(start + expression.len()).unwrap()
    );
    assert_eq!(console.output(), "");
}

#[test]
fn malformed_verifier_plan_has_unique_stable_cases() {
    let mut ids = BTreeSet::new();
    let mut lines = MALFORMED_CASES.lines();
    assert_eq!(lines.next(), Some("id\tphase\tmutation\trequired_reason"));
    for line in lines {
        let columns = line.split('\t').collect::<Vec<_>>();
        assert_eq!(columns.len(), 4, "malformed corpus row: {line}");
        assert!(ids.insert(columns[0]), "duplicate case ID: {}", columns[0]);
        assert!(!columns[1].is_empty());
        assert!(!columns[2].is_empty());
        assert!(!columns[3].is_empty());
    }
    assert_eq!(ids.len(), 22);
}
