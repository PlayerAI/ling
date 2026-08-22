# PLC-4802-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0173` as test-only evidence in
`crates/ling-types/tests/placement_selection_evidence.rs`. The test records
sixty provisional selection boundaries, sorts them by explicit local rank,
rejects duplicates, and compares canonical opaque bytes for forward/reverse
input order.

## Verification

- `cargo test -p ling-types --test placement_selection_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No production crate, selector, replay protocol, diagnostic allocation,
dependency, target, cache/runtime API, source syntax, or Unicode behavior
changed. The public PLC-4802 task remains `BlockedSpec`.

## Deferred work

RFC-H405, selector semantics, verified artifacts, runtime availability,
policy/cost, fallback/rejection, profile-specific replay, migration/cache
schemas, diagnostics, fixtures beyond boundary evidence, and public support
remain open.
