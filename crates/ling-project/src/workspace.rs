//! Deterministic locked-project loading for future compiler hosts.
//!
//! This is a library boundary around the already accepted local graph and
//! lock validation.  It intentionally does not select workspaces, compile
//! source, execute programs, or publish a CLI/protocol result.

use std::path::Path;

use crate::Manifest;
use crate::lockfile::{LockFile, LockMode, LockedGraphFailure, resolve_package_graph_with_lock};
use crate::package_graph::PackageGraph;

/// One fully validated, locked local project snapshot.
///
/// The snapshot owns the manifest supplied by the caller, the deterministic
/// package graph, and the canonical lock projection.  It stores no root path,
/// filesystem ordering, or host-specific state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockedProject {
    manifest: Manifest,
    graph: PackageGraph,
    lock: LockFile,
}

impl LockedProject {
    /// Returns the validated root manifest.
    #[must_use]
    pub const fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Returns the deterministic local package graph.
    #[must_use]
    pub const fn graph(&self) -> &PackageGraph {
        &self.graph
    }

    /// Returns the canonical lock projection of the validated graph.
    #[must_use]
    pub const fn lock(&self) -> &LockFile {
        &self.lock
    }
}

/// Loads one project in read-only locked mode.
///
/// Graph resolution and lock validation are delegated to the RFC-0002
/// boundary.  A failure returns no partial snapshot and does not create or
/// rewrite a lock file.  The caller remains responsible for reading and
/// parsing the explicit manifest bytes.
pub fn load_locked_project(
    project_root: &Path,
    manifest: &Manifest,
) -> Result<LockedProject, LockedGraphFailure> {
    let graph = resolve_package_graph_with_lock(project_root, manifest, LockMode::Locked)?;
    let lock = LockFile::from_graph(&graph);
    Ok(LockedProject {
        manifest: manifest.clone(),
        graph,
        lock,
    })
}
