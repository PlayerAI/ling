# SIMD-4302-OBSERVATION Authority Audit — Portable SIMD IR Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

SIMD-4302-OBSERVATION is limited to test-local vocabulary for the proposed
portable SIMD IR. It adds no IR type, instruction schema, encoder/decoder,
verifier, capability registry, scalarization record, or public protocol. Public
SIMD-4302 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:198-208` is
  non-normative; Portable SIMD IR behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel legality, numeric determinism, Native IR/layout/ABI, target
  capabilities, and backends.
- RFC-0013/RFC-H401 and the plan's Native/SIMD dependencies are not Accepted
  authorities.

## Current implementation evidence

- No portable SIMD IR type, verifier, canonical serializer, decoder, lane/
  mask operation set, target capability registry, scalarization record, or
  SIMD corpus exists in the workspace.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish lane types/counts and vector values;
load/store addressing/alignment/bounds/alias; mask truth, shuffle indexes,
horizontal reduction/order; scalarization/fallback, effects/memory/Faults,
shape/layout/index/overflow, strict/relaxed FP and determinism; capability
identity/required/optional features and target rejection; canonical
schema/version/identity, fixtures, diagnostics, CPU/device differential, and
protocol contracts.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, Native backend,
memory category, ownership behavior, Kernel or Device Buffer surface,
diagnostics, schema, Semantic ID, source span, CLI, support claim, dependency
lock, target/toolchain, or Unicode 17.0.0 behavior. Portable SIMD IR,
lowering, differential evidence, migration, protocol integration, and support
claims remain deferred.
