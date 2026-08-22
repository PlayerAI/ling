# LSP-2504-BYTE-ACCOUNTING implementation report

Status: Done (bounded internal child only)

This report records the implementation authorized by Accepted DEC-0033. The
parent `LSP-2504` task remains `BlockedSpec`: no public LSP quota, limit
diagnostic, configuration, request rejection, or host-memory behavior is
implemented.

## Normative scope

- DEC-0033 §§1–5 authorize only deterministic UTF-8-byte arithmetic in a
  `pub(crate)` in-process budget.
- RFC-0002, bytecode limits, and existing transport limits remain in their own
  domains and are not reused as LSP resource semantics.
- The execution-plan bullet in `04-LSP-IMPLEMENTATION.md` is non-normative and
  cannot define public quotas or diagnostics.

## Implementation

- `crates/ling-lsp/src/resource.rs` implements `ByteBudget` and typed
  `ByteBudgetError` values for exact-boundary reserve/release accounting.
- The budget counts UTF-8 bytes supplied by its owner, rejects over-limit
  reserves without mutation, and rejects releases beyond current usage without
  underflow. Zero operations are valid no-ops.
- The module is internal and intentionally not wired to `LspServer`, the VFS,
  transport, document overlays, request snapshots, or a public diagnostic.

## Evidence

Focused commands executed:

```text
cargo fmt --all -- --check
cargo test -p ling-lsp --all-features --locked --offline
cargo clippy -p ling-lsp --all-targets --all-features --locked --offline -- -D warnings
```

Tests cover exact-boundary and failed-reserve stability, checked release and
zero operations, and independent-budget isolation.

## Compatibility and determinism

No language syntax, Typed Core, interpreter, VM, bytecode, diagnostics,
schemas, Semantic IDs, source spans, CLI, JSON-RPC methods, protocol inventory,
support matrix, or Unicode 17.0.0 data changed. The arithmetic does not observe
allocator state, process memory, CPU, wall-clock time, or host paths.

## Deferred work

Public units and scopes, defaults and negotiation, hard/soft quotas, pending
requests and result accounting, completion/diagnostic/solver limits,
dependency/generated-file policy, cancellation/stale precedence, retry,
no-partial-publication, host-memory handling, and bilingual diagnostic
allocation remain deferred to parent `LSP-2504`.
