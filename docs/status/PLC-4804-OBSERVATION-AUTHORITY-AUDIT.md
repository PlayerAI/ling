# PLC-4804-OBSERVATION Authority Audit — Placement-Explain Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0175` permits only test-local vocabulary for the proposed
Placement explain output. It does not authorize a `ling` command, any stale
`zero` command, a machine-readable schema, decision/replay/cache protocol,
privacy filter, diagnostics, editor integration, or support claim.

## Traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:480-493` is
  non-normative and contains stale `zero` wording.
- `docs/ROADMAP-1.0.md:381-431` requires explainable Placement decisions but
  does not define a command or payload.
- `docs/status/PLC-4804-AUTHORITY-AUDIT.md` records missing explain, privacy,
  replay/cache, CLI, and diagnostic authority.
- `DEC-0174` remains prerequisite Cost Model evidence only.

## Current implementation evidence

The observation adds one isolated test with sixty explicit boundaries,
deterministic local ordering, duplicate rejection, and an opaque observation
tag. No production command, schema, dependency, target, cache/runtime API,
diagnostic, editor route, or support claim is introduced. The stale `zero`
name is excluded from code and fixtures.

## Required authority and compatibility

Accepted authority must define stable explain fields and ordering, provenance,
Semantic IDs/spans, rejection/transfer/numeric/fallback/cache/replay identity,
`ling` CLI transport, bilingual rendering, privacy/redaction, migration and
unknown-field behavior, diagnostics, and offline fixtures. Seed behavior,
Semantic IDs, UTF-8 spans, CLI, dependencies, and Unicode 17.0.0 remain
unchanged.

## Deferred work

PLC-4804 explain implementation, command/protocol/schema, decision rendering,
privacy, replay/cache, diagnostics, editor integration, and public Placement
claims remain deferred until RFC-H405 or an Accepted replacement and the
prerequisite Placement/Device/backend authorities exist.
