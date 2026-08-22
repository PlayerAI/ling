# DEC-0097: Internal Actor mailbox observation / 内部 Actor mailbox 观察

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: actor-semantics  
> 相关规范/缺口：`DEC-0096` | `DEC-0095` | `DEC-0010` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable mailbox
observation boundary for the bounded `ACT-2303-MAILBOX-OBSERVATION` child. It
records opaque mailbox identities, optional opaque Actor owners, and labels
that mirror the design vocabulary. It does not select mailbox, capacity,
queue, send, backpressure, ordering, supervision, or runtime semantics.

本决定只授权 publish-disabled、不可执行的邮箱观察边界，供
`ACT-2303-MAILBOX-OBSERVATION` 子任务使用。它记录不透明的 mailbox identity、可选的
Actor owner，以及与设计词汇对应的标签；不选择 mailbox、容量、队列、发送、背压、顺序、
监督或运行时语义。

## Question

The Actor plan needs a deterministic place to preserve future mailbox evidence,
but the accepted language authority does not define capacity units, queue
ownership, send outcomes, overflow behavior, ordering, or supervision. What
strictly structural data can be recorded without creating a mailbox API?

Actor 计划需要确定性地保留未来 mailbox 证据，但已接受的语言权威尚未定义容量单位、队列
所有权、发送结果、溢出行为、顺序或监督。哪些纯结构数据可以在不创建 mailbox API 的前提
下记录？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `MailboxObservationModel`, `MailboxId`, and `MailboxObservation` values.
2. An observation contains a nonzero opaque mailbox identity, an optional
   nonzero opaque `ActorTypeId` owner, a structural label from `Wait`,
   `Reject`, `DropNewest`, `DropOldest`, or `Coalesce`, and an optional source
   span. These fields are evidence only; they do not describe executable
   policy, capacity, queue contents, sender/receiver behavior, or a public
   type.
3. Construction rejects unresolved or duplicate mailbox identities and
   unresolved owner identities, then stores observations in mailbox-identity
   order. Canonical bytes are deterministic and omit source spans and paths.
4. The child adds no capacity value, queue storage, enqueue/dequeue operation,
   send result, suspension point, Backpressure Effect, overflow algorithm,
   coalescing key, ordering guarantee, fairness rule, close/termination rule,
   supervision transition, runtime, serializer, diagnostic, Semantic ID,
   CLI/LSP command, public protocol, or migration rule. Public `ACT-2303`
   remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor mailbox, ordering,
  supervision, and differential contracts before executable message delivery.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` name future bounded-mailbox
  constraints but keep Actor/Task execution outside the v0.0.1 Seed subset.
- `DEC-0010` authorizes Seed State and Capability behavior only; it does not
  define Actor mailbox ownership or send semantics.
- `DEC-0095` and `DEC-0096` authorize the preceding opaque Actor and message
  schema identity boundaries.
- `GAP-ACTOR-MAILBOX-SUPERVISOR-001` remains Open; this decision records
  evidence without resolving that gap.

## Conformance plan

- Build observations with optional owners and every structural label and assert
  deterministic mailbox-identity ordering.
- Reject zero identities and duplicate mailbox identities before publication.
- Compare equivalent models with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep capacity, queue, send, backpressure, ordering, fairness, coalescing,
  close/termination, supervision, runtime, stress, differential, and migration
  fixtures deferred to the parent authority.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only internal publish-disabled checked-data tests. No public mailbox
  protocol, Actor behavior, or v0.2 support claim is registered.

## Unresolved alternatives

Capacity units and bounds, queue ownership and quotas, typed send outcomes,
Wait suspension, overflow and coalescing semantics, ordering/fairness,
cancellation and shutdown, supervision/Fault, local/remote delivery, schema
identity, runtime ABI, and migration remain open under the Actor gaps and
`ACT-2303`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
