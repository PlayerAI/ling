# NODE-5307-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0191` as test-only evidence in
`crates/ling-types/tests/node_conformance_evidence.rs`. The test records
sixty provisional Node-conformance protocol, fixture/oracle, state/timing/
input, Fault/fallback, replay/target, diagnostic, and evidence boundaries.
It sorts them by explicit local rank, rejects duplicates, and compares
canonical opaque bytes for forward/reverse input order.

## Verification

- `cargo test -p ling-types --test node_conformance_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No Node conformance runner, fixture manifest, oracle, runtime behavior,
diagnostic allocation, dependency, CLI/LSP action, replay protocol, support
claim, or Unicode behavior changed. Public `NODE-5307` remains `BlockedSpec`.

## Deferred work

Conformance protocol, manifest/oracle, initialization/tick/state, rate/input,
deadline/Fault/fallback/restart/safe mode, replay, target evidence,
diagnostics, fixtures beyond boundary evidence, and public support remain
open.
