# LSP-2202-BATCH implementation report

Status: Done (bounded internal child only)

This report records the implementation authorized by Accepted DEC-0035. The
parent `LSP-2202` task remains `BlockedSpec`: no `publishDiagnostics` handler,
trigger, snapshot/version association, clearing, replacement, or public LSP
diagnostic schema is implemented.

## Normative scope

- DEC-0034 supplies the canonical diagnostic ordering key.
- DEC-0035 §§1–4 authorize only an internal immutable batch of opaque IDs and
  ordering keys; equal keys remain distinct and insertion-stable.
- The execution-plan push-diagnostic bullets remain non-normative and cannot
  authorize a wire method or publication lifecycle.

## Implementation

- `crates/ling-lsp/src/diagnostic_batch.rs` implements `DiagnosticBatch` and
  `DiagnosticItem`.
- `finish` consumes the mutable collection, sorts by `DiagnosticOrderKey`, and
  returns an immutable boxed slice. No deduplication, suppression, truncation,
  conversion, serialization, or publication occurs.
- The module is internal and intentionally disconnected from `LspServer`,
  compiler diagnostics, transport, protocol inventory, and support claims.

## Evidence

Focused commands executed:

```text
cargo fmt --all -- --check
cargo test -p ling-lsp --all-features --locked --offline
cargo clippy -p ling-lsp --all-targets --all-features --locked --offline -- -D warnings
```

Tests cover empty batches, canonical ordering, duplicate/equal-key stability,
and repeated immutable output.

## Compatibility and determinism

No language syntax, Typed Core, interpreter, VM, bytecode, diagnostics,
schemas, Semantic IDs, source-span identity, CLI, JSON-RPC methods, protocol
inventory, support matrix, or Unicode 17.0.0 data changed. No diagnostic code
was allocated and no partial result can be published by this child.

## Deferred work

Public diagnostic triggers, push/pull selection, snapshot/version association,
URI/range projection, severity/tags, clear/replace, suppression, caps,
truncation, cancellation/stale precedence, localization, and protocol lifecycle
remain deferred to parent `LSP-2202`, `LSP-2203`, and `LSP-2204`.
