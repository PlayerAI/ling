use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use serde::{Deserialize, Serialize};

use super::discovery::stable_io_kind;
use super::{
    DependencyGraphFailure, MAX_DEPENDENCIES, Manifest, PackageGraph, PackageIdentity, PackageName,
    PackageSourceId, resolve_package_graph, validate_package_name, validate_package_version,
};

/// Exact project-lock filename fixed by RFC-0002.
pub const LOCK_FILE_NAME: &str = "ling.lock";
/// Only lock format accepted and emitted by this implementation.
pub const LOCK_FILE_FORMAT: &str = "ling.lock/1";
/// Maximum accepted encoded lock size in bytes.
pub const MAX_LOCK_FILE_BYTES: usize = 512 * 1024 * 1024;

const MAX_LOCK_PACKAGES: usize = 4_096;
const MAX_PROJECT_DIRECTORY_ENTRIES: usize = 65_536;
const MAX_TEMPORARY_FILE_ATTEMPTS: usize = 1_024;
static TEMPORARY_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Policy applied after an offline local package graph has fully validated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LockMode {
    /// Create a missing lock or replace a valid stale lock.
    Update,
    /// Require a byte-valid lock that exactly matches the resolved graph.
    Locked,
}

/// One canonical dependency reference in `ling.lock/1`.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LockedDependency {
    name: PackageName,
    content: PackageSourceId,
}

impl LockedDependency {
    #[must_use]
    pub const fn content(&self) -> &PackageSourceId {
        &self.content
    }

    #[must_use]
    pub const fn name(&self) -> &PackageName {
        &self.name
    }
}

/// One content-identified package record in `ling.lock/1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockedPackage {
    identity: PackageIdentity,
    dependencies: Box<[LockedDependency]>,
}

impl LockedPackage {
    #[must_use]
    pub const fn identity(&self) -> &PackageIdentity {
        &self.identity
    }

    #[must_use]
    pub fn dependencies(&self) -> &[LockedDependency] {
        &self.dependencies
    }
}

/// Fully validated canonical model of `ling.lock/1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockFile {
    packages: Box<[LockedPackage]>,
    root: PackageIdentity,
}

impl LockFile {
    /// Projects a fully checked package graph into the path-free lock model.
    #[must_use]
    pub fn from_graph(graph: &PackageGraph) -> Self {
        let mut packages = graph
            .packages()
            .iter()
            .map(|package| {
                let mut dependencies = graph
                    .edges()
                    .iter()
                    .filter(|edge| edge.from() == package.identity())
                    .map(|edge| LockedDependency {
                        content: edge.to().source().clone(),
                        name: edge.dependency().clone(),
                    })
                    .collect::<Vec<_>>();
                dependencies.sort();
                LockedPackage {
                    identity: package.identity().clone(),
                    dependencies: dependencies.into_boxed_slice(),
                }
            })
            .collect::<Vec<_>>();
        packages.sort_by(|left, right| left.identity.cmp(&right.identity));
        Self {
            packages: packages.into_boxed_slice(),
            root: graph.root().clone(),
        }
    }

    #[must_use]
    pub const fn format(&self) -> &'static str {
        LOCK_FILE_FORMAT
    }

    #[must_use]
    pub fn packages(&self) -> &[LockedPackage] {
        &self.packages
    }

    #[must_use]
    pub const fn root(&self) -> &PackageIdentity {
        &self.root
    }

    /// Emits RFC-0002 compact canonical JSON followed by exactly one LF.
    #[must_use]
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        let encoded = EncodedLock::from(self);
        let mut bytes = serde_json::to_vec(&encoded)
            .expect("the validated lock domain contains only JSON-serializable strings");
        bytes.push(b'\n');
        bytes
    }

    #[must_use]
    pub fn matches_graph(&self, graph: &PackageGraph) -> bool {
        self == &Self::from_graph(graph)
    }
}

/// Structured, diagnostic-bearing lock read, validation, or persistence failure.
#[derive(Debug)]
pub struct LockFileFailure {
    diagnostics: Box<[Diagnostic]>,
}

