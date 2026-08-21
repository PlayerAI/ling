# PLC-4802 Authority Audit — Static Candidates and Runtime Selection

Status: BlockedSpec

Date: 2026-08-22

## Outcome

PLC-4802 proposes a placement pipeline: compile-time legality and capability
filter, artifact preparation, runtime available-device matching, policy/cost
choice, and decision recording. It also proposes fixed Placement for
Critical/Strict replay and recorded-then-replayed choices for ordinary Native.

No candidate filter, runtime selector, artifact-preparation pass, decision
record, replay schema, or policy API can be added yet. `RFC-H405` is absent and
the accepted contracts for capabilities, topology, target identity, Device IR,
cost, fallback, replay, Critical/Strict, and Native are unresolved. A selector
would otherwise turn host availability and heuristics into language-visible
behavior or an unstable cache/replay protocol.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:448-464` is a
  non-normative implementation plan. It sketches static filtering, runtime
  matching, cost/policy choice, recording, and replay but does not define
  legality, capability identity, artifact schema, available-device facts,
  policy precedence, determinism, replay compatibility, or error behavior.
- `docs/ROADMAP-1.0.md:381-431` requires Placement decisions to be explicit,
  explainable, recordable, and replayable, and requires fallback/device-missing
  behavior and support matrices. It does not authorize a selector or define
  when runtime facts may influence semantics.
- `docs/SEMANTICS.md:1473-1480, 1858-1931` and
  `docs/LANGUAGE.md:946-975, 1347-1381` reserve Placement/device behavior and
  exclude Kernel/GPU/Native from v0.0.1. Current Interpreter/VM scheduling is
  not a placement decision contract.
- `PLC-4801` is `BlockedSpec`; `DIR-4501` through `DIR-4503` and GPU/accelerator
  prerequisites are blocked. `GAP-KERNEL-DEVICE-001` leaves capability,
  determinism, buffers, and fallback unresolved; `GAP-NATIVE-BACKEND-ABI-001`
  leaves Native target/layout/ABI unresolved.
- No `RFC-H405` or Accepted replay/placement authority exists. The plan's
  Critical/Strict and Native wording cannot create those protocols.

## Current implementation evidence

- The repository has no compile-time Placement filter, candidate artifact
  model, runtime available-device matcher, policy/cost selector, decision
  recorder, replay reader/writer, or placement-selection fixtures under
  `crates` or `tests`.
- No accepted rule fixes the relation between static capability legality and
  runtime availability, target/feature versions, device identity, dynamic
  topology, buffer location, remote boundaries, policy precedence, cost,
  tie-breaking, retries, cancellation, or fallback effects.
- No accepted rule defines replay format/versioning, stale or unavailable
  device handling, cache invalidation, provenance/privacy, or how Critical and
  Strict profiles reject a changed environment while ordinary Native may
  replay a recorded choice.
- No placement diagnostic allocation, public protocol, dependency, CLI
  command, target, or support claim is required or changed by this audit. The
  public CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Placement and Device IR model defining static legality,
   capability predicates, target/device identity, buffer/address-space facts,
   remote boundaries, and the exact verified input to each phase.
2. Deterministic phase ordering and policy semantics for compile-time filter,
   artifact preparation, runtime match, cost/policy choice, fallback/rejection,
   conflict, unavailable device, cancellation, and resource/Fault behavior.
3. Profile-specific rules for Critical/Strict fixed placement and Native
   record/replay, including environment mismatch, stale decision, migration,
   privacy, and explicit failure rather than silent semantic drift.
4. Canonical decision, replay, explain, and cache schemas with provenance,
   target/capability/toolchain inputs, versioning, corruption handling, and
   exclusion of paths, addresses, timestamps, allocation order, and unstable
   driver/debug text from Ling identity.
5. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for static
   illegality, capability mismatch, unavailable device, policy conflict,
   replay mismatch, fallback, cost/resource limit, and Fault cases.
6. Offline positive/negative, topology, capability, policy, cost, fallback,
   replay, migration, determinism, source-map/Unicode, and CPU/device
   differential fixtures.

## Evidence and compatibility impact

The eventual selector must be a deterministic, explainable consumer of
verified artifacts. Runtime availability and cost may select only among
accepted legal choices; fallback must preserve effects, ownership, numeric
class, and Fault semantics. Decisions and replay records are evidence and
cache inputs, not a license to expose host state as Ling semantics.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

PLC-4802 implementation, static candidate filtering, artifact preparation,
runtime matching, policy/cost choice, decision recording, replay/explain/cache
schemas, diagnostics, editor support, and public protocol claims remain
deferred until RFC-H405 (or an Accepted replacement), PLC-4801, and the
Kernel/Device IR, runtime, ownership, numeric, Native/backend, and
support-matrix authorities are Accepted.
