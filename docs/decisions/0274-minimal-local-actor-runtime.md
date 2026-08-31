# DEC-0274: Minimal local Actor runtime / 最小本地 Actor Runtime

> 状态：Accepted<br>
> 提出日期：2026-08-31<br>
> 决定日期：2026-08-31<br>
> Owner role：actor-runtime<br>
> 相关 RFC/缺口：DEC-0010 | DEC-0013 | DEC-0099 | DEC-0266 | DEC-0268 | DEC-0270 | DEC-0271 | DEC-0272 | DEC-0273 | GAP-ACTOR-MAILBOX-SUPERVISOR-001 | ACT-2305<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal defines the smallest executable, in-process Actor runtime that
can complete ACT-2305 over the checked-only profile accepted by DEC-0270
through DEC-0273. It is an internal Experimental runtime boundary: it does not
add Actor expressions, a public Actor entry point, CLI Actor execution,
supervision, Replay, serialization, remote delivery, bytecode, VM, or native
execution.

本提案定义可在 DEC-0270 至 DEC-0273 已接受 checked-only profile 之上完成
ACT-2305 的最小可执行进程内 Actor Runtime。它是内部 Experimental runtime
边界：不增加 Actor 表达式、公开 Actor 入口、CLI Actor 执行、监督、Replay、序列化、
远程交付、bytecode、VM 或 native 执行。

## Question

What exact bounded local runtime may allocate Actor incarnations, admit typed
messages, dispatch non-suspending turns, publish state atomically, stop and
clean up instances, and integrate their lifetime with the accepted local Task
runtime without inventing source-level Actor operations or later Supervisor,
Replay, remote, and backend contracts?

## Decision

1. **Accepted inputs only.** ACT-2305 consumes one immutable successful
   `CheckedProgram` and its exact DEC-0270 through DEC-0273
   `CheckedActorCore` values. Runtime construction revalidates the Actor type,
   message schema, mailbox contract, turn contract, expression/binding
   ownership, and checked program identity. AST, unresolved HIR, source text,
   malformed Core, Semantic Graph JSON, and observation-only DEC-0099 data are
   never executable inputs.

2. **Internal execution boundary.** The first runtime is implemented inside
   `ling-eval`, where checked expressions and runtime values already have one
   evaluator authority. `ling-concurrency` may hold behavior-free identity,
   lifecycle, envelope, and limit types, but it must not execute host closures
   as substitutes for Ling Core. The runtime API remains crate-internal or
   explicitly Experimental Rust embedding surface. Actor-bearing file/project
   `run`, `test`, `build`, REPL, artifact, bytecode, VM, Native, Wasm, LSP, and
   editor paths continue to stop before execution with `L-ACTOR-0002`.

3. **Run and incarnation identity.** One runtime is owned by exactly one local
   checked-program run and receives a nonzero opaque run identity from that
   owner; it uses no process-global allocator. Within the run, `ActorId` values
   are allocated monotonically from one, are nonzero, and are never reused,
   including after stop or failed spawn. Exhaustion fails before registry
   publication. A local runtime reference contains the run identity,
   `ActorId`, `ActorTypeId`, and exact message-schema identity; it exposes no
   state, address, scheduler slot, OS thread, source path, or remote endpoint.

4. **Explicit bounds.** Runtime construction requires nonzero limits for
   created incarnations, simultaneously live actors, total queued messages,
   lifecycle events, runtime commands, turns, retained Fault causes, and
   shutdown work. Each checked mailbox retains DEC-0272's exact per-Actor
   capacity. Invalid relations, integer overflow, or exhaustion is detected
   before the bounded mutation and returns a typed runtime error; it cannot
   publish a partial actor, envelope, turn, state, event, or successful result.

5. **Failure-atomic spawn.** `spawn(actor_definition)` resolves exactly one
   checked Actor declaration, evaluates its pure checked initializer, verifies
   the resulting closed Value against the checked state type, and creates an
   empty bounded mailbox. The `ActorId` is reserved before initializer
   evaluation and is retired even when initialization fails. Only successful
   initialization atomically inserts a `Running` incarnation and returns its
   local typed reference. Failure inserts no live registry entry and returns
   bounded initializer Fault provenance.

6. **Runtime values and typed envelopes.** State and message payloads admit
   exactly DEC-0270/DEC-0271 closed ordinary Values: `Unit`, `Bool`, `Int`,
   `Float64`, `Text`, and recursively admitted tuple/list/nominal values.
   Function, continuation, Task handle, Actor reference, Capability, Resource,
   Managed graph, borrow, Cell, open type, and unknown future values are
   rejected. Each admitted envelope owns one value plus target Actor/run/type,
   exact schema identity, opaque local sender identity, and monotonically
   increasing per-sender sequence. It contains no Rust layout, pointer,
   serialized payload, source path, wall time, thread, or remote identity.

