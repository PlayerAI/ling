# MC-5601-OBSERVATION Authority Audit — Finite-State Projection Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0202` permits only test-local finite-state projection
vocabulary. It does not authorize a projection IR, projection relation,
property language, state hash, model checker, result/counterexample schema,
diagnostic, protocol, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:431-444` is a
  non-normative finite-state projection checklist.
- `docs/status/MC-5601-AUTHORITY-AUDIT.md` records missing Node/Task/Actor,
  boundedness, projection, model-check, proof, and evidence authority.
- RFC-K501/K502/K504/K506/K507 are absent or unresolved;
  `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future.

## Current implementation evidence

The observation adds one isolated test with sixty explicit projection,
Task/Actor/Node, state, transition, bound, property, result, provenance,
diagnostic, and fixture boundaries. It sorts by explicit local rank, rejects
duplicates, compares canonical opaque bytes for forward/reverse input order,
and uses an observation-only tag. No projection, checker, schema, diagnostic,
protocol, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define checked concurrency source/Core semantics,
state/transition/mailbox/scheduler/Fault/restart/time/property rules,
canonical projection and stable IDs/spans, explicit bound dimensions and
non-proof claims, deterministic resource-bounded exploration, fail-closed
unknown/incomplete behavior, counterexample/replay and provenance linkage,
bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline interleaving
fixtures. Seed behavior, Semantic IDs, UTF-8 spans, dependencies, and Unicode
17.0.0 remain unchanged.

## Deferred work

MC-5601 implementation, projection IR/relation, property language, state
identity/hash, concurrency/bound semantics, result/counterexample schemas,
diagnostics, protocols, and public support remain deferred until accepted
authority and executable offline evidence exist. No placeholder model API is
created.
