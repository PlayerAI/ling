//! Deterministic, in-memory compiler queries for Ling source snapshots.
//!
//! This crate is an internal implementation boundary. It deliberately owns no
//! host filesystem access, persistence, wire schema, CLI command, or language
//! semantics. Query results are immutable `Arc` values keyed by the exact VFS
//! snapshot and the selected workspace revisions.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use ling_ast::{LowerError, Program, lower};
use ling_cache::{CacheKey, CacheStore};
use ling_diagnostics::Diagnostic;
use ling_effects::{self, CheckedProgram, EffectError};
use ling_hir::{self, LowerError as HirLowerError};
use ling_project::PackageGraphId;
use ling_resolve::{ResolveError, ResolvedModule, ResolvedProgram, resolve};
use ling_semantic::{self, ProgramSnapshot};
use ling_source::{
    ChangeEvent, FileOrigin, FileSnapshot, InputChange, LexicalOffset, Revision, SourceError,
    SourceId, VfsError, VirtualFileSystem, WorkspaceInput, WorkspaceStateSnapshot,
};
use ling_syntax::{LexedSource, ParsedSource, lex, parse};
use ling_types::{self, TypeError};

mod checked_hover_index;
mod checked_token_source_index;
mod completion_metadata_index;
mod completion_source_index;
mod definition_index;
mod project_snapshot;
mod reference_index;
mod reference_span_index;
mod rename_identifier;
mod resolved_outline;
mod token_source_index;
mod typed_definition_index;

pub use checked_hover_index::{
    CheckedHoverEntry, CheckedHoverIndex, CheckedHoverIndexError, CheckedHoverKind,
    CheckedHoverTraitSelection, MAX_CHECKED_HOVER_ENTRIES,
};
pub use checked_token_source_index::{CheckedTokenSource, CheckedTokenSourceIndex};
pub use completion_metadata_index::{ResolvedCompletionMetadata, ResolvedCompletionMetadataIndex};
pub use completion_source_index::{
    ResolvedCompletionSource, ResolvedCompletionSourceIdentity, ResolvedCompletionSourceIndex,
    ResolvedCompletionSourceKind,
};
pub use definition_index::{
    ResolvedDefinitionIndex, ResolvedDefinitionKind, ResolvedDefinitionSymbol,
};
pub use project_snapshot::ProjectSnapshotError;
pub use reference_index::{
    ResolvedReferenceBindingTarget, ResolvedReferenceDefinitionTarget, ResolvedReferenceEntry,
    ResolvedReferenceIndex, ResolvedReferenceReverseEntry, ResolvedReferenceReverseIndex,
    ResolvedReferenceSource, ResolvedReferenceTarget, ResolvedReferenceTargetKey,
    ResolvedReferenceTargetKind,
};
pub use reference_span_index::{ResolvedReferenceSpan, ResolvedReferenceSpanIndex};
pub use rename_identifier::{
    RenameIdentifierObservation, RenameIdentifierStatus, observe_rename_identifier,
};
pub use resolved_outline::{
    MAX_RESOLVED_OUTLINE_NODES, ResolvedOutline, ResolvedOutlineError, ResolvedOutlineKind,
    ResolvedOutlineNode,
};
pub use token_source_index::{TokenSource, TokenSourceIndex, TokenSourceIndexError};
pub use typed_definition_index::{TypedDefinitionIndex, TypedDefinitionSymbol};

const LANGUAGE_VERSION: (u16, u16, u16) = (0, 1, 0);
const UNICODE_VERSION: (u8, u8, u8) = (17, 0, 0);
const QUERY_SCHEMA_VERSION: u16 = 1;

/// The query families implemented by the internal compiler database.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum QueryKind {
    SourceBytes,
    LineIndex,
    Tokens,
    TokenSourceIndex,
    CheckedTokenSourceIndex,
    Parse,
    Ast,
    Hir,
    ModuleGraph,
    Resolve,
    TypeEffect,
    Semantic,
}

/// Whether a query result was reused or computed during the current request.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum QueryOutcome {
    Hit,
    Miss,
}

/// Test-only-compatible query evidence containing no host paths or addresses.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QueryEvent {
    kind: QueryKind,
    file: SourceId,
    revision: Revision,
    outcome: QueryOutcome,
}

impl QueryEvent {
    #[must_use]
    pub const fn kind(self) -> QueryKind {
        self.kind
    }

    #[must_use]
    pub const fn file(self) -> SourceId {
        self.file
    }

    #[must_use]
    pub const fn revision(self) -> Revision {
        self.revision
    }

    #[must_use]
    pub const fn outcome(self) -> QueryOutcome {
        self.outcome
    }
}

/// A normalized lexical line index derived from one immutable source snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineIndex {
    source: SourceId,
    starts: Box<[LexicalOffset]>,
    lexical_len: LexicalOffset,
}

impl LineIndex {
    fn from_source(source: &ling_source::SourceFile) -> Self {
        let mut starts = vec![LexicalOffset::new(0)];
        starts.extend(
            source
                .lexical_text()
                .bytes()
                .enumerate()
                .filter(|(_, byte)| *byte == b'\n')
                .map(|(index, _)| {
                    LexicalOffset::new(
                        u32::try_from(index + 1).expect("source length was checked by SourceFile"),
                    )
                }),
        );
        Self {
            source: source.id(),
            starts: starts.into_boxed_slice(),
            lexical_len: source.source_map().lexical_len(),
        }
    }

    #[must_use]
    pub const fn source(&self) -> SourceId {
        self.source
    }

    #[must_use]
    pub fn line_count(&self) -> usize {
        self.starts.len()
    }

    #[must_use]
    pub fn line_start(&self, line: usize) -> Option<LexicalOffset> {
        self.starts.get(line).copied()
    }

    /// Returns the zero-based line containing a lexical byte offset.
    #[must_use]
    pub fn line_for(&self, offset: LexicalOffset) -> Option<usize> {
        if offset > self.lexical_len {
            return None;
        }
        Some(
            self.starts
                .partition_point(|line_start| *line_start <= offset)
                .saturating_sub(1),
        )
    }

    fn cache_payload(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(15 + self.starts.len() * 4);
        bytes.extend_from_slice(b"LIDX\0");
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(
            &u32::try_from(self.starts.len())
                .expect("line count is bounded by the source length")
                .to_le_bytes(),
        );
        bytes.extend_from_slice(&self.lexical_len.get().to_le_bytes());
        for start in &self.starts {
            bytes.extend_from_slice(&start.get().to_le_bytes());
        }
        bytes
    }

    fn from_cache(source: &ling_source::SourceFile, bytes: &[u8]) -> Option<Self> {
        const HEADER: usize = 5 + 2 + 4 + 4;
        if bytes.len() < HEADER || &bytes[..5] != b"LIDX\0" {
            return None;
        }
        let version = u16::from_le_bytes(bytes.get(5..7)?.try_into().ok()?);
        let count = usize::try_from(u32::from_le_bytes(bytes.get(7..11)?.try_into().ok()?)).ok()?;
        let lexical_len =
            LexicalOffset::new(u32::from_le_bytes(bytes.get(11..15)?.try_into().ok()?));
        if version != 1
            || count == 0
            || bytes.len() != HEADER.checked_add(count.checked_mul(4)?)?
            || lexical_len != source.source_map().lexical_len()
        {
            return None;
        }
        let mut starts = Vec::with_capacity(count);
        let mut cursor = HEADER;
        for _ in 0..count {
            let end = cursor.checked_add(4)?;
            starts.push(LexicalOffset::new(u32::from_le_bytes(
                bytes.get(cursor..end)?.try_into().ok()?,
            )));
            cursor = end;
        }
        if starts.first().copied() != Some(LexicalOffset::new(0))
            || starts.windows(2).any(|pair| pair[0] >= pair[1])
            || starts.iter().any(|start| *start > lexical_len)
        {
            return None;
        }
        Some(Self {
            source: source.id(),
            starts: starts.into_boxed_slice(),
            lexical_len,
        })
    }
}

/// A canonical source/module node used by the resolve query boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleNode {
    file: SourceId,
    name: String,
    imports: Box<[String]>,
    exports: Box<[String]>,
}

impl ModuleNode {
    #[must_use]
    pub const fn file(&self) -> SourceId {
        self.file
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn imports(&self) -> &[String] {
        &self.imports
    }

    #[must_use]
    pub fn exports(&self) -> &[String] {
        &self.exports
    }
}

/// A canonical directed module import edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleEdge {
    from: String,
    to: String,
}

impl ModuleEdge {
    #[must_use]
    pub fn from(&self) -> &str {
        &self.from
    }

    #[must_use]
    pub fn to(&self) -> &str {
        &self.to
    }
}

/// An immutable module graph derived from the current VFS/HIR snapshots.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleGraph {
    nodes: Box<[ModuleNode]>,
    edges: Box<[ModuleEdge]>,
}

impl ModuleGraph {
    #[must_use]
    pub fn nodes(&self) -> &[ModuleNode] {
        &self.nodes
    }

    #[must_use]
    pub fn edges(&self) -> &[ModuleEdge] {
        &self.edges
    }

    #[must_use]
    pub fn node(&self, file: SourceId) -> Option<&ModuleNode> {
        self.nodes.iter().find(|node| node.file == file)
    }

    #[must_use]
    pub fn node_by_name(&self, name: &str) -> Option<&ModuleNode> {
        self.nodes.iter().find(|node| node.name == name)
    }
}

/// A stable type/effect summary for one resolved module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeEffectDefinition {
    name: String,
    type_display: String,
    effects: Box<[String]>,
}

impl TypeEffectDefinition {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn type_display(&self) -> &str {
        &self.type_display
    }

    #[must_use]
    pub fn effects(&self) -> &[String] {
        &self.effects
    }
}

/// Immutable type/effect information projected to one module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeEffectModule {
    name: String,
    definitions: Box<[TypeEffectDefinition]>,
    capabilities: Box<[String]>,
}

impl TypeEffectModule {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn definitions(&self) -> &[TypeEffectDefinition] {
        &self.definitions
    }

    #[must_use]
    pub fn definition(&self, name: &str) -> Option<&TypeEffectDefinition> {
        self.definitions
            .iter()
            .find(|definition| definition.name == name)
    }

    #[must_use]
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }
}

/// A definition fragment from a canonical semantic graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticDefinitionFragment {
    name: String,
    definition_id: String,
    body_id: String,
    type_name: String,
    effects: Box<[String]>,
}

impl SemanticDefinitionFragment {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn definition_id(&self) -> &str {
        &self.definition_id
    }

    #[must_use]
    pub fn body_id(&self) -> &str {
        &self.body_id
    }

    #[must_use]
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    #[must_use]
    pub fn effects(&self) -> &[String] {
        &self.effects
    }
}

/// A module-local semantic graph fragment. The full canonical writer remains
/// available through [`CompilerDb::semantic_snapshot`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticModuleFragment {
    module: String,
    requires: Box<[String]>,
    imports: Box<[String]>,
    definitions: Box<[SemanticDefinitionFragment]>,
    node_ids: Box<[String]>,
    references: Box<[String]>,
}

impl SemanticModuleFragment {
    #[must_use]
    pub fn module(&self) -> &str {
        &self.module
    }

    #[must_use]
    pub fn requires(&self) -> &[String] {
        &self.requires
    }

    #[must_use]
    pub fn imports(&self) -> &[String] {
        &self.imports
    }

    #[must_use]
    pub fn definitions(&self) -> &[SemanticDefinitionFragment] {
        &self.definitions
    }

    #[must_use]
    pub fn definition(&self, name: &str) -> Option<&SemanticDefinitionFragment> {
        self.definitions
            .iter()
            .find(|definition| definition.name == name)
    }

    #[must_use]
    pub fn node_ids(&self) -> &[String] {
        &self.node_ids
    }

    #[must_use]
    pub fn references(&self) -> &[String] {
        &self.references
    }
}

/// Errors raised while materializing a query result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryError {
    UnknownFile {
        file: SourceId,
    },
    InvalidSource {
        file: SourceId,
        error: SourceError,
    },
    AstLowering {
        file: SourceId,
        error: LowerError,
    },
    HirLowering {
        file: SourceId,
        error: HirLowerError,
    },
    ResolvedModuleMissing {
        file: SourceId,
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
    SemanticSnapshot {
        message: String,
    },
    ProjectSnapshot {
        error: ProjectSnapshotError,
    },
    TokenSourceIndex {
        message: String,
    },
    ResolvedOutline {
        file: SourceId,
        error: ResolvedOutlineError,
    },
    CheckedHover {
        file: SourceId,
        error: CheckedHoverIndexError,
    },
}

