# PLC-4804-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0175` as test-only evidence in
`crates/ling-types/tests/placement_explain_evidence.rs`. The test records
sixty provisional explain boundaries, sorts them by explicit local rank,
rejects duplicates, and compares canonical opaque bytes for forward/reverse
input order.

## Verification

- `cargo test -p ling-types --test placement_explain_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No command, `zero` or otherwise, production crate, schema, protocol,
diagnostic allocation, dependency, target, cache/runtime API, source syntax,
editor route, or Unicode behavior changed. The public PLC-4804 task remains
`BlockedSpec`.

## Deferred work

Explain schema/transport, `ling` CLI behavior, field ordering, privacy,
redaction, replay/cache identity, diagnostics, fixtures beyond boundary
evidence, and public support remain open.
