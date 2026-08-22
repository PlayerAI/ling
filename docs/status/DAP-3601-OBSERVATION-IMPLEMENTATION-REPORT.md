# DAP-3601-OBSERVATION Implementation Report

## Scope

This child implements Accepted `DEC-0144` as test-only debugger boundary
evidence. It does not add a DAP transport, stdio process, debugger command,
runtime hook, source-map bridge, extension, or public protocol.

## Implemented

- Test-local inventory for sixty proposed DAP/debugger boundaries.
- Explicit local ordering and duplicate rejection.
- Deterministic opaque evidence bytes for forward/reverse insertion order.
- Tests that state the evidence has no debugger protocol or public API
  authority.

## Verification

- `cargo fmt --all`
- `cargo test -p ling-types --test dap_debugger_boundary_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

The full workspace gates and milestone commit binding remain required.

## Compatibility and handoff

Accepted Seed tests, source acceptance, diagnostics, schemas, Semantic IDs,
CLI/LSP behavior, runtime, bytecode, VM, dependencies, and Unicode 17.0.0
remain unchanged. Public `DAP-3601` remains `BlockedSpec` for protocol
schema/framing, lifecycle, source-map/identity, debug semantics, security,
metadata, editor integration, fixtures, and support claims.
