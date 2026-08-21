# PROTO-6203 Authority Audit

- Task: `PROTO-6203` — Semantic Hash Upgrade Rehearsal
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:123-134`
- Release: G6
- Status: `BlockedSpec`

## Decision

PROTO-6203 is `BlockedSpec`. The G6 checklist requests a test-branch
simulation of a semantic hash/schema upgrade, including old and new algorithm
identifiers, dual readers, explicit migration, dependency and lock updates,
cache invalidation, replay/evidence linkage, a stable diagnostic, and a rule
against silently recomputing identity. It does not select the algorithm and
canonical-byte versioning model, define the reader/writer or migration
contract, or authorize changes to the current Semantic ID prefix.

The accepted identity decision deliberately keeps the current
`experimental:blake3:` text form stable and requires an explicit Semantic
Schema or ID-prefix upgrade when the algorithm, encoding, or normalization
rules change. The open semantic-hash and semantic-protocol lifecycle gaps
identify RFC-0004 as a candidate for that work. A rehearsal that invents an
algorithm registry, dual-reader behavior, migration edge, or cache identity
transition before that authority exists would turn a non-normative checklist
into a public compatibility promise.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:123-134` is a non-normative rehearsal
  checklist. It names the evidence categories but does not define algorithm
  IDs, canonical-byte projections, schema versions, dependency propagation,
  cache invalidation, replay/evidence schemas, or diagnostic codes.
- Accepted `docs/decisions/0012-semantic-identity-and-canonical-bytes.md`
  defines the exact experimental BLAKE3 text form, domain-separated and
  versioned canonical bytes, and the prohibition on silently changing the
  meaning of an existing prefix. It requires an explicit schema or ID-prefix
  upgrade and migration explanation when those inputs change.
- `GAP-SEMANTIC-HASH-LIFECYCLE-001` is `Open`, P0, and blocks v0.1 and v1.0.
  Its required action is to draft RFC-0004 migration and reader/writer rules
  without changing the current experimental prefix. Its alternatives cover
  coupling the algorithm to a versioned Semantic Schema or using a separate
  algorithm registry with separately versioned canonical projections.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` is also open and blocks the coordinated
  Semantic Graph/Transaction identity, reader/writer, stale-edit, and schema
  migration contract. These protocols cannot be used as an unstated cache or
  replay migration authority.
- `docs/governance/protocol-inventory.toml` registers `PROTO-SEMANTIC-ID` as
  Experimental with `experimental:blake3:` as the current version and a
  `None` migration tool. Its writer and reader policies reject unknown
  prefixes, algorithms, lengths, and non-hex text; incompatible identity
  evolution requires an explicit upgrade rather than silent reuse.
- `schemas/registry.toml` records current schema readers/writers and
  `NoPreviousVersion`; its own preamble distinguishes that state from an
  N-1 reader or migration adapter. It therefore does not authorize a semantic
  hash dual reader or a cross-protocol upgrade rehearsal.
- Root `AGENTS.md` requires accepted authority before public protocols,
  deterministic/offline evidence, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics,
  preserved UTF-8 spans, Unicode 17.0.0, checked Typed Core inputs, and no
  placeholder or stale `zero` surfaces.

## Evidence in this repository

The repository has deterministic Semantic ID and canonical-byte evidence for
the current experimental scheme, along with scoped schema compatibility
records. The active protocol inventory explicitly reports no migration tool
for Semantic IDs, and no registered fixture claims an old/new algorithm
reader, explicit identity migration, dependency/lock propagation, cache
invalidation, or replay/evidence linkage. Existing `NoPreviousVersion` entries
are first-version declarations, not proof of a supported hash upgrade path.

No accepted RFC currently defines the stable diagnostic, migration manifest,
lockfile update, cache-key invalidation event, or replay/evidence reference
that such a rehearsal would need. The plan's request is consequently useful
as acceptance criteria for future RFC-0004 work, but is not implementation
authority today.

## Required authority before implementation

An accepted semantic-identity decision (RFC-0004 or an accepted replacement)
must define, at minimum:

1. Algorithm identifiers, Semantic Schema and canonical-byte versions,
   prefixes, domain separation, normalization, and the exact rule for
   rejecting an unknown or incompatible identity.
2. Old/new reader and writer ownership, dual-reader scope, explicit migration
   inputs/outputs, idempotence, rollback/failure behavior, and an explicit
   prohibition on silently reusing or recomputing an existing identity.
3. Dependency and lockfile propagation, graph/transaction preconditions,
   cache-key derivation and invalidation ordering, stale-entry handling, and
   cross-process determinism. Path, allocation, map-order, and source-layout
   details must remain outside the identity.
4. Replay and AI-provenance/evidence linkage, including how an identity
   upgrade is recorded, verified, and rejected when evidence or dependencies
   no longer match.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and deterministic,
   offline positive, negative, cross-version, migration, dependency,
   invalidation, replay/evidence, Unicode 17.0.0, and corruption fixtures,
   with generated registry/report drift checks.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, cache
format, lockfile, replay/evidence format, or Semantic ID rule. It preserves
the accepted `ling` CLI and `.ling` source extension, the current
`experimental:blake3:` prefix, canonical-byte and span exclusions, checked
Typed Core boundaries, Unicode 17.0.0, deterministic/offline behavior, and
explicit Experimental/Preview/Future states.

It deliberately adds no algorithm registry, new hash prefix, dual reader,
migration tool, cache rewrite, dependency/lock update, replay/evidence
protocol, diagnostic, CLI command, public API, or placeholder. Future work
may implement a rehearsal only after the identity and protocol lifecycle
authority is Accepted, its dependency is resolved, and executable migration
and invalidation evidence is registered. The implementation must consume
checked Typed Core data and must never silently recompute a stable identity.
