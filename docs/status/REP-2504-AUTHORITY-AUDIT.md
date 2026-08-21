# REP-2504 Authority Audit: Replay Player

## Outcome

`REP-2504` is correctly recorded as `BlockedSpec`. The G2 plan requires a
player to validate Program ID, Schema, profile, capabilities, configuration,
and message schema before replay, reject mismatches rather than “best effort,”
and support checkpoint plus event-log replay. No accepted replay schema,
determinism class, Effect recorder, checkpoint identity, divergence policy,
privacy rule, or player protocol exists.

No replay player, preflight validator, checkpoint format, event-log reader,
divergence engine, CLI command, diagnostic, protocol, or placeholder G2 API
was added.

## Normative traceability

- The G2 execution package is non-normative. Its preflight list and player
  sketch cannot authorize a replay command, mismatch behavior, checkpoint ABI,
  or cross-process equivalence promise.
- REP-2504 depends on REP-2501 through REP-2503 and RFC-C205. No Accepted
  RFC-C205 or replacement RFC-0010 exists; REP-2501/2502/2503 are
  `BlockedSpec`, and RFC-0001 remains a Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` describes replay as Checkpoint + EffectLog +
  ProgramSnapshot and requires Actor message-log fields, but does not define
  checkpoint contents/identity, validation order, event application, mismatch
  diagnostics, divergence equivalence, security, or migration. v0.0.1 has no
  Replay Core/runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit replay/version
  boundaries and refusal of unsupported behavior, but do not define a stable
  player CLI, protocol, or compatibility matrix.
- Accepted DEC-0012 governs existing Semantic ID/canonical bytes, DEC-0021
  covers only compiler-query determinism, and RFC-0020 excludes Task/Actor
  replay. None authorizes a player.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open and leaves schema, effect log,
  cross-process compatibility, corruption, privacy, and divergence unresolved.

## Current implementation evidence

- The workspace has no replay player, checkpoint/event-log reader, preflight
  validator, divergence comparator, or offline replay command. `ling-cli`
  executes source/Seed sessions only; no replay command is registered.
- `ling-semantic` and `ling-bytecode` formats are not replay snapshots and have
  no replay compatibility fields. `ling-eval`/`ling-vm` have no event-input
  or checkpoint restore path.
- No public protocol inventory entry, diagnostic allocation, fixture, or
  compatibility lock defines mismatch categories, schema negotiation,
  checkpoint versioning, or divergence output.
- Existing differential tests compare Seed evaluation/VM behavior, not replay
  from an accepted event log or checkpoint.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. checkpoint/program snapshot identity, canonical bytes, versioning, schema
   and class metadata, profile/target/toolchain/capability/config binding, and
   message schema identity;
2. preflight validation order and exact rejection diagnostics for every
   mismatch, unsupported/new field, missing capability, version migration, and
   corrupt/incomplete input;
3. event application semantics, Effect/Actor/Task ordering, logical time,
   retries/duplicates, checkpoint boundaries, resource limits, cancellation,
   Fault handling, divergence relation, and whether replay is read-only;
4. privacy/redaction and authorization, offline/remote boundaries, integrity,
   chunking, retention, migration, diagnostics, Semantic Graph/Audit Source
   projection, and public CLI/protocol stability; and
5. executable positive/negative/migration/cross-process/corruption/privacy/
   divergence fixtures covering valid/mismatched checkpoints, each required
   preflight field, event order, schema/version changes, Faults, cancellation,
   Unicode/CRLF/BOM spans, deterministic output, and interpreter/VM/runtime
   equivalence without unchecked-AST execution.

Until these decisions are Accepted, a player could silently accept incompatible
  state or turn an implementation-specific divergence policy into language
  semantics.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0012, DEC-0010, DEC-0013,
DEC-0018, DEC-0021, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, CLI, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
replay player, diagnostic, schema, Semantic ID, source-span, runtime, or
Unicode 17.0.0 behavior changed.

## Intentionally deferred

`REP-2504` can begin only after Accepted RFC-C205 (or replacement RFC-0010),
REP-2501 through REP-2503, and the Actor/Task dependencies resolve replay
identity, effect/event ordering, checkpoints, privacy, corruption, divergence,
and migration. The future player must validate all accepted bindings before
applying checked Core/runtime events, remain offline by default, and publish
cross-process and cross-backend evidence before exposing replay playback.
