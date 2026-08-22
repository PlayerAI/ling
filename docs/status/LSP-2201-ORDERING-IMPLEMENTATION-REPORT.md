# LSP-2201-ORDERING implementation report

Status: Done (bounded internal child only)

This report records the implementation authorized by Accepted DEC-0034. The
parent `LSP-2201` task remains `BlockedSpec`: no public LSP diagnostic adapter,
range projection, severity mapping, fix-data field, or publication behavior is
implemented.

## Normative scope

- DEC-0001 requires stable registered diagnostic codes and deterministic
  compatibility; DEC-0002 preserves original UTF-8 byte spans.
- DEC-0034 §§1–4 authorize only a path-free internal ordering key over logical
  file text, byte offsets, code text, and an explicit local tie-breaker.
- The execution-plan diagnostic mapping remains non-normative and cannot
  authorize an LSP wire schema or editor-visible result.

## Implementation

- `crates/ling-lsp/src/diagnostics.rs` implements `DiagnosticOrderKey` with
  canonical `(file, start byte, code, end byte, tie-breaker)` ordering.
- The key preserves supplied UTF-8 byte offsets and does not normalize paths,
  convert positions, inspect map order, or use severity/messages/facts/repairs.
- The module is internal and intentionally not wired to `LspServer`, compiler
  diagnostics, transport, protocol inventory, or result publication.

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

Public diagnostic field mapping, severity/tags, localization, URI policy,
position conversion, snapshot/version association, related information,
root-cause/deduplication, caps/truncation, clear/replace, cancellation,
publication, and Stable versus Experimental lifecycle remain deferred to
parent `LSP-2201` and `LSP-2204`.
