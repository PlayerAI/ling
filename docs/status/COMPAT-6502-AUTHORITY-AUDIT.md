# COMPAT-6502 Authority Audit

- Task: `COMPAT-6502` — 1.0 Compiler Compatibility Matrix
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:280-291`
- Release: G6
- Status: `BlockedSpec`

## Decision

`COMPAT-6502` is `BlockedSpec`. The G6 checklist proposes four outcomes for
each historical version—accept unchanged, accept with warning, auto-migrate,
or reject with an actionable diagnostic—but it does not define the historical
version inventory, compatibility units, source/protocol readers, migration
authority, deprecation policy, or matrix schema. A rejection cannot cite a
specific accepted RFC or migration reason until those versions and authorities
exist.

The repository currently has a v0.0.1 Seed conformance corpus and several
independently versioned Experimental protocols. It does not have an accepted
1.0 compiler compatibility promise, a v0.1–v0.5 release corpus, a general
migration transaction, or an N-1 reader policy. Implementing a matrix now
would convert planning labels and Draft RFC material into compatibility
commitments.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:280-291` is a non-normative matrix outline.
  It supplies outcome words but no version identifiers, reader/writer ranges,
  syntax/semantic compatibility rules, diagnostic fields, migration artifacts,
  or rollback behavior.
- `ROADMAP-1.0` §11-§12 requires compatibility evidence, explicit Accepted
  RFC chains, and milestone traceability, but `ROADMAP-1.0` is planning
  authority with `stable_basis = false`; it does not define a compiler matrix.
- The active `CONFORMANCE` authority covers v0.0.1 Seed behavior only. No
  accepted v0.1–v0.5 language or compiler specification exists, and RFC-0001
  remains Draft under the accepted lifecycle decision.
- Accepted `DEC-0001` stabilizes diagnostic-code allocation, retirement, and
  bilingual payload compatibility. It does not define compiler acceptance,
  warnings, migration, or source/protocol version outcomes.
- Accepted `RFC-0002` defines explicit manifest/lock version readers and says
  incompatible formats require new versions and migration evidence. That
  scoped rule cannot be generalized to the compiler, Semantic Graph, bytecode,
  editor, replay, or evidence surfaces.
- `docs/governance/SCHEMA-LIFECYCLE.md` is Draft. It distinguishes current-only
  readers from N-1 support and requires explicit compatibility edges and
  fixtures; it does not authorize a compiler matrix or migration tool.
- Accepted bytecode RFCs define Experimental 1.0–1.2 readers but explicitly
  make no general release or N-1 promise. The protocol inventory and support
  matrix keep Replay, evidence, package publication, LSP/Zed, formatter,
  profiles, and native targets Future or Unsupported.
- Root `AGENTS.md` requires Accepted authority before semantic/public-protocol
  changes, registered `L-<DOMAIN>-<NUMBER>` diagnostics, original UTF-8 spans,
  Unicode 17.0.0, deterministic/offline behavior, checked Typed Core inputs,
  and no placeholder APIs.

## Evidence in this repository

Current evidence supports only bounded, surface-specific claims:

1. v0.0.1 parser/compiler conformance with deterministic bilingual
   diagnostics, source spans, and Seed semantics;
2. separate Audit Source, Semantic Graph, project/lock, bytecode, VM, and
   cache protocols with their own version markers and Experimental/Future
   states; and
3. lifecycle, error-code, schema, support, and traceability registries that
   reject unsupported stability/N-1 claims.

It does not provide:

- a release/version table mapping every compiler surface to an Accepted
  specification and fixture set;
- compatibility definitions for source bytes, parser/CST/AST, Checked Typed
  Core, diagnostics, Semantic IDs, Audit, bytecode, package/lock, CLI, or
  editors across releases;
- a matrix schema for outcome, warning, migration, rejection, reason, source
  span, replacement, rollback, and toolchain requirements;
- an AST/semantic migration transaction, dry-run/report protocol, or stable
  deprecation and warning lifecycle; or
- N-1 readers, canonical compatibility fixtures, cross-process determinism,
  Unicode 17.0.0, and cross-platform evidence for historical versions.

Generated status or schema snapshots are not a compiler compatibility matrix;
they must not be promoted by copying or relabeling them.

## Required authority before implementation

An accepted 1.0 compatibility decision must define, at minimum:

1. The supported release/version inventory and governing Accepted RFC/decision
   for each language, compiler, diagnostic, Semantic Graph, Audit, bytecode,
   package/lock, replay, evidence, CLI, formatter, LSP/editor, profile,
   target, and backend surface.
2. The matrix schema and deterministic identity for each input/output, including
   source bytes, Unicode version, original UTF-8 spans, toolchain, package,
   profile/target, canonical bytes, Semantic IDs, diagnostics, and provenance.
3. Exact outcome semantics: unchanged acceptance, warning codes and
   suppression, parser/semantic migration, actionable rejection, unknown
   version/field behavior, rollback, no silent reinterpretation, and explicit
   Stable/Preview/Experimental/Unsupported handling.
4. Migration authority and tooling boundaries: parser/checked semantic
   transformation rather than regex, dry-run and machine report, semantic
   diff, stale-input checks, atomic backup/commit, formatter/post-check, and
   human escalation for ambiguous changes.
5. Positive/negative, corruption, migration, differential, deterministic,
   cross-process, cross-platform, Unicode 17.0.0, security, and offline
   fixtures, plus registered bilingual diagnostics and generated protocol,
   schema, support, traceability, and status checks.

## Compatibility and deferred work

This audit changes no source grammar, parser, resolver, evaluator, Semantic ID,
diagnostic, schema, bytecode, package/lock, CLI, formatter, editor,
dependency, or public API behavior. It preserves the active v0.0.1 corpus,
accepted surface-specific protocol versions, Unicode 17.0.0, original UTF-8
spans, checked Typed Core boundaries, and explicit
Experimental/Preview/Future/Unsupported states.

It deliberately adds no compatibility matrix, historical version claim,
warning/rejection diagnostic, migration transaction, reader range, report
schema, deprecation API, dependency, public protocol, or placeholder. Future
implementation remains deferred until release authorities, outcome semantics,
migration rules, and executable cross-version evidence are Accepted.
