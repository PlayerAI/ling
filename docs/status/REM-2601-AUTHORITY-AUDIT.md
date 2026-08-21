# REM-2601 Authority Audit: RemoteRef and Endpoint

## Outcome

`REM-2601` is correctly recorded as `BlockedSpec`. The G2 plan requires a
typed `RemoteRef<Message>` distinct from local `ActorRef`, with EndpointId,
RemoteActorId, ProtocolVersion, and CapabilityToken, and explicitly forbids
serializing an ActorRef directly as a network address. No accepted RFC defines
remote identity allocation/reuse, endpoint authority, protocol negotiation,
capability authentication, incarnation/liveness, or local/remote semantics.

No RemoteRef type, endpoint registry, remote identity, capability token,
network Effect, delivery/Fault type, transport adapter, diagnostic, protocol,
or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its RemoteRef sketch cannot
  authorize network syntax, identity wire fields, authentication, or a public
  remote protocol.
- REM-2601 requires RFC-C206 and is blocked by the Actor/message/replay
  dependencies. No Accepted RFC-C206 or replacement RFC-0009 exists; RFC-0001
  remains a Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` distinguishes `ActorRef<Message>` from
  `RemoteActorRef<Message, Failure, Delivery>` and leaves Remote Actor delivery
  strategy as a future RFC question. It does not fix endpoint identity,
  incarnation, protocol negotiation, capability-token semantics, trust roots,
  or failure/partition behavior; v0.0.1 implements no Actor/Remote Core.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit network
  Effects, remote delivery, authentication, and Capability boundaries, but do
  not define a stable remote type/ABI or transport protocol.
- Accepted DEC-0010 defines current Seed Capability authorization, DEC-0013
  main/runtime Faults, DEC-0012 Semantic IDs, and RFC-0020 host-VM boundaries;
  none authorizes remote identity or network delivery.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open and blocks REM-2601 through
  REM-2605, requiring partition, duplication, ordering, migration, and
  security evidence.

## Current implementation evidence

- The workspace has no Actor runtime, RemoteRef, endpoint registry, network
  Effect, transport, authentication hook, delivery/Fault type, or remote
  protocol. `ling-eval`/`ling-vm` execute Seed locally and have no network
  boundary.
- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no remote declaration or local/remote identity judgment. `ling-semantic` and
  protocol inventory have no accepted remote identity/schema entry.
- No diagnostic or fixture defines endpoint mismatch, stale incarnation,
  revoked capability, protocol version negotiation, partition, or remote
  delivery failure.
- Existing Semantic IDs/canonical bytes do not imply network identity or
  authorization tokens; compiler-query scheduling is unrelated.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. local Actor versus RemoteRef identity, EndpointId/RemoteActorId allocation,
   incarnation/reuse, lifetime/liveness, equality, serialization prohibition,
   and typed reference ownership;
2. endpoint discovery/authority, protocol-version negotiation, CapabilityToken
   trust/issuance/revocation/attenuation, authentication/authorization,
   privacy, and local/remote security boundaries;
3. network and ActorSend Effects, delivery/Fault results, timeout/partition/
   disconnect behavior, mailbox/backpressure interaction, ordering/retry/
   deduplication, replay, migration, and resource limits;
4. canonical identity/schema bytes, diagnostics, Semantic Graph/Audit Source
   projection, protocol inventory/versioning, and compatibility behavior; and
5. executable positive/negative/migration/partition/duplication/ordering/
   security fixtures covering stale identities, endpoint mismatch, protocol
   negotiation, capability revoke, Unicode/CRLF/BOM spans, deterministic
   output, and interpreter/VM/runtime behavior without unchecked-AST
   execution.

Until these decisions are Accepted, implementing RemoteRef or endpoints would
freeze identity, security, failure, interoperability, and delivery guarantees
that the language authority intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0012, DEC-0013,
DEC-0018, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
RemoteRef, endpoint, network, diagnostic, schema, Semantic ID, source-span,
runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`REM-2601` can begin only after Accepted RFC-C206 (or replacement RFC-0009),
ACT-2301 through ACT-2306, REP-2501 through REP-2506, and the Effect/Task
dependencies resolve local/remote identity, sendability, delivery, replay,
security, and failure boundaries. The future implementation must consume
accepted types and checked Core only, keep local ActorRef distinct from
RemoteRef, and publish identity, authentication, migration, partition, and
interpreter/VM evidence before exposing remote actors.
