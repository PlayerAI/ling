//! Bounded Preview lifecycle, document overlays, and stdio transport for
//! `ling lsp --stdio`.
//!
//! RFC-0004 is the authority for lifecycle and transport; RFC-0023 is the
//! authority for the full-text document overlay boundary; RFC-0029 extends it
//! with bounded incremental changes; RFC-0030 governs atomic workspace reload;
//! RFC-0026 governs the bounded document-formatting response; DEC-0029 remains
//! the authority for position projection; RFC-0036 governs Document Symbols;
//! RFC-0037 governs checked Hover; RFC-0038 governs resolver navigation;
//! RFC-0039 governs checked References; RFC-0040 governs Prepare Rename;
//! RFC-0041 governs checked transactional Rename.

use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub use ling_source::{FileOrigin, PositionEncoding, Revision, WorkspaceInput};
use ling_source::{
    LspPosition, LspPositionEdit, SourceFile, SourceId, VfsError, VirtualFileSystem,
    WorkspaceSnapshot, negotiate_position_encoding, validate_logical_name,
};
use serde_json::{Map, Value, json};

// DEC-0032 is an internal child boundary; transport/server wiring remains
// deferred until the parent LSP scheduling contract is Accepted.
#[allow(dead_code)]
mod scheduler;
// DEC-0033 is an internal arithmetic child; public quota and diagnostic
// behavior remains deferred to the parent LSP-2504 contract.
#[allow(dead_code)]
mod resource;
// RFC-0031 owns the public compiler-diagnostic adapter. Publication remains a
// separate LSP-2202 concern.
#[allow(dead_code)]
mod diagnostics;
pub use diagnostics::{
    AdaptedDiagnostic, DIAGNOSTIC_PROTOCOL_VERSION, DiagnosticAdapterError, DiagnosticAdapterInput,
    DiagnosticProjectionError, DiagnosticSource, RelatedDiagnosticLabel, adapt_diagnostics,
};
mod diagnostic_control;
pub use diagnostic_control::{
    DEFAULT_MAX_DIAGNOSTICS_PER_DOCUMENT, DEFAULT_MAX_DIAGNOSTICS_PER_WORKSPACE,
    DIAGNOSTIC_CONTROL_PROTOCOL_VERSION, DiagnosticControlError, MAX_DIAGNOSTICS_PER_DOCUMENT,
    MAX_DIAGNOSTICS_PER_WORKSPACE,
};
use diagnostic_control::{DiagnosticLimits, parse_diagnostic_limits};
mod publication;
pub use publication::{
    DiagnosticAnalysisError, DiagnosticAnalysisResult, DiagnosticAnalysisTicket,
    PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION,
};
mod pull_diagnostics;
pub use pull_diagnostics::{MAX_PULL_PREVIOUS_RESULTS, PULL_DIAGNOSTICS_PROTOCOL_VERSION};
mod document_symbols;
pub use document_symbols::{DOCUMENT_SYMBOL_PROTOCOL_VERSION, MAX_DOCUMENT_SYMBOLS};
mod hover;
pub use hover::{HOVER_PROTOCOL_VERSION, MAX_HOVER_CONTENT_BYTES, MAX_HOVER_ENTRIES};
mod location_projection;
mod navigation;
pub use navigation::{MAX_NAVIGATION_TARGETS, NAVIGATION_PROTOCOL_VERSION};
mod references;
pub use references::{MAX_REFERENCE_LOCATIONS, REFERENCES_PROTOCOL_VERSION};
mod prepare_rename;
pub use prepare_rename::PREPARE_RENAME_PROTOCOL_VERSION;
mod rename;
pub use rename::RENAME_PROTOCOL_VERSION;
// DEC-0035 remains the internal immutable collection child. RFC-0032 owns the
// separate public push-publication lifecycle without broadening this module.
#[allow(dead_code)]
mod diagnostic_batch;

/// Version marker for the current Preview lifecycle protocol.
pub const PROTOCOL_VERSION: &str = "ling.lsp.lifecycle/0.1";
/// Version marker for the incremental document overlay Experimental extension.
pub const OVERLAY_PROTOCOL_VERSION: &str = "ling.lsp.overlay/0.2";
/// Version marker for the bounded document-formatting Experimental extension.
pub const FORMATTING_PROTOCOL_VERSION: &str = "ling.lsp.formatting/0.1";
/// Version marker for the atomic workspace-reload Experimental extension.
pub const WORKSPACE_PROTOCOL_VERSION: &str = "ling.lsp.workspace/0.1";
/// JSON-RPC protocol version accepted by this server.
pub const JSON_RPC_VERSION: &str = "2.0";
/// Maximum JSON body size accepted by the transport.
pub const MAX_FRAME_BYTES: usize = 1_048_576;
/// Maximum complete header-block size accepted by the transport.
pub const MAX_HEADER_BYTES: usize = 8_192;
/// Maximum number of workspace folders accepted during initialization.
pub const MAX_WORKSPACE_FOLDERS: usize = 32;
/// Maximum UTF-8 byte length of one workspace URI.
pub const MAX_WORKSPACE_URI_BYTES: usize = 4_096;
/// Maximum UTF-8 byte length of one workspace display name.
pub const MAX_WORKSPACE_NAME_BYTES: usize = 256;
/// Maximum UTF-8 byte length of one open-document text value.
pub const MAX_DOCUMENT_BYTES: usize = MAX_FRAME_BYTES;
/// Maximum number of ordered entries accepted in one incremental change batch.
pub const MAX_CONTENT_CHANGES: usize = 64;
/// Maximum number of source deltas in one workspace reload.
pub const MAX_RELOAD_SOURCES: usize = 1_024;
/// Maximum combined UTF-8 bytes across workspace reload text fields.
pub const MAX_RELOAD_TEXT_BYTES: usize = MAX_FRAME_BYTES;

const PARSE_ERROR: i32 = -32_700;
const INVALID_REQUEST: i32 = -32_600;
const METHOD_NOT_FOUND: i32 = -32_601;
const INVALID_PARAMS: i32 = -32_602;
const INTERNAL_ERROR: i32 = -32_603;
const SERVER_NOT_INITIALIZED: i32 = -32_002;
const SERVER_SHUTTING_DOWN: i32 = -32_003;
const DOCUMENT_STALE: i32 = -32_004;
const DOCUMENT_READ_ONLY: i32 = -32_005;
const DOCUMENT_URI: i32 = -32_006;
const WORKSPACE_STALE: i32 = -32_007;

/// Lifecycle state of one LSP server process.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleState {
    /// No successful initialize request has been processed.
    Uninitialized,
    /// Initialize succeeded and the initialized notification is pending.
    AwaitingInitialized,
    /// The server can process the Preview method set.
    Ready,
    /// Shutdown succeeded; only the exit notification is meaningful.
    ShutdownRequested,
    /// The server has processed exit and will not accept more messages.
    Exited,
}

/// Opaque, validated workspace-folder metadata retained for the current process.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkspaceFolder {
    uri: String,
    name: String,
}

impl WorkspaceFolder {
    /// Returns the exact client-provided URI bytes as UTF-8 text.
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the exact client-provided display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A public, path-free view of one document tracked by the Preview overlay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentView {
    uri: String,
    version: i64,
    open: bool,
    writable: bool,
    text: String,
}

/// An owned, immutable view of one visible document captured for in-process
/// analysis. It intentionally exposes no session-local `SourceId`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestDocument {
    uri: String,
    logical_name: String,
    revision: Revision,
    origin: FileOrigin,
    open: bool,
    writable: bool,
    temporary: bool,
    client_version: Option<i64>,
    bytes: Box<[u8]>,
}

impl RequestDocument {
    /// Returns the exact path-free URI supplied by the editor or host.
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the canonical logical source name used by the VFS.
    #[must_use]
    pub fn logical_name(&self) -> &str {
        &self.logical_name
    }

    /// Returns the session-local VFS revision visible at capture time.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns whether the visible bytes came from an editor overlay or disk.
    #[must_use]
    pub const fn origin(&self) -> FileOrigin {
        self.origin
    }

