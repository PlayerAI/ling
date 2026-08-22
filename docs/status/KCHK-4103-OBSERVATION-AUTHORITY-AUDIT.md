# KCHK-4103-OBSERVATION Authority Audit — Kernel Shape/Index/Bounds Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

KCHK-4103-OBSERVATION is limited to test-local vocabulary for future Kernel
shape, index, and bounds validation. It does not add Kernel syntax, a shape
schema, verifier, Device Buffer API, or bounds semantics. Public KCHK-4103
remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:103-111` is
  non-normative and cannot define shape/layout/index/bounds semantics.
- `docs/SEMANTICS.md`/`docs/LANGUAGE.md` reserve Kernel and Device Buffer
  behavior outside v0.0.1. RFC-0013/RFC-H401 are not Accepted.
- `GAP-KERNEL-DEVICE-001` remains Open for shapes, ownership/address spaces,
  synchronization, numeric determinism, Placement, and backend discovery.

## Current implementation evidence

- The workspace has no Kernel shape/index/bounds checker, schema, Device Buffer
  API, verifier, or backend integration.
- The new test records sixty provisional boundary labels, explicit local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.
- No accepted rule defines shape/layout/index semantics, bounds/overflow,
  slicing/broadcasting, alias/race interaction, ownership, numeric mode,
  target/profile scope, or fallback.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. Shape/rank/extent/stride/layout and index arity/origin/normalization rules,
   including slicing, gather/scatter, broadcast, reshape, and transpose.
2. Bounds, lower/upper, negative-index, overflow, division, empty/zero,
   dynamic/static/symbolic shape and inference/proof rejection categories.
3. Buffer/address/ownership/device/profile/target and alias/race semantics,
   numeric determinism, provenance, diagnostics, canonical bytes, migrations,
   and protocol inventory.
4. A verifier consuming checked Typed Core or a verified derivative while
   preserving UTF-8 spans and Semantic IDs, plus CPU-reference and
   device-differential fixtures.

## Compatibility and intentionally deferred work

This audit changes no parser, Typed Core, evaluator, bytecode, VM, memory
category, ownership behavior, Device Buffer, scheduler, diagnostic registry,
schema, Semantic ID, source span, CLI, support claim, dependency lock,
target/toolchain, or Unicode 17.0.0 behavior. Shape/index/bounds semantics,
verifier, alias/race/numeric/device policy, CPU reference, diagnostics,
migration, protocol integration, and support claims remain deferred.
