# DIR-4501-OBSERVATION Authority Audit — Device IR Schema Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DIR-4501-OBSERVATION is limited to test-local vocabulary for the proposed
backend-neutral Device IR schema boundary. It adds no IR type model, schema,
encoder/decoder, validator, canonicalizer, source-map carrier, capability
registry, operation verifier, diagnostic, public protocol, or support claim.
Public DIR-4501 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:285-304` is
  non-normative and explicitly depends on RFC-H404; Device IR behavior remains
  outside v0.0.1.
- RFC-H404 is absent from docs and governance registries and is not an
  Accepted authority.
- `GAP-KERNEL-DEVICE-001` and `GAP-NATIVE-BACKEND-ABI-001` remain Open for
  Kernel/device memory, synchronization, numeric, IR, ABI, layout, target,
  capability, and backend behavior.

## Current implementation evidence

- No Device IR type model, schema, encoder/decoder, validator, canonicalizer,
  source-map carrier, capability registry, operation verifier, or corpus exists
  under crates or tests.
- The new test records sixty provisional labels, deterministic local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.

## Required authority before implementation

Accepted decisions must establish checked Typed Core to Device IR mapping;
workgroup/grid, scalar/vector/tensor, address-space, memory/control-flow,
barrier/atomic, shape/layout, numeric, source-map, capability/required-feature,
Fault/cancellation, ownership/synchronization, target negotiation, canonical
versioned encoding, stable IDs/spans, corruption/migration/redaction,
bilingual diagnostics, fixtures, and Internal/Preview/Stable lifecycle status.

## Compatibility and intentionally deferred work

This audit changes no Seed compiler/evaluator, bytecode, VM, Native backend,
memory category, ownership behavior, Kernel or Device Buffer surface,
diagnostics, schema, Semantic ID, source span, CLI, support claim, dependency
lock, target/toolchain, or Unicode 17.0.0 behavior. Device IR types/operations,
schema/codec/validator, canonicalization, source maps, capability negotiation,
backend integration, migration, protocol integration, and support claims remain
deferred.
