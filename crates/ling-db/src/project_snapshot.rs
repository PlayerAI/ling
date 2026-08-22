//! Deterministic package-aware semantic snapshots for an already locked graph.
//!
//! This module is deliberately narrower than a project compiler host.  It
//! accepts only the immutable `LockedProject` boundary, consumes the retained
//! package source bytes, and returns the existing package semantic snapshot.
//! It does not select a project, read the host filesystem, execute code, build
//! artifacts, or publish a CLI/protocol result.

use std::fmt;

use ling_ast::LowerError;
use ling_effects::EffectError;
use ling_hir::LowerError as HirLowerError;
use ling_project::{LockedProject, PackageIdentity, PackageSource};
use ling_resolve::{PackagePrograms, ProjectResolveFailure, ResolveError, resolve_project};
use ling_semantic::ProjectProgramSnapshot;
use ling_source::{SourceError, SourceFile, SourceId};
use ling_syntax::parse;
use ling_types::TypeError;

/// Failure while materializing a package-aware checked semantic snapshot.
///
/// The package and logical-source fields are graph coordinates, never host
/// paths.  Keeping the underlying compiler errors preserves the existing
/// diagnostic conversion and deterministic source spans for callers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectSnapshotError {
    SourceIdOverflow {
        package: String,
        logical_name: String,
    },
    InvalidSource {
        package: String,
        logical_name: String,
        error: SourceError,
    },
    AstLowering {
        package: String,
        logical_name: String,
        error: LowerError,
    },
    HirLowering {
        package: String,
        logical_name: String,
        error: HirLowerError,
    },
    Input {
        reason: String,
        package: Option<String>,
        module: Option<String>,
    },
    Resolution {
        errors: Box<[ResolveError]>,
    },
    TypeChecking {
        errors: Box<[TypeError]>,
    },
    EffectChecking {
        errors: Box<[EffectError]>,
    },
    Semantic {
        message: String,
    },
}

impl fmt::Display for ProjectSnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceIdOverflow {
                package,
                logical_name,
            } => write!(
                formatter,
                "project source-id space exhausted at package `{package}` source `{logical_name}`"
            ),
            Self::InvalidSource {
                package,
                logical_name,
                error,
            } => write!(
                formatter,
                "package `{package}` source `{logical_name}` is invalid: {error}"
            ),
            Self::AstLowering {
                package,
                logical_name,
                error,
            } => write!(
                formatter,
                "package `{package}` source `{logical_name}` cannot lower to AST: {error}"
            ),
            Self::HirLowering {
                package,
                logical_name,
                error,
            } => write!(
                formatter,
                "package `{package}` source `{logical_name}` cannot lower to HIR: {error}"
            ),
            Self::Input {
                reason,
                package,
                module,
            } => write!(
                formatter,
                "invalid project resolver input ({reason}, package={package:?}, module={module:?})"
            ),
            Self::Resolution { errors } => {
                write!(
                    formatter,
                    "project resolution produced {} error(s)",
                    errors.len()
                )
            }
            Self::TypeChecking { errors } => {
                write!(
                    formatter,
                    "project type checking produced {} error(s)",
                    errors.len()
                )
            }
            Self::EffectChecking { errors } => write!(
                formatter,
                "project effect checking produced {} error(s)",
                errors.len()
            ),
            Self::Semantic { message } => {
                write!(formatter, "project semantic snapshot failed: {message}")
            }
        }
    }
}

impl std::error::Error for ProjectSnapshotError {}

/// Builds a package-aware semantic snapshot from an already validated locked
/// project.  All source ordering and names come from the graph's canonical
/// package/source order, so repeated builds are byte deterministic.
pub fn build(project: &LockedProject) -> Result<ProjectProgramSnapshot, ProjectSnapshotError> {
    let mut next_source_id = 0_u32;
    let mut packages = Vec::with_capacity(project.graph().packages().len());

    for package in project.graph().packages() {
        let identity = package.identity().clone();
        let mut programs = Vec::with_capacity(package.sources().len());
        for source in package.sources() {
            let logical_name = source.logical_path().to_string();
            let package_name = identity.name().to_string();
            let source_name = source_name(&identity, source);
            let source_id = next_source_id;
            next_source_id = next_source_id.checked_add(1).ok_or_else(|| {
                ProjectSnapshotError::SourceIdOverflow {
                    package: package_name.clone(),
                    logical_name: logical_name.clone(),
                }
            })?;
            let source_file = SourceFile::from_bytes(
                SourceId::new(source_id),
                source_name.clone(),
                source.bytes().to_vec(),
            )
            .map_err(|error| ProjectSnapshotError::InvalidSource {
                package: package_name.clone(),
                logical_name: logical_name.clone(),
                error,
            })?;
            let parsed = parse(&source_file);
            let ast = ling_ast::lower(&source_file, &parsed).map_err(|error| {
                ProjectSnapshotError::AstLowering {
                    package: package_name.clone(),
                    logical_name: logical_name.clone(),
                    error,
                }
            })?;
            let hir = ling_hir::lower(source_name, &ast).map_err(|error| {
                ProjectSnapshotError::HirLowering {
                    package: package_name.clone(),
                    logical_name: logical_name.clone(),
                    error,
                }
            })?;
            programs.push(hir);
        }
        packages.push(PackagePrograms::new(identity, programs));
    }

    let resolved = resolve_project(project.graph(), packages).map_err(project_resolution_error)?;
    let typed =
        ling_types::check(resolved).map_err(|errors| ProjectSnapshotError::TypeChecking {
            errors: errors.into_boxed_slice(),
        })?;
    let checked =
        ling_effects::check(typed).map_err(|errors| ProjectSnapshotError::EffectChecking {
            errors: errors.into_boxed_slice(),
        })?;
    ling_semantic::build_project(checked).map_err(|error| ProjectSnapshotError::Semantic {
        message: error.to_string(),
    })
}

fn source_name(identity: &PackageIdentity, source: &PackageSource) -> String {
    format!("package:{}/{}", identity.name(), source.logical_path())
}

fn project_resolution_error(error: ProjectResolveFailure) -> ProjectSnapshotError {
    match error {
        ProjectResolveFailure::Input(error) => ProjectSnapshotError::Input {
            reason: error.reason.to_owned(),
            package: error.package,
            module: error.module,
        },
        ProjectResolveFailure::Resolution(errors) => ProjectSnapshotError::Resolution {
            errors: errors.into_boxed_slice(),
        },
    }
}
