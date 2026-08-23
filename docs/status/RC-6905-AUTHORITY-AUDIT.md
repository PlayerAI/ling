# RC-6905 Authority Audit

- Task: `RC-6905` — v1.0 release artifacts
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:524-542`
- Release: G6
- Status: `BlockedSpec`; no v1.0 publication is asserted.

## Decision

`RC-6905` remains `BlockedSpec`. The plan lists the desired v1.0 publication
set, but it does not authorize artifact formats, packaging, signatures,
SBOM/provenance, standard-library stability, Zed/LSP distribution, migration,
support commitments, or an evidence-bundle schema. The repository has a
published v0.0.1 Seed tag and substantial implementation evidence; it has no
complete v1.0 artifact set and must not promote Seed or Experimental records
to Stable by inventory alone.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:524-542` is a non-normative release checklist;
  it does not define any public artifact or evidence protocol.
- `docs/ROADMAP-1.0.md:540-573` requires accepted Stable scope, traceability,
  Tier1 support, compatibility, security, offline reproducibility, and
  candidate identity before a 1.0 claim.
- `docs/SEED-RELEASE-REPORT.md` and the remote `v0.0.1` tag are immutable Seed
  evidence; they do not satisfy the v1.0 publication list.
- `docs/governance/support-matrix.toml` keeps the support report at
  `1.0-draft`, marks profiles unavailable and hosts Tier2 without release
  artifacts, and explicitly excludes package publication and Zed/LSP support.
- `docs/governance/protocol-inventory.toml` records 27 non-Stable/current or
  Future protocols; `PROTO-EVIDENCE` is Future with no schema or fixtures.
- `docs/status/COMPAT-6502-AUTHORITY-AUDIT.md` through
  `COMPAT-6504-AUTHORITY-AUDIT.md`, the PKG/REL/ZED/RC audits, and
  `AGENTS.md` prohibit inventing migration, package, security, editor, or
  release-protocol behavior without Accepted authority.

## Evidence and gaps

`docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md` maps every item in the plan's
v1.0 publication list to current Seed/Experimental/Preview/Future/Unsupported
evidence and the required promotion evidence. Local status, governance,
support, and traceability gates validate repository consistency only; they do
not publish or sign artifacts.

The missing evidence includes a v1.0 source tag and candidate manifest,
compiler/runtime artifacts, checksums/signatures, SBOM/provenance, stable
standard library, Zed/LSP packages, complete reference and migration manuals,
final support matrix, Stable protocol schemas/golden corpus, Tier1
conformance, security policy, and a versioned independent evidence bundle.

The bounded `RC-6905-SEED` child is now protected by
`cargo xtask v1 verify`. That internal command checks the exact fourteen
release items, their documented non-Stable/blocked states, the immutable-
Seed/no-publication boundary, and nine linked audit-marker files. It validates
inventory drift only and does not create a v1.0 manifest, artifact, or Stable
claim.

Accepted `DEC-0249` additionally authorizes the bounded
`RC-6905-CURRENT-EVIDENCE` child. The v1 verifier composes the current
RC2→RC3→RC1→RC0 chain, corrects the protocol total from 21 to 27, and replaces
the false no-LSP-executable statement with the precise source-built Preview
server boundary. No distributable LSP artifact or release exit is created.

## Compatibility and deferred work

This audit changes no language semantics, diagnostics, schemas, Semantic IDs,
CLI, package behavior, runtime, editor protocol, dependencies, release tag,
or public API. It preserves the immutable v0.0.1 Seed evidence, `ling`/`.ling`,
Unicode 17.0.0, original UTF-8 spans, deterministic/offline validation, and
explicit stability boundaries.

No v1.0 tag, artifact, installer, extension, language server, signature,
SBOM, provenance record, migration executable, evidence bundle, network
request, or system configuration was created. Publication remains deferred
until the preceding RC gates and all required Accepted authorities exist.