impl LockFileFailure {
    #[must_use]
    pub fn diagnostic(&self) -> &Diagnostic {
        &self.diagnostics[0]
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

impl fmt::Display for LockFileFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "project lock operation produced {} diagnostic(s)",
            self.diagnostics.len()
        )
    }
}

impl Error for LockFileFailure {}

/// Atomic failure from graph resolution followed by lock enforcement.
#[derive(Debug)]
pub enum LockedGraphFailure {
    Graph(DependencyGraphFailure),
    Lock(LockFileFailure),
}

impl LockedGraphFailure {
    #[must_use]
    pub fn diagnostics(&self) -> Option<&[Diagnostic]> {
        match self {
            Self::Graph(failure) => failure.diagnostics(),
            Self::Lock(failure) => Some(failure.diagnostics()),
        }
    }
}

impl fmt::Display for LockedGraphFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Graph(failure) => failure.fmt(formatter),
            Self::Lock(failure) => failure.fmt(formatter),
        }
    }
}

impl Error for LockedGraphFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Graph(failure) => Some(failure),
            Self::Lock(failure) => Some(failure),
        }
    }
}

impl From<DependencyGraphFailure> for LockedGraphFailure {
    fn from(failure: DependencyGraphFailure) -> Self {
        Self::Graph(failure)
    }
}

impl From<LockFileFailure> for LockedGraphFailure {
    fn from(failure: LockFileFailure) -> Self {
        Self::Lock(failure)
    }
}

/// Reads and validates only exact canonical `ling.lock/1` bytes.
pub fn parse_lock_file(source_name: &str, bytes: &[u8]) -> Result<LockFile, LockFileFailure> {
    if bytes.len() > MAX_LOCK_FILE_BYTES {
        return Err(invalid_lock_failure(
            source_name,
            0..0,
            "lock_too_large",
            None,
        ));
    }

    let raw = serde_json::from_slice::<RawLock>(bytes).map_err(|error| {
        invalid_lock_failure(
            source_name,
            json_error_span(bytes, &error),
            "invalid_json",
            None,
        )
    })?;
    if raw.format != LOCK_FILE_FORMAT {
        return Err(invalid_lock_failure(
            source_name,
            full_span(bytes),
            "unsupported_format",
            None,
        ));
    }
    if raw.packages.len() > MAX_LOCK_PACKAGES {
        return Err(invalid_lock_failure(
            source_name,
            full_span(bytes),
            "too_many_packages",
            None,
        ));
    }

    let root = parse_identity(&raw.root.content, &raw.root.name, &raw.root.version).map_err(
        |failure| {
            invalid_lock_failure(
                source_name,
                full_span(bytes),
                failure.reason,
                failure.package.as_deref(),
            )
        },
    )?;
    let mut packages = Vec::with_capacity(raw.packages.len());
    let mut package_names = BTreeSet::new();
    for raw_package in raw.packages {
        let identity = parse_identity(
            &raw_package.content,
            &raw_package.name,
            &raw_package.version,
        )
        .map_err(|failure| {
            invalid_lock_failure(
                source_name,
                full_span(bytes),
                failure.reason,
                failure.package.as_deref(),
            )
        })?;
        let package_name = identity.name().as_str().to_owned();
        if !package_names.insert(package_name.clone()) {
            return Err(invalid_lock_failure(
                source_name,
                full_span(bytes),
                "duplicate_package_name",
                Some(&package_name),
            ));
        }
        if raw_package.dependencies.len() > MAX_DEPENDENCIES {
            return Err(invalid_lock_failure(
                source_name,
                full_span(bytes),
                "too_many_dependencies",
                Some(&package_name),
            ));
        }

        let mut dependency_names = BTreeSet::new();
        let mut dependencies = Vec::with_capacity(raw_package.dependencies.len());
        for raw_dependency in raw_package.dependencies {
            let name = validate_package_name(&raw_dependency.name).map_err(|_| {
                invalid_lock_failure(
                    source_name,
                    full_span(bytes),
                    "invalid_package_name",
                    Some(&package_name),
                )
            })?;
            if !dependency_names.insert(name.clone()) {
                return Err(invalid_lock_failure(
                    source_name,
                    full_span(bytes),
                    "duplicate_dependency_name",
                    Some(&package_name),
                ));
            }
            let content = parse_content_id(&raw_dependency.content).map_err(|reason| {
                invalid_lock_failure(source_name, full_span(bytes), reason, Some(&package_name))
            })?;
            dependencies.push(LockedDependency { content, name });
        }
        dependencies.sort();
        packages.push(LockedPackage {
            identity,
            dependencies: dependencies.into_boxed_slice(),
        });
    }
    packages.sort_by(|left, right| left.identity.cmp(&right.identity));

    validate_locked_graph(&packages, &root).map_err(|failure| {
        invalid_lock_failure(
            source_name,
            full_span(bytes),
            failure.reason,
            failure.package.as_deref(),
        )
    })?;

    let lock = LockFile {
        packages: packages.into_boxed_slice(),
        root,
    };
    if lock.to_canonical_bytes() != bytes {
        return Err(invalid_lock_failure(
            source_name,
            full_span(bytes),
            "noncanonical_bytes",
            None,
        ));
    }
    Ok(lock)
}

