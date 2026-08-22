# DEC-0101: Internal Supervisor observation / 内部 Supervisor 观察

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: actor-supervision  
> 相关规范/缺口：`DEC-0100` | `DEC-0099` | `DEC-0098` | `DEC-0097` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable Supervisor
observation boundary for the bounded `SUP-2401-OBSERVATION` child. It records
opaque observation identities, optional opaque Actor instances, and labels for
the supervision vocabulary. It does not choose restart, stop, escalation,
child lifetime, strategy, state restore, Fault channel, or runtime semantics.

本决定只授权 publish-disabled、不可执行的 Supervisor 观察边界，供
`SUP-2401-OBSERVATION` 子任务使用。它记录不透明的 observation identity、可选的 Actor
instance，以及监督词汇的标签；不选择 restart、stop、escalate、child lifetime、strategy、
state restore、Fault channel 或运行时语义。

## Question

The supervision plan needs a deterministic place to preserve future tree and
recovery evidence, but accepted authority does not define child ownership,
strategy transitions, restart budgets, state restore, or Fault channels. What
structural evidence can be recorded without creating a Supervisor API?

监督计划需要确定性地保留未来树和恢复证据，但已接受的权威尚未定义 child 所有权、策略转换、重启
预算、状态恢复或 Fault channel。哪些结构证据可以在不创建 Supervisor API 的前提下记录？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `SupervisorObservationModel`, `SupervisorObservationId`, and
   `SupervisorObservation` values.
2. An observation contains a nonzero opaque observation identity, an optional
   nonzero opaque `ActorId`, a structural label from `ChildSpec`, `Restart`,
   `Stop`, `Escalate`, `OneForOne`, `RestForOne`, `Transient`, `Permanent`,
   `Temporary`, `StateRestore`, or `FaultChannel`, and an optional source span.
   These fields are evidence vocabulary only; they do not encode recovery
   behavior, budgets, state, or Fault results.
3. Construction rejects unresolved or duplicate observation identities and
   unresolved Actor identities, then stores observations in identity order.
   Canonical bytes are deterministic and omit source spans and paths.
4. The child adds no Supervisor type, child registry, restart/stop/escalate
   operation, strategy state machine, restart budget, state snapshot/restore,
   Fault channel, diagnostic, Semantic ID, CLI/LSP command, public protocol, or
   migration rule. Public `SUP-2401` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor mailbox, runtime,
  supervision, determinism, and differential contracts before recovery
  behavior can be exposed.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` name future supervision concepts
  but keep Actor/Task execution outside the v0.0.1 Seed subset.
- `DEC-0095` through `DEC-0100` authorize only preceding opaque Actor,
  message, mailbox, turn, runtime, and property observation boundaries.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open; this decision records
  evidence without resolving that gap.

## Conformance plan

- Build observations with optional actors and every structural label and assert
  deterministic observation-identity ordering.
- Reject zero identities and duplicate observation identities before publication.
- Compare equivalent models with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep child strategy, restart budget, state restore, Fault, shutdown,
  scheduler, runtime, stress, differential, and migration fixtures deferred to
  the parent authority.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only internal publish-disabled checked-data tests. No public Supervisor
  protocol, Actor behavior, or v0.2 support claim is registered.

## Unresolved alternatives

Child ownership and state, restart/stop/escalate semantics, one-for-one versus
rest-for-one, lifetime classes, budgets/windows/backoff, state restore and
invariant checks, Fault aggregation/channel, mailbox cleanup, local/remote
behavior, runtime ABI, and migration remain open under the supervision gap and
`SUP-2401`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
