use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::ops::Range;
use std::path::{Path, PathBuf};

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use sha2::{Digest, Sha256};

use super::discovery::{
    DiscoveryFailure, PreparedPackage, analyze_package, prepare_package, stable_io_kind,
};
use super::{
    MANIFEST_FILE_NAME, MANIFEST_VERSION, MAX_MANIFEST_BYTES, Manifest, PackageName,
    PackageVersion, QualifiedModuleName, diagnostic_fact, manifest_too_large_diagnostic,
    parse_manifest, project_resource_limit_diagnostic,
};

const PACKAGE_CONTENT_DOMAIN: &[u8] = b"ling.package-content/1";
const PACKAGE_GRAPH_DOMAIN: &[u8] = b"ling.package-graph/1";

/// Maximum number of distinct package identities accepted in one graph.
const MAX_RESOLVED_PACKAGES: usize = 4_096;
/// Maximum number of physical package instances inspected in one graph.
const MAX_PACKAGE_INSTANCES: usize = 8_192;
/// Maximum exact Ling source bytes retained across one package graph.
const MAX_GRAPH_SOURCE_BYTES: usize = 512 * 1024 * 1024;
/// Maximum entries inspected while resolving one dependency path component or manifest.
const MAX_DEPENDENCY_DIRECTORY_ENTRIES: usize = 65_536;

/// SHA-256 identity of one manifest model and its exact discovered source bytes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackageSourceId(String);

impl PackageSourceId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PackageSourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Secure identity of one resolved package in a version-1 project graph.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackageIdentity {
    name: PackageName,
    version: PackageVersion,
    source: PackageSourceId,
}

impl PackageIdentity {
    #[must_use]
    pub const fn name(&self) -> &PackageName {
        &self.name
    }

    #[must_use]
    pub const fn version(&self) -> PackageVersion {
        self.version
    }

    #[must_use]
    pub const fn source(&self) -> &PackageSourceId {
        &self.source
    }
}

/// SHA-256 identity of a resolved root package and its canonical dependency edges.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackageGraphId(String);

impl PackageGraphId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PackageGraphId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One validated package and its package-local module graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedPackage {
    identity: PackageIdentity,
    entry: QualifiedModuleName,
    exports: Box<[QualifiedModuleName]>,
    modules: super::ModuleGraph,
}

impl ResolvedPackage {
    #[must_use]
    pub const fn identity(&self) -> &PackageIdentity {
        &self.identity
    }

    #[must_use]
    pub const fn entry(&self) -> &QualifiedModuleName {
        &self.entry
    }

    #[must_use]
    pub fn exports(&self) -> &[QualifiedModuleName] {
        &self.exports
    }

    #[must_use]
    pub const fn modules(&self) -> &super::ModuleGraph {
        &self.modules
    }

    #[must_use]
    pub fn exports_module(&self, module: &QualifiedModuleName) -> bool {
        self.exports.binary_search(module).is_ok()
    }
}

/// One canonical direct-dependency edge between secure package identities.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PackageDependencyEdge {
    from: PackageIdentity,
    dependency: PackageName,
    to: PackageIdentity,
}

impl PackageDependencyEdge {
    #[must_use]
    pub const fn from(&self) -> &PackageIdentity {
        &self.from
    }

    #[must_use]
    pub const fn dependency(&self) -> &PackageName {
        &self.dependency
    }

    #[must_use]
    pub const fn to(&self) -> &PackageIdentity {
        &self.to
    }
}

/// Deterministic, fully validated local dependency graph for one root package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageGraph {
    id: PackageGraphId,
    root: PackageIdentity,
    packages: Box<[ResolvedPackage]>,
    edges: Box<[PackageDependencyEdge]>,
}

impl PackageGraph {
    #[must_use]
    pub const fn id(&self) -> &PackageGraphId {
        &self.id
    }

    #[must_use]
    pub const fn root(&self) -> &PackageIdentity {
        &self.root
    }

    #[must_use]
    pub fn packages(&self) -> &[ResolvedPackage] {
        &self.packages
    }

    #[must_use]
    pub fn edges(&self) -> &[PackageDependencyEdge] {
        &self.edges
    }

    #[must_use]
    pub fn package(&self, identity: &PackageIdentity) -> Option<&ResolvedPackage> {
        self.packages
            .binary_search_by(|package| package.identity.cmp(identity))
            .ok()
            .map(|index| &self.packages[index])
    }

