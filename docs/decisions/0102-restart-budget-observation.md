# DEC-0102: Internal restart-budget observation / 内部重启预算观察

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: actor-supervision
> 相关规范/缺口：`DEC-0101` | `DEC-0100` | `DEC-0099` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-DETERMINISTIC-REPLAY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable restart-budget
observation boundary for the bounded `SUP-2402-OBSERVATION` child. It records
opaque observation identities, optional opaque Actor instances, and labels for
the budget/circuit vocabulary. It does not choose clocks, windows, counters,
backoff, circuit transitions, Fault provenance, or runtime query semantics.

本决定只授权 publish-disabled、不可执行的重启预算观察边界，供
`SUP-2402-OBSERVATION` 子任务使用。它记录不透明的 observation identity、可选的 Actor
instance，以及预算/熔断词汇的标签；不选择时钟、窗口、计数器、backoff、熔断转换、Fault
provenance 或 runtime query 语义。

## Question

The restart plan needs a deterministic place to preserve future budget and
circuit evidence, but accepted authority does not define time, budget scope,
backoff, persistence, circuit transitions, or replay. What structural evidence
can be recorded without creating a restart controller or query protocol?

重启计划需要确定性地保留未来预算和熔断证据，但已接受的权威尚未定义时间、预算范围、backoff、
持久化、熔断转换或 replay。哪些结构证据可以在不创建重启控制器或 query protocol 的前提下记录？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `BudgetObservationModel`, `BudgetObservationId`, and `BudgetObservation`
   values.
2. An observation contains a nonzero opaque observation identity, an optional
   nonzero opaque `ActorId`, a structural label from `RestartCount`, `Window`,
   `Backoff`, `MaxRestarts`, `FaultProvenance`, `CircuitClosed`,
   `CircuitOpen`, or `CircuitHalfOpen`, and an optional source span. These
   fields are evidence vocabulary only; they do not encode values, clocks,
   transitions, or recovery behavior.
3. Construction rejects unresolved or duplicate observation identities and
   unresolved Actor identities, then stores observations in identity order.
   Canonical bytes are deterministic and omit source spans and paths.
4. The child adds no counter, time/logical clock, backoff scheduler, circuit
   state machine, Fault store, runtime query, administration protocol,
   diagnostic, Semantic ID, CLI/LSP command, public protocol, or migration
   rule. References to stale `zero` commands remain excluded. Public `SUP-2402`
   remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor supervision,
  determinism/replay, runtime, and differential contracts before restart
  control can be exposed.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` prohibit unlimited rapid restart
  but do not define budget units, clocks, transitions, or a query protocol.
- `DEC-0095` through `DEC-0101` authorize only preceding opaque Actor,
  message, mailbox, turn, runtime, property, and Supervisor observation
  boundaries.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` and `GAP-DETERMINISTIC-REPLAY-001`
  remain Open; this decision records evidence without resolving them.

## Conformance plan

- Build observations with optional actors and every structural label and assert
  deterministic observation-identity ordering.
- Reject zero identities and duplicate observation identities before publication.
- Compare equivalent models with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep clocks, counters, windows, backoff, circuit transitions, Fault
  provenance, query/admin protocols, scheduler, runtime, stress, replay,
  differential, and migration fixtures deferred to the parent authority.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only internal publish-disabled checked-data tests. No public restart,
  circuit, query, or Actor behavior claim is registered.

## Unresolved alternatives

Budget scope and units, time/logical windows, persistence, backoff and jitter,
restart transitions, circuit states, concurrent Fault handling, mailbox/Task
interaction, provenance/privacy, query authorization, replay, runtime ABI, and
migration remain open under the supervision/replay gaps and `SUP-2402`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
