# PROTO-6202 Authority Audit

- Task: `PROTO-6202` — Reader/Writer Compatibility Tests
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:109-121`
- Release: G6
- Status: `BlockedSpec`

## Decision

PROTO-6202 is `BlockedSpec`. The G6 checklist requests current/current and
current/N-1 reader/writer tests, unknown and missing fields, future-version
rejection, corrupt/truncated input, canonical re-encoding, migration, and
size/depth limits. It does not select which protocols have an N-1 version,
define compatibility versus stability, or authorize migration and canonical
identity behavior across the Future ABI, replay, evidence, device, LSP, and
package surfaces.

The repository already has scoped schema and bytecode validation for accepted
formats. `schemas/registry.toml` deliberately records first-version
`NoPreviousVersion` separately from supported N-1 reading, and
`SCHEMA-LIFECYCLE.md` is Draft. Generalizing those tests into a 1.0 promise
would invent compatibility edges and readers for protocols that have no
accepted schema or implementation.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:109-121` is a non-normative test checklist. It
  does not define protocol-specific version graphs, reader/writer ownership,
  unknown-field policy, migration semantics, canonical identity, or the limits
  and failure diagnostics for each format.
- `docs/governance/SCHEMA-LIFECYCLE.md:43-57` explicitly separates stability
  from parse compatibility and states that first-version `NoPreviousVersion`
  is not an N-1 reader. Its policy is Draft and does not define package,
  bytecode, replay, ABI, evidence, transaction, Native, device, or profile
  schemas.
- `schemas/registry.toml:7-12` makes compatibility explicit per concrete
  schema, uses `CurrentOnly` writers, and forbids implicit compatibility. The
  existing entries mostly have `NoPreviousVersion`; this is not a completed
  cross-protocol migration contract.
- Root `AGENTS.md` requires accepted authority before public protocols, stable
  claims only after ROADMAP gates and executable fixtures, deterministic/offline
  behavior, bilingual diagnostics, and no stale `zero` surfaces. Compatibility
  tests cannot be used to create a missing protocol or public API.
- The active protocol inventory reports 0 Stable current public protocols and
  Future records with no versions/fixtures. The support matrix marks ABI,
  evidence, replay, build metadata, and other later protocols Future; Native,
  Critical, and device targets remain unavailable or unsupported.
- Accepted RFC-0014 through RFC-0020 provide compatibility/robustness rules for
  their covered bytecode, VM, diagnostics, cancellation, and differential
  slices only. They do not establish N-1 readers for every public protocol or
  migration for future formats.
- `docs/ROADMAP-1.0.md:514-522` is a planned G6 gate requiring per-schema
  version, unknown-field, N-1/migration, canonical, golden, and corrupt-input
  evidence. The roadmap does not authorize those edges before protocol RFCs.

## Evidence in this repository

The repository has deterministic schema validation and corrupt-input commands,
JSON valid/invalid fixtures, and bytecode malformed/decode tests for existing
protocols. These readers and writers are scoped to current versions; no general
N-1 reader, migration tool, cross-protocol compatibility harness, or canonical
re-encoder exists for Future ABI/replay/evidence/device/LSP formats. Existing
schema records explicitly use `NoPreviousVersion`, and no public protocol claims
that a compatibility edge has been proven beyond its registered fixtures.

## Required authority before implementation

An accepted compatibility decision must define, at minimum:

1. A version graph and reader/writer owner for each protocol/schema, including
   first-version `NoPreviousVersion`, supported N-1 edges, unknown-version
   rejection, and which Future or Unsupported records are excluded.
2. Per-format unknown-field, missing-field, default, canonical encoding,
   identity/hash, migration, size/depth/resource, security, and corruption
   semantics, with no cross-protocol inference.
3. Accepted public schemas and implementation adapters for current/current,
   N-1, migration, canonical re-encoding, and future-version rejection;
   malformed input must remain isolated from Typed Core and evaluation.
4. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, original UTF-8
   byte-span behavior where applicable, deterministic ordering, and fail-closed
   handling of unknown fields, truncated data, invalid versions, overlimits,
   migration failures, and identity mismatches.
5. Offline positive, negative, N-1, unknown-field, missing-field,
   future-version, corrupt/truncated, canonical, migration, size/depth,
   cross-process, repeated-build, Unicode 17.0.0, BOM/CRLF, and release
   fixtures, with generated registry and report drift checks.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, support
claim, Semantic ID rule, or existing reader/writer. It preserves the accepted
`ling` CLI and `.ling` source extension, current schema/bytecode policies,
checked Typed Core boundary, original UTF-8 spans, Unicode 17.0.0, deterministic
identity rules, and explicit `NoPreviousVersion`/Experimental/Preview/Future
states.

It deliberately adds no N-1 reader, migration tool, canonical re-encoder,
compatibility edge, protocol version, ABI/device/replay/evidence reader,
diagnostic, CLI command, public API, or placeholder, and introduces no stale
`zero` names. PROTO-6202 remains deferred until PROTO-6201, protocol-specific
Accepted authority, and executable compatibility evidence exist for each
claimed edge.
