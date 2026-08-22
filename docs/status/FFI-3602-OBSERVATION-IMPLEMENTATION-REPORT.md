# FFI-3602-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0138` as test-only C ABI boundary evidence.
It does not add C declaration/import syntax, a layout result, a linker or
compiler probe, callback/handle runtime, allocator bridge, or executable ABI.

## Implemented

- Test-local inventory for sixty proposed C ABI interoperability boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no layout, linker, pointer, or public
  protocol authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --all-targets --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
CLI/LSP behavior, runtime, bytecode, VM, dependencies, and Unicode 17.0.0
remain unchanged. Public `FFI-3602` remains `BlockedSpec` for C layout,
calling/linker, span/callback/handle/allocator safety, ownership/lifetime,
Error/Fault, target/profile, schema, and cross-target decisions.
