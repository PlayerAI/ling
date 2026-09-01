# DEC-0276: Minimal local Supervisor containment / 最小本地 Supervisor 故障包含

> 状态：Accepted<br>
> 提出日期：2026-08-31<br>
> 决定日期：2026-09-01<br>
> Owner role：actor-semantics<br>
> 相关 RFC/缺口：DEC-0010 | DEC-0013 | DEC-0101 | DEC-0268 | DEC-0270 | DEC-0271 | DEC-0272 | DEC-0273 | DEC-0274 | DEC-0275 | GAP-ACTOR-MAILBOX-SUPERVISOR-001 | SUP-2401<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines the smallest internal, in-process Supervisor boundary
that can contain one local Actor fault without inventing restart, source-level,
public-protocol, Replay, remote, or backend semantics. It is Accepted scoped
authority for the internal SUP-2401 slice only.

本决定定义最小的内部、进程内 Supervisor 边界，使其能够包含单个本地 Actor
故障，同时不引入 restart、源码级操作、公开协议、Replay、远程或后端语义。它只为
SUP-2401 的内部纵切提供 Accepted scoped authority。

## Question

What exact checked-Core-only, bounded local Supervisor may own the DEC-0274
Actor runtime's children, contain one child Fault, continue unaffected siblings,
and integrate with structured Task cancellation without defining automatic
restart, state restore, scheduling, or any public Ling behavior?

## Decision

1. **Accepted authority boundary.** This decision authorizes only an internal
   Experimental Rust Supervisor profile for SUP-2401. It does not close
   `GAP-ACTOR-MAILBOX-SUPERVISOR-001` or authorize SUP-2402, SUP-2403, a public
   Supervisor feature, or any source-level Actor execution.

2. **Accepted executable inputs only.** The Supervisor may consume exactly one
   successful immutable `CheckedProgram`, the DEC-0274 local `ActorRuntime`,
   and checked Actor definitions whose ownership, state, message schema,
   mailbox, turn, expression, and binding evidence already passed DEC-0270
   through DEC-0274 validation. AST, unresolved HIR, source text, malformed
   Core, Semantic Graph JSON, and DEC-0101 observation values are never
   executable Supervisor inputs.

3. **One optional root.** One local Actor runtime may have either no Supervisor
   or exactly one run-owned root Supervisor. The existing no-Supervisor profile
   retains DEC-0274 behavior. The proposed Supervisor is owned by the same
   structured local Task root as the Actor runtime, cannot be detached, and
   cannot own or be owned by another Supervisor. There is no Supervisor tree,
   dynamic nesting, process-global registry, ambient service, or language-visible
   Supervisor identity.

4. **Fixed child slots.** Construction receives a non-empty duplicate-free set
   of checked Actor definitions. The coordinator orders it by accepted
   `ActorTypeId`, creates one child slot per definition, and permits exactly one
   live Actor incarnation in each slot. A slot is identified internally by the
   run identity and `ActorTypeId`; it is not a Semantic ID, Actor reference,
   serialized identifier, or public handle. Multiple instances of one Actor
   type, dynamic add/remove, detached children, and child migration are rejected.

5. **Failure-atomic construction.** The Supervisor spawns initial children in
   ascending child-slot order using DEC-0274 spawn and initializer rules. It is
   published only after every child reaches `Running`. If validation,
   initialization, capacity, command, event, Fault-retention, or cleanup
   preflight fails, every successfully spawned child is stopped in ascending
   `ActorId` order, all owned queues and state are released, and no Supervisor
   becomes visible. Reserved Actor IDs remain retired as DEC-0274 requires.

6. **Supervisor and slot lifecycle.** The Supervisor lifecycle is exactly
   `Starting -> Running -> Stopping -> Stopped` or
   `Starting/Running -> Failed`. `Starting` is not registry-visible. Each child
   slot is `Starting`, `Running(ActorId)`, `Contained(ActorId, ActorFault)`, or
   `Stopped(ActorId)`. A contained slot is terminal for the run and cannot
   return to `Starting` or `Running`. A child Fault does not by itself move the
   Supervisor out of `Running`.

7. **Single containment policy.** The only policy is `ContainOne`: a Fault
   terminates and seals exactly the failing child slot while all other
   `Running` siblings retain their committed state, admitted mailbox contents,
   and readiness. No sibling is restarted, stopped, reordered, or re-evaluated
   because of that Fault. `Restart`, `OneForOne` restart, `RestForOne`,
   `OneForAll`, escalation-by-child-class, and configurable strategies are
   rejected rather than represented by placeholder variants.

