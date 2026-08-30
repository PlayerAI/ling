# TASK-2205 Authority Audit: Production Local Task Scheduler

## Outcome

`TASK-2205` remains `BlockedSpec` pending acceptance of Proposed DEC-0268.
TASK-2201 through TASK-2204 are Done under Accepted DEC-0264 through DEC-0267,
so checked Task Core, state machines, the scheduler-neutral lifecycle runtime,
and deterministic test scheduling are complete dependencies. Proposed DEC-0268
isolates the remaining correctness-first local production boundary: an exact
checked `task main ()` interpreter entry, fixed worker pool, bounded central
queue, wake/park, cancellation and shutdown, internal Task-tree snapshots,
per-scope child preflight, and nonsemantic metrics. Work stealing is explicitly
deferred.

No worker-pool runtime, queue, wake/park primitive, production cancellation
path, Task-tree query, per-scope quota, metrics surface, scheduler diagnostic,
threading dependency, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative; its scheduler checklist does not
  authorize worker behavior, a runtime thread ABI, or an observable Task-tree
  protocol.
- TASK-2201 through TASK-2204 are Done. Accepted DEC-0264 through DEC-0267
  define checked Task source/Core, machines, scheduler-neutral lifecycle,
  cancellation/cleanup/Fault precedence, logical test deadlines, typed traces,
  strict replay, and bounded test exploration. They deliberately leave the
  production worker/queue/wake, shutdown, metrics, and public entry boundary to
  TASK-2205. RFC-0008/RFC-C202 is not Accepted.
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

- `ling-eval` now contains the checked DEC-0266 `TaskRuntime` and DEC-0267
  deterministic test driver, but no production worker pool, central work queue,
  condition-variable park/wake, host cancellation control, shutdown/join,
  production Task-tree snapshot, or metrics boundary. `ling-vm` still executes
  verified Seed/Handler bytecode, not Task work.
- The repository's accepted query scheduling decisions are internal compiler
  boundaries and explicitly exclude runtime scheduling and structured Task
  cancellation. Reusing them would conflate compiler jobs with source Tasks.
- No runtime metrics, worker lifecycle, wake/park, quota, shutdown, or
  production scheduler protocol exists in the current CLI, VM, diagnostics,
  schemas, or tests.
- No fixture covers worker exhaustion, queue ordering, wake/park races,
  cancellation during shutdown, scope quota precedence, orphan cleanup,
  metrics noninterference, or interpreter/VM scheduler equivalence.

## Proposed authority before implementation

Proposed DEC-0268 defines the following boundary, but it is not implementation
authority until Accepted:

1. checked-only runtime input; an exact `task main ()` file/project interpreter
   entry; explicit bounded worker configuration; a central mutex/condition-
   variable FIFO; serialized one-step runtime transitions; wake/park and
   shutdown/join rules; and an explicit production nondeterminism class;
2. cancellation topology and checkpoints, scope/task-tree identity and query
   boundary, join/cleanup/orphan behavior, and race precedence among normal
   completion, Fault, cancellation, timeout, and shutdown;
3. per-scope direct-child preflight plus runtime/worker/queue/transition/wake
   limits, failure precedence, host/worker-panic containment, internal bounded
   snapshots, and metrics that cannot affect scheduling or program results;
4. interpreter-only execution while test/build/REPL/artifact/bytecode/VM and
   editor routes retain `L-TASK-0004`, existing diagnostic reuse, unchanged
   Semantic IDs/Audit/schema bytes, and an offline `std` threading policy; and
5. executable positive/negative/migration/differential fixtures for fixed and
   empty pools, queue/wake/park, nested scopes, cancellation and shutdown,
   quota exhaustion, child Fault/cleanup, orphan rejection, metrics
   noninterference, host resource bounds, Unicode/CRLF/BOM spans, and no
   unchecked-AST execution.

Until these decisions are Accepted, worker timing or CPU count could leak into
program behavior, cancellation could strand work, quotas could be applied in
the wrong order, or shutdown could leave an orphan Task or resource leak.

## Evidence and compatibility

This refreshed audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0013, DEC-0019, DEC-0021,
DEC-0018, RFC-0001, RFC-0020, Accepted DEC-0264 through DEC-0267, Proposed
DEC-0268,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current database, evaluator, bytecode, VM, CLI, diagnostics, and test
crates.

No compiler, interpreter, VM, bytecode, scheduler, threading dependency,
diagnostic, schema, Semantic ID, source-span, runtime, or Unicode 17.0.0
behavior changed.

## Intentionally deferred

`TASK-2205` can begin when DEC-0268 is Accepted. The proposal deliberately
retains work stealing, public worker/metrics/Task-tree protocols, Task
test/build/REPL/artifact execution, Task bytecode/VM/native ABI, wall-clock
Clock/sleep, I/O wake injection, detach, user Resource finalizers, recoverable
allocation quotas, Replay, Actor crossing, migration, million-short-task
stress, and Stable scheduling compatibility for TASK-2206 or later authority.
