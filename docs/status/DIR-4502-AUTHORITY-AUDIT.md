# DIR-4502 Authority Audit — Kernel Core to Device IR

Status: BlockedSpec

Date: 2026-08-22

## Outcome

DIR-4502 proposes a verifier-preserving lowering from Kernel Core to Device IR,
partitioned into elementwise operations, index and shape handling, local
memory, reductions, vector/tensor operations, synchronization, and source
diagnostic mapping.

No lowering pass can be implemented yet. The Kernel source and checked Core
subset, Device IR schema, ownership and synchronization rules, numeric
determinism, and target capability contracts are not Accepted. A partial
lowerer would have to invent which programs are legal, how proofs are carried,
and how backend failures become diagnostics.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:305-315 lists the
  lowering slices and says to preserve the verifier, but does not define
  source/Typed Core nodes, proof witnesses, mapping invariants, rejected
  constructs, effect/ownership transfer, numeric modes, or target feature
  requirements.
- docs/ROADMAP-1.0.md:381-429 requires a Kernel verifier before lowering,
  scalar CPU reference semantics, a backend-neutral Device IR, and differential
  evidence. It is a roadmap gate, not an Accepted lowering specification.
- docs/SEMANTICS.md reserves Kernel and Device lowering for later releases and
  excludes them from v0.0.1. docs/LANGUAGE.md does not authorize a Kernel
  compiler path or Device IR implementation.
- RFC-H404 is absent; DIR-4501 is BlockedSpec. GAP-KERNEL-DEVICE-001 leaves
  Kernel types/effects, buffers, synchronization, numeric determinism,
  Placement, and backend capability unresolved. GAP-NATIVE-BACKEND-ABI-001
  leaves Native/backend IR and target support unresolved.
- DBUF-4401 through DBUF-4404 and SIMD-4301 through SIMD-4303 are BlockedSpec,
  so the proposed lowerer has no accepted memory, numeric, capability,
  synchronization, or differential semantics to preserve.

## Current implementation evidence

- No Kernel verifier, Typed Core Kernel node set, Device IR lowerer, operation
  partitioner, shape/index proof carrier, local-memory lowering, reduction or
  vector/tensor mapper, synchronization mapper, or source diagnostic map exists
  under crates or tests.
- No accepted rule fixes which elementwise/index/shape/local-memory/reduction/
  vector/tensor/synchronization constructs are legal; how bounds and alias
  proofs are represented; or how ownership, effects, Faults, and determinism
  survive lowering.
- No contract defines rejection before backend compilation, stable provenance
  for a lowerer diagnostic, source-map byte spans, Semantic IDs, target
  capability checks, fallback behavior, or migration when Device IR changes.
- No lowering protocol, schema, diagnostic allocation, dependency,
  target/toolchain selection, CLI command, or public support claim is required
  or changed by this audit. The public CLI and source extension remain ling
  and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A Kernel source and checked Typed Core contract with legal types, effects,
   control flow, shapes/indexes, buffers, ownership, reductions, vector/tensor
   operations, synchronization, Faults, determinism, and source-span/Semantic
   ID witnesses.
2. A versioned Kernel-to-Device IR mapping for each lowering slice, including
   preconditions, proof preservation, ownership/effect transitions, numeric
   and reduction semantics, required capabilities, and explicit rejection.
3. A Device IR schema and verifier defining operations, address spaces,
   memory/order, barriers/atomics, shape/layout, numeric modes, source maps,
   required features, stable identity, malformed input, and migration.
4. A target capability and fallback contract that distinguishes unsupported
   targets from invalid Kernel programs, records deterministic reasons, and
   never replaces language-level diagnostics with backend compile failures.
5. Differential rules against the CPU reference for exact and tolerance-based
   results, Fault identity and location, committed effects, determinism class,
   resource limits, cancellation, and allowed backend differences.
6. Bilingual L-<DOMAIN>-<NUMBER> diagnostics plus positive, negative,
   property, corruption, migration, Unicode/source-map, determinism, bounds,
   ownership, reduction, synchronization, CPU-reference, and cross-target
   fixtures executable offline.

## Evidence and compatibility impact

The eventual lowerer must consume checked Typed Core or a verified derivative
only and must never interpret unchecked AST nodes. It must preserve original
UTF-8 byte spans, Semantic IDs, Unicode 17.0.0 behavior, declared effects,
ownership and Fault provenance, and deterministic ordering. Host pointers,
driver output, target paths, timing, and debug text must not become Ling
semantics or stable artifacts.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

DIR-4502 implementation, Kernel verifier extensions, lowering mappings,
proof/provenance carriers, shape/index and memory lowering, reductions,
vector/tensor and synchronization lowering, source maps, differential corpus,
Unicode/source-map cases, editor integration, and public protocol claims remain
deferred until DIR-4501, the Kernel/device and Native/backend gaps, and the
required Accepted authority and executable evidence exist.
