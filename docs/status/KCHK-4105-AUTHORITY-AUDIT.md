# KCHK-4105 Authority Audit — Kernel Core and Verifier

Status: BlockedSpec

Date: 2026-08-22

## Outcome

KCHK-4105 proposes a device-independent, versioned Kernel Core with an
independent verifier. The listed invariants cover legal types and control
flow, forbidden Effects, buffer and address-space consistency, barriers,
race/alias annotations, and deterministic serialization.

The task cannot be implemented as an IR, verifier, schema, or backend
boundary today. No Kernel Core representation, encoder/decoder, verifier,
serialization protocol, source-map format, diagnostic, Device IR, backend,
or command is added. The preceding KCHK-4101 through KCHK-4104 contracts and
the Kernel/device authority remain unresolved.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:124-136 is a
  non-normative plan sketch. It names a versioned Kernel Core and checks but
  does not define its node set, trust boundary, canonical bytes, identity,
  versioning, migration, resource limits, or diagnostic behavior.
- docs/ROADMAP-1.0.md:381-429 requires a separate Kernel verifier before
  lowering, a backend-neutral representation, CPU-reference differential
  evidence, and explicit unsupported-target behavior. These G4 gates are
  planning requirements, not an Accepted Kernel Core specification.
- docs/SEMANTICS.md:1429-1480 sketches Kernel purity, Device Buffers,
  deterministic reductions, and lowering targets; docs/SEMANTICS.md:1872-1928
  reserves Kernel in the non-implemented v0.0.1 schema. No accepted Core or
  Device IR schema follows from those descriptions.
- RFC-0014, RFC-0018, and RFC-0019 define experimental/foundation VM
  bytecode, source-map, Fault, verifier, cancellation, and
  Interpreter/VM differential boundaries for the scalar VM slice. They do
  not authorize Kernel types, Device Buffers, barriers, address spaces,
  alias/race annotations, or device serialization.
- GAP-KERNEL-DEVICE-001 is Open in docs/governance/gap-register.toml and
  blocks KCHK-4105 and downstream device work. It leaves the Kernel subset,
  effects, ownership/address spaces, synchronization, numeric determinism,
  Placement, and backend capability contracts unresolved.
- RFC-0013 and the plan's RFC-H401 dependency are not accepted documents.
  RFC-0001 remains Draft under
  docs/decisions/0018-rfc-0001-lifecycle.md, and no Kernel Core or Device IR
  protocol is registered in docs/governance/protocol-inventory.toml.

## Current implementation evidence

- The repository has no Kernel Core, Device IR, encoder/decoder, independent
  verifier, source-map bridge, or deterministic Kernel serializer under
  crates or tests. Existing bytecode and VM verifiers are limited to the
  accepted scalar VM foundations and do not consume Kernel artifacts.
- No accepted rule fixes the Core node grammar, legal type/control-flow
  subset, effect/capability and allocation encoding, shape/index and
  address-space model, barrier scope, alias/race annotations, reduction
  determinism, Fault provenance, or target capability declarations.
- No accepted artifact identity or migration contract fixes Program/Semantic
  IDs, source spans, canonical ordering, schema versioning, unknown fields,
  corruption handling, resource limits, or cross-version reader behavior for
  Kernel artifacts.
- Kernel CPU, GPU, and accelerator backends are Unsupported and Experimental
  in docs/governance/support-matrix.toml behind GAP-KERNEL-DEVICE-001. No
  diagnostic allocation, dependency, target/toolchain, public protocol, or
  CLI command is required or changed by this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Kernel RFC and Core schema defining the complete device-neutral
   node set, legal types/control flow, Effect and Capability representation,
   allocation, shape/index/layout, address spaces, buffers, barriers,
   alias/race annotations, reductions, atomics, Faults, and determinism.
2. A trust boundary with an independent bounded decoder/verifier that is the
   only constructor of executable or lowerable Kernel artifacts. It must
   consume checked Typed Core or a verified derivative and reject malformed,
   unsupported, resource-exhausting, or semantically inconsistent input
   before any backend sees it.
3. Canonical deterministic serialization and identity rules for source maps,
   Semantic IDs, Program/Graph identity, target-independent Core bytes,
   target specialization, schema versions, unknown fields, corruption,
   migrations, and compatibility claims. Host paths, addresses, driver
   output, allocation order, and hash-map iteration must be excluded.
4. A verifier semantics for legal control flow, effect/capability closure,
   buffer/address-space consistency, barrier legality, alias/race evidence,
   shape/bounds/overflow, reduction/atomic ordering, Fault mapping, and
   target/profile capability rejection.
5. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics with deterministic
   ordering and structured facts for malformed artifacts, each invariant
   failure, resource limits, unsupported versions, migration failure, and
   backend mismatch. Original UTF-8 byte spans and Semantic IDs must survive.
6. Positive, negative, corruption, resource-limit, round-trip, migration,
   Unicode/source-map, canonicality, CPU-reference, VM/Native differential,
   unsupported-target, and deterministic evidence before any public protocol,
   backend, or editor support claim.

## Evidence and compatibility impact

The eventual implementation must publish the versioned Core schema and
reader/writer fixtures, independent verifier tests, deterministic canonical
bytes, bounded decode/verify behavior, and explicit migration/compatibility
rules. It must compare the CPU reference with every supported lowering under
accepted exact or tolerance rules, reject unsupported devices without
silently changing Effects or numerics, and keep vendor details outside Ling
semantics. Any public Kernel Core or verifier protocol needs an inventory
entry, Accepted authority, schema lifecycle, and an explicit Preview or
Stable status.

This audit changes no parser, Typed Core, evaluator, bytecode, VM, Native
backend, memory category, ownership behavior, effect or capability checker,
Device Buffer, scheduler, diagnostics, schema, Semantic IDs, source spans,
CLI, dependency lock, target/toolchain, support claim, or Unicode 17.0.0
behavior.

## Intentionally deferred

KCHK-4105 implementation, Kernel Core and Device IR schemas, encoder/decoder,
independent verifier, canonical serialization, source maps, diagnostics,
CPU reference, SIMD/GPU/accelerator backends, Placement, editor support, and
public protocol claims remain deferred until GAP-KERNEL-DEVICE-001 and the
preceding Kernel/Effect/ownership/bounds authorities are Accepted and the
required executable evidence exists.
