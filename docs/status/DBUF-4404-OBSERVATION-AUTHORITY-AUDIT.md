# DBUF-4404-OBSERVATION Authority Audit — Device Synchronization Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DBUF-4404-OBSERVATION is limited to test-local vocabulary for the proposed
Device synchronization boundary. It adds no queue/event/fence type, barrier or
hazard checker, host-await or cancellation runtime, device-loss handler,
diagnostic, public protocol, or support claim. Public DBUF-4404 remains
`BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:272-283` is
  non-normative; Device synchronization behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-OWNERSHIP-MODEL-001` remain Open for
  queue ownership, visibility/order, hazards, cleanup, determinism, Faults,
  and backends.
- Accepted Seed/VM runtime decisions do not authorize device queues or
  synchronization; RFC-0013/RFC-H401 and the plan's dependencies are not
  Accepted synchronization authorities.

## Current implementation evidence

- No command queue, event/fence, host-await, barrier, hazard checker,
  cross-queue ordering graph, cancellation path, device-loss Fault mapper, or
  synchronization corpus exists under crates or tests.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish checked queue/event/fence/barrier/await
state, queue ownership, cross-queue happens-before, memory visibility and
acquire/release ordering, hazard classes and proofs, view/transfer dependency,
cancellation/timeout/drop/cleanup, device loss/Fault, resource limits,
committed effects, stable Semantic IDs and UTF-8 spans, bilingual diagnostics,
canonical versioned synchronization/evidence fields, fixtures, migration,
redaction, and protocol lifecycle.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, Native backend,
memory category, ownership behavior, Kernel or Device Buffer surface,
diagnostics, schema, Semantic ID, source span, CLI, support claim, dependency
lock, target/toolchain, or Unicode 17.0.0 behavior. Queue/event/fence types,
barriers, hazards, ordering, await/cancellation, device-loss handling,
synchronization evidence, migration, protocol integration, and support claims
remain deferred.
