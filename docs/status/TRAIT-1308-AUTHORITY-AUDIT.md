# TRAIT-1308 Authority Audit: Trait IDE support

## Outcome

`TRAIT-1308` remains `BlockedSpec` for the full IDE/LSP surface. Accepted
RFC-0022 authorizes and the child task `TRAIT-1308-PROJECTION` implements only a
bounded, data-only `x-ling-trait-ide` extension on the existing Semantic Graph.
It exposes checked dictionary-witness identities and original source spans; it
does not implement hover, go-to implementation/trait, completion,
identity-preserving rename, repairs, or LSP transactions.

No IDE adapter, LSP request/response method, public Trait diagnostic,
placeholder repair, or Stable 1.0 editor claim was added. Existing v0.0.1 Seed
language and editor behavior remains unchanged.

## Normative traceability

- Accepted RFC-0005 §4.1–§4.2 requires a selected immutable witness and its
  resolved Trait/implementation identity in the checked semantic projection;
  later tools must consume that identity rather than search for an impl again.
- Accepted RFC-0005 §5.1–§5.3 allocates no Trait diagnostic code, requires
  bilingual registered diagnostics before user-visible errors, and forbids a
  CLI/LSP/protocol entry point from claiming Trait support without independent
  fixtures.
- RFC-0005 compatibility clauses leave editor exposure to separate protocol
  decisions and require any new public projection to be versioned with resolved
  implementation identity.
- Accepted DEC-0027 keeps the witness module crate-private and explicitly
  defers public diagnostics and CLI/LSP integration to later authority.
- Accepted RFC-0022 §§1–8 defines the optional `x-ling-trait-ide` projection,
  deterministic path-free Trait/implementation identities, member definition
  mapping, original UTF-8 byte spans, structural reader validation, and the
  explicit non-goals for the remaining IDE/LSP surface.
- `docs/SEMANTICS.md` requires stable Semantic IDs, original UTF-8 spans, and
  atomic Semantic Transactions; it does not define a Trait-specific editor
  schema or repair contract. Trait syntax remains outside the v0.0.1 Seed
  support matrix.

## Current interface evidence

The current repository confirms that only the bounded graph projection exists:

- `ling-types::checked_core` now supplies the immutable witness consumed by
  `ling-semantic`; `ProgramSnapshot` and `ProjectProgramSnapshot` optionally
  carry the RFC-0022 projection with stable member definition IDs and original
  source spans.
- The projection is data-only. It does not define callable method-slot
  requests, hover rendering, completion ranking, rename edits, or repair facts.
- The v0.0.1 support matrix is unchanged; the v0.1 static Trait slice is an
  accepted implementation boundary and is not a claim that the full editor
  protocol exists.
- The open governance gaps
  `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave position/snapshot/version
  preconditions, Workspace Edit fields, and Stable versus Experimental
  compatibility unresolved.
- `GAP-TRAIT-COHERENCE-001` still records the unintegrated full Trait IDE/LSP
  surface; RFC-0022 and the child projection resolve only its graph-data
  sub-slice.

## Required authority before implementation

The remaining parent task requires accepted authority for, at minimum:

1. hover, completion, definition, implementation, and rename request/response
   schemas, including position encoding, document-version preconditions,
   transaction atomicity, and Stable versus Experimental lifecycle labels;
2. bilingual registered diagnostic codes and structured Facts/repairs for
   missing, ambiguous, orphan, overlapping, and unsatisfied constraints,
   including the rule that repairs cannot synthesize an unsafe orphan impl;
3. cross-package identity and source-span behavior for editor operations,
   without re-running Trait selection or exposing host paths; and
4. positive, negative, deterministic, cross-package, stale-document, rename,
   and editor/semantic differential fixtures.

Until those decisions are Accepted, changing an LSP/IDE crate or adding
request/response fields would invent public behavior, leak unresolved
transaction semantics, or claim support that the v0.0.1 matrix explicitly does
not authorize.

## Evidence and compatibility

This audit was checked against `docs/RFC-0005.md`,
`docs/RFC-0022.md`,
`docs/decisions/0027-trait-checked-core-dictionary-witness.md`,
`docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`,
`docs/ling_execution_plan/03-G1-V0.1-LIVING.md`, and the current
`ling-types`, `ling-effects`, `ling-semantic`, and editor-facing source tree.
The bounded projection changed the optional Semantic Graph extension only; no
diagnostic allocation, LSP wire method, bytecode, VM, or Unicode 17.0.0
contract changed.

## Intentionally deferred

The child projection is complete under RFC-0022. The parent `TRAIT-1308`
remains deferred until the full method identity/query surface, diagnostic
registry, LSP/Semantic Transaction contracts, and their fixtures are Accepted.
Any follow-on implementation must consume the projected immutable witness and
must not re-run Trait selection or invent repair behavior.