    #[must_use]
    pub fn package_by_name(&self, name: &str) -> Option<&ResolvedPackage> {
        self.packages
            .binary_search_by(|package| package.identity.name.as_str().cmp(name))
            .ok()
            .map(|index| &self.packages[index])
    }
}

/// Atomic failure from recursive local package resolution.
#[derive(Debug)]
pub enum DependencyGraphFailure {
    Diagnostics {
        package: PackageName,
        diagnostics: Box<[Diagnostic]>,
    },
    Internal(String),
}

impl DependencyGraphFailure {
    #[must_use]
    pub const fn package(&self) -> Option<&PackageName> {
        match self {
            Self::Diagnostics { package, .. } => Some(package),
            Self::Internal(_) => None,
        }
    }

    #[must_use]
    pub fn diagnostics(&self) -> Option<&[Diagnostic]> {
        match self {
            Self::Diagnostics { diagnostics, .. } => Some(diagnostics),
            Self::Internal(_) => None,
        }
    }
}

impl fmt::Display for DependencyGraphFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Diagnostics {
                package,
                diagnostics,
            } => write!(
                formatter,
                "dependency graph for package `{package}` produced {} diagnostic(s)",
                diagnostics.len()
            ),
            Self::Internal(message) => formatter.write_str(message),
        }
    }
}

impl Error for DependencyGraphFailure {}

/// Resolves content-identified, local path dependencies beneath `project_root`.
///
/// Resolution is recursive and offline. It performs no parent search, registry
/// lookup, network request, shell execution, lockfile read/write, or source write.
pub fn resolve_package_graph(
    project_root: &Path,
    root_manifest: &Manifest,
) -> Result<PackageGraph, DependencyGraphFailure> {
    PackageResolver::new().run(project_root, root_manifest)
}

struct PackageResolver {
    packages: BTreeMap<PackageIdentity, PreparedResolvedPackage>,
    identities_by_name: BTreeMap<PackageName, PackageIdentity>,
    edge_targets: BTreeMap<(PackageIdentity, PackageName), PackageIdentity>,
    identities_by_root: BTreeMap<PathBuf, PackageIdentity>,
    active_positions: BTreeMap<PackageIdentity, usize>,
    active_stack: Vec<PackageIdentity>,
    source_bytes: usize,
}

