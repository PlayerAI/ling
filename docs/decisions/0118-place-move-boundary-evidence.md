# DEC-0118: Internal Place and Move-analysis boundary evidence / 内部 Place 与 Move 分析边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: ownership-design
> 相关规范/缺口：`DEC-0117` | `DEC-0009` | `RFC-0017` | `GAP-OWNERSHIP-MODEL-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed Place and
Move-analysis boundaries for the bounded `OWN-3201-OBSERVATION` child. It
checks deterministic, duplicate-free vocabulary. It does not define move,
copy, borrow, lifetime, dataflow, diagnostics, or ownership semantics.

本决定只授权 test-only 的拟议 Place 与 Move 分析边界清单，供
`OWN-3201-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 move、copy、borrow、lifetime、dataflow、
diagnostic 或 ownership 语义。

## Question

The G3 plan names local/field/index places, projections, move/copy/borrow,
initialization and partial moves, closure and aggregate analysis, branch/loop
joins, and Task/Actor/FFI boundaries. Which evidence can be retained without
freezing a future ownership lattice or public lifetime contract?

G3 计划列出 local/field/index place、projection、move/copy/borrow、initialization 与 partial move、closure 与
aggregate analysis、branch/loop join 以及 Task/Actor/FFI boundary。在不冻结未来 ownership lattice 或 public
lifetime 契约的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-types/tests/place_move_evidence.rs` keeps a test-local
   inventory of thirty-four provisional boundaries: local/field/index places,
   projections, move/copy/borrow/borrow_mut, initialization and partial
   moves, reinitialization/destructuring/closure/aggregate/generic forms,
   branch/loop/match joins, Error/Fault/cancellation, Task/Actor/suspension,
   Resource/Managed, lifetime/region, FFI/Native, diagnostics, Semantic
   Graph/Audit Source, Unicode spans, differential evidence, deterministic
   termination, and Seed migration.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.place-move-observation/0`. These bytes are not a Place, move state,
   borrow, lifetime, dataflow result, diagnostic, or ownership contract.
3. The child adds no future Typed Core place form, move/borrow state, dataflow
   solver, diagnostic, Semantic ID, public protocol, or migration rule.
   Accepted Seed Place behavior remains unchanged and public `OWN-3201`
   remains `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its ownership sketch cannot
  authorize future Typed Core states or public lifetime behavior.
- Accepted RFC-0017 and DEC-0009 govern only the Seed mutable local/record-field
  Place slice and do not define future move/borrow states.
- `DEC-0117` keeps Managed/island vocabulary test-only while memory authority
  is absent.
- `GAP-OWNERSHIP-MODEL-001` remains Open; this decision records Place/Move
  vocabulary without resolving the gap.

## Conformance plan

- Assert all thirty-four provisional Place/Move boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep ownership judgments, dataflow/fixed points, diagnostics, lifetimes,
  suspension/Actor boundaries, FFI, migration, fuzzing, and
  interpreter/VM/Native fixtures deferred.

## Compatibility impact

- Accepted Seed source acceptance, diagnostics, schemas, Semantic IDs,
  CLI/LSP, runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are
  unchanged.
- Adds only test-only boundary evidence. No public ownership, lifetime, or
  Place/Move protocol claim is registered.

## Unresolved alternatives

Place forms and projections, move/copy/borrow judgments, partial moves and
reinitialization, dataflow joins/fixed points, closure/aggregate/generic
interaction, lifetime/region projection, suspension/Task/Actor escape,
Resource/Managed/FFI/Native behavior, diagnostics, migration, and differential
semantics remain open under `GAP-OWNERSHIP-MODEL-001` and `OWN-3201`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
