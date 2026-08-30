use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::ops::Range;
use std::path::{Path, PathBuf};

use ling_ast::{Item, Program};
use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_source::{SourceError, SourceFile, SourceId, Span};
use unicode_normalization::UnicodeNormalization;

use super::{
    LogicalPath, MANIFEST_FILE_NAME, Manifest, PackageName, QualifiedModuleName, diagnostic_fact,
    project_resource_limit_diagnostic, validate_logical_path, validate_module_name,
};

const IMPLICIT_ENTRY_MODULE: &str = "Main";
const MAX_DISCOVERED_PATHS: usize = 262_144;
const MAX_PACKAGE_SOURCE_FILES: usize = 65_536;
const MAX_PACKAGE_SOURCE_BYTES: usize = 256 * 1024 * 1024;

/// A deterministic, validated module graph for one manifest package.
///
/// Physical host paths are deliberately absent. A node is identified only by
/// its package-local module name and manifest-relative logical source path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleGraph {
    package: PackageName,
    entry: QualifiedModuleName,
    dependencies: Box<[PackageName]>,
    nodes: Box<[ModuleNode]>,
    edges: Box<[ModuleEdge]>,
}

impl ModuleGraph {
    #[must_use]
    pub const fn package(&self) -> &PackageName {
        &self.package
    }

    #[must_use]
    pub const fn entry(&self) -> &QualifiedModuleName {
        &self.entry
    }

    #[must_use]
    pub fn dependencies(&self) -> &[PackageName] {
        &self.dependencies
    }

    #[must_use]
    pub fn dependency_name(&self, name: &str) -> Option<&PackageName> {
        self.dependencies
            .binary_search_by(|candidate| candidate.as_str().cmp(name))
            .ok()
            .map(|index| &self.dependencies[index])
    }

    #[must_use]
    pub fn nodes(&self) -> &[ModuleNode] {
        &self.nodes
    }

    #[must_use]
    pub fn edges(&self) -> &[ModuleEdge] {
        &self.edges
    }

    #[must_use]
    pub fn node(&self, name: &QualifiedModuleName) -> Option<&ModuleNode> {
        self.nodes
            .binary_search_by(|candidate| candidate.name.cmp(name))
            .ok()
            .map(|index| &self.nodes[index])
    }

    #[must_use]
    pub fn node_by_name(&self, name: &str) -> Option<&ModuleNode> {
        self.nodes
            .binary_search_by(|candidate| candidate.name.as_str().cmp(name))
            .ok()
            .map(|index| &self.nodes[index])
    }
}

/// One discovered `.ling` file and its deterministic path-to-module mapping.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleNode {
    name: QualifiedModuleName,
    source_root: LogicalPath,
    relative_path: LogicalPath,
    logical_path: LogicalPath,
    declaration_span: Option<Range<u32>>,
}

/// Exact, path-free source snapshot retained for later compiler stages.
///
/// The logical coordinates are part of the validated module graph. The byte
/// payload is the same snapshot used by package-content identity and module
/// discovery; no physical host path is retained or exposed.
#[derive(Clone, Eq, PartialEq)]
pub struct PackageSource {
    module: QualifiedModuleName,
    source_root: LogicalPath,
    relative_path: LogicalPath,
    logical_path: LogicalPath,
    bytes: Box<[u8]>,
}

impl PackageSource {
    #[must_use]
    pub const fn module(&self) -> &QualifiedModuleName {
        &self.module
    }

    #[must_use]
    pub const fn source_root(&self) -> &LogicalPath {
        &self.source_root
    }

    #[must_use]
    pub const fn relative_path(&self) -> &LogicalPath {
        &self.relative_path
    }

    #[must_use]
    pub const fn logical_path(&self) -> &LogicalPath {
        &self.logical_path
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Debug for PackageSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PackageSource")
            .field("module", &self.module)
            .field("source_root", &self.source_root)
            .field("relative_path", &self.relative_path)
            .field("logical_path", &self.logical_path)
            .field("byte_len", &self.bytes.len())
            .finish()
    }
}

impl ModuleNode {
    #[must_use]
    pub const fn name(&self) -> &QualifiedModuleName {
        &self.name
    }

    #[must_use]
    pub const fn source_root(&self) -> &LogicalPath {
        &self.source_root
    }

    #[must_use]
    pub const fn relative_path(&self) -> &LogicalPath {
        &self.relative_path
    }

    #[must_use]
    pub const fn logical_path(&self) -> &LogicalPath {
        &self.logical_path
    }

    #[must_use]
    pub fn declaration_span(&self) -> Option<Range<u32>> {
        self.declaration_span.clone()
    }
}

/// A sorted import edge emitted from the source declarations of one module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleEdge {
    from: QualifiedModuleName,
    target: ImportTarget,
    source: LogicalPath,
    span: Range<u32>,
}

impl ModuleEdge {
    #[must_use]
    pub const fn from(&self) -> &QualifiedModuleName {
        &self.from
    }

    #[must_use]
    pub const fn target(&self) -> &ImportTarget {
        &self.target
    }

    #[must_use]
    pub const fn source(&self) -> &LogicalPath {
        &self.source
    }

    #[must_use]
    pub fn span(&self) -> Range<u32> {
        self.span.clone()
    }

    #[must_use]
    pub const fn dependency_module(&self) -> Option<&QualifiedModuleName> {
        match &self.target {
            ImportTarget::Dependency { module, .. } => Some(module),
            ImportTarget::Local(_) => None,
        }
    }
}

/// The namespace selected by a source import.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ImportTarget {
    Local(QualifiedModuleName),
    Dependency {
        package: PackageName,
        module: QualifiedModuleName,
    },
}

/// Atomic failure from module discovery.
///
/// User-controlled failures are a deterministically ordered diagnostic set.
/// `Internal` is reserved for an invariant failure between a valid CST and AST.
#[derive(Debug)]
pub enum DiscoveryFailure {
    Diagnostics(Box<[Diagnostic]>),
    Internal(String),
}

impl DiscoveryFailure {
    #[must_use]
    pub fn diagnostics(&self) -> Option<&[Diagnostic]> {
        match self {
            Self::Diagnostics(diagnostics) => Some(diagnostics),
            Self::Internal(_) => None,
        }
    }
}

