# ACT-2301 Authority Audit: Actor Identity and State Isolation

## Outcome

`ACT-2301` is correctly recorded as `BlockedSpec`. The G2 plan proposes
`ActorTypeId`, runtime `ActorId`, typed `ActorRef<Message>`, turn-local state
mutation, prohibition of borrowing `&mut state` to outside code, and a strict
separation between local ActorRef serialization and RemoteRef. The Actor
identity, turn, state, message-ownership, reentry, and remote-boundary
contracts are not accepted.

No Actor syntax, ActorTypeId/ActorId model, ActorRef type, state-isolation
checker, turn runtime, borrow rule, serialization schema, diagnostic
allocation, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative; its identity/state checklist does
  not authorize source syntax, runtime identity, or a local/remote wire
  boundary.
- The plan explicitly requires Accepted RFC-C203 for Actor identity, state
  isolation, turn, await reentry, and message ownership before ACT work. No
  Accepted RFC-0009 or replacement exists; RFC-0001 remains a Draft baseline
  under DEC-0018.
- `docs/SEMANTICS.md` describes ActorRef, private state, turn-local mutation,
  Borrow restrictions, mailbox, ordering, supervision, and RemoteRef as future
  design. It also leaves Actor `await` reentry and remote delivery among the
  questions that require an RFC, and v0.0.1 explicitly excludes Actor.
- `GAP-ACTOR-AWAIT-REENTRY-001` leaves state invariants and reentry at await
  open. `GAP-ACTOR-MAILBOX-SUPERVISOR-001` and
  `GAP-ACTOR-REMOTE-DELIVERY-001` leave message ordering, bounded delivery,
  supervision, serialization, and local/remote guarantees open.
- Accepted DEC-0010 defines current Seed local State effects and host
  Capability authorization, not Actor-owned state, turn boundaries, or
  ActorRef identity. RFC-0020 defines only host VM cancellation and cannot be
  generalized to Actor turns.

## Current implementation evidence

- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no Actor declaration, ActorRef/RemoteRef type, ActorTypeId/ActorId, turn
  context, message schema, or state-isolation check.
- The current interpreter supports only checked Seed local mutable bindings;
  it has no Actor state store, turn scheduler, mailbox, or cross-turn borrow
  boundary. The VM has no Actor runtime or Actor identity table.
- Existing `State<T>` effect and local `PlaceAssign` semantics cannot be used
  as Actor state: they intentionally describe current-function Seed locals and
  do not prevent aliasing or turn reentry across a future Actor boundary.
- No fixture or public schema covers Actor identity stability/reuse, typed
  references, state mutation outside a turn, `&mut` escape, await reentry,
  message ownership, local/remote serialization separation, or differential
  Actor execution.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Actor declaration and Checked Core representation, `ActorTypeId` and
   `ActorId` identity scope/lifetime/reuse, typed `ActorRef<Message>` creation,
   comparison, capability, ownership, source spans, Semantic IDs, and
   local/remote identity separation;
2. turn execution and state isolation: allowed reads/writes, `&mut` and
   Resource rules, borrow/aliasing across await, reentry policy, atomicity,
   cancellation/Fault/cleanup boundaries, and interaction with Effect,
   Capability, Task, and supervision;
3. message Sendability, schema identity, move/copy/Managed rules, prohibition
   on capability forgery or state references in messages, mailbox/ordering and
   local-vs-remote serialization boundaries;
4. interpreter and VM Actor runtime/ABI, scheduler and identity table
   behavior, deterministic/replay classes, diagnostics, protocol/schema
   versioning, migration, privacy, resource limits, and malformed reference
   rejection; and
5. executable positive/negative/migration/differential fixtures for identity
   stability/reuse, multiple turns, nested/recursive sends, state isolation,
   mutable-borrow escape, await reentry, cancellation/Fault cleanup, message
   ownership, local/remote separation, Unicode/CRLF/BOM spans, deterministic
   output, and no unchecked-AST execution.

Until these decisions are Accepted, an Actor could expose private state,
resume with an invalid borrow, process messages during an unsafe suspension,
confuse local and remote identities, or make replay and VM behavior diverge.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
RFC-0001, RFC-0020, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, evaluator, bytecode, VM,
diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, Actor runtime, diagnostic, schema,
Semantic ID, source-span, scheduler, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`ACT-2301` can begin only after Accepted RFC-C203/RFC-0009 (or replacement),
resolution of the Actor reentry, mailbox/supervision, and remote-delivery gaps,
and the required Task/Effect boundaries. The future implementation must lower
only accepted Actor syntax to checked Core, enforce turn-local state and
Sendable messages, keep local/remote identity distinct, and publish
interpreter/VM and isolation evidence before exposing Actor execution.
