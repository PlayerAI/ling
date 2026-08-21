# NODE-5303 Authority Audit — Static Scheduling

Status: BlockedSpec

Date: 2026-08-22

## Outcome

NODE-5303 proposes generating a topological or statically cyclic schedule for a
Node graph, covering dependency analysis, rate/clock compatibility, multi-rate
bridges, priority/period, release/deadline, overrun policy, and a scheduler
manifest.

No Accepted RFC-K502 or replacement defines the Node schedule, clock model,
graph ordering, bridge semantics, release/deadline behavior, overrun policy,
manifest schema, or target/scheduler evidence. The repository's deterministic
single-threaded scheduler decision is an internal incremental-query scheduler,
not a Ling Node runtime contract. Implementing this task now would invent
observable timing and Fault behavior.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:242-252` is a
  non-normative plan fragment. It names schedule outputs but defines no graph
  identity, ordering tie-breakers, clock/rate units, bridge state, priority
  policy, release semantics, deadline admission, overrun transition, or
  manifest version/compatibility rules.
- `docs/SEMANTICS.md:1380-1425` gives only a conceptual Node tick/deadline
  outline; v0.0.1 implements neither Node nor a scheduler. The Node reserve at
  `:1914-1931` forbids silently exposing the feature.
- `docs/ROADMAP-1.0.md:441-466` requires Node timing/Fault semantics,
  boundedness, virtual-clock evidence, and a target-bound Critical Profile;
  it does not authorize static scheduling before the accepted gates close.
- `GAP-CRITICAL-PROFILE-001` leaves Node timing/Fault semantics, boundedness,
  Critical boundaries, and evidence Open. `GAP-STRUCTURED-TASK-001`,
  `GAP-ACTOR-AWAIT-REENTRY-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, and
  `GAP-DETERMINISTIC-REPLAY-001` leave cancellation, reentry, queue behavior,
  ordering, and replay classes unresolved.
- `docs/decisions/0019-incremental-query-boundary.md:39-49` specifies a
  deterministic single-threaded scheduler for internal compiler queries. It
  does not define Node release/deadline scheduling, priority, clock domains,
  overrun Faults, or a public scheduler manifest.
- RFC-K502 is only a plan label; no RFC-K502 or Accepted replacement, Node
  scheduler protocol, or scheduler manifest is present in the repository.

## Current implementation evidence

- The compiler has no Node graph, static schedule, dependency/rate analysis,
  multi-rate bridge, priority/release/deadline calculator, overrun policy, or
  scheduler manifest under `crates` or `tests`.
- No deterministic tie-break rule exists for independent nodes, same-period
  releases, clock conversion, feedback cycles, or priority conflicts. No
  semantics fixes bridge buffering, interpolation/decimation, state ownership,
  or event loss.
- There is no admission or schedulability proof, WCET/target/toolchain model,
  interrupt/preemption assumption, static-memory interaction, or schedule
  identity/migration rule.
- Existing `ling-db` query scheduling and VM instruction limits are internal or
  host-safety behavior; neither is a Ling Node schedule or real-time guarantee.
- No stable bilingual diagnostic or schema fixes incompatible rates/clocks,
  cyclic dependencies, missed release/deadline, overrun, priority conflict,
  unsupported bridge, or manifest mismatch.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Node graph and schedule model with stable node/edge identity,
   topological and legal cyclic forms, deterministic tie-breakers, schedule
   serialization, and source-span/Semantic-ID provenance.
2. Clock, period, rate, phase, release, deadline, jitter, and multi-rate bridge
   units and semantics, including buffering, interpolation/decimation,
   backpressure, loss, and state ownership.
3. Priority, preemption, cooperative execution, admission/schedulability,
   WCET, target/compiler/toolchain, interrupt/cache/bus assumptions, and the
   boundary between evidence and language semantics.
4. Overrun, missed release, deadline failure, cancellation, restart, Fault,
   fallback, and recovery transitions, including bounded memory/queue/task/
   actor effects and deterministic replay behavior.
5. Critical Profile and Node/Actor/Task/Kernel/Device/Native interactions,
   unsupported-target behavior, and scheduler-manifest version/migration/
   compatibility rules; no internal query scheduler may be reused as a public
   protocol without authority.
6. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for graph/schedule rejection, clock/rate mismatch, bridge overflow,
   priority conflict, unschedulable paths, deadline/overrun, target mismatch,
   and manifest incompatibility.
7. Offline executable graph/schedule, rate/clock, bridge, cycle, priority,
   release/deadline, overrun/Fault, target/compiler, migration, virtual-clock,
   Unicode/CRLF/BOM, replay, determinism, and interpreter/VM/Native
   differential fixtures with bounded output and resource use.

## Evidence and compatibility impact

The eventual implementation must consume Node Checked Core only after the
schedule authority is accepted. Schedule identity and evidence must be
deterministic and target-bound while excluding host paths, wall-clock timing,
thread interleavings, addresses, allocator details, hash order, and debug text
from Ling identity. Diagnostics must preserve original UTF-8 spans and
Semantic IDs and distinguish analysis results from runtime observations.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, internal query scheduler, Node runtime, Task, Actor,
Native backend, memory or ownership behavior, diagnostics, schemas, Semantic
IDs, source spans, CLI, LSP, dependency lock, target/toolchain support claim,
or Unicode 17.0.0 behavior.

## Intentionally deferred

NODE-5303 implementation, graph/schedule analysis, multi-rate bridges,
schedulability/WCET evidence, scheduler manifest, diagnostics, CLI/LSP and
runtime protocols, and support claims remain deferred until RFC-K502 (or an
Accepted replacement), `GAP-CRITICAL-PROFILE-001`, the concurrency/mailbox and
replay authorities, and the dependent BND/ownership/Native/Device decisions
are resolved with independent offline fixtures. No placeholder scheduler,
manifest, or public API is created.
