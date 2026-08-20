use std::fs;
use std::path::{Path, PathBuf};

use ling_project::{PackageGraph, parse_manifest, resolve_package_graph};
use ling_resolve::{
    DefinitionOrigin, PackagePrograms, ProjectResolveFailure, ReferenceTarget, resolve_project,
};
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

fn lower_packages(graph: &PackageGraph) -> Vec<PackagePrograms> {
    graph
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
                    assert!(parsed.is_valid(), "package graph retained valid syntax");
                    let ast = ling_ast::lower(&source_file, &parsed)
                        .expect("validated syntax lowers to AST");
                    ling_hir::lower(source_name, &ast)
                        .expect("validated package source lowers to HIR")
                })
                .collect();
            PackagePrograms::new(package.identity().clone(), programs)
        })
        .collect()
}

#[test]
fn resolves_direct_exported_modules_with_package_qualified_definition_ids() {
    let graph = package_graph("valid-cross-package");
    let resolved = resolve_project(&graph, lower_packages(&graph)).expect("project resolves");
    let project = resolved.project().expect("project metadata is retained");
    assert_eq!(project.graph_id(), graph.id());
    assert_eq!(project.root(), graph.root());
    assert_eq!(project.packages().len(), 2);

    let main_modules = resolved
        .modules()
        .iter()
        .filter(|module| module.hir.module.name.normalized() == "Main")
        .collect::<Vec<_>>();
    assert_eq!(main_modules.len(), 2, "module names are package-local");
    assert_ne!(main_modules[0].package, main_modules[1].package);

    let app_answer = resolved
        .definitions()
        .values()
        .find(|definition| {
            definition.name == "answer"
                && definition
                    .package
                    .as_ref()
                    .is_some_and(|package| package.name().as_str() == "app")
        })
        .expect("app answer exists");
    let math_answer = resolved
        .definitions()
        .values()
        .find(|definition| {
            definition.name == "answer"
                && definition
                    .package
                    .as_ref()
                    .is_some_and(|package| package.name().as_str() == "math")
        })
        .expect("math answer exists");
    assert_ne!(app_answer.id, math_answer.id);
    assert_eq!(
        math_answer.source_name.as_deref(),
        Some("package:math/src/Algebra.ling")
    );
    let math_span = math_answer
        .span
        .expect("user definitions retain their original UTF-8 byte span");
    assert_eq!((math_span.start().get(), math_span.end().get()), (20, 26));
    assert!(matches!(app_answer.origin, DefinitionOrigin::User { .. }));
    assert!(
        resolved.references().values().any(
            |target| matches!(target, ReferenceTarget::Definition(id) if id == &math_answer.id)
        )
    );
}

#[test]
fn project_resolution_is_invariant_to_supplied_package_and_module_order() {
    let graph = package_graph("valid-cross-package");
    let canonical = resolve_project(&graph, lower_packages(&graph)).expect("project resolves");
    let mut permuted = lower_packages(&graph)
        .into_iter()
        .map(|package| {
            PackagePrograms::new(
                package.identity().clone(),
                package.programs().iter().cloned().rev().collect(),
            )
        })
        .collect::<Vec<_>>();
    permuted.reverse();

    assert_eq!(
        resolve_project(&graph, permuted).expect("permuted project resolves"),
        canonical
    );
}

#[test]
fn rejects_hir_sets_that_do_not_exactly_match_the_package_graph() {
    let graph = package_graph("valid-cross-package");
    let mut packages = lower_packages(&graph);
    packages.retain(|package| package.identity().name().as_str() != "app");

    let failure = resolve_project(&graph, packages).expect_err("missing HIR must fail");
    let ProjectResolveFailure::Input(error) = failure else {
        panic!("expected a structural input failure");
    };
    assert_eq!(error.reason, "missing_package");
    assert_eq!(error.package.as_deref(), Some("app"));
    assert_eq!(error.module, None);
}