impl PackageResolver {
    fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            identities_by_name: BTreeMap::new(),
            edge_targets: BTreeMap::new(),
            identities_by_root: BTreeMap::new(),
            active_positions: BTreeMap::new(),
            active_stack: Vec::new(),
            source_bytes: 0,
        }
    }

    fn run(
        mut self,
        project_root: &Path,
        root_manifest: &Manifest,
    ) -> Result<PackageGraph, DependencyGraphFailure> {
        let mut root = self.load_instance(project_root, root_manifest.clone())?;
        let root_identity = root.identity.clone();
        self.register_identity(&mut root, None)?;
        self.activate_instance(&root);
        self.identities_by_root
            .insert(root.canonical_root.clone(), root_identity.clone());
        let mut frames = vec![Frame::new(root)];

        while let Some(mut frame) = frames.pop() {
            let Some(dependency_name) = frame.next_dependency() else {
                self.finish_instance(&frame.instance);
                continue;
            };
            let parent_identity = frame.instance.identity.clone();
            let parent_name = parent_identity.name.clone();
            let dependency = frame
                .instance
                .manifest
                .dependencies()
                .get(&dependency_name)
                .expect("dependency names are captured from the same manifest")
                .clone();
            frames.push(frame);

            let parent = &frames
                .last()
                .expect("parent frame was restored before dependency loading")
                .instance;
            let directory =
                resolve_dependency_directory(&parent.canonical_root, dependency.path().as_str())
                    .map_err(|error| self.path_failure(parent, &dependency_name, error))?;
            if let Some(child_identity) = self.identities_by_root.get(&directory).cloned() {
                if child_identity.name() != &dependency_name {
                    let diagnostic = self
                        .edge_diagnostic(parent, &dependency_name, true, "dependency_name_mismatch")
                        .with_fact("expected", dependency_name.as_str())
                        .with_fact("actual", child_identity.name().as_str());
                    return Err(diagnostics_failure(parent_name, vec![diagnostic]));
                }
                self.register_edge(parent, &dependency_name, &child_identity)?;
                if let Some(position) = self.active_positions.get(&child_identity).copied() {
                    let cycle = canonical_cycle(&self.active_stack[position..]);
                    let diagnostic = self
                        .edge_diagnostic(parent, &dependency_name, false, "package_cycle")
                        .with_fact("cycle", diagnostic_fact(&cycle));
                    return Err(diagnostics_failure(parent_name, vec![diagnostic]));
                }
                continue;
            }
            if self.identities_by_root.len() >= MAX_PACKAGE_INSTANCES {
                let diagnostic = self.edge_resource_diagnostic(
                    parent,
                    &dependency_name,
                    "package_instances",
                    MAX_PACKAGE_INSTANCES,
                    self.identities_by_root.len().saturating_add(1),
                );
                return Err(diagnostics_failure(parent_name, vec![diagnostic]));
            }
            let manifest_label = package_manifest_label(&dependency_name);
            let manifest_bytes = match read_exact_manifest(&directory) {
                Ok(bytes) => bytes,
                Err(ManifestReadFailure::Path(error)) => {
                    return Err(self.path_failure(parent, &dependency_name, error));
                }
                Err(ManifestReadFailure::TooLarge) => {
                    return Err(diagnostics_failure(
                        dependency_name.clone(),
                        vec![manifest_too_large_diagnostic(&manifest_label)],
                    ));
                }
            };
            let child_manifest =
                parse_manifest(&manifest_label, &manifest_bytes).map_err(|error| {
                    diagnostics_failure(dependency_name.clone(), vec![error.diagnostic()])
                })?;
            if child_manifest.package().name() != &dependency_name {
                let diagnostic = self
                    .edge_diagnostic(parent, &dependency_name, true, "dependency_name_mismatch")
                    .with_fact("expected", dependency_name.as_str())
                    .with_fact("actual", child_manifest.package().name().as_str());
                return Err(diagnostics_failure(parent_name, vec![diagnostic]));
            }

            let mut child = self.load_instance(&directory, child_manifest)?;
            self.register_identity(&mut child, Some((parent, &dependency_name)))?;
            self.register_edge(parent, &dependency_name, &child.identity)?;

            if let Some(position) = self.active_positions.get(&child.identity).copied() {
                let cycle = canonical_cycle(&self.active_stack[position..]);
                let diagnostic = self
                    .edge_diagnostic(parent, &dependency_name, false, "package_cycle")
                    .with_fact("cycle", diagnostic_fact(&cycle));
                return Err(diagnostics_failure(parent_name, vec![diagnostic]));
            }

            self.activate_instance(&child);
            self.identities_by_root
                .insert(child.canonical_root.clone(), child.identity.clone());
            frames.push(Frame::new(child));
        }

        let mut packages = Vec::with_capacity(self.packages.len());
        for (identity, prepared) in self.packages {
            let manifest_source_name = prepared.manifest.locations.source_name.clone();
            let package_name = identity.name.clone();
            let modules =
                analyze_package(&prepared.manifest, prepared.sources).map_err(|failure| {
                    contextualize_discovery_failure(package_name, &manifest_source_name, failure)
                })?;
            packages.push(ResolvedPackage {
                identity,
                entry: prepared.manifest.source().entry().clone(),
                exports: prepared.manifest.exports().to_vec().into_boxed_slice(),
                modules,
            });
        }
        let edges = self
            .edge_targets
            .into_iter()
            .map(|((from, dependency), to)| PackageDependencyEdge {
                from,
                dependency,
                to,
            })
            .collect::<Vec<_>>();
        let id = compute_package_graph_id(&root_identity, &edges);
        Ok(PackageGraph {
            id,
            root: root_identity,
            packages: packages.into_boxed_slice(),
            edges: edges.into_boxed_slice(),
        })
    }

    fn load_instance(
        &self,
        project_root: &Path,
        manifest: Manifest,
    ) -> Result<LoadedInstance, DependencyGraphFailure> {
        let package_name = manifest.package().name().clone();
        let manifest_source_name = manifest.locations.source_name.clone();
        let sources = prepare_package(project_root, &manifest).map_err(|failure| {
            contextualize_discovery_failure(package_name.clone(), &manifest_source_name, failure)
        })?;
        let canonical_root = fs::canonicalize(project_root).map_err(|error| {
            let diagnostic = io_graph_diagnostic(
                &package_manifest_label(&package_name),
                None,
                error.kind(),
                "读取包根目录失败",
                "failed to read the package root",
            );
            diagnostics_failure(package_name.clone(), vec![diagnostic])
        })?;
        let source = compute_package_source_id(&manifest, &sources);
        let identity = PackageIdentity {
            name: package_name,
            version: manifest.package().version(),
            source,
        };
        Ok(LoadedInstance {
            identity,
            manifest,
            canonical_root,
            sources: Some(sources),
        })
    }

    fn register_identity(
        &mut self,
        instance: &mut LoadedInstance,
        incoming: Option<(&LoadedInstance, &PackageName)>,
    ) -> Result<(), DependencyGraphFailure> {
        if let Some(previous) = self.identities_by_name.get(instance.identity.name()) {
            if previous != &instance.identity {
                let (parent, dependency) =
                    incoming.expect("only the root package is registered without an incoming edge");
                let reason = if previous.version() == instance.identity.version() {
                    "package_name_collision"
                } else {
                    "package_version_conflict"
                };
                let diagnostic = self
                    .edge_diagnostic(parent, dependency, true, reason)
                    .with_fact("first_content", previous.source().as_str())
                    .with_fact("second_content", instance.identity.source().as_str())
                    .with_fact("expected", previous.version().to_string())
                    .with_fact("actual", instance.identity.version().to_string());
                return Err(diagnostics_failure(
                    parent.identity.name.clone(),
                    vec![diagnostic],
                ));
            }
        } else {
            if self.packages.len() >= MAX_RESOLVED_PACKAGES {
                let (package, diagnostic) = incoming.map_or_else(
                    || {
                        let diagnostic = project_resource_limit_diagnostic(
                            &package_manifest_label(instance.identity.name()),
                            None,
                            instance.identity.name(),
                            "resolved_packages",
                            MAX_RESOLVED_PACKAGES,
                            self.packages.len().saturating_add(1),
                        );
                        (instance.identity.name.clone(), diagnostic)
                    },
                    |(parent, dependency)| {
                        let diagnostic = self.edge_resource_diagnostic(
                            parent,
                            dependency,
                            "resolved_packages",
                            MAX_RESOLVED_PACKAGES,
                            self.packages.len().saturating_add(1),
                        );
                        (parent.identity.name.clone(), diagnostic)
                    },
                );
                return Err(diagnostics_failure(package, vec![diagnostic]));
            }
            self.identities_by_name
                .insert(instance.identity.name.clone(), instance.identity.clone());
        }

        if !self.packages.contains_key(&instance.identity) {
            let package_source_bytes = instance
                .sources
                .as_ref()
                .expect("a newly loaded package retains its source snapshot")
                .source_byte_len();
            let graph_source_bytes = self.source_bytes.saturating_add(package_source_bytes);
            if graph_source_bytes > MAX_GRAPH_SOURCE_BYTES {
                let (package, diagnostic) = incoming.map_or_else(
                    || {
                        let diagnostic = project_resource_limit_diagnostic(
                            &package_manifest_label(instance.identity.name()),
                            None,
                            instance.identity.name(),
                            "graph_source_bytes",
                            MAX_GRAPH_SOURCE_BYTES,
                            graph_source_bytes,
                        );
                        (instance.identity.name.clone(), diagnostic)
                    },
                    |(parent, dependency)| {
                        let diagnostic = self.edge_resource_diagnostic(
                            parent,
                            dependency,
                            "graph_source_bytes",
                            MAX_GRAPH_SOURCE_BYTES,
                            graph_source_bytes,
                        );
                        (parent.identity.name.clone(), diagnostic)
                    },
                );
                return Err(diagnostics_failure(package, vec![diagnostic]));
            }
            let sources = instance
                .sources
                .take()
                .expect("a newly loaded package retains its source snapshot");
            self.source_bytes = graph_source_bytes;
            self.packages.insert(
                instance.identity.clone(),
                PreparedResolvedPackage {
                    manifest: instance.manifest.clone(),
                    sources,
                },
            );
        }
        Ok(())
    }

    fn register_edge(
        &mut self,
        parent: &LoadedInstance,
        dependency: &PackageName,
        child: &PackageIdentity,
    ) -> Result<(), DependencyGraphFailure> {
        let key = (parent.identity.clone(), dependency.clone());
        if let Some(previous) = self.edge_targets.get(&key) {
            if previous != child {
                let diagnostic = self
                    .edge_diagnostic(parent, dependency, false, "dependency_target_collision")
                    .with_fact("first_content", previous.source().as_str())
                    .with_fact("second_content", child.source().as_str());
                return Err(diagnostics_failure(
                    parent.identity.name.clone(),
                    vec![diagnostic],
                ));
            }
        } else {
            self.edge_targets.insert(key, child.clone());
        }
        Ok(())
    }

    fn activate_instance(&mut self, instance: &LoadedInstance) {
        self.active_positions
            .insert(instance.identity.clone(), self.active_stack.len());
        self.active_stack.push(instance.identity.clone());
    }

    fn finish_instance(&mut self, instance: &LoadedInstance) {
        let popped = self.active_stack.pop();
        debug_assert_eq!(popped.as_ref(), Some(&instance.identity));
        self.active_positions.remove(&instance.identity);
    }

    fn path_failure(
        &self,
        parent: &LoadedInstance,
        dependency: &PackageName,
        error: PathFailure,
    ) -> DependencyGraphFailure {
        let diagnostic = match error {
            PathFailure::Project { reason } => {
                self.edge_diagnostic(parent, dependency, false, reason)
            }
            PathFailure::Io { kind } => {
                let location = parent
                    .manifest
                    .locations
                    .dependencies
                    .get(dependency)
                    .expect("validated dependencies retain source locations");
                io_graph_diagnostic(
                    &package_manifest_label(parent.identity.name()),
                    Some(location.path.clone()),
                    kind,
                    "读取工程依赖失败",
                    "failed to read a project dependency",
                )
            }
            PathFailure::Resource {
                resource,
                maximum,
                actual,
            } => self.edge_resource_diagnostic(parent, dependency, resource, maximum, actual),
        };
        diagnostics_failure(parent.identity.name.clone(), vec![diagnostic])
    }

    fn edge_diagnostic(
        &self,
        parent: &LoadedInstance,
        dependency: &PackageName,
        name_span: bool,
        reason: &'static str,
    ) -> Diagnostic {
        let location = parent
            .manifest
            .locations
            .dependencies
            .get(dependency)
            .expect("validated dependencies retain source locations");
        let span = if name_span {
            location.name.clone()
        } else {
            location.path.clone()
        };
        base_graph_diagnostic(
            parent.identity.name(),
            &package_manifest_label(parent.identity.name()),
            Some(span),
            reason,
        )
        .with_fact("dependency", dependency.as_str())
    }

    fn edge_resource_diagnostic(
        &self,
        parent: &LoadedInstance,
        dependency: &PackageName,
        resource: &'static str,
        maximum: usize,
        actual: usize,
    ) -> Diagnostic {
        let location = parent
            .manifest
            .locations
            .dependencies
            .get(dependency)
            .expect("validated dependencies retain source locations");
        project_resource_limit_diagnostic(
            &package_manifest_label(parent.identity.name()),
            Some(location.path.clone()),
            parent.identity.name(),
            resource,
            maximum,
            actual,
        )
    }
}