impl fmt::Display for QueryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownFile { file } => write!(formatter, "unknown source file {}", file.get()),
            Self::InvalidSource { file, error } => {
                write!(formatter, "source file {} is invalid: {error}", file.get())
            }
            Self::AstLowering { file, error } => {
                write!(
                    formatter,
                    "source file {} cannot lower to AST: {error}",
                    file.get()
                )
            }
            Self::HirLowering { file, error } => {
                write!(
                    formatter,
                    "source file {} cannot lower to HIR: {error}",
                    file.get()
                )
            }
            Self::ResolvedModuleMissing { file } => {
                write!(
                    formatter,
                    "resolved module for source file {} is missing",
                    file.get()
                )
            }
            Self::Resolution { errors } => {
                write!(
                    formatter,
                    "module resolution produced {} error(s)",
                    errors.len()
                )
            }
            Self::TypeChecking { errors } => {
                write!(
                    formatter,
                    "type checking produced {} error(s)",
                    errors.len()
                )
            }
            Self::EffectChecking { errors } => {
                write!(
                    formatter,
                    "effect checking produced {} error(s)",
                    errors.len()
                )
            }
            Self::SemanticSnapshot { message } => {
                write!(formatter, "semantic snapshot failed: {message}")
            }
            Self::ProjectSnapshot { error } => error.fmt(formatter),
            Self::TokenSourceIndex { message } => {
                write!(formatter, "token source index failed: {message}")
            }
            Self::ResolvedOutline { file, error } => {
                write!(
                    formatter,
                    "resolved outline for source file {} failed: {error}",
                    file.get()
                )
            }
            Self::CheckedHover { file, error } => {
                write!(
                    formatter,
                    "checked hover index for source file {} failed: {error}",
                    file.get()
                )
            }
        }
    }
}

impl Error for QueryError {}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct QueryKey {
    compiler_version: &'static str,
    language_version: (u16, u16, u16),
    unicode_version: (u8, u8, u8),
    schema_version: u16,
    file: SourceId,
    logical_name: String,
    source_revision: Revision,
    origin: FileOrigin,
    workspace_revisions: [Option<Revision>; 4],
}

