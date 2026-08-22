# NODE-5306-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0190` as test-only evidence in
`crates/ling-types/tests/node_actor_boundary_evidence.rs`. The test records
sixty provisional Node/Actor identity, envelope, queue/delivery,
ownership, turn/lifecycle, replay/profile, diagnostic, and fixture
boundaries. It sorts them by explicit local rank, rejects duplicates, and
compares canonical opaque bytes for forward/reverse input order.

## Verification

- `cargo test -p ling-types --test node_actor_boundary_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No Node/Actor queue, bridge envelope, runtime, ownership/replay schema,
diagnostic allocation, dependency, CLI/LSP action, runtime protocol, support
claim, or Unicode behavior changed. Public `NODE-5306` remains `BlockedSpec`.

## Deferred work

Bridge queues/envelopes, capacity/backpressure/drop/expiry, sampling/commit,
delivery/order, ownership/serialization, turn/reentry, supervision/Fault,
restart/fallback, replay, diagnostics, fixtures beyond boundary evidence,
and public support remain open.
