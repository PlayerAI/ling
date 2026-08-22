# CTR-5406-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0197` as test-only evidence in
`crates/ling-types/tests/contract_optimizer_evidence.rs`. The test records
sixty provisional optimizer status/admission, transformation/preservation,
invalidation, proof/evidence, diagnostic, and fixture boundaries. It sorts
them by explicit local rank, rejects duplicates, and compares canonical
opaque bytes for forward/reverse input order.

## Verification

- `cargo test -p ling-types --test contract_optimizer_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No optimizer pass, safety-check elimination, proof/assumption reader,
invalidation implementation, evidence schema, diagnostic allocation,
dependency, CLI/LSP action, protocol, performance claim, or Unicode behavior
changed. Public `CTR-5406` remains `BlockedSpec`.

## Deferred work

Status trust/admission, transformations/preservation, invalidation,
proof/evidence, fixtures beyond boundary evidence, and public optimization
support remain open.
