# IDE-2303 implementation report

> Status: Implemented; status-ledger binding pending the implementation commit
> Task: `IDE-2303`
> Authority: Accepted `RFC-0038`, `RFC-0004`, `RFC-0005`, `RFC-0023`,
> `RFC-0029`, `RFC-0030`, `RFC-0037`, `DEC-0002`, `DEC-0012`, `DEC-0019`,
> `DEC-0029`, `DEC-0071`, and `DEC-0075`

## Scope

This milestone adds Preview resolver-backed `textDocument/definition`,
`textDocument/declaration`, and `textDocument/typeDefinition`. It returns at
most one exact tracked source location and uses `null` for source-less or
unsupported targets rather than inventing virtual documents or paths.

## Normative clauses covered

- RFC-0038 §1: known capability validation, three static standard providers,
  and exact versioned discovery.
- RFC-0038 §2: Ready-state request-only dispatch, exact URI/u32 position,
  immutable snapshots, temporary isolation, freshness, and notification
  silence.
- RFC-0038 §3: bounded resolver-reference joins, exact HIR member-name spans,
  deterministic selection, complete-join rejection, and no public identities.
- RFC-0038 §4: identical Seed definition/declaration semantics, exact local and
  user locations, source-less null, and tracked workspace/dependency URI reuse.
- RFC-0038 §5: complete checked type-definition analysis, direct nominal
  record/variant targets, function-result peeling, and explicit non-nominal
  null behavior.
- RFC-0038 §6: exact `Location` or `null`, negotiated range projection,
  response bounds, fixed errors, atomic failure, and Preview migration policy.

## Implementation and fixtures

- `ling-db::definition_projection` centralizes exact public member names and
  identifier spans so hover and navigation cannot expose resolver
  implementation ordinals or whole declaration-body spans.
- `ling-db::NavigationIndex` builds deterministic resolved or checked entries
  over exact reference spans, user/binding targets, and optional direct
  nominal type definitions, bounded at 16,384 entries.
- `ling-lsp::navigation` validates standard requests, captures one exact
  snapshot, chooses the resolved or checked compiler query, maps canonical
  logical sources back to unique tracked URIs, projects exact target ranges,
  and rechecks freshness before success.
- Integration tests cover exact provider discovery and malformed capability
  rejection; definition/declaration parity; values, parameters, locals,
  constructors, function-result types, Trait members; same-file and read-only
  dependency targets; insertion-order invariance; UTF-8/16/32 with BOM/CRLF,
  emoji/combining/Chinese prefixes; source-less null; invalid params;
  notification silence; and definition behavior under checked failure.
- Exact diagnostic transcripts record the additive initialize providers and
  marker without changing diagnostic bodies.

## Tests and evidence

- `cargo test -p ling-db navigation_index --locked --offline` passes.
- `cargo test -p ling-lsp --test navigation --locked --offline` passes.
- Remaining workspace, governance, and release gates are recorded only after
  execution against the implementation commit.

## Compatibility, determinism, and Unicode impact

- Adds public Preview `ling.lsp.navigation/0.1` with no predecessor and the
  three standard static provider booleans.
- Exact transcript fixtures provide migration evidence for additive initialize
  discovery. Result shape remains the standard single `Location` or `null`.
- Canonical source/reference ordering, bounded indexes, immutable snapshots,
  unique logical-source lookup, and repeated/insertion-order tests exclude map
  order, allocation, host paths, clock, environment, and debug details.
- No diagnostic allocation or Semantic ID/schema change occurs. Unicode stays
  at 17.0.0 and all locations retain original UTF-8 byte provenance.

## Intentionally deferred

Generated/primitive virtual documents, alias hops, multiple targets,
`LocationLink`, declaration-source selection, nested composite-type search,
implementation witnesses, dynamic registration, progress, partial results,
asynchronous cancellation, caching promises, Workspace Edits, Semantic
Transactions, and Stable lifecycle require separate Accepted authority.
