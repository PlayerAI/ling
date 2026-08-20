//! Deterministic `ling.toml` v1 reader, module discovery, and offline local package graph.

mod discovery;
mod lockfile;
mod package_graph;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::ops::Range;

use ling_diagnostics::{Diagnostic, DiagnosticCode, DiagnosticSpan, Severity, codes};
use serde::Deserialize;
use serde::de::{Deserializer, MapAccess, Visitor};
use toml::Spanned;
use unicode_normalization::UnicodeNormalization;

pub use discovery::{
    DiscoveryFailure, ImportTarget, ModuleEdge, ModuleGraph, ModuleNode, PackageSource,
    discover_modules,
};
pub use lockfile::{
    LOCK_FILE_FORMAT, LOCK_FILE_NAME, LockFile, LockFileFailure, LockMode, LockedDependency,
    LockedGraphFailure, LockedPackage, MAX_LOCK_FILE_BYTES, parse_lock_file,
    resolve_package_graph_with_lock,
};
pub use package_graph::{
    DependencyGraphFailure, PackageDependencyEdge, PackageGraph, PackageGraphId, PackageIdentity,
    PackageSourceId, ResolvedPackage, resolve_package_graph,
};

/// Exact project-manifest filename fixed by RFC-0002.
pub const MANIFEST_FILE_NAME: &str = "ling.toml";
/// Public protocol marker for the decoded manifest model.
pub const MANIFEST_PROTOCOL: &str = "ling.manifest/1";
/// Only manifest version accepted by this reader.
pub const MANIFEST_VERSION: u32 = 1;
/// Maximum accepted encoded manifest size in bytes.
pub const MAX_MANIFEST_BYTES: usize = 1_048_576;
/// Maximum encoded length of a logical path.
pub const MAX_LOGICAL_PATH_BYTES: usize = 4_096;
/// Maximum number of source roots in one manifest.
pub const MAX_SOURCE_ROOTS: usize = 32;
/// Maximum number of local dependencies in one manifest.
pub const MAX_DEPENDENCIES: usize = 1_024;
/// Maximum number of exported modules in one manifest.
pub const MAX_EXPORTS: usize = 4_096;
const MAX_DIAGNOSTIC_FACT_PREFIX_BYTES: usize = 192;

/// A validated graph-local package name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackageName(String);

impl PackageName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Optional, non-identity Unicode package metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DisplayName(String);

impl DisplayName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DisplayName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Restricted three-component package version used by manifest version 1.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackageVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl PackageVersion {
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl fmt::Display for PackageVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The language compatibility version accepted by manifest version 1.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LanguageVersion;

impl LanguageVersion {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        "0.1"
    }
}

impl fmt::Display for LanguageVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// An NFC, dot-separated sequence of Unicode 17 XID identifiers.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QualifiedModuleName(String);

impl QualifiedModuleName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for QualifiedModuleName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A normalized, project-relative logical path using `/` separators.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LogicalPath(String);

impl LogicalPath {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LogicalPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Metadata that identifies one package within a resolved graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageMetadata {
    name: PackageName,
    display_name: Option<DisplayName>,
    version: PackageVersion,
    language: LanguageVersion,
}

impl PackageMetadata {
    #[must_use]
    pub const fn name(&self) -> &PackageName {
        &self.name
    }

    #[must_use]
    pub const fn display_name(&self) -> Option<&DisplayName> {
        self.display_name.as_ref()
    }

    #[must_use]
    pub const fn version(&self) -> PackageVersion {
        self.version
    }

    #[must_use]
    pub const fn language(&self) -> LanguageVersion {
        self.language
    }
}

/// Deterministic source-root and entry-module declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceLayout {
    roots: Box<[LogicalPath]>,
    entry: QualifiedModuleName,
}

impl SourceLayout {
    #[must_use]
    pub fn roots(&self) -> &[LogicalPath] {
        &self.roots
    }

    #[must_use]
    pub const fn entry(&self) -> &QualifiedModuleName {
        &self.entry
    }
}

/// A version-1 local dependency declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalDependency {
    path: LogicalPath,
}

impl LocalDependency {
    #[must_use]
    pub const fn path(&self) -> &LogicalPath {
        &self.path
    }
}

/// The validated semantic model of `ling.manifest/1`.
#[derive(Clone)]
pub struct Manifest {
    package: PackageMetadata,
    source: SourceLayout,
    exports: Box<[QualifiedModuleName]>,
    dependencies: BTreeMap<PackageName, LocalDependency>,
    locations: ManifestLocations,
}

impl PartialEq for Manifest {
    fn eq(&self, other: &Self) -> bool {
        self.package == other.package
            && self.source == other.source
            && self.exports == other.exports
            && self.dependencies == other.dependencies
    }
}

impl Eq for Manifest {}

impl fmt::Debug for Manifest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Manifest")
            .field("package", &self.package)
            .field("source", &self.source)
            .field("exports", &self.exports)
            .field("dependencies", &self.dependencies)
            .finish()
    }
}

#[derive(Clone, Debug)]
struct ManifestLocations {
    source_name: String,
    source_roots: BTreeMap<LogicalPath, Range<u32>>,
    entry: Range<u32>,
    exports: BTreeMap<QualifiedModuleName, Range<u32>>,
    dependencies: BTreeMap<PackageName, DependencyLocation>,
}

#[derive(Clone, Debug)]
struct DependencyLocation {
    name: Range<u32>,
    path: Range<u32>,
}

struct ValidatedSource {
    layout: SourceLayout,
    root_locations: BTreeMap<LogicalPath, Range<u32>>,
    entry_location: Range<u32>,
}

struct ValidatedExports {
    modules: Box<[QualifiedModuleName]>,
    locations: BTreeMap<QualifiedModuleName, Range<u32>>,
}

