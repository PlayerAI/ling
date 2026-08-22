# SIMD-4301-OBSERVATION Authority Audit — Legality Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

SIMD-4301-OBSERVATION is limited to test-local vocabulary for proposed SIMD
legality and scalar-fallback reasoning. It adds no legality pass, vector IR,
target-feature negotiation, fallback behavior, or public protocol. Public
SIMD-4301 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:181-196` is
  non-normative; SIMD behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel legality, numeric determinism, target capabilities, Native IR/ABI,
  and backends.
- RFC-0013/RFC-H401 and the plan's SIMD/Native dependencies are not Accepted
  authorities.

## Current implementation evidence

- No SIMD legality pass, vector IR, Kernel verifier, CPU reference backend,
  target-feature registry, fallback record, or SIMD corpus exists in the
  workspace.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish iteration/dependence and
effect/shape/bounds/alias/ownership proofs; alignment/width/tail/overflow;
strict/relaxed FP and reduction order; target-feature identity/negotiation;
fallback permission/reason/equivalence; canonical proof facts, provenance,
fixtures, diagnostics, CPU/SIMD differential, Native IR/ABI, target backends,
and protocol contracts.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, Native backend,
memory category, ownership behavior, Kernel or Device Buffer surface,
diagnostics, schema, Semantic ID, source span, CLI, support claim, dependency
lock, target/toolchain, or Unicode 17.0.0 behavior. SIMD legality, vector IR,
fallback, differential evidence, migration, protocol integration, and support
claims remain deferred.
