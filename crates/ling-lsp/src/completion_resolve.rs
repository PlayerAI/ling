//! Snapshot-bound lazy completion presentation authorized by RFC-0043.

use std::collections::BTreeMap;
use std::sync::Arc;

use ling_db::{
    CompilerDb, QueryError, ResolvedCompletionMetadata, ResolvedCompletionSourceIdentity,
    ResolvedCompletionSourceKind,
};
use ling_format::{FormatIrBuildError, FormatNode, FormatToken, build_format_ir};
use ling_source::{FileOrigin, SourceError, SourceFile, Span, WorkspaceInput};
use serde_json::{Map, Value, json};

use super::publication::compiler_for_snapshot;
use super::{
    HandleOutcome, INVALID_PARAMS, LifecycleState, LspServer, MAX_FRAME_BYTES, RequestSnapshot,
    RequestSnapshotError, error_or_none, success_response,
};

/// Negotiated completion protocol marker when resolve is enabled.
pub const COMPLETION_RESOLVE_COMPLETION_VERSION: &str = "ling.lsp.completion/0.2";
/// Completion-item resolve handle and presentation marker.
pub const COMPLETION_RESOLVE_PROTOCOL_VERSION: &str = "ling.lsp.completion-resolve/0.1";
/// Maximum number of live session-local completion handles.
pub const MAX_COMPLETION_RESOLVE_HANDLES: usize = 1_024;
/// Maximum checked signature presentation size.
pub const MAX_COMPLETION_DETAIL_BYTES: usize = 4_096;
/// Maximum combined checked and attached Author Source documentation size.
pub const MAX_COMPLETION_DOCUMENTATION_BYTES: usize = 16_384;

