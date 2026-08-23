# LSP-2104 implementation report

> Status: Done / 已完成
> Task: `LSP-2104`
> Authority: Accepted `RFC-0029`, `RFC-0023`, `DEC-0029`, `DEC-0069`, and `DEC-0070`

## Scope

This milestone implements bounded incremental `textDocument/didChange` over
the existing session-local overlay. One to 64 full or ranged entries are
applied in protocol order using the negotiated position encoding, with one
final VFS and version publication after complete validation.

## Normative clauses covered

- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` LSP-2104: negotiated
  range conversion, protocol-order batches, rebuilt line indexes, rejection
  without VFS pollution, and full-replacement equivalence.
- `RFC-0029` §§1–5: protocol revision/capability, input schema, projection,
  bounds, atomic publication, errors, compatibility, and explicit non-claims.
- `RFC-0023` §§3–8: URI, open/writable state, monotonic versions, overlay
  publication, and stable JSON-RPC error meanings.
- `DEC-0029`, `DEC-0069`, and `DEC-0070`: strict SourceMap conversion and
  immutable ordered source-edit application.

## Implementation

- `ling.lsp.overlay/0.2` and initialize capabilities advertise incremental
  synchronization and the exact 64-entry limit.
- `crates/ling-lsp/src/lib.rs` parses full/range entries, non-negative bounded
  positions, and rejects empty/over-limit arrays and `rangeLength`.
- The server validates document state/version before transformation, rebuilds
  `SourceFile`/line mappings before each ordered ranged entry, enforces 1 MiB
  after every step, and publishes the final exact UTF-8 bytes once.
- Existing RFC-0023 single-full-change messages remain accepted even when the
  exact editor buffer is not yet a valid Ling source snapshot.

## Tests and evidence

- `crates/ling-lsp/tests/incremental_changes.rs` covers capability bytes,
  UTF-8/16/32 equivalence, emoji, Chinese and combining text, BOM/CRLF,
  ordered line-index changes, mixed full/range batches, malformed positions,
  array and size limits, legacy invalid-source full overlays, and later-entry
  failure atomicity.
- Existing overlay, lifecycle, formatting, source position/edit, and CLI
  process suites provide regression coverage for the composed boundaries.

## Compatibility and determinism

- The Experimental overlay advances from 0.1 to 0.2; all valid 0.1 full-sync
  inputs remain accepted, so no migration tool is needed.
- No Ling diagnostic allocation, syntax, Checked Core, Semantic ID, compiler
  span, runtime, bytecode, VM, ABI, package, filesystem, network, or Unicode
  17.0.0 behavior changes.
- Array order is semantic input; local immutable transformations precede one
  deterministic publication and expose no host or Rust implementation detail.

## Verification

The milestone is accepted only after focused LSP/source tests and the full
locked, offline workspace, CI, governance, support, status, RC0, traceability,
Clippy, formatting, and deterministic-diff gates pass. The exact implementation
commit is `492754b066da11e4ae2fe58774e5c7096e3703a5`, bound in
`docs/status/implementation-status.toml`.

## Intentionally deferred

Save semantics, host-path resolution, project reload, compiler snapshots,
stale analysis, diagnostics publication, navigation, cancellation, Workspace
Edits, Semantic Transactions, and Stable editor compatibility remain owned by
later tasks and Accepted authorities.
