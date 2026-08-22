# DEC-0155: Internal SIMD legality boundary evidence / 内部 SIMD 合法性边界证据

> 状态：Accepted  
> 提出日期：2026-08-23  
> 决定日期：2026-08-23  
> Owner role: backend-quality  
> 相关规范/缺口：`DEC-0154` | `DEC-0152` | `ROADMAP-1.0` | `GAP-KERNEL-DEVICE-001` | `GAP-NATIVE-BACKEND-ABI-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes only a test-local inventory of the proposed
`SIMD-4301-OBSERVATION` legality and scalar-fallback boundary. It records
provisional proof vocabulary while Kernel, CPU-reference, and Native/backend
authorities remain unresolved.

本决定只授权 `SIMD-4301-OBSERVATION` 使用 test-local 的拟议 SIMD 合法性与 scalar fallback 边界清单，
在 Kernel、CPU-reference 与 Native/backend 权威尚未解决时，只记录临时 proof 词汇。

## Question

SIMD-4301 proposes independent-iteration, alignment, vector-width, tail,
alias, reduction, floating-point, target-feature, and fallback reasoning.
Which planning vocabulary can be retained as bounded evidence without
defining vectorization legality, target negotiation, or optimizer behavior?

## Decision

1. `crates/ling-types/tests/simd_legality_evidence.rs` keeps a test-local
   inventory of sixty provisional input, legality, proof, numeric, fallback,
   differential, diagnostic, and protocol boundaries.
2. The inventory sorts by explicit local rank, rejects duplicates, and emits
   opaque bytes tagged `ling.simd-legality-observation/0`. They are not
   legality proofs, vector IR, optimizer decisions, diagnostics, Semantic IDs,
   backend protocols, or support claims.
3. No SIMD legality pass, vector IR, target-feature registry, scalar fallback
   record, dependency, diagnostic, protocol, or placeholder API is added.
   Public `SIMD-4301` remains `BlockedSpec`.

## Normative basis

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:181-196` is
  non-normative; SIMD behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel legality, numeric determinism, target capabilities, Native IR/ABI,
  and backends.
- RFC-0013/RFC-H401 and the plan's SIMD/Native dependencies are not Accepted
  authorities.

## Conformance plan

- Assert all sixty boundaries and local order; compare forward/reverse opaque
  bytes; reject duplicates.
- Defer legality proofs, target negotiation, fallback equivalence, numeric
  modes, diagnostics, migration, differential, and protocol behavior until
  accepted authority exists.

## Compatibility impact

Seed behavior, diagnostics, schemas, Semantic IDs, CLI/LSP, runtime, bytecode,
VM, dependencies, and Unicode 17.0.0 are unchanged. Only test-local evidence
is added; no vectorizer, fallback behavior, or support claim exists.

## Unresolved alternatives

Iteration/dependence and effect/shape/bounds/alias/ownership proofs;
alignment/width/tail/overflow; strict/relaxed FP and reduction order;
target-feature identity/negotiation; fallback permission/reason/equivalence;
canonical proof facts, provenance, fixtures, diagnostics, CPU/SIMD
differential, Native IR/ABI, target backends, protocol inventory, and public
legality status remain open under SIMD-4301, CPU-4201 through CPU-4203,
KCHK-4101 through KCHK-4105, GAP-KERNEL-DEVICE-001,
GAP-NATIVE-BACKEND-ABI-001, and missing authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
