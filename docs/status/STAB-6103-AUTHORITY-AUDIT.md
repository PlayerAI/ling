# STAB-6103 Authority Audit

- Task: `STAB-6103` — Feature State Metadata
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:63-82`
- Release: G6
- Status: `BlockedSpec`

## Decision

STAB-6103 is `BlockedSpec` for its proposed public surfaces. The plan asks
every public capability to expose `Experimental`, `Preview`, `Stable`,
`Deprecated`, or `Removed` metadata through a feature command, build manifest,
LSP, documentation, package metadata, and Zed compatibility table. It does not
define a public schema, lifecycle transition rules, feature identity, source
of truth, compatibility policy, or the meaning of each consumer's fields. The
plan's `zero features --json` name is also stale under the repository's accepted
`ling` CLI authority.

G0 governance already provides an internal, drift-checked feature-state view.
That fixture explicitly is not a public contract and the future `ling support`
command is unimplemented. Exposing it as a public protocol, or fabricating the
listed LSP/build/package/Zed consumers, would create an unsupported API.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:63-82` is a non-normative checklist. It gives a
  vocabulary and consumer list but no schema, version marker, lifecycle
  transition matrix, compatibility guarantee, or public command contract.
- `docs/ROADMAP-1.0.md:58-66` separates Experimental, Preview, Stable,
  Deprecated, and Removed states, while §2.3 requires explicit state and
  versioning. The roadmap does not select a public feature-state protocol or
  authorize a CLI/LSP/package/Zed implementation.
- GOV-0109's accepted G0 governance slice extends
  `docs/status/implementation-status.toml` and generates the bilingual status
  page, release-note input, and an internal governance fixture. Its report
  explicitly records `implemented: false`, `public_contract: false`, no
  accepted public feature-status CLI schema, and no fabricated LSP/build/package
  consumers. It is internal evidence, not STAB-6103 completion.
- `tests/fixtures/status/feature-state.governance.json` uses the internal
  `ling.governance.feature-state-fixture/1` marker and proposes—but does not
  implement—`ling support --format json`. `docs/status/implementation-status.md`
  repeats that the fixture is internal and the command is not implemented.
- `docs/governance/support-matrix.toml` is the current support authority, but
  its matrix target is `1.0-draft`; Seed features are Experimental and Native/
  Critical profiles are Unavailable. It does not define a public feature-state
  reader/writer or package/LSP/Zed metadata contract.
- `docs/governance/protocol-inventory.toml` marks CLI and diagnostic protocols
  Preview, and lists no feature-state protocol. Accepted CLI rules require the
  `ling` command, stable exits, bilingual diagnostics, and original UTF-8
  spans; a stale `zero` command or compatibility alias is not authorized.
- The root `AGENTS.md` prohibits placeholder public APIs, stale `zero` names,
  and unsupported claims, and requires accepted authority before a public
  protocol. Existing Seed/VM RFCs do not define feature-state metadata.

## Evidence in this repository

`tools/xtask/src/status.rs` reads the status registry and deterministically
generates `implementation-status.md`, `release-status.md`, and the internal
fixture; it does not expose a runtime CLI, build-manifest, LSP, package, or Zed
feature-state consumer. The internal fixture declares `implemented: false` and
`public_contract: false`. No public `ling features`/`ling support` command,
feature-state schema, build metadata field, LSP capability, package field, or
Zed compatibility table exists. Current support/traceability views therefore
provide governance evidence only and must not be relabeled Stable.

## Required authority before implementation

An accepted feature-state decision must define, at minimum:

1. Stable feature/profile/target identity, state vocabulary, lifecycle
   transitions, ownership, release/version fields, blocker semantics, and the
   distinction between implementation state and compatibility stability.
2. A versioned public schema and canonical source of truth spanning CLI/build
   manifest/package metadata/LSP/Zed/documentation, including unknown-field,
   unknown-state, N-1, migration, and fail-closed behavior.
3. Accepted links for every published feature and consumer, with parser,
   checker, Typed Core, interpreter/VM/Native/device, diagnostics, conformance,
   limitations, and support-matrix evidence. Internal governance fixtures must
   remain distinguishable from public contracts.
4. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and original UTF-8
   byte-span rules for unknown, conflicting, unavailable, deprecated, removed,
   or unsupported state; no status must imply a profile/target not supported by
   the matrix.
5. Offline positive, negative, malformed, version, migration, repeated-build,
   CLI, LSP, package, build-manifest, documentation, and editor compatibility
   fixtures, plus deterministic generated-consumer drift checks.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, support
claim, editor integration, package metadata, or Semantic ID rule. It preserves
the accepted `ling` CLI and `.ling` source extension, checked Typed Core
boundary, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the existing internal governance status views.

It deliberately adds no public feature-state schema, command, build/package
field, LSP/Zed route, compatibility promise, diagnostic, protocol, or
placeholder API, and introduces no stale `zero` names. STAB-6103 remains
deferred until an accepted public status protocol and lifecycle exist, the
support matrix is no longer a draft-only claim, and all advertised consumers
have executable offline evidence.