impl fmt::Display for DiscoveryFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Diagnostics(diagnostics) => {
                write!(
                    formatter,
                    "module discovery produced {} diagnostic(s)",
                    diagnostics.len()
                )
            }
            Self::Internal(message) => formatter.write_str(message),
        }
    }
}

impl Error for DiscoveryFailure {}

/// Discovers and validates every `.ling` module declared by `manifest`.
///
/// Discovery performs no parent search, dependency traversal, network access,
/// lockfile operation, or CLI selection. All filesystem reads stay beneath the
/// explicitly supplied project root after symlink-aware canonicalization.
pub fn discover_modules(
    project_root: &Path,
    manifest: &Manifest,
) -> Result<ModuleGraph, DiscoveryFailure> {
    let package = prepare_package(project_root, manifest)?;
    analyze_package(manifest, package).map(|analyzed| analyzed.graph)
}

pub(crate) fn prepare_package(
    project_root: &Path,
    manifest: &Manifest,
) -> Result<PreparedPackage, DiscoveryFailure> {
    let canonical_project_root = match canonical_project_root(project_root, manifest) {
        Ok(root) => root,
        Err(error) => return Err(diagnostics_failure(vec![error])),
    };

    let mut errors = Vec::new();
    let mut roots = Vec::with_capacity(manifest.source().roots().len());
    for logical in manifest.source().roots() {
        match resolve_source_root(project_root, &canonical_project_root, manifest, logical) {
            Ok(root) => roots.push(root),
            Err(error) => errors.push(error),
        }
    }
    if !errors.is_empty() {
        return Err(diagnostics_failure(errors));
    }

    validate_resolved_root_overlap(manifest, &roots, &mut errors);
    if !errors.is_empty() {
        return Err(diagnostics_failure(errors));
    }

    let mut candidates = Vec::new();
    let mut discovered_paths = 0;
    for root in &roots {
        if !discover_root(
            &canonical_project_root,
            manifest,
            root,
            &mut discovered_paths,
            &mut candidates,
            &mut errors,
        ) {
            break;
        }
    }
    if !errors.is_empty() {
        return Err(diagnostics_failure(errors));
    }
    candidates.sort_by(|left, right| {
        (left.root.as_str(), left.relative.as_str())
            .cmp(&(right.root.as_str(), right.relative.as_str()))
    });

    let mut by_module = BTreeMap::<QualifiedModuleName, usize>::new();
    for (index, candidate) in candidates.iter().enumerate() {
        if let Some(previous) = by_module.get(&candidate.module).copied() {
            errors.push(duplicate_module_diagnostic(
                &candidates[previous],
                candidate,
            ));
        } else {
            by_module.insert(candidate.module.clone(), index);
        }
    }
    if !errors.is_empty() {
        return Err(diagnostics_failure(errors));
    }

    validate_required_modules(manifest, &by_module, &mut errors);
    validate_dependency_namespaces(manifest, &candidates, &mut errors);
    if !errors.is_empty() {
        return Err(diagnostics_failure(errors));
    }

    if candidates.len() > MAX_PACKAGE_SOURCE_FILES {
        let candidate = &candidates[MAX_PACKAGE_SOURCE_FILES];
        errors.push(resource_limit_diagnostic(
            manifest,
            candidate.logical.as_str(),
            "source_files",
            MAX_PACKAGE_SOURCE_FILES,
            candidates.len(),
        ));
        return Err(diagnostics_failure(errors));
    }

    let mut modules = Vec::with_capacity(candidates.len());
    let mut source_bytes = 0;
    for candidate in candidates {
        match read_candidate(candidate, manifest, source_bytes) {
            Ok(module) => {
                source_bytes = source_bytes.saturating_add(module.bytes.len());
                modules.push(module);
            }
            Err(CandidateFailure::Diagnostics(mut diagnostics)) => {
                errors.append(&mut diagnostics);
            }
            Err(CandidateFailure::Internal(message)) => {
                return Err(DiscoveryFailure::Internal(message));
            }
        }
    }
    if !errors.is_empty() {
        return Err(diagnostics_failure(errors));
    }

    Ok(PreparedPackage {
        modules: modules.into_boxed_slice(),
    })
}

pub(crate) fn analyze_package(
    manifest: &Manifest,
    package: PreparedPackage,
) -> Result<AnalyzedPackage, DiscoveryFailure> {
    let mut errors = Vec::new();
    let mut parsed = Vec::with_capacity(package.modules.len());
    for module in package.modules.into_vec() {
        match parse_candidate(module, manifest) {
            Ok(module) => parsed.push(module),
            Err(CandidateFailure::Diagnostics(mut diagnostics)) => {
                errors.append(&mut diagnostics);
            }
            Err(CandidateFailure::Internal(message)) => {
                return Err(DiscoveryFailure::Internal(message));
            }
        }
    }
    if !errors.is_empty() {
        return Err(diagnostics_failure(errors));
    }

    let node_names = parsed
        .iter()
        .map(|module| module.node.name.clone())
        .collect::<BTreeSet<_>>();
    let mut edges = resolve_edges(manifest, &parsed, &node_names, &mut errors);
    if !errors.is_empty() {
        return Err(diagnostics_failure(errors));
    }

    if let Some(error) = import_cycle_diagnostic(&edges, &node_names) {
        return Err(diagnostics_failure(vec![error]));
    }

    parsed.sort_by(|left, right| left.node.name.cmp(&right.node.name));
    let mut nodes = Vec::with_capacity(parsed.len());
    let mut sources = Vec::with_capacity(parsed.len());
    for module in parsed {
        nodes.push(module.node);
        sources.push(module.source);
    }
    nodes.sort_by(|left, right| left.name.cmp(&right.name));
    edges.sort_by(|left, right| {
        (&left.from, &left.target, left.span.start, left.span.end).cmp(&(
            &right.from,
            &right.target,
            right.span.start,
            right.span.end,
        ))
    });

    Ok(AnalyzedPackage {
        graph: ModuleGraph {
            package: manifest.package().name().clone(),
            entry: manifest.source().entry().clone(),
            dependencies: manifest
                .dependencies()
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            nodes: nodes.into_boxed_slice(),
            edges: edges.into_boxed_slice(),
        },
        sources: sources.into_boxed_slice(),
    })
}

pub(crate) struct AnalyzedPackage {
    pub(crate) graph: ModuleGraph,
    pub(crate) sources: Box<[PackageSource]>,
}

#[derive(Debug)]
pub(crate) struct PreparedPackage {
    modules: Box<[PreparedModule]>,
}

