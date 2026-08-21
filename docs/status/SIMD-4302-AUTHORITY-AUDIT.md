# SIMD-4302 Authority Audit — Portable SIMD IR

Status: BlockedSpec

Date: 2026-08-22

## Outcome

SIMD-4302 proposes a portable intermediate representation for lanes, vector
loads and stores, masks, shuffles, horizontal reductions, scalarization
fallback, and target capability. The proposal places this representation in
Native or Device IR.

No such IR can be added yet. The language has no Accepted Kernel contract,
Native/backend-neutral IR contract, vector operation semantics, or target
capability protocol. Defining a schema or public API now would freeze layout,
numeric, effect, and fallback choices that are explicitly unresolved.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:198-208 lists the
  Portable SIMD IR fields but does not define lane element types, shape/layout,
  memory effects, mask truth values, shuffle indexing, reduction order, Faults,
  fallback observability, capability identity, or version compatibility.
- docs/ROADMAP-1.0.md:381-429 places SIMD after the Kernel subset and scalar CPU
  reference. The roadmap requires a backend-neutral device IR or an explicitly
  reused intermediate layer, but it does not itself authorize a public IR
  schema or implementation.
- docs/SEMANTICS.md reserves Kernel and Native behavior for later releases and
  excludes both from v0.0.1. docs/LANGUAGE.md describes SIMD as a future
  lowering target, not as a currently executable or stable IR.
- GAP-KERNEL-DEVICE-001 leaves Kernel types, effects, buffers, numeric and
  reduction determinism, and backend capability unresolved. GAP-NATIVE-
  BACKEND-ABI-001 leaves Native IR validity, layout, ABI, target packages, and
  backend support unresolved. Their candidate RFCs are not Accepted.
- SIMD-4301 is BlockedSpec because legality facts, scalar fallback, CPU
  reference, and target-feature rules are absent. SIMD-4302 cannot represent
  proof results or lowerings against an undefined legality contract.
- RFC-0001 lists Native and Kernel RFCs as future work; RFC-0014, RFC-0018, and
  RFC-0019 cover only accepted Seed/VM slices and do not authorize SIMD IR,
  Device IR, or a Native execution oracle.

## Current implementation evidence

- No portable SIMD IR data type, verifier, canonical serializer, decoder,
  lane/mask operation set, target capability registry, scalarization record, or
  SIMD conformance corpus exists under crates or tests.
- No accepted rule fixes vector element types, lane counts, alignment,
  load/store bounds and aliasing, mask representation, shuffle semantics,
  horizontal reduction ordering, overflow or floating-point modes, or the
  distinction between exact and tolerance-based results.
- No contract defines whether the IR is internal, Preview, or Stable; how
  Semantic IDs and original UTF-8 byte spans are carried; how malformed or
  unsupported instructions are rejected; or how target features are named,
  versioned, and compared deterministically.
- No SIMD protocol, schema, diagnostic allocation, dependency, target/toolchain
  selection, CLI command, or public support claim is required or changed by
  this audit. The public CLI and source extension remain ling and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A checked Kernel-to-IR boundary that accepts only verified Typed Core or a
   verified derivative and defines the legal source operations, effects,
   ownership, bounds, aliases, and Semantic-ID/source-span witnesses.
2. A versioned, backend-neutral SIMD IR contract covering lane types/counts,
   vector load/store addressing and alignment, masks, shuffle indexes,
   horizontal reductions, scalarization fallback, Faults, side effects,
   determinism class, and target capability requirements.
3. A canonical representation and schema lifecycle: field presence, ordering,
   numeric encoding, stable identities, source mapping, malformed-input
   rejection, compatibility policy, migration, and corruption behavior.
4. A capability and lowering contract defining feature-set identity, required
   versus optional features, unsupported-target rejection, fallback conditions,
   backend lowering obligations, and reproducible artifact identity without
   exposing host paths, driver text, allocation order, or debug output.
5. Scalar CPU reference and differential rules for integer exactness, strict
   floating-point semantics, relaxed tolerances, tails, unaligned access,
   overflow, reductions, Fault locations, and allowed backend differences.
6. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics and positive, negative,
   property, corruption, migration, Unicode/source-map, determinism,
   CPU-reference, fallback, and cross-target fixtures.

## Evidence and compatibility impact

The eventual IR must preserve original UTF-8 byte spans, Semantic IDs, Unicode
17.0.0 behavior, deterministic ordering, and declared effects. It must not
silently reinterpret a mask, shuffle, reduction, memory access, or fallback
when a target lacks a capability. Internal evidence must not be promoted to a
public protocol without inventory registration, an Accepted authority, and
executable compatibility fixtures.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

SIMD-4302 implementation, vector IR types and operations, canonical schema,
serializer/decoder, verifier, capability registry, scalarization records,
CPU/SIMD differential fixtures, Unicode/source-map cases, editor integration,
and public protocol claims remain deferred until SIMD-4301, the Kernel/device
gap, and the Native/backend gap are resolved by Accepted authority and the
required executable evidence exists.
