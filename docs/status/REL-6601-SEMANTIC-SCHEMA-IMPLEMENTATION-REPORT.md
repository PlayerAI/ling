# REL-6601-SEMANTIC-SCHEMA Implementation Report

## Scope

Added a bounded `semantic_schema_bytes` libFuzzer harness for both implemented
Semantic Graph readers, two reviewed corpus files, a normal regression test,
locked offline compilation, inventory drift checks, and Ubuntu CI replay.

The harness compares repeated reader outcomes and keeps decoded graphs
data-only. The inventory advances from eight harnesses/eighteen seeds to nine
harnesses/twenty seeds.

## Verification

- `cargo check --manifest-path fuzz/Cargo.toml --bins --locked --offline`
- `cargo test -p ling-semantic --test fuzz_corpus --locked --offline`
- `cargo xtask fuzz verify`
- `cargo xtask ci verify`
- `cargo test --workspace --all-targets --locked --offline --quiet`
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`
- `cargo xtask governance check-all`
- `cargo xtask status verify`
- `cargo fmt --all -- --check`
- `git diff --check`

The libFuzzer replay command is configured in the pinned Ubuntu CI job. This
Windows host performed the documented compile gate; it does not claim an MSVC
sanitizer run.

## Compatibility and deferral

No public schema, reader behavior, migration, Semantic ID, diagnostic, CLI,
package, runtime, Unicode, or released dependency changes. Parent `REL-6601`
remains blocked for future protocol harnesses and G6 release evidence.
