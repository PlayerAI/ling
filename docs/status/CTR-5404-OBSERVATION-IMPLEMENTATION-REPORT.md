# CTR-5404-OBSERVATION Implementation Report

## Scope

Implemented Accepted `DEC-0195` as test-only evidence in
`crates/ling-types/tests/contract_vc_evidence.rs`. The test records sixty
provisional Proof IR/VC identity, control-flow, arithmetic/memory/effect,
assumption/certificate, boundedness, evidence, diagnostic, and fixture
boundaries. It sorts them by explicit local rank, rejects duplicates, and
compares canonical opaque bytes for forward/reverse input order.

## Verification

- `cargo test -p ling-types --test contract_vc_evidence --locked --offline`
- `cargo clippy -p ling-types --all-targets --locked --offline -- -D warnings`

No Proof IR, VC generator, Contract-to-VC lowering, assumption registry,
solver/checker, evidence schema, diagnostic allocation, dependency, CLI/LSP
action, protocol, soundness claim, or Unicode behavior changed. Public
`CTR-5404` remains `BlockedSpec`.

## Deferred work

Proof grammar/translation, arithmetic/alias/effect rules, boundedness and
soundness, TCB/assumptions, solver/checker, evidence, fixtures beyond
boundary evidence, and public support remain open.
