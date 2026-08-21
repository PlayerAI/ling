# REP-2506 Authority Audit: Cross-Process Replay Acceptance

## Outcome

`REP-2506` is correctly recorded as `BlockedSpec`. The G2 plan requires two
independent processes with clean caches and a fixed toolchain to generate a
log, replay it, compare observable results, repeat the run N times, and reject
mutated Program IDs or Schemas. No accepted definition fixes process
isolation, toolchain identity, cache scope, observable-equivalence relation,
repeat count/statistics, mutation diagnostics, or cross-platform boundary.

No cross-process harness, replay acceptance test, process fixture, cache
protocol, comparison relation, mutation tool, diagnostic, protocol, or
placeholder G2 API was added.

## Normative traceability

- The G2 execution package is non-normative. Its two-process checklist cannot
  authorize a replay protocol, reproducibility claim, process/cache contract,
  or CI acceptance threshold.
- REP-2506 depends on REP-2501 through REP-2505 and RFC-C205. No Accepted
  RFC-C205 or replacement RFC-0010 exists; all preceding REP tasks are
  `BlockedSpec`, and RFC-0001 remains a Draft baseline under DEC-0018.
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
- `GAP-DETERMINISTIC-REPLAY-001` remains Open and requires cross-process,
  corruption, privacy, migration, and divergence evidence before resolution.

## Current implementation evidence

- The workspace has no replay generator/player, event schema, process harness,
  clean-cache isolation, toolchain lock, observable-result comparator,
  repeatability runner, or Program ID/Schema mutation validator. `ling-cli`
  has no replay command.
- Existing compiler and VM tests run in one repository process/environment and
  do not provide replay logs or cross-process equivalence evidence. DEC-0021's
  compiler-query tests are not runtime replay tests.
- No protocol inventory entry, diagnostic allocation, fixture schema, privacy
  metadata, or compatibility lock defines acceptance results, rejected
  mutations, environment fingerprints, or nondeterministic allowances.
- No Actor/Task/Effect runtime exists whose observable outputs could be
  compared across independent processes.

## Required authority before implementation

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

Until these decisions are Accepted, a cross-process test could certify local
environment coincidence or hide incompatible logs behind an unspecified
comparison rule.

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
replay acceptance harness, diagnostic, schema, Semantic ID, source-span,
runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`REP-2506` can begin only after Accepted RFC-C205 (or replacement RFC-0010)
and REP-2501 through REP-2505 resolve replay identity, process/toolchain
provenance, observable equivalence, privacy, corruption, divergence, and
migration. The future harness must run accepted generator/player protocols in
independent clean environments, fail closed on Program/Schema mismatch, and
publish repeatable cross-process and cross-backend evidence before replay
acceptance is claimed.
