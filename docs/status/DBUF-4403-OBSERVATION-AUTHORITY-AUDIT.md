# DBUF-4403-OBSERVATION Authority Audit — Transfer Effect Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DBUF-4403-OBSERVATION is limited to test-local vocabulary for the proposed
Transfer Effect boundary. It adds no transfer expression, Effect Row or
DeviceTransfer capability API, address-space model, lifecycle runtime, cost
reporter, Fault mapper, diagnostic, public protocol, or support claim. Public
DBUF-4403 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:252-270` is
  non-normative; Transfer Effect behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-OWNERSHIP-MODEL-001` remain Open for
  Device/capability discovery, address spaces, ownership transitions,
  synchronization, Faults, determinism, and backends.
- Existing Seed Effect/Fault decisions do not authorize DeviceTransfer or
  device execution; RFC-0013/RFC-H401 and the plan's dependencies are not
  Accepted transfer authorities.

## Current implementation evidence

- No transfer expression, Transfer Effect, DeviceTransfer capability,
  address-space type, byte-count checker, synchronization token, transfer
  Fault mapper, or Semantic Graph/Audit transfer record exists under crates or
  tests.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish typed transfer syntax and result/effect
rows; DeviceTransfer capability identity and discovery; direction, bytes,
address spaces, shape/layout/alignment/bounds; ownership transitions;
TransferToken and synchronization; visibility/coherence; asynchronous
completion, cancellation, timeout, device loss, Fault, drop, resource limits,
and committed effects. They must also define canonical Semantic Graph/Audit
fields, cost/evidence status, redaction, stable Semantic IDs and UTF-8 spans,
bilingual diagnostics, version/migration, and executable fixtures.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, Native backend,
memory category, ownership behavior, Kernel or Device Buffer surface,
diagnostics, schema, Semantic ID, source span, CLI, support claim, dependency
lock, target/toolchain, or Unicode 17.0.0 behavior. Transfer syntax/effects,
capability discovery, address spaces, ownership transitions, lifecycle,
cost/Fault evidence, migration, protocol integration, and support claims remain
deferred.
