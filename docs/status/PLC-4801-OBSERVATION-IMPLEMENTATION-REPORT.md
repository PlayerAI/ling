# PLC-4801-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0172` as test-only evidence in
`crates/ling-types/tests/placement_constraint_evidence.rs`. The test records
sixty provisional Placement boundaries, sorts them by explicit local rank,
rejects duplicates, and compares canonical opaque bytes for forward/reverse
input order.

## Verification

- `cargo test -p ling-types --test placement_constraint_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No production crate, public protocol, diagnostic allocation, dependency,
target, cache/runtime API, source syntax, or Unicode behavior changed. The
public PLC-4801 task remains `BlockedSpec`.

## Deferred work

RFC-H405, Placement grammar and solver semantics, verified device/topology and
buffer inputs, fallback/rejection/cost behavior, explain/replay/cache schemas,
diagnostics, fixtures beyond boundary evidence, and public support remain open.
