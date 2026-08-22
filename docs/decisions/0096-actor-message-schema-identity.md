# DEC-0096: Internal Actor message-schema identity / 内部 Actor 消息 schema 身份

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: actor-semantics  
> 相关规范/缺口：`DEC-0095` | `DEC-0008` | `DEC-0009` | `DEC-0010` | `GAP-ACTOR-MAILBOX-SUPERVISOR-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled, non-executable message-schema
identity model for the bounded `ACT-2302` child. It records opaque schema and
field identities without selecting Sendable, ownership, Capability, or wire
semantics.

本决定只授权 `ACT-2302` 的 publish-disabled、不可执行消息 schema 身份模型。它记录不透明
的 schema 与 field identity，但不选择 Sendable、所有权、Capability 或 wire 语义。

## Question

The Actor message target needs a checked identity boundary for future Semantic
Graph evidence, but message types, recursive ownership, Resource/Managed rules,
Capability transfer, and local/remote serialization remain open. What data can
be recorded without creating a message checker or public schema?

Actor message 目标需要用于未来 Semantic Graph 证据的 checked identity 边界，但 message
类型、递归所有权、Resource/Managed 规则、Capability transfer 和 local/remote 序列化仍
未确定。哪些 data 可以记录而不创建 message checker 或公开 schema？

## Decision

1. The internal `ling-concurrency` crate provides immutable
   `MessageSchemaIdentityModel`, `MessageSchemaId`, and `MessageFieldId` values.
2. A schema may carry an optional opaque Actor type owner and an ordered set of
   opaque field identities. These are identity facts only; fields have no type,
   payload, ownership, effect, Capability, or encoding meaning.
3. Construction rejects zero/unresolved schema, owner, and field identities,
   duplicate schemas, and repeated fields within one schema. Schemas and fields
   are stored deterministically by identity.
4. Source spans are evidence only. Canonical bytes contain no source paths,
   spans, payload values, host addresses, allocation order, wire fields, or
   public schema/version data.
5. No Sendable judgment, borrow/move/Resource/Managed rule, Capability filter,
   AST/HIR/typed-program node, Semantic Graph projection, mailbox, serializer,
   runtime, diagnostic, Semantic ID, CLI/LSP command, public protocol, or
   migration rule is added. Public `ACT-2302` remains `BlockedSpec`.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor message ownership,
  mailbox, serialization, and differential contracts before message execution.
- `docs/SEMANTICS.md` keeps Actor messages and Ownership outside the v0.0.1
  Seed subset.
- `DEC-0008`, `DEC-0009`, and `DEC-0010` authorize Seed value, mutation, State,
  and Capability behavior only; they do not define Actor message transfer.
- `DEC-0095` authorizes the preceding opaque Actor identity boundary.
- `GAP-ACTOR-AWAIT-REENTRY-001`, `GAP-ACTOR-MAILBOX-SUPERVISOR-001`, and
  `GAP-ACTOR-REMOTE-DELIVERY-001` remain open.

## Conformance plan

- Build a model with multiple schemas, optional owners, and unsorted field
  identities and assert deterministic schema/field ordering.
- Reject zero identities, duplicate schemas, and repeated fields before
  publication.
- Compare equivalent models with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep Sendable, borrow/move/Resource/Managed, Capability, payload typing,
  serialization, mailbox, runtime, differential, and migration fixtures
  deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0: unchanged.
- Adds only internal publish-disabled checked-data tests and no public protocol
  or Actor message support claim.

## Unresolved alternatives

Message type identity and recursion, Sendable/ownership judgments,
Resource/Managed profiles, Capability transfer/non-forgery, schema versioning,
local/remote wire formats, mailbox outcomes, runtime ABI, and migration remain
open under the registered Actor gaps and `ACT-2302`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
