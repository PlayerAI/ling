use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ling_project::{
    LOCK_FILE_NAME, LockMode, PackageGraph, parse_lock_file, parse_manifest, resolve_package_graph,
    resolve_package_graph_with_lock,
};

const FIXTURE_ROOT: &str = "../../tests/projects/dependency-v1/valid-basic";
const APP_CONTENT: &str = "sha256:9784dc68f2c10713f5945024e5c6085e34b7735be86acc21e27d523e31a918f1";
const MATH_CONTENT: &str =
    "sha256:76c6c29d652bbd86f607a472a6091c5df95c7656d68fcc8d5c14f23517b65ba3";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_ROOT)
}

fn canonical_lock() -> Vec<u8> {
    format!(
        "{{\"format\":\"ling.lock/1\",\"packages\":[{{\"content\":\"{APP_CONTENT}\",\"dependencies\":[{{\"content\":\"{MATH_CONTENT}\",\"name\":\"math\"}}],\"name\":\"app\",\"version\":\"1.0.0\"}},{{\"content\":\"{MATH_CONTENT}\",\"dependencies\":[],\"name\":\"math\",\"version\":\"2.1.0\"}}],\"root\":{{\"content\":\"{APP_CONTENT}\",\"name\":\"app\",\"version\":\"1.0.0\"}}}}\n"
    )
    .into_bytes()
}

fn resolve(root: &Path) -> PackageGraph {
    let manifest_bytes = fs::read(root.join(ling_project::MANIFEST_FILE_NAME)).unwrap();
    let manifest = parse_manifest("ling.toml", &manifest_bytes).unwrap();
    resolve_package_graph(root, &manifest).unwrap()
}

fn resolve_with_lock(
    root: &Path,
    mode: LockMode,
) -> Result<PackageGraph, ling_project::LockedGraphFailure> {
    let manifest_bytes = fs::read(root.join(ling_project::MANIFEST_FILE_NAME)).unwrap();
    let manifest = parse_manifest("ling.toml", &manifest_bytes).unwrap();
    resolve_package_graph_with_lock(root, &manifest, mode)
}

fn failure_reason(bytes: &[u8]) -> (String, String) {
    let failure = parse_lock_file(LOCK_FILE_NAME, bytes).expect_err("lock must be rejected");
    let diagnostic = failure.diagnostic();
    let rendered: serde_json::Value =
        serde_json::from_str(&diagnostic.render_json().unwrap()).unwrap();
    (
        diagnostic.code().to_string(),
        rendered["facts"]["reason"].as_str().unwrap().to_owned(),
    )
}

fn locked_failure_reason(failure: &ling_project::LockedGraphFailure) -> (String, String) {
    let diagnostic = &failure.diagnostics().unwrap()[0];
    let rendered: serde_json::Value =
        serde_json::from_str(&diagnostic.render_json().unwrap()).unwrap();
    (
        diagnostic.code().to_string(),
        rendered["facts"]["reason"].as_str().unwrap().to_owned(),
    )
}

#[test]
fn canonical_writer_matches_the_frozen_lock_and_round_trips_exactly() {
    let graph = resolve(&fixture());
    let lock = ling_project::LockFile::from_graph(&graph);
    let bytes = lock.to_canonical_bytes();

    assert_eq!(bytes, canonical_lock());
    let decoded = parse_lock_file(LOCK_FILE_NAME, &bytes).unwrap();
    assert_eq!(decoded, lock);
    assert_eq!(decoded.to_canonical_bytes(), bytes);
    assert!(decoded.matches_graph(&graph));
}

