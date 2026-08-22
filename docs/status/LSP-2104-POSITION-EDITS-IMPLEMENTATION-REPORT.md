# LSP-2104-POSITION-EDITS Implementation Report

## Outcome

The source layer now composes explicit UTF-8/UTF-16/UTF-32 lexical positions
with immutable original-byte edits. Single edits and ordered batches return
new validated `SourceFile` values; invalid positions or replacements return
typed errors without changing the input. This completes the bounded child,
not the public `LSP-2104` parent.

## Implementation

- Added `ling_source::LspPositionEdit` and `LspPositionEditError`.
- Added `SourceFile::apply_lsp_position_edit` and
  `apply_lsp_position_edits`, using `SourceMap`/`PositionEncoding` before
  delegating to `Utf8Edit`.
- Added tests for UTF-8/16/32, Chinese/emoji, BOM/CRLF, full replacement,
  ordered batch behavior, no-clamping errors, and atomic failure.

## Verification

Executed locally:

- `cargo fmt --all`
- `cargo test -p ling-source --all-targets --locked --offline` (24 tests)
- `cargo clippy -p ling-source --all-targets --locked --offline -- -D warnings`

## Compatibility and determinism

No Ling syntax, semantics, diagnostics, schemas, Semantic IDs, CLI, LSP wire
method, VFS publication, bytecode, runtime, VM, ABI, or Unicode 17.0.0 data
changed. Results depend only on immutable source bytes, explicit encoding, and
ordered edit values; host paths and allocation/hash order are not observable.

## Intentionally deferred

Negotiation/capability advertisement, URI/document versions, JSON-RPC
`didChange`, VFS transactions, stale compiler results, cancellation,
diagnostics, Workspace Edits, and public compatibility remain deferred to the
blocked parent.

