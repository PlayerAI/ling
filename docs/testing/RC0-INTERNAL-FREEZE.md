# RC-6901 RC0 Internal Freeze

Status: `BlockedSpec` (2026-08-23). This document audits the RC0 gate; it
does not assert that an RC0 freeze has happened and does not create a release
tag, artifact, protocol, or feature commitment.

## Boundary

The G6 plan defines RC0 as an internal release-candidate gate with eight
checks: feature freeze, protocol-freeze candidate, final support-matrix draft,
P0/P1 triage, historical-corpus execution, security scan, release-artifact
rehearsal, and documentation completeness. A release gate cannot authorize
language semantics or public APIs. The repository therefore keeps this task
blocked until the G1-G5 exits, accepted release authorities, and executable
evidence exist.

## Gate matrix

| RC0 criterion | Current evidence | State | Required exit evidence |
| --- | --- | --- | --- |
| Feature freeze | The status registry has 499 tasks and 300 `Done` tasks; seven tracked features still have stabilization blockers, and G1-G5 release work is not closed. | BlockedSpec | Accepted 1.0 support scope, change-control policy, candidate commit identity, and a reviewed freeze record. |
| Protocol-freeze candidate | The protocol inventory has 28 records: 0 Stable, 13 Experimental, 10 Preview, 1 Internal, and 4 Future. | BlockedSpec | Accepted versions, reader/unknown-field/migration rules, golden and corrupt-input fixtures, and an inventory revision tied to the candidate. |
| Support-matrix draft final | `docs/governance/support-matrix.md` is explicitly `1.0-draft`; all three hosts are Tier2 with no release artifacts, profiles are unavailable, and no Tier1 target is claimed. | BlockedSpec | Accepted feature/profile/target matrix, Tier1 artifact evidence, limitations, and support-matrix review record. |
| P0/P1 triage | No repository-authoritative P0/P1 issue and disposition registry proves that correctness, security, and data-loss blockers are closed; unresolved G6 tasks remain in the ledger. | BlockedSpec | Complete issue inventory, severity/owner/disposition, regression links, and an independently reviewed no-open-blocker decision. |
| Historical corpus run | `COMPAT-6501` remains blocked: only v0.0.1 Seed fixtures have accepted authority; no cross-version corpus manifest or migration/equivalence policy exists. | BlockedSpec | Versioned corpus manifest, provenance/checksums, expected outcomes, migration rules, and reproducible run logs. |
| Security scan | `REL-6603` and `docs/testing/SECURITY-AUDIT.md` record Seed controls, but no accepted threat model, advisory/license result, SBOM, provenance, or disclosure process is complete. | BlockedSpec | Locked dependency advisory/license scan, SBOM and provenance outputs, threat-model review, and incident-response ownership. |
| Release-artifact rehearsal | `docs/SEED-RELEASE-REPORT.md` closes the v0.0.1 Seed candidate only. No v1.0 clean-tag artifact manifest, signature/checksum set, or evidence bundle exists. | BlockedSpec | Clean-tag locked/offline build, per-host artifacts, manifest/checksum/signature verification, and repeatable install evidence. |
| Documentation completeness | DOC-6701 through DOC-6703 inventory and tutorial evidence cover the implemented Seed boundary; they do not constitute complete 1.0 reference, package, migration, security, or support manuals. | BlockedSpec | Bilingual 1.0 documentation inventory linked to accepted clauses, implementation symbols, fixtures, compatibility limits, and release state. |

## Verification boundary

The following offline gates validate the registry and the current negative
boundary without claiming RC0 completion:

```text
cargo xtask rc0 verify
cargo run -p xtask --locked --offline -- status verify
cargo run -p xtask --locked --offline -- governance check-all
cargo run -p xtask --locked --offline -- support verify
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
```

`cargo xtask rc0 verify` deterministically checks the exact eight criteria,
their `BlockedSpec` states, the no-freeze/no-publication policy, the ten linked
release-audit marker files, and the current status/protocol registry summaries.
It is an inventory check only; it does not
execute a freeze, release build, security scan, issue-tracker operation, or
artifact publication.

No release tag was created, no package or artifact was published, no issue
tracker was changed, and no external security or platform result is inferred
from these checks. A release build or clean-install rehearsal is not reported
as RC0 evidence until a candidate scope and artifact protocol are accepted.

## Exit and promotion rules

RC-6901 may leave `BlockedSpec` only when every row has an Accepted authority,
an identified owner, deterministic/offline executable evidence, and a
candidate identity that can be independently verified. RC0 must not freeze
unsupported or Experimental capabilities, and it must not relabel the current
Seed report as a v1.0 release. Any scope change after a freeze requires a new
candidate identity, impact analysis, regression evidence, and review.

No placeholder command, schema, protocol, artifact, issue status, migration
promise, or stale legacy name is added here.
