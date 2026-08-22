# ACT-2306-PROPERTY-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0100` in the publish-disabled
`ling-concurrency` crate. It is a checked-data identity model only and does not
run or certify Actor properties.

## Implemented

- Opaque `PropertyObservationId` identities and optional opaque Actor instances.
- Structural labels for the eight planned property vocabulary alternatives.
- Deterministic observation ordering and validation for unresolved/duplicate IDs.
- Path/span-free `ling.actor-property-observation/0` canonical bytes.
- Unit tests for positive, negative, ordering, and source-evidence boundaries.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-concurrency --all-targets --locked --offline` — 31 tests passed
- `cargo clippy -p ling-concurrency --all-targets --locked --offline -- -D warnings` — passed

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP behavior,
runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 remain unchanged.
Public `ACT-2306` remains `BlockedSpec` for property execution, stress,
scheduling, replay, runtime, and interpreter/VM semantics.