struct Frame {
    instance: LoadedInstance,
    dependency_names: Vec<PackageName>,
    next: usize,
}

impl Frame {
    fn new(instance: LoadedInstance) -> Self {
        let dependency_names = instance.manifest.dependencies().keys().cloned().collect();
        Self {
            instance,
            dependency_names,
            next: 0,
        }
    }

    fn next_dependency(&mut self) -> Option<PackageName> {
        let dependency = self.dependency_names.get(self.next).cloned();
        self.next = self.next.saturating_add(usize::from(dependency.is_some()));
        dependency
    }
}

struct LoadedInstance {
    identity: PackageIdentity,
    manifest: Manifest,
    canonical_root: PathBuf,
    sources: Option<PreparedPackage>,
}

struct PreparedResolvedPackage {
    manifest: Manifest,
    sources: PreparedPackage,
}

#[derive(Clone, Copy, Debug)]
enum PathFailure {
    Project {
        reason: &'static str,
    },
    Io {
        kind: io::ErrorKind,
    },
    Resource {
        resource: &'static str,
        maximum: usize,
        actual: usize,
    },
}

enum ManifestReadFailure {
    Path(PathFailure),
    TooLarge,
}

impl PathFailure {
    const fn reason(reason: &'static str) -> Self {
        Self::Project { reason }
    }

