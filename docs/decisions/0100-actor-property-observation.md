# DEC-0100: Internal Actor property observation / 内部 Actor 性质观察

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: actor-verification  
> 相关规范/缺口：`DEC-0099` | `DEC-0098` | `DEC-0097` | `GAP-ACTOR-AWAIT-REENTRY-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-DETERMINISTIC-REPLAY-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable property
observation boundary for the bounded `ACT-2306-PROPERTY-OBSERVATION` child. It
records opaque observation identities, optional opaque Actor instances, and
labels for the planned property vocabulary. It does not assert serializability,
parallelism, boundedness, ordering, cleanup, or stress-test outcomes.

本决定只授权 publish-disabled、不可执行的性质观察边界，供
`ACT-2306-PROPERTY-OBSERVATION` 子任务使用。它记录不透明的 observation identity、可选的
Actor instance，以及计划性质词汇的标签；不声称串行性、并行性、有界性、顺序、清理或压力
测试结果。

## Question

The Actor plan needs a deterministic place to preserve future property-test
metadata, but accepted authority does not define the semantic relation,
interleaving equivalence, stress thresholds, resource budgets, or replay
schema. What structural evidence can be recorded without certifying a runtime?

Actor 计划需要确定性地保留未来性质测试元数据，但已接受的权威尚未定义语义关系、interleaving
等价关系、压力阈值、资源预算或 replay schema。哪些结构证据可以在不认证运行时的前提下记录？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `PropertyObservationModel`, `PropertyObservationId`, and
   `PropertyObservation` values.
2. An observation contains a nonzero opaque observation identity, an optional
   nonzero opaque `ActorId`, a structural label from `SerialState`,
   `ParallelActors`, `BoundedMailbox`, `SlowConsumer`, `PostStopSend`,
   `FaultCleanup`, `DeclaredOrdering`, or `ShutdownCleanup`, and an optional
   source span. These fields are evidence vocabulary only; they do not encode
   a property result, scheduler, resource limit, or runtime behavior.
3. Construction rejects unresolved or duplicate observation identities and
   unresolved Actor identities, then stores observations in identity order.
   Canonical bytes are deterministic and omit source spans and paths.
4. The child adds no property runner, stress harness, scheduler, replay format,
   fixture schema, threshold, diagnostic, Semantic ID, CLI/LSP command, public
   protocol, or migration rule. Public `ACT-2306` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor runtime, determinism,
  replay, and differential contracts before property evidence can certify
  execution.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` name future Actor properties but
  keep Actor/Task execution outside the v0.0.1 Seed subset.
- `DEC-0095` through `DEC-0099` authorize only preceding opaque Actor,
  message, mailbox, turn, and runtime observation boundaries.
- `GAP-ACTOR-AWAIT-REENTRY-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, and
  `GAP-DETERMINISTIC-REPLAY-001` remain Open; this decision records evidence
  without resolving those gaps.

## Conformance plan

- Build observations with optional actors and every structural label and assert
  deterministic observation-identity ordering.
- Reject zero identities and duplicate observation identities before publication.
- Compare equivalent models with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep property execution, stress thresholds, scheduler/interleaving,
  resource accounting, replay, shutdown, cross-backend, and migration fixtures
  deferred to the parent authority.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only internal publish-disabled checked-data tests. No public property
  protocol, Actor behavior, or v0.2 support claim is registered.

## Unresolved alternatives

Property relation and assertion polarity, serial/parallel guarantees,
interleaving classes, mailbox/backpressure outcomes, Fault/cleanup invariants,
stress budgets and thresholds, replay privacy/versioning, platform scope,
runtime differential comparison, and migration remain open under the Actor
gaps and `ACT-2306`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
