# DEC-0046: Seed example-matrix drift gate / Seed 示例矩阵漂移门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: documentation-engineering  
> Related authority/gap: `RFC-0001`, `RFC-0019`, `DEC-0018`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `DOC-6702-SEED` child. It does not
complete the G6 two-layer-example release gate or authorize Stable capability
claims, future syntax, profiles, ownership rules, backends, migration
behavior, or public protocols. The parent `DOC-6702` remains `BlockedSpec`
until the 1.0 support matrix and release evidence are Accepted.

## Question

The Seed example-coverage document already maps seven two-layer requirements
and seven Seed feature IDs to positive, negative, Audit/Semantic, and deferred
evidence. Without a drift check, rows or feature links could change while
support and traceability registries retain a different boundary. A
documentation-only verifier can protect this inventory without running or
modifying examples.

## Decision

1. `cargo xtask examples verify` is an internal governance command. It reads
   `docs/testing/EXAMPLE-COVERAGE.md` and validates the exact seven two-layer
   requirement rows and seven `FTR-SEED-0001` through `FTR-SEED-0007` rows,
   requiring non-empty evidence cells.
2. The verifier rejects duplicate, missing, or unexpected rows, malformed or
   empty evidence cells, and removal of the explicit experimental,
   anti-placeholder, stale-name, and no-future-promotion policy phrases. It
   fails closed with internal `GOV-EXAMPLES-MATRIX-*` messages.
3. The command validates inventory only. It does not run examples, generate
   source, define syntax or APIs, promote `Experimental`/`Preview` to Stable,
   or emit a public diagnostic, schema, protocol, or support claim.
4. The command is included in the governance-authority CI gate. A future
   capability or state promotion requires its own Accepted authority,
   conformance/error evidence, owner, and retained release evidence before
   changing this matrix.

## Conformance plan

- Run `cargo xtask examples verify` offline and assert seven requirement and
  seven feature-traceability rows.
- Mutate an isolated row, remove an evidence cell, or remove a policy phrase
  and verify the gate fails closed.
- Run the existing locked process-level and conformance example tests without
  treating the inventory gate as execution or Stable-support evidence.
- Repeat independent processes and verify no source, semantic, diagnostic,
  schema, protocol, support, or release-state output is generated.

## Compatibility impact

- Adds only an internal `cargo xtask` validation command and CI preflight.
  Ling syntax, semantics, Checked Core, runtime, diagnostics, schemas,
  Semantic IDs, dependencies, public protocols, and Unicode 17.0.0 behavior
  are unchanged.
- No example, syntax, API, profile, ownership rule, migration promise,
  security claim, or placeholder public surface is added.

## Unresolved alternatives

Stable capability support, profile/effect/ownership notes, future backends,
package/editor tooling, migration behavior, and release example policy remain
governed by the parent `DOC-6702` and later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
