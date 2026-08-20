use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ling_diagnostics::Diagnostic;
use ling_project::{
    ImportTarget, LOCK_FILE_NAME, LockMode, PackageGraph, PackageIdentity, parse_manifest,
    resolve_package_graph_with_lock,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const FIXTURE_ROOT: &str = "../../tests/projects";
const CASES: [&str; 7] = [
    "single-package",
    "multi-module",
    "path-dependency",
    "cycle",
    "visibility",
    "offline-lock",
    "unicode-names",
];
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ExpectedOutcome {
    Success,
    Failure,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
enum FixtureMode {
    Update,
    Locked,
}

impl From<FixtureMode> for LockMode {
    fn from(mode: FixtureMode) -> Self {
        match mode {
            FixtureMode::Update => Self::Update,
            FixtureMode::Locked => Self::Locked,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Expectation {
    outcome: ExpectedOutcome,
    mode: FixtureMode,
    diagnostics: String,
    graph: String,
    lock: String,
}

#[derive(Serialize)]
struct GraphSnapshot {
    graph_id: String,
    root: IdentitySnapshot,
    packages: Vec<PackageSnapshot>,
    edges: Vec<DependencyEdgeSnapshot>,
}

#[derive(Clone, Serialize)]
struct IdentitySnapshot {
    content: String,
    name: String,
    version: String,
}

#[derive(Serialize)]
struct PackageSnapshot {
    identity: IdentitySnapshot,
    entry: String,
    exports: Vec<String>,
    modules: ModuleGraphSnapshot,
    sources: Vec<SourceSnapshot>,
}

#[derive(Serialize)]
struct ModuleGraphSnapshot {
    dependencies: Vec<String>,
    nodes: Vec<ModuleNodeSnapshot>,
    edges: Vec<ModuleEdgeSnapshot>,
}

#[derive(Serialize)]
struct ModuleNodeSnapshot {
    name: String,
    source_root: String,
    relative_path: String,
    logical_path: String,
    declaration_span: Option<SpanSnapshot>,
}

#[derive(Serialize)]
struct SourceSnapshot {
    module: String,
    source_root: String,
    relative_path: String,
    logical_path: String,
    byte_len: usize,
}

#[derive(Serialize)]
struct ModuleEdgeSnapshot {
    from: String,
    target: ImportTargetSnapshot,
    source: String,
    span: SpanSnapshot,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ImportTargetSnapshot {
    Local { module: String },
    Dependency { package: String, module: String },
}

#[derive(Serialize)]
struct DependencyEdgeSnapshot {
    from: IdentitySnapshot,
    dependency: String,
    to: IdentitySnapshot,
}

#[derive(Clone, Serialize)]
struct SpanSnapshot {
    start_byte: u32,
    end_byte: u32,
}

#[test]
fn named_project_fixtures_match_expected_diagnostics_graph_and_lock() {
    for case in CASES {
        run_fixture(case, false);
    }
}

#[test]
fn path_dependency_lock_matches_the_independent_schema_golden() {
    let fixture_lock = fs::read(fixture_root().join("path-dependency/expected.ling.lock"))
        .expect("path-dependency expected lock is readable");
    let schema_golden = include_bytes!("../../../schemas/lock/1/canonical/basic.bin").as_slice();
    assert_eq!(fixture_lock, schema_golden);
}

#[test]
#[ignore = "rewrites checked-in PRJ-1106 expected artifacts"]
fn bless_named_project_fixture_expectations() {
    for case in CASES {
        run_fixture(case, true);
    }
}

fn run_fixture(case: &str, bless: bool) {
    let fixture = fixture_root().join(case);
    assert!(
        fixture.is_dir(),
        "missing PRJ-1106 fixture directory: {case}"
    );
    let expectation = read_expectation(&fixture);
    validate_expectation(case, &expectation);

    let project = TempProject::copy_fixture(case, &fixture);
    if bless {
        let temporary_lock = project.root.join(LOCK_FILE_NAME);
        if temporary_lock.exists() {
            fs::remove_file(&temporary_lock).expect("temporary copied lock is removable");
        }
    }
    let manifest_bytes = fs::read(project.root.join(ling_project::MANIFEST_FILE_NAME))
        .expect("fixture manifest is readable");
    let source_name = format!("tests/projects/{case}/ling.toml");
    let manifest = parse_manifest(&source_name, &manifest_bytes)
        .unwrap_or_else(|error| panic!("{case} manifest must be valid: {error}"));
    let mode = if bless {
        LockMode::Update
    } else {
        expectation.mode.into()
    };

    match resolve_package_graph_with_lock(&project.root, &manifest, mode) {
        Ok(graph) => {
            assert_eq!(
                expectation.outcome,
                ExpectedOutcome::Success,
                "{case} unexpectedly resolved"
            );
            verify_or_bless(
                bless,
                &fixture.join(&expectation.diagnostics),
                &diagnostic_bytes(&[]),
            );
            verify_or_bless(
                bless,
                &fixture.join(&expectation.graph),
                &graph_snapshot_bytes(&graph),
            );
            let lock = fs::read(project.root.join(LOCK_FILE_NAME))
                .expect("successful fixture emits or retains a lock");
            verify_or_bless(bless, &fixture.join(&expectation.lock), &lock);
        }
        Err(failure) => {
            assert_eq!(
                expectation.outcome,
                ExpectedOutcome::Failure,
                "{case} unexpectedly failed: {failure}"
            );
            let diagnostics = failure
                .diagnostics()
                .unwrap_or_else(|| panic!("{case} failed without public diagnostics"));
            verify_or_bless(
                bless,
                &fixture.join(&expectation.diagnostics),
                &diagnostic_bytes(diagnostics),
            );
            assert_eq!(expectation.graph, "absent", "{case} graph expectation");
            assert_eq!(expectation.lock, "absent", "{case} lock expectation");
            assert!(
                !project.root.join(LOCK_FILE_NAME).exists(),
                "{case} published a lock after failure"
            );
        }
    }
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_ROOT)
}

fn read_expectation(fixture: &Path) -> Expectation {
    let text =
        fs::read_to_string(fixture.join("expect.toml")).expect("fixture expectation is readable");
    toml::from_str(&text).expect("fixture expectation is valid TOML")
}

fn validate_expectation(case: &str, expectation: &Expectation) {
    assert_eq!(
        expectation.diagnostics, "expected-diagnostics.json",
        "{case} diagnostic artifact name"
    );
    match expectation.outcome {
        ExpectedOutcome::Success => {
            assert_eq!(expectation.graph, "expected-graph.json", "{case} graph");
            assert!(
                matches!(
                    expectation.lock.as_str(),
                    "expected.ling.lock" | "ling.lock"
                ),
                "{case} lock artifact name"
            );
        }
        ExpectedOutcome::Failure => {
            assert_eq!(expectation.mode, FixtureMode::Update, "{case} failure mode");
            assert_eq!(expectation.graph, "absent", "{case} graph state");
            assert_eq!(expectation.lock, "absent", "{case} lock state");
        }
    }
}

fn verify_or_bless(bless: bool, path: &Path, actual: &[u8]) {
    if bless {
        fs::write(path, actual).expect("expected fixture artifact is writable");
    } else {
        let expected = fs::read(path).expect("expected fixture artifact is readable");
        assert_eq!(
            actual,
            expected,
            "fixture artifact drift: {}",
            path.display()
        );
    }
}

fn diagnostic_bytes(diagnostics: &[Diagnostic]) -> Vec<u8> {
    let values = diagnostics
        .iter()
        .map(|diagnostic| {
            serde_json::from_str::<Value>(
                &diagnostic.render_json().expect("diagnostic JSON renders"),
            )
            .expect("rendered diagnostic JSON parses")
        })
        .collect::<Vec<_>>();
    json_bytes(&values)
}

fn graph_snapshot_bytes(graph: &PackageGraph) -> Vec<u8> {
    let snapshot = GraphSnapshot {
        graph_id: graph.id().as_str().to_owned(),
        root: identity_snapshot(graph.root()),
        packages: graph
            .packages()
            .iter()
            .map(|package| {
                let modules = package.modules();
                PackageSnapshot {
                    identity: identity_snapshot(package.identity()),
                    entry: package.entry().as_str().to_owned(),
                    exports: package
                        .exports()
                        .iter()
                        .map(|module| module.as_str().to_owned())
                        .collect(),
                    modules: ModuleGraphSnapshot {
                        dependencies: modules
                            .dependencies()
                            .iter()
                            .map(|dependency| dependency.as_str().to_owned())
                            .collect(),
                        nodes: modules
                            .nodes()
                            .iter()
                            .map(|node| ModuleNodeSnapshot {
                                name: node.name().as_str().to_owned(),
                                source_root: node.source_root().as_str().to_owned(),
                                relative_path: node.relative_path().as_str().to_owned(),
                                logical_path: node.logical_path().as_str().to_owned(),
                                declaration_span: node.declaration_span().map(span_snapshot),
                            })
                            .collect(),
                        edges: modules
                            .edges()
                            .iter()
                            .map(|edge| ModuleEdgeSnapshot {
                                from: edge.from().as_str().to_owned(),
                                target: match edge.target() {
                                    ImportTarget::Local(module) => ImportTargetSnapshot::Local {
                                        module: module.as_str().to_owned(),
                                    },
                                    ImportTarget::Dependency { package, module } => {
                                        ImportTargetSnapshot::Dependency {
                                            package: package.as_str().to_owned(),
                                            module: module.as_str().to_owned(),
                                        }
                                    }
                                },
                                source: edge.source().as_str().to_owned(),
                                span: span_snapshot(edge.span()),
                            })
                            .collect(),
                    },
                    sources: package
                        .sources()
                        .iter()
                        .map(|source| SourceSnapshot {
                            module: source.module().as_str().to_owned(),
                            source_root: source.source_root().as_str().to_owned(),
                            relative_path: source.relative_path().as_str().to_owned(),
                            logical_path: source.logical_path().as_str().to_owned(),
                            byte_len: source.bytes().len(),
                        })
                        .collect(),
                }
            })
            .collect(),
        edges: graph
            .edges()
            .iter()
            .map(|edge| DependencyEdgeSnapshot {
                from: identity_snapshot(edge.from()),
                dependency: edge.dependency().as_str().to_owned(),
                to: identity_snapshot(edge.to()),
            })
            .collect(),
    };
    json_bytes(&snapshot)
}

fn identity_snapshot(identity: &PackageIdentity) -> IdentitySnapshot {
    IdentitySnapshot {
        content: identity.source().as_str().to_owned(),
        name: identity.name().as_str().to_owned(),
        version: identity.version().to_string(),
    }
}

fn span_snapshot(span: std::ops::Range<u32>) -> SpanSnapshot {
    SpanSnapshot {
        start_byte: span.start,
        end_byte: span.end,
    }
}

fn json_bytes(value: &impl Serialize) -> Vec<u8> {
    let mut bytes = serde_json::to_vec_pretty(value).expect("fixture snapshot is serializable");
    bytes.push(b'\n');
    bytes
}

struct TempProject {
    root: PathBuf,
}

impl TempProject {
    fn copy_fixture(case: &str, fixture: &Path) -> Self {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ling-prj-1106-{case}-{}-{sequence}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale temporary fixture is removable");
        }
        copy_tree(fixture, &root);
        Self { root }
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        if self.root.exists() {
            fs::remove_dir_all(&self.root).expect("temporary fixture is removable");
        }
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("temporary fixture directory is creatable");
    let mut entries = fs::read_dir(source)
        .expect("fixture directory is readable")
        .collect::<Result<Vec<_>, _>>()
        .expect("fixture entries are readable");
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let target = destination.join(entry.file_name());
        if entry
            .file_type()
            .expect("fixture type is readable")
            .is_dir()
        {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("fixture file is copyable");
        }
    }
}
