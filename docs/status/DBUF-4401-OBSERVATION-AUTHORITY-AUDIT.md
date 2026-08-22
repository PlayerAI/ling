# DBUF-4401-OBSERVATION Authority Audit — Device Capability Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DBUF-4401-OBSERVATION is limited to test-local vocabulary for the proposed
Device and capability boundary. It adds no Device/Buffer type, capability
registry, view/token API, Fence/Event runtime, raw-pointer interface, or public
protocol. Public DBUF-4401 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:221-238` is
  non-normative; Device and capability behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` and `GAP-OWNERSHIP-MODEL-001` remain Open for
  address spaces, buffers, synchronization, ownership, determinism,
  capability discovery, and backends.
- RFC-0013/RFC-H401 and the plan's Device dependencies are not Accepted
  authorities.

## Current implementation evidence

- No DeviceId, DeviceKind, DeviceCapability, AddressSpace, Buffer, ReadView,
  WriteView, TransferToken, Fence/Event type, verifier, capability registry,
  or Device Buffer corpus exists in the workspace.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish Device identity/kind/capability/version/set;
address spaces; Buffer element/shape/layout/identity; views, tokens,
Fence/Event; raw-pointer prohibition; Typed Core/profiles/targets/effects/
capabilities; ownership/alias/bounds/resources; transfer direction/order,
async lifetime, cancellation/drop, device loss/Faults; canonical
schema/version/migration, fixtures, CPU/device differential, diagnostics, host
exclusion, and protocol contracts.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, Native backend,
memory category, ownership behavior, Kernel or Device Buffer surface,
diagnostics, schema, Semantic ID, source span, CLI, support claim, dependency
lock, target/toolchain, or Unicode 17.0.0 behavior. Device types/capabilities,
Buffer/views/tokens, synchronization, transfer evidence, migration, protocol
integration, and support claims remain deferred.
