# PROF-5102-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0178` as test-only evidence in
`crates/ling-types/tests/forbidden_capability_evidence.rs`. The test records
sixty provisional forbidden-capability boundaries, sorts them by explicit
local rank, rejects duplicates, and compares canonical opaque bytes for
forward/reverse input order.

## Verification

- `cargo test -p ling-types --test forbidden_capability_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No policy/checker, profile format, diagnostic allocation, dependency, target,
source syntax, CLI/LSP option, editor route, runtime, or Unicode behavior
changed. Existing Seed effect/type checks remain unchanged; public PROF-5102
remains `BlockedSpec`.

## Deferred work

Capability/effect taxonomy, rejection phase, transitive checking, profile
policy, bounds/topology/numeric/Fault/FFI semantics, diagnostics, fixtures
beyond boundary evidence, and public support remain open.
