# ACT-2303-MAILBOX-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0097` in the publish-disabled
`ling-concurrency` crate. It is a checked-data identity model only and does not
implement or expose Actor mailboxes.

## Implemented

- Opaque `MailboxId` identities and optional opaque Actor-type owners.
- Structural labels for the five design vocabulary alternatives.
- Deterministic mailbox ordering and validation for unresolved/duplicate IDs.
- Path/span-free `ling.actor-mailbox-observation/0` canonical bytes.
- Unit tests for positive, negative, ordering, and source-evidence boundaries.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-concurrency --all-targets --locked --offline` — 22 tests passed
- `cargo clippy -p ling-concurrency --all-targets --locked --offline -- -D warnings` — passed

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP behavior,
runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 remain unchanged.
Public `ACT-2303` remains `BlockedSpec` for capacity, queue, send,
backpressure, ordering, supervision, runtime, and interpreter/VM semantics.
