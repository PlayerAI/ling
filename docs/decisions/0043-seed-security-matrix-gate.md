# DEC-0043: Seed security-audit matrix gate / Seed 安全审计矩阵门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: security-engineering  
> Related authority/gap: `DEC-0022`, `RFC-0002`, `RFC-0020`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `REL-6603-SEED` child. It does not
complete the G6 security release gate or authorize a threat model, FFI/TCB,
archive/build sandbox, remote, replay/evidence, device, editor-binary,
advisory, license, SBOM, provenance, or disclosure protocol. The parent
`REL-6603` remains `BlockedSpec` until those authorities and release evidence
are Accepted.

## Question

The Seed security audit already records nine required surfaces and separates
current controls from future work. Without an internal drift check, a row or
state could change while the authority audit and release boundary remain
unchanged. A documentation-only verifier can protect that inventory without
inventing security behavior or claiming that the implementation is secure.

## Decision

1. `cargo xtask security verify` is an internal governance command. It reads
   `docs/testing/SECURITY-AUDIT.md` and validates the exact nine surface names
   and states: three Covered variants, two Partial rows, and four Deferred
   rows.
2. The verifier rejects duplicate, missing, or unexpected rows, state drift,
   and removal of the threat-model, accepted-decision, hostile-input,
   advisory/license/SBOM/provenance, incident/disclosure, and explicit
   no-security-API policy phrases. It fails closed with internal
   `GOV-SECURITY-*` messages.
3. The command validates audit inventory only. It does not run advisory or
   license scanners, inspect remote services, parse hostile archives, verify
   binaries, define a threat model, emit a public diagnostic or protocol, or
   make a vulnerability-free or G6 sign-off claim.
4. The command is included in the existing Seed reproducibility CI gate. A
   future security surface or state promotion requires its own Accepted
   authority, deterministic fixtures, owner, and retained evidence before the
   matrix or gate is changed.

## Conformance plan

- Run `cargo xtask security verify` offline and assert nine rows with the
  expected 3/2/4 Covered/Partial/Deferred distribution.
- Mutate an isolated matrix row and verify a missing or changed state fails
  closed; remove a required policy phrase and verify the gate fails closed.
- Run the existing locked project, effects, VM, workspace, metadata, and
  Clippy checks as evidence for current controls, without treating the matrix
  gate as an advisory scan, penetration test, or cross-platform security
  result.
- Repeat independent processes and verify no source, semantic, diagnostic,
  schema, protocol, support, or release-state output is generated.

## Compatibility impact

- Adds only an internal `cargo xtask` validation command and CI preflight.
  Ling source syntax, Checked Core, runtime, bytecode, diagnostics, schemas,
  Semantic IDs, public protocols, dependencies, and Unicode 17.0.0 behavior
  are unchanged.
- No security API, FFI, sandbox, remote adapter, replay/evidence decoder,
  device runtime, editor updater, SBOM schema, advisory result, or public
  support claim is introduced.

## Unresolved alternatives

Threat-model scope, trust boundaries, native/FFI isolation, archive quotas,
remote authentication/provenance, replay/evidence privacy, editor trust roots,
advisory/license/SBOM tooling, and disclosure ownership remain governed by
the parent `REL-6603` and later Accepted security authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
