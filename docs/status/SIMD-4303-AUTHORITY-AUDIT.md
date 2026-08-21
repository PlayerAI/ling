# SIMD-4303 Authority Audit — SIMD Differential

Status: BlockedSpec

Date: 2026-08-22

## Outcome

SIMD-4303 proposes differential comparison between a scalar CPU reference and
SIMD execution for exact integers, strict floating point, relaxed floating point
with declared tolerances, tail and unaligned access, overflow, and Fault
locations.

No differential harness or corpus can be added yet. The scalar reference,
Kernel numeric and Fault semantics, SIMD IR, legality results, target
capabilities, and comparison schema are not Accepted. A test that chooses its
own equality, tolerance, or Fault-location rules would become an unreviewed
language and backend contract.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:210-219 lists the
  comparison classes but does not define source fixtures, input generation,
  numeric formats, NaN and signed-zero policy, tolerance calculation, overflow
  behavior, tail or unaligned semantics, Fault identity, or artifact schema.
- docs/ROADMAP-1.0.md:381-429 requires CPU reference, SIMD, and stable device
  differential evidence at the G4 exit. Its G4.3 step is a roadmap gate, not an
  Accepted differential protocol.
- docs/SEMANTICS.md reserves Kernel and Native behavior for later releases and
  excludes both from v0.0.1. docs/LANGUAGE.md describes CPU SIMD as a future
  lowering target and does not define observable comparison behavior.
- GAP-KERNEL-DEVICE-001 leaves Kernel effects, buffers, numeric and reduction
  determinism, and backend capability unresolved. GAP-NATIVE-BACKEND-ABI-001
  leaves Native IR, layout, ABI, target packages, and backend support
  unresolved. Their candidate RFCs are not Accepted authorities.
- CPU-4201, CPU-4202, and CPU-4203 are BlockedSpec; SIMD-4301 and SIMD-4302
  are also BlockedSpec. There is no authoritative scalar baseline or portable
  SIMD artifact against which a differential result can be judged.
- Existing Seed/VM differential work under accepted RFC-0014, RFC-0018, and
  RFC-0019 does not define CPU/SIMD numeric comparison, and RFC-0001 lists
  Kernel and Native as future work.

## Current implementation evidence

- No CPU/SIMD differential runner, comparison-result type, tolerance policy,
  Fault-location mapper, input corpus, target-feature matrix, or SIMD evidence
  exists under crates or tests.
- No accepted rule fixes integer width and overflow, floating-point rounding,
  NaN and signed-zero behavior, strict versus relaxed modes, reduction order,
  tail and unaligned access, or exact/tolerance comparison boundaries.
- No contract defines whether a mismatch is a language error, backend error,
  unsupported-target result, or evidence failure; how committed effects and
  source spans are compared; or how results are canonicalized without host
  paths, driver text, timing, addresses, or allocation order.
- No differential protocol, schema, diagnostic allocation, dependency,
  target/toolchain selection, CLI command, or public support claim is required
  or changed by this audit. The public CLI and source extension remain ling
  and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A scalar CPU reference contract and a checked Typed Core input boundary,
   including supported types, effects, shapes/indexes, buffers, reductions,
   Faults, ownership, source spans, Semantic IDs, and deterministic execution.
2. A SIMD IR and legality contract defining lane operations, masks, loads and
   stores, shuffles, reductions, tails, unaligned access, target capabilities,
   scalar fallback, and declared determinism class.
3. Comparison semantics for integer and structural exactness; strict floating
   point including rounding, NaN, infinities, signed zero, and overflow; relaxed
   floating point with named tolerance and error metric; and permitted backend
   differences.
4. Fault and effect equivalence rules that preserve stable identity, original
   UTF-8 byte spans, committed effects, ordering, cancellation, resource
   limits, and the distinction between unsupported capability and mismatch.
5. A versioned differential schema and canonical runner contract for source and
   program identity, inputs, target features, capability evidence, outputs,
   Faults, traces, tolerances, redaction, corruption, migration, and
   deterministic ordering.
6. Bilingual L-<DOMAIN>-<NUMBER> diagnostics plus positive, negative,
   property, corruption, migration, Unicode/source-map, determinism,
   CPU-reference, fallback, overflow, floating-point, tail, unaligned, and
   cross-target fixtures executable offline.

## Evidence and compatibility impact

Differential evidence must compare verified artifacts only and must not promote
an implementation detail to Ling semantics. Exact and tolerance-based results
must be explicit, reproducible, and tied to the declared determinism class.
Fault locations must use original UTF-8 byte spans and stable Semantic IDs; host
addresses, instruction counts, driver versions, and debug text are not
observable language outputs.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

SIMD-4303 implementation, scalar/SIMD runner, comparison schema, tolerance and
Fault rules, target matrix, corpus, differential snapshots, Unicode/source-map
cases, editor integration, and public protocol claims remain deferred until
CPU-4201 through CPU-4203, SIMD-4301/4302, and the Kernel/device and
Native/backend gaps are resolved by Accepted authority and executable evidence.
