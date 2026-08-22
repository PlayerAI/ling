# OWN-3204-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0121` as test-only cross-suspension and
Actor-turn boundary evidence. It does not implement `await`, suspension,
pinning, state-machine lowering, cross-turn borrow checking, Actor reentry,
message sendability, diagnostics, or ownership semantics.

## Implemented

- Test-local inventory for thirty-seven proposed suspension/Actor boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no await or Actor authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-concurrency --all-targets --locked --offline`
- `cargo clippy -p ling-concurrency --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed behavior, source acceptance, diagnostics, schemas, Semantic IDs,
CLI/LSP behavior, runtime, bytecode, VM, ABI, dependencies, and Unicode
17.0.0 remain unchanged. Public `OWN-3204` remains `BlockedSpec` for await,
suspension, pinning, Actor reentry, message sendability, cancellation/Drop,
diagnostics, migration, and differential semantics.
