//! Bounded Preview lifecycle and stdio transport for `ling lsp --stdio`.
//!
//! This crate deliberately does not implement document synchronization or
//! compiler queries. RFC-0004 is the authority for this lifecycle boundary;
//! DEC-0029 remains the authority for position projection.

use std::fmt;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

pub use ling_source::PositionEncoding;
use ling_source::negotiate_position_encoding;
use serde_json::{Map, Value, json};

/// Version marker for the current Preview lifecycle protocol.
pub const PROTOCOL_VERSION: &str = "ling.lsp.lifecycle/0.1";
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

const PARSE_ERROR: i32 = -32_700;
const INVALID_REQUEST: i32 = -32_600;
const METHOD_NOT_FOUND: i32 = -32_601;
const INVALID_PARAMS: i32 = -32_602;
const SERVER_NOT_INITIALIZED: i32 = -32_002;
const SERVER_SHUTTING_DOWN: i32 = -32_003;

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
        }
    }
}

impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for TransportError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// A single-process, deterministic Preview lifecycle server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LspServer {
    state: LifecycleState,
    position_encoding: PositionEncoding,
    workspace_folders: Vec<WorkspaceFolder>,
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

        let (encoding, folders) = match parse_initialize_params(&params) {
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
        self.state = LifecycleState::AwaitingInitialized;
        HandleOutcome::Response(success_response(id, initialize_result(encoding)))
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
    }
}

fn parse_initialize_params(params: &Value) -> Result<(PositionEncoding, Vec<WorkspaceFolder>), ()> {
    let Some(object) = params.as_object() else {
        return Err(());
    };

    let mut labels = Vec::new();
    if let Some(capabilities) = object.get("capabilities") {
        let Some(capabilities) = capabilities.as_object() else {
            return Err(());
        };
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
    }
    let encoding = negotiate_position_encoding(&labels);

    let folders = match object.get("workspaceFolders") {
        None | Some(Value::Null) => Vec::new(),
        Some(value) => parse_workspace_folders(value)?,
    };
    Ok((encoding, folders))
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

fn initialize_result(encoding: PositionEncoding) -> Value {
    json!({
        "capabilities": {
            "positionEncoding": encoding.wire_name(),
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
    })
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
