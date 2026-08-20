use std::fs;
use std::path::{Path, PathBuf};

use ling_effects::CheckedProgram;
use ling_project::{PackageGraph, parse_manifest, resolve_package_graph};
use ling_resolve::{PackagePrograms, ReferenceTarget, resolve_project};
use ling_semantic::{PROJECT_SEMANTIC_SCHEMA, build_project, read_project_json};
use ling_source::{SourceFile, SourceId};

const FIXTURE_ROOT: &str = "../../tests/projects/resolution-v1";

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(FIXTURE_ROOT)
        .join(name)
}

fn package_graph(name: &str) -> PackageGraph {
    let root = fixture(name);
    let manifest_path = root.join(ling_project::MANIFEST_FILE_NAME);
    let bytes = fs::read(&manifest_path).expect("root manifest is readable");
    let manifest =
        parse_manifest(&manifest_path.to_string_lossy(), &bytes).expect("root manifest is valid");
    resolve_package_graph(&root, &manifest).expect("package graph is valid")
}

fn checked_project(graph: &PackageGraph) -> CheckedProgram {
    let packages = graph
        .packages()
        .iter()
        .map(|package| {
            let programs = package
                .sources()
                .iter()
                .enumerate()
                .map(|(index, source)| {
                    let source_name = format!(
                        "package:{}/{}",
                        package.identity().name(),
                        source.logical_path()
                    );
                    let source_file = SourceFile::from_bytes(
                        SourceId::new(u32::try_from(index).unwrap_or(u32::MAX)),
                        source_name.clone(),
                        source.bytes().to_vec(),
                    )
                    .expect("package graph retained valid UTF-8");
                    let parsed = ling_syntax::parse(&source_file);
                    assert!(parsed.is_valid(), "package source has valid syntax");
                    let ast = ling_ast::lower(&source_file, &parsed)
                        .expect("validated syntax lowers to AST");
                    ling_hir::lower(source_name, &ast)
                        .expect("validated package source lowers to HIR")
                })
                .collect();
            PackagePrograms::new(package.identity().clone(), programs)
        })
        .collect();
    let resolved = resolve_project(graph, packages).expect("project resolves");
    let typed = ling_types::check(resolved).expect("project type-checks");
    ling_effects::check(typed).expect("project effects check")
}

fn checked_file(text: &str) -> CheckedProgram {
    let source = SourceFile::from_bytes(SourceId::new(0), "Main.ling", text.as_bytes().to_vec())
        .expect("file source is valid UTF-8");
    let parsed = ling_syntax::parse(&source);
    assert!(parsed.is_valid(), "file source has valid syntax");
    let ast = ling_ast::lower(&source, &parsed).expect("file source lowers to AST");
    let hir = ling_hir::lower(source.name(), &ast).expect("file source lowers to HIR");
    let resolved = ling_resolve::resolve(vec![hir], "Main").expect("file source resolves");
    let typed = ling_types::check(resolved).expect("file source type-checks");
    ling_effects::check(typed).expect("file source effects check")
}

#[test]
fn project_snapshot_is_package_qualified_path_free_and_deterministic() {
    let graph = package_graph("valid-cross-package");
    let snapshot = build_project(checked_project(&graph)).expect("project snapshot builds");
    let repeated = build_project(checked_project(&graph)).expect("project snapshot rebuilds");

    assert_eq!(snapshot.json(), repeated.json());
    assert_eq!(snapshot.program_id(), repeated.program_id());
    assert_eq!(
        snapshot.program_id().as_str(),
        "experimental:blake3:6f3d67e85b5820959041b90cfc9feee4dbca260afe88198ce612fbf4713b2cda"
    );
    assert_eq!(snapshot.graph().schema, PROJECT_SEMANTIC_SCHEMA);
    assert_eq!(
        snapshot.graph().package_graph_id.as_deref(),
        Some(graph.id().as_str())
    );
    assert_eq!(
        snapshot
            .graph()
            .root_package
            .as_ref()
            .map(|package| package.name.as_str()),
        Some("app")
    );
    assert_eq!(snapshot.graph().packages.len(), 2);

    let main_packages = snapshot
        .graph()
        .modules
        .iter()
        .filter(|module| module.name == "Main")
        .map(|module| {
            module
                .package
                .as_ref()
                .expect("project modules are package-qualified")
                .name
                .as_str()
        })
        .collect::<Vec<_>>();
    assert_eq!(main_packages, ["app", "math"]);

    let math_answer = snapshot
        .graph()
        .definitions
        .iter()
        .find(|definition| {
            definition.name == "answer"
                && definition
                    .package
                    .as_ref()
                    .is_some_and(|package| package.name == "math")
        })
        .expect("dependency definition is represented");
    assert_eq!(
        math_answer.definition_id,
        "experimental:blake3:40cdc27a3113254a05f5786cb25402b81dfc27e1f0cdce5958af04c7329de5eb"
    );
    assert_eq!(
        math_answer.body_id,
        "experimental:blake3:c1f6796db11f7b12827f6d6b433f8c1c6adcdb215556c20001791246b4ae700c"
    );
    assert!(snapshot.graph().references.iter().any(|reference| {
        reference.target_kind == "definition" && reference.target == math_answer.definition_id
    }));
    assert!(snapshot.graph().definitions.iter().all(|definition| {
        if definition.origin == "user" {
            definition.package.is_some()
        } else {
            definition.package.is_none()
        }
    }));

    assert!(!snapshot.json().contains("resolution-v1"));
    assert!(!snapshot.json().contains("C:\\"));
    assert_eq!(
        read_project_json(snapshot.json()).expect("writer output passes the project reader"),
        *snapshot.graph()
    );
}

