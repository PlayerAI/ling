# REM-2604 Authority Audit: Minimal Reference Transport

## Outcome

`REM-2604` is correctly recorded as `BlockedSpec`. The G2 plan proposes an
in-process loopback transport for deterministic tests and independent TCP/QUIC
adapters for real transport, while requiring that transports have no business
message deserialization Capability beyond their contract and that codec
failures become Typed Faults. Those requirements depend on the still-unaccepted
remote identity, envelope, delivery, security, and failure protocols.

No loopback transport, TCP/QUIC adapter, codec, decoder, transport Capability,
Typed Fault, diagnostic, protocol, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its reference-transport choices
  cannot authorize a wire format, adapter API, Capability boundary, failure
  mapping, or public protocol.
- REM-2604 requires REM-2601 through REM-2603 and the remote-actor authority
  RFC-C206 or replacement RFC-0009. No such RFC is present or Accepted;
  RFC-0001 remains a Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` distinguishes local and remote Actor references and
  sketches delivery and replay concepts, but does not define a transport
  interface, canonical framing, codec ownership, endpoint negotiation,
  Typed-Fault mapping, or loopback equivalence; v0.0.1 implements no Actor or
  Remote Core.
- `docs/LANGUAGE.md` requires remote operations to expose timeout, partition,
  duplicate, reorder, retry, and partial failure, while
  `docs/ROADMAP-1.0.md` requires a transport-neutral protocol before a minimal
  reference transport. Neither establishes an adapter ABI or wire contract.
- Accepted DEC-0010 defines current Seed Capability authorization, DEC-0013
  defines main/runtime Faults, DEC-0012 defines Semantic IDs and canonical
  bytes, DEC-0018 records RFC-0001 lifecycle, and RFC-0020 defines host-VM
  cancellation. None authorizes a network or loopback transport.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open and blocks REM-2601 through
  REM-2605, including transport, serialization, failure, security, and
  Capability boundaries.

## Current implementation evidence

- The workspace has no Actor or Remote runtime, transport trait, loopback
  scheduler, network adapter, frame codec, decoder budget, endpoint registry,
  network Effect, or remote Typed Fault. `ling-eval` and `ling-vm` execute the
  checked Seed subset locally.
- The syntax, AST, HIR, types, effects, semantic, bytecode, and VM crates have
  no transport declaration or adapter boundary. Existing bytecode encoding,
  Semantic IDs, VM cancellation, and local file tooling are not a remote
  message framing or transport protocol.
- No diagnostic or fixture defines malformed frames, codec failure,
  transport disconnect, timeout, partition, duplicate/reorder, protocol
  mismatch, unauthorized decoder capability, or loopback-versus-independent
  process behavior. No versioned public transport protocol is registered.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. A transport-neutral interface and canonical envelope/framing contract:
   version negotiation, message/schema/correlation IDs, byte encoding,
   checksums, extension rules, size/depth limits, and malformed-frame
   rejection.
2. The boundary between transport, codec, runtime, and business-message
   deserialization, including the exact Capability set each layer receives,
   authentication/authorization, privacy, and resource accounting.
3. Loopback semantics and its relation to independent TCP/QUIC adapters:
   timing, ordering, disconnect/partition, backpressure, cancellation,
   retry/deduplication, stale incarnation, restart, and which differences are
   intentionally observable; no accidental location transparency.
4. Typed Fault taxonomy and diagnostics for codec, endpoint, protocol,
   schema, capability, timeout, cancellation, partition, and resource errors,
   plus Effects, replay/determinism, interpreter/VM parity, and migration
   behavior.
5. Canonical Semantic IDs, protocol-inventory/versioning, compatibility and
   deprecation rules, and executable positive/negative/security fixtures for
   malformed frames, unsupported versions, capability isolation, Unicode/
   CRLF/BOM spans, deterministic loopback, and independent-process behavior.

Until those decisions are Accepted, implementing a reference transport would
freeze codec authority, Capability isolation, failure observability,
interoperability, and loopback/network equivalence that the language authority
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
RemoteRef, envelope, delivery, transport, network, diagnostic, schema,
Semantic ID, source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`REM-2604` can begin only after accepted remote identity, envelope, delivery,
and security authorities resolve the transport/codec boundary and after the
Actor, supervision, replay, and Effect/Task dependencies provide compatible
runtime contracts. The future implementation must consume accepted types and
checked Core only, keep loopback deterministic without changing language
semantics, isolate business deserialization Capabilities, and publish codec,
failure, partition, security, and interpreter/VM evidence before exposing a
reference transport.
