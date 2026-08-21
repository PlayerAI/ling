# PROTO-6201 Authority Audit

- Task: `PROTO-6201` — Protocol Registry
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:86-107`
- Release: G6
- Status: `BlockedSpec`

## Decision

PROTO-6201 is `BlockedSpec`. The plan proposes a new
`docs/protocols/registry.toml` containing protocol owners, schema versions,
stability, reader/writer policy, and golden corpora. It does not define the
registry's authority, public versus internal scope, lifecycle transitions,
canonical identity, or how it relates to the already active governance
protocol inventory and schema registry. Creating a second source of truth would
allow protocol/version drift.

The repository already has an active governance registry at
`docs/governance/protocol-inventory.toml`, a generated report, and
`schemas/registry.toml`. The inventory records 21 protocols, 0 Stable current
public protocols, and Future records with no versions or fixtures. It is
governance evidence, not an accepted 1.0 public protocol registry. No task may
promote it or the proposed path to Stable without the missing G1-G5 exits,
Accepted protocol decisions, and executable compatibility evidence.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:86-107` is a non-normative checklist. It does
  not define protocol identity, schema ownership, lifecycle transitions,
  reader/writer compatibility, unknown-field behavior, canonical encodings,
  migration, or the boundary between a registry record and a public protocol.
- Root `AGENTS.md` requires every implemented or planned public protocol to be
  registered in `docs/governance/protocol-inventory.toml` and forbids Stable
  claims before the ROADMAP-1.0 gates, Accepted authority, and executable
  fixtures. A new ungoverned `docs/protocols/registry.toml` would violate that
  single-inventory rule.
- `docs/governance/authority.toml:655-665` declares
  `PROTOCOL-INVENTORY` (`docs/governance/protocol-inventory.toml`) an active
  governance registry with `stable_basis = false`; it covers versions,
  stability, reader/writer policies, unknown fields, canonical encodings, and
  Future boundaries. `SCHEMA-LIFECYCLE-POLICY` is Draft and
  `SCHEMA-REGISTRY` is an active compatibility registry dependent on it, so
  neither supplies a Stable protocol decision.
- The generated `docs/governance/protocol-inventory.md` reports 21 records,
  15 current public, 1 internal, 5 Future, and 0 Stable. It states that Stable
  means the ROADMAP-1.0 commitment and that no current Seed protocol has passed
  that gate.
- `docs/governance/support-matrix.toml` marks CLI and diagnostics Preview,
  current graph/package/bytecode protocols Experimental, and ABI, replay,
  build metadata, and evidence protocols Future with no versions or fixtures.
  Native ABI and Device artifact metadata therefore cannot be listed as Stable
  merely because the plan names them conditionally.
- `docs/ROADMAP-1.0.md:500-573` requires G1-G5 exits, Accepted RFCs,
  bidirectional traceability, compatibility/corrupt-input tests, deterministic
  offline builds, and independent release evidence. It is a release plan, not
  a protocol schema.
- Accepted Seed/VM RFCs define only their covered protocols (for example
  `ling.bytecode/1.x` and its VM evidence). They do not authorize a universal
  registry, future ABI/evidence/replay protocols, or a second public metadata
  source. Stale `zero` protocol examples from lower plans are not authority.

## Evidence in this repository

`docs/protocols/registry.toml` does not exist. The active governance inventory
and generated report are the only protocol registry views; `schemas/registry.toml`
tracks public JSON schemas and is not a replacement for a semantic protocol
lifecycle decision. Future inventory records have empty versions/fixtures and
current public records remain Experimental or Preview. No independent G6
registry writer, public registry reader, migration tool, canonical registry
encoding, or universal golden corpus exists.

## Required authority before implementation

An accepted protocol-registry decision must define, at minimum:

1. The single source of truth and ownership for protocol records, public versus
   internal/planned scope, stable identity, version markers, lifecycle states,
   supersession, and the relationship among governance inventory, schema
   registry, support matrix, traceability, and release evidence.
2. Required fields and semantics for owner, schema version, stability,
   visibility, reader/writer ranges, unknown/missing fields, canonical
   encoding, hash/identity domains, migration/N-1 policy, size/depth limits,
   security and resource constraints, and golden/corrupt corpus references.
3. Accepted authority and compatibility boundaries for each listed protocol:
   diagnostics, Semantic Graph, canonical bytes/IDs, Audit Source,
   transactions, package/lock/build metadata, bytecode, replay, ABI, device
   artifacts, evidence, LSP, and DAP. Future or unsupported entries must not
   gain a version or public implementation claim.
4. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics for registry/schema
   errors, original UTF-8 byte-span behavior, deterministic ordering, and
   fail-closed handling of duplicate IDs, unknown versions, malformed records,
   unsupported protocols, and missing fixtures.
5. Offline positive, negative, N-1, unknown-field, migration, canonical
   re-encoding, corruption/truncation, size/depth, cross-process, repeated-
   build, Unicode 17.0.0, and release-candidate fixtures, plus generated-report
   drift checks that cannot silently create a second registry.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, support
claim, Semantic ID rule, or registry source. It preserves the accepted `ling`
CLI and `.ling` source extension, existing governance inventory/schema registry,
checked Typed Core boundary, original UTF-8 spans, Unicode 17.0.0, deterministic
identity rules, and current Experimental/Preview/Future states.

It deliberately creates no `docs/protocols/registry.toml`, protocol writer or
reader, public registry schema, ABI/device/evidence entry, migration tool,
diagnostic, CLI command, or placeholder API, and introduces no stale `zero`
names. PROTO-6201 remains deferred until the single registry authority,
protocol lifecycle, G1-G5 exits, Accepted protocol decisions, and executable
compatibility/golden evidence are complete.
