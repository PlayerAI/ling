# DEC-0055: Seed RC2/final change-control inventory gate / Seed RC2/Final 变更控制盘点门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: release-engineering  
> Related authority/gap: `ROADMAP-1.0`, `DEC-0054`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `RC-6904-SEED` inventory child. It does
not approve a blocker, create an RC2 or Final candidate, freeze source, or
make a Go decision. The parent `RC-6904` remains `BlockedSpec` until blocker,
risk, impact, candidate, and review authorities are accepted.

## Question

The G6 plan permits only blocker fixes at RC2/Final, while the repository has
no accepted blocker taxonomy, candidate manifest, risk schema, or change-
control protocol. How can the project protect the exact six evidence classes
without classifying ordinary feature work as a release blocker?

## Decision

1. `cargo xtask rc2 verify` is an internal, offline governance command. It
   validates exactly six RC2 evidence classes in
   `docs/testing/RC2-FINAL-CHANGE-CONTROL.md` and requires their documented
   `BlockedSpec` or partial Seed states with non-empty evidence and exit cells.
2. The verifier checks the explicit blocker-only/no-claim boundary and linked
   RC0, RC1, RC3, support, protocol, and authority-audit markers. It fails
   closed with internal `GOV-RC2-CHANGE-CONTROL-*` messages on row, state,
   policy, stale-name, or marker drift.
3. The command validates inventory and historical-audit evidence only. It does
   not approve a blocker, create a candidate manifest, calculate risk, bind a
   protocol/artifact impact, run a matrix, or create a tag, and performs no
   network or system mutation.
4. The command is included in the governance-authority CI gate. RC2 promotion
   still requires accepted blocker/P0/P1 taxonomy and disposition, regression,
   risk and impact records, full relevant matrix evidence, immutable candidate
   identity, and reviewer approval. Final claims require RC-6905 publication.

## Conformance plan

- Run `cargo xtask rc2 verify` offline and assert six evidence classes with
  five `BlockedSpec`, one partial Seed state, and seven audit files.
- Mutate an evidence row/state, policy phrase, evidence cell, or linked audit
  marker and verify the gate fails closed.
- Run `cargo xtask ci verify` and the locked governance, status, support, and
  traceability checks without treating the inventory as blocker approval or a
  release candidate.
- Repeat independent processes and verify that no source fix, blocker status,
  risk acceptance, candidate, tag, artifact, network request, or system
  configuration changes.

## Compatibility impact

- Adds only an internal `cargo xtask` validator, documentation evidence, and
  CI preflight. Ling syntax, semantics, Checked Core, runtime, bytecode,
  diagnostics, schemas, Semantic IDs, dependencies, public protocols, and
  Unicode 17.0.0 behavior are unchanged.
- Existing Seed tests, audits, support and protocol inventories retain their
  current boundaries and are not promoted to RC2 or Final evidence.

## Unresolved alternatives

Blocker/P0/P1 taxonomy, owner/disposition rules, regression baseline, risk and
impact schema, protocol/artifact/version map, full matrix selection, candidate
identity/provenance, reviewer approval, rollback, and candidate regeneration
remain governed by RC2 and later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
