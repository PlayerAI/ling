# DIR-4501 Authority Audit — Device IR Schema

Status: BlockedSpec

Date: 2026-08-22

## Outcome

DIR-4501 proposes a backend-neutral Device IR with workgroup/grid structure,
scalar/vector/tensor types, address spaces, memory operations, control flow,
barriers/atomics, shape/layout, numeric mode, source maps, capabilities, and
required backend features. The execution plan marks it dependent on RFC-H404.

No Device IR schema can be added yet. RFC-H404 is only a plan reference and
has no Accepted document, lifecycle record, schema, verifier, or fixtures.
Kernel/device semantics, synchronization, ownership, numeric determinism, and
backend capability are also unresolved. Creating a schema now would freeze
hardware-facing language and protocol choices without authority.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:285-304 lists the
  proposed fields and declares dependency on RFC-H404, but does not define
  types, operation semantics, address-space rules, shape/layout invariants,
  barrier/atomic memory order, numeric modes, source-map identity, capability
  negotiation, error behavior, or schema compatibility.
- docs/ROADMAP-1.0.md:381-429 requires a backend-neutral Device IR or an
  explicitly reused intermediate layer, verified Kernel lowering, differential
  evidence, and unsupported-target rejection. It is a roadmap gate, not an
  Accepted Device IR schema.
- docs/SEMANTICS.md reserves Kernel, Device Buffer, synchronization, numeric
  determinism, and backend lowering for later releases; v0.0.1 excludes these
  surfaces. docs/LANGUAGE.md does not authorize a public Device IR.
- RFC-H404 is absent from docs and governance lifecycle/authority registries.
  GAP-KERNEL-DEVICE-001 leaves Kernel operations, buffers, synchronization,
  numeric determinism, Placement, and backend capability unresolved.
  GAP-NATIVE-BACKEND-ABI-001 leaves Native IR, ABI, layout, target packages,
  and backend support unresolved.
- DBUF-4401 through DBUF-4404 and SIMD-4301 through SIMD-4303 are BlockedSpec,
  so the proposed Device IR has no verified source, memory, capability, or
  differential semantics to encode.

## Current implementation evidence

- No Device IR type model, schema, encoder/decoder, validator, canonicalizer,
  source-map carrier, capability registry, operation verifier, or corpus exists
  under crates or tests.
- No accepted rule fixes workgroup/grid identity, scalar/vector/tensor lanes,
  memory operation bounds and effects, address spaces, control flow, barriers,
  atomics, shape/layout, numeric or determinism modes, or required feature sets.
- No contract defines schema versioning, stable Semantic IDs, original UTF-8
  byte spans, malformed input handling, canonical ordering, migration,
  corruption behavior, or whether the IR is internal, Preview, or Stable.
- No Device IR protocol, diagnostic allocation, dependency, target/toolchain
  selection, CLI command, or public support claim is required or changed by
  this audit. The public CLI and source extension remain ling and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A checked Typed Core to Device IR mapping boundary covering permitted Kernel
   types/control flow/effects, workgroup/grid, shapes/indexes, memory safety,
   ownership, synchronization, numeric modes, and Semantic-ID/source-span
   witnesses.
2. A backend-neutral, versioned IR schema for scalar/vector/tensor types,
   address spaces, loads/stores, control flow, barriers/atomics, shape/layout,
   numeric mode, capabilities, required features, Faults, and cancellation.
3. Memory, ownership, and synchronization semantics for each operation,
   including aliasing, bounds, atomic order, barrier scope, visibility,
   queue interaction, resource limits, and deterministic reduction behavior.
4. Capability and target negotiation rules for required versus optional
   features, unsupported-target rejection/fallback, backend adapter identity,
   reproducible lowering, and cache/provenance identity.
5. Canonical schema lifecycle with deterministic ordering, stable IDs, source
   maps, exact numeric encoding, corruption/migration rules, redaction, and
   explicit Internal/Preview/Stable status.
6. Bilingual L-<DOMAIN>-<NUMBER> diagnostics and positive, negative, property,
   corruption, migration, Unicode/source-map, determinism, bounds, ownership,
   synchronization, numeric, CPU-reference, and cross-target fixtures.

## Evidence and compatibility impact

The eventual schema must be consumed only from checked Typed Core or a verified
derivative and must preserve original UTF-8 byte spans, Semantic IDs, Unicode
17.0.0 behavior, deterministic ordering, declared effects, and explicit Fault
provenance. Host pointers, driver text, hardware IDs, timing, addresses, and
debug output must not become Ling semantics or stable protocol fields.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

DIR-4501 implementation, Device IR types and operations, schema/encoder/
decoder/validator, canonicalization, source-map carrier, capability registry,
backend feature negotiation, corpus, Unicode/source-map cases, editor
integration, and public protocol claims remain deferred until RFC-H404 (or an
Accepted replacement), DBUF-4401 through DBUF-4404, and the Kernel/device and
Native/backend gaps are resolved by Accepted authority and executable evidence.
