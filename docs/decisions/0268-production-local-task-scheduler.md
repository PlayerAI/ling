# DEC-0268: Production local Task scheduler / 生产本地 Task 调度器

> 状态：Proposed<br>
> 提出日期：2026-08-30<br>
> 决定日期：Pending<br>
> Owner role：concurrency-design<br>
> 相关 RFC/缺口：DEC-0266 | DEC-0267 | GAP-STRUCTURED-TASK-001 | TASK-2205<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal closes only the first correctness-oriented local production
scheduler and interpreter entry boundary of TASK-2205. It does not authorize
work stealing, detached Tasks, Task bytecode/VM/native ABI, wall-clock or sleep
syntax, public scheduler metrics or Task-tree protocols, cross-process work,
Replay, Actor crossing, or Stable scheduling compatibility.

本提案仅关闭 TASK-2205 第一版以正确性为先的本地生产调度器与解释器入口边界；不授权
work stealing、detach、Task bytecode/VM/native ABI、墙钟或 sleep 语法、公开的
scheduler metrics/Task-tree 协议、跨进程执行、Replay、Actor crossing 或 Stable
调度兼容性。

## Question

What is the smallest bounded local scheduler that may execute DEC-0264 through
DEC-0266 checked Tasks from an explicit public interpreter entry, use a real
fixed worker pool and queue without leaking worker timing into required Ling
results, and shut down without lost cleanup or orphan Tasks?

## Decision

1. **Execution input.** The local scheduler consumes only one successful
   `CheckedProgram`, its exact DEC-0264 root `CheckedTaskCore`, matching
   DEC-0265 machine, DEC-0266 `TaskRuntime`, validated arguments, injected host
   capabilities, and explicit limits. It never evaluates AST, unchecked HIR,
   source text, test-scheduler traces, bytecode, or VM state.

2. **Task entry.** The only public Task entry in this slice is a declaration
   named `main` in module `Main` with exactly the unit parameter and unit result:
   `task main () = scope ... return ()`. File and project `ling run` select this
   checked Task entry when no ordinary `let main ()` exists. Duplicate or
   otherwise invalid entry declarations fail through the existing checked
   entry diagnostics before scheduler construction. Ordinary Seed/Handler main
   behavior is unchanged.

3. **Public boundary.** Only source-file and project interpreter `run` routes
   gain Task success. `check` retains its current behavior. `test`, `build`,
   REPL, serialized project artifacts, bytecode 1.0 through 1.4, VM, Native,
   Wasm, LSP, and editor execution continue to reject checked Tasks with
   `L-TASK-0004`. Help and documentation must state this exact boundary.

4. **Configuration and bounds.** Construction requires non-zero limits for
   worker count, queued Tasks, TaskRuntime tasks/scopes/steps/faults, direct
   children per lexical scope, scheduler transitions, park/wake cycles, and
   shutdown transitions. Worker count is at most 64 and queue capacity cannot
   exceed the runtime Task limit. Every invalid relation, count overflow, or
   statically excessive direct-child scope fails before a worker starts or host
   Effect occurs. CLI `run` uses a documented constant configuration and never
   derives semantics from CPU count; embedding tests may supply other valid
   configurations.

5. **Ownership architecture.** One scoped local worker pool and one central
   coordinator own a run. The coordinator state contains the exact
   `TaskRuntime`, a bounded FIFO of canonical `TaskPath` identities, queued-set
   membership, cancellation/shutdown state, internal observations, and metrics,
   protected by one mutex and condition variable. A worker removes one ready
   identity and executes exactly one DEC-0266 step while holding exclusive
   coordinator ownership. Runtime transitions and host Effect boundaries are
   therefore serialized in this first implementation; worker parallel speedup
   is not claimed.

