# DEC-0045: Seed documentation-inventory drift gate / Seed 文档清单漂移门禁

> 状态：Accepted  
> Status: Accepted  
> Proposed date: 2026-08-22  
> Decision date: 2026-08-22  
> Owner role: documentation-engineering  
> Related authority/gap: `RFC-0002`, `DEC-0018`, `GAP-REGISTER`  
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision closes only the bounded `DOC-6701-SEED` child. It does not
complete the G6 formal-documentation release gate or authorize future Task,
Actor, Replay, Native, Ownership, FFI, Kernel, Device, Critical, Contract,
Evidence, LSP, Zed, migration, security, or disclosure manuals as implemented
features. The parent `DOC-6701` remains `BlockedSpec` until their authorities,
implementations, and release evidence are Accepted.

## Question

The Seed documentation inventory already lists twelve planned manuals and
separates accepted/implemented Seed material from future or unsupported
surfaces. Without a drift check, a manual name or state could change while
governance and release reports retain a different boundary. A documentation-
only verifier can protect the inventory without writing future chapters or
promoting a support state.

## Decision

1. `cargo xtask docs verify` is an internal governance command. It reads
   `docs/testing/DOCUMENTATION-INVENTORY.md` and validates the exact twelve
   formal-set manual names and states, including four `Future / Unsupported`
   rows.
2. The verifier rejects duplicate, missing, or unexpected rows, state drift,
   a missing Formal set section, and removal of the anti-promotion policy
   phrases for plan mentions, accepted manual authority, stale names, and
   future-manual status. It fails closed with internal `GOV-DOCS-MATRIX-*`
   messages.
3. The command validates inventory only. It does not generate manuals, add
   examples, define syntax or protocols, run a documentation renderer, change
   diagnostics or schemas, or claim any future feature or support level.
4. The command is included in the governance-authority CI gate. A future
   manual or state promotion requires its own Accepted authority, normative
   links, implementation/conformance evidence, owner, and retained release
   evidence before changing the inventory.

## Conformance plan

- Run `cargo xtask docs verify` offline and assert twelve rows with four
  `Future / Unsupported` states.
- Mutate an isolated manual row or remove a policy phrase and verify the gate
  fails closed.
- Run the existing governance, status, traceability, and documentation-link
  checks without treating the inventory gate as a generated manual or feature
  implementation.
- Repeat independent processes and verify no source, semantic, diagnostic,
  schema, protocol, support, or release-state output is generated.

## Compatibility impact

- Adds only an internal `cargo xtask` validation command and CI preflight.
  Ling syntax, semantics, Checked Core, runtime, diagnostics, schemas,
  Semantic IDs, dependencies, public protocols, and Unicode 17.0.0 behavior
  are unchanged.
- No future manual, example, command, protocol, migration promise, security
  claim, or placeholder public API is introduced.

## Unresolved alternatives

Manual content, stable profile/target support, generated-doc tooling, future
examples, migration/deprecation guidance, editor/LSP documentation, and
security/disclosure operations remain governed by the parent `DOC-6701` and
later Accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