/// Resolves the complete local graph, then applies RFC-0002 lock policy.
///
/// This function performs no network request, ambient package lookup, or shell
/// execution. A graph or lock failure returns no partial graph.
pub fn resolve_package_graph_with_lock(
    project_root: &Path,
    root_manifest: &Manifest,
    mode: LockMode,
) -> Result<PackageGraph, LockedGraphFailure> {
    let graph = resolve_package_graph(project_root, root_manifest)?;
    let expected = LockFile::from_graph(&graph);
    let expected_bytes = expected.to_canonical_bytes();
    let existing = read_exact_lock(project_root)?;

    match existing {
        None if mode == LockMode::Locked => Err(lock_mismatch_failure("missing_lock").into()),
        None => {
            persist_lock(project_root, &expected_bytes)?;
            Ok(graph)
        }
        Some(bytes) => {
            let actual = parse_lock_file(LOCK_FILE_NAME, &bytes)?;
            if actual == expected {
                return Ok(graph);
            }
            if mode == LockMode::Locked {
                return Err(lock_mismatch_failure("graph_mismatch").into());
            }
            persist_lock(project_root, &expected_bytes)?;
            Ok(graph)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLock {
    format: String,
    packages: Vec<RawPackage>,
    root: RawIdentity,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPackage {
    content: String,
    dependencies: Vec<RawDependency>,
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDependency {
    content: String,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawIdentity {
    content: String,
    name: String,
    version: String,
}

#[derive(Serialize)]
struct EncodedLock<'a> {
    format: &'static str,
    packages: Vec<EncodedPackage<'a>>,
    root: EncodedIdentity<'a>,
}

impl<'a> From<&'a LockFile> for EncodedLock<'a> {
    fn from(lock: &'a LockFile) -> Self {
        Self {
            format: LOCK_FILE_FORMAT,
            packages: lock.packages.iter().map(EncodedPackage::from).collect(),
            root: EncodedIdentity::from(&lock.root),
        }
    }
}

#[derive(Serialize)]
struct EncodedPackage<'a> {
    content: &'a str,
    dependencies: Vec<EncodedDependency<'a>>,
    name: &'a str,
    version: String,
}

impl<'a> From<&'a LockedPackage> for EncodedPackage<'a> {
    fn from(package: &'a LockedPackage) -> Self {
        Self {
            content: package.identity.source().as_str(),
            dependencies: package
                .dependencies
                .iter()
                .map(EncodedDependency::from)
                .collect(),
            name: package.identity.name().as_str(),
            version: package.identity.version().to_string(),
        }
    }
}

#[derive(Serialize)]
struct EncodedDependency<'a> {
    content: &'a str,
    name: &'a str,
}

impl<'a> From<&'a LockedDependency> for EncodedDependency<'a> {
    fn from(dependency: &'a LockedDependency) -> Self {
        Self {
            content: dependency.content.as_str(),
            name: dependency.name.as_str(),
        }
    }
}

