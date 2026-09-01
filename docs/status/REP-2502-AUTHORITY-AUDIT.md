# REP-2502 Authority Audit: Replay Log Schema

## Outcome

`REP-2502` remains correctly recorded as `BlockedSpec`, but its registered
dependency is now satisfied: REP-2501 is Done for the private DEC-0279
baseline. The G2 plan lists a versioned replay event containing
Program/Semantic ID, toolchain/runtime, profile/target, Effect operation ID,
logical Actor/Task ID, logical sequence, input/result/Fault, schema version,
privacy metadata, and an integrity checksum. No Accepted RFC defines the
canonical encoding, event ordering, identity scope, payload types, redaction
rules, corruption behavior, or version migration.

Proposed DEC-0280 defines the smallest honest next slice: a crate-private,
non-serialized five-case structure-evidence matrix over real validated
DEC-0267 `TaskScheduleTrace` values. It keeps all thirteen DEC-0105 concerns
traceable while explicitly deferring checksum, determinism-class metadata,
toolchain, profile, migration, and privacy. No implementation may begin unless
DEC-0280 becomes Accepted.

No replay schema, encoder/decoder, event ID, checksum rule, privacy layer,
protocol inventory entry, diagnostic, fixture, or placeholder G2 API was
added.

## Normative traceability

- The G2 execution package is non-normative. Its “at least” field list cannot
  authorize a public wire format, checksum, event ordering, or retention
  policy.
- REP-2502's registered implementation dependency REP-2501 is Done under
  Accepted DEC-0279. The plan also requires RFC-C205, but no Accepted RFC-C205
  or replacement RFC-0010 exists; `GAP-DETERMINISTIC-REPLAY-001` remains Open,
  and RFC-0001 remains a Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` sketches Actor replay fields (schema ID, sender/receiver,
  delivery ID, logical time, payload hash, retry/duplicate data) but does not
  fix the event envelope, canonical bytes, ordering, version negotiation,
  payload serialization, privacy, corruption, or divergence semantics. The
  repository implements bounded private Task/Actor execution evidence but no
  public Replay Core, schema, or runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require versioned effects,
  explicit replay/privacy boundaries, and no unstable host data, but do not
  establish a stable schema or public command.
- Accepted DEC-0012 defines Seed Semantic ID/canonical-byte boundaries for the
  existing semantic protocol, not replay event encoding. DEC-0021 covers only
  internal compiler-query scheduling. Accepted DEC-0267 defines a private
  typed Task schedule trace and in-process replay; Accepted DEC-0279 completes
  a private determinism evidence matrix. Neither trace is a public Effect Log,
  and RFC-0020 excludes Task/Actor replay.
- `GAP-DETERMINISTIC-REPLAY-001` explicitly leaves canonical Effect Log versus
  higher-level event protocol, migration, privacy, corruption, and divergence
  alternatives open.

## Current implementation evidence

- The workspace has no public replay log schema, encoder/decoder, checksum,
  redaction policy, replay header, or cross-process reader. The private
  DEC-0267 `TaskScheduleTrace` has validated event identities, logical order,
  typed Task/host/terminal observations, and canonical test bytes; it is
  publish-disabled and distinct from a replay protocol.
- `ling-semantic` and `ling-bytecode` do not expose Program/Actor/Task replay
  event nodes; `ling-eval`/`ling-vm` have no public Effect recorder or event
  sink. REP-2501's private matrix adds no schema field.
- No protocol inventory entry, diagnostic allocation, fixture, or compatibility
  lock defines replay event IDs, canonical payloads, schema versions, privacy
  metadata, checksum scope, or unsupported-event behavior.
- Existing source-span and Unicode tests cannot establish replay serialization;
  VM resource/cancellation evidence does not provide replay records.

## Proposed bounded implementation authority

Proposed DEC-0280 would authorize only the following private baseline:

1. exactly five crate-private cases execute validated DEC-0267 trace envelope,
   event identity/kind/order, typed payload/terminal, mutation/limit rejection,
   and negative public-surface assertions;
2. all thirteen DEC-0105 concern labels occur once and are divided between
   seven existing private trace evidence concerns and six explicitly deferred
   public concerns;
3. canonical trace bytes remain non-serialized test evidence and are not
   promoted into an Effect Log, checksum, public schema, or compatibility
   revision;
4. Unicode/BOM/CRLF/source identity and original-span sidecars remain covered,
   while physical paths, timing, workers, addresses, allocation, debug text,
   and unspecified scheduler metrics stay outside equality; and
5. no production/public schema, writer/reader, checksum, privacy/migration
   field, diagnostic, CLI, fixture, schema-registry entry, or implemented
   protocol is added.

DEC-0280 must become Accepted before this private matrix is implemented. A
public implementation still requires an Accepted RFC defining, at minimum:

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

Until that public authority is Accepted, writing a schema would freeze wire bytes,
event order, data retention, and compatibility behavior that the authority
intentionally leaves open.

## Evidence and compatibility

This refreshed audit and Proposed DEC-0280 were checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0012, DEC-0010, DEC-0013,
DEC-0018, DEC-0021, DEC-0105, DEC-0267, DEC-0279, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
replay schema, diagnostic, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

## Intentionally deferred

`REP-2502` may begin only if DEC-0280 is Accepted, and then only as its private
non-serialized five-case structure-evidence baseline. Public Replay schema work
still requires Accepted RFC-C205/RFC-0010 (or replacement authority) to resolve
event ordering, canonical bytes, payloads, integrity, privacy, corruption,
divergence, and migration. A future public implementation must consume
accepted effect/runtime traces and checked Core only, register versioned schema
and integrity/privacy rules, and publish cross-process and cross-backend
evidence before exposing replay logs.
