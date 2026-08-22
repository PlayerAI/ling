# FFI-3601-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0137` as test-only FFI declaration-boundary
evidence. It does not add declaration syntax, an ABI schema, foreign symbol
resolution, an unsafe pointer surface, or an executable FFI call.

## Implemented

- Test-local inventory for sixty proposed FFI declaration boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no syntax, ABI, ownership, target, or
  public-protocol authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --all-targets --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
CLI/LSP behavior, runtime, bytecode, VM, dependencies, and Unicode 17.0.0
remain unchanged. Public `FFI-3601` remains `BlockedSpec` for declaration
grammar, ABI/layout, ownership/lifetime, callback/thread/reentry, Error/Fault,
Capability/Profile/Target, schema, and cross-target decisions.
