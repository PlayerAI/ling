# DEC-0091: Internal Structured Task Checked-Core model / 内部 Structured Task Checked-Core 模型

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: concurrency-design  
> 相关规范/缺口：`ROADMAP-1.0` | `GAP-STRUCTURED-TASK-001` | `DEC-0089`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes one internal, publish-disabled checked-data slice for
`TASK-2201`. It validates the identity graph that a later accepted Task
lowering may produce; it does not accept Task source syntax or runtime
semantics.

本决定只授权 `TASK-2201` 的一个内部、禁止发布的 checked-data 子切片。它验证未来
被接受的 Task lowering 可能产生的 identity graph，但不接受 Task 源语法或运行时语义。

## Question

The execution plan names scope, parent/child, suspension, cancellation, and
cleanup identities, but the Structured Task lifecycle and source/Core contract
remain open. What can be implemented without making those unresolved choices
observable?

执行计划列出了 scope、父子关系、suspension、取消和 cleanup identity，但 Structured
Task 生命周期以及 source/Core contract 仍未确定。如何在不把未决选择变成可观察行为的
前提下实现一个最小边界？

## Decision

1. A `publish = false` `ling-concurrency` workspace crate provides immutable
   `TaskCore`, `TaskNode`, `SuspensionPoint`, and identity wrappers for scope,
   task body, suspension, cancellation, cleanup, and task identities.
2. `TaskCore::new` rejects zero/unresolved identities, an absent or parented
   root, duplicate tasks or suspension points, unknown parents, parent cycles,
   and incomplete detach evidence. Nodes and suspension points are stored in
   deterministic identity order.
3. Source `Span` values are retained as diagnostic evidence only. Canonical
   bytes contain no source paths, spans, host addresses, allocation order, or
   debug text; they include only the checked identity graph and the explicit
   optional detach evidence.
4. The model does not interpret AST/HIR, create a Typed Program, define a
   Task type, authorize `scope`, `let!`, `await`, `spawn`, `join`, `return`, or
   `detach`, execute cancellation or cleanup, schedule tasks, aggregate Faults,
   or expose a capability.
5. No parser, lexer keyword, diagnostic code, Semantic ID, schema, CLI/LSP
   command, bytecode instruction, VM/native ABI, scheduler, public protocol, or
   migration rule is added. Public `TASK-2201` remains `BlockedSpec` until an
   Accepted Task authority resolves the lifecycle and source/Core contract.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires explicit lifecycle, cancellation,
  cleanup, suspension, and deterministic evidence before Structured Task
  support is promoted.
- `docs/SEMANTICS.md` §18 and `docs/LANGUAGE.md` §19 keep Task outside the
  v0.0.1 Seed language and require a later concurrent specification gate.
- `DEC-0002` makes original UTF-8 byte spans authoritative for retained source
  evidence.
- `DEC-0089` protects the current negative Seed boundary and leaves positive
  Task semantics deferred.

## Conformance plan

- Build a nested root/child graph and assert canonical task ordering,
  parent/child lookup, suspension identity retention, and optional detach
  evidence.
- Reject zero identities, missing/parented roots, duplicate tasks or
  suspension points, unknown parents, parent cycles, and incomplete detach
  evidence before publication.
- Construct equivalent graphs with different insertion order and source spans
  and assert identical path-free canonical bytes.
- Keep positive source syntax, Typed-Core publication, cancellation/cleanup
  execution, scheduler interleavings, Fault aggregation, bytecode/VM,
  differential, protocol, and migration fixtures deferred.

## Compatibility impact

- Seed source acceptance, parser behavior, diagnostics, Semantic IDs, schemas,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0:
  unchanged.
- Adds only a publish-disabled internal workspace crate and unit tests. It does
  not register a public protocol or claim v0.2 Task support.

## Unresolved alternatives

Task grammar, scope ownership, suspension/resume semantics, cancellation
propagation, cleanup order, Fault aggregation, detach authorization,
deterministic scheduling, runtime lowering, message/effect boundaries, and
migration remain open under `GAP-STRUCTURED-TASK-001` and `TASK-2201` through
`TASK-2206`. A later Accepted RFC or decision may supersede this data-only
projection with explicit migration evidence.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
