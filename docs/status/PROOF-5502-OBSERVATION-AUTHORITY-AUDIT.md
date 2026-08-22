# PROOF-5502-OBSERVATION Authority Audit — Independent Checker Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0200` permits only test-local independent-checker vocabulary.
It does not authorize a checker, parser, certificate/query format, trusted
kernel, TCB registry, result schema, command, diagnostic, protocol, or
support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:400-410` is a
  non-normative independent-checker checklist.
- `docs/status/PROOF-5502-AUTHORITY-AUDIT.md` records missing Proof IR,
  certificate, kernel, TCB, result, and evidence authority.
- RFC-K505/K507 are absent or unresolved; `GAP-CRITICAL-PROFILE-001` remains
  open and `PROTO-EVIDENCE` is Future.

## Current implementation evidence

The observation adds one isolated test with sixty explicit independent-
checker, Proof IR, certificate, result, resource, TCB, provenance,
diagnostic, and fixture boundaries. It sorts by explicit local rank, rejects
duplicates, compares canonical opaque bytes for forward/reverse input order,
and uses an observation-only tag. No checker, decoder, schema, command,
diagnostic, protocol, dependency, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define the versioned Proof IR and certificate/query
envelope, kernel/checker and soundness/TCB boundary, bounded deterministic
offline checking, result states and fail-closed malformed/unknown/timeout
behavior, machine-readable output and exit codes, stable IDs/spans and
provenance, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, fuzz/replay and
cross-checker evidence, and offline fixtures. Seed behavior, Semantic IDs,
UTF-8 spans, dependencies, and Unicode 17.0.0 remain unchanged.

## Deferred work

PROOF-5502 implementation, checker/parser/decoder, certificates, kernel,
TCB/result schema, command, diagnostics, evidence protocol, fixtures beyond
boundary evidence, and public support remain deferred until accepted
authority and executable offline evidence exist. No placeholder checker API
or `zero-proof-check` command is created.
