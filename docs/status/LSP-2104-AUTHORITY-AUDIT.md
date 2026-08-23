# LSP-2104 Authority Audit: Incremental text changes

## Outcome

`LSP-2104` is implementation-ready and complete under Accepted `RFC-0029`.
The RFC composes the existing `DEC-0069` UTF-8 edit and `DEC-0070` negotiated-
position edit primitives with the Done LSP-2102 negotiation and LSP-2103
overlay boundaries.

The server now accepts bounded ordered incremental `didChange` batches,
rebuilds source mapping after every entry, and publishes one final VFS snapshot
only after the entire batch succeeds. The valid RFC-0023 single full-
replacement form remains accepted.

## Normative traceability

- Accepted `DEC-0002` preserves original UTF-8 byte spans.
- Accepted `DEC-0019` owns immutable VFS snapshots and deterministic revisions.
- Accepted `DEC-0029` defines strict UTF-8/16/32 position projection without
  clamping.
- Accepted `RFC-0023` defines URI identity, open state, monotonic client
  versions, writability, overlay precedence, and full-sync compatibility.
- Accepted `DEC-0069` defines ordered immutable original-byte edits and source-
  map rebuilding; `DEC-0070` composes explicit positions with that primitive.
- Accepted `RFC-0029` defines `ling.lsp.overlay/0.2`, capability advertisement,
  the 1–64 entry schema, protocol-order application, size/boundary failures,
  and one-shot VFS/version publication.
- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` requires exactly those
  range conversion, ordering, line-index, failure-atomicity, and full-
  replacement-equivalence properties.

## Current implementation evidence

- Initialize advertises `textDocumentSync` with `openClose: true` and
  incremental `change: 2`, plus the exact overlay version and batch limit.
- `parse_change_params` accepts one through 64 full or ranged changes, rejects
  `rangeLength`, and validates position integers as non-negative `u32` values.
- `change_document` validates URI/open/writable/version state first, transforms
  immutable `SourceFile` snapshots in array order using the negotiated
  encoding, enforces the 1 MiB limit after every entry, and calls the VFS once.
- Projection failures, malformed ranges, invalid BOM/source results, stale
  versions, read-only documents, and oversized intermediates leave visible
  bytes, client version, last-version history, and VFS revision unchanged.
- Tests cover UTF-8/16/32 equivalence, ordered and mixed batches, rebuilt line
  indexes, Chinese text, emoji, combining marks, BOM, CRLF, malformed shapes,
  limits, later-entry failure atomicity, and 0.1 full-sync compatibility.

## Specification gaps retained

`RFC-0029` closes only document synchronization. It does not close
`GAP-LSP-TRANSACTION-PROTOCOL-001` for compiler request snapshots, stale
analysis publication, diagnostics, cancellation, Workspace Edits, or Semantic
Transactions. Those surfaces remain assigned to later tasks and cannot be
inferred from VFS publication.

## Compatibility and determinism

The Experimental overlay advances from 0.1 to 0.2 and advertises the new
capability. Every valid 0.1 single-full-change message remains valid, so no
migration tool is required. No Ling diagnostic, syntax, Checked Core,
Semantic ID, compiler span, runtime, bytecode, VM, ABI, package, filesystem,
network, or Unicode 17.0.0 behavior changes.

Batch order is explicit protocol input. Source projection and line-index
rebuilding depend only on immutable bytes and the negotiated encoding; VFS
publication occurs once and exposes no host path, allocation, or hash order.

## Intentionally deferred

Save semantics, host-file resolution, project reload, compiler snapshots,
stale analysis, diagnostics publication, navigation, cancellation, Workspace
Edits, Semantic Transactions, and Stable editor compatibility remain governed
by later execution-plan tasks.
