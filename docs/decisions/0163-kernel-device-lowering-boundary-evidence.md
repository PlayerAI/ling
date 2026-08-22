# DEC-0163: Internal Kernel-to-Device lowering boundary evidence / 内部 Kernel 到 Device Lowering 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: lowering-quality  
> 相关规范/缺口：`DEC-0162` | `DEC-0157` | `DEC-0151` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`DIR-4502-OBSERVATION` Kernel Core to Device IR lowering boundary. It records
provisional source/Checked Core, lowering slices, proof/provenance,
ownership/effect/synchronization/numeric, capability/rejection, differential,
diagnostic, and protocol vocabulary while Kernel, Device IR, and backend
authorities remain unresolved.

本决定只授权 `DIR-4502-OBSERVATION` 使用 test-local 的拟议 Kernel Core 到 Device IR lowering 边界清单，
在 Kernel、Device IR 与 backend 权威尚未解决时，只记录临时 source/Checked Core、lowering slices、proof/provenance、ownership/effect/synchronization/numeric、capability/rejection、differential、diagnostic 与 protocol 词汇。

## Question

DIR-4502 proposes verifier-preserving lowering for elementwise, index/shape,
local memory, reduction, vector/tensor, synchronization, and source-diagnostic
map slices. Which planning vocabulary can be retained as bounded evidence
without defining Kernel legality, Device IR mappings, proof preservation,
rejection, or backend behavior?

## Decision

1. `crates/ling-types/tests/kernel_device_lowering_evidence.rs` keeps a
   test-local inventory of sixty provisional source/Typed Core, lowering-slice,
   proof/provenance, ownership/effect/synchronization/numeric,
   capability/rejection, differential, diagnostic, fixture, host-exclusion,
   and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.kernel-device-lowering-observation/0`. They are
   not Kernel nodes, lowering maps, proof witnesses, IR operations,
   capabilities, diagnostics, Semantic IDs, public protocols, or backend
   support claims.
3. No Kernel verifier extension, lowerer, mapper, proof carrier, dependency,
   diagnostic, protocol, or placeholder API is added. Public `DIR-4502`
   remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:305-315` is
  non-normative; lowering behavior remains outside v0.0.1.
- RFC-H404 is absent; DIR-4501 is BlockedSpec and does not authorize a
  lowerer.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel types/effects, memory, synchronization, numeric, IR, ABI, target,
  capability, and backend behavior.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer verifier/lowering slices, proof preservation, ownership/effects,
  numeric/differential semantics, capability/rejection/fallback, source maps,
  diagnostics, migration, and protocol behavior until accepted authority
  exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no Kernel lowerer, Device IR mapping, proof, backend, or support
claim exists.

## Unresolved alternatives

Kernel source/Profile and Typed Core legal subset; elementwise/index/shape/
bounds/local-memory/reduction/vector/tensor/synchronization slices;
types/effects/ownership/alias proofs; memory/address spaces/layout/numeric and
determinism; Fault/cancellation; capability/required features, target scope,
fallback/rejection; pre/postconditions, provenance, identity/spans, resource
limits, differential CPU reference, canonical version/migration, diagnostics,
host/driver redaction, protocol inventory, and public lowering status remain
open under DIR-4502, DIR-4501, KCHK-4101 through KCHK-4105, CPU-4201 through
CPU-4203, SIMD-4301 through SIMD-4303, DBUF-4401 through DBUF-4404,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
