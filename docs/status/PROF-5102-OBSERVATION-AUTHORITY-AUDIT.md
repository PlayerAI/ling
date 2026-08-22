# PROF-5102-OBSERVATION Authority Audit — Forbidden-Capability Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0178` permits only test-local forbidden-capability vocabulary.
It does not authorize a Critical policy, capability/effect checker,
pre-lowering rejection pass, profile semantics, diagnostics, CLI option, or
support claim.

## Traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:108-120` is a
  non-normative implementation list.
- `docs/ROADMAP-1.0.md:118` requires explicit Critical boundaries but does
  not define checker semantics.
- `docs/status/PROF-5102-AUTHORITY-AUDIT.md` records open profile,
  ownership, concurrency, Kernel/Device, Native/ABI, numeric, and Fault
  authority.
- `DEC-0177` remains prerequisite Profile evidence only.

## Current implementation evidence

The observation adds one isolated test with sixty explicit boundaries,
deterministic local ordering, duplicate rejection, and an opaque observation
tag. No production checker, dependency, target, policy, diagnostic, CLI/LSP
option, runtime, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define capability/effect taxonomy and profile policy,
checked Typed-Core input and phase/transitive checking, bounds/topology/
numeric/Fault/FFI rules, Forbidden/Unavailable/Assumed/RuntimeChecked/
Proved/Experimental states, conflict/migration, bilingual
`L-<DOMAIN>-<NUMBER>` diagnostics, and offline fixtures. Seed behavior,
Semantic IDs, UTF-8 spans, CLI, dependencies, and Unicode 17.0.0 remain
unchanged.

## Deferred work

PROF-5102 implementation, policy/checker, boundedness/topology checks,
profile enforcement, diagnostics, CLI/LSP/editor integration, and support
claims remain deferred until RFC-0012/RFC-K501 (or an Accepted replacement),
the Critical Profile and ownership/concurrency/Kernel/Native authorities, and
executable fixtures are accepted.
