# MC-5604-OBSERVATION Authority Audit — Replay Counterexample Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0205` permits only test-local replay-counterexample vocabulary.
It does not authorize a converter, replay schema, reader/writer, scheduler
trace, reference-runtime route, source-link protocol, diagnostic, or support
claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:471-473` is a
  non-normative replay proposal.
- `docs/status/MC-5604-AUTHORITY-AUDIT.md` records missing conversion,
  replay, scheduler, runtime, source-link, failure, and evidence authority.
- `GAP-DETERMINISTIC-REPLAY-001` and `GAP-CRITICAL-PROFILE-001` remain open;
  `PROTO-REPLAY` and `PROTO-EVIDENCE` are Future.

## Current implementation evidence

The observation adds one isolated test with sixty explicit conversion,
replay, scheduler, runtime, event, identity, source-link, failure, privacy,
diagnostic, and fixture boundaries. It sorts by explicit local rank, rejects
duplicates, compares canonical opaque bytes for forward/reverse input order,
and uses an observation-only tag. No converter, replay consumer, schema,
runtime route, diagnostic, protocol, dependency, or support claim is
introduced.

## Required authority and compatibility

Accepted authority must define checked model/result/counterexample conversion,
versioned replay bytes and identities, scheduler/event/time/input/effect and
Fault/restart semantics, reference-runtime behavior, stable Semantic IDs and
UTF-8 spans, provenance/privacy/redaction, fail-closed divergence/corruption/
unknown behavior, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline
replay/source-link/determinism fixtures. Seed behavior, Semantic IDs,
dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

MC-5604 implementation, converter, replay schema/reader/writer, scheduler
trace, reference-runtime route, source linkage, diagnostics, protocols, and
public support remain deferred until accepted authority and executable
offline evidence exist. No placeholder replay API is created.
