# PLC-4802-OBSERVATION Authority Audit — Placement-Selection Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0173` permits only test-local vocabulary for the proposed
Placement selection pipeline. It does not authorize static candidate filtering,
artifact preparation, runtime device matching, policy/cost selection,
decision/replay/cache protocols, profile semantics, diagnostics, or support.

## Traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:448-464` is a
  non-normative phase sketch and depends on RFC-H405.
- `docs/ROADMAP-1.0.md:381-431` requires explicit, explainable and replayable
  Placement decisions without defining selector semantics.
- `docs/status/PLC-4802-AUTHORITY-AUDIT.md` records missing Device IR,
  capability/target, cost, fallback, profile, replay, and diagnostic authority.
- `DEC-0172` remains test-local Placement constraint evidence only.

## Current implementation evidence

The observation adds one isolated test with sixty explicit boundaries,
deterministic local ordering, duplicate rejection, and an opaque observation
tag. No production selector, dependency, target, cache/runtime API,
diagnostic, CLI/LSP command, or support claim is introduced.

## Required authority and compatibility

Accepted authority must define verified artifacts and phase ordering, static
legality, runtime availability, target/capability/toolchain/topology identity,
policy/cost precedence, deterministic tie-breaking, fallback/rejection,
Critical/Strict/Native profiles, decision/replay/migration/cache schemas,
privacy rules, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline
fixtures. Seed behavior, Semantic IDs, UTF-8 spans, CLI, dependencies, and
Unicode 17.0.0 remain unchanged.

## Deferred work

PLC-4802 selector implementation, candidate artifacts, runtime matching,
policy/cost choice, decision/replay/explain/cache protocols, diagnostics,
editor support, and public Placement selection claims remain deferred until
RFC-H405 or an Accepted replacement and prerequisite backend authorities exist.
