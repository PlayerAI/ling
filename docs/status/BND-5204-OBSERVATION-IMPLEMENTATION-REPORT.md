# BND-5204-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0184` as test-only evidence in
`crates/ling-types/tests/resource_budget_diagnostics_evidence.rs`. The test
records sixty provisional resource-budget diagnostic fact, proof/provenance,
schema/transaction, repair, and fixture boundaries, sorts them by explicit
local rank, rejects duplicates, and compares canonical opaque bytes for
forward/reverse input order.

## Verification

- `cargo test -p ling-types --test resource_budget_diagnostics_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No budget diagnostic code, schema, fact producer, proof state, repair,
Workspace Edit, Semantic Transaction, dependency, target, CLI/LSP action,
runtime, or Unicode behavior changed. Existing Preview diagnostics and VM
Runtime Faults remain unchanged; public BND-5204 remains `BlockedSpec`.

## Deferred work

Budget facts, code/schema allocation, proof/provenance semantics, localization
and migration, repair/transaction behavior, diagnostics, fixtures beyond
boundary evidence, and public support remain open.
