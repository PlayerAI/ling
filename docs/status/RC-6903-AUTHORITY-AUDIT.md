# RC-6903 Authority Audit

- Task: `RC-6903` — independent verification
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:502-512`
- Release: G6
- Status: `BlockedSpec`; no independent Go decision is asserted.

## Decision

`RC-6903` remains `BlockedSpec`. The plan requires a reviewer or team that did
not implement the candidate, but it does not define candidate identity,
independence/conflict rules, evidence retention, artifact verification,
toolchain capture, sign-off format, or rerun policy. The repository can run
self-validation for the v0.0.1 Seed boundary; that is not independent RC3
evidence and must not be labeled as such.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:502-512` is a non-normative checklist and does
  not authorize a release tag, evidence-bundle schema, reviewer identity, or
  Go/No-Go protocol.
- `docs/ROADMAP-1.0.md:565-573` requires executable conformance, security and
  compatibility evidence, release identity consistency, and independent
  candidate verification before a 1.0 claim.
- `docs/status/RC-6901-AUTHORITY-AUDIT.md` and
  `docs/status/RC-6902-AUTHORITY-AUDIT.md` keep RC0/RC1 blocked because the
  candidate, artifact, support, installation, migration, and security
  authorities are absent.
- `docs/SEED-RELEASE-REPORT.md` records v0.0.1 candidate and CI evidence, but
  it does not establish an independent v1.0 reviewer or Stable release.
- `docs/testing/SECURITY-AUDIT.md`, the support matrix, and protocol inventory
  preserve partial Seed controls and non-Stable protocols without claiming a
  complete independent security or release review.
- `AGENTS.md` requires accepted authority, deterministic/offline evidence,
  original UTF-8 spans, Unicode 17.0.0, bilingual diagnostics, and no
  placeholder public APIs or stale legacy names.

## Evidence and gaps

`docs/testing/RC3-INDEPENDENT-VERIFICATION.md` maps all seven RC3 checks to
current evidence, partial Seed evidence, and missing independent evidence.
Repository status, governance, support, and traceability gates validate local
consistency only; they do not establish reviewer independence, candidate
immutability, artifact provenance, or an external sign-off.

The missing evidence includes an immutable candidate tag and manifest,
independent clean build, artifact/provenance verification, candidate-wide
conformance and corruption corpus, TCB/unsafe/FFI review, reproducible
representative evidence, and a signed tag/hash/manifest comparison.

The bounded `RC-6903-SEED` child is now protected by
`cargo xtask rc3 verify`. That internal command checks the exact seven checks,
their documented `BlockedSpec` or partial Seed states, the
no-independent-sign-off boundary, and seven linked audit-marker files. It
validates inventory drift only and does not create any independent evidence or
Go/No-Go decision.

## Compatibility and deferred work

This audit changes no language semantics, diagnostics, schemas, Semantic IDs,
CLI, package behavior, runtime, editor protocol, dependencies, release tag,
or public API. It preserves `ling`/`.ling`, Unicode 17.0.0, original UTF-8
spans, deterministic/offline validation, and explicit stability boundaries.

No tag, artifact, evidence bundle, reviewer identity, signature, issue status,
network request, system configuration, or independent Go decision was created.
Independent verification and all Stable-release claims remain deferred.
