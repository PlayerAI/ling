# DEC-0273: Checked non-suspending Actor turn / 已检查的不可挂起 Actor turn

> 状态：Proposed<br>
> 提出日期：2026-08-31<br>
> 决定日期：Pending<br>
> Owner role：actor-semantics<br>
> 相关 RFC/缺口：DEC-0008 | DEC-0009 | DEC-0010 | DEC-0013 | DEC-0098 | DEC-0270 | DEC-0271 | DEC-0272 | GAP-ACTOR-AWAIT-REENTRY-001 | ACT-2304<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal defines the checked-only, non-suspending Actor turn contract
needed by ACT-2304. It selects no reentry for the first local Actor profile and
does not create a runtime turn, dequeue messages, commit state, schedule work,
observe wall time, or execute Actor code.

本提案定义 ACT-2304 所需的已检查、不可挂起 Actor turn 合同。首个本地 Actor
profile 不允许重入；本提案不创建 runtime turn、不出队消息、不提交状态、不调度工作、
不观察墙上时间，也不执行 Actor 代码。

## Question

What exact turn, state-commit, and reentry rules can complete the checked Actor
profile established by DEC-0270 through DEC-0272 while Actor execution,
suspension, self-send, cancellation, watchdog timing, scheduling, supervision,
and Replay remain unavailable?

## Decision

1. **Checked-only boundary.** ACT-2304 consumes only the immutable
   `CheckedActorCore`, message contract, and mailbox contract accepted by
   DEC-0270 through DEC-0272. It adds one immutable checked turn contract and a
   data-only Semantic Graph projection. It adds no Actor instance, dequeue,
   queue mutation, state cell, `spawn`, `send`, `stop`, suspension,
   continuation, scheduler, timer, interpreter path, bytecode, VM instruction,
   native ABI, supervisor, serializer, remote endpoint, or Replay event.
   Actor-bearing programs continue to stop at `L-ACTOR-0002` before execution.

2. **One-message turn.** One future successful mailbox dequeue supplies exactly
   one DEC-0271 checked local message value to exactly one invocation of the
   declaration's `receive state message` transition. A turn never batches,
   skips, duplicates, or recursively dispatches another message. The executing
   message is outside the queued-slot count as fixed by DEC-0272. ACT-2304
   records this rule but performs no dequeue or invocation.

3. **Inputs and result.** A turn takes the Actor incarnation's last committed
   state value and the one dequeued message value as immutable inputs. The
   transition result must be exactly the declared state type and denotes the
   sole candidate next state. The state and message bindings remain local to
   the transition body and cannot escape through another checked type.

4. **Non-suspending first profile.** The transition has the closed empty
   residual Effect row already required by DEC-0270. `await`, Task spawn,
   Effect handlers with a residual Effect, blocking, `send`, Actor construction,
   and every other suspension or runtime operation are forbidden within it.
   A checked turn therefore has no suspension point or continuation.

5. **No reentry.** An Actor incarnation may have at most one active turn. No
   other message can begin for that incarnation until the active transition has
   either produced its candidate next state or terminated unsuccessfully.
   Because the first profile cannot suspend, there is no state-version token,
   optimistic validation, guard retry, interleaved state view, or explicit
   reentry point. A later suspending profile requires a new Accepted decision
   and an explicit compatibility version; it cannot reinterpret this profile.

6. **Atomic state publication.** The previous committed state remains the only
   observable Actor state while a turn is active. A normally completed
   transition publishes its complete candidate next state once, at the turn
   boundary. There is no partial field publication, in-place externally
   observable mutation, intermediate state read, rollback log, or multiple
   commits. ACT-2304 represents this as checked contract evidence only; ACT-2305
   must implement the atomic runtime publication before Actor execution exists.

7. **Unsuccessful completion.** A transition that does not complete normally
   publishes no candidate state; the previous committed state remains
   unchanged. The checked profile defines no source-level cancellation, timeout,
   shutdown, retry, restart, supervisor action, or Actor Fault outcome. Host
   panic or unwind is never a Ling completion class. Later runtime and
   supervision authority must define disjoint typed outcomes and cleanup without
   weakening the no-partial-commit rule.

8. **Borrow and ownership boundary.** The current state and message types remain
   within the closed ordinary-Value profile accepted by DEC-0270 and DEC-0271.
   No Borrow, mutable reference, Resource, Managed graph, Capability, Task
   handle, Actor reference, or external Cell can cross or survive the turn.
   Consequently no active borrow crosses a turn boundary in this profile; this
   is rejection by admitted representation, not a general lifetime theorem.

9. **Self-send and recursion.** No source `send` operation exists in ACT-2304.
   Any later self-send must use the same checked bounded mailbox admission as
   every other local send and cannot invoke `receive` recursively or bypass
   capacity, `Reject`/`Full`, ownership, or FIFO rules. Ordinary function calls
   and recursion inside a pure transition do not constitute Actor reentry.

10. **Ordering boundary.** A future runtime starts a turn only after one message
    has been admitted and selected under the accepted mailbox order. A turn does
    not select the next message. DEC-0272's per-sender admission order remains
    intact; concurrent-sender selection, scheduler fairness, logical time, and
    replay order remain unspecified and cannot enter checked turn identity.