#[test]
fn corrupt_noncanonical_and_incompatible_locks_are_rejected() {
    let canonical = canonical_lock();
    let text = String::from_utf8(canonical.clone()).unwrap();
    let cases = [
        ("whitespace", format!(" {text}"), "noncanonical_bytes"),
        (
            "key-order",
            text.replacen(
                "{\"format\":\"ling.lock/1\",\"packages\":",
                "{\"packages\":",
                1,
            )
            .replacen(",\"root\":", ",\"format\":\"ling.lock/1\",\"root\":", 1),
            "noncanonical_bytes",
        ),
        (
            "uppercase-digest",
            text.replacen("9784dc68", "9784DC68", 1),
            "invalid_content_id",
        ),
        (
            "unknown-field",
            text.replacen("{\"format\":", "{\"extra\":\"value\",\"format\":", 1),
            "invalid_json",
        ),
        (
            "incompatible-format",
            text.replacen("ling.lock/1", "ling.lock/2", 1),
            "unsupported_format",
        ),
        (
            "dangling-content",
            text.replacen(MATH_CONTENT, &format!("sha256:{}", "0".repeat(64)), 1),
            "dangling_dependency",
        ),
        (
            "truncated",
            text[..text.len() - 2].to_owned(),
            "invalid_json",
        ),
    ];

    for (label, bytes, reason) in cases {
        assert_eq!(
            failure_reason(bytes.as_bytes()),
            ("L-PROJECT-0018".to_owned(), reason.to_owned()),
            "{label}"
        );
    }

    assert_eq!(
        failure_reason(&canonical[..canonical.len() - 1]),
        ("L-PROJECT-0018".to_owned(), "noncanonical_bytes".to_owned())
    );
}

#[test]
fn update_and_locked_modes_preserve_transactional_lock_semantics() {
    let project = TempProject::copy_fixture("modes");
    let lock_path = project.root.join(LOCK_FILE_NAME);

    let initial = resolve_with_lock(&project.root, LockMode::Update).unwrap();
    assert_eq!(fs::read(&lock_path).unwrap(), canonical_lock());

    let original_permissions = fs::metadata(&lock_path).unwrap().permissions();
    let mut read_only = original_permissions.clone();
    read_only.set_readonly(true);
    fs::set_permissions(&lock_path, read_only).unwrap();
    resolve_with_lock(&project.root, LockMode::Update)
        .expect("an unchanged lock is never rewritten");
    fs::set_permissions(&lock_path, original_permissions).unwrap();

    assert_eq!(
        resolve_with_lock(&project.root, LockMode::Locked)
            .unwrap()
            .id(),
        initial.id()
    );

    fs::write(
        project.root.join("vendor/math/src/Algebra.ling"),
        "module Algebra\n\nlet add left right = left + right + 1\n",
    )
    .unwrap();
    let previous = fs::read(&lock_path).unwrap();
    let failure = resolve_with_lock(&project.root, LockMode::Locked).unwrap_err();
    assert_eq!(
        failure.diagnostics().unwrap()[0].code().as_str(),
        "L-PROJECT-0019"
    );
    assert_eq!(fs::read(&lock_path).unwrap(), previous);

    let updated = resolve_with_lock(&project.root, LockMode::Update).unwrap();
    assert_ne!(updated.id(), initial.id());
    assert_ne!(fs::read(&lock_path).unwrap(), previous);
}

#[test]
fn missing_and_corrupt_locks_never_get_silently_rewritten() {
    let missing = TempProject::copy_fixture("missing");
    let missing_path = missing.root.join(LOCK_FILE_NAME);
    let failure = resolve_with_lock(&missing.root, LockMode::Locked).unwrap_err();
    assert_eq!(
        failure.diagnostics().unwrap()[0].code().as_str(),
        "L-PROJECT-0019"
    );
    assert!(!missing_path.exists());

    let corrupt = TempProject::copy_fixture("corrupt");
    let corrupt_path = corrupt.root.join(LOCK_FILE_NAME);
    fs::write(&corrupt_path, b"{not-json}\n").unwrap();
    let previous = fs::read(&corrupt_path).unwrap();
    let failure = resolve_with_lock(&corrupt.root, LockMode::Update).unwrap_err();
    assert_eq!(
        failure.diagnostics().unwrap()[0].code().as_str(),
        "L-PROJECT-0018"
    );
    assert_eq!(fs::read(&corrupt_path).unwrap(), previous);
}

