//! Deterministic, in-memory compiler queries for Ling source snapshots.
//!
//! This crate is an internal implementation boundary. It deliberately owns no
//! host filesystem access, persistence, wire schema, CLI command, or language
//! semantics. Query results are immutable `Arc` values keyed by the exact VFS
//! snapshot and the selected workspace revisions.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use ling_ast::{LowerError, Program, lower};
use ling_hir::{self, LowerError as HirLowerError};
use ling_resolve::{ResolveError, ResolvedModule, resolve};
use ling_source::{
    ChangeEvent, FileOrigin, FileSnapshot, InputChange, LexicalOffset, Revision, SourceError,
    SourceId, VfsError, VirtualFileSystem, WorkspaceInput,
};
use ling_syntax::{LexedSource, ParsedSource, lex, parse};

const LANGUAGE_VERSION: (u16, u16, u16) = (0, 1, 0);
const UNICODE_VERSION: (u8, u8, u8) = (17, 0, 0);
const QUERY_SCHEMA_VERSION: u16 = 1;

/// The query families implemented by the internal compiler database.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum QueryKind {
    SourceBytes,
    LineIndex,
    Tokens,
    Parse,
    Ast,
    Hir,
    ModuleGraph,
    Resolve,
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

/// Internal, deterministic compiler query database.
#[derive(Debug, Default)]
pub struct CompilerDb {
    vfs: VirtualFileSystem,
    source_bytes: BTreeMap<QueryKey, Arc<FileSnapshot>>,
    sources: BTreeMap<QueryKey, Result<Arc<ling_source::SourceFile>, SourceError>>,
    line_indexes: BTreeMap<QueryKey, Arc<LineIndex>>,
    tokens: BTreeMap<QueryKey, Arc<LexedSource>>,
    parses: BTreeMap<QueryKey, Arc<ParsedSource>>,
    asts: BTreeMap<QueryKey, Result<Arc<Program>, LowerError>>,
    hirs: BTreeMap<QueryKey, Result<Arc<ling_hir::Program>, HirLowerError>>,
    module_graphs: BTreeMap<ModuleGraphKey, Arc<ModuleGraph>>,
    resolved_modules: BTreeMap<ModuleResolveKey, Result<Arc<ResolvedModule>, Box<[ResolveError]>>>,
    trace: Vec<QueryEvent>,
}

impl CompilerDb {
    /// Creates an empty database with no ambient filesystem or environment.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            vfs: VirtualFileSystem::new(),
            source_bytes: BTreeMap::new(),
            sources: BTreeMap::new(),
            line_indexes: BTreeMap::new(),
            tokens: BTreeMap::new(),
            parses: BTreeMap::new(),
            asts: BTreeMap::new(),
            hirs: BTreeMap::new(),
            module_graphs: BTreeMap::new(),
            resolved_modules: BTreeMap::new(),
            trace: Vec::new(),
        }
    }

    #[must_use]
    pub const fn vfs(&self) -> &VirtualFileSystem {
        &self.vfs
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
        let index = Arc::new(LineIndex::from_source(&source));
        self.line_indexes.insert(key, index.clone());
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

        let files = graph
            .nodes()
            .iter()
            .map(ModuleNode::file)
            .collect::<Vec<_>>();
        let mut programs = Vec::with_capacity(files.len());
        for module_file in files {
            programs.push((*self.hir(module_file)?).clone());
        }
        let result = match resolve(programs, &node.name) {
            Ok(program) => program
                .modules()
                .iter()
                .find(|module| module.hir.module.name.normalized() == node.name)
                .cloned()
                .map(Arc::new)
                .ok_or(QueryError::ResolvedModuleMissing { file }),
            Err(errors) => {
                let errors = errors.into_boxed_slice();
                self.resolved_modules.insert(key, Err(errors.clone()));
                return Err(QueryError::Resolution { errors });
            }
        };
        if let Ok(module) = &result {
            self.resolved_modules.insert(key, Ok(module.clone()));
        }
        result
    }

    /// Parses all visible files in canonical logical-name order.
    pub fn parse_all(&mut self) -> Result<Vec<(SourceId, Arc<ParsedSource>)>, QueryError> {
        let files = self
            .vfs
            .snapshots()
            .into_iter()
            .map(|snapshot| snapshot.id())
            .collect::<Vec<_>>();
        files
            .into_iter()
            .map(|file| self.parse(file).map(|parsed| (file, parsed)))
            .collect()
    }

    /// Returns test-only query evidence accumulated since the last clear.
    #[must_use]
    pub fn trace(&self) -> &[QueryEvent] {
        &self.trace
    }

    pub fn clear_trace(&mut self) {
        self.trace.clear();
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

    fn record(&mut self, kind: QueryKind, snapshot: &FileSnapshot, outcome: QueryOutcome) {
        self.trace.push(QueryEvent {
            kind,
            file: snapshot.id(),
            revision: snapshot.revision(),
            outcome,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
