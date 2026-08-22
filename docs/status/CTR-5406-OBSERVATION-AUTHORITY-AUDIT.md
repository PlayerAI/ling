# CTR-5406-OBSERVATION Authority Audit — Contract Optimizer Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0197` permits only test-local Contract optimizer vocabulary.
It does not authorize an optimizer pass, status trust/admission model,
transformation contract, proof/assumption reader, invalidation policy,
diagnostic, or support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:371-373` is a
  non-normative optimizer checklist.
- `docs/status/CTR-5406-AUTHORITY-AUDIT.md` records absent Contract/proof/
  profile/evidence and optimizer authorities.
- `GAP-CRITICAL-PROFILE-001` remains open and `PROTO-EVIDENCE` is Future;
  RFC-K503/K505 are not Accepted.

## Current implementation evidence

The observation adds one isolated test with sixty explicit optimizer
status/admission, transformation/preservation, invalidation, proof/evidence,
diagnostic, and fixture boundaries. It sorts by explicit local rank, rejects
duplicates, compares canonical opaque bytes for forward/reverse input order,
and uses an observation-only tag. No optimizer, pass, proof reader,
diagnostic, CLI/LSP action, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define status trust/admission, profile and Critical
gates, transformation catalogue and semantic preservation for Effects,
Faults, cleanup, resources, ownership, timing, Node/Task/Actor, numeric and
FFI behavior, stable IDs/spans, invalidation, fail-closed handling, stable
bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline optimized/
unoptimized differential fixtures. Seed behavior and Unicode 17.0.0 remain
unchanged.

## Deferred work

CTR-5406 implementation, optimizer passes, safety-check elimination,
proof/assumption/evidence readers, invalidation, diagnostics,
CLI/LSP/protocols, and support claims remain deferred until accepted
authority and executable offline evidence exist. No placeholder optimizer API
is created.
