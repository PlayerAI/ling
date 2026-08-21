# COMPAT-6503 Authority Audit

- Task: `COMPAT-6503` — Language Migration Tool
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:293-305`
- Release: G6
- Status: `BlockedSpec`

## Decision

`COMPAT-6503` is `BlockedSpec`. The G6 checklist requires migration based on
the parser and a semantic transaction rather than regular expressions, a dry
run, semantic diff, stale-edit checks, backup/transactional writes, formatter
integration, post-check/test, machine-readable reporting, and human choice
when automation is ambiguous. These requirements are not an accepted command,
transaction, migration-schema, or source-version contract.

The plan's original command label is a stale legacy placeholder. The current
authority fixes the public CLI as `ling` and the source extension as `.ling`;
the placeholder must not enter implementation, fixtures, schemas, or editor
integration. Renaming the tracking title does not authorize a migration tool.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:293-305` is a non-normative migration checklist.
  It does not define source versions, grammar changes, AST/CST mapping,
  semantic identity, edit transaction fields, command spelling, exit classes,
  report schema, or compatibility/migration lifecycle.
- `ROADMAP-1.0` §11-§12 requires Accepted authority, traceability, and
  reproducible evidence for release work. It does not accept a migration
  command or semantic-edit protocol; `ROADMAP-1.0` has `stable_basis = false`.
- Accepted `DEC-0002` fixes UTF-8 byte spans and line/column projection.
  Accepted `DEC-0015` fixes Audit Source canonicalization and round-trip; it is
  not a source migration API. Accepted `DEC-0023` defines Author Source
  preservation/idempotence and explicitly adds no formatter CLI, LSP command,
  JSON report, localized view, or public stability claim.
- Accepted `RFC-0002` defines only manifest/lock version migration rules and
  preserves file-oriented Seed behavior when no manifest is selected. It does
  not define source-language migration, Semantic Transaction, formatter CLI,
  or editor application semantics.
- Open `GAP-FORMATTER-AUTHOR-SOURCE-001`, `GAP-FORMATTER-CLI-PROTOCOL-001`,
  `GAP-AUTHOR-SOURCE-LOCALIZATION-001`, `GAP-LSP-TRANSACTION-PROTOCOL-001`,
  and `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave formatter/edit/report,
  localized source, snapshot preconditions, and semantic-mutation boundaries
  unresolved. `UNSUP-LSP-EDITOR` records the corresponding unsupported area.
- Accepted `DEC-0019` and `DEC-0022` authorize internal query/cache boundaries,
  not a source-edit transaction, migration command, or public cache/report
  protocol. `DEC-0001` governs diagnostic-code compatibility only.
- Root `AGENTS.md` requires accepted authority before semantic or public CLI/
  protocol changes, checked Typed Core input, original UTF-8 spans, Unicode
  17.0.0, deterministic/offline behavior, bilingual registered diagnostics,
  and no stale legacy command or placeholder API.

## Evidence in this repository

The current compiler and formatter evidence supports bounded operations:

1. source bytes retain original UTF-8 spans, including CRLF and BOM cases;
2. the parser/CST/compiler pipeline is the semantic authority and evaluation
   consumes checked Typed Core rather than unresolved AST/HIR;
3. Audit Source has an accepted canonical renderer/reader and round-trip
   boundary; Author Source formatting has accepted preservation/idempotence
   rules but no public command or edit transport; and
4. diagnostics, Semantic IDs, and package/lock protocols have separate,
   surface-specific registries and stability states.

It does not provide:

- accepted source-language version pairs and a complete change/removal map;
- a typed CST/AST/Checked Core migration transformation with semantic-ID,
  effect/capability, package, and source-span preservation rules;
- snapshot/version preconditions, Workspace Edit or Semantic Transaction
  protocol, formatter CLI, machine report schema, exit-code mapping, backup
  and rollback semantics, or human-escalation behavior; or
- positive/negative/migration/ambiguous-choice fixtures and independent
  cross-process, Unicode 17.0.0, CRLF, and stale-edit evidence.

Adding a migration binary, regex rewrite, edit schema, command alias, or
placeholder report would create irreversible compatibility behavior without an
accepted source version or transaction oracle.

## Required authority before implementation

An accepted migration and semantic-edit RFC/decision must define, at minimum:

1. Source/compiler version inventory, supported compatibility outcomes, exact
   syntax and semantic changes, deprecation reasons, and the governing
   Accepted RFC/decision for every migration case.
2. Parser/CST/Checked Core transformation rules, identity/effect/capability
   preservation, package/lock impact, canonical bytes, Unicode 17.0.0,
   original UTF-8 spans, and explicit no-regex/no-heuristic boundaries.
3. The `ling` command, manifest/path selection, dry-run and check modes,
   stdout/stderr, exit classes, human/JSON report schema, stable bilingual
   `L-<DOMAIN>-<NUMBER>` diagnostics, and path/environment redaction.
4. Snapshot identity and stale-edit preconditions, semantic transaction or
   LSP Workspace Edit projection, backup/atomic commit/rollback, formatter
   preservation, post-check/test, cancellation, and human-choice protocol.
5. Deterministic/offline positive, negative, migration, ambiguity,
   corruption, stale-edit, crash/interruption, cross-process, cross-platform,
   Unicode, package/lock, Semantic ID, and diagnostic fixtures, with generated
   protocol/schema/support/traceability/status drift checks.

## Compatibility and deferred work

This audit changes no source grammar, parser, resolver, evaluator, formatter,
Audit Source, Semantic ID, diagnostic, schema, package/lock, CLI, editor,
dependency, or public API behavior. It preserves the accepted `ling`/`.ling`
names, checked Typed Core boundary, original UTF-8 spans, Unicode 17.0.0,
deterministic/offline requirements, and explicit
Experimental/Preview/Future/Unsupported states.

It deliberately adds no migration executable, command alias, regex rewrite,
semantic transaction, LSP edit, formatter CLI, report schema, warning or
rejection diagnostic, backup/rollback protocol, dependency, public API, or
placeholder. Future migration remains deferred until source-version authority,
semantic/edit transaction rules, and executable migration evidence are
Accepted.
