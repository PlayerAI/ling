# ACT-2301-IDENTITY-MODEL Implementation Report

## Scope

This child implements Accepted `DEC-0095` in the publish-disabled
`ling-concurrency` crate. It is a checked-data identity model only and does not
execute or authorize Actor programs.

## Implemented

- Opaque `ActorTypeId`, `ActorId`, and `ActorRefId` identities.
- Immutable Actor type, instance, and reference values.
- Structural Local/Remote reference labels with no transport meaning.
- Validation for nonzero and duplicate identities plus known type/actor targets.
- Deterministic identity ordering and path/span-free
  `ling.actor-identity/0` canonical bytes.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-concurrency --all-targets --locked --offline` — 16 tests passed
- `cargo clippy -p ling-concurrency --all-targets --locked --offline -- -D warnings` — passed

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP behavior,
runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 remain
unchanged. Public `ACT-2301` remains `BlockedSpec` for Actor syntax, turns,
state isolation, borrow/sendability, mailbox, serialization, scheduler,
runtime, and interpreter/VM semantics.
