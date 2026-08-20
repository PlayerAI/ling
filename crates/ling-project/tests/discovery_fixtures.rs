use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ling_project::{ImportTarget, ModuleGraph, discover_modules, parse_manifest};

const FIXTURE_ROOT: &str = "../../tests/projects/discovery-v1";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(FIXTURE_ROOT)
        .join(name)
}

fn discover_fixture(name: &str) -> Result<ModuleGraph, ling_project::DiscoveryFailure> {
    let root = fixture(name);
    let manifest_path = root.join(ling_project::MANIFEST_FILE_NAME);
    let bytes = fs::read(&manifest_path).expect("fixture manifest is readable");
    let manifest = parse_manifest(&manifest_path.to_string_lossy(), &bytes)
        .expect("fixture manifest itself is valid");
    discover_modules(&root, &manifest)
}

fn error_codes(name: &str) -> Vec<String> {
    discover_fixture(name)
        .expect_err("fixture must fail discovery")
        .diagnostics()
        .expect("fixture failure must be public diagnostics")
        .iter()
        .map(|diagnostic| diagnostic.code().to_string())
        .collect()
}

#[test]
fn valid_multi_root_graph_has_sorted_nodes_edges_and_logical_paths() {
    let graph = discover_fixture("valid-multi-root").expect("fixture must discover");

    assert_eq!(graph.package().as_str(), "hello");
    assert_eq!(graph.entry().as_str(), "Main");
    assert_eq!(
        graph
            .nodes()
            .iter()
            .map(|node| node.name().as_str())
            .collect::<Vec<_>>(),
        ["Game.Math", "Main", "Util"]
    );
    assert_eq!(
        graph
            .nodes()
            .iter()
            .map(|node| (
                node.name().as_str(),
                node.source_root().as_str(),
                node.relative_path().as_str(),
                node.logical_path().as_str(),
            ))
            .collect::<Vec<_>>(),
        [
            ("Game.Math", "src", "Game/Math.ling", "src/Game/Math.ling"),
            ("Main", "src", "Main.ling", "src/Main.ling"),
            ("Util", "generated", "Util.ling", "generated/Util.ling"),
        ]
    );
    assert_eq!(
        graph
            .edges()
            .iter()
            .map(|edge| {
                let target = match edge.target() {
                    ImportTarget::Local(module) => format!("local:{}", module.as_str()),
                    ImportTarget::Dependency { package, module } => {
                        format!("dependency:{}:{}", package.as_str(), module.as_str())
                    }
                };
                (edge.from().as_str(), target)
            })
            .collect::<Vec<_>>(),
        [
            ("Game.Math", "local:Util".to_owned()),
            ("Main", "local:Game.Math".to_owned()),
            ("Main", "local:Util".to_owned()),
        ]
    );
    assert!(graph.node(graph.entry()).is_some());
    assert!(graph.node_by_name("Missing").is_none());
}

#[test]
fn dependency_imports_are_namespaced_without_resolving_dependency_contents() {
    let graph = discover_fixture("valid-dependency-edge").expect("fixture must discover");
    let edge = graph.edges().first().expect("one dependency edge");
    assert_eq!(edge.from().as_str(), "Main");
    assert_eq!(
        edge.target(),
        &ImportTarget::Dependency {
            package: graph
                .dependency_name("math")
                .expect("declared dependency is retained")
                .clone(),
            module: edge.dependency_module().expect("dependency module").clone(),
        }
    );
    assert_eq!(edge.dependency_module().unwrap().as_str(), "Algebra");
}

#[test]
fn invalid_project_graph_fixtures_use_distinct_registered_root_causes() {
    for (name, expected) in [
        ("source-root-case", "L-PROJECT-0008"),
        ("invalid-source-path", "L-PROJECT-0009"),
        ("extension-case-mismatch", "L-PROJECT-0009"),
        ("module-mismatch", "L-PROJECT-0010"),
        ("missing-declaration", "L-PROJECT-0010"),
        ("implicit-non-main", "L-PROJECT-0010"),
        ("duplicate-module", "L-PROJECT-0011"),
        ("missing-entry", "L-PROJECT-0012"),
        ("missing-export", "L-PROJECT-0012"),
        ("missing-import", "L-PROJECT-0012"),
        ("import-cycle", "L-PROJECT-0013"),
        ("import-cycle-three", "L-PROJECT-0013"),
        ("dependency-namespace-collision", "L-PROJECT-0013"),
    ] {
        assert_eq!(error_codes(name), [expected], "{name}");
    }
}

