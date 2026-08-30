# ACT-2302 Authority Audit: Actor Message Sendability Checking

## Outcome

`ACT-2302` is correctly recorded as `Ready`. Its dependency `ACT-2301` is
`Done` under Accepted `DEC-0270`, and Accepted `DEC-0271` now authorizes the
bounded checked-only local Value profile, canonical local message schema, and
Experimental Semantic Graph extension needed by this task.

Acceptance changes authority only. No Sendable judgment, message schema,
Semantic Graph extension, diagnostic behavior, or placeholder G2 API has yet
been implemented; those changes remain ACT-2302 work and must satisfy the
Accepted decision's completion boundary before the task can become `Done`.

Accepted `DEC-0096` authorizes the bounded child
`ACT-2302-MESSAGE-SCHEMA-MODEL`, which records only immutable schema/field
identities. It does not close any Sendable, ownership, Capability, payload,
serialization, mailbox, or runtime gap listed below.

## Normative traceability

- The G2 execution package is non-normative; its sendability checklist does
  not authorize a new type judgment, ownership model, schema, or Capability
  boundary.
- ACT-2301 is `Done` under Accepted DEC-0270. Accepted DEC-0271 replaces the
  previously missing ACT-2302 authority with a deliberately bounded
  `SendableLocal(T)` judgment; RFC-0001 remains a Draft baseline under
  DEC-0018 and does not broaden that judgment.
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

- `ling-types` checks the Seed value subset and currently uses a private
  `supports_actor_value` predicate for DEC-0270's conservative Actor boundary;
  it does not expose a named Sendable judgment,
  Borrow, Move, Resource, Managed, ActorRef, or message-schema judgments.
  `ling-effects` checks only Seed closed rows and module Capabilities.
- `ling-syntax`, `ling-ast`, and `ling-hir` carry DEC-0270's single checked
  Actor message type, while `ling-semantic` deliberately filters Actor
  definitions and has no Actor message/schema extension. The interpreter and
  VM cannot receive typed Actor messages.
- Seed's local parameter/value restriction intentionally rejects the future
  borrow/move patterns; it cannot establish safety for data crossing an Actor
  turn or a local/remote boundary.
- No fixture covers active-borrow rejection, Resource move/copy/drop,
  profile-dependent Managed sharing, Capability forgery, schema identity,
  nested message values, local/remote serialization, or interpreter/VM
  sendability equivalence.

## Authorized implementation boundary

Accepted `DEC-0271` authorizes only the following complete vertical slice:

1. publish `SendableLocal(T)` for the closed local Value types enumerated in
   clauses 2 through 6 and reject every unsupported or future category by
   default;
2. attach an immutable, canonical local message contract to
   `CheckedActorCore`, preserving the original UTF-8 message-type span;
3. derive `ling.actor-message-schema-id/v1` from the normalized checked type
   graph without host layout, path, allocation, arena, or runtime facts;
4. publish and strictly validate the optional Experimental
   `x-ling-actor/0.1` Semantic Graph extension while preserving byte-identical
   non-Actor graph output; and
5. preserve `L-ACTOR-0002` at every execution boundary and provide the
   positive, negative, recursive, determinism, corruption, Unicode/BOM/CRLF,
   collision, and no-execution evidence required by clause 15.

Resource transfer, Managed sharing, Borrow lifetimes, Capability transfer,
mailbox/backpressure, turn/reentry, runtime execution, serialization, and
remote delivery remain explicitly unauthorized. Their absence is part of the
ACT-2302 acceptance criteria, not a missing placeholder to fill.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0008, DEC-0009, DEC-0010,
DEC-0013, DEC-0018, DEC-0096, DEC-0270, Accepted DEC-0271, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, semantic, evaluator,
bytecode, VM, and schema crates.

This acceptance transition changes governance authority and task readiness
only. No compiler, interpreter, VM, bytecode, ownership checker, Actor
protocol, diagnostic, schema, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

The DEC-0096 child implementation report remains focused evidence for its
publish-disabled identity boundary; it is not a substitute for ACT-2302.

## Intentionally deferred

Accepted DEC-0271 permits ACT-2302 to begin now. It keeps Resource, Managed,
Borrow, Capability transfer, mailbox/runtime, and remote delivery rejected, so
those broader contracts remain under later tasks and registered gaps. The
implementation must consume checked types and Checked Actor Core only,
default-deny unsupported categories, register canonical local message schemas,
publish validated Semantic Graph evidence, and preserve the explicit
no-execution boundary.
