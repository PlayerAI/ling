# ACT-2302-MESSAGE-SCHEMA-MODEL Implementation Report

## Scope

This child implements Accepted `DEC-0096` in the publish-disabled
`ling-concurrency` crate. It is a checked-data identity model only and does not
check or execute Actor messages.

## Implemented

- Opaque `MessageSchemaId` and `MessageFieldId` identities.
- Immutable schema identities with optional opaque Actor-type ownership.
- Deterministic schema and field ordering.
- Validation for nonzero identities, duplicate schemas, and repeated fields.
- Path/span-free `ling.actor-message-schema/0` canonical bytes.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-concurrency --all-targets --locked --offline` — 19 tests passed
- `cargo clippy -p ling-concurrency --all-targets --locked --offline -- -D warnings` — passed

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Seed source behavior, diagnostics, schemas, Semantic IDs, CLI/LSP behavior,
runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 remain
unchanged. Public `ACT-2302` remains `BlockedSpec` for Sendable, ownership,
Resource/Managed, Capability, payload typing, serialization, mailbox, runtime,
and interpreter/VM semantics.
