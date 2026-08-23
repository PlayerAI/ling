//! Opt-in incremental performance evidence.
//!
//! This module records timing samples for the existing `ling-db` query
//! boundary. It deliberately makes no absolute performance promise and is not
//! part of normal builds or language semantics.

use std::fmt::Write as _;
use std::hint::black_box;
use std::time::Instant;

use ling_db::{CompilerDb, QueryOutcome};
use ling_source::{ChangeEvent, SourceId, WorkspaceInput};

pub(crate) const SAMPLE_COUNT: usize = 3;
pub(crate) const SYNTHETIC_FILE_COUNT: usize = 10_000;
pub(crate) const SCENARIO_NAMES: [&str; 8] = [
    "cold_check",
    "warm_check",
    "single_character_edit",
    "signature_edit",
    "cross_package_edit",
    "synthetic_10k_cold_parse",
    "synthetic_10k_warm_parse",
    "synthetic_10k_single_edit",
];

#[derive(Clone, Copy, Debug)]
struct Observation {
    elapsed_ns: u128,
    trace_events: usize,
    misses: usize,
    hits: usize,
    completed_items: usize,
}

#[derive(Debug)]
struct Measurement {
    name: &'static str,
    observations: Vec<Observation>,
}

/// Runs the bounded baseline and returns a machine-readable JSON evidence
/// document. Fixture construction is intentionally outside each timed region;
/// the output records that scope explicitly.
pub fn baseline() -> Result<String, String> {
    let measurements = [
        measure(SCENARIO_NAMES[0], cold_check)?,
        measure(SCENARIO_NAMES[1], warm_check)?,
        measure(SCENARIO_NAMES[2], single_character_edit)?,
        measure(SCENARIO_NAMES[3], signature_edit)?,
        measure(SCENARIO_NAMES[4], cross_package_edit)?,
        measure(SCENARIO_NAMES[5], synthetic_10k_cold_parse)?,
        measure(SCENARIO_NAMES[6], synthetic_10k_warm_parse)?,
        measure(SCENARIO_NAMES[7], synthetic_10k_single_edit)?,
    ];
    Ok(render(&measurements))
}

fn measure(
    name: &'static str,
    mut operation: impl FnMut() -> Result<Observation, String>,
) -> Result<Measurement, String> {
    let mut observations = Vec::with_capacity(SAMPLE_COUNT);
    for _ in 0..SAMPLE_COUNT {
        observations.push(operation()?);
    }
    Ok(Measurement { name, observations })
}

fn cold_check() -> Result<Observation, String> {
    let (mut database, main) = checked_fixture()?;
    let started = Instant::now();
    let completed_items = check(&mut database, main)?;
    Ok(observe(started, &database, completed_items))
}

fn warm_check() -> Result<Observation, String> {
    let (mut database, main) = checked_fixture()?;
    let _ = check(&mut database, main)?;
    database.clear_trace();
    let started = Instant::now();
    let completed_items = check(&mut database, main)?;
    Ok(observe(started, &database, completed_items))
}

fn single_character_edit() -> Result<Observation, String> {
    let (mut database, main) = checked_fixture()?;
    let _ = check(&mut database, main)?;
    database.clear_trace();
    set_snapshot(
        &mut database,
        "src/Lib.ling",
        b"module Lib\n\nlet answer: Int = 43\n",
    )?;
    let started = Instant::now();
    let completed_items = check(&mut database, main)?;
    Ok(observe(started, &database, completed_items))
}

fn signature_edit() -> Result<Observation, String> {
    let (mut database, main) = checked_fixture()?;
    let _ = check(&mut database, main)?;
    database.clear_trace();
    set_snapshot(
        &mut database,
        "src/Lib.ling",
        b"module Lib\n\nlet answer: Text = \"answer\"\n",
    )?;
    let started = Instant::now();
    let completed_items = check(&mut database, main)?;
    Ok(observe(started, &database, completed_items))
}

fn cross_package_edit() -> Result<Observation, String> {
    let (mut database, main) = checked_fixture()?;
    set_workspace_input(
        &mut database,
        WorkspaceInput::PackageManifest,
        b"package-v1",
    )?;
    let _ = check(&mut database, main)?;
    database.clear_trace();
    set_workspace_input(
        &mut database,
        WorkspaceInput::PackageManifest,
        b"package-v2",
    )?;
    let started = Instant::now();
    let completed_items = check(&mut database, main)?;
    Ok(observe(started, &database, completed_items))
}

fn synthetic_10k_cold_parse() -> Result<Observation, String> {
    let mut database = synthetic_database()?;
    let started = Instant::now();
    let parsed = database
        .parse_all()
        .map_err(|_| "synthetic cold parse failed".to_owned())?;
    let completed_items = black_box(parsed.len());
    Ok(observe(started, &database, completed_items))
}

fn synthetic_10k_warm_parse() -> Result<Observation, String> {
    let mut database = synthetic_database()?;
    let _ = database
        .parse_all()
        .map_err(|_| "synthetic warm setup parse failed".to_owned())?;
    database.clear_trace();
    let started = Instant::now();
    let parsed = database
        .parse_all()
        .map_err(|_| "synthetic warm parse failed".to_owned())?;
    let completed_items = black_box(parsed.len());
    Ok(observe(started, &database, completed_items))
}

