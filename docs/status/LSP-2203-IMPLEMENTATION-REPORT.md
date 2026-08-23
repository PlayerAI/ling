# LSP-2203 implementation report

> Status: Done / completed
> Task: `LSP-2203`
> Authority: Accepted `RFC-0033`, `RFC-0032`, `RFC-0031`, `RFC-0030`,
> `RFC-0029`, `RFC-0023`, `RFC-0004`, `DEC-0019`, `DEC-0034`, `DEC-0071`, and
> `DEC-0072`

## Scope

This milestone implements capability-gated LSP 3.17 document and workspace
pull diagnostics. It reuses the current immutable RFC-0032 compiler analysis
and exact `ling.lsp.diagnostic/0.2` values, adds stateless full/unchanged result
identity, and leaves push scheduling and publication state untouched.

## Normative clauses covered

- RFC-0033 §§1–2: exact conditional `diagnosticProvider`, negotiated method
  availability, identifiers, current-document validation, unique bounded
  workspace previous results, and ignored forward-compatible fields/tokens.
- RFC-0033 §§3–4: one complete current snapshot, checked compiler analysis,
  exact push/pull value parity, no push-ledger mutation, and deterministic
  length-prefixed BLAKE3 result IDs.
- RFC-0033 §§5–6: document and workspace full/unchanged reports, URI order,
  open integer versions, closed/removed `null` versions, empty results, and
  previous-only removal clearance.
- RFC-0033 §7: fully encoded bounded success responses and fixed bilingual
  InvalidParams, MethodNotFound, RequestFailed, and internal failure behavior.

## Implementation

- `LspServer` records pull support during successful initialization and
  advertises the exact provider object only for supporting clients.
- `pull_diagnostics.rs` validates requests before analysis, derives reports
  from a single freshness-checked immutable ticket, and stores no client
  result cache.
- `publication.rs` exposes one non-mutating current-analysis helper and one
  shared URI grouping function, so push and pull use the same compiler adapter
  values and deterministic order.
- Workspace reports union current documents with valid previous-only URIs;
  their `BTreeMap` construction provides exact URI order and deterministic
  empty clearance after removal.

## Tests and evidence

- `crates/ling-lsp/tests/pull_diagnostics.rs` covers negotiation, invalid
  capability shapes, notifications, document full/unchanged/change behavior,
  exact push parity, pending-state preservation, deterministic workspace
  ordering and versions, removal clearance, empty workspaces, unknown-field
  tolerance, invalid/duplicate/untracked requests, the 1024-entry bound,
  temporary syntax diagnostics, and oversized RequestFailed responses.
- Unit tests independently cover the result-ID domain, length prefixes, URI
  separation, and diagnostic-byte separation.
- `crates/ling-lsp/tests/diagnostic_adapter.rs` continues to prove strict
  original-byte projection under UTF-8, UTF-16, and UTF-32 for the exact values
  reused by pull responses.
- Focused `ling-lsp` tests and strict Clippy pass offline. The complete locked-
  offline workspace, Clippy, CI, governance, LSP, support, status, RC0,
  traceability, formatting, and diff gates pass. The exact implementation
  commit is `da69abff0c74765283d3e52e182a7c0ae2f8dc3a`.

## Compatibility, determinism, and Unicode impact

- Adds Preview `ling.lsp.pull-diagnostics/0.1` and
  `ling.lsp.pull-result/0.1:blake3:` with no predecessor. Clients that do not
  negotiate the capability receive no provider advertisement.
- Push and diagnostic-adapter versions and bytes are unchanged. No diagnostic
  allocation, schema field, Semantic ID, Ling syntax/semantics, Typed Core,
  runtime, bytecode, VM, ABI, or Unicode 17.0.0 table changes.
- Result identity excludes versions, revisions, paths, clocks, process state,
  allocation order, and hash-map iteration. Compilation remains locked,
  offline, in-process, and based only on captured repository inputs.

## Intentionally deferred

Dynamic registration, observable cancellation, progress, partial results,
refresh, related-document maps, notebooks, persistence, background scheduling,
root-cause grouping, deduplication, caps, suppression, fixes, Workspace Edits,
Semantic Transactions, and Stable compatibility remain future work.
