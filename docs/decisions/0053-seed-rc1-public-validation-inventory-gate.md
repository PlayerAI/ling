# DEC-0053: Seed RC1 public-validation inventory gate / Seed RC1 公开验证盘点门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: release-engineering  
> Related authority/gap: `ROADMAP-1.0`, `DEC-0052`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `RC-6902-SEED` inventory child. It does
not perform or authorize a public RC1 validation, publish an artifact, expose
an installation surface, or claim Ling 1.0 support. The parent `RC-6902`
remains `BlockedSpec` until RC0, G1-G5, and every public-release authority and
executable exit are complete.

## Question

The G6 plan lists nine RC1 public-validation checks, while the repository has
only bounded Seed evidence and explicit unsupported/blocked release surfaces.
How can the project protect the exact RC1 matrix and its no-publication
boundary without inventing artifact, installer, migration, or issue protocols?

## Decision

1. `cargo xtask rc1 verify` is an internal, offline governance command. It
   validates exactly nine RC1 criteria in
   `docs/testing/RC1-PUBLIC-VALIDATION.md` and requires their documented
   `BlockedSpec`, `Unsupported`, or partial Seed/repository states with
   non-empty evidence and exit cells.
2. The verifier checks the explicit no-publication policy and linked Seed,
   RC0, support, security, compatibility, editor, and authority-audit markers.
   It fails closed with internal `GOV-RC1-VALIDATION-*` messages on row, state,
   policy, stale-name, or marker drift.
3. The command validates inventory and historical-audit evidence only. It does
   not create or verify a public artifact, download URL, installer, Zed
   package, signature, SBOM, migration tool, issue form, schema reset, public
   API, or release claim, and performs no network/system mutation.
4. The command is included in the governance-authority CI gate. RC1 promotion
   still requires RC0 and G1-G5 exits, Accepted public protocols, candidate-
   bound artifacts, clean installation, migration fixtures, issue ownership,
   and independent public-validation evidence.

## Conformance plan

- Run `cargo xtask rc1 verify` offline and assert nine criteria with four
  `BlockedSpec`, two `Unsupported`, and three partial states plus eight audit
  files.
- Mutate a criterion row/state, policy phrase, evidence cell, or linked audit
  marker and verify the gate fails closed.
- Run `cargo xtask ci verify` and the locked governance, status, support, and
  traceability checks without treating the inventory as public validation.
- Repeat independent processes and verify that no artifact, package,
  installer, signature, issue form, migration executable, network request, or
  system configuration changes.

## Compatibility impact

- Adds only an internal `cargo xtask` validator, documentation evidence, and
  CI preflight. Ling syntax, semantics, Checked Core, runtime, bytecode,
  diagnostics, schemas, Semantic IDs, dependencies, public protocols, and
  Unicode 17.0.0 behavior are unchanged.
- Existing Seed examples, locked builds, and editor grammar evidence retain
  their current boundaries and are not promoted to RC1 or Stable 1.0 support.

## Unresolved alternatives

Artifact and target identity, checksums/SBOM/provenance/signing, acquisition
and clean-install behavior, Zed packaging, sample manifests, source/schema
migration, issue/security intake, schema-reset change control, and public
support ownership remain governed by RC1 and later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
