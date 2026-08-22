# DIR-4503-OBSERVATION Authority Audit — Device IR Canonicalization Evidence

Status: BlockedSpec

Date: 2026-08-23

## Outcome

Accepted `DEC-0164` authorizes test-local vocabulary only. No canonicalizer,
hash API, schema registry, migration reader/writer, diagnostic allocation,
public protocol, dependency, toolchain, or support claim is added. DIR-4503
remains `BlockedSpec` for Device IR canonicalization.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:317-326` lists
  canonicalization goals but is non-normative and does not define node
  identity, block ordering, constant encoding, hash domains, target inputs,
  schema versions, or migration behavior.
- `docs/ROADMAP-1.0.md:381-429` requires reproducible Device lowering and
  differential evidence, but does not authorize a Device IR schema, canonical
  byte protocol, or public identity.
- `docs/decisions/0012-semantic-identity-and-canonical-bytes.md` defines the
  accepted canonical projections for current semantic identities; it does not
  define Device IR identity, target specialization, or hardware hashing.
- RFC-H404 is absent and DIR-4501/4502 remain `BlockedSpec`.
  `GAP-KERNEL-DEVICE-001` leaves Kernel/device operations, synchronization,
  numeric determinism, placement, and capabilities unresolved;
  `GAP-NATIVE-BACKEND-ABI-001` leaves backend target and layout identity
  unresolved.

## Current implementation evidence

- No Device IR model, canonical serializer, node/block ordering pass,
  canonical constant encoder, target-independent hash, specialization hash,
  schema registry, migration reader/writer, or canonical corpus exists under
  `crates` or `tests`.
- No contract fixes whether source spans, Semantic IDs, capabilities, numeric
  modes, target features, workgroup sizes, layouts, or resource limits
  contribute to either hash; nor does one define equivalence under
  commutative or target-specific transformations.
- No contract defines unordered collections, NaN or signed-zero constants,
  integer widths, opaque handles, atomics/barriers, source maps, optional or
  unknown fields, corruption, redaction, or version migration.
- No Device IR protocol, diagnostic allocation, dependency, target/toolchain
  selection, CLI command, or public support claim is required or changed by
  this evidence.

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
6. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics plus positive, negative,
   property, corruption, migration, Unicode/source-map, determinism,
   cross-target, and collision-resistant fixtures executable offline.

## Evidence and compatibility impact

The eventual canonicalizer must consume checked Typed Core or a verified
Device IR only, preserve original UTF-8 byte spans and Semantic IDs where the
schema requires them, retain Unicode 17.0.0 behavior, and use deterministic
ordering and domain separation. Hashes must not expose host paths, allocation
addresses, driver versions, timestamps, timing, or debug text as Ling identity.

This evidence changes no parser, resolver, type or effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schema, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or runtime.

## Intentionally deferred

Canonical ordering and constants, target-independent and specialization hashes,
schema registry and migration, corruption corpus, Unicode/source-map cases,
editor integration, and public protocol claims remain deferred until
DIR-4501/4502, RFC-H404 (or an Accepted replacement), and the Kernel/device
and Native/backend gaps are resolved by Accepted authority and executable
evidence.
