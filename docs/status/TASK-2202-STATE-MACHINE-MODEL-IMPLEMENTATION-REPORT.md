# TASK-2202-STATE-MACHINE-MODEL Implementation Report

## Scope

This child implements Accepted `DEC-0092` in the publish-disabled
`ling-concurrency` crate. It is a checked-data model only and does not lower or
execute Task programs.

## Implemented

- Typed state, transition, continuation, and live-local identities.
- Immutable state nodes with deterministic local ordering.
- Structural `Resume`, `Cancel`, `Cleanup`, and `Fault` transition labels.
- Validation for entry existence, unique states/transitions, live-local
  uniqueness, known endpoints, and duplicate edge tuples.
- Path/span-free `ling.task-state-machine/0` canonical bytes.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-concurrency --all-targets --locked --offline` — 7 tests passed
- `cargo clippy -p ling-concurrency --all-targets --locked --offline -- -D warnings` — passed

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP behavior,
runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 remain
unchanged. Public `TASK-2202` remains `BlockedSpec` for actual state-machine
lowering and execution semantics.
