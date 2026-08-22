use std::fs;
use std::path::Path;

use ling_project::{
    LOCK_FILE_NAME, LockedProject, MANIFEST_FILE_NAME, load_locked_project, parse_manifest,
};

const FIXTURE_ROOT: &str = "../../tests/projects/offline-lock";

fn load_fixture() -> LockedProject {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_ROOT);
    let bytes = fs::read(root.join(MANIFEST_FILE_NAME)).expect("fixture manifest is readable");
    let manifest = parse_manifest(MANIFEST_FILE_NAME, &bytes).expect("fixture manifest is valid");
    load_locked_project(&root, &manifest).expect("fixture lock is valid")
}

#[test]
fn locked_project_snapshot_is_repeatable_and_path_free() {
    let first = load_fixture();
    let second = load_fixture();

    assert_eq!(first, second);
    assert_eq!(first.manifest(), second.manifest());
    assert_eq!(first.graph(), second.graph());
    assert_eq!(first.lock(), second.lock());
    assert_eq!(first.graph().id(), second.graph().id());
}

#[test]
fn locked_project_load_does_not_rewrite_the_lock() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_ROOT);
    let before = fs::read(root.join(LOCK_FILE_NAME)).expect("fixture lock is readable");
    let loaded = load_fixture();
    let after = fs::read(root.join(LOCK_FILE_NAME)).expect("fixture lock remains readable");

    assert_eq!(before, after);
    assert_eq!(loaded.lock().to_canonical_bytes(), before);
}
