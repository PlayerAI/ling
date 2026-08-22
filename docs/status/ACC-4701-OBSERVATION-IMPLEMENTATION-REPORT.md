# ACC-4701-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0170` as test-only accelerator-plugin
interface boundary evidence. It does not add a plugin trait, registry, loader,
manifest, dependency, target package, cache API, diagnostic, or public
protocol.

## Implemented

- Test-local inventory for sixty accelerator-plugin interface boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --test accelerator_plugin_interface_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
bytecode, VM, dependencies, and Unicode 17.0.0 remain unchanged. Public
`ACC-4701` remains `BlockedSpec` for plugin ABI, registry/loader, trust,
capability, cache, protocol integration, and support claims.
