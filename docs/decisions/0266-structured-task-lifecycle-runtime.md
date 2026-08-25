# DEC-0266: Scheduler-neutral Structured Task lifecycle runtime / 与调度器解耦的 Structured Task 生命周期 Runtime

> 状态：Proposed<br>
> 提出日期：2026-08-26<br>
> 决定日期：Pending<br>
> Owner role：concurrency-design<br>
> 相关 RFC/缺口：DEC-0093 | DEC-0264 | DEC-0265 | GAP-STRUCTURED-TASK-001 | TASK-2203<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal closes only the scheduler-neutral lifecycle-kernel boundary of
TASK-2203. It does not authorize a scheduler policy, public Task entry point,
CLI execution, bytecode/VM Task ABI, detach, wall-clock deadline, production
worker pool, Replay, Actor crossing, or Stable protocol.

本提案仅关闭 TASK-2203 与调度器解耦的生命周期内核边界，不授权 scheduler policy、
公开 Task 入口、CLI 执行、bytecode/VM Task ABI、detach、wall-clock deadline、
生产 worker pool、Replay、Actor crossing 或 Stable protocol。

## Question

What bounded executable lifecycle kernel can consume Accepted Checked Task
machines, enforce lexical parent/child ownership, join, cancellation, cleanup,
and deterministic Fault aggregation, and expose explicit scheduling choices to
TASK-2204 without interpreting unchecked syntax or silently choosing production
scheduling semantics?

## Decision

1. **Authority and input.** TASK-2203 adds one Experimental, publish-disabled
   `TaskRuntime` in `ling-eval`. Construction accepts only a successful
   `CheckedProgram`, one exact Task `DefinitionId`, checked argument values, and
   explicit non-zero limits. The selected definition must own matching
   DEC-0264 Checked Task Core and DEC-0265 `ling.task-machine/0.1` evidence.
   Missing or disagreeing definitions, signatures, scopes, spawns,
   suspensions, frames, edges, or source identities fail before a runtime task
   is published. AST, source text, unresolved HIR, debug output, and ordinary
   unvalidated Task-shaped values are never execution inputs.

2. **No implicit source entry.** The caller-selected root is an internal test
   and embedding boundary, not a Ling entry-point rule. DEC-0013 `let main ()`
   remains the only public `run` entry. File/project run and test, REPL,
   artifacts, existing interpreter entry points, bytecode 1.0–1.4, and VM paths
   continue to reject any checked Task program with `L-TASK-0004`. A public
   Task root or CLI success path requires separate Accepted authority after a
   scheduler is available.

3. **Runtime identities.** A runtime Task identity is the canonical lexical
   path `Root / (spawn TaskId)*`; a runtime scope identity is that Task path
   plus its Checked `ScopeId`. Because the accepted source subset has no Task
   loops or recursive spawn chain, one lexical spawn site executes at most once
   per parent instance. Identities therefore do not depend on allocation,
   thread, wake, insertion, or scheduler order. Duplicate paths, unknown
   parents, and scope/definition disagreement are internal checked-core
   failures.

4. **Lifecycle states.** A Task instance is in exactly one state:
   `Ready`, `Running`, `Suspended(awaited child)`, `Joining(scope)`,
   `Cancelling(cause)`, `Cleaning(reason)`, `Completed(value)`, `Cancelled`, or
   `Faulted(fault set)`. A scope is `Open`, `Closing(reason)`, or
   `Closed(reason)`. Terminal Task and closed-scope states are immutable.
   Every transition validates its source state, checked machine edge, owning
   scope, and related Task identity before mutation; failure publishes no
   partial transition.

5. **Linear handle registry amendment.** This decision narrows DEC-0264 clause
   7 only as follows: another immutable, unconsumed Task handle owned by the
   same lexical scope MAY remain live across a suspension. It remains linear,
   non-copyable, non-aggregate, non-closure, non-Handler, non-returnable, and
   observable exactly once on every path; cross-scope use remains rejected.
   Such handles are retained by the runtime scope registry and are not ordinary
   suspension values, `SuspensionLiveBinding` entries, or DEC-0265 typed frame
   slots. Mutable bindings, borrows, Handler continuations, and all other
   rejected live values remain rejected. Existing Task Core, machine, Semantic
   Graph, and Audit bytes for previously accepted programs do not change.

6. **Registration and spawn boundary.** Executing a checked spawn atomically
   creates the child identity, cancellation child token, cleanup obligation,
   argument environment, and handle registry entry before the child becomes
   `Ready`. A spawn is a scheduling boundary: after registration, both the
   parent continuation and child may be ready, and TASK-2203 assigns neither
   priority. Failure during registration leaves no child, handle, or partial
   lifecycle event.