struct ValidatedDependencies {
    dependencies: BTreeMap<PackageName, LocalDependency>,
    locations: BTreeMap<PackageName, DependencyLocation>,
}

impl Manifest {
    #[must_use]
    pub const fn protocol(&self) -> &'static str {
        MANIFEST_PROTOCOL
    }

    #[must_use]
    pub const fn package(&self) -> &PackageMetadata {
        &self.package
    }

    #[must_use]
    pub const fn source(&self) -> &SourceLayout {
        &self.source
    }

    #[must_use]
    pub fn exports(&self) -> &[QualifiedModuleName] {
        &self.exports
    }

    #[must_use]
    pub const fn dependencies(&self) -> &BTreeMap<PackageName, LocalDependency> {
        &self.dependencies
    }
}

/// A bounded manifest failure with an original UTF-8 byte span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManifestError {
    source_name: String,
    span: Range<u32>,
    kind: ManifestErrorKind,
}

impl ManifestError {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub fn span(&self) -> Range<u32> {
        self.span.clone()
    }

    #[must_use]
    pub fn code(&self) -> DiagnosticCode {
        self.kind.code()
    }

    #[must_use]
    pub fn diagnostic(&self) -> Diagnostic {
        let (message_zh, message_en) = self.kind.messages();
        let mut diagnostic = Diagnostic::new(self.code(), Severity::Error, message_zh, message_en)
            .with_primary_span(DiagnosticSpan::at(
                self.source_name.clone(),
                self.span.start,
                self.span.end,
            ));

        match &self.kind {
            ManifestErrorKind::Bytes {
                reason,
                valid_up_to,
            } => {
                diagnostic = diagnostic.with_fact("reason", reason.as_str());
                if *reason == ByteReason::TooLarge {
                    diagnostic = diagnostic.with_fact(
                        "maximum_byte_len",
                        u64::try_from(MAX_MANIFEST_BYTES).unwrap_or(u64::MAX),
                    );
                }
                if let Some(valid_up_to) = valid_up_to {
                    diagnostic = diagnostic.with_fact("valid_up_to", u64::from(*valid_up_to));
                }
            }
            ManifestErrorKind::Structure { reason } => {
                diagnostic = diagnostic.with_fact("reason", reason.as_str());
            }
            ManifestErrorKind::Version {
                field,
                expected,
                found,
            } => {
                diagnostic = diagnostic
                    .with_fact("field", *field)
                    .with_fact("expected", *expected)
                    .with_fact("found", found.clone());
            }
            ManifestErrorKind::Package {
                field,
                reason,
                value,
            }
            | ManifestErrorKind::Source {
                field,
                reason,
                value,
            } => {
                diagnostic = diagnostic
                    .with_fact("field", *field)
                    .with_fact("reason", reason.as_str());
                if let Some(value) = value {
                    diagnostic = diagnostic.with_fact("value", value.clone());
                }
            }
            ManifestErrorKind::Export { reason, module } => {
                diagnostic = diagnostic.with_fact("reason", reason.as_str());
                if let Some(module) = module {
                    diagnostic = diagnostic.with_fact("module", module.clone());
                }
            }
            ManifestErrorKind::Dependency {
                reason,
                dependency,
                value,
            } => {
                diagnostic = diagnostic.with_fact("reason", reason.as_str());
                if let Some(dependency) = dependency {
                    diagnostic = diagnostic.with_fact("dependency", dependency.clone());
                }
                if let Some(value) = value {
                    diagnostic = diagnostic.with_fact("value", value.clone());
                }
            }
        }
        diagnostic
    }

    fn new(source_name: &str, span: Range<usize>, kind: ManifestErrorKind) -> Self {
        Self {
            source_name: source_name.to_owned(),
            span: bounded_span(span),
            kind,
        }
    }
}

impl fmt::Display for ManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} at {}:{}..{}",
            self.code(),
            self.source_name,
            self.span.start,
            self.span.end
        )
    }
}

impl Error for ManifestError {}

/// Decodes and validates one exact `ling.toml` byte sequence.
///
/// This function performs no filesystem access, project discovery, dependency
/// traversal, or source loading. Those responsibilities belong to later PRJ
/// tasks.
pub fn parse_manifest(source_name: &str, bytes: &[u8]) -> Result<Manifest, ManifestError> {
    validate_manifest_bytes(source_name, bytes)?;
    let source = std::str::from_utf8(bytes).map_err(|error| {
        let start = error.valid_up_to();
        let end = start
            .saturating_add(error.error_len().unwrap_or(1))
            .min(bytes.len());
        ManifestError::new(
            source_name,
            start..end,
            ManifestErrorKind::Bytes {
                reason: ByteReason::InvalidUtf8,
                valid_up_to: Some(to_u32(start)),
            },
        )
    })?;

    if let Some((span, reason)) = reject_toml_1_1_syntax(source) {
        return Err(ManifestError::new(
            source_name,
            span,
            ManifestErrorKind::Structure { reason },
        ));
    }

    let raw: RawManifest = toml::from_str(source).map_err(|error| {
        let reason = classify_toml_error(error.message());
        let span = normalize_toml_error_span(source, error.span(), error.message(), reason);
        ManifestError::new(source_name, span, ManifestErrorKind::Structure { reason })
    })?;
    validate_raw_manifest(source_name, raw)
}

pub(crate) fn manifest_too_large_diagnostic(source_name: &str) -> Diagnostic {
    ManifestError::new(
        source_name,
        0..0,
        ManifestErrorKind::Bytes {
            reason: ByteReason::TooLarge,
            valid_up_to: None,
        },
    )
    .diagnostic()
}

