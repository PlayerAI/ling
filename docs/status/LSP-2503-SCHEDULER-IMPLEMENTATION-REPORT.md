# LSP-2503-SCHEDULER implementation report

Status: Done (bounded internal child only)

This report records the implementation authorized by Accepted DEC-0032. The
parent `LSP-2503` task remains `BlockedSpec`: no public LSP debounce, priority,
fairness, freshness, cancellation, or publication behavior is implemented.

## Normative scope

- DEC-0021 §3 supplies canonical internal ordering and serial publication
  principles for deterministic work; it does not authorize an LSP wire API.
- DEC-0032 §§1–5 authorize only a `pub(crate)` in-process queue over opaque
  work IDs with three logical priorities and a local monotonic FIFO sequence.
- The execution-plan bullets in `04-LSP-IMPLEMENTATION.md` remain
  non-normative and cannot define timers, event triggers, or editor behavior.

## Implementation

- `crates/ling-lsp/src/scheduler.rs` implements `InternalWorkQueue`,
  `WorkPriority`, `ScheduledWork`, and the typed sequence-exhaustion error.
- Queue keys are `(priority rank, local enqueue sequence)`, so ordering is
  independent of map insertion order and equal-priority items are FIFO.
- The queue does not spawn workers, sleep, inspect host timing/CPU state,
  coalesce identifiers, carry revisions/request IDs, or execute work.
- `crates/ling-lsp/src/lib.rs` keeps the module internal and intentionally does
  not wire it into `LspServer` or the stdio transport pending parent authority.

## Evidence

Focused commands executed:

```text
cargo test -p ling-lsp --all-features --locked --offline
cargo clippy -p ling-lsp --all-targets --all-features --locked --offline -- -D warnings
cargo fmt --all -- --check
```

The focused suite covers priority-before-FIFO ordering, duplicate identifiers
as independent items, deterministic clear behavior, and sequence monotonicity.

## Compatibility and determinism

No language syntax, Typed Core, interpreter, VM, bytecode, diagnostics,
schemas, Semantic IDs, source spans, CLI, JSON-RPC methods, protocol inventory,
support matrix, or Unicode 17.0.0 data changed. The queue's sequence is local
to one process and is not serialized or observable through the editor protocol.

## Deferred work

Public event triggers, debounce/coalescing, priority fairness and starvation
bounds, worker/resource budgets, dependency expansion, revision supersession,
cancellation/result precedence, diagnostic replacement, progress, and Stable
versus Experimental lifecycle remain deferred to the parent `LSP-2503` task.
