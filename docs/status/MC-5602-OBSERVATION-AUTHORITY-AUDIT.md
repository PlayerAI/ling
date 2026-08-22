# MC-5602-OBSERVATION Authority Audit — Exploration Engine Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0203` permits only test-local exploration-engine vocabulary. It
does not authorize an engine, state hash, work queue, BFS/DFS ordering,
partial-order reduction, result/counterexample schema, diagnostic, protocol,
or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:446-456` is a
  non-normative exploration checklist.
- `docs/status/MC-5602-AUTHORITY-AUDIT.md` records missing projected-model,
  traversal, reduction, state identity, resource, result, and evidence
  authority.
- RFC-K501/K502/K504/K505/K506/K507 are absent or unresolved;
  `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.

## Current implementation evidence

The observation adds one isolated test with sixty explicit exploration,
state/hash, traversal/reduction, bound/resource, result, provenance,
diagnostic, and fixture boundaries. It sorts by explicit local rank, rejects
duplicates, compares canonical opaque bytes for forward/reverse input order,
and uses an observation-only tag. No engine, state hash, reduction, schema,
diagnostic, protocol, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define the canonical projected model and state/event
bytes, hash/version/collision and ordering rules, BFS/DFS and reduction
semantics, explicit resource bounds and fail-closed incomplete/unknown
behavior, result/counterexample/replay schemas, stable IDs/spans and
provenance, bounded non-proof wording, bilingual
`L-<DOMAIN>-<NUMBER>` diagnostics, and offline interleaving/reduction/
resource fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies,
and Unicode 17.0.0 remain unchanged.

## Deferred work

MC-5602 implementation, state canonicalization/hash, work queues, BFS/DFS,
partial-order reduction, bounds/resources, result/counterexample schemas,
diagnostics, protocols, and public support remain deferred until accepted
authority and executable offline evidence exist. No placeholder engine API is
created.
