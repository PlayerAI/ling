# RC-6905 v1.0 Release Artifact Inventory

Status: `BlockedSpec` (2026-08-22). This inventory records the required v1.0
publication set and current evidence; it is not a release manifest, download
index, signature, or Stable-support claim.

## Artifact matrix

| Required release item | Current repository evidence | State | Required before v1.0 publication |
| --- | --- | --- | --- |
| Source tag | Remote `v0.0.1` is an annotated Seed tag at the documented candidate; no v1.0 tag exists. | Partial Seed evidence | Immutable v1.0 source tag and candidate identity verified by RC3. |
| Compiler/runtime artifacts | Offline release builds and VM library artifacts are covered by Seed/implementation reports; no v1.0 downloadable compiler/runtime set is published. | Partial Seed evidence | Per-host, versioned artifacts with target metadata, reproducible build inputs, and manifest entries. |
| Checksums/signatures | No v1.0 artifact manifest, checksum set, signature, or trust-root protocol exists. | Unavailable | Checksums, signatures, verification tooling, rotation/rollback rules, and fixtures. |
| SBOM/licenses/provenance | `docs/DEPENDENCIES.md` and Seed license inventories are engineering evidence; no generated v1.0 SBOM/provenance bundle exists. | Partial Seed evidence | Versioned SBOM, license report, provenance attestation, signing identity, and independent verification. |
| Standard library | `Ling.Prelude` is builtin-only `0.0.1-dev` with Preview stability and no package publication. | Preview / not packaged | Accepted stable symbol/effect/capability surface, versioned package, profile limits, and conformance. |
| Zed extension | Tree-sitter grammar/query assets exist; ZED audits record no Zed extension package or marketplace artifact. | Unsupported | Accepted extension package, metadata/license, installation and compatibility fixtures. |
| Language server | No LSP executable, acquisition manifest, or public LSP protocol is present. | Unsupported | Accepted LSP lifecycle/position/schema, signed per-platform binaries, discovery, and crash/restart evidence. |
| Reference documentation | `LANGUAGE.md`, `TUTORIAL.md`, examples, and inventories document Seed; a complete 1.0 reference/tool/package/migration set is absent. | Partial Seed evidence | Bilingual 1.0 manuals linked to accepted clauses, symbols, diagnostics, fixtures, and support limits. |
| Migration guide | COMPAT-6502 through COMPAT-6504 remain blocked; RFC-0002 only covers bounded manifest/lock evolution. | BlockedSpec | Accepted source/protocol migration policy, tooling, diagnostics, rollback, and versioned fixtures. |
| Support matrix | Generated support report is `1.0-draft`; all hosts are Tier2 without release artifacts, profiles are unavailable, and no Stable feature is listed. | Draft | Final feature/profile/target matrix with Tier1 evidence, limitations, and review. |
| Protocol schemas/golden corpus | 21 protocol records and many current golden/corruption fixtures exist, but none is Stable and `PROTO-EVIDENCE` is Future. | Experimental / Preview / Future | Stable schemas, compatibility/N-1 policy, golden/corrupt corpus, migration, and release binding. |
| Conformance suite | v0.0.1 Seed conformance is executable and traced; no v1.0 Stable scope or Tier1 matrix is accepted. | Partial Seed evidence | Candidate-wide Stable conformance suite, host runs, negative/security cases, and independent results. |
| Security policy | Security audit documents Seed controls and gaps; no accepted 1.0 threat/disclosure/response policy or complete SBOM/advisory result exists. | BlockedSpec | Accepted security policy, threat model, advisory/license scan, disclosure SLA, and response fixtures. |
| Release evidence bundle | `PROTO-EVIDENCE` is Planned public/Future with no schema, reader, writer, or fixtures; Markdown reports are not a public bundle. | Unavailable | Versioned evidence schema, provenance, checksums, test/proof links, redaction, independent verifier, and manifest identity. |

## Publication boundary

The v0.0.1 Seed tag and reports must remain immutable historical evidence. A
v1.0 publication may not reuse the Seed tag, silently promote Experimental or
Preview protocols, or imply support for Unsupported Zed/LSP/package surfaces.
Every artifact must identify the same source candidate, toolchain, Unicode
17.0.0 data, protocol versions, and evidence bundle; any mismatch requires a
new candidate and repeat of the affected RC gates.

## Verification boundary

These commands validate current repository registries and Seed traceability;
they do not create or publish v1.0 artifacts:

```text
cargo xtask v1 verify
cargo run -p xtask --locked --offline -- status verify
cargo run -p xtask --locked --offline -- governance check-all
cargo run -p xtask --locked --offline -- support verify
cargo run -p xtask --locked --offline -- traceability verify --release v0.0.1
```

`cargo xtask v1 verify` deterministically checks the exact fourteen release
items, their partial/unavailable/unsupported/draft/experimental/blocked
states, the no-publication boundary, and nine linked audit-marker files. It
is an inventory check only; it does not build, sign, upload, install, or
advertise a v1.0 release.

No package upload, installer, signing service, release tag, network request,
or system configuration was exercised by this inventory.

## Promotion rules

RC-6905 may leave `BlockedSpec` only after RC0 through RC4 are complete, the
v1.0 support matrix and protocols are Stable where claimed, all artifact rows
have reproducible evidence, and independent verification binds the source
tag, manifests, digests, signatures, SBOM/provenance, documentation, and
evidence bundle. Missing optional capabilities must be explicitly excluded or
deferred rather than represented by empty placeholder artifacts.

No placeholder command, download, package, schema, protocol, artifact,
migration promise, or stale legacy name is added here.
