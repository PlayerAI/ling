# DEC-0089: Structured Task syntax rejection gate / Structured Task 语法拒绝门

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: concurrency-design
> 相关规范/缺口：`SEMANTICS` | `LANGUAGE` | `ROADMAP-1.0` | `GAP-STRUCTURED-TASK-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only negative Seed conformance evidence for the
unimplemented Structured Task surface. A Task-shaped top-level declaration
must be rejected by the existing parser diagnostic before AST/HIR lowering,
checked snapshot publication, or execution. It does not define Task syntax,
scope ownership, suspension, cancellation, cleanup, Fault aggregation,
detach, scheduling, bytecode, VM behavior, or a public Task protocol.

本决定只授权尚未实现的 Structured Task 表面的 Seed 负向一致性证据。类似 Task
的顶层声明必须由现有 parser diagnostic 在 AST/HIR lowering、checked snapshot
发布或执行之前拒绝。本决定不定义 Task 语法、作用域所有权、suspension、取消、清理、
Fault 聚合、detach、调度、字节码、VM 行为或公共 Task protocol。

## Question

The current Seed language explicitly excludes Task/Actor/Node/Kernel, while
the execution plan reserves `TASK-2201` for a future RFC-C202/RFC-0008
contract. How can the repository prove that a Task-shaped declaration cannot
accidentally enter the checked pipeline without choosing the missing Task
semantics?

当前 Seed 语言明确不实现 Task/Actor/Node/Kernel，而执行计划把 `TASK-2201` 留给未来
RFC-C202/RFC-0008 合约。在不选择缺失的 Task 语义前提下，如何证明类似 Task 的声明
不会意外进入 checked pipeline？

## Decision

1. `ling-cli::compile_source` may compile a source containing a top-level
   `task` declaration shape and MUST return the existing
   `L-SYNTAX-0010`/`UNEXPECTED_TOKEN` diagnostic. The diagnostic retains the
   original UTF-8 byte span and bilingual message fields.
2. The fixture MUST prove that no `Compiled` value or checked
   `ProgramSnapshot` is published. It checks only the parser boundary and
   existing diagnostic serialization.
3. The fixture MUST NOT reserve `task` as a new lexer token, add AST/HIR/Core
   nodes, define `scope`, `let!`, `await`, `spawn`, `join`, cancellation,
   cleanup, detach, scheduler, Fault, Effect, Capability, bytecode, VM, LSP,
   schema, Semantic ID, or migration behavior.
4. Public `TASK-2201` remains `BlockedSpec` until an Accepted Task authority
   resolves the registered lifecycle and suspension gap.

1. `ling-cli::compile_source` 可以编译包含顶层 `task` 声明形状的源码，但必须返回现有
   `L-SYNTAX-0010`/`UNEXPECTED_TOKEN` diagnostic，并保留原始 UTF-8 byte span 和双语消息字段。
2. fixture 必须证明不会发布 `Compiled` 值或 checked `ProgramSnapshot`，只检查 parser 边界
   和现有 diagnostic 序列化。
3. fixture 不得把 `task` 注册为新的 lexer token，不得添加 AST/HIR/Core 节点，也不得定义
   `scope`、`let!`、`await`、`spawn`、`join`、取消、清理、detach、scheduler、Fault、Effect、
   Capability、bytecode、VM、LSP、schema、Semantic ID 或 migration 行为。
4. 在接受 Task authority 解决已登记的生命周期与 suspension 缺口前，公开 `TASK-2201`
   仍为 `BlockedSpec`。

## Normative basis

- `docs/LANGUAGE.md` §19 excludes Task from v0.0.1 Seed.
- `docs/SEMANTICS.md` §18 states that Task is not implemented before v0.2
  and leaves the lifecycle contract for the concurrent specification gate.
- `docs/ROADMAP-1.0.md` §6.2 requires an accepted Structured Task lifecycle,
  cancellation, detach, and suspension contract before implementation.
- `DEC-0001` and `DEC-0002` govern the existing diagnostic registry and
  original UTF-8 byte-span units.

## Conformance plan

- Pass `module Main` followed by `task main () = ()` through the shared CLI
  compiler.
- Assert `L-SYNTAX-0010`, both bilingual message fields, the original
  `task` span, and JSON serialization.
- Assert no checked snapshot is returned and defer all positive Task,
  lifecycle, runtime, scheduler, bytecode, VM, differential, and migration
  fixtures.

## Compatibility impact

- Adds one internal negative CLI fixture and this decision/status metadata.
- Reuses the existing `L-SYNTAX-0010`; no diagnostic allocation or lexer
  keyword change is made.
- Seed source behavior, checked Core, Semantic IDs, schemas, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0 data are unchanged.

## Unresolved alternatives

Task grammar, scope identity, parent/child ownership, suspension frames,
cancellation propagation, cleanup order, Fault aggregation, detach authority,
deterministic scheduler, interpreter/VM lowering, public diagnostics, schemas,
protocols, and migration remain open under `TASK-2201` through `TASK-2206` and
`GAP-STRUCTURED-TASK-001`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
