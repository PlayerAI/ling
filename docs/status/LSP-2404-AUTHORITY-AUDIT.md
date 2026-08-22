# LSP-2404 Authority Audit: Semantic-Token Fixtures

## Outcome

`LSP-2404` is correctly recorded as `BlockedSpec`. The execution plan asks for
semantic-token fixtures covering Chinese columns, emoji prefixes, same names in
different scopes, mutable fields, variant constructors, Effect/Capability, and
error recovery. The repository has no accepted token taxonomy, typed/fallback
origin contract, full/delta transport schema, position/version rule, or
fixture schema that can make those expected outputs normative.

No semantic-token fixture corpus, expected token output, protocol schema,
diagnostic allocation, Semantic ID change, or placeholder LSP surface was
added. Accepted DEC-0087 and the bounded `LSP-2404-CHECKED-SOURCE-FIXTURES`
child add only compiler-owned evidence for original bytes, spans, source order,
and VFS revision invalidation; public semantic-token fixtures remain blocked.

## Normative traceability

- The execution package is non-normative; its fixture list does not authorize
  token categories, source positions, fallback behavior, or wire fields.
- LSP-2401 taxonomy, LSP-2402 typed generation, and LSP-2403 full/delta
  transport are `BlockedSpec`, so fixture expectations cannot freeze their
  unresolved interfaces.
- DEC-0002 makes original UTF-8 `SourceId + Span` authoritative and requires
  an explicit SourceMap projection for LSP UTF-16 positions. It does not define
  token legends, column encoding in fixtures, document versions, or recovery
  markers.
- DEC-0012 fixes Semantic IDs and canonical bytes. The registered Semantic
  Graph projections are Experimental and do not define token fixture identity,
  source provenance, or cache compatibility.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` leaves snapshot/version preconditions and
  Stable versus Experimental editor fields open, while
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leaves protocol stability, reader/writer
  compatibility, stale rejection, and migration open.
- DEC-0019 defines an internal incremental-query boundary and explicitly does
  not authorize a public LSP request, position encoding, or protocol field.
- Accepted DEC-0087 authorizes only internal checked-token source fixtures; no
  expected semantic-token output or fixture wire schema is defined.

## Current fixture evidence

- Existing syntax, semantic-graph, project, diagnostic, and bytecode fixtures
  validate their own accepted contracts; none emits or reads semantic-token
  legends, typed/fallback origin, full/delta results, or token modifiers.
- `ling-source` preserves original UTF-8 spans and SourceMap data, but no
  fixture binds a token result to one document snapshot or validates UTF-16
  projection, Chinese scalar columns, emoji surrogate pairs, CRLF, or BOM
  behavior.
- `ling-types`, `ling-effects`, and `ling-semantic` expose checked semantic
  data, but no fixture defines token mapping for mutable bindings, variant
  constructors, Effect/Capability labels, shadowed names, or generated and
  dependency regions.
- No fixture proves error-region fallback, rejects unchecked-AST-derived
  tokens, compares full and delta results, validates non-overlap/order, or
  fixes deterministic bytes and result-ID behavior.
- The checked-source fixture child covers BOM/CRLF/Unicode original-byte
  slices, source order, cache reuse, and revision invalidation without adding
  token categories or protocol fields.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. a versioned fixture/schema model, token legend and modifiers, typed versus
   parsed-fallback origin, source provenance, and the exact expected output
   representation;
2. UTF-8 span truth and UTF-8/UTF-16 position projection, including Chinese
   scalar columns, emoji prefixes/surrogate pairs, CRLF, BOM, and document
   snapshot/version binding;
3. mapping rules for declarations, references, same-name different scopes,
   mutable fields, variant constructors, Effect/Capability, generated or
   dependency data, and redaction/unsupported cases;
4. error recovery boundaries, incomplete input behavior, unchecked-AST
   prohibition, full/delta equivalence, ordering/non-overlap, deterministic
   serialization, cancellation, limits, stale results, and result-ID/base
   handling; and
5. executable positive/negative/migration fixtures with independent readers,
   protocol inventory entries, Stable versus Experimental labels, diagnostic
   expectations, and compatibility/version-upgrade rules.

Until those decisions are Accepted, checked-in expected token outputs could
turn an execution-plan example into an accidental public protocol, encode the
wrong position unit, or bless semantic labels for unresolved/error data.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0002, DEC-0012, DEC-0019,
`docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, source, type, effect, semantic, diagnostic, and
fixture directories.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

The bounded checked-source fixture child is complete under DEC-0087. Public
`LSP-2404` can begin after LSP-2401 taxonomy, LSP-2402 generation, LSP-2403
transport, LSP-2501/LSP-2502 snapshot and cancellation decisions, and the
Semantic Graph lifecycle contract are Accepted. The future fixture corpus must
be independent, deterministic, versioned, position-unit explicit, and sourced
only from checked Typed Core/Resolved HIR or explicitly marked parsed fallback.
