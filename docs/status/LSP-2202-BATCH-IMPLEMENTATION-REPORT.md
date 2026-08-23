# LSP-2202-BATCH implementation report

Status: Done (bounded internal child only)

This report records the implementation authorized by Accepted DEC-0035. The
child remains a bounded internal collection and was completed before the
parent publication contract existed. Accepted RFC-0032 now separately
authorizes and implements parent `LSP-2202`; it does not retroactively broaden
this child's API or authority.

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

RFC-0032 now owns parent push triggers, snapshot/version association, strict
URI/range projection, clear/replace, and stale precedence. Pull selection and
parity remain LSP-2203; suppression, root-cause grouping, and caps remain
LSP-2204; cancellation requests, tags, localization policy, and Stable
protocol lifecycle remain deferred.
