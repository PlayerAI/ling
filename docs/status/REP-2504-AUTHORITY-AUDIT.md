# REP-2504 Authority Audit: Replay Player

## Outcome

`REP-2504` is Done only for the private Experimental baseline authorized by
Accepted DEC-0282 and bound to implementation commit
`c3e05bac815e55ace2bef58779094869f342b237`. REP-2501 through REP-2503 are also
Done only for their registered private Experimental baselines. Accepted
DEC-0267 supplies the validated in-memory Task trace and strict fresh-runtime
replay path that this evidence exercises.

The remaining public blocker is explicit: no Accepted RFC-C205, RFC-0010, or
replacement defines checkpoint identity, persisted log input, public binding
fields, privacy, integrity, migration, cross-process equivalence, or a Replay
Player protocol. Accepted DEC-0282 therefore authorizes only a private
five-case executable evidence slice over the existing DEC-0267 Task replay
path.

No production or public replay player, checkpoint format, event-log reader,
CLI command, diagnostic, protocol, or placeholder G2 API was added. The new
matrix validates only the existing private replay path and its first-divergence
behavior.

## Normative traceability

- The G2 execution package is non-normative. Its preflight list and player
  sketch cannot authorize a replay command, mismatch behavior, checkpoint ABI,
  or cross-process equivalence promise.
- REP-2504 depends on REP-2501 through REP-2503. Those tasks are now Done only
  for private Experimental baselines under DEC-0279 through DEC-0281. No
  Accepted RFC-C205 or replacement RFC-0010 exists, and RFC-0001 remains a
  Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` describes replay as Checkpoint + EffectLog +
  ProgramSnapshot and requires Actor message-log fields, but does not define
  checkpoint contents/identity, validation order, event application, mismatch
  diagnostics, divergence equivalence, security, or migration. v0.0.1 has no
  Replay Core/runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require explicit replay/version
  boundaries and refusal of unsupported behavior, but do not define a stable
  player CLI, protocol, or compatibility matrix.
- Accepted DEC-0012 governs existing Semantic ID/canonical bytes, and
  DEC-0267 defines the private Task trace, exact fresh-runtime replay,
  first-divergence error, and source-independent equivalence. DEC-0280 and
  DEC-0281 add private structure and Effect-boundary evidence only; none
  authorizes a public player.
- Accepted DEC-0107 is vocabulary-only evidence for eleven proposed player
  boundaries. Accepted DEC-0282 may execute only the existing private Task
  replay path and keeps checkpoint, privacy, integrity, and migration deferred.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open and leaves schema, effect log,
  cross-process compatibility, corruption, privacy, and divergence unresolved.

## Current implementation evidence

- The workspace has no public replay player, checkpoint/event-log reader,
  offline replay command, or untrusted-input decoder. `ling-cli` executes
  source/Seed sessions only; no replay command is registered.
- `ling-eval::replay_task_schedule` validates one private typed trace, compares
  a checked runtime-recipe identity, reconstructs a fresh Task runtime, consumes
  recorded selections without seed fallback, and reports the first private
  event mismatch. It does not restore a checkpoint or interpret an Effect Log.
- Commit `c3e05bac815e55ace2bef58779094869f342b237` adds the exact five-case
  DEC-0282 matrix. It also includes every reachable accepted DEC-0012 Task Body
  ID in preflight and advances the opaque private identity domain from `/0` to
  `/1`, so changed checked Task behavior is rejected at event `0`.
- `ling-semantic` and `ling-bytecode` formats are not replay snapshots and have
  no replay compatibility fields. `ling-eval`/`ling-vm` have no event-input
  or checkpoint restore path.
- No public protocol inventory entry, diagnostic allocation, fixture, or
  compatibility lock defines mismatch categories, schema negotiation,
  checkpoint versioning, or divergence output.
- Existing differential tests compare Seed evaluation/VM behavior, not replay
  from an accepted event log or checkpoint.

## Accepted bounded authority

Accepted DEC-0282 authorizes only this internal Experimental baseline:

1. one crate-private `cfg(test)` matrix executes exactly five cases through
   checked Task sources, validated DEC-0267 traces, and fresh-runtime replay;
2. equivalent Unicode/BOM/CRLF/source-identity reconstruction must replay
   exactly, while changed checked behavior, root, or arguments fail private
   recipe preflight before scheduling;
3. complete existing mutation assertions prove structural rejection and the
   first private event divergence;
4. bounded host-Fault and deadline-cancellation traces replay to identical
   terminal, cleanup, Fault, host, and canonical trace evidence; and
5. all eleven DEC-0107 concerns retain explicit dispositions while negative
   evidence proves no checkpoint reader, public player, CLI, diagnostic,
   schema, privacy/integrity/migration, or implemented Replay protocol exists.

This Accepted decision deliberately treats private replay as deterministic
fresh-runtime re-execution, not checkpoint restoration or public event-log
application.

## Required authority for a public implementation

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

Until these decisions are Accepted, a public player could silently accept
incompatible state or turn an implementation-specific divergence policy into
language semantics.

## Evidence and compatibility

This refreshed audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, RFC-0006, RFC-0020, DEC-0010,
DEC-0012, DEC-0013, DEC-0107, DEC-0267, DEC-0280, DEC-0281, DEC-0282,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, CLI, diagnostic, and schema crates.

No Ling compiler, interpreter, VM, bytecode, mailbox, Actor protocol, public
replay player, diagnostic, schema, public Semantic ID, source-span, or Unicode
17.0.0 behavior changed. The private Task scheduler recipe identity now includes
accepted Task Body IDs and uses the unpersisted `/1` domain; this creates no
public compatibility or migration obligation.

## Intentionally deferred

REP-2504 is Done only for the private DEC-0282 test baseline. Public playback
still requires Accepted RFC-0010 or replacement authority and must define
checkpoint/log inputs, privacy, integrity, migration, diagnostics,
cross-process behavior, and interpreter/VM/backend evidence before exposure.
See `docs/status/REP-2504-IMPLEMENTATION-REPORT.md` for the exact matrix,
implementation finding, verification, compatibility impact, and deferred work.