impl PreparedPackage {
    pub(crate) fn sources(
        &self,
    ) -> impl ExactSizeIterator<Item = (&LogicalPath, &LogicalPath, &[u8])> {
        self.modules.iter().map(|module| {
            (
                &module.candidate.root,
                &module.candidate.relative,
                module.bytes.as_ref(),
            )
        })
    }

    pub(crate) fn source_byte_len(&self) -> usize {
        self.modules.iter().map(|module| module.bytes.len()).sum()
    }
}

#[derive(Debug)]
struct PreparedModule {
    candidate: Candidate,
    bytes: Box<[u8]>,
}

#[derive(Debug)]
struct PendingDiagnostic {
    logical_path: String,
    diagnostic: Box<Diagnostic>,
}

impl PendingDiagnostic {
    fn new(logical_path: impl Into<String>, diagnostic: Diagnostic) -> Self {
        Self {
            logical_path: logical_path.into(),
            diagnostic: Box::new(diagnostic),
        }
    }
}

#[derive(Debug)]
struct ResolvedRoot {
    logical: LogicalPath,
    physical: PathBuf,
    canonical: PathBuf,
}

#[derive(Debug)]
struct Candidate {
    root: LogicalPath,
    relative: LogicalPath,
    logical: LogicalPath,
    module: QualifiedModuleName,
    physical: PathBuf,
}

#[derive(Debug)]
struct ParsedModule {
    node: ModuleNode,
    imports: Vec<RawImport>,
    source: PackageSource,
}

#[derive(Debug)]
struct RawImport {
    from: QualifiedModuleName,
    target: QualifiedModuleName,
    source: LogicalPath,
    span: Range<u32>,
}

enum CandidateFailure {
    Diagnostics(Vec<PendingDiagnostic>),
    Internal(String),
}

enum WalkFrame {
    Enter {
        physical: PathBuf,
        relative: Vec<String>,
    },
    Exit(PathBuf),
}

fn canonical_project_root(
    project_root: &Path,
    manifest: &Manifest,
) -> Result<PathBuf, PendingDiagnostic> {
    let canonical = fs::canonicalize(project_root).map_err(|error| {
        source_root_diagnostic(
            manifest,
            None,
            ".",
            if error.kind() == io::ErrorKind::NotFound {
                "project_root_missing"
            } else {
                "project_root_unavailable"
            },
        )
    })?;
    let metadata = fs::metadata(&canonical).map_err(|error| {
        io_diagnostic(
            ".",
            &canonical,
            error.kind(),
            "读取工程根目录失败",
            "failed to read the project root",
        )
    })?;
    if !metadata.is_dir() {
        return Err(source_root_diagnostic(
            manifest,
            None,
            ".",
            "project_root_not_directory",
        ));
    }
    Ok(canonical)
}

fn resolve_source_root(
    project_root: &Path,
    canonical_project_root: &Path,
    manifest: &Manifest,
    logical: &LogicalPath,
) -> Result<ResolvedRoot, PendingDiagnostic> {
    let mut current = project_root.to_path_buf();
    for component in logical.as_str().split('/') {
        let entries = fs::read_dir(&current).map_err(|error| {
            io_diagnostic(
                logical.as_str(),
                &current,
                error.kind(),
                "读取源码根目录路径失败",
                "failed to read a source-root path",
            )
        })?;
        let mut exact = None;
        let mut ascii_case_match = false;
        for entry in entries {
            let entry = entry.map_err(|error| {
                io_diagnostic(
                    logical.as_str(),
                    &current,
                    error.kind(),
                    "读取源码根目录项失败",
                    "failed to read a source-root directory entry",
                )
            })?;
            let name = entry.file_name();
            if name == OsStr::new(component) {
                exact = Some(entry.path());
                break;
            }
            if name
                .to_str()
                .is_some_and(|name| name.eq_ignore_ascii_case(component))
            {
                ascii_case_match = true;
            }
        }
        let Some(next) = exact else {
            return Err(source_root_diagnostic(
                manifest,
                Some(logical),
                logical.as_str(),
                if ascii_case_match {
                    "case_mismatch"
                } else {
                    "missing"
                },
            ));
        };
        let canonical = fs::canonicalize(&next).map_err(|error| {
            source_root_diagnostic(
                manifest,
                Some(logical),
                logical.as_str(),
                if error.kind() == io::ErrorKind::NotFound {
                    "dangling_symlink"
                } else {
                    "unresolvable"
                },
            )
        })?;
        if !canonical.starts_with(canonical_project_root) {
            return Err(source_root_diagnostic(
                manifest,
                Some(logical),
                logical.as_str(),
                "escapes_project",
            ));
        }
        current = next;
    }

    let metadata = fs::metadata(&current).map_err(|error| {
        io_diagnostic(
            logical.as_str(),
            &current,
            error.kind(),
            "读取源码根目录失败",
            "failed to read the source root",
        )
    })?;
    if !metadata.is_dir() {
        return Err(source_root_diagnostic(
            manifest,
            Some(logical),
            logical.as_str(),
            "not_directory",
        ));
    }
    let canonical = fs::canonicalize(&current).map_err(|error| {
        io_diagnostic(
            logical.as_str(),
            &current,
            error.kind(),
            "解析源码根目录失败",
            "failed to resolve the source root",
        )
    })?;
    Ok(ResolvedRoot {
        logical: logical.clone(),
        physical: current,
        canonical,
    })
}

fn validate_resolved_root_overlap(
    manifest: &Manifest,
    roots: &[ResolvedRoot],
    errors: &mut Vec<PendingDiagnostic>,
) {
    for (index, root) in roots.iter().enumerate() {
        if roots[..index].iter().any(|previous| {
            root.canonical == previous.canonical
                || root.canonical.starts_with(&previous.canonical)
                || previous.canonical.starts_with(&root.canonical)
        }) {
            errors.push(source_root_diagnostic(
                manifest,
                Some(&root.logical),
                root.logical.as_str(),
                "resolved_overlap",
            ));
        }
    }
}