6. **Queue discipline.** Initial and newly ready identities are obtained only
   from `TaskRuntime::ready()`, verified in strict canonical order, and appended
   once if absent from the queued set. Pop is FIFO. Unknown, duplicate,
   non-ready, or capacity-exceeding queue entries are internal scheduler
   failures detected before the affected runtime step. FIFO and worker
   acquisition order are implementation behavior, not Ling scheduling
   semantics or a Stable fairness promise.

7. **Wake and park.** A worker parks on the condition variable only while the
   queue is empty and the run is neither terminal nor shutting down. Queue
   insertion, cancellation, root termination, and shutdown update the guarded
   predicate before notifying. Workers always recheck that predicate after a
   spurious wake. No wall time, polling sleep, CPU duration, timezone, or I/O
   readiness creates a wake.

8. **Progress class.** With a finite ready set, a non-failing host, available
   worker progress, and unexhausted limits, every queued identity is eventually
   removed because no worker may retain or bypass a queue item. This is a local
   liveness property of this implementation, not an ordering guarantee. Host
   thread starvation, process suspension, machine failure, and resource
   exhaustion are explicit non-claims.

9. **Cancellation and shutdown.** An injected clonable host control token may
   request cancellation but cannot select a Task or mutate the tree. Workers
   observe it only at guarded scheduling boundaries and request DEC-0266
   `Requested` cancellation on the root. Normal root completion, cancellation,
   or Fault begins scheduler shutdown only after the structured runtime reaches
   its terminal state, which already implies descendant drain and exactly-once
   cleanup. The coordinator then marks shutdown, wakes every worker, joins all
   workers, and publishes one result. Dropping a control token does nothing.

10. **Failure precedence and containment.** DEC-0266 Fault dominates
    cancellation, which dominates normal return; committed host Effects remain
    committed. A worker or host panic is caught at the scheduler boundary,
    requests root cancellation where runtime progress remains possible, wakes
    all workers, joins them, and returns a bounded internal scheduler failure;
    it never fabricates a successful Task value, catchable Ling Fault, or host
    event. Mutex poisoning, lost coordinator ownership, queue corruption, and
    worker-join failure use the same internal failure boundary.

11. **Resource quotas.** Existing DEC-0266 run-wide limits remain authoritative.
    TASK-2205 additionally preflights the checked maximum direct spawn sites of
    every lexical scope against the configured per-scope child limit and bounds
    live queue entries, worker count, scheduler transitions, wake/park cycles,
    and shutdown work. Since this Task profile has no accepted user Resource or
    recoverable allocator model, byte-heap, file, socket, and finalizer quotas
    are not invented here and remain GAP-STRUCTURED-TASK-001 work.

12. **Task-tree observation.** An injected internal control handle may read an
    immutable bounded snapshot at completed scheduling boundaries. A snapshot
    contains a monotonically increasing epoch plus canonical `(TaskPath,
    TaskRuntimeState, cleanup_count)` records and the root state. It excludes
    source paths, worker/thread identities, addresses, queue acquisition order,
    host timing, and mutable runtime references. It is Experimental in-process
    evidence only: no CLI flag, JSON, schema, protocol inventory, or Stable
    query is added.

13. **Metrics.** Internal monotonic counters may record completed steps,
    enqueues, dequeues, parks, wakes, cancellation observations, worker exits,
    and maximum queue width. Counter reads occur only after the corresponding
    guarded mutation and cannot influence selection, cancellation, Fault,
    cleanup, host Effects, return values, diagnostics, or exit codes. Metrics
    are not a clock, performance guarantee, public protocol, or language data.

14. **Observable nondeterminism.** The language does not expose worker identity
    or selection order. Host Effects from different Tasks may occur in any order
    admitted by checked step boundaries and DEC-0266 lifecycle rules; each host
    call remains one indivisible checked Effect boundary. Programs requiring an
    Effect order must express a dependency through structured await. Worker
    count or scheduling order may choose among admitted Effect/Fault races but
    cannot change canonical Task identities, final retained Fault ordering,
    cleanup multiplicity, type safety, or memory safety.

