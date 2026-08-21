# TRAIT-1308 Authority Audit: Trait IDE support

## Outcome

`TRAIT-1308` is correctly recorded as `BlockedSpec`. It is the next
execution-plan item after the dictionary-lowering boundary, but the accepted
authorities do not yet define the public editor projection needed to implement
hover, go-to implementation/trait, member completion, identity-preserving
rename, or safe constraint fixes.

No IDE adapter, LSP field, public Trait diagnostic, Semantic ID rule,
placeholder repair, or editor-facing method table was added. Existing Seed
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
  defers `TypedProgram`, the Semantic Graph projection, public diagnostics, and
  CLI/LSP integration to later authority.
- `docs/SEMANTICS.md` requires stable Semantic IDs, original UTF-8 spans, and
  atomic Semantic Transactions; it does not define a Trait-specific editor
  schema or repair contract. Trait syntax remains outside the v0.0.1 Seed
  support matrix.

## Current interface evidence

The current repository confirms that the IDE boundary is missing:

- `ling-types::checked_core` contains internal witness identity only; it is not
  attached to `TypedProgram`, `CheckedProgram`, `ProgramSnapshot`, or a public
  Semantic Graph snapshot.
- The witness stores ordered member name evidence, not callable definition or
  stable editor method-slot identity. An IDE cannot safely navigate to or
  complete a member from that evidence alone.
- The current checker rejects Trait-bearing programs at the Seed boundary, so
  there is no accepted checked diagnostic stream from which a Trait hover or
  constraint repair could be projected.
- The open governance gaps
  `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave position/snapshot/version
  preconditions, Workspace Edit fields, and Stable versus Experimental
  compatibility unresolved.
- `GAP-TRAIT-COHERENCE-001` records the still-unintegrated Trait projection and
  lowering boundary. No accepted protocol inventory entry or executable editor
  fixture authorizes a Trait-specific public surface.

## Required authority before implementation

An implementation-ready decision or RFC must define, at minimum:

1. the versioned Semantic Graph projection for Trait declarations,
   obligations, selected implementations, ordered members, and their stable
   Semantic IDs;
2. the identity and source-span mapping from each witness member to an
   executable definition or method slot, including go-to and rename behavior
   across packages and localized aliases;
3. bilingual registered diagnostic codes and structured Facts/repairs for
   missing, ambiguous, orphan, overlapping, and unsatisfied constraints,
   including the rule that repairs cannot synthesize an unsafe orphan impl;
4. hover, completion, definition, implementation, and rename request/response
   schemas, position encoding, document-version preconditions, transaction
   atomicity, and Stable versus Experimental lifecycle labels; and
5. positive, negative, deterministic, cross-package, stale-document, rename,
   and editor/semantic differential fixtures.

Until those decisions are Accepted, changing an LSP/IDE crate or adding
Trait-specific fields to an existing protocol would invent public behavior,
leak unstable identity, or claim support that the v0.0.1 matrix explicitly
does not authorize.

## Evidence and compatibility

This audit was checked against `docs/RFC-0005.md`,
`docs/decisions/0027-trait-checked-core-dictionary-witness.md`,
`docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/governance/gap-register.toml`,
`docs/governance/protocol-inventory.toml`,
`docs/ling_execution_plan/03-G1-V0.1-LIVING.md`, and the current
`ling-types`, `ling-effects`, `ling-semantic`, and editor-facing source tree.
No code or public protocol behavior changed; no diagnostic allocation, schema,
Semantic ID, source-span, bytecode, VM, or Unicode 17.0.0 claim is made.

## Intentionally deferred

`TRAIT-1308` can begin after the Trait semantic projection, method identity,
diagnostic registry, and LSP/Semantic Transaction contracts are Accepted and
their fixtures exist. Its first implementation should project the approved
immutable witness and stable identities; it must not re-run Trait selection or
invent repair behavior.
