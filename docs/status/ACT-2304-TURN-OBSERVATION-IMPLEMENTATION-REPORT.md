# ACT-2304-TURN-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0098` in the publish-disabled
`ling-concurrency` crate. It is a checked-data identity model only and does not
implement or expose Actor turns.

## Implemented

- Opaque `TurnId` identities and optional opaque Actor-type owners.
- Structural labels for the six turn/reentry design vocabulary alternatives.
- Deterministic turn ordering and validation for unresolved/duplicate IDs.
- Path/span-free `ling.actor-turn-observation/0` canonical bytes.
- Unit tests for positive, negative, ordering, and source-evidence boundaries.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-concurrency --all-targets --locked --offline` — 25 tests passed
- `cargo clippy -p ling-concurrency --all-targets --locked --offline -- -D warnings` — passed

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP behavior,
runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 remain unchanged.
Public `ACT-2304` remains `BlockedSpec` for await, reentry, state guards,
self-send, watchdog, scheduler, runtime, and interpreter/VM semantics.
