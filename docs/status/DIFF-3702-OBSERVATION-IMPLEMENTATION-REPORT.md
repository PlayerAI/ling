# DIFF-3702-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0143` as test-only allowed-difference
boundary evidence. It does not add a registry, registry reader, comparison
predicate, backend exemption, Native adapter, numeric/replay rule, or
equivalence claim.

## Implemented

- Test-local inventory for sixty proposed allowed-difference boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no registry, equivalence, or public
  protocol authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --test allowed_difference_registry_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
CLI/LSP behavior, runtime, bytecode, VM, dependencies, and Unicode 17.0.0
remain unchanged. Public `DIFF-3702` remains `BlockedSpec` for registry
schema/reader, fail-closed/conflict/expiry policy, allowed predicates,
numeric/replay/cleanup/scheduling/FFI/target semantics, protocol integration,
and cross-target/compiler evidence.
