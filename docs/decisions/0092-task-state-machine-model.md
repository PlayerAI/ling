# DEC-0092: Internal Task state-machine identity model / 内部 Task 状态机 identity 模型

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: concurrency-design  
> 相关规范/缺口：`ROADMAP-1.0` | `GAP-STRUCTURED-TASK-001` | `DEC-0091`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a publish-disabled structural model for the
`TASK-2202` state-machine boundary. It records opaque state, continuation,
live-local, and edge identities without defining how a Task executes.

本决定只授权 `TASK-2202` 的 publish-disabled 结构模型。它记录 opaque state、
continuation、live-local 和 edge identity，但不定义 Task 如何执行。

## Question

The execution plan names live locals, continuation state, cancellation, cleanup,
Fault, and source-map edges, while the Task lowering and bytecode ABI remain
unresolved. Which checked-data invariants can be frozen without choosing a
runtime or serialization contract?

执行计划列出 live locals、continuation state、cancellation、cleanup、Fault 和
source-map edge，但 Task lowering 与 bytecode ABI 尚未确定。哪些 checked-data
不变量可以在不选择运行时或序列化合约的情况下先固定？

## Decision

1. The internal `ling-concurrency` crate adds `StateMachineModel`, immutable
   state nodes, structural transition edges, and typed identities for states,
   transitions, continuations, and live locals.
2. Construction rejects zero identities, a missing entry state, duplicate
   states or transition IDs, repeated live locals, invalid or unknown edge
   endpoints, and duplicate `(from, to, kind)` edges. States, locals, and
   transitions are stored in deterministic identity order.
3. `Resume`, `Cancel`, `Cleanup`, and `Fault` are labels only. They do not
   authorize cancellation propagation, cleanup execution, Fault aggregation,
   scheduling, borrow rules, resource ownership, or state-machine lowering.
4. Source spans are evidence only. Canonical bytes contain no source paths,
   spans, allocation order, host addresses, debug text, instruction encoding,
   version migration, or public schema fields.
5. No parser, AST/HIR/typed-program integration, bytecode opcode, verifier rule,
   VM/native ABI, interpreter behavior, CLI/LSP command, diagnostic, Semantic
   ID, public protocol, or migration rule is added. Public `TASK-2202` remains
   `BlockedSpec` until the Task lifecycle and state-machine authority is
   Accepted.

## Normative basis

- `docs/ROADMAP-1.0.md` §6.2 requires explicit suspension, cleanup,
  cancellation, deterministic scheduling, and differential evidence before
  Structured Task support is promoted.
- `docs/SEMANTICS.md` §18 keeps Task outside the v0.0.1 Seed subset.
- `DEC-0002` fixes original UTF-8 byte spans as source evidence.
- `DEC-0091` provides the preceding internal Task identity graph without
  accepting source syntax or runtime semantics.

## Conformance plan

- Build a model with resume and cleanup/fault-labelled edges and assert sorted
  state/local/transition identities.
- Reject missing entries, duplicate states, repeated locals, unknown endpoints,
  duplicate transition IDs, and duplicate structural edges.
- Compare equivalent models with different insertion order and source spans and
  require identical path-free canonical bytes.
- Keep actual suspension lowering, continuation liveness, cancellation,
  cleanup/Fault behavior, bytecode/VM, differential, and migration fixtures
  deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime,
  bytecode, VM, ABI, dependencies, and Unicode 17.0.0: unchanged.
- Adds only internal publish-disabled checked-data tests and no public protocol
  or v0.2 support claim.

## Unresolved alternatives

State numbering, continuation frame layout, live-local typing, borrow and
resource rules across suspension, cancellation/cleanup/Fault semantics,
instruction encoding, verifier limits, interpreter/VM equivalence, source-map
projection, and migration remain open under `GAP-STRUCTURED-TASK-001` and
`TASK-2202`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
