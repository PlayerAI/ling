# COMPAT-6501 Authority Audit

- Task: `COMPAT-6501` — Historical Corpus
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:265-278`
- Release: G6
- Status: `BlockedSpec`

## Decision

`COMPAT-6501` is `BlockedSpec`. The G6 checklist asks for representative
corpora from Seed through v0.5 covering source programs, parser trees,
diagnostics, Semantic Graph, Audit, bytecode, package/lock, replay, evidence,
and Zed/LSP fixtures. The repository has an accepted v0.0.1 Seed conformance
boundary and several independently versioned Experimental protocols, but it
does not define the historical language/protocol versions, corpus manifest,
compatibility meaning, or migration policy required to freeze a cross-version
corpus.

The list also includes Replay, evidence bundles, Zed/LSP, package publication,
and other surfaces that the current support matrix marks Future or Unsupported.
Creating fixtures for those names would imply semantic or public-protocol
support that is not authorized. Existing v0.0.1 fixtures remain valid evidence
and must not be relabeled as a v0.5 historical corpus. Accepted `DEC-0230` now
authorizes a bounded child that freezes their actual bytes and classifies every
requested surface without claiming nonexistent release history.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:263-305` is a non-normative compatibility
  checklist. It names a desired corpus and future migration workflow but does
  not define release versions, accepted syntax/semantics for each version,
  artifact IDs, expected outcomes, reader ranges, or migration schemas.
- `ROADMAP-1.0` §11.5 and §12 require historical corpus and compatibility
  evidence as release engineering practice. `ROADMAP-1.0` is a planning
  document with `stable_basis = false`; it cannot authorize v0.1–v0.5 language
  behavior or public protocol versions by itself.
- The active `CONFORMANCE` authority covers the v0.0.1 `tests/conformance`
  Seed behavior only and is not a Stable basis. There is no accepted
  versioned historical-corpus manifest or v0.5 conformance authority.
- Accepted `DEC-0001` governs diagnostic-code allocation, retirement, and
  compatibility meaning, not source/protocol compatibility or migration. Its
  registry evidence cannot stand in for a complete compiler corpus.
- Accepted `RFC-0002` supplies migration rules only for its future manifest and
  lock format versions and explicitly requires new format versions rather than
  silently recognizing pre-RFC inputs. It does not define a historical corpus
  for the language, parser, Semantic Graph, registry, or editors.
- `docs/governance/SCHEMA-LIFECYCLE.md` is Draft policy for already inventoried
  JSON schemas; first versions are `NoPreviousVersion`, and future schemas
  require explicit reader/migration edges and fixtures. It does not authorize
  Replay, evidence, package-publication, or LSP corpus packages.
- Accepted bytecode RFCs keep `ling.bytecode/1.x` Experimental and explicitly
  make no general release or N-1 compatibility promise. The protocol inventory
  marks Replay and evidence as Future and the support matrix marks LSP/Zed and
  package publication outside the supported surface.
- RFC-0001 remains Draft; the accepted lifecycle decision requires dedicated
  Accepted successor RFCs for post-Seed behavior. No historical v0.5 claims
  may derive from RFC-0001 examples or the execution-plan wording.
- Root `AGENTS.md` requires Accepted authority before semantic/public-protocol
  expansion, stable diagnostics and schemas, deterministic/offline evidence,
  original UTF-8 spans, Unicode 17.0.0, checked Typed Core inputs, and no
  placeholder APIs.

## Evidence in this repository

The repository currently contains useful bounded corpora:

1. v0.0.1 parser/compiler conformance cases with bilingual diagnostics and
   original UTF-8 byte spans;
2. accepted Audit Source and Semantic Graph fixtures with their own protocol
   versions and canonical-byte policies;
3. Experimental bytecode 1.0–1.2, VM, project manifest/graph, and `ling.lock/1`
   fixtures governed by their individual RFCs; and
4. editor/tree-sitter and internal cache evidence that is explicitly not a
   compiler or stable editor compatibility promise.

These artifacts do not provide:

- a versioned v0.1–v0.5 language feature and diagnostic inventory;
- source/parser/Typed Core/Semantic ID equivalence expectations per release;
- canonical corpus manifest fields, fixture identity, provenance, or release
  selection rules;
- compatibility outcomes (`accept`, warning, migrate, reject), N-1 readers,
  migration tools, deprecation diagnostics, or rollback behavior; or
- accepted Replay, evidence, package-publication, Zed/LSP, formatter, or
  cross-platform corpus protocols.

Copying current fixtures into new version directories would create false
historical claims, while adding future fixtures would blur Unsupported and
Experimental boundaries.

## Required authority for the blocked remainder

An accepted compatibility and corpus decision must define, at minimum:

1. The release/version inventory and governing Accepted specification for each
   language, diagnostic, Semantic Graph, Audit, bytecode, package/lock, replay,
   evidence, editor, and platform surface; Draft RFC-0001 material must be
   excluded or explicitly marked provenance only.
2. A deterministic corpus manifest with fixture IDs, source bytes, expected
   parser/Typed Core/diagnostic/graph/audit/protocol outputs, canonical bytes,
   Unicode and span versions, provenance, and integrity checksums.
3. The compatibility matrix and migration lifecycle: accepted unchanged,
   warning, automatic migration, or actionable rejection; reader ranges,
   unknown-field behavior, semantic-ID/hash impact, deprecation, rollback, and
   no silent reinterpretation rules.
4. Separate authority and support claims for Future/Experimental/Preview
   Replay, evidence, package, Zed/LSP, formatter, VM, and backend surfaces,
   including their CLI/schema/editor contracts where applicable.
5. Positive/negative, corruption, migration, determinism, cross-process,
   cross-platform, Unicode 17.0.0, UTF-8 byte-span, and security fixtures,
   plus generated protocol/schema/support/traceability/status drift checks.

## Compatibility and deferred work

This audit changes no source grammar, parser, resolver, evaluator, Semantic ID,
diagnostic, schema, bytecode, package/lock, replay, evidence, editor, CLI,
formatter, dependency, or public API behavior. It preserves the existing
v0.0.1 conformance corpus, accepted individual protocol versions, Unicode
17.0.0 and original UTF-8 spans, checked Typed Core boundaries, and explicit
Experimental/Preview/Future/Unsupported states.

It deliberately adds no historical-version directory, corpus manifest,
compatibility matrix, migration tool, warning/rejection diagnostic, Replay or
evidence schema, package/editor fixture, dependency, protocol, or placeholder.
The bounded `COMPAT-6501-SEED` child freezes only v0.0.1. Cross-release corpus
freezing remains deferred until release authorities, per-surface compatibility
rules, original historical artifacts, and executable migration/equivalence
evidence are Accepted.
