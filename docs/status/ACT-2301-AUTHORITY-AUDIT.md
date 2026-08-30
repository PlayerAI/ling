# ACT-2301 Authority Audit: Actor Identity and State Isolation

## Outcome

`ACT-2301` is ready for the bounded checked-only implementation authorized by
Accepted `DEC-0270`. Accepted `DEC-0090` previously closed the bounded
`ACT-2301-ACTOR-SYNTAX-REJECTION` evidence child without
adding Actor grammar. The G2 plan proposes
`ActorTypeId`, runtime `ActorId`, typed `ActorRef<Message>`, turn-local state
mutation, prohibition of borrowing `&mut state` to outside code, and a strict
separation between local ActorRef serialization and RemoteRef. DEC-0270 accepts
only the checked identity and pure state-transition subset; runtime identity
allocation, executable turns, message ownership, reentry, and remote-boundary
contracts remain unaccepted.

At its historical milestone, the rejection child proved only that an
Actor-shaped top-level declaration was rejected by the bilingual syntax
diagnostic before checked snapshot publication. It added no Actor syntax,
ActorTypeId/ActorId model, ActorRef type, state-isolation checker, turn runtime,
borrow rule, serialization schema, diagnostic allocation, or placeholder G2
API.

Accepted `DEC-0095` now authorizes the bounded child
`ACT-2301-IDENTITY-MODEL`, which records only immutable structural identities
and Local/Remote labels. It does not close any Actor semantic or runtime gap
listed below.

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
- Accepted `DEC-0090` reuses `L-SYNTAX-0010` for negative Actor-syntax
  evidence only; it does not reserve a lexer keyword or authorize positive
  Actor semantics.

## Current implementation evidence

- The accepted declaration now lowers through CST, AST, HIR, resolver, types,
  and effects into publish-disabled Checked Actor Core.
- Checked Core records deterministic Actor type identity, the unallocated
  runtime ActorId contract, a typed local reference descriptor, pure
  state-transition identities, and original UTF-8 source spans.
- The interpreter and VM still have no Actor state store, turn scheduler,
  mailbox, runtime identity table, send operation, or remote boundary.
- `crates/ling-cli/tests/actor_boundary.rs` covers checked publication,
  deterministic reconstruction, Unicode/BOM/CRLF spans, invalid declarations,
  non-first-class use, and explicit no-execution diagnostics.
- The prior syntax-rejection and identity-model child reports remain historical
  evidence; `docs/status/ACT-2301-IMPLEMENTATION-REPORT.md` records the completed
  DEC-0270 slice.

## Required authority before expanding implementation

Beyond DEC-0270's checked-only subset, an Accepted RFC or decision must define,
at minimum:

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

## Proposed resolution

`DEC-0270` is Proposed as the smallest complete checked-only ACT-2301 boundary.
It specifies one contextual Actor declaration, semantic `ActorTypeId`, an opaque
runtime-scoped `ActorId` contract without allocation, an internal typed local
`ActorRef<Message>` descriptor, and a pure non-suspending state transition whose
shape prevents state escape. It deliberately leaves Sendable/schema rules,
mailbox/backpressure, Effectful turns, await/reentry, runtime, serialization,
supervision, and remote delivery to ACT-2302 through ACT-2305 and their gaps.

DEC-0270 was Accepted on 2026-08-30. ACT-2301 may now implement only that
checked-only boundary; the current negative syntax gate remains authoritative
for malformed and out-of-profile Actor forms.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
RFC-0001, RFC-0020, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, evaluator, bytecode, VM,
diagnostic, and schema crates.

The compiler and diagnostic registry now implement DEC-0270. Interpreter, VM,
bytecode format, Actor runtime, public schemas, existing Semantic IDs,
scheduler, and Unicode 17.0.0 behavior remain unchanged.

The child implementation report and authority audit provide focused historical
evidence for the identity boundary; DEC-0270 now authorizes the bounded
checked-only Actor identity and state-isolation implementation.

## Intentionally deferred

The bounded `ACT-2301-ACTOR-SYNTAX-REJECTION` child is complete under
`DEC-0090`. Public `ACT-2301` may implement the checked-only DEC-0270 slice.
Effectful or suspending turns, Sendable expansion, mailbox/supervision, runtime,
and remote delivery remain blocked by their separate open gaps and later tasks.
The implementation must lower only the accepted Actor syntax to Checked Core,
keep local/remote identity distinct, and prove Actor execution remains
unavailable.
