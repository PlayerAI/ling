# RC-6902 Authority Audit

- Task: `RC-6902` — RC1 public validation
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:490-500`
- Release: G6
- Status: `BlockedSpec`; no public RC1 is asserted.

## Decision

`RC-6902` remains `BlockedSpec`. The checklist names public-release evidence,
but it does not define artifact formats, acquisition, signing/provenance,
issue taxonomy, migration schemas, candidate identity, or the release-reset
workflow. The repository has bounded v0.0.1 Seed builds, examples, project
fixtures, and editor grammar evidence; it has no public 1.0 artifact or
installation surface to validate.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:490-500` is a non-normative RC checklist and
  does not authorize a package, installer, Zed extension, schema, or migration
  protocol.
- `docs/ROADMAP-1.0.md:540-573` requires accepted compatibility, security,
  offline, platform, support, and independent release evidence before a 1.0
  claim.
- `docs/governance/support-matrix.toml` marks publication/installation and
  LSP/Zed support outside the current supported surface; its generated report
  remains `1.0-draft` with no Tier1 release artifacts.
- `docs/governance/protocol-inventory.toml` has no Stable protocol and no
  acquisition, artifact, evidence-bundle, or migration record for RC1.
- `COMPAT-6502`, `COMPAT-6503`, `COMPAT-6504`, `PKG-6401` through `PKG-6404`,
  `ZED-6801` through `ZED-6804`, and `REL-6603` remain blocked audits for the
  missing public-release authorities.
- `AGENTS.md` requires Accepted authority, checked Typed Core boundaries,
  original UTF-8 spans, Unicode 17.0.0, bilingual diagnostics, deterministic
  offline builds, and no placeholder public APIs or stale legacy names.

## Evidence and gaps

`docs/testing/RC1-PUBLIC-VALIDATION.md` maps all nine RC1 checklist rows to
current evidence, partial Seed evidence, explicit unsupported areas, and the
missing promotion evidence. The status, governance, support, and traceability
gates validate registry consistency only; they do not perform installation,
publication, signing, issue management, or public verification.

The missing evidence includes a candidate-bound multi-platform artifact set,
checksum/signature/SBOM/provenance verification, clean installation, accepted
Zed/LSP packaging, versioned sample projects, general migration tests, issue
templates and response ownership, and a schema-change candidate reset rule.

The bounded `RC-6902-SEED` child is now protected by
`cargo xtask rc1 verify`. That internal command checks the exact nine criteria,
their documented `BlockedSpec`, `Unsupported`, and partial states, the
no-publication boundary, and eight linked audit-marker files. It validates
inventory drift only and does not create any of the missing public-release
evidence.

Accepted `DEC-0246` additionally authorizes the bounded
`RC-6902-CURRENT-EVIDENCE` child. The RC1 verifier now composes the current RC0
and Zed acceptance gates and corrects the stale claim that no LSP executable
exists. The source-built Preview server remains only a prerequisite; no Zed
extension, acquisition surface, debugger integration, or RC1 exit is created.

## Compatibility and deferred work

This audit changes no language semantics, diagnostics, schemas, Semantic IDs,
CLI, package behavior, runtime, editor protocol, dependencies, release tag,
or public API. It preserves `ling`/`.ling`, Unicode 17.0.0, original UTF-8
spans, deterministic/offline validation, and explicit stability boundaries.

No artifact, package, installer, extension, signing key, SBOM, provenance
record, issue form, migration executable, network request, or system
configuration was created. Public RC1 validation, acquisition, migration,
schema-reset, and Stable-support claims remain deferred.
