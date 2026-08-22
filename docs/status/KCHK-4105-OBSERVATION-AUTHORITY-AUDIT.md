# KCHK-4105-OBSERVATION Authority Audit — Kernel Core/Verifier Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

KCHK-4105-OBSERVATION is limited to test-local vocabulary for the proposed
versioned, device-independent Kernel Core and independent verifier. It adds
no Core schema, verifier proof, decoder, backend admission rule, or public
protocol. Public KCHK-4105 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:124-136` is
  non-normative; Kernel Core and verifier rules remain outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` remains Open for the Kernel subset, verifier trust
  boundary, identity, determinism, Device IR, and backends.
- RFC-0013/RFC-H401 are not Accepted authorities; scalar VM verifier RFCs do
  not authorize Kernel artifacts.

## Current implementation evidence

- No Kernel Core schema, encoder/decoder, independent verifier, Device IR,
  source-map bridge, canonical Kernel serializer, or backend admission hook
  exists in the workspace.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish the Core node grammar and legal types/control
flow; the checked Typed Core/verified-derivative trust boundary; witness and
identity rules; effect/capability, shape/bounds, alias/race,
ownership/resource, profile/target/device, and determinism semantics; bounded
decode/verify behavior; canonical bytes/source maps/migration/resource limits;
diagnostics; CPU/device differential evidence; and protocol inventory.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, memory category,
ownership behavior, Device Buffer, scheduler, diagnostics, schema, Semantic
ID, source span, CLI, support claim, dependency lock, target/toolchain, or
Unicode 17.0.0 behavior. Kernel Core/verifier semantics, schemas, diagnostics,
CPU/device evidence, migration, protocol integration, and support claims
remain deferred.
