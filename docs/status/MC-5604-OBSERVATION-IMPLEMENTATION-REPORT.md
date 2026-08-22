# MC-5604-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0205` as test-only evidence in
`crates/ling-concurrency/tests/replay_counterexample_evidence.rs`. The test
records sixty provisional conversion, replay, scheduler, runtime, event,
identity, source-link, failure, privacy, diagnostic, and fixture boundaries.
It sorts them by explicit local rank, rejects duplicates, and compares
canonical opaque bytes for forward/reverse input order.

## Verification

- `cargo test -p ling-concurrency --test replay_counterexample_evidence --locked --offline`
- `cargo clippy -p ling-concurrency --all-targets --locked --offline -- -D warnings`

No converter, replay schema/reader/writer, scheduler trace, reference-runtime
route, source-link protocol, diagnostic allocation, dependency, CLI/LSP
action, public protocol, support claim, or Unicode behavior changed. Public
`MC-5604` remains `BlockedSpec`.

## Deferred work

Counterexample conversion, replay/runtime implementation, event/scheduler/
effect/source-link semantics, diagnostics, fixtures beyond boundary evidence,
protocols, and public support remain open.
