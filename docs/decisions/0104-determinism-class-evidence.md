# DEC-0104: Internal determinism-class evidence / 内部确定性等级证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: determinism-design
> 相关规范/缺口：`DEC-0021` | `GAP-DETERMINISTIC-REPLAY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only vocabulary and evidence boundary for
the bounded `REP-2501-OBSERVATION` child. It records the four provisional
class labels in the execution plan and compares them with existing checked
effect canonicalization. It does not classify programs or define replay,
scheduling, metadata, or runtime semantics.

本决定只授权 test-only 的词汇与证据边界，供 `REP-2501-OBSERVATION` 子任务使用。它记录执行计划中的
四个临时等级标签，并使用现有 checked effect canonicalization 做比较；不对程序分类，也不定义 replay、
scheduler、metadata 或 runtime 语义。

## Question

The execution plan proposes Strict, Seeded, RecordedEffects, and BestEffort,
but Accepted authority does not define their names, ordering, inference,
observable equivalence, metadata placement, or replay headers. What evidence
can be added without freezing those choices?

执行计划提出 Strict、Seeded、RecordedEffects 和 BestEffort，但已接受的权威尚未定义名称、顺序、推导、
可观察等价、metadata 位置或 replay header。如何在不冻结这些选择的情况下增加证据？

## Decision

1. `crates/ling-effects/tests/determinism_evidence.rs` keeps the four labels in
   a test-local enum: `Strict`, `Seeded`, `RecordedEffects`, and `BestEffort`.
2. Test-only evidence combines one provisional label with the existing
   `EffectRowModel::canonical_bytes()` output. Equivalent effect-label input
   order must produce identical evidence bytes; this verifies only the
   existing checked effect projection.
3. The child adds no determinism enum to production crates, class inference,
   build-metadata field, Semantic Graph field, replay header, scheduler
   contract, diagnostic, Semantic ID, CLI/LSP command, public protocol, or
   migration rule. Public `REP-2501` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Determinism Class, Effect Log,
  Replay version, and privacy boundaries before replay support.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` do not define runtime replay or
  determinism classification in the v0.0.1 Seed subset.
- `DEC-0021` authorizes deterministic compiler-query scheduling only; it is not
  runtime replay authority.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open; this decision records vocabulary
  without resolving it.

## Conformance plan

- Assert the four provisional labels and their test-local order.
- Compare equivalent effect rows with different input order and require
  identical test-only evidence bytes.
- Keep class inference, effect/runtime boundaries, scheduler interleavings,
  build metadata, Semantic Graph fields, replay headers, privacy, corruption,
  divergence, cross-process, differential, and migration fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only evidence. No public determinism or replay claim is
  registered.

## Unresolved alternatives

Class names and ordering, declaration/inference, composition, observable
equivalence, effect/scheduler boundaries, metadata and graph/header placement,
versioning, privacy, corruption, divergence, diagnostics, runtime ABI, and
migration remain open under `GAP-DETERMINISTIC-REPLAY-001` and `REP-2501`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
