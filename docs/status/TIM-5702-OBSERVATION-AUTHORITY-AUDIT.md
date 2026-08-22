# TIM-5702-OBSERVATION Authority Audit — Timing-Analysis Separation Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0207` permits only test-local timing-analysis separation
vocabulary. It does not authorize a timing status enum, measurement API,
estimate, static bound, proof, WCET result, analyzer, evidence schema,
diagnostic, protocol, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:489-501` is a
  non-normative label list and safety warning.
- `docs/status/TIM-5702-AUTHORITY-AUDIT.md` records missing result, sampling,
  soundness, target, identity, failure, and evidence authority.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.
- Accepted `DEC-0206` provides only test-local predecessor vocabulary, not a
  Timing IR or WCET representation.

## Current implementation evidence

The observation adds one isolated test with sixty explicit result, separation,
sampling, uncertainty, target, identity, provenance, failure, diagnostic, and
fixture boundaries. It sorts by explicit local rank, rejects duplicates,
compares canonical opaque bytes for forward/reverse input order, and keeps an
observed maximum distinct from the WCET-claim exclusion. No timing enum,
measurement, analyzer, schema, diagnostic, protocol, dependency, or support
claim is introduced.

## Required authority and compatibility

Accepted authority must define a versioned canonical result schema and closed
status transitions; measurement/estimate/static-bound/proof/assumption
separation; sampling, uncertainty, calibration and soundness rules; target and
TCB identity; Timing IR/path and source linkage with stable Semantic IDs and
UTF-8 spans; independent verification and fail-closed behavior; bilingual
`L-<DOMAIN>-<NUMBER>` diagnostics; and offline calibration/target/determinism
fixtures. Seed behavior, Semantic IDs, dependencies, and Unicode 17.0.0 remain
unchanged.

## Deferred work

TIM-5702 implementation, measurement/instrumentation, estimation and static
analysis, result transitions, WCET claims, schemas, diagnostics, protocols,
and public support remain deferred until Accepted authority and executable
offline evidence exist. No placeholder timing-result API is created.
