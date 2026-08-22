# FFI-3603-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0139` as test-only FFI shim-generator
boundary evidence. It does not add a generator, template, generated source,
layout/pointer check, ownership adapter, callback trampoline, provenance record,
build-hash input, or executable shim.

## Implemented

- Test-local inventory for sixty proposed shim-generator boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no generator, artifact, build-hash, or
  public-protocol authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --all-targets --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
CLI/LSP behavior, runtime, bytecode, VM, dependencies, and Unicode 17.0.0
remain unchanged. Public `FFI-3603` remains `BlockedSpec` for shim schemas,
generated checks/adapters, trust/TCB, provenance/build hash, diagnostics,
migration, sanitizer/fuzz, and cross-target decisions.
