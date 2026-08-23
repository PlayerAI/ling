use std::collections::BTreeMap;

use ling_db::{CompilerDb, QueryError};
use ling_diagnostics::Diagnostic;
use ling_source::{SourceError, SourceFile, SourceId, VfsError};
use serde_json::{Map, Value, json};

use super::{
    AdaptedDiagnostic, CancellationToken, DiagnosticAdapterError, DiagnosticAdapterInput,
    DiagnosticControlError, DiagnosticSource, JSON_RPC_VERSION, LifecycleState, LspServer,
    MAX_FRAME_BYTES, RequestSnapshot, RequestSnapshotError, adapt_diagnostics, encode,
};
use crate::diagnostic_control::{DiagnosticLimits, controlled_diagnostics};

/// Version marker for deterministic push-diagnostic publication.
pub const PUBLISH_DIAGNOSTICS_PROTOCOL_VERSION: &str = "ling.lsp.publish-diagnostics/0.2";

/// Immutable RFC-0032 analysis input captured from one complete server state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticAnalysisTicket {
    snapshot: RequestSnapshot,
    cancellation: CancellationToken,
}

impl DiagnosticAnalysisTicket {
    /// Returns the exact immutable request snapshot carried by this ticket.
    #[must_use]
    pub const fn snapshot(&self) -> &RequestSnapshot {
        &self.snapshot
    }

    /// Runs the locked-offline compiler pipeline against only captured bytes.
    pub fn compile(self) -> Result<DiagnosticAnalysisResult, DiagnosticAnalysisError> {
        self.cancellation
            .check()
            .map_err(|_| DiagnosticAnalysisError::Cancelled)?;
        let mut sources = Vec::with_capacity(self.snapshot.documents().len());
        let mut syntax_diagnostics = Vec::new();
        for (ordinal, document) in self.snapshot.documents().iter().enumerate() {
            self.cancellation
                .check()
                .map_err(|_| DiagnosticAnalysisError::Cancelled)?;
            let source_id = u32::try_from(ordinal)
                .map(SourceId::new)
                .map_err(|_| DiagnosticAnalysisError::TooManySources)?;
            let source = SourceFile::from_bytes(
                source_id,
                document.logical_name().to_owned(),
                document.bytes().to_vec(),
            )
            .map_err(|error| DiagnosticAnalysisError::Source {
                uri: document.uri().to_owned(),
                error,
            })?;
            let parsed = ling_syntax::parse(&source);
            if parsed.lexical_errors().is_empty() {
                syntax_diagnostics.extend(
                    parsed
                        .parse_errors()
                        .iter()
                        .map(|error| error.to_diagnostic(source.name())),
                );
            } else {
                syntax_diagnostics.extend(
                    parsed
                        .lexical_errors()
                        .iter()
                        .map(|error| error.to_diagnostic(source.name())),
                );
            }
            sources.push(DiagnosticSource::new(document.uri(), source));
        }

        self.cancellation
            .check()
            .map_err(|_| DiagnosticAnalysisError::Cancelled)?;
        let diagnostics = if syntax_diagnostics.is_empty() {
            compile_checked_workspace(&self.snapshot, &self.cancellation)?
        } else {
            syntax_diagnostics
        };
        let inputs = diagnostics
            .into_iter()
            .map(DiagnosticAdapterInput::new)
            .collect::<Vec<_>>();
        let adapted = if sources.is_empty() {
            Box::new([])
        } else {
            adapt_diagnostics(self.snapshot.position_encoding(), &sources, &inputs)?
        };
        self.cancellation
            .check()
            .map_err(|_| DiagnosticAnalysisError::Cancelled)?;
        Ok(DiagnosticAnalysisResult {
            snapshot: self.snapshot,
            diagnostics: adapted,
            cancellation: self.cancellation,
        })
    }
}

/// Complete adapted result tied to the exact ticket that produced it.
#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticAnalysisResult {
    snapshot: RequestSnapshot,
    diagnostics: Box<[AdaptedDiagnostic]>,
    cancellation: CancellationToken,
}

impl DiagnosticAnalysisResult {
    #[must_use]
    pub const fn snapshot(&self) -> &RequestSnapshot {
        &self.snapshot
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[AdaptedDiagnostic] {
        &self.diagnostics
    }
}

/// Atomic analysis or publication failure. No failure changes the ledger.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiagnosticAnalysisError {
    NotReady,
    Cancelled,
    Snapshot(RequestSnapshotError),
    TooManySources,
    Source { uri: String, error: SourceError },
    CompilerInput(VfsError),
    Compiler(QueryError),
    Adapter(DiagnosticAdapterError),
    Stale,
    UnknownResultUri { uri: String },
    Control(DiagnosticControlError),
    NotificationTooLarge { length: usize },
}