    /// Returns whether the document was open at capture time.
    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.open
    }

    /// Returns whether editor-originated changes may target this document.
    #[must_use]
    pub const fn is_writable(&self) -> bool {
        self.writable
    }

    /// Returns whether this is an untitled editor-only document.
    #[must_use]
    pub const fn is_temporary(&self) -> bool {
        self.temporary
    }

    /// Returns the client version for an open document, or `None` for a
    /// disk-only/closed document.
    #[must_use]
    pub const fn client_version(&self) -> Option<i64> {
        self.client_version
    }

    /// Returns the exact immutable UTF-8 bytes captured from the visible VFS
    /// layer.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Errors raised when an internal request snapshot cannot be captured
/// atomically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestSnapshotError {
    /// A tracked document no longer has its VFS entry.
    MissingDocument { uri: String },
}

impl fmt::Display for RequestSnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDocument { uri } => {
                write!(formatter, "tracked document is missing from the VFS: {uri}")
            }
        }
    }
}

impl std::error::Error for RequestSnapshotError {}

/// An immutable, path-free capture of the visible LSP document set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestSnapshot {
    state: LifecycleState,
    position_encoding: PositionEncoding,
    revision: Revision,
    documents: Box<[RequestDocument]>,
    inputs: Box<[WorkspaceSnapshot]>,
}

impl RequestSnapshot {
    /// Returns the lifecycle state at capture time.
    #[must_use]
    pub const fn state(&self) -> LifecycleState {
        self.state
    }

    /// Returns the negotiated position encoding at capture time.
    #[must_use]
    pub const fn position_encoding(&self) -> PositionEncoding {
        self.position_encoding
    }

    /// Returns the latest session-local VFS revision observed at capture.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns documents in deterministic URI order.
    #[must_use]
    pub fn documents(&self) -> &[RequestDocument] {
        &self.documents
    }

    /// Returns one captured document by its path-free URI.
    #[must_use]
    pub fn document(&self, uri: &str) -> Option<&RequestDocument> {
        self.documents
            .binary_search_by(|document| document.uri().cmp(uri))
            .ok()
            .map(|index| &self.documents[index])
    }

    /// Returns project inputs in the declared canonical input order.
    #[must_use]
    pub fn inputs(&self) -> &[WorkspaceSnapshot] {
        &self.inputs
    }

    /// Returns one captured project input.
    #[must_use]
    pub fn input(&self, kind: WorkspaceInput) -> Option<&WorkspaceSnapshot> {
        self.inputs.iter().find(|input| input.kind() == kind)
    }
}

/// A clone-shared, monotonic cancellation signal for in-process analysis.
///
/// This token carries no JSON-RPC request ID, document version, deadline, or
/// result state. It is deliberately separate from VM host cancellation.
#[derive(Clone, Debug)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationToken {
    /// Creates an active token.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Requests cancellation. Repeated calls are idempotent.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Checks the cooperative cancellation checkpoint.
    pub fn check(&self) -> Result<(), CancellationError> {
        if self.is_cancelled() {
            Err(CancellationError::Cancelled)
        } else {
            Ok(())
        }
    }
}

/// The typed result of an internal LSP cancellation checkpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationError {
    /// The shared token has been cancelled.
    Cancelled,
}

impl fmt::Display for CancellationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => formatter.write_str("LSP analysis was cancelled"),
        }
    }
}

impl std::error::Error for CancellationError {}

impl DocumentView {
    /// Returns the exact URI supplied by the editor.
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the last accepted editor version.
    #[must_use]
    pub const fn version(&self) -> i64 {
        self.version
    }

    /// Returns whether the document currently has an open overlay.
    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.open
    }

    /// Returns whether the URI accepts editor changes.
    #[must_use]
    pub const fn is_writable(&self) -> bool {
        self.writable
    }

    /// Returns the exact visible UTF-8 document text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DocumentRecord {
    file: SourceId,
    logical_name: String,
    version: i64,
    open: bool,
    writable: bool,
    temporary: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DocumentIdentity {
    logical_name: String,
    writable: bool,
    temporary: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DocumentChange {
    Full(String),
    Range(LspPositionEdit),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceReload {
    uri: String,
    text: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InputReload {
    kind: WorkspaceInput,
    text: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkspaceReload {
    base_revision: u64,
    sources: Vec<SourceReload>,
    inputs: Vec<InputReload>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkspaceReloadResult {
    changed: bool,
    revision: Revision,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorkspaceReloadError {
    InvalidParams,
    Stale { current: Revision, requested: u64 },
    Vfs(VfsError),
}

/// Errors raised while publishing or applying an RFC-0023/RFC-0029 document overlay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OverlayError {
    InvalidParams,
    InvalidRange,
    InvalidUri,
    UnknownDocument,
    AlreadyOpen,
    NotOpen,
    StaleVersion { current: i64, requested: i64 },
    ReadOnly,
    TextTooLarge,
    Vfs(VfsError),
}

/// Result of handling one decoded JSON-RPC body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HandleOutcome {
    /// A response body that must be sent with a transport frame.
    Response(Vec<u8>),
    /// A notification or an ignored malformed notification with no response.
    NoResponse,
    /// The exit notification ended the server stream.
    Exit { code: u8 },
}

/// Result of the generic stdio loop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunResult {
    exit_code: u8,
    state: LifecycleState,
}

impl RunResult {
    /// Returns the process status prescribed by the lifecycle contract.
    #[must_use]
    pub const fn exit_code(self) -> u8 {
        self.exit_code
    }

    /// Returns the final server state observed by the loop.
    #[must_use]
    pub const fn state(self) -> LifecycleState {
        self.state
    }
}

/// Errors raised by the byte-oriented framing layer.
#[derive(Debug)]
pub enum TransportError {
    /// The underlying reader or writer failed.
    Io(io::Error),
    /// The header block was not a CRLF-terminated ASCII header block.
    InvalidHeader,
    /// More than one Content-Length header was supplied.
    DuplicateContentLength,
    /// Content-Length was missing.
    MissingContentLength,
    /// Content-Length contained something other than decimal digits.
    InvalidContentLength,
    /// The header block exceeded MAX_HEADER_BYTES.
    HeadersTooLarge,
    /// The message body exceeded MAX_FRAME_BYTES.
    FrameTooLarge { length: usize },
    /// The input ended in the middle of a header or body.
    UnexpectedEof,
    /// Synchronous diagnostic analysis failed before publication.
    DiagnosticAnalysis(DiagnosticAnalysisError),
}

impl fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "LSP transport I/O failed: {error}"),
            Self::InvalidHeader => formatter.write_str("invalid LSP header block"),
            Self::DuplicateContentLength => formatter.write_str("duplicate Content-Length header"),
            Self::MissingContentLength => formatter.write_str("missing Content-Length header"),
            Self::InvalidContentLength => formatter.write_str("invalid Content-Length value"),
            Self::HeadersTooLarge => formatter.write_str("LSP header block exceeds 8192 bytes"),
            Self::FrameTooLarge { length } => write!(
                formatter,
                "LSP body length {length} exceeds the {}-byte limit",
                MAX_FRAME_BYTES
            ),
            Self::UnexpectedEof => formatter.write_str("truncated LSP frame"),
            Self::DiagnosticAnalysis(error) => {
                write!(formatter, "LSP diagnostic analysis failed: {error}")
            }
        }
    }
}

impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::DiagnosticAnalysis(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for TransportError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<DiagnosticAnalysisError> for TransportError {
    fn from(error: DiagnosticAnalysisError) -> Self {
        Self::DiagnosticAnalysis(error)
    }
}

/// A single-process, deterministic Preview lifecycle server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LspServer {
    state: LifecycleState,
    position_encoding: PositionEncoding,
    workspace_folders: Vec<WorkspaceFolder>,
    vfs: VirtualFileSystem,
    documents: BTreeMap<String, DocumentRecord>,
    last_versions: BTreeMap<String, i64>,
    diagnostics_pending: bool,
    published_diagnostics: BTreeMap<String, Value>,
    outbound_notifications: Vec<Vec<u8>>,
    pull_diagnostics_supported: bool,
    diagnostic_limits: DiagnosticLimits,
    hierarchical_document_symbols: bool,
    hover_markup: hover::HoverMarkup,
    transactional_rename_supported: bool,
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LspServer {
    /// Creates an uninitialized server with the DEC-0029 UTF-16 fallback.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: LifecycleState::Uninitialized,
            position_encoding: PositionEncoding::Utf16,
            workspace_folders: Vec::new(),
            vfs: VirtualFileSystem::new(),
            documents: BTreeMap::new(),
            last_versions: BTreeMap::new(),
            diagnostics_pending: false,
            published_diagnostics: BTreeMap::new(),
            outbound_notifications: Vec::new(),
            pull_diagnostics_supported: false,
            diagnostic_limits: DiagnosticLimits::DEFAULT,
            hierarchical_document_symbols: false,
            hover_markup: hover::HoverMarkup::Plaintext,
            transactional_rename_supported: false,
        }
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn state(&self) -> LifecycleState {
        self.state
    }

    /// Returns the negotiated position encoding.
    #[must_use]
    pub const fn position_encoding(&self) -> PositionEncoding {
        self.position_encoding
    }

    /// Returns the validated workspace folders in client order.
    #[must_use]
    pub fn workspace_folders(&self) -> &[WorkspaceFolder] {
        &self.workspace_folders
    }

    /// Returns tracked documents in deterministic URI order.
    #[must_use]
    pub fn documents(&self) -> Vec<DocumentView> {
        self.documents
            .keys()
            .filter_map(|uri| self.document(uri))
            .collect()
    }

    /// Returns one tracked document without exposing the internal `SourceId`.
    #[must_use]
    pub fn document(&self, uri: &str) -> Option<DocumentView> {
        let record = self.documents.get(uri)?;
        let snapshot = self.vfs.snapshot(record.file)?;
        let text = std::str::from_utf8(snapshot.bytes()).ok()?.to_owned();
        Some(DocumentView {
            uri: uri.to_owned(),
            version: record.version,
            open: record.open,
            writable: record.writable,
            text,
        })
    }

    /// Captures the complete visible document set as an owned immutable value.
    ///
    /// This is an internal analysis boundary, not a JSON-RPC method. The
    /// server can continue publishing later VFS changes after this method
    /// returns without mutating the captured bytes.
    pub fn capture_request_snapshot(&self) -> Result<RequestSnapshot, RequestSnapshotError> {
        let workspace = self.vfs.workspace_snapshot();
        let mut documents = Vec::with_capacity(self.documents.len());
        for (uri, record) in &self.documents {
            let snapshot = self
                .vfs
                .snapshot(record.file)
                .ok_or_else(|| RequestSnapshotError::MissingDocument { uri: uri.clone() })?;
            documents.push(RequestDocument {
                uri: uri.clone(),
                logical_name: snapshot.logical_name().to_owned(),
                revision: snapshot.revision(),
                origin: snapshot.origin(),
                open: record.open,
                writable: record.writable,
                temporary: record.temporary,
                client_version: record.open.then_some(record.version),
                bytes: snapshot.bytes().into(),
            });
        }
        Ok(RequestSnapshot {
            state: self.state,
            position_encoding: self.position_encoding,
            revision: workspace.revision(),
            documents: documents.into_boxed_slice(),
            inputs: workspace.inputs().to_vec().into_boxed_slice(),
        })
    }

    /// Publishes a host-provided disk snapshot without reading the filesystem.
    ///
    /// An open overlay continues to hide this snapshot until `didClose`.
    pub fn publish_disk_snapshot(&mut self, uri: &str, text: &str) -> Result<(), OverlayError> {
        let previous_revision = self.vfs.revision();
        let identity = document_identity(uri)?;
        if identity.temporary || text.len() > MAX_DOCUMENT_BYTES {
            return Err(if identity.temporary {
                OverlayError::InvalidUri
            } else {
                OverlayError::TextTooLarge
            });
        }
        let file = self.ensure_file(uri, &identity, text.as_bytes())?;
        self.vfs
            .set_disk_snapshot(identity.logical_name.clone(), text.as_bytes().to_vec())
            .map_err(OverlayError::Vfs)?;
        if !self.documents.contains_key(uri) {
            self.documents.insert(
                uri.to_owned(),
                DocumentRecord {
                    file,
                    logical_name: identity.logical_name,
                    version: 0,
                    open: false,
                    writable: identity.writable,
                    temporary: identity.temporary,
                },
            );
        }
        if self.vfs.revision() != previous_revision {
            self.mark_diagnostics_pending();
        }
        Ok(())
    }

    /// Handles one JSON-RPC body and returns a response body or state event.
    ///
    /// Parsing and protocol failures are converted into deterministic JSON-RPC
    /// responses. Framing remains the responsibility of [`run_stdio`].
    #[must_use]
    pub fn handle_json(&mut self, body: &[u8]) -> HandleOutcome {
        let value = match serde_json::from_slice::<Value>(body) {
            Ok(value) => value,
            Err(_) => {
                return HandleOutcome::Response(error_response(
                    None,
                    PARSE_ERROR,
                    "JSON 解析失败 / JSON parse error",
                ));
            }
        };
        let Some(object) = value.as_object() else {
            return HandleOutcome::Response(error_response(
                None,
                INVALID_REQUEST,
                "请求必须是 JSON 对象 / request must be a JSON object",
            ));
        };

        let id_present = object.contains_key("id");
        let id = object.get("id").cloned().unwrap_or(Value::Null);
        if id_present && !valid_request_id(&id) {
            return response_or_none(
                id_present,
                id,
                INVALID_REQUEST,
                "请求 ID 无效 / invalid request id",
            );
        }
        if object.get("jsonrpc") != Some(&Value::String(JSON_RPC_VERSION.to_owned())) {
            return response_or_none(
                id_present,
                id,
                INVALID_REQUEST,
                "JSON-RPC 版本无效 / invalid JSON-RPC version",
            );
        }
        let Some(method) = object.get("method").and_then(Value::as_str) else {
            return response_or_none(
                id_present,
                id,
                INVALID_REQUEST,
                "缺少方法名 / missing method",
            );
        };
        let params = object.get("params").cloned().unwrap_or_else(empty_object);

        match method {
            "initialize" => self.initialize(id_present, id, params),
            "initialized" => self.initialized(id_present, id),
            "shutdown" => self.shutdown(id_present, id, params),
            "exit" => self.exit(id_present, id),
            "textDocument/didOpen" => self.did_open(id_present, id, params),
            "textDocument/didChange" => self.did_change(id_present, id, params),
            "textDocument/didClose" => self.did_close(id_present, id, params),
            "textDocument/formatting" => self.document_formatting(id_present, id, params),
            "textDocument/diagnostic" => self.document_diagnostic(id_present, id, params),
            "textDocument/documentSymbol" => self.document_symbols(id_present, id, params),
            "textDocument/hover" => self.hover(id_present, id, params),
            "textDocument/definition" => self.navigation(
                id_present,
                id,
                params,
                navigation::NavigationMethod::Definition,
            ),
            "textDocument/declaration" => self.navigation(
                id_present,
                id,
                params,
                navigation::NavigationMethod::Declaration,
            ),
            "textDocument/typeDefinition" => self.navigation(
                id_present,
                id,
                params,
                navigation::NavigationMethod::TypeDefinition,
            ),
            "textDocument/references" => self.references(id_present, id, params),
            "textDocument/prepareRename" => self.prepare_rename(id_present, id, params),
            "textDocument/rename" => self.rename(id_present, id, params),
            "workspace/diagnostic" => self.workspace_diagnostic(id_present, id, params),
            "ling/workspace/reload" => self.workspace_reload_request(id_present, id, params),
            _ => self.unknown_method(id_present, id, method),
        }
    }

    fn initialize(&mut self, is_request: bool, id: Value, params: Value) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Uninitialized {
            return error_or_none(
                true,
                id,
                INVALID_REQUEST,
                "初始化状态无效 / initialize is invalid in the current state",
            );
        }

        let ParsedInitializeParams {
            encoding,
            folders,
            pull_diagnostics_supported,
            diagnostic_limits,
            hierarchical_document_symbols,
            hover_markup,
            transactional_rename_supported,
        } = match parse_initialize_params(&params) {
            Ok(value) => value,
            Err(()) => {
                return error_or_none(
                    true,
                    id,
                    INVALID_PARAMS,
                    "初始化参数无效 / invalid initialize parameters",
                );
            }
        };
        self.position_encoding = encoding;
        self.workspace_folders = folders;
        self.pull_diagnostics_supported = pull_diagnostics_supported;
        self.diagnostic_limits = diagnostic_limits;
        self.hierarchical_document_symbols = hierarchical_document_symbols;
        self.hover_markup = hover_markup;
        self.transactional_rename_supported = transactional_rename_supported;
        self.state = LifecycleState::AwaitingInitialized;
        HandleOutcome::Response(success_response(
            id,
            initialize_result(
                encoding,
                pull_diagnostics_supported,
                diagnostic_limits,
                hierarchical_document_symbols,
                hover_markup,
            ),
        ))
    }

    fn initialized(&mut self, is_request: bool, id: Value) -> HandleOutcome {
        if is_request {
            return error_or_none(
                true,
                id,
                INVALID_REQUEST,
                "initialized 必须是通知 / initialized must be a notification",
            );
        }
        if self.state == LifecycleState::AwaitingInitialized {
            self.state = LifecycleState::Ready;
        }
        HandleOutcome::NoResponse
    }

    fn shutdown(&mut self, is_request: bool, id: Value, params: Value) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        if !params.is_null() && !params.as_object().is_some_and(Map::is_empty) {
            return error_or_none(
                true,
                id,
                INVALID_PARAMS,
                "shutdown 参数必须为空 / shutdown parameters must be empty",
            );
        }
        self.state = LifecycleState::ShutdownRequested;
        HandleOutcome::Response(success_response(id, Value::Null))
    }

    fn exit(&mut self, is_request: bool, id: Value) -> HandleOutcome {
        if is_request {
            return error_or_none(
                true,
                id,
                INVALID_REQUEST,
                "exit 必须是通知 / exit must be a notification",
            );
        }
        let code = u8::from(self.state != LifecycleState::ShutdownRequested);
        self.state = LifecycleState::Exited;
        HandleOutcome::Exit { code }
    }

    fn did_open(&mut self, is_request: bool, id: Value, params: Value) -> HandleOutcome {
        if self.state != LifecycleState::Ready {
            return self.state_error_for(is_request, id);
        }
        let result = parse_open_params(&params)
            .and_then(|(uri, version, text)| self.open_document(uri, version, text));
        if result.is_ok() {
            self.mark_diagnostics_pending();
        }
        self.overlay_outcome(is_request, id, result)
    }

    fn did_change(&mut self, is_request: bool, id: Value, params: Value) -> HandleOutcome {
        if self.state != LifecycleState::Ready {
            return self.state_error_for(is_request, id);
        }
        let result = parse_change_params(&params)
            .and_then(|(uri, version, changes)| self.change_document(&uri, version, changes));
        if result.is_ok() {
            self.mark_diagnostics_pending();
        }
        self.overlay_outcome(is_request, id, result)
    }

    fn did_close(&mut self, is_request: bool, id: Value, params: Value) -> HandleOutcome {
        if self.state != LifecycleState::Ready {
            return self.state_error_for(is_request, id);
        }
        let result = parse_close_params(&params).and_then(|uri| self.close_document(&uri));
        if result.is_ok() {
            self.mark_diagnostics_pending();
        }
        self.overlay_outcome(is_request, id, result)
    }

    fn workspace_reload_request(
        &mut self,
        is_request: bool,
        id: Value,
        params: Value,
    ) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        let result =
            parse_workspace_reload(&params).and_then(|reload| self.reload_workspace(reload));
        match result {
            Ok(result) => {
                if result.changed {
                    self.mark_diagnostics_pending();
                }
                HandleOutcome::Response(success_response(
                    id,
                    json!({
                        "changed": result.changed,
                        "revision": result.revision.get().to_string(),
                    }),
                ))
            }
            Err(WorkspaceReloadError::Stale { .. }) => error_or_none(
                true,
                id,
                WORKSPACE_STALE,
                "工作区基准版本过旧 / workspace base revision is stale",
            ),
            Err(WorkspaceReloadError::InvalidParams | WorkspaceReloadError::Vfs(_)) => {
                error_or_none(
                    true,
                    id,
                    INVALID_PARAMS,
                    "工作区重载参数无效 / invalid workspace reload parameters",
                )
            }
        }
    }

    fn document_formatting(&self, is_request: bool, id: Value, params: Value) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        let uri = match parse_formatting_params(&params) {
            Ok(uri) => uri,
            Err(error) => {
                let (code, message) = overlay_error_details(&error);
                return error_or_none(true, id, code, message);
            }
        };
        let Some(record) = self.documents.get(&uri) else {
            return error_or_none(
                true,
                id,
                INVALID_PARAMS,
                "文档状态无效 / invalid document state",
            );
        };
        if !record.open {
            return error_or_none(
                true,
                id,
                INVALID_PARAMS,
                "文档状态无效 / invalid document state",
            );
        }
        if !record.writable {
            return error_or_none(
                true,
                id,
                DOCUMENT_READ_ONLY,
                "依赖文档只读 / dependency document is read-only",
            );
        }
        let Some(snapshot) = self.vfs.snapshot(record.file) else {
            return formatting_internal_error(id);
        };
        let source = match SourceFile::from_bytes(
            record.file,
            record.logical_name.clone(),
            snapshot.bytes().to_vec(),
        ) {
            Ok(source) => source,
            Err(_) => return formatting_internal_error(id),
        };
        let parsed = ling_syntax::parse(&source);
        let document = match ling_format::build_format_ir(&source, &parsed) {
            Ok(document) => document,
            Err(_) => return formatting_internal_error(id),
        };
        let edit = match ling_format::format_core_edit(&document) {
            Ok(edit) => edit,
            Err(_) => return formatting_internal_error(id),
        };
        let Some(edit) = edit else {
            return HandleOutcome::Response(success_response(id, json!([])));
        };
        let end =
            match source.lsp_position(source.source_map().original_len(), self.position_encoding) {
                Ok(position) => position,
                Err(_) => return formatting_internal_error(id),
            };
        let replacement = if source.had_bom() {
            edit.replacement().strip_prefix('\u{feff}')
        } else {
            Some(edit.replacement())
        };
        let Some(replacement) = replacement else {
            return formatting_internal_error(id);
        };
        HandleOutcome::Response(success_response(
            id,
            json!([{
                "newText": replacement,
                "range": {
                    "end": {"character": end.character(), "line": end.line()},
                    "start": {"character": 0, "line": 0},
                },
            }]),
        ))
    }

    fn open_document(
        &mut self,
        uri: String,
        version: i64,
        text: String,
    ) -> Result<(), OverlayError> {
        let identity = document_identity(&uri)?;
        if let Some(last) = self.last_versions.get(&uri).copied() {
            if version <= last {
                return Err(OverlayError::StaleVersion {
                    current: last,
                    requested: version,
                });
            }
        }
        if self
            .documents
            .get(&uri)
            .is_some_and(|document| document.open)
        {
            return Err(OverlayError::AlreadyOpen);
        }
        let file = self.ensure_file(&uri, &identity, text.as_bytes())?;
        self.vfs
            .open_overlay(file, text.into_bytes())
            .map_err(OverlayError::Vfs)?;
        self.documents.insert(
            uri.clone(),
            DocumentRecord {
                file,
                logical_name: identity.logical_name,
                version,
                open: true,
                writable: identity.writable,
                temporary: identity.temporary,
            },
        );
        self.last_versions.insert(uri, version);
        Ok(())
    }

    fn change_document(
        &mut self,
        uri: &str,
        version: i64,
        changes: Vec<DocumentChange>,
    ) -> Result<(), OverlayError> {
        document_identity(uri)?;
        let record = self
            .documents
            .get(uri)
            .cloned()
            .ok_or(OverlayError::UnknownDocument)?;
        if !record.open {
            return Err(OverlayError::NotOpen);
        }
        if !record.writable {
            return Err(OverlayError::ReadOnly);
        }
        if version <= record.version {
            return Err(OverlayError::StaleVersion {
                current: record.version,
                requested: version,
            });
        }
        let snapshot = self
            .vfs
            .snapshot(record.file)
            .ok_or(OverlayError::UnknownDocument)?;
        let mut bytes = snapshot.bytes().to_vec();
        for change in changes {
            bytes = match change {
                // RFC-0023 accepts the exact UTF-8 JSON string as an overlay,
                // including text that is not yet a valid Ling source file.
                DocumentChange::Full(text) => text.into_bytes(),
                DocumentChange::Range(edit) => {
                    SourceFile::from_bytes(record.file, record.logical_name.clone(), bytes)
                        .map_err(|_| OverlayError::InvalidRange)?
                        .apply_lsp_position_edit(self.position_encoding, &edit)
                        .map_err(|_| OverlayError::InvalidRange)?
                        .into_original_bytes()
                }
            };
            if bytes.len() > MAX_DOCUMENT_BYTES {
                return Err(OverlayError::TextTooLarge);
            }
        }
        self.vfs
            .open_overlay(record.file, bytes)
            .map_err(OverlayError::Vfs)?;
        let current = self
            .documents
            .get_mut(uri)
            .ok_or(OverlayError::UnknownDocument)?;
        current.version = version;
        self.last_versions.insert(uri.to_owned(), version);
        Ok(())
    }

    fn reload_workspace(
        &mut self,
        reload: WorkspaceReload,
    ) -> Result<WorkspaceReloadResult, WorkspaceReloadError> {
        let current = self.vfs.revision();
        if reload.base_revision != current.get() {
            return Err(WorkspaceReloadError::Stale {
                current,
                requested: reload.base_revision,
            });
        }

        let mut candidate = self.clone();
        for source in reload.sources {
            match source.text {
                Some(text) => candidate
                    .publish_disk_snapshot(&source.uri, &text)
                    .map_err(|error| match error {
                        OverlayError::Vfs(error) => WorkspaceReloadError::Vfs(error),
                        _ => WorkspaceReloadError::InvalidParams,
                    })?,
                None => {
                    let record = candidate
                        .documents
                        .get(&source.uri)
                        .ok_or(WorkspaceReloadError::InvalidParams)?;
                    if record.open || record.temporary {
                        return Err(WorkspaceReloadError::InvalidParams);
                    }
                    candidate
                        .vfs
                        .remove_disk_snapshot(record.file)
                        .map_err(WorkspaceReloadError::Vfs)?;
                    candidate.documents.remove(&source.uri);
                }
            }
        }
        for input in reload.inputs {
            match input.text {
                Some(text) => {
                    candidate
                        .vfs
                        .set_workspace_input(input.kind, text.into_bytes())
                        .map_err(WorkspaceReloadError::Vfs)?;
                }
                None => {
                    candidate
                        .vfs
                        .remove_workspace_input(input.kind)
                        .map_err(WorkspaceReloadError::Vfs)?;
                }
            }
        }

        let revision = candidate.vfs.revision();
        let result = WorkspaceReloadResult {
            changed: revision != current,
            revision,
        };
        *self = candidate;
        Ok(result)
    }

    fn close_document(&mut self, uri: &str) -> Result<(), OverlayError> {
        document_identity(uri)?;
        let record = self
            .documents
            .get(uri)
            .cloned()
            .ok_or(OverlayError::UnknownDocument)?;
        if !record.open {
            return Err(OverlayError::NotOpen);
        }
        self.vfs
            .close_overlay(record.file)
            .map_err(OverlayError::Vfs)?;
        if record.temporary {
            self.vfs
                .remove_file(record.file)
                .map_err(OverlayError::Vfs)?;
            self.documents.remove(uri);
        } else if let Some(current) = self.documents.get_mut(uri) {
            current.open = false;
        }
        Ok(())
    }

    fn ensure_file(
        &mut self,
        uri: &str,
        identity: &DocumentIdentity,
        bytes: &[u8],
    ) -> Result<SourceId, OverlayError> {
        if let Some(file) = self.vfs.file_id(&identity.logical_name) {
            if self
                .documents
                .iter()
                .any(|(other_uri, document)| other_uri != uri && document.file == file)
            {
                return Err(OverlayError::InvalidUri);
            }
            return Ok(file);
        }
        self.vfs
            .set_disk_snapshot(identity.logical_name.clone(), bytes.to_vec())
            .map_err(OverlayError::Vfs)
            .and_then(|_| {
                self.vfs
                    .file_id(&identity.logical_name)
                    .ok_or(OverlayError::Vfs(VfsError::FileIdExhausted))
            })
    }

    fn overlay_outcome(
        &self,
        is_request: bool,
        id: Value,
        result: Result<(), OverlayError>,
    ) -> HandleOutcome {
        match result {
            Ok(()) if is_request => HandleOutcome::Response(success_response(id, Value::Null)),
            Ok(()) => HandleOutcome::NoResponse,
            Err(error) if is_request => {
                let (code, message) = overlay_error_details(&error);
                error_or_none(true, id, code, message)
            }
            Err(_) => HandleOutcome::NoResponse,
        }
    }

    fn state_error_for(&self, is_request: bool, id: Value) -> HandleOutcome {
        if is_request {
            self.state_error(id)
        } else {
            HandleOutcome::NoResponse
        }
    }

    fn unknown_method(&self, is_request: bool, id: Value, method: &str) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state == LifecycleState::Uninitialized
            || self.state == LifecycleState::AwaitingInitialized
        {
            return error_or_none(
                true,
                id,
                SERVER_NOT_INITIALIZED,
                "服务器尚未初始化 / server not initialized",
            );
        }
        if self.state == LifecycleState::ShutdownRequested || self.state == LifecycleState::Exited {
            return error_or_none(
                true,
                id,
                SERVER_SHUTTING_DOWN,
                "服务器正在关闭 / server is shutting down",
            );
        }
        let _ = method;
        error_or_none(true, id, METHOD_NOT_FOUND, "未知方法 / method not found")
    }

    fn state_error(&self, id: Value) -> HandleOutcome {
        if self.state == LifecycleState::Uninitialized
            || self.state == LifecycleState::AwaitingInitialized
        {
            error_or_none(
                true,
                id,
                SERVER_NOT_INITIALIZED,
                "服务器尚未初始化 / server not initialized",
            )
        } else {
            error_or_none(
                true,
                id,
                SERVER_SHUTTING_DOWN,
                "服务器正在关闭 / server is shutting down",
            )
        }
    }
}

