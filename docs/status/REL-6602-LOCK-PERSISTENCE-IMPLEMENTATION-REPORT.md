# REL-6602-LOCK-PERSISTENCE Implementation Report

## Scope

Added a private two-stage lock-persistence boundary retaining the production
write/sync/rename behavior, plus deterministic injected failures for partial
write storage exhaustion and interruption after sync but before replacement.

Both cases preserve the prior lock byte-for-byte, remove the adjacent
temporary file, and emit the existing bilingual `L-IO-0002` diagnostic with
stable operation/I/O-kind facts. The fault inventory now contains three
Covered, zero Partial, and eight Deferred scenarios.

## Verification

- `cargo test -p ling-project lockfile::tests --locked --offline`
- `cargo test -p ling-project --test lockfile_fixtures --locked --offline`
- `cargo clippy -p ling-project --all-targets --locked --offline -- -D warnings`
- `cargo xtask fault verify`
- `cargo xtask governance check-error-codes`
- `cargo test --workspace --all-targets --locked --offline --quiet`
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`
- `cargo xtask ci verify`
- `cargo xtask governance check-all`
- `cargo xtask status verify`
- `cargo fmt --all -- --check`
- `git diff --check`

## Compatibility and deferral

Canonical locks, project graphs, production filesystem ordering, error code,
fact types, messages, spans, CLI, dependencies, schemas, Semantic IDs,
Unicode, and runtime behavior remain unchanged. `StorageFull` now reports the
more precise `storage_full` value rather than `other`. Parent `REL-6602`
remains blocked for OS/process crash recovery and future-system faults.
