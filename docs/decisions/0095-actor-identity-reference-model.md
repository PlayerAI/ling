# DEC-0095: Internal Actor identity/reference model / 内部 Actor 身份/引用模型

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: actor-semantics  
> 相关规范/缺口：`DEC-0002` | `DEC-0090` | `ROADMAP-1.0` | `GAP-ACTOR-AWAIT-REENTRY-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable Actor
identity/reference model for the bounded `ACT-2301` child. It gives later
work a checked identity boundary without defining Actor syntax, turns, state
isolation, message ownership, serialization, or runtime behavior.

本决定只授权 `ACT-2301` 的 publish-disabled、不可执行 Actor 身份/引用模型。它为后续
工作提供 checked identity 边界，但不定义 Actor 语法、turn、状态隔离、消息所有权、序列化
或 runtime 行为。

## Question

The Actor target needs stable type, instance, and local/remote reference
evidence, but the identity lifetime, turn, state, message, and transport
contracts remain open. What checked data can be recorded without making Actor
semantics executable?

Actor 目标需要稳定的类型、实例和 local/remote 引用证据，但 identity 生命周期、turn、
state、message 与 transport 合约仍未确定。哪些 checked data 可以在不使 Actor 语义可
执行的情况下被记录？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `ActorIdentityModel`, `ActorTypeId`, `ActorId`, `ActorRefId`, and opaque
   Actor type/instance/reference values.
2. `Local` and `Remote` reference labels are structural observations only. They
   do not define serialization, delivery, comparison, capability, lifetime,
   mailbox, or network behavior.
3. Construction rejects zero/unresolved type, actor, and reference identities,
   duplicate identities, actor instances with unknown types, and references to
   unknown actors. Values are stored in deterministic identity order.
4. Source spans are evidence only. Canonical bytes contain no source paths,
   spans, host addresses, allocation order, turn state, message payloads,
   serialization fields, or public schema fields.
5. No Actor syntax, AST/HIR/typed-program node, turn checker, state-isolation
   rule, borrow/aliasing rule, Sendable judgment, mailbox, scheduler, runtime,
   bytecode/VM ABI, diagnostic, Semantic ID, CLI/LSP command, public protocol,
   or migration rule is added. Public `ACT-2301` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor identity, state,
  sendability, mailbox, supervision, and differential contracts before Actor
  execution.
- `docs/SEMANTICS.md` keeps Actor outside the v0.0.1 Seed subset.
- `DEC-0002` fixes original UTF-8 byte spans as evidence.
- `DEC-0090` authorizes only negative Actor-shaped syntax evidence.
- `GAP-ACTOR-AWAIT-REENTRY-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, and
  `GAP-ACTOR-REMOTE-DELIVERY-001` remain open.

## Conformance plan

- Build a model with multiple actor types, instances, and both reference labels
  and assert deterministic identity ordering.
- Reject zero identities, duplicate identities, unknown actor types, and
  unknown reference targets before publication.
- Compare equivalent models with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep turn/state/borrow/sendability, mailbox, scheduler, local/remote
  serialization, runtime, differential, and migration fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0: unchanged.
- Adds only internal publish-disabled checked-data tests and no public protocol
  or Actor support claim.

## Unresolved alternatives

Actor identity lifetime/reuse, typed reference payloads, state ownership and
turn isolation, borrow/aliasing across await, Sendable/Resource/Capability
rules, local/remote serialization, mailbox delivery, scheduler interaction,
interpreter/VM ABI, and migration remain open under the registered Actor gaps
and `ACT-2301`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