    const fn io(kind: io::ErrorKind) -> Self {
        Self::Io { kind }
    }

    const fn resource(resource: &'static str, maximum: usize, actual: usize) -> Self {
        Self::Resource {
            resource,
            maximum,
            actual,
        }
    }
}

fn resolve_dependency_directory(
    package_root: &Path,
    logical_path: &str,
) -> Result<PathBuf, PathFailure> {
    let mut current = package_root.to_path_buf();
    for component in logical_path.split('/') {
        let entries = fs::read_dir(&current).map_err(|error| PathFailure::io(error.kind()))?;
        let mut exact = None;
        let mut ascii_case_match = false;
        let mut entry_count = 0_usize;
        for entry in entries {
            entry_count = entry_count.saturating_add(1);
            if entry_count > MAX_DEPENDENCY_DIRECTORY_ENTRIES {
                return Err(PathFailure::resource(
                    "dependency_directory_entries",
                    MAX_DEPENDENCY_DIRECTORY_ENTRIES,
                    entry_count,
                ));
            }
            let entry = entry.map_err(|error| PathFailure::io(error.kind()))?;
            let name = entry.file_name();
            if name == OsStr::new(component) {
                exact = Some(entry.path());
            }
            if name
                .to_str()
                .is_some_and(|name| name.eq_ignore_ascii_case(component))
            {
                ascii_case_match = true;
            }
        }
        let Some(next) = exact else {
            return Err(PathFailure::reason(if ascii_case_match {
                "dependency_path_case_mismatch"
            } else {
                "dependency_path_missing"
            }));
        };
        let canonical = fs::canonicalize(&next).map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                PathFailure::reason("dependency_dangling_symlink")
            } else {
                PathFailure::io(error.kind())
            }
        })?;
        if !canonical.starts_with(package_root) {
            return Err(PathFailure::reason("dependency_path_escape"));
        }
        current = canonical;
    }
    let metadata = fs::metadata(&current).map_err(|error| PathFailure::io(error.kind()))?;
    if !metadata.is_dir() {
        return Err(PathFailure::reason("dependency_not_directory"));
    }
    Ok(current)
}

