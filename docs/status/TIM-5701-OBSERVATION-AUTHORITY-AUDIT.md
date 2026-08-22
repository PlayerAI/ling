# TIM-5701-OBSERVATION Authority Audit — Timing IR and Path Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0206` permits only test-local Timing IR/path vocabulary. It does
not authorize a Timing IR, target-cost model, path solver, WCET result,
reader/writer, deadline hook, diagnostic, protocol, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:477-487` is a
  non-normative timing-data checklist.
- `docs/status/TIM-5701-AUTHORITY-AUDIT.md` records missing representation,
  target, bound, interference, source-link, failure, and evidence authority.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.

## Current implementation evidence

The observation adds one isolated test with sixty explicit representation,
target, control-flow, bound, assumption, identity, source-link, failure,
diagnostic, and fixture boundaries. It sorts by explicit local rank, rejects
duplicates, compares canonical opaque bytes for forward/reverse input order,
and uses an observation-only tag. No Timing IR, analyzer, cost model, schema,
diagnostic, protocol, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define a versioned canonical Timing IR; target/profile/
build and path identity; bounds and their proof/assumption status; target cost
and interference models; checked representation and source-map linkage; stable
Semantic IDs and UTF-8 spans; provenance and failure behavior; measurement
versus static-analysis status; bilingual `L-<DOMAIN>-<NUMBER>` diagnostics;
and offline target/path/determinism fixtures. Seed behavior, Semantic IDs,
dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

TIM-5701 implementation, Timing IR, target-cost model, path solver, WCET
claims, schemas, diagnostics, protocols, and public support remain deferred
until Accepted authority and executable offline evidence exist. No placeholder
timing API is created.
