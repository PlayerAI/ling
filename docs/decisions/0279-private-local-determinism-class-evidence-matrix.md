# DEC-0279: Private local determinism-class evidence matrix / 私有本地确定性等级证据矩阵

> 状态：Proposed<br>
> 提出日期：2026-09-01<br>
> 决定日期：Pending<br>
> Owner role：determinism-design<br>
> 相关 RFC/缺口：DEC-0104 | DEC-0267 | DEC-0268 | DEC-0274 | DEC-0278 | GAP-DETERMINISTIC-REPLAY-001 | REP-2501<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision proposes the smallest executable evidence package that can close
an internal Experimental REP-2501 baseline over existing Accepted local
execution contracts. It does not classify Ling programs publicly, add build or
Semantic Graph metadata, define a Replay header, or resolve the broader Replay
gap.

本决定提议基于既有 Accepted 本地执行契约，建立可完成内部 Experimental
REP-2501 基线的最小可执行证据包。它不公开分类 Ling 程序，不增加构建或 Semantic
Graph metadata，不定义 Replay header，也不解决更广泛的 Replay 缺口。

## Question

What exact private matrix may exercise the five determinism categories named by
`SEMANTICS.md` section 22.1 against existing checked execution routes, while
keeping the execution plan's four provisional labels, public metadata, Effect
Log, Replay schema, and compatibility claims outside the implemented surface?

## Decision

1. **Scoped authority.** This decision authorizes only one crate-private,
   `cfg(test)` REP-2501 executable evidence matrix in `ling-eval`. It may test
   behavior already fixed by Accepted checked execution, DEC-0267, DEC-0268,
   DEC-0274, and DEC-0278. It adds no production classifier, runtime
   transition, public API, or protocol and does not close
   `GAP-DETERMINISTIC-REPLAY-001`.

2. **Authoritative vocabulary.** The matrix uses exactly the five category
   shapes named by higher-authority `SEMANTICS.md` section 22.1:
   `PureDeterministic`, `SeedDeterministic<RandomSource>`,
   `InputDeterministic<EffectLog>`, `ScheduleDeterministic`, and
   `Nondeterministic(reason)`. They are test-local evidence labels, not source
   syntax, production Rust types, serialized tags, or Stable names.

3. **Provisional-plan separation.** DEC-0104's `Strict`, `Seeded`,
   `RecordedEffects`, and `BestEffort` labels remain a separate vocabulary-only
   record of the lower-authority execution plan. The matrix must not silently
   alias, order, or serialize them as the five section 22.1 categories. A
   future RFC may reconcile or replace either naming set.

4. **Evidence case set.** The matrix contains exactly these bounded case
   families:
   `pure-deterministic-checked-execution`,
   `seed-deterministic-task-schedule`,
   `input-deterministic-task-replay`,
   `schedule-deterministic-actor-script`, and
   `nondeterministic-production-task-boundary`. New category meanings or case
   families require separate Accepted authority.

5. **Execution-contract scope.** A case classifies only one fully stated test
   execution contract: successful immutable `CheckedProgram`, selected entry,
   validated arguments, exact limits, injected host source, scheduling inputs,
   and allowed comparison projection. It does not infer a class for a function,
   component, package, build artifact, arbitrary checked program, or source
   declaration.

6. **Pure-deterministic evidence.** The pure case uses a checked ordinary Seed
   entry whose transitive residual Effect row and required host Capability set
   are empty. Repeated execution and equivalent checked reconstruction must
   produce the same canonical Value-or-Fault projection with no host event.
   Paths, source IDs, BOM/LF/CRLF spelling, and definition insertion order may
   vary; authoritative original UTF-8 spans remain corresponding sidecar
   evidence rather than equality inputs.

7. **Seed-deterministic evidence.** The seeded case directly drives the
   DEC-0267 Task test scheduler with one explicit `u64` seed, exact deadlines,
   limits, arguments, and deterministic host script. Repeated runs and
   equivalent checked reconstruction must produce identical validated
   `TaskScheduleTrace::canonical_bytes()`. The fixed DEC-0267 SplitMix64
   mapping is test evidence only and is not a general Ling `RandomSource` ABI.

8. **Input-deterministic evidence.** The input case replays one complete,
   validated DEC-0267 Task trace against a fresh equivalent checked runtime and
   the exact recorded deterministic host script. Replay must consume recorded
   choices and host outcomes, reject the first identity/event mutation, never
   fall back to the seed, and reproduce identical canonical trace bytes. The
   typed trace is not the future public `EffectLog` or Replay schema.

9. **Schedule-deterministic evidence.** The schedule case drives the DEC-0274
   Actor runtime and DEC-0276/DEC-0277 Supervisor only through one finite
   explicit script of typed send, `step(ActorId)`, `advance_to(u64)`, stop, or
   owner cancellation operations. Equivalent checked reconstruction and the
   same explicit script must produce the same bounded lifecycle, Fault,
   mailbox, restart, cleanup, and logical-tick projection. This does not promise
   that an unrecorded production scheduler chooses that script.

10. **Nondeterministic evidence.** The production Task case is labeled only as
    test-local `Nondeterministic("unrecorded-local-task-scheduler")` because
    DEC-0268 permits unrecorded worker acquisition and host Effect/Fault order
    and grants no Stable ordering promise. Tests across fixed worker counts may
    compare only Accepted invariants: canonical Task identities, structurally
    valid terminal state/Fault set, exactly-once cleanup, and bounded shutdown.
    Worker metrics or allowed Effect order must not be promoted into equality.