8. **Synchronous child-Fault report.** After DEC-0274 has atomically moved the
   child to `Failed`, preserved no candidate state, closed admission, drained
   its queue, and completed exactly-once cleanup, the Actor coordinator delivers
   one synchronous `ChildFaultReport` to the Supervisor before accepting the
   next runtime command. The report contains only the run identity, child slot,
   `ActorId`, `ActorTypeId`, phase, checked body/expression identity, original
   UTF-8 source span, registered Fault category, discard count, and cleanup
   count. It contains no payload value, path, wall time, thread, address,
   allocation, host panic text, or Rust debug output and is not a mailbox,
   Effect, catchable Ling value, public event, or queued protocol.

9. **Fault containment and root fallback.** The Supervisor accepts exactly one
   matching report for the current `Running` incarnation, records the slot as
   `Contained`, and acknowledges the Fault as handled. Only that acknowledgement
   suppresses DEC-0274's no-Supervisor root-cancellation fallback. Unknown,
   cross-run, wrong-type, stale, duplicate, out-of-order, malformed, or
   resource-unrecordable reports are typed internal invariant failures: the
   Supervisor closes child admission, enters `Failed`, stops every remaining
   live child in ascending `ActorId` order, and requests root Task cancellation.

10. **No restart or state restore.** A contained slot accepts no send, step,
    stop, replacement, mailbox transfer, retry, or state query other than the
    existing terminal evidence needed for cleanup verification. Its previous
    state and unprocessed messages are not restored, cloned, replayed, or moved
    to another Actor. There is no restart counter, budget, time window, backoff,
    jitter, circuit breaker, snapshot, restore hook, or replacement identity.
    Those semantics require separate Accepted SUP-2402 authority.

11. **No new scheduling semantics.** Existing explicit DEC-0274 ready-set and
    `step(ActorId)` coordination remains the only dispatch surface. The
    Supervisor never chooses a ready child, observes worker completion order,
    inserts implicit steps, preempts a turn, or supplies fairness/liveness.
    DEC-0275 parallel execution remains limited to distinct pure normal-return
    turns. Child Fault delivery and containment are serialized coordinator
    boundaries; parallel Fault, stop, cancellation, or recovery is rejected.

12. **Explicit stop and Task cancellation.** `stop_supervisor` is accepted only
    at a coordinator boundary. It closes admission for all `Running` children,
    enters `Stopping`, stops those children in ascending `ActorId` order,
    performs exactly-once cleanup, and reaches `Stopped`. Contained or already
    stopped slots are not cleaned again. Root Task cancellation or Fault invokes
    the same bounded stop sequence before Task scheduler shutdown. Repeated stop
    returns `AlreadyStopped` without another lifecycle event or cleanup.

13. **Supervisor failure.** The Supervisor executes no Ling initializer,
    handler, callback, or finalizer and therefore has no source-level Fault.
    Only invalid construction, invariant failure, or exhausted bounded runtime
    evidence can move it to `Failed`. Such failure is reported to the owning
    Task coordinator as a typed internal runtime failure and requests root
    cancellation; it is not a child `ActorFault`, Ling value, diagnostic, or
    public Supervisor Fault protocol.

14. **Explicit resource bounds.** The fixed slot count must fit the existing
    DEC-0274 created/live Actor limits. Each contained child consumes at most
    one retained Fault cause and one bounded terminal slot; Supervisor
    lifecycle and containment facts consume the existing runtime command,
    event, Fault-retention, and shutdown-work budgets. Every multi-step
    transition preflights its complete worst-case evidence and cleanup work.
    Exhaustion before a child turn or stop leaves the previous state unchanged;
    exhaustion after an Actor Fault follows clause 9's root-fallback cleanup
    because the Fault cannot be safely acknowledged.

15. **Deterministic evidence projection.** Internal tests may observe the
    run-relative child-slot order, Actor IDs, Supervisor/slot lifecycle,
    canonical child Fault facts, sibling state/mailbox preservation, terminal
    reason, discard count, and cleanup count. Collections are rendered in
    ascending `ActorTypeId` or `ActorId` order as applicable. Physical paths,
    source IDs, insertion order, hash-map order, allocation, worker identity,
    wall time, duration, and host scheduling are excluded. This projection is
    test evidence only, not Semantic Graph, Audit, Replay, metrics, JSON, or a
    compatibility protocol.