/// Runs the RFC-0004 framed stdio loop.
pub fn run_stdio<R: Read, W: Write>(input: R, output: W) -> Result<RunResult, TransportError> {
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let mut server = LspServer::new();

    loop {
        let Some(body) = read_frame(&mut reader)? else {
            writer.flush()?;
            return Ok(RunResult {
                exit_code: 0,
                state: server.state(),
            });
        };
        match server.handle_json(&body) {
            HandleOutcome::Response(response) => write_frame(&mut writer, &response)?,
            HandleOutcome::NoResponse => {}
            HandleOutcome::Exit { code } => {
                writer.flush()?;
                return Ok(RunResult {
                    exit_code: code,
                    state: server.state(),
                });
            }
        }
        if server.state() == LifecycleState::Ready {
            server.flush_pending_diagnostics()?;
            for notification in server.take_notifications() {
                write_frame(&mut writer, &notification)?;
            }
        }
    }
}

struct ParsedInitializeParams {
    encoding: PositionEncoding,
    folders: Vec<WorkspaceFolder>,
    pull_diagnostics_supported: bool,
    diagnostic_limits: DiagnosticLimits,
    hierarchical_document_symbols: bool,
    hover_markup: hover::HoverMarkup,
    transactional_rename_supported: bool,
}

fn parse_initialize_params(params: &Value) -> Result<ParsedInitializeParams, ()> {
    let Some(object) = params.as_object() else {
        return Err(());
    };

    let mut labels = Vec::new();
    let mut pull_diagnostics_supported = false;
    let mut hierarchical_document_symbols = false;
    let mut hover_markup = hover::HoverMarkup::Plaintext;
    let mut transactional_rename_supported = false;
    if let Some(capabilities) = object.get("capabilities") {
        let Some(capabilities) = capabilities.as_object() else {
            return Err(());
        };
        transactional_rename_supported = rename::parse_workspace_edit_capability(capabilities)?;
        if let Some(general) = capabilities.get("general") {
            let Some(general) = general.as_object() else {
                return Err(());
            };
            if let Some(position_encodings) = general.get("positionEncodings") {
                let Some(position_encodings) = position_encodings.as_array() else {
                    return Err(());
                };
                for label in position_encodings {
                    let Some(label) = label.as_str() else {
                        return Err(());
                    };
                    labels.push(label);
                }
            }
        }
        if let Some(text_document) = capabilities.get("textDocument") {
            let Some(text_document) = text_document.as_object() else {
                return Err(());
            };
            if let Some(diagnostic) = text_document.get("diagnostic") {
                if !diagnostic.is_object() {
                    return Err(());
                }
                pull_diagnostics_supported = true;
            }
            if let Some(document_symbol) = text_document.get("documentSymbol") {
                let document_symbol = document_symbol.as_object().ok_or(())?;
                if let Some(hierarchical) = document_symbol.get("hierarchicalDocumentSymbolSupport")
                {
                    hierarchical_document_symbols = hierarchical.as_bool().ok_or(())?;
                }
            }
            hover_markup = hover::parse_hover_capability(text_document)?;
            navigation::parse_navigation_capabilities(text_document)?;
            references::parse_references_capability(text_document)?;
            prepare_rename::parse_prepare_rename_capability(text_document)?;
        }
    }
    let encoding = negotiate_position_encoding(&labels);
    let diagnostic_limits = parse_diagnostic_limits(object)?;

    let folders = match object.get("workspaceFolders") {
        None | Some(Value::Null) => Vec::new(),
        Some(value) => parse_workspace_folders(value)?,
    };
    Ok(ParsedInitializeParams {
        encoding,
        folders,
        pull_diagnostics_supported,
        diagnostic_limits,
        hierarchical_document_symbols,
        hover_markup,
        transactional_rename_supported,
    })
}

