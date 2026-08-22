# RC-6904 Authority Audit

- Task: `RC-6904` — RC2 / Final change control
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:514-522`
- Release: G6
- Status: `BlockedSpec`; no RC2 or Final candidate is asserted.

## Decision

`RC-6904` remains `BlockedSpec`. The plan says that only blocker fixes may be
accepted and lists six required evidence classes, but it does not define a
blocker taxonomy, candidate manifest, change-control protocol, risk schema,
impact/version rules, reviewer approval, or candidate regeneration process.
The current Seed tests and audits cannot be promoted to an RC2 baseline.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:514-522` is a non-normative checklist; it does
  not authorize any change, tag, release artifact, protocol version, or risk
  decision.
- `docs/ROADMAP-1.0.md:565-573` requires closed correctness/security/data-loss
  blockers, stable support/protocol evidence, release identity consistency,
  and independent verification before a 1.0 claim.
- `docs/status/RC-6901-AUTHORITY-AUDIT.md`, `RC-6902-AUTHORITY-AUDIT.md`, and
  `RC-6903-AUTHORITY-AUDIT.md` keep the predecessor candidate gates blocked;
  RC2 cannot be entered without them.
- The support and protocol registries remain draft/non-Stable and the
  implementation ledger has no accepted blocker disposition or candidate
  manifest.
- `AGENTS.md` requires Accepted authority, deterministic/offline evidence,
  original UTF-8 spans, Unicode 17.0.0, bilingual diagnostics, and no
  placeholder public APIs or stale legacy names.

## Evidence and gaps

`docs/testing/RC2-FINAL-CHANGE-CONTROL.md` maps the six required evidence
classes and the allowed/forbidden change boundary. Local status, governance,
support, and traceability gates validate registry consistency only; they do not
approve a blocker, risk, protocol impact, candidate identity, or Final result.

The missing evidence includes an accepted blocker/P0/P1 registry, candidate
baseline, regression and risk schemas, protocol/artifact impact manifest, full
matrix rerun, immutable candidate identity, and reviewer approval for each
change.

The bounded `RC-6904-SEED` child is now protected by
`cargo xtask rc2 verify`. That internal command checks the exact six evidence
classes, their documented `BlockedSpec` or partial Seed states, the
blocker-only/no-claim boundary, and seven linked audit-marker files. It
validates inventory drift only and does not approve a blocker, create a
candidate, or produce a Final result.

## Compatibility and deferred work

This audit changes no language semantics, diagnostics, schemas, Semantic IDs,
CLI, package behavior, runtime, editor protocol, dependencies, release tag,
or public API. It preserves `ling`/`.ling`, Unicode 17.0.0, original UTF-8
spans, deterministic/offline validation, and explicit stability boundaries.

No source fix, tag, release artifact, blocker disposition, risk acceptance,
network request, system configuration, or Final/Go decision was created. RC2
change acceptance and RC-6905 publication remain deferred.
