# DIR-4502-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0163` as test-only Kernel-to-Device
lowering boundary evidence. It does not add Kernel verifier extensions,
lowerers, mappings, proof carriers, source maps, diagnostics, or a public
protocol.

## Implemented

- Test-local inventory for sixty Kernel-to-Device lowering boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --test kernel_device_lowering_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
bytecode, VM, dependencies, and Unicode 17.0.0 remain unchanged. Public
`DIR-4502` remains `BlockedSpec` for verifier extensions, lowering mappings,
proof/provenance carriers, source maps, differential evidence, migration,
protocol integration, and support claims.
