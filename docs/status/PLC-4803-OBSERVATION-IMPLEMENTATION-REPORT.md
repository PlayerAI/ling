# PLC-4803-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0174` as test-only evidence in
`crates/ling-types/tests/cost_model_evidence.rs`. The test records sixty
provisional Cost Model boundaries, sorts them by explicit local rank, rejects
duplicates, and compares canonical opaque bytes for forward/reverse input
order.

## Verification

- `cargo test -p ling-types --test cost_model_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No production crate, estimator, benchmark claim, diagnostic allocation,
dependency, target, cache/runtime API, source syntax, or Unicode behavior
changed. The public PLC-4803 task remains `BlockedSpec`.

## Deferred work

Units, calibration, uncertainty, estimator semantics, policy integration,
profile/replay/cache schemas, diagnostics, fixtures beyond boundary evidence,
and public support remain open.
