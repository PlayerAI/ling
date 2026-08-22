# DIR-4502-OBSERVATION Authority Audit — Kernel-to-Device Lowering Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DIR-4502-OBSERVATION is limited to test-local vocabulary for the proposed
verifier-preserving Kernel Core to Device IR lowering boundary. It adds no
Kernel verifier extension, lowerer, mapper, proof carrier, operation
partitioner, source diagnostic map, diagnostic, public protocol, or support
claim. Public DIR-4502 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:305-315` is
  non-normative; lowering behavior remains outside v0.0.1.
- RFC-H404 is absent; DIR-4501 is BlockedSpec and does not authorize a
  lowerer.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel types/effects, memory, synchronization, numeric, IR, ABI, target,
  capability, and backend behavior.

## Current implementation evidence

- No Kernel verifier, Typed Core Kernel node set, Device IR lowerer, operation
  partitioner, shape/index proof carrier, local-memory lowering,
  reduction/vector/tensor mapper, synchronization mapper, or source diagnostic
  map exists under crates or tests.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish Kernel source/Profile and Checked Core legal
subset; verifier proof witnesses; all lowering slices; memory/address-space,
shape/index/bounds, ownership/alias, effect/synchronization, numeric/reduction,
Fault/cancellation, capability/required-feature, target/fallback/rejection,
pre/postconditions, provenance, stable IDs/spans, resource limits, CPU
differential, canonical versioned mapping, diagnostics, fixtures,
corruption/migration, redaction, and protocol lifecycle.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, Native backend,
memory category, ownership behavior, Kernel or Device Buffer surface,
diagnostics, schema, Semantic ID, source span, CLI, support claim, dependency
lock, target/toolchain, or Unicode 17.0.0 behavior. Kernel verifier/lowering,
proof/provenance carriers, mappings, source maps, differential evidence,
migration, protocol integration, and support claims remain deferred.
