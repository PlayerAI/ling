# DEC-0099: Internal Actor runtime observation / 内部 Actor runtime 观察

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: actor-runtime  
> 相关规范/缺口：`DEC-0098` | `DEC-0097` | `DEC-0096` | `DEC-0095` | `GAP-ACTOR-AWAIT-REENTRY-001` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001` | `GAP-STRUCTURED-TASK-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable runtime
observation boundary for the bounded `ACT-2305-RUNTIME-OBSERVATION` child. It
records opaque observation identities, optional opaque Actor instances, and
labels that mirror the future runtime vocabulary. It does not choose spawn,
stop, dispatch, lifecycle, Fault, registry, scheduler, ABI, or runtime
semantics.

本决定只授权 publish-disabled、不可执行的 runtime 观察边界，供
`ACT-2305-RUNTIME-OBSERVATION` 子任务使用。它记录不透明的 observation identity、可选的
Actor instance，以及与未来 runtime 词汇对应的标签；不选择 spawn、stop、dispatch、生命周期、
Fault、registry、调度器、ABI 或运行时语义。

## Question

The Actor plan needs a deterministic place to preserve future runtime evidence,
but accepted authority does not define runtime ownership, lifecycle
transitions, dispatch ABI, registry lifetime, or Fault provenance. What
strictly structural data can be recorded without creating a runtime API?

Actor 计划需要确定性地保留未来 runtime 证据，但已接受的权威尚未定义 runtime 所有权、生命周期
转换、dispatch ABI、registry 生命周期或 Fault provenance。哪些纯结构数据可以在不创建 runtime
API 的前提下记录？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `RuntimeObservationModel`, `RuntimeObservationId`, and `RuntimeObservation`
   values.
2. An observation contains a nonzero opaque observation identity, an optional
   nonzero opaque `ActorId`, a structural label from `Spawn`, `Start`,
   `Dispatch`, `Suspend`, `Stop`, `Stopped`, `Failed`, or `Restart`, and an
   optional source span. These fields are evidence only; they do not describe
   executable lifecycle, ownership, scheduling, delivery, or Fault behavior.
3. Construction rejects unresolved or duplicate observation identities and
   unresolved Actor identities, then stores observations in identity order.
   Canonical bytes are deterministic and omit source spans and paths.
4. The child adds no runtime crate, spawn/stop/dispatch operation, typed
   envelope, mailbox storage, lifecycle state machine, registry, scheduler
   hook, Task integration, Fault schema, serializer, diagnostic, Semantic ID,
   CLI/LSP command, public protocol, or migration rule. Public `ACT-2305`
   remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Task, Actor, mailbox,
  supervision, runtime, and differential contracts before execution.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` describe future Actor runtime
  constraints but keep Actor/Task execution outside the v0.0.1 Seed subset.
- `DEC-0095` through `DEC-0098` authorize only preceding opaque Actor,
  message, mailbox, and turn observation boundaries.
- `GAP-ACTOR-AWAIT-REENTRY-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, and
  `GAP-STRUCTURED-TASK-001` remain Open; this decision records evidence without
  resolving those gaps.

## Conformance plan

- Build observations with optional actors and every structural label and assert
  deterministic observation-identity ordering.
- Reject zero identities and duplicate observation identities before publication.
- Compare equivalent models with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep spawn/stop, dispatch, lifecycle, Fault, registry, scheduler, Task,
  serialization, runtime, stress, differential, and migration fixtures
  deferred to the parent authority.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only internal publish-disabled checked-data tests. No public runtime
  protocol, Actor behavior, or v0.2 support claim is registered.

## Unresolved alternatives

Runtime identity allocation/reuse, registry scope, lifecycle transitions,
spawn/stop/send/receive ABI, Task and scheduler integration, Fault provenance,
shutdown, local/remote behavior, security, limits, runtime differential tests,
and migration remain open under the Actor/Task gaps and `ACT-2305`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
