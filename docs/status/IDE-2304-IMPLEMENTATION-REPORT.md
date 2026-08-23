# IDE-2304 implementation report

> Status: Implemented; status-ledger binding pending the implementation commit
> Task: `IDE-2304`
> Authority: Accepted `RFC-0039`, `RFC-0038`, `RFC-0004`, `RFC-0005`,
> `RFC-0023`, `RFC-0029`, `RFC-0030`, `DEC-0002`, `DEC-0012`, `DEC-0019`,
> `DEC-0029`, `DEC-0071`, `DEC-0075`, `DEC-0076`, and `DEC-0078`

## Scope

This milestone adds Preview `textDocument/references`. It selects exact
source-backed declarations or resolved expression references from one complete
checked snapshot and returns canonical standard `Location` arrays with exact
optional declaration inclusion.

## Normative clauses covered

- RFC-0039 §1: capability validation, static provider discovery, exact Preview
  marker, complete relation vocabulary, and explicit emitted subset.
- RFC-0039 §2: Ready-state request-only dispatch, exact URI/u32 position,
  required `includeDeclaration`, immutable snapshots, temporary isolation,
  freshness, and notification silence.
- RFC-0039 §3: complete checked construction, atomic resolver/span/target
  joins, exact member declarations, read/write/call classification, explicit
  type/implementation non-inference, deterministic ordering, and bounds.
- RFC-0039 §4: declaration/reference selection, canonical `Location[]`, exact
  URI reuse, negotiated original-byte range projection, and empty results.
- RFC-0039 §5: fixed bilingual errors, response bound, atomic failure, and
  Preview migration behavior.

## Implementation and fixtures

- `ling-db::ResolvedReferenceSpanIndex` now attaches mutually exclusive exact
  `read`, `write`, or `call` relations to resolver-owned expression spans.
- `ling-db::ReferenceSearchIndex` atomically joins checked resolver targets,
  exact reference spans, source-backed declaration identifiers, and canonical
  target groups, with 16,384-entry/result bounds.
- `ling-lsp::references` validates standard requests, captures one immutable
  snapshot, consumes the checked compiler query, maps logical sources to exact
  registered URIs, projects negotiated ranges, and rechecks freshness.
- Shared `location_projection` keeps navigation and references URI/range
  behavior identical without duplicating source-map policy.
- Integration tests cover exact discovery, malformed capabilities,
  declaration/reference selection, inclusion control, source-less builtins,
  workspace/dependency targets, UTF-8/16/32 plus BOM/CRLF/Unicode, empty
  selection, invalid params, notification silence, and checked failure.
- Exact diagnostic transcripts record additive initialize discovery without
  changing diagnostic bodies.

## Tests and evidence

- `cargo test -p ling-db reference_search_index --locked --offline` passes.
- `cargo test -p ling-lsp --test references --locked --offline` passes.
- `cargo test -p ling-lsp --locked --offline` passes.
- Focused strict Clippy for `ling-db` and `ling-lsp` passes.
- Remaining workspace, governance, and release gates are recorded only after
  execution against the implementation commit.

## Compatibility, determinism, and Unicode impact

- Adds public Preview `ling.lsp.references/0.1` with no predecessor and the
  standard static `referencesProvider` boolean.
- Exact transcript fixtures provide additive discovery migration evidence.
  Result shape remains standard `Location[]`; no relation or identity metadata
  enters the wire.
- Logical-source/range/target ordering, immutable snapshots, unique URI lookup,
  complete joins, and fixed bounds exclude hash-map, allocation, registration,
  host-path, clock, environment, and debug details.
- No diagnostic allocation or Semantic ID/schema change occurs. Unicode stays
  at 17.0.0 and all ranges retain original UTF-8 byte provenance.

## Intentionally deferred

Resolver-owned type and implementation occurrences, generated/virtual
documents, custom relation results, persistent indexes, progress, partial
results, asynchronous cancellation, Workspace Edits, Semantic Transactions,
and Stable lifecycle require separate Accepted authority.
