# REM-2602 Authority Audit: Transport-Neutral Envelope

## Outcome

`REM-2602` is correctly recorded as `BlockedSpec`. The G2 plan lists protocol
version, sender/receiver semantic type IDs, message schema/message/correlation
IDs, deadline/cancellation, delivery policy, authentication metadata, and a
payload checksum, while explicitly stating that serialization must be decided
by an RFC and that the first implementation format is not language semantics.
No accepted RFC fixes canonical bytes, extension/version rules, identity
binding, authentication, payload encoding, cancellation/deadline semantics,
or resource limits.

No envelope struct, serializer/deserializer, checksum, protocol version,
authentication metadata, transport adapter, diagnostic, schema, or placeholder
G2 API was added.

## Normative traceability

- The G2 execution package is non-normative and expressly defers the
  serialization format to an RFC. Its field list cannot authorize a wire ABI.
- REM-2602 requires RFC-C206 and depends on REM-2601 plus Actor/message/replay
  authority. No Accepted RFC-C206 or replacement RFC-0009 exists; RFC-0001
  remains Draft under DEC-0018.
- `docs/SEMANTICS.md` distinguishes local ActorRef/RemoteActorRef and requires
  remote messages to be serializable with schema identity, but does not define
  envelope canonicalization, field encoding, extension negotiation,
  authentication, cancellation/deadline, or checksum semantics; v0.0.1 has no
  Actor/Remote Core.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit network
  Effects, message schemas, security, and partial-failure behavior, but do not
  define a stable envelope protocol or transport-neutral ABI.
- Accepted DEC-0012 defines existing Semantic ID/canonical bytes, not a remote
  message envelope; DEC-0010/DEC-0013 define Seed authorization/Faults only;
  RFC-0020 excludes Task/Actor/network protocols. None authorizes this wire
  surface.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open and blocks REM-2602, requiring
  delivery, ordering, partition, duplication, migration, and security evidence.

## Current implementation evidence

- The workspace has no RemoteRef, envelope, serializer, decoder, network
  Effect, authentication metadata, checksum, deadline/cancellation protocol,
  or transport adapter. `ling-eval`/`ling-vm` execute Seed locally.
- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no remote message schema or envelope judgment. `ling-semantic`, bytecode,
  and protocol inventory contain no accepted remote envelope schema.
- No diagnostic or fixture defines malformed envelopes, version mismatch,
  wrong sender/receiver type, schema mismatch, deadline/cancellation,
  authentication failure, checksum failure, or frame/resource limits.
- Existing Semantic IDs/canonical bytes and bytecode formats cannot be reused
  as network envelopes without an accepted remote protocol.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. canonical envelope bytes, field order/encoding, required/optional/extension
   fields, protocol-version negotiation, unknown-version/field behavior, and
   schema/Program/Semantic ID binding;
2. sender/receiver identity and incarnation binding, message/correlation IDs,
   deadline/cancellation units and propagation, delivery policy, ordering,
   retries/duplicates, and replay interaction;
3. payload serialization and canonical checksum/integrity scope, size/depth/
   resource limits, compression/framing, Capability/authentication metadata,
   trust/revocation/privacy, and local/remote boundaries;
4. typed network/ActorSend Effects and Faults, diagnostics, Semantic Graph/
   Audit Source projection, protocol inventory/versioning, migration, and
   transport adapter responsibilities; and
5. executable positive/negative/migration/partition/duplication/ordering/
   security/resource fixtures covering malformed/oversized envelopes,
   version/schema mismatch, identity/capability failure, deadline/cancel,
   checksum failure, Unicode/CRLF/BOM spans, deterministic output, and
   interpreter/VM/runtime behavior without unchecked-AST execution.

Until these decisions are Accepted, implementing an envelope would freeze wire
bytes, compatibility, security, and failure semantics that the authority
intentionally leaves open.

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
RemoteRef, envelope, network, diagnostic, schema, Semantic ID, source-span,
runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`REM-2602` can begin only after Accepted RFC-C206 (or replacement RFC-0009),
REM-2601, ACT-2301 through ACT-2306, and REP-2501 through REP-2506 resolve
identity, sendability, delivery, replay, security, serialization, and
resource boundaries. The future implementation must consume accepted types and
checked Core only, register a versioned canonical envelope, and publish
malformed/compatibility/security, partition, and interpreter/VM evidence before
exposing remote messaging.
