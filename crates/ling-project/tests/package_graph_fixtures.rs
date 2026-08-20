use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ling_project::{PackageGraph, parse_manifest, resolve_package_graph};

const FIXTURE_ROOT: &str = "../../tests/projects/dependency-v1";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(FIXTURE_ROOT)
        .join(name)
}

fn resolve_root(root: &Path) -> Result<PackageGraph, ling_project::DependencyGraphFailure> {
    let manifest_path = root.join(ling_project::MANIFEST_FILE_NAME);
    let bytes = fs::read(&manifest_path).expect("root manifest is readable");
    let manifest =
        parse_manifest(&manifest_path.to_string_lossy(), &bytes).expect("root manifest is valid");
    resolve_package_graph(root, &manifest)
}

fn resolve_fixture(name: &str) -> Result<PackageGraph, ling_project::DependencyGraphFailure> {
    resolve_root(&fixture(name))
}

fn failure_reason(name: &str) -> (String, String, String) {
    let failure = resolve_fixture(name).expect_err("fixture must fail");
    let package = failure
        .package()
        .expect("public graph failure identifies its package")
        .as_str()
        .to_owned();
    let diagnostic = failure
        .diagnostics()
        .expect("fixture failure is diagnostic")
        .first()
        .expect("one root-cause diagnostic");
    let value: serde_json::Value =
        serde_json::from_str(&diagnostic.render_json().unwrap()).unwrap();
    (
        package,
        diagnostic.code().to_string(),
        value["facts"]["reason"].as_str().unwrap().to_owned(),
    )
}

