//! REP-2501 private executable evidence matrix authorized by Accepted DEC-0279.
//!
//! The labels in this module classify only the five exact test executions
//! below. They are not source syntax, a production classifier, serialized
//! metadata, or a public Replay contract.

use ling_ast::lower as lower_ast;
use ling_effects::{CheckedProgram, check, locate_main};
use ling_hir::lower as lower_hir;
use ling_resolve::{DefinitionId, resolve};
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::check as check_types;

use crate::{
    HostErrorCategory, LocalTaskControl, LocalTaskSchedulerConfig, LocalTaskTerminal,
    MemoryConsole, TaskDeadline, TaskHostOutcome, TaskHostResponse, TaskHostScript, TaskPath,
    TaskRuntimeLimits, TaskRuntimeState, TaskScheduleConfig, TaskScheduleEventKind,
    TaskScheduleTerminal, TaskSchedulerLimits, TaskValue, execute_main, replay_task_schedule,
    run_local_task, run_task_schedule,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceCategory {
    PureDeterministic,
    SeedDeterministic(&'static str),
    InputDeterministic(&'static str),
    ScheduleDeterministic,
    Nondeterministic(&'static str),
}

impl EvidenceCategory {
    fn name(self) -> &'static str {
        match self {
            Self::PureDeterministic => "PureDeterministic",
            Self::SeedDeterministic("RandomSource") => "SeedDeterministic<RandomSource>",
            Self::InputDeterministic("EffectLog") => "InputDeterministic<EffectLog>",
            Self::ScheduleDeterministic => "ScheduleDeterministic",
            Self::Nondeterministic("unrecorded-local-task-scheduler") => {
                "Nondeterministic(unrecorded-local-task-scheduler)"
            }
            Self::SeedDeterministic(_) => "invalid-seed-evidence-parameter",
            Self::InputDeterministic(_) => "invalid-input-evidence-parameter",
            Self::Nondeterministic(_) => "invalid-nondeterministic-evidence-reason",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceCase {
    PureCheckedExecution,
    SeedTaskSchedule,
    InputTaskReplay,
    ScheduleActorScript,
    NondeterministicProductionTaskBoundary,
}

const EVIDENCE_CASES: [(EvidenceCase, EvidenceCategory, &str); 5] = [
    (
        EvidenceCase::PureCheckedExecution,
        EvidenceCategory::PureDeterministic,
        "pure-deterministic-checked-execution",
    ),
    (
        EvidenceCase::SeedTaskSchedule,
        EvidenceCategory::SeedDeterministic("RandomSource"),
        "seed-deterministic-task-schedule",
    ),
    (
        EvidenceCase::InputTaskReplay,
        EvidenceCategory::InputDeterministic("EffectLog"),
        "input-deterministic-task-replay",
    ),
    (
        EvidenceCase::ScheduleActorScript,
        EvidenceCategory::ScheduleDeterministic,
        "schedule-deterministic-actor-script",
    ),
    (
        EvidenceCase::NondeterministicProductionTaskBoundary,
        EvidenceCategory::Nondeterministic("unrecorded-local-task-scheduler"),
        "nondeterministic-production-task-boundary",
    ),
];

fn assert_registered(
    case: EvidenceCase,
    category: EvidenceCategory,
    expected_name: &str,
    expected_category: &str,
) {
    assert_eq!(EVIDENCE_CASES.len(), 5);
    let matches = EVIDENCE_CASES
        .iter()
        .filter(|(candidate, _, _)| *candidate == case)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "each DEC-0279 case occurs exactly once");
    assert_eq!(matches[0].1, category);
    assert_eq!(matches[0].2, expected_name);
    assert_eq!(matches[0].1.name(), expected_category);

    let names = EVIDENCE_CASES
        .iter()
        .map(|(_, _, name)| *name)
        .collect::<Vec<_>>();
    assert_eq!(names.len(), 5);
    for provisional_plan_label in ["Strict", "Seeded", "RecordedEffects", "BestEffort"] {
        assert!(
            EVIDENCE_CASES
                .iter()
                .all(
                    |(_, category, name)| category.name() != provisional_plan_label
                        && *name != provisional_plan_label
                ),
            "DEC-0104 plan vocabulary must remain separate"
        );
    }
}

fn checked_at(source_id: u32, source_name: &str, bytes: Vec<u8>) -> CheckedProgram {
    let source = SourceFile::from_bytes(SourceId::new(source_id), source_name, bytes)
        .expect("valid UTF-8 source");
    let parsed = parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = lower_ast(&source, &parsed).expect("valid AST");
    let hir = lower_hir(source.name(), &ast).expect("valid HIR");
    let resolved = resolve(vec![hir], "Main").expect("resolved program");
    let typed = check_types(resolved).expect("typed program");
    check(typed).expect("checked program")
}

fn snapshot_at(source_id: u32, source_name: &str, bytes: Vec<u8>) -> ProgramSnapshot {
    ling_semantic::build(checked_at(source_id, source_name, bytes)).expect("semantic snapshot")
}

fn task(checked: &CheckedProgram, name: &str) -> DefinitionId {
    checked
        .task_cores()
        .keys()
        .find(|definition| {
            checked
                .typed()
                .resolved()
                .definition(definition)
                .is_some_and(|info| info.name == name)
        })
        .cloned()
        .expect("checked Task definition")
}

fn task_schedule_config(seed: u64) -> TaskScheduleConfig {
    TaskScheduleConfig::new(
        seed,
        TaskRuntimeLimits::new(32, 32, 256, 16),
        TaskSchedulerLimits::new(128, 128, 8, 512, 2_048, 128, 16),
    )
}

const TASK_LF: &str = concat!(
    "module Main\n",
    "    requires Console.Write\n\n",
    "task 子任务 text =\n",
    "    scope\n",
    "        Console.write text\n",
    "        return text\n\n",
    "task 父任务 text =\n",
    "    scope\n",
    "        let handle = spawn 子任务 text\n",
    "        let result = await handle\n",
    "        return result\n",
);

const TASK_BOM_CRLF: &str = concat!(
    "\u{feff}module Main\r\n",
    "    requires Console.Write\r\n\r\n",
    "task 子任务 text =\r\n",
    "    scope\r\n",
    "        Console.write text\r\n",
    "        return text\r\n\r\n",
    "task 父任务 text =\r\n",
    "    scope\r\n",
    "        let handle = spawn 子任务 text\r\n",
    "        let result = await handle\r\n",
    "        return result\r\n",
);

fn terminal(trace: &crate::TaskScheduleTrace) -> &TaskScheduleTerminal {
    match trace.events().last().expect("closure event").kind() {
        TaskScheduleEventKind::Closure { terminal, .. } => terminal,
        other => panic!("expected closure, found {other:?}"),
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PureProjection {
    program_id: String,
    body_id: String,
    outcome: &'static str,
    host_events: usize,
}

fn pure_projection(snapshot: &ProgramSnapshot) -> PureProjection {
    let main = locate_main(snapshot.checked()).expect("ordinary Seed main");
    let module = snapshot.checked().typed().resolved().entry_module();
    assert!(
        snapshot
            .checked()
            .definition_effect(&main)
            .expect("main residual Effect row")
            .is_pure()
    );
    assert!(
        snapshot
            .checked()
            .module_capabilities(module.id)
            .expect("module Capability closure")
            .is_empty()
    );

    let mut console = MemoryConsole::default();
    execute_main(snapshot, &main, &mut console).expect("pure checked execution");
    assert!(console.output().is_empty());
    PureProjection {
        program_id: snapshot.program_id().as_str().to_owned(),
        body_id: snapshot
            .body_id(&main)
            .expect("main body ID")
            .as_str()
            .to_owned(),
        outcome: "Value(Unit)",
        host_events: 0,
    }
}

#[test]
fn pure_deterministic_checked_execution() {
    assert_registered(
        EvidenceCase::PureCheckedExecution,
        EvidenceCategory::PureDeterministic,
        "pure-deterministic-checked-execution",
        "PureDeterministic",
    );
    let left_source = concat!(
        "module Main\n\n",
        "let 前置 = 1\n",
        "let main () = ()\n",
        "let 后置 = 2\n",
    );
    let right_source = concat!(
        "\u{feff}module Main\r\n\r\n",
        "let 后置 = 2\r\n",
        "let main () = ()\r\n",
        "let 前置 = 1\r\n",
    );
    let left = snapshot_at(7, "左侧/纯函数.ling", left_source.as_bytes().to_vec());
    let right = snapshot_at(77, "different/right.ling", right_source.as_bytes().to_vec());

    let first = pure_projection(&left);
    let repeated = pure_projection(&left);
    let reconstructed = pure_projection(&right);
    assert_eq!(first, repeated);
    assert_eq!(first, reconstructed);

    let left_main = locate_main(left.checked()).expect("left main");
    let right_main = locate_main(right.checked()).expect("right main");
    let left_info = left
        .checked()
        .typed()
        .resolved()
        .definition(&left_main)
        .expect("left definition");
    let right_info = right
        .checked()
        .typed()
        .resolved()
        .definition(&right_main)
        .expect("right definition");
    assert_ne!(left_info.source_name, right_info.source_name);
    assert_ne!(left_info.span, right_info.span);
}

#[test]
fn seed_deterministic_task_schedule() {
    assert_registered(
        EvidenceCase::SeedTaskSchedule,
        EvidenceCategory::SeedDeterministic("RandomSource"),
        "seed-deterministic-task-schedule",
        "SeedDeterministic<RandomSource>",
    );
    let left = checked_at(11, "左侧/task.ling", TASK_LF.as_bytes().to_vec());
    let right = checked_at(
        111,
        "different/right-task.ling",
        TASK_BOM_CRLF.as_bytes().to_vec(),
    );
    let left_root = task(&left, "父任务");
    let right_root = task(&right, "父任务");
    let arguments = vec![TaskValue::Text("你好".to_owned())];
    let deadlines = vec![TaskDeadline::new(64, TaskPath::root())];
    let config = task_schedule_config(0xd15e_a5e5_0279_0001);

    for (response, expected_host, expected_faulted) in [
        (
            TaskHostResponse::Complete,
            TaskHostOutcome::Completed,
            false,
        ),
        (
            TaskHostResponse::Fail(HostErrorCategory::BrokenPipe),
            TaskHostOutcome::Failed(HostErrorCategory::BrokenPipe),
            true,
        ),
    ] {
        let host_script = TaskHostScript::new([response]);
        let first = run_task_schedule(
            &left,
            &left_root,
            arguments.clone(),
            config,
            deadlines.clone(),
            host_script.clone(),
        )
        .expect("seeded Task schedule");
        let repeated = run_task_schedule(
            &left,
            &left_root,
            arguments.clone(),
            config,
            deadlines.clone(),
            host_script.clone(),
        )
        .expect("repeated seeded Task schedule");
        let reconstructed = run_task_schedule(
            &right,
            &right_root,
            arguments.clone(),
            config,
            deadlines.clone(),
            host_script,
        )
        .expect("reconstructed seeded Task schedule");

        first.validate().expect("validated first trace");
        repeated.validate().expect("validated repeated trace");
        reconstructed
            .validate()
            .expect("validated reconstructed trace");
        assert_eq!(first.canonical_bytes(), repeated.canonical_bytes());
        assert_eq!(first.canonical_bytes(), reconstructed.canonical_bytes());
        assert!(first.events().iter().any(|event| matches!(
            event.kind(),
            TaskScheduleEventKind::Host { text, outcome }
                if text == "你好\n" && outcome == &expected_host
        )));
        assert_eq!(
            matches!(terminal(&first), TaskScheduleTerminal::Faulted { .. }),
            expected_faulted
        );
    }
}

#[test]
fn input_deterministic_task_replay() {
    assert_registered(
        EvidenceCase::InputTaskReplay,
        EvidenceCategory::InputDeterministic("EffectLog"),
        "input-deterministic-task-replay",
        "InputDeterministic<EffectLog>",
    );
    let original = checked_at(12, "recorded/task.ling", TASK_LF.as_bytes().to_vec());
    let reconstructed = checked_at(
        212,
        "fresh/reconstructed.ling",
        TASK_BOM_CRLF.as_bytes().to_vec(),
    );
    let original_root = task(&original, "父任务");
    let reconstructed_root = task(&reconstructed, "父任务");
    let arguments = vec![TaskValue::Text("回放输入".to_owned())];
    let trace = run_task_schedule(
        &original,
        &original_root,
        arguments.clone(),
        task_schedule_config(u64::MAX),
        vec![TaskDeadline::new(64, TaskPath::root())],
        TaskHostScript::new([TaskHostResponse::Complete]),
    )
    .expect("complete recorded Task trace");
    let replayed = replay_task_schedule(
        &reconstructed,
        &reconstructed_root,
        arguments.clone(),
        &trace,
    )
    .expect("strict in-process replay");
    assert_eq!(trace.canonical_bytes(), replayed.canonical_bytes());

    let identity_error = replay_task_schedule(
        &reconstructed,
        &reconstructed_root,
        vec![TaskValue::Text("mutated input".to_owned())],
        &trace,
    )
    .expect_err("mutated runtime identity must fail before replay");
    assert_eq!(identity_error.event_id(), 0);
    assert_eq!(identity_error.reason(), "runtime_identity_mismatch");

    crate::task_scheduler::tests::replay_reports_the_first_mutated_selection_field();
    crate::task_scheduler::tests::replay_reports_deadline_host_and_terminal_mutations_at_their_event();
}

#[test]
fn schedule_deterministic_actor_script() {
    assert_registered(
        EvidenceCase::ScheduleActorScript,
        EvidenceCategory::ScheduleDeterministic,
        "schedule-deterministic-actor-script",
        "ScheduleDeterministic",
    );
    crate::actor_supervisor::evidence_tests::unicode_reconstruction_determinism();
}

#[derive(Debug, PartialEq)]
struct ProductionTaskProjection {
    terminal: LocalTaskTerminal,
    records: Vec<(TaskPath, TaskRuntimeState, usize)>,
}

fn production_task_projection(worker_count: usize) -> ProductionTaskProjection {
    const SOURCE: &str = concat!(
        "module Main\n\n",
        "task child () =\n",
        "    scope\n",
        "        return ()\n\n",
        "task main () =\n",
        "    scope\n",
        "        let handle = spawn child ()\n",
        "        let value = await handle\n",
        "        return value\n",
    );
    let checked = checked_at(31, "production-boundary.ling", SOURCE.as_bytes().to_vec());
    let root = task(&checked, "main");
    let mut console = MemoryConsole::default();
    let run = run_local_task(
        &checked,
        &root,
        vec![TaskValue::Unit],
        &mut console,
        LocalTaskSchedulerConfig::new(
            worker_count,
            16,
            8,
            128,
            128,
            64,
            TaskRuntimeLimits::new(16, 16, 128, 16),
        ),
        &LocalTaskControl::new(),
    )
    .expect("bounded production Task run");
    assert!(console.output().is_empty());
    assert_eq!(
        run.terminal(),
        &LocalTaskTerminal::Completed(TaskValue::Unit)
    );
    assert_eq!(run.snapshot().records().len(), 2);
    assert!(run.snapshot().records().iter().all(|record| {
        record.cleanup_count() == 1
            && matches!(
                record.state(),
                TaskRuntimeState::Completed(_)
                    | TaskRuntimeState::Cancelled
                    | TaskRuntimeState::Faulted { .. }
            )
    }));
    ProductionTaskProjection {
        terminal: run.terminal().clone(),
        records: run
            .snapshot()
            .records()
            .iter()
            .map(|record| {
                (
                    record.path().clone(),
                    record.state().clone(),
                    record.cleanup_count(),
                )
            })
            .collect(),
    }
}

fn assert_public_surfaces_absent() {
    const EVAL_SOURCE: &str = include_str!("lib.rs");
    const EFFECT_SOURCE: &str = include_str!("../../ling-effects/src/lib.rs");
    const SEMANTIC_SOURCE: &str = include_str!("../../ling-semantic/src/lib.rs");
    const SYNTAX_TOKEN_SOURCE: &str = include_str!("../../ling-syntax/src/token.rs");
    const SYNTAX_PARSER_SOURCE: &str = include_str!("../../ling-syntax/src/parser.rs");
    const AST_SOURCE: &str = include_str!("../../ling-ast/src/lib.rs");
    const PROJECT_SOURCE: &str = include_str!("../../ling-project/src/lib.rs");
    const CLI_CATALOG_SOURCE: &str = include_str!("../../ling-cli/src/command_catalog.rs");
    const DIAGNOSTIC_SOURCE: &str = include_str!("../../ling-diagnostics/src/lib.rs");
    const SCHEMA_REGISTRY: &str = include_str!("../../../schemas/registry.toml");
    const PROTOCOL_INVENTORY: &str =
        include_str!("../../../docs/governance/protocol-inventory.toml");

    for production_source in [EVAL_SOURCE, EFFECT_SOURCE, SEMANTIC_SOURCE, PROJECT_SOURCE] {
        for forbidden_surface in [
            "pub enum DeterminismClass",
            "pub struct DeterminismClass",
            "pub trait DeterminismClassifier",
            "pub fn classify_determinism",
            "determinism_class:",
            "pub struct EffectLog",
            "pub enum EffectLog",
            "effect_log:",
            "pub struct ReplayHeader",
            "replay_header:",
            "pub fn write_replay",
            "pub fn decode_replay",
        ] {
            assert!(
                !production_source.contains(forbidden_surface),
                "forbidden DEC-0279 production surface: {forbidden_surface}"
            );
        }
    }
    for source_surface in [SYNTAX_TOKEN_SOURCE, SYNTAX_PARSER_SOURCE, AST_SOURCE] {
        assert!(!source_surface.contains("DeterminismClass"));
        assert!(!source_surface.contains("ReplayHeader"));
        assert!(!source_surface.contains("EffectLog"));
    }
    assert!(!CLI_CATALOG_SOURCE.contains("Command::Replay"));
    assert!(!CLI_CATALOG_SOURCE.contains("\"replay\" =>"));
    assert!(!DIAGNOSTIC_SOURCE.contains("L-REPLAY-"));
    assert!(!DIAGNOSTIC_SOURCE.contains("L-DETERMINISM-"));
    assert!(!SCHEMA_REGISTRY.to_ascii_lowercase().contains("replay"));

    let replay_protocol = PROTOCOL_INVENTORY
        .split("[[protocol]]")
        .find(|block| block.contains("id = \"PROTO-REPLAY\""))
        .expect("planned Replay inventory record");
    for future_boundary in [
        "current_version = \"\"",
        "stability = \"Future\"",
        "implemented = false",
        "public_schema = false",
        "canonical = false",
        "fixtures = []",
        "version_markers = []",
    ] {
        assert!(replay_protocol.contains(future_boundary));
    }
}

#[test]
fn nondeterministic_production_task_boundary() {
    assert_registered(
        EvidenceCase::NondeterministicProductionTaskBoundary,
        EvidenceCategory::Nondeterministic("unrecorded-local-task-scheduler"),
        "nondeterministic-production-task-boundary",
        "Nondeterministic(unrecorded-local-task-scheduler)",
    );
    let one_worker = production_task_projection(1);
    let four_workers = production_task_projection(4);
    assert_eq!(one_worker, four_workers);
    assert_public_surfaces_absent();
}
