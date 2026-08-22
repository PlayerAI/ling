# DEC-0056: Seed v1 release-artifact inventory gate / Seed v1 发布物盘点门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: release-engineering  
> Related authority/gap: `ROADMAP-1.0`, `DEC-0055`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `RC-6905-SEED` inventory child. It does
not perform or authorize a v1.0 release, publish an artifact, define an
acquisition protocol, or promote any capability to Stable. The parent
`RC-6905` remains `BlockedSpec` until all release authorities and executable
evidence exist.

## Question

The G6 plan lists fourteen v1.0 publication items, while the repository has a
published v0.0.1 Seed tag and bounded Experimental/Preview/Future evidence but
no complete v1.0 artifact set. How can the project protect the exact
publication inventory without inventing packaging, signing, migration,
support, or evidence protocols?

## Decision

1. `cargo xtask v1 verify` is an internal, offline governance command. It
   validates exactly fourteen release items in
   `docs/testing/V1-RELEASE-ARTIFACT-INVENTORY.md` and requires their
   documented partial, unavailable, unsupported, draft, experimental, preview,
   or `BlockedSpec` states with non-empty evidence and exit cells.
2. The verifier checks the immutable-Seed/no-publication policy and linked
   RC0–RC4, support, protocol, Seed-release, and authority-audit markers. It
   fails closed with internal `GOV-V1-ARTIFACT-*` messages on row, state,
   policy, stale-name, or marker drift.
3. The command validates inventory and historical-audit evidence only. It does
   not build, sign, upload, download, install, or advertise a v1.0 artifact;
   define schemas, migration, standard-library, Zed/LSP, security, or support
   protocols; or perform network/system mutation.
4. The command is included in the governance-authority CI gate. RC-6905
   promotion still requires RC0–RC4, Stable support/protocol authority,
   reproducible artifacts, signatures/SBOM/provenance, documentation,
   security, migration, and independent evidence-bundle verification.

## Conformance plan

- Run `cargo xtask v1 verify` offline and assert fourteen items with five
  partial Seed, two unavailable, two unsupported, one preview/not packaged,
  one draft, one experimental/preview/future, and two BlockedSpec states.
- Mutate an artifact row/state, policy phrase, evidence cell, or linked audit
  marker and verify the gate fails closed.
- Run `cargo xtask ci verify` and the locked governance, status, support, and
  traceability checks without treating the inventory as a release manifest.
- Repeat independent processes and verify that no artifact, package,
  signature, network request, system configuration, or Stable claim changes.

## Compatibility impact

- Adds only an internal `cargo xtask` validator, documentation evidence, and
  CI preflight. Ling syntax, semantics, Checked Core, runtime, bytecode,
  diagnostics, schemas, Semantic IDs, dependencies, public protocols, and
  Unicode 17.0.0 behavior are unchanged.
- The v0.0.1 Seed tag and reports remain immutable historical evidence;
  Experimental, Preview, Future, Draft, and Unsupported rows are not promoted.

## Unresolved alternatives

Source/candidate identity, compiler/runtime packaging, checksums/signatures,
SBOM/provenance, standard-library publication, Zed/LSP distribution,
reference/migration manuals, final support matrix, Stable protocol corpus,
Tier1 conformance, security policy, and evidence-bundle schema remain governed
by RC-6905 and later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
