# DEC-0090: Actor syntax rejection gate / Actor 语法拒绝门

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: concurrency-design
> 相关规范/缺口：`SEMANTICS` | `LANGUAGE` | `ROADMAP-1.0` | `GAP-ACTOR-AWAIT-REENTRY-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only negative Seed conformance evidence for the
unimplemented Actor surface. An Actor-shaped top-level declaration must be
rejected by the existing parser diagnostic before AST/HIR lowering, checked
snapshot publication, or execution. It does not define Actor syntax,
identity, turns, state isolation, message ownership, mailbox, supervision,
remote delivery, bytecode, VM behavior, or any public Actor protocol.

本决定只授权尚未实现的 Actor 表面的 Seed 负向一致性证据。类似 Actor 的顶层声明必须
由现有 parser diagnostic 在 AST/HIR lowering、checked snapshot 发布或执行之前拒绝。
本决定不定义 Actor 语法、身份、turn、状态隔离、消息所有权、mailbox、监督、远程交付、
字节码、VM 行为或任何公共 Actor protocol。

## Question

The current Seed language explicitly excludes Actor, while the execution plan
reserves `ACT-2301` for a future RFC-C203/RFC-0009 contract. How can the
repository prove that an Actor-shaped declaration cannot accidentally enter
the checked pipeline without choosing the missing Actor semantics?

当前 Seed 语言明确不实现 Actor，而执行计划把 `ACT-2301` 留给未来 RFC-C203/RFC-0009 合约。
在不选择缺失的 Actor 语义前提下，如何证明类似 Actor 的声明不会意外进入 checked pipeline？

## Decision

1. `ling-cli::compile_source` may compile a source containing a top-level
   `actor` declaration shape and MUST return the existing
   `L-SYNTAX-0010`/`UNEXPECTED_TOKEN` diagnostic with bilingual messages and
   the original UTF-8 byte span.
2. The fixture MUST prove that no `Compiled` value or checked
   `ProgramSnapshot` is published. It checks only the parser boundary and
   existing diagnostic serialization.
3. The fixture MUST NOT reserve `actor` as a new lexer token, add AST/HIR/Core
   nodes, or define Actor identity, turns, state isolation, borrow/sendability,
   mailbox, supervision, remote delivery, scheduling, Fault, Effect,
   Capability, bytecode, VM, LSP, schema, Semantic ID, or migration behavior.
4. Public `ACT-2301` remains `BlockedSpec` until an Accepted Actor authority
   resolves the registered identity, reentry, mailbox, supervision, and
   remote-delivery gaps.

1. `ling-cli::compile_source` 可以编译包含顶层 `actor` 声明形状的源码，但必须返回现有
   `L-SYNTAX-0010`/`UNEXPECTED_TOKEN` diagnostic、双语消息和原始 UTF-8 byte span。
2. fixture 必须证明不会发布 `Compiled` 值或 checked `ProgramSnapshot`，只检查 parser 边界
   和现有 diagnostic 序列化。
3. fixture 不得把 `actor` 注册为新的 lexer token，不得添加 AST/HIR/Core 节点，也不得定义
   Actor 身份、turn、状态隔离、borrow/sendability、mailbox、监督、远程交付、调度、Fault、
   Effect、Capability、bytecode、VM、LSP、schema、Semantic ID 或 migration 行为。
4. 在接受 Actor authority 解决已登记的身份、reentry、mailbox、监督和远程交付缺口前，公开
   `ACT-2301` 仍为 `BlockedSpec`。

## Normative basis

- `docs/LANGUAGE.md` §19 excludes Actor from v0.0.1 Seed.
- `docs/SEMANTICS.md` §19 describes Actor as future design and leaves turn
  reentry, message ownership, supervision, and RemoteRef boundaries to the
  concurrent specification gate.
- `docs/ROADMAP-1.0.md` §6.2 requires accepted Actor identity, turn,
  mailbox, supervision, and remote-delivery contracts before implementation.
- `DEC-0001` and `DEC-0002` govern the existing diagnostic registry and
  original UTF-8 byte-span units.

## Conformance plan

- Pass `module Main` followed by `actor Logger = ()` through the shared CLI
  compiler.
- Assert `L-SYNTAX-0010`, both bilingual message fields, the original
  `actor` span, and JSON serialization.
- Assert no checked snapshot is returned and defer all positive Actor,
  identity, state, mailbox, supervision, runtime, bytecode, VM, differential,
  and migration fixtures.

## Compatibility impact

- Adds one internal negative CLI fixture and this decision/status metadata.
- Reuses the existing `L-SYNTAX-0010`; no diagnostic allocation or lexer
  keyword change is made.
- Seed source behavior, checked Core, Semantic IDs, schemas, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 data are unchanged.

## Unresolved alternatives

Actor grammar, identity scope/reuse, turn/reentry, state isolation, message
Sendability, mailbox/backpressure, supervision, remote delivery, runtime/VM
ABI, public protocols, and migration remain open under `ACT-2301` through
`ACT-2306` and the registered Actor gaps.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
