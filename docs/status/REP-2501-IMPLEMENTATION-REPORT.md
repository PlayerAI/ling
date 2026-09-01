# REP-2501 Implementation Report

## Outcome

REP-2501 is complete only for the private Experimental baseline authorized by
Accepted DEC-0279. Implementation commit
`45ea3c9749aea87da96b03857d34d5de908593cd` adds one crate-private,
`cfg(test)` evidence matrix to `ling-eval`. It executes exactly five bounded
contracts over existing Accepted checked, Task, Actor, and Supervisor routes.

This completion does not classify Ling programs publicly. It adds no
production classifier, source annotation, build or Semantic Graph field,
Effect Log, Replay header/schema/player, diagnostic, protocol, or Stable
compatibility claim. `GAP-DETERMINISTIC-REPLAY-001` remains Open.

## Normative clauses covered

- DEC-0279 clauses 1-4 and 11: `determinism_evidence.rs` is test-only, records
  exactly the five authorized case families, and keeps the five SEMANTICS
  category shapes separate from DEC-0104's four provisional plan labels. It
  defines no lattice, alias, ordering, composition, or inference rule.
- Clauses 5-6: the pure case consumes successful immutable Checked Core,
  verifies an empty residual Effect row and Capability closure, and compares a
  bounded Value/host projection across repetition, Unicode/BOM/CRLF, source
  identity, and definition insertion-order reconstruction. Original spans are
  retained and asserted separately.
- Clause 7: the seeded case drives the real DEC-0267 scheduler with one explicit
  seed, arguments, logical deadline, runtime/scheduler limits, and deterministic
  host success/failure scripts. Repeated and reconstructed runs have identical
  validated canonical trace bytes.
- Clause 8: the input case strictly replays one complete validated DEC-0267
  trace against reconstructed Checked Core. Exact replay matches byte-for-byte;
  runtime-identity mutation fails at event 0, while selection, step, tick,
  deadline, host, and closure mutations fail at their first affected event
  without seed fallback.
- Clause 9: the schedule case directly reuses the complete DEC-0278 Unicode
  reconstruction assertion, which drives the real DEC-0274/DEC-0276/DEC-0277
  Actor/Supervisor path through finite explicit operations and compares its
  bounded containment/restart/cleanup projection.
- Clause 10: the production Task case uses only the test-local reason
  `unrecorded-local-task-scheduler`, runs the DEC-0268 local scheduler with one
  and four workers, and compares only Task paths, structurally valid terminal
  states, and exactly-once cleanup. Worker metrics and allowed Effect order do
  not enter the equality projection.
- Clauses 12-16: every input, limit, deadline, host script, and operation list
  is finite. The negative inventory checks bounded syntax/AST, effects,
  semantic/build, evaluator, CLI, diagnostics, schema-registry, and protocol
  sources; the existing `PROTO-REPLAY` record remains Future and unimplemented.
- Clause 17: focused and full gates passed, and the evidence is bound to the
  implementation commit above. Clause 18's public work remains deferred.

## Exact evidence matrix

The dedicated matrix contains exactly these case families:

1. `pure-deterministic-checked-execution`
2. `seed-deterministic-task-schedule`
3. `input-deterministic-task-replay`
4. `schedule-deterministic-actor-script`
5. `nondeterministic-production-task-boundary`

The category names are test-local labels matching the shapes named by
`SEMANTICS.md` section 22.1. `InputDeterministic<EffectLog>` does not rename the
private `TaskScheduleTrace` as a public Effect Log. Likewise,
`SeedDeterministic<RandomSource>` does not make DEC-0267 SplitMix64 a Ling
RandomSource ABI.

Existing focused assertions are reused only by direct function calls that
execute their complete bodies. The matrix does not treat test names or a
second observation model as execution evidence.

## Executed verification

Commands executed locally on 2026-09-01:

- `cargo test -p ling-eval --lib --locked --offline` — passed: 66 tests,
  including all five DEC-0279 matrix cases.
- `cargo test -p ling-eval --all-targets --locked --offline` — passed: 66 unit,
  12 Actor runtime, 13 local scheduler, 20 Task runtime, and 14 Task scheduler
  tests.
- `cargo clippy -p ling-eval --all-targets --locked --offline -- -D warnings`
  — passed.
- `cargo test -p ling-cli --test task_boundary --locked --offline` — passed:
  4 tests.
- `cargo test -p ling-cli --test actor_boundary --locked --offline` — passed:
  10 tests; the public Actor route remains `L-ACTOR-0002`.
- `cargo test --workspace --all-targets --locked --offline` — passed.
- `cargo test -p xtask --locked --offline` — passed: 174 tests.
- `cargo xtask governance check-all`, `cargo xtask status verify`,
  `cargo xtask docs verify`, and `cargo xtask rc0 verify` — passed.
- `cargo clippy -p xtask --all-targets --locked --offline -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` — passed.

## Compatibility impact

- Source, CLI/REPL/LSP/editor behavior, diagnostics, schemas, Semantic IDs,
  protocols, package/ABI versions, stored data, dependencies, and migration:
  unchanged.
- Production runtime: unchanged. The only visibility adjustments are between
  `cfg(test)` modules so the matrix can directly invoke complete existing Task
  replay and Actor/Supervisor assertions.
- Determinism and Replay: no public classification or equivalence promise is
  created. The typed in-process Task trace remains publish-disabled and is not
  a public Replay schema.
- Unicode remains 17.0.0. Original UTF-8 byte spans remain authoritative and
  are excluded from logical equality where DEC-0279 requires sidecar evidence.

## Specification gaps and deferred work

No conflict was found inside DEC-0279's scoped private evidence contract.
`GAP-DETERMINISTIC-REPLAY-001` remains Open and still tracks REP-2501 as the
completed private anchor while blocking REP-2502 through REP-2506's unresolved
public/protocol work.

Public class names and parameters, inference/declaration, composition and
equivalence, build/Semantic Graph/Replay metadata, Effect Log and event order,
privacy/redaction, integrity, corruption, divergence, checkpoints, resource
limits, migration, cross-process/backend replay, and Stable support remain
intentionally deferred pending Accepted RFC-0010 or replacement authority.
