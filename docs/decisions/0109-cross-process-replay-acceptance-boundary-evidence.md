# DEC-0109: Internal cross-process replay acceptance boundary evidence / 内部跨进程 Replay 验收边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: determinism-design
> 相关规范/缺口：`DEC-0108` | `DEC-0012` | `GAP-DETERMINISTIC-REPLAY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed cross-process
replay acceptance boundaries for the bounded `REP-2506-OBSERVATION` child. It
checks deterministic, duplicate-free boundary vocabulary. It does not spawn
processes, generate or play logs, compare runtime results, validate mutations,
or claim reproducibility.

本决定只授权 test-only 的拟议跨进程 Replay 验收边界清单，供
`REP-2506-OBSERVATION` 子任务使用。它只检查确定性、无重复的边界词汇；不启动进程、不生成或播放日志、不比较
runtime 结果、不验证 mutation，也不宣称可复现性。

## Question

The cross-process plan requires isolated processes, fixed toolchains and
caches, generator/player bindings, observable-result comparison, repeated
runs, mutation rejection, and provenance, but Accepted replay authority does
not define those contracts. What evidence can be retained without certifying a
cross-process replay?

跨进程计划要求进程隔离、固定 toolchain 与 cache、generator/player binding、可观察结果比较、重复运行、mutation
拒绝与 provenance，但已接受的 Replay 权威尚未定义这些契约。如何在不认证跨进程 replay 的情况下保留证据？

## Decision

1. `crates/ling-concurrency/tests/replay_cross_process_evidence.rs` keeps a
   test-local inventory of eighteen provisional boundaries: process
   isolation, toolchain/profile/target identity, cache isolation, input
   snapshots, log generation, replay playback, Program/Schema binding,
   mutation rejection, observable equivalence, repeatability, divergence,
   provenance, resource limits, platform boundaries, and offline mode.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.replay-cross-process-observation/0`. These bytes are not a process
   harness, replay result, acceptance artifact, or reproducibility claim.
3. The child adds no process runner, cache isolation mechanism, toolchain lock,
   generator/player, comparator, mutation validator, diagnostic, Semantic ID,
   public protocol, or CI acceptance rule. Public `REP-2506` remains
   `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted replay, privacy, and
  differential contracts before cross-process replay acceptance.
- `DEC-0012` governs Seed Semantic IDs and canonical bytes only, not process
  identity, observable equivalence, or replay acceptance.
- `DEC-0108` keeps privacy and integrity boundaries test-only while replay
  authority is absent.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open; this decision records
  cross-process vocabulary without resolving the gap.

## Conformance plan

- Assert all eighteen provisional boundaries and their test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep process/toolchain/profile/target identity, cache/input isolation,
  generator/player behavior, Program/Schema binding, mutation rejection,
  observable equivalence, repeat thresholds, divergence, provenance,
  resources, platform differences, offline enforcement, privacy/corruption,
  differential, and runtime fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public cross-process replay or
  reproducibility claim is registered.

## Unresolved alternatives

Process and cache isolation, toolchain identity, input provenance, generator /
player protocol, binding and mutation semantics, observable-equivalence
relation, repeatability threshold, divergence reporting, platform variance,
offline guarantees, diagnostics, resource limits, runtime ABI, and migration
remain open under `GAP-DETERMINISTIC-REPLAY-001` and `REP-2506`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