fn discover_root(
    canonical_project_root: &Path,
    manifest: &Manifest,
    root: &ResolvedRoot,
    discovered_paths: &mut usize,
    candidates: &mut Vec<Candidate>,
    errors: &mut Vec<PendingDiagnostic>,
) -> bool {
    let mut active = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut stack = vec![WalkFrame::Enter {
        physical: root.physical.clone(),
        relative: Vec::new(),
    }];

    while let Some(frame) = stack.pop() {
        match frame {
            WalkFrame::Exit(canonical) => {
                active.remove(&canonical);
            }
            WalkFrame::Enter { physical, relative } => {
                let canonical = match fs::canonicalize(&physical) {
                    Ok(path) => path,
                    Err(error) => {
                        errors.push(io_diagnostic(
                            &logical_walk_path(&root.logical, &relative),
                            &physical,
                            error.kind(),
                            "解析源码目录失败",
                            "failed to resolve a source directory",
                        ));
                        continue;
                    }
                };
                if !canonical.starts_with(canonical_project_root) {
                    errors.push(source_path_diagnostic(
                        &logical_walk_path(&root.logical, &relative),
                        "symlink_escape",
                    ));
                    continue;
                }
                if active.contains(&canonical) {
                    errors.push(source_path_diagnostic(
                        &logical_walk_path(&root.logical, &relative),
                        "symlink_cycle",
                    ));
                    continue;
                }
                if !seen.insert(canonical.clone()) {
                    errors.push(source_path_diagnostic(
                        &logical_walk_path(&root.logical, &relative),
                        "symlink_alias",
                    ));
                    continue;
                }
                active.insert(canonical.clone());
                stack.push(WalkFrame::Exit(canonical));

                let entries = match fs::read_dir(&physical) {
                    Ok(entries) => entries,
                    Err(error) => {
                        errors.push(io_diagnostic(
                            &logical_walk_path(&root.logical, &relative),
                            &physical,
                            error.kind(),
                            "读取源码目录失败",
                            "failed to read a source directory",
                        ));
                        continue;
                    }
                };
                let mut children = Vec::new();
                let mut non_utf8 = false;
                for entry in entries {
                    if *discovered_paths >= MAX_DISCOVERED_PATHS {
                        let logical = logical_walk_path(&root.logical, &relative);
                        errors.push(resource_limit_diagnostic(
                            manifest,
                            &logical,
                            "discovered_paths",
                            MAX_DISCOVERED_PATHS,
                            (*discovered_paths).saturating_add(1),
                        ));
                        return false;
                    }
                    *discovered_paths = (*discovered_paths).saturating_add(1);
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(error) => {
                            errors.push(io_diagnostic(
                                &logical_walk_path(&root.logical, &relative),
                                &physical,
                                error.kind(),
                                "读取源码目录项失败",
                                "failed to read a source directory entry",
                            ));
                            continue;
                        }
                    };
                    let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
                        non_utf8 = true;
                        continue;
                    };
                    children.push((name, entry.path()));
                }
                if non_utf8 {
                    errors.push(source_path_diagnostic(
                        &format!("{}/<non-utf8>", logical_walk_path(&root.logical, &relative)),
                        "non_utf8_component",
                    ));
                    continue;
                }
                children.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));

                for (name, path) in children.into_iter().rev() {
                    let mut child_relative = relative.clone();
                    child_relative.push(name.clone());
                    let metadata = match fs::metadata(&path) {
                        Ok(metadata) => metadata,
                        Err(error) => {
                            let symlink = fs::symlink_metadata(&path)
                                .ok()
                                .is_some_and(|metadata| metadata.file_type().is_symlink());
                            if symlink && error.kind() == io::ErrorKind::NotFound {
                                errors.push(source_path_diagnostic(
                                    &logical_walk_path(&root.logical, &child_relative),
                                    "dangling_symlink",
                                ));
                            } else {
                                errors.push(io_diagnostic(
                                    &logical_walk_path(&root.logical, &child_relative),
                                    &path,
                                    error.kind(),
                                    "读取源码路径失败",
                                    "failed to read a source path",
                                ));
                            }
                            continue;
                        }
                    };
                    if metadata.is_dir() {
                        stack.push(WalkFrame::Enter {
                            physical: path,
                            relative: child_relative,
                        });
                    } else if metadata.is_file() && name.ends_with(".ling") {
                        let canonical = match fs::canonicalize(&path) {
                            Ok(path) => path,
                            Err(error) => {
                                errors.push(io_diagnostic(
                                    &logical_walk_path(&root.logical, &child_relative),
                                    &path,
                                    error.kind(),
                                    "解析源码文件失败",
                                    "failed to resolve a source file",
                                ));
                                continue;
                            }
                        };
                        if !canonical.starts_with(canonical_project_root) {
                            errors.push(source_path_diagnostic(
                                &logical_walk_path(&root.logical, &child_relative),
                                "symlink_escape",
                            ));
                            continue;
                        }
                        match candidate_from_path(root, &child_relative, path) {
                            Ok(candidate) => candidates.push(candidate),
                            Err(error) => errors.push(error),
                        }
                    } else if metadata.is_file()
                        && name
                            .as_bytes()
                            .get(name.len().saturating_sub(".ling".len())..)
                            .is_some_and(|suffix| suffix.eq_ignore_ascii_case(b".ling"))
                    {
                        errors.push(source_path_diagnostic(
                            &logical_walk_path(&root.logical, &child_relative),
                            "extension_case_mismatch",
                        ));
                    } else if name.ends_with(".ling") {
                        errors.push(source_path_diagnostic(
                            &logical_walk_path(&root.logical, &child_relative),
                            "not_regular_file",
                        ));
                    }
                }
            }
        }
    }
    true
}

fn candidate_from_path(
    root: &ResolvedRoot,
    components: &[String],
    physical: PathBuf,
) -> Result<Candidate, PendingDiagnostic> {
    let relative_raw = components.join("/");
    let logical_raw = format!("{}/{}", root.logical.as_str(), relative_raw);
    let invalid = |reason| source_path_diagnostic(&logical_raw, reason);
    let Some(file_name) = components.last() else {
        return Err(invalid("empty_relative_path"));
    };
    let Some(stem) = file_name.strip_suffix(".ling") else {
        return Err(invalid("invalid_extension"));
    };
    if stem.is_empty() {
        return Err(invalid("empty_module_segment"));
    }

    let mut module_segments = components[..components.len() - 1].to_vec();
    module_segments.push(stem.to_owned());
    for segment in &module_segments {
        if segment.nfc().ne(segment.chars()) {
            return Err(invalid("non_nfc_component"));
        }
        if ling_unicode::validate_identifier(segment).is_err() {
            return Err(invalid("invalid_module_segment"));
        }
    }

    let relative =
        validate_logical_path(&relative_raw).map_err(|reason| invalid(reason.as_str()))?;
    let logical = validate_logical_path(&logical_raw).map_err(|reason| invalid(reason.as_str()))?;
    let module = validate_module_name(&module_segments.join("."))
        .map_err(|reason| invalid(reason.as_str()))?;
    Ok(Candidate {
        root: root.logical.clone(),
        relative,
        logical,
        module,
        physical,
    })
}