#[test]
fn declaration_mismatch_span_uses_original_utf8_bytes() {
    let failure = discover_fixture("module-mismatch").expect_err("fixture must fail");
    let diagnostic = failure
        .diagnostics()
        .unwrap()
        .first()
        .expect("one diagnostic");
    let span = diagnostic.primary_span().expect("source-local span");
    let source = fs::read(fixture("module-mismatch").join("src/Game/Math.ling")).unwrap();
    let expected_start = source
        .windows("module".len())
        .position(|window| window == b"module")
        .expect("module token");

    assert_eq!(span.start_byte(), u32::try_from(expected_start).unwrap());
    assert_eq!(
        span.end_byte(),
        u32::try_from(expected_start + "module Game.math".len()).unwrap()
    );
    assert_eq!(
        &source[span.start_byte() as usize..span.end_byte() as usize],
        b"module Game.math"
    );
}

#[test]
fn manifest_origin_is_not_part_of_semantic_equality_or_debug_output() {
    let contents = manifest(&["src"], "Main", &[], &[]);
    let first = parse_manifest("C:/private/first/ling.toml", contents.as_bytes()).unwrap();
    let second = parse_manifest("D:/different/second/ling.toml", contents.as_bytes()).unwrap();

    assert_eq!(first, second);
    assert_eq!(format!("{first:?}"), format!("{second:?}"));
    assert!(!format!("{first:?}").contains("private"));
}

#[test]
fn malformed_source_is_reported_without_publishing_a_partial_graph() {
    let project = TempProject::new("invalid-utf8");
    project.write_manifest(&manifest(&["src"], "Main", &[], &[]));
    project.write_bytes("src/Main.ling", b"module Main\n\nlet value = \xff\n");

    let failure = project.discover().expect_err("invalid UTF-8 must fail");
    let diagnostics = failure.diagnostics().expect("public source diagnostic");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code().as_str(), "L-LEX-0001");
    assert_eq!(diagnostics[0].primary_span().unwrap().start_byte(), 25);
}

#[test]
fn duplicate_misplaced_and_late_import_declarations_are_rejected() {
    for (label, source, expected) in [
        (
            "duplicate-declaration",
            "module Main\nmodule Main\n\nlet main () = ()\n",
            "L-PROJECT-0010",
        ),
        (
            "misplaced-declaration",
            "let value = 1\nmodule Main\n",
            "L-PROJECT-0010",
        ),
        (
            "late-import",
            "module Main\n\nlet value = 1\nimport Main\n",
            "L-PROJECT-0013",
        ),
    ] {
        let project = TempProject::new(label);
        project.write_manifest(&manifest(&["src"], "Main", &[], &[]));
        project.write("src/Main.ling", source);
        let failure = project.discover().expect_err("invalid declaration order");
        assert_eq!(
            failure.diagnostics().unwrap()[0].code().as_str(),
            expected,
            "{label}"
        );
    }
}

#[test]
fn unicode_paths_crlf_and_alias_imports_preserve_determinism() {
    let project = TempProject::new("unicode-crlf");
    project.write_manifest(&manifest(&["源"], "主", &["工具.数学"], &[]));
    project.write_bytes(
        "源/主.ling",
        "// 😀\r\nmodule 主\r\nimport 工具.数学 as 数学\r\n\r\nlet main () = ()\r\n".as_bytes(),
    );
    project.write_bytes(
        "源/工具/数学.ling",
        "module 工具.数学\r\n\r\nlet 答案 = 42\r\n".as_bytes(),
    );

    let graph = project.discover().expect("Unicode project must discover");
    assert_eq!(
        graph
            .nodes()
            .iter()
            .map(|node| node.name().as_str())
            .collect::<Vec<_>>(),
        ["主", "工具.数学"]
    );
    assert_eq!(graph.edges().len(), 1);
    assert_eq!(graph.edges()[0].from().as_str(), "主");
    assert_eq!(
        graph.edges()[0].target(),
        &ImportTarget::Local(
            graph
                .node_by_name("工具.数学")
                .expect("imported module")
                .name()
                .clone()
        )
    );
}

#[test]
fn user_controlled_import_facts_remain_bounded() {
    let project = TempProject::new("bounded-import");
    project.write_manifest(&manifest(&["src"], "Main", &[], &[]));
    let target = "A".repeat(8_192);
    project.write(
        "src/Main.ling",
        &format!("module Main\n\nimport {target}\n\nlet main () = ()\n"),
    );

    let failure = project
        .discover()
        .expect_err("missing long import must fail");
    let rendered = failure.diagnostics().unwrap()[0].render_json().unwrap();
    assert!(
        rendered.len() < 2_048,
        "diagnostic was {} bytes",
        rendered.len()
    );
}

#[test]
fn creation_order_and_physical_root_do_not_change_the_graph() {
    let first = TempProject::new("order-a");
    let second = TempProject::new("order-b");
    let manifest = manifest(&["src", "generated"], "Main", &["Game.Math"], &[]);
    first.write_manifest(&manifest);
    second.write_manifest(&manifest);

    for (project, paths) in [
        (
            &first,
            [
                ("src/Main.ling", "import Game.Math\n\nlet main () = ()\n"),
                ("src/Game/Math.ling", "module Game.Math\n\nlet value = 1\n"),
                ("generated/Util.ling", "module Util\n\nlet value = 2\n"),
            ],
        ),
        (
            &second,
            [
                ("generated/Util.ling", "module Util\n\nlet value = 2\n"),
                ("src/Game/Math.ling", "module Game.Math\n\nlet value = 1\n"),
                ("src/Main.ling", "import Game.Math\n\nlet main () = ()\n"),
            ],
        ),
    ] {
        for (path, source) in paths {
            project.write(path, source);
        }
    }

    assert_eq!(first.discover().unwrap(), second.discover().unwrap());
}

