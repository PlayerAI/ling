# DEC-0156: Internal Portable SIMD IR boundary evidence / 内部 Portable SIMD IR 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: backend-quality  
> 相关规范/缺口：`DEC-0155` | `DEC-0154` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`SIMD-4302-OBSERVATION` portable SIMD IR boundary. It records provisional lane,
memory, mask, shuffle, reduction, fallback, capability, and schema vocabulary
while Kernel and Native/backend authorities remain unresolved.

本决定只授权 `SIMD-4302-OBSERVATION` 使用 test-local 的拟议 Portable SIMD IR 边界清单，
在 Kernel 与 Native/backend 权威尚未解决时，只记录临时 lane、memory、mask、shuffle、reduction、fallback、capability 与 schema 词汇。

## Question

SIMD-4302 proposes lanes, vector loads/stores, masks, shuffles, horizontal
reductions, scalarization fallback, and target capability in a Native/Device
IR. Which planning vocabulary can be retained as bounded evidence without
defining an IR schema, operation semantics, or backend protocol?

## Decision

1. `crates/ling-types/tests/portable_simd_ir_evidence.rs` keeps a test-local
   inventory of sixty provisional input, lane, memory, mask, operation,
   fallback, capability, schema, differential, diagnostic, and protocol
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.portable-simd-ir-observation/0`. They are not IR
   instructions, schemas, verifier proofs, diagnostics, Semantic IDs, public
   protocols, or support claims.
3. No portable SIMD IR type, encoder/decoder, verifier, capability registry,
   scalarization record, dependency, diagnostic, protocol, or placeholder API
   is added. Public `SIMD-4302` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:198-208` is
  non-normative; Portable SIMD IR behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel legality, numeric determinism, Native IR/layout/ABI, target
  capabilities, and backends.
- RFC-0013/RFC-H401 and the plan's Native/SIMD dependencies are not Accepted
  authorities.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer IR grammar, lane/memory/mask/shuffle/reduction semantics, fallback,
  capability identity, schema migration, diagnostics, differential, and
  protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no SIMD IR, lowering, or support claim exists.

## Unresolved alternatives

Lane types/counts and vector values; load/store addressing/alignment/bounds/
alias; mask truth, shuffle indexes, horizontal reduction/order;
scalarization/fallback, effects/memory/Faults, shape/layout/index/overflow,
strict/relaxed FP and determinism; capability identity/required/optional
features and target rejection; canonical schema/version/identity, fixtures,
diagnostics, CPU/device differential, protocol inventory, and public IR status
remain open under SIMD-4302, SIMD-4301, CPU-4201 through CPU-4203,
KCHK-4101 through KCHK-4105, GAP-KERNEL-DEVICE-001,
GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