7. **Send admission and ownership.** A runtime send validates the reference's
   run, live incarnation, Actor type, message schema, and payload type before
   inspecting capacity. A `Running` actor with a free slot returns `Accepted`
   and moves exactly one envelope into the FIFO queue. A full queue returns
   `Full`; a stopping/stopped/failed actor returns `Closed`; an unknown,
   cross-run, wrong-type, wrong-schema, or malformed payload returns a distinct
   typed internal error. Every non-accepted result returns the original payload
   to the embedding caller and leaves queue state and sequence counters
   unchanged. There is no `Wait`, drop, coalescing, retry, implicit clone, or
   remote send.

8. **Mailbox ordering.** Each mailbox is a bounded FIFO of accepted envelopes.
   Per-sender sequence must be contiguous and therefore preserves DEC-0272
   admission order. The runtime coordinator serializes admissions in this first
   implementation; the resulting cross-sender queue order is explicit runtime
   input/implementation behavior, not a Stable language fairness or concurrent
   arrival guarantee. The message removed for an active turn no longer counts
   toward queued capacity.

9. **Explicit dispatch surface.** The runtime publishes a canonical sorted set
   of `Running` Actor IDs whose mailbox is non-empty and accepts `step(id)` only
   for a member of that set. One step removes exactly one envelope, evaluates
   exactly one DEC-0273 non-suspending transition with the last committed state
   and message bindings, and reaches a turn boundary before returning. The
   runtime never chooses among multiple ready actors, observes wall time,
   preempts, reenters, batches messages, or evaluates another actor
   recursively. A production coordinator may choose a ready actor; that choice
   is not part of checked Actor identity.

10. **Atomic state publication.** A normally returned candidate is revalidated
    against the exact checked state type and replaces the committed state once
    after transition completion. Until then only the previous state is
    committed. Evaluation Fault, panic containment, invalid candidate, limit
    failure, stop, or cancellation publishes no candidate and preserves the
    previous state through terminal cleanup. No partial field mutation,
    rollback log, state getter, external Cell, or state serialization is added.

11. **Turn Fault and containment.** A source-level/runtime failure in an
    initializer or turn becomes bounded `ActorFault` provenance containing the
    run, Actor type, optional incarnation, phase, checked expression/body
    identity, original source span, and underlying registered runtime category.
    A turn Fault atomically closes admission, moves the incarnation to `Failed`,
    drains its queued messages during cleanup, and reports the Fault to the
    owning runtime coordinator. No host panic/unwind payload, path, address,
    thread identity, or Rust debug text crosses the boundary. Supervisor
    interception, restart, retry, state restore, escalation, and catchable
    Actor Fault values remain SUP-2401 and later work.

12. **Lifecycle.** The exact first lifecycle is
    `Starting -> Running -> Stopping -> Stopped` or
    `Starting/Running -> Failed`. `Starting` is not registry-visible until
    successful spawn. Terminal records retain only bounded identity, terminal
    reason, cleanup count, and Fault reference needed to prove non-reuse and
    cleanup. No restart transition exists. Lifecycle events are emitted only
    after the corresponding state mutation and cannot drive scheduling or
    affect Ling results.

13. **Stop semantics.** `stop(ref)` is accepted only at coordinator boundaries.
    On a `Running` actor it closes admission, enters `Stopping`, removes queued
    envelopes without executing them, performs runtime-owned cleanup exactly
    once, and reaches `Stopped`. A stop requested while a non-suspending turn is
    executing is ordered after that turn boundary and cannot preempt it. A
    repeated stop of the same terminal incarnation returns `AlreadyStopped`
    without another event or cleanup; unknown and cross-run references are
    typed errors. No graceful drain, poison message, user finalizer, or
    supervisor action is implied.

14. **Structured Task ownership.** Every runtime and Actor incarnation is
    structurally owned by one accepted local Task-runtime root; detached Actors
    do not exist. The Task coordinator is the sole owner of the Actor registry
    and command boundary. Root cancellation or Fault requests stop of every
    live Actor in canonical Actor-ID order, drains runtime-owned queues, performs
    exactly-once Actor cleanup, and only then permits Task scheduler shutdown.
    An unhandled Actor Fault is reported to that coordinator and requests root
    Task cancellation in the no-Supervisor profile. This adds no source Task to
    Actor crossing, Actor handle value, new Task Core form, Task bytecode/VM
    ABI, or public combined scheduler protocol.