#[derive(Serialize)]
struct EncodedIdentity<'a> {
    content: &'a str,
    name: &'a str,
    version: String,
}

impl<'a> From<&'a PackageIdentity> for EncodedIdentity<'a> {
    fn from(identity: &'a PackageIdentity) -> Self {
        Self {
            content: identity.source().as_str(),
            name: identity.name().as_str(),
            version: identity.version().to_string(),
        }
    }
}

struct ValidationFailure {
    reason: &'static str,
    package: Option<String>,
}

fn parse_identity(
    content: &str,
    name: &str,
    version: &str,
) -> Result<PackageIdentity, ValidationFailure> {
    let name = validate_package_name(name).map_err(|_| ValidationFailure {
        reason: "invalid_package_name",
        package: None,
    })?;
    let package = Some(name.as_str().to_owned());
    let version = validate_package_version(version).map_err(|_| ValidationFailure {
        reason: "invalid_package_version",
        package: package.clone(),
    })?;
    let source = parse_content_id(content).map_err(|reason| ValidationFailure {
        reason,
        package: package.clone(),
    })?;
    Ok(PackageIdentity::new(name, version, source))
}

fn parse_content_id(raw: &str) -> Result<PackageSourceId, &'static str> {
    let Some(hex) = raw.strip_prefix("sha256:") else {
        return Err("invalid_content_id");
    };
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("invalid_content_id");
    }
    Ok(PackageSourceId::from_validated(raw.to_owned()))
}

fn validate_locked_graph(
    packages: &[LockedPackage],
    root: &PackageIdentity,
) -> Result<(), ValidationFailure> {
    let indexes = packages
        .iter()
        .enumerate()
        .map(|(index, package)| (package.identity.name().as_str(), index))
        .collect::<BTreeMap<_, _>>();
    let Some(&root_index) = indexes.get(root.name().as_str()) else {
        return Err(ValidationFailure {
            reason: "missing_root",
            package: Some(root.name().as_str().to_owned()),
        });
    };
    if packages[root_index].identity != *root {
        return Err(ValidationFailure {
            reason: "missing_root",
            package: Some(root.name().as_str().to_owned()),
        });
    }

    let mut adjacency = vec![Vec::new(); packages.len()];
    let mut indegrees = vec![0_usize; packages.len()];
    for (from, package) in packages.iter().enumerate() {
        for dependency in &package.dependencies {
            let Some(&to) = indexes.get(dependency.name.as_str()) else {
                return Err(ValidationFailure {
                    reason: "dangling_dependency",
                    package: Some(package.identity.name().as_str().to_owned()),
                });
            };
            if packages[to].identity.source() != &dependency.content {
                return Err(ValidationFailure {
                    reason: "dangling_dependency",
                    package: Some(package.identity.name().as_str().to_owned()),
                });
            }
            adjacency[from].push(to);
            indegrees[to] = indegrees[to].saturating_add(1);
        }
    }

    let mut ready = BTreeSet::new();
    for (index, indegree) in indegrees.iter().enumerate() {
        if *indegree == 0 {
            ready.insert(index);
        }
    }
    let mut visited = 0_usize;
    while let Some(index) = ready.pop_first() {
        visited += 1;
        for &target in &adjacency[index] {
            indegrees[target] -= 1;
            if indegrees[target] == 0 {
                ready.insert(target);
            }
        }
    }
    if visited != packages.len() {
        let package = indegrees
            .iter()
            .position(|indegree| *indegree != 0)
            .map(|index| packages[index].identity.name().as_str().to_owned());
        return Err(ValidationFailure {
            reason: "graph_cycle",
            package,
        });
    }

    let mut reachable = vec![false; packages.len()];
    let mut queue = VecDeque::from([root_index]);
    reachable[root_index] = true;
    while let Some(index) = queue.pop_front() {
        for &target in &adjacency[index] {
            if !reachable[target] {
                reachable[target] = true;
                queue.push_back(target);
            }
        }
    }
    if let Some(index) = reachable.iter().position(|value| !*value) {
        return Err(ValidationFailure {
            reason: "unreachable_package",
            package: Some(packages[index].identity.name().as_str().to_owned()),
        });
    }
    Ok(())
}