7. **Await and wake.** Await consumes the exact same-scope handle. If the child
   completed, its value resumes the matching Checked suspension. If it is
   nonterminal, the parent becomes `Suspended` and cannot be selected. When the
   child becomes terminal, the parent becomes `Ready`; it does not resume
   implicitly. If the parent was suspended on a child that Faulted or was
   cancelled, the terminal outcome is delivered at that suspension only after
   the owning scope completes its required cancellation and cleanup drain. A
   Fault detected before its source-level await is delivered at the owner's
   next mandatory lifecycle boundary without fabricating an await result.

8. **Structured close and join.** Leaving a lexical scope changes it to
   `Closing` and forbids new children. The owner cannot pass the scope boundary
   until every registered child is terminal. Normal close additionally
   requires every handle's one statically checked observation to have occurred.
   Cancel/Fault close consumes any remaining registry handles only as linear
   ownership obligations after cancelling and draining their children; it does
   not fabricate an await, value, or successful observation. Nonterminal
   children form an explicit join set; the owner becomes `Joining` until it is
   empty. Descendant scopes close before their parent scope. `detach` has no
   syntax, Capability, runtime command, or orphan escape in this version. A
   root may become terminal only after all of its scopes are closed, so
   termination with an orphan is impossible.

9. **Cancellation.** Cancellation is an explicit, monotonic, idempotent
   request with cause `Requested`, `Ancestor`, or `Deadline`. A `Deadline`
   request may be injected only by a future explicit logical `Clock` adapter;
   TASK-2203 reads no wall clock and creates no timer or thread. Observing
   cancellation on a nonterminal parent atomically marks all nonterminal
   descendants with `Ancestor`, prevents new spawn and ordinary resume, and
   begins the join/cleanup drain. Already completed operations and host Effects
   remain committed; cancellation performs no rollback. A task checks its
   token before each selected lifecycle segment, host Effect, spawn,
   suspension resume, scope close, and cleanup transition.

10. **Fault collection and precedence.** A Task Fault is an existing
    source-mapped `RuntimeFault` attached to the faulting runtime Task identity.
    A child Fault records `Closing(FaultPending)` on its owning scope, makes the
    owner `Ready` for a mandatory propagation transition, and prevents new
    spawn or further ordinary owner work in that scope. Already ready/running
    siblings may reach one boundary before that owner transition is selected;
    their independently produced Faults are retained. Selecting the owner then
    changes the reason to `Closing(Fault)`, cancels the faulting child's
    nonterminal descendants plus all remaining nonterminal siblings, and
    drains them. The final immutable fault set
    contains the owner's own Fault, if any, plus every descendant Fault,
    sorted by canonical runtime Task path and stable Fault facts—not occurrence
    time. Fault dominates cancellation; cancellation dominates normal return.
    If an owner Fault exists it is primary, otherwise the smallest canonical
    child path is primary; remaining Faults are related causes. Cancellation of
    a sibling during fail-fast drain is not fabricated as a Fault. The complete
    aggregate uses existing bilingual `L-RUNTIME-0001` with category
    `task_fault_aggregate`; it adds structured task-path/count/related-span
    facts but no new diagnostic code and is not catchable source data.

11. **Cleanup guarantee.** Every Checked cleanup identity is entered exactly
    once for `Return`, `Cancel`, or `Fault`, retains that reason, and reaches
    the matching DEC-0265 terminal edge only after descendant drain. Cleanup
    may release runtime-owned handle, frame, and registry state; it must not
    expose Rust destructor order, allocation, address, or container order as
    Ling behavior. The current source language has no user Resource finalizer,
    so no user cleanup callback or Effect is invented. Future Resource cleanup
    must compose with this children-first, exactly-once boundary under separate
    Accepted authority.

12. **Explicit scheduling surface.** `TaskRuntime` exposes the canonical sorted
    set of currently `Ready` task identities and accepts `step(id)` only for an
    identity in that set. One step executes a bounded checked segment until the
    next host Effect completion, spawn, suspension, join, cancellation,
    cleanup, terminal, or Fault boundary. The kernel never chooses among
    multiple ready tasks, reads time,
    parks a thread, or promises effect interleaving order. Tests may provide an
    explicit sequence of task identities as input evidence; this is not a
    scheduler. Seeded choice, virtual time, wake policy, bounded interleaving,
    and trace export remain TASK-2204.

13. **Bounds and atomic failure.** Construction requires explicit limits for
    runtime Task instances, nested runtime scopes, lifecycle steps, and retained
    Fault causes. Exhaustion is detected before the bounded action, becomes an
    existing `L-RUNTIME-0001` `resource_limit` Fault on the selected Task, and
    enters the same cancellation/join/cleanup drain. Arithmetic overflow,
    allocation failure, invalid driver choice, or checked invariant failure
    cannot publish a child, transition, output, or terminal result guessed from
    partial state. Host Effects completed before a later failure remain
    committed.

