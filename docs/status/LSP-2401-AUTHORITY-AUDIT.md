# LSP-2401 Authority Audit: Semantic-Token Taxonomy

## Outcome

`LSP-2401` is correctly recorded as `BlockedSpec`. The execution plan proposes
standard LSP token types, Ling-specific types such as `effect`, `capability`,
`resource`, `actor`, `node`, `kernel`, and `semanticId`, and modifiers such as
`declaration`, `mutable`, `borrowed`, and `generated`. No accepted repository
decision defines this taxonomy, client negotiation, semantic mapping, or
fallback behavior.

No token taxonomy RFC, protocol registry entry, token generator, custom token
kind, modifier, diagnostic allocation, or placeholder LSP surface was added.

## Normative traceability

- The execution package is non-normative; the listed standard/custom token
  names and modifiers are design inputs only.
- `docs/SEMANTICS.md` defines language concepts such as Effect, Capability,
  Ownership, and Semantic IDs, but it does not authorize their presentation as
  LSP token types or modifiers.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires an
  explicit SourceMap projection for LSP UTF-16 positions. It does not define
  token ranges, overlap, ordering, or client encoding negotiation.
- DEC-0012 fixes Semantic IDs/canonical bytes. The registered
  `PROTO-SEMANTIC-GRAPH-JSON` projections are Experimental and do not define a
  semantic-token taxonomy or `semanticId` token disclosure.
- `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves graph/transaction field
  stability, compatibility, stale handling, and migration open;
  `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves editor position/version and
  protocol fields open. RFC-0005/DEC-0027 explicitly provide no public Trait or
  witness projection.

## Current interface evidence

- `ling-semantic` produces checked/Experimental graph definitions, nodes, and
  references, but no token categories, modifiers, source-origin policy, or LSP
  writer.
- Compiler syntax/highlight fixtures and Tree-sitter captures are not a typed
  Semantic Token protocol and cannot authorize semantic categories.
- `ling-source` preserves byte spans and scalar columns, but no negotiated
  UTF-16 mapping, same-version requirement, non-overlap validation, or client
  fallback behavior exists.
- No token taxonomy registry, capability negotiation, profile/redaction policy,
  or fixture covers standard/custom mappings, unknown custom kinds, effect/
  capability disclosure, Semantic IDs, generated/error-recovery regions,
  Unicode/CRLF/BOM, ordering, or migration.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. standard-token mapping and the necessity, names, versioning, and client
   capability negotiation of every Ling-specific type/modifier;
2. source categories and precedence for syntax, resolved HIR/Checked Core,
   Effect/Capability, ownership/mutability, declarations, generated/dependency,
   and error-recovery fallback, including redaction and profile policy;
3. token span truth, UTF-8/UTF-16 SourceMap projection, same-document-version
   binding, non-overlap/position ordering, duplicate/conflict resolution, and
   Semantic ID/provenance treatment;
4. full/delta protocol fields, unknown-token fallback, cancellation/limits,
   protocol inventory, Stable versus Experimental lifecycle, localization, and
   migration; and
5. executable positive/negative fixtures for standard/custom mappings,
   modifiers, unknown-client fallback, Chinese columns/emoji, CRLF/BOM,
   nested/shadowed symbols, mutable fields, effects/capabilities, generated and
   syntax-error regions, deterministic ordering, version mismatch, and
   migration.

Until these decisions are Accepted, token kinds could expose unapproved
capability/effect information, bind semantic output to non-authoritative spans,
or become an irreversible client compatibility surface.

## Evidence and compatibility

This audit was checked against `docs/SEMANTICS.md`, `docs/LANGUAGE.md`,
`docs/ROADMAP-1.0.md`, DEC-0002, DEC-0012, RFC-0005,
`docs/decisions/0027-trait-checked-core-dictionary-witness.md`,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the `ling-semantic`, `ling-source`, `ling-syntax`, and editor fixture
directories.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`LSP-2401` can begin after the token taxonomy, position/version, Semantic Graph
lifecycle, and client negotiation decisions are Accepted. The future
implementation must derive tokens from checked data, preserve source-span and
identity truth, redact capabilities explicitly, provide deterministic fallback,
and label experimental categories.
