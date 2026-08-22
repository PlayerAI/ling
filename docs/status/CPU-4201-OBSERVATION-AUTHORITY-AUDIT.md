# CPU-4201-OBSERVATION Authority Audit — Scalar Reference Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

CPU-4201-OBSERVATION is limited to test-local vocabulary for the proposed
scalar Kernel reference path. It adds no evaluator, backend, Device Buffer,
reduction semantics, Fault mapping, differential oracle, or public protocol.
Public CPU-4201 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:137-151` is
  non-normative; the scalar reference remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` remains Open for Kernel execution, ownership,
  numeric determinism, Faults, differential rules, and backends.
- RFC-0013/RFC-H401 are not Accepted authorities; scalar VM foundations do
  not define a Kernel execution oracle.

## Current implementation evidence

- No Kernel evaluator, scalar reference backend, Device Buffer, reduction
  implementation, Kernel Fault path, differential runner, or reference
  output protocol exists in the workspace.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish the verified input and work-item model;
map/index/conditional/loop/buffer and reduction semantics; shape/bounds,
alias/race, ownership/resource, numeric/tolerance, cancellation, Fault,
canonical output/trace, diagnostics, fixture, migration, CPU/device
differential, target rejection, and protocol contracts.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, memory category,
ownership behavior, Device Buffer, scheduler, diagnostics, schema, Semantic
ID, source span, CLI, support claim, dependency lock, target/toolchain, or
Unicode 17.0.0 behavior. Scalar Kernel execution, Faults, reductions,
differential evidence, migration, protocol integration, and support claims
remain deferred.