15. **Registry and shutdown.** The Actor registry is one run-owned bounded map,
    never a static, singleton, ambient service, or language-visible mutable
    global. Shutdown closes admission before cleanup, orders terminal cleanup
    by `ActorId`, rejects further commands, retires all identities, clears all
    queues and state, and publishes one terminal runtime result. Dropping a
    Rust handle must not silently replace explicit shutdown in executable
    paths; test-only drop guards may assert incomplete cleanup.

16. **Determinism and observations.** Actor IDs, ready sets, terminal records,
    Fault causes, and shutdown cleanup are canonically ordered. Scheduler
    choice, cross-sender admissions, and host cancellation arrival are explicit
    runtime inputs. Internal bounded lifecycle/queue/turn snapshots may be used
    for ACT-2305/ACT-2306 evidence, but they are not Semantic Graph, Audit,
    Replay, metrics, public JSON, or a compatibility protocol and exclude
    source paths, allocation, wall time, threads, and hash-map iteration.

17. **Compatibility.** No source grammar, checked Actor Core, Actor message or
    mailbox contract, `x-ling-actor/0.3`, Semantic ID, Audit Source, public JSON
    schema, diagnostic allocation, CLI exit contract, bytecode/VM/native ABI,
    package format, or Unicode 17.0.0 behavior changes. `L-ACTOR-0002` remains
    the public Actor execution boundary. Internal resource/Fault mapping may
    reuse `L-RUNTIME-0001` only where its registered category is exact; every
    invariant error otherwise remains a typed internal error.

18. **Completion boundary.** ACT-2305 is complete only when clauses 1 through
    17 are implemented with positive, negative, boundary, ordering,
    failure-atomicity, cancellation, Fault, cleanup, deterministic
    reconstruction, Unicode/BOM/CRLF, and no-public-execution evidence; targeted
    and workspace gates pass; status and traceability are current; and no
    Supervisor, Replay, remote, serialization, public Actor expression,
    bytecode/VM/native, or Stable placeholder is introduced.

## Conformance plan

- Construct a runtime only from a successful checked program; independently
  corrupt Actor owner/type/schema/mailbox/turn/expression/binding evidence and
  require rejection before allocation or evaluation.
- Spawn zero-state and structured-state Actors; cover initializer return and
  Fault, monotonically retired IDs, live/created/queue/event/turn/Fault limits,
  arithmetic overflow, and failure atomicity.
- Send valid primitive and nested nominal messages; reject wrong runtime,
  Actor, schema, type and malformed values; exercise capacity one, maximum
  checked capacity, exact `Full`, `Closed`, returned-payload, FIFO and
  per-sender-order boundaries.
- Dispatch explicit ready Actor IDs through normal and Faulting pure turns;
  require one message per turn, no reentry/suspension, publish-on-return,
  preserve-on-failure, and no host unwind leakage.
- Stop before send, with queued messages, after turns, repeatedly, during an
  explicitly synchronized active turn, and during Task cancellation/Fault;
  require closed admission, bounded discard evidence, canonical shutdown,
  exactly-once cleanup, no orphan, and no ID reuse.
- Reconstruct equivalent checked programs under different source IDs, physical
  paths, insertion orders, Unicode names, BOM and LF/CRLF; compare runtime
  identities relative to the same explicit command schedule, terminal classes,
  state snapshots, Fault ordering, and cleanup counts.
- Retain `L-ACTOR-0002` across every public CLI/interpreter/bytecode/VM/native
  route and keep non-Actor Seed, Handler, and Task results byte-identical.

## Compatibility impact

- Source/CLI: none; no Actor expression or executable entry is exposed and all
  public Actor-bearing routes retain `L-ACTOR-0002`.
- Runtime: adds an internal Experimental local Actor registry, typed envelope,
  bounded mailbox, explicit dispatcher, lifecycle/Fault boundary, and
  structured Task-owned shutdown.
- Diagnostics/schema/identity: no new diagnostic code, public schema, Semantic
  Graph version, checked Actor identity, bytecode, VM ABI, or artifact format.
- Determinism/Unicode: canonical outcomes depend on explicit runtime commands,
  not host timing or container order; original UTF-8 spans and Unicode 17.0.0
  remain authoritative.

## Unresolved alternatives

Source-level `spawn`/`send`/`stop`, Actor entry points, ActorRef values, turn
Effects, suspension/reentry, self-send, concurrent dispatch, priorities,
fairness, graceful mailbox drain, watchdog timing, Supervisor restart/escalate,
Replay, public lifecycle protocols, serialization, RemoteRef, Task/Actor
bytecode/VM/native ABI, Resource finalizers, Managed messages, and Stable
compatibility remain later Accepted work.

The first implementation may serialize all runtime commands for correctness.
This is not a language guarantee: ACT-2306 requires separate accepted property
and stress authority before claiming parallel progress between different
Actors.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
