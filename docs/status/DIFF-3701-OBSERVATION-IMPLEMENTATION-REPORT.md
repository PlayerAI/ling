# DIFF-3701-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0142` as test-only differential-harness
boundary evidence. It does not add a harness, Native backend, engine adapter,
trace schema, normalizer, corpus, replay tool, allowed-difference registry, or
equivalence claim.

## Implemented

- Test-local inventory for sixty proposed differential boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no execution, equivalence, or public
  protocol authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --all-targets --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
CLI/LSP behavior, runtime, bytecode, VM, dependencies, and Unicode 17.0.0
remain unchanged. Public `DIFF-3701` remains `BlockedSpec` for engine adapters,
Native execution, trace/normalization schemas, equivalence/allowed differences,
replay, corpus, diagnostics, migration, and cross-target/compiler decisions.