fn validate_required_modules(
    manifest: &Manifest,
    by_module: &BTreeMap<QualifiedModuleName, usize>,
    errors: &mut Vec<PendingDiagnostic>,
) {
    let entry = manifest.source().entry();
    if !by_module.contains_key(entry) {
        errors.push(missing_manifest_module_diagnostic(manifest, entry, "entry"));
    }
    for exported in manifest.exports() {
        if !by_module.contains_key(exported) {
            errors.push(missing_manifest_module_diagnostic(
                manifest, exported, "export",
            ));
        }
    }
}

fn validate_dependency_namespaces(
    manifest: &Manifest,
    candidates: &[Candidate],
    errors: &mut Vec<PendingDiagnostic>,
) {
    for candidate in candidates {
        let first = candidate
            .module
            .as_str()
            .split('.')
            .next()
            .expect("validated module names are nonempty");
        if manifest
            .dependencies()
            .keys()
            .any(|dependency| dependency.as_str() == first)
        {
            errors.push(import_graph_diagnostic(
                candidate.logical.as_str(),
                None,
                "dependency_namespace_collision",
                Some(candidate.module.as_str()),
                None,
                None,
            ));
        }
    }
}

fn read_candidate(
    candidate: Candidate,
    manifest: &Manifest,
    consumed_bytes: usize,
) -> Result<PreparedModule, CandidateFailure> {
    let metadata = fs::metadata(&candidate.physical).map_err(|error| {
        CandidateFailure::Diagnostics(vec![io_diagnostic(
            candidate.logical.as_str(),
            &candidate.physical,
            error.kind(),
            "读取源码文件失败",
            "failed to read a source file",
        )])
    })?;
    let remaining_bytes = MAX_PACKAGE_SOURCE_BYTES.saturating_sub(consumed_bytes);
    let declared_len = usize::try_from(metadata.len()).unwrap_or(usize::MAX);
    if declared_len > remaining_bytes {
        return Err(CandidateFailure::Diagnostics(vec![
            resource_limit_diagnostic(
                manifest,
                candidate.logical.as_str(),
                "source_bytes",
                MAX_PACKAGE_SOURCE_BYTES,
                consumed_bytes.saturating_add(declared_len),
            ),
        ]));
    }
    let file = fs::File::open(&candidate.physical).map_err(|error| {
        CandidateFailure::Diagnostics(vec![io_diagnostic(
            candidate.logical.as_str(),
            &candidate.physical,
            error.kind(),
            "读取源码文件失败",
            "failed to read a source file",
        )])
    })?;
    let limit = u64::try_from(remaining_bytes)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut bytes = Vec::with_capacity(declared_len.min(64 * 1024));
    file.take(limit).read_to_end(&mut bytes).map_err(|error| {
        CandidateFailure::Diagnostics(vec![io_diagnostic(
            candidate.logical.as_str(),
            &candidate.physical,
            error.kind(),
            "读取源码文件失败",
            "failed to read a source file",
        )])
    })?;
    if bytes.len() > remaining_bytes {
        return Err(CandidateFailure::Diagnostics(vec![
            resource_limit_diagnostic(
                manifest,
                candidate.logical.as_str(),
                "source_bytes",
                MAX_PACKAGE_SOURCE_BYTES,
                consumed_bytes.saturating_add(bytes.len()),
            ),
        ]));
    }
    Ok(PreparedModule {
        candidate,
        bytes: bytes.into_boxed_slice(),
    })
}

fn resource_limit_diagnostic(
    manifest: &Manifest,
    logical_path: &str,
    resource: &'static str,
    maximum: usize,
    actual: usize,
) -> PendingDiagnostic {
    PendingDiagnostic::new(
        logical_path,
        project_resource_limit_diagnostic(
            logical_path,
            None,
            manifest.package().name(),
            resource,
            maximum,
            actual,
        ),
    )
}

fn parse_candidate(
    module: PreparedModule,
    manifest: &Manifest,
) -> Result<ParsedModule, CandidateFailure> {
    let PreparedModule { candidate, bytes } = module;
    let source = SourceFile::from_bytes(
        SourceId::new(0),
        candidate.logical.as_str().to_owned(),
        bytes.into_vec(),
    )
    .map_err(|error| {
        CandidateFailure::Diagnostics(vec![PendingDiagnostic::new(
            candidate.logical.as_str(),
            source_error_diagnostic(candidate.logical.as_str(), error),
        )])
    })?;
    let parsed = ling_syntax::parse(&source);
    if !parsed.lexical_errors().is_empty() || !parsed.parse_errors().is_empty() {
        let mut diagnostics = parsed
            .lexical_errors()
            .iter()
            .map(|error| {
                PendingDiagnostic::new(
                    candidate.logical.as_str(),
                    error.to_diagnostic(source.name()),
                )
            })
            .chain(parsed.parse_errors().iter().map(|error| {
                PendingDiagnostic::new(
                    candidate.logical.as_str(),
                    error.to_diagnostic(source.name()),
                )
            }))
            .collect::<Vec<_>>();
        sort_pending_diagnostics(&mut diagnostics);
        return Err(CandidateFailure::Diagnostics(diagnostics));
    }
    let program = ling_ast::lower(&source, &parsed).map_err(|error| {
        CandidateFailure::Internal(format!(
            "valid source could not be lowered while discovering modules: {error}"
        ))
    })?;
    let (node, imports) = validate_program(&candidate, manifest, &program)?;
    let retained_source = PackageSource {
        module: candidate.module,
        source_root: candidate.root,
        relative_path: candidate.relative,
        logical_path: candidate.logical,
        bytes: source.into_original_bytes().into_boxed_slice(),
    };
    Ok(ParsedModule {
        node,
        imports,
        source: retained_source,
    })
}