impl QueryKey {
    fn new(database: &VirtualFileSystem, snapshot: &FileSnapshot) -> Self {
        let workspace_revisions = [
            WorkspaceInput::PackageManifest,
            WorkspaceInput::Config,
            WorkspaceInput::Profile,
            WorkspaceInput::Target,
        ]
        .map(|kind| database.workspace_input(kind).map(|input| input.revision()));
        Self {
            compiler_version: env!("CARGO_PKG_VERSION"),
            language_version: LANGUAGE_VERSION,
            unicode_version: UNICODE_VERSION,
            schema_version: QUERY_SCHEMA_VERSION,
            file: snapshot.id(),
            logical_name: snapshot.logical_name().to_owned(),
            source_revision: snapshot.revision(),
            origin: snapshot.origin(),
            workspace_revisions,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ModuleHeaderKey {
    file: SourceId,
    logical_name: String,
    name: String,
    imports: Box<[String]>,
    exports: Box<[String]>,
    workspace_revisions: [Option<Revision>; 4],
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ModuleTopologyKey {
    name: String,
    imports: Box<[String]>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ModuleGraphKey {
    headers: Box<[ModuleHeaderKey]>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SurfaceKey {
    name: String,
    exports: Box<[String]>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ModuleResolveKey {
    topology: Box<[ModuleTopologyKey]>,
    file: SourceId,
    source: QueryKey,
    imported_surfaces: Box<[SurfaceKey]>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ModuleInterfaceKey {
    file: SourceId,
    logical_name: String,
    name: String,
    imports: Box<[String]>,
    requires: Box<[String]>,
    definitions: Box<[String]>,
    types: Box<[String]>,
    body_revision: Option<Revision>,
    workspace_revisions: [Option<Revision>; 4],
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WorkspaceResolveKey {
    graph: ModuleGraphKey,
    entry: String,
    sources: Box<[QueryKey]>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ProjectSemanticKey {
    graph: PackageGraphId,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct TypeEffectKey {
    graph: ModuleGraphKey,
    file: SourceId,
    source: QueryKey,
    imported_interfaces: Box<[ModuleInterfaceKey]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum TypeEffectFailure {
    Type(Box<[TypeError]>),
    Effect(Box<[EffectError]>),
}

/// Internal, deterministic compiler query database.
#[derive(Debug, Default)]
pub struct CompilerDb {
    vfs: VirtualFileSystem,
    persistent_cache: Option<CacheStore>,
    source_bytes: BTreeMap<QueryKey, Arc<FileSnapshot>>,
    sources: BTreeMap<QueryKey, Result<Arc<ling_source::SourceFile>, SourceError>>,
    line_indexes: BTreeMap<QueryKey, Arc<LineIndex>>,
    tokens: BTreeMap<QueryKey, Arc<LexedSource>>,
    parses: BTreeMap<QueryKey, Arc<ParsedSource>>,
    asts: BTreeMap<QueryKey, Result<Arc<Program>, LowerError>>,
    hirs: BTreeMap<QueryKey, Result<Arc<ling_hir::Program>, HirLowerError>>,
    module_graphs: BTreeMap<ModuleGraphKey, Arc<ModuleGraph>>,
    resolved_modules: BTreeMap<ModuleResolveKey, Result<Arc<ResolvedModule>, Box<[ResolveError]>>>,
    resolved_programs:
        BTreeMap<WorkspaceResolveKey, Result<Arc<ResolvedProgram>, Box<[ResolveError]>>>,
    checked_programs: BTreeMap<WorkspaceResolveKey, Result<Arc<CheckedProgram>, TypeEffectFailure>>,
    type_effects: BTreeMap<TypeEffectKey, Result<Arc<TypeEffectModule>, TypeEffectFailure>>,
    semantic_snapshots: BTreeMap<WorkspaceResolveKey, Result<Arc<ProgramSnapshot>, String>>,
    project_semantic_snapshots: BTreeMap<
        ProjectSemanticKey,
        Result<Arc<ling_semantic::ProjectProgramSnapshot>, ProjectSnapshotError>,
    >,
    token_source_indexes: BTreeMap<QueryKey, Result<Arc<TokenSourceIndex>, String>>,
    checked_token_source_indexes: BTreeMap<QueryKey, Arc<CheckedTokenSourceIndex>>,
    semantic_fragments: BTreeMap<String, Arc<SemanticModuleFragment>>,
    trace: Vec<QueryEvent>,
}

impl CompilerDb {
    /// Creates an empty database with no ambient filesystem or environment.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            vfs: VirtualFileSystem::new(),
            persistent_cache: None,
            source_bytes: BTreeMap::new(),
            sources: BTreeMap::new(),
            line_indexes: BTreeMap::new(),
            tokens: BTreeMap::new(),
            parses: BTreeMap::new(),
            asts: BTreeMap::new(),
            hirs: BTreeMap::new(),
            module_graphs: BTreeMap::new(),
            resolved_modules: BTreeMap::new(),
            resolved_programs: BTreeMap::new(),
            checked_programs: BTreeMap::new(),
            type_effects: BTreeMap::new(),
            semantic_snapshots: BTreeMap::new(),
            project_semantic_snapshots: BTreeMap::new(),
            token_source_indexes: BTreeMap::new(),
            checked_token_source_indexes: BTreeMap::new(),
            semantic_fragments: BTreeMap::new(),
            trace: Vec::new(),
        }
    }

    /// Creates a database with an explicit disposable persistent-cache root.
    /// Cache reads are safe misses on corruption and never deserialize
    /// unchecked compiler values.
    #[must_use]
    pub fn with_persistent_cache(root: impl Into<PathBuf>) -> Self {
        let mut database = Self::new();
        database.persistent_cache = Some(CacheStore::new(root));
        database
    }

    #[must_use]
    pub const fn vfs(&self) -> &VirtualFileSystem {
        &self.vfs
    }

    /// Captures the visible source and workspace-input state for one
    /// deterministic in-process compiler observation boundary.
    #[must_use]
    pub fn workspace_snapshot(&self) -> WorkspaceStateSnapshot {
        self.vfs.workspace_snapshot()
    }

    pub fn set_disk_snapshot(
        &mut self,
        logical_name: impl Into<String>,
        bytes: Vec<u8>,
    ) -> Result<ChangeEvent, VfsError> {
        self.vfs.set_disk_snapshot(logical_name, bytes)
    }

    pub fn open_overlay(
        &mut self,
        file: SourceId,
        bytes: Vec<u8>,
    ) -> Result<ChangeEvent, VfsError> {
        self.vfs.open_overlay(file, bytes)
    }

    pub fn close_overlay(&mut self, file: SourceId) -> Result<ChangeEvent, VfsError> {
        self.vfs.close_overlay(file)
    }

    pub fn set_workspace_input(
        &mut self,
        kind: WorkspaceInput,
        bytes: Vec<u8>,
    ) -> Result<InputChange, VfsError> {
        self.vfs.set_workspace_input(kind, bytes)
    }

    /// Returns the immutable source-byte query result for the visible layer.
    pub fn source_bytes(&mut self, file: SourceId) -> Result<Arc<FileSnapshot>, QueryError> {
        let snapshot = self
            .vfs
            .snapshot(file)
            .ok_or(QueryError::UnknownFile { file })?;
        let key = QueryKey::new(&self.vfs, &snapshot);
        if let Some(cached) = self.source_bytes.get(&key).cloned() {
            self.record(QueryKind::SourceBytes, &snapshot, QueryOutcome::Hit);
            return Ok(cached);
        }
        let snapshot = Arc::new(snapshot);
        self.source_bytes.insert(key, snapshot.clone());
        self.record(QueryKind::SourceBytes, &snapshot, QueryOutcome::Miss);
        Ok(snapshot)
    }

    /// Returns the normalized lexical line index for one source revision.
    pub fn line_index(&mut self, file: SourceId) -> Result<Arc<LineIndex>, QueryError> {
        let (key, snapshot, source) = self.source(file)?;
        if let Some(cached) = self.line_indexes.get(&key).cloned() {
            self.record(QueryKind::LineIndex, &snapshot, QueryOutcome::Hit);
            return Ok(cached);
        }
        let persistent_key = self.persistent_cache_key("line_index", &snapshot);
        if let Some(cache) = self.persistent_cache.clone() {
            if let Some(cached) = cache
                .load(&persistent_key)
                .and_then(|payload| LineIndex::from_cache(&source, &payload))
            {
                let cached = Arc::new(cached);
                self.line_indexes.insert(key, cached.clone());
                self.record(QueryKind::LineIndex, &snapshot, QueryOutcome::Hit);
                return Ok(cached);
            }
        }
        let index = Arc::new(LineIndex::from_source(&source));
        self.line_indexes.insert(key, index.clone());
        if let Some(cache) = self.persistent_cache.clone() {
            let _ = cache.store(&persistent_key, &index.cache_payload());
        }
        self.record(QueryKind::LineIndex, &snapshot, QueryOutcome::Miss);
        Ok(index)
    }

    /// Returns the lossless lexer result for one source revision.
    pub fn tokens(&mut self, file: SourceId) -> Result<Arc<LexedSource>, QueryError> {
        let (key, snapshot, source) = self.source(file)?;
        if let Some(cached) = self.tokens.get(&key).cloned() {
            self.record(QueryKind::Tokens, &snapshot, QueryOutcome::Hit);
            return Ok(cached);
        }
        let tokens = Arc::new(lex(&source));
        self.tokens.insert(key, tokens.clone());
        self.record(QueryKind::Tokens, &snapshot, QueryOutcome::Miss);
        Ok(tokens)
    }

    /// Builds an immutable original-byte lexical inventory for one source.
    /// This is a source observation for future semantic-token analysis, not an
    /// LSP semantic-token legend, range response, or protocol value.
    pub fn token_source_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<TokenSourceIndex>, QueryError> {
        let (key, snapshot, source) = self.source(file)?;
        if let Some(cached) = self.token_source_indexes.get(&key).cloned() {
            self.record(QueryKind::TokenSourceIndex, &snapshot, QueryOutcome::Hit);
            return cached.map_err(|message| QueryError::TokenSourceIndex { message });
        }
        let lexed = self.tokens(file)?;
        let result = TokenSourceIndex::from_lexed(
            source.id(),
            snapshot.logical_name().to_owned(),
            source.original_text(),
            &lexed,
        )
        .map(Arc::new)
        .map_err(|error| error.to_string());
        if let Ok(index) = &result {
            self.token_source_indexes.insert(key, Ok(index.clone()));
        }
        self.record(QueryKind::TokenSourceIndex, &snapshot, QueryOutcome::Miss);
        result.map_err(|message| QueryError::TokenSourceIndex { message })
    }

    /// Joins lexical source entries with exact checked definition facts. This
    /// is an internal identity observation, not semantic-token generation or
    /// an editor presentation model.
    pub fn checked_token_source_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<CheckedTokenSourceIndex>, QueryError> {
        let (key, snapshot, _) = self.source(file)?;
        if let Some(cached) = self.checked_token_source_indexes.get(&key).cloned() {
            self.record(
                QueryKind::CheckedTokenSourceIndex,
                &snapshot,
                QueryOutcome::Hit,
            );
            return Ok(cached);
        }
        let lexical = self.token_source_index(file)?;
        let typed = self.typed_definition_index(file)?;
        let index = Arc::new(CheckedTokenSourceIndex::from_indexes(
            &lexical,
            &typed,
            snapshot.revision(),
        ));
        self.checked_token_source_indexes.insert(key, index.clone());
        self.record(
            QueryKind::CheckedTokenSourceIndex,
            &snapshot,
            QueryOutcome::Miss,
        );
        Ok(index)
    }

    /// Returns the lossless parse result, including bounded syntax errors.
    pub fn parse(&mut self, file: SourceId) -> Result<Arc<ParsedSource>, QueryError> {
        let (key, snapshot, source) = self.source(file)?;
        if let Some(cached) = self.parses.get(&key).cloned() {
            self.record(QueryKind::Parse, &snapshot, QueryOutcome::Hit);
            return Ok(cached);
        }
        let parsed = Arc::new(parse(&source));
        self.parses.insert(key, parsed.clone());
        self.record(QueryKind::Parse, &snapshot, QueryOutcome::Miss);
        Ok(parsed)
    }

    /// Computes the complete registered compiler diagnostic set for the
    /// current visible workspace without filesystem or environment access.
    ///
    /// Lexical errors take precedence over parse errors within one source. If
    /// any source has a syntax error, semantic checking is skipped for the
    /// complete workspace so unchecked syntax cannot create cascades.
    pub fn workspace_diagnostics(&mut self) -> Result<Box<[Diagnostic]>, QueryError> {
        let snapshots = self.vfs.snapshots();
        let mut diagnostics = Vec::new();
        for snapshot in &snapshots {
            let parsed = self.parse(snapshot.id())?;
            if parsed.lexical_errors().is_empty() {
                diagnostics.extend(
                    parsed
                        .parse_errors()
                        .iter()
                        .map(|error| error.to_diagnostic(snapshot.logical_name())),
                );
            } else {
                diagnostics.extend(
                    parsed
                        .lexical_errors()
                        .iter()
                        .map(|error| error.to_diagnostic(snapshot.logical_name())),
                );
            }
        }
        if !diagnostics.is_empty() || snapshots.is_empty() {
            return Ok(diagnostics.into_boxed_slice());
        }

        let (graph_key, graph) = match self.module_graph_query() {
            Ok(graph) => graph,
            Err(QueryError::HirLowering { file, error }) => {
                let source_name = snapshots
                    .iter()
                    .find(|snapshot| snapshot.id() == file)
                    .map_or("<unknown>", FileSnapshot::logical_name);
                return Ok(
                    vec![project_snapshot::hir_error_diagnostic(source_name, &error)]
                        .into_boxed_slice(),
                );
            }
            Err(error) => return Err(error),
        };
        let entry = graph
            .nodes()
            .first()
            .ok_or(QueryError::ResolvedModuleMissing {
                file: snapshots[0].id(),
            })?
            .name()
            .to_owned();
        match self.checked_workspace(&graph_key, &graph, &entry) {
            Ok(_) => Ok(Box::new([])),
            Err(QueryError::Resolution { errors }) => Ok(errors
                .iter()
                .map(ResolveError::to_diagnostic)
                .collect::<Vec<_>>()
                .into_boxed_slice()),
            Err(QueryError::TypeChecking { errors }) => Ok(errors
                .iter()
                .map(TypeError::to_diagnostic)
                .collect::<Vec<_>>()
                .into_boxed_slice()),
            Err(QueryError::EffectChecking { errors }) => Ok(errors
                .iter()
                .map(EffectError::to_diagnostic)
                .collect::<Vec<_>>()
                .into_boxed_slice()),
            Err(error) => Err(error),
        }
    }

    /// Returns the AST lowered from a valid parse result.
    pub fn ast(&mut self, file: SourceId) -> Result<Arc<Program>, QueryError> {
        let (key, snapshot, source) = self.source(file)?;
        if let Some(cached) = self.asts.get(&key).cloned() {
            self.record(QueryKind::Ast, &snapshot, QueryOutcome::Hit);
            return cached.map_err(|error| QueryError::AstLowering { file, error });
        }
        let parsed = self.parse(file)?;
        let result = lower(&source, &parsed).map(Arc::new);
        self.asts.insert(key, result.clone());
        self.record(QueryKind::Ast, &snapshot, QueryOutcome::Miss);
        result.map_err(|error| QueryError::AstLowering { file, error })
    }

    /// Returns unresolved HIR lowered from the valid AST query.
    pub fn hir(&mut self, file: SourceId) -> Result<Arc<ling_hir::Program>, QueryError> {
        let (key, snapshot, _) = self.source(file)?;
        if let Some(cached) = self.hirs.get(&key).cloned() {
            self.record(QueryKind::Hir, &snapshot, QueryOutcome::Hit);
            return cached.map_err(|error| QueryError::HirLowering { file, error });
        }
        let ast = self.ast(file)?;
        let result = ling_hir::lower(snapshot.logical_name().to_owned(), &ast).map(Arc::new);
        self.hirs.insert(key, result.clone());
        self.record(QueryKind::Hir, &snapshot, QueryOutcome::Miss);
        result.map_err(|error| QueryError::HirLowering { file, error })
    }

    /// Returns the canonical module graph derived from the current HIR set.
    pub fn module_graph(&mut self) -> Result<Arc<ModuleGraph>, QueryError> {
        let (_, graph) = self.module_graph_query()?;
        Ok(graph)
    }

    /// Resolves one module body against the current module set and import
    /// surfaces. Private body edits only change that module's resolve key;
    /// imported surface changes invalidate its dependents.
    pub fn resolve_module(&mut self, file: SourceId) -> Result<Arc<ResolvedModule>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let snapshot = self.source_bytes(file)?;
        let source_key = QueryKey::new(&self.vfs, &snapshot);
        let topology = graph_key
            .headers
            .iter()
            .map(|header| ModuleTopologyKey {
                name: header.name.clone(),
                imports: header.imports.clone(),
            })
            .collect::<Vec<_>>();
        let mut imported_surfaces = node
            .imports
            .iter()
            .map(|name| {
                graph.node_by_name(name).map_or_else(
                    || SurfaceKey {
                        name: name.clone(),
                        exports: Box::new([]),
                    },
                    |target| SurfaceKey {
                        name: target.name.clone(),
                        exports: target.exports.clone(),
                    },
                )
            })
            .collect::<Vec<_>>();
        imported_surfaces.sort();
        imported_surfaces.dedup();
        let key = ModuleResolveKey {
            topology: topology.into_boxed_slice(),
            file,
            source: source_key,
            imported_surfaces: imported_surfaces.into_boxed_slice(),
        };
        if let Some(cached) = self.resolved_modules.get(&key).cloned() {
            return cached.map_err(|errors| QueryError::Resolution { errors });
        }

        let result = self
            .resolved_workspace(&graph_key, &graph, &node.name)?
            .modules()
            .iter()
            .find(|module| module.hir.module.name.normalized() == node.name)
            .cloned()
            .map(Arc::new)
            .ok_or(QueryError::ResolvedModuleMissing { file });
        if let Ok(module) = &result {
            self.resolved_modules.insert(key, Ok(module.clone()));
        }
        result
    }

    /// Builds an immutable source-order inventory from the validated resolver
    /// result. This is an internal compiler observation, not an LSP response.
    pub fn resolved_definition_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<ResolvedDefinitionIndex>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let resolved = self.resolved_workspace(&graph_key, &graph, &node.name)?;
        Ok(Arc::new(ResolvedDefinitionIndex::from_resolved(&resolved)))
    }

    /// Builds one bounded module-rooted outline from validated resolved HIR.
    ///
    /// The result owns only compiler structural kinds and original byte spans;
    /// it contains no editor position, URI, lifecycle, or wire representation.
    pub fn resolved_outline(&mut self, file: SourceId) -> Result<Arc<ResolvedOutline>, QueryError> {
        let snapshot = self.source_bytes(file)?;
        let original =
            std::str::from_utf8(snapshot.bytes()).map_err(|_| QueryError::ResolvedOutline {
                file,
                error: ResolvedOutlineError::InvalidSpan,
            })?;
        let module = self.resolve_module(file)?;
        ResolvedOutline::from_module(&module, original)
            .map(Arc::new)
            .map_err(|error| QueryError::ResolvedOutline { file, error })
    }

    /// Builds an immutable source/module-order inventory of resolved
    /// references and their resolver-owned targets. This is not navigation.
    pub fn resolved_reference_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<ResolvedReferenceIndex>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let resolved = self.resolved_workspace(&graph_key, &graph, &node.name)?;
        Ok(Arc::new(ResolvedReferenceIndex::from_resolved(&resolved)))
    }

    /// Builds an immutable target-to-source reverse observation over resolved
    /// references. This is not an editor references response or cache.
    pub fn resolved_reference_reverse_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<ResolvedReferenceReverseIndex>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let resolved = self.resolved_workspace(&graph_key, &graph, &node.name)?;
        Ok(Arc::new(ResolvedReferenceReverseIndex::from_resolved(
            &resolved,
        )))
    }

    /// Builds an immutable original-byte span observation for resolved
    /// references. This is not an editor range or a rename edit.
    pub fn resolved_reference_span_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<ResolvedReferenceSpanIndex>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let resolved = self.resolved_workspace(&graph_key, &graph, &node.name)?;
        Ok(Arc::new(ResolvedReferenceSpanIndex::from_resolved(
            &resolved,
        )))
    }

    /// Builds an immutable inventory of resolver-backed names for future
    /// completion analysis. This is not a completion response or edit.
    pub fn resolved_completion_source_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<ResolvedCompletionSourceIndex>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let resolved = self.resolved_workspace(&graph_key, &graph, &node.name)?;
        Ok(Arc::new(ResolvedCompletionSourceIndex::from_resolved(
            &resolved,
        )))
    }

    /// Builds an immutable checked metadata observation for future completion
    /// resolve analysis. This is not a completion item or documentation.
    pub fn resolved_completion_metadata_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<ResolvedCompletionMetadataIndex>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let checked = self.checked_workspace(&graph_key, &graph, &node.name)?;
        Ok(Arc::new(ResolvedCompletionMetadataIndex::from_checked(
            &checked,
        )))
    }

    /// Builds an immutable source-order observation of checked user
    /// definitions. This is not an LSP hover response or presentation model.
    pub fn typed_definition_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<TypedDefinitionIndex>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let checked = self.checked_workspace(&graph_key, &graph, &node.name)?;
        Ok(Arc::new(TypedDefinitionIndex::from_checked(&checked)))
    }

    /// Builds deterministic checked hover targets for one resolved workspace.
    ///
    /// The result retains compiler facts and original byte spans only. It has
    /// no URI, editor position, markup, localization, or wire behavior.
    pub fn checked_hover_index(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<CheckedHoverIndex>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let checked = self.checked_workspace(&graph_key, &graph, &node.name)?;
        CheckedHoverIndex::from_checked(&checked)
            .map(Arc::new)
            .map_err(|error| QueryError::CheckedHover { file, error })
    }

    /// Type-checks and effect-checks one module against the current resolved
    /// workspace, returning only that module's immutable public projection.
    /// Imported interface keys intentionally omit implementation bodies, while
    /// the requested module retains its complete source key.
    pub fn type_effect(&mut self, file: SourceId) -> Result<Arc<TypeEffectModule>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let (source_key, snapshot, _) = self.source(file)?;
        let interfaces = self.module_interface_keys(&graph)?;
        let imported_interfaces = imported_interface_keys(&graph, &interfaces, &node.name);
        let key = TypeEffectKey {
            graph: graph_key.clone(),
            file,
            source: source_key,
            imported_interfaces: imported_interfaces.into_boxed_slice(),
        };
        if let Some(cached) = self.type_effects.get(&key).cloned() {
            self.record(QueryKind::TypeEffect, &snapshot, QueryOutcome::Hit);
            return cached.map_err(type_effect_failure);
        }

        let checked = match self.checked_workspace(&graph_key, &graph, &node.name) {
            Ok(checked) => checked,
            Err(error @ QueryError::TypeChecking { .. })
            | Err(error @ QueryError::EffectChecking { .. }) => {
                self.type_effects.insert(
                    key,
                    Err(match &error {
                        QueryError::TypeChecking { errors } => {
                            TypeEffectFailure::Type(errors.clone())
                        }
                        QueryError::EffectChecking { errors } => {
                            TypeEffectFailure::Effect(errors.clone())
                        }
                        _ => unreachable!("checked workspace returned a checking error"),
                    }),
                );
                self.record(QueryKind::TypeEffect, &snapshot, QueryOutcome::Miss);
                return Err(error);
            }
            Err(error) => return Err(error),
        };
        let module = project_type_effect(&checked, &node.name)
            .ok_or(QueryError::ResolvedModuleMissing { file })?;
        let module = Arc::new(module);
        self.type_effects.insert(key, Ok(module.clone()));
        self.record(QueryKind::TypeEffect, &snapshot, QueryOutcome::Miss);
        Ok(module)
    }

    /// Builds and caches the canonical file-mode semantic snapshot for the
    /// workspace entry represented by `file`.
    pub fn semantic_snapshot(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<ProgramSnapshot>, QueryError> {
        let (graph_key, graph) = self.module_graph_query()?;
        let node = graph
            .node(file)
            .cloned()
            .ok_or(QueryError::UnknownFile { file })?;
        let (_, snapshot, _) = self.source(file)?;
        let key = self.workspace_resolve_key(&graph_key, &graph, &node.name)?;
        if let Some(cached) = self.semantic_snapshots.get(&key).cloned() {
            self.record(QueryKind::Semantic, &snapshot, QueryOutcome::Hit);
            return cached.map_err(|message| QueryError::SemanticSnapshot { message });
        }
        let checked = self.checked_workspace(&graph_key, &graph, &node.name)?;
        let result = ling_semantic::build((*checked).clone())
            .map(Arc::new)
            .map_err(|error| error.to_string());
        self.semantic_snapshots.insert(key, result.clone());
        self.record(QueryKind::Semantic, &snapshot, QueryOutcome::Miss);
        result.map_err(|message| QueryError::SemanticSnapshot { message })
    }

    /// Builds and caches the package-aware semantic snapshot for an already
    /// validated locked project.  This is an internal compiler observation;
    /// it does not select projects, read host paths, execute code, build
    /// artifacts, or publish a CLI/LSP response.
    pub fn project_semantic_snapshot(
        &mut self,
        project: &ling_project::LockedProject,
    ) -> Result<Arc<ling_semantic::ProjectProgramSnapshot>, QueryError> {
        let key = ProjectSemanticKey {
            graph: project.graph().id().clone(),
        };
        if let Some(cached) = self.project_semantic_snapshots.get(&key).cloned() {
            return cached.map_err(|error| QueryError::ProjectSnapshot { error });
        }
        let result = project_snapshot::build(project).map(Arc::new);
        self.project_semantic_snapshots.insert(key, result.clone());
        result.map_err(|error| QueryError::ProjectSnapshot { error })
    }

    /// Returns a canonical semantic definition/reference fragment for one
    /// module. The fragment cache is identity-based, so presentation-only
    /// edits and unrelated module body changes can reuse an identical fragment
    /// even though a full workspace program ID may change.
    pub fn semantic_fragment(
        &mut self,
        file: SourceId,
    ) -> Result<Arc<SemanticModuleFragment>, QueryError> {
        let snapshot = self.semantic_snapshot(file)?;
        let name = self
            .module_graph_query()?
            .1
            .node(file)
            .map(|node| node.name().to_owned())
            .ok_or(QueryError::UnknownFile { file })?;
        let (fragment, key) = project_semantic_fragment(&snapshot, &name)
            .ok_or(QueryError::ResolvedModuleMissing { file })?;
        let (_, source, _) = self.source(file)?;
        if let Some(cached) = self.semantic_fragments.get(&key).cloned() {
            self.record(QueryKind::Semantic, &source, QueryOutcome::Hit);
            return Ok(cached);
        }
        let fragment = Arc::new(fragment);
        self.semantic_fragments.insert(key, fragment.clone());
        self.record(QueryKind::Semantic, &source, QueryOutcome::Miss);
        Ok(fragment)
    }

    /// Parses all visible files in canonical logical-name order.
    pub fn parse_all(&mut self) -> Result<Vec<(SourceId, Arc<ParsedSource>)>, QueryError> {
        self.parse_all_with_schedule_seed(0)
    }

    fn parse_all_with_schedule_seed(
        &mut self,
        schedule_seed: u64,
    ) -> Result<Vec<(SourceId, Arc<ParsedSource>)>, QueryError> {
        let files = self
            .vfs
            .snapshots()
            .into_iter()
            .map(|snapshot| snapshot.id())
            .collect::<Vec<_>>();
        let mut keys = vec![None; files.len()];
        let mut snapshots = vec![None; files.len()];
        let mut parsed = vec![None; files.len()];
        let mut misses = Vec::new();

        for (index, file) in files.iter().copied().enumerate() {
            let (key, snapshot, source) = self.source(file)?;
            keys[index] = Some(key);
            snapshots[index] = Some(snapshot);
            if let Some(cached) = self
                .parses
                .get(keys[index].as_ref().expect("query key"))
                .cloned()
            {
                parsed[index] = Some(cached);
            } else {
                misses.push((index, source));
            }
        }

        let order = schedule_order(misses.len(), schedule_seed);
        let worker_count = thread::available_parallelism()
            .map_or(1, std::num::NonZeroUsize::get)
            .min(order.len().max(1));
        let chunk_size = order.len().div_ceil(worker_count).max(1);
        let mut computed = Vec::new();
        let misses_ref = &misses;
        thread::scope(|scope| {
            let handles = order.chunks(chunk_size).map(|chunk| {
                let misses = misses_ref;
                scope.spawn(move || {
                    chunk
                        .iter()
                        .map(|position| {
                            let (index, source) = &misses[*position];
                            (*index, Arc::new(parse(source)))
                        })
                        .collect::<Vec<_>>()
                })
            });
            for handle in handles {
                computed.extend(handle.join().expect("parallel parse worker must not panic"));
            }
        });

        for (index, result) in computed {
            parsed[index] = Some(result);
        }

        for (index, file) in files.iter().copied().enumerate() {
            let key = keys[index].take().expect("query key was collected");
            let snapshot = snapshots[index]
                .as_ref()
                .expect("source snapshot was collected");
            let result = parsed[index].take().expect("parse result was collected");
            let outcome = if misses.iter().any(|(miss, _)| *miss == index) {
                self.parses.insert(key, result.clone());
                QueryOutcome::Miss
            } else {
                QueryOutcome::Hit
            };
            self.record(QueryKind::Parse, snapshot, outcome);
            parsed[index] = Some(result);
            debug_assert_eq!(file, snapshot.id());
        }

        Ok(files
            .into_iter()
            .zip(
                parsed
                    .into_iter()
                    .map(|result| result.expect("parse result was published")),
            )
            .collect())
    }

    /// Returns test-only query evidence accumulated since the last clear.
    #[must_use]
    pub fn trace(&self) -> &[QueryEvent] {
        &self.trace
    }

    pub fn clear_trace(&mut self) {
        self.trace.clear();
    }

    fn resolved_workspace(
        &mut self,
        graph_key: &ModuleGraphKey,
        graph: &ModuleGraph,
        entry: &str,
    ) -> Result<Arc<ResolvedProgram>, QueryError> {
        let key = self.workspace_resolve_key(graph_key, graph, entry)?;
        if let Some(cached) = self.resolved_programs.get(&key).cloned() {
            return cached.map_err(|errors| QueryError::Resolution { errors });
        }
        let mut programs = Vec::with_capacity(graph.nodes().len());
        for node in graph.nodes() {
            programs.push((*self.hir(node.file)?).clone());
        }
        let result = match resolve(programs, entry) {
            Ok(program) => Ok(Arc::new(program)),
            Err(errors) => Err(errors.into_boxed_slice()),
        };
        self.resolved_programs.insert(key, result.clone());
        result.map_err(|errors| QueryError::Resolution { errors })
    }

    fn workspace_resolve_key(
        &mut self,
        graph_key: &ModuleGraphKey,
        graph: &ModuleGraph,
        entry: &str,
    ) -> Result<WorkspaceResolveKey, QueryError> {
        let mut sources = Vec::with_capacity(graph.nodes().len());
        for node in graph.nodes() {
            let (key, _, _) = self.source(node.file)?;
            sources.push(key);
        }
        Ok(WorkspaceResolveKey {
            graph: graph_key.clone(),
            entry: entry.to_owned(),
            sources: sources.into_boxed_slice(),
        })
    }

    fn checked_workspace(
        &mut self,
        graph_key: &ModuleGraphKey,
        graph: &ModuleGraph,
        entry: &str,
    ) -> Result<Arc<CheckedProgram>, QueryError> {
        let key = self.workspace_resolve_key(graph_key, graph, entry)?;
        if let Some(cached) = self.checked_programs.get(&key).cloned() {
            return cached.map_err(type_effect_failure);
        }
        let resolved = self.resolved_workspace(graph_key, graph, entry)?;
        let typed = match ling_types::check((*resolved).clone()) {
            Ok(typed) => typed,
            Err(errors) => {
                let failure = TypeEffectFailure::Type(errors.into_boxed_slice());
                self.checked_programs.insert(key, Err(failure.clone()));
                return Err(type_effect_failure(failure));
            }
        };
        let checked = match ling_effects::check(typed) {
            Ok(checked) => Arc::new(checked),
            Err(errors) => {
                let failure = TypeEffectFailure::Effect(errors.into_boxed_slice());
                self.checked_programs.insert(key, Err(failure.clone()));
                return Err(type_effect_failure(failure));
            }
        };
        self.checked_programs.insert(key, Ok(checked.clone()));
        Ok(checked)
    }

    fn module_interface_keys(
        &mut self,
        graph: &ModuleGraph,
    ) -> Result<Vec<ModuleInterfaceKey>, QueryError> {
        let mut interfaces = Vec::with_capacity(graph.nodes().len());
        for node in graph.nodes() {
            let hir = self.hir(node.file)?;
            let snapshot = self.source_bytes(node.file)?;
            let query_key = QueryKey::new(&self.vfs, &snapshot);
            let mut imports = hir
                .imports
                .iter()
                .map(|import| {
                    let mut value = String::new();
                    push_key_part(&mut value, &import.module.normalized());
                    push_key_part(&mut value, &import.alias.normalized);
                    value
                })
                .collect::<Vec<_>>();
            imports.sort();
            imports.dedup();
            let mut requires = hir
                .module
                .requires
                .iter()
                .map(|requirement| requirement.normalized())
                .collect::<Vec<_>>();
            requires.sort();
            requires.dedup();
            let mut definitions = hir
                .definitions
                .iter()
                .map(definition_interface)
                .collect::<Vec<_>>();
            definitions.sort();
            definitions.dedup();
            let mut types = hir.types.iter().map(type_interface).collect::<Vec<_>>();
            types.sort();
            types.dedup();
            let body_revision = hir
                .definitions
                .iter()
                .any(|definition| {
                    definition.annotation.is_none()
                        || expression_has_effect_surface(&definition.value)
                })
                .then_some(query_key.source_revision);
            interfaces.push(ModuleInterfaceKey {
                file: node.file,
                logical_name: snapshot.logical_name().to_owned(),
                name: node.name.clone(),
                imports: imports.into_boxed_slice(),
                requires: requires.into_boxed_slice(),
                definitions: definitions.into_boxed_slice(),
                types: types.into_boxed_slice(),
                body_revision,
                workspace_revisions: query_key.workspace_revisions,
            });
        }
        interfaces.sort();
        Ok(interfaces)
    }

    fn module_graph_query(&mut self) -> Result<(ModuleGraphKey, Arc<ModuleGraph>), QueryError> {
        let snapshots = self.vfs.snapshots();
        let mut headers = Vec::with_capacity(snapshots.len());
        let mut nodes = Vec::with_capacity(snapshots.len());
        for snapshot in snapshots {
            let file = snapshot.id();
            let hir = self.hir(file)?;
            let name = hir.module.name.normalized();
            let mut imports = hir
                .imports
                .iter()
                .map(|import| import.module.normalized())
                .collect::<Vec<_>>();
            imports.sort();
            imports.dedup();
            let mut exports = hir
                .definitions
                .iter()
                .map(|definition| definition.name.normalized.clone())
                .chain(
                    hir.types
                        .iter()
                        .map(|declaration| declaration.name.normalized.clone()),
                )
                .collect::<Vec<_>>();
            exports.sort();
            exports.dedup();
            let query_key = QueryKey::new(&self.vfs, &snapshot);
            headers.push(ModuleHeaderKey {
                file,
                logical_name: snapshot.logical_name().to_owned(),
                name: name.clone(),
                imports: imports.clone().into_boxed_slice(),
                exports: exports.clone().into_boxed_slice(),
                workspace_revisions: query_key.workspace_revisions,
            });
            nodes.push(ModuleNode {
                file,
                name,
                imports: imports.into_boxed_slice(),
                exports: exports.into_boxed_slice(),
            });
        }
        headers.sort_by(|left, right| {
            left.name
                .cmp(&right.name)
                .then_with(|| left.logical_name.cmp(&right.logical_name))
                .then_with(|| left.file.cmp(&right.file))
        });
        nodes.sort_by(|left, right| {
            left.name
                .cmp(&right.name)
                .then_with(|| left.file.cmp(&right.file))
        });
        let mut edges = nodes
            .iter()
            .flat_map(|node| {
                node.imports.iter().map(|import| ModuleEdge {
                    from: node.name.clone(),
                    to: import.clone(),
                })
            })
            .collect::<Vec<_>>();
        edges.sort_by(|left, right| {
            left.from
                .cmp(&right.from)
                .then_with(|| left.to.cmp(&right.to))
        });
        edges.dedup();
        let key = ModuleGraphKey {
            headers: headers.into_boxed_slice(),
        };
        if let Some(cached) = self.module_graphs.get(&key).cloned() {
            return Ok((key, cached));
        }
        let graph = Arc::new(ModuleGraph {
            nodes: nodes.into_boxed_slice(),
            edges: edges.into_boxed_slice(),
        });
        self.module_graphs.insert(key.clone(), graph.clone());
        Ok((key, graph))
    }

    fn source(
        &mut self,
        file: SourceId,
    ) -> Result<(QueryKey, Arc<FileSnapshot>, Arc<ling_source::SourceFile>), QueryError> {
        let snapshot = self.source_bytes(file)?;
        let key = QueryKey::new(&self.vfs, &snapshot);
        if let Some(cached) = self.sources.get(&key).cloned() {
            return cached
                .map(|source| (key, snapshot, source))
                .map_err(|error| QueryError::InvalidSource { file, error });
        }
        let result = ling_source::SourceFile::from_bytes(
            snapshot.id(),
            snapshot.logical_name().to_owned(),
            snapshot.bytes().to_vec(),
        )
        .map(Arc::new);
        self.sources.insert(key.clone(), result.clone());
        result
            .map(|source| (key, snapshot, source))
            .map_err(|error| QueryError::InvalidSource { file, error })
    }

    fn persistent_cache_key(&self, query: &str, snapshot: &FileSnapshot) -> CacheKey {
        let package = self.workspace_cache_dimension(WorkspaceInput::PackageManifest);
        let lock = self.workspace_cache_dimension(WorkspaceInput::PackageLock);
        let config = self.workspace_cache_dimension(WorkspaceInput::Config);
        let profile = self.workspace_cache_dimension(WorkspaceInput::Profile);
        let target = self.workspace_cache_dimension(WorkspaceInput::Target);
        CacheKey::new(
            env!("CARGO_PKG_VERSION"),
            [LANGUAGE_VERSION.0, LANGUAGE_VERSION.1, LANGUAGE_VERSION.2],
            [UNICODE_VERSION.0, UNICODE_VERSION.1, UNICODE_VERSION.2],
            QUERY_SCHEMA_VERSION,
            format!("profile={profile};package={package};lock={lock};config={config}"),
            format!("target={target}"),
            query,
            snapshot.logical_name(),
            snapshot.bytes(),
        )
    }

    fn workspace_cache_dimension(&self, kind: WorkspaceInput) -> String {
        self.vfs
            .workspace_input(kind)
            .map(|input| ling_cache::bytes_digest(input.bytes()))
            .unwrap_or_else(|| "none".to_owned())
    }

    fn record(&mut self, kind: QueryKind, snapshot: &FileSnapshot, outcome: QueryOutcome) {
        self.trace.push(QueryEvent {
            kind,
            file: snapshot.id(),
            revision: snapshot.revision(),
            outcome,
        });
    }
}

fn type_effect_failure(failure: TypeEffectFailure) -> QueryError {
    match failure {
        TypeEffectFailure::Type(errors) => QueryError::TypeChecking { errors },
        TypeEffectFailure::Effect(errors) => QueryError::EffectChecking { errors },
    }
}

fn imported_interface_keys(
    graph: &ModuleGraph,
    interfaces: &[ModuleInterfaceKey],
    target: &str,
) -> Vec<ModuleInterfaceKey> {
    let mut pending = graph
        .node_by_name(target)
        .map(|node| {
            node.imports()
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();
    let mut visited = std::collections::BTreeSet::new();
    let mut output = Vec::new();
    while let Some(name) = pending.iter().next().cloned() {
        pending.remove(&name);
        if !visited.insert(name.clone()) {
            continue;
        }
        if let Some(interface) = interfaces.iter().find(|interface| interface.name == name) {
            output.push(interface.clone());
        }
        if let Some(node) = graph.node_by_name(&name) {
            pending.extend(node.imports().iter().cloned());
        }
    }
    output.sort();
    output
}

fn project_type_effect(checked: &CheckedProgram, name: &str) -> Option<TypeEffectModule> {
    let resolved = checked.typed().resolved();
    let module = resolved
        .modules()
        .iter()
        .find(|module| module.hir.module.name.normalized() == name)?;
    let mut definitions = Vec::with_capacity(module.hir.definitions.len());
    for definition in &module.hir.definitions {
        let id = resolved.definition_id(module.id, &definition.name.normalized)?;
        let type_id = checked.typed().definition_type(id)?;
        let effects = checked
            .definition_effect(id)
            .map_or_else(Vec::new, |row| row.canonical_names());
        definitions.push(TypeEffectDefinition {
            name: definition.name.normalized.clone(),
            type_display: checked.typed().display_type(type_id),
            effects: effects.into_boxed_slice(),
        });
    }
    definitions.sort_by(|left, right| left.name.cmp(&right.name));
    let capabilities = checked
        .module_capabilities(module.id)
        .into_iter()
        .flatten()
        .map(|capability| capability.name().to_owned())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Some(TypeEffectModule {
        name: name.to_owned(),
        definitions: definitions.into_boxed_slice(),
        capabilities,
    })
}

fn project_semantic_fragment(
    snapshot: &ProgramSnapshot,
    name: &str,
) -> Option<(SemanticModuleFragment, String)> {
    let graph = snapshot.graph();
    let module = graph.modules.iter().find(|module| module.name == name)?;
    let mut requires = module.requires.clone();
    requires.sort();
    requires.dedup();
    let mut imports = module
        .imports
        .iter()
        .map(|import| {
            let mut value = String::new();
            push_key_part(&mut value, &import.alias);
            push_key_part(&mut value, &import.module);
            value
        })
        .collect::<Vec<_>>();
    imports.sort();
    imports.dedup();

    let mut definitions = graph
        .definitions
        .iter()
        .filter(|definition| definition.module == name)
        .map(|definition| SemanticDefinitionFragment {
            name: definition.name.clone(),
            definition_id: definition.definition_id.clone(),
            body_id: definition.body_id.clone(),
            type_name: definition.type_name.clone(),
            effects: definition.effects.clone().into_boxed_slice(),
        })
        .collect::<Vec<_>>();
    definitions.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.definition_id.cmp(&right.definition_id))
    });
    let mut node_ids = graph
        .nodes
        .iter()
        .filter(|node| node.module == name)
        .map(|node| node.node_id.clone())
        .collect::<Vec<_>>();
    node_ids.sort();
    node_ids.dedup();
    let mut references = graph
        .references
        .iter()
        .filter(|reference| reference.module == name)
        .map(|reference| {
            let mut value = String::new();
            push_key_part(&mut value, &reference.source_kind);
            push_key_part(&mut value, &reference.reference.to_string());
            push_key_part(&mut value, &reference.target_kind);
            push_key_part(&mut value, &reference.target);
            value
        })
        .collect::<Vec<_>>();
    references.sort();
    references.dedup();

    let fragment = SemanticModuleFragment {
        module: name.to_owned(),
        requires: requires.into_boxed_slice(),
        imports: imports.clone().into_boxed_slice(),
        definitions: definitions.into_boxed_slice(),
        node_ids: node_ids.into_boxed_slice(),
        references: references.clone().into_boxed_slice(),
    };
    let mut key = String::new();
    push_key_part(&mut key, &fragment.module);
    for require in &fragment.requires {
        push_key_part(&mut key, require);
    }
    for import in imports {
        push_key_part(&mut key, &import);
    }
    for definition in &fragment.definitions {
        push_key_part(&mut key, &definition.name);
        push_key_part(&mut key, &definition.definition_id);
        push_key_part(&mut key, &definition.body_id);
        push_key_part(&mut key, &definition.type_name);
        for effect in &definition.effects {
            push_key_part(&mut key, effect);
        }
    }
    for node_id in &fragment.node_ids {
        push_key_part(&mut key, node_id);
    }
    for reference in references {
        push_key_part(&mut key, &reference);
    }
    Some((fragment, key))
}

fn definition_interface(definition: &ling_hir::Definition) -> String {
    let mut output = String::from("definition");
    push_key_part(&mut output, &definition.name.normalized);
    push_key_part(
        &mut output,
        if definition.mutable {
            "mutable"
        } else {
            "immutable"
        },
    );
    push_key_part(
        &mut output,
        if definition.recursive {
            "recursive"
        } else {
            "nonrecursive"
        },
    );
    for parameter in &definition.parameters {
        pattern_interface(parameter, &mut output);
    }
    match &definition.annotation {
        Some(annotation) => {
            push_key_part(&mut output, "annotation");
            type_syntax_interface(annotation, &mut output);
        }
        None => push_key_part(&mut output, "inferred"),
    }
    output
}

fn type_interface(declaration: &ling_hir::TypeDeclaration) -> String {
    let mut output = String::from("type");
    push_key_part(&mut output, &declaration.name.normalized);
    for parameter in &declaration.parameters {
        push_key_part(&mut output, &parameter.normalized);
    }
    match &declaration.definition {
        ling_hir::TypeDefinition::Record(fields) => {
            push_key_part(&mut output, "record");
            for field in fields {
                push_key_part(&mut output, &field.name.normalized);
                push_key_part(
                    &mut output,
                    if field.mutable {
                        "mutable"
                    } else {
                        "immutable"
                    },
                );
                type_syntax_interface(&field.field_type, &mut output);
            }
        }
        ling_hir::TypeDefinition::Variant(cases) => {
            push_key_part(&mut output, "variant");
            for case in cases {
                push_key_part(&mut output, &case.name.normalized);
                match &case.payload {
                    Some(payload) => type_syntax_interface(payload, &mut output),
                    None => push_key_part(&mut output, "unit"),
                }
            }
        }
        ling_hir::TypeDefinition::Alias(alias) => {
            push_key_part(&mut output, "alias");
            type_syntax_interface(alias, &mut output);
        }
    }
    output
}

fn pattern_interface(pattern: &ling_hir::Pattern, output: &mut String) {
    match &pattern.kind {
        ling_hir::PatternKind::Binding { name, .. } => {
            push_key_part(output, "binding");
            push_key_part(output, &name.normalized);
        }
        ling_hir::PatternKind::Wildcard => push_key_part(output, "wildcard"),
        ling_hir::PatternKind::Unit => push_key_part(output, "unit"),
        ling_hir::PatternKind::Literal(literal) => {
            push_key_part(output, "literal");
            literal_interface(literal, output);
        }
        ling_hir::PatternKind::Tuple(patterns) => {
            push_key_part(output, "tuple");
            for pattern in patterns {
                pattern_interface(pattern, output);
            }
        }
        ling_hir::PatternKind::Record(fields) => {
            push_key_part(output, "record");
            for field in fields {
                push_key_part(output, &field.name.normalized);
                pattern_interface(&field.pattern, output);
            }
        }
        ling_hir::PatternKind::Constructor {
            qualifier,
            name,
            arguments,
        } => {
            push_key_part(output, "constructor");
            if let Some(qualifier) = qualifier {
                push_key_part(output, &qualifier.normalized);
            }
            push_key_part(output, &name.normalized);
            for argument in arguments {
                pattern_interface(argument, output);
            }
        }
    }
}

fn literal_interface(literal: &ling_hir::Literal, output: &mut String) {
    let kind = match literal {
        ling_hir::Literal::Integer { .. } => "integer",
        ling_hir::Literal::Float(_) => "float",
        ling_hir::Literal::Text(_) => "text",
        ling_hir::Literal::Boolean(_) => "boolean",
    };
    push_key_part(output, kind);
}

fn type_syntax_interface(syntax: &ling_hir::TypeSyntax, output: &mut String) {
    for atom in &syntax.atoms {
        match atom {
            ling_hir::TypeAtom::Name(name) => {
                push_key_part(output, "name");
                push_key_part(output, &name.normalized);
            }
            ling_hir::TypeAtom::Variable(name) => {
                push_key_part(output, "variable");
                push_key_part(output, &name.normalized);
            }
            ling_hir::TypeAtom::Arrow => push_key_part(output, "arrow"),
            ling_hir::TypeAtom::Product => push_key_part(output, "product"),
            ling_hir::TypeAtom::LeftParen => push_key_part(output, "left-paren"),
            ling_hir::TypeAtom::RightParen => push_key_part(output, "right-paren"),
            ling_hir::TypeAtom::LeftAngle => push_key_part(output, "left-angle"),
            ling_hir::TypeAtom::RightAngle => push_key_part(output, "right-angle"),
            ling_hir::TypeAtom::Comma => push_key_part(output, "comma"),
            ling_hir::TypeAtom::Dot => push_key_part(output, "dot"),
        }
    }
}

fn expression_has_effect_surface(expression: &ling_hir::Expression) -> bool {
    use ling_hir::{ExpressionKind, SequenceElement};

    match &expression.kind {
        ExpressionKind::Sequence(elements) => elements.iter().any(|element| match element {
            SequenceElement::Let(binding) => expression_has_effect_surface(&binding.value),
            SequenceElement::Expression(expression) => expression_has_effect_surface(expression),
        }),
        ExpressionKind::Handle { .. } => true,
        ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            expression_has_effect_surface(condition)
                || expression_has_effect_surface(then_branch)
                || expression_has_effect_surface(else_branch)
        }
        ExpressionKind::Match { scrutinee, cases } => {
            expression_has_effect_surface(scrutinee)
                || cases.iter().any(|case| {
                    case.guard
                        .as_ref()
                        .is_some_and(expression_has_effect_surface)
                        || expression_has_effect_surface(&case.body)
                })
        }
        ExpressionKind::Assignment { .. } => true,
        ExpressionKind::Application {
            function,
            arguments,
        } => {
            expression_has_effect_surface(function)
                || arguments.iter().any(expression_has_effect_surface)
        }
        ExpressionKind::Projection { field, target, .. } => {
            field.normalized == "write" || expression_has_effect_surface(target)
        }
        ExpressionKind::Binary { left, right, .. } => {
            expression_has_effect_surface(left) || expression_has_effect_surface(right)
        }
        ExpressionKind::Unary { operand, .. } => expression_has_effect_surface(operand),
        ExpressionKind::Tuple(elements) | ExpressionKind::List(elements) => {
            elements.iter().any(expression_has_effect_surface)
        }
        ExpressionKind::Record(fields) => fields
            .iter()
            .any(|field| expression_has_effect_surface(&field.value)),
        ExpressionKind::RecordUpdate { base, fields } => {
            expression_has_effect_surface(base)
                || fields
                    .iter()
                    .any(|field| expression_has_effect_surface(&field.value))
        }
        ExpressionKind::Name { .. } | ExpressionKind::Literal(_) | ExpressionKind::Unit => false,
    }
}

