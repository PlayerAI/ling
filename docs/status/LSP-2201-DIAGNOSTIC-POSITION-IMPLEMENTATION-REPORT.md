# LSP-2201-DIAGNOSTIC-POSITION Implementation Report

## Outcome

`ling-lsp` now contains a private, deterministic helper that projects a
compiler `DiagnosticSpan` from original UTF-8 byte offsets into an explicit
`LspPosition` range. It validates logical source identity, range order, offset
width, and source-map boundaries without clamping or mutating state. Accepted
RFC-0031 now consumes this helper in the public adapter.

## Implementation

- Added the local `ling-diagnostics` dependency to consume the existing
  `DiagnosticSpan` value without duplicating diagnostic facts.
- Added private `DiagnosticPositionRange`, `DiagnosticProjectionError`, and
  `project_span` in `crates/ling-lsp/src/diagnostics.rs`.
- Added tests for UTF-8/16/32 positions, Unicode, BOM/CRLF, final lines,
  identity/range/offset failures, and strict source-boundary rejection.

## Verification

Executed locally:

- `cargo fmt --all -- --check`
- `cargo test -p ling-lsp --all-targets --locked --offline` (17 tests passed)
- `cargo clippy -p ling-lsp --all-targets --locked --offline -- -D warnings`
- `cargo xtask governance check-all` (112 documents, 28 gaps, 87 lifecycle
  records, 27 protocols, and 89 diagnostic codes)

Implementation commit: `e66669142f5b8720a39157532edd77b8bc46269a`.

## Compatibility and determinism

No Ling syntax, semantics, diagnostic registry, JSON/CLI output, Semantic IDs,
LSP wire method, runtime, bytecode, VM, ABI, or Unicode 17.0.0 data changed.
The helper depends only on explicit source bytes, logical-name identity,
position encoding, and the authoritative SourceMap; host paths, URI parsing,
allocation order, and map iteration are not observable.

## Intentionally deferred

Publication, URI/document versions, snapshot freshness, stale-result handling,
cancellation, suppression, root-cause grouping, caps, tags, repair application,
and JSON-RPC publication compatibility remain deferred to LSP-2202 through
LSP-2205 and future Accepted decisions.
