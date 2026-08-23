# COMPAT-6504-READINESS Implementation Report

## Scope

Implemented Accepted `DEC-0233` with an exact seven-area deprecation-policy
readiness inventory, generated report, source-backed boundary checks, and an
always-on CI drift gate. Six requirements remain unavailable. Diagnostic
lifecycle is a guarded subset limited to DEC-0001 code retirement/non-reuse.

## Verification

- `cargo xtask deprecation verify`
- `cargo xtask governance check-error-codes`
- `cargo xtask schema compatibility --from N-1 --to N`
- `cargo xtask support verify`
- `cargo xtask migration verify`
- `cargo xtask compatibility verify`
- `cargo test -p xtask --bin xtask deprecation_readiness --locked --offline`
- `cargo test --workspace --all-targets --locked --offline --quiet`
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`
- `cargo xtask ci verify`
- `cargo xtask governance check-all`
- `cargo xtask status verify`
- `cargo fmt --all -- --check`
- `git diff --check`

## Compatibility and deferral

The milestone adds governance evidence only. It changes no language,
diagnostic allocation, warning, schema, protocol, support state, target,
profile, package, CLI, Semantic ID, dependency, Unicode, compiler, or runtime
behavior. Parent `COMPAT-6504` remains `BlockedSpec` until a complete public
policy and cross-version evidence are accepted.
