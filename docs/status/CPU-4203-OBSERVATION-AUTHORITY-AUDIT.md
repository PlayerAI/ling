# CPU-4203-OBSERVATION Authority Audit — Kernel Corpus Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

CPU-4203-OBSERVATION is limited to test-local vocabulary for the proposed
Kernel corpus. It adds no source fixture, manifest, expected output, Fault
snapshot, differential runner, or public protocol. Public CPU-4203 remains
`BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:165-180` is
  non-normative; Kernel corpus behavior remains outside v0.0.1.
- `GAP-KERNEL-DEVICE-001` remains Open for Kernel syntax, execution, expected
  results, numeric determinism, differential rules, and backends.
- RFC-0013/RFC-H401 are not Accepted authorities; Seed conformance fixtures do
  not authorize Kernel fixtures.

## Current implementation evidence

- No Kernel source fixture, corpus manifest, CPU reference runner, trace
  snapshot, Device Buffer case, reduction/atomic case, or Kernel source-map
  corpus exists in the workspace.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish manifest/fixture identity and versioning;
source bytes and `.ling` mapping; profiles/targets, inputs/outputs/Faults/
traces; vector/matrix/filter/reduction/histogram/atomic cases;
bounds/alias/numeric/determinism; positive/negative/property/corruption/
migration/Unicode/source-map fixtures; exact/tolerance comparison;
CPU/device differential; diagnostics, host exclusion, and protocol contracts.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, memory category,
ownership behavior, Device Buffer, scheduler, diagnostics, schema, Semantic
ID, source span, CLI, support claim, dependency lock, target/toolchain, or
Unicode 17.0.0 behavior. Kernel fixtures, manifests, expected outputs,
Fault/trace snapshots, differential evidence, migration, protocol integration,
and support claims remain deferred.