fn validate_program(
    candidate: &Candidate,
    manifest: &Manifest,
    program: &Program,
) -> Result<(ModuleNode, Vec<RawImport>), CandidateFailure> {
    let modules = program
        .items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| match item {
            Item::Module(module) => Some((index, module)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let declaration_span;
    if modules.len() > 1 {
        let span = span_range(modules[1].1.span);
        return Err(CandidateFailure::Diagnostics(vec![
            module_declaration_diagnostic(candidate, span, "duplicate_declaration", None),
        ]));
    }
    if let Some((index, declaration)) = modules.first() {
        let span = span_range(declaration.span);
        if *index != 0 {
            return Err(CandidateFailure::Diagnostics(vec![
                module_declaration_diagnostic(candidate, span, "declaration_not_first", None),
            ]));
        }
        let actual = qualified_name(&declaration.name);
        if actual != candidate.module.as_str() {
            return Err(CandidateFailure::Diagnostics(vec![
                module_declaration_diagnostic(candidate, span, "path_mismatch", Some(&actual)),
            ]));
        }
        declaration_span = Some(span);
    } else if &candidate.module == manifest.source().entry()
        && candidate.module.as_str() == IMPLICIT_ENTRY_MODULE
    {
        declaration_span = None;
    } else {
        return Err(CandidateFailure::Diagnostics(vec![
            module_declaration_diagnostic(candidate, 0..0, "missing_declaration", None),
        ]));
    }

    let mut imports = Vec::new();
    let mut body_started = false;
    for item in &program.items {
        match item {
            Item::Module(_) => {}
            Item::Import(import) => {
                if body_started {
                    return Err(CandidateFailure::Diagnostics(vec![
                        import_graph_diagnostic(
                            candidate.logical.as_str(),
                            Some(span_range(import.span)),
                            "import_after_declaration",
                            Some(candidate.module.as_str()),
                            Some(&qualified_name(&import.module)),
                            None,
                        ),
                    ]));
                }
                imports.push(RawImport {
                    from: candidate.module.clone(),
                    target: validate_module_name(&qualified_name(&import.module))
                        .expect("AST qualified names satisfy module-name validation"),
                    source: candidate.logical.clone(),
                    span: span_range(import.span),
                });
            }
            Item::Let(_)
            | Item::Task(_)
            | Item::Actor(_)
            | Item::Type(_)
            | Item::Trait(_)
            | Item::Impl(_) => {
                body_started = true;
            }
        }
    }

    Ok((
        ModuleNode {
            name: candidate.module.clone(),
            source_root: candidate.root.clone(),
            relative_path: candidate.relative.clone(),
            logical_path: candidate.logical.clone(),
            declaration_span,
        },
        imports,
    ))
}

fn resolve_edges(
    manifest: &Manifest,
    modules: &[ParsedModule],
    node_names: &BTreeSet<QualifiedModuleName>,
    errors: &mut Vec<PendingDiagnostic>,
) -> Vec<ModuleEdge> {
    let dependencies = manifest
        .dependencies()
        .keys()
        .map(|name| (name.as_str(), name))
        .collect::<BTreeMap<_, _>>();
    let mut edges = BTreeMap::<(QualifiedModuleName, ImportTarget), ModuleEdge>::new();

    for import in modules.iter().flat_map(|module| &module.imports) {
        let mut segments = import.target.as_str().split('.');
        let first = segments
            .next()
            .expect("validated import names contain a segment");
        let target = if let Some(package) = dependencies.get(first) {
            let remaining = segments.collect::<Vec<_>>();
            if remaining.is_empty() {
                errors.push(import_graph_diagnostic(
                    import.source.as_str(),
                    Some(import.span.clone()),
                    "missing_dependency_module",
                    Some(import.from.as_str()),
                    Some(import.target.as_str()),
                    None,
                ));
                continue;
            }
            ImportTarget::Dependency {
                package: (*package).clone(),
                module: validate_module_name(&remaining.join("."))
                    .expect("a suffix of a validated qualified name is valid"),
            }
        } else if node_names.contains(&import.target) {
            ImportTarget::Local(import.target.clone())
        } else {
            errors.push(missing_import_diagnostic(import));
            continue;
        };
        let edge = ModuleEdge {
            from: import.from.clone(),
            target: target.clone(),
            source: import.source.clone(),
            span: import.span.clone(),
        };
        edges.entry((import.from.clone(), target)).or_insert(edge);
    }
    edges.into_values().collect()
}

fn import_cycle_diagnostic(
    edges: &[ModuleEdge],
    node_names: &BTreeSet<QualifiedModuleName>,
) -> Option<PendingDiagnostic> {
    let mut adjacency = node_names
        .iter()
        .map(|name| (name.as_str().to_owned(), Vec::new()))
        .collect::<BTreeMap<_, Vec<String>>>();
    let mut edge_locations = BTreeMap::new();
    for edge in edges {
        if let ImportTarget::Local(target) = &edge.target {
            adjacency
                .get_mut(edge.from.as_str())
                .expect("edge source is a discovered node")
                .push(target.as_str().to_owned());
            edge_locations.insert(
                (edge.from.as_str().to_owned(), target.as_str().to_owned()),
                (edge.source.clone(), edge.span.clone()),
            );
        }
    }
    for neighbors in adjacency.values_mut() {
        neighbors.sort();
        neighbors.dedup();
    }

    let mut states = adjacency
        .keys()
        .map(|name| (name.clone(), VisitState::Unvisited))
        .collect::<BTreeMap<_, _>>();
    for start in adjacency.keys() {
        if states[start] != VisitState::Unvisited {
            continue;
        }
        let mut stack = vec![(start.clone(), 0_usize)];
        let mut path = vec![start.clone()];
        let mut positions = BTreeMap::from([(start.clone(), 0_usize)]);
        states.insert(start.clone(), VisitState::Active);

        while let Some((current, next_index)) = stack.last_mut() {
            let neighbors = &adjacency[current];
            if *next_index >= neighbors.len() {
                let finished = current.clone();
                stack.pop();
                path.pop();
                positions.remove(&finished);
                states.insert(finished, VisitState::Done);
                continue;
            }
            let next = neighbors[*next_index].clone();
            *next_index += 1;
            match states[&next] {
                VisitState::Unvisited => {
                    positions.insert(next.clone(), path.len());
                    path.push(next.clone());
                    states.insert(next.clone(), VisitState::Active);
                    stack.push((next, 0));
                }
                VisitState::Active => {
                    let position = positions[&next];
                    let cycle = canonical_cycle(&path[position..]);
                    let (source, span) = edge_locations
                        .get(&(current.clone(), next.clone()))
                        .expect("local graph edge retains its source location");
                    return Some(import_graph_diagnostic(
                        source.as_str(),
                        Some(span.clone()),
                        "cycle",
                        Some(current),
                        Some(&next),
                        Some(&cycle),
                    ));
                }
                VisitState::Done => {}
            }
        }
    }
    None
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VisitState {
    Unvisited,
    Active,
    Done,
}

fn canonical_cycle(nodes: &[String]) -> String {
    let start = (0..nodes.len())
        .min_by(|left, right| {
            nodes[*left..]
                .iter()
                .chain(nodes[..*left].iter())
                .cmp(nodes[*right..].iter().chain(nodes[..*right].iter()))
        })
        .unwrap_or(0);
    let mut ordered = nodes[start..]
        .iter()
        .chain(nodes[..start].iter())
        .cloned()
        .collect::<Vec<_>>();
    if let Some(first) = ordered.first().cloned() {
        ordered.push(first);
    }
    ordered.join(" -> ")
}

fn qualified_name(name: &ling_ast::QualifiedName) -> String {
    name.segments
        .iter()
        .map(|segment| segment.normalized.as_str())
        .collect::<Vec<_>>()
        .join(".")
}

fn span_range(span: Span) -> Range<u32> {
    span.start().get()..span.end().get()
}

fn logical_walk_path(root: &LogicalPath, relative: &[String]) -> String {
    if relative.is_empty() {
        root.as_str().to_owned()
    } else {
        format!("{}/{}", root.as_str(), relative.join("/"))
    }
}

fn diagnostics_failure(mut errors: Vec<PendingDiagnostic>) -> DiscoveryFailure {
    sort_pending_diagnostics(&mut errors);
    DiscoveryFailure::Diagnostics(
        errors
            .into_iter()
            .map(|error| *error.diagnostic)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}

fn sort_pending_diagnostics(errors: &mut [PendingDiagnostic]) {
    errors.sort_by(|left, right| {
        let left_span = left.diagnostic.primary_span();
        let right_span = right.diagnostic.primary_span();
        (
            left.logical_path.as_str(),
            left_span.map_or(0, DiagnosticSpan::start_byte),
            left_span.map_or(0, DiagnosticSpan::end_byte),
            left.diagnostic.code().as_str(),
        )
            .cmp(&(
                right.logical_path.as_str(),
                right_span.map_or(0, DiagnosticSpan::start_byte),
                right_span.map_or(0, DiagnosticSpan::end_byte),
                right.diagnostic.code().as_str(),
            ))
    });
}

fn source_root_diagnostic(
    manifest: &Manifest,
    logical: Option<&LogicalPath>,
    root: &str,
    reason: &'static str,
) -> PendingDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::INVALID_PROJECT_SOURCE_ROOT,
        Severity::Error,
        "工程源码根目录无效",
        "project source root is invalid",
    )
    .with_fact("reason", reason)
    .with_fact("root", diagnostic_fact(root));
    if let Some(logical) = logical {
        if let Some(span) = manifest.locations.source_roots.get(logical) {
            diagnostic = diagnostic.with_primary_span(DiagnosticSpan::at(
                manifest.locations.source_name.clone(),
                span.start,
                span.end,
            ));
        }
    }
    PendingDiagnostic::new(MANIFEST_FILE_NAME, diagnostic)
}

fn source_path_diagnostic(source: &str, reason: &'static str) -> PendingDiagnostic {
    PendingDiagnostic::new(
        source,
        Diagnostic::new(
            codes::INVALID_PROJECT_SOURCE_PATH,
            Severity::Error,
            "工程源码路径无效",
            "project source path is invalid",
        )
        .with_fact("reason", reason)
        .with_fact("source", diagnostic_fact(source)),
    )
}

fn module_declaration_diagnostic(
    candidate: &Candidate,
    span: Range<u32>,
    reason: &'static str,
    actual: Option<&str>,
) -> PendingDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::INVALID_PROJECT_MODULE_DECLARATION,
        Severity::Error,
        "工程源码的 module 声明无效",
        "project source has an invalid module declaration",
    )
    .with_primary_span(DiagnosticSpan::at(
        candidate.logical.as_str(),
        span.start,
        span.end,
    ))
    .with_fact(
        "expected_module",
        diagnostic_fact(candidate.module.as_str()),
    )
    .with_fact("reason", reason);
    if let Some(actual) = actual {
        diagnostic = diagnostic.with_fact("actual_module", diagnostic_fact(actual));
    }
    PendingDiagnostic::new(candidate.logical.as_str(), diagnostic)
}

