# DEC-0116: Internal Resource and Drop boundary evidence / 内部 Resource 与 Drop 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: memory-design
> 相关规范/缺口：`DEC-0115` | `DEC-0009` | `DEC-0010` | `DEC-0013` | `GAP-OWNERSHIP-MODEL-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed Resource and
Drop boundaries for the bounded `MEM-3103-OBSERVATION` child. It checks
deterministic, duplicate-free vocabulary. It does not define ownership,
destruction timing, cleanup Effects/Faults, cancellation behavior, Managed
finalization, or FFI transfer semantics.

本决定只授权 test-only 的拟议 Resource 与 Drop 边界清单，供
`MEM-3103-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 ownership、销毁时机、cleanup
Effect/Fault、cancellation 行为、Managed finalization 或 FFI transfer 语义。

## Question

The G3 plan requires unique ownership, move/use-after-move rejection, explicit
or derived Drop, cleanup across returns and failures, cancellation-safe
release, FFI transfer, and separation from GC finalization. Which evidence can
be retained without freezing a Resource or cleanup contract?

G3 计划要求 unique ownership、move/use-after-move rejection、explicit/derived Drop、return/failure cleanup、
cancellation-safe release、FFI transfer 以及与 GC finalization 的分离。在不冻结 Resource 或 cleanup 契约的情况下，
可以保留哪些证据？

## Decision

1. `crates/ling-effects/tests/resource_drop_evidence.rs` keeps a test-local
   inventory of thirty-three provisional boundaries: Resource identity and
   ownership, move/use-after-move, explicit/derived and aggregate/branch/
   loop/closure Drop order, generic/Trait/Actor/suspension interactions,
   cleanup on return/error/Fault/cancellation/timeout/termination/partial
   failure, Drop Effects/Faults, Capability restrictions, Managed finalizer
   separation, FFI transfer/lifetime/thread boundaries, Native ABI,
   diagnostics, Unicode spans, interpreter/VM/Native differentials, and
   deterministic bounded cleanup.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.resource-drop-observation/0`. These bytes are not a Resource,
   ownership token, Drop operation, cleanup Effect/Fault, finalizer, FFI mode,
   or runtime contract.
3. The child adds no Resource type, ownership checker, Drop lowering, cleanup
   stack, Effect/Fault, cancellation hook, FFI transfer mode, diagnostic,
   Semantic ID, public protocol, or migration rule. Public `MEM-3103` remains
   `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its Resource checklist cannot
  authorize affine ownership, destruction timing, cleanup failure, or FFI ABI.
- `DEC-0115` keeps Value-layout and Copy/Move vocabulary test-only while the
  memory/ownership authority is absent.
- `DEC-0009` governs Seed mutation boundaries and explicitly does not
  implement Resource, Borrow, or Drop behavior.
- `DEC-0010` and `DEC-0013` govern Seed capabilities and main/runtime Faults;
  they do not define Resource cleanup or FFI transfer.
- `GAP-OWNERSHIP-MODEL-001` remains Open; this decision records Resource
  vocabulary without resolving the gap.

## Conformance plan

- Assert all thirty-three provisional Resource/Drop boundaries and their
  test-local order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep ownership, move checking, Drop order, cleanup failure/cancellation,
  Effect/Fault mapping, Managed finalization, FFI transfer, diagnostics,
  migration, fuzzing, and interpreter/VM/Native fixtures deferred.

## Compatibility impact

- Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public Resource, Drop, ownership,
  or cleanup protocol claim is registered.

## Unresolved alternatives

Resource identity and ownership transfer, Drop order, cleanup failure and
cancellation behavior, Effect/Fault representation, Capability restrictions,
Managed/GC separation, FFI lifetime/thread/ABI rules, diagnostics,
optimization, migration, and differential behavior remain open under
`GAP-OWNERSHIP-MODEL-001` and `MEM-3103`.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