16. **Public boundary.** No Ling grammar, source operation, Actor or Supervisor
    value, CLI/REPL route, public Rust API, schema, protocol, diagnostic,
    Semantic ID, package format, bytecode, VM, Native, Wasm, LSP, editor, or
    migration behavior is added. Every public Actor-bearing execution route
    continues to stop with `L-ACTOR-0002`. Unicode stays at 17.0.0 and original
    UTF-8 byte spans remain authoritative.

17. **Completion boundary.** This decision is sufficient only for the internal
    SUP-2401 ownership, construction, lifecycle, `ContainOne`, Fault
    acknowledgement, root fallback, stop, resource, determinism, and negative
    public boundaries. Implementation and `Done` status require separate
    executable evidence and repository gates. Completion must not promote
    SUP-2402, SUP-2403, the open supervision gap, or any public support claim.

## Conformance plan

- Construct no-Supervisor and single-Supervisor runtimes from successful
  checked programs; reject empty children, duplicate Actor types, wrong-program
  definitions, malformed checked evidence, AST/HIR/source/observation inputs,
  and every zero, maximum, overflow, and exhaustion boundary before publication.
- Spawn children in canonical order; inject initializer failure at the first,
  middle, and final child and require no published Supervisor, retired Actor
  IDs, ascending cleanup, empty queues/state, and no orphan.
- Fault one selected child during a serial coordinator step; require one
  synchronous report, a terminal `Contained` slot, closed later sends/steps,
  no candidate publication, and byte-identical state, queue, readiness, and
  later outcomes for unaffected siblings.
- Reject unknown, cross-run, wrong-type, stale, duplicate, malformed, and
  unrecordable Fault reports; require Supervisor `Failed`, closed admission,
  canonical stop/cleanup, and root Task cancellation without host-text leakage.
- Stop before any turn, after normal turns, after one or all children are
  contained, repeatedly, and under owner cancellation/Fault; prove exactly-once
  cleanup, no mailbox transfer, no restart, no state restore, and no task/Actor
  orphan.
- Reconstruct equivalent checked inputs with different insertion order,
  source names/IDs, physical paths, Unicode identifiers/text, BOM, and LF/CRLF;
  compare only the clause 15 projection and original UTF-8 spans.
- Retain `L-ACTOR-0002` across CLI, interpreter, bytecode, VM, Native, Wasm,
  LSP, and editor paths. Future interpreter/VM/runtime differential evidence is
  required before any backend or public supervision promotion; this proposal
  authorizes no such backend execution.

## Compatibility impact

- Source and CLI: none; no syntax, value, operation, entry point, or execution
  route is added, and `L-ACTOR-0002` remains unchanged.
- Diagnostics, schemas, Semantic IDs, protocols, packages, and ABI: none; all
  Supervisor state, reports, and evidence are internal Experimental Rust data
  and are neither registered nor serialized publicly.
- Runtime: authorizes one optional, non-nested, fixed-child local Supervisor
  with `ContainOne`; the no-Supervisor DEC-0274 profile remains available and
  unchanged.
- Determinism and Unicode: canonical behavior depends only on checked identities
  and explicit coordinator commands, never host timing or container order;
  Unicode 17.0.0 and original UTF-8 byte spans are unchanged.
- Migration: none for source, artifacts, public APIs, or stored data. Any future
  public or restarting Supervisor profile needs an explicit version and
  migration decision.

## Unresolved alternatives

- Automatic restart, replacement identity, restart budgets/windows, backoff,
  jitter, circuit breakers, state snapshot/restore, mailbox transfer, and
  restart provenance remain SUP-2402 work.
- Dynamic child sets, duplicate Actor-type instances, nested Supervisor trees,
  `Permanent`/`Transient`/`Temporary` lifetimes, `OneForOne` restart,
  `RestForOne`, `OneForAll`, configurable escalation, parallel Fault/recovery,
  graceful drain, watchdogs, and user callbacks remain unresolved alternatives.
- Source-level Supervisor operations, public Fault channels, Replay, remote
  delivery, serialization, backend ABIs, fairness/liveness/performance claims,
  and Stable compatibility require later Accepted RFC and executable evidence.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