pub(crate) fn parse_text_document_position(params: &Value) -> Option<(&str, LspPosition)> {
    let object = params.as_object()?;
    let uri = object
        .get("textDocument")?
        .as_object()?
        .get("uri")?
        .as_str()?;
    let position = object.get("position")?.as_object()?;
    let line = position
        .get("line")?
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())?;
    let character = position
        .get("character")?
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())?;
    Some((uri, LspPosition::new(line, character)))
}

fn parse_open_params(params: &Value) -> Result<(String, i64, String), OverlayError> {
    let text_document = params
        .as_object()
        .and_then(|object| object.get("textDocument"))
        .and_then(Value::as_object)
        .ok_or(OverlayError::InvalidParams)?;
    let uri = text_document
        .get("uri")
        .and_then(Value::as_str)
        .ok_or(OverlayError::InvalidParams)?
        .to_owned();
    let version = parse_version(text_document.get("version"))?;
    let text = parse_text(text_document.get("text"))?;
    Ok((uri, version, text))
}

fn parse_change_params(params: &Value) -> Result<(String, i64, Vec<DocumentChange>), OverlayError> {
    let object = params.as_object().ok_or(OverlayError::InvalidParams)?;
    let text_document = object
        .get("textDocument")
        .and_then(Value::as_object)
        .ok_or(OverlayError::InvalidParams)?;
    let uri = text_document
        .get("uri")
        .and_then(Value::as_str)
        .ok_or(OverlayError::InvalidParams)?
        .to_owned();
    let version = parse_version(text_document.get("version"))?;
    let changes = object
        .get("contentChanges")
        .and_then(Value::as_array)
        .ok_or(OverlayError::InvalidParams)?;
    if changes.is_empty() || changes.len() > MAX_CONTENT_CHANGES {
        return Err(OverlayError::InvalidParams);
    }
    let mut parsed = Vec::with_capacity(changes.len());
    for change in changes {
        let change = change.as_object().ok_or(OverlayError::InvalidParams)?;
        if change.contains_key("rangeLength") {
            return Err(OverlayError::InvalidParams);
        }
        let text = parse_text(change.get("text"))?;
        if let Some(range) = change.get("range") {
            let (start, end) = parse_range(range)?;
            parsed.push(DocumentChange::Range(LspPositionEdit::new(
                start,
                end,
                text.into_bytes(),
            )));
        } else {
            parsed.push(DocumentChange::Full(text));
        }
    }
    Ok((uri, version, parsed))
}

