# FFI-3604-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0140` as test-only Target Primitive Package
and `lingabi` boundary evidence. It does not add a target package, manifest,
schema reader, primitive registry, capability/TCB checker, proof verifier,
target selector, or executable primitive.

## Implemented

- Test-local inventory for sixty proposed Target Primitive Package boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no package, capability, TCB, or public
  protocol authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --all-targets --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
CLI/LSP behavior, runtime, bytecode, VM, dependencies, and Unicode 17.0.0
remain unchanged. Public `FFI-3604` remains `BlockedSpec` for package/`lingabi`,
target/profile, capability/TCB, proof, ownership/ABI, provenance, diagnostics,
migration, sanitizer/fuzz, and cross-target decisions.