pub(crate) fn project_resource_limit_diagnostic(
    source_name: &str,
    span: Option<Range<u32>>,
    package: &PackageName,
    resource: &'static str,
    maximum: usize,
    actual: usize,
) -> Diagnostic {
    let mut diagnostic = Diagnostic::new(
        codes::PROJECT_RESOURCE_LIMIT_EXCEEDED,
        Severity::Error,
        "工程输入超过资源上限",
        "project input exceeds a resource limit",
    )
    .with_fact("actual", actual.to_string())
    .with_fact("maximum", maximum.to_string())
    .with_fact("package", package.as_str())
    .with_fact("resource", resource);
    if let Some(span) = span {
        diagnostic =
            diagnostic.with_primary_span(DiagnosticSpan::at(source_name, span.start, span.end));
    }
    diagnostic
}

fn validate_manifest_bytes(source_name: &str, bytes: &[u8]) -> Result<(), ManifestError> {
    if bytes.len() > MAX_MANIFEST_BYTES {
        return Err(ManifestError::new(
            source_name,
            0..0,
            ManifestErrorKind::Bytes {
                reason: ByteReason::TooLarge,
                valid_up_to: None,
            },
        ));
    }
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(ManifestError::new(
            source_name,
            0..3,
            ManifestErrorKind::Bytes {
                reason: ByteReason::ByteOrderMark,
                valid_up_to: None,
            },
        ));
    }
    if let Some(offset) = bytes.iter().position(|byte| *byte == 0) {
        return Err(ManifestError::new(
            source_name,
            offset..offset + 1,
            ManifestErrorKind::Bytes {
                reason: ByteReason::Nul,
                valid_up_to: None,
            },
        ));
    }
    Ok(())
}

fn validate_raw_manifest(source_name: &str, raw: RawManifest) -> Result<Manifest, ManifestError> {
    if *raw.manifest_version.get_ref() != i64::from(MANIFEST_VERSION) {
        return Err(ManifestError::new(
            source_name,
            raw.manifest_version.span(),
            ManifestErrorKind::Version {
                field: "manifest-version",
                expected: "1",
                found: raw.manifest_version.to_string(),
            },
        ));
    }

    let package = validate_package(source_name, raw.package)?;
    let source = validate_source(source_name, raw.source)?;
    let exports = validate_exports(source_name, raw.exports)?;
    let dependencies = validate_dependencies(source_name, raw.dependencies)?;

    Ok(Manifest {
        package,
        source: source.layout,
        exports: exports.modules,
        dependencies: dependencies.dependencies,
        locations: ManifestLocations {
            source_name: source_name.to_owned(),
            source_roots: source.root_locations,
            entry: source.entry_location,
            exports: exports.locations,
            dependencies: dependencies.locations,
        },
    })
}

fn validate_package(source_name: &str, raw: RawPackage) -> Result<PackageMetadata, ManifestError> {
    let name = validate_package_name(raw.name.get_ref()).map_err(|reason| {
        ManifestError::new(
            source_name,
            raw.name.span(),
            ManifestErrorKind::Package {
                field: "package.name",
                reason,
                value: Some(diagnostic_fact(raw.name.get_ref())),
            },
        )
    })?;

    let display_name = raw
        .display_name
        .map(|value| {
            validate_display_name(value.get_ref()).map_err(|reason| {
                ManifestError::new(
                    source_name,
                    value.span(),
                    ManifestErrorKind::Package {
                        field: "package.display-name",
                        reason,
                        value: Some(diagnostic_fact(value.get_ref())),
                    },
                )
            })
        })
        .transpose()?;

    let version = validate_package_version(raw.version.get_ref()).map_err(|reason| {
        ManifestError::new(
            source_name,
            raw.version.span(),
            ManifestErrorKind::Package {
                field: "package.version",
                reason,
                value: Some(diagnostic_fact(raw.version.get_ref())),
            },
        )
    })?;

    if raw.language.get_ref() != "0.1" {
        return Err(ManifestError::new(
            source_name,
            raw.language.span(),
            ManifestErrorKind::Version {
                field: "package.language",
                expected: "0.1",
                found: diagnostic_fact(raw.language.get_ref()),
            },
        ));
    }

    Ok(PackageMetadata {
        name,
        display_name,
        version,
        language: LanguageVersion,
    })
}

fn validate_source(source_name: &str, raw: RawSource) -> Result<ValidatedSource, ManifestError> {
    if raw.roots.get_ref().is_empty() {
        return Err(ManifestError::new(
            source_name,
            raw.roots.span(),
            ManifestErrorKind::Source {
                field: "source.roots",
                reason: ValidationReason::Empty,
                value: None,
            },
        ));
    }
    if raw.roots.get_ref().len() > MAX_SOURCE_ROOTS {
        return Err(ManifestError::new(
            source_name,
            raw.roots.span(),
            ManifestErrorKind::Source {
                field: "source.roots",
                reason: ValidationReason::TooMany,
                value: Some(raw.roots.get_ref().len().to_string()),
            },
        ));
    }

    let mut roots_with_spans = Vec::with_capacity(raw.roots.get_ref().len());
    for raw_root in raw.roots.into_inner() {
        let root = validate_logical_path(raw_root.get_ref()).map_err(|reason| {
            ManifestError::new(
                source_name,
                raw_root.span(),
                ManifestErrorKind::Source {
                    field: "source.roots",
                    reason,
                    value: Some(diagnostic_fact(raw_root.get_ref())),
                },
            )
        })?;
        if roots_with_spans
            .iter()
            .any(|(previous, _)| paths_overlap(previous, &root))
        {
            return Err(ManifestError::new(
                source_name,
                raw_root.span(),
                ManifestErrorKind::Source {
                    field: "source.roots",
                    reason: ValidationReason::Overlapping,
                    value: Some(diagnostic_fact(raw_root.get_ref())),
                },
            ));
        }
        roots_with_spans.push((root, raw_root.span()));
    }
    roots_with_spans.sort_by(|(left, _), (right, _)| left.cmp(right));
    let root_locations = roots_with_spans
        .iter()
        .map(|(root, span)| (root.clone(), bounded_span(span.clone())))
        .collect::<BTreeMap<_, _>>();
    let roots = roots_with_spans
        .into_iter()
        .map(|(root, _)| root)
        .collect::<Vec<_>>();

    let entry_span = bounded_span(raw.entry.span());
    let entry = validate_module_name(raw.entry.get_ref()).map_err(|reason| {
        ManifestError::new(
            source_name,
            raw.entry.span(),
            ManifestErrorKind::Source {
                field: "source.entry",
                reason,
                value: Some(diagnostic_fact(raw.entry.get_ref())),
            },
        )
    })?;

    Ok(ValidatedSource {
        layout: SourceLayout {
            roots: roots.into_boxed_slice(),
            entry,
        },
        root_locations,
        entry_location: entry_span,
    })
}

