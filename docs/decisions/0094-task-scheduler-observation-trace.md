# DEC-0094: Internal Task scheduler observation trace / 内部 Task 调度器观测 trace

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: concurrency-design  
> 相关规范/缺口：`DEC-0021` | `DEC-0093` | `ROADMAP-1.0` | `GAP-STRUCTURED-TASK-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable scheduler
observation trace for `TASK-2204`. It provides deterministic evidence
identities without selecting Task scheduling, clock, wake, seed, replay, or
production-runtime behavior.

本决定只授权 `TASK-2204` 的 publish-disabled、不可执行调度器观测 trace。它为证据
提供确定性的 identity，但不选择 Task 调度、时钟、唤醒、seed、replay 或生产 runtime
行为。

## Question

The deterministic scheduler target needs evidence boundaries, but the Task
lifecycle, suspension, seed interpretation, virtual-clock units, wake ordering,
interleaving, and replay contracts remain open. What checked data can be
recorded without making a scheduler executable?

确定性 scheduler 目标需要证据边界，但 Task 生命周期、suspension、seed 解释、虚拟
时钟单位、唤醒顺序、交错和 replay 合约仍未确定。哪些 checked data 可以在不使 scheduler
可执行的情况下被记录？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `SchedulerObservationTrace`, `SchedulerObservation`, `SchedulerEventId`,
   and `SchedulerTraceId` values.
2. Event labels (`SeedObserved`, `ReadyObserved`, `WakeObserved`,
   `ClockObserved`, `InterleavingObserved`, and `TraceClosed`) are observations
   only. They do not define queue order, clock units, seed mapping, fairness,
   wake injection, interleaving exploration, replay equivalence, or effects.
3. Construction rejects zero/unresolved trace, event, scope, or task identities
   and duplicate event identities. Observations are stored in deterministic
   event-identity order.
4. Source spans are evidence only. Canonical bytes contain no source paths,
   spans, host timing, allocation order, scheduler decisions, runtime outcomes,
   or public schema fields.
5. No queue, worker, virtual clock, seed algorithm, wake API, exploration
   engine, replay protocol, parser, AST/HIR/typed-program node, bytecode/VM ABI,
   diagnostic, Semantic ID, CLI/LSP command, public protocol, or migration rule
   is added. Public `TASK-2204` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires deterministic-scheduler and differential
  evidence before Task execution is promoted.
- `docs/SEMANTICS.md` §18 keeps Task outside the v0.0.1 Seed subset.
- `DEC-0021` authorizes deterministic internal compiler-query scheduling only;
  it does not authorize a Task scheduler.
- `DEC-0093` authorizes only the preceding lifecycle observation boundary.

## Conformance plan

- Build and sort a trace containing each structural scheduler-observation label
  and optional scope/task identities.
- Reject zero identities and duplicate event identities before publication.
- Compare equivalent traces with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep queue execution, virtual time, seed mapping, wake ordering, bounded
  exploration, replay, cancellation/cleanup races, and migration fixtures
  deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0: unchanged.
- Adds only internal publish-disabled checked-data tests and no public protocol
  or v0.2 support claim.

## Unresolved alternatives

Seed-to-order mapping, ready/wake tie-breaks, virtual-clock representation,
fairness and starvation, exploration bounds, trace privacy/corruption,
replay/equivalence, interpreter/VM scheduler ABI, and migration remain open
under `GAP-STRUCTURED-TASK-001` and `TASK-2204`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