#[test]
fn source_root_symlink_escape_is_rejected_when_symlinks_are_available() {
    let project = TempProject::new("symlink-project");
    let outside = TempProject::new("symlink-outside");
    outside.write("Main.ling", "module Main\n\nlet main () = ()\n");
    project.write_manifest(&manifest(&["src"], "Main", &[], &[]));

    let link = project.root.join("src");
    if let Err(error) = create_directory_symlink(&outside.root, &link) {
        if cfg!(windows) && error.kind() == std::io::ErrorKind::PermissionDenied {
            return;
        }
        panic!("cannot create test symlink: {error}");
    }

    let failure = project.discover().expect_err("escaping root must fail");
    assert_eq!(
        failure.diagnostics().unwrap()[0].code().as_str(),
        "L-PROJECT-0008"
    );
}

#[test]
fn directory_symlink_cycle_is_rejected_when_symlinks_are_available() {
    let project = TempProject::new("symlink-cycle");
    project.write_manifest(&manifest(&["src"], "Main", &[], &[]));
    project.write("src/Main.ling", "module Main\n\nlet main () = ()\n");
    let link = project.root.join("src/loop");
    if let Err(error) = create_directory_symlink(&project.root.join("src"), &link) {
        if cfg!(windows) && error.kind() == std::io::ErrorKind::PermissionDenied {
            return;
        }
        panic!("cannot create test symlink: {error}");
    }

    let failure = project.discover().expect_err("symlink cycle must fail");
    assert_eq!(
        failure.diagnostics().unwrap()[0].code().as_str(),
        "L-PROJECT-0009"
    );
}

#[test]
fn directory_symlink_alias_is_rejected_when_symlinks_are_available() {
    let project = TempProject::new("symlink-alias");
    project.write_manifest(&manifest(&["src"], "Main", &[], &[]));
    project.write("src/Main.ling", "module Main\n\nlet main () = ()\n");
    project.write(
        "src/shared/Helper.ling",
        "module shared.Helper\n\nlet value = 1\n",
    );
    let link = project.root.join("src/alias");
    if let Err(error) = create_directory_symlink(&project.root.join("src/shared"), &link) {
        if cfg!(windows) && error.kind() == std::io::ErrorKind::PermissionDenied {
            return;
        }
        panic!("cannot create test symlink: {error}");
    }

    let failure = project.discover().expect_err("symlink alias must fail");
    assert_eq!(
        failure.diagnostics().unwrap()[0].code().as_str(),
        "L-PROJECT-0009"
    );
}

fn manifest(
    roots: &[&str],
    entry: &str,
    exports: &[&str],
    dependencies: &[(&str, &str)],
) -> String {
    let roots = roots
        .iter()
        .map(|root| format!("\"{root}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let exports = exports
        .iter()
        .map(|module| format!("\"{module}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let dependencies = dependencies
        .iter()
        .map(|(name, path)| format!("{name} = {{ path = \"{path}\" }}\n"))
        .collect::<String>();
    format!(
        "manifest-version = 1\n\n[package]\nname = \"hello\"\nversion = \"0.1.0\"\nlanguage = \"0.1\"\n\n[source]\nroots = [{roots}]\nentry = \"{entry}\"\n\n[exports]\nmodules = [{exports}]\n\n[dependencies]\n{dependencies}"
    )
}

struct TempProject {
    root: PathBuf,
}

impl TempProject {
    fn new(label: &str) -> Self {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ling-prj-1102-{label}-{}-{sequence}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale test directory is removable");
        }
        fs::create_dir_all(&root).expect("temporary project root is creatable");
        Self { root }
    }

    fn write_manifest(&self, contents: &str) {
        self.write(ling_project::MANIFEST_FILE_NAME, contents);
    }

    fn write(&self, relative: &str, contents: &str) {
        self.write_bytes(relative, contents.as_bytes());
    }

    fn write_bytes(&self, relative: &str, contents: &[u8]) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("fixture parent is creatable");
        }
        fs::write(path, contents).expect("fixture file is writable");
    }

    fn discover(&self) -> Result<ModuleGraph, ling_project::DiscoveryFailure> {
        let manifest_path = self.root.join(ling_project::MANIFEST_FILE_NAME);
        let bytes = fs::read(&manifest_path).expect("temporary manifest is readable");
        let manifest = parse_manifest(&manifest_path.to_string_lossy(), &bytes).unwrap();
        discover_modules(&self.root, &manifest)
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
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
