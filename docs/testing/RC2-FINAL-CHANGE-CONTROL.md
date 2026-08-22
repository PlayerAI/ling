# RC-6904 RC2 / Final Change Control

Status: `BlockedSpec` (2026-08-22). This document records the proposed
blocker-only change boundary; it does not declare an RC2, Final candidate, Go
decision, or source freeze.

## Boundary

The G6 plan permits only blocker fixes at RC2/Final. Every accepted fix must
have a regression test, risk analysis, affected protocol/artifact analysis, a
full relevant support matrix, and a new release-candidate identity. No
accepted blocker taxonomy, candidate manifest, evidence bundle, or change-
control authority currently exists, so no change may be classified as an RC2
fix from this document alone.

## Change-control matrix

| Required RC2 evidence | Current repository state | State | Required before an RC2 change |
| --- | --- | --- | --- |
| Blocker-only scope | G1-G5 exits and RC0/RC1/RC3 remain blocked; no authoritative P0/P1/blocker registry or severity-to-release rule exists. | BlockedSpec | Accepted blocker taxonomy, owner/disposition record, and proof that the change is release-blocking rather than feature work. |
| Regression test | Seed conformance, schema, bytecode, and governance tests exist, but no immutable candidate-specific regression baseline exists. | Partial Seed evidence | Reproducible regression fixture linked to the exact affected behavior and candidate. |
| Risk analysis | Existing audits describe risks and deferred work; no RC2 risk record, accepted severity, or residual-risk approval exists. | BlockedSpec | Structured risk/impact record, assumptions, mitigations, rollback, and independent review. |
| Affected protocol/artifact analysis | The 21-record protocol inventory and support matrix describe current boundaries; no candidate artifact/protocol manifest binds a change to release outputs. | BlockedSpec | Version/schema/diagnostic/Semantic ID/span/artifact impact map and compatibility decision. |
| Full relevant matrix | The support report remains `1.0-draft`; hosts are Tier2 without release artifacts and public protocols are not Stable. | BlockedSpec | Re-run all affected host/profile/backend/protocol/conformance/security checks and record unsupported rows explicitly. |
| New candidate identity | No RC2 tag, source hash, release manifest, or evidence-bundle identity exists. | BlockedSpec | Immutable candidate identity, checksums, provenance, and a complete predecessor-to-candidate diff. |

## Allowed and forbidden changes

Until the authority exists, no feature, syntax, protocol, diagnostic meaning,
schema field, dependency, backend, editor package, migration promise, or
support tier may be added under the RC2 label. A future accepted RC2 fix must
be minimal, reviewable, failure-atomic, and independently reproducible; a
non-blocker or scope expansion must return to release planning and create a
new candidate process rather than bypassing the gate.

## Verification boundary

These offline commands validate repository consistency only; they do not
approve a change or produce a candidate:

```text
cargo xtask rc2 verify
cargo run -p xtask --locked --offline -- status verify
cargo run -p xtask --locked --offline -- governance check-all
cargo run -p xtask --locked --offline -- support verify
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
```

`cargo xtask rc2 verify` deterministically checks the exact six evidence
classes, their `BlockedSpec` or partial Seed states, the blocker-only/no-claim
boundary, and seven linked audit-marker files. It is an inventory check only;
it does not approve a blocker, create a candidate, or produce a release.

No tag, release manifest, artifact, issue disposition, network request, or
system configuration was created by this audit.

## Promotion rules

RC-6904 may leave `BlockedSpec` only after RC0, RC1, and independent
verification are complete; a blocker registry and candidate identity are
accepted; and every change has the required regression, risk, impact, matrix,
and reviewer evidence. Each accepted change invalidates the predecessor
candidate and produces a new identity. Final/Go claims require the separate
RC-6905 publication gate.

No placeholder command, tag, artifact, schema, protocol, blocker status, or
stale legacy name is added here.
