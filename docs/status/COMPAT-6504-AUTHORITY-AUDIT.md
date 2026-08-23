# COMPAT-6504 Authority Audit

- Task: `COMPAT-6504` — Deprecation Policy
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:307-317`
- Release: G6
- Status: `BlockedSpec`

## Decision

`COMPAT-6504` is `BlockedSpec`. Accepted `DEC-0233` now authorizes a bounded
readiness-evidence child, but not the public policy. The G6 checklist requests a 1.x
compatibility promise, a minimum deprecation period, diagnostic lifecycle,
schema N-1 policy, target/profile support lifecycle, security exceptions, and a
migration-tooling commitment. It does not define the public subjects of
deprecation, version ranges, warning or error behavior, transition timing,
removal rules, replacement obligations, or the authority for any of those
contracts.

Implementing a warning, a deprecation attribute, a compatibility gate, a
schema reader range, a support-state transition, or a migration command would
therefore create new language or public-protocol semantics from a
non-normative checklist. The existing surface-specific registries must remain
explicitly Experimental, Preview, Future, or Unsupported where their accepted
authority and fixtures are not complete.

The completed `COMPAT-6504-READINESS` child records all seven requested areas:
six are `Unavailable`, while diagnostic lifecycle is a `GuardedSubset` limited
to DEC-0001 code non-reuse and retired-code exclusion. This evidence does not
reduce the blocked public-policy scope.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:307-317` is a non-normative release checklist.
  It lists policy topics but does not accept a deprecation schema, lifecycle
  transition matrix, source warning syntax, support guarantee, or migration
  interface.
- `docs/ROADMAP-1.0.md` is planning authority with `stable_basis = false`.
  Its 1.0 final-definition checklist requires published compatibility,
  deprecation, support-cycle, and security-response policies; it does not
  define those policies or authorize an implementation.
- Accepted `docs/decisions/0001-error-code-policy.md` stabilizes diagnostic
  identities, payload types, and non-reuse of retired codes. It does not set a
  general source/API deprecation period or define feature-removal behavior.
- Accepted `DEC-0018` keeps `RFC-0001` Draft and requires a dedicated Accepted
  RFC or decision before post-Seed behavior is implemented. Accepted
  `RFC-0002` scopes migration to manifest/lock versions and does not define a
  general source, package, or protocol deprecation policy.
- `SCHEMA-LIFECYCLE-POLICY` in `docs/governance/authority.toml` is Draft. The
  active `SCHEMA-REGISTRY` depends on that policy and records schema-specific
  evidence; it is not a general N-1 promise. The `SUPPORT-MATRIX` is also
  Draft, and the protocol inventory contains no accepted deprecation
  lifecycle for all public consumers.
- Existing formatter, LSP, package, build, replay, evidence, and editor
  entries are surface-specific and carry explicit stability states. Their
  open gaps do not authorize a shared deprecation field or a new command.
- `AGENTS.md` requires Accepted authority for semantic and public-protocol
  changes, checked Typed Core execution, original UTF-8 spans, bilingual
  registered diagnostics, Unicode 17.0.0, deterministic/offline behavior, and
  no placeholder public APIs.

## Evidence in this repository

The repository currently provides bounded compatibility evidence:

1. `docs/ERROR-CODES.md` and its generated lock preserve diagnostic code
   meanings, payload types, and retired allocations without reusing codes.
2. `schemas/registry.toml` records schema-specific readers, writers, and
   fixtures, while the lifecycle policy required to make N-1 claims remains
   Draft.
3. `docs/governance/support-matrix.toml` and
   `docs/governance/protocol-inventory.toml` expose current states and
   explicit unsupported/future boundaries, not a stable 1.x lifecycle.
4. Accepted source-position, formatter-preservation, and package/lock
   decisions preserve their own byte-span, canonicalization, and migration
   boundaries; none provides a general deprecation API.
5. Accepted `DEC-0233`, `docs/governance/deprecation-readiness.toml`, and
   `cargo xtask deprecation verify` preserve the exact readiness states and
   Draft/absent boundaries without publishing lifecycle semantics.

No accepted artifact currently defines:

- the complete feature, API, diagnostic, schema, protocol, profile, target,
  and package inventory subject to deprecation;
- minimum periods, version/support ranges, warning severity and diagnostic
  facts, replacement/removal rules, or source and CLI behavior;
- schema and protocol reader N-1 guarantees, target/profile support states,
  security exceptions, or a decision process for extending or shortening a
  lifecycle;
- migration-tooling ownership, command/report semantics, editor/formatter
  boundaries, or package/lock consequences; or
- deterministic/offline bilingual fixtures for deprecation, removal,
  migration, N-1, security exceptions, cross-process behavior, and Unicode
  17.0.0 byte-span compatibility.

Adding any of these surfaces now would make an unreviewed compatibility
promise and could silently constrain future accepted semantics.

## Required authority before implementation

An Accepted deprecation and compatibility RFC/decision must define, at
minimum:

1. A complete public-surface inventory with owner, identity, version,
   stability, support state, deprecation eligibility, and replacement/removal
   outcome.
2. Normative deprecation semantics: source/API/protocol/schema/package
   subjects, warning versus error behavior, stable bilingual diagnostics and
   facts, minimum notice period, version ranges, no-reuse rules, and the
   effect on Semantic IDs, canonical bytes, checked Typed Core, and original
   UTF-8 spans.
3. Schema and protocol reader/writer ranges, N-1 compatibility and migration
   rules, target/profile lifecycle states, security-exception authority,
   offline distribution rules, and support-matrix transitions.
4. The migration-tooling contract, including command and report schemas,
   formatter/LSP/editor boundaries, package/lock behavior, exit classes,
   rollback requirements, and human escalation for ambiguous changes.
5. Positive, negative, removal, migration, N-1, security, diagnostics,
   Unicode 17.0.0, byte-span, deterministic/offline, cross-process, and
   cross-platform fixtures, with generated registry and status drift checks.

## Compatibility and deferred work

This audit changes no language grammar, parser, resolver, evaluator,
diagnostic allocation, schema, package/lock behavior, CLI, profile, target,
protocol, dependency, or public API. It preserves the accepted `ling`/`.ling`
names, checked Typed Core boundary, original UTF-8 spans, Unicode 17.0.0,
deterministic/offline requirements, and explicit
Experimental/Preview/Future/Unsupported states.

It deliberately adds no deprecation attribute, warning or rejection
diagnostic, compatibility flag, schema reader range, support transition,
security exception path, migration executable, dependency, or placeholder.
`COMPAT-6504` remains deferred until the lifecycle authority and executable
compatibility evidence are Accepted.