fn read_exact_manifest(package_root: &Path) -> Result<Vec<u8>, ManifestReadFailure> {
    let entries = fs::read_dir(package_root)
        .map_err(|error| ManifestReadFailure::Path(PathFailure::io(error.kind())))?;
    let mut exact = None;
    let mut ascii_case_match = false;
    let mut entry_count = 0_usize;
    for entry in entries {
        entry_count = entry_count.saturating_add(1);
        if entry_count > MAX_DEPENDENCY_DIRECTORY_ENTRIES {
            return Err(ManifestReadFailure::Path(PathFailure::resource(
                "dependency_directory_entries",
                MAX_DEPENDENCY_DIRECTORY_ENTRIES,
                entry_count,
            )));
        }
        let entry =
            entry.map_err(|error| ManifestReadFailure::Path(PathFailure::io(error.kind())))?;
        let name = entry.file_name();
        if name == OsStr::new(MANIFEST_FILE_NAME) {
            exact = Some(entry.path());
        }
        if name
            .to_str()
            .is_some_and(|name| name.eq_ignore_ascii_case(MANIFEST_FILE_NAME))
        {
            ascii_case_match = true;
        }
    }
    let Some(path) = exact else {
        return Err(ManifestReadFailure::Path(PathFailure::reason(
            if ascii_case_match {
                "dependency_manifest_case_mismatch"
            } else {
                "dependency_manifest_missing"
            },
        )));
    };
    let canonical = fs::canonicalize(&path).map_err(|error| {
        ManifestReadFailure::Path(if error.kind() == io::ErrorKind::NotFound {
            PathFailure::reason("dependency_manifest_dangling_symlink")
        } else {
            PathFailure::io(error.kind())
        })
    })?;
    if !canonical.starts_with(package_root) {
        return Err(ManifestReadFailure::Path(PathFailure::reason(
            "dependency_manifest_escape",
        )));
    }
    let metadata = fs::metadata(&canonical)
        .map_err(|error| ManifestReadFailure::Path(PathFailure::io(error.kind())))?;
    if !metadata.is_file() {
        return Err(ManifestReadFailure::Path(PathFailure::reason(
            "dependency_manifest_not_file",
        )));
    }
    if metadata.len() > u64::try_from(MAX_MANIFEST_BYTES).unwrap_or(u64::MAX) {
        return Err(ManifestReadFailure::TooLarge);
    }

    let file = fs::File::open(canonical)
        .map_err(|error| ManifestReadFailure::Path(PathFailure::io(error.kind())))?;
    let limit = u64::try_from(MAX_MANIFEST_BYTES)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut bytes = Vec::new();
    file.take(limit)
        .read_to_end(&mut bytes)
        .map_err(|error| ManifestReadFailure::Path(PathFailure::io(error.kind())))?;
    if bytes.len() > MAX_MANIFEST_BYTES {
        return Err(ManifestReadFailure::TooLarge);
    }
    Ok(bytes)
}

