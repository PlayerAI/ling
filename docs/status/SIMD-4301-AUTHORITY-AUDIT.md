# SIMD-4301 Authority Audit — Vectorization Legality Analysis

Status: BlockedSpec

Date: 2026-08-22

## Outcome

SIMD-4301 proposes a legality analysis for independent iterations, alignment,
vector width, tail handling, aliasing, reductions, strict or relaxed floating
point behavior, and target features. It also requires a recorded reason when a
scalar fallback is selected.

The analysis cannot be implemented as language or optimizer behavior yet. The
execution plan is a non-normative proposal and neither Kernel semantics nor the
Native/backend contract defines the inputs, proof obligations, vector IR,
fallback observables, or target capability model. Implementing it now would
make unresolved choices into de facto semantics.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:181-196 lists the
  SIMD work items and the fallback requirement, but does not define source
  syntax, Typed Core facts, legality proof witnesses, vector width policy,
  target feature identity, floating-point modes, or diagnostic behavior.
- docs/ROADMAP-1.0.md:381-429 makes CPU SIMD a G4.3 step after the Kernel
  subset and scalar CPU reference. It requires legal vectorization, scalar
  fallback, alignment, tail, overflow, floating-point, and determinism
  evidence; it is not an Accepted SIMD authority.
- docs/SEMANTICS.md reserves Kernel and Native behavior for later releases
  and excludes both from v0.0.1. docs/LANGUAGE.md describes SIMD as a future
  lowering target, not as a currently executable language feature.
- GAP-KERNEL-DEVICE-001 keeps Kernel types, effects, buffers, numeric and
  reduction determinism, and backend capability unresolved. GAP-NATIVE-
  BACKEND-ABI-001 keeps Native IR, layout, ABI, target packages, and backend
  support unresolved. Their candidate RFCs are not Accepted authorities.
- CPU-4201, CPU-4202, and CPU-4203 are recorded as BlockedSpec because the
  scalar reference, trace, and Kernel corpus contracts are absent. SIMD
  legality cannot be compared against an undefined CPU semantic baseline.
- RFC-0001 lists the Kernel and Native RFCs as future work; RFC-0014, RFC-0018,
  and RFC-0019 only cover the accepted Seed/VM slices and do not authorize
  vectorization or a Native execution oracle.

## Current implementation evidence

- No SIMD legality pass, vector IR, Kernel verifier, CPU reference backend,
  target-feature registry, fallback record, or SIMD conformance corpus exists
  under crates or tests.
- No accepted rule fixes the representation of independent iterations,
  shape/index and bounds facts, alias or ownership proof, alignment guarantee,
  legal vector widths, tail strategy, overflow behavior, reduction order,
  strict versus relaxed floating-point mode, or target feature negotiation.
- No accepted contract defines whether scalar fallback is observable, which
  reasons are stable evidence, how unsupported targets are rejected, or how
  optimizer diagnostics preserve Semantic IDs and original UTF-8 byte spans.
- No SIMD protocol, schema, diagnostic allocation, dependency, target/toolchain
  selection, CLI command, or public support claim is required or changed by
  this audit. The public CLI and source extension remain ling and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A Kernel and Typed Core legality contract covering iteration independence,
   effects, shapes and indexes, bounds, buffer ownership, aliasing, mutation,
   reductions, numeric modes, and source-span/Semantic-ID witnesses.
2. A versioned SIMD legality result with canonical proof facts for alignment,
   vector width, tail handling, overflow, reduction order, strict or relaxed
   floating point, target features, and a deterministic scalar fallback reason.
3. A scalar CPU reference and differential contract defining exact versus
   tolerance comparison, permitted floating-point differences, determinism
   classes, unsupported-feature rejection, and the conditions under which
   fallback is semantically equivalent.
4. A Native/backend-neutral IR and ABI authority for vector operations, target
   capability discovery, layout, calling conventions, feature identity, and
   reproducible lowering. Backend failure must not replace a language-level
   legality diagnostic.
5. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics and structured facts for
   dependence or alias conflicts, invalid bounds, unsupported widths or
   features, alignment failures, overflow, reduction nondeterminism, malformed
   evidence, and resource limits.
6. Positive, negative, property, corruption, migration, Unicode/source-map,
   determinism, CPU-reference, fallback, and cross-target fixtures with
   offline reproducibility and no host paths, timing, or driver details.

## Evidence and compatibility impact

The future implementation must consume checked Typed Core or a verified
derivative only; it must never interpret unchecked AST nodes. Legality facts
must preserve original UTF-8 byte spans, Semantic IDs, Unicode 17.0.0 behavior,
canonical ordering, and deterministic serialization. Exact and tolerance-based
comparisons must be explicit, and scalar fallback must not silently change
declared effects, numeric semantics, or determinism.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

SIMD-4301 implementation, legality proof data structures, vector IR, target
feature discovery, scalar fallback records, CPU/SIMD differential tests,
Unicode/source-map cases, editor integration, and public protocol claims remain
deferred until the Kernel/device and Native/backend gaps are resolved by Accepted
authority and the required executable evidence exists.
