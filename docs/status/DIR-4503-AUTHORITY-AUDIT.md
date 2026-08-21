# DIR-4503 Authority Audit — Device IR Canonicalization

Status: BlockedSpec

Date: 2026-08-22

## Outcome

DIR-4503 proposes deterministic Device IR node and block ordering, canonical
constants, a target-independent hash, a separate target-specialization hash,
exclusion of driver paths and timestamps, and schema-version/migration tests.

No canonicalizer or Device IR hash can be added yet. The Device IR schema,
semantic identity inputs, target capability model, specialization boundary, and
migration policy are not Accepted. The existing semantic identity decision
governs checked semantic identities, not an unreviewed hardware IR. Choosing
fields or hash domains now would create incompatible protocol commitments.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:317-324 lists
  canonicalization goals but does not define node identity, block ordering,
  constant encoding, target-independent inputs, specialization inputs,
  capability normalization, hash algorithm/domain, schema versions, or
  migration behavior.
- docs/ROADMAP-1.0.md:381-429 requires reproducible Device lowering and
  differential evidence, but it does not authorize a Device IR schema,
  canonical byte protocol, or public identity.
- docs/decisions/0012-semantic-identity-and-canonical-bytes.md defines the
  accepted canonical projections for current semantic identities and excludes
  spans, host paths, and implementation details; it does not define Device IR
  identity, target specialization, or hardware capability hashing.
- RFC-H404 is absent and DIR-4501/4502 are BlockedSpec. GAP-KERNEL-DEVICE-001
  leaves Kernel/device operations, synchronization, numeric determinism,
  Placement, and capabilities unresolved; GAP-NATIVE-BACKEND-ABI-001 leaves
  backend target and layout identity unresolved.
- The v0.0.1 Seed protocols and current Semantic Graph canonical bytes cannot
  be silently reused for a future Device IR with different operational and
  target inputs.

## Current implementation evidence

- No Device IR model, canonical serializer, node/block ordering pass,
  canonical constant encoder, target-independent hash, specialization hash,
  schema registry, migration reader/writer, or canonical corpus exists under
  crates or tests.
- No accepted rule fixes whether source spans, Semantic IDs, capabilities,
  numeric modes, target features, workgroup sizes, layouts, or resource limits
  contribute to each hash; nor does it define equivalence under commutative or
  target-specific transformations.
- No contract defines canonical handling of unordered collections, NaN or
  signed-zero constants, integer widths, opaque handles, atomics, barriers,
  source maps, optional fields, unknown fields, corruption, or version
  migration.
- No Device IR protocol, diagnostic allocation, dependency, target/toolchain
  selection, CLI command, or public support claim is required or changed by
  this audit. The public CLI and source extension remain ling and .ling.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Device IR schema and semantic identity model, including closed
   core fields, extension policy, source-map/Semantic-ID treatment, operation
   equivalence, and exact canonical bytes.
2. Deterministic ordering for nodes, blocks, operands, capabilities, layouts,
   constants, maps, and resource declarations, with explicit treatment of
   commutativity, alpha-renaming, and unreachable or rejected IR.
3. Separate domain-separated hashes for target-independent IR identity and
   target specialization, specifying included capabilities/features, backend
   version, numeric mode, layout, workgroup/grid, limits, and excluded host
   paths, timestamps, addresses, driver text, and debug output.
4. Canonical numeric and opaque-value encodings, including widths, endianness,
   NaN/signed-zero policy, atomics/barriers, source maps, and resource facts.
5. Schema lifecycle, reader/writer compatibility, malformed and corruption
   rejection, migration evidence, cache invalidation, and reproducibility
   requirements across processes and supported targets.
6. Bilingual L-<DOMAIN>-<NUMBER> diagnostics plus positive, negative, property,
   corruption, migration, Unicode/source-map, determinism, cross-target, and
   hash-collision-resistant fixtures executable offline.

## Evidence and compatibility impact

The eventual canonicalizer must consume checked Typed Core or a verified
Device IR only, preserve original UTF-8 byte spans and Semantic IDs where the
schema requires them, retain Unicode 17.0.0 behavior, and use deterministic
ordering and domain separation. Hashes must not expose host paths, allocation
addresses, driver versions, timestamps, timing, or debug text as Ling identity.

This audit changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

DIR-4503 implementation, canonical ordering and constants, target-independent
and specialization hashes, schema registry and migration, corruption corpus,
Unicode/source-map cases, editor integration, and public protocol claims remain
deferred until DIR-4501/4502, RFC-H404 (or an Accepted replacement), and the
Kernel/device and Native/backend gaps are resolved by Accepted authority and
executable evidence.
