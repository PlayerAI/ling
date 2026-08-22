# MC-5603-OBSERVATION Authority Audit — Model-Check Report Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0204` permits only test-local model-check report vocabulary. It
does not authorize a report enum/schema, result validity rules,
counterexample payload, exit-code contract, diagnostic, protocol, or support
claim. Bounded absence is explicitly paired with non-proof and prohibited
safety-claim markers.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:458-469` is a
  non-normative report checklist.
- `docs/status/MC-5603-AUTHORITY-AUDIT.md` records missing report, result,
  counterexample, proof, replay, and evidence authority.
- RFC-K501/K502/K504/K505/K506/K507 are absent or unresolved;
  `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.

## Current implementation evidence

The observation adds one isolated test with sixty explicit result, identity,
bound, resource, provenance, counterexample, diagnostic, and fixture
boundaries. It sorts by explicit local rank, rejects duplicates, compares
canonical opaque bytes for forward/reverse input order, and uses an
observation-only tag. The bounded-absence test requires `BoundedNonProof` and
`SafetyClaimProhibited`. No report, schema, diagnostic, protocol, dependency,
or support claim is introduced.

## Required authority and compatibility

Accepted authority must define versioned canonical report/result schemas,
precise result validity and fail-closed behavior, model/property/bound/
assumption/counterexample identities, resource and tool disclosure,
counterexample/replay/proof/evidence linkage, stable IDs/spans, provenance and
migration, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, exit codes, and
offline fixtures for every state. It must prohibit presenting bounded absence
as global safety proof. Seed behavior, Semantic IDs, UTF-8 spans,
dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

MC-5603 implementation, report/result/counterexample schemas, validity and
exit semantics, diagnostics, protocols, and public support remain deferred
until accepted authority and executable offline evidence exist. No
placeholder report API is created.