#[test]
fn reader_limits_depth_package_count_dependency_count_and_file_size() {
    let deeply_nested = format!(
        "{{\"format\":\"ling.lock/1\",\"packages\":{}",
        "[".repeat(256)
    );
    assert_eq!(
        failure_reason(deeply_nested.as_bytes()),
        ("L-PROJECT-0018".to_owned(), "invalid_json".to_owned())
    );

    let package = |name: &str| {
        format!(
            "{{\"content\":\"sha256:{}\",\"dependencies\":[],\"name\":\"{name}\",\"version\":\"1.0.0\"}}",
            "0".repeat(64)
        )
    };
    let packages = (0..=4_096)
        .map(|index| package(&format!("p{index}")))
        .collect::<Vec<_>>()
        .join(",");
    let too_many_packages = format!(
        "{{\"format\":\"ling.lock/1\",\"packages\":[{packages}],\"root\":{{\"content\":\"sha256:{}\",\"name\":\"p0\",\"version\":\"1.0.0\"}}}}\n",
        "0".repeat(64)
    );
    assert_eq!(
        failure_reason(too_many_packages.as_bytes()).1,
        "too_many_packages"
    );

    let dependencies = (0..=1_024)
        .map(|index| {
            format!(
                "{{\"content\":\"sha256:{}\",\"name\":\"p{index}\"}}",
                "0".repeat(64)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let too_many_dependencies = format!(
        "{{\"format\":\"ling.lock/1\",\"packages\":[{{\"content\":\"sha256:{}\",\"dependencies\":[{dependencies}],\"name\":\"app\",\"version\":\"1.0.0\"}}],\"root\":{{\"content\":\"sha256:{}\",\"name\":\"app\",\"version\":\"1.0.0\"}}}}\n",
        "0".repeat(64),
        "0".repeat(64)
    );
    assert_eq!(
        failure_reason(too_many_dependencies.as_bytes()).1,
        "too_many_dependencies"
    );

    let oversized = TempProject::copy_fixture("oversized");
    let lock_path = oversized.root.join(LOCK_FILE_NAME);
    fs::File::create(&lock_path)
        .unwrap()
        .set_len(u64::try_from(ling_project::MAX_LOCK_FILE_BYTES).unwrap() + 1)
        .unwrap();
    let failure = resolve_with_lock(&oversized.root, LockMode::Update).unwrap_err();
    assert_eq!(
        locked_failure_reason(&failure),
        ("L-PROJECT-0018".to_owned(), "lock_too_large".to_owned())
    );
    assert_eq!(
        fs::metadata(lock_path).unwrap().len(),
        u64::try_from(ling_project::MAX_LOCK_FILE_BYTES).unwrap() + 1
    );
}

#[test]
fn exact_lock_filename_case_is_enforced() {
    let project = TempProject::copy_fixture("filename-case");
    fs::write(project.root.join("LING.LOCK"), canonical_lock()).unwrap();

    let failure = resolve_with_lock(&project.root, LockMode::Locked).unwrap_err();
    assert_eq!(
        locked_failure_reason(&failure),
        (
            "L-PROJECT-0018".to_owned(),
            "lock_filename_case_mismatch".to_owned()
        )
    );
    assert!(!project.root.join(LOCK_FILE_NAME).exists() || cfg!(windows));
}

#[test]
fn lock_resolution_has_no_network_or_process_surface() {
    const SOURCE: &str = include_str!("../src/lockfile.rs");
    for forbidden in [
        "std::net::",
        "std::process::Command",
        "Command::new",
        "TcpStream",
        "UdpSocket",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "forbidden lock API: {forbidden}"
        );
    }
}

struct TempProject {
    root: PathBuf,
}

impl TempProject {
    fn copy_fixture(label: &str) -> Self {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ling-prj-1105-{label}-{}-{sequence}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }
        copy_tree(&fixture(), &root);
        Self { root }
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        if self.root.exists() {
            fs::remove_dir_all(&self.root).unwrap();
        }
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    let mut entries = fs::read_dir(source)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}