fn parse_workspace_reload(params: &Value) -> Result<WorkspaceReload, WorkspaceReloadError> {
    let object = params
        .as_object()
        .ok_or(WorkspaceReloadError::InvalidParams)?;
    let base_revision = object
        .get("baseRevision")
        .and_then(parse_canonical_revision)
        .ok_or(WorkspaceReloadError::InvalidParams)?;
    let source_values = object
        .get("sources")
        .and_then(Value::as_array)
        .ok_or(WorkspaceReloadError::InvalidParams)?;
    let input_values = object
        .get("inputs")
        .and_then(Value::as_array)
        .ok_or(WorkspaceReloadError::InvalidParams)?;
    if source_values.len() > MAX_RELOAD_SOURCES
        || input_values.len() > 5
        || source_values.is_empty() && input_values.is_empty()
    {
        return Err(WorkspaceReloadError::InvalidParams);
    }

    let mut total_bytes = 0usize;
    let mut sources = Vec::with_capacity(source_values.len());
    for value in source_values {
        let entry = value
            .as_object()
            .ok_or(WorkspaceReloadError::InvalidParams)?;
        let uri = entry
            .get("uri")
            .and_then(Value::as_str)
            .ok_or(WorkspaceReloadError::InvalidParams)?
            .to_owned();
        let identity = document_identity(&uri).map_err(|_| WorkspaceReloadError::InvalidParams)?;
        if identity.temporary {
            return Err(WorkspaceReloadError::InvalidParams);
        }
        let text = parse_reload_text(entry.get("text"), &mut total_bytes)?;
        sources.push(SourceReload { uri, text });
    }
    sources.sort_by(|left, right| left.uri.cmp(&right.uri));
    if sources.windows(2).any(|pair| pair[0].uri == pair[1].uri) {
        return Err(WorkspaceReloadError::InvalidParams);
    }

    let mut inputs = Vec::with_capacity(input_values.len());
    for value in input_values {
        let entry = value
            .as_object()
            .ok_or(WorkspaceReloadError::InvalidParams)?;
        let kind = match entry.get("name").and_then(Value::as_str) {
            Some("manifest") => WorkspaceInput::PackageManifest,
            Some("lock") => WorkspaceInput::PackageLock,
            Some("config") => WorkspaceInput::Config,
            Some("profile") => WorkspaceInput::Profile,
            Some("target") => WorkspaceInput::Target,
            _ => return Err(WorkspaceReloadError::InvalidParams),
        };
        let text = parse_reload_text(entry.get("text"), &mut total_bytes)?;
        inputs.push(InputReload { kind, text });
    }
    inputs.sort_by_key(|input| input.kind);
    if inputs.windows(2).any(|pair| pair[0].kind == pair[1].kind) {
        return Err(WorkspaceReloadError::InvalidParams);
    }

    Ok(WorkspaceReload {
        base_revision,
        sources,
        inputs,
    })
}

fn parse_reload_text(
    value: Option<&Value>,
    total_bytes: &mut usize,
) -> Result<Option<String>, WorkspaceReloadError> {
    match value {
        Some(Value::Null) => Ok(None),
        Some(Value::String(text)) if text.len() <= MAX_DOCUMENT_BYTES => {
            *total_bytes = total_bytes
                .checked_add(text.len())
                .filter(|total| *total <= MAX_RELOAD_TEXT_BYTES)
                .ok_or(WorkspaceReloadError::InvalidParams)?;
            Ok(Some(text.clone()))
        }
        _ => Err(WorkspaceReloadError::InvalidParams),
    }
}

fn parse_canonical_revision(value: &Value) -> Option<u64> {
    let text = value.as_str()?;
    if text == "0" {
        return Some(0);
    }
    let bytes = text.as_bytes();
    if !matches!(bytes.first(), Some(b'1'..=b'9')) || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    text.parse().ok()
}

fn parse_range(value: &Value) -> Result<(LspPosition, LspPosition), OverlayError> {
    let range = value.as_object().ok_or(OverlayError::InvalidParams)?;
    Ok((
        parse_position(range.get("start"))?,
        parse_position(range.get("end"))?,
    ))
}

fn parse_position(value: Option<&Value>) -> Result<LspPosition, OverlayError> {
    let position = value
        .and_then(Value::as_object)
        .ok_or(OverlayError::InvalidParams)?;
    let line = position
        .get("line")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or(OverlayError::InvalidParams)?;
    let character = position
        .get("character")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or(OverlayError::InvalidParams)?;
    Ok(LspPosition::new(line, character))
}