fn synthetic_10k_single_edit() -> Result<Observation, String> {
    let mut database = synthetic_database()?;
    let _ = database
        .parse_all()
        .map_err(|_| "synthetic edit setup parse failed".to_owned())?;
    database.clear_trace();
    set_snapshot(
        &mut database,
        "synthetic/M05000.ling",
        b"module M05000\n\nlet value = 50001\n",
    )?;
    let started = Instant::now();
    let parsed = database
        .parse_all()
        .map_err(|_| "synthetic edit parse failed".to_owned())?;
    let completed_items = black_box(parsed.len());
    Ok(observe(started, &database, completed_items))
}

fn checked_fixture() -> Result<(CompilerDb, SourceId), String> {
    let mut database = CompilerDb::new();
    let main = set_snapshot(
        &mut database,
        "src/Main.ling",
        b"module Main\n\nimport Lib\n\nlet main () = Lib.answer\n",
    )?;
    set_snapshot(
        &mut database,
        "src/Lib.ling",
        b"module Lib\n\nlet answer: Int = 42\n",
    )?;
    Ok((database, main))
}

fn synthetic_database() -> Result<CompilerDb, String> {
    let mut database = CompilerDb::new();
    for index in 0..SYNTHETIC_FILE_COUNT {
        let module = format!("M{index:05}");
        let logical_name = format!("synthetic/{module}.ling");
        let source = format!("module {module}\n\nlet value = {index}\n");
        set_snapshot(&mut database, &logical_name, source.as_bytes())?;
    }
    Ok(database)
}

fn check(database: &mut CompilerDb, main: SourceId) -> Result<usize, String> {
    let snapshot = database
        .semantic_snapshot(main)
        .map_err(|_| "checked query failed".to_owned())?;
    Ok(black_box(snapshot.json().len()))
}

fn set_snapshot(
    database: &mut CompilerDb,
    logical_name: &str,
    bytes: &[u8],
) -> Result<SourceId, String> {
    database
        .set_disk_snapshot(logical_name, bytes.to_vec())
        .map(change_file)
        .map_err(|_| format!("cannot set snapshot {logical_name}"))
}

fn set_workspace_input(
    database: &mut CompilerDb,
    kind: WorkspaceInput,
    bytes: &[u8],
) -> Result<(), String> {
    database
        .set_workspace_input(kind, bytes.to_vec())
        .map(|_| ())
        .map_err(|_| "cannot set workspace input".to_owned())
}

fn change_file(event: ChangeEvent) -> SourceId {
    match event {
        ChangeEvent::Added { file, .. }
        | ChangeEvent::Changed { file, .. }
        | ChangeEvent::Unchanged { file, .. } => file,
    }
}

fn observe(started: Instant, database: &CompilerDb, completed_items: usize) -> Observation {
    let elapsed_ns = started.elapsed().as_nanos();
    let (misses, hits) = database
        .trace()
        .iter()
        .fold((0, 0), |(misses, hits), event| match event.outcome() {
            QueryOutcome::Miss => (misses + 1, hits),
            QueryOutcome::Hit => (misses, hits + 1),
        });
    Observation {
        elapsed_ns,
        trace_events: database.trace().len(),
        misses,
        hits,
        completed_items,
    }
}

fn render(measurements: &[Measurement]) -> String {
    let mut output = String::new();
    output.push_str("{\n");
    output.push_str("  \"schema\": \"ling.performance-baseline/1\",\n");
    output.push_str("  \"sample_count\": ");
    let _ = writeln!(output, "{SAMPLE_COUNT},");
    output.push_str("  \"synthetic_file_count\": ");
    let _ = writeln!(output, "{SYNTHETIC_FILE_COUNT},");
    output.push_str("  \"timed_region_excludes_fixture_setup\": true,\n");
    output.push_str("  \"scenarios\": [\n");
    for (index, measurement) in measurements.iter().enumerate() {
        let _ = writeln!(output, "    {{\"name\": \"{}\",", measurement.name);
        write_array(
            &mut output,
            "samples_ns",
            measurement,
            |observation| observation.elapsed_ns,
            true,
        );
        write_array(
            &mut output,
            "trace_events",
            measurement,
            |observation| observation.trace_events as u128,
            true,
        );
        write_array(
            &mut output,
            "misses",
            measurement,
            |observation| observation.misses as u128,
            true,
        );
        write_array(
            &mut output,
            "hits",
            measurement,
            |observation| observation.hits as u128,
            true,
        );
        write_array(
            &mut output,
            "completed_items",
            measurement,
            |observation| observation.completed_items as u128,
            false,
        );
        output.push_str(if index + 1 == measurements.len() {
            "    }\n"
        } else {
            "    },\n"
        });
    }
    output.push_str("  ]\n}\n");
    output
}

fn write_array(
    output: &mut String,
    name: &str,
    measurement: &Measurement,
    value: impl Fn(Observation) -> u128,
    trailing_comma: bool,
) {
    let _ = write!(output, "      \"{name}\": [");
    for (index, observation) in measurement.observations.iter().copied().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        let _ = write!(output, "{}", value(observation));
    }
    output.push_str(if trailing_comma { "],\n" } else { "]\n" });
}
