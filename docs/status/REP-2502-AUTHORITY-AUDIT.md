# REP-2502 Authority Audit: Replay Log Schema

## Outcome

`REP-2502` is correctly recorded as `BlockedSpec`. The G2 plan lists a
versioned replay event containing Program/Semantic ID, toolchain/runtime,
profile/target, Effect operation ID, logical Actor/Task ID, logical sequence,
input/result/Fault, schema version, privacy metadata, and an integrity
checksum. No accepted RFC defines the canonical encoding, event ordering,
identity scope, payload types, redaction rules, corruption behavior, or
version migration.

No replay schema, encoder/decoder, event ID, checksum rule, privacy layer,
protocol inventory entry, diagnostic, fixture, or placeholder G2 API was
added.

## Normative traceability

- The G2 execution package is non-normative. Its “at least” field list cannot
  authorize a public wire format, checksum, event ordering, or retention
  policy.
- REP-2502 depends on RFC-C205 and REP-2501. No Accepted RFC-C205 or
  replacement RFC-0010 exists; `GAP-DETERMINISTIC-REPLAY-001` remains Open,
  and RFC-0001 remains a Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` sketches Actor replay fields (schema ID, sender/receiver,
  delivery ID, logical time, payload hash, retry/duplicate data) but does not
  fix the event envelope, canonical bytes, ordering, version negotiation,
  payload serialization, privacy, corruption, or divergence semantics. v0.0.1
  implements no Actor/Task/Replay Core or runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require versioned effects,
  explicit replay/privacy boundaries, and no unstable host data, but do not
  establish a stable schema or public command.
- Accepted DEC-0012 defines Seed Semantic ID/canonical-byte boundaries for the
  existing semantic protocol, not replay event encoding. DEC-0021 covers only
  internal compiler-query scheduling. RFC-0020 excludes Task/Actor replay.
- `GAP-DETERMINISTIC-REPLAY-001` explicitly leaves canonical Effect Log versus
  higher-level event protocol, migration, privacy, corruption, and divergence
  alternatives open.

## Current implementation evidence

- The workspace has no replay log schema, event model, encoder/decoder,
  checksum, redaction policy, replay header, or cross-process reader. Existing
  Semantic Graph/bytecode formats are distinct from a replay protocol.
- `ling-semantic` and `ling-bytecode` do not expose Program/Actor/Task replay
  event nodes; `ling-eval`/`ling-vm` have no Effect recorder or event sink.
- No protocol inventory entry, diagnostic allocation, fixture, or compatibility
  lock defines replay event IDs, canonical payloads, schema versions, privacy
  metadata, checksum scope, or unsupported-event behavior.
- Existing source-span and Unicode tests cannot establish replay serialization;
  VM resource/cancellation evidence does not provide replay records.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. canonical event envelope and bytes, event IDs, field types, optional versus
   required fields, sequence/order rules, actor/task/effect identity, logical
   time, and checksum/integrity scope;
2. Program/Semantic ID, toolchain/runtime, profile/target, class, schema
   version, capability, and migration compatibility, including unknown/new
   event handling and cross-process negotiation;
3. input/result/Fault/payload serialization, schema identity, nested/recursive
   values, Resource/Managed/Capability privacy and authority boundaries,
   redaction, retention, encryption/integrity, and sensitive-data failure
   behavior;
4. event ordering for effects, mailbox/interleavings, retries/duplicates,
   cancellation and supervision, divergence/corruption detection, limits,
   diagnostics, Semantic Graph/Audit Source projection, and public protocol
   stability; and
5. executable positive/negative/migration/cross-process/corruption/privacy/
   divergence fixtures covering each field, omitted/unknown fields, checksum
   failures, schema mismatch, Unicode/CRLF/BOM spans, deterministic output,
   and interpreter/VM/runtime equivalence without unchecked-AST execution.

Until these decisions are Accepted, writing a schema would freeze wire bytes,
event order, data retention, and compatibility behavior that the authority
intentionally leaves open.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0012, DEC-0010, DEC-0013,
DEC-0018, DEC-0021, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
replay schema, diagnostic, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

## Intentionally deferred

`REP-2502` can begin only after Accepted RFC-C205 (or replacement RFC-0010)
and REP-2501 resolve determinism class, event ordering, canonical bytes,
privacy, corruption, divergence, and migration. The future implementation
must consume accepted effect/runtime traces and checked Core only, register a
versioned schema and integrity/privacy rules, and publish cross-process and
cross-backend evidence before exposing replay logs.