const REQUEST_FAILED: i32 = -32_803;
const HANDLE_DIGEST_HEX_BYTES: usize = 64;
const HANDLE_PREFIX: &str = "ling.lsp.completion-resolve/0.1:blake3:";
const REQUIRED_ITEM_FIELDS: &[&str] = &[
    "label",
    "kind",
    "sortText",
    "filterText",
    "insertTextFormat",
    "textEdit",
    "data",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CompletionDocumentationFormat {
    Plaintext,
    Markdown,
}

impl CompletionDocumentationFormat {
    pub(crate) const fn wire_name(self) -> &'static str {
        match self {
            Self::Plaintext => "plaintext",
            Self::Markdown => "markdown",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CompletionResolveOptions {
    enabled: bool,
    documentation_format: CompletionDocumentationFormat,
}

impl CompletionResolveOptions {
    pub(crate) const fn disabled() -> Self {
        Self {
            enabled: false,
            documentation_format: CompletionDocumentationFormat::Plaintext,
        }
    }

    pub(crate) const fn enabled(self) -> bool {
        self.enabled
    }

    pub(crate) const fn documentation_format(self) -> CompletionDocumentationFormat {
        self.documentation_format
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CompletionResolveRecord {
    snapshot: Arc<RequestSnapshot>,
    uri: String,
    item: Value,
    name: String,
    metadata_identity: Option<ResolvedCompletionSourceIdentity>,
    documentation_format: CompletionDocumentationFormat,
}

impl CompletionResolveRecord {
    pub(crate) fn new(
        snapshot: Arc<RequestSnapshot>,
        uri: String,
        item: Value,
        name: String,
        metadata_identity: Option<ResolvedCompletionSourceIdentity>,
        documentation_format: CompletionDocumentationFormat,
    ) -> Self {
        Self {
            snapshot,
            uri,
            item,
            name,
            metadata_identity,
            documentation_format,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CompletionResolveState {
    options: CompletionResolveOptions,
    records: BTreeMap<String, CompletionResolveRecord>,
}

impl CompletionResolveState {
    pub(crate) const fn new() -> Self {
        Self {
            options: CompletionResolveOptions::disabled(),
            records: BTreeMap::new(),
        }
    }

    pub(crate) const fn options(&self) -> CompletionResolveOptions {
        self.options
    }

    pub(crate) fn configure(&mut self, options: CompletionResolveOptions) {
        self.options = options;
        self.records.clear();
    }

    pub(crate) fn publish(
        &mut self,
        batch: Vec<(String, CompletionResolveRecord)>,
    ) -> Result<(), CompletionResolvePublishError> {
        if batch.len() > MAX_COMPLETION_RESOLVE_HANDLES {
            return Err(CompletionResolvePublishError::Bound);
        }
        let mut unique = BTreeMap::new();
        for (handle, record) in batch {
            if let Some(existing) = unique.insert(handle, record.clone()) {
                if existing != record {
                    return Err(CompletionResolvePublishError::Collision);
                }
            }
        }
        for (handle, record) in &unique {
            if self
                .records
                .get(handle)
                .is_some_and(|existing| existing != record)
            {
                return Err(CompletionResolvePublishError::Collision);
            }
        }
        let additional = unique
            .keys()
            .filter(|handle| !self.records.contains_key(*handle))
            .count();
        if self.records.len().saturating_add(additional) > MAX_COMPLETION_RESOLVE_HANDLES {
            self.records.clear();
        }
        self.records.extend(unique);
        Ok(())
    }

    fn record(&self, handle: &str) -> Option<&CompletionResolveRecord> {
        self.records.get(handle)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CompletionResolvePublishError {
    Bound,
    Collision,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CompletionResolveError {
    InvalidParams,
    Unavailable,
    Snapshot(RequestSnapshotError),
    CompilerInput(ling_source::VfsError),
    Compiler(QueryError),
    Source(SourceError),
    Format(FormatIrBuildError),
    MetadataMismatch,
    UnsafeMarkup,
    DetailBound,
    DocumentationBound,
    Stale,
    ResponseTooLarge,
}

impl LspServer {
    pub(crate) fn completion_resolve(
        &self,
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
        if !self.completion_resolve.options().enabled() {
            return completion_resolve_error(id, &CompletionResolveError::Unavailable);
        }
        match self.completion_resolve_result(&params) {
            Ok(result) => {
                let response = success_response(id.clone(), result);
                if response.len() > MAX_FRAME_BYTES {
                    return completion_resolve_error(id, &CompletionResolveError::ResponseTooLarge);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => completion_resolve_error(id, &error),
        }
    }

    fn completion_resolve_result(&self, params: &Value) -> Result<Value, CompletionResolveError> {
        let handle = parse_resolve_item(params)?;
        let record = self
            .completion_resolve
            .record(handle)
            .ok_or(CompletionResolveError::Unavailable)?;
        validate_retained_item(params, &record.item)?;
        let current = self
            .capture_request_snapshot()
            .map_err(CompletionResolveError::Snapshot)?;
        if current != *record.snapshot {
            return Err(CompletionResolveError::Stale);
        }

        let mut result = record.item.clone();
        if let Some(identity) = &record.metadata_identity {
            let document = record
                .snapshot
                .document(&record.uri)
                .ok_or(CompletionResolveError::MetadataMismatch)?;
            let temporary = document.is_temporary().then_some(document.uri());
            let mut compiler = compiler_for_snapshot(&record.snapshot, temporary)
                .map_err(CompletionResolveError::CompilerInput)?;
            let file = compiler
                .vfs()
                .file_id(document.logical_name())
                .ok_or(CompletionResolveError::MetadataMismatch)?;
            let index = compiler
                .resolved_completion_metadata_index(file)
                .map_err(CompletionResolveError::Compiler)?;
            let metadata = index
                .identity(identity)
                .filter(|metadata| metadata.name() == record.name)
                .ok_or(CompletionResolveError::MetadataMismatch)?;
            if metadata.type_display().is_some() {
                let author_documentation =
                    attached_author_documentation(&mut compiler, &record.snapshot, metadata)?;
                add_checked_presentation(
                    &mut result,
                    metadata,
                    author_documentation.as_deref(),
                    record.documentation_format,
                )?;
            }
        }

        if self
            .capture_request_snapshot()
            .map_err(CompletionResolveError::Snapshot)?
            != *record.snapshot
        {
            return Err(CompletionResolveError::Stale);
        }
        Ok(result)
    }
}

pub(crate) fn parse_completion_resolve_capability(
    text_document: &Map<String, Value>,
) -> Result<CompletionResolveOptions, ()> {
    let Some(completion) = text_document.get("completion") else {
        return Ok(CompletionResolveOptions::disabled());
    };
    let completion = completion.as_object().ok_or(())?;
    let Some(item) = completion.get("completionItem") else {
        return Ok(CompletionResolveOptions::disabled());
    };
    let item = item.as_object().ok_or(())?;
    let documentation_format = parse_documentation_format(item)?;
    let Some(resolve) = item.get("resolveSupport") else {
        return Ok(CompletionResolveOptions::disabled());
    };
    let resolve = resolve.as_object().ok_or(())?;
    let properties = resolve
        .get("properties")
        .and_then(Value::as_array)
        .filter(|properties| !properties.is_empty())
        .ok_or(())?;
    if properties.iter().any(|property| !property.is_string()) {
        return Err(());
    }
    let supports_detail = properties
        .iter()
        .any(|property| property.as_str() == Some("detail"));
    let supports_documentation = properties
        .iter()
        .any(|property| property.as_str() == Some("documentation"));
    Ok(CompletionResolveOptions {
        enabled: supports_detail && supports_documentation,
        documentation_format,
    })
}

fn parse_documentation_format(
    item: &Map<String, Value>,
) -> Result<CompletionDocumentationFormat, ()> {
    let Some(formats) = item.get("documentationFormat") else {
        return Ok(CompletionDocumentationFormat::Plaintext);
    };
    let formats = formats
        .as_array()
        .filter(|formats| !formats.is_empty())
        .ok_or(())?;
    if formats.iter().any(|format| !format.is_string()) {
        return Err(());
    }
    formats
        .iter()
        .filter_map(Value::as_str)
        .find_map(|format| match format {
            "plaintext" => Some(CompletionDocumentationFormat::Plaintext),
            "markdown" => Some(CompletionDocumentationFormat::Markdown),
            _ => None,
        })
        .ok_or(())
}

pub(crate) fn completion_resolve_handle(
    snapshot_digest: &[u8; 32],
    uri: &str,
    replacement: Span,
    request_offset: u32,
    ordinal: u32,
    kind: u8,
    identity: &str,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hash_part(&mut hasher, COMPLETION_RESOLVE_PROTOCOL_VERSION.as_bytes());
    hasher.update(snapshot_digest);
    hash_part(&mut hasher, uri.as_bytes());
    hasher.update(&replacement.start().get().to_le_bytes());
    hasher.update(&replacement.end().get().to_le_bytes());
    hasher.update(&request_offset.to_le_bytes());
    hasher.update(&ordinal.to_le_bytes());
    hasher.update(&[kind]);
    hash_part(&mut hasher, identity.as_bytes());
    format!("{HANDLE_PREFIX}{}", hasher.finalize().to_hex())
}

pub(crate) fn completion_resolve_snapshot_digest(snapshot: &RequestSnapshot) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hash_part(&mut hasher, b"ling.lsp.completion-resolve.snapshot/0.1");
    hasher.update(&[lifecycle_tag(snapshot.state())]);
    hash_part(
        &mut hasher,
        snapshot.position_encoding().wire_name().as_bytes(),
    );
    hasher.update(&snapshot.revision().get().to_le_bytes());
    hasher.update(
        &u64::try_from(snapshot.documents().len())
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    for document in snapshot.documents() {
        hash_part(&mut hasher, document.uri().as_bytes());
        hash_part(&mut hasher, document.logical_name().as_bytes());
        hasher.update(&document.revision().get().to_le_bytes());
        hasher.update(&[file_origin_tag(document.origin())]);
        hasher.update(&[
            u8::from(document.is_open()),
            u8::from(document.is_writable()),
            u8::from(document.is_temporary()),
        ]);
        match document.client_version() {
            Some(version) => {
                hasher.update(&[1]);
                hasher.update(&version.to_le_bytes());
            }
            None => {
                hasher.update(&[0]);
            }
        }
        hash_part(&mut hasher, document.bytes());
    }
    hasher.update(
        &u64::try_from(snapshot.inputs().len())
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    for input in snapshot.inputs() {
        hasher.update(&[workspace_input_tag(input.kind())]);
        hasher.update(&input.revision().get().to_le_bytes());
        hash_part(&mut hasher, input.bytes());
    }
    *hasher.finalize().as_bytes()
}

pub(crate) fn completion_resolve_data(handle: &str) -> Value {
    json!({
        "handle": handle,
        "version": COMPLETION_RESOLVE_PROTOCOL_VERSION,
    })
}

fn hash_part(hasher: &mut blake3::Hasher, bytes: &[u8]) {
    hasher.update(&u64::try_from(bytes.len()).unwrap_or(u64::MAX).to_le_bytes());
    hasher.update(bytes);
}

const fn lifecycle_tag(state: LifecycleState) -> u8 {
    match state {
        LifecycleState::Uninitialized => 0,
        LifecycleState::AwaitingInitialized => 1,
        LifecycleState::Ready => 2,
        LifecycleState::ShutdownRequested => 3,
        LifecycleState::Exited => 4,
    }
}

const fn file_origin_tag(origin: FileOrigin) -> u8 {
    match origin {
        FileOrigin::Disk => 0,
        FileOrigin::Overlay => 1,
    }
}

const fn workspace_input_tag(input: WorkspaceInput) -> u8 {
    match input {
        WorkspaceInput::PackageManifest => 0,
        WorkspaceInput::PackageLock => 1,
        WorkspaceInput::Config => 2,
        WorkspaceInput::Profile => 3,
        WorkspaceInput::Target => 4,
    }
}

fn parse_resolve_item(params: &Value) -> Result<&str, CompletionResolveError> {
    let item = params
        .as_object()
        .ok_or(CompletionResolveError::InvalidParams)?;
    if item.contains_key("detail") || item.contains_key("documentation") {
        return Err(CompletionResolveError::InvalidParams);
    }
    if REQUIRED_ITEM_FIELDS
        .iter()
        .any(|field| !item.contains_key(*field))
    {
        return Err(CompletionResolveError::InvalidParams);
    }
    let data = item
        .get("data")
        .and_then(Value::as_object)
        .filter(|data| data.len() == 2)
        .ok_or(CompletionResolveError::InvalidParams)?;
    if data.get("version").and_then(Value::as_str) != Some(COMPLETION_RESOLVE_PROTOCOL_VERSION) {
        return Err(CompletionResolveError::InvalidParams);
    }
    let handle = data
        .get("handle")
        .and_then(Value::as_str)
        .filter(|handle| valid_handle(handle))
        .ok_or(CompletionResolveError::InvalidParams)?;
    Ok(handle)
}

fn valid_handle(handle: &str) -> bool {
    handle.strip_prefix(HANDLE_PREFIX).is_some_and(|digest| {
        digest.len() == HANDLE_DIGEST_HEX_BYTES
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn validate_retained_item(request: &Value, retained: &Value) -> Result<(), CompletionResolveError> {
    let request = request
        .as_object()
        .ok_or(CompletionResolveError::InvalidParams)?;
    let retained = retained
        .as_object()
        .ok_or(CompletionResolveError::MetadataMismatch)?;
    if REQUIRED_ITEM_FIELDS
        .iter()
        .any(|field| request.get(*field) != retained.get(*field))
    {
        return Err(CompletionResolveError::InvalidParams);
    }
    Ok(())
}

fn add_checked_presentation(
    result: &mut Value,
    metadata: &ResolvedCompletionMetadata,
    author_documentation: Option<&str>,
    format: CompletionDocumentationFormat,
) -> Result<(), CompletionResolveError> {
    let detail = render_detail(metadata).ok_or(CompletionResolveError::MetadataMismatch)?;
    if detail.len() > MAX_COMPLETION_DETAIL_BYTES {
        return Err(CompletionResolveError::DetailBound);
    }
    let documentation = render_documentation(metadata, &detail, author_documentation, format)?;
    if documentation.len() > MAX_COMPLETION_DOCUMENTATION_BYTES {
        return Err(CompletionResolveError::DocumentationBound);
    }
    let object = result
        .as_object_mut()
        .ok_or(CompletionResolveError::MetadataMismatch)?;
    object.insert("detail".to_owned(), Value::String(detail));
    object.insert(
        "documentation".to_owned(),
        json!({"kind": format.wire_name(), "value": documentation}),
    );
    Ok(())
}

fn render_detail(metadata: &ResolvedCompletionMetadata) -> Option<String> {
    let mut detail = format!("{}: {}", metadata.name(), metadata.type_display()?);
    if let Some(effects) = metadata.effects().filter(|effects| !effects.is_empty()) {
        detail.push_str(" ! {");
        detail.push_str(&effects.join(", "));
        detail.push('}');
    }
    if let Some(capabilities) = metadata
        .capabilities()
        .filter(|capabilities| !capabilities.is_empty())
    {
        detail.push_str(" requires {");
        detail.push_str(&capabilities.join(", "));
        detail.push('}');
    }
    Some(detail)
}

fn render_documentation(
    metadata: &ResolvedCompletionMetadata,
    detail: &str,
    author_documentation: Option<&str>,
    format: CompletionDocumentationFormat,
) -> Result<String, CompletionResolveError> {
    let mut facts = vec![("种类 / kind", metadata_kind(metadata.kind()).to_owned())];
    if let Some(effects) = metadata.effects().filter(|effects| !effects.is_empty()) {
        facts.push(("效果 / effects", effects.join(", ")));
    }
    if let Some(capabilities) = metadata
        .capabilities()
        .filter(|capabilities| !capabilities.is_empty())
    {
        facts.push(("能力 / capabilities", capabilities.join(", ")));
    }
    match format {
        CompletionDocumentationFormat::Plaintext => {
            let mut output = String::from("已检查的 Ling 符号 / checked Ling symbol\n");
            output.push_str(detail);
            if let Some(documentation) = author_documentation {
                output.push_str("\n文档 / documentation:\n");
                output.push_str(documentation);
            }
            for (label, value) in facts {
                output.push('\n');
                output.push_str(label);
                output.push_str(": ");
                output.push_str(&value);
            }
            Ok(output)
        }
        CompletionDocumentationFormat::Markdown => {
            if detail.contains('`') || facts.iter().any(|(_, value)| value.contains('`')) {
                return Err(CompletionResolveError::UnsafeMarkup);
            }
            let mut output =
                format!("已检查的 Ling 符号 / checked Ling symbol\n\n```ling\n{detail}\n```");
            if let Some(documentation) = author_documentation {
                output.push_str("\n\n文档 / documentation:");
                for line in documentation.split('\n') {
                    output.push_str("\n> ");
                    output.push_str(&escape_markdown(line));
                }
            }
            for (label, value) in facts {
                output.push_str("\n- ");
                output.push_str(label);
                output.push_str(": `");
                output.push_str(&value);
                output.push('`');
            }
            Ok(output)
        }
    }
}

fn attached_author_documentation(
    compiler: &mut CompilerDb,
    snapshot: &RequestSnapshot,
    metadata: &ResolvedCompletionMetadata,
) -> Result<Option<String>, CompletionResolveError> {
    let document = snapshot
        .documents()
        .iter()
        .find(|document| document.logical_name() == metadata.source_name())
        .ok_or(CompletionResolveError::MetadataMismatch)?;
    let file = compiler
        .vfs()
        .file_id(document.logical_name())
        .ok_or(CompletionResolveError::MetadataMismatch)?;
    if file != metadata.span().source() {
        return Err(CompletionResolveError::MetadataMismatch);
    }
    let source = SourceFile::from_bytes(
        file,
        document.logical_name().to_owned(),
        document.bytes().to_vec(),
    )
    .map_err(CompletionResolveError::Source)?;
    let parsed = compiler
        .parse(file)
        .map_err(CompletionResolveError::Compiler)?;
    let format_document =
        build_format_ir(&source, &parsed).map_err(CompletionResolveError::Format)?;
    let declaration_start = enclosing_declaration_start(format_document.root(), metadata.span())
        .ok_or(CompletionResolveError::MetadataMismatch)?;
    let lines = preceding_documentation_lines(format_document.tokens(), declaration_start)?;
    if lines.is_empty() {
        Ok(None)
    } else {
        Ok(Some(lines.join("\n")))
    }
}

fn enclosing_declaration_start(node: &FormatNode, span: Span) -> Option<usize> {
    let node_span = node.span()?;
    if node_span.source() != span.source()
        || node_span.start() > span.start()
        || node_span.end() < span.end()
    {
        return None;
    }
    if let Some(start) = node
        .children()
        .iter()
        .find_map(|child| enclosing_declaration_start(child, span))
    {
        return Some(start);
    }
    is_documentable_declaration(node.kind()).then(|| node.token_range().start)
}

const fn is_documentable_declaration(kind: ling_syntax::NodeKind) -> bool {
    matches!(
        kind,
        ling_syntax::NodeKind::LetDeclaration
            | ling_syntax::NodeKind::TypeDeclaration
            | ling_syntax::NodeKind::TraitDeclaration
            | ling_syntax::NodeKind::TraitMember
            | ling_syntax::NodeKind::ImplDeclaration
            | ling_syntax::NodeKind::ImplMember
            | ling_syntax::NodeKind::FieldDeclaration
            | ling_syntax::NodeKind::VariantCase
    )
}

fn preceding_documentation_lines(
    tokens: &[FormatToken],
    declaration_start: usize,
) -> Result<Vec<String>, CompletionResolveError> {
    let mut cursor = declaration_start.min(tokens.len());
    let mut reversed = Vec::new();
    loop {
        let mut newline_count = 0_u8;
        while let Some(token) = cursor.checked_sub(1).and_then(|index| tokens.get(index)) {
            match token.kind() {
                ling_syntax::TokenKind::Whitespace => cursor -= 1,
                ling_syntax::TokenKind::Newline | ling_syntax::TokenKind::SoftNewline => {
                    newline_count = newline_count.saturating_add(1);
                    cursor -= 1;
                    if newline_count > 1 {
                        break;
                    }
                }
                kind if kind.is_layout() => cursor -= 1,
                _ => break,
            }
        }
        if newline_count != 1 {
            break;
        }
        let Some(comment_index) = cursor.checked_sub(1) else {
            break;
        };
        let comment = &tokens[comment_index];
        if comment.kind() != ling_syntax::TokenKind::DocComment {
            break;
        }
        let body = comment
            .text()
            .strip_prefix("///")
            .ok_or(CompletionResolveError::MetadataMismatch)?;
        reversed.push(body.strip_prefix(' ').unwrap_or(body).to_owned());
        cursor = comment_index;
    }
    reversed.reverse();
    Ok(reversed)
}

fn escape_markdown(input: &str) -> String {
    const PUNCTUATION: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    let mut output = String::with_capacity(input.len());
    for character in input.chars() {
        if character.is_ascii() && PUNCTUATION.contains(character) {
            output.push('\\');
        }
        output.push(character);
    }
    output
}

const fn metadata_kind(kind: ResolvedCompletionSourceKind) -> &'static str {
    match kind {
        ResolvedCompletionSourceKind::Definition => "definition",
        ResolvedCompletionSourceKind::Binding => "binding",
        ResolvedCompletionSourceKind::ImportAlias => "import_alias",
    }
}

fn completion_resolve_error(id: Value, error: &CompletionResolveError) -> HandleOutcome {
    match error {
        CompletionResolveError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "补全项解析参数无效 / invalid completion-item resolve parameters",
        ),
        CompletionResolveError::Unavailable
        | CompletionResolveError::Snapshot(_)
        | CompletionResolveError::CompilerInput(_)
        | CompletionResolveError::Compiler(_)
        | CompletionResolveError::Source(_)
        | CompletionResolveError::Format(_)
        | CompletionResolveError::MetadataMismatch
        | CompletionResolveError::UnsafeMarkup
        | CompletionResolveError::DetailBound
        | CompletionResolveError::DocumentationBound
        | CompletionResolveError::Stale
        | CompletionResolveError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "补全项解析不可用 / completion-item resolve unavailable",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(label: &str) -> CompletionResolveRecord {
        let server = LspServer::new();
        CompletionResolveRecord::new(
            Arc::new(
                server
                    .capture_request_snapshot()
                    .expect("empty snapshot is valid"),
            ),
            "ling://workspace/src/Main.ling".to_owned(),
            json!({"label": label}),
            label.to_owned(),
            None,
            CompletionDocumentationFormat::Plaintext,
        )
    }

    #[test]
    fn handle_validation_is_exact() {
        let valid = format!("{HANDLE_PREFIX}{}", "a".repeat(HANDLE_DIGEST_HEX_BYTES));
        assert!(valid_handle(&valid));
        assert!(!valid_handle(&format!(
            "{HANDLE_PREFIX}{}",
            "A".repeat(HANDLE_DIGEST_HEX_BYTES)
        )));
        assert!(!valid_handle(&format!("{valid}0")));
    }

    #[test]
    fn capability_requires_both_lazy_properties() {
        let capable = json!({
            "completion": {
                "completionItem": {
                    "documentationFormat": ["markdown", "plaintext"],
                    "resolveSupport": {"properties": ["documentation", "detail"]},
                }
            }
        });
        let options =
            parse_completion_resolve_capability(capable.as_object().expect("capability object"))
                .expect("valid capability");
        assert!(options.enabled());
        assert_eq!(
            options.documentation_format(),
            CompletionDocumentationFormat::Markdown
        );

        let partial = json!({
            "completion": {
                "completionItem": {
                    "resolveSupport": {"properties": ["documentation"]},
                }
            }
        });
        assert!(
            !parse_completion_resolve_capability(partial.as_object().expect("capability object"))
                .expect("valid fallback capability")
                .enabled()
        );
    }

    #[test]
    fn handle_store_expires_older_batch_at_the_exact_bound() {
        let mut state = CompletionResolveState::new();
        state.configure(CompletionResolveOptions {
            enabled: true,
            documentation_format: CompletionDocumentationFormat::Plaintext,
        });
        let batch = (0..MAX_COMPLETION_RESOLVE_HANDLES)
            .map(|index| (format!("old-{index:04}"), record("old")))
            .collect();
        state.publish(batch).expect("exact-bound batch");
        assert_eq!(state.records.len(), MAX_COMPLETION_RESOLVE_HANDLES);

        state
            .publish(vec![("new".to_owned(), record("new"))])
            .expect("new batch expires older handles");
        assert_eq!(state.records.len(), 1);
        assert!(state.record("new").is_some());
        assert!(state.record("old-0000").is_none());
    }

    #[test]
    fn handle_store_rejects_bound_and_collision_without_partial_state() {
        let mut state = CompletionResolveState::new();
        let oversized = (0..=MAX_COMPLETION_RESOLVE_HANDLES)
            .map(|index| (format!("handle-{index:04}"), record("same")))
            .collect();
        assert_eq!(
            state.publish(oversized),
            Err(CompletionResolvePublishError::Bound)
        );
        assert!(state.records.is_empty());

        state
            .publish(vec![("same-handle".to_owned(), record("first"))])
            .expect("first record");
        assert_eq!(
            state.publish(vec![("same-handle".to_owned(), record("second"))]),
            Err(CompletionResolvePublishError::Collision)
        );
        assert_eq!(state.records.len(), 1);
        assert_eq!(state.record("same-handle").expect("original").name, "first");
    }
}