fn read_exact_lock(project_root: &Path) -> Result<Option<Vec<u8>>, LockFileFailure> {
    let entries =
        fs::read_dir(project_root).map_err(|error| lock_io_failure("read", error.kind()))?;
    let mut exact_path = None;
    let mut case_mismatch = false;
    let mut entry_count = 0_usize;
    for entry in entries {
        entry_count = entry_count.saturating_add(1);
        if entry_count > MAX_PROJECT_DIRECTORY_ENTRIES {
            return Err(invalid_lock_failure(
                LOCK_FILE_NAME,
                0..0,
                "directory_entry_limit",
                None,
            ));
        }
        let entry = entry.map_err(|error| lock_io_failure("read", error.kind()))?;
        let name = entry.file_name();
        if name == OsStr::new(LOCK_FILE_NAME) {
            exact_path = Some(entry.path());
        } else if name
            .to_str()
            .is_some_and(|name| name.eq_ignore_ascii_case(LOCK_FILE_NAME))
        {
            case_mismatch = true;
        }
    }
    if case_mismatch {
        return Err(invalid_lock_failure(
            LOCK_FILE_NAME,
            0..0,
            "lock_filename_case_mismatch",
            None,
        ));
    }
    let Some(path) = exact_path else {
        return Ok(None);
    };
    let metadata =
        fs::symlink_metadata(&path).map_err(|error| lock_io_failure("read", error.kind()))?;
    if metadata.file_type().is_symlink() {
        return Err(invalid_lock_failure(
            LOCK_FILE_NAME,
            0..0,
            "lock_symlink",
            None,
        ));
    }
    if !metadata.is_file() {
        return Err(invalid_lock_failure(
            LOCK_FILE_NAME,
            0..0,
            "lock_not_file",
            None,
        ));
    }
    if metadata.len() > u64::try_from(MAX_LOCK_FILE_BYTES).unwrap_or(u64::MAX) {
        return Err(invalid_lock_failure(
            LOCK_FILE_NAME,
            0..0,
            "lock_too_large",
            None,
        ));
    }
    let limit = u64::try_from(MAX_LOCK_FILE_BYTES)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|error| lock_io_failure("read", error.kind()))?
        .take(limit)
        .read_to_end(&mut bytes)
        .map_err(|error| lock_io_failure("read", error.kind()))?;
    if bytes.len() > MAX_LOCK_FILE_BYTES {
        return Err(invalid_lock_failure(
            LOCK_FILE_NAME,
            0..0,
            "lock_too_large",
            None,
        ));
    }
    Ok(Some(bytes))
}

fn persist_lock(project_root: &Path, bytes: &[u8]) -> Result<(), LockFileFailure> {
    let target = project_root.join(LOCK_FILE_NAME);
    let (temporary, mut file) = create_temporary_lock(project_root)
        .map_err(|error| lock_io_failure("write", error.kind()))?;
    let write_result = file.write_all(bytes).and_then(|()| file.sync_all());
    drop(file);
    if let Err(error) = write_result {
        let _ = fs::remove_file(&temporary);
        return Err(lock_io_failure("write", error.kind()));
    }
    if let Err(error) = commit_temporary_lock(&temporary, &target) {
        let _ = fs::remove_file(&temporary);
        return Err(lock_io_failure("replace", error.kind()));
    }
    Ok(())
}

fn create_temporary_lock(project_root: &Path) -> io::Result<(PathBuf, File)> {
    for _ in 0..MAX_TEMPORARY_FILE_ATTEMPTS {
        let sequence = TEMPORARY_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = project_root.join(format!(".ling.lock.tmp.{sequence}"));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "cannot reserve an adjacent temporary lock file",
    ))
}

fn commit_temporary_lock(temporary: &Path, target: &Path) -> io::Result<()> {
    fs::rename(temporary, target)
}

