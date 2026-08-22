# DEC-0107: Internal replay-player boundary evidence / 内部 Replay Player 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: determinism-design
> 相关规范/缺口：`DEC-0106` | `DEC-0012` | `GAP-DETERMINISTIC-REPLAY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed replay-player
preflight and comparison boundaries for the bounded `REP-2504-OBSERVATION`
child. It checks deterministic, duplicate-free boundary vocabulary. It does
not read logs, validate checkpoints, apply events, restore state, or define
divergence behavior.

本决定只授权 test-only 的拟议 Replay Player preflight 与 comparison 边界清单，供
`REP-2504-OBSERVATION` 子任务使用。它检查确定性、无重复的边界词汇；不读取日志、不验证 checkpoint、不
应用 event、不恢复 state，也不定义 divergence 行为。

## Question

The replay-player plan requires checkpoint/program binding, preflight checks,
event application, ordering, divergence, Fault, cancellation, privacy,
integrity, and migration, but Accepted RFC-C205 authority does not define
their contracts. What evidence can be retained without implementing a player?

Replay Player 计划要求 checkpoint/program binding、preflight checks、event application、ordering、
divergence、Fault、cancellation、privacy、integrity 和 migration，但已接受的 RFC-C205 权威尚未定义其
契约。如何在不实现 player 的情况下保留证据？

## Decision

1. `crates/ling-concurrency/tests/replay_player_evidence.rs` keeps a
   test-local inventory of eleven provisional boundaries: checkpoint identity,
   program canonical bytes, preflight binding, event application, ordering,
   divergence, Fault, cancellation, privacy, integrity, and migration.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.replay-player-observation/0`. These bytes are not a checkpoint, log, or
   player protocol.
3. The child adds no replay player, preflight validator, checkpoint format,
   event-log reader, divergence engine, CLI command, diagnostic, Semantic ID,
   public protocol, or migration rule. Public `REP-2504` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Determinism Class, Effect Log,
  Replay version, privacy, and differential contracts before replay playback.
- `DEC-0012` governs Seed Semantic IDs/canonical bytes only, not replay
  checkpoint or player authority.
- `DEC-0106` keeps effect recording boundaries test-only.
- `GAP-DETERMINISTIC-REPLAY-001` remains Open; this decision records player
  vocabulary without resolving the gap.

## Conformance plan

- Assert all eleven provisional boundaries and their test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep checkpoint/program binding, preflight mismatch diagnostics, event
  application, ordering, Fault/cancellation, divergence, privacy, integrity,
  migration, CLI/protocol, cross-process, differential, and runtime fixtures
  deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public replay-player or protocol
  claim is registered.

## Unresolved alternatives

Checkpoint identity and bytes, preflight binding, event application and order,
Fault/cancellation, divergence equivalence, privacy/integrity, migration,
diagnostics, CLI authorization, resource limits, runtime ABI, and cross-process
behavior remain open under `GAP-DETERMINISTIC-REPLAY-001` and `REP-2504`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
