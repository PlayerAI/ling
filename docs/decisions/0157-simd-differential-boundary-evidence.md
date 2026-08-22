# DEC-0157: Internal SIMD differential boundary evidence / 内部 SIMD Differential 边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: backend-quality  
> 相关规范/缺口：`DEC-0156` | `DEC-0155` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`SIMD-4303-OBSERVATION` scalar/SIMD differential boundary. It records
provisional comparison, tolerance, Fault, provenance, and artifact vocabulary
while the execution and backend authorities remain unresolved.

本决定只授权 `SIMD-4303-OBSERVATION` 使用 test-local 的拟议 scalar/SIMD differential 边界清单，
在 execution 与 backend 权威尚未解决时，只记录临时 comparison、tolerance、Fault、provenance 与 artifact 词汇。

## Question

SIMD-4303 proposes integer exactness, strict and relaxed floating-point
comparison, tails, unaligned access, overflow, and Fault-location comparison.
Which planning vocabulary can be retained as bounded evidence without defining
an equality/tolerance contract or differential protocol?

## Decision

1. `crates/ling-types/tests/simd_differential_evidence.rs` keeps a test-local
   inventory of sixty provisional input, scalar/SIMD result, comparison,
   numeric, Fault, effect, provenance, fixture, diagnostic, and protocol
   boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.simd-differential-observation/0`. They are not
   comparison results, tolerance rules, Fault classifications, diagnostics,
   Semantic IDs, public protocols, or support claims.
3. No differential runner, comparison-result schema, tolerance policy, Fault
   mapper, target matrix, dependency, diagnostic, protocol, or placeholder API
   is added. Public `SIMD-4303` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:210-219` is
  non-normative; SIMD differential behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel numeric/Fault semantics, Native IR/ABI, target capabilities, and
  backends.
- RFC-0013/RFC-H401 and the plan's differential dependencies are not Accepted
  authorities.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer exact/tolerance equality, numeric modes, Fault/effect equivalence,
  target mismatch classification, canonical results, diagnostics, migration,
  differential, and protocol behavior until accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no differential runner, comparison contract, or support claim exists.

## Unresolved alternatives

Verified scalar/SIMD inputs and artifacts; integer/structural exactness; strict
FP rounding/NaN/infinity/signed-zero/overflow; relaxed FP tolerances/metrics;
reduction/tail/unaligned/alignment/determinism; Fault identity/spans and effect
equivalence; unsupported capability vs mismatch vs evidence failure; target
features, canonical outputs/traces, redaction/corruption/migration, fixtures,
diagnostics, CPU/device differential, protocol inventory, and public
differential status remain open under SIMD-4303, SIMD-4301/4302,
CPU-4201 through CPU-4203, KCHK-4101 through KCHK-4105,
GAP-KERNEL-DEVICE-001, GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