fn invalid_lock_failure(
    source_name: &str,
    span: std::ops::Range<u32>,
    reason: &'static str,
    package: Option<&str>,
) -> LockFileFailure {
    let mut diagnostic = Diagnostic::new(
        codes::INVALID_PROJECT_LOCK,
        Severity::Error,
        "工程锁文件无效",
        "project lockfile is invalid",
    )
    .with_fact("reason", reason)
    .with_primary_span(DiagnosticSpan::at(source_name, span.start, span.end));
    if let Some(package) = package {
        diagnostic = diagnostic.with_fact("package", package);
    }
    LockFileFailure {
        diagnostics: vec![diagnostic].into_boxed_slice(),
    }
}

fn lock_mismatch_failure(reason: &'static str) -> LockFileFailure {
    LockFileFailure {
        diagnostics: vec![
            Diagnostic::new(
                codes::PROJECT_LOCK_MISMATCH,
                Severity::Error,
                "工程锁文件与工程依赖图不匹配",
                "project lockfile does not match the project dependency graph",
            )
            .with_fact("reason", reason)
            .with_primary_span(DiagnosticSpan::at(LOCK_FILE_NAME, 0, 0)),
        ]
        .into_boxed_slice(),
    }
}

fn lock_io_failure(operation: &'static str, kind: io::ErrorKind) -> LockFileFailure {
    LockFileFailure {
        diagnostics: vec![
            Diagnostic::new(
                codes::PROJECT_LOCK_IO_FAILED,
                Severity::Error,
                "工程锁文件操作失败",
                "project lockfile operation failed",
            )
            .with_fact("io_kind", stable_io_kind(kind))
            .with_fact("operation", operation)
            .with_primary_span(DiagnosticSpan::at(LOCK_FILE_NAME, 0, 0)),
        ]
        .into_boxed_slice(),
    }
}

fn full_span(bytes: &[u8]) -> std::ops::Range<u32> {
    0..u32::try_from(bytes.len()).unwrap_or(u32::MAX)
}

fn json_error_span(bytes: &[u8], error: &serde_json::Error) -> std::ops::Range<u32> {
    let target_line = error.line().max(1);
    let target_column = error.column().max(1);
    let mut line = 1_usize;
    let mut column = 1_usize;
    let mut offset = 0_usize;
    while offset < bytes.len() && (line < target_line || column < target_column) {
        if bytes[offset] == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
        offset += 1;
    }
    let start = u32::try_from(offset).unwrap_or(u32::MAX);
    let end =
        u32::try_from(offset.saturating_add(usize::from(offset < bytes.len()))).unwrap_or(u32::MAX);
    start..end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_ids_are_exact_lowercase_sha256_text() {
        assert!(parse_content_id(&format!("sha256:{}", "0".repeat(64))).is_ok());
        for invalid in [
            format!("sha256:{}", "0".repeat(63)),
            format!("sha256:{}", "0".repeat(65)),
            format!("sha256:{}A", "0".repeat(63)),
            format!("blake3:{}", "0".repeat(64)),
        ] {
            assert_eq!(
                parse_content_id(&invalid).unwrap_err(),
                "invalid_content_id"
            );
        }
    }

    #[test]
    fn dependencies_sort_by_name_before_content() {
        let mut dependencies = [
            LockedDependency {
                name: validate_package_name("zeta").unwrap(),
                content: parse_content_id(&format!("sha256:{}", "0".repeat(64))).unwrap(),
            },
            LockedDependency {
                name: validate_package_name("alpha").unwrap(),
                content: parse_content_id(&format!("sha256:{}", "f".repeat(64))).unwrap(),
            },
        ];
        dependencies.sort();
        assert_eq!(
            dependencies.map(|dependency| dependency.name.as_str().to_owned()),
            ["alpha", "zeta"]
        );
    }

    #[test]
    fn malformed_bytes_return_bounded_diagnostics_without_panicking() {
        for bytes in [b"".as_slice(), b"{".as_slice(), b"\xff".as_slice()] {
            let failure = parse_lock_file(LOCK_FILE_NAME, bytes).unwrap_err();
            let span = failure.diagnostic().primary_span().unwrap();
            assert_eq!(span.file(), LOCK_FILE_NAME);
            assert!(span.start_byte() <= span.end_byte());
            assert!(span.end_byte() <= u64::try_from(bytes.len()).unwrap());
        }
    }
}