11. **No lattice or composition claim.** The five test labels have no ordering,
    subtyping, join, meet, call composition, Actor/Task propagation, declaration
    rule, or invalid-claim diagnostic in this slice. A case proves only its own
    stated execution contract; it cannot upgrade another route or a complete
    program by analogy.

12. **Bounded projection.** Retained evidence may contain only canonical
    checked program/body identities, canonical Task/Actor identities, values or
    Fault phase/category, logical ticks, explicit ready/selection sets,
    lifecycle/terminal state, bounded host events, queue/discard counts,
    cleanup counts, restart/circuit state, and original source spans as
    sidecars. Every collection and command script is explicitly finite.

13. **Forbidden observations.** Filesystem paths, physical source names or
    IDs, wall time, duration, thread or worker identity, addresses, allocation
    layout, hash-map order, panic text, Rust debug output, host locale, and
    unspecified scheduler metrics cannot select a category or enter an exact
    equality projection.

14. **Negative surface evidence.** The matrix must prove that no production
    determinism enum/classifier, source annotation, build/package metadata
    field, Semantic Graph/Audit field, Replay header, Effect Log schema,
    decoder/writer, CLI command, diagnostic, or protocol-inventory entry is
    created. Tests must not add placeholders merely to make absence observable.

15. **Public boundary.** No Ling syntax, value, type, Effect, Capability,
    function/component claim, CLI/REPL/LSP/editor route, public Rust API,
    diagnostic, schema, Semantic ID, protocol, package/ABI, bytecode, VM,
    Native, Wasm, remote Actor, migration, or Stable behavior is added.

16. **No Replay or cross-backend claim.** In-process equality under bounded
    explicit inputs is not a public Replay log, cross-process guarantee,
    interpreter/VM/backend equivalence class, platform guarantee, privacy
    policy, corruption strategy, or performance result. REP-2502 through
    REP-2506 retain those concerns.

17. **Completion boundary.** REP-2501 is Done only for this internal
    Experimental baseline when all five exact cases execute against the real
    accepted routes, explicit negative boundaries pass, task-specific and full
    repository gates pass, evidence is bound to a commit, and status/backlog/
    gap records are synchronized. Existing tests may be reused only when the
    matrix directly executes their complete assertions; names alone are not
    evidence.

18. **Deferred public classification.** Public class names/parameters,
    inference, declarations, composition, equivalence promises, build metadata,
    Semantic Graph and Replay-header fields, Semantic ID interaction, target/
    profile/toolchain identity, diagnostics, privacy/redaction, integrity,
    corruption, divergence, migration, cross-process replay, and Stable support
    remain blocked pending Accepted RFC-0010 or a replacement.

## Conformance plan

- Add one dedicated private `ling-eval` determinism evidence module with an
  exact five-case table and no production classifier.
- Drive a pure checked Seed entry twice and through Unicode/BOM/CRLF/source-
  identity/insertion-order reconstruction; compare the bounded Value/Fault and
  empty-host projection.
- Reuse the real DEC-0267 seeded run and strict replay paths, including exact
  seed vectors, canonical trace bytes, recorded host success/failure, identity
  rejection, first-event divergence, and no seed fallback.
- Reuse the real DEC-0274/DEC-0276/DEC-0277 explicit Actor/Supervisor script
  evidence for schedule-deterministic reconstruction.
- Drive DEC-0268 with fixed worker counts and assert only its stable lifecycle,
  Fault, cleanup, and shutdown invariants while excluding worker metrics and
  allowed host order from equality.
- Add bounded source/module inventory assertions for the absent public and
  production classification/Replay surfaces.
- Run focused `ling-eval` tests and strict Clippy, retained CLI Task/Actor
  boundaries, the full locked/offline workspace suite, governance/status/docs/
  RC0 gates, formatting, and diff checks before marking REP-2501 Done.

## Compatibility impact

- Source, CLI/LSP/editor, diagnostics, schemas, Semantic IDs, protocols,
  package/ABI versions, stored data, bytecode/VM/backends, dependencies, and
  migration: none; this proposal authorizes private `cfg(test)` evidence only.
- Runtime: no production transition, classifier, field, or public API is
  added. Tests execute only existing Accepted routes.
- Determinism: the matrix records bounded evidence claims without declaring a
  public class or Replay equivalence. Unicode remains 17.0.0 and original
  UTF-8 byte spans remain authoritative.

## Unresolved alternatives

- Public class syntax or metadata, a production classifier, inferred class
  lattice, Semantic Graph/Audit projection, build metadata, and Replay header
  require a coherent Accepted public contract rather than this test matrix.
- Canonical Effect Log versus higher-level event protocol, recordable Effect
  inventory, scheduler/Actor message logging, checkpoints, privacy/redaction,
  integrity, corruption, divergence, resource limits, migration, and
  cross-process/backend replay remain RFC-0010 and REP-2502 through REP-2506.
- Treating DEC-0104's four plan labels as aliases for the five SEMANTICS labels
  is rejected because no Accepted one-to-one mapping or equivalence relation
  exists.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
