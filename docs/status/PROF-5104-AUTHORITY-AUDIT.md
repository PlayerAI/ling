# PROF-5104 Authority Audit — Profile Audit and LSP

Status: BlockedSpec

Date: 2026-08-22

## Outcome

PROF-5104 proposes profile checks and audits plus editor feedback for the
specific Effect, capability, or unbounded source that violates a Profile. Its
plan examples use `zero check`, `zero audit`, and `zero explain` commands.

No accepted Profile schema/checker exists, and the LSP and Semantic
Transaction protocol lifecycles remain open. The accepted public names are
`ling` and `.ling`; the stale `zero` commands are not implementation targets.
Adding a CLI route, LSP method, diagnostic payload, or profile explanation now
would create an unauthorized public protocol and freeze unresolved Critical
semantics.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:126-136` is a
  non-normative plan fragment. It does not define command names, exit/status
  behavior, profile source and selection, explanation fields, LSP methods,
  document/version identity, position encoding, cancellation, or lifecycle.
- `docs/ROADMAP-1.0.md:142-151` requires separate compatibility rules for CLI,
  diagnostics, Semantic Graph, and future tooling protocols; G5 depends on
  earlier replay/resource/lowering authorities. It does not authorize a
  Profile audit or editor protocol.
- `GAP-CRITICAL-PROFILE-001` is Open and blocks PROF-5101 through the Critical
  checker chain. `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` are Open; they leave LSP request,
  diagnostics, Semantic Graph/Transaction fields, versioning, and migration
  unresolved.
- Accepted DEC-0002/DEC-0012 and the existing experimental Semantic Graph
  surfaces do not authorize a profile-audit schema or LSP publication. The
  support matrix explicitly marks profile selection/enforcement and LSP/editor
  capabilities unavailable.
- The repository instructions and `docs/LANGUAGE.md` fix the public CLI as
  `ling`; plan references to `zero` must not enter commands, fixtures, schemas,
  editor integration, or documentation of implemented behavior.

## Current implementation evidence

- The repository has no Profile checker/audit command, explanation report,
  profile-to-source diagnostic mapper, or profile-aware LSP service under
  `crates` or `tests`. `ling-db`'s internal Profile workspace input is only a
  cache/revision dimension, not a public profile or audit protocol.
- No accepted schema fixes diagnostic severity, stable code/fact fields,
  source/related ranges, Effect/capability/unbounded provenance, quick-fix
  safety, profile/target identity, localization, or unknown-field policy.
- No accepted LSP contract fixes initialize/shutdown, document URI/FileId and
  version, UTF-16 positions, workspace/profile selection, request cancellation,
  stale-result rejection, ordering, limits, or capability negotiation for
  profile violations.
- No CLI lifecycle or protocol-inventory entry authorizes `ling check --profile`,
  `ling audit --profile`, `ling explain profile`, JSON output, exit codes, or
  migration. The plan's `zero` names are explicitly stale placeholders.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Profile audit/explanation schema with canonical machine fields,
   diagnostic-only text, Effect/capability/bound provenance, Semantic IDs and
   UTF-8 byte spans, related ranges, severity, stable codes, ordering, limits,
   redaction, localization, unknown-field handling, and migration.
2. An accepted `ling` CLI contract for profile selection, check/audit/explain
   behavior, manifest/config precedence, exit codes, human/JSON formats,
   diagnostics, offline operation, and compatibility lifecycle.
3. LSP request/notification methods, initialization/capabilities, document
   version and position rules, workspace/profile identity, cancellation,
   stale-result rejection, deterministic publication, limits, and error
   mapping under `GAP-LSP-TRANSACTION-PROTOCOL-001` and
   `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001`.
4. The accepted Critical profile, capability/effect, bounds, ownership,
   concurrency, numeric, Device/Native, Fault, and verification authorities
   that make an audit finding meaningful rather than a guessed policy.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured repair
   facts for malformed/unavailable/conflicting profiles, forbidden effects or
   capabilities, unbounded sources, stale requests, and unsupported targets.
6. Offline CLI/LSP positive and negative fixtures covering Unicode/CRLF,
   source maps, profile layers, transitive effects, bounds, revisions,
   cancellation, stale results, deterministic ordering, JSON migration,
   privacy, and differential behavior.

## Evidence and compatibility impact

The eventual audit must report checked compiler facts and never become a second
profile solver or claim proof from a green editor indicator. It must preserve
original UTF-8 byte spans and Semantic IDs, keep machine fields stable and
diagnostic text localized, and avoid exposing host paths, addresses, timing,
allocation order, or debug output as Ling identity.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
diagnostics, schemas, Semantic IDs, source spans, CLI, LSP, dependency lock,
target/toolchain, support claim, or Unicode 17.0.0 behavior.

## Intentionally deferred

PROF-5104 implementation, Profile audit/report schema, `ling` command routes,
LSP/Zed integration, explanation/quick-fix behavior, diagnostics, migration,
and public protocol claims remain deferred until Accepted Profile/RFC-0012 (or
replacement), Critical capability authority, LSP/Semantic Transaction
lifecycles, and executable offline fixtures exist. The stale `zero` commands
remain prohibited; no placeholder CLI or editor API is created.
