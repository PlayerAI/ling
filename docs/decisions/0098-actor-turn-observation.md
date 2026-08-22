# DEC-0098: Internal Actor turn observation / 内部 Actor turn 观察

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: actor-semantics  
> 相关规范/缺口：`DEC-0097` | `DEC-0096` | `DEC-0095` | `GAP-ACTOR-AWAIT-REENTRY-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable turn
observation boundary for the bounded `ACT-2304-TURN-OBSERVATION` child. It
records opaque turn identities, optional opaque Actor owners, and labels that
mirror the design vocabulary. It does not choose await, reentry, self-send,
watchdog, state-version, scheduler, or runtime semantics.

本决定只授权 publish-disabled、不可执行的 turn 观察边界，供
`ACT-2304-TURN-OBSERVATION` 子任务使用。它记录不透明的 turn identity、可选的 Actor
owner，以及与设计词汇对应的标签；不选择 await、重入、self-send、watchdog、state version、
调度器或运行时语义。

## Question

The Actor plan needs a deterministic place to preserve future turn evidence,
but the accepted language authority does not define the await choice, state
guards, self-send route, or long-turn watchdog. What strictly structural data
can be recorded without creating a turn API or fixing reentry behavior?

Actor 计划需要确定性地保留未来 turn 证据，但已接受的语言权威尚未定义 await 选择、状态保护、
self-send 路径或长 turn watchdog。哪些纯结构数据可以在不创建 turn API、不固定重入行为的
前提下记录？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `TurnObservationModel`, `TurnId`, and `TurnObservation` values.
2. An observation contains a nonzero opaque turn identity, an optional
   nonzero opaque `ActorTypeId` owner, a structural label from `NoAwait`,
   `FreezeAndRelease`, `ForbidReentry`, `GuardedReentry`, `SelfSend`, or
   `Watchdog`, and an optional source span. These fields are evidence only;
   they do not describe executable turn, await, reentry, state, or watchdog
   behavior.
3. Construction rejects unresolved or duplicate turn identities and unresolved
   owner identities, then stores observations in turn-identity order. Canonical
   bytes are deterministic and omit source spans and paths.
4. The child adds no await form, turn lifecycle, state-version token, reentry
   guard, self-send operation, mailbox route, watchdog limit/event, scheduler
   hook, cancellation/Fault transition, runtime, serializer, diagnostic,
   Semantic ID, CLI/LSP command, public protocol, or migration rule. Public
   `ACT-2304` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor turn, await/reentry,
  mailbox, supervision, and differential contracts before execution.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` name future turn constraints but
  keep Actor/Task execution outside the v0.0.1 Seed subset.
- `DEC-0095`, `DEC-0096`, and `DEC-0097` authorize only preceding opaque Actor,
  message-schema, and mailbox-observation identity boundaries.
- `GAP-ACTOR-AWAIT-REENTRY-001` remains Open; this decision records evidence
  without resolving that gap.

## Conformance plan

- Build observations with optional owners and every structural label and assert
  deterministic turn-identity ordering.
- Reject zero identities and duplicate turn identities before publication.
- Compare equivalent models with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep await, reentry, state guards, self-send, watchdog, cancellation,
  supervision, scheduler, runtime, stress, differential, and migration fixtures
  deferred to the parent authority.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only internal publish-disabled checked-data tests. No public turn
  protocol, Actor behavior, or v0.2 support claim is registered.

## Unresolved alternatives

Turn lifecycle, await suspension, reentry policy, state-version/borrow guards,
self-send ordering, cancellation and Fault cleanup, watchdog observability,
scheduler/replay interaction, local/remote behavior, runtime ABI, and migration
remain open under the Actor gaps and `ACT-2304`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
