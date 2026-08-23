# IDE-2302 implementation report

> Status: Implemented; status-ledger binding pending the implementation commit
> Task: `IDE-2302`
> Authority: Accepted `RFC-0037`, `RFC-0004`, `RFC-0005`, `RFC-0023`,
> `RFC-0029`, `RFC-0030`, `RFC-0036`, `DEC-0002`, `DEC-0012`, `DEC-0019`,
> `DEC-0027`, `DEC-0029`, `DEC-0060`, `DEC-0071`, `DEC-0074`, `DEC-0075`, `DEC-0078`,
> and `DEC-0080`

## Scope

This milestone adds Preview `textDocument/hover` over complete checked compiler
facts. It composes a compiler-owned checked hover index with the accepted LSP
snapshot and position boundaries, returning deterministic bilingual plaintext
or Markdown for one exact identifier target.

## Normative clauses covered

- RFC-0037 §1: exact capability validation, first-supported-format
  negotiation, static provider advertisement, and versioned discovery.
- RFC-0037 §2: Ready-state request-only dispatch, exact current URI and u32
  position validation, path-free immutable snapshots, temporary isolation,
  freshness revalidation, and notification silence.
- RFC-0037 §§3–4: complete checked-pipeline indexing, declaration/binding/
  parameter/reference joins, target taxonomy, canonical type variables,
  checked Effect/Capability/Trait facts, deterministic bounds, and smallest
  exact source-span selection.
- RFC-0037 §5: fixed bilingual fact order, negotiated plaintext/Markdown,
  exact range projection, markup safety, and content/response bounds.
- RFC-0037 §6: fixed InvalidParams/RequestFailed behavior, atomic compiler and
  join failures, no partial output, and Preview migration policy.

## Implementation and fixtures

- `ling-db::checked_hover_index` consumes one complete checked workspace and
  builds at most 16,384 immutable compiler entries. Resolver identity remains
  internal; repeated variables are alpha-canonicalized for public display.
- `ling-lsp::hover` validates lifecycle and request shapes, captures the exact
  visible snapshot, converts negotiated positions to original bytes, renders
  only accepted facts, rechecks freshness, and enforces 65,536-byte content
  plus transport bounds.
- Unit and integration tests cover declarations, locals, parameters,
  references, builtins, canonical polymorphic types, Effects, Capabilities,
  a concrete Trait selection, plaintext/Markdown, UTF-8/16/32, Chinese, emoji,
  combining marks, BOM/CRLF, temporary isolation, repeatability, null,
  malformed requests, compiler failure/recovery, notifications, and overflow.
- Exact diagnostic transcripts record the additive initialize capability and
  protocol marker without changing their diagnostic bodies.

## Tests and evidence

- `cargo test -p ling-lsp --test hover --locked --offline` passes.
- Remaining focused, workspace, governance, and release gates are recorded
  only after they are executed against the implementation commit.

## Compatibility, determinism, and Unicode impact

- Adds public Preview `ling.lsp.hover/0.1` with no predecessor. Clients select
  plaintext by default or the first supported advertised content format.
- Adds `hoverProvider` and `lingHover` to successful initialize responses;
  exact transcript fixtures provide migration evidence.
- Canonical sorting, bounded joins, immutable snapshots, exact ranges, and
  repeated-response tests prevent allocation, map iteration, host paths,
  clock, environment, and debug details from becoming observable.
- No diagnostic allocation or Semantic ID/schema change occurs. Unicode stays
  at 17.0.0 and all source ranges retain original UTF-8 byte provenance.

## Intentionally deferred

Documentation, arbitrary expression hover, imports/modules, profile and
resource/borrow facts, unresolved recovery, dynamic registration, progress,
partial results, asynchronous cancellation, caching promises, fixes,
Workspace Edits, Semantic Transactions, and Stable lifecycle require separate
Accepted authority.
