# DBUF-4402-OBSERVATION Authority Audit — Buffer Ownership Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DBUF-4402-OBSERVATION is limited to test-local vocabulary for the proposed
Buffer ownership boundary. It adds no ownership checker, Buffer/view type,
mapping or pinning runtime, transfer-lifetime state machine, cancellation/drop
behavior, task/actor crossing protocol, or public support claim. Public
DBUF-4402 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:240-250` is
  non-normative; Buffer ownership behavior remains outside v0.0.1.
- `GAP-OWNERSHIP-MODEL-001` and `GAP-KERNEL-DEVICE-001` remain Open for
  ownership calculus, address spaces, transfer, synchronization, cleanup,
  determinism, and backends.
- RFC-0013/RFC-H401 and the plan's ownership dependencies are not Accepted
  authorities.

## Current implementation evidence

- No Device Buffer ownership checker, state machine, borrow/view type,
  subview proof, mapping or pinning operation, asynchronous transfer lifetime
  model, drop/cancel implementation, or actor/task crossing test exists under
  crates or tests.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish an ownership calculus for Value/Managed/
Resource/Buffer, Copy/Move, shared-read and exclusive-write borrows, alias and
subview proofs, region escape, drop order, Profile restrictions, and Checked
Core witnesses. They must also define Device address-space ownership,
mapping/pinning, visibility/coherence, transfer completion, cancellation,
device-loss/Fault cleanup, task/actor crossing, stable Semantic IDs and source
spans, bilingual diagnostics, canonical versioned schemas, fixtures, migration,
and protocol boundaries.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, Native backend,
memory category, ownership behavior, Kernel or Device Buffer surface,
diagnostics, schema, Semantic ID, source span, CLI, support claim, dependency
lock, target/toolchain, or Unicode 17.0.0 behavior. Ownership types/checks,
views, mapping/pinning, transfer lifecycle, cancellation/drop, crossing
evidence, migration, protocol integration, and support claims remain deferred.
