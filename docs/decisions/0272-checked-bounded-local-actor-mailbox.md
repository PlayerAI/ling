# DEC-0272: Checked bounded local Actor mailbox / 已检查的有界本地 Actor mailbox

> 状态：Accepted<br>
> 提出日期：2026-08-31<br>
> 决定日期：2026-08-31<br>
> Owner role：actor-semantics<br>
> 相关 RFC/缺口：DEC-0010 | DEC-0012 | DEC-0013 | DEC-0097 | DEC-0270 | DEC-0271 | GAP-ACTOR-MAILBOX-SUPERVISOR-001 | ACT-2303<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision defines the checked-only local mailbox contract needed to complete
ACT-2303 without creating Actor instances, queues, send expressions, scheduling,
turn execution, supervision, serialization, or remote delivery.

本决定定义完成 ACT-2303 所需的已检查本地 mailbox 合同，但不创建 Actor 实例、队列、
send 表达式、调度、turn 执行、监督、序列化或远程交付。

## Question

What is the smallest explicit bounded-mailbox and backpressure contract that can
be attached to DEC-0271's checked Actor declaration while runtime admission,
turn execution, lifecycle, supervision, and remote behavior remain unavailable?

## Decision

1. **Checked-only boundary.** ACT-2303 consumes the immutable
   `CheckedActorCore` and `CheckedActorMessageContract` authorized by DEC-0270
   and DEC-0271. It adds a checked mailbox contract and a data-only Semantic
   Graph projection. No source Actor value, `spawn`, `send`, mailbox queue,
   scheduler, suspension, interpreter path, bytecode, VM instruction, native
   ABI, supervisor, serializer, or remote endpoint is added. Actor-bearing
   programs continue to stop at `L-ACTOR-0002` before execution.

2. **Explicit contextual source form.** Every Actor declaration in this profile
   has exactly one mailbox clause before `state`:

   ```ling
   actor Counter : Int =
       mailbox capacity 16 overflow Reject
       state Int = 0
       receive state message =
           state + message
   ```

   The extended grammar is:

   ```text
   ActorDeclaration := "actor" Name ":" Type "=" NEWLINE INDENT
                       MailboxClause StateClause ReceiveClause DEDENT
   MailboxClause     := "mailbox" "capacity" DecimalInteger
                       "overflow" "Reject" NEWLINE
   ```

   `mailbox`, `capacity`, `overflow`, and `Reject` remain contextual
   identifiers. Missing, duplicate, reordered, nested, dynamic, profile-based,
   or additional mailbox clauses fail before Checked Actor Core publication.

3. **Capacity.** Capacity is a compile-time base-10 integer counting queued
   messages. It must be in the closed range `1..=65_535`. The message currently
   executing a turn is not queued and does not consume a slot. A new mailbox is
   logically empty. Capacity cannot be zero, negative, inferred, resized, or
   overridden by a profile in this version.

4. **Resource boundary.** The capacity bounds message count, not payload bytes,
   heap size, serialization size, or host allocation. `Text`, `List`, and other
   admitted local Values may still have dynamic size. Therefore this contract
   does not establish Critical bounded allocation or a runtime memory quota;
   ACT-2305 and later profile authority must define allocation failure and
   resource accounting without changing the slot meaning silently.

5. **First policy.** `Reject` is the only admitted overflow policy. `Wait`,
   `DropNewest`, `DropOldest`, `Coalesce`, aliases, user-defined policies, and
   unknown future policies are rejected. `Wait` depends on suspension,
   cancellation, and ACT-2304 reentry rules. Drop and coalescing policies depend
   on explicit loss observability, key equality, metrics/logging, and Replay
   authority. None is reserved by accepting `Reject`.

6. **Admission classification.** For an open local mailbox whose checked queue
   length is less than capacity, one future admission is classified `Accepted`.
   At exactly capacity it is classified `Full`. `Full` leaves the logical queue
   and message ownership unchanged, performs no drop or replacement, and never
   suspends. A queue length greater than capacity is an internal invariant
   failure, not a third source-visible outcome. ACT-2303 publishes this pure
   classification as checked model evidence but exposes no send operation.

7. **Lifecycle boundary.** Closed, stopped, failed, restarting, and unknown
   receivers have no ACT-2303 send outcome because no runtime receiver exists.
   ACT-2305 and Supervisor decisions must add disjoint typed outcomes and cleanup
   rules; they cannot reinterpret `Full` as closure, Fault, retry, or message
   loss.

8. **Ordering.** A future local queue implementing this contract must dequeue
   admitted messages in admission order. Program-order admissions from one
   sender to one receiver must therefore remain FIFO. The relative admission
   order of concurrent senders is not language-deterministic in this decision;
   a runtime or Replay claim must make that choice observable through its own
   accepted scheduler/log authority. `Reject` creates no waiter queue and makes
   no fairness or starvation guarantee.

9. **Ownership.** ACT-2303 admits only DEC-0271 `SendableLocal(Value)` schemas.
   `Accepted` would transfer one immutable local Value into a future queue;
   `Full` would transfer nothing. Because no send expression or runtime value
   exists, no transfer occurs in this task. Resource, Managed, Borrow,
   Capability, ActorRef, and remote ownership remain rejected.