fn validate_exports(
    source_name: &str,
    raw: Option<RawExports>,
) -> Result<ValidatedExports, ManifestError> {
    let Some(raw_modules) = raw.and_then(|exports| exports.modules) else {
        return Ok(ValidatedExports {
            modules: Box::default(),
            locations: BTreeMap::new(),
        });
    };
    if raw_modules.get_ref().len() > MAX_EXPORTS {
        return Err(ManifestError::new(
            source_name,
            raw_modules.span(),
            ManifestErrorKind::Export {
                reason: ValidationReason::TooMany,
                module: None,
            },
        ));
    }

    let mut modules = BTreeMap::new();
    for raw_module in raw_modules.into_inner() {
        let span = raw_module.span();
        let module = validate_module_name(raw_module.get_ref()).map_err(|reason| {
            ManifestError::new(
                source_name,
                raw_module.span(),
                ManifestErrorKind::Export {
                    reason,
                    module: Some(diagnostic_fact(raw_module.get_ref())),
                },
            )
        })?;
        if modules
            .insert(module.clone(), bounded_span(span.clone()))
            .is_some()
        {
            return Err(ManifestError::new(
                source_name,
                span,
                ManifestErrorKind::Export {
                    reason: ValidationReason::Duplicate,
                    module: Some(diagnostic_fact(raw_module.get_ref())),
                },
            ));
        }
    }
    let names = modules
        .keys()
        .cloned()
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Ok(ValidatedExports {
        modules: names,
        locations: modules,
    })
}

fn validate_dependencies(
    source_name: &str,
    raw: Option<Spanned<RawDependencies>>,
) -> Result<ValidatedDependencies, ManifestError> {
    let Some(raw) = raw else {
        return Ok(ValidatedDependencies {
            dependencies: BTreeMap::new(),
            locations: BTreeMap::new(),
        });
    };
    if raw.get_ref().entries.len() > MAX_DEPENDENCIES {
        return Err(ManifestError::new(
            source_name,
            raw.span(),
            ManifestErrorKind::Dependency {
                reason: ValidationReason::TooMany,
                dependency: None,
                value: Some(raw.get_ref().entries.len().to_string()),
            },
        ));
    }

    let mut dependencies = BTreeMap::new();
    let mut locations = BTreeMap::new();
    for (raw_name, raw_dependency) in raw.into_inner().entries {
        let name_span = bounded_span(raw_name.span());
        let name = validate_package_name(raw_name.get_ref()).map_err(|reason| {
            ManifestError::new(
                source_name,
                raw_name.span(),
                ManifestErrorKind::Dependency {
                    reason,
                    dependency: Some(diagnostic_fact(raw_name.get_ref())),
                    value: None,
                },
            )
        })?;
        let raw_path = raw_dependency.into_inner().path;
        let path_span = bounded_span(raw_path.span());
        let path = validate_logical_path(raw_path.get_ref()).map_err(|reason| {
            ManifestError::new(
                source_name,
                raw_path.span(),
                ManifestErrorKind::Dependency {
                    reason,
                    dependency: Some(diagnostic_fact(raw_name.get_ref())),
                    value: Some(diagnostic_fact(raw_path.get_ref())),
                },
            )
        })?;
        locations.insert(
            name.clone(),
            DependencyLocation {
                name: name_span,
                path: path_span,
            },
        );
        dependencies.insert(name, LocalDependency { path });
    }
    Ok(ValidatedDependencies {
        dependencies,
        locations,
    })
}

fn validate_package_name(raw: &str) -> Result<PackageName, ValidationReason> {
    if raw.is_empty() {
        return Err(ValidationReason::Empty);
    }
    if raw.len() > 63 {
        return Err(ValidationReason::TooLong);
    }
    let bytes = raw.as_bytes();
    if !bytes[0].is_ascii_lowercase() {
        return Err(ValidationReason::InvalidFormat);
    }
    let mut previous_was_hyphen = false;
    for byte in bytes {
        match byte {
            b'a'..=b'z' | b'0'..=b'9' => previous_was_hyphen = false,
            b'-' if !previous_was_hyphen => previous_was_hyphen = true,
            _ => return Err(ValidationReason::InvalidFormat),
        }
    }
    if previous_was_hyphen {
        return Err(ValidationReason::InvalidFormat);
    }
    Ok(PackageName(raw.to_owned()))
}

