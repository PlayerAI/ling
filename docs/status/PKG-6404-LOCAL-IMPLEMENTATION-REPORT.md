# PKG-6404-LOCAL Implementation Report

## Scope

Implemented Accepted `DEC-0229` with representative hostile local package
tests, stronger disposable internal-cache corruption evidence, and an exact
ten-attack assessment matching the execution plan. Four attacks have bounded
RFC-0002 local-subset evidence; six remain explicitly unavailable because no
accepted prerequisite protocol exists.

## Verification

- `cargo test -p ling-project --test supply_chain_boundary --locked --offline`
- `cargo test -p ling-cache --lib hostile_envelopes_are_bounded_safe_misses --locked --offline`
- `cargo test -p ling-types --test supply_chain_boundary_evidence --locked --offline`
- `cargo test --workspace --all-targets --locked --offline --quiet`
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`
- `cargo xtask ci verify`
- `cargo xtask governance check-all`
- `cargo xtask status verify`
- `cargo fmt --all -- --check`
- `git diff --check`

## Compatibility and deferral

The milestone changes tests and governance evidence only. It changes no
manifest, lock, package identity, resolver, compiler, cache format, diagnostic,
CLI, Semantic ID, source span, dependency, Unicode version, or public protocol.
Parent `PKG-6404` remains `BlockedSpec` pending future Accepted registry,
archive, signing, package-cache, and hermetic-build security authority.
