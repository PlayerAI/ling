# CTR-5405-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0196` as test-only evidence in
`crates/ling-types/tests/solver_proof_checker_evidence.rs`. The test records
sixty provisional solver/query, certificate, checker/trust, result,
identity, provenance, diagnostic, and fixture boundaries. It sorts them by
explicit local rank, rejects duplicates, and compares canonical opaque bytes
for forward/reverse input order.

## Verification

- `cargo test -p ling-types --test solver_proof_checker_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No solver dependency, proof checker, query/certificate format, TCB registry,
evidence schema, diagnostic allocation, dependency, CLI/LSP action, protocol,
soundness claim, or Unicode behavior changed. Public `CTR-5405` remains
`BlockedSpec`.

## Deferred work

Solver/query/certificate schemas, checker/soundness/TCB, timeout/unknown and
fail-closed rules, evidence, fixtures beyond boundary evidence, and public
support remain open.