fn compute_package_source_id(manifest: &Manifest, sources: &PreparedPackage) -> PackageSourceId {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, PACKAGE_CONTENT_DOMAIN);
    encode_bytes(&mut hasher, MANIFEST_VERSION.to_string().as_bytes());
    encode_bytes(&mut hasher, manifest.package().name().as_str().as_bytes());
    encode_bytes(
        &mut hasher,
        manifest.package().version().to_string().as_bytes(),
    );
    encode_bytes(
        &mut hasher,
        manifest.package().language().as_str().as_bytes(),
    );

    encode_count(&mut hasher, manifest.source().roots().len());
    for root in manifest.source().roots() {
        encode_bytes(&mut hasher, root.as_str().as_bytes());
    }
    encode_bytes(&mut hasher, manifest.source().entry().as_str().as_bytes());
    encode_count(&mut hasher, manifest.exports().len());
    for module in manifest.exports() {
        encode_bytes(&mut hasher, module.as_str().as_bytes());
    }
    encode_count(&mut hasher, manifest.dependencies().len());
    for dependency in manifest.dependencies().keys() {
        encode_bytes(&mut hasher, dependency.as_str().as_bytes());
    }
    encode_count(&mut hasher, sources.sources().len());
    for (source_root, relative_path, bytes) in sources.sources() {
        encode_count(&mut hasher, 3);
        encode_bytes(&mut hasher, source_root.as_str().as_bytes());
        encode_bytes(&mut hasher, relative_path.as_str().as_bytes());
        encode_bytes(&mut hasher, bytes);
    }
    PackageSourceId(format_sha256(hasher.finalize().as_slice()))
}

fn compute_package_graph_id(
    root: &PackageIdentity,
    edges: &[PackageDependencyEdge],
) -> PackageGraphId {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, PACKAGE_GRAPH_DOMAIN);
    encode_identity(&mut hasher, root);
    encode_count(&mut hasher, edges.len());
    for edge in edges {
        encode_count(&mut hasher, 3);
        encode_identity(&mut hasher, &edge.from);
        encode_bytes(&mut hasher, edge.dependency.as_str().as_bytes());
        encode_identity(&mut hasher, &edge.to);
    }
    PackageGraphId(format_sha256(hasher.finalize().as_slice()))
}

fn encode_identity(hasher: &mut Sha256, identity: &PackageIdentity) {
    encode_count(hasher, 3);
    encode_bytes(hasher, identity.name.as_str().as_bytes());
    encode_bytes(hasher, identity.version.to_string().as_bytes());
    encode_bytes(hasher, identity.source.as_str().as_bytes());
}

fn encode_count(hasher: &mut Sha256, count: usize) {
    let count = u64::try_from(count).expect("bounded package collections fit u64");
    hasher.update(count.to_be_bytes());
}

fn encode_bytes(hasher: &mut Sha256, bytes: &[u8]) {
    encode_count(hasher, bytes.len());
    hasher.update(bytes);
}

