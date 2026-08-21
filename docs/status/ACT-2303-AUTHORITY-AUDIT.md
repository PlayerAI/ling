# ACT-2303 Authority Audit: Bounded Mailbox and Backpressure

## Outcome

`ACT-2303` is correctly recorded as `BlockedSpec`. The G2 plan requires each
Actor to have an explicit mailbox capacity and an RFC-approved overflow policy,
but the repository has no accepted Actor/Mailbox/Supervision contract. The
high-level design names `Wait`, `Reject`, `DropNewest`, `DropOldest`, and
`Coalesce<Key>`; it does not fix their executable send results, effect rows,
ordering, fairness, resource accounting, termination behavior, or interaction
with supervision and Faults.

No mailbox data structure, backpressure effect, send-result type, overflow
policy, scheduler, Actor runtime, diagnostic, public protocol, or placeholder
G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its example and policy checklist
  cannot authorize a runtime queue, a public send API, or a new Effect.
- The plan requires RFC-C204 for mailbox capacity, backpressure, ordering,
  termination, restart budgets, and escalation. No Accepted RFC-C204 or
  replacement RFC-0009 exists; RFC-0001 remains a Draft baseline under
  DEC-0018. ACT-2301 and ACT-2302 are also `BlockedSpec`, so the mailbox cannot
  be typed independently of the missing Actor identity and message contracts.
- `docs/SEMANTICS.md` supplies only the future high-level constraints: a
  bounded `Nat` capacity, the named policy alternatives, same-sender local
  ordering, and a recommendation for `Wait` or `Reject`. It does not define
  capacity validation, full/closed-queue races, policy result types,
  suspension and cancellation behavior, fairness, quota ownership, or
  supervisor-visible failure transitions. v0.0.1 explicitly implements none of
  the Actor/Task/Handler Core forms.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit bounded
  mailboxes, observable backpressure, ordering, and full-queue/slow-consumer
  stress evidence, but they do not establish a stable syntax, ABI, diagnostic,
  or wire/schema contract.
- Accepted DEC-0010 defines current Seed State and Capability authorization,
  DEC-0013 defines main/runtime failures, and DEC-0018 defines the RFC-0001
  lifecycle. They do not define Actor mailbox ownership, queue limits,
  backpressure Effects, supervision, or restart semantics. RFC-0020 is
  explicitly host-VM cancellation only and excludes Ling Task/Actor
  cancellation, scheduling, and replay.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open and blocks ACT-2303 and the
  Supervisor tasks. It requires positive, negative, migration, stress,
  ordering, backpressure, and resource-limit evidence before resolution.

## Current implementation evidence

- The workspace has no Actor, mailbox, scheduler, supervision, or concurrent
  runtime crate. `ling-eval` and `ling-vm` execute the Seed checked subset only;
  their cancellation support is a host control boundary, not a Ling mailbox
  suspension or send result.
- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no accepted Actor declaration, mailbox declaration, capacity expression,
  overflow policy, `MailboxFull`/backpressure result, or mailbox Effect.
- No Semantic Graph node, schema registry entry, diagnostic allocation, or
  versioned protocol identifies mailbox capacity, send outcomes, dropped
  messages, coalescing keys, queue ownership, or supervisor Fault provenance.
- The existing deterministic compiler-query scheduling decisions do not define
  an Actor scheduler or message order. Seed tests contain no full-queue,
  slow-consumer, drop/coalesce, close/termination, cancellation, or
  interpreter/VM mailbox fixtures.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. capacity units and valid bounds (including zero, overflow, dynamic/profile
   values), ownership, memory/resource quotas, initialization, resizing, and
   deterministic accounting;
2. the typed send operation and outcomes for available, full, closed, stopped,
   cancelled, and failed receivers, including whether `Wait` is a suspension
   point or a typed Backpressure Effect and how cancellation/Fault cleanup
   resolves a waiting sender;
3. exact `Reject`, `DropNewest`, `DropOldest`, and `Coalesce<Key>` semantics,
   key equality, acknowledgement/observability, prohibited silent loss,
   Critical-profile restrictions, and migration/version behavior;
4. same-sender ordering, cross-sender ordering guarantees or nondeterminism
   class, fairness, starvation, reentrancy, batching, and interaction with the
   deterministic scheduler and replay log;
5. Actor close/termination behavior, supervisor and Fault transitions,
   restart/stop/escalate interaction, mailbox draining or dropping, resource
   release, local versus remote boundaries, diagnostics, Semantic Graph/Audit
   Source projection, and any public protocol/schema identity; and
6. executable positive/negative/migration/stress fixtures covering full and
   zero-capacity queues, slow consumers, concurrent senders, ordering,
   cancellation and shutdown cleanup, each overflow policy, coalescing,
   resource limits, Unicode/CRLF/BOM spans, deterministic output, and
   interpreter/VM equivalence without unchecked-AST execution.

Until these decisions are Accepted, implementing a queue or `send` API would
silently freeze liveness, loss, ordering, resource, and failure-recovery
semantics that the language authority intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
RFC-0001, RFC-0020, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
diagnostic, schema, Semantic ID, source-span, runtime, or Unicode 17.0.0
behavior changed.

## Intentionally deferred

`ACT-2303` can begin only after ACT-2301/ACT-2302 and Accepted RFC-C203/C204
(or replacement RFC-0009) resolve Actor identity, message sendability, turn
ownership, mailbox/supervision, and local/remote boundaries. The future
implementation must consume accepted types and checked Core only, expose
explicit capacity and typed send outcomes, implement only accepted overflow
policies, preserve the specified ordering and determinism class, and publish
bounded stress and interpreter/VM evidence before exposing Actor mailboxes.
