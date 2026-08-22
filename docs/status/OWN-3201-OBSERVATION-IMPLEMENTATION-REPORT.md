# OWN-3201-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0118` as test-only Place and Move-analysis
boundary evidence. It does not implement move/borrow states, ownership
dataflow, diagnostics, lifetimes, or Typed Core semantics.

## Implemented

- Test-local inventory for thirty-four proposed Place/Move boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no ownership authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --all-targets --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed Place behavior, source acceptance, diagnostics, schemas, Semantic
IDs, CLI/LSP behavior, runtime, bytecode, VM, ABI, dependencies, and Unicode
17.0.0 remain unchanged. Public `OWN-3201` remains `BlockedSpec` for future
ownership dataflow, lifetimes, diagnostics, boundary rules, migration, and
differential semantics.
