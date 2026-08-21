# ACT-2302 Authority Audit: Actor Message Sendability Checking

## Outcome

`ACT-2302` is correctly recorded as `BlockedSpec`. The G2 plan requires the
checker to reject messages carrying active cross-turn borrows, define
Resource move/copy behavior, use profile-specific Managed sharing rules,
prevent Capability forgery through ordinary messages, and place message
schemas in the Semantic Graph. No accepted Actor, ownership, profile, or
message-schema contract fixes these judgments.

No Sendable trait/judgment, message type checker, cross-turn borrow rule,
Resource move/copy pass, Managed sharing rule, Capability filter, message
schema, diagnostic allocation, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative; its sendability checklist does
  not authorize a new type judgment, ownership model, schema, or Capability
  boundary.
- ACT-2301 is `BlockedSpec` and the plan requires Accepted RFC-C203 for Actor
  message ownership. No Accepted RFC-0009/C203 exists; RFC-0001 remains a
  Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` describes future Borrow/Move/Resource, Managed,
  ActorRef, message Sendability, and Capability isolation, but v0.0.1 Seed
  explicitly does not implement Ownership/Borrow Checker, Resource, Task, or
  Actor. The design text does not fix profile-specific sharing, schema
  identity, or migration.
- Accepted DEC-0008/DEC-0009 define Seed value and borrow/mutation boundaries,
  while DEC-0010 defines Seed State and Capability authorization. They do not
  authorize cross-turn Actor borrows, Resource transfer, Managed graph
  sharing, or message schemas.
- `GAP-ACTOR-AWAIT-REENTRY-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, and
  `GAP-ACTOR-REMOTE-DELIVERY-001` leave turn lifetime, ordering, delivery,
  serialization, and Capability boundaries open.

## Current implementation evidence

- `ling-types` checks the Seed value subset and does not expose Sendable,
  Borrow, Move, Resource, Managed, ActorRef, or message-schema judgments.
  `ling-effects` checks only Seed closed rows and module Capabilities.
- `ling-syntax`, `ling-ast`, `ling-hir`, and `ling-semantic` have no Actor
  message declaration, schema identity, ownership facts, or Semantic Graph
  message node. The interpreter and VM cannot receive typed Actor messages.
- Seed's local parameter/value restriction intentionally rejects the future
  borrow/move patterns; it cannot establish safety for data crossing an Actor
  turn or a local/remote boundary.
- No fixture covers active-borrow rejection, Resource move/copy/drop,
  profile-dependent Managed sharing, Capability forgery, schema identity,
  nested message values, local/remote serialization, or interpreter/VM
  sendability equivalence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the Sendable judgment and closed set of message types, recursive/nominal
   rules, type variables and variance, ActorRef handling, diagnostics, source
   spans, and Checked Core representation;
2. borrow and aliasing lifetimes across a turn/await, prohibition of active
   mutable borrows, Resource move/copy/drop/duplication behavior, Managed
   sharing and profile constraints, and interaction with Task/Effect;
3. Capability non-forgery, capability-bearing message restrictions, authority
   transfer, local versus remote message boundaries, schema identity,
   canonical bytes, versioning, migration, and privacy/security rules;
4. interpreter and VM message ABI/verifier, mailbox/backpressure/order
   interactions, Fault/cleanup behavior for rejected or failed sends,
   Semantic Graph/Audit Source projection, resource limits, and deterministic
   behavior; and
5. executable positive/negative/migration/differential fixtures for active
   borrow rejection, move/copy/drop, Managed profiles, Capability forgery,
   nested/recursive messages, schema/version mismatch, local/remote
   serialization, cancellation/Fault cleanup, Unicode/CRLF/BOM spans,
   deterministic output, and no unchecked-AST execution.

Until these decisions are Accepted, a message could leak mutable state or
Capability authority, double-move a Resource, use profile-incompatible
sharing, or become an unstable wire/schema contract.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0008, DEC-0009, DEC-0010,
DEC-0013, DEC-0018, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, semantic, evaluator,
bytecode, VM, and schema crates.

No compiler, interpreter, VM, bytecode, ownership checker, Actor protocol,
diagnostic, schema, Semantic ID, source-span, runtime, or Unicode 17.0.0
behavior changed.

## Intentionally deferred

`ACT-2302` can begin only after ACT-2301 and Accepted RFC-C203/RFC-0009 (or
replacement) resolve the Actor identity/turn, message ownership, mailbox,
remote-delivery, and profile boundaries. The future checker must consume
accepted types and checked Core only, reject cross-turn borrows and forged
Capabilities, enforce Resource/Managed rules, register canonical message
schemas, and publish interpreter/VM sendability evidence before exposing Actor
messages.
