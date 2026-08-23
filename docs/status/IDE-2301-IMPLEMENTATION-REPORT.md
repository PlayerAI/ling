# IDE-2301 implementation report

> Status: implementation complete; status binding pending
> Task: `IDE-2301`
> Authority: Accepted `RFC-0036`, `RFC-0004`, `RFC-0023`, `RFC-0029`,
> `RFC-0030`, `DEC-0002`, `DEC-0012`, `DEC-0019`, `DEC-0029`, `DEC-0071`,
> and `DEC-0073`

## Scope

This milestone adds the bounded Preview `textDocument/documentSymbol`
provider. It composes a compiler-owned resolved structural outline with the
existing immutable LSP snapshot and source-position boundaries, returning a
hierarchical tree only to clients that declare support and the standard flat
fallback otherwise.

## Normative clauses covered

- RFC-0036 §1: exact capability validation, immutable hierarchical/flat mode,
  static provider advertisement, and versioned Experimental discovery.
- RFC-0036 §2: Ready-state request-only dispatch, exact current URI lookup,
  path-free immutable snapshots, isolated temporary sources, project inputs,
  freshness validation, and notification silence.
- RFC-0036 §§3–4: module-rooted resolved outlines, all Seed structural kinds,
  original spelling/spans, nested members, implementation display names,
  containment validation, 4096-node bound, and deterministic source order.
- RFC-0036 §§5–6: exact hierarchical fields, flat pre-order/container fields,
  request URI preservation, and negotiated UTF-8/16/32 range projection.
- RFC-0036 §7: fixed InvalidParams/RequestFailed behavior, atomic overflow and
  compiler failure, response-size enforcement, and Preview migration policy.

## Implementation and fixtures

- `ling-db::resolved_outline` consumes resolver-validated HIR and produces one
  immutable, bounded module tree with original UTF-8 full/selection spans. It
  contains no LSP values or presentation identity.
- `ling-lsp::document_symbols` validates params and lifecycle, captures one
  request snapshot, resolves exact visible bytes, projects through the
  negotiated position encoding, rechecks freshness, and emits no partial
  success on failure.
- Snapshot compiler construction is shared with diagnostics while preserving
  different inclusion policies: diagnostics compile non-temporary workspace
  documents; a temporary symbol request compiles only that exact temporary
  source plus project inputs.
- Integration tests cover every reachable Seed structural kind, hierarchy, flat
  pre-order, containers, original ranges, BOM/CRLF, Chinese, emoji, combining
  marks, UTF-8/16/32, repeated output, isolation, malformed capabilities and
  params, resolution recovery, notifications, unknown URIs, and overflow.
- Existing exact diagnostic transcripts now record the additive initialize
  capability and Preview marker; their diagnostic bodies and behavior remain
  unchanged.

## Tests and evidence

- `cargo test -p ling-lsp --test document_symbols --locked --offline` passes.
- `cargo test -p ling-db --locked --offline` passes.
- `cargo test -p ling-lsp --locked --offline` passes, including exact
  diagnostic transcript replay.
- Focused strict Clippy for `ling-db` and `ling-lsp` passes.
- The complete repository gate result and implementation commit are bound in
  the final status update after the implementation commit exists.

## Compatibility, determinism, and Unicode impact

- Adds public Preview `ling.lsp.document-symbol/0.1` with no predecessor.
  Existing clients default to the standard flat response; hierarchical output
  requires explicit capability negotiation.
- Adds `documentSymbolProvider` and `lingDocumentSymbols` to initialize output;
  exact transcript fixtures provide migration evidence for that additive wire
  change.
- Stable byte-span/source/kind/name ordering, bounded traversal, immutable
  snapshots, and repeated-response tests prevent host iteration, path,
  allocation, clock, environment, and cache details from entering results.
- No diagnostic allocation or Semantic ID/schema change occurs. Unicode stays
  at 17.0.0, and original UTF-8 byte spans are projected without clamping.

## Intentionally deferred

Local pattern bindings, generated-only structures, inferred details,
documentation, tags, dynamic registration, observable asynchronous
cancellation, progress, partial results, background scheduling, caching
promises, fixes, Workspace Edits, Semantic Transactions, and Stable lifecycle
remain future work requiring separate Accepted authority.
