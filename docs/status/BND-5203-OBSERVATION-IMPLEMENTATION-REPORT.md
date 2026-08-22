# BND-5203-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0183` as test-only evidence in
`crates/ling-types/tests/memory_budgets_evidence.rs`. The test records sixty
provisional memory-budget, allocation/lifetime, queue/task/device,
proof/target, fallback, diagnostic, and fixture boundaries, sorts them by
explicit local rank, rejects duplicates, and compares canonical opaque bytes
for forward/reverse input order.

## Verification

- `cargo test -p ling-types --test memory_budgets_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No memory-budget analyzer, allocation/ownership model, target binding, proof
state, runtime guard, diagnostic allocation, dependency, target, CLI/LSP
action, runtime, or Unicode behavior changed. Existing VM and decoder limits
remain unchanged; public BND-5203 remains `BlockedSpec`.

## Deferred work

Memory units/layout, allocation and lifetime accounting, peak/path analysis,
proof/estimate semantics, target/compiler binding, runtime/fallback behavior,
diagnostics, fixtures beyond boundary evidence, and public support remain
open.
