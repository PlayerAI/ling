# DEC-0093: Internal Task lifecycle observation trace / 内部 Task 生命周期观测 trace

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: concurrency-design  
> 相关规范/缺口：`ROADMAP-1.0` | `GAP-STRUCTURED-TASK-001` | `DEC-0092`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable lifecycle
observation trace for `TASK-2203`. It provides deterministic event and Fault
identities for future evidence without selecting runtime behavior.

本决定只授权 `TASK-2203` 的 publish-disabled、不可执行生命周期观测 trace。它为未来
证据提供确定性的 event 与 Fault identity，但不选择运行时行为。

## Question

The lifecycle runtime target needs evidence for scope creation, child
registration, join, cancellation, Fault, cleanup, and closure, but the
ordering, propagation, timeout, and cleanup contracts remain open. What
checked data can be captured without making those choices executable?

生命周期 runtime 目标需要 scope 创建、子任务注册、join、取消、Fault、cleanup 和关闭
的证据，但顺序、传播、timeout 与 cleanup 合约仍未确定。哪些 checked data 可以在不使
这些选择可执行的情况下被记录？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `LifecycleTrace`, `LifecycleEvent`, `LifecycleEventId`, and `FaultId` values.
2. Event labels (`ScopeCreated`, `ChildRegistered`, `JoinObserved`,
   `CancellationObserved`, `FaultObserved`, `CleanupObserved`, and
   `ScopeClosed`) are observations only. The model does not validate or imply
   ordering, propagation, join obligations, timeout precedence, cleanup
   idempotence, Fault aggregation, or orphan policy.
3. Construction rejects zero/unresolved scope, event, task, related-task, or
   Fault identities and duplicate event identities. Events are stored in
   deterministic identity order.
4. Source spans are evidence only. Canonical bytes contain no source paths,
   spans, host addresses, allocation order, debug text, scheduler decisions,
   runtime outcomes, or public schema fields.
5. No runtime, scheduler, timeout API, cancellation token propagation,
   cleanup executor, Fault aggregator, parser, AST/HIR/typed-program node,
   bytecode/VM ABI, diagnostic, Semantic ID, CLI/LSP command, public protocol,
   or migration rule is added. Public `TASK-2203` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted lifecycle, cancellation,
  cleanup, scheduler, and differential contracts before Task execution.
- `docs/SEMANTICS.md` §18 keeps Task outside the v0.0.1 Seed subset.
- `DEC-0002` fixes original UTF-8 byte spans as evidence.
- `DEC-0091` and `DEC-0092` authorize only preceding internal Task identity
  and structural state-machine models.

## Conformance plan

- Build and sort a trace containing each structural event label and optional
  related task/Fault identities.
- Reject zero identities and duplicate event identities before publication.
- Compare equivalent traces with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep runtime ordering, join/cancel/timeout semantics, cleanup/Fault behavior,
  scheduler, differential, and migration fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0: unchanged.
- Adds only internal publish-disabled checked-data tests and no public protocol
  or v0.2 support claim.

## Unresolved alternatives

Scope/child ownership, join and result observation, cancellation propagation,
timeout races, Fault aggregation, cleanup ordering, orphan policy, scheduler
semantics, interpreter/VM lifecycle ABI, and migration remain open under
`GAP-STRUCTURED-TASK-001` and `TASK-2203`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