#[test]
fn project_reader_rejects_missing_required_fields() {
    let graph = package_graph("valid-cross-package");
    let snapshot = build_project(checked_project(&graph)).expect("project snapshot builds");
    let value: serde_json::Value =
        serde_json::from_str(snapshot.json()).expect("snapshot JSON parses");

    for field in ["package_graph_id", "nodes"] {
        let mut missing = value.clone();
        missing
            .as_object_mut()
            .expect("snapshot root is an object")
            .remove(field);
        let error = read_project_json(
            &serde_json::to_string(&missing).expect("root-field mutation serializes"),
        )
        .expect_err("0.2 reader rejects every missing required root field");
        assert_eq!(
            error.kind,
            ling_semantic::SemanticReadErrorKind::MissingProjectField {
                field: field.to_owned()
            }
        );
        assert_eq!(error.path, format!("$.{field}"));
    }

    let mut missing = value;
    missing["nodes"][0]
        .as_object_mut()
        .expect("snapshot contains an object node")
        .remove("effects");
    let error = read_project_json(
        &serde_json::to_string(&missing).expect("node-field mutation serializes"),
    )
    .expect_err("0.2 reader rejects required node fields despite Serde defaults");
    assert_eq!(
        error.kind,
        ling_semantic::SemanticReadErrorKind::MissingProjectField {
            field: "effects".to_owned()
        }
    );
    assert_eq!(error.path, "$.nodes[0].effects");
}

#[test]
fn project_reader_enforces_import_and_export_boundaries() {
    let graph = package_graph("valid-cross-package");
    let snapshot = build_project(checked_project(&graph)).expect("project snapshot builds");
    let value: serde_json::Value =
        serde_json::from_str(snapshot.json()).expect("snapshot JSON parses");

    let mut private_import = value.clone();
    let math_package = private_import["packages"]
        .as_array_mut()
        .expect("packages is an array")
        .iter_mut()
        .find(|package| package["identity"]["name"] == "math")
        .expect("math package is represented");
    math_package["exports"] = serde_json::json!([]);
    let error = read_project_json(
        &serde_json::to_string(&private_import).expect("private-import mutation serializes"),
    )
    .expect_err("0.2 reader rejects imports of private dependency modules");
    assert!(matches!(
        error.kind,
        ling_semantic::SemanticReadErrorKind::PrivatePackageModule { .. }
    ));

    let mut unimported_reference = value;
    let app_main = unimported_reference["modules"]
        .as_array_mut()
        .expect("modules is an array")
        .iter_mut()
        .find(|module| module["package"]["name"] == "app" && module["name"] == "Main")
        .expect("app Main module is represented");
    app_main["imports"] = serde_json::json!([]);
    let error = read_project_json(
        &serde_json::to_string(&unimported_reference)
            .expect("unimported-reference mutation serializes"),
    )
    .expect_err("0.2 reader rejects cross-module references without a matching import");
    assert!(matches!(
        error.kind,
        ling_semantic::SemanticReadErrorKind::UnimportedReference { .. }
    ));
}

#[test]
fn project_builder_rejects_file_mode_checked_programs() {
    let error = build_project(checked_file("module Main\n\nlet main () = ()\n"))
        .expect_err("file-mode checked programs have no package identity");
    assert!(matches!(
        error,
        ling_semantic::ProjectSnapshotError::MissingProjectContext
    ));
}

#[test]
fn project_references_retain_cross_package_definition_targets() {
    let graph = package_graph("valid-cross-package");
    let checked = checked_project(&graph);
    assert!(
        checked
            .typed()
            .resolved()
            .references()
            .values()
            .any(|target| {
                matches!(target, ReferenceTarget::Definition(definition) if checked
            .typed()
            .resolved()
            .definitions()
            .get(definition)
            .and_then(|info| info.package.as_ref())
            .is_some_and(|package| package.name().as_str() == "math"))
            })
    );
}
