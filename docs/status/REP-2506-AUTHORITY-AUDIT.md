# REP-2506 Authority Audit: Cross-Process Replay Acceptance

## Outcome

`REP-2506` is Done only for the private Experimental same-binary reconstruction
baseline authorized by Accepted DEC-0284 and bound to implementation commit
`e5f16355e02abb76680f6984427207ad96ae7b0a`. Its five-case matrix starts fresh
copies of the current `ling-eval` unit-test executable, clears the inherited
environment, reconstructs checked DEC-0267 Task traces from fixed in-memory
inputs, and compares complete private canonical bytes.

The public G2 plan still requires clean build caches, fixed and identified
toolchains, a versioned generator/player protocol, persisted log playback,
normative observable equivalence, Program/Schema mutation rejection,
provenance, cross-backend evidence, and cross-platform acceptance. No public
process harness, Replay protocol, cache tool, toolchain lock, comparator,
mutation validator, diagnostic, schema, or placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its two-process checklist cannot
  authorize a replay protocol, reproducibility claim, process/cache contract,
  or CI acceptance threshold.
- REP-2501 through REP-2505 are Done only for their private Experimental
  baselines. No Accepted RFC-C205 or replacement RFC-0010 exists, and RFC-0001
  remains a Draft baseline under DEC-0018.
- `docs/SEMANTICS.md` describes replay and deterministic classes but does not
  define independent-process identity, clean-cache requirements, toolchain
  provenance, observable comparison, repeat-count confidence, or mutated
  header rejection. v0.0.1 implements no Replay runtime.
- `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` require reproducible builds,
  explicit versioning/privacy, and cross-process evidence, but do not define
  a stable acceptance harness, environment matrix, or failure schema.
- Accepted DEC-0012 covers current Semantic ID/canonical bytes, DEC-0021 covers
  only compiler-query scheduling, and RFC-0020 excludes Task/Actor replay.
  None authorizes cross-process replay semantics.
- Accepted DEC-0109 records the eighteen boundary concerns without process
  execution. Accepted DEC-0284 narrowly authorizes same-binary process
  reconstruction evidence while explicitly deferring public playback,
  Program/Schema mutation refusal, provenance, cross-build, cross-backend, and
  cross-platform contracts.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open and requires cross-process,
  corruption, privacy, migration, and divergence evidence before resolution.

## Current implementation evidence

- The workspace has no public replay generator/player, event schema, persisted
  process artifact, clean-cache tool, toolchain lock, normative observable
  comparator, or Program ID/Schema mutation validator. `ling-cli` has no
  replay command.
- `replay_cross_process_execution_evidence.rs` contains the exact DEC-0284
  parent matrix and four ignored child probes. Parent cases invoke children
  only through fresh same-binary processes, require zero inherited environment
  entries, repeat the LF case three times, compare LF with BOM/CRLF, and
  distinguish changed body and argument recipes.
- Child probes build only checked Task Core from constants and emit a private
  hexadecimal transport of existing canonical trace bytes. They read no
  source, trace, cache, schema, dependency, or configuration file and use no
  network. This is not log playback or a process protocol.
- No protocol inventory entry, diagnostic allocation, fixture schema, privacy
  metadata, or compatibility lock defines acceptance results, rejected
  mutations, environment fingerprints, or nondeterministic allowances.
- The existing private Task scheduler supplies the trace under DEC-0267. Actor,
  Effect Log, bytecode, VM, cross-backend, and public Replay outputs do not
  participate in this evidence.

## Required authority before public implementation

An Accepted RFC or decision must define, at minimum:

1. process/toolchain/profile/target identity, cache/network isolation, source
   and dependency inputs, resource/time limits, environment fingerprint, and
   allowed platform differences;
2. log generation/player protocol, checkpoint/schema/Program/Semantic ID
   bindings, capability/config/message-schema validation, mutation rejection,
   privacy/corruption handling, and offline requirements;
3. the observable-result equivalence relation (values, Effects, Faults,
   ordering, Actor/Task traces, diagnostics), determinism class, repeat count,
   seed/log inputs, statistical threshold, and divergence reporting;
4. acceptance artifact schema, provenance, integrity, migration, diagnostics,
   Semantic Graph/Audit Source projection, CI/public protocol stability, and
   cross-backend comparison rules; and
5. executable positive/negative/migration/cross-process/corruption/privacy/
   divergence fixtures covering clean and warm caches, fixed toolchains,
   repeated runs, Program ID/Schema mutations, platform boundaries,
   Unicode/CRLF/BOM spans, deterministic output, and interpreter/VM/runtime
   behavior without unchecked-AST execution.

DEC-0284 avoids these unresolved choices by certifying only bounded
same-binary reconstruction of one existing private checked Task trace. Until
the remaining decisions are Accepted, that result must not be presented as
public Replay acceptance, reproducible-build evidence, log compatibility, or
cross-platform/backend equivalence.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0012, DEC-0010, DEC-0013,
DEC-0018, DEC-0021, DEC-0109, DEC-0267, DEC-0282, DEC-0283, DEC-0284,
RFC-0001, RFC-0020,
`docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`, and the current syntax, AST, HIR,
types, effects, evaluator, bytecode, VM, CLI, diagnostic, and schema crates.

No production compiler, interpreter, VM, bytecode, scheduler, mailbox, Actor
protocol, Replay behavior, diagnostic, schema, Semantic ID, source-span,
runtime, or Unicode 17.0.0 behavior changed. All new process execution and
stdout markers are confined to `cfg(test)` evidence.

## Intentionally deferred

REP-2506 is Done only for the private DEC-0284 Experimental baseline. Public
generator/player protocols, persisted logs, process/toolchain/profile/target
provenance, clean-cache tooling, observable equivalence, Program/Schema
mutation rejection, privacy, integrity, migration, divergence, cross-backend
evidence, cross-platform matrices, diagnostics, CI artifacts, and Stable
support still require Accepted RFC-0010 or replacement authority. See
`docs/status/REP-2506-IMPLEMENTATION-REPORT.md` for the exact evidence and
executed verification.