fn parse_close_params(params: &Value) -> Result<String, OverlayError> {
    params
        .as_object()
        .and_then(|object| object.get("textDocument"))
        .and_then(Value::as_object)
        .and_then(|document| document.get("uri"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or(OverlayError::InvalidParams)
}

fn parse_formatting_params(params: &Value) -> Result<String, OverlayError> {
    let object = params.as_object().ok_or(OverlayError::InvalidParams)?;
    if object.len() != 2 {
        return Err(OverlayError::InvalidParams);
    }
    let text_document = object
        .get("textDocument")
        .and_then(Value::as_object)
        .ok_or(OverlayError::InvalidParams)?;
    if text_document.len() != 1 {
        return Err(OverlayError::InvalidParams);
    }
    let uri = text_document
        .get("uri")
        .and_then(Value::as_str)
        .ok_or(OverlayError::InvalidParams)?;
    document_identity(uri)?;
    let options = object
        .get("options")
        .and_then(Value::as_object)
        .ok_or(OverlayError::InvalidParams)?;
    if options.len() != 2
        || options.get("tabSize").and_then(Value::as_u64) != Some(4)
        || options.get("insertSpaces").and_then(Value::as_bool) != Some(true)
    {
        return Err(OverlayError::InvalidParams);
    }
    Ok(uri.to_owned())
}

fn parse_version(value: Option<&Value>) -> Result<i64, OverlayError> {
    let version = value
        .and_then(Value::as_i64)
        .ok_or(OverlayError::InvalidParams)?;
    if version < 0 {
        return Err(OverlayError::InvalidParams);
    }
    Ok(version)
}

fn parse_text(value: Option<&Value>) -> Result<String, OverlayError> {
    let text = value
        .and_then(Value::as_str)
        .ok_or(OverlayError::InvalidParams)?;
    if text.len() > MAX_DOCUMENT_BYTES {
        return Err(OverlayError::TextTooLarge);
    }
    Ok(text.to_owned())
}

fn document_identity(uri: &str) -> Result<DocumentIdentity, OverlayError> {
    if !valid_text(uri, 1, MAX_WORKSPACE_URI_BYTES)
        || uri.contains('?')
        || uri.contains('#')
        || uri.contains('%')
    {
        return Err(OverlayError::InvalidUri);
    }
    let (prefix, path) = if let Some(path) = uri.strip_prefix("ling://workspace/") {
        ("workspace", path)
    } else if let Some(path) = uri.strip_prefix("ling://dependency/") {
        ("dependency", path)
    } else if let Some(path) = uri.strip_prefix("untitled://ling/") {
        ("untitled", path)
    } else {
        return Err(OverlayError::InvalidUri);
    };
    if path.is_empty() || !path.ends_with(".ling") {
        return Err(OverlayError::InvalidUri);
    }
    let logical_name = match prefix {
        "workspace" => path.to_owned(),
        "dependency" => {
            let (package, logical) = path.split_once('/').ok_or(OverlayError::InvalidUri)?;
            if package.is_empty() || package.contains('/') {
                return Err(OverlayError::InvalidUri);
            }
            format!("dependencies/{package}/{logical}")
        }
        "untitled" => format!("untitled/{path}"),
        _ => return Err(OverlayError::InvalidUri),
    };
    validate_logical_name(&logical_name).map_err(|_| OverlayError::InvalidUri)?;
    Ok(DocumentIdentity {
        logical_name,
        writable: prefix != "dependency",
        temporary: prefix == "untitled",
    })
}

fn overlay_error_details(error: &OverlayError) -> (i32, &'static str) {
    match error {
        OverlayError::StaleVersion { .. } => (
            DOCUMENT_STALE,
            "文档版本过旧或不单调 / document version is stale or non-monotonic",
        ),
        OverlayError::ReadOnly => (
            DOCUMENT_READ_ONLY,
            "依赖文档只读 / dependency document is read-only",
        ),
        OverlayError::InvalidUri => (
            DOCUMENT_URI,
            "文档 URI 无效 / invalid or unsupported document URI",
        ),
        OverlayError::TextTooLarge => (
            INVALID_PARAMS,
            "文档文本过大 / document text exceeds the size limit",
        ),
        OverlayError::UnknownDocument | OverlayError::NotOpen => {
            (INVALID_PARAMS, "文档状态无效 / invalid document state")
        }
        OverlayError::AlreadyOpen => (INVALID_PARAMS, "文档已经打开 / document is already open"),
        OverlayError::InvalidRange => (
            INVALID_PARAMS,
            "文档编辑范围无效 / invalid document edit range",
        ),
        OverlayError::InvalidParams | OverlayError::Vfs(_) => {
            (INVALID_PARAMS, "文档参数无效 / invalid document parameters")
        }
    }
}

fn parse_workspace_folders(value: &Value) -> Result<Vec<WorkspaceFolder>, ()> {
    let Some(array) = value.as_array() else {
        return Err(());
    };
    if array.len() > MAX_WORKSPACE_FOLDERS {
        return Err(());
    }

    let mut folders = Vec::with_capacity(array.len());
    for value in array {
        let Some(object) = value.as_object() else {
            return Err(());
        };
        let Some(uri) = object.get("uri").and_then(Value::as_str) else {
            return Err(());
        };
        let Some(name) = object.get("name").and_then(Value::as_str) else {
            return Err(());
        };
        if !valid_text(uri, 1, MAX_WORKSPACE_URI_BYTES)
            || !valid_text(name, 1, MAX_WORKSPACE_NAME_BYTES)
            || folders
                .iter()
                .any(|folder: &WorkspaceFolder| folder.uri == uri)
        {
            return Err(());
        }
        folders.push(WorkspaceFolder {
            uri: uri.to_owned(),
            name: name.to_owned(),
        });
    }
    Ok(folders)
}

fn valid_text(value: &str, minimum: usize, maximum: usize) -> bool {
    let length = value.len();
    length >= minimum
        && length <= maximum
        && !value
            .chars()
            .any(|character| character == '\0' || character.is_control())
}

fn valid_request_id(value: &Value) -> bool {
    matches!(value, Value::Null | Value::String(_) | Value::Number(_))
}

fn parse_content_length(value: &[u8]) -> Result<usize, TransportError> {
    if value.is_empty() || !value.iter().all(u8::is_ascii_digit) {
        return Err(TransportError::InvalidContentLength);
    }
    let mut length = 0usize;
    for digit in value {
        length = length
            .checked_mul(10)
            .and_then(|value| value.checked_add(usize::from(digit - b'0')))
            .ok_or(TransportError::FrameTooLarge { length: usize::MAX })?;
    }
    if length > MAX_FRAME_BYTES {
        return Err(TransportError::FrameTooLarge { length });
    }
    Ok(length)
}

fn read_frame(reader: &mut impl BufRead) -> Result<Option<Vec<u8>>, TransportError> {
    let mut header_bytes = 0usize;
    let mut content_length = None;
    let mut line = Vec::new();
    loop {
        line.clear();
        let read = reader.read_until(b'\n', &mut line)?;
        if read == 0 {
            if header_bytes == 0 {
                return Ok(None);
            }
            return Err(TransportError::UnexpectedEof);
        }
        header_bytes = header_bytes
            .checked_add(read)
            .ok_or(TransportError::HeadersTooLarge)?;
        if header_bytes > MAX_HEADER_BYTES {
            return Err(TransportError::HeadersTooLarge);
        }
        if !line.ends_with(b"\r\n") {
            return Err(TransportError::InvalidHeader);
        }
        line.truncate(line.len() - 2);
        if line.is_empty() {
            break;
        }
        if !line.is_ascii() {
            return Err(TransportError::InvalidHeader);
        }
        let Some(colon) = line.iter().position(|byte| *byte == b':') else {
            return Err(TransportError::InvalidHeader);
        };
        if colon == 0 {
            return Err(TransportError::InvalidHeader);
        }
        let name = line[..colon]
            .iter()
            .map(u8::to_ascii_lowercase)
            .collect::<Vec<_>>();
        let value = line[colon + 1..]
            .iter()
            .copied()
            .skip_while(u8::is_ascii_whitespace)
            .collect::<Vec<_>>();
        if name == b"content-length" {
            if content_length.is_some() {
                return Err(TransportError::DuplicateContentLength);
            }
            content_length = Some(parse_content_length(&value)?);
        }
    }

    let length = content_length.ok_or(TransportError::MissingContentLength)?;
    let mut body = vec![0_u8; length];
    reader
        .read_exact(&mut body)
        .map_err(|error| match error.kind() {
            io::ErrorKind::UnexpectedEof => TransportError::UnexpectedEof,
            _ => TransportError::Io(error),
        })?;
    Ok(Some(body))
}

fn write_frame(writer: &mut impl Write, body: &[u8]) -> Result<(), TransportError> {
    if body.len() > MAX_FRAME_BYTES {
        return Err(TransportError::FrameTooLarge { length: body.len() });
    }
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
    writer.write_all(body)?;
    writer.flush()?;
    Ok(())
}

fn parse_request_id(id: &Value) -> Value {
    id.clone()
}

fn response_or_none(is_request: bool, id: Value, code: i32, message: &str) -> HandleOutcome {
    if is_request {
        error_or_none(true, parse_request_id(&id), code, message)
    } else {
        HandleOutcome::NoResponse
    }
}

fn error_or_none(is_request: bool, id: Value, code: i32, message: &str) -> HandleOutcome {
    if is_request {
        HandleOutcome::Response(error_response(Some(id), code, message))
    } else {
        HandleOutcome::NoResponse
    }
}

fn success_response(id: Value, result: Value) -> Vec<u8> {
    encode(&json!({
        "jsonrpc": JSON_RPC_VERSION,
        "id": id,
        "result": result,
    }))
}

fn error_response(id: Option<Value>, code: i32, message: &str) -> Vec<u8> {
    encode(&json!({
        "jsonrpc": JSON_RPC_VERSION,
        "id": id.unwrap_or(Value::Null),
        "error": {
            "code": code,
            "message": message,
        },
    }))
}

fn formatting_internal_error(id: Value) -> HandleOutcome {
    error_or_none(
        true,
        id,
        INTERNAL_ERROR,
        "文档格式化失败 / document formatting failed",
    )
}

fn initialize_result(
    encoding: PositionEncoding,
    pull_diagnostics_supported: bool,
    diagnostic_limits: DiagnosticLimits,
    hierarchical_document_symbols: bool,
    hover_markup: hover::HoverMarkup,
) -> Value {
    let mut result = json!({
        "capabilities": {
            "declarationProvider": true,
            "definitionProvider": true,
            "positionEncoding": encoding.wire_name(),
            "documentFormattingProvider": true,
            "documentSymbolProvider": true,
            "hoverProvider": true,
            "referencesProvider": true,
            "renameProvider": {
                "prepareProvider": true,
                "workDoneProgress": false,
            },
            "typeDefinitionProvider": true,
            "experimental": {
                "lingHover": {
                    "maxContentBytes": MAX_HOVER_CONTENT_BYTES,
                    "maxEntries": MAX_HOVER_ENTRIES,
                    "markup": hover_markup.wire_name(),
                    "version": HOVER_PROTOCOL_VERSION,
                },
                "lingNavigation": {
                    "maxTargets": MAX_NAVIGATION_TARGETS,
                    "version": NAVIGATION_PROTOCOL_VERSION,
                },
                "lingReferences": {
                    "emittedRelationKinds": ["read", "write", "call"],
                    "maxLocations": MAX_REFERENCE_LOCATIONS,
                    "relationKinds": ["read", "write", "call", "type", "implementation"],
                    "version": REFERENCES_PROTOCOL_VERSION,
                },
                "lingPrepareRename": {
                    "result": "rangeWithPlaceholder",
                    "version": PREPARE_RENAME_PROTOCOL_VERSION,
                },
                "lingRename": {
                    "newName": "unicode17-xid-nfc-allowed-non-suspicious",
                    "result": "versionedDocumentChanges",
                    "transactional": true,
                    "version": RENAME_PROTOCOL_VERSION,
                },
                "lingDocumentSymbols": {
                    "maxSymbols": MAX_DOCUMENT_SYMBOLS,
                    "mode": if hierarchical_document_symbols { "hierarchical" } else { "flat" },
                    "version": DOCUMENT_SYMBOL_PROTOCOL_VERSION,
                },
                "lingOverlay": {
                    "changeLimit": MAX_CONTENT_CHANGES,
                    "version": OVERLAY_PROTOCOL_VERSION,
                },
                "lingWorkspaceReload": {
                    "inputLimit": 5,
                    "sourceLimit": MAX_RELOAD_SOURCES,
                    "totalByteLimit": MAX_RELOAD_TEXT_BYTES,
                    "version": WORKSPACE_PROTOCOL_VERSION,
                },
                "lingPublishDiagnostics": {
                    "adapterVersion": DIAGNOSTIC_PROTOCOL_VERSION,
                    "debounce": "message-boundary",
                    "version": PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION,
                },
                "lingDiagnosticControl": {
                    "maxPerDocument": diagnostic_limits.per_document(),
                    "maxPerWorkspace": diagnostic_limits.per_workspace(),
                    "version": DIAGNOSTIC_CONTROL_PROTOCOL_VERSION,
                },
            },
            "textDocumentSync": {
                "change": 2,
                "openClose": true,
            },
            "workspace": {
                "workspaceFolders": {
                    "changeNotifications": false,
                    "supported": true,
                },
            },
        },
        "serverInfo": {
            "name": "ling",
            "version": env!("CARGO_PKG_VERSION"),
        },
    });
    if pull_diagnostics_supported {
        result["capabilities"]["diagnosticProvider"] = json!({
            "identifier": PULL_DIAGNOSTICS_PROTOCOL_VERSION,
            "interFileDependencies": true,
            "workDoneProgress": false,
            "workspaceDiagnostics": true,
        });
    }
    result
}

fn encode(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_else(|_| {
        b"{\"error\":{\"code\":-32603,\"message\":\"internal serialization error\"},\"id\":null,\"jsonrpc\":\"2.0\"}".to_vec()
    })
}

fn empty_object() -> Value {
    Value::Object(Map::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(id: u64, method: &str, params: Value) -> Vec<u8> {
        serde_json::to_vec(&json!({
            "jsonrpc": JSON_RPC_VERSION,
            "id": id,
            "method": method,
            "params": params,
        }))
        .expect("test request is serializable")
    }

    #[test]
    fn initialize_negotiates_encoding_and_folders() {
        let mut server = LspServer::new();
        let outcome = server.handle_json(&request(
            1,
            "initialize",
            json!({
                "capabilities": {"general": {"positionEncodings": ["unknown", "utf-32", "utf-8"]}},
                "workspaceFolders": [{"uri": "file:///ling", "name": "凌"}],
            }),
        ));
        assert!(matches!(outcome, HandleOutcome::Response(_)));
        assert_eq!(server.position_encoding(), PositionEncoding::Utf32);
        assert_eq!(server.state(), LifecycleState::AwaitingInitialized);
        assert_eq!(server.workspace_folders()[0].name(), "凌");
    }

    #[test]
    fn malformed_json_is_a_parse_error_response() {
        let mut server = LspServer::new();
        let HandleOutcome::Response(response) = server.handle_json(b"{") else {
            panic!("parse error must produce a response")
        };
        let value: Value = serde_json::from_slice(&response).expect("error response is JSON");
        assert_eq!(value["error"]["code"], PARSE_ERROR);
        assert_eq!(value["id"], Value::Null);
    }

    #[test]
    fn early_exit_has_failure_status_and_normal_exit_succeeds() {
        let mut early = LspServer::new();
        assert_eq!(
            early.handle_json(
                &json!({"jsonrpc": "2.0", "method": "exit"})
                    .to_string()
                    .into_bytes()
            ),
            HandleOutcome::Exit { code: 1 }
        );

        let mut normal = LspServer::new();
        let _ = normal.handle_json(&request(1, "initialize", json!({})));
        let _ = normal.handle_json(
            &json!({"jsonrpc": "2.0", "method": "initialized"})
                .to_string()
                .into_bytes(),
        );
        let _ = normal.handle_json(&request(2, "shutdown", Value::Null));
        assert_eq!(
            normal.handle_json(
                &json!({"jsonrpc": "2.0", "method": "exit"})
                    .to_string()
                    .into_bytes()
            ),
            HandleOutcome::Exit { code: 0 }
        );
    }
}