fn format_sha256(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity("sha256:".len() + bytes.len() * 2);
    output.push_str("sha256:");
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

fn canonical_cycle(identities: &[PackageIdentity]) -> String {
    let names = identities
        .iter()
        .map(|identity| identity.name.as_str())
        .collect::<Vec<_>>();
    let start = (0..names.len())
        .min_by(|left, right| {
            names[*left..]
                .iter()
                .chain(names[..*left].iter())
                .cmp(names[*right..].iter().chain(names[..*right].iter()))
        })
        .unwrap_or(0);
    let mut ordered = names[start..]
        .iter()
        .chain(names[..start].iter())
        .copied()
        .collect::<Vec<_>>();
    if let Some(first) = ordered.first().copied() {
        ordered.push(first);
    }
    ordered.join(" -> ")
}

fn contextualize_discovery_failure(
    package: PackageName,
    manifest_source_name: &str,
    failure: DiscoveryFailure,
) -> DependencyGraphFailure {
    match failure {
        DiscoveryFailure::Diagnostics(diagnostics) => {
            let prefix = package_source_prefix(&package);
            let manifest_label = package_manifest_label(&package);
            let diagnostics = diagnostics
                .into_vec()
                .into_iter()
                .map(|diagnostic| {
                    let Some(span) = diagnostic.primary_span().cloned() else {
                        return diagnostic;
                    };
                    let file = if span.file() == manifest_source_name {
                        manifest_label.clone()
                    } else if span.file().starts_with("package:") {
                        span.file().to_owned()
                    } else {
                        format!("{prefix}/{}", span.file())
                    };
                    diagnostic.with_primary_span(DiagnosticSpan::at(
                        file,
                        span.start_byte(),
                        span.end_byte(),
                    ))
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
            DependencyGraphFailure::Diagnostics {
                package,
                diagnostics,
            }
        }
        DiscoveryFailure::Internal(message) => DependencyGraphFailure::Internal(format!(
            "module discovery for package `{package}` failed internally: {message}"
        )),
    }
}

fn diagnostics_failure(
    package: PackageName,
    diagnostics: Vec<Diagnostic>,
) -> DependencyGraphFailure {
    DependencyGraphFailure::Diagnostics {
        package,
        diagnostics: diagnostics.into_boxed_slice(),
    }
}

fn base_graph_diagnostic(
    package: &PackageName,
    manifest_label: &str,
    span: Option<Range<u32>>,
    reason: &'static str,
) -> Diagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::INVALID_PROJECT_DEPENDENCY_GRAPH,
        Severity::Error,
        "工程依赖图无效",
        "project dependency graph is invalid",
    )
    .with_fact("package", package.as_str())
    .with_fact("reason", reason);
    if let Some(span) = span {
        diagnostic =
            diagnostic.with_primary_span(DiagnosticSpan::at(manifest_label, span.start, span.end));
    }
    diagnostic
}

fn io_graph_diagnostic(
    manifest_label: &str,
    span: Option<Range<u32>>,
    kind: io::ErrorKind,
    message_zh: &'static str,
    message_en: &'static str,
) -> Diagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::SOURCE_READ_FAILED,
        Severity::Error,
        message_zh,
        message_en,
    )
    .with_fact("io_kind", stable_io_kind(kind));
    if let Some(span) = span {
        diagnostic =
            diagnostic.with_primary_span(DiagnosticSpan::at(manifest_label, span.start, span.end));
    }
    diagnostic
}

fn package_source_prefix(package: &PackageName) -> String {
    format!("package:{}", package.as_str())
}

fn package_manifest_label(package: &PackageName) -> String {
    format!("{}/{}", package_source_prefix(package), MANIFEST_FILE_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_text_is_lowercase_and_prefixed() {
        let rendered = format_sha256(&[0, 1, 0xab, 0xff]);
        assert_eq!(rendered, "sha256:0001abff");
    }

    #[test]
    fn package_cycle_uses_smallest_name_rotation() {
        let identities = ["z", "a", "m"].map(|name| PackageIdentity {
            name: super::super::validate_package_name(name).unwrap(),
            version: super::super::validate_package_version("0.1.0").unwrap(),
            source: PackageSourceId(format!("sha256:{name}")),
        });
        assert_eq!(canonical_cycle(&identities), "a -> m -> z -> a");
    }

    #[test]
    fn host_read_failures_keep_the_io_diagnostic_domain() {
        let diagnostic = io_graph_diagnostic(
            "package:app/ling.toml",
            Some(7..11),
            io::ErrorKind::PermissionDenied,
            "读取工程依赖失败",
            "failed to read a project dependency",
        );
        let rendered: serde_json::Value =
            serde_json::from_str(&diagnostic.render_json().unwrap()).unwrap();

        assert_eq!(diagnostic.code(), codes::SOURCE_READ_FAILED);
        assert_eq!(rendered["facts"]["io_kind"], "permission_denied");
        assert!(rendered["facts"].get("reason").is_none());
        let span = diagnostic.primary_span().unwrap();
        assert_eq!(span.file(), "package:app/ling.toml");
        assert_eq!((span.start_byte(), span.end_byte()), (7, 11));
    }

    #[test]
    fn project_resource_failures_have_a_distinct_bounded_contract() {
        let package = super::super::validate_package_name("app").unwrap();
        let diagnostic = project_resource_limit_diagnostic(
            "package:app/ling.toml",
            Some(12..20),
            &package,
            "package_instances",
            8_192,
            8_193,
        );
        let rendered: serde_json::Value =
            serde_json::from_str(&diagnostic.render_json().unwrap()).unwrap();

        assert_eq!(diagnostic.code(), codes::PROJECT_RESOURCE_LIMIT_EXCEEDED);
        assert_eq!(rendered["facts"]["actual"], "8193");
        assert_eq!(rendered["facts"]["maximum"], "8192");
        assert_eq!(rendered["facts"]["package"], "app");
        assert_eq!(rendered["facts"]["resource"], "package_instances");
    }
}
