# PROOF-5501-OBSERVATION Authority Audit — Proof IR Boundary Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0199` permits only test-local Proof IR vocabulary. It does not
authorize a Proof IR grammar, canonical representation, certificate/query
format, parser, proof kernel, checker, assumption registry, Contract
translation, diagnostic, protocol, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:386-398` is a
  non-normative Proof IR checklist.
- `docs/status/PROOF-5501-AUTHORITY-AUDIT.md` records missing proof grammar,
  certificate, kernel, assumption, Contract translation, and evidence
  authority.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future;
  RFC-K504/K505/K506/K507 remain absent or unresolved.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Proof IR, term,
theorem, axiom, provenance, Contract/Typed-Core, checking, evidence,
diagnostic, and fixture boundaries. It sorts by explicit local rank, rejects
duplicates, compares canonical opaque bytes for forward/reverse input order,
and uses an observation-only tag. No proof representation, parser, checker,
schema, diagnostic, protocol, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define a versioned grammar and canonical bytes,
stable Proof IDs/source spans, explicit Contract/Typed-Core translation,
kernel and soundness/TCB boundaries, bounded deterministic checking,
assumptions and provenance/invalidation, fail-closed malformed/unknown
behavior, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline proof and
evidence fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies,
and Unicode 17.0.0 remain unchanged.

## Deferred work

PROOF-5501 implementation, proof grammar, parser, certificates, kernel,
checker, assumptions, Contract translation, diagnostics, protocols, and
support claims remain deferred until accepted authority and executable
offline evidence exist. No placeholder Proof IR API is created.