fn duplicate_module_diagnostic(first: &Candidate, second: &Candidate) -> PendingDiagnostic {
    PendingDiagnostic::new(
        second.logical.as_str(),
        Diagnostic::new(
            codes::DUPLICATE_PROJECT_MODULE,
            Severity::Error,
            "工程中存在重复 module",
            "project contains a duplicate module",
        )
        .with_fact("first_source", diagnostic_fact(first.logical.as_str()))
        .with_fact("module", diagnostic_fact(second.module.as_str()))
        .with_fact("second_source", diagnostic_fact(second.logical.as_str())),
    )
}

fn missing_manifest_module_diagnostic(
    manifest: &Manifest,
    module: &QualifiedModuleName,
    role: &'static str,
) -> PendingDiagnostic {
    let span = if role == "entry" {
        Some(&manifest.locations.entry)
    } else {
        manifest.locations.exports.get(module)
    };
    let mut diagnostic = Diagnostic::new(
        codes::PROJECT_MODULE_NOT_FOUND,
        Severity::Error,
        "工程引用的 module 不存在",
        "project references an absent module",
    )
    .with_fact("module", diagnostic_fact(module.as_str()))
    .with_fact("role", role);
    if let Some(span) = span {
        diagnostic = diagnostic.with_primary_span(DiagnosticSpan::at(
            manifest.locations.source_name.clone(),
            span.start,
            span.end,
        ));
    }
    PendingDiagnostic::new(MANIFEST_FILE_NAME, diagnostic)
}

