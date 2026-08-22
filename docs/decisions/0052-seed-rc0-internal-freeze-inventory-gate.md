# DEC-0052: Seed RC0 internal-freeze inventory gate / Seed RC0 内部冻结盘点门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: release-engineering  
> Related authority/gap: `ROADMAP-1.0`, `GAP-REGISTER`, `RC-6901`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `RC-6901-SEED` inventory child. It does
not perform or authorize an RC0 freeze, create a candidate identity, promote a
protocol, publish an artifact, or claim Ling 1.0 readiness. The parent
`RC-6901` remains `BlockedSpec` until every release authority and executable
exit is accepted.

## Question

The G6 plan lists eight RC0 checks, while the repository currently has only a
v0.0.1 Seed release report and negative/blocked G6 audits. How can the project
protect the exact RC0 checklist and its no-release-claim boundary without
inventing a freeze protocol or release artifact?

## Decision

1. `cargo xtask rc0 verify` is an internal, offline governance command. It
   validates exactly eight RC0 criteria in `docs/testing/RC0-INTERNAL-FREEZE.md`
   and requires every current state to remain `BlockedSpec` with non-empty
   evidence and exit-evidence cells.
2. The verifier checks the explicit no-freeze/no-publication policy and the
   linked Seed release, support, compatibility, security, documentation, and
   DAP audit markers. It fails closed with internal `GOV-RC0-FREEZE-*`
   messages on row, state, policy, or marker drift.
3. The command validates inventory and historical-audit evidence only. It does
   not create a tag, artifact, signature, SBOM, issue disposition, protocol,
   migration promise, public API, or release claim, and it performs no network
   or system mutation.
4. The command is included in the governance-authority CI gate. RC0 promotion
   still requires Accepted feature/protocol/support/change-control authority,
   complete P0/P1 disposition, historical-corpus and security evidence,
   reproducible artifacts, and complete 1.0 documentation.

## Conformance plan

- Run `cargo xtask rc0 verify` offline and assert eight `BlockedSpec` criteria
  and ten required audit-marker files.
- Mutate a criterion row/state, policy phrase, evidence cell, or linked audit
  marker and verify the gate fails closed.
- Run `cargo xtask ci verify` and the locked governance, status, support, and
  traceability checks without treating the inventory as a release freeze or
  artifact verification.
- Repeat independent processes and verify that no tag, artifact, protocol,
  issue-tracker state, network request, or system configuration changes.

## Compatibility impact

- Adds only an internal `cargo xtask` validator, documentation evidence, and
  CI preflight. Ling syntax, semantics, Checked Core, runtime, bytecode,
  diagnostics, schemas, Semantic IDs, dependencies, public protocols, and
  Unicode 17.0.0 behavior are unchanged.
- The v0.0.1 Seed release report remains distinct from a v1.0 RC0 candidate;
  all unsupported and incomplete release surfaces remain explicitly deferred.

## Unresolved alternatives

Candidate identity and change control, protocol reader/migration rules,
support tiers and artifacts, P0/P1 severity/disposition, historical corpus
format, threat model and advisory workflow, SBOM/provenance/signatures,
installation, and complete 1.0 documentation remain governed by the parent
RC0 and later release authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
