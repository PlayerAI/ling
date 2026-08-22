# KCHK-4101-OBSERVATION Authority Audit — Kernel Capability-Matrix Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

KCHK-4101-OBSERVATION is limited to test-local vocabulary for a future Kernel
capability matrix. It does not define Kernel syntax, a checker, a schema,
Graph/Audit fields, a CPU reference, or a backend capability API. Public
KCHK-4101 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:69-83` is
  non-normative; its table shape and examples cannot define Kernel meaning,
  schema, diagnostics, or compatibility.
- `docs/ROADMAP-1.0.md:381-429` places Kernel/device work in G4 but is not an
  Accepted semantic authority. `docs/SEMANTICS.md` and `docs/LANGUAGE.md`
  reserve Kernel and exclude it from v0.0.1.
- RFC-0001 lists RFC-0013 as future work; RFC-0013/RFC-H401 are not Accepted
  documents. `GAP-KERNEL-DEVICE-001` remains Open and blocks KCHK-4101 and
  related device tasks.
- Existing support entries mark Kernel CPU/GPU/accelerator surfaces
  Unsupported or Experimental. No Kernel protocol is registered in
  `docs/governance/protocol-inventory.toml`.

## Current implementation evidence

- The repository has no Kernel, Device Buffer, Placement, or capability
  checker implementation under crates or tests.
- The new test records sixty provisional boundary labels, explicit local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.
- No accepted rule defines Typed Core admission, value/layout/ADT limits,
  allocation/recursion/loop/call policy, Effect/Capability rows, buffer
  ownership/address spaces, alias/race proofs, numeric/reduction determinism,
  target discovery, or fallback.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. RFC-0013 or an accepted replacement resolving RFC-H401 and defining Kernel
   profiles, Typed Core/Native/Device IR relationships, and CPU reference
   execution.
2. A canonical, versioned matrix schema with stable capability IDs,
   conditions/rejection categories, profile/target scope, provenance,
   Graph/Audit projection, canonical bytes, migrations, and deterministic
   ordering without host paths, addresses, driver logs, or allocation order.
3. Normative value/layout/ADT, Managed/Resource, allocation, recursion/loop,
   calls/dispatch, Effect/Capability, Device/Buffer/address-space, alias/race,
   bounds/overflow, synchronization, and numeric/reduction rules.
4. A verifier consuming checked Typed Core or a verified derivative, preserving
   UTF-8 spans and Semantic IDs and rejecting unsupported constructs before
   backend compilation.
5. Stable bilingual diagnostics and CPU-reference plus positive/negative,
   migration, Unicode, determinism, and device-differential fixtures.

## Compatibility and intentionally deferred work

This audit changes no parser, Typed Core, evaluator, bytecode, VM, Native
backend, memory category, ownership behavior, Device Buffer, scheduler,
diagnostic registry, schema, Semantic ID, source span, CLI, support claim,
dependency lock, target/toolchain, or Unicode 17.0.0 behavior. Matrix/schema,
checker, Graph/Audit projection, diagnostics, CPU reference, Device IR/backends,
Placement, editor support, and all device capability claims remain deferred.