impl std::fmt::Display for DiagnosticAnalysisError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotReady => formatter.write_str("LSP diagnostic analysis requires Ready state"),
            Self::Cancelled => formatter.write_str("LSP diagnostic analysis was cancelled"),
            Self::Snapshot(error) => error.fmt(formatter),
            Self::TooManySources => formatter.write_str("diagnostic source count exceeds u32"),
            Self::Source { uri, error } => {
                write!(formatter, "diagnostic source {uri:?} is invalid: {error}")
            }
            Self::CompilerInput(error) => {
                write!(formatter, "diagnostic compiler input failed: {error}")
            }
            Self::Compiler(error) => write!(formatter, "diagnostic compiler query failed: {error}"),
            Self::Adapter(error) => error.fmt(formatter),
            Self::Stale => formatter.write_str("diagnostic analysis result is stale"),
            Self::UnknownResultUri { uri } => {
                write!(formatter, "diagnostic result names unknown URI {uri:?}")
            }
            Self::Control(error) => error.fmt(formatter),
            Self::NotificationTooLarge { length } => write!(
                formatter,
                "diagnostic notification length {length} exceeds {MAX_FRAME_BYTES} bytes"
            ),
        }
    }
}

impl std::error::Error for DiagnosticAnalysisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Snapshot(error) => Some(error),
            Self::Source { error, .. } => Some(error),
            Self::CompilerInput(error) => Some(error),
            Self::Compiler(error) => Some(error),
            Self::Adapter(error) => Some(error),
            Self::Control(error) => Some(error),
            Self::NotReady
            | Self::Cancelled
            | Self::TooManySources
            | Self::Stale
            | Self::UnknownResultUri { .. }
            | Self::NotificationTooLarge { .. } => None,
        }
    }
}

impl From<RequestSnapshotError> for DiagnosticAnalysisError {
    fn from(error: RequestSnapshotError) -> Self {
        Self::Snapshot(error)
    }
}

impl From<DiagnosticAdapterError> for DiagnosticAnalysisError {
    fn from(error: DiagnosticAdapterError) -> Self {
        Self::Adapter(error)
    }
}

fn compile_checked_workspace(
    snapshot: &RequestSnapshot,
    cancellation: &CancellationToken,
) -> Result<Vec<Diagnostic>, DiagnosticAnalysisError> {
    let mut compiler =
        compiler_for_snapshot(snapshot, None).map_err(DiagnosticAnalysisError::CompilerInput)?;
    compiler
        .workspace_diagnostics_with_cancellation(&|| cancellation.is_cancelled())
        .map(Vec::from)
        .map_err(|error| match error {
            QueryError::Cancelled => DiagnosticAnalysisError::Cancelled,
            error => DiagnosticAnalysisError::Compiler(error),
        })
}

pub(crate) fn compiler_for_snapshot(
    snapshot: &RequestSnapshot,
    requested_temporary_uri: Option<&str>,
) -> Result<CompilerDb, VfsError> {
    compiler_for_snapshot_with_overrides(snapshot, requested_temporary_uri, &BTreeMap::new())
}

pub(crate) fn compiler_for_snapshot_with_overrides(
    snapshot: &RequestSnapshot,
    requested_temporary_uri: Option<&str>,
    overrides: &BTreeMap<String, Vec<u8>>,
) -> Result<CompilerDb, VfsError> {
    let mut compiler = CompilerDb::new();
    for input in snapshot.inputs() {
        compiler.set_workspace_input(input.kind(), input.bytes().to_vec())?;
    }
    for document in snapshot.documents().iter().filter(|document| {
        requested_temporary_uri.map_or_else(
            || !document.is_temporary(),
            |uri| document.is_temporary() && document.uri() == uri,
        )
    }) {
        let bytes = overrides
            .get(document.logical_name())
            .map_or_else(|| document.bytes().to_vec(), Clone::clone);
        compiler.set_disk_snapshot(document.logical_name(), bytes)?;
    }
    Ok(compiler)
}

impl LspServer {
    /// Computes diagnostics from one current immutable snapshot without
    /// consuming pending push work or mutating the publication ledger.
    pub(crate) fn analyze_current_diagnostics(
        &self,
    ) -> Result<DiagnosticAnalysisResult, DiagnosticAnalysisError> {
        if self.state != LifecycleState::Ready {
            return Err(DiagnosticAnalysisError::NotReady);
        }
        let result = DiagnosticAnalysisTicket {
            snapshot: self.capture_request_snapshot()?,
            cancellation: CancellationToken::new(),
        }
        .compile()?;
        if self.capture_request_snapshot()? != result.snapshot {
            return Err(DiagnosticAnalysisError::Stale);
        }
        Ok(result)
    }

    /// Returns whether one or more successful mutations await diagnostics.
    #[must_use]
    pub const fn diagnostics_pending(&self) -> bool {
        self.diagnostics_pending
    }

    /// Captures the newest pending state without clearing pending work.
    pub fn begin_diagnostic_analysis(
        &self,
    ) -> Result<Option<DiagnosticAnalysisTicket>, DiagnosticAnalysisError> {
        if self.state != LifecycleState::Ready {
            return Err(DiagnosticAnalysisError::NotReady);
        }
        if !self.diagnostics_pending {
            return Ok(None);
        }
        Ok(Some(DiagnosticAnalysisTicket {
            snapshot: self.capture_request_snapshot()?,
            cancellation: self.diagnostic_cancellation.clone().unwrap_or_default(),
        }))
    }

