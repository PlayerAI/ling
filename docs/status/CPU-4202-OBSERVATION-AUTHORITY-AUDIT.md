# CPU-4202-OBSERVATION Authority Audit — Reference Trace Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

CPU-4202-OBSERVATION is limited to test-local vocabulary for the proposed
scalar Kernel reference trace. It adds no trace producer, event schema,
serializer, CLI flag, runtime hook, or public protocol. Public CPU-4202 remains
`BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:153-164` is
  non-normative and explicitly limits trace output to test/explanation use.
- `GAP-KERNEL-DEVICE-001` remains Open for Kernel execution, event ordering,
  determinism, Faults, differential rules, and backends.
- RFC-0013/RFC-H401 are not Accepted authorities; scalar VM tracing does not
  define a Kernel reference trace.

## Current implementation evidence

- No Kernel scalar backend, trace event type, trace serializer, test-mode
  switch, or reference corpus exists in the workspace.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish event identity/order/payload and observation
points; work-item/index and buffer/view identity; operation/reduction/
atomic/barrier/Fault semantics; provenance, numeric/determinism, sampling,
event/byte limits, truncation, redaction and sensitive-data exclusion;
canonicalization, corruption, migration, fixtures, CPU/device differential,
diagnostics, and protocol contracts.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, memory category,
ownership behavior, Device Buffer, scheduler, diagnostics, schema, Semantic
ID, source span, CLI, support claim, dependency lock, target/toolchain, or
Unicode 17.0.0 behavior. Trace events/serialization, test-mode CLI,
redaction/limits, differential evidence, migration, protocol integration, and
support claims remain deferred.