11. **Watchdog boundary.** A future long-turn watchdog may observe and report an
    active turn, but it must not force-kill, preempt, reenter, or publish partial
    state. ACT-2304 defines no duration threshold, clock, metric, event, warning,
    diagnostic, or public query because no runtime turn or accepted timing
    authority exists. ACT-2305/ACT-2306 must add observable watchdog evidence
    before claiming executable long-turn behavior.

12. **Checked turn contract.** Each `CheckedActorCore` gains exactly one
    immutable contract containing the governing Actor definition and type,
    one-message dispatch, non-suspending execution, forbidden reentry,
    publish-on-normal-return state commit, mailbox-only future self-send, the
    transition expression and state/message bindings, and the original receive
    clause/body spans. Construction is atomic and rejects owner, type,
    expression, binding, Effect-row, span, or contract mismatches before checked
    publication.

13. **Canonical identity.** Turn canonical bytes use domain
    `ling.checked-actor-turn/1` and encode the exact semantic mode, commit rule,
    reentry rule, and governing checked identities. Source spans, source names,
    paths, trivia, duration, host thread, allocation, queue contents, runtime
    state, and Rust debug output are excluded. The bytes participate in
    Actor-bearing Body and Program identity; source evidence does not.

14. **Diagnostics.** Syntax-level attempts to place `await` where Actor syntax
    does not admit it continue to use registered syntax diagnostics. A complete
    transition whose checked Effect row is non-empty continues to use bilingual
    `L-ACTOR-0001` at the original transition span with stable reason
    `actor_transition_must_have_empty_residual_effect_row`. Malformed internal
    turn contracts return typed Rust errors. No new public diagnostic code or
    reason is allocated merely to restate the no-suspension invariant.

15. **Semantic Graph.** Actor-bearing file-mode `ling.semantic/0.1` snapshots
    replace the exact Actor extension version `x-ling-actor/0.2` with
    `x-ling-actor/0.3`. Each Actor entry adds its one-message, non-suspending,
    no-reentry, publish-on-return checked turn contract and receive evidence.
    The isolated reader validates exact version, ownership, enum values,
    canonical bytes, expression/binding correspondence, span source/order, and
    Actor order. It remains data-only and cannot construct a runtime turn or
    executable Core.

16. **Compatibility.** Actor syntax does not change. Consumers that interpret
    the Experimental Actor extension must migrate explicitly from
    `x-ling-actor/0.2` to `x-ling-actor/0.3`; there is no automatic JSON adapter
    because 0.2 does not state a turn contract. Checked Actor Core advances to a
    new exact version. Non-Actor `ling.semantic/0.1` bytes and IDs,
    package-aware `ling.semantic/0.2`, CLI execution behavior, bytecode/VM
    formats, and Unicode 17.0.0 behavior remain unchanged.

17. **Completion boundary.** ACT-2304 is complete only when clauses 1 through
    16 are implemented through checked turn Core, Actor identity, and Semantic
    Graph; positive, effect/await rejection, owner/type/binding/span corruption,
    deterministic reconstruction, Unicode/BOM/CRLF, one-message, atomic-commit,
    no-reentry, self-send-boundary, reader-corruption, and no-execution evidence
    passes; protocol/schema/status traceability is current; and every deferred
    runtime, cancellation, watchdog, scheduling, supervision, and Replay
    capability remains unavailable rather than represented by a placeholder API.

## Conformance plan

- Construct the exact checked turn contract from DEC-0270's typed pure
  transition; reject non-empty Effect rows and inconsistent Actor owner, type,
  expression, binding, mode, commit, or span evidence before publication.
- Validate a pure one-message state transition model at normal completion and
  unsuccessful completion boundaries without allocating state Cells, queues,
  messages, continuations, timers, or runtime Actor instances.
- Freeze checked turn canonical bytes, Actor Program identities, and
  `x-ling-actor/0.3` across reconstruction, insertion order, Unicode Actor
  names, BOM, LF/CRLF, comments, and source names; require semantic contract
  changes to alter Actor identity.
- Corrupt extension version, owner, mode, commit, reentry, canonical bytes,
  expression/binding correspondence, order, and spans independently and require
  the isolated reader to reject without producing executable Core.
- Prove that Actor-bearing run/test/REPL/bytecode/VM/native paths still stop
  before execution and that no self-send, cancellation, watchdog, scheduler,
  supervisor, Replay, or remote operation is exposed.

## Compatibility impact

- Source/diagnostics: no new syntax or diagnostic code; the existing pure Actor
  transition and `L-ACTOR-0001` boundary become an explicit checked turn
  contract.
- Schema/Semantic ID: the Experimental Actor extension advances to
  `x-ling-actor/0.3`, checked Actor Core advances exactly once, and Actor-bearing
  identity includes canonical turn rules. Non-Actor bytes and IDs are unchanged.
- Runtime/ABI/wire: none; no dequeue, state storage, send, scheduler, watchdog,
  interpreter, bytecode, VM, native, supervisor, Replay, or remote contract is
  implemented.
- Determinism/Unicode: canonical bytes are path-free and exclude runtime order;
  Unicode remains fixed at 17.0.0.

## Unresolved alternatives

Suspending turns, guarded reentry, state versions, continuation ownership,
`Wait`, cancellation/timeout/shutdown outcomes, Fault cleanup, retry/restart,
self-send results, concurrent-sender selection, scheduler fairness, watchdog
thresholds/events, supervision, Replay, serialization, and remote delivery
remain later Accepted work. A future suspending profile must use an explicit
source/Core/protocol migration and cannot silently broaden this no-reentry
profile.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