#[test]
fn valid_basic_graph_has_frozen_content_and_graph_hashes() {
    let graph = resolve_fixture("valid-basic").expect("fixture must resolve");
    let identities = graph
        .packages()
        .iter()
        .map(|package| {
            (
                package.identity().name().as_str(),
                package.identity().source().as_str(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        (
            identities,
            graph.id().as_str(),
            graph.root().name().as_str(),
        ),
        (
            vec![
                (
                    "app",
                    "sha256:9784dc68f2c10713f5945024e5c6085e34b7735be86acc21e27d523e31a918f1",
                ),
                (
                    "math",
                    "sha256:76c6c29d652bbd86f607a472a6091c5df95c7656d68fcc8d5c14f23517b65ba3",
                ),
            ],
            "sha256:ac20007193def9b78cc55bc082dbc6cd27abb9ad42720091d38f540a9f3fb2e8",
            "app",
        )
    );
    assert_eq!(graph.edges().len(), 1);
    assert_eq!(graph.edges()[0].from().name().as_str(), "app");
    assert_eq!(graph.edges()[0].dependency().as_str(), "math");
    assert_eq!(graph.edges()[0].to().name().as_str(), "math");

    let math = graph.package_by_name("math").expect("math package exists");
    assert!(
        math.exports_module(
            math.modules()
                .node_by_name("Algebra")
                .expect("exported module exists")
                .name()
        )
    );
}

#[test]
fn transitive_packages_and_edges_are_canonically_sorted() {
    let graph = resolve_fixture("valid-transitive").expect("fixture must resolve");
    assert_eq!(
        graph
            .packages()
            .iter()
            .map(|package| package.identity().name().as_str())
            .collect::<Vec<_>>(),
        ["app", "leaf", "left"]
    );
    assert_eq!(
        graph
            .edges()
            .iter()
            .map(|edge| (
                edge.from().name().as_str(),
                edge.dependency().as_str(),
                edge.to().name().as_str(),
            ))
            .collect::<Vec<_>>(),
        [("app", "left", "left"), ("left", "leaf", "leaf")]
    );
}

#[test]
fn one_physical_package_snapshot_can_satisfy_direct_and_transitive_edges() {
    let project = TempProject::new("shared-physical-package");
    project.write(
        "ling.toml",
        &manifest(
            "app",
            "1.0.0",
            &[
                ("alpha", "vendor/alpha"),
                ("beta", "vendor/alpha/vendor/beta"),
            ],
        ),
    );
    project.write("src/Main.ling", "let main () = ()\n");
    project.write_package(
        "vendor/alpha",
        "alpha",
        "1.0.0",
        &[("beta", "vendor/beta")],
        "module Main\n\nlet value = 1\n",
    );
    project.write_package(
        "vendor/alpha/vendor/beta",
        "beta",
        "1.0.0",
        &[],
        "module Main\n\nlet value = 2\n",
    );

    let graph = project
        .resolve()
        .expect("shared physical package must resolve");
    assert_eq!(
        graph
            .packages()
            .iter()
            .map(|package| package.identity().name().as_str())
            .collect::<Vec<_>>(),
        ["alpha", "app", "beta"]
    );
    assert_eq!(
        graph
            .edges()
            .iter()
            .map(|edge| (
                edge.from().name().as_str(),
                edge.dependency().as_str(),
                edge.to().name().as_str(),
            ))
            .collect::<Vec<_>>(),
        [
            ("alpha", "beta", "beta"),
            ("app", "alpha", "alpha"),
            ("app", "beta", "beta"),
        ]
    );
}

#[test]
fn checked_failures_have_stable_package_and_root_cause() {
    for (fixture, package, reason) in [
        ("missing-manifest", "app", "dependency_manifest_missing"),
        ("manifest-case", "app", "dependency_manifest_case_mismatch"),
        ("path-case", "app", "dependency_path_case_mismatch"),
        ("name-mismatch", "app", "dependency_name_mismatch"),
        ("package-cycle", "beta", "package_cycle"),
    ] {
        assert_eq!(
            failure_reason(fixture),
            (
                package.to_owned(),
                "L-PROJECT-0014".to_owned(),
                reason.to_owned(),
            ),
            "{fixture}"
        );
    }
}

#[test]
fn physical_root_manifest_spelling_creation_order_and_locator_do_not_change_identity() {
    let first = TempProject::new("canonical-first");
    first.write_package(
        "vendor/math-a",
        "math",
        "1.0.0",
        &[],
        "module Main\n\nlet value = 1\n",
    );
    first.write(
        "ling.toml",
        &manifest("app", "1.0.0", &[("math", "vendor/math-a")]),
    );
    first.write("src/Main.ling", "let main () = ()\n");

    let second = TempProject::new("canonical-second");
    second.write("src/Main.ling", "let main () = ()\n");
    let reordered = "# Cosmetic text and CRLF are excluded from identity.\r\nmanifest-version = 1\r\n\r\n[source]\r\nentry = \"Main\"\r\nroots = [\"src\"]\r\n\r\n[package]\r\nlanguage = \"0.1\"\r\nversion = \"1.0.0\"\r\ndisplay-name = \"另一个应用\"\r\nname = \"app\"\r\n\r\n[dependencies]\r\nmath = { path = \"deps/math-copy\" }\r\n";
    second.write("ling.toml", reordered);
    second.write_package(
        "deps/math-copy",
        "math",
        "1.0.0",
        &[],
        "module Main\n\nlet value = 1\n",
    );

    let first_graph = first.resolve().unwrap();
    let second_graph = second.resolve().unwrap();
    assert_eq!(first_graph, second_graph);
    assert_eq!(first_graph.id(), second_graph.id());
    let debug = format!("{first_graph:?}");
    assert!(!debug.contains(&first.root.to_string_lossy().to_string()));
    assert!(!debug.contains(&second.root.to_string_lossy().to_string()));
}

#[test]
fn changing_one_source_byte_changes_content_and_graph_identity() {
    let first = TempProject::new("source-one");
    first.write_package("", "app", "1.0.0", &[], "module Main\n\nlet value = 1\n");
    let second = TempProject::new("source-two");
    second.write_package("", "app", "1.0.0", &[], "module Main\n\nlet value = 2\n");

    let first_graph = first.resolve().unwrap();
    let second_graph = second.resolve().unwrap();
    assert_ne!(first_graph.root().source(), second_graph.root().source());
    assert_ne!(first_graph.id(), second_graph.id());
}

#[test]
fn changing_dependency_content_changes_only_the_child_and_graph_identities() {
    let first = TempProject::new("dependency-source-one");
    first.write_package(
        "vendor/math",
        "math",
        "1.0.0",
        &[],
        "module Main\n\nlet value = 1\n",
    );
    first.write(
        "ling.toml",
        &manifest("app", "1.0.0", &[("math", "vendor/math")]),
    );
    first.write("src/Main.ling", "let main () = ()\n");

    let second = TempProject::new("dependency-source-two");
    second.write_package(
        "vendor/math",
        "math",
        "1.0.0",
        &[],
        "module Main\n\nlet value = 2\n",
    );
    second.write(
        "ling.toml",
        &manifest("app", "1.0.0", &[("math", "vendor/math")]),
    );
    second.write("src/Main.ling", "let main () = ()\n");

    let first_graph = first.resolve().unwrap();
    let second_graph = second.resolve().unwrap();
    assert_eq!(first_graph.root().source(), second_graph.root().source());
    assert_ne!(
        first_graph
            .package_by_name("math")
            .unwrap()
            .identity()
            .source(),
        second_graph
            .package_by_name("math")
            .unwrap()
            .identity()
            .source()
    );
    assert_ne!(first_graph.id(), second_graph.id());
}

#[test]
fn dependency_graph_failures_precede_ling_source_parsing() {
    let project = TempProject::new("graph-before-source");
    project.write(
        "ling.toml",
        &manifest(
            "app",
            "1.0.0",
            &[("alpha", "vendor/alpha"), ("zeta", "vendor/zeta")],
        ),
    );
    project.write("src/Main.ling", "let main ( =\n");
    project.write_package(
        "vendor/alpha",
        "alpha",
        "1.0.0",
        &[],
        "module Main\n\nlet value ( =\n",
    );

    let failure = project.resolve().expect_err("missing zeta must fail");
    let diagnostic = &failure.diagnostics().unwrap()[0];
    let value: serde_json::Value =
        serde_json::from_str(&diagnostic.render_json().unwrap()).unwrap();
    assert_eq!(failure.package().unwrap().as_str(), "app");
    assert_eq!(diagnostic.code().as_str(), "L-PROJECT-0014");
    assert_eq!(value["facts"]["reason"], "dependency_path_missing");
}

#[test]
fn dependency_manifest_diagnostics_keep_contextual_original_byte_spans() {
    let project = TempProject::new("dependency-manifest-span");
    project.write(
        "ling.toml",
        &manifest("app", "1.0.0", &[("math", "vendor/math")]),
    );
    project.write("src/Main.ling", "let main () = ()\n");
    let child_manifest = "# 😀\nmanifest-version = 2\n\n[package]\nname = \"math\"\nversion = \"1.0.0\"\nlanguage = \"0.1\"\n\n[source]\nroots = [\"src\"]\nentry = \"Main\"\n";
    project.write("vendor/math/ling.toml", child_manifest);

    let failure = project
        .resolve()
        .expect_err("unsupported child manifest must fail");
    let diagnostic = &failure.diagnostics().unwrap()[0];
    let span = diagnostic
        .primary_span()
        .expect("manifest error has a span");
    assert_eq!(failure.package().unwrap().as_str(), "math");
    assert_eq!(diagnostic.code().as_str(), "L-PROJECT-0003");
    assert_eq!(span.file(), "package:math/ling.toml");
    assert_eq!(
        &child_manifest.as_bytes()[span.start_byte() as usize..span.end_byte() as usize],
        b"2"
    );
}

#[test]
fn dependency_manifest_reads_enforce_the_accepted_byte_limit() {
    let project = TempProject::new("dependency-manifest-limit");
    project.write(
        "ling.toml",
        &manifest("app", "1.0.0", &[("math", "vendor/math")]),
    );
    project.write("src/Main.ling", "let main () = ()\n");
    project.write(
        "vendor/math/ling.toml",
        &" ".repeat(ling_project::MAX_MANIFEST_BYTES + 1),
    );

    let failure = project.resolve().expect_err("oversized manifest must fail");
    let diagnostic = &failure.diagnostics().unwrap()[0];
    let rendered: serde_json::Value =
        serde_json::from_str(&diagnostic.render_json().unwrap()).unwrap();
    assert_eq!(failure.package().unwrap().as_str(), "math");
    assert_eq!(diagnostic.code().as_str(), "L-PROJECT-0001");
    assert_eq!(rendered["facts"]["reason"], "manifest_too_large");
    assert_eq!(
        rendered["facts"]["maximum_byte_len"],
        ling_project::MAX_MANIFEST_BYTES
    );
    let span = diagnostic.primary_span().unwrap();
    assert_eq!(span.file(), "package:math/ling.toml");
    assert_eq!((span.start_byte(), span.end_byte()), (0, 0));
}

#[test]
fn resolver_has_no_network_or_process_execution_surface() {
    const SOURCE: &str = include_str!("../src/package_graph.rs");
    const MANIFEST: &str = include_str!("../Cargo.toml");

    for forbidden in [
        "std::net::",
        "std::process::",
        "Command::new",
        "TcpStream",
        "UdpSocket",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "forbidden resolver API: {forbidden}"
        );
    }
    for forbidden in ["reqwest", "ureq", "curl", "git2"] {
        assert!(
            !MANIFEST.contains(forbidden),
            "forbidden resolver dependency: {forbidden}"
        );
    }
}

#[test]
fn same_name_content_and_version_conflicts_are_distinct() {
    for (label, right_version, right_source, expected) in [
        (
            "content-collision",
            "1.0.0",
            "module Main\n\nlet value = 2\n",
            "package_name_collision",
        ),
        (
            "version-conflict",
            "2.0.0",
            "module Main\n\nlet value = 1\n",
            "package_version_conflict",
        ),
    ] {
        let project = colliding_project(label, right_version, right_source);
        let failure = project.resolve().expect_err("collision must fail");
        let diagnostic = &failure.diagnostics().unwrap()[0];
        let value: serde_json::Value =
            serde_json::from_str(&diagnostic.render_json().unwrap()).unwrap();
        assert_eq!(failure.package().unwrap().as_str(), "right");
        assert_eq!(diagnostic.code().as_str(), "L-PROJECT-0014");
        assert_eq!(value["facts"]["reason"], expected, "{label}");
    }
}

#[test]
fn read_only_dependency_sources_are_never_modified() {
    let project = TempProject::new("read-only");
    project.write_package(
        "vendor/math",
        "math",
        "1.0.0",
        &[],
        "module Main\n\nlet value = 1\n",
    );
    project.write(
        "ling.toml",
        &manifest("app", "1.0.0", &[("math", "vendor/math")]),
    );
    project.write("src/Main.ling", "let main () = ()\n");

    let source = project.root.join("vendor/math/src/Main.ling");
    let original_permissions = fs::metadata(&source).unwrap().permissions();
    let mut read_only = original_permissions.clone();
    read_only.set_readonly(true);
    fs::set_permissions(&source, read_only).unwrap();
    let result = project.resolve();
    fs::set_permissions(&source, original_permissions).unwrap();

    result.expect("read-only dependency source must resolve");
}

#[test]
fn dependency_symlink_escape_is_rejected_when_symlinks_are_available() {
    let project = TempProject::new("dependency-escape");
    project.write(
        "ling.toml",
        &manifest("app", "1.0.0", &[("math", "vendor/math")]),
    );
    project.write("src/Main.ling", "let main () = ()\n");
    fs::create_dir_all(project.root.join("vendor")).unwrap();

    let external = TempProject::new("dependency-external");
    external.write_package("", "math", "1.0.0", &[], "module Main\n\nlet value = 1\n");
    let link = project.root.join("vendor/math");
    if let Err(error) = create_directory_symlink(&external.root, &link) {
        if cfg!(windows) && error.kind() == std::io::ErrorKind::PermissionDenied {
            return;
        }
        panic!("cannot create dependency symlink: {error}");
    }

    let failure = project
        .resolve()
        .expect_err("escaping dependency must fail");
    let value: serde_json::Value =
        serde_json::from_str(&failure.diagnostics().unwrap()[0].render_json().unwrap()).unwrap();
    assert_eq!(value["facts"]["reason"], "dependency_path_escape");
}

fn colliding_project(label: &str, right_version: &str, right_source: &str) -> TempProject {
    let project = TempProject::new(label);
    project.write(
        "ling.toml",
        &manifest(
            "app",
            "1.0.0",
            &[("left", "vendor/left"), ("right", "vendor/right")],
        ),
    );
    project.write("src/Main.ling", "let main () = ()\n");
    project.write_package(
        "vendor/left",
        "left",
        "1.0.0",
        &[("shared", "vendor/shared")],
        "module Main\n\nlet value = 1\n",
    );
    project.write_package(
        "vendor/left/vendor/shared",
        "shared",
        "1.0.0",
        &[],
        "module Main\n\nlet value = 1\n",
    );
    project.write_package(
        "vendor/right",
        "right",
        "1.0.0",
        &[("shared", "vendor/shared")],
        "module Main\n\nlet value = 1\n",
    );
    project.write_package(
        "vendor/right/vendor/shared",
        "shared",
        right_version,
        &[],
        right_source,
    );
    project
}

fn manifest(name: &str, version: &str, dependencies: &[(&str, &str)]) -> String {
    let dependencies = dependencies
        .iter()
        .map(|(dependency, path)| format!("{dependency} = {{ path = \"{path}\" }}\n"))
        .collect::<String>();
    format!(
        "manifest-version = 1\n\n[package]\nname = \"{name}\"\nversion = \"{version}\"\nlanguage = \"0.1\"\n\n[source]\nroots = [\"src\"]\nentry = \"Main\"\n\n[dependencies]\n{dependencies}"
    )
}

struct TempProject {
    root: PathBuf,
}

impl TempProject {
    fn new(label: &str) -> Self {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ling-prj-1104-{label}-{}-{sequence}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale test directory is removable");
        }
        fs::create_dir_all(&root).expect("temporary project root is creatable");
        Self { root }
    }

    fn write_package(
        &self,
        relative: &str,
        name: &str,
        version: &str,
        dependencies: &[(&str, &str)],
        source: &str,
    ) {
        let manifest_path = join_relative(relative, ling_project::MANIFEST_FILE_NAME);
        let source_path = join_relative(relative, "src/Main.ling");
        self.write(&manifest_path, &manifest(name, version, dependencies));
        self.write(&source_path, source);
    }

    fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("fixture parent is creatable");
        }
        fs::write(path, contents).expect("fixture file is writable");
    }

    fn resolve(&self) -> Result<PackageGraph, ling_project::DependencyGraphFailure> {
        resolve_root(&self.root)
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        if self.root.exists() {
            fs::remove_dir_all(&self.root).expect("temporary project is removable");
        }
    }
}

fn join_relative(base: &str, child: &str) -> String {
    if base.is_empty() {
        child.to_owned()
    } else {
        format!("{base}/{child}")
    }
}

#[cfg(unix)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
