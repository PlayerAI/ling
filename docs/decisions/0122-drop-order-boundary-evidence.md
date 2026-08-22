# DEC-0122: Internal Drop-order and cleanup boundary evidence / 内部 Drop 顺序与 Cleanup 边界证据

> 状态：Accepted
> Status: Accepted
> 提出日期：2026-08-22
> 决定日期：2026-08-22
> Owner role: ownership-design
> 相关规范/缺口：`DEC-0121` | `DEC-0116` | `DEC-0009` | `GAP-OWNERSHIP-MODEL-001`
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-only inventory of proposed Drop-order
and cleanup boundaries for the bounded `OWN-3205-OBSERVATION` child. It
checks deterministic, duplicate-free vocabulary. It does not define Resource
ownership, implicit or explicit Drop, Cleanup Core, destruction order,
cancellation cleanup, failure aggregation, diagnostics, or backend behavior.

本决定只授权 test-only 的拟议 Drop 顺序与 cleanup 边界清单，供
`OWN-3205-OBSERVATION` 子任务使用。它只检查确定性、无重复的词汇；不定义 Resource ownership、implicit/explicit Drop、Cleanup Core、
destruction order、cancellation cleanup、failure aggregation、diagnostic 或 backend 行为。

## Question

The G3 plan sketches implicit Drop insertion into Cleanup Core, reverse
declaration or RFC-defined order, branch/early-return/error/Fault paths,
cancellation, partial initialization, panic/unwind rejection, and explicit
drop-failure rules. Which evidence can be retained without freezing cleanup
semantics?

G3 计划列出将 implicit Drop 插入 Cleanup Core、reverse declaration 或 RFC order、branch/early-return/error/Fault path、
cancellation、partial initialization、panic/unwind rejection 与显式 drop-failure rule。在不冻结 cleanup 语义的情况下，可以保留哪些证据？

## Decision

1. `crates/ling-effects/tests/drop_order_evidence.rs` keeps a test-local
   inventory of forty-one provisional boundaries: Resource identity and
   ownership transfer, Move/Borrow/Region, implicit/explicit Drop, aggregate/
   branch/loop/closure order, reverse declaration versus RFC-defined order,
   partial initialization and replacement, Cleanup Core, normal/early return,
   `?`/Error/Fault, cancellation/timeout, Task/Actor termination and process
   shutdown, panic/unwind rejection, idempotence/partial cleanup/failure
   aggregation, Effects/Faults/bounds, Managed separation, Capability/network,
   Native/FFI ABI, Profile/Critical, migration, deterministic optimization,
   diagnostics, projections, Unicode spans, differential evidence, and Seed
   migration.
2. The test-only inventory sorts boundaries by an explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.drop-order-observation/0`. These bytes are not a Resource, Drop
   operation, order, Cleanup Core, failure result, diagnostic, or ownership
   contract.
3. The child adds no Resource/Drop Core node, cleanup lowering, destruction
   order, cancellation cleanup, failure mapping, diagnostic, Semantic ID,
   public protocol, or migration rule. Public `OWN-3205` remains
   `BlockedSpec`.

## Normative basis

- The G3 execution package is non-normative; its cleanup checklist cannot
  authorize destruction order, implicit operations, failure aggregation,
  cancellation, or backend unwinding behavior.
- `DEC-0121` keeps suspension/Actor boundaries test-only, and `DEC-0116`
  keeps Resource/Drop vocabulary test-only while ownership authority is absent.
- `DEC-0009` governs Seed mutable-place writes and excludes Resource, Borrow,
  and Drop semantics.
- `GAP-OWNERSHIP-MODEL-001` remains Open; this decision records cleanup
  vocabulary without resolving the gap.

## Conformance plan

- Assert all forty-one provisional Drop/cleanup boundaries and their test-local
  order.
- Compare forward and reversed boundary insertion order and require identical
  test-only evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep Resource ownership, Drop order, Cleanup Core, cancellation/failure,
  diagnostics, migration, fuzzing, and interpreter/VM/Native fixtures
  deferred.

## Compatibility impact

- Accepted Seed source acceptance, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, ABI, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-only boundary evidence. No public Resource, Drop, cleanup, or
  failure protocol claim is registered.

## Unresolved alternatives

Resource identity and transfer, implicit versus explicit Drop, aggregate and
control-flow order, Cleanup Core shape, partial initialization, cancellation/
termination, failure aggregation, Effects/Faults, Managed separation,
Capability/network, Native/FFI ABI, Profile/Critical, diagnostics, migration,
and differential semantics remain open under `GAP-OWNERSHIP-MODEL-001`,
`OWN-3205`, and missing RFC-N304/RFC-0007 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
