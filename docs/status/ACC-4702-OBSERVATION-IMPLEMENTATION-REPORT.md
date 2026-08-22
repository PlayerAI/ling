# ACC-4702-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0171` as test-only Experimental
accelerator-adapter boundary evidence. It does not add an adapter, plugin
package, graph bridge, target/support entry, dependency, cache/runtime API,
diagnostic, or public protocol.

## Implemented

- Test-local inventory for sixty Experimental accelerator-adapter boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --test experimental_accelerator_adapter_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
bytecode, VM, dependencies, and Unicode 17.0.0 remain unchanged. Public
`ACC-4702` remains `BlockedSpec` for adapter implementation, package/graph
bridge, Experimental lifecycle, trust, protocol integration, and support
claims.