fn validate_display_name(raw: &str) -> Result<DisplayName, ValidationReason> {
    let scalar_count = raw.chars().count();
    if scalar_count == 0 {
        return Err(ValidationReason::Empty);
    }
    if scalar_count > 128 {
        return Err(ValidationReason::TooLong);
    }
    if raw.nfc().ne(raw.chars()) {
        return Err(ValidationReason::NotNfc);
    }
    if raw.chars().next().is_some_and(ling_unicode::is_white_space)
        || raw
            .chars()
            .next_back()
            .is_some_and(ling_unicode::is_white_space)
    {
        return Err(ValidationReason::BoundaryWhitespace);
    }
    for character in raw.chars() {
        let codepoint = u32::from(character);
        if codepoint <= 0x1f
            || (0x7f..=0x9f).contains(&codepoint)
            || matches!(codepoint, 0x2028 | 0x2029)
        {
            return Err(ValidationReason::ControlCharacter);
        }
        if ling_unicode::is_bidi_control(character) {
            return Err(ValidationReason::BidiControl);
        }
        if ling_unicode::is_default_ignorable(character) {
            return Err(ValidationReason::DefaultIgnorable);
        }
    }
    Ok(DisplayName(raw.to_owned()))
}

fn validate_package_version(raw: &str) -> Result<PackageVersion, ValidationReason> {
    let mut components = raw.split('.');
    let major = parse_version_component(components.next())?;
    let minor = parse_version_component(components.next())?;
    let patch = parse_version_component(components.next())?;
    if components.next().is_some() {
        return Err(ValidationReason::InvalidFormat);
    }
    Ok(PackageVersion {
        major,
        minor,
        patch,
    })
}

fn parse_version_component(component: Option<&str>) -> Result<u32, ValidationReason> {
    let Some(component) = component else {
        return Err(ValidationReason::InvalidFormat);
    };
    if component.is_empty()
        || !component.bytes().all(|byte| byte.is_ascii_digit())
        || (component.len() > 1 && component.starts_with('0'))
    {
        return Err(ValidationReason::InvalidFormat);
    }
    component
        .parse::<u32>()
        .map_err(|_| ValidationReason::InvalidFormat)
}

fn validate_module_name(raw: &str) -> Result<QualifiedModuleName, ValidationReason> {
    if raw.is_empty() {
        return Err(ValidationReason::Empty);
    }
    if raw.nfc().ne(raw.chars()) {
        return Err(ValidationReason::NotNfc);
    }
    for segment in raw.split('.') {
        if segment.is_empty() || ling_unicode::validate_identifier(segment).is_err() {
            return Err(ValidationReason::InvalidModuleSegment);
        }
    }
    Ok(QualifiedModuleName(raw.to_owned()))
}

fn validate_logical_path(raw: &str) -> Result<LogicalPath, ValidationReason> {
    if raw.is_empty() {
        return Err(ValidationReason::Empty);
    }
    if raw.len() > MAX_LOGICAL_PATH_BYTES {
        return Err(ValidationReason::TooLong);
    }
    if raw.contains('\0') {
        return Err(ValidationReason::NulCharacter);
    }
    if raw.contains('\\') {
        return Err(ValidationReason::Backslash);
    }
    if raw.starts_with('/') {
        return Err(ValidationReason::AbsolutePath);
    }
    if has_windows_drive_prefix(raw) {
        return Err(ValidationReason::DrivePrefix);
    }
    if has_uri_scheme(raw) {
        return Err(ValidationReason::UriScheme);
    }
    for segment in raw.split('/') {
        if segment.is_empty() {
            return Err(ValidationReason::EmptyPathSegment);
        }
        if segment == "." {
            return Err(ValidationReason::DotPathSegment);
        }
        if segment == ".." {
            return Err(ValidationReason::ParentPathSegment);
        }
        if segment.nfc().ne(segment.chars()) {
            return Err(ValidationReason::NotNfc);
        }
    }
    Ok(LogicalPath(raw.to_owned()))
}

fn has_windows_drive_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

fn has_uri_scheme(path: &str) -> bool {
    let mut bytes = path.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    for byte in bytes {
        match byte {
            b':' => return true,
            b'/' => return false,
            byte if byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.') => {}
            _ => return false,
        }
    }
    false
}

fn paths_overlap(left: &LogicalPath, right: &LogicalPath) -> bool {
    left == right
        || is_path_ancestor(left.as_str(), right.as_str())
        || is_path_ancestor(right.as_str(), left.as_str())
}