    /// Publishes a complete result only if its full ticket remains current.
    pub fn complete_diagnostic_analysis(
        &mut self,
        result: DiagnosticAnalysisResult,
    ) -> Result<usize, DiagnosticAnalysisError> {
        if self.state != LifecycleState::Ready {
            return Err(DiagnosticAnalysisError::NotReady);
        }
        if self.capture_request_snapshot()? != result.snapshot {
            return Err(DiagnosticAnalysisError::Stale);
        }
        result
            .cancellation
            .check()
            .map_err(|_| DiagnosticAnalysisError::Cancelled)?;

        let candidate = publication_params(&result, self.diagnostic_limits)?;
        let mut changed = BTreeMap::new();
        for (uri, params) in &candidate {
            if self.published_diagnostics.get(uri) != Some(params) {
                changed.insert(uri.clone(), params.clone());
            }
        }
        for (uri, previous) in &self.published_diagnostics {
            if !candidate.contains_key(uri) {
                changed.insert(uri.clone(), clearance_params(uri, previous));
            }
        }
        let notifications = changed
            .into_values()
            .map(|params| {
                encode(&json!({
                    "jsonrpc": JSON_RPC_VERSION,
                    "method": "textDocument/publishDiagnostics",
                    "params": params,
                }))
            })
            .collect::<Vec<_>>();
        if let Some(notification) = notifications
            .iter()
            .find(|notification| notification.len() > MAX_FRAME_BYTES)
        {
            return Err(DiagnosticAnalysisError::NotificationTooLarge {
                length: notification.len(),
            });
        }
        result
            .cancellation
            .check()
            .map_err(|_| DiagnosticAnalysisError::Cancelled)?;
        let count = notifications.len();
        self.published_diagnostics = candidate;
        self.diagnostics_pending = false;
        self.outbound_notifications.extend(notifications);
        Ok(count)
    }

    /// Compiles and publishes the newest pending snapshot synchronously.
    pub fn flush_pending_diagnostics(&mut self) -> Result<usize, DiagnosticAnalysisError> {
        let Some(ticket) = self.begin_diagnostic_analysis()? else {
            return Ok(0);
        };
        let result = match ticket.compile() {
            Ok(result) => result,
            Err(DiagnosticAnalysisError::Cancelled) => return Ok(0),
            Err(error) => return Err(error),
        };
        match self.complete_diagnostic_analysis(result) {
            Err(DiagnosticAnalysisError::Cancelled | DiagnosticAnalysisError::Stale) => Ok(0),
            result => result,
        }
    }

    /// Drains complete encoded JSON-RPC notification bodies in wire order.
    pub fn take_notifications(&mut self) -> Box<[Vec<u8>]> {
        std::mem::take(&mut self.outbound_notifications).into_boxed_slice()
    }

    pub(crate) fn mark_diagnostics_pending(&mut self) {
        if let Some(cancellation) = &self.diagnostic_cancellation {
            cancellation.cancel();
        }
        self.diagnostic_cancellation = Some(CancellationToken::new());
        self.diagnostics_pending = true;
    }
}

fn publication_params(
    result: &DiagnosticAnalysisResult,
    limits: DiagnosticLimits,
) -> Result<BTreeMap<String, Value>, DiagnosticAnalysisError> {
    let grouped = controlled_diagnostics(result, limits)?;
    Ok(grouped
        .into_iter()
        .map(|(uri, diagnostics)| {
            let version = result
                .snapshot
                .document(&uri)
                .and_then(super::RequestDocument::client_version);
            let params = diagnostic_params(&uri, diagnostics, version);
            (uri, params)
        })
        .collect())
}

pub(crate) fn grouped_diagnostics(
    result: &DiagnosticAnalysisResult,
) -> Result<BTreeMap<String, Vec<Value>>, DiagnosticAnalysisError> {
    let mut grouped = result
        .snapshot
        .documents()
        .iter()
        .map(|document| (document.uri().to_owned(), Vec::new()))
        .collect::<BTreeMap<_, _>>();
    for diagnostic in &result.diagnostics {
        grouped
            .get_mut(diagnostic.uri())
            .ok_or_else(|| DiagnosticAnalysisError::UnknownResultUri {
                uri: diagnostic.uri().to_owned(),
            })?
            .push(diagnostic.value().clone());
    }
    Ok(grouped)
}

fn diagnostic_params(uri: &str, diagnostics: Vec<Value>, version: Option<i64>) -> Value {
    let mut params = Map::new();
    params.insert("diagnostics".to_owned(), Value::Array(diagnostics));
    params.insert("uri".to_owned(), Value::String(uri.to_owned()));
    if let Some(version) = version {
        params.insert("version".to_owned(), Value::from(version));
    }
    Value::Object(params)
}

fn clearance_params(uri: &str, previous: &Value) -> Value {
    diagnostic_params(
        uri,
        Vec::new(),
        previous.get("version").and_then(Value::as_i64),
    )
}
