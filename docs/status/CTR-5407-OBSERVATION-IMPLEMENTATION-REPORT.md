# CTR-5407-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0198` as test-only evidence in
`crates/ling-types/tests/contract_lsp_zed_evidence.rs`. The test records sixty
provisional Contract editor, protocol, snapshot/transaction, position,
data-validity, diagnostic, and fixture boundaries. It sorts them by explicit
local rank, rejects duplicates, and compares canonical opaque bytes for
forward/reverse input order.

## Verification

- `cargo test -p ling-types --test contract_lsp_zed_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No LSP method, Contract projection, JSON schema, proof/evidence/counterexample
link, rename transaction, diagnostic allocation, dependency, CLI/LSP action,
Zed extension, protocol, support claim, or Unicode behavior changed. Public
`CTR-5407` remains `BlockedSpec`.

## Deferred work

LSP/editor protocol, Contract data projection, snapshot/transaction/position
rules, diagnostics, client fixtures beyond boundary evidence, Zed
integration, and public support remain open.
