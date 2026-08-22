# BND-5201-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0181` as test-only evidence in
`crates/ling-types/tests/bound_types_expressions_evidence.rs`. The test records
sixty provisional Bound/type/expression boundaries, sorts them by explicit local
rank, rejects duplicates, and compares canonical opaque bytes for forward/
reverse input order.

## Verification

- `cargo test -p ling-types --test bound_types_expressions_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No Bound syntax, AST/HIR/Typed-Core node, solver, profile parameter, diagnostic
allocation, dependency, target, CLI/LSP option, runtime, or Unicode behavior
changed. Existing implementation safety limits remain unchanged; public
BND-5201 remains `BlockedSpec`.

## Deferred work

Bound grammar/types, arithmetic and symbolic evaluation, proof/resource states,
profile/target integration, diagnostics, fixtures beyond boundary evidence, and
public support remain open.
