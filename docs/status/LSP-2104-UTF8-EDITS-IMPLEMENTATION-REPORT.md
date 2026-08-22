# LSP-2104-UTF8-EDITS Implementation Report

## Outcome

The source layer now provides a deterministic in-process UTF-8 edit primitive.
One edit or an ordered batch produces a new validated `SourceFile`; malformed
ranges or replacements return typed errors without mutating the input. This
is evidence for the bounded child only. The public `LSP-2104` target remains
`BlockedSpec`.

## Implementation

- Added `ling_source::Utf8Edit` with original-byte range and replacement bytes.
- Added `SourceFile::apply_utf8_edit` and `apply_utf8_edits`, preserving source
  identity/name and rebuilding lexical/source-map data from exact bytes.
- Rejected reversed/out-of-bounds ranges, UTF-8 scalar interiors, CRLF-pair
  interiors, invalid UTF-8 or BOM replacements, and oversized results.
- Added source-layer tests for Unicode, BOM/CRLF, full-replacement equivalence,
  ordered edits, deterministic output, and atomic failure.

## Verification

Executed locally:

- `cargo fmt --all`
- `cargo test -p ling-source --all-targets --locked --offline` (21 tests)
- `cargo clippy -p ling-source --all-targets --locked --offline -- -D warnings`

## Compatibility and determinism

No Ling syntax, semantics, diagnostics, schemas, Semantic IDs, CLI, LSP wire
method, VFS publication, bytecode, runtime, VM, ABI, or Unicode 17.0.0 data
changed. The result depends only on immutable source bytes and the ordered edit
values; paths, allocation addresses, hash-map order, and debug formatting are
not observable.

## Intentionally deferred

Negotiated LSP positions, URI/document versions, JSON-RPC `didChange`, VFS
transactions, stale compiler results, cancellation, diagnostics, Workspace
Edits, and public compatibility remain deferred to the blocked parent.