fn is_path_ancestor(parent: &str, child: &str) -> bool {
    child
        .strip_prefix(parent)
        .is_some_and(|suffix| suffix.starts_with('/'))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ByteReason {
    TooLarge,
    ByteOrderMark,
    Nul,
    InvalidUtf8,
}

impl ByteReason {
    const fn as_str(self) -> &'static str {
        match self {
            Self::TooLarge => "manifest_too_large",
            Self::ByteOrderMark => "utf8_bom",
            Self::Nul => "nul_byte",
            Self::InvalidUtf8 => "invalid_utf8",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StructureReason {
    Toml10Required,
    Syntax,
    DuplicateField,
    UnknownField,
    MissingField,
    WrongType,
}

impl StructureReason {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Toml10Required => "toml_1_1_syntax",
            Self::Syntax => "toml_syntax",
            Self::DuplicateField => "duplicate_field",
            Self::UnknownField => "unknown_field",
            Self::MissingField => "missing_field",
            Self::WrongType => "wrong_type",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ValidationReason {
    Empty,
    TooLong,
    TooMany,
    InvalidFormat,
    NotNfc,
    BoundaryWhitespace,
    ControlCharacter,
    BidiControl,
    DefaultIgnorable,
    InvalidModuleSegment,
    NulCharacter,
    Backslash,
    AbsolutePath,
    DrivePrefix,
    UriScheme,
    EmptyPathSegment,
    DotPathSegment,
    ParentPathSegment,
    Duplicate,
    Overlapping,
}

impl ValidationReason {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::TooLong => "too_long",
            Self::TooMany => "too_many",
            Self::InvalidFormat => "invalid_format",
            Self::NotNfc => "not_nfc",
            Self::BoundaryWhitespace => "leading_or_trailing_whitespace",
            Self::ControlCharacter => "forbidden_control",
            Self::BidiControl => "bidi_control",
            Self::DefaultIgnorable => "default_ignorable",
            Self::InvalidModuleSegment => "invalid_module_segment",
            Self::NulCharacter => "nul_character",
            Self::Backslash => "backslash_separator",
            Self::AbsolutePath => "absolute_path",
            Self::DrivePrefix => "windows_drive_prefix",
            Self::UriScheme => "uri_scheme",
            Self::EmptyPathSegment => "empty_path_segment",
            Self::DotPathSegment => "dot_path_segment",
            Self::ParentPathSegment => "parent_path_segment",
            Self::Duplicate => "duplicate",
            Self::Overlapping => "overlapping",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ManifestErrorKind {
    Bytes {
        reason: ByteReason,
        valid_up_to: Option<u32>,
    },
    Structure {
        reason: StructureReason,
    },
    Version {
        field: &'static str,
        expected: &'static str,
        found: String,
    },
    Package {
        field: &'static str,
        reason: ValidationReason,
        value: Option<String>,
    },
    Source {
        field: &'static str,
        reason: ValidationReason,
        value: Option<String>,
    },
    Export {
        reason: ValidationReason,
        module: Option<String>,
    },
    Dependency {
        reason: ValidationReason,
        dependency: Option<String>,
        value: Option<String>,
    },
}

impl ManifestErrorKind {
    const fn code(&self) -> DiagnosticCode {
        match self {
            Self::Bytes { .. } => codes::INVALID_PROJECT_MANIFEST_BYTES,
            Self::Structure { .. } => codes::INVALID_PROJECT_MANIFEST_STRUCTURE,
            Self::Version { .. } => codes::UNSUPPORTED_PROJECT_MANIFEST_VERSION,
            Self::Package { .. } => codes::INVALID_PROJECT_PACKAGE_METADATA,
            Self::Source { .. } => codes::INVALID_PROJECT_SOURCE_LAYOUT,
            Self::Export { .. } => codes::INVALID_PROJECT_EXPORT,
            Self::Dependency { .. } => codes::INVALID_PROJECT_DEPENDENCY,
        }
    }

    const fn messages(&self) -> (&'static str, &'static str) {
        match self {
            Self::Bytes { .. } => (
                "工程清单的字节编码或大小无效",
                "project manifest byte encoding or size is invalid",
            ),
            Self::Structure { .. } => (
                "工程清单的 TOML 结构无效",
                "project manifest TOML structure is invalid",
            ),
            Self::Version { .. } => (
                "工程清单声明了不受支持的版本",
                "project manifest declares an unsupported version",
            ),
            Self::Package { .. } => (
                "工程清单的包元数据无效",
                "project manifest package metadata is invalid",
            ),
            Self::Source { .. } => (
                "工程清单的源码布局无效",
                "project manifest source layout is invalid",
            ),
            Self::Export { .. } => (
                "工程清单包含无效的导出模块",
                "project manifest contains an invalid exported module",
            ),
            Self::Dependency { .. } => (
                "工程清单包含无效的本地依赖",
                "project manifest contains an invalid local dependency",
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    #[serde(rename = "manifest-version")]
    manifest_version: Spanned<i64>,
    package: RawPackage,
    source: RawSource,
    #[serde(default)]
    exports: Option<RawExports>,
    #[serde(default)]
    dependencies: Option<Spanned<RawDependencies>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPackage {
    name: Spanned<String>,
    #[serde(rename = "display-name")]
    display_name: Option<Spanned<String>>,
    version: Spanned<String>,
    language: Spanned<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSource {
    roots: Spanned<Vec<Spanned<String>>>,
    entry: Spanned<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExports {
    #[serde(default)]
    modules: Option<Spanned<Vec<Spanned<String>>>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDependency {
    path: Spanned<String>,
}

#[derive(Debug, Default)]
struct RawDependencies {
    entries: Vec<(Spanned<String>, Spanned<RawDependency>)>,
}

impl<'de> Deserialize<'de> for RawDependencies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DependenciesVisitor;

        impl<'de> Visitor<'de> for DependenciesVisitor {
            type Value = RawDependencies;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a table of local dependencies")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut entries = Vec::new();
                while let Some(entry) = map.next_entry()? {
                    entries.push(entry);
                }
                Ok(RawDependencies { entries })
            }
        }

        deserializer.deserialize_map(DependenciesVisitor)
    }
}

fn classify_toml_error(message: &str) -> StructureReason {
    let message = message.to_ascii_lowercase();
    if message.contains("duplicate") {
        StructureReason::DuplicateField
    } else if message.contains("unknown field") {
        StructureReason::UnknownField
    } else if message.contains("missing field") {
        StructureReason::MissingField
    } else if message.contains("invalid type") {
        StructureReason::WrongType
    } else {
        StructureReason::Syntax
    }
}

fn normalize_toml_error_span(
    source: &str,
    span: Option<Range<usize>>,
    message: &str,
    reason: StructureReason,
) -> Range<usize> {
    let span = clamp_span(span.unwrap_or(0..0), source.len());
    match reason {
        StructureReason::DuplicateField => trimmed_line_span(source, span.start),
        StructureReason::UnknownField => unknown_field_span(source, &span, message).unwrap_or(span),
        _ => span,
    }
}

fn trimmed_line_span(source: &str, offset: usize) -> Range<usize> {
    let offset = offset.min(source.len());
    let start = source.as_bytes()[..offset]
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(0, |index| index + 1);
    let end = source.as_bytes()[offset..]
        .iter()
        .position(|byte| *byte == b'\n')
        .map_or(source.len(), |index| offset + index);
    let line = &source[start..end];
    let leading = line.len() - line.trim_start_matches([' ', '\t']).len();
    let trailing = line.len() - line.trim_end_matches([' ', '\t', '\r']).len();
    start + leading..end.saturating_sub(trailing)
}

fn unknown_field_span(
    source: &str,
    error_span: &Range<usize>,
    message: &str,
) -> Option<Range<usize>> {
    let field = extract_unknown_field(message)?;
    let line = trimmed_line_span(source, error_span.start);
    source[line.clone()]
        .find(field)
        .map(|relative| line.start + relative..line.start + relative + field.len())
        .or_else(|| {
            let matches = source.match_indices(field).collect::<Vec<_>>();
            (matches.len() == 1).then(|| matches[0].0..matches[0].0 + field.len())
        })
}

fn extract_unknown_field(message: &str) -> Option<&str> {
    let suffix = message.strip_prefix("unknown field ")?;
    for delimiter in ['`', '\'', '"'] {
        if let Some(rest) = suffix.strip_prefix(delimiter) {
            if let Some(end) = rest.find(delimiter) {
                return Some(&rest[..end]);
            }
        }
    }
    None
}

fn clamp_span(span: Range<usize>, byte_len: usize) -> Range<usize> {
    let start = span.start.min(byte_len);
    let end = span.end.max(start).min(byte_len);
    start..end
}

fn bounded_span(span: Range<usize>) -> Range<u32> {
    to_u32(span.start)..to_u32(span.end)
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

fn diagnostic_fact(raw: &str) -> String {
    if raw.len() <= MAX_DIAGNOSTIC_FACT_PREFIX_BYTES {
        return raw.to_owned();
    }
    let mut end = MAX_DIAGNOSTIC_FACT_PREFIX_BYTES;
    while !raw.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…[{} bytes]", &raw[..end], raw.len())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TomlLexState {
    Normal,
    Comment,
    Basic,
    MultilineBasic,
    Literal,
    MultilineLiteral,
}

/// The workspace TOML decoder tracks the newest TOML release. RFC-0002 fixes
/// this protocol to TOML 1.0, so reject the five syntax extensions introduced
/// by TOML 1.1 before decoding. Optional seconds are irrelevant because the
/// manifest schema accepts no date/time values.
fn reject_toml_1_1_syntax(source: &str) -> Option<(Range<usize>, StructureReason)> {
    let bytes = source.as_bytes();
    let mut index = 0;
    let mut state = TomlLexState::Normal;
    let mut inline_tables = Vec::<Option<usize>>::new();

    while index < bytes.len() {
        match state {
            TomlLexState::Normal => match bytes[index] {
                b'#' => {
                    state = TomlLexState::Comment;
                    index += 1;
                }
                b'"' if starts_with_three(bytes, index, b'"') => {
                    mark_inline_token(&mut inline_tables, index);
                    state = TomlLexState::MultilineBasic;
                    index += 3;
                }
                b'"' => {
                    mark_inline_token(&mut inline_tables, index);
                    state = TomlLexState::Basic;
                    index += 1;
                }
                b'\'' if starts_with_three(bytes, index, b'\'') => {
                    mark_inline_token(&mut inline_tables, index);
                    state = TomlLexState::MultilineLiteral;
                    index += 3;
                }
                b'\'' => {
                    mark_inline_token(&mut inline_tables, index);
                    state = TomlLexState::Literal;
                    index += 1;
                }
                b'{' => {
                    mark_inline_token(&mut inline_tables, index);
                    inline_tables.push(None);
                    index += 1;
                }
                b'}' => {
                    if inline_tables
                        .last()
                        .copied()
                        .flatten()
                        .is_some_and(|last| bytes[last] == b',')
                    {
                        let comma = inline_tables.last().copied().flatten().unwrap_or(index);
                        return Some((comma..comma + 1, StructureReason::Toml10Required));
                    }
                    inline_tables.pop();
                    mark_inline_token(&mut inline_tables, index);
                    index += 1;
                }
                b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                    if !inline_tables.is_empty() {
                        return Some((index..index + 2, StructureReason::Toml10Required));
                    }
                    index += 2;
                }
                b'\n' => {
                    if !inline_tables.is_empty() {
                        return Some((index..index + 1, StructureReason::Toml10Required));
                    }
                    index += 1;
                }
                byte if byte.is_ascii_whitespace() => index += 1,
                _ => {
                    mark_inline_token(&mut inline_tables, index);
                    index += 1;
                }
            },
            TomlLexState::Comment => match bytes[index] {
                b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                    if !inline_tables.is_empty() {
                        return Some((index..index + 2, StructureReason::Toml10Required));
                    }
                    state = TomlLexState::Normal;
                    index += 2;
                }
                b'\n' => {
                    if !inline_tables.is_empty() {
                        return Some((index..index + 1, StructureReason::Toml10Required));
                    }
                    state = TomlLexState::Normal;
                    index += 1;
                }
                _ => index += 1,
            },
            TomlLexState::Basic | TomlLexState::MultilineBasic => match bytes[index] {
                b'\\' if matches!(bytes.get(index + 1), Some(b'e' | b'x')) => {
                    return Some((index..index + 2, StructureReason::Toml10Required));
                }
                b'\\' => index = (index + 2).min(bytes.len()),
                b'"' if state == TomlLexState::MultilineBasic => {
                    let run = repeated_byte_count(bytes, index, b'"');
                    if run >= 3 {
                        state = TomlLexState::Normal;
                    }
                    index += run;
                }
                b'"' if state == TomlLexState::Basic => {
                    state = TomlLexState::Normal;
                    index += 1;
                }
                _ => index += 1,
            },
            TomlLexState::Literal | TomlLexState::MultilineLiteral => match bytes[index] {
                b'\'' if state == TomlLexState::MultilineLiteral => {
                    let run = repeated_byte_count(bytes, index, b'\'');
                    if run >= 3 {
                        state = TomlLexState::Normal;
                    }
                    index += run;
                }
                b'\'' if state == TomlLexState::Literal => {
                    state = TomlLexState::Normal;
                    index += 1;
                }
                _ => index += 1,
            },
        }
    }
    None
}

fn starts_with_three(bytes: &[u8], index: usize, byte: u8) -> bool {
    bytes.get(index..index.saturating_add(3)) == Some(&[byte, byte, byte])
}

fn repeated_byte_count(bytes: &[u8], index: usize, byte: u8) -> usize {
    bytes[index..]
        .iter()
        .take_while(|candidate| **candidate == byte)
        .count()
}

fn mark_inline_token(inline_tables: &mut [Option<usize>], index: usize) {
    if let Some(last) = inline_tables.last_mut() {
        *last = Some(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: &str = r#"manifest-version = 1

[package]
name = "hello"
version = "0.1.0"
language = "0.1"

[source]
roots = ["src"]
entry = "Main"
"#;

    #[test]
    fn public_model_is_sorted_and_protocol_versioned() {
        let input = r#"manifest-version = 1

[package]
name = "hello"
version = "1.2.3"
language = "0.1"

[source]
roots = ["z", "a"]
entry = "Main"

[exports]
modules = ["Zed", "Alpha"]

[dependencies]
zed = { path = "deps/zed" }
alpha = { path = "deps/alpha" }
"#;
        let manifest = parse_manifest("ling.toml", input.as_bytes()).unwrap();

        assert_eq!(manifest.protocol(), "ling.manifest/1");
        assert_eq!(manifest.package().version().major(), 1);
        assert_eq!(manifest.package().version().minor(), 2);
        assert_eq!(manifest.package().version().patch(), 3);
        assert_eq!(
            manifest
                .source()
                .roots()
                .iter()
                .map(LogicalPath::as_str)
                .collect::<Vec<_>>(),
            ["a", "z"]
        );
        assert_eq!(
            manifest
                .exports()
                .iter()
                .map(QualifiedModuleName::as_str)
                .collect::<Vec<_>>(),
            ["Alpha", "Zed"]
        );
        assert_eq!(
            manifest
                .dependencies()
                .keys()
                .map(PackageName::as_str)
                .collect::<Vec<_>>(),
            ["alpha", "zed"]
        );
    }

    #[test]
    fn package_name_and_version_boundaries_are_exact() {
        assert!(validate_package_name("a").is_ok());
        assert!(validate_package_name(&format!("a{}", "1".repeat(62))).is_ok());
        for invalid in ["", "A", "a_1", "a--b", "a-", "-a", "é"] {
            assert!(validate_package_name(invalid).is_err(), "{invalid}");
        }
        assert!(validate_package_name(&format!("a{}", "1".repeat(63))).is_err());

        assert_eq!(
            validate_package_version("0.0.0").unwrap().to_string(),
            "0.0.0"
        );
        assert_eq!(
            validate_package_version("4294967295.0.1")
                .unwrap()
                .to_string(),
            "4294967295.0.1"
        );
        for invalid in ["1", "1.2", "1.2.3.4", "01.2.3", "1.-2.3", "4294967296.0.0"] {
            assert!(validate_package_version(invalid).is_err(), "{invalid}");
        }
    }

    #[test]
    fn display_names_use_pinned_normalization_and_security_rules() {
        assert!(validate_display_name("凌云 Game").is_ok());
        for invalid in [
            "",
            " leading",
            "trailing\u{3000}",
            "Cafe\u{301}",
            "hidden\u{200b}",
            "direction\u{202e}",
            "line\u{2028}break",
        ] {
            assert!(validate_display_name(invalid).is_err(), "{invalid:?}");
        }
        assert!(validate_display_name(&"文".repeat(128)).is_ok());
        assert!(validate_display_name(&"文".repeat(129)).is_err());
    }

    #[test]
    fn logical_paths_reject_host_and_traversal_forms() {
        for valid in ["src", "deps/math", "资源/数学"] {
            assert!(validate_logical_path(valid).is_ok(), "{valid}");
        }
        for invalid in [
            "",
            "/src",
            "C:/src",
            "http:src",
            "src\\main",
            "src//main",
            "src/./main",
            "src/../main",
            "src/\0hidden",
            "Cafe\u{301}",
        ] {
            assert!(validate_logical_path(invalid).is_err(), "{invalid:?}");
        }
    }

    #[test]
    fn toml_1_1_only_syntax_is_rejected() {
        let escaped = MINIMAL.replace("hello", "h\\x65llo");
        assert_eq!(
            parse_manifest("escape/ling.toml", escaped.as_bytes())
                .unwrap_err()
                .code(),
            codes::INVALID_PROJECT_MANIFEST_STRUCTURE
        );

        let trailing = format!("{MINIMAL}\n[dependencies]\nmath = {{ path = \"deps/math\", }}\n");
        assert_eq!(
            parse_manifest("trailing/ling.toml", trailing.as_bytes())
                .unwrap_err()
                .code(),
            codes::INVALID_PROJECT_MANIFEST_STRUCTURE
        );

        let multiline = format!("{MINIMAL}\n[dependencies]\nmath = {{\npath = \"deps/math\"\n}}\n");
        assert_eq!(
            parse_manifest("multiline/ling.toml", multiline.as_bytes())
                .unwrap_err()
                .code(),
            codes::INVALID_PROJECT_MANIFEST_STRUCTURE
        );
    }

    #[test]
    fn malformed_input_returns_an_error_without_panicking() {
        let samples: &[&[u8]] = &[
            b"",
            b"[",
            b"manifest-version = []",
            b"\xff",
            b"\0",
            b"manifest-version = 2",
        ];
        for sample in samples {
            assert!(parse_manifest("fuzz/ling.toml", sample).is_err());
        }
    }
}
