# REP-2501 Authority Audit: Determinism Class

## Outcome

`REP-2501` is correctly recorded as `BlockedSpec`. The G2 plan proposes
`Strict`, `Seeded`, `RecordedEffects`, and `BestEffort` classes and says that a
class must appear in build metadata, the Semantic Graph, and a replay header.
The repository has no accepted classification relation, claim/inference
rules, effect boundary, scheduling model, replay header, or privacy and
migration contract.

No determinism enum, class inference, build-metadata field, Semantic Graph
field, replay header, diagnostic, protocol, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its “minimum recommended” class
  names cannot freeze a public classification, metadata field, or replay ABI;
  the plan itself says final names are subject to an RFC.
- REP-2501 requires RFC-C205. No Accepted RFC-C205 or replacement RFC-0010
  exists; RFC-0001 remains a Draft baseline under DEC-0018, and the
  deterministic replay gap blocks REP-2501 through REP-2506.
- `docs/SEMANTICS.md` sketches determinism classes and Actor replay fields, but
  does not fix class ordering/meaning, inference versus declaration, effect and
  scheduler boundaries, equivalence, version migration, privacy, or
  divergence handling. v0.0.1 implements only the Seed Core subset and no
  replay runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit effects,
  deterministic observable behavior, and replay privacy/version boundaries,
  but do not define a stable class schema or user-facing compatibility rule.
- Accepted DEC-0021 defines deterministic scheduling only for independent
  internal compiler queries. It does not define runtime/Actor determinism,
  effect logs, replay headers, or cross-process equivalence. DEC-0010/DEC-0013
  and RFC-0020 likewise do not authorize replay semantics.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open, with alternative canonical-log
  versus higher-level-event designs and required positive/negative/migration,
  cross-process, corruption, privacy, and divergence evidence.

## Current implementation evidence

- The workspace has no determinism-class model, effect recorder, replay log,
  replay header, player, scheduler trace, or cross-process comparison tool.
  `ling-eval` and `ling-vm` provide Seed execution/differential evidence only.
- `ling-semantic` has no accepted determinism-class Semantic Graph node or
  build/replay metadata projection. `ling-effects` computes only the Seed
  closed effect rows and module Capability closure.
- No public protocol inventory entry, schema, diagnostic, or fixture defines
  class claims, unrecorded scheduling, external-effect boundaries, or replay
  divergence. Compiler-query scheduling evidence from DEC-0021 is intentionally
  internal and cannot be reused as runtime replay evidence.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. class names, meanings, ordering/lattice, declaration versus inference,
   composition for calls/Actors/Tasks, invalid claims, and the exact observable
   equivalence promised by each class;
2. boundaries for Clock, Random, input, network, storage/device reads,
   scheduler nondeterminism, mailbox/interleaving, Faults, and derived state;
3. canonical build metadata, Semantic Graph and replay-header fields,
   Semantic ID/version interaction, target/profile/toolchain identity,
   compatibility and migration rules, and local/remote scope;
4. privacy/redaction, sensitive payload handling, integrity, corruption and
   unsupported-class behavior, resource limits, diagnostics, and Audit Source
   projection; and
5. executable positive/negative/migration/cross-process/corruption/privacy/
   divergence fixtures covering each class, nested calls/effects, scheduler
   interleavings, Actor/Task boundaries, Unicode/CRLF/BOM spans, deterministic
   reruns, and interpreter/VM/runtime equivalence without unchecked-AST
   execution.

Until these decisions are Accepted, adding a class field or enum would turn an
unapproved replay equivalence and data-retention policy into public semantics.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0013, DEC-0018,
DEC-0021, RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, diagnostic, and schema crates.

No compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor protocol,
replay, diagnostic, schema, Semantic ID, source-span, runtime, or Unicode
17.0.0 behavior changed.

## Intentionally deferred

`REP-2501` can begin only after Accepted RFC-C205 (or replacement RFC-0010)
resolves determinism classes, effect/scheduler boundaries, replay identity,
privacy, corruption, divergence, and migration. The future implementation
must consume accepted effect/runtime traces and checked Core only, publish
versioned class metadata consistently across Semantic Graph and replay
headers, and provide cross-process and cross-backend evidence before claiming
determinism support.
