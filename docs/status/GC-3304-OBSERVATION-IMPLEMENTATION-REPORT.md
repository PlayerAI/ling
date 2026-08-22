# GC-3304-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0128` as test-only Managed Profile
boundary evidence. It does not implement Profile syntax or checking,
manifests, capabilities, `no_gc`, allocation restrictions, runtime assertions,
diagnostics, or runtime semantics.

## Implemented

- Test-local inventory for forty-four proposed Profile and `no_gc` boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no checker, syntax, or runtime authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --all-targets --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
CLI/LSP behavior, runtime, bytecode, VM, ABI, dependencies, and Unicode
17.0.0 remain unchanged. Public `GC-3304` remains `BlockedSpec` for Profile
identity/versioning, feature legality, `no_gc`, Managed allocation, Native
Islands, Critical restrictions, assertions/Faults, diagnostics, migration, and
differential contracts.
