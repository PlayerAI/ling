use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;

const FIXTURE_PATH: &str = "../../editors/tree-sitter-ling/test/fixtures/conformance-syntax.tsv";
const CONFORMANCE_PATH: &str = "../../tests/conformance";
const EXPECTED_CASE_COUNT: usize = 42;
const EXPECTED_VALID_COUNT: usize = 34;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CompilerSyntax {
    Valid,
    Invalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TreeSitterPolicy {
    Clean,
    Error,
    Tolerated,
}

#[derive(Debug)]
struct ManifestEntry {
    relative_path: String,
    compiler_syntax: CompilerSyntax,
    tree_sitter_policy: TreeSitterPolicy,
}

#[test]
fn compiler_matches_the_shared_whole_program_syntax_manifest() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = crate_root.join(FIXTURE_PATH);
    let conformance_root = crate_root.join(CONFORMANCE_PATH);
    let entries = read_manifest(&fixture_path);

    assert_manifest_coverage(&entries, &conformance_root);

    let mut valid_count = 0;
    for (index, entry) in entries.iter().enumerate() {
        let source_path = conformance_root.join(&entry.relative_path);
        let source_bytes = fs::read(&source_path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", source_path.display()));
        let source = SourceFile::from_bytes(
            SourceId::new(u32::try_from(index).expect("fixture count fits u32")),
            entry.relative_path.clone(),
            source_bytes,
        )
        .unwrap_or_else(|error| panic!("{} is not valid UTF-8: {error:?}", source_path.display()));
        let parsed = parse(&source);
        let expected_valid = entry.compiler_syntax == CompilerSyntax::Valid;
        valid_count += usize::from(expected_valid);

        assert_eq!(
            parsed.is_valid(),
            expected_valid,
            "{}: lexical={:?}, parse={:?}",
            entry.relative_path,
            parsed.lexical_errors(),
            parsed.parse_errors()
        );
    }

    assert_eq!(
        valid_count, EXPECTED_VALID_COUNT,
        "the compiler syntax classification changed unexpectedly"
    );
}

fn read_manifest(path: &Path) -> Vec<ManifestEntry> {
    let fixture = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    let mut entries = Vec::new();
    let mut previous_path: Option<&str> = None;

    for (index, line) in fixture.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let [relative_path, compiler_syntax, tree_sitter_policy] = line
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("invalid manifest row {}: {line}", index + 1));
        assert_safe_relative_path(relative_path, index + 1);
        if let Some(previous) = previous_path {
            assert!(
                previous < relative_path,
                "manifest paths must be unique and sorted: {previous:?} before {relative_path:?}"
            );
        }
        previous_path = Some(relative_path);

        let compiler_syntax = match compiler_syntax {
            "valid" => CompilerSyntax::Valid,
            "invalid" => CompilerSyntax::Invalid,
            other => panic!("unknown compiler syntax {other:?} in row {}", index + 1),
        };
        let tree_sitter_policy = match tree_sitter_policy {
            "clean" => TreeSitterPolicy::Clean,
            "error" => TreeSitterPolicy::Error,
            "tolerated" => TreeSitterPolicy::Tolerated,
            other => panic!("unknown Tree-sitter policy {other:?} in row {}", index + 1),
        };
        match compiler_syntax {
            CompilerSyntax::Valid => assert_eq!(
                tree_sitter_policy,
                TreeSitterPolicy::Clean,
                "valid compiler input must have a clean Tree-sitter policy: {relative_path}"
            ),
            CompilerSyntax::Invalid => assert_ne!(
                tree_sitter_policy,
                TreeSitterPolicy::Clean,
                "invalid compiler input must use error or an explicit tolerance: {relative_path}"
            ),
        }
        entries.push(ManifestEntry {
            relative_path: relative_path.to_owned(),
            compiler_syntax,
            tree_sitter_policy,
        });
    }

    assert_eq!(
        entries.len(),
        EXPECTED_CASE_COUNT,
        "the TS-3108 manifest changed unexpectedly"
    );
    assert_eq!(
        entries
            .iter()
            .filter(|entry| entry.tree_sitter_policy == TreeSitterPolicy::Tolerated)
            .count(),
        1,
        "each Tree-sitter tolerance must remain explicit and reviewed"
    );
    entries
}

fn assert_safe_relative_path(relative_path: &str, row: usize) {
    let path = Path::new(relative_path);
    assert!(
        !path.is_absolute()
            && path
                .components()
                .all(|component| matches!(component, Component::Normal(_))),
        "manifest row {row} has an unsafe path: {relative_path:?}"
    );
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("case.ling"),
        "manifest row {row} must name a case.ling file"
    );
}

fn assert_manifest_coverage(entries: &[ManifestEntry], conformance_root: &Path) {
    let manifest_paths = entries
        .iter()
        .map(|entry| PathBuf::from(&entry.relative_path))
        .collect::<BTreeSet<_>>();
    let mut discovered_paths = BTreeSet::new();

    for directory in fs::read_dir(conformance_root)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", conformance_root.display()))
    {
        let directory = directory.expect("conformance directory entry is readable");
        if !directory
            .file_type()
            .expect("conformance entry type is readable")
            .is_dir()
        {
            continue;
        }
        let relative_path = PathBuf::from(directory.file_name()).join("case.ling");
        if conformance_root.join(&relative_path).is_file() {
            discovered_paths.insert(relative_path);
        }
    }

    assert_eq!(
        manifest_paths, discovered_paths,
        "the TS-3108 manifest must cover every compiler conformance source exactly once"
    );
}
