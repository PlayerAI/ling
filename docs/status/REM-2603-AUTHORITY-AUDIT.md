# REM-2603 Authority Audit: Delivery Semantics

## Outcome

`REM-2603` is correctly recorded as `BlockedSpec`. The G2 plan permits only
delivery modes explicitly supported by an accepted RFC (for example,
at-most-once or an explicitly repeatable mode) and prohibits presenting retry
as “exactly once”. It also requires observable behavior for timeout,
disconnect, duplication, reordering, stale actor incarnations, remote restart,
incompatible message schemas, and revoked capabilities. No accepted RFC fixes
those guarantees or their failure protocol.

No delivery-policy type, retry or deduplication algorithm, ordering contract,
remote Fault, capability-revocation path, transport adapter, diagnostic,
protocol, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its delivery examples and test
  list cannot authorize a language type, runtime guarantee, network behavior,
  or public protocol.
- REM-2603 requires the remote-actor authority (RFC-C206 or its replacement
  RFC-0009), but no such RFC is present or Accepted; RFC-0001 remains a Draft
  baseline under DEC-0018. REM-2601 and REM-2602 therefore do not supply a
  delivery contract either.
- `docs/SEMANTICS.md` sketches `RemoteActorRef<..., Delivery>` with
  `AtMostOnce`, `AtLeastOnce`, and `IdempotentRetry<Key>` and rejects an
  unconditional Exactly Once claim. Its replay section names delivery and
  retry metadata, but it does not define outcomes, deduplication identity,
  ordering across senders, partition behavior, incarnation/restart rules, or
  the checked Core/runtime protocol; v0.0.1 has no Actor or Remote Core.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit remote
  delivery, network Effects, Faults, authentication, and Capability boundaries
  but do not establish a stable delivery ABI or interoperability contract.
- Accepted DEC-0010 covers current Seed Capability authorization, DEC-0012
  covers Semantic IDs and canonical bytes, DEC-0013 covers main/runtime
  Faults, DEC-0018 records RFC-0001 lifecycle, and RFC-0020 covers host-VM
  cancellation. None defines remote delivery guarantees or retries.
- `GAP-ACTOR-REMOTE-DELIVERY-001` remains Open, blocks REM-2601 through
  REM-2605, and explicitly lists ordering, retry/deduplication, delivery
  guarantees, transport failure, security, and Capability boundaries as
  unaccepted.

## Current implementation evidence

- The workspace has no Actor or Remote runtime, delivery-policy value,
  network Effect, remote Fault, mailbox-to-transport bridge, retry queue,
  deduplication store, ordering ledger, incarnation registry, or capability
  revocation hook. `ling-eval` and `ling-vm` execute the checked Seed subset
  locally.
- The syntax, AST, HIR, type, effect, semantic, bytecode, and VM crates have
  no remote delivery declaration or judgment. Existing local mailbox, VM
  cancellation, Semantic ID, and replay-related planning material cannot be
  treated as a remote delivery protocol.
- No diagnostic or fixture defines timeout/disconnect/partition, duplicate or
  reordered delivery, stale incarnation, restart recovery, schema mismatch,
  or revoked Capability outcomes. No versioned public protocol is registered
  for these behaviors.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Delivery classes and their observable guarantees, including loss and
   duplicate behavior; explicit limits on retry claims; idempotence-key scope,
   lifetime, collision handling, and deduplication state.
2. Ordering and causality rules for one sender, multiple senders, retries,
   duplicates, reordering, timeouts, disconnects, partitions, stale actor
   incarnations, remote restarts, and endpoint migration.
3. Interaction with the canonical envelope, message schema/version checks,
   deadlines/cancellation, bounded mailboxes/backpressure, authentication,
   Capability issuance/revocation, privacy, resource limits, and failure
   observability through Effects and Faults.
4. Determinism and replay semantics: logical time, delivery IDs, retry and
   duplicate records, interpreter/VM/runtime parity, and what nondeterminism
   is intentionally exposed to programs.
5. Canonical bytes and Semantic IDs, stable bilingual diagnostics, protocol
   inventory/versioning, compatibility and migration rules, and rejection of
   malformed or incompatible messages without unchecked-AST execution.
6. Executable positive, negative, migration, partition, duplication,
   ordering, restart, schema-compatibility, and security fixtures, including
   Unicode/CRLF/BOM source-span preservation and deterministic output.

Until those decisions are Accepted, implementing delivery semantics would
freeze data-loss, duplicate-side-effect, ordering, security, interoperability,
and replay guarantees that the language authority intentionally leaves open.

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
RemoteRef, envelope, delivery, network, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`REM-2603` can begin only after the remote identity/envelope authority and
the Actor, supervision, replay, and Effect/Task dependencies resolve delivery,
failure, security, and determinism boundaries. The future implementation must
consume accepted types and checked Core only, never promise unconditional
Exactly Once, and publish timeout, partition, duplication, ordering, restart,
schema, capability, and interpreter/VM evidence before exposing remote
messaging.
