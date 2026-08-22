# KCHK-4102-OBSERVATION Authority Audit — Kernel Effect/Capability Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

KCHK-4102-OBSERVATION is limited to test-local vocabulary for future Kernel
Effect and Capability checks. It does not alter the accepted Seed checker or
add Kernel admission behavior. Public KCHK-4102 remains `BlockedSpec`.

## Normative traceability

- Accepted RFC-0018 governs Seed Effect closure and Capability preflight only;
  it does not define Kernel admission, Device effects, profile/target rows, or
  a public Kernel checker.
- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:90-99` is
  non-normative, and Kernel remains reserved outside v0.0.1 under
  `docs/SEMANTICS.md`/`docs/LANGUAGE.md`.
- RFC-0013/RFC-H401 are not Accepted and `GAP-KERNEL-DEVICE-001` remains Open;
  no Kernel Effect/Capability protocol is registered.

## Current implementation evidence

- The workspace has no Kernel Effect/Capability checker, admission schema,
  profile/target row, Device Buffer API, or backend integration.
- The new test records sixty provisional boundary labels, explicit local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.
- Existing Seed Effect/Capability behavior remains the sole accepted semantic
  path; no Kernel-specific row, mismatch, diagnostic, or support claim exists.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. Kernel Effect/Capability schemas and closure rules over checked Typed Core,
   including allowed/forbidden IO, Network, Task, Actor, Device, Resource,
   Managed, allocation, mutation, call, recursion, and trait effects.
2. Profile/target scope, mismatch/rejection categories, diagnostic facts,
   source/Semantic-ID provenance, Graph/Audit projection, canonical ordering,
   migrations, and public protocol inventory.
3. A verifier boundary that consumes checked Typed Core or a verified
   derivative, preserves UTF-8 spans/Semantic IDs, and rejects unsupported
   constructs before backend compilation.
4. CPU reference, numeric/determinism/fallback, Device IR/backend, and
   positive/negative/cross-module/package/migration evidence.

## Compatibility and intentionally deferred work

This audit changes no Seed Effect/Capability checker, compiler, evaluator,
bytecode, VM, memory category, ownership behavior, Device Buffer, scheduler,
diagnostic registry, schema, Semantic ID, source span, CLI, support claim,
dependency lock, target/toolchain, or Unicode 17.0.0 behavior. Kernel-specific
rows/checking/admission, diagnostics, CPU reference, Device IR/backends,
migration, protocol integration, and support claims remain deferred.
