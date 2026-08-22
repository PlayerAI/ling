# DEC-0054: Seed RC3 independent-verification inventory gate / Seed RC3 独立验证盘点门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: release-engineering  
> Related authority/gap: `ROADMAP-1.0`, `DEC-0053`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `RC-6903-SEED` inventory child. It does
not perform or authorize independent verification, identify a reviewer,
create a tag or artifact, or make a Go/No-Go decision. The parent `RC-6903`
remains `BlockedSpec` until an immutable candidate, accepted review protocol,
and independently reproduced evidence exist.

## Question

The G6 plan requires an independent reviewer to build, verify, reproduce, and
compare a release candidate, while the repository currently has only
self-validation for the v0.0.1 Seed boundary. How can the project protect the
exact RC3 readiness matrix without labeling an implementation-agent check as
independent evidence?

## Decision

1. `cargo xtask rc3 verify` is an internal, offline governance command. It
   validates exactly seven RC3 checks in
   `docs/testing/RC3-INDEPENDENT-VERIFICATION.md` and requires their documented
   `BlockedSpec` or partial Seed states with non-empty evidence and exit cells.
2. The verifier checks the explicit no-independent-sign-off boundary and
   linked RC0, RC1, Seed, security, support, and authority-audit markers. It
   fails closed with internal `GOV-RC3-VERIFICATION-*` messages on row, state,
   policy, stale-name, or marker drift.
3. The command validates inventory and historical-audit evidence only. It does
   not build a tag, verify an artifact, contact a reviewer or service, create a
   signature/evidence bundle, or make a Go/No-Go decision, and performs no
   network or system mutation.
4. The command is included in the governance-authority CI gate. RC3 promotion
   still requires an Accepted candidate identity, independence/conflict policy,
   clean-environment and toolchain capture, reproducible artifact and
   conformance evidence, retention rules, and recorded sign-off.

## Conformance plan

- Run `cargo xtask rc3 verify` offline and assert seven checks with three
  `BlockedSpec`, four partial Seed states, and seven audit files.
- Mutate a check row/state, policy phrase, evidence cell, or linked audit
  marker and verify the gate fails closed.
- Run `cargo xtask ci verify` and the locked governance, status, support, and
  traceability checks without treating the inventory as independent review.
- Repeat independent processes and verify that no tag, artifact, reviewer
  identity, signature, evidence bundle, network request, or system
  configuration changes.

## Compatibility impact

- Adds only an internal `cargo xtask` validator, documentation evidence, and
  CI preflight. Ling syntax, semantics, Checked Core, runtime, bytecode,
  diagnostics, schemas, Semantic IDs, dependencies, public protocols, and
  Unicode 17.0.0 behavior are unchanged.
- Existing Seed self-validation remains explicitly non-independent and is not
  promoted to an RC3 or Stable 1.0 sign-off.

## Unresolved alternatives

Candidate identity and immutability, reviewer independence/conflict
disclosure, clean environment and toolchain capture, artifact/signature/
provenance verification, candidate-wide conformance/corruption scope, TCB and
security review, evidence retention, rerun policy, and Go/No-Go format remain
governed by RC3 and later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
