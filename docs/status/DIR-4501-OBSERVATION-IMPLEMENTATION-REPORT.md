# DIR-4501-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0162` as test-only backend-neutral Device
IR schema boundary evidence. It does not add IR types/operations, a schema or
codec, validator/canonicalizer, source-map carrier, capability registry,
diagnostics, or a public protocol.

## Implemented

- Test-local inventory for sixty Device IR schema boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --test device_ir_schema_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
bytecode, VM, dependencies, and Unicode 17.0.0 remain unchanged. Public
`DIR-4501` remains `BlockedSpec` for IR types/operations, schema/codec,
verification, canonicalization, source maps, capability negotiation, backend
integration, migration, protocol integration, and support claims.
