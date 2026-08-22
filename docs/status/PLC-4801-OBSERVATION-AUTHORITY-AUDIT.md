# PLC-4801-OBSERVATION Authority Audit — Placement-Constraint Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0172` permits only test-local Placement boundary vocabulary.
It does not authorize Placement syntax, an AST/HIR/Typed-Core field, a
constraint solver, device topology, capability facts, fallback, explain/replay,
cache semantics, diagnostics, or a public protocol.

## Traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:429-446` is a
  non-normative plan and depends on RFC-H405.
- `docs/ROADMAP-1.0.md:381-431` requires explicit and explainable Placement
  decisions but does not define their source or runtime semantics.
- `docs/status/PLC-4801-AUTHORITY-AUDIT.md` records the missing RFC-H405,
  Kernel/Device, ownership, Native/backend, and support authority.
- `DEC-0171` and `DEC-0170` remain prerequisite test-local evidence only.

## Current implementation evidence

The observation adds one isolated test and no production Placement surface.
The inventory has sixty explicit boundaries, deterministic local ordering,
duplicate rejection, and an opaque observation tag. No source grammar,
dependency, target, cache/runtime API, diagnostic, CLI/LSP command, or support
claim is introduced.

## Required authority and compatibility

Accepted authority must define hard/soft constraints, capability and topology
identity, buffer/address-space ownership, transfer/synchronization, remote
semantics, deterministic solving, rejection/fallback legality, cost and user
intent, explain/replay/cache schemas, migration/corruption/privacy rules,
bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and offline fixtures. Seed
behavior, Semantic IDs, UTF-8 spans, CLI, dependencies, and Unicode 17.0.0
remain unchanged.

## Deferred work

PLC-4801 implementation, solver behavior, syntax/Core fields, target policy,
fallback/cost/replay/explain/cache protocols, diagnostics, editor support, and
public Placement claims remain deferred until RFC-H405 or an Accepted
replacement and its prerequisite Kernel/Device and backend authorities exist.
