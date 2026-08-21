# TASK-2205 Authority Audit: Production Local Task Scheduler

## Outcome

`TASK-2205` is correctly recorded as `BlockedSpec`. The G2 plan proposes a
correctness-first local scheduler with a fixed worker pool, work queue,
wake/park, cancellation, queryable Task trees, per-scope task/resource limits,
and metrics that do not affect program semantics; work stealing is explicitly
deferred. No accepted Task runtime or scheduler contract defines these
behaviors.

No worker-pool runtime, queue, wake/park primitive, production cancellation
path, Task-tree query, per-scope quota, metrics surface, scheduler diagnostic,
threading dependency, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative; its scheduler checklist does not
  authorize worker behavior, a runtime thread ABI, or an observable Task-tree
  protocol.
- TASK-2201 through TASK-2204 are `BlockedSpec`, and
  `GAP-STRUCTURED-TASK-001` leaves parent/child lifetime, suspension,
  cancellation, cleanup, Fault, and deterministic-scheduler behavior open.
  RFC-0008/RFC-C202 is not Accepted.
- Accepted DEC-0019/DEC-0021 authorize only internal compiler query scheduling
  over immutable data. They do not authorize Ling Task workers, wake/park,
  production cancellation, quotas, metrics, or runtime observability.
- `docs/ROADMAP-1.0.md` requires a Task scheduler and resource behavior for
  v0.2, but does not define queue fairness, worker shutdown, CPU-count
  independence, Task-tree identity, quota precedence, or production
  nondeterminism classes.
- RFC-0020 defines host-owned cancellation for the existing VM entry point
  only. It does not authorize source Task cancellation propagation, scheduler
  threads, shutdown, cleanup, or a runtime control protocol.

## Current implementation evidence

- `ling-eval` is a synchronous checked-Seed interpreter with no worker pool,
  queue, parked task, Task tree, or scope quota. `ling-vm` executes verified
  Seed bytecode with bounded frames/steps and host capabilities, not Task work.
- The repository's accepted query scheduling decisions are internal compiler
  boundaries and explicitly exclude runtime scheduling and structured Task
  cancellation. Reusing them would conflate compiler jobs with source Tasks.
- No runtime metrics, worker lifecycle, wake/park, quota, shutdown, or
  production scheduler protocol exists in the current CLI, VM, diagnostics,
  schemas, or tests.
- No fixture covers worker exhaustion, queue ordering, wake/park races,
  cancellation during shutdown, scope quota precedence, orphan cleanup,
  metrics noninterference, or interpreter/VM scheduler equivalence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the Task runtime input and worker ownership model, queue and wake/park
   semantics, fairness and starvation class, worker-pool sizing, shutdown and
   restart behavior, and distinction between production nondeterminism and
   language-observable Effects;
2. cancellation topology and checkpoints, scope/task-tree identity and query
   boundary, join/cleanup/orphan behavior, and race precedence among normal
   completion, Fault, cancellation, timeout, and shutdown;
3. per-scope task and resource quotas, allocation/step/worker limits, failure
   precedence, backpressure, host-panic/deadlock containment, and whether
   metrics are test-only, internal, or a versioned public protocol;
4. interpreter reference semantics and VM/runtime ABI, scheduler/host
   capability boundaries, diagnostics, source/provenance and Semantic IDs,
   Audit Source, schema/version migration, security and offline dependency
   policy; and
5. executable positive/negative/migration/differential fixtures for fixed and
   empty pools, queue/wake/park, nested scopes, cancellation and shutdown,
   quota exhaustion, child Fault/cleanup, orphan rejection, metrics
   noninterference, host resource bounds, Unicode/CRLF/BOM spans, and no
   unchecked-AST execution.

Until these decisions are Accepted, worker timing or CPU count could leak into
program behavior, cancellation could strand work, quotas could be applied in
the wrong order, or shutdown could leave an orphan Task or resource leak.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0013, DEC-0019, DEC-0021,
DEC-0018, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current database, evaluator, bytecode, VM, CLI, diagnostics, and test
crates.

No compiler, interpreter, VM, bytecode, scheduler, threading dependency,
diagnostic, schema, Semantic ID, source-span, runtime, or Unicode 17.0.0
behavior changed.

## Intentionally deferred

`TASK-2205` can begin only after TASK-2201 through TASK-2204 and an Accepted
RFC-0008 (or replacement) resolve `GAP-STRUCTURED-TASK-001` and define the
production scheduler boundary. The future implementation must consume checked
Task state machines only, keep metrics nonsemantic, enforce explicit scope
limits and cleanup, and publish worker-pool/VM differential evidence without
promising a production scheduling order as language behavior.
