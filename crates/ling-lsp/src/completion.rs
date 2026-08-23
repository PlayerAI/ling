use std::collections::{BTreeMap, BTreeSet};

use ling_db::{
    CheckedCompletionCandidate, CheckedCompletionCatalog, CheckedCompletionKind,
    MAX_CHECKED_COMPLETION_CANDIDATES, QueryError,
};
use ling_source::{SourceError, SourceFile, Span, VfsError};
use ling_syntax::{CstNode, NodeKind, TokenKind};
use serde_json::{Value, json};

use super::location_projection::{LocationProjectionError, range_value};
use super::publication::{compiler_for_snapshot, compiler_for_snapshot_with_overrides};
use super::{
    HandleOutcome, INVALID_PARAMS, LifecycleState, LspServer, MAX_FRAME_BYTES, RequestDocument,
    RequestSnapshot, RequestSnapshotError, error_or_none, parse_text_document_position,
    success_response,
};

/// Current checked completion Preview marker.
pub const COMPLETION_PROTOCOL_VERSION: &str = "ling.lsp.completion/0.1";
/// Maximum number of completion items emitted by one request.
pub const MAX_COMPLETION_ITEMS: usize = 256;

const REQUEST_FAILED: i32 = -32_803;

const KEYWORDS: &[&str] = &[
    "and", "as", "else", "false", "if", "impl", "import", "let", "match", "module", "mutable",
    "of", "rec", "requires", "then", "trait", "true", "type", "when", "with",
];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CompletionContext {
    Expression,
    Member,
    Type,
    Pattern,
    Module,
    Keyword,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CompletionSite {
    context: CompletionContext,
    replacement: Span,
    prefix: String,
    qualifier: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RankedCandidate {
    label: String,
    kind: CheckedCompletionKind,
    rank: Rank,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Rank {
    prefix_class: u8,
    scope_class: u8,
    scope_distance: u32,
    explicit_import_class: u8,
    label: String,
    kind: CheckedCompletionKind,
    identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CompletionError {
    InvalidParams,
    Snapshot(RequestSnapshotError),
    CompilerInput(VfsError),
    Compiler(QueryError),
    Source(SourceError),
    Projection(LocationProjectionError),
    InvalidSource,
    CandidateBound,
    Stale,
    ResponseTooLarge,
}

impl LspServer {
    pub(crate) fn completion(&self, is_request: bool, id: Value, params: Value) -> HandleOutcome {
        if !is_request {
            return HandleOutcome::NoResponse;
        }
        if self.state != LifecycleState::Ready {
            return self.state_error(id);
        }
        match self.completion_result(&params) {
            Ok(result) => {
                let response = success_response(id.clone(), result);
                if response.len() > MAX_FRAME_BYTES {
                    return completion_error(id, &CompletionError::ResponseTooLarge);
                }
                HandleOutcome::Response(response)
            }
            Err(error) => completion_error(id, &error),
        }
    }

    fn completion_result(&self, params: &Value) -> Result<Value, CompletionError> {
        validate_completion_context(params)?;
        let (uri, position) =
            parse_text_document_position(params).ok_or(CompletionError::InvalidParams)?;
        let snapshot = self
            .capture_request_snapshot()
            .map_err(CompletionError::Snapshot)?;
        let document = snapshot
            .document(uri)
            .ok_or(CompletionError::InvalidParams)?;
        let temporary = document.is_temporary().then_some(document.uri());
        let mut compiler =
            compiler_for_snapshot(&snapshot, temporary).map_err(CompletionError::CompilerInput)?;
        let file = compiler
            .vfs()
            .file_id(document.logical_name())
            .ok_or(CompletionError::InvalidParams)?;
        let source = SourceFile::from_bytes(
            file,
            document.logical_name().to_owned(),
            document.bytes().to_vec(),
        )
        .map_err(CompletionError::Source)?;
        let offset = source
            .original_offset(position, snapshot.position_encoding())
            .map_err(|_| CompletionError::InvalidParams)?;
        let lexical = compiler
            .token_source_index(file)
            .map_err(CompletionError::Compiler)?;
        let parsed = compiler.parse(file).map_err(CompletionError::Compiler)?;
        if !lexical.is_valid() || !parsed.is_valid() {
            return Err(CompletionError::InvalidSource);
        }
        let site = completion_site(&lexical, parsed.tree().root(), offset.get(), document)
            .ok_or(CompletionError::InvalidParams)?;
        let catalog = compiler
            .checked_completion_catalog(file)
            .map_err(CompletionError::Compiler)?;
        if catalog.candidates().len() > MAX_CHECKED_COMPLETION_CANDIDATES {
            return Err(CompletionError::CandidateBound);
        }

        let current_module = catalog
            .candidates()
            .iter()
            .find(|candidate| {
                candidate.kind() == CheckedCompletionKind::Module
                    && candidate.source_name() == Some(document.logical_name())
            })
            .and_then(CheckedCompletionCandidate::module_id);
        let module_target = member_module_target(&site, &catalog, current_module);
        let mut ranked = candidate_pool(&site, &catalog, current_module, module_target);
        ranked.sort_by(|left, right| left.rank.cmp(&right.rank));

        let start = usize::try_from(site.replacement.start().get())
            .map_err(|_| CompletionError::InvalidSource)?;
        let end = usize::try_from(site.replacement.end().get())
            .map_err(|_| CompletionError::InvalidSource)?;
        if start > end || end > document.bytes().len() {
            return Err(CompletionError::InvalidSource);
        }
        let mut valid = Vec::new();
        let mut labels = BTreeSet::new();
        for candidate in ranked.into_iter().take(MAX_CHECKED_COMPLETION_CANDIDATES) {
            if !labels.insert(candidate.label.clone()) {
                continue;
            }
            if replacement_checks(
                &snapshot,
                temporary,
                document,
                file,
                start,
                end,
                &candidate.label,
            )? {
                valid.push(candidate);
            }
        }

        let is_incomplete = valid.len() > MAX_COMPLETION_ITEMS;
        valid.truncate(MAX_COMPLETION_ITEMS);
        let range = range_value(
            document.logical_name(),
            site.replacement,
            &snapshot,
            &compiler,
        )
        .map_err(CompletionError::Projection)?;
        let items = valid
            .into_iter()
            .enumerate()
            .map(|(ordinal, candidate)| {
                json!({
                    "filterText": candidate.label,
                    "insertTextFormat": 1,
                    "kind": completion_item_kind(candidate.kind),
                    "label": candidate.label,
                    "sortText": format!("{ordinal:06}"),
                    "textEdit": {
                        "newText": candidate.label,
                        "range": range,
                    },
                })
            })
            .collect::<Vec<_>>();
        self.finish_completion(
            &snapshot,
            json!({"isIncomplete": is_incomplete, "items": items}),
        )
    }

    fn finish_completion(
        &self,
        snapshot: &RequestSnapshot,
        result: Value,
    ) -> Result<Value, CompletionError> {
        if self
            .capture_request_snapshot()
            .map_err(CompletionError::Snapshot)?
            != *snapshot
        {
            return Err(CompletionError::Stale);
        }
        Ok(result)
    }
}

fn validate_completion_context(params: &Value) -> Result<(), CompletionError> {
    let object = params.as_object().ok_or(CompletionError::InvalidParams)?;
    let Some(context) = object.get("context") else {
        return Ok(());
    };
    let context = context.as_object().ok_or(CompletionError::InvalidParams)?;
    let trigger = context
        .get("triggerKind")
        .and_then(Value::as_u64)
        .filter(|value| (1..=3).contains(value))
        .ok_or(CompletionError::InvalidParams)?;
    match (trigger, context.get("triggerCharacter")) {
        (2, Some(Value::String(value))) if value == "." => Ok(()),
        (2, _) => Err(CompletionError::InvalidParams),
        (_, None) => Ok(()),
        _ => Err(CompletionError::InvalidParams),
    }
}

fn completion_site(
    lexical: &ling_db::TokenSourceIndex,
    root: &CstNode,
    offset: u32,
    document: &RequestDocument,
) -> Option<CompletionSite> {
    let (token_index, token) = lexical.tokens().iter().enumerate().find(|(_, token)| {
        token.span().start().get() <= offset
            && offset <= token.span().end().get()
            && (token.kind() == TokenKind::Identifier || is_keyword(token.kind()))
    })?;
    let mut ancestors = Vec::new();
    collect_ancestors(root, token_index, &mut ancestors);
    let previous = previous_significant(lexical, token_index);
    let is_member = previous.is_some_and(|(_, token)| token.kind() == TokenKind::Dot)
        && ancestors.contains(&NodeKind::ProjectionExpression);
    let context = if is_member {
        CompletionContext::Member
    } else if ancestors.contains(&NodeKind::TypeExpression) {
        CompletionContext::Type
    } else if ancestors.contains(&NodeKind::Pattern) {
        CompletionContext::Pattern
    } else if ancestors.contains(&NodeKind::ImportDeclaration) {
        CompletionContext::Module
    } else if is_keyword(token.kind()) {
        CompletionContext::Keyword
    } else if ancestors.contains(&NodeKind::NameExpression) {
        CompletionContext::Expression
    } else {
        return None;
    };

    let replacement = if context == CompletionContext::Module {
        qualified_name_span(root, token_index, lexical)?
    } else {
        token.span()
    };
    let start = usize::try_from(replacement.start().get()).ok()?;
    let cursor = usize::try_from(offset).ok()?;
    let end = usize::try_from(replacement.end().get()).ok()?;
    if start > cursor || cursor > end || end > document.bytes().len() {
        return None;
    }
    let prefix = std::str::from_utf8(&document.bytes()[start..cursor])
        .ok()?
        .to_owned();
    let qualifier = if context == CompletionContext::Member {
        let (dot_index, _) = previous?;
        previous_significant(lexical, dot_index)
            .filter(|(_, token)| token.kind() == TokenKind::Identifier)
            .map(|(_, token)| token.text().to_owned())
    } else {
        None
    };
    Some(CompletionSite {
        context,
        replacement,
        prefix,
        qualifier,
    })
}

fn collect_ancestors(node: &CstNode, token_index: usize, ancestors: &mut Vec<NodeKind>) -> bool {
    let range = node.token_range();
    if !(range.start <= token_index && token_index < range.end) {
        return false;
    }
    ancestors.push(node.kind());
    for child in node.children() {
        if collect_ancestors(child, token_index, ancestors) {
            break;
        }
    }
    true
}

fn qualified_name_span(
    node: &CstNode,
    token_index: usize,
    lexical: &ling_db::TokenSourceIndex,
) -> Option<Span> {
    let range = node.token_range();
    if !(range.start <= token_index && token_index < range.end) {
        return None;
    }
    for child in node.children() {
        if child.kind() == NodeKind::QualifiedName {
            let child_range = child.token_range();
            if child_range.start <= token_index && token_index < child_range.end {
                let identifiers = lexical.tokens()[child_range]
                    .iter()
                    .filter(|token| token.kind() == TokenKind::Identifier)
                    .collect::<Vec<_>>();
                let first = identifiers.first()?.span();
                let last = identifiers.last()?.span();
                return Span::new(first.source(), first.start(), last.end()).ok();
            }
        }
        if let Some(span) = qualified_name_span(child, token_index, lexical) {
            return Some(span);
        }
    }
    None
}

fn previous_significant(
    lexical: &ling_db::TokenSourceIndex,
    index: usize,
) -> Option<(usize, &ling_db::TokenSource)> {
    lexical.tokens()[..index]
        .iter()
        .enumerate()
        .rev()
        .find(|(_, token)| !token.kind().is_trivia() && !token.kind().is_layout())
}

fn member_module_target(
    site: &CompletionSite,
    catalog: &CheckedCompletionCatalog,
    current_module: Option<u32>,
) -> Option<u32> {
    let qualifier = site.qualifier.as_deref()?;
    catalog
        .candidates()
        .iter()
        .find(|candidate| {
            candidate.kind() == CheckedCompletionKind::ImportAlias
                && candidate.name() == qualifier
                && candidate.module_id() == current_module
        })
        .and_then(CheckedCompletionCandidate::target_module_id)
}

fn candidate_pool(
    site: &CompletionSite,
    catalog: &CheckedCompletionCatalog,
    current_module: Option<u32>,
    member_target: Option<u32>,
) -> Vec<RankedCandidate> {
    if site.context == CompletionContext::Keyword {
        return keyword_pool(site);
    }

    let mut candidates = catalog
        .candidates()
        .iter()
        .filter(|candidate| candidate.name().starts_with(&site.prefix))
        .filter(|candidate| candidate_in_context(site, candidate, member_target))
        .map(|candidate| RankedCandidate {
            label: candidate.name().to_owned(),
            kind: candidate.kind(),
            rank: rank_candidate(site, candidate, current_module),
        })
        .collect::<Vec<_>>();
    if site.context == CompletionContext::Pattern && "_".starts_with(&site.prefix) {
        candidates.push(RankedCandidate {
            label: "_".to_owned(),
            kind: CheckedCompletionKind::Keyword,
            rank: Rank {
                prefix_class: u8::from(site.prefix != "_"),
                scope_class: 9,
                scope_distance: 0,
                explicit_import_class: 1,
                label: "_".to_owned(),
                kind: CheckedCompletionKind::Keyword,
                identity: "keyword:_".to_owned(),
            },
        });
    }
    candidates
}

fn keyword_pool(site: &CompletionSite) -> Vec<RankedCandidate> {
    KEYWORDS
        .iter()
        .filter(|keyword| keyword.starts_with(&site.prefix))
        .map(|keyword| RankedCandidate {
            label: (*keyword).to_owned(),
            kind: CheckedCompletionKind::Keyword,
            rank: Rank {
                prefix_class: u8::from(*keyword != site.prefix),
                scope_class: 9,
                scope_distance: 0,
                explicit_import_class: 1,
                label: (*keyword).to_owned(),
                kind: CheckedCompletionKind::Keyword,
                identity: format!("keyword:{keyword}"),
            },
        })
        .collect()
}

fn candidate_in_context(
    site: &CompletionSite,
    candidate: &CheckedCompletionCandidate,
    member_target: Option<u32>,
) -> bool {
    match site.context {
        CompletionContext::Expression => match candidate.kind() {
            CheckedCompletionKind::Value
            | CheckedCompletionKind::Constructor
            | CheckedCompletionKind::Binding
            | CheckedCompletionKind::ImportAlias => true,
            CheckedCompletionKind::Builtin => candidate.qualifier().is_none(),
            CheckedCompletionKind::Type
            | CheckedCompletionKind::Module
            | CheckedCompletionKind::Field
            | CheckedCompletionKind::Keyword => false,
        },
        CompletionContext::Member => {
            candidate.kind() == CheckedCompletionKind::Field
                || candidate.qualifier() == site.qualifier.as_deref()
                || member_target.is_some_and(|module| candidate.module_id() == Some(module))
        }
        CompletionContext::Type => candidate.kind() == CheckedCompletionKind::Type,
        CompletionContext::Pattern => candidate.kind() == CheckedCompletionKind::Constructor,
        CompletionContext::Module => candidate.kind() == CheckedCompletionKind::Module,
        CompletionContext::Keyword => false,
    }
}

fn rank_candidate(
    site: &CompletionSite,
    candidate: &CheckedCompletionCandidate,
    current_module: Option<u32>,
) -> Rank {
    let request_start = site.replacement.start().get();
    let (scope_class, scope_distance) = match candidate.kind() {
        CheckedCompletionKind::Binding if candidate.module_id() == current_module => {
            let candidate_start = candidate.span().map_or(u32::MAX, |span| span.start().get());
            if candidate_start <= request_start {
                (0, request_start - candidate_start)
            } else {
                (1, candidate_start - request_start)
            }
        }
        CheckedCompletionKind::Value
        | CheckedCompletionKind::Type
        | CheckedCompletionKind::Constructor
            if candidate.module_id() == current_module =>
        {
            (2, 0)
        }
        CheckedCompletionKind::ImportAlias => (3, 0),
        CheckedCompletionKind::Value
        | CheckedCompletionKind::Type
        | CheckedCompletionKind::Constructor => (4, 0),
        CheckedCompletionKind::Builtin => (5, 0),
        CheckedCompletionKind::Module => (6, 0),
        CheckedCompletionKind::Field => (7, 0),
        CheckedCompletionKind::Binding => (8, 0),
        CheckedCompletionKind::Keyword => (9, 0),
    };
    Rank {
        prefix_class: u8::from(candidate.name() != site.prefix),
        scope_class,
        scope_distance,
        explicit_import_class: u8::from(candidate.kind() != CheckedCompletionKind::ImportAlias),
        label: candidate.name().to_owned(),
        kind: candidate.kind(),
        identity: candidate.identity().to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn replacement_checks(
    snapshot: &RequestSnapshot,
    temporary: Option<&str>,
    document: &RequestDocument,
    original_file: ling_source::SourceId,
    start: usize,
    end: usize,
    replacement: &str,
) -> Result<bool, CompletionError> {
    let mut bytes = document.bytes().to_vec();
    bytes.splice(start..end, replacement.bytes());
    let mut overrides = BTreeMap::new();
    overrides.insert(document.logical_name().to_owned(), bytes);
    let mut compiler = compiler_for_snapshot_with_overrides(snapshot, temporary, &overrides)
        .map_err(CompletionError::CompilerInput)?;
    let file = compiler
        .vfs()
        .file_id(document.logical_name())
        .ok_or(CompletionError::InvalidParams)?;
    debug_assert_eq!(file, original_file);
    match compiler.checked_completion_catalog(file) {
        Ok(_) => Ok(true),
        Err(QueryError::UnknownFile { .. }) => Err(CompletionError::InvalidParams),
        Err(_) => Ok(false),
    }
}

const fn completion_item_kind(kind: CheckedCompletionKind) -> u8 {
    match kind {
        CheckedCompletionKind::Builtin => 3,
        CheckedCompletionKind::Constructor => 4,
        CheckedCompletionKind::Field => 5,
        CheckedCompletionKind::Binding => 6,
        CheckedCompletionKind::Type => 7,
        CheckedCompletionKind::ImportAlias | CheckedCompletionKind::Module => 9,
        CheckedCompletionKind::Value => 12,
        CheckedCompletionKind::Keyword => 14,
    }
}

const fn is_keyword(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Let
            | TokenKind::Mutable
            | TokenKind::Rec
            | TokenKind::And
            | TokenKind::Type
            | TokenKind::Of
            | TokenKind::Match
            | TokenKind::With
            | TokenKind::When
            | TokenKind::If
            | TokenKind::Then
            | TokenKind::Else
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Module
            | TokenKind::Import
            | TokenKind::As
            | TokenKind::Requires
            | TokenKind::Trait
            | TokenKind::Impl
    )
}

fn completion_error(id: Value, error: &CompletionError) -> HandleOutcome {
    match error {
        CompletionError::InvalidParams => error_or_none(
            true,
            id,
            INVALID_PARAMS,
            "补全参数无效 / invalid completion parameters",
        ),
        CompletionError::Snapshot(_)
        | CompletionError::CompilerInput(_)
        | CompletionError::Compiler(_)
        | CompletionError::Source(_)
        | CompletionError::Projection(_)
        | CompletionError::InvalidSource
        | CompletionError::CandidateBound
        | CompletionError::Stale
        | CompletionError::ResponseTooLarge => error_or_none(
            true,
            id,
            REQUEST_FAILED,
            "补全不可用 / completion unavailable",
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use ling_source::{ByteOffset, SourceId};

    use super::*;

    #[test]
    fn rank_is_a_total_deterministic_order() {
        let first = Rank {
            prefix_class: 0,
            scope_class: 0,
            scope_distance: 1,
            explicit_import_class: 1,
            label: "alpha".to_owned(),
            kind: CheckedCompletionKind::Binding,
            identity: "binding:0:1".to_owned(),
        };
        let mut second = first.clone();
        second.identity = "binding:0:2".to_owned();
        assert_eq!(first.cmp(&second), Ordering::Less);
    }

    #[test]
    fn keyword_set_is_sorted_and_unique() {
        assert!(KEYWORDS.windows(2).all(|pair| pair[0] < pair[1]));
        let span = Span::new(SourceId::new(0), ByteOffset::new(0), ByteOffset::new(3))
            .expect("valid span");
        let site = CompletionSite {
            context: CompletionContext::Keyword,
            replacement: span,
            prefix: "le".to_owned(),
            qualifier: None,
        };
        let candidates = keyword_pool(&site);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].label, "let");
    }
}
