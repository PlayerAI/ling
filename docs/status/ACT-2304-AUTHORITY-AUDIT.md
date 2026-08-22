# ACT-2304 Authority Audit: Actor Turn and Reentry Rules

## Outcome

`ACT-2304` is correctly recorded as `BlockedSpec`. The G2 plan requires one
Actor turn at a time, then leaves the central `await` choice open: suspend and
release the turn, forbid reentry, or permit explicitly guarded reentry. It also
requires self-send and recursive processing to go through the mailbox and a
non-destructive watchdog for long turns. No accepted RFC fixes these choices or
their interaction with state ownership, Task suspension, cancellation, mailbox
ordering, supervision, or replay.

No turn state machine, reentry guard, state-version token, await lowering,
self-send path, watchdog, scheduler hook, diagnostic, or placeholder G2 API
was added.

Accepted `DEC-0098` now authorizes the bounded child
`ACT-2304-TURN-OBSERVATION`, which records only immutable turn identities and
structural vocabulary labels. It does not close the await, reentry, state
guard, self-send, watchdog, scheduler, supervision, or runtime gaps described
below.

## Normative traceability

- The G2 execution package is non-normative. Its alternatives and high-risk
  checklist cannot authorize `await` syntax, a turn ABI, or a runtime reentry
  policy.
- The plan requires RFC-C203 for Actor identity/state isolation/turn and
  await-reentry semantics, and RFC-C204 for mailbox ordering and supervision.
  No Accepted RFC-C203/C204 or replacement RFC-0009 exists; RFC-0001 remains a
  Draft baseline under DEC-0018. ACT-2301, ACT-2302, and ACT-2303 are already
  `BlockedSpec`, so turn behavior cannot be implemented independently of their
  identity, message, and mailbox contracts.
- `docs/SEMANTICS.md` establishes the high-level constraints that an Actor
  processes one turn at a time, that an Actor-state mutable Borrow cannot leave
  the turn, that a no-`await` turn is an atomic observation boundary, and that
  `await` is a suspension point. It explicitly leaves the exact reentry rule to
  a future RFC and v0.0.1 implements neither `AwaitTask` nor Actor Core forms.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` describe structured concurrency,
  turn isolation, and the need for observable long-turn behavior, but do not
  define whether suspension releases the turn, how state versions/guards are
  represented, or how watchdog events affect program semantics.
- Accepted DEC-0008/DEC-0009 define only Seed value parameters and local borrow
  restrictions; DEC-0010 defines local Seed State/Capability behavior; DEC-0013
  defines main/runtime failures; and DEC-0018 defines RFC lifecycle. RFC-0020
  explicitly excludes Ling Task/Actor cancellation, scheduling, and replay.
  None authorizes Actor turn suspension, reentry, or self-send behavior.
- `GAP-ACTOR-AWAIT-REENTRY-001` remains Open, blocks ACT-2301, ACT-2304,
  ACT-2305, and ACT-2306, and requires positive/negative/migration,
  interleaving, state-invariant, and replay evidence before resolution.

## Current implementation evidence

- The workspace has no Actor or Task runtime, turn context, suspension
  lowering, mailbox dispatch, reentry guard, watchdog, or supervisor runtime.
  `ling-eval` and `ling-vm` execute only the Seed checked subset; VM cancellation
  is a host control boundary and is not an Actor await/reentry mechanism.
- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no accepted Actor/Task `await` form, turn token, state-version fact,
  reentrancy judgment, self-send operation, or watchdog effect.
- Existing recursive function support is ordinary call-stack behavior and does
  not imply recursive Actor message processing. Compiler-query deterministic
  scheduling decisions do not define a runtime Actor scheduler or source-level
  interleaving semantics.
- No Semantic Graph node, schema, diagnostic, replay record, or conformance
  fixture captures turn suspension, state guards, reentry interleavings,
  self-send mailbox ordering, long-turn observation, cancellation cleanup, or
  interpreter/VM equivalence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the turn lifecycle and atomic observation boundary, including when a turn
   starts/ends, whether a turn may process one or many messages, and how state
   visibility relates to mailbox dequeue and commit;
2. `await` suspension behavior: forbid, freeze-and-release, or explicit guarded
   reentry, with the Typed Core representation, state-version/guard rules,
   active-borrow restrictions, and Task/Effect interaction for each case;
3. self-send and recursive sends, including the required mailbox route,
   ordering relative to the current turn, queue capacity/backpressure result,
   and prevention of unbounded reentrant recursion;
4. cancellation, Fault, shutdown, and supervisor behavior at every suspension
   and reentry boundary, including resource cleanup and whether a suspended
   turn can be resumed, abandoned, or retried;
5. long-turn watchdog observability, limits, metrics/events, diagnostic and
   Semantic Graph/Audit Source projection, and a prohibition on force-killing
   a turn in a way that violates Resource or state semantics;
6. local/remote and replay boundaries, ordering/determinism classes, migration
   rules, and compatibility with mailbox/supervision and message schemas; and
7. executable positive/negative/interleaving/migration fixtures covering no
   `await`, each reentry choice, nested/recursive self-send, concurrent
   senders, state-version conflicts, active-borrow rejection, cancellation,
   Fault cleanup, watchdog observation, Unicode/CRLF/BOM spans, deterministic
   output, and interpreter/VM differential behavior without unchecked-AST
   execution.

Until these decisions are Accepted, a turn implementation would silently fix
state invariants, race/order behavior, liveness, replay meaning, and cleanup
semantics that the language authority intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0008, DEC-0009, DEC-0010,
DEC-0013, DEC-0018, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, Actor protocol, diagnostic,
schema, Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`ACT-2304` can begin only after ACT-2301/ACT-2302/ACT-2303 and Accepted
RFC-C203/C204 (or replacement RFC-0009) resolve Actor identity, message
sendability, mailbox/backpressure, turn lifetime, await reentry, supervision,
and local/remote boundaries. The future implementation must consume accepted
types and checked Core only, enforce turn-local state and borrow invariants,
route self-send through the accepted mailbox, expose only an accepted
watchdog, and publish interleaving, cleanup, replay, and interpreter/VM
evidence before exposing Actor execution.