15. **Diagnostics and compatibility.** Invalid checked entry/runtime evidence
    and exhausted semantic limits use existing bilingual registered entry,
    `L-TASK-0004`, or `L-RUNTIME-0001` categories as applicable; no new code is
    allocated without a separate registry change. Internal scheduler failures
    are typed Rust errors mapped at the CLI boundary without Rust debug text,
    paths, addresses, thread IDs, or panic payloads. No Semantic ID, checked
    Task Core/machine byte, Audit Source, or public schema version changes.

16. **Completion boundary.** TASK-2205 is complete only with positive,
    negative, boundary, cancellation, Fault, cleanup, queue, wake/park,
    shutdown, worker-panic, host-panic, quota, snapshot, metrics-noninterference,
    repeated-run, worker-count differential, Unicode/span, file/project CLI,
    and retained rejection tests. The implementation must remain offline after
    dependencies are locked and add no threading dependency where `std` is
    sufficient.

## Conformance plan

- Run checked `task main ()` through file and project interpreter routes with
  one and multiple workers; require the same terminal class, canonical Fault
  set, cleanup counts, and allowed host-Effect partial order.
- Cover empty/oversized pools, queue capacity one and maximum, nested scopes,
  multiple simultaneously ready children, cancellation before start/ready/
  suspended/host/cleanup/terminal boundaries, and every scheduler/runtime
  limit with mutation-before-failure checks.
- Force park, wake, spurious wake, cancellation wake, normal shutdown, Fault
  shutdown, worker panic, host panic, mutex poison, and join failure through
  bounded injected test seams; require no lost wake, orphan, duplicate step,
  repeated Effect, or missed cleanup.
- Compare internal snapshots and metrics across repeated checked
  reconstruction, physical paths, source IDs, BOM/CRLF, Chinese identifiers,
  and worker counts; metrics may differ but deleting their reads must not change
  outcomes or host events.
- Retain `L-TASK-0004` before output/artifact publication for test/build/REPL,
  bytecode 1.0–1.4, VM, Native/Wasm, LSP, and editor routes. Preserve ordinary
  Seed/Handler file/project run bytes and exits.
- Run targeted loom-equivalent state-machine tests if an already locked tool is
  available; otherwise use deterministic barriers/channels and repeated bounded
  stress without adding a network dependency. TASK-2206 retains performance and
  million-short-task claims.

## Compatibility impact

- Source: adds no syntax; it assigns executable-entry meaning to the already
  accepted exact `task main ()` form only on file/project interpreter `run`.
- Runtime: adds an Experimental fixed local worker pool, central bounded queue,
  wake/park, cancellation control, shutdown/join, internal snapshots, and
  nonsemantic metrics over checked TaskRuntime.
- CLI: file/project `ling run` may succeed for the exact checked Task entry.
  Other Task routes retain `L-TASK-0004`; no worker-count or metrics flag is
  added in this slice.
- Diagnostics/schemas: no new diagnostic code, public schema, Semantic ID,
  protocol, artifact, bytecode, VM, ABI, or Replay format.
- Determinism/Unicode: scheduler order is an explicitly limited nondeterminism;
  canonical identities, Fault order, cleanup, original UTF-8 spans, and Unicode
  17.0.0 remain authoritative.

## Unresolved alternatives

- Work stealing, per-worker deques, lock-free queues, preemption, priorities,
  affinity, CPU-count-derived defaults, public fairness guarantees, public
  Task-tree/metrics APIs, and scheduler trace files are rejected from the first
  production slice.
- Task test/build/REPL/artifact execution, bytecode/VM/native Task ABI,
  wall-clock Clock/sleep, I/O readiness, detach, user Resource finalizers,
  recoverable allocation budgets, Replay, Actor crossing, migration, stress,
  million-short-task performance, and Stable compatibility remain TASK-2206 or
  later Accepted work.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