fn push_key_part(output: &mut String, value: &str) {
    output.push_str(&value.len().to_string());
    output.push(':');
    output.push_str(value);
    output.push(';');
}

fn schedule_order(length: usize, seed: u64) -> Vec<usize> {
    let mut order = (0..length).collect::<Vec<_>>();
    for index in (1..length).rev() {
        let mixed = splitmix64(seed.wrapping_add(index as u64));
        let swap = (mixed % (index as u64 + 1)) as usize;
        order.swap(index, swap);
    }
    order
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut mixed = value;
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    mixed ^ (mixed >> 31)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn clean_database(source: &CompilerDb) -> CompilerDb {
        let mut clean = CompilerDb::new();
        for snapshot in source.vfs.snapshots() {
            clean
                .set_disk_snapshot(snapshot.logical_name(), snapshot.bytes().to_vec())
                .expect("canonical snapshots can be reloaded into a clean VFS");
        }
        for kind in [
            WorkspaceInput::PackageManifest,
            WorkspaceInput::PackageLock,
            WorkspaceInput::Config,
            WorkspaceInput::Profile,
            WorkspaceInput::Target,
        ] {
            if let Some(input) = source.vfs.workspace_input(kind) {
                clean
                    .set_workspace_input(kind, input.bytes().to_vec())
                    .expect("workspace inputs can be reloaded into a clean VFS");
            }
        }
        clean
    }

    fn locked_project_fixture() -> ling_project::LockedProject {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/projects/offline-lock");
        let manifest_path = root.join(ling_project::MANIFEST_FILE_NAME);
        let bytes = std::fs::read(&manifest_path).expect("locked fixture manifest is readable");
        let manifest = ling_project::parse_manifest(&manifest_path.to_string_lossy(), &bytes)
            .expect("locked fixture manifest is valid");
        ling_project::load_locked_project(&root, &manifest).expect("locked fixture is valid")
    }

    fn clean_file(source: &CompilerDb, clean: &CompilerDb, file: SourceId) -> SourceId {
        let name = source
            .vfs
            .snapshot(file)
            .expect("source exists")
            .logical_name()
            .to_owned();
        clean.vfs.file_id(&name).expect("clean source exists")
    }

    fn assert_clean_equivalent(db: &mut CompilerDb, file: SourceId) {
        let mut clean = clean_database(db);
        let clean_file = clean_file(db, &clean, file);
        let incremental_types = db.type_effect(file).expect("incremental type/effect");
        let clean_types = clean.type_effect(clean_file).expect("clean type/effect");
        assert_eq!(&*incremental_types, &*clean_types);

        let incremental_snapshot = db
            .semantic_snapshot(file)
            .expect("incremental semantic snapshot");
        let clean_snapshot = clean
            .semantic_snapshot(clean_file)
            .expect("clean semantic snapshot");
        assert_eq!(incremental_snapshot.json(), clean_snapshot.json());

        let incremental_audit = ling_format::render_audit(&incremental_snapshot.audit_model())
            .expect("incremental audit formatting");
        let clean_audit = ling_format::render_audit(&clean_snapshot.audit_model())
            .expect("clean audit formatting");
        assert_eq!(incremental_audit, clean_audit);

        let incremental_main = ling_effects::locate_main(incremental_snapshot.checked())
            .expect("incremental Main entry");
        let clean_main =
            ling_effects::locate_main(clean_snapshot.checked()).expect("clean Main entry");
        let mut incremental_console = ling_eval::MemoryConsole::default();
        let mut clean_console = ling_eval::MemoryConsole::default();
        let incremental_result = ling_eval::execute_main(
            &incremental_snapshot,
            &incremental_main,
            &mut incremental_console,
        )
        .map_err(|error| {
            error
                .to_diagnostic()
                .render_json()
                .expect("incremental runtime diagnostic JSON")
        });
        let clean_result =
            ling_eval::execute_main(&clean_snapshot, &clean_main, &mut clean_console).map_err(
                |error| {
                    error
                        .to_diagnostic()
                        .render_json()
                        .expect("clean runtime diagnostic JSON")
                },
            );
        assert_eq!(incremental_result, clean_result);
        assert_eq!(incremental_console.output(), clean_console.output());
    }

    fn file(event: ChangeEvent) -> SourceId {
        match event {
            ChangeEvent::Added { file, .. }
            | ChangeEvent::Changed { file, .. }
            | ChangeEvent::Unchanged { file, .. } => file,
        }
    }

    fn parse_events(db: &CompilerDb, file: SourceId) -> Vec<QueryEvent> {
        db.trace()
            .iter()
            .copied()
            .filter(|event| event.kind() == QueryKind::Parse && event.file() == file)
            .collect()
    }

    #[test]
    fn repeated_queries_reuse_immutable_results() {
        let mut db = CompilerDb::new();
        let file = file(
            db.set_disk_snapshot("src/Main.ling", b"let main () = ()\n".to_vec())
                .unwrap(),
        );
        let first = db.parse(file).unwrap();
        let second = db.parse(file).unwrap();
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(
            parse_events(&db, file)
                .iter()
                .map(|event| event.outcome())
                .collect::<Vec<_>>(),
            [QueryOutcome::Miss, QueryOutcome::Hit,]
        );
        assert_eq!(
            db.source_bytes(file).unwrap().bytes(),
            b"let main () = ()\n"
        );
    }

    #[test]
    fn a_source_edit_invalidates_only_that_source_and_matches_clean_parse() {
        let mut incremental = CompilerDb::new();
        let first = file(
            incremental
                .set_disk_snapshot("z/First.ling", b"let first () = ()\n".to_vec())
                .unwrap(),
        );
        let second = file(
            incremental
                .set_disk_snapshot("a/Second.ling", b"let second () = ()\n".to_vec())
                .unwrap(),
        );
        let _ = incremental.parse_all().unwrap();
        incremental.clear_trace();
        incremental
            .set_disk_snapshot("z/First.ling", b"let first () = 1\n".to_vec())
            .unwrap();
        let changed = incremental.parse(first).unwrap();
        let reused = incremental.parse(second).unwrap();
        assert_eq!(
            parse_events(&incremental, first)[0].outcome(),
            QueryOutcome::Miss
        );
        assert_eq!(
            parse_events(&incremental, second)[0].outcome(),
            QueryOutcome::Hit
        );

        let mut clean = CompilerDb::new();
        let clean_first = file(
            clean
                .set_disk_snapshot("z/First.ling", b"let first () = 1\n".to_vec())
                .unwrap(),
        );
        let clean_second = file(
            clean
                .set_disk_snapshot("a/Second.ling", b"let second () = ()\n".to_vec())
                .unwrap(),
        );
        assert_eq!(&*changed, &*clean.parse(clean_first).unwrap());
        assert_eq!(&*reused, &*clean.parse(clean_second).unwrap());
    }

    #[test]
    fn workspace_revisions_participate_in_query_identity() {
        let mut db = CompilerDb::new();
        let file = file(
            db.set_disk_snapshot("Main.ling", b"let main () = ()\n".to_vec())
                .unwrap(),
        );
        let _ = db.parse(file).unwrap();
        db.clear_trace();
        db.set_workspace_input(WorkspaceInput::PackageManifest, b"manifest-v2".to_vec())
            .unwrap();
        let _ = db.parse(file).unwrap();
        assert_eq!(parse_events(&db, file)[0].outcome(), QueryOutcome::Miss);
    }

    #[test]
    fn workspace_snapshot_captures_inputs_and_visible_sources() {
        let mut db = CompilerDb::new();
        let file = file(
            db.set_disk_snapshot("Main.ling", b"let main () = ()\n".to_vec())
                .unwrap(),
        );
        db.open_overlay(file, b"let main () = ()\n".to_vec())
            .unwrap();
        db.set_workspace_input(WorkspaceInput::Config, b"config-v1".to_vec())
            .unwrap();

        let snapshot = db.workspace_snapshot();
        assert_eq!(snapshot.file(file).unwrap().origin(), FileOrigin::Overlay);
        assert_eq!(snapshot.file(file).unwrap().bytes(), b"let main () = ()\n");
        assert_eq!(
            snapshot.input(WorkspaceInput::Config).unwrap().bytes(),
            b"config-v1"
        );
        assert_eq!(snapshot.revision(), db.vfs().revision());
    }

    #[test]
    fn token_and_ast_queries_publish_immutable_results() {
        let mut db = CompilerDb::new();
        let file = file(
            db.set_disk_snapshot("Main.ling", b"let main () = ()\n".to_vec())
                .unwrap(),
        );
        let tokens = db.tokens(file).unwrap();
        assert!(tokens.errors().is_empty());
        let ast = db.ast(file).unwrap();
        assert_eq!(ast.items.len(), 1);
        assert!(Arc::ptr_eq(&tokens, &db.tokens(file).unwrap()));
        assert!(Arc::ptr_eq(&ast, &db.ast(file).unwrap()));
        assert!(
            db.trace()
                .iter()
                .any(|event| event.kind() == QueryKind::Tokens
                    && event.outcome() == QueryOutcome::Hit)
        );
        assert!(db
            .trace()
            .iter()
            .any(|event| event.kind() == QueryKind::Ast && event.outcome() == QueryOutcome::Hit));
    }

    #[test]
    fn token_source_index_preserves_original_spelling_and_reuses_results() {
        let mut db = CompilerDb::new();
        let source = "\u{feff}module Main\r\n\r\nlet 人物 = 1\r\n";
        let file = file(
            db.set_disk_snapshot("unicode/Main.ling", source.as_bytes().to_vec())
                .unwrap(),
        );

        let first = db
            .token_source_index(file)
            .expect("valid source has a lexical index");
        let repeated = db
            .token_source_index(file)
            .expect("repeated lexical index is deterministic");
        assert!(Arc::ptr_eq(&first, &repeated));
        assert!(first.is_valid());
        assert_eq!(first.source(), file);
        assert_eq!(first.source_name(), "unicode/Main.ling");

        let person = first
            .tokens()
            .iter()
            .find(|token| token.text() == "人物")
            .expect("Chinese identifier is retained");
        assert_eq!(person.kind().name(), "identifier");
        let bytes = db.vfs.snapshot(file).expect("source snapshot exists");
        let start = usize::try_from(person.span().start().get()).unwrap();
        let end = usize::try_from(person.span().end().get()).unwrap();
        assert_eq!(&bytes.bytes()[start..end], "人物".as_bytes());
        assert!(db.trace().iter().any(|event| {
            event.kind() == QueryKind::TokenSourceIndex && event.outcome() == QueryOutcome::Hit
        }));
    }

    #[test]
    fn checked_token_source_index_joins_definition_facts_without_presentation() {
        let mut db = CompilerDb::new();
        let file = file(
            db.set_disk_snapshot(
                "checked/Main.ling",
                b"module Main\n\nlet helper = 1\n\nlet main () = helper\n".to_vec(),
            )
            .unwrap(),
        );

        let first = db
            .checked_token_source_index(file)
            .expect("checked token source index builds");
        let repeated = db
            .checked_token_source_index(file)
            .expect("checked token source index repeats");
        assert!(Arc::ptr_eq(&first, &repeated));
        assert_eq!(first.source(), file);
        assert_eq!(first.revision().get(), 1);
        assert_eq!(first.source_name(), "checked/Main.ling");
        let helper = first
            .entries()
            .iter()
            .find(|entry| entry.token().text() == "helper")
            .expect("helper token is present");
        assert!(helper.definition_id().is_some());
        assert_eq!(helper.type_display(), Some("Int"));
        assert_eq!(helper.effects(), Some([].as_slice()));
        assert!(
            first
                .entries()
                .iter()
                .any(|entry| { entry.token().text() == "main" && entry.definition_id().is_some() })
        );
        assert!(db.trace().iter().any(|event| {
            event.kind() == QueryKind::CheckedTokenSourceIndex
                && event.outcome() == QueryOutcome::Hit
        }));
    }

    #[test]
    fn malformed_utf8_is_an_error_without_partial_parse_publication() {
        let mut db = CompilerDb::new();
        let file = file(db.set_disk_snapshot("bad.ling", vec![0xff, b'\n']).unwrap());
        let error = db.parse(file).unwrap_err();
        assert!(matches!(error, QueryError::InvalidSource { .. }));
        assert!(db.ast(file).is_err());
        assert!(
            db.trace()
                .iter()
                .all(|event| event.kind() == QueryKind::SourceBytes)
        );
    }

    #[test]
    fn line_index_preserves_normalized_crlf_and_bom_boundaries() {
        let mut db = CompilerDb::new();
        let file = file(
            db.set_disk_snapshot("lines.ling", "\u{feff}人物\r\n血量\n".as_bytes().to_vec())
                .unwrap(),
        );
        let index = db.line_index(file).unwrap();
        assert_eq!(index.source(), file);
        assert_eq!(index.line_count(), 3);
        assert_eq!(index.line_start(0), Some(LexicalOffset::new(0)));
        assert_eq!(index.line_start(1), Some(LexicalOffset::new(7)));
        assert_eq!(index.line_start(2), Some(LexicalOffset::new(14)));
        assert_eq!(index.line_for(LexicalOffset::new(8)), Some(1));
        assert_eq!(index.line_for(LexicalOffset::new(99)), None);
    }

    fn persistent_cache_test_root(label: &str) -> PathBuf {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        std::env::temp_dir().join(format!(
            "ling-db-cache-{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ))
    }

    fn last_query_outcome(db: &CompilerDb, kind: QueryKind) -> QueryOutcome {
        db.trace()
            .iter()
            .rev()
            .find(|event| event.kind() == kind)
            .expect("query trace contains the requested kind")
            .outcome()
    }

    #[test]
    fn persistent_line_index_cache_reuses_checked_result_and_invalidates_source() {
        let root = persistent_cache_test_root("reuse");
        let bytes = "\u{feff}第一行\r\n第二行\n".as_bytes().to_vec();
        {
            let mut db = CompilerDb::with_persistent_cache(&root);
            let file = file(db.set_disk_snapshot("lines.ling", bytes.clone()).unwrap());
            let index = db.line_index(file).unwrap();
            assert_eq!(index.line_count(), 3);
            assert_eq!(
                last_query_outcome(&db, QueryKind::LineIndex),
                QueryOutcome::Miss
            );
        }
        {
            let mut db = CompilerDb::with_persistent_cache(&root);
            let file = file(db.set_disk_snapshot("lines.ling", bytes).unwrap());
            let reused = db.line_index(file).unwrap();
            assert_eq!(reused.source(), file);
            assert_eq!(reused.line_count(), 3);
            assert_eq!(
                last_query_outcome(&db, QueryKind::LineIndex),
                QueryOutcome::Hit
            );

            db.clear_trace();
            db.set_disk_snapshot("lines.ling", b"only one line\n".to_vec())
                .unwrap();
            let changed = db.line_index(file).unwrap();
            assert_eq!(changed.line_count(), 2);
            assert_eq!(
                last_query_outcome(&db, QueryKind::LineIndex),
                QueryOutcome::Miss
            );
        }
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn persistent_line_index_cache_treats_corruption_as_a_safe_miss() {
        let root = persistent_cache_test_root("corruption");
        let bytes = b"first\nsecond\n".to_vec();
        {
            let mut db = CompilerDb::with_persistent_cache(&root);
            let file = file(db.set_disk_snapshot("lines.ling", bytes.clone()).unwrap());
            let _ = db.line_index(file).unwrap();
        }
        let cache_file = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| path.extension().and_then(|value| value.to_str()) == Some("lcache"))
            .expect("line-index cache file is published");
        let mut encoded = std::fs::read(&cache_file).unwrap();
        let last = encoded.last_mut().expect("cache envelope has a checksum");
        *last ^= 0xFF;
        std::fs::write(&cache_file, encoded).unwrap();

        let mut db = CompilerDb::with_persistent_cache(&root);
        let file = file(db.set_disk_snapshot("lines.ling", bytes).unwrap());
        let index = db.line_index(file).unwrap();
        assert_eq!(index.line_count(), 3);
        assert_eq!(
            last_query_outcome(&db, QueryKind::LineIndex),
            QueryOutcome::Miss
        );
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn persistent_line_index_cache_key_includes_profile_and_target_inputs() {
        let root = persistent_cache_test_root("dimensions");
        let bytes = b"one\ntwo\n".to_vec();
        {
            let mut db = CompilerDb::with_persistent_cache(&root);
            db.set_workspace_input(WorkspaceInput::Profile, b"debug".to_vec())
                .unwrap();
            db.set_workspace_input(WorkspaceInput::Target, b"host".to_vec())
                .unwrap();
            let file = file(db.set_disk_snapshot("lines.ling", bytes.clone()).unwrap());
            let _ = db.line_index(file).unwrap();
        }
        {
            let mut db = CompilerDb::with_persistent_cache(&root);
            db.set_workspace_input(WorkspaceInput::Profile, b"release".to_vec())
                .unwrap();
            db.set_workspace_input(WorkspaceInput::Target, b"host".to_vec())
                .unwrap();
            let file = file(db.set_disk_snapshot("lines.ling", bytes.clone()).unwrap());
            let _ = db.line_index(file).unwrap();
            assert_eq!(
                last_query_outcome(&db, QueryKind::LineIndex),
                QueryOutcome::Miss
            );
        }
        {
            let mut db = CompilerDb::with_persistent_cache(&root);
            db.set_workspace_input(WorkspaceInput::Profile, b"debug".to_vec())
                .unwrap();
            db.set_workspace_input(WorkspaceInput::Target, b"wasm32".to_vec())
                .unwrap();
            let file = file(db.set_disk_snapshot("lines.ling", bytes).unwrap());
            let _ = db.line_index(file).unwrap();
            assert_eq!(
                last_query_outcome(&db, QueryKind::LineIndex),
                QueryOutcome::Miss
            );
        }
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn parse_all_uses_canonical_logical_name_order() {
        let mut db = CompilerDb::new();
        let z = file(
            db.set_disk_snapshot("z/Main.ling", b"let z () = ()\n".to_vec())
                .unwrap(),
        );
        let a = file(
            db.set_disk_snapshot("a/Main.ling", b"let a () = ()\n".to_vec())
                .unwrap(),
        );
        let parsed = db.parse_all().unwrap();
        assert_eq!(
            parsed.into_iter().map(|(id, _)| id).collect::<Vec<_>>(),
            [a, z]
        );
    }

    #[test]
    fn parallel_parse_scheduling_is_deterministic_across_task_seeds() {
        let mut expected = None;
        for seed in [0, 1, 7, 17, 0xDEAD_BEEF] {
            let mut db = CompilerDb::new();
            for (name, source) in [
                ("z/Main.ling", b"let main () = ()\n".as_slice()),
                ("a/Lib.ling", b"let answer = 42\n".as_slice()),
                ("m/中文.ling", "let 人物 = 1\n".as_bytes()),
                ("b/Bad.ling", b"let =\n".as_slice()),
            ] {
                db.set_disk_snapshot(name, source.to_vec()).unwrap();
            }
            let parsed = db.parse_all_with_schedule_seed(seed).unwrap();
            let observation = (
                parsed
                    .into_iter()
                    .map(|(file, parsed)| {
                        (
                            db.vfs.snapshot(file).unwrap().logical_name().to_owned(),
                            (*parsed).clone(),
                        )
                    })
                    .collect::<Vec<_>>(),
                db.trace().to_vec(),
            );
            if let Some(expected) = &expected {
                assert_eq!(expected, &observation);
            } else {
                expected = Some(observation);
            }
        }
    }

    #[test]
    fn module_graph_is_canonical_and_retains_exports_and_edges() {
        let mut db = CompilerDb::new();
        let main = file(
            db.set_disk_snapshot(
                "src/Main.ling",
                b"module Main\n\nimport Lib\n\nlet main = Lib.answer\n".to_vec(),
            )
            .unwrap(),
        );
        let lib = file(
            db.set_disk_snapshot("src/Lib.ling", b"module Lib\n\nlet answer = 42\n".to_vec())
                .unwrap(),
        );
        let graph = db.module_graph().unwrap();
        assert_eq!(
            graph
                .nodes()
                .iter()
                .map(|node| node.name())
                .collect::<Vec<_>>(),
            ["Lib", "Main"]
        );
        assert_eq!(
            graph
                .node(main)
                .unwrap()
                .imports()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["Lib"]
        );
        assert_eq!(
            graph
                .node(lib)
                .unwrap()
                .exports()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["answer"]
        );
        assert_eq!(graph.edges()[0].from(), "Main");
        assert_eq!(graph.edges()[0].to(), "Lib");
        assert!(Arc::ptr_eq(&graph, &db.module_graph().unwrap()));
    }

    #[test]
    fn resolved_definition_index_preserves_original_spans_and_source_order() {
        let mut db = CompilerDb::new();
        let source = "\u{feff}module Main\r\n\r\nlet 人物 = 1\r\ntype Choice =\r\n    | First\r\n    | Second\r\n";
        let file = file(
            db.set_disk_snapshot("unicode/Main.ling", source.as_bytes().to_vec())
                .unwrap(),
        );

        let index = db
            .resolved_definition_index(file)
            .expect("valid source resolves");
        let repeated = db
            .resolved_definition_index(file)
            .expect("repeated index resolves");
        assert_eq!(&*index, &*repeated);
        assert_eq!(
            index
                .symbols()
                .iter()
                .map(ResolvedDefinitionSymbol::name_source)
                .collect::<Vec<_>>(),
            ["人物", "Choice", "First", "Second"]
        );
        assert_eq!(
            index
                .symbols()
                .iter()
                .map(|symbol| symbol.kind().as_str())
                .collect::<Vec<_>>(),
            ["value", "type", "constructor", "constructor"]
        );

        let bytes = db.vfs.snapshot(file).expect("source snapshot exists");
        for symbol in index.symbols() {
            assert_eq!(symbol.source_name(), "unicode/Main.ling");
            assert_eq!(symbol.span().source(), file);
            let start = symbol.span().start().get() as usize;
            let end = symbol.span().end().get() as usize;
            assert_eq!(
                std::str::from_utf8(&bytes.bytes()[start..end]).unwrap(),
                symbol.name_source()
            );
        }
        assert_eq!(index.source_symbols("unicode/Main.ling").len(), 4);
        assert_eq!(index.module_symbols("Main").len(), 4);
        assert_eq!(index.name_symbols("Choice").len(), 1);
        assert!(index.module_symbols("Missing").is_empty());
        assert!(index.name_symbols("missing").is_empty());
        assert!(index.definition("missing-definition-id").is_none());
    }

    #[test]
    fn resolved_definition_index_does_not_publish_after_source_failure() {
        let mut db = CompilerDb::new();
        let file = file(db.set_disk_snapshot("bad/Main.ling", vec![0xFF]).unwrap());
        assert!(matches!(
            db.resolved_definition_index(file),
            Err(QueryError::InvalidSource { .. })
        ));
    }

    #[test]
    fn resolved_reference_index_preserves_targets_and_source_spans() {
        let mut db = CompilerDb::new();
        let source = "\u{feff}module Main\r\n\r\nlet helper = 1\r\n\r\nlet main () = helper\r\n";
        let file = file(
            db.set_disk_snapshot("unicode/Main.ling", source.as_bytes().to_vec())
                .unwrap(),
        );

        let index = db
            .resolved_reference_index(file)
            .expect("valid source resolves references");
        let repeated = db
            .resolved_reference_index(file)
            .expect("repeated references resolve");
        assert_eq!(&*index, &*repeated);
        assert!(!index.entries().is_empty());
        assert_eq!(
            index.source_entries("unicode/Main.ling").len(),
            index.entries().len()
        );

        let helper_reference = index
            .entries()
            .iter()
            .find(|entry| {
                matches!(
                    entry.target(),
                    ResolvedReferenceTarget::Definition(target) if target.name() == Some("helper")
                )
            })
            .expect("helper reference is indexed");
        assert_eq!(helper_reference.source_name(), "unicode/Main.ling");
        assert_eq!(helper_reference.source_module(), "Main");
        let ResolvedReferenceTarget::Definition(target) = helper_reference.target() else {
            unreachable!("helper target is a definition");
        };
        assert_eq!(target.source_name(), Some("unicode/Main.ling"));
        let target_span = target.span().expect("helper has a source span");
        let bytes = db.vfs.snapshot(file).expect("source snapshot exists");
        let start = target_span.start().get() as usize;
        let end = target_span.end().get() as usize;
        assert_eq!(
            std::str::from_utf8(&bytes.bytes()[start..end]).unwrap(),
            "helper"
        );
    }

    #[test]
    fn resolved_reference_index_does_not_publish_after_source_failure() {
        let mut db = CompilerDb::new();
        let file = file(db.set_disk_snapshot("bad/Main.ling", vec![0xFF]).unwrap());
        assert!(matches!(
            db.resolved_reference_index(file),
            Err(QueryError::InvalidSource { .. })
        ));
    }

    #[test]
    fn resolved_completion_source_index_does_not_publish_after_source_failure() {
        let mut db = CompilerDb::new();
        let file = file(db.set_disk_snapshot("bad/Main.ling", vec![0xFF]).unwrap());
        assert!(matches!(
            db.resolved_completion_source_index(file),
            Err(QueryError::InvalidSource { .. })
        ));
    }

    #[test]
    fn resolved_completion_metadata_index_does_not_publish_after_source_failure() {
        let mut db = CompilerDb::new();
        let file = file(db.set_disk_snapshot("bad/Main.ling", vec![0xFF]).unwrap());
        assert!(matches!(
            db.resolved_completion_metadata_index(file),
            Err(QueryError::InvalidSource { .. })
        ));
    }

    #[test]
    fn resolved_reference_reverse_index_groups_existing_targets_deterministically() {
        let mut db = CompilerDb::new();
        let source = "module Main\n\nlet helper = 1\n\nlet main () = helper\n";
        let file = file(
            db.set_disk_snapshot("src/Main.ling", source.as_bytes().to_vec())
                .unwrap(),
        );

        let forward = db
            .resolved_reference_index(file)
            .expect("valid source resolves references");
        let reverse = db
            .resolved_reference_reverse_index(file)
            .expect("valid source builds reverse index");
        let repeated = db
            .resolved_reference_reverse_index(file)
            .expect("repeated reverse index is deterministic");
        assert_eq!(&*reverse, &*repeated);

        let helper_reference = forward
            .entries()
            .iter()
            .find(|entry| {
                matches!(
                    entry.target(),
                    ResolvedReferenceTarget::Definition(target) if target.name() == Some("helper")
                )
            })
            .expect("helper reference is indexed");
        let target_key = helper_reference.target().key();
        let grouped = reverse
            .target(&target_key)
            .expect("helper target is grouped");
        assert_eq!(grouped.target(), &target_key);
        assert_eq!(grouped.sources().len(), 1);
        assert_eq!(grouped.sources()[0].source_name(), "src/Main.ling");
        assert_eq!(
            grouped.sources()[0].reference_id(),
            helper_reference.reference_id()
        );
    }

    #[test]
    fn resolved_reference_reverse_index_does_not_publish_after_source_failure() {
        let mut db = CompilerDb::new();
        let file = file(db.set_disk_snapshot("bad/Main.ling", vec![0xFF]).unwrap());
        assert!(matches!(
            db.resolved_reference_reverse_index(file),
            Err(QueryError::InvalidSource { .. })
        ));
    }

    #[test]
    fn typed_definition_index_preserves_checked_facts_and_source_spans() {
        let mut db = CompilerDb::new();
        let source = "\u{feff}module Main\r\n    requires Console.Write\r\n\r\nlet 人物 = 1\r\n\r\nlet main () = Console.write (Text.format \"{}\" 人物)\r\n";
        let file = file(
            db.set_disk_snapshot("unicode/Main.ling", source.as_bytes().to_vec())
                .unwrap(),
        );

        let index = db
            .typed_definition_index(file)
            .expect("valid source type-checks");
        let repeated = db
            .typed_definition_index(file)
            .expect("repeated observation type-checks");
        assert_eq!(&*index, &*repeated);
        assert_eq!(
            index
                .symbols()
                .iter()
                .map(TypedDefinitionSymbol::name_source)
                .collect::<Vec<_>>(),
            ["人物", "main"]
        );

        let person = index
            .symbols()
            .iter()
            .find(|symbol| symbol.name_source() == "人物")
            .expect("Chinese definition is indexed");
        assert_eq!(person.type_display(), Some("Int"));
        assert_eq!(person.effects(), Some([].as_slice()));
        assert_eq!(
            person.capabilities(),
            Some(["Console.Write".to_owned()].as_slice())
        );

        let main = index
            .symbols()
            .iter()
            .find(|symbol| symbol.name_source() == "main")
            .expect("main definition is indexed");
        assert_eq!(
            main.effects(),
            Some(["Console.Write".to_owned()].as_slice())
        );
        assert_eq!(
            main.capabilities(),
            Some(["Console.Write".to_owned()].as_slice())
        );

        let bytes = db.vfs.snapshot(file).expect("source snapshot exists");
        for symbol in index.symbols() {
            assert_eq!(symbol.source_name(), "unicode/Main.ling");
            assert_eq!(symbol.span().source(), file);
            let start = symbol.span().start().get() as usize;
            let end = symbol.span().end().get() as usize;
            assert_eq!(
                std::str::from_utf8(&bytes.bytes()[start..end]).unwrap(),
                symbol.name_source()
            );
        }
        assert_eq!(index.source_symbols("unicode/Main.ling").len(), 2);
    }

    #[test]
    fn typed_definition_index_does_not_publish_after_source_failure() {
        let mut db = CompilerDb::new();
        let file = file(db.set_disk_snapshot("bad/Main.ling", vec![0xFF]).unwrap());
        assert!(matches!(
            db.typed_definition_index(file),
            Err(QueryError::InvalidSource { .. })
        ));
    }

    #[test]
    fn resolve_queries_reuse_private_bodies_and_invalidate_dependents_on_exports() {
        let mut db = CompilerDb::new();
        let main = file(
            db.set_disk_snapshot(
                "src/Main.ling",
                b"module Main\n\nimport Lib\n\nlet main = Lib.answer\n".to_vec(),
            )
            .unwrap(),
        );
        let lib = file(
            db.set_disk_snapshot("src/Lib.ling", b"module Lib\n\nlet answer = 42\n".to_vec())
                .unwrap(),
        );

        let first_main = db.resolve_module(main).unwrap();
        let first_lib = db.resolve_module(lib).unwrap();
        db.set_disk_snapshot("src/Lib.ling", b"module Lib\n\nlet answer = 7\n".to_vec())
            .unwrap();
        assert!(Arc::ptr_eq(&first_main, &db.resolve_module(main).unwrap()));
        assert!(!Arc::ptr_eq(&first_lib, &db.resolve_module(lib).unwrap()));

        db.set_disk_snapshot("src/Lib.ling", b"module Lib\n\nlet other = 7\n".to_vec())
            .unwrap();
        assert!(matches!(
            db.resolve_module(main),
            Err(QueryError::Resolution { .. })
        ));
    }

    #[test]
    fn type_effect_queries_project_types_and_reuse_private_imported_bodies() {
        let mut db = CompilerDb::new();
        let main = file(
            db.set_disk_snapshot(
                "src/Main.ling",
                b"module Main\n\nimport Lib\n\nlet main () = Lib.answer\n".to_vec(),
            )
            .unwrap(),
        );
        let lib = file(
            db.set_disk_snapshot(
                "src/Lib.ling",
                b"module Lib\n\nlet answer: Int = 42\n".to_vec(),
            )
            .unwrap(),
        );

        let first_main = db.type_effect(main).unwrap();
        assert_eq!(first_main.name(), "Main");
        assert_eq!(
            first_main.definition("main").unwrap().type_display(),
            "Unit -> Int"
        );
        assert!(first_main.capabilities().is_empty());
        let first_lib = db.type_effect(lib).unwrap();

        db.set_disk_snapshot(
            "src/Lib.ling",
            b"module Lib\n\nlet answer: Int = 7\n".to_vec(),
        )
        .unwrap();
        assert!(Arc::ptr_eq(&first_main, &db.type_effect(main).unwrap()));
        assert!(!Arc::ptr_eq(&first_lib, &db.type_effect(lib).unwrap()));

        db.set_disk_snapshot(
            "src/Lib.ling",
            b"module Lib\n\nlet answer: Text = \"seven\"\n".to_vec(),
        )
        .unwrap();
        let changed_main = db.type_effect(main).unwrap();
        assert!(!Arc::ptr_eq(&first_main, &changed_main));
        assert_eq!(
            changed_main.definition("main").unwrap().type_display(),
            "Unit -> Text"
        );
    }

    #[test]
    fn type_effect_queries_cache_structured_effect_failures() {
        let mut db = CompilerDb::new();
        let main = file(
            db.set_disk_snapshot(
                "Main.ling",
                b"module Main\n\nlet main () = Console.write \"x\"\n".to_vec(),
            )
            .unwrap(),
        );
        let first = db.type_effect(main).unwrap_err();
        assert!(matches!(first, QueryError::EffectChecking { .. }));
        let second = db.type_effect(main).unwrap_err();
        assert_eq!(first, second);
        assert!(db.trace().iter().any(|event| {
            event.kind() == QueryKind::TypeEffect && event.outcome() == QueryOutcome::Hit
        }));
    }

    #[test]
    fn inferred_public_type_changes_invalidate_importers() {
        let mut db = CompilerDb::new();
        let main = file(
            db.set_disk_snapshot(
                "Main.ling",
                b"module Main\n\nimport Lib\n\nlet main () = Lib.answer\n".to_vec(),
            )
            .unwrap(),
        );
        file(
            db.set_disk_snapshot("Lib.ling", b"module Lib\n\nlet answer = 42\n".to_vec())
                .unwrap(),
        );
        let first = db.type_effect(main).unwrap();
        db.set_disk_snapshot(
            "Lib.ling",
            b"module Lib\n\nlet answer = \"text\"\n".to_vec(),
        )
        .unwrap();
        let changed = db.type_effect(main).unwrap();
        assert!(!Arc::ptr_eq(&first, &changed));
        assert_eq!(
            changed.definition("main").unwrap().type_display(),
            "Unit -> Text"
        );
    }

    #[test]
    fn semantic_queries_publish_canonical_snapshots_and_identity_fragments() {
        let mut db = CompilerDb::new();
        let main = file(
            db.set_disk_snapshot(
                "src/Main.ling",
                b"module Main\n\nlet callee value = value + 1\nlet caller value = callee value\n"
                    .to_vec(),
            )
            .unwrap(),
        );
        let first_snapshot = db.semantic_snapshot(main).unwrap();
        assert!(!first_snapshot.program_id().to_string().is_empty());
        assert_eq!(
            ling_semantic::read_json(first_snapshot.json()).unwrap(),
            *first_snapshot.graph()
        );
        assert!(Arc::ptr_eq(
            &first_snapshot,
            &db.semantic_snapshot(main).unwrap()
        ));
        let first_fragment = db.semantic_fragment(main).unwrap();
        assert_eq!(first_fragment.module(), "Main");
        assert_eq!(first_fragment.definitions().len(), 2);
        assert!(first_fragment.definition("callee").is_some());
        assert!(Arc::ptr_eq(
            &first_fragment,
            &db.semantic_fragment(main).unwrap()
        ));

        db.set_disk_snapshot(
            "src/Main.ling",
            b"module Main\n\n// presentation-only\nlet callee value =\n    value + 1\nlet caller value = callee value\n"
                .to_vec(),
        )
        .unwrap();
        let comment_snapshot = db.semantic_snapshot(main).unwrap();
        assert_eq!(first_snapshot.program_id(), comment_snapshot.program_id());
        let comment_fragment = db.semantic_fragment(main).unwrap();
        assert!(!Arc::ptr_eq(&first_fragment, &comment_fragment));
        assert_eq!(
            first_fragment.definition("callee").unwrap().body_id(),
            comment_fragment.definition("callee").unwrap().body_id()
        );
    }

    #[test]
    fn project_semantic_snapshot_is_locked_path_free_and_cached() {
        let project = locked_project_fixture();
        let mut db = CompilerDb::new();
        let first = db
            .project_semantic_snapshot(&project)
            .expect("locked project sources resolve and type-check");
        let repeated = db
            .project_semantic_snapshot(&project)
            .expect("repeated locked project snapshot succeeds");

        assert!(Arc::ptr_eq(&first, &repeated));
        assert_eq!(
            first.graph().package_graph_id.as_deref(),
            Some(project.graph().id().as_str())
        );
        assert_eq!(
            first.graph().packages.len(),
            project.graph().packages().len()
        );
        assert_eq!(first.graph().schema, ling_semantic::PROJECT_SEMANTIC_SCHEMA);
        assert!(!first.json().contains("offline-lock"));
        assert!(!first.json().contains("\\"));
    }

    #[test]
    fn semantic_fragments_reuse_dependents_when_an_imported_body_changes() {
        let mut db = CompilerDb::new();
        let main = file(
            db.set_disk_snapshot(
                "Main.ling",
                b"module Main\n\nimport Lib\n\nlet main () = Lib.answer\n".to_vec(),
            )
            .unwrap(),
        );
        file(
            db.set_disk_snapshot("Lib.ling", b"module Lib\n\nlet answer: Int = 42\n".to_vec())
                .unwrap(),
        );
        let first_snapshot = db.semantic_snapshot(main).unwrap();
        let first_fragment = db.semantic_fragment(main).unwrap();
        let first_body = first_fragment.definition("main").unwrap().body_id();

        db.set_disk_snapshot("Lib.ling", b"module Lib\n\nlet answer: Int = 7\n".to_vec())
            .unwrap();
        let changed_snapshot = db.semantic_snapshot(main).unwrap();
        let changed_fragment = db.semantic_fragment(main).unwrap();
        assert!(!Arc::ptr_eq(&first_snapshot, &changed_snapshot));
        assert!(Arc::ptr_eq(&first_fragment, &changed_fragment));
        assert_eq!(
            first_body,
            changed_fragment.definition("main").unwrap().body_id()
        );
        assert_ne!(first_snapshot.program_id(), changed_snapshot.program_id());
    }

    #[test]
    fn clean_and_incremental_pipelines_match_across_deterministic_edit_sequence() {
        let mut db = CompilerDb::new();
        let main = file(
            db.set_disk_snapshot(
                "src/Main.ling",
                b"module Main\n    requires Console.Write\n\nlet helper = 1\n\nlet main () = Console.write (Text.format \"{}\" helper)\n"
                    .to_vec(),
            )
            .unwrap(),
        );
        assert_clean_equivalent(&mut db, main);

        for edit in [
            b"module Main\n    requires Console.Write\n\nlet helper = 2\n\nlet main () = Console.write (Text.format \"{}\" helper)\n".as_slice(),
            b"module Main\r\n    requires Console.Write\r\n\r\n// presentation edit\r\nlet helper = 2\r\n\r\nlet main () = Console.write (Text.format \"{}\" helper)\r\n".as_slice(),
            b"module Main\n    requires Console.Write\n\nlet helper = 3\n\nlet main () = Console.write (Text.format \"{}\" helper)\n".as_slice(),
        ] {
            db.set_disk_snapshot("src/Main.ling", edit.to_vec())
                .unwrap();
            assert_clean_equivalent(&mut db, main);
        }
    }

    #[test]
    fn clean_and_incremental_diagnostics_match_for_invalid_effects() {
        let mut db = CompilerDb::new();
        let main = file(
            db.set_disk_snapshot(
                "Main.ling",
                b"module Main\n\nlet main () = Console.write \"x\"\n".to_vec(),
            )
            .unwrap(),
        );
        let mut clean = clean_database(&db);
        let clean_main = clean_file(&db, &clean, main);
        let incremental_error = db.type_effect(main).unwrap_err();
        let clean_error = clean.type_effect(clean_main).unwrap_err();
        assert_eq!(incremental_error, clean_error);
        assert!(matches!(
            incremental_error,
            QueryError::EffectChecking { .. }
        ));
    }
}
