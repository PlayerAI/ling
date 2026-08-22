# CTR-5403-OBSERVATION Authority Audit — Contract Runtime-Check Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0194` permits only test-local Contract runtime-check
vocabulary. It does not authorize an evaluator, runtime hook, check order,
effect isolation rule, Contract Fault, profile gate, schema, diagnostic, or
support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:331-345` is a
  non-normative runtime-check checklist.
- `docs/status/CTR-5403-AUTHORITY-AUDIT.md` records the absent checked Core,
  exact timing/order, isolation, Fault/status, profile, identity, and evidence
  contracts.
- `GAP-CRITICAL-PROFILE-001` remains open; RFC-K503 and a Contract
  proof/runtime/evidence replacement are not Accepted.

## Current implementation evidence

The observation adds one isolated test with sixty explicit Contract
runtime-check input, assertion-boundary, effect, Fault/status, profile,
evidence, diagnostic, and fixture boundaries. It sorts by explicit local
rank, rejects duplicates, compares canonical opaque bytes for forward/reverse
input order, and uses an observation-only tag. No evaluator, runtime hook,
Fault, profile switch, diagnostic, CLI/LSP action, dependency, or support
claim is introduced.

## Required authority and compatibility

Accepted authority must define checked Contract Core input, binding and
evaluation order, exact call/return/invariant/instance boundaries,
effect/atomicity/committed-state rules, Fault/status/provenance and stable
IDs/spans, profile and Critical non-weakening policy, evidence/replay/schema
compatibility, stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and
offline fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and
Unicode 17.0.0 remain unchanged.

## Deferred work

CTR-5403 implementation, Contract evaluator/runtime hook, check semantics,
effect isolation, Fault/status, profile controls, evidence schema,
diagnostics, CLI/LSP/protocols, and support claims remain deferred until
accepted authority and executable offline evidence exist. No placeholder
runtime-check API is created.
