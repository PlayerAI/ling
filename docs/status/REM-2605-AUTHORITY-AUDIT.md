# REM-2605 Authority Audit: Security and Resource Limits

## Outcome

`REM-2605` is correctly recorded as `BlockedSpec`. The G2 plan lists maximum
frame/message sizes, decoder depth, mailbox ingress limits,
authentication/authorization hooks, replay protection, rate limits, unknown
schema rejection, and fuzzing of every decoder. These controls are safety and
interoperability behavior for the unaccepted remote identity, envelope,
delivery, transport, and Capability protocols.

No frame or message limit, decoder, ingress policy, authentication hook, replay
protector, rate limiter, schema gate, fuzz harness, diagnostic, protocol, or
placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its security checklist cannot
  authorize a public resource policy, authentication ABI, decoder behavior,
  or network protocol.
- REM-2605 depends on REM-2601 through REM-2604 and the remote-actor authority
  RFC-C206 or replacement RFC-0009. No such RFC is present or Accepted;
  RFC-0001 remains a Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` defines Seed Capability as an unforgeable authorization
  boundary, requires remote messages to be serializable with schema identity,
  and gives bounded local mailbox examples. It does not define remote trust
  roots, authentication, authorization/revocation, decoder quotas, replay
  protection, rate limiting, unknown-schema policy, or network resource Faults;
  v0.0.1 implements no Actor or Remote Core.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit network
  Effects/Capabilities, failure observability, security boundaries, bounded
  resources, and rejection of unsupported schemas, but do not establish a
  stable remote security or resource-limit contract.
- Accepted DEC-0010 defines current Seed Capability authorization and host
  handle injection, DEC-0013 defines main/runtime Faults, DEC-0012 defines
  Semantic IDs/canonical bytes, DEC-0018 records RFC-0001 lifecycle, and
  RFC-0020 defines host-VM cancellation. None authorizes remote
  authentication, quotas, or replay protection.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open and blocks REM-2601 through
  REM-2605, explicitly listing security and Capability boundaries alongside
  serialization, delivery, and transport failure.

## Current implementation evidence

- The workspace has no Actor or Remote runtime, frame codec, decoder, network
  Effect, authentication/authorization provider, Capability revocation hook,
  replay window, rate limiter, mailbox ingress quota, or remote resource
  accounting. `ling-eval` and `ling-vm` execute the checked Seed subset
  locally.
- The syntax, AST, HIR, types, effects, semantic, bytecode, VM, diagnostic,
  and schema crates have no remote security declaration, quota judgment, or
  transport boundary. Existing local Capability checks and host cancellation
  do not establish network authentication or resource semantics.
- No diagnostic or fixture defines oversized/deep frames, unknown schemas,
  malformed encodings, replay/duplicate attacks, revoked credentials, rate
  exhaustion, mailbox ingress overflow, authorization failure, or deterministic
  fuzz outcomes. No versioned public security protocol is registered.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Resource limits and accounting for frames, messages, nested decoder depth,
   decompression/allocation, mailbox ingress, in-flight retries, replay windows,
   connection count, and rate limits; specify whether limits are language
   semantics, Profile policy, or host configuration and how they are exposed.
2. Authentication and authorization: trust roots, credential/Capability
   issuance, attenuation, revocation, endpoint and actor binding, replay
   protection, privacy, key rotation, failure timing, and the exact boundary
   between transport, codec, runtime, and business code.
3. Unknown-schema, malformed, oversized, and resource-exhaustion behavior;
   Typed Fault categories, stable bilingual diagnostics, exit/Effect mapping,
   and no raw host error or unchecked-AST path crossing the boundary.
4. Delivery/replay integration: message and delivery identity, duplicate
   detection, ordering, stale incarnation/restart, deadline/cancellation,
   deterministic replay, and what security/resource nondeterminism is visible
   to programs.
5. Canonical bytes, Semantic IDs, protocol inventory/versioning,
   compatibility/migration and deprecation rules, plus Unicode/CRLF/BOM source
   span preservation and deterministic output requirements.
6. Executable positive, negative, migration, partition, duplication,
   ordering, security, quota, and decoder-fuzz fixtures covering both
   loopback and independent transports, with interpreter/VM/runtime parity.

Until those decisions are Accepted, implementing these controls would freeze
security guarantees, denial-of-service behavior, host-resource exposure,
interoperability, and replay expectations that the language authority
intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0012, DEC-0013,
DEC-0018, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, types, effects, evaluator, bytecode, VM,
diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
RemoteRef, envelope, delivery, transport, security, diagnostic, schema,
Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`REM-2605` can begin only after accepted remote identity, envelope, delivery,
transport, and security authorities resolve the resource and authentication
boundary, and after the Actor, supervision, replay, and Effect/Task
dependencies provide compatible runtime contracts. The future implementation
must consume accepted types and checked Core only, enforce explicit bounded
resources and least privilege, reject unknown or malformed schemas, and
publish quota, decoder-fuzz, security, replay, and interpreter/VM evidence
before exposing remote messaging.
