# GC-3302 Authority Audit — First Managed Collector

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

GC-3302 is a post-Seed runtime task. The execution plan asks the project to
choose a first collector and document pause behavior, root registration, cycle
handling, safe points, Task/Actor interaction, memory limits, metrics, and
stress/fuzz evidence. The choice is expressly an implementation choice, but
the boundaries it must honor are language and runtime contracts that are not
yet accepted.

No collector algorithm, Managed heap, scheduler hook, root registry, memory
limit API, metrics schema, OOM behavior, runtime protocol, or placeholder crate
is added. GC-3302 cannot become executable until GC-3301's object model and the
Managed/ownership and Task/Actor authorities are Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:256-269` is non-normative and
  explicitly describes GC-3302 as a first implementation after the Managed
  Runtime contract. It cannot select observable semantics or authorize a public
  collector API.
- GC-3301 is itself `BlockedSpec` because RFC-N303 and the RFC-0007
  Value/Managed/Resource decision are absent or not Accepted. Without the
  object model there is no authoritative root, edge, identity, allocation, or
  finalization input for a collector.
- `GAP-OWNERSHIP-MODEL-001` remains Open and leaves Managed roots/finalization,
  memory categories, cleanup, and Profile boundaries unresolved. The plan's
  RFC-N303 gate is therefore not satisfied by the design sketches in
  `docs/SEMANTICS.md` or `docs/LANGUAGE.md`.
- `GAP-STRUCTURED-TASK-001` and `GAP-ACTOR-AWAIT-REENTRY-001` leave Task/Actor
  lifetime, suspension, cancellation, reentry, and cleanup ordering
  unresolved. A safe point or collection pause cannot silently define those
  interactions.
- Accepted DEC-0013 defines the existing compile/host/internal/runtime-fault
  separation, and accepted VM cancellation decisions remain VM-host behavior;
  neither specifies a Managed allocator limit, collector pause, or public OOM
  Fault. RFC-0017 and DEC-0009 authorize only the Seed Place/mutable-place
  slice.
- v0.0.1 reserves Managed GC and does not expose a collector. A test-only
  allocation counter or Rust garbage collector would not establish Ling
  semantics.

## Current implementation evidence

- The workspace has no Managed-runtime or collector crate and no object model,
  root registration, reachability traversal, safe-point protocol, pause event,
  heap limit, collector metric, or OOM boundary.
- The current evaluator, bytecode, and VM execute the accepted Seed Typed Core
  synchronously. Host cancellation and runtime error paths do not provide a
  Task/Actor scheduler or Managed collection hook.
- Existing tests cover Seed values, effects, diagnostics, bytecode/VM
  equivalence, and bounded host behavior. They do not provide Managed roots,
  cycles, collector pauses, memory-limit semantics, or future stress/fuzz
  oracles.
- Rust allocation, addresses, timing, thread scheduling, destructor behavior,
  and map iteration are deliberately not Ling-observable. Adding a collector
  around them without an accepted contract would leak host/runtime details.

## Required authority before implementation

RFC-N303 and the accepted RFC-0007 memory model must define the following
before a collector can be selected or integrated:

1. The GC-3301 object and root model, including reachability, cycles, identity,
   weak/finalization behavior, and the boundary between Managed collection and
   deterministic Resource Drop.
2. The implementation-only latitude for choosing a collector, plus the
   language-visible pause/safepoint guarantees, progress/fairness constraints,
   thread/attachment rules, and prohibition on making collector strategy a
   Semantic ID or source-level behavior.
3. Root registration and deregistration for stack frames, closures, globals,
   Tasks, Actors, callbacks, and Native Islands; safe-point placement; and
   behavior when a root is created, dropped, cancelled, detached, restarted, or
   crosses a suspension boundary.
4. Cycle handling, write-barrier requirements, mutation ordering, and the
   interaction with future ownership/region/borrow and pinning rules. A
   collector must not infer legality from Rust references or permit raw pointer
   escape.
5. Memory limits and allocation failure: deterministic bounds, retry/recovery,
   cancellation/shutdown ordering, bilingual diagnostics and registered error
   identity, Fault payload/schema, and interpreter/VM/Native equivalence.
6. Metrics and pause reporting: whether any information is private diagnostic
   telemetry or a versioned protocol, its ordering and resource bounds, and the
   rule that wall-clock time, addresses, thread interleavings, and allocator
   layout are not semantic inputs.
7. Task/Actor and Profile contracts, including Explore, Native Managed Island,
   Critical restrictions, FFI callbacks, and replay/determinism implications.

## Evidence and compatibility impact

The future vertical slice needs positive and negative root-registration cases,
cycle and unreachable-object cases, safe-point and pause traces, Task/Actor
cancel/restart/shutdown cases, bounded memory-limit/OOM cases, metric-schema
fixtures (if exposed), and stress/property/fuzz evidence. Repeated runs must
use deterministic seeds and bounds and compare checked results and permitted
traces across interpreter, VM, and Native implementations without depending on
host addresses, timing, allocator order, or map order.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, diagnostic registry, schema, Semantic ID, source span, runtime,
or Unicode behavior. It allocates no error code and adds no collector or
metrics protocol. Existing Seed tests and offline build/test behavior remain
unchanged.

## Intentionally deferred

Collector algorithm selection, heap and root registry, pause/safepoint policy,
cycle collector, Task/Actor integration, memory limits, OOM Faults, metrics,
stress/fuzz targets, and Profile-specific runtime behavior remain deferred
until GC-3301, RFC-N303/RFC-0007, and the structured concurrency authorities
are Accepted and executable evidence is registered.
