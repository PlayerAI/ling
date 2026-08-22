# ACT-2305 Authority Audit: Actor Runtime

## Outcome

`ACT-2305` is correctly recorded as `BlockedSpec`. The G2 plan asks for a
minimal local runtime with `spawn`/`stop`, bounded mailboxes, typed envelopes,
turn dispatch, lifecycle events, Fault provenance, Task integration, and a
non-global actor registry. None of those surfaces can be implemented safely
without Accepted Actor identity, sendability, mailbox/backpressure,
await-reentry, Task/Effect, and supervision contracts.

No Actor runtime crate, spawn/stop API, mailbox implementation, envelope
schema, dispatcher, lifecycle event, registry, scheduler integration,
diagnostic, protocol, or placeholder G2 API was added.

Accepted `DEC-0099` now authorizes the bounded child
`ACT-2305-RUNTIME-OBSERVATION`, which records only immutable runtime
observation identities and structural lifecycle labels. It does not close the
spawn, stop, dispatch, Fault, registry, scheduler, ABI, or runtime gaps
described below.

## Normative traceability

- The G2 execution package is non-normative. Its “minimal local runtime” list
  cannot authorize a new runtime crate, source syntax, ABI, public protocol, or
  lifecycle event vocabulary.
- The plan requires RFC-C203 for Actor identity/state/turn/message ownership,
  RFC-C204 for mailbox/supervision, RFC-C202 for Structured Task, and their
  implementation dependencies before ACT-2305. No Accepted RFC-C202/C203/C204
  or replacement RFC-0008/RFC-0009 exists; RFC-0001 remains a Draft baseline
  under DEC-0018. ACT-2301 through ACT-2304 are `BlockedSpec`.
- `docs/SEMANTICS.md` describes future Actor identity, private state, bounded
  mailbox, Sendable messages, one-turn processing, supervision, and
  `RemoteActorRef`, but v0.0.1 implements only the first twelve Core forms and
  `Console.Write`; Actor, Task, `SendActor`, `ReceiveActor`, and handler forms
  are not executable. The design does not fix runtime ownership, thread/task
  integration, event schemas, registry lifetime, or shutdown protocol.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require lifecycle/error
  observability, explicit bounded delivery, and resource cleanup, but do not
  define stable runtime commands, event payloads, scheduling guarantees,
  process boundaries, or compatibility/migration rules.
- Accepted DEC-0010 covers current Seed State and Capability authorization;
  DEC-0013 covers main/runtime failures; DEC-0018 covers RFC lifecycle; and
  RFC-0020 explicitly excludes Ling Task/Actor cancellation, schedulers,
  logical heaps, and replay. None authorizes a local Actor runtime.
- `GAP-ACTOR-AWAIT-REENTRY-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`,
  `GAP-ACTOR-REMOTE-DELIVERY-001`, and `GAP-STRUCTURED-TASK-001` remain Open
  and block the required runtime boundaries.

## Current implementation evidence

- The workspace contains no Actor, Task, scheduler, mailbox, supervision, or
  runtime crate. `ling-eval` and `ling-vm` execute Seed checked Core/bytecode;
  VM cancellation is a host request boundary and not Actor lifecycle or Task
  cancellation propagation.
- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no Actor declaration, ActorRef/RemoteRef runtime identity, typed envelope,
  lifecycle event, turn dispatcher, mailbox ownership, or runtime registry.
- `ling-semantic` has no accepted Semantic Graph node or schema for Actor
  instances, mailbox state, lifecycle events, Fault provenance, or runtime
  handles. No public protocol inventory entry or stable diagnostic defines
  `spawn`, `stop`, delivery failure, actor termination, or registry shutdown.
- Existing recursion, compiler-query scheduling, and VM frame/resource limits
  are unrelated implementation mechanisms; they do not establish Actor
  interleaving, identity reuse, mailbox delivery, Task ownership, or cleanup.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. runtime identity and ownership: ActorTypeId/ActorId allocation and reuse,
   ActorRef lifetime, registry scope, root ownership, handle capabilities,
   isolation, and local versus remote identity boundaries;
2. the accepted Typed Core and runtime ABI for spawn, stop, send, receive,
   typed envelopes, mailbox capacity/policies, turn dispatch, await/reentry,
   Task integration, and scheduler interaction;
3. lifecycle states and transitions (starting, running, suspended, stopping,
   stopped, failed, restarting if applicable), idempotent stop behavior,
   queued-message disposition, delivery results, and resource accounting;
4. Fault provenance, diagnostic/schema identifiers, supervisor visibility,
   cancellation and shutdown cleanup, host/process termination behavior, and
   prohibition of leaked host unwinds or global mutable language state;
5. deterministic ordering and replay boundaries, local/remote serialization,
   security/Capability checks, version migration, limits, and public protocol
   stability; and
6. executable positive/negative/interleaving/migration/stress fixtures for
   identity stability/reuse, spawn/stop races, typed envelopes, full/closed
   mailboxes, turn/await behavior, cancellation/Fault cleanup, registry
   lifetime, resource limits, Unicode/CRLF/BOM spans, deterministic output,
   and interpreter/VM/runtime differential behavior without unchecked-AST
   execution.

Until these decisions are Accepted, a runtime implementation would freeze
identity, liveness, ordering, cleanup, scheduling, and failure-recovery
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

`ACT-2305` can begin only after ACT-2301 through ACT-2304, TASK-2203, and
Accepted RFC-C202/C203/C204 (or replacement RFC-0008/RFC-0009) resolve Task
ownership, Actor identity, message sendability, mailbox/backpressure,
turn/reentry, supervision, and local/remote boundaries. The future runtime
must consume accepted types and checked Core only, expose the accepted
versioned ABI and lifecycle state machine, keep registry ownership explicit,
and publish bounded resource, cleanup, ordering, and interpreter/VM evidence
before Actor execution becomes available.