fn missing_import_diagnostic(import: &RawImport) -> PendingDiagnostic {
    PendingDiagnostic::new(
        import.source.as_str(),
        Diagnostic::new(
            codes::PROJECT_MODULE_NOT_FOUND,
            Severity::Error,
            "工程引用的 module 不存在",
            "project references an absent module",
        )
        .with_primary_span(DiagnosticSpan::at(
            import.source.as_str(),
            import.span.start,
            import.span.end,
        ))
        .with_fact("importer", diagnostic_fact(import.from.as_str()))
        .with_fact("module", diagnostic_fact(import.target.as_str()))
        .with_fact("role", "import"),
    )
}

fn import_graph_diagnostic(
    source: &str,
    span: Option<Range<u32>>,
    reason: &'static str,
    module: Option<&str>,
    target: Option<&str>,
    cycle: Option<&str>,
) -> PendingDiagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::INVALID_PROJECT_IMPORT_GRAPH,
        Severity::Error,
        "工程 import graph 无效",
        "project import graph is invalid",
    )
    .with_fact("reason", reason);
    if let Some(span) = span {
        diagnostic = diagnostic.with_primary_span(DiagnosticSpan::at(source, span.start, span.end));
    }
    if let Some(module) = module {
        diagnostic = diagnostic.with_fact("module", diagnostic_fact(module));
    }
    if let Some(target) = target {
        diagnostic = diagnostic.with_fact("target", diagnostic_fact(target));
    }
    if let Some(cycle) = cycle {
        diagnostic = diagnostic.with_fact("cycle", diagnostic_fact(cycle));
    }
    PendingDiagnostic::new(source, diagnostic)
}

fn io_diagnostic(
    logical_path: &str,
    physical_path: &Path,
    kind: io::ErrorKind,
    message_zh: &'static str,
    message_en: &'static str,
) -> PendingDiagnostic {
    PendingDiagnostic::new(
        logical_path,
        Diagnostic::new(
            codes::SOURCE_READ_FAILED,
            Severity::Error,
            format!("{message_zh}：“{}”", physical_path.display()),
            format!("{message_en}: `{}`", physical_path.display()),
        )
        .with_fact("io_kind", stable_io_kind(kind)),
    )
}

pub(crate) const fn stable_io_kind(kind: io::ErrorKind) -> &'static str {
    match kind {
        io::ErrorKind::NotFound => "not_found",
        io::ErrorKind::PermissionDenied => "permission_denied",
        io::ErrorKind::InvalidData => "invalid_data",
        io::ErrorKind::Interrupted => "interrupted",
        io::ErrorKind::StorageFull => "storage_full",
        _ => "other",
    }
}

fn source_error_diagnostic(path: &str, error: SourceError) -> Diagnostic {
    match error {
        SourceError::InvalidUtf8 {
            valid_up_to,
            error_len,
        } => {
            let end = valid_up_to.saturating_add(error_len.unwrap_or(1));
            Diagnostic::new(
                codes::INVALID_UTF8,
                Severity::Error,
                "源码不是有效的 UTF-8",
                "source is not valid UTF-8",
            )
            .with_primary_span(DiagnosticSpan::at(
                path,
                u32::try_from(valid_up_to).unwrap_or(u32::MAX),
                u32::try_from(end).unwrap_or(u32::MAX),
            ))
            .with_fact(
                "valid_up_to",
                u64::try_from(valid_up_to).unwrap_or(u64::MAX),
            )
        }
        SourceError::MisplacedByteOrderMark { byte_offset } => Diagnostic::new(
            codes::MISPLACED_BOM,
            Severity::Error,
            "UTF-8 BOM 只能出现在文件开头",
            "the UTF-8 byte-order mark is only allowed at the start of a file",
        )
        .with_primary_span(DiagnosticSpan::at(
            path,
            u32::try_from(byte_offset).unwrap_or(u32::MAX),
            u32::try_from(byte_offset.saturating_add(3)).unwrap_or(u32::MAX),
        )),
        SourceError::TooLarge { byte_len } => Diagnostic::new(
            codes::SOURCE_TOO_LARGE,
            Severity::Error,
            "源码文件超过当前实现支持的大小",
            "source file exceeds the size supported by this implementation",
        )
        .with_fact("byte_len", u64::try_from(byte_len).unwrap_or(u64::MAX))
        .with_fact("maximum_byte_len", u64::from(u32::MAX)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ling_diagnostics::DiagnosticCode;

    #[test]
    fn canonical_cycle_uses_the_lexicographically_smallest_rotation() {
        assert_eq!(
            canonical_cycle(&["Z".to_owned(), "A".to_owned(), "M".to_owned()]),
            "A -> M -> Z -> A"
        );
    }

    #[test]
    fn graph_contains_no_physical_project_root() {
        let graph = ModuleGraph {
            package: super::super::validate_package_name("hello").unwrap(),
            entry: validate_module_name("Main").unwrap(),
            dependencies: Box::default(),
            nodes: Box::default(),
            edges: Box::default(),
        };
        let debug = format!("{graph:?}");
        assert!(!debug.contains("C:\\"));
        assert!(!debug.contains("/tmp/"));
    }

    #[test]
    fn diagnostics_sort_by_logical_path_span_and_code() {
        let mut diagnostics = vec![
            PendingDiagnostic::new(
                "z.ling",
                Diagnostic::new(
                    codes::INVALID_PROJECT_SOURCE_PATH,
                    Severity::Error,
                    "错误",
                    "error",
                ),
            ),
            PendingDiagnostic::new(
                "a.ling",
                Diagnostic::new(
                    codes::INVALID_PROJECT_MODULE_DECLARATION,
                    Severity::Error,
                    "错误",
                    "error",
                )
                .with_primary_span(DiagnosticSpan::at("a.ling", 8, 9)),
            ),
            PendingDiagnostic::new(
                "a.ling",
                Diagnostic::new(
                    codes::INVALID_PROJECT_SOURCE_PATH,
                    Severity::Error,
                    "错误",
                    "error",
                )
                .with_primary_span(DiagnosticSpan::at("a.ling", 1, 2)),
            ),
        ];
        sort_pending_diagnostics(&mut diagnostics);
        assert_eq!(
            diagnostics
                .iter()
                .map(|pending| pending.diagnostic.code())
                .collect::<Vec<DiagnosticCode>>(),
            [
                codes::INVALID_PROJECT_SOURCE_PATH,
                codes::INVALID_PROJECT_MODULE_DECLARATION,
                codes::INVALID_PROJECT_SOURCE_PATH,
            ]
        );
    }
}
