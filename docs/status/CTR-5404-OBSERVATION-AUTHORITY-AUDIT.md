# CTR-5404-OBSERVATION Authority Audit — Contract VC Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0195` permits only test-local Proof IR/VC vocabulary. It does
not authorize a Proof IR, VC generator, Contract-to-VC translator, soundness
claim, assumption registry, solver/checker adapter, evidence schema,
diagnostic, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:347-358` is a
  non-normative VC checklist.
- `docs/status/CTR-5404-AUTHORITY-AUDIT.md` records the absent RFC-K505 and
  unresolved translation, arithmetic/alias/effect, boundedness, TCB, and
  evidence contracts.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future;
  RFC-K505/RFC-K506 are not Accepted.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Proof IR/VC
identity, control-flow, arithmetic/memory/effect, assumption/certificate,
boundedness, evidence, diagnostic, and fixture boundaries. It sorts by
explicit local rank, rejects duplicates, compares canonical opaque bytes for
forward/reverse input order, and uses an observation-only tag. No proof IR,
translator, solver/checker, schema, diagnostic, CLI/LSP action, dependency,
or support claim is introduced.

## Required authority and compatibility

Accepted authority must define a versioned Proof IR/VC grammar and canonical
identity, Contract-to-VC translation, arithmetic/memory/alias/Effect rules,
soundness and boundedness non-claims, trusted assumptions/TCB, solver versus
checked certificate, fail-closed unknown/timeout behavior, deterministic
limits, evidence/replay/migration, stable bilingual
`L-<DOMAIN>-<NUMBER>` diagnostics, and offline fixtures. Seed behavior,
Semantic IDs, UTF-8 spans, dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

CTR-5404 implementation, Proof IR/VC, translation, assumption registry,
solver/checker, evidence schema, diagnostics, CLI/LSP/protocols, and support
claims remain deferred until accepted authority and executable offline
evidence exist. No placeholder proof API is created.
