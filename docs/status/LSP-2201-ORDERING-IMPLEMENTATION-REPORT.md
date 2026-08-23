# LSP-2201-ORDERING implementation report

Status: Done (bounded internal child consumed by RFC-0031)

This report records the implementation authorized by Accepted DEC-0034. The
parent `LSP-2201` now consumes this primitive under Accepted RFC-0031. This
child itself still defines no field mapping or publication behavior.

## Normative scope

- DEC-0001 requires stable registered diagnostic codes and deterministic
  compatibility; DEC-0002 preserves original UTF-8 byte spans.
- DEC-0034 §§1–4 authorize only a path-free internal ordering key over logical
  file text, byte offsets, code text, and an explicit local tie-breaker.
- Accepted RFC-0031, rather than the execution-plan checklist, authorizes the
  separate public adapter schema and editor-facing value.

## Implementation

- `crates/ling-lsp/src/diagnostics.rs` implements `DiagnosticOrderKey` with
  canonical `(file, start byte, code, end byte, tie-breaker)` ordering.
- The key preserves supplied UTF-8 byte offsets and does not normalize paths,
  convert positions, inspect map order, or use severity/messages/facts/repairs.
- The key remains internal and is consumed by RFC-0031's pure adapter. It is
  not wired to `LspServer`, transport, or result publication.

## Evidence

Focused commands executed:

```text
cargo fmt --all -- --check
cargo test -p ling-lsp --all-features --locked --offline
cargo clippy -p ling-lsp --all-targets --all-features --locked --offline -- -D warnings
```

Tests cover file/span/code ordering, explicit tie-breakers, Unicode names,
CRLF-oriented byte offsets, and repeated deterministic sorting.

## Compatibility and determinism

No language syntax, Typed Core, interpreter, VM, bytecode, diagnostics,
schemas, Semantic IDs, source-span identity, CLI, JSON-RPC methods, protocol
inventory, support matrix, or Unicode 17.0.0 data changed. No diagnostic code
was allocated and no LSP position encoding was selected.

## Deferred work

Snapshot/version association, root-cause/deduplication, caps/truncation,
clear/replace, cancellation, publication, tags, repair application, and Stable
lifecycle remain deferred to LSP-2202 through LSP-2205.
