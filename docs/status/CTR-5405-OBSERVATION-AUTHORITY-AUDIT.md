# CTR-5405-OBSERVATION Authority Audit — Solver/Proof Checker Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0196` permits only test-local solver/proof-checker vocabulary.
It does not authorize a solver adapter, query/certificate schema,
independent checker, soundness claim, TCB registry, evidence protocol,
diagnostic, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:360-369` is a
  non-normative solver/checker checklist.
- `docs/status/CTR-5405-AUTHORITY-AUDIT.md` records missing RFC-K505/K506/K507
  and unresolved query/certificate, checker, trust, TCB, and evidence rules.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.

## Current implementation evidence

The observation adds one isolated test with sixty explicit solver/query,
certificate, checker/trust, result, identity, provenance, diagnostic, and
fixture boundaries. It sorts by explicit local rank, rejects duplicates,
compares canonical opaque bytes for forward/reverse input order, and uses an
observation-only tag. No solver dependency, adapter, checker, schema,
diagnostic, CLI/LSP action, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define versioned proof/query/certificate schemas,
candidate-only solver trust, fixed identity/configuration, replay,
timeout/unknown/malformed/corrupt handling, independent checker and
soundness/TCB rules, evidence/provenance/checksum/signature/redaction,
migration, stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline
fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and
Unicode 17.0.0 remain unchanged.

## Deferred work

CTR-5405 implementation, solver adapter, query/certificate schema, checker,
TCB registry, evidence protocol, diagnostics, CLI/LSP/protocols, and support
claims remain deferred until accepted authority and executable offline
evidence exist. No placeholder solver/checker API is created.
