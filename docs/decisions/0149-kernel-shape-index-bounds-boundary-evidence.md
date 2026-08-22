# DEC-0149: Internal Kernel shape/index/bounds boundary evidence / 内部 Kernel Shape、Index 与 Bounds 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: kernel-quality  
> 相关规范/缺口：`DEC-0148` | `DEC-0147` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`KCHK-4103-OBSERVATION` Kernel shape, index, and bounds boundary. It records
provisional shape/index vocabulary while Kernel layout, numeric, ownership,
alias/race, and device authorities remain unresolved.

本决定只授权 `KCHK-4103-OBSERVATION` 使用 test-local 的拟议 Kernel Shape、Index 与 Bounds
边界清单，在 Kernel layout、numeric、ownership、alias/race 与 device 权威尚未解决时，只记录临时词汇。

## Question

KCHK-4103 proposes shape/rank/extent/layout and index/bounds validation for
Kernel values, including slicing, broadcasting, reductions, alias/race, and
device buffers. Which planning vocabulary can be retained as bounded evidence
without adding Kernel shape syntax, a verifier, or device bounds semantics?

KCHK-4103 计划为 Kernel 值提供 shape/rank/extent/layout 与 index/bounds 验证，包含 slicing、
broadcasting、reduction、alias/race 与 device buffers。在不添加 Kernel shape 语法、verifier
或 device bounds 语义的前提下，哪些规划词汇可以作为有界证据保留？

## Decision

1. `crates/ling-types/tests/kernel_shape_index_bounds_evidence.rs` keeps a
   test-local inventory of sixty provisional boundaries covering shape/schema,
   dimensions/rank/extents/strides/layout, index types/arity/origin/
   normalization, slicing/gather/scatter/broadcast/reshape/transpose,
   bounds/lower/upper/negative/overflow/division/out-of-range rejection,
   empty/zero/dynamic/static/symbolic shapes and inference/proofs,
   alias/race/buffer/address/ownership/device/profile/target, checked Typed
   Core/verified derivative, canonical/provenance/spans/Semantic IDs,
   diagnostics/Unicode, fixtures/golden/round-trip/unknown/migration, CPU
   reference/device differential, host exclusion, and protocol inventory.
2. The test-local inventory sorts boundaries by explicit local rank, rejects
   duplicates, and emits opaque evidence bytes tagged
   `ling.kernel-shape-index-observation/0`. These bytes are not shape or bounds
   semantics, a verifier result, diagnostic, provenance record, Semantic ID,
   public protocol, or device support claim.
3. The child adds no Kernel syntax, shape/index schema, checker/verifier,
   Device Buffer API, backend, diagnostic, dependency, toolchain, CLI command,
   protocol, or placeholder API. Public `KCHK-4103` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:103-111` is
  non-normative and cannot define shape/layout/index/bounds semantics.
- `docs/SEMANTICS.md`/`docs/LANGUAGE.md` reserve Kernel and Device Buffer
  behavior outside v0.0.1. RFC-0013/RFC-H401 are not Accepted.
- `GAP-KERNEL-DEVICE-001` remains Open for shapes, ownership/address spaces,
  synchronization, numeric determinism, Placement, and backend discovery.

## Conformance plan

- Assert all sixty provisional shape/index/bounds boundaries and their
  test-local order.
- Compare forward and reversed insertion order and require identical opaque
  evidence bytes.
- Reject duplicate boundary vocabulary.
- Keep shape/index/bounds semantics, verifier, alias/race/numeric/device policy,
  CPU reference, migration, diagnostics, and public support behavior deferred.

## Compatibility impact

- Accepted Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP,
  runtime, bytecode, VM, dependencies, and Unicode 17.0.0 are unchanged.
- Adds only test-local boundary evidence. No Kernel shape/checker decision,
  Device Buffer API, diagnostic, dependency, protocol, backend, or support
  claim is registered.

## Unresolved alternatives

Shape/layout/index model; slicing/gather/scatter/broadcast/reshape/transpose;
bounds/overflow/empty/dynamic/static/symbolic/inference/proofs; alias/race,
buffer/address/ownership/device/profile/target; Typed Core/verifier, numeric,
CPU reference, differential, diagnostics, migration, protocol inventory, and
backend/editor support remain open under KCHK-4103, KCHK-4101/4102,
GAP-KERNEL-DEVICE-001, and missing RFC-0013/RFC-H401 authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
