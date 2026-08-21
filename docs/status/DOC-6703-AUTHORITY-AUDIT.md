# DOC-6703 Authority Audit

- Task: `DOC-6703` — Tutorial and Chinese-first examples
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:417-426`
- Release: G6
- Status: `BlockedSpec`; the bilingual Seed tutorial is preparatory evidence.

## Decision

`DOC-6703` remains `BlockedSpec` because the current support matrix has no
`Stable` feature and G6 still depends on the G1-G5 exits. The repository now
contains a runnable Chinese-first tutorial and an equivalent English tutorial,
but both are explicitly scoped to v0.0.1 Seed and its Experimental/Preview
tooling protocols.

The two source files use idiomatic domain identifiers instead of mechanically
translating names. The bilingual tutorial explains checked execution,
Chinese identifiers, `Console.Write` Effect/Capability, correct errors, and
the unavailable Profile/ownership/runtime boundaries without defining new
language behavior.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:417-426` is a non-normative tutorial checklist;
  it does not authorize new syntax, localized aliases, or a Stable release.
- `docs/LANGUAGE.md`, `docs/SEMANTICS.md`, accepted Seed decisions, and the
  conformance fixtures authorize the syntax and semantics used by the two
  tutorial sources.
- `docs/governance/support-matrix.toml` records Seed as `Experimental`,
  Semantic/Audit protocols as Experimental/Preview, and future Profile,
  ownership, package, concurrency, Native, device, Critical, LSP, and Zed
  surfaces as unavailable or unsupported.
- `AGENTS.md` requires `ling`/`.ling`, bilingual registered diagnostics,
  Unicode 17.0.0, original UTF-8 spans, checked Typed Core execution,
  deterministic/offline builds, and no placeholder or stale public names.

## Evidence

- `examples/tutorial-zh.ling` is a Chinese-first runnable record/mutation
  example and prints `存活`.
- `examples/tutorial-en.ling` is the semantically equivalent English example
  with idiomatic `Person`, `health`, `takeDamage`, and `statusText` names; it
  prints `alive`.
- `docs/TUTORIAL.md` gives bilingual copyable commands, Semantic/Audit output,
  the registered missing-Capability negative fixture, and explicit boundaries.
- `crates/ling-cli/tests/conformance.rs` includes both tutorial files in the
  process-level check/run/Semantic matrix; the existing Audit determinism test
  and Seed traceability registry provide protocol evidence.

## Compatibility and deferred work

This audit and the tutorial add no grammar rule, diagnostic allocation,
schema, Semantic ID rule, CLI command, package behavior, runtime feature,
editor protocol, dependency, or public API. The new sources exercise only the
existing checked Seed path and preserve Unicode 17.0.0 and original UTF-8
spans.

A future promotion requires accepted localization/alias policy where needed,
Stable support-matrix entries, positive and negative fixtures for each stable
capability, cross-platform reproducibility, migration/compatibility guidance,
and release evidence. Until then, the tutorial must not imply Profile,
ownership, Native/FFI, concurrency, package, LSP, or Zed support.
