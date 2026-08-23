# DOC-6702 Authority Audit

- Task: `DOC-6702` — Two-layer examples
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:406-415`
- Release: G6
- Status: `BlockedSpec`; the Seed example matrix is preparatory evidence.

## Decision

`DOC-6702` remains `BlockedSpec`. The plan requires six example dimensions for
every capability that will be `Stable` in 1.0. The current support matrix has
seven implemented Seed features, all `Experimental`, and no `Stable` feature;
the standard Prelude is `Preview`. It is therefore possible to index the
current Seed examples and negative fixtures, but not to claim completion of a
future 1.0 example set.

`docs/testing/EXAMPLE-COVERAGE.md` records the two layers, command lines,
expected observable output, accepted authorities, conformance evidence, and
explicit deferred boundaries. It adds no language syntax, API, protocol,
profile, ownership rule, backend, or migration behavior.

Accepted `DEC-0046` closes only the bounded `DOC-6702-SEED` child: the
internal `cargo xtask examples verify` command prevents drift in the seven
two-layer requirement rows and seven Seed feature-traceability rows. It does
not run examples or promote the current Experimental/Preview evidence to a
Stable 1.0 support claim.

Accepted `DEC-0239` additionally closes only the bounded
`DOC-6702-EXECUTION-MANIFEST` child. It makes the six checked-in example paths,
roles, expected outputs, and Semantic witnesses a single strict source shared
by the governance verifier and CLI process test. It does not promote the
examples, schema, or features to Stable.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:406-415` is a non-normative checklist. It does
  not authorize a Stable capability or future example syntax.
- `docs/ROADMAP-1.0.md:14-23` requires each stable capability to be backed by
  specification, implementation, conformance, support, and release evidence;
  it forbids implementation-first semantics and unmarked experimental claims.
- `docs/governance/support-matrix.toml` records the seven Seed features as
  `Implemented`/`Experimental`, the Prelude as `Preview`, and future profiles,
  ownership, package, VM CLI, concurrency, Native, device, Critical, and LSP
  surfaces as unavailable or unsupported.
- `docs/traceability/v0.0.1.md`, `docs/SEED-TRACEABILITY.md`, and the
  conformance runner link positive and negative examples to accepted decisions,
  diagnostics, implementation paths, and release evidence.
- `AGENTS.md` requires accepted authority, original UTF-8 spans, Unicode
  17.0.0, bilingual registered diagnostics, deterministic/offline behavior,
  checked Typed Core execution, and no placeholder or stale public names.

## Evidence and gaps

The current matrix covers the minimal hello example, realistic person/ADT/
pipeline examples, Chinese identifiers, `Console.Write` Effect/Capability,
Semantic/Audit output, and the registered negative conformance corpus. It also
records that ownership notes are an explicit Seed exclusion and that no
selectable Profile exists.

The missing release evidence is a future Stable support matrix, feature-specific
profile/target policy, complete cross-platform output fixtures, and examples
for capabilities whose Accepted semantics and implementation do not yet
exist. Those are G1-G5 dependencies, not gaps to fill with guessed syntax.
The matrix verifier checks the inventory, anti-placeholder policy text, and
strict six-case execution manifest. The CLI workspace test executes every
manifest case; this remains current Seed evidence rather than G6 completion.

## Compatibility and deferred work

This audit changes no source semantics, diagnostics, schemas, Semantic IDs,
CLI behavior, package behavior, runtime, editor integration, dependency, or
public API. It preserves `ling`/`.ling`, exact original UTF-8 spans, Unicode
17.0.0, deterministic ordering, and locked offline validation.

The current example outputs and Semantic/Audit protocols remain
Experimental/Preview. A future promotion must add the accepted clauses,
conformance/error corpus, compatibility and migration notes, profile/effect/
ownership boundaries, and release evidence before marking this task complete.