10. **Checked mailbox core.** Each `CheckedActorCore` gains exactly one immutable
    mailbox contract containing the governing Actor definition and Actor type,
    validated capacity, `Reject` policy, canonical contract bytes, and original
    clause/capacity/policy UTF-8 spans. Construction is atomic; missing or
    duplicate clauses, invalid bounds, unsupported policies, owner mismatch, or
    malformed checked data prevents publication.

11. **Canonical identity.** Mailbox canonical bytes use domain
    `ling.checked-local-mailbox/1` and encode the exact semantic capacity and
    policy. Source spans, spelling, comments, source IDs, paths, allocation,
    host integers, queue contents, scheduling, and Rust debug output are
    excluded. The bytes participate in Actor-bearing Body and Program identity;
    changing capacity or policy changes that identity, while trivia and source
    evidence do not.

12. **Diagnostics.** A syntactically complete mailbox clause with an invalid
    capacity or policy reports the existing bilingual `L-ACTOR-0001` at the
    original capacity or policy byte span with distinct stable reasons
    `mailbox_capacity_out_of_range` or `mailbox_overflow_policy_unsupported`.
    Structural syntax failures use the existing syntax diagnostic family.
    Internal invariant failures remain typed Rust errors. No new diagnostic code
    is allocated.

13. **Semantic Graph.** Actor-bearing file-mode `ling.semantic/0.1` snapshots
    replace the exact Actor extension version `x-ling-actor/0.1` with
    `x-ling-actor/0.2`. Each Actor entry adds the validated capacity, policy,
    canonical mailbox bytes, and original mailbox spans. The isolated reader
    checks exact version, Actor ownership, bounds, policy, canonical bytes,
    spans, ordering, and correspondence with the existing message schema. It
    remains data-only and cannot construct a queue or executable Core.

14. **Compatibility.** Because DEC-0270 Actor syntax is Experimental, the
    incompatible migration is explicit: every prior checked Actor declaration
    must insert `mailbox capacity 1 overflow Reject` (or another accepted
    capacity) before `state`. Actor consumers must require `x-ling-actor/0.2`;
    there is no automatic JSON adapter from 0.1 because 0.1 lacks a capacity.
    Non-Actor `ling.semantic/0.1` bytes and IDs, package-aware
    `ling.semantic/0.2`, CLI exit behavior, bytecode/VM formats, and Unicode
    17.0.0 behavior remain unchanged.

15. **Completion boundary.** ACT-2303 is complete only when clauses 1 through
    14 are implemented through CST, AST, HIR, checked mailbox Core, Actor
    identity, and Semantic Graph; positive, zero/max/overflow, unsupported-policy,
    migration, deterministic reconstruction, reader-corruption, Unicode/BOM/CRLF,
    admission-boundary, bounded stress, and no-execution evidence passes;
    protocol/schema/status traceability is current; and every deferred runtime,
    suspension, loss, lifecycle, supervision, and remote capability remains
    unavailable rather than represented by a placeholder API.

## Conformance plan

- Parse and lower the exact mailbox clause with original spans; accept capacities
  1 and 65,535 and reject zero, 65,536, malformed integers, missing clauses,
  duplicate/reordered clauses, and every policy except exact `Reject`.
- Validate pure admission classification at empty, capacity-minus-one, capacity,
  and invalid over-capacity counts; exercise a large deterministic count corpus
  without allocating queued payloads.
- Freeze checked mailbox canonical bytes and Actor program identities across
  source names, comments, Unicode Actor names, BOM, LF/CRLF, and reconstructed
  checked input; require capacity changes to change Actor identity.
- Corrupt extension version, capacity, policy, canonical bytes, owner, order,
  and spans independently and require the isolated reader to reject without
  producing executable Core.
- Prove that non-Actor Semantic Graph bytes remain unchanged and Actor-bearing
  run/test/REPL/bytecode/VM/native paths still stop before execution.

## Compatibility impact

- Source: Experimental Actor declarations gain one mandatory mailbox clause;
  the exact manual migration is documented above.
- Diagnostics: no new code; `L-ACTOR-0001` gains two stable mailbox reasons.
- Schema/Semantic ID: Actor extension advances to `x-ling-actor/0.2`, and
  Actor-bearing identity includes canonical mailbox configuration. Non-Actor
  bytes and IDs remain unchanged.
- Runtime/ABI/wire: none; no queue, send, scheduler, serialization, bytecode,
  VM, native, or remote contract is implemented.
- Unicode/determinism: Unicode remains 17.0.0; canonical mailbox bytes are
  path-free and source-evidence-independent.

## Unresolved alternatives

`Wait`, silent or observable drop, coalescing and key equality, per-payload or
byte quotas, dynamic/profile capacities, queue implementation, sender ownership,
closed/stopped/Fault outcomes, cancellation, scheduling, turn reentry,
supervision, Replay, serialization, and remote delivery remain later Accepted
work. A future version may add them but cannot change `Reject`, slot capacity,
`Accepted`, or `Full` silently.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
