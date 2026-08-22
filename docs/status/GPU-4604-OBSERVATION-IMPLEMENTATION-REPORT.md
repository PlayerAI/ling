# GPU-4604-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0168` as test-only differential and
hardware-matrix boundary evidence. It does not add a CPU/GPU harness, matrix
schema, comparator, tolerance registry, hardware claim, diagnostic, or public
protocol.

## Implemented

- Test-local inventory for sixty differential and hardware-matrix boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --test differential_hardware_matrix_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
bytecode, VM, dependencies, and Unicode 17.0.0 remain unchanged. Public
`GPU-4604` remains `BlockedSpec` for differential semantics, hardware matrix,
numeric comparison, support status, protocol integration, and stable claims.