14. **Observation and publication.** The kernel may populate the Accepted
    DEC-0093 lifecycle observation model as non-authoritative evidence. Runtime
    results, ready sets, Fault aggregates, and traces remain internal and
    publish-disabled; no Semantic Graph, Audit Source, schema, artifact, Replay,
    CLI/LSP, package, or protocol version changes. TASK-2203 is complete only
    when positive, negative, bounded, cancellation, cleanup, Fault, and
    schedule-sequence differential tests consume the checked runtime boundary.

15. **Later authority.** TASK-2204 defines deterministic test scheduling and
    virtual time over this explicit ready-set interface. TASK-2205 defines the
    production scheduler, worker/queue/wake behavior, metrics, shutdown, and
    public execution integration. TASK-2206 retains full conformance, stress,
    million-short-task, race, and shutdown evidence. Task bytecode/VM/native
    ABI, public root entry, detach, Resource finalizers, Replay, Actor crossing,
    and Stable compatibility remain separately governed.

## Conformance plan

- Start a caller-selected checked root and exercise empty, nested, sequential,
  and multiple-child scopes through explicit driver sequences; require exact
  identity paths, registration-before-ready, suspension/wake, join, cleanup,
  and terminal states.
- Accept multiple same-scope linear handles across suspension through the
  scope registry; continue rejecting mutable values, cross-scope handles,
  copies, aggregates, closures, Handler capture, return, double observation,
  and leaks before runtime publication.
- Drive equivalent two-child boundary outcomes in opposite completion orders
  before owner propagation and compare canonical Fault sets, cleanup counts,
  and closed task trees. Treat different ready-task choices, host-Effect
  interleavings, and cancellation races as explicit inputs rather than claiming
  schedule-independent observable values.
- Cover cancellation before root start, after spawn, while suspended, during
  join, after committed Console output, and during Fault drain; require
  monotonic descendant propagation, no later ordinary operation, no rollback,
  children-first exactly-once cleanup, and no orphan.
- Produce owner-only, child-only, simultaneous multi-child, transitive, and
  Fault-plus-cancellation cases; freeze owner-primary/canonical-child-primary
  selection, related cause ordering, original UTF-8 spans, bilingual
  `L-RUNTIME-0001`, and bounded aggregate facts.
- Exhaust each explicit limit at the exact boundary and inject allocation,
  host, invalid-driver, and checked-invariant failures; prove failure atomicity
  apart from already committed Effects.
- Preserve deterministic logical results across reconstruction, insertion
  order, physical paths, source IDs, BOM/CRLF, Chinese identifiers, and
  equivalent explicit schedules. Keep ordinary Seed/Handler interpreter and
  bytecode/VM differential suites byte-identical.
- Keep `L-TASK-0004` rejection evidence for all public file/project run/test/
  build, REPL, artifact, bytecode 1.0–1.4, and VM paths until later Accepted
  entry/scheduler authority removes a named boundary.

## Compatibility impact

- Source: permits only same-scope linear Task handles to remain runtime-owned
  across suspension; all prior accepted programs retain their syntax, checked
  meaning, Task Core bytes, and machine bytes. No detach, deadline, Resource,
  public root, or new Task syntax is added.
- Runtime: adds an internal Experimental scheduler-neutral lifecycle kernel and
  checked Task test/embedding boundary. Existing public interpreter, CLI,
  project, artifact, bytecode, and VM behavior remains rejection-only.
- Diagnostics: reuses `L-RUNTIME-0001` for bounded resource and Task Fault
  aggregate facts; `L-TASK-0004` remains the public execution boundary. No code
  allocation or existing meaning changes.
- Schemas/protocols/Semantic IDs: none. DEC-0093 trace data remains internal;
  Semantic Graph and Audit Source bytes are unchanged.
- Determinism/Unicode: runtime identities and Fault ordering are path-free and
  scheduler-order-independent; explicit schedules remain inputs. Original
  UTF-8 spans and Unicode 17.0.0 remain authoritative.

## Unresolved alternatives

- Implicit FIFO/depth-first scheduling, eager child priority, ambient threads,
  wall-clock polling, rollback, occurrence-time Fault precedence, silent
  orphaning, implicit detach, and Rust destructor order as semantics are
  rejected.
- Making Task handles ordinary frame values is rejected: the lexical scope
  registry retains their linear identity without changing DEC-0265 frame
  bytes or exposing a Task-handle runtime ABI.
- Public `task main`, ordinary-main spawn, scheduler defaults, virtual Clock,
  user Resource finalizers, catchable Task Fault values, Task bytecode/VM/
  native lowering, worker pools, work stealing, Replay, Actor crossing,
  migration, and Stable compatibility remain unresolved later work.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
