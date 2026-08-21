# PLC-4801 Authority Audit — Placement Constraint Model

Status: BlockedSpec

Date: 2026-08-22

## Outcome

PLC-4801 proposes an explicit Placement constraint model, distinguishing
`requires gpu`, `prefers gpu`, `forbids remote`, `same_node_as X`, `near
BufferY`, and `fallback cpu`. The plan says placement is constraint solving,
not an arbitrary runtime guess, and declares a dependency on `RFC-H405`.

No placement syntax, constraint AST/Core field, solver, target policy, cost
model, fallback behavior, or placement protocol can be added yet. `RFC-H405`
is absent and no Accepted decision fixes device identity, capability facts,
buffer location, remote semantics, fallback legality, explainability, or cache
identity. Implementing placement now would invent source-visible semantics and
could silently move data or change effects.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:429-446` is a
  non-normative implementation plan and explicitly depends on RFC-H405. Its
  examples do not define grammar, type/effect meaning, solver completeness,
  device topology, buffer/region identity, remote boundaries, fallback,
  diagnostics, or cache/replay semantics.
- `docs/ROADMAP-1.0.md:381-431` requires Placement to be an explicit
  constraint, with capability discovery, fallback, device-missing behavior,
  cost information, explainability, and replayable decisions. It does not
  authorize Placement syntax or a solver before the G4 specification gates.
- `docs/SEMANTICS.md:1473-1480, 1858-1931` and
  `docs/LANGUAGE.md:946-975, 1347-1381` reserve Placement/device behavior
  while excluding Kernel/GPU/Native behavior from v0.0.1. Existing Seed
  location words or Resource examples are not a Placement contract.
- `GAP-KERNEL-DEVICE-001` is Open for Device Buffer ownership/address spaces,
  determinism, Placement fallback, and backend capability. `GAP-NATIVE-BACKEND-
  ABI-001` is Open for target/layout/ABI and cross-target support. Device,
  backend, and accelerator support entries remain Unsupported/Experimental.
- No `RFC-H405` or Accepted Placement authority exists. RFC-0013 is only a
  candidate Kernel topic; lower-authority plan text cannot substitute for it.

## Current implementation evidence

- The repository has no Placement grammar, AST/HIR/Typed-Core representation,
  constraint solver, topology model, capability predicate, buffer-location
  identity, fallback planner, explain/replay schema, or placement fixtures
  under `crates` or `tests`.
- No accepted rule fixes the distinction between hard and soft constraints,
  conflict/unsatisfiable behavior, remote/host/device effects, same-node and
  proximity meaning, buffer migration cost, ownership/aliasing, cancellation,
  resource limits, or deterministic tie-breaking.
- No accepted policy defines what target/driver/toolchain/device facts enter
  Placement or cache identity, how unavailable devices are rejected, how a
  fallback may alter cost but not semantics, or how decisions are explained,
  recorded, and replayed without exposing host paths and timestamps.
- No placement diagnostic allocation, public protocol, dependency, CLI
  command, target, or support claim is required or changed by this audit. The
  public CLI and source extension remain `ling` and `.ling`.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. Versioned Placement syntax and semantic model with hard/soft constraints,
   capability predicates, topology and device identity, buffer/address-space
   relations, remote boundaries, deterministic solver behavior, and conflict
   diagnostics.
2. Kernel/Device IR, ownership, transfer, synchronization, numeric, and Fault
   contracts that Placement may reference without duplicating semantics or
   permitting unchecked AST execution.
3. Fallback and rejection rules defining when CPU/device alternatives are
   legal, how effects and determinism are preserved, how cost and availability
   are reported, and how user intent (`requires` versus `prefers`) is honored.
4. Stable Placement decision, explain, replay, and cache schemas with
   canonical inputs, provenance, versioning, migration/corruption handling,
   privacy, and exclusion of paths, addresses, timestamps, allocation order,
   and unstable driver/debug text from Ling identity.
5. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for
   unsatisfiable/conflicting constraints, missing capability/device, illegal
   remote or buffer placement, fallback, cost-limit, resource, and Fault cases.
6. Offline positive/negative, topology, capability, conflict, fallback,
   determinism, source-map/Unicode, migration, explain/replay, cache, and
   CPU/device differential fixtures.

## Evidence and compatibility impact

The eventual Placement solver must be a deterministic constraint pass over
verified artifacts and must not be a heuristic runtime guess. It must preserve
program effects, ownership, numeric class, and Fault semantics across legal
fallbacks; decisions and explain output are evidence, not new source meaning.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

PLC-4801 implementation, syntax/Core fields, placement constraints and solver,
topology/capability model, fallback/cost/replay/explain schemas, cache inputs,
diagnostics, editor support, and public protocol claims remain deferred until
RFC-H405 (or an Accepted replacement) and the Kernel/Device IR, runtime,
ownership, numeric, Native/backend, and support-matrix authorities are Accepted.
