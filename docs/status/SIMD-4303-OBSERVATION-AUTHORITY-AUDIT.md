# SIMD-4303-OBSERVATION Authority Audit — Differential Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

SIMD-4303-OBSERVATION is limited to test-local vocabulary for scalar/SIMD
differential comparison. It adds no differential runner, comparison schema,
tolerance policy, Fault mapper, target matrix, or public protocol. Public
SIMD-4303 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:210-219` is
  non-normative; SIMD differential behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel numeric/Fault semantics, Native IR/ABI, target capabilities, and
  backends.
- RFC-0013/RFC-H401 and the plan's differential dependencies are not Accepted
  authorities.

## Current implementation evidence

- No CPU/SIMD differential runner, comparison-result type, tolerance policy,
  Fault-location mapper, input corpus, target-feature matrix, or SIMD evidence
  exists in the workspace.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish verified scalar/SIMD inputs and artifacts;
integer/structural exactness; strict FP rounding/NaN/infinity/signed-zero/
overflow; relaxed FP tolerances/metrics; reduction/tail/unaligned/alignment/
determinism; Fault identity/spans and effect equivalence; unsupported
capability vs mismatch vs evidence failure; target features, canonical
outputs/traces, redaction/corruption/migration, fixtures, diagnostics,
CPU/device differential, and protocol contracts.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, Native backend,
memory category, ownership behavior, Kernel or Device Buffer surface,
diagnostics, schema, Semantic ID, source span, CLI, support claim, dependency
lock, target/toolchain, or Unicode 17.0.0 behavior. Differential comparison,
tolerance/Fault rules, target matrix, corpus, migration, protocol integration,
and support claims remain deferred.
